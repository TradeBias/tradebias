use super::ema::ema;

/// Triple Exponential Moving Average (TEMA)
pub fn tema(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period == 0 { return out; }
    
    let ema1 = ema(data, period);
    
    let mut vs1 = 0;
    while vs1 < ema1.len() && ema1[vs1].is_nan() { vs1 += 1; }
    if vs1 < ema1.len() {
        let ema2_v = ema(&ema1[vs1..], period);
        
        let mut vs2 = 0;
        while vs2 < ema2_v.len() && ema2_v[vs2].is_nan() { vs2 += 1; }
        if vs2 < ema2_v.len() {
            let ema3_v = ema(&ema2_v[vs2..], period);
            for i in 0..ema3_v.len() {
                if !ema3_v[i].is_nan() {
                    let e1 = ema1[vs1 + vs2 + i];
                    let e2 = ema2_v[vs2 + i];
                    let e3 = ema3_v[i];
                    out[vs1 + vs2 + i] = 3.0 * e1 - 3.0 * e2 + e3;
                }
            }
        }
    }
    out
}
