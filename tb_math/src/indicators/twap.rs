/// Time Weighted Average Price (TWAP)
pub fn twap(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let mut tp = vec![0.0; close.len()];
    for i in 0..close.len() {
        tp[i] = (high[i] + low[i] + close[i]) / 3.0;
    }
    super::sma::sma(&tp, period)
}
