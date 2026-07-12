/// Heikin-Ashi
pub struct HeikinAshi {
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
}

pub fn heikin_ashi(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> HeikinAshi {
    let n = open.len();
    let mut res = HeikinAshi {
        open: vec![0.0; n],
        high: vec![0.0; n],
        low: vec![0.0; n],
        close: vec![0.0; n],
    };
    if n == 0 { return res; }
    
    res.close[0] = (open[0] + high[0] + low[0] + close[0]) / 4.0;
    res.open[0] = open[0];
    res.high[0] = high[0];
    res.low[0] = low[0];
    
    for i in 1..n {
        res.close[i] = (open[i] + high[i] + low[i] + close[i]) / 4.0;
        res.open[i] = (res.open[i - 1] + res.close[i - 1]) / 2.0;
        res.high[i] = high[i].max(res.open[i]).max(res.close[i]);
        res.low[i] = low[i].min(res.open[i]).min(res.close[i]);
    }
    res
}

pub fn heikin_ashi_close(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
    heikin_ashi(open, high, low, close).close
}
