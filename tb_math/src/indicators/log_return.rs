/// Log Return
pub fn log_return(data: &[f64]) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    for i in 1..data.len() {
        if data[i - 1] > 0.0 && data[i] > 0.0 {
            out[i] = (data[i] / data[i - 1]).ln();
        }
    }
    out
}
