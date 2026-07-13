use crate::engine::BitwiseEngine;
use tb_core::ast::TradeDirection;

/// Evaluates a strategy exactly once, subtracting a fixed percentage from
/// the profit of every single trade to simulate deterministic slippage.
pub fn generate_slippage_curve(
    engine: &BitwiseEngine,
    condition_indexes: &[usize],
    direction: &TradeDirection,
    slippage_penalty_pct: f64,
) -> Vec<f64> {
    let total_bars = engine.targets.long_pnl.len();
    let mut curve = vec![0.0; total_bars];
    
    if condition_indexes.is_empty() {
        return curve;
    }

    let num_blocks = engine.grid.conditions[0].bits.len();
    let mut current_pnl = 0.0;

    for i in 0..num_blocks {
        let mut strategy_block = u64::MAX;
        for &idx in condition_indexes {
            strategy_block &= engine.grid.conditions[idx].bits[i];
        }

        for bit_idx in 0..64 {
            let global_idx = (i * 64) + bit_idx;
            if global_idx >= total_bars {
                break;
            }

            if (strategy_block & (1 << bit_idx)) != 0 {
                // Determine absolute value to compute the penalty properly?
                // Actually, if we apply a flat % friction drag:
                // Slippage is ALWAYS bad. It eats gross profit and worsens losses.
                // We could do: pnl - (abs(pnl) * slippage_pct) OR pnl - (trade_value * slippage_pct).
                // Since we only have raw PnL, we'll assume slippage decreases the PnL by a flat amount or %.
                // We'll subtract slippage_penalty_pct from the *raw* PnL directly. Wait, if PnL is in %, 1.0 = 100%. 
                // So slippage_penalty_pct = 0.05 means 5% of asset price. This is standard.
                // BUT in our engine long_pnl is already absolute return. 
                // So we subtract the slippage penalty amount directly.
                
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
