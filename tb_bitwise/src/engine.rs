use tracing::info;
use crate::precompute::EngineCache;
use crate::targets::TargetOutcomes;
use std::sync::Arc;
use tb_core::ast::Expr;
use tb_math::primitives;

use dashmap::DashMap;

pub use crate::metrics::StrategyResult;
use crate::metrics::StrategyMetrics;

#[derive(Debug, Default)]
pub struct TelemetryRecord {
    pub calls: std::sync::atomic::AtomicU64,
    pub total_time_us: std::sync::atomic::AtomicU64,
}

pub struct BitwiseEngine {
    pub cache: Arc<EngineCache>,
    pub targets: Arc<TargetOutcomes>,
    pub cache_hits: std::sync::atomic::AtomicU64,
    pub cache_misses: std::sync::atomic::AtomicU64,
    pub telemetry: DashMap<&'static str, Arc<TelemetryRecord>>,
}

impl BitwiseEngine {
    pub fn new(cache: Arc<EngineCache>, targets: Arc<TargetOutcomes>) -> Self {
        info!("Initializing JIT Bitwise Engine for continuous AST evaluation.");
        Self { 
            cache, 
            targets,
            cache_hits: std::sync::atomic::AtomicU64::new(0),
            cache_misses: std::sync::atomic::AtomicU64::new(0),
            telemetry: DashMap::new(),
        }
    }

    /// Recursively evaluates a float expression using the EngineCache for speed,
    /// falling back to JIT calculation if the expression isn't cached.
    pub fn eval_float(&self, expr: &Expr) -> Arc<Vec<f64>> {
        // FAST PATH: Intercept leaf nodes to bypass DashMap lock contention entirely
        match expr {
            Expr::Close => return self.cache.close.clone(),
            Expr::Open => return self.cache.open.clone(),
            Expr::High => return self.cache.high.clone(),
            Expr::Low => return self.cache.low.clone(),
            Expr::Volume => return self.cache.volume.clone(),
            Expr::TrueRange => return self.cache.true_range.clone(),
            _ => {} // Fall through for complex expressions
        }

        if let Some(cached) = self.cache.series.get(expr) {
            self.cache_hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return cached.clone();
        }

        self.cache_misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let start_time = std::time::Instant::now();
        let (node_name, data) = match expr {
            Expr::Constant { value } => ("Constant", vec![*value; self.cache.data_len]),
            Expr::Add { lhs, rhs } => ("Add", primitives::add(&self.eval_float(lhs), &self.eval_float(rhs))),
            Expr::Sub { lhs, rhs } => ("Sub", primitives::sub(&self.eval_float(lhs), &self.eval_float(rhs))),
            Expr::Mul { lhs, rhs } => ("Mul", primitives::mul(&self.eval_float(lhs), &self.eval_float(rhs))),
            Expr::Div { lhs, rhs } => ("Div", primitives::div(&self.eval_float(lhs), &self.eval_float(rhs))),
            Expr::Abs { source } => ("Abs", primitives::abs(&self.eval_float(source))),
            
            Expr::Delay { source, period } => ("Delay", primitives::delay(&self.eval_float(source), *period as usize)),
            Expr::TsMax { source, period } => ("TsMax", primitives::ts_max(&self.eval_float(source), *period as usize)),
            Expr::TsMin { source, period } => ("TsMin", primitives::ts_min(&self.eval_float(source), *period as usize)),
            Expr::TsSum { source, period } => ("TsSum", primitives::ts_sum(&self.eval_float(source), *period as usize)),
            
            Expr::Sma { source, period } => ("Sma", primitives::sma(&self.eval_float(source), *period as usize)),
            Expr::Ema { source, period } => ("Ema", primitives::ema(&self.eval_float(source), *period as usize)),
            Expr::Wma { source, period } => ("Wma", primitives::wma(&self.eval_float(source), *period as usize)),
            Expr::Rma { source, period } => ("Rma", primitives::rma(&self.eval_float(source), *period as usize)),
            
            Expr::StdDev { source, period } => ("StdDev", primitives::std_dev(&self.eval_float(source), *period as usize)),
            Expr::LinRegSlope { source, period } => ("LinRegSlope", primitives::lin_reg_slope(&self.eval_float(source), *period as usize)),
            
            _ => ("Unknown", vec![f64::NAN; self.cache.data_len]),
        };

        let elapsed = start_time.elapsed().as_micros() as u64;
        let record = self.telemetry.entry(node_name).or_insert_with(|| Arc::new(TelemetryRecord::default()));
        record.calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        record.total_time_us.fetch_add(elapsed, std::sync::atomic::Ordering::Relaxed);

        let data = Arc::new(data);
        self.cache.series.insert(expr.clone(), data.clone());
        data
    }

