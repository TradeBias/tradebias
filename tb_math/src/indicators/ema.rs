/// Exponential Moving Average (EMA)
pub fn ema(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period == 0 {
        return out;
    }
    
    let multiplier = 2.0 / (period as f64 + 1.0);
    
    // Seed the EMA with a simple average of the first 'period' values
    let mut sum = 0.0;
    for i in 0..period {
        sum += data[i];
    }
    let mut prev_ema = sum / period as f64;
    out[period - 1] = prev_ema;
    
    for i in period..data.len() {
        let current_ema = (data[i] - prev_ema) * multiplier + prev_ema;
        out[i] = current_ema;
        prev_ema = current_ema;
    }
    out
}
