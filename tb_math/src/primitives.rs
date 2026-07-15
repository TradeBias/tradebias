use std::collections::VecDeque;

/// Mathematical and Structural Primitives for the Alpha Foundry

/// Replaces all NaN values with a fallback, or carries the last valid value forward.
fn fill_nan(data: &mut [f64], default: f64) {
    let mut last = default;
    for x in data.iter_mut() {
        if x.is_nan() {
            *x = last;
        } else {
            last = *x;
        }
    }
}

/// Addition
pub fn add(lhs: &[f64], rhs: &[f64]) -> Vec<f64> {
    lhs.iter().zip(rhs.iter()).map(|(a, b)| a + b).collect()
}

/// Subtraction
pub fn sub(lhs: &[f64], rhs: &[f64]) -> Vec<f64> {
    lhs.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect()
}

/// Multiplication
pub fn mul(lhs: &[f64], rhs: &[f64]) -> Vec<f64> {
    lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect()
}

/// Division
pub fn div(lhs: &[f64], rhs: &[f64]) -> Vec<f64> {
    lhs.iter().zip(rhs.iter()).map(|(a, b)| if *b == 0.0 { f64::NAN } else { a / b }).collect()
}

/// Absolute Value
pub fn abs(data: &[f64]) -> Vec<f64> {
    data.iter().map(|a| a.abs()).collect()
}

/// Delay (Shift backward in time)
pub fn delay(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if period < data.len() {
        for i in period..data.len() {
            out[i] = data[i - period];
        }
    }
    out
}

/// Rolling Maximum
pub fn ts_max(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if period == 0 || data.len() < period { return out; }
    let mut deque: VecDeque<usize> = VecDeque::with_capacity(period);
    
    for i in 0..data.len() {
        while let Some(&idx) = deque.front() {
            if i >= period && idx <= i - period { deque.pop_front(); } else { break; }
        }
        while let Some(&idx) = deque.back() {
            if data[idx] <= data[i] { deque.pop_back(); } else { break; }
        }
        deque.push_back(i);
        if i >= period - 1 { out[i] = data[*deque.front().unwrap()]; }
    }
    out
}

/// Rolling Minimum
pub fn ts_min(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if period == 0 || data.len() < period { return out; }
    let mut deque: VecDeque<usize> = VecDeque::with_capacity(period);
    
    for i in 0..data.len() {
        while let Some(&idx) = deque.front() {
            if i >= period && idx <= i - period { deque.pop_front(); } else { break; }
        }
        while let Some(&idx) = deque.back() {
            if data[idx] >= data[i] { deque.pop_back(); } else { break; }
        }
        deque.push_back(i);
        if i >= period - 1 { out[i] = data[*deque.front().unwrap()]; }
    }
    out
}

/// Rolling Sum
pub fn ts_sum(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if period == 0 || data.len() < period { return out; }
    let mut sum = 0.0;
    for i in 0..period {
        sum += data[i];
    }
    out[period - 1] = sum;
    for i in period..data.len() {
        sum += data[i] - data[i - period];
        out[i] = sum;
    }
    out
}

/// Simple Moving Average (SMA)
pub fn sma(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = ts_sum(data, period);
    for x in out.iter_mut() {
        *x /= period as f64;
    }
    out
}

/// Exponential Moving Average (EMA)
pub fn ema(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if period == 0 || data.len() < period { return out; }
    let alpha = 2.0 / (period as f64 + 1.0);
    
    // SMA for first value
    let mut sum = 0.0;
    for i in 0..period { sum += data[i]; }
    let mut prev = sum / period as f64;
    out[period - 1] = prev;

    for i in period..data.len() {
        prev = (data[i] - prev) * alpha + prev;
        out[i] = prev;
    }
    out
}

/// Weighted Moving Average (WMA)
pub fn wma(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if period == 0 || data.len() < period { return out; }
    let denominator = (period * (period + 1)) as f64 / 2.0;
    
    let mut sum = 0.0;
    let mut weight_sum = 0.0;
    
    for i in 0..period {
        sum += data[i];
        weight_sum += data[i] * (i + 1) as f64;
    }
    out[period - 1] = weight_sum / denominator;
    
    for i in period..data.len() {
        let prev_sum = sum;
        sum += data[i] - data[i - period];
        weight_sum += data[i] * period as f64 - prev_sum;
        out[i] = weight_sum / denominator;
    }
    out
}

/// Wilder's Smoothing (RMA)
pub fn rma(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if period == 0 || data.len() < period { return out; }
    let alpha = 1.0 / period as f64;
    
    // SMA for first value
    let mut sum = 0.0;
    for i in 0..period { sum += data[i]; }
    let mut prev = sum / period as f64;
    out[period - 1] = prev;

    for i in period..data.len() {
        prev = (data[i] - prev) * alpha + prev;
        out[i] = prev;
    }
    out
}

/// Rolling Standard Deviation
pub fn std_dev(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if period < 2 || data.len() < period { return out; }
    
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    
    for i in 0..period {
        sum += data[i];
        sum_sq += data[i] * data[i];
    }
    
    let mean = sum / period as f64;
    let var = (sum_sq / period as f64) - (mean * mean);
    out[period - 1] = var.max(0.0).sqrt();
    
    for i in period..data.len() {
        sum += data[i] - data[i - period];
        sum_sq += data[i] * data[i] - data[i - period] * data[i - period];
        
        let mean = sum / period as f64;
        let var = (sum_sq / period as f64) - (mean * mean);
        out[i] = var.max(0.0).sqrt();
    }
    out
}

/// Linear Regression Slope
pub fn lin_reg_slope(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if period < 2 || data.len() < period { return out; }
    
    let sum_x = (period * (period - 1)) as f64 / 2.0;
    let sum_x_sq = (period * (period - 1) * (2 * period - 1)) as f64 / 6.0;
    let divisor = (period as f64 * sum_x_sq) - (sum_x * sum_x);
    
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    
    for i in 0..period {
        sum_y += data[i];
        sum_xy += data[i] * i as f64;
    }
    
    out[period - 1] = ((period as f64 * sum_xy) - (sum_x * sum_y)) / divisor;
    
    for i in period..data.len() {
        let prev_sum_y = sum_y;
        sum_xy += data[i] * (period - 1) as f64 - (prev_sum_y - data[i - period]);
        sum_y += data[i] - data[i - period];
        
        out[i] = ((period as f64 * sum_xy) - (sum_x * sum_y)) / divisor;
    }
    out
}

/// True Range
pub fn true_range(high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; high.len()];
    if high.is_empty() { return out; }
    out[0] = high[0] - low[0];
    
    for i in 1..high.len() {
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        out[i] = hl.max(hc).max(lc);
    }
    out
}
