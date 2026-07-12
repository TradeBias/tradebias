/// Kaufman Adaptive Moving Average (KAMA)
pub fn kama(data: &[f64], period: usize, fast_ema: usize, slow_ema: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period == 0 { return out; }
    
    let fast_sc = 2.0 / (fast_ema as f64 + 1.0);
    let slow_sc = 2.0 / (slow_ema as f64 + 1.0);
    
    let mut prev_kama = data[period - 1]; // Seed with price
    out[period - 1] = prev_kama;
    
    for i in period..data.len() {
        let change = (data[i] - data[i - period]).abs();
        let mut volatility = 0.0;
        for j in 0..period {
            volatility += (data[i - j] - data[i - j - 1]).abs();
        }
        
        let er = if volatility == 0.0 { 0.0 } else { change / volatility };
        let sc = (er * (fast_sc - slow_sc) + slow_sc).powi(2);
        let kama_val = prev_kama + sc * (data[i] - prev_kama);
        out[i] = kama_val;
        prev_kama = kama_val;
    }
    out
}
