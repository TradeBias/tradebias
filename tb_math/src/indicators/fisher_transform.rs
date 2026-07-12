/// Fisher Transform
pub fn fisher_transform(high: &[f64], low: &[f64], period: usize) -> (Vec<f64>, Vec<f64>) {
    let mut fisher = vec![f64::NAN; high.len()];
    let mut trigger = vec![f64::NAN; high.len()];
    if high.len() < period || period == 0 { return (fisher, trigger); }
    
    let mut value = vec![0.0; high.len()];
    
    for i in (period - 1)..high.len() {
        let mut highest = high[i + 1 - period];
        let mut lowest = low[i + 1 - period];
        for j in 0..period {
            let h = high[i - j];
            let l = low[i - j];
            if h > highest { highest = h; }
            if l < lowest { lowest = l; }
        }
        
        let price = (high[i] + low[i]) / 2.0;
        let mut v = if highest == lowest {
            0.0
        } else {
            0.66 * ((price - lowest) / (highest - lowest) - 0.5)
        };
        
        if i > period - 1 {
            v += 0.67 * value[i - 1];
        }
        
        if v > 0.999 { v = 0.999; }
        if v < -0.999 { v = -0.999; }
        value[i] = v;
        
        let mut f = 0.5 * ((1.0 + v) / (1.0 - v)).ln();
        if i > period - 1 {
            f += 0.5 * fisher[i - 1];
        }
        fisher[i] = f;
        
        if i > 0 {
            trigger[i] = fisher[i - 1];
        }
    }
    
    (fisher, trigger)
}

