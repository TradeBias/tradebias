/// VORTEX Indicator (VI)
pub fn vortex(high: &[f64], low: &[f64], close: &[f64], period: usize) -> (Vec<f64>, Vec<f64>) {
    let mut vi_plus = vec![f64::NAN; close.len()];
    let mut vi_minus = vec![f64::NAN; close.len()];
    if close.len() <= period || period == 0 { return (vi_plus, vi_minus); }
    
    let mut vm_plus = vec![0.0; close.len()];
    let mut vm_minus = vec![0.0; close.len()];
    let mut tr = vec![0.0; close.len()];
    
    for i in 1..close.len() {
        vm_plus[i] = (high[i] - low[i - 1]).abs();
        vm_minus[i] = (low[i] - high[i - 1]).abs();
        
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        tr[i] = hl.max(hc).max(lc);
    }
    
    for i in period..close.len() {
        let mut sum_plus = 0.0;
        let mut sum_minus = 0.0;
        let mut sum_tr = 0.0;
        for j in 0..period {
            sum_plus += vm_plus[i - j];
            sum_minus += vm_minus[i - j];
            sum_tr += tr[i - j];
        }
        if sum_tr == 0.0 {
            vi_plus[i] = 1.0;
            vi_minus[i] = 1.0;
        } else {
            vi_plus[i] = sum_plus / sum_tr;
            vi_minus[i] = sum_minus / sum_tr;
        }
    }
    
    (vi_plus, vi_minus)
}
