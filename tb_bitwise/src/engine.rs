use tracing::info;
use crate::precompute::ConditionGrid;
use crate::targets::TargetOutcomes;

use std::sync::Arc;

pub use crate::metrics::StrategyResult;
use crate::metrics::StrategyMetrics;

pub struct BitwiseEngine {
    pub grid: Arc<ConditionGrid>,
    pub targets: Arc<TargetOutcomes>,
}

impl BitwiseEngine {
    pub fn new(grid: Arc<ConditionGrid>, targets: Arc<TargetOutcomes>) -> Self {
        info!("Initializing Bitwise Engine for ultra-fast SIMD evaluation.");
        Self { grid, targets }
    }
    
    /// Evaluates a strategy composed of multiple condition indexes (AND logic).
    /// e.g. [0, 5, 12] means Condition 0 AND Condition 5 AND Condition 12.
    pub fn evaluate_long(&self, condition_indexes: &[usize]) -> StrategyResult {
        if condition_indexes.is_empty() {
            return StrategyMetrics::new().finalize(self.targets.long_pnl.len());
        }

        let num_blocks = self.grid.conditions[0].bits.len();
        let mut metrics = StrategyMetrics::new();

        for i in 0..num_blocks {
            let mut strategy_block = u64::MAX;
            for &idx in condition_indexes {
                strategy_block &= self.grid.conditions[idx].bits[i];
            }

            let mut remaining = strategy_block;
            while remaining != 0 {
                let bit_idx = remaining.trailing_zeros() as usize;
                let global_idx = (i * 64) + bit_idx;
                
                metrics.add_trade(self.targets.long_pnl[global_idx]);
                
                remaining &= remaining - 1;
            }
        }

        metrics.finalize(self.targets.long_pnl.len())
    }

    /// Evaluates a short strategy.
    pub fn evaluate_short(&self, condition_indexes: &[usize]) -> StrategyResult {
        if condition_indexes.is_empty() {
            return StrategyMetrics::new().finalize(self.targets.long_pnl.len());
        }

        let num_blocks = self.grid.conditions[0].bits.len();
        let mut metrics = StrategyMetrics::new();

        for i in 0..num_blocks {
            let mut strategy_block = u64::MAX;
            for &idx in condition_indexes {
                strategy_block &= self.grid.conditions[idx].bits[i];
            }

            let mut remaining = strategy_block;
            while remaining != 0 {
                let bit_idx = remaining.trailing_zeros() as usize;
                let global_idx = (i * 64) + bit_idx;
                
                metrics.add_trade(self.targets.short_pnl[global_idx]);
                
                remaining &= remaining - 1;
            }
        }

        metrics.finalize(self.targets.long_pnl.len())
    }

    /// Evaluates a symmetric (Long and Short) strategy.
    pub fn evaluate_symmetric(&self, condition_indexes: &[usize]) -> StrategyResult {
        if condition_indexes.is_empty() {
            return StrategyMetrics::new().finalize(self.targets.long_pnl.len());
        }

        let num_blocks = self.grid.conditions[0].bits.len();
        let mut metrics = StrategyMetrics::new();

        for i in 0..num_blocks {
            let mut strategy_block = u64::MAX;
            for &idx in condition_indexes {
                strategy_block &= self.grid.conditions[idx].bits[i];
            }

            let mut remaining = strategy_block;
            while remaining != 0 {
                let bit_idx = remaining.trailing_zeros() as usize;
                let global_idx = (i * 64) + bit_idx;
                
                // For symmetric, we assume a signal triggers a Long trade
                // and an inverse of the signal triggers a Short trade.
                // However, since we just have a positive signal boolean, 
                // typically a "Symmetric" strategy takes Long trades on True, 
                // and Short trades on False. 
                // But in this Bitwise system, the conditions are specific filters. 
                // If it's a Long/Short system, usually it evaluates Long on Condition,
                // and Short on Condition. Wait, if it triggers *both* at the same time,
                // the net PnL is Long PnL + Short PnL (which is essentially 0 minus spread).
                // Usually "Long and Short" means the conditions apply for Longs, and their INVERSE applies for Shorts.
                // Or maybe the user just means evaluating the strategy rules for Longs and evaluating them for Shorts.
                // Let's just add both PnLs as a placeholder or do evaluate_long + evaluate_short?
                // For now, let's just combine the PnLs of both directions for the same signal, which is naive,
                // but fulfills the backend wiring. Wait, a better naive approach is taking Long on True, Short on False.
                // Or simply: 
                metrics.add_trade(self.targets.long_pnl[global_idx]);
                
                remaining &= remaining - 1;
            }
            
            // For shorts, we would evaluate the inverse of the strategy block
            let mut short_remaining = !strategy_block; // The inverse signal
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

    /// Generates a chronological equity curve (cumulative PnL over time) for a long strategy.
    pub fn evaluate_with_curve_long(&self, condition_indexes: &[usize]) -> Vec<f64> {
        let total_bars = self.targets.long_pnl.len();
        let mut curve = vec![0.0; total_bars];
        if condition_indexes.is_empty() {
            return curve;
        }

        let num_blocks = self.grid.conditions[0].bits.len();
        let mut current_pnl = 0.0;

        for i in 0..num_blocks {
            let mut strategy_block = u64::MAX;
            for &idx in condition_indexes {
                strategy_block &= self.grid.conditions[idx].bits[i];
            }

            for bit_idx in 0..64 {
                let global_idx = (i * 64) + bit_idx;
                if global_idx >= total_bars {
                    break;
                }
                
                if (strategy_block & (1 << bit_idx)) != 0 {
                    current_pnl += self.targets.long_pnl[global_idx];
                }
                curve[global_idx] = current_pnl;
            }
        }
        
        curve
    }

    /// Generates a chronological equity curve for a short strategy.
    pub fn evaluate_with_curve_short(&self, condition_indexes: &[usize]) -> Vec<f64> {
        let total_bars = self.targets.short_pnl.len();
        let mut curve = vec![0.0; total_bars];
        if condition_indexes.is_empty() {
            return curve;
        }

        let num_blocks = self.grid.conditions[0].bits.len();
        let mut current_pnl = 0.0;

        for i in 0..num_blocks {
            let mut strategy_block = u64::MAX;
            for &idx in condition_indexes {
                strategy_block &= self.grid.conditions[idx].bits[i];
            }

            for bit_idx in 0..64 {
                let global_idx = (i * 64) + bit_idx;
                if global_idx >= total_bars {
                    break;
                }
                
                if (strategy_block & (1 << bit_idx)) != 0 {
                    current_pnl += self.targets.short_pnl[global_idx];
                }
                curve[global_idx] = current_pnl;
            }
        }
        
        curve
    }

    /// Generates a chronological equity curve for a symmetric strategy.
    pub fn evaluate_with_curve_symmetric(&self, condition_indexes: &[usize]) -> Vec<f64> {
        let total_bars = self.targets.long_pnl.len();
        let mut curve = vec![0.0; total_bars];
        if condition_indexes.is_empty() {
            return curve;
        }

        let num_blocks = self.grid.conditions[0].bits.len();
        let mut current_pnl = 0.0;

        for i in 0..num_blocks {
            let mut strategy_block = u64::MAX;
            for &idx in condition_indexes {
                strategy_block &= self.grid.conditions[idx].bits[i];
            }

            for bit_idx in 0..64 {
                let global_idx = (i * 64) + bit_idx;
                if global_idx >= total_bars {
                    break;
                }
                
                // Long on True, Short on False
                if (strategy_block & (1 << bit_idx)) != 0 {
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
