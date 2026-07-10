use polars::prelude::*;

/// Computes a Simple Moving Average (SMA) over the given period.
pub fn compute_sma(source: Expr, period: u32) -> Expr {
    source.rolling_mean(RollingOptionsFixedWindow {
        window_size: period as usize,
        min_periods: period as usize,
        weights: None,
        center: false,
        fn_params: None,
    })
}

/// Computes an Exponential Moving Average (EMA) over the given period.
pub fn compute_ema(source: Expr, period: u32) -> Expr {
    // Standard EMA formula alpha = 2 / (period + 1)
    let alpha = 2.0 / (period as f64 + 1.0);
    
    source.ewm_mean(EWMOptions {
        alpha,
        adjust: false,
        bias: false,
        min_periods: period as usize,
        ignore_nulls: true,
    })
}

/// Computes the Relative Strength Index (RSI) using Wilder's Smoothing
pub fn compute_rsi(source: Expr, period: u32) -> Expr {
    let delta = source.clone() - source.clone().shift(lit(1));
    
    let gain = when(delta.clone().gt(lit(0.0))).then(delta.clone()).otherwise(lit(0.0));
    let loss = when(delta.clone().lt(lit(0.0))).then(lit(0.0) - delta.clone()).otherwise(lit(0.0));

    // Wilder's Smoothing is essentially an EMA with alpha = 1 / period
    let alpha = 1.0 / (period as f64);
    
    let avg_gain = gain.ewm_mean(EWMOptions {
        alpha, adjust: false, bias: false, min_periods: period as usize, ignore_nulls: true
    });
    
    let avg_loss = loss.ewm_mean(EWMOptions {
        alpha, adjust: false, bias: false, min_periods: period as usize, ignore_nulls: true
    });
    
    let rs = avg_gain / avg_loss;
    lit(100.0) - (lit(100.0) / (lit(1.0) + rs))
}

/// Computes the Moving Average Convergence Divergence (MACD) line.
pub fn compute_macd(source: Expr, fast: u32, slow: u32) -> Expr {
    let fast_ema = compute_ema(source.clone(), fast);
    let slow_ema = compute_ema(source, slow);
    fast_ema - slow_ema
}
