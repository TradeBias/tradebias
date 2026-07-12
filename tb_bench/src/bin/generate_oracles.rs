use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let output_dir = "tb_bench/oracles";
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    generate_bull(format!("{}/oracle_bull.csv", output_dir).as_str());
    generate_bear(format!("{}/oracle_bear.csv", output_dir).as_str());
    generate_flat(format!("{}/oracle_flat.csv", output_dir).as_str());
    generate_whipsaw(format!("{}/oracle_whipsaw.csv", output_dir).as_str());

    println!("Successfully generated all 500-bar Oracle datasets in {}", output_dir);
}

fn generate_bull(path: &str) {
    let mut file = File::create(path).unwrap();
    writeln!(file, "Date,Open,High,Low,Close,Volume").unwrap();
    
    let mut close = 100.0;
    
    for i in 1..=500 {
        let mut open = close;
        
        // Inject a massive gap up at bar 100
        if i == 100 {
            open += 10.0;
        }

        close = open + 1.0;
        let high = close + 0.5;
        let low = open - 0.5;
        
        // Inject a zero-volume bar at bar 200
        let vol = if i == 200 { 0.0 } else { 1000.0 };

        writeln!(file, "2020-01-{:02},{},{},{},{},{}", (i % 28) + 1, open, high, low, close, vol).unwrap();
    }
}

fn generate_bear(path: &str) {
    let mut file = File::create(path).unwrap();
    writeln!(file, "Date,Open,High,Low,Close,Volume").unwrap();
    
    let mut close = 1000.0;
    
    for i in 1..=500 {
        let mut open = close;
        
        // Inject a massive gap down at bar 150
        if i == 150 {
            open -= 10.0;
        }

        close = open - 1.0;
        let high = open + 0.5;
        let low = close - 0.5;
        
        let vol = 1000.0;
        writeln!(file, "2020-01-{:02},{},{},{},{},{}", (i % 28) + 1, open, high, low, close, vol).unwrap();
    }
}

fn generate_flat(path: &str) {
    let mut file = File::create(path).unwrap();
    writeln!(file, "Date,Open,High,Low,Close,Volume").unwrap();
    
    for i in 1..=500 {
        let price = 100.0;
        writeln!(file, "2020-01-{:02},{},{},{},{},{}", (i % 28) + 1, price, price, price, price, 1000.0).unwrap();
    }
}

fn generate_whipsaw(path: &str) {
    let mut file = File::create(path).unwrap();
    writeln!(file, "Date,Open,High,Low,Close,Volume").unwrap();
    
    let mut close = 100.0;
    for i in 1..=500 {
        let open = close;
        
        // Alternate up and down aggressively
        if i % 2 == 0 {
            close = open + 5.0; // Up day
        } else {
            close = open - 5.0; // Down day
        }

        let high = f64::max(open, close) + 2.0;
        let low = f64::min(open, close) - 2.0;

        writeln!(file, "2020-01-{:02},{},{},{},{},{}", (i % 28) + 1, open, high, low, close, 1000.0).unwrap();
    }
}
