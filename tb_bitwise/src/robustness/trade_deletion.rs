use crate::engine::BitwiseEngine;
use tb_core::ast::{Expr, TradeDirection};
use rand::Rng;

/// Randomly deletes a percentage of trades to test for statistical significance.
/// If removing 10% of trades destroys the equity curve, the strategy is overfit to a few lucky trades.
pub fn generate_deletion_curve(
    engine: &BitwiseEngine,
    conditions: &[Expr],
    direction: &TradeDirection,
    drop_percentage: f64,
) -> Vec<f64> {
    let total_bars = engine.targets.long_pnl.len();
    let mut curve = vec![0.0; total_bars];
    
    if conditions.is_empty() {
        return curve;
    }

    let strategy_bitset = engine.merge_conditions(conditions);
    let mut current_pnl = 0.0;
    let mut rng = rand::thread_rng();

    for i in 0..strategy_bitset.len() {
        for bit_idx in 0..64 {
            let global_idx = (i * 64) + bit_idx;
            if global_idx >= total_bars {
                break;
            }

            if (strategy_bitset[i] & (1 << bit_idx)) != 0 {
                // Determine if we should drop this trade
                if rng.gen_bool(drop_percentage) {
                    // Dropped! Don't add PnL
                    curve[global_idx] = current_pnl;
                    continue;
                }

                match direction {
                    TradeDirection::Long => {
                        current_pnl += engine.targets.long_pnl[global_idx];
                    }
                    TradeDirection::Short => {
                        if global_idx < engine.targets.short_pnl.len() {
                            current_pnl += engine.targets.short_pnl[global_idx];
                        }
                    }
                    TradeDirection::LongAndShort => {
                        current_pnl += engine.targets.long_pnl[global_idx];
                    }
                }
            } else if matches!(direction, TradeDirection::LongAndShort) {
                if global_idx < engine.targets.short_pnl.len() {
                    if !rng.gen_bool(drop_percentage) {
                        current_pnl += engine.targets.short_pnl[global_idx];
                    }
                }
            }
            
            curve[global_idx] = current_pnl;
        }
    }

    curve
}
