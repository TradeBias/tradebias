use crate::engine::BitwiseEngine;
use tb_core::ast::{Expr, TradeDirection};

/// Runs feature ablation on the given strategy.
/// Returns a list of equity curves, each representing the strategy with one condition removed.
pub fn run_ablations(engine: &BitwiseEngine, conditions: &[Expr], direction: &TradeDirection) -> Vec<Vec<f64>> {
    let mut curves = Vec::new();
    
    // For each condition, evaluate the strategy *without* that condition
    for i in 0..conditions.len() {
        let mut ablated_conditions = conditions.to_vec();
        ablated_conditions.remove(i);
        
        let curve = match direction {
            TradeDirection::Long => engine.evaluate_with_curve_long(&ablated_conditions),
            TradeDirection::Short => engine.evaluate_with_curve_short(&ablated_conditions),
            TradeDirection::LongAndShort => engine.evaluate_with_curve_symmetric(&ablated_conditions),
        };
        curves.push(curve);
    }
    
    curves
}
