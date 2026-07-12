use super::wma::wma;

/// Hull Moving Average (HMA)
pub fn hma(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period < 2 { return out; }
    
    let half_period = period / 2;
    let sqrt_period = (period as f64).sqrt().round() as usize;
    
    let wma_half = wma(data, half_period);
    let wma_full = wma(data, period);
    
    let mut diff = vec![f64::NAN; data.len()];
    for i in 0..data.len() {
        if !wma_half[i].is_nan() && !wma_full[i].is_nan() {
            diff[i] = 2.0 * wma_half[i] - wma_full[i];
        }
    }
    
    let mut valid_start = 0;
    while valid_start < diff.len() && diff[valid_start].is_nan() { valid_start += 1; }
    
    if valid_start < diff.len() {
        let valid_diff = &diff[valid_start..];
        let hma_valid = wma(valid_diff, sqrt_period);
        for i in 0..hma_valid.len() {
            out[valid_start + i] = hma_valid[i];
        }
    }
    out
}
