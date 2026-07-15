/// Hurst Exponent (Simplified R/S analysis)
pub fn hurst_exponent(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period < 4 { return out; }
    
    for i in (period - 1)..data.len() {
        let mut mean = 0.0;
        for j in 0..period {
            mean += data[i + 1 + j - period];
        }
        mean /= period as f64;
        
        let mut sum_sq = 0.0;
        let mut rolling_sum = 0.0;
        
        let mut max_sum = f64::MIN;
        let mut min_sum = f64::MAX;
        
        for j in 0..period {
            let val = data[i + 1 + j - period] - mean;
            rolling_sum += val;
            sum_sq += val * val;
            
            if rolling_sum > max_sum { max_sum = rolling_sum; }
            if rolling_sum < min_sum { min_sum = rolling_sum; }
        }
        
        let r = max_sum - min_sum;
        let s = (sum_sq / (period as f64)).sqrt();
        
        if s != 0.0 {
            out[i] = (r / s).ln() / (period as f64).ln();
        } else {
            out[i] = 0.5;
        }
    }
    out
}

