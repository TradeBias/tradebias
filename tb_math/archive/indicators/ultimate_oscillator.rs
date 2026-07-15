/// Ultimate Oscillator (UO)
pub fn ultimate_oscillator(high: &[f64], low: &[f64], close: &[f64], p1: usize, p2: usize, p3: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    let max_p = p1.max(p2).max(p3);
    if close.len() <= max_p || p1 == 0 || p2 == 0 || p3 == 0 { return out; }
    
    let mut bp = vec![0.0; close.len()];
    let mut tr = vec![0.0; close.len()];
    
    for i in 1..close.len() {
        let prev_close = close[i - 1];
        let true_low = low[i].min(prev_close);
        let true_high = high[i].max(prev_close);
        
        bp[i] = close[i] - true_low;
        tr[i] = true_high - true_low;
    }
    
    for i in max_p..close.len() {
        let sum_bp1: f64 = bp[(i - p1 + 1)..=i].iter().sum();
        let sum_tr1: f64 = tr[(i - p1 + 1)..=i].iter().sum();
        let avg1 = if sum_tr1 == 0.0 { 0.0 } else { sum_bp1 / sum_tr1 };
        
        let sum_bp2: f64 = bp[(i - p2 + 1)..=i].iter().sum();
        let sum_tr2: f64 = tr[(i - p2 + 1)..=i].iter().sum();
        let avg2 = if sum_tr2 == 0.0 { 0.0 } else { sum_bp2 / sum_tr2 };
        
        let sum_bp3: f64 = bp[(i - p3 + 1)..=i].iter().sum();
        let sum_tr3: f64 = tr[(i - p3 + 1)..=i].iter().sum();
        let avg3 = if sum_tr3 == 0.0 { 0.0 } else { sum_bp3 / sum_tr3 };
        
        out[i] = 100.0 * (4.0 * avg1 + 2.0 * avg2 + avg3) / 7.0;
    }
    out
}
