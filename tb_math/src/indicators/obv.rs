/// OBV (On-Balance Volume)
pub fn obv(close: &[f64], volume: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; close.len()];
    if close.is_empty() { return out; }
    out[0] = volume[0];
    for i in 1..close.len() {
        if close[i] > close[i - 1] {
            out[i] = out[i - 1] + volume[i];
        } else if close[i] < close[i - 1] {
            out[i] = out[i - 1] - volume[i];
        } else {
            out[i] = out[i - 1];
        }
    }
    out
}
