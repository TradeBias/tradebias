use polars::prelude::*;
use tb_core::SessionConfig;
use tb_foundry::AlphaFoundry;

fn main() {
    let config = SessionConfig::default().phase1;

    let (tx, _rx) = crossbeam_channel::unbounded();
    let foundry = AlphaFoundry::new(config, tx, None);

    // Create some fake data
    let df = df!(
        "close" => &(0..1000).map(|x| (x as f64) + 10.0).collect::<Vec<_>>(),
        "high" => &(0..1000).map(|x| (x as f64) + 11.0).collect::<Vec<_>>(),
        "low" => &(0..1000).map(|x| (x as f64) + 9.0).collect::<Vec<_>>(),
        "open" => &(0..1000).map(|x| (x as f64) + 10.0).collect::<Vec<_>>(),
        "volume" => &(0..1000).map(|x| (x as f64) + 100.0).collect::<Vec<_>>(),
    ).unwrap().lazy();

    println!("Starting run...");
    let res = foundry.run_generations(50, 100, df);
    println!("Result: {:?}", res);
}
