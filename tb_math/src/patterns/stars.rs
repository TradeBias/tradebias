pub fn morning_star(open: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; open.len()];
    for i in 2..open.len() {
        if open[i-2].is_nan() || close[i-2].is_nan() ||
           open[i-1].is_nan() || close[i-1].is_nan() ||
           open[i].is_nan() || close[i].is_nan() {
            continue;
        }
        
        // 1. First candle is large bearish
        let c1_is_bearish = close[i-2] < open[i-2];
        let c1_body = open[i-2] - close[i-2];
        
        // 2. Second candle has small body and gaps down (or opens below c1 close)
        let c2_body = (close[i-1] - open[i-1]).abs();
        let c2_gap_down = open[i-1] < close[i-2]; // Simple gap definition
        
        // 3. Third candle is large bullish and closes at least halfway into c1's body
        let c3_is_bullish = close[i] > open[i];
        let c3_closes_into_c1 = close[i] > close[i-2] + (c1_body / 2.0);
        
        if c1_is_bearish && c2_body < c1_body * 0.3 && c2_gap_down && c3_is_bullish && c3_closes_into_c1 {
            out[i] = 1.0;
        } else {
            out[i] = 0.0;
        }
    }
    out
}

pub fn evening_star(open: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; open.len()];
    for i in 2..open.len() {
        if open[i-2].is_nan() || close[i-2].is_nan() ||
           open[i-1].is_nan() || close[i-1].is_nan() ||
           open[i].is_nan() || close[i].is_nan() {
            continue;
        }
        
        // 1. First candle is large bullish
        let c1_is_bullish = close[i-2] > open[i-2];
        let c1_body = close[i-2] - open[i-2];
        
        // 2. Second candle has small body and gaps up (or opens above c1 close)
        let c2_body = (close[i-1] - open[i-1]).abs();
        let c2_gap_up = open[i-1] > close[i-2];
        
        // 3. Third candle is large bearish and closes at least halfway down into c1's body
        let c3_is_bearish = close[i] < open[i];
        let c3_closes_into_c1 = close[i] < close[i-2] - (c1_body / 2.0);
        
        if c1_is_bullish && c2_body < c1_body * 0.3 && c2_gap_up && c3_is_bearish && c3_closes_into_c1 {
            out[i] = 1.0;
        } else {
            out[i] = 0.0;
        }
    }
    out
}
