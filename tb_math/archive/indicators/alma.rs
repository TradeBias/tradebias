/// Arnaud Legoux Moving Average (ALMA)
pub fn alma(data: &[f64], period: usize, offset: f64, sigma: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period == 0 { return out; }

    let m = offset * (period as f64 - 1.0);
    let s = (period as f64) / sigma;
    
    let mut weights = vec![0.0; period];
    let mut weight_sum = 0.0;
    for i in 0..period {
        let w = (-( (i as f64 - m).powi(2) ) / (2.0 * s * s)).exp();
        weights[i] = w;
        weight_sum += w;
    }
    
    for i in (period - 1)..data.len() {
        let mut sum = 0.0;
        for j in 0..period {
            sum += data[i + 1 + j - period] * weights[j];
        }
        out[i] = sum / weight_sum;
    }
    out
}

