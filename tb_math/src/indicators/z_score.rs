/// Z-Score (Standardized Price)
pub fn z_score(data: &[f64], period: usize) -> Vec<f64> {
    let sma = super::sma::sma(data, period);
    let std_dev = super::standard_deviation::standard_deviation(data, period);
    let mut out = vec![f64::NAN; data.len()];
    
    for i in 0..data.len() {
        if !std_dev[i].is_nan() && std_dev[i] != 0.0 {
            out[i] = (data[i] - sma[i]) / std_dev[i];
        } else if !sma[i].is_nan() {
            out[i] = 0.0;
        }
    }
    out
}
