/// Triple Exponential Average Oscillator (TRIX)
pub fn trix(close: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period * 3 || period == 0 { return out; }
    
    let ema1 = super::ema::ema(close, period);
    
    let mut ema2 = vec![f64::NAN; close.len()];
    let mut sum = 0.0;
    for i in 0..period {
        sum += ema1[period - 1 + i];
    }
    let mut prev = sum / period as f64;
    ema2[period * 2 - 2] = prev;
    let m = 2.0 / (period as f64 + 1.0);
    for i in (period * 2 - 1)..close.len() {
        prev = (ema1[i] - prev) * m + prev;
        ema2[i] = prev;
    }
    
    let mut ema3 = vec![f64::NAN; close.len()];
    sum = 0.0;
    for i in 0..period {
        sum += ema2[period * 2 - 2 + i];
    }
    prev = sum / period as f64;
    let start_ema3 = period * 3 - 3;
    ema3[start_ema3] = prev;
    for i in (start_ema3 + 1)..close.len() {
        prev = (ema2[i] - prev) * m + prev;
        ema3[i] = prev;
    }
    
    for i in (start_ema3 + 1)..close.len() {
        let prev_val = ema3[i - 1];
        if prev_val != 0.0 {
            out[i] = ((ema3[i] - prev_val) / prev_val) * 100.0;
        } else {
            out[i] = 0.0;
        }
    }
    out
}
