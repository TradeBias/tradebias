/// Bollinger Band Width
pub fn bollinger_band_width(close: &[f64], period: usize, std_dev: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 { return out; }
    
    let (upper, lower, mid) = super::bollinger::bollinger_bands(close, period, std_dev);
    
    for i in 0..close.len() {
        if !mid[i].is_nan() && mid[i] != 0.0 {
            out[i] = (upper[i] - lower[i]) / mid[i] * 100.0;
        }
    }
    out
}
