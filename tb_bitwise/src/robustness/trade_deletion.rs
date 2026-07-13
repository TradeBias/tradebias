use crate::engine::BitwiseEngine;
use tb_core::ast::TradeDirection;
use std::collections::HashSet;

/// Evaluates a strategy, identifies the top N most profitable trades,
/// removes them from the ledger, and returns the resulting equity curve.
pub fn generate_deletion_curve(
    engine: &BitwiseEngine,
    condition_indexes: &[usize],
    direction: &TradeDirection,
    top_n_to_delete: usize,
) -> Vec<f64> {
    if condition_indexes.is_empty() {
        return vec![0.0; engine.targets.long_pnl.len()];
    }

    let num_blocks = engine.grid.conditions[0].bits.len();
    let total_bars = engine.targets.long_pnl.len();
    let mut trades: Vec<(usize, f64)> = Vec::new();

    // 1. Collect all trades
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

            match direction {
                TradeDirection::Long => {
                    if (strategy_block & (1 << bit_idx)) != 0 {
                        trades.push((global_idx, engine.targets.long_pnl[global_idx]));
                    }
                }
                TradeDirection::Short => {
                    if (strategy_block & (1 << bit_idx)) != 0 {
                        if global_idx < engine.targets.short_pnl.len() {
                            trades.push((global_idx, engine.targets.short_pnl[global_idx]));
                        }
                    }
                }
                TradeDirection::LongAndShort => {
                    if (strategy_block & (1 << bit_idx)) != 0 {
                        trades.push((global_idx, engine.targets.long_pnl[global_idx]));
                    } else if global_idx < engine.targets.short_pnl.len() {
                        trades.push((global_idx, engine.targets.short_pnl[global_idx]));
                    }
                }
            }
        }
    }

    // 2. Identify top N trades to delete
    // Sort descending by PnL
    trades.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    let to_delete: HashSet<usize> = trades.into_iter()
        .take(top_n_to_delete)
        .map(|(idx, _)| idx)
        .collect();

    // 3. Rebuild curve skipping deleted trades
    let mut curve = vec![0.0; total_bars];
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

            if !to_delete.contains(&global_idx) {
                match direction {
                    TradeDirection::Long => {
                        if (strategy_block & (1 << bit_idx)) != 0 {
                            current_pnl += engine.targets.long_pnl[global_idx];
                        }
                    }
                    TradeDirection::Short => {
                        if (strategy_block & (1 << bit_idx)) != 0 {
                            if global_idx < engine.targets.short_pnl.len() {
                                current_pnl += engine.targets.short_pnl[global_idx];
                            }
                        }
                    }
                    TradeDirection::LongAndShort => {
                        if (strategy_block & (1 << bit_idx)) != 0 {
                            current_pnl += engine.targets.long_pnl[global_idx];
                        } else if global_idx < engine.targets.short_pnl.len() {
                            current_pnl += engine.targets.short_pnl[global_idx];
                        }
                    }
                }
            }
            
            curve[global_idx] = current_pnl;
        }
    }

    curve
}
