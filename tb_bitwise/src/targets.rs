use tracing::info;
use crate::data::RawData;
use tb_core::stops::{StopCalculation, TakeProfit};
use rayon::prelude::*;

pub struct TargetOutcomes {
    pub long_pnl: Vec<f64>,
    pub short_pnl: Vec<f64>,
    pub long_winning_mask: Vec<u64>,
    pub short_winning_mask: Vec<u64>,
}

impl TargetOutcomes {
    /// Generates outcomes based on a fixed N-bar holding period.
    /// This is the simplest, fastest, and most robust way to calculate PnL.
    pub fn generate_fixed_hold(data: &RawData, hold_bars: usize, slippage_penalty: f64) -> Self {
        info!("Pre-computing Target Outcomes (Fixed Hold: {} bars)...", hold_bars);
        let start_time = std::time::Instant::now();
        
        let mut long_pnl = vec![0.0; data.close.len()];
        let mut short_pnl = vec![0.0; data.close.len()];
        
        let num_blocks = (data.close.len() + 63) / 64;
        let mut long_winning_mask = vec![0u64; num_blocks];
        let mut short_winning_mask = vec![0u64; num_blocks];

        for i in 0..data.close.len() {
            if i + hold_bars >= data.close.len() {
                continue;
            }

            let entry_price = data.open[i];
            let exit_price = data.close[i + hold_bars];

            let l_pnl = (exit_price - entry_price) - slippage_penalty;
            let s_pnl = (entry_price - exit_price) - slippage_penalty;

            long_pnl[i] = l_pnl;
            short_pnl[i] = s_pnl;

            if l_pnl > 0.0 {
                long_winning_mask[i / 64] |= 1 << (i % 64);
            }
            if s_pnl > 0.0 {
                short_winning_mask[i / 64] |= 1 << (i % 64);
            }
        }

        info!("Target PnL arrays and Winning Trade Bitmasks generated in {:?}", start_time.elapsed());
        
        Self {
            long_pnl,
            short_pnl,
            long_winning_mask,
            short_winning_mask,
        }
    }

    fn get_stop_distance(calc: &tb_core::stops::StopCalculation, entry_price: f64, idx: usize, atr_data: Option<&[f64]>) -> f64 {
        match calc {
            tb_core::stops::StopCalculation::Atr { multiplier } => {
                atr_data.map(|atr| atr[idx] * multiplier).unwrap_or(0.0)
            },
            tb_core::stops::StopCalculation::Fixed { points } => *points,
            tb_core::stops::StopCalculation::Percentage { pct } => entry_price * (pct / 100.0),
        }
    }

    fn get_tp_distance(tp: &tb_core::stops::TakeProfit, stop_dist: f64, entry_price: f64, idx: usize, atr_data: Option<&[f64]>) -> Option<f64> {
        match tp {
            tb_core::stops::TakeProfit::RiskReward { multiplier } => Some(stop_dist * multiplier),
            tb_core::stops::TakeProfit::Atr { multiplier } => {
                Some(atr_data.map(|atr| atr[idx] * multiplier).unwrap_or(0.0))
            },
            tb_core::stops::TakeProfit::Fixed { points } => Some(*points),
            tb_core::stops::TakeProfit::Percentage { pct } => Some(entry_price * (pct / 100.0)),
            tb_core::stops::TakeProfit::None => None,
        }
    }

    pub fn generate_advanced_stop(
        data: &RawData, 
        stop_type: &tb_core::stops::StopType, 
        take_profit: &tb_core::stops::TakeProfit,
        atr_data: Option<&[f64]>,
        slippage_penalty: f64
    ) -> Self {
        match stop_type {
            tb_core::stops::StopType::FixedBarHold { bars } => {
                Self::generate_fixed_hold(data, *bars, slippage_penalty)
            },
            tb_core::stops::StopType::StandardStop { calc } => {
                Self::generate_static_stop(data, calc, take_profit, atr_data, slippage_penalty)
            },
            tb_core::stops::StopType::TrailingStop { calc } => {
                Self::generate_trailing_stop(data, calc, take_profit, atr_data, slippage_penalty)
            }
        }
    }

