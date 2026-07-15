fn nan_safe_sma(data: &[f64], start_idx: usize, period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < start_idx + period { return out; }
    
    let mut sum = 0.0;
    for i in 0..period {
        sum += data[start_idx + i];
    }
    out[start_idx + period - 1] = sum / period as f64;
    
    for i in (start_idx + period)..data.len() {
        sum += data[i] - data[i - period];
        out[i] = sum / period as f64;
    }
    out
}

/// Know Sure Thing (KST)
pub fn kst(close: &[f64], r1: usize, r2: usize, r3: usize, r4: usize, s1: usize, s2: usize, s3: usize, s4: usize) -> (Vec<f64>, Vec<f64>) {
    let mut out = vec![f64::NAN; close.len()];
    let roc1 = super::roc::roc(close, r1);
    let roc2 = super::roc::roc(close, r2);
    let roc3 = super::roc::roc(close, r3);
    let roc4 = super::roc::roc(close, r4);
    
    let rcma1 = nan_safe_sma(&roc1, r1, s1);
    let rcma2 = nan_safe_sma(&roc2, r2, s2);
    let rcma3 = nan_safe_sma(&roc3, r3, s3);
    let rcma4 = nan_safe_sma(&roc4, r4, s4);
    
    for i in 0..close.len() {
        if !rcma1[i].is_nan() && !rcma2[i].is_nan() && !rcma3[i].is_nan() && !rcma4[i].is_nan() {
            out[i] = rcma1[i] * 1.0 + rcma2[i] * 2.0 + rcma3[i] * 3.0 + rcma4[i] * 4.0;
        }
    }
    
    let start_idx = (r1 + s1).max(r2 + s2).max(r3 + s3).max(r4 + s4) - 1;
    let signal = nan_safe_sma(&out, start_idx, 9);
    
    (out, signal)
}
