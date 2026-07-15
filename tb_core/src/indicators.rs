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

/// Computes a Weighted Moving Average (WMA) over the given period.
pub fn compute_wma(source: Expr, period: u32) -> Expr {
    let weights: Vec<f64> = (1..=period).map(|v| v as f64).collect();
    source.rolling_mean(RollingOptionsFixedWindow {
        window_size: period as usize,
        min_periods: period as usize,
        weights: Some(weights),
        center: false,
        fn_params: None,
    })
}

/// Computes Wilder's Smoothing (RMA) over the given period.
pub fn compute_rma(source: Expr, period: u32) -> Expr {
    let alpha = 1.0 / (period as f64);
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

pub fn compute_ts_max(source: Expr, period: u32) -> Expr {
    source.rolling_max(RollingOptionsFixedWindow {
        window_size: period as usize,
        min_periods: period as usize,
        weights: None,
        center: false,
        fn_params: None,
    })
}

pub fn compute_ts_min(source: Expr, period: u32) -> Expr {
    source.rolling_min(RollingOptionsFixedWindow {
        window_size: period as usize,
        min_periods: period as usize,
        weights: None,
        center: false,
        fn_params: None,
    })
}

pub fn compute_ts_sum(source: Expr, period: u32) -> Expr {
    source.rolling_sum(RollingOptionsFixedWindow {
        window_size: period as usize,
        min_periods: period as usize,
        weights: None,
        center: false,
        fn_params: None,
    })
}

pub fn compute_std_dev(source: Expr, period: u32) -> Expr {
    source.rolling_std(RollingOptionsFixedWindow {
        window_size: period as usize,
        min_periods: period as usize,
        weights: None,
        center: false,
        fn_params: None,
    })
}

pub fn compute_true_range(high: Expr, low: Expr, close: Expr) -> Expr {
    let hl = high.clone() - low.clone();
    
    let hc_diff = high - close.clone().shift(lit(1));
    let hc = when(hc_diff.clone().lt(lit(0.0))).then(lit(0.0) - hc_diff.clone()).otherwise(hc_diff);
    
    let lc_diff = low - close.shift(lit(1));
    let lc = when(lc_diff.clone().lt(lit(0.0))).then(lit(0.0) - lc_diff.clone()).otherwise(lc_diff);
    
    // Polars doesn't have a variadic max, so we chain them
    let max1 = when(hl.clone().gt(hc.clone())).then(hl).otherwise(hc);
    when(max1.clone().gt(lc.clone())).then(max1).otherwise(lc)
}
