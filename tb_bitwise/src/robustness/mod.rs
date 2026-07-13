use crate::engine::BitwiseEngine;
use tb_core::ast::TradeDirection;

pub mod ablation;
pub mod monte_carlo;
pub mod noise;
pub mod trade_deletion;

/// A unified report for a single strategy's robustness tests.
#[derive(Debug, Clone)]
pub struct RobustnessReport {
    pub baseline_curve: Vec<f64>,
    pub ablation_curves: Vec<Vec<f64>>,
    pub slippage_curve: Vec<f64>,
    pub deletion_curve: Vec<f64>,
    pub monte_carlo_curves: Vec<Vec<f64>>,
}

pub fn generate_report(
    engine: &BitwiseEngine, 
    condition_indexes: &[usize], 
    direction: &TradeDirection,
    noise_variance_pct: f64,
    top_n_to_delete: usize,
) -> RobustnessReport {
    let baseline_curve = match direction {
        TradeDirection::Long => engine.evaluate_with_curve_long(condition_indexes),
        TradeDirection::Short => engine.evaluate_with_curve_short(condition_indexes),
        TradeDirection::LongAndShort => engine.evaluate_with_curve_symmetric(condition_indexes),
    };
    
    let ablation_curves = ablation::run_ablations(engine, condition_indexes, direction);
    let monte_carlo_curves = monte_carlo::run_block_bootstrap(&baseline_curve, 100);
    
    let slippage_curve = noise::generate_slippage_curve(
        engine,
        condition_indexes,
        direction,
        noise_variance_pct,
    );
    let deletion_curve = trade_deletion::generate_deletion_curve(engine, condition_indexes, direction, top_n_to_delete);

    RobustnessReport {
        baseline_curve,
        ablation_curves,
        slippage_curve,
        deletion_curve,
        monte_carlo_curves,
    }
}
