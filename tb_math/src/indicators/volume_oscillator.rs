/// Volume Oscillator (VOSC)
pub fn volume_oscillator(volume: &[f64], fast: usize, slow: usize) -> Vec<f64> {
    let sma_fast = super::sma::sma(volume, fast);
    let sma_slow = super::sma::sma(volume, slow);
    let mut out = vec![f64::NAN; volume.len()];
    for i in 0..volume.len() {
        if !sma_slow[i].is_nan() && !sma_fast[i].is_nan() && sma_slow[i] > 0.0 {
            out[i] = ((sma_fast[i] - sma_slow[i]) / sma_slow[i]) * 100.0;
        }
    }
    out
}
