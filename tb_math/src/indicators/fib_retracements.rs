/// Fibonacci Retracements
pub struct FibRetracements {
    pub level_236: Vec<f64>,
    pub level_382: Vec<f64>,
    pub level_500: Vec<f64>,
    pub level_618: Vec<f64>,
    pub level_786: Vec<f64>,
}

pub fn fib_retracements(high: &[f64], low: &[f64], period: usize) -> FibRetracements {
    let mut res = FibRetracements {
        level_236: vec![f64::NAN; high.len()],
        level_382: vec![f64::NAN; high.len()],
        level_500: vec![f64::NAN; high.len()],
        level_618: vec![f64::NAN; high.len()],
        level_786: vec![f64::NAN; high.len()],
    };
    if high.len() < period || period == 0 { return res; }
    
    for i in (period - 1)..high.len() {
        let mut highest = f64::MIN;
        let mut lowest = f64::MAX;
        for j in 0..period {
            if high[i - j] > highest { highest = high[i - j]; }
            if low[i - j] < lowest { lowest = low[i - j]; }
        }
        
        let diff = highest - lowest;
        res.level_236[i] = highest - diff * 0.236;
        res.level_382[i] = highest - diff * 0.382;
        res.level_500[i] = highest - diff * 0.500;
        res.level_618[i] = highest - diff * 0.618;
        res.level_786[i] = highest - diff * 0.786;
    }
    res
}

pub fn fib_level_236(h: &[f64], l: &[f64], p: usize) -> Vec<f64> { fib_retracements(h, l, p).level_236 }
pub fn fib_level_382(h: &[f64], l: &[f64], p: usize) -> Vec<f64> { fib_retracements(h, l, p).level_382 }
pub fn fib_level_500(h: &[f64], l: &[f64], p: usize) -> Vec<f64> { fib_retracements(h, l, p).level_500 }
pub fn fib_level_618(h: &[f64], l: &[f64], p: usize) -> Vec<f64> { fib_retracements(h, l, p).level_618 }
pub fn fib_level_786(h: &[f64], l: &[f64], p: usize) -> Vec<f64> { fib_retracements(h, l, p).level_786 }
