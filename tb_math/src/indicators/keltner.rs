use super::{ema::ema, atr::atr};

/// Keltner Channels
pub fn keltner_channels(
    high: &[f64], low: &[f64], close: &[f64],
    period: usize, multiplier: f64
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut upper = vec![f64::NAN; close.len()];
    let mut mid = vec![f64::NAN; close.len()];
    let mut lower = vec![f64::NAN; close.len()];

    if close.len() < period || period == 0 { return (upper, mid, lower); }

    let ema_line = ema(close, period);
    let atr_line = atr(high, low, close, period);

    for i in 0..close.len() {
        if !ema_line[i].is_nan() && !atr_line[i].is_nan() {
            mid[i] = ema_line[i];
            upper[i] = ema_line[i] + (multiplier * atr_line[i]);
            lower[i] = ema_line[i] - (multiplier * atr_line[i]);
        }
    }
    (upper, mid, lower)
}
