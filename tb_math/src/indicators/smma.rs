/// Smoothed Moving Average (SMMA)
pub fn smma(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period == 0 { return out; }
    
    let mut sum = 0.0;
    for i in 0..period { sum += data[i]; }
    let mut prev_smma = sum / period as f64;
    out[period - 1] = prev_smma;
    
    for i in period..data.len() {
        let smma = (prev_smma * (period as f64 - 1.0) + data[i]) / period as f64;
        out[i] = smma;
        prev_smma = smma;
    }
    out
}
