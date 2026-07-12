/// Stochastic Oscillator
pub fn stochastic(high: &[f64], low: &[f64], close: &[f64], period: usize, k_period: usize, d_period: usize) -> (Vec<f64>, Vec<f64>) {
    let mut raw_k = vec![f64::NAN; close.len()];
    
    if close.len() < period || period == 0 { return (raw_k.clone(), raw_k); }

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
            raw_k[i] = 50.0;
        } else {
            raw_k[i] = 100.0 * (close[i] - lowest) / (highest - lowest);
        }
    }
    
    let k_line = super::sma::sma(&raw_k, k_period);
    let d_line = super::sma::sma(&k_line, d_period);
    
    (k_line, d_line)
}

