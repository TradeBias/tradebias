/// Commodity Channel Index (CCI)
pub fn cci(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    
    if close.len() < period || period == 0 { return out; }
    
    let mut tp = vec![f64::NAN; close.len()];
    for i in 0..close.len() {
        tp[i] = (high[i] + low[i] + close[i]) / 3.0;
    }
    
    let tp_sma = super::sma::sma(&tp, period);
    
    for i in (period - 1)..close.len() {
        let sma = tp_sma[i];
        let mut mean_dev = 0.0;
        for j in 0..period {
            mean_dev += (tp[i - j] - sma).abs();
        }
        mean_dev /= period as f64;
        
        if mean_dev == 0.0 {
            out[i] = 0.0;
        } else {
            out[i] = (tp[i] - sma) / (0.015 * mean_dev);
        }
    }
    out
}
