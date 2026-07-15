/// Donchian Channels
pub fn donchian_channels(high: &[f64], low: &[f64], period: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut upper = vec![f64::NAN; high.len()];
    let mut mid = vec![f64::NAN; high.len()];
    let mut lower = vec![f64::NAN; high.len()];

    if high.len() < period || period == 0 { return (upper, mid, lower); }

    for i in (period - 1)..high.len() {
        let mut highest = high[i + 1 - period];
        let mut lowest = low[i + 1 - period];
        
        for j in 0..period {
            let h = high[i - j];
            let l = low[i - j];
            if h > highest { highest = h; }
            if l < lowest { lowest = l; }
        }
        
        upper[i] = highest;
        lower[i] = lowest;
        mid[i] = (highest + lowest) / 2.0;
    }
    (upper, mid, lower)
}

