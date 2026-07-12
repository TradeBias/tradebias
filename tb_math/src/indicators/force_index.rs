/// Force Index
pub fn force_index(close: &[f64], volume: &[f64], period: usize) -> Vec<f64> {
    let mut fi1 = vec![0.0; close.len()];
    if close.is_empty() { return fi1; }
    
    for i in 1..close.len() {
        fi1[i] = (close[i] - close[i - 1]) * volume[i];
    }
    super::ema::ema(&fi1, period)
}