    pub fn eval_boolean(&self, expr: &Expr) -> Arc<Vec<u64>> {
        if let Some(cached) = self.cache.bool_series.get(expr) {
            return cached.clone();
        }
        
        let num_blocks = (self.cache.data_len + 63) / 64;
        let mut bitset = vec![0u64; num_blocks];

        let start_time = std::time::Instant::now();
        let mut node_name = "UnknownBool";

        match expr {
            Expr::GreaterThan { lhs, rhs } => {
                node_name = "GreaterThan";
                let l = self.eval_float(lhs);
                let r = self.eval_float(rhs);
                for i in 0..self.cache.data_len {
                    if !l[i].is_nan() && !r[i].is_nan() && l[i] > r[i] {
                        bitset[i / 64] |= 1 << (i % 64);
                    }
                }
            }
            Expr::LessThan { lhs, rhs } => {
                node_name = "LessThan";
                let l = self.eval_float(lhs);
                let r = self.eval_float(rhs);
                for i in 0..self.cache.data_len {
                    if !l[i].is_nan() && !r[i].is_nan() && l[i] < r[i] {
                        bitset[i / 64] |= 1 << (i % 64);
                    }
                }
            }
            Expr::CrossAbove { lhs, rhs } => {
                node_name = "CrossAbove";
                let l = self.eval_float(lhs);
                let r = self.eval_float(rhs);
                for i in 1..self.cache.data_len {
                    if !l[i].is_nan() && !r[i].is_nan() && !l[i-1].is_nan() && !r[i-1].is_nan() {
                        if l[i-1] <= r[i-1] && l[i] > r[i] {
                            bitset[i / 64] |= 1 << (i % 64);
                        }
                    }
                }
            }
            Expr::CrossBelow { lhs, rhs } => {
                node_name = "CrossBelow";
                let l = self.eval_float(lhs);
                let r = self.eval_float(rhs);
                for i in 1..self.cache.data_len {
                    if !l[i].is_nan() && !r[i].is_nan() && !l[i-1].is_nan() && !r[i-1].is_nan() {
                        if l[i-1] >= r[i-1] && l[i] < r[i] {
                            bitset[i / 64] |= 1 << (i % 64);
                        }
                    }
                }
            }
            Expr::And { lhs, rhs } => {
                node_name = "And";
                let l = self.eval_boolean(lhs);
                let r = self.eval_boolean(rhs);
                for i in 0..num_blocks {
                    bitset[i] = l[i] & r[i];
                }
            }
            Expr::Or { lhs, rhs } => {
                node_name = "Or";
                let l = self.eval_boolean(lhs);
                let r = self.eval_boolean(rhs);
                for i in 0..num_blocks {
                    bitset[i] = l[i] | r[i];
                }
            }
            _ => {}
        }
        
        let elapsed = start_time.elapsed().as_micros() as u64;
        let record = self.telemetry.entry(node_name).or_insert_with(|| Arc::new(TelemetryRecord::default()));
        record.calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        record.total_time_us.fetch_add(elapsed, std::sync::atomic::Ordering::Relaxed);

        let bitset = Arc::new(bitset);
        self.cache.bool_series.insert(expr.clone(), bitset.clone());
        bitset
    }

