/// Volume Flow Indicator (VFI)
pub fn vfi(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() < period || period == 0 { return out; }
    
    let mut tp = vec![0.0; close.len()];
    for i in 0..close.len() {
        tp[i] = (high[i] + low[i] + close[i]) / 3.0;
    }
    
    let mut log_ret = vec![0.0; close.len()];
    for i in 1..close.len() {
        if tp[i - 1] > 0.0 && tp[i] > 0.0 {
            log_ret[i] = (tp[i] / tp[i - 1]).ln();
        }
    }
    
    let std_dev = super::standard_deviation::standard_deviation(&log_ret, 30);
    let mut vmf = vec![0.0; close.len()];
    
    for i in 1..close.len() {
        let cutoff = if !std_dev[i].is_nan() { 0.2 * std_dev[i] * close[i] } else { 0.0 };
        let diff = tp[i] - tp[i - 1];
        let dir = if diff > cutoff { 1.0 } else if diff < -cutoff { -1.0 } else { 0.0 };
        vmf[i] = dir * volume[i];
    }
    
    let mut sum_vmf = vec![0.0; close.len()];
    for i in (period - 1)..close.len() {
        let mut sum = 0.0;
        for j in 0..period {
            sum += vmf[i - j];
        }
        sum_vmf[i] = sum;
    }
    
    let ema_vol = super::ema::ema(volume, period);
    
    let mut vfi_raw = vec![0.0; close.len()];
    for i in 0..close.len() {
        if !ema_vol[i].is_nan() && ema_vol[i] > 0.0 {
            vfi_raw[i] = (sum_vmf[i] / ema_vol[i]) * 100.0;
        }
    }
    
    super::ema::ema(&vfi_raw, 3)
}
