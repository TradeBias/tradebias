fn tri_ma(data: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; data.len()];
    for i in 3..data.len() {
        out[i] = (data[i] + 2.0 * data[i - 1] + 2.0 * data[i - 2] + data[i - 3]) / 6.0;
    }
    out
}

/// Relative Vigor Index (RVI)
pub fn rvi(open: &[f64], high: &[f64], low: &[f64], close: &[f64], period: usize) -> (Vec<f64>, Vec<f64>) {
    let mut rvi = vec![f64::NAN; close.len()];
    let mut sig = vec![f64::NAN; close.len()];
    if close.len() < period + 3 || period == 0 { return (rvi, sig); }
    
    let mut co = vec![0.0; close.len()];
    let mut hl = vec![0.0; close.len()];
    for i in 0..close.len() {
        co[i] = close[i] - open[i];
        hl[i] = high[i] - low[i];
    }
    
    let val1 = tri_ma(&co);
    let val2 = tri_ma(&hl);
    
    for i in (period + 2)..close.len() {
        let mut num = 0.0;
        let mut den = 0.0;
        for j in 0..period {
            num += val1[i - j];
            den += val2[i - j];
        }
        if den == 0.0 {
            rvi[i] = 0.0;
        } else {
            rvi[i] = num / den;
        }
    }
    
    for i in (period + 5)..close.len() {
        sig[i] = (rvi[i] + 2.0 * rvi[i - 1] + 2.0 * rvi[i - 2] + rvi[i - 3]) / 6.0;
    }
    
    (rvi, sig)
}
