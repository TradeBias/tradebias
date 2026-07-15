/// VWAP (Cumulative Volume Weighted Average Price)
pub fn vwap(high: &[f64], low: &[f64], close: &[f64], volume: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    let mut cum_pv = 0.0;
    let mut cum_v = 0.0;
    for i in 0..close.len() {
        let typ_price = (high[i] + low[i] + close[i]) / 3.0;
        cum_pv += typ_price * volume[i];
        cum_v += volume[i];
        if cum_v > 0.0 {
            out[i] = cum_pv / cum_v;
        }
    }
    out
}
