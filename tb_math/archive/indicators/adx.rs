use super::atr::atr;
use super::smma::smma;

/// Average Directional Index (ADX)
pub fn adx(high: &[f64], low: &[f64], close: &[f64], period: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut adx_line = vec![f64::NAN; close.len()];
    let mut plus_di = vec![f64::NAN; close.len()];
    let mut minus_di = vec![f64::NAN; close.len()];
    
    if close.len() < period || period <= 1 { return (adx_line, plus_di, minus_di); }

    let mut plus_dm = vec![0.0; close.len()];
    let mut minus_dm = vec![0.0; close.len()];
    
    for i in 1..close.len() {
        let up_move = high[i] - high[i - 1];
        let down_move = low[i - 1] - low[i];
        
        if up_move > down_move && up_move > 0.0 {
            plus_dm[i] = up_move;
        }
        if down_move > up_move && down_move > 0.0 {
            minus_dm[i] = down_move;
        }
    }
    
    let tr = atr(high, low, close, period); 
    
    let smooth_plus_dm = smma(&plus_dm, period);
    let smooth_minus_dm = smma(&minus_dm, period);
    
    let mut dx = vec![f64::NAN; close.len()];
    for i in 0..close.len() {
        if !tr[i].is_nan() && !smooth_plus_dm[i].is_nan() && !smooth_minus_dm[i].is_nan() {
            let tr_val = if tr[i] == 0.0 { 1e-10 } else { tr[i] };
            let di_plus = 100.0 * (smooth_plus_dm[i] / tr_val);
            let di_minus = 100.0 * (smooth_minus_dm[i] / tr_val);
            
            plus_di[i] = di_plus;
            minus_di[i] = di_minus;
            
            let diff = (di_plus - di_minus).abs();
            let sum = di_plus + di_minus;
            if sum == 0.0 {
                dx[i] = 0.0;
            } else {
                dx[i] = 100.0 * (diff / sum);
            }
        }
    }
    
    adx_line = smma(&dx, period);
    
    (adx_line, plus_di, minus_di)
}
