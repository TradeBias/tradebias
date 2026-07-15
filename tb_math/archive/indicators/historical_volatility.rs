/// Historical Volatility (HV)
pub fn historical_volatility(close: &[f64], period: usize, annual_factor: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() <= period || period == 0 { return out; }
    
    let mut log_returns = vec![0.0; close.len()];
    for i in 1..close.len() {
        if close[i - 1] > 0.0 && close[i] > 0.0 {
            log_returns[i] = (close[i] / close[i - 1]).ln();
        }
    }
    
    let multiplier = annual_factor.sqrt();
    
    for i in period..close.len() {
        let mut sum = 0.0;
        for j in 0..period {
            sum += log_returns[i - j];
        }
        let mean = sum / period as f64;
        
        let mut variance_sum = 0.0;
        for j in 0..period {
            let diff = log_returns[i - j] - mean;
            variance_sum += diff * diff;
        }
        
        let variance = variance_sum / (period as f64 - 1.0).max(1.0);
        out[i] = variance.sqrt() * multiplier * 100.0;
    }
    
    out
}
