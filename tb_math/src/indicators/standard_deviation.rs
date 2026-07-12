/// Standard Deviation (Rolling)
pub fn standard_deviation(close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 { return out; }
    
    for i in (period - 1)..close.len() {
        let mut sum = 0.0;
        for j in 0..period {
            sum += close[i - j];
        }
        let mean = sum / period as f64;
        
        let mut variance_sum = 0.0;
        for j in 0..period {
            let diff = close[i - j] - mean;
            variance_sum += diff * diff;
        }
        out[i] = (variance_sum / period as f64).sqrt();
    }
    out
}
