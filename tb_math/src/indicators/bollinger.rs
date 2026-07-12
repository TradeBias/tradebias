use super::sma::sma;

/// Bollinger Bands (Returns Upper, Middle, Lower)
pub fn bollinger_bands(data: &[f64], period: usize, std_devs: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut upper = vec![f64::NAN; data.len()];
    let mut lower = vec![f64::NAN; data.len()];
    let middle = sma(data, period); // Middle band is just SMA
    
    if data.len() < period || period == 0 {
        return (upper, middle, lower);
    }
    
    for i in (period - 1)..data.len() {
        let slice = &data[(i + 1 - period)..=i];
        let mean = middle[i];
        
        let mut variance_sum = 0.0;
        for val in slice {
            variance_sum += (val - mean).powi(2);
        }
        let variance = variance_sum / period as f64;
        let std_dev = variance.sqrt();
        
        upper[i] = mean + (std_devs * std_dev);
        lower[i] = mean - (std_devs * std_dev);
    }
    
    (upper, middle, lower)
}
