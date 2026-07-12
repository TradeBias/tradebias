use super::atr::atr;

/// Supertrend
pub fn supertrend(high: &[f64], low: &[f64], close: &[f64], period: usize, multiplier: f64) -> (Vec<f64>, Vec<f64>) {
    let mut st = vec![f64::NAN; close.len()];
    let mut dir = vec![f64::NAN; close.len()]; // 1.0 for up, -1.0 for down
    
    if close.len() < period || period == 0 { return (st, dir); }
    let atr_line = atr(high, low, close, period);
    
    let mut final_upper = vec![f64::NAN; close.len()];
    let mut final_lower = vec![f64::NAN; close.len()];
    let mut trend = 1.0;
    
    for i in period..close.len() {
        let hl2 = (high[i] + low[i]) / 2.0;
        let basic_upper = hl2 + multiplier * atr_line[i];
        let basic_lower = hl2 - multiplier * atr_line[i];
        
        let prev_upper = if final_upper[i - 1].is_nan() { basic_upper } else { final_upper[i - 1] };
        let prev_lower = if final_lower[i - 1].is_nan() { basic_lower } else { final_lower[i - 1] };
        
        let current_upper = if basic_upper < prev_upper || close[i - 1] > prev_upper { basic_upper } else { prev_upper };
        let current_lower = if basic_lower > prev_lower || close[i - 1] < prev_lower { basic_lower } else { prev_lower };
        
        final_upper[i] = current_upper;
        final_lower[i] = current_lower;
        
        if close[i] > current_upper { trend = 1.0; }
        else if close[i] < current_lower { trend = -1.0; }
        
        dir[i] = trend;
        st[i] = if trend == 1.0 { current_lower } else { current_upper };
    }
    
    (st, dir)
}
