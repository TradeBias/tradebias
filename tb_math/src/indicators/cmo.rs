/// Chande Momentum Oscillator (CMO)
pub fn cmo(close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() <= period || period == 0 { return out; }
    
    let mut up = vec![0.0; close.len()];
    let mut down = vec![0.0; close.len()];
    
    for i in 1..close.len() {
        let diff = close[i] - close[i - 1];
        if diff > 0.0 {
            up[i] = diff;
        } else if diff < 0.0 {
            down[i] = diff.abs();
        }
    }
    
    for i in period..close.len() {
        let mut sum_up = 0.0;
        let mut sum_down = 0.0;
        for j in 0..period {
            sum_up += up[i - j];
            sum_down += down[i - j];
        }
        let total = sum_up + sum_down;
        if total == 0.0 {
            out[i] = 0.0;
        } else {
            out[i] = 100.0 * ((sum_up - sum_down) / total);
        }
    }
    
    out
}
