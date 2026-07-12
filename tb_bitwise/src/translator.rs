use tb_core::ast::{Expr, Sketch, TradeDirection};
use crate::precompute::ConditionGrid;
use crate::ga::Genome;

/// Translates a raw Bitwise Genome into a standardized TradeBias JSON Sketch.
pub fn translate_to_sketch(genome: &Genome, grid: &ConditionGrid, direction: TradeDirection) -> Option<Sketch> {
    if genome.conditions.is_empty() {
        return None;
    }

    let mut entry_expr = grid.conditions[genome.conditions[0]].ast.clone();

    // Iterate through remaining conditions, binding them with logical AND
    for i in 1..genome.conditions.len() {
        let rhs_expr = grid.conditions[genome.conditions[i]].ast.clone();
        
        entry_expr = Expr::And {
            lhs: Box::new(entry_expr),
            rhs: Box::new(rhs_expr),
        };
    }

    Some(Sketch {
        name: format!("Bitwise_King_{}T", genome.metrics.total_trades),
        direction,
        entry: entry_expr,
        exit: None, // Exits are structurally enforced via fixed-hold in Bitwise Engine Phase 1
    })
}
