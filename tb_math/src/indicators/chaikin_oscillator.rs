/// Chaikin Oscillator
pub fn chaikin_oscillator(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], fast: usize, slow: usize) -> Vec<f64> {
    let ad = super::ad_line::ad_line(high, low, close, volume);
    let ema_fast = super::ema::ema(&ad, fast);
    let ema_slow = super::ema::ema(&ad, slow);
    
    let mut out = vec![f64::NAN; close.len()];
    for i in 0..close.len() {
        if !ema_fast[i].is_nan() && !ema_slow[i].is_nan() {
            out[i] = ema_fast[i] - ema_slow[i];
        }
    }
    out
}
