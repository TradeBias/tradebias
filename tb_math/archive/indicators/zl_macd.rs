pub fn zlema(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    let lag = (period - 1) / 2;
    if data.len() < lag + 1 { return out; }
    
    let mut de_lagged = vec![f64::NAN; data.len()];
    for i in lag..data.len() {
        de_lagged[i] = data[i] + (data[i] - data[i - lag]);
    }
    
    crate::indicators::ema::ema(&de_lagged, period)
}

pub struct ZlMacd {
    pub macd: Vec<f64>,
    pub signal: Vec<f64>,
    pub hist: Vec<f64>,
}

pub fn zl_macd(data: &[f64], fast: usize, slow: usize, signal: usize) -> ZlMacd {
    let fast_zlema = zlema(data, fast);
    let slow_zlema = zlema(data, slow);
    let n = data.len();
    let mut macd = vec![f64::NAN; n];
    for i in 0..n {
        macd[i] = fast_zlema[i] - slow_zlema[i];
    }
    let sig_ema = crate::indicators::ema::ema(&macd, signal);
    let mut hist = vec![f64::NAN; n];
    for i in 0..n {
        hist[i] = macd[i] - sig_ema[i];
    }
    ZlMacd { macd, signal: sig_ema, hist }
}

pub fn zl_macd_line(d: &[f64], f: usize, s: usize, sig: usize) -> Vec<f64> { zl_macd(d, f, s, sig).macd }
pub fn zl_macd_signal(d: &[f64], f: usize, s: usize, sig: usize) -> Vec<f64> { zl_macd(d, f, s, sig).signal }
pub fn zl_macd_hist(d: &[f64], f: usize, s: usize, sig: usize) -> Vec<f64> { zl_macd(d, f, s, sig).hist }
