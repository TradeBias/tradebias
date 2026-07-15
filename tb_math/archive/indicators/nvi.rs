/// Negative Volume Index (NVI)
pub fn nvi(close: &[f64], volume: &[f64]) -> Vec<f64> {
    let mut out = vec![1000.0; close.len()];
    if close.is_empty() { return out; }
    
    for i in 1..close.len() {
        if volume[i] < volume[i - 1] {
            let prev_c = close[i - 1];
            if prev_c != 0.0 {
                let roc = (close[i] - prev_c) / prev_c;
                out[i] = out[i - 1] + (roc * out[i - 1]);
            } else {
                out[i] = out[i - 1];
            }
        } else {
            out[i] = out[i - 1];
        }
    }
    out
}
