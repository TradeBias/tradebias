/// Parabolic SAR
pub fn psar(high: &[f64], low: &[f64], af_step: f64, af_max: f64) -> Vec<f64> {
    let mut sar = vec![f64::NAN; high.len()];
    if high.len() < 2 { return sar; }

    let mut is_long = true;
    let mut ep = high[0]; // Extreme point
    let mut af = af_step;
    let mut current_sar = low[0];

    sar[0] = current_sar;

    for i in 1..high.len() {
        let prev_sar = current_sar;
        current_sar = prev_sar + af * (ep - prev_sar);

        if is_long {
            if i > 1 {
                current_sar = current_sar.min(low[i - 1]).min(low[i - 2]);
            }
            if low[i] < current_sar {
                is_long = false;
                current_sar = ep;
                ep = low[i];
                af = af_step;
            } else {
                if high[i] > ep {
                    ep = high[i];
                    af = (af + af_step).min(af_max);
                }
            }
        } else {
            if i > 1 {
                current_sar = current_sar.max(high[i - 1]).max(high[i - 2]);
            }
            if high[i] > current_sar {
                is_long = true;
                current_sar = ep;
                ep = high[i];
                af = af_step;
            } else {
                if low[i] < ep {
                    ep = low[i];
                    af = (af + af_step).min(af_max);
                }
            }
        }
        sar[i] = current_sar;
    }
    sar
}
