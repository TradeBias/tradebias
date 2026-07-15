fn nan_safe_ema(data: &[f64], start_idx: usize, period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < start_idx + period || period == 0 { return out; }
    
    let mut sum = 0.0;
    for i in 0..period { sum += data[start_idx + i]; }
    let mut prev = sum / period as f64;
    out[start_idx + period - 1] = prev;
    
    let mult = 2.0 / (period as f64 + 1.0);
    for i in (start_idx + period)..data.len() {
        prev = (data[i] - prev) * mult + prev;
        out[i] = prev;
    }
    out
}

/// Schaff Trend Cycle (STC)
pub fn stc(close: &[f64], fast: usize, slow: usize, period: usize, ema_period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < slow { return out; }
    
    let ema_fast = super::ema::ema(close, fast);
    let ema_slow = super::ema::ema(close, slow);
    let mut macd = vec![f64::NAN; close.len()];
    for i in (slow - 1)..close.len() {
        macd[i] = ema_fast[i] - ema_slow[i];
    }
    
    let start_stoch1 = slow - 1 + period - 1;
    if close.len() <= start_stoch1 { return out; }
    
    let mut stoch1 = vec![f64::NAN; close.len()];
    for i in start_stoch1..close.len() {
        let mut hh = macd[i + 1 - period];
        let mut ll = macd[i + 1 - period];
        for j in 0..period {
            let val = macd[i - j];
            if val > hh { hh = val; }
            if val < ll { ll = val; }
        }
        if hh == ll { stoch1[i] = 0.0; }
        else { stoch1[i] = 100.0 * (macd[i] - ll) / (hh - ll); }
    }
    
    let smooth1 = nan_safe_ema(&stoch1, start_stoch1, ema_period);
    
    let start_stoch2 = start_stoch1 + ema_period - 1 + period - 1;
    if close.len() <= start_stoch2 { return out; }
    
    let mut stoch2 = vec![f64::NAN; close.len()];
    for i in start_stoch2..close.len() {
        let mut hh = smooth1[i + 1 - period];
        let mut ll = smooth1[i + 1 - period];
        for j in 0..period {
            let val = smooth1[i - j];
            if val > hh { hh = val; }
            if val < ll { ll = val; }
        }
        if hh == ll { stoch2[i] = 0.0; }
        else { stoch2[i] = 100.0 * (smooth1[i] - ll) / (hh - ll); }
    }
    
    nan_safe_ema(&stoch2, start_stoch2, ema_period)
}

