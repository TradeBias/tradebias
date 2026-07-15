/// Average True Range (ATR)
pub fn atr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; high.len()];
    if high.len() < period || period == 0 {
        return out;
    }
    
    let mut tr = vec![0.0; high.len()];
    tr[0] = high[0] - low[0];
    for i in 1..high.len() {
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        tr[i] = hl.max(hc).max(lc);
    }
    
    // First ATR is simple average of TR
    let mut sum = 0.0;
    for i in 0..period {
        sum += tr[i];
    }
    let mut prev_atr = sum / period as f64;
    out[period - 1] = prev_atr;
    
    for i in period..high.len() {
        let current_atr = (prev_atr * (period as f64 - 1.0) + tr[i]) / period as f64;
        out[i] = current_atr;
        prev_atr = current_atr;
    }
    
    out
}
