pub struct Gator {
    pub top: Vec<f64>,
    pub bottom: Vec<f64>,
}

pub fn gator_oscillator(data: &[f64]) -> Gator {
    let n = data.len();
    let jaw_smma = crate::indicators::smma::smma(data, 13);
    let teeth_smma = crate::indicators::smma::smma(data, 8);
    let lips_smma = crate::indicators::smma::smma(data, 5);
    
    let mut jaw = vec![f64::NAN; n];
    let mut teeth = vec![f64::NAN; n];
    let mut lips = vec![f64::NAN; n];
    
    for i in 8..n { jaw[i] = jaw_smma[i-8]; }
    for i in 5..n { teeth[i] = teeth_smma[i-5]; }
    for i in 3..n { lips[i] = lips_smma[i-3]; }
    
    let mut top = vec![f64::NAN; n];
    let mut bottom = vec![f64::NAN; n];
    for i in 0..n {
        top[i] = (jaw[i] - teeth[i]).abs();
        bottom[i] = -(teeth[i] - lips[i]).abs();
    }
    Gator { top, bottom }
}

pub fn gator_top(d: &[f64]) -> Vec<f64> { gator_oscillator(d).top }
pub fn gator_bottom(d: &[f64]) -> Vec<f64> { gator_oscillator(d).bottom }
