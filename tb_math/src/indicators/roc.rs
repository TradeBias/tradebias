/// Rate of Change (ROC)
pub fn roc(close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() <= period || period == 0 { return out; }
    
    for i in period..close.len() {
        let prev = close[i - period];
        if prev == 0.0 {
            out[i] = 0.0;
        } else {
            out[i] = 100.0 * (close[i] - prev) / prev;
        }
    }
    out
}
