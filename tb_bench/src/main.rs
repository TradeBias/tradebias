use clap::Parser;
use tracing::{info, Level};
use std::time::Instant;
use tb_bitwise::data::RawData;
use tb_bitwise::precompute::{ConditionGrid, BaseArray, SemanticType};
use tb_bitwise::targets::TargetOutcomes;
use tb_bitwise::engine::BitwiseEngine;
use tb_bitwise::ga::GeneticAlgorithm;
use tb_bitwise::translator::translate_to_sketch;
use tb_core::ast::TradeDirection;

mod oracle;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The engine to benchmark (e.g., 'polars_continuous', 'polars_discrete', 'bitwise')
    #[arg(short, long, default_value = "bitwise")]
    engine: String,

    /// Whether to run the Oracle Verification tests before benchmarking
    #[arg(long, default_value_t = true)]
    verify_oracle: bool,
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let args = Args::parse();

    info!("Starting Agentic Test Bench (tb_bench)");
    info!("Selected Engine: {}", args.engine);

    if args.verify_oracle {
        info!("Running Mathematical Oracle Verification...");
        match oracle::run_oracle_tests(&args.engine) {
            Ok(_) => info!("Oracle Tests PASSED. Math is locked."),
            Err(e) => {
                tracing::error!("ORACLE FAILURE: {}", e);
                std::process::exit(1);
            }
        }
    }

    info!("Starting Bitwise Matrix Initialization...");
    
    // Load a 500-bar oracle dataset to test the Generator
    let data = RawData::from_csv("tb_bench/oracles/oracle_bull.csv").expect("Failed to load dataset");
    
    // Generate native float arrays
    let sma_10 = tb_math::indicators::sma(&data.close, 10);
    let sma_50 = tb_math::indicators::sma(&data.close, 50);
    let _rsi_14 = tb_math::indicators::rsi(&data.close, 14);
    let _macd = tb_math::indicators::macd(&data.close, 12, 26, 9);
    let bb = tb_math::indicators::bollinger_bands(&data.close, 20, 2.0);

    // Box them into BaseArrays with Semantic Types and AST Expressions
    let mut base_arrays = vec![
        BaseArray { name: "SMA_10".to_string(), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::Sma { source: Box::new(tb_core::ast::Expr::Close), period: 10 }, data: sma_10 },
        BaseArray { name: "SMA_50".to_string(), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::Sma { source: Box::new(tb_core::ast::Expr::Close), period: 50 }, data: sma_50 },
    ];
    base_arrays.push(BaseArray { name: "RSI_14".to_string(), semantic_type: SemanticType::Oscillator, ast: tb_core::ast::Expr::Rsi { source: Box::new(tb_core::ast::Expr::Close), period: 14 }, data: tb_math::indicators::rsi(&data.close, 14) });
    let (macd_line, _sig_line, _hist_line) = tb_math::indicators::macd(&data.close, 12, 26, 9);
    base_arrays.push(BaseArray { name: "MACD".to_string(), semantic_type: SemanticType::Momentum, ast: tb_core::ast::Expr::MacdLine { source: Box::new(tb_core::ast::Expr::Close), fast: 12, slow: 26 }, data: macd_line });
    base_arrays.push(BaseArray { name: "BB_Upper".to_string(), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::Close, data: bb.0 });
    base_arrays.push(BaseArray { name: "BB_Lower".to_string(), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::Close, data: bb.1 });

    // Procedurally generate the Grid
    let mut grid = ConditionGrid::new();
    grid.generate(&base_arrays);
    
    // Print a few sample conditions
    info!("Sample Conditions Generated:");
    for i in 0..std::cmp::min(5, grid.conditions.len()) {
        info!("  - [{}] Hash: {}", grid.conditions[i].name, grid.conditions[i].id);
    }

    // Run the Sparsity Culler (Remove rules triggering <1% or >95% of the time)
    grid.cull_sparsity(data.close.len(), 0.01, 0.95);
    
    // Run the Correlation Culler (Remove anything 95% identical)
    grid.cull_correlated(0.95);

    // Generate Target Outcomes (Fixed 5-bar hold)
    let targets = TargetOutcomes::generate_fixed_hold(&data, 5);

    // Initialize the SIMD Engine
    let arc_grid = std::sync::Arc::new(grid);
    let arc_targets = std::sync::Arc::new(targets);
    let engine = std::sync::Arc::new(BitwiseEngine::new(arc_grid.clone(), arc_targets));

    // Run the MAP-Elites Genetic Algorithm
    let archive = tb_bitwise::archive::MapArchive::new(tb_core::archive::ArchiveTrait::MarketExposure, tb_core::archive::ArchiveTrait::WinRate, tb_core::fitness::FitnessFunction::PnlOverDd, 10, data.close.len() as u32, 5);
    let mut ga = GeneticAlgorithm::new(engine, 5_000, 100, TradeDirection::Long, archive);
    
    let start_eval = Instant::now();
    ga.run();
    let eval_time = start_eval.elapsed();
    
    // Extract Final Kings
    let final_kings: Vec<_> = ga.archive.grid.iter().flat_map(|col| col.iter()).filter_map(|c| c.clone()).collect();
    
    info!("MAP-Elites Evolution completed in {:?}", eval_time);
    info!("Found {} unique Elite Strategies out of {} possible grid buckets.", final_kings.len(), 12);
    
    for (i, king) in final_kings.iter().enumerate() {
        info!("  King {}: Trades = {}, Win Rate = {:.2}%, Rules = {:?}", i, king.total_trades, king.win_rate, king.conditions);
        if let Some(sketch) = translate_to_sketch(king, &arc_grid, TradeDirection::Long) {
            let json = serde_json::to_string_pretty(&sketch).unwrap();
            println!("--- WINNING STRATEGY AST ---\n{}\n----------------------------", json);
        }
    }

    // Output final JSON profiler report
    println!("--- PROFILER REPORT ---");
    let report = format!(r#"{{
  "engine": "bitwise",
  "total_time_ms": {},
  "peak_ram_mb": 0.0,
  "correlation_culls": {},
  "sparsity_culls": 0,
  "elites_found": {}
}}"#, eval_time.as_millis(), arc_grid.conditions.len(), final_kings.len());
    println!("{}", report);
    println!("-----------------------");
}

#[derive(serde::Serialize)]
struct ProfilerReport {
    engine: String,
    strategies_per_second: f64,
    peak_ram_mb: f64,
    correlation_culls: usize,
    sparsity_culls: usize,
}
