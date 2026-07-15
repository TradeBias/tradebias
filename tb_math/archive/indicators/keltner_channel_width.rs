/// Keltner Channel Width
pub fn keltner_channel_width(high: &[f64], low: &[f64], close: &[f64], period: usize, multiplier: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 { return out; }
    
    let (upper, mid, lower) = super::keltner::keltner_channels(high, low, close, period, multiplier);
    
    for i in 0..close.len() {
        if !mid[i].is_nan() && mid[i] != 0.0 {
            out[i] = (upper[i] - lower[i]) / mid[i] * 100.0;
        }
    }
    out
}
