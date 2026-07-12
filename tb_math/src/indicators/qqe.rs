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

/// Quantitative Qualitative Estimation (QQE)
pub fn qqe(close: &[f64], period: usize, sf: usize, factor: f64) -> (Vec<f64>, Vec<f64>) {
    let mut fast = vec![f64::NAN; close.len()];
    let mut slow = vec![f64::NAN; close.len()];
    if close.len() < period + sf { return (fast, slow); }
    
    let rsi = super::rsi::rsi(close, period);
    fast = nan_safe_ema(&rsi, period - 1, sf);
    
    let mut tr = vec![f64::NAN; close.len()];
    let start_tr = period - 1 + sf;
    for i in start_tr..close.len() {
        tr[i] = (fast[i] - fast[i - 1]).abs();
    }
    
    let wilders_p = period * 2 - 1;
    let smooth_tr = nan_safe_ema(&tr, start_tr, wilders_p);
    let darvas = nan_safe_ema(&smooth_tr, start_tr + wilders_p - 1, wilders_p);
    
    let start_slow = start_tr + wilders_p * 2 - 2;
    if close.len() <= start_slow { return (fast, slow); }
    
    slow[start_slow] = fast[start_slow];
    
    for i in (start_slow + 1)..close.len() {
        let prev_slow = slow[i - 1];
        let curr_fast = fast[i];
        let prev_fast = fast[i - 1];
        let cur_darvas = darvas[i] * factor;
        
        let mut new_slow = prev_slow;
        
        if prev_fast < prev_slow && curr_fast > prev_slow {
            new_slow = curr_fast - cur_darvas;
        } else if prev_fast > prev_slow && curr_fast < prev_slow {
            new_slow = curr_fast + cur_darvas;
        } else if curr_fast < prev_slow {
            new_slow = prev_slow.min(curr_fast + cur_darvas);
        } else if curr_fast > prev_slow {
            new_slow = prev_slow.max(curr_fast - cur_darvas);
        }
        
        slow[i] = new_slow;
    }
    
    (fast, slow)
}
