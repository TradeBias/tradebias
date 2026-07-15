use crate::engine::BitwiseEngine;
use tb_core::ast::{Expr, TradeDirection};

/// Evaluates a strategy exactly once, subtracting a fixed percentage from
/// the profit of every single trade to simulate deterministic slippage.
pub fn generate_slippage_curve(
    engine: &BitwiseEngine,
    conditions: &[Expr],
    direction: &TradeDirection,
    slippage_penalty_pct: f64,
) -> Vec<f64> {
    let total_bars = engine.targets.long_pnl.len();
    let mut curve = vec![0.0; total_bars];
    
    if conditions.is_empty() {
        return curve;
    }

    let strategy_bitset = engine.merge_conditions(conditions);
    let mut current_pnl = 0.0;

    for i in 0..strategy_bitset.len() {
        for bit_idx in 0..64 {
            let global_idx = (i * 64) + bit_idx;
            if global_idx >= total_bars {
                break;
            }

            if (strategy_bitset[i] & (1 << bit_idx)) != 0 {
                match direction {
                    TradeDirection::Long => {
                        current_pnl += engine.targets.long_pnl[global_idx] - slippage_penalty_pct;
                    }
                    TradeDirection::Short => {
                        if global_idx < engine.targets.short_pnl.len() {
                            current_pnl += engine.targets.short_pnl[global_idx] - slippage_penalty_pct;
                        }
                    }
                    TradeDirection::LongAndShort => {
                        current_pnl += engine.targets.long_pnl[global_idx] - slippage_penalty_pct;
                    }
                }
            } else if matches!(direction, TradeDirection::LongAndShort) {
                if global_idx < engine.targets.short_pnl.len() {
                    current_pnl += engine.targets.short_pnl[global_idx] - slippage_penalty_pct;
                }
            }
            
            curve[global_idx] = current_pnl;
        }
    }

    curve
}
