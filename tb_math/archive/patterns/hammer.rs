pub fn hammer(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; open.len()];
    for i in 0..open.len() {
        if open[i].is_nan() || high[i].is_nan() || low[i].is_nan() || close[i].is_nan() {
            continue;
        }
        
        let body_size = (close[i] - open[i]).abs();
        let lower_shadow = open[i].min(close[i]) - low[i];
        let upper_shadow = high[i] - open[i].max(close[i]);
        let total_range = high[i] - low[i];
        
        if total_range == 0.0 {
            out[i] = 0.0;
            continue;
        }
        
        // Lower shadow is at least twice the real body
        // Upper shadow is less than 10% of the total range (little to no upper shadow)
        if lower_shadow >= body_size * 2.0 && upper_shadow <= total_range * 0.10 && body_size > 0.0 {
            out[i] = 1.0;
        } else {
            out[i] = 0.0;
        }
    }
    out
}

pub fn shooting_star(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; open.len()];
    for i in 0..open.len() {
        if open[i].is_nan() || high[i].is_nan() || low[i].is_nan() || close[i].is_nan() {
            continue;
        }
        
        let body_size = (close[i] - open[i]).abs();
        let lower_shadow = open[i].min(close[i]) - low[i];
        let upper_shadow = high[i] - open[i].max(close[i]);
        let total_range = high[i] - low[i];
        
        if total_range == 0.0 {
            out[i] = 0.0;
            continue;
        }
        
        // Upper shadow is at least twice the real body
        // Lower shadow is less than 10% of the total range
        if upper_shadow >= body_size * 2.0 && lower_shadow <= total_range * 0.10 && body_size > 0.0 {
            out[i] = 1.0;
        } else {
            out[i] = 0.0;
        }
    }
    out
}
