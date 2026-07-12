/// Weighted Moving Average (WMA)
pub fn wma(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period == 0 { return out; }
    let weight_sum = (period * (period + 1)) as f64 / 2.0;
    for i in (period - 1)..data.len() {
        let mut sum = 0.0;
        for j in 0..period {
            sum += data[i - j] * (period - j) as f64;
        }
        out[i] = sum / weight_sum;
    }
    out
}
