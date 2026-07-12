fn nan_safe_ema(data: &[f64], start_idx: usize, period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < start_idx + period { return out; }
    
    let mut sum = 0.0;
    for i in 0..period {
        sum += data[start_idx + i];
    }
    let mut prev = sum / period as f64;
    out[start_idx + period - 1] = prev;
    
    let mult = 2.0 / (period as f64 + 1.0);
    for i in (start_idx + period)..data.len() {
        prev = (data[i] - prev) * mult + prev;
        out[i] = prev;
    }
    out
}

/// Bressert Double Smoothed Stochastic (DSS)
pub fn dss_bressert(high: &[f64], low: &[f64], close: &[f64], period: usize, ema_period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 || ema_period == 0 { return out; }
    
    let mut stoch1 = vec![f64::NAN; close.len()];
    for i in (period - 1)..close.len() {
        let mut hh = high[i + 1 - period];
        let mut ll = low[i + 1 - period];
        for j in 0..period {
            if high[i - j] > hh { hh = high[i - j]; }
            if low[i - j] < ll { ll = low[i - j]; }
        }
        if hh == ll { stoch1[i] = 0.0; } 
        else { stoch1[i] = 100.0 * (close[i] - ll) / (hh - ll); }
    }
    
    let smooth1 = nan_safe_ema(&stoch1, period - 1, ema_period);
    
    let mut stoch2 = vec![f64::NAN; close.len()];
    let start2 = period - 1 + ema_period - 1;
    if close.len() < start2 + period { return out; }
    
    for i in (start2 + period - 1)..close.len() {
        let mut hh = smooth1[i + 1 - period];
        let mut ll = smooth1[i + 1 - period];
        for j in 0..period {
            let v = smooth1[i - j];
            if v > hh { hh = v; }
            if v < ll { ll = v; }
        }
        if hh == ll { stoch2[i] = 0.0; }
        else { stoch2[i] = 100.0 * (smooth1[i] - ll) / (hh - ll); }
    }
    
    nan_safe_ema(&stoch2, start2 + period - 1, ema_period)
}

