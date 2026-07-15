/// Money Flow Index (MFI)
pub fn mfi(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; close.len()];
    if close.len() <= period || period == 0 { return out; }
    
    let mut tp = vec![0.0; close.len()];
    let mut rm = vec![0.0; close.len()]; 
    
    for i in 0..close.len() {
        tp[i] = (high[i] + low[i] + close[i]) / 3.0;
        rm[i] = tp[i] * volume[i];
    }
    
    for i in period..close.len() {
        let mut pos_flow = 0.0;
        let mut neg_flow = 0.0;
        
        for j in 0..period {
            let idx = i - j;
            if tp[idx] > tp[idx - 1] {
                pos_flow += rm[idx];
            } else if tp[idx] < tp[idx - 1] {
                neg_flow += rm[idx];
            }
        }
        
        if neg_flow == 0.0 {
            out[i] = 100.0;
        } else {
            let mfr = pos_flow / neg_flow;
            out[i] = 100.0 - (100.0 / (1.0 + mfr));
        }
    }
    out
}
