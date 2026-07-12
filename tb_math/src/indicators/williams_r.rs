/// Williams %R
pub fn williams_r(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    
    if close.len() < period || period == 0 { return out; }

    for i in (period - 1)..close.len() {
        let mut highest = high[i + 1 - period];
        let mut lowest = low[i + 1 - period];
        
        for j in 0..period {
            let h = high[i - j];
            let l = low[i - j];
            if h > highest { highest = h; }
            if l < lowest { lowest = l; }
        }
        
        if highest == lowest {
            out[i] = -50.0;
        } else {
            out[i] = -100.0 * (highest - close[i]) / (highest - lowest);
        }
    }
    out
}

