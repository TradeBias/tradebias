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
            return StrategyMetrics::new().finalize();
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

        metrics.finalize()
    }

    /// Evaluates a short strategy.
    pub fn evaluate_short(&self, condition_indexes: &[usize]) -> StrategyResult {
        if condition_indexes.is_empty() {
            return StrategyMetrics::new().finalize();
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

        metrics.finalize()
    }
}
