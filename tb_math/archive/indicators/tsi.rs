fn double_ema(data: &[f64], period1: usize, period2: usize) -> Vec<f64> {
    let ema1 = super::ema::ema(data, period1);
    let mut ema2 = vec![f64::NAN; data.len()];
    if data.len() < period1 + period2 - 1 { return ema2; }
    
    let mult = 2.0 / (period2 as f64 + 1.0);
    let start_idx = period1 - 1;
    let mut sum = 0.0;
    for i in 0..period2 {
        sum += ema1[start_idx + i];
    }
    
    let mut prev = sum / period2 as f64;
    let ema2_start = start_idx + period2 - 1;
    ema2[ema2_start] = prev;
    
    for i in (ema2_start + 1)..data.len() {
        let current = (ema1[i] - prev) * mult + prev;
        ema2[i] = current;
        prev = current;
    }
    ema2
}

/// True Strength Index (TSI)
pub fn tsi(close: &[f64], long_period: usize, short_period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() <= long_period + short_period || long_period == 0 || short_period == 0 { return out; }
    
    let mut m = vec![0.0; close.len()];
    let mut abs_m = vec![0.0; close.len()];
    
    for i in 1..close.len() {
        m[i] = close[i] - close[i - 1];
        abs_m[i] = m[i].abs();
    }
    
    let double_smoothed_pc = double_ema(&m, long_period, short_period);
    let double_smoothed_abs_pc = double_ema(&abs_m, long_period, short_period);
    
    for i in 0..close.len() {
        if !double_smoothed_pc[i].is_nan() && !double_smoothed_abs_pc[i].is_nan() {
            if double_smoothed_abs_pc[i] == 0.0 {
                out[i] = 0.0;
            } else {
                out[i] = 100.0 * (double_smoothed_pc[i] / double_smoothed_abs_pc[i]);
            }
        }
    }
    out
}
