/// Pivot Points
pub struct PivotPoints {
    pub p: Vec<f64>,
    pub r1: Vec<f64>,
    pub s1: Vec<f64>,
    pub r2: Vec<f64>,
    pub s2: Vec<f64>,
}

pub fn pivot_points(high: &[f64], low: &[f64], close: &[f64], period: usize) -> PivotPoints {
    let mut res = PivotPoints {
        p: vec![f64::NAN; close.len()],
        r1: vec![f64::NAN; close.len()],
        s1: vec![f64::NAN; close.len()],
        r2: vec![f64::NAN; close.len()],
        s2: vec![f64::NAN; close.len()],
    };
    if close.len() < period || period == 0 { return res; }
    
    for i in period..close.len() {
        let mut highest = f64::MIN;
        let mut lowest = f64::MAX;
        let prev_close = close[i - 1];
        
        for j in 1..=period {
            let h = high[i - j];
            let l = low[i - j];
            if h > highest { highest = h; }
            if l < lowest { lowest = l; }
        }
        
        let p = (highest + lowest + prev_close) / 3.0;
        let r1 = (2.0 * p) - lowest;
        let s1 = (2.0 * p) - highest;
        let r2 = p + (highest - lowest);
        let s2 = p - (highest - lowest);
        
        res.p[i] = p;
        res.r1[i] = r1;
        res.s1[i] = s1;
        res.r2[i] = r2;
        res.s2[i] = s2;
    }
    res
}

pub fn pivot_points_p(h: &[f64], l: &[f64], c: &[f64], p: usize) -> Vec<f64> { pivot_points(h, l, c, p).p }
pub fn pivot_points_r1(h: &[f64], l: &[f64], c: &[f64], p: usize) -> Vec<f64> { pivot_points(h, l, c, p).r1 }
pub fn pivot_points_s1(h: &[f64], l: &[f64], c: &[f64], p: usize) -> Vec<f64> { pivot_points(h, l, c, p).s1 }
pub fn pivot_points_r2(h: &[f64], l: &[f64], c: &[f64], p: usize) -> Vec<f64> { pivot_points(h, l, c, p).r2 }
pub fn pivot_points_s2(h: &[f64], l: &[f64], c: &[f64], p: usize) -> Vec<f64> { pivot_points(h, l, c, p).s2 }
