fn nan_safe_ema(data: &[f64], start_idx: usize, period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < start_idx + period { return out; }
    
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

/// Percentage Price Oscillator (PPO)
pub fn ppo(close: &[f64], fast: usize, slow: usize, signal: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut ppo_line = vec![f64::NAN; close.len()];
    let mut signal_line = vec![f64::NAN; close.len()];
    let mut hist = vec![f64::NAN; close.len()];
    if close.len() < slow { return (ppo_line, signal_line, hist); }
    
    let ema_fast = super::ema::ema(close, fast);
    let ema_slow = super::ema::ema(close, slow);
    
    for i in (slow - 1)..close.len() {
        if ema_slow[i] == 0.0 {
            ppo_line[i] = 0.0;
        } else {
            ppo_line[i] = (ema_fast[i] - ema_slow[i]) / ema_slow[i] * 100.0;
        }
    }
    
    signal_line = nan_safe_ema(&ppo_line, slow - 1, signal);
    
    for i in 0..close.len() {
        if !ppo_line[i].is_nan() && !signal_line[i].is_nan() {
            hist[i] = ppo_line[i] - signal_line[i];
        }
    }
    
    (ppo_line, signal_line, hist)
}
