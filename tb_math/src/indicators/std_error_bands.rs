/// Standard Error Bands
pub struct StdErrorBands {
    pub upper: Vec<f64>,
    pub lower: Vec<f64>,
}

pub fn standard_error_bands(data: &[f64], period: usize, multiplier: f64) -> StdErrorBands {
    let mut res = StdErrorBands {
        upper: vec![f64::NAN; data.len()],
        lower: vec![f64::NAN; data.len()],
    };
    if data.len() < period || period < 3 { return res; }
    
    let linreg = super::linreg::linear_regression(data, period);
    
    for i in (period - 1)..data.len() {
        let mut sum_sq_err = 0.0;
        let slope = linreg.slope[i];
        let intercept = linreg.intercept[i];
        
        for j in 0..period {
            let x = (j + 1) as f64;
            let y = data[i + 1 + j - period];
            let y_hat = intercept + slope * x;
            sum_sq_err += (y - y_hat) * (y - y_hat);
        }
        
        let see = (sum_sq_err / ((period - 2) as f64)).sqrt();
        let curve = linreg.curve[i];
        
        res.upper[i] = curve + see * multiplier;
        res.lower[i] = curve - see * multiplier;
    }
    res
}

pub fn std_error_bands_upper(data: &[f64], period: usize, multiplier: f64) -> Vec<f64> { standard_error_bands(data, period, multiplier).upper }
pub fn std_error_bands_lower(data: &[f64], period: usize, multiplier: f64) -> Vec<f64> { standard_error_bands(data, period, multiplier).lower }

