/// Moving Volume Weighted Average Price (MVWAP)
pub fn mvwap(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 { return out; }
    
    let mut tp_v = vec![0.0; close.len()];
    for i in 0..close.len() {
        let tp = (high[i] + low[i] + close[i]) / 3.0;
        tp_v[i] = tp * volume[i];
    }
    
    for i in (period - 1)..close.len() {
        let mut sum_tp_v = 0.0;
        let mut sum_v = 0.0;
        for j in 0..period {
            sum_tp_v += tp_v[i - j];
            sum_v += volume[i - j];
        }
        if sum_v > 0.0 {
            out[i] = sum_tp_v / sum_v;
        }
    }
    out
}
