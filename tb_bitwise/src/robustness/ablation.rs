use crate::engine::BitwiseEngine;
use tb_core::ast::TradeDirection;

/// Runs feature ablation on the given strategy.
/// Returns a list of equity curves, each representing the strategy with one condition removed.
pub fn run_ablations(engine: &BitwiseEngine, condition_indexes: &[usize], direction: &TradeDirection) -> Vec<Vec<f64>> {
    let mut curves = Vec::new();
    
    // For each condition, evaluate the strategy *without* that condition
    for i in 0..condition_indexes.len() {
        let mut ablated_indexes = condition_indexes.to_vec();
        ablated_indexes.remove(i);
        
        let curve = match direction {
            TradeDirection::Long => engine.evaluate_with_curve_long(&ablated_indexes),
            TradeDirection::Short => engine.evaluate_with_curve_short(&ablated_indexes),
            TradeDirection::LongAndShort => engine.evaluate_with_curve_symmetric(&ablated_indexes),
        };
        curves.push(curve);
    }
    
    curves
}
