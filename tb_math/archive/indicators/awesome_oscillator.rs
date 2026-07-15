/// Awesome Oscillator (AO)
pub fn awesome_oscillator(high: &[f64], low: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; high.len()];
    if high.len() < 34 { return out; }
    
    let mut median = vec![0.0; high.len()];
    for i in 0..high.len() {
        median[i] = (high[i] + low[i]) / 2.0;
    }
    
    let sma5 = super::sma::sma(&median, 5);
    let sma34 = super::sma::sma(&median, 34);
    
    for i in 0..high.len() {
        if !sma5[i].is_nan() && !sma34[i].is_nan() {
            out[i] = sma5[i] - sma34[i];
        }
    }
    out
}
