/// Linear Regression Math
pub struct LinRegResult {
    pub slope: Vec<f64>,
    pub intercept: Vec<f64>,
    pub r_squared: Vec<f64>,
    pub curve: Vec<f64>,
}

pub fn linear_regression(data: &[f64], period: usize) -> LinRegResult {
    let n = data.len();
    let mut res = LinRegResult {
        slope: vec![f64::NAN; n],
        intercept: vec![f64::NAN; n],
        r_squared: vec![f64::NAN; n],
        curve: vec![f64::NAN; n],
    };
    
    if n < period || period < 2 { return res; }
    
    let p_f64 = period as f64;
    let sum_x = p_f64 * (p_f64 + 1.0) / 2.0;
    let mean_x = sum_x / p_f64;
    
    let mut sum_x_sq = 0.0;
    for x in 1..=period {
        sum_x_sq += (x as f64) * (x as f64);
    }
    let ss_x = sum_x_sq - (sum_x * sum_x) / p_f64;

    for i in (period - 1)..n {
        let mut sum_y = 0.0;
        for j in 0..period {
            sum_y += data[i + 1 + j - period];
        }
        let mean_y = sum_y / p_f64;
        
        let mut ss_xy = 0.0;
        let mut sum_y_sq = 0.0;
        
        for j in 0..period {
            let x = (j + 1) as f64;
            let y = data[i + 1 + j - period];
            ss_xy += (x - mean_x) * (y - mean_y);
            sum_y_sq += (y - mean_y) * (y - mean_y);
        }
        
        let slope = ss_xy / ss_x;
        let intercept = mean_y - slope * mean_x;
        let curve = intercept + slope * p_f64;
        
        let r_squared = if sum_y_sq != 0.0 {
            let r = ss_xy / (ss_x.sqrt() * sum_y_sq.sqrt());
            r * r
        } else {
            0.0
        };
        
        res.slope[i] = slope;
        res.intercept[i] = intercept;
        res.curve[i] = curve;
        res.r_squared[i] = r_squared;
    }
    
    res
}

pub fn linreg_slope(data: &[f64], period: usize) -> Vec<f64> { linear_regression(data, period).slope }
pub fn linreg_intercept(data: &[f64], period: usize) -> Vec<f64> { linear_regression(data, period).intercept }
pub fn linreg_r_squared(data: &[f64], period: usize) -> Vec<f64> { linear_regression(data, period).r_squared }
pub fn linreg_curve(data: &[f64], period: usize) -> Vec<f64> { linear_regression(data, period).curve }