    pub fn unroll_macros(expr: &mut Expr) {
        match expr {
            Expr::Macro { name, output, source, params } => {
                Self::unroll_macros(source);
                let src = *source.clone();
                
                *expr = match name.as_str() {
                    "SMA" => {
                        let period = params.iter().find(|(k, _)| k == "LEN").map(|(_, v)| *v as u32).unwrap_or(14);
                        Expr::Sma { source: Box::new(src), period }
                    },
                    "EMA" => {
                        let period = params.iter().find(|(k, _)| k == "LEN").map(|(_, v)| *v as u32).unwrap_or(14);
                        Expr::Ema { source: Box::new(src), period }
                    },
                    "BOLL" => {
                        let period = params.iter().find(|(k, _)| k == "LEN").map(|(_, v)| *v as u32).unwrap_or(20);
                        let mult = params.iter().find(|(k, _)| k == "MULT").map(|(_, v)| *v).unwrap_or(2.0);
                        match output.as_str() {
                            "Upper" => tb_indicators::templates::bollinger_upper(src, period, mult),
                            "Basis" => Expr::Sma { source: Box::new(src), period },
                            "Lower" => tb_indicators::templates::bollinger_lower(src, period, mult),
                            _ => tb_indicators::templates::bollinger_upper(src, period, mult)
                        }
                    },
                    "MACD" => {
                        let fast = params.iter().find(|(k, _)| k == "FAST").map(|(_, v)| *v as u32).unwrap_or(12);
                        let slow = params.iter().find(|(k, _)| k == "SLOW").map(|(_, v)| *v as u32).unwrap_or(26);
                        let signal = params.iter().find(|(k, _)| k == "SIGNAL").map(|(_, v)| *v as u32).unwrap_or(9);
                        match output.as_str() {
                            "Line" => tb_indicators::templates::macd_line(src, fast, slow),
                            "Signal" => tb_indicators::templates::macd_signal(src, fast, slow, signal),
                            _ => tb_indicators::templates::macd_line(src, fast, slow)
                        }
                    },
                    "RSI" => {
                        let period = params.iter().find(|(k, _)| k == "LEN").map(|(_, v)| *v as u32).unwrap_or(14);
                        tb_indicators::templates::normalized_momentum(src, period)
                    },
                    "ATR" => {
                        let period = params.iter().find(|(k, _)| k == "LEN").map(|(_, v)| *v as u32).unwrap_or(14);
                        Expr::Rma { source: Box::new(src), period }
                    },
                    _ => src 
                };
                
                Self::unroll_macros(expr);
            },
            Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } | Expr::Mul { lhs, rhs } | Expr::Div { lhs, rhs } |
            Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } | Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
            Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => {
                Self::unroll_macros(lhs);
                Self::unroll_macros(rhs);
            },
            Expr::Sma { source, .. } | Expr::Ema { source, .. } | Expr::Wma { source, .. } | Expr::Rma { source, .. } |
            Expr::TsMax { source, .. } | Expr::TsMin { source, .. } | Expr::TsSum { source, .. } |
            Expr::StdDev { source, .. } | Expr::LinRegSlope { source, .. } | Expr::Delay { source, .. } | Expr::Abs { source } => {
                Self::unroll_macros(source);
            },
            _ => {}
        }
    }

    pub fn merge_conditions(&self, conditions: &[Expr]) -> Vec<u64> {
        let num_blocks = (self.cache.data_len + 63) / 64;
        let mut final_bitset = vec![u64::MAX; num_blocks];
        
        for cond in conditions {
            let mut unrolled = cond.clone();
            Self::unroll_macros(&mut unrolled);
            let cond_bits = self.eval_boolean(&unrolled);
            for i in 0..num_blocks {
                final_bitset[i] &= cond_bits[i];
            }
        }
        final_bitset
    }

    pub fn evaluate_long(&self, conditions: &[Expr]) -> StrategyResult {
        if conditions.is_empty() {
            return StrategyMetrics::new().finalize(self.targets.long_pnl.len());
        }

        let strategy_bitset = self.merge_conditions(conditions);
        let mut metrics = StrategyMetrics::new();

        for i in 0..strategy_bitset.len() {
            let mut remaining = strategy_bitset[i];
            while remaining != 0 {
                let bit_idx = remaining.trailing_zeros() as usize;
                let global_idx = (i * 64) + bit_idx;
                if global_idx < self.targets.long_pnl.len() {
                    metrics.add_trade(self.targets.long_pnl[global_idx]);
                }
                remaining &= remaining - 1;
            }
        }
        metrics.finalize(self.targets.long_pnl.len())
    }

    pub fn evaluate_short(&self, conditions: &[Expr]) -> StrategyResult {
        if conditions.is_empty() {
            return StrategyMetrics::new().finalize(self.targets.short_pnl.len());
        }

        let strategy_bitset = self.merge_conditions(conditions);
        let mut metrics = StrategyMetrics::new();

        for i in 0..strategy_bitset.len() {
            let mut remaining = strategy_bitset[i];
            while remaining != 0 {
                let bit_idx = remaining.trailing_zeros() as usize;
                let global_idx = (i * 64) + bit_idx;
                if global_idx < self.targets.short_pnl.len() {
                    metrics.add_trade(self.targets.short_pnl[global_idx]);
                }
                remaining &= remaining - 1;
            }
        }
        metrics.finalize(self.targets.short_pnl.len())
    }

    pub fn evaluate_symmetric(&self, conditions: &[Expr]) -> StrategyResult {
        if conditions.is_empty() {
            return StrategyMetrics::new().finalize(self.targets.long_pnl.len());
        }

        let strategy_bitset = self.merge_conditions(conditions);
        let mut metrics = StrategyMetrics::new();

        for i in 0..strategy_bitset.len() {
            let mut remaining = strategy_bitset[i];
            while remaining != 0 {
                let bit_idx = remaining.trailing_zeros() as usize;
                let global_idx = (i * 64) + bit_idx;
                if global_idx < self.targets.long_pnl.len() {
                    metrics.add_trade(self.targets.long_pnl[global_idx]);
                }
                remaining &= remaining - 1;
            }
            
            // For shorts, evaluate the inverse
            let mut short_remaining = !strategy_bitset[i];
            while short_remaining != 0 {
                let bit_idx = short_remaining.trailing_zeros() as usize;
                let global_idx = (i * 64) + bit_idx;
                if global_idx < self.targets.short_pnl.len() {
                    metrics.add_trade(self.targets.short_pnl[global_idx]);
                }
                short_remaining &= short_remaining - 1;
            }
        }
        metrics.finalize(self.targets.long_pnl.len())
    }

    /// Generates a chronological equity curve for a long strategy.
    pub fn evaluate_with_curve_long(&self, conditions: &[Expr]) -> Vec<f64> {
        let total_bars = self.targets.long_pnl.len();
        let mut curve = vec![0.0; total_bars];
        if conditions.is_empty() {
            return curve;
        }

        let strategy_bitset = self.merge_conditions(conditions);
        let mut current_pnl = 0.0;

        for i in 0..strategy_bitset.len() {
            for bit_idx in 0..64 {
                let global_idx = (i * 64) + bit_idx;
                if global_idx >= total_bars {
                    break;
                }
                
                if (strategy_bitset[i] & (1 << bit_idx)) != 0 {
                    current_pnl += self.targets.long_pnl[global_idx];
                }
                curve[global_idx] = current_pnl;
            }
        }
        
        curve
    }

    /// Generates a chronological equity curve for a short strategy.
    pub fn evaluate_with_curve_short(&self, conditions: &[Expr]) -> Vec<f64> {
        let total_bars = self.targets.short_pnl.len();
        let mut curve = vec![0.0; total_bars];
        if conditions.is_empty() {
            return curve;
        }

        let strategy_bitset = self.merge_conditions(conditions);
        let mut current_pnl = 0.0;

        for i in 0..strategy_bitset.len() {
            for bit_idx in 0..64 {
                let global_idx = (i * 64) + bit_idx;
                if global_idx >= total_bars {
                    break;
                }
                
                if (strategy_bitset[i] & (1 << bit_idx)) != 0 {
                    current_pnl += self.targets.short_pnl[global_idx];
                }
                curve[global_idx] = current_pnl;
            }
        }
        
        curve
    }

    /// Generates a chronological equity curve for a symmetric strategy.
    pub fn evaluate_with_curve_symmetric(&self, conditions: &[Expr]) -> Vec<f64> {
        let total_bars = self.targets.long_pnl.len();
        let mut curve = vec![0.0; total_bars];
        if conditions.is_empty() {
            return curve;
        }

        let strategy_bitset = self.merge_conditions(conditions);
        let mut current_pnl = 0.0;

        for i in 0..strategy_bitset.len() {
            for bit_idx in 0..64 {
                let global_idx = (i * 64) + bit_idx;
                if global_idx >= total_bars {
                    break;
                }
                
                if (strategy_bitset[i] & (1 << bit_idx)) != 0 {
                    current_pnl += self.targets.long_pnl[global_idx];
                } else if global_idx < self.targets.short_pnl.len() {
                    current_pnl += self.targets.short_pnl[global_idx];
                }
                
                curve[global_idx] = current_pnl;
            }
        }
        
        curve
    }
}
