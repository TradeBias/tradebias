use super::ema::ema;

/// Moving Average Convergence Divergence (MACD)
/// Returns a tuple of 3 vectors: (MACD Line, Signal Line, Histogram)
pub fn macd(data: &[f64], fast_period: usize, slow_period: usize, signal_period: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let fast_ema = ema(data, fast_period);
    let slow_ema = ema(data, slow_period);
    
    let mut macd_line = vec![f64::NAN; data.len()];
    for i in 0..data.len() {
        if !fast_ema[i].is_nan() && !slow_ema[i].is_nan() {
            macd_line[i] = fast_ema[i] - slow_ema[i];
        }
    }
    
    // To calculate the signal line (EMA of MACD line), we need to extract the valid MACD values
    // Find the first non-NaN index
    let mut first_valid = 0;
    while first_valid < macd_line.len() && macd_line[first_valid].is_nan() {
        first_valid += 1;
    }
    
    let mut signal_line = vec![f64::NAN; data.len()];
    if first_valid < macd_line.len() {
        let valid_macd = &macd_line[first_valid..];
        let valid_signal = ema(valid_macd, signal_period);
        for i in 0..valid_signal.len() {
            signal_line[first_valid + i] = valid_signal[i];
        }
    }
    
    let mut histogram = vec![f64::NAN; data.len()];
    for i in 0..data.len() {
        if !macd_line[i].is_nan() && !signal_line[i].is_nan() {
            histogram[i] = macd_line[i] - signal_line[i];
        }
    }
    
    (macd_line, signal_line, histogram)
}
