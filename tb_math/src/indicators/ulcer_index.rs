/// Ulcer Index
pub fn ulcer_index(close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 { return out; }
    
    let mut squared_drawdowns = vec![0.0; close.len()];
    
    for i in (period - 1)..close.len() {
        let mut highest = close[i + 1 - period];
        for j in 1..period {
            if close[i + 1 + j - period] > highest {
                highest = close[i + 1 + j - period];
            }
        }
        
        let mut drawdown = 0.0;
        if highest > 0.0 {
            drawdown = 100.0 * (close[i] - highest) / highest;
        }
        squared_drawdowns[i] = drawdown * drawdown;
    }
    
    for i in (period * 2 - 2)..close.len() {
        let mut sum = 0.0;
        for j in 0..period {
            sum += squared_drawdowns[i - j];
        }
        out[i] = (sum / period as f64).sqrt();
    }
    out
}

