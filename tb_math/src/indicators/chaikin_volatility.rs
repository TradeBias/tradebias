/// Chaikin Volatility (CV)
pub fn chaikin_volatility(high: &[f64], low: &[f64], ema_period: usize, roc_period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; high.len()];
    if high.len() < ema_period + roc_period || ema_period == 0 || roc_period == 0 { return out; }
    
    let mut hl = vec![0.0; high.len()];
    for i in 0..high.len() {
        hl[i] = high[i] - low[i];
    }
    
    let ema_hl = super::ema::ema(&hl, ema_period);
    
    for i in roc_period..high.len() {
        let prev = ema_hl[i - roc_period];
        if prev != 0.0 && !prev.is_nan() && !ema_hl[i].is_nan() {
            out[i] = ((ema_hl[i] - prev) / prev) * 100.0;
        }
    }
    
    out
}
