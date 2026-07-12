use polars::prelude::*;

/// Builds the Polars expressions needed to calculate backtest metrics for a given strategy signal.
/// Returns a tuple of (fitness_expr, risk_expr, freq_expr) where fitness is Total Return, risk is Volatility, and freq is trade count.
pub fn build_metric_expressions(signal_expr: Expr, strat_id: &str) -> (Expr, Expr, Expr) {
    // We assume the DataFrame already has a `forward_return` column pre-calculated for speed.
    // Strategy Return = Signal (0 or 1) * Forward Return
    // If signal is boolean, cast to Float64
    let strat_ret = signal_expr.clone().cast(DataType::Float64) * col("forward_return");
    
    // FAST APPROXIMATION FOR PHASE 1:
    // Instead of sequentially calculating cum_prod and cum_max (which locks threads and destroys SIMD),
    // we use O(N) parallel reductions.
    
    // Fitness Proxy: Sum of returns
    let total_return = (strat_ret.clone().sum() * lit(100.0)).alias(&format!("ret_{}", strat_id));
    
    // Risk Proxy: Volatility (Standard Deviation)
    // We keep the alias "max_dd_" so the engine parser doesn't break, but this is now Volatility.
    let risk = (strat_ret.std(1) * lit(100.0)).alias(&format!("max_dd_{}", strat_id));
    
    // Frequency Proxy: Count of true signals
    let freq = signal_expr.cast(DataType::Float64).sum().alias(&format!("freq_{}", strat_id));
    
    // Return all three metrics in a tuple, but wait, the engine expects a tuple of 2 (fit_expr, risk_expr)
    // Actually we can return a tuple of 3, but that breaks engine.rs. We'll update engine.rs too.
    (total_return, risk, freq)
}

/// Helper to precalculate base columns like `forward_return` on the raw DataFrame
pub fn prepare_data_for_evaluation(mut data: LazyFrame) -> LazyFrame {
    // 1. Normalize all column names to lowercase to prevent User CSV case issues
    if let Ok(schema) = data.schema() {
        let mut old_names = Vec::new();
        let mut new_names = Vec::new();
        for name in schema.iter_names() {
            let lower = name.to_lowercase();
            if name.as_str() != lower {
                old_names.push(name.to_string());
                new_names.push(lower);
            }
        }
        if !old_names.is_empty() {
            data = data.rename(old_names, new_names);
        }
    }

    // 2. Calculate Virtual Binary Target (5-bar horizon hit rate)
    // We project forward 5 bars to see if the trade was a win or loss.
    // By clamping it to +1.0 (Win) and -1.0 (Loss), we completely eliminate
    // the "God Strategy" bug where a single outlier trade dominates the sum.
    let n_bars = 5;
    let future_close = col("close").shift(lit(-n_bars));
    let fwd_return = (future_close - col("close")) / col("close");
    
    let target = when(fwd_return.clone().gt(lit(0.0)))
        .then(lit(1.0))
        .when(fwd_return.lt(lit(0.0)))
        .then(lit(-1.0))
        .otherwise(lit(0.0))
        .alias("forward_return"); // keep same column name for engine.rs compat

    data.with_column(target)
}
