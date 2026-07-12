/// Stochastic RSI
pub fn stoch_rsi(close: &[f64], rsi_period: usize, stoch_period: usize, k_period: usize, d_period: usize) -> (Vec<f64>, Vec<f64>) {
    let rsi_line = super::rsi::rsi(close, rsi_period);
    
    let mut raw_k = vec![f64::NAN; close.len()];
    if close.len() < rsi_period + stoch_period { return (raw_k.clone(), raw_k); }
    
    for i in (rsi_period + stoch_period - 1)..close.len() {
        let mut highest = rsi_line[i - stoch_period + 1];
        let mut lowest = rsi_line[i - stoch_period + 1];
        
        for j in 0..stoch_period {
            let val = rsi_line[i - j];
            if val > highest { highest = val; }
            if val < lowest { lowest = val; }
        }
        
        if highest == lowest {
            raw_k[i] = 50.0;
        } else {
            raw_k[i] = 100.0 * (rsi_line[i] - lowest) / (highest - lowest);
        }
    }
    
    let k_line = super::sma::sma(&raw_k, k_period);
    let d_line = super::sma::sma(&k_line, d_period);
    
    (k_line, d_line)
}
