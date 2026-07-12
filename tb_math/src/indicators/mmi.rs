pub fn market_meanness_index(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < period || period < 2 { return out; }
    
    for i in (period - 1)..data.len() {
        let m = &data[(i + 1 - period)..=i];
        let median = {
            let mut sorted = m.to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            sorted[period / 2]
        };
        
        let mut nh = 0;
        let mut nl = 0;
        
        for j in 1..period {
            if m[j] > median && m[j] > m[j-1] { nh += 1; }
            if m[j] < median && m[j] < m[j-1] { nl += 1; }
        }
        
        out[i] = 100.0 * (nh + nl) as f64 / (period - 1) as f64;
    }
    out
}