    fn generate_static_stop(data: &RawData, calc: &tb_core::stops::StopCalculation, tp: &tb_core::stops::TakeProfit, atr_data: Option<&[f64]>, slippage_penalty: f64) -> Self {
        info!("Pre-computing Target Outcomes (Static Stop)...");
        let start_time = std::time::Instant::now();
        
        let mut long_pnl = vec![0.0; data.close.len()];
        let mut short_pnl = vec![0.0; data.close.len()];
        let num_blocks = (data.close.len() + 63) / 64;
        let mut long_winning_mask = vec![0u64; num_blocks];
        let mut short_winning_mask = vec![0u64; num_blocks];

        let results: Vec<_> = (0..data.close.len()).into_par_iter().map(|i| {
            let entry_price = data.open[i];
            let stop_dist = Self::get_stop_distance(calc, entry_price, i, atr_data);
            let tp_dist = Self::get_tp_distance(tp, stop_dist, entry_price, i, atr_data);
            
            let long_stop = entry_price - stop_dist;
            let short_stop = entry_price + stop_dist;
            
            let long_tp = tp_dist.map(|d| entry_price + d);
            let short_tp = tp_dist.map(|d| entry_price - d);
            
            let mut long_exit_price = data.close.last().copied().unwrap_or(entry_price);
            let mut short_exit_price = data.close.last().copied().unwrap_or(entry_price);
            
            let mut long_closed = false;
            let mut short_closed = false;

            for j in (i + 1)..data.close.len() {
                // Long checks
                if !long_closed {
                    // Pessimistic execution: assume stop is hit first if both hit
                    if data.low[j] <= long_stop {
                        long_exit_price = long_stop;
                        long_closed = true;
                    } else if let Some(tp_level) = long_tp {
                        if data.high[j] >= tp_level {
                            long_exit_price = tp_level;
                            long_closed = true;
                        }
                    }
                }
                
                // Short checks
                if !short_closed {
                    if data.high[j] >= short_stop {
                        short_exit_price = short_stop;
                        short_closed = true;
                    } else if let Some(tp_level) = short_tp {
                        if data.low[j] <= tp_level {
                            short_exit_price = tp_level;
                            short_closed = true;
                        }
                    }
                }
                
                if long_closed && short_closed {
                    break;
                }
            }

            let l_pnl = (long_exit_price - entry_price) - slippage_penalty;
            let s_pnl = (entry_price - short_exit_price) - slippage_penalty;

            (l_pnl, s_pnl)
        }).collect();

        for (i, &(l_pnl, s_pnl)) in results.iter().enumerate() {
            long_pnl[i] = l_pnl;
            short_pnl[i] = s_pnl;
            if l_pnl > 0.0 { long_winning_mask[i / 64] |= 1 << (i % 64); }
            if s_pnl > 0.0 { short_winning_mask[i / 64] |= 1 << (i % 64); }
        }

        info!("Target PnL arrays generated in {:?}", start_time.elapsed());
        Self { long_pnl, short_pnl, long_winning_mask, short_winning_mask }
    }

    fn generate_trailing_stop(data: &RawData, calc: &tb_core::stops::StopCalculation, tp: &tb_core::stops::TakeProfit, atr_data: Option<&[f64]>, slippage_penalty: f64) -> Self {
        info!("Pre-computing Target Outcomes (Trailing Stop)...");
        let start_time = std::time::Instant::now();
        
        let mut long_pnl = vec![0.0; data.close.len()];
        let mut short_pnl = vec![0.0; data.close.len()];
        let num_blocks = (data.close.len() + 63) / 64;
        let mut long_winning_mask = vec![0u64; num_blocks];
        let mut short_winning_mask = vec![0u64; num_blocks];

        let results: Vec<_> = (0..data.close.len()).into_par_iter().map(|i| {
            let entry_price = data.open[i];
            
            // For Trailing stop, TP distance is evaluated at entry candle
            let initial_stop_dist = Self::get_stop_distance(calc, entry_price, i, atr_data);
            let tp_dist = Self::get_tp_distance(tp, initial_stop_dist, entry_price, i, atr_data);
            let long_tp = tp_dist.map(|d| entry_price + d);
            let short_tp = tp_dist.map(|d| entry_price - d);
            
            let mut long_highest = entry_price;
            let mut short_lowest = entry_price;
            
            let mut long_exit_price = data.close.last().copied().unwrap_or(entry_price);
            let mut short_exit_price = data.close.last().copied().unwrap_or(entry_price);
            
            let mut long_closed = false;
            let mut short_closed = false;

            for j in (i + 1)..data.close.len() {
                let dist = Self::get_stop_distance(calc, entry_price, j, atr_data);
                
                if !long_closed {
                    long_highest = long_highest.max(data.high[j]);
                    let trail_stop = long_highest - dist;
                    if data.low[j] <= trail_stop {
                        long_exit_price = trail_stop;
                        long_closed = true;
                    } else if let Some(tp_level) = long_tp {
                        if data.high[j] >= tp_level {
                            long_exit_price = tp_level;
                            long_closed = true;
                        }
                    }
                }
                
                if !short_closed {
                    short_lowest = short_lowest.min(data.low[j]);
                    let trail_stop = short_lowest + dist;
                    if data.high[j] >= trail_stop {
                        short_exit_price = trail_stop;
                        short_closed = true;
                    } else if let Some(tp_level) = short_tp {
                        if data.low[j] <= tp_level {
                            short_exit_price = tp_level;
                            short_closed = true;
                        }
                    }
                }
                
                if long_closed && short_closed {
                    break;
                }
            }

            let l_pnl = (long_exit_price - entry_price) - slippage_penalty;
            let s_pnl = (entry_price - short_exit_price) - slippage_penalty;

            (l_pnl, s_pnl)
        }).collect();

        for (i, &(l_pnl, s_pnl)) in results.iter().enumerate() {
            long_pnl[i] = l_pnl;
            short_pnl[i] = s_pnl;
            if l_pnl > 0.0 { long_winning_mask[i / 64] |= 1 << (i % 64); }
            if s_pnl > 0.0 { short_winning_mask[i / 64] |= 1 << (i % 64); }
        }

        info!("Target PnL arrays generated in {:?}", start_time.elapsed());
        Self { long_pnl, short_pnl, long_winning_mask, short_winning_mask }
    }
}
