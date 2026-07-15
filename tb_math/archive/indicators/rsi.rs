/// Relative Strength Index (RSI)
pub fn rsi(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() <= period || period == 0 {
        return out;
    }

    let mut gain_sum = 0.0;
    let mut loss_sum = 0.0;

    // First average gain/loss
    for i in 1..=period {
        let diff = data[i] - data[i - 1];
        if diff >= 0.0 {
            gain_sum += diff;
        } else {
            loss_sum += diff.abs();
        }
    }

    let mut avg_gain = gain_sum / period as f64;
    let mut avg_loss = loss_sum / period as f64;

    if avg_loss == 0.0 {
        out[period] = 100.0;
    } else {
        let rs = avg_gain / avg_loss;
        out[period] = 100.0 - (100.0 / (1.0 + rs));
    }

    // Smoothed subsequent values
    for i in (period + 1)..data.len() {
        let diff = data[i] - data[i - 1];
        let mut gain = 0.0;
        let mut loss = 0.0;
        if diff >= 0.0 {
            gain = diff;
        } else {
            loss = diff.abs();
        }

        avg_gain = ((avg_gain * (period as f64 - 1.0)) + gain) / period as f64;
        avg_loss = ((avg_loss * (period as f64 - 1.0)) + loss) / period as f64;

        if avg_loss == 0.0 {
            out[i] = 100.0;
        } else {
            let rs = avg_gain / avg_loss;
            out[i] = 100.0 - (100.0 / (1.0 + rs));
        }
    }
    out
}
