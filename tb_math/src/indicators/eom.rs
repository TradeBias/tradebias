/// Ease of Movement (EOM)
pub fn eom(high: &[f64], low: &[f64], volume: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; high.len()];
    if high.len() < period + 1 || period == 0 { return out; }
    
    let mut eom_1 = vec![0.0; high.len()];
    
    for i in 1..high.len() {
        let dm = ((high[i] + low[i]) / 2.0) - ((high[i - 1] + low[i - 1]) / 2.0);
        let hl = high[i] - low[i];
        if volume[i] == 0.0 {
            eom_1[i] = 0.0;
        } else {
            eom_1[i] = dm * hl / volume[i];
        }
    }
    
    for i in period..high.len() {
        let mut sum = 0.0;
        for j in 0..period {
            sum += eom_1[i - j];
        }
        out[i] = sum / period as f64;
    }
    
    out
}
