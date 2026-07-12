use super::ema::ema;

/// Double Exponential Moving Average (DEMA)
pub fn dema(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period == 0 { return out; }
    
    let ema1 = ema(data, period);
    
    let mut valid_start = 0;
    while valid_start < ema1.len() && ema1[valid_start].is_nan() { valid_start += 1; }
    
    if valid_start < ema1.len() {
        let ema1_valid = &ema1[valid_start..];
        let ema2_valid = ema(ema1_valid, period);
        for i in 0..ema2_valid.len() {
            if !ema2_valid[i].is_nan() {
                out[valid_start + i] = 2.0 * ema1_valid[i] - ema2_valid[i];
            }
        }
    }
    out
}
