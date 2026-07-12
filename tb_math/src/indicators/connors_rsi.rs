/// Connors RSI (CRSI)
pub fn connors_rsi(close: &[f64], rsi_period: usize, streak_rsi_period: usize, percent_rank_period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < rsi_period.max(streak_rsi_period).max(percent_rank_period) { return out; }

    let rsi = super::rsi::rsi(close, rsi_period);
    
    let mut streak = vec![0.0; close.len()];
    let mut current_streak = 0.0;
    
    let mut roc1 = vec![0.0; close.len()];
    
    for i in 1..close.len() {
        let diff = close[i] - close[i - 1];
        if diff > 0.0 {
            if current_streak < 0.0 { current_streak = 1.0; }
            else { current_streak += 1.0; }
        } else if diff < 0.0 {
            if current_streak > 0.0 { current_streak = -1.0; }
            else { current_streak -= 1.0; }
        } else {
            current_streak = 0.0;
        }
        streak[i] = current_streak;
        
        let prev = close[i - 1];
        if prev != 0.0 {
            roc1[i] = (diff / prev) * 100.0;
        }
    }
    
    let streak_rsi = super::rsi::rsi(&streak, streak_rsi_period);
    
    for i in (percent_rank_period - 1)..close.len() {
        let current_roc = roc1[i];
        let mut less_count = 0;
        for j in 0..percent_rank_period {
            if roc1[i - j] < current_roc {
                less_count += 1;
            }
        }
        let percent_rank = (less_count as f64 / percent_rank_period as f64) * 100.0;
        
        if !rsi[i].is_nan() && !streak_rsi[i].is_nan() {
            out[i] = (rsi[i] + streak_rsi[i] + percent_rank) / 3.0;
        }
    }
    out
}
