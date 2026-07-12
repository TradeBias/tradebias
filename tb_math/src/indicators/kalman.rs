pub fn kalman_filter(data: &[f64], r: f64, q: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.is_empty() { return out; }
    
    let mut estimate = data[0];
    let mut error_est = 1.0;
    
    for i in 0..data.len() {
        if data[i].is_nan() { continue; }
        let error_pred = error_est + q;
        
        let kalman_gain = error_pred / (error_pred + r);
        estimate = estimate + kalman_gain * (data[i] - estimate);
        error_est = (1.0 - kalman_gain) * error_pred;
        
        out[i] = estimate;
    }
    out
}
