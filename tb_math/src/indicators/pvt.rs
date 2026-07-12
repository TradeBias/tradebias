/// Price Volume Trend (PVT)
pub fn pvt(close: &[f64], volume: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; close.len()];
    if close.is_empty() { return out; }
    
    for i in 1..close.len() {
        let prev = close[i - 1];
        if prev != 0.0 {
            let change = (close[i] - prev) / prev;
            out[i] = out[i - 1] + change * volume[i];
        } else {
            out[i] = out[i - 1];
        }
    }
    out
}
