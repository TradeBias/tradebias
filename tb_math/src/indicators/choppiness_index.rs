/// Choppiness Index (CHOP)
pub fn choppiness_index(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() <= period || period < 2 { return out; }
    
    let mut tr = vec![0.0; close.len()];
    for i in 1..close.len() {
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        tr[i] = hl.max(hc).max(lc);
    }
    
    let log_n = (period as f64).log10();
    
    for i in period..close.len() {
        let mut sum_tr = 0.0;
        let mut hh = high[i + 1 - period];
        let mut ll = low[i + 1 - period];
        
        for j in 0..period {
            let idx = i - j;
            sum_tr += tr[idx];
            if high[idx] > hh { hh = high[idx]; }
            if low[idx] < ll { ll = low[idx]; }
        }
        
        let range = hh - ll;
        if range == 0.0 {
            out[i] = 100.0;
        } else {
            let ratio = sum_tr / range;
            out[i] = 100.0 * (ratio.log10() / log_n);
        }
    }
    out
}

