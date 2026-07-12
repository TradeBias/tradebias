/// Chaikin Money Flow (CMF)
pub fn chaikin_money_flow(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 { return out; }
    
    let mut mfv = vec![0.0; close.len()];
    for i in 0..close.len() {
        let diff = high[i] - low[i];
        if diff != 0.0 {
            mfv[i] = ((close[i] - low[i]) - (high[i] - close[i])) / diff * volume[i];
        }
    }
    
    for i in (period - 1)..close.len() {
        let mut sum_mfv = 0.0;
        let mut sum_vol = 0.0;
        for j in 0..period {
            sum_mfv += mfv[i - j];
            sum_vol += volume[i - j];
        }
        if sum_vol != 0.0 {
            out[i] = sum_mfv / sum_vol;
        } else {
            out[i] = 0.0;
        }
    }
    out
}
