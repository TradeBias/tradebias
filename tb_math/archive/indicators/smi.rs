fn double_ema_smi(data: &[f64], p1: usize, p2: usize) -> Vec<f64> {
    let mut ema1 = vec![f64::NAN; data.len()];
    if data.len() < p1 { return ema1; }
    
    let mut first_valid = 0;
    while first_valid < data.len() && data[first_valid].is_nan() {
        first_valid += 1;
    }
    if data.len() - first_valid < p1 { return ema1; }
    
    let mut sum = 0.0;
    for i in 0..p1 {
        sum += data[first_valid + i];
    }
    let mut prev = sum / p1 as f64;
    ema1[first_valid + p1 - 1] = prev;
    let m1 = 2.0 / (p1 as f64 + 1.0);
    for i in (first_valid + p1)..data.len() {
        prev = (data[i] - prev) * m1 + prev;
        ema1[i] = prev;
    }
    
    let mut ema2 = vec![f64::NAN; data.len()];
    let fv2 = first_valid + p1 - 1;
    if data.len() - fv2 < p2 { return ema2; }
    
    let mut sum2 = 0.0;
    for i in 0..p2 {
        sum2 += ema1[fv2 + i];
    }
    let mut prev2 = sum2 / p2 as f64;
    ema2[fv2 + p2 - 1] = prev2;
    let m2 = 2.0 / (p2 as f64 + 1.0);
    for i in (fv2 + p2)..data.len() {
        prev2 = (ema1[i] - prev2) * m2 + prev2;
        ema2[i] = prev2;
    }
    ema2
}

/// Stochastic Momentum Index (SMI)
pub fn smi(high: &[f64], low: &[f64], close: &[f64], period: usize, ema1_p: usize, ema2_p: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period { return out; }
    
    let mut rel_m = vec![f64::NAN; close.len()];
    let mut diff = vec![f64::NAN; close.len()];
    
    for i in (period - 1)..close.len() {
        let mut hh = high[i + 1 - period];
        let mut ll = low[i + 1 - period];
        for j in 0..period {
            if high[i - j] > hh { hh = high[i - j]; }
            if low[i - j] < ll { ll = low[i - j]; }
        }
        let center = (hh + ll) / 2.0;
        rel_m[i] = close[i] - center;
        diff[i] = hh - ll;
    }
    
    let num = double_ema_smi(&rel_m, ema1_p, ema2_p);
    let den = double_ema_smi(&diff, ema1_p, ema2_p);
    
    for i in 0..close.len() {
        if !num[i].is_nan() && !den[i].is_nan() {
            if den[i] == 0.0 {
                out[i] = 0.0;
            } else {
                out[i] = 100.0 * (num[i] / (0.5 * den[i]));
            }
        }
    }
    out
}

