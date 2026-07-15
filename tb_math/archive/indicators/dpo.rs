/// Detrended Price Oscillator (DPO)
pub fn dpo(close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period < 2 { return out; }
    
    let sma = super::sma::sma(close, period);
    let offset = period / 2 + 1;
    
    for i in (period - 1 + offset)..close.len() {
        out[i] = close[i] - sma[i - offset];
    }
    out
}
