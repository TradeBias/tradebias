/// Bollinger %B
pub fn bollinger_percent_b(close: &[f64], period: usize, std_dev: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 { return out; }
    
    let (upper, lower, _mid) = super::bollinger::bollinger_bands(close, period, std_dev);
    
    for i in 0..close.len() {
        if !upper[i].is_nan() && !lower[i].is_nan() {
            let diff = upper[i] - lower[i];
            if diff != 0.0 {
                out[i] = (close[i] - lower[i]) / diff * 100.0;
            } else {
                out[i] = 0.0;
            }
        }
    }
    out
}
