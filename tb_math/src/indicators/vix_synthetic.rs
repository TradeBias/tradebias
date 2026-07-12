/// VIX Synthetic (Williams Vix Fix)
pub fn vix_synthetic(close: &[f64], low: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 { return out; }
    
    for i in (period - 1)..close.len() {
        let mut highest_close = close[i + 1 - period];
        for j in 1..period {
            if close[i + 1 + j - period] > highest_close {
                highest_close = close[i + 1 + j - period];
            }
        }
        
        if highest_close > 0.0 {
            out[i] = ((highest_close - low[i]) / highest_close) * 100.0;
        }
    }
    out
}

