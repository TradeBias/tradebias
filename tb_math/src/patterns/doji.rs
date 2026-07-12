pub fn doji(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; open.len()];
    for i in 0..open.len() {
        if open[i].is_nan() || high[i].is_nan() || low[i].is_nan() || close[i].is_nan() {
            continue;
        }
        
        let body_size = (close[i] - open[i]).abs();
        let total_range = high[i] - low[i];
        
        if total_range == 0.0 {
            out[i] = 0.0; // Flat candle
            continue;
        }
        
        // Body is less than 10% of total range
        if body_size <= total_range * 0.10 {
            out[i] = 1.0;
        } else {
            out[i] = 0.0;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doji() {
        let open = vec![10.0];
        let high = vec![12.0];
        let low = vec![8.0];
        let close = vec![10.05]; // small body
        let res = doji(&open, &high, &low, &close);
        assert_eq!(res[0], 1.0);
    }
}
