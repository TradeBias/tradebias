/// Simple Moving Average (SMA)
pub fn sma(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period == 0 {
        return out;
    }
    
    let mut sum = 0.0;
    for i in 0..period {
        sum += data[i];
    }
    out[period - 1] = sum / period as f64;
    
    for i in period..data.len() {
        sum += data[i] - data[i - period];
        out[i] = sum / period as f64;
    }
    out
}
