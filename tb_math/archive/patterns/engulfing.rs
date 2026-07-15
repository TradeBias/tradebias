pub fn bullish_engulfing(open: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; open.len()];
    for i in 1..open.len() {
        if open[i-1].is_nan() || close[i-1].is_nan() || open[i].is_nan() || close[i].is_nan() {
            continue;
        }
        
        let prev_is_bearish = close[i-1] < open[i-1];
        let curr_is_bullish = close[i] > open[i];
        
        // Engulfing condition: Current body completely covers previous body
        let engulfs = close[i] > open[i-1] && open[i] <= close[i-1];
        
        if prev_is_bearish && curr_is_bullish && engulfs {
            out[i] = 1.0;
        } else {
            out[i] = 0.0;
        }
    }
    out
}

pub fn bearish_engulfing(open: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; open.len()];
    for i in 1..open.len() {
        if open[i-1].is_nan() || close[i-1].is_nan() || open[i].is_nan() || close[i].is_nan() {
            continue;
        }
        
        let prev_is_bullish = close[i-1] > open[i-1];
        let curr_is_bearish = close[i] < open[i];
        
        // Engulfing condition: Current body completely covers previous body
        let engulfs = close[i] < open[i-1] && open[i] >= close[i-1];
        
        if prev_is_bullish && curr_is_bearish && engulfs {
            out[i] = 1.0;
        } else {
            out[i] = 0.0;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullish_engulfing() {
        let open = vec![10.0, 9.0];
        let close = vec![9.0, 11.0];
        let res = bullish_engulfing(&open, &close);
        assert_eq!(res[1], 1.0);
    }
}
