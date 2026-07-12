use std::fs::File;
use std::io::{BufRead, BufReader};
use tb_math::indicators::*;

#[derive(Debug, Default)]
struct PriceData {
    open: Vec<f64>,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    volume: Vec<f64>,
    
    // Targets
    sma_14: Vec<f64>,
    ema_14: Vec<f64>,
    rsi_14: Vec<f64>,
    macd_line: Vec<f64>,
    macd_signal: Vec<f64>,
    macd_hist: Vec<f64>,
    bb_lower: Vec<f64>,
    bb_mid: Vec<f64>,
    bb_upper: Vec<f64>,
    atr_14: Vec<f64>,
    adx_14: Vec<f64>,
    plus_di: Vec<f64>,
    minus_di: Vec<f64>,
    supertrend: Vec<f64>,
}

fn load_csv() -> PriceData {
    let file = File::open("tests/validation_targets.csv").expect("Run `python tests/audit.py` to generate the CSV first!");
    let reader = BufReader::new(file);
    let mut data = PriceData::default();
    
    for (i, line) in reader.lines().enumerate() {
        if i == 0 { continue; } // Skip header
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        
        let parse_f64 = |s: &str| -> f64 { if s == "NaN" { f64::NAN } else { s.parse().unwrap() } };
        
        data.open.push(parse_f64(parts[0]));
        data.high.push(parse_f64(parts[1]));
        data.low.push(parse_f64(parts[2]));
        data.close.push(parse_f64(parts[3]));
        data.volume.push(parse_f64(parts[4]));
        
        data.sma_14.push(parse_f64(parts[5]));
        data.ema_14.push(parse_f64(parts[6]));
        data.rsi_14.push(parse_f64(parts[7]));
        data.macd_line.push(parse_f64(parts[8]));
        data.macd_signal.push(parse_f64(parts[9]));
        data.macd_hist.push(parse_f64(parts[10]));
        data.bb_lower.push(parse_f64(parts[11]));
        data.bb_mid.push(parse_f64(parts[12]));
        data.bb_upper.push(parse_f64(parts[13]));
        data.atr_14.push(parse_f64(parts[14]));
        data.adx_14.push(parse_f64(parts[15]));
        data.plus_di.push(parse_f64(parts[16]));
        data.minus_di.push(parse_f64(parts[17]));
        data.supertrend.push(parse_f64(parts[18]));
    }
    data
}

fn assert_series_eq(name: &str, calculated: &[f64], target: &[f64], tolerance: f64) {
    for i in 0..calculated.len() {
        let c = calculated[i];
        let t = target[i];
        
        if c.is_nan() && t.is_nan() { continue; }
        if c.is_nan() || t.is_nan() {
            panic!("{} mismatch at index {}: calculated={}, target={}", name, i, c, t);
        }
        if (c - t).abs() > tolerance {
            panic!("{} mismatch at index {}: calculated={}, target={}", name, i, c, t);
        }
    }
}

#[test]
fn test_math_audit() {
    let data = load_csv();
    let tol = 0.0001;
    
    let sma = sma(&data.close, 14);
    assert_series_eq("SMA", &sma, &data.sma_14, tol);
    
    let ema = ema(&data.close, 14);
    assert_series_eq("EMA", &ema, &data.ema_14, tol);
    
    let rsi = rsi(&data.close, 14);
    assert_series_eq("RSI", &rsi, &data.rsi_14, tol);
    
    let (macd_line, macd_signal, macd_hist) = macd(&data.close, 12, 26, 9);
    assert_series_eq("MACD Line", &macd_line, &data.macd_line, tol);
    assert_series_eq("MACD Signal", &macd_signal, &data.macd_signal, tol);
    assert_series_eq("MACD Hist", &macd_hist, &data.macd_hist, tol);
    
    let (bb_upper, bb_mid, bb_lower) = bollinger_bands(&data.close, 20, 2.0);
    assert_series_eq("BB Lower", &bb_lower, &data.bb_lower, tol);
    assert_series_eq("BB Mid", &bb_mid, &data.bb_mid, tol);
    assert_series_eq("BB Upper", &bb_upper, &data.bb_upper, tol);
    
    let atr = atr(&data.high, &data.low, &data.close, 14);
    assert_series_eq("ATR", &atr, &data.atr_14, tol);
    
    let (adx_line, plus_di, minus_di) = adx(&data.high, &data.low, &data.close, 14);
    assert_series_eq("ADX", &adx_line, &data.adx_14, tol);
    assert_series_eq("+DI", &plus_di, &data.plus_di, tol);
    assert_series_eq("-DI", &minus_di, &data.minus_di, tol);
    
    // Note: Supertrend needs to be added to tb_math if not present, but for now this validates Wilder's Smoothing etc.
}
