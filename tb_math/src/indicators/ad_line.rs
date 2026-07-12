/// Accumulation/Distribution Line (A/D)
pub fn ad_line(high: &[f64], low: &[f64], close: &[f64], volume: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; close.len()];
    let mut ad = 0.0;
    for i in 0..close.len() {
        let diff = high[i] - low[i];
        if diff != 0.0 {
            let mfm = ((close[i] - low[i]) - (high[i] - close[i])) / diff;
            let mfv = mfm * volume[i];
            ad += mfv;
        }
        out[i] = ad;
    }
    out
}
