/// Median, Typical, and Weighted Price Transforms

pub fn median_price(high: &[f64], low: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; high.len()];
    for i in 0..high.len() {
        out[i] = (high[i] + low[i]) / 2.0;
    }
    out
}

pub fn typical_price(high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; high.len()];
    for i in 0..high.len() {
        out[i] = (high[i] + low[i] + close[i]) / 3.0;
    }
    out
}

pub fn weighted_close(high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; high.len()];
    for i in 0..high.len() {
        out[i] = (high[i] + low[i] + close[i] * 2.0) / 4.0;
    }
    out
}
