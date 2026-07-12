/// Klinger Oscillator
pub fn klinger_oscillator(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], fast: usize, slow: usize) -> Vec<f64> {
    let mut vf = vec![0.0; close.len()];
    let mut trend = 1.0;
    
    for i in 1..close.len() {
        let tp = (high[i] + low[i] + close[i]) / 3.0;
        let prev_tp = (high[i - 1] + low[i - 1] + close[i - 1]) / 3.0;
        if tp > prev_tp {
            trend = 1.0;
        } else if tp < prev_tp {
            trend = -1.0;
        }
        vf[i] = volume[i] * trend * 100.0;
    }
    
    let ema_fast = super::ema::ema(&vf, fast);
    let ema_slow = super::ema::ema(&vf, slow);
    
    let mut out = vec![f64::NAN; close.len()];
    for i in 0..close.len() {
        if !ema_fast[i].is_nan() && !ema_slow[i].is_nan() {
            out[i] = ema_fast[i] - ema_slow[i];
        }
    }
    out
}
