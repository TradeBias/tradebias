use std::sync::Arc;
use tb_bitwise::data::RawData;
use tb_bitwise::precompute::EngineCache;
use tb_bitwise::targets::TargetOutcomes;
use tb_bitwise::engine::BitwiseEngine;
use tb_bitwise::ast_gen::AstGenerator;
use tb_bitwise::ga::GeneticAlgorithm;
use tb_core::ast::TradeDirection;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct TelemetryDump {
    total_cache_hits: u64,
    total_cache_misses: u64,
    hit_rate_percent: f64,
    generation_time_ms: u64,
    pipeline_evaluating_ms: u128,
    pipeline_archiving_ms: u128,
    pipeline_breeding_ms: u128,
    pipeline_deduping_ms: u128,
    nodes: HashMap<String, NodeTelemetry>,
}

#[derive(Serialize)]
struct NodeTelemetry {
    calls: u64,
    total_time_ms: f64,
}

fn main() {
    println!("Loading 10,000 bars of dummy data for benchmark...");
    let data_len = 10_000;
    
    // Create dummy raw data
    let mut close = Vec::with_capacity(data_len);
    let mut open = Vec::with_capacity(data_len);
    let mut high = Vec::with_capacity(data_len);
    let mut low = Vec::with_capacity(data_len);
    let mut volume = Vec::with_capacity(data_len);
    
    for i in 0..data_len {
        let f = i as f64;
        close.push(100.0 + (f * 0.01).sin() * 10.0);
        open.push(100.0 + (f * 0.01).sin() * 10.0 - 1.0);
        high.push(100.0 + (f * 0.01).sin() * 10.0 + 2.0);
        low.push(100.0 + (f * 0.01).sin() * 10.0 - 2.0);
        volume.push(1000.0 + (f * 0.05).cos() * 100.0);
    }
    
    let raw_data = RawData {
        close, open, high, low, volume,
    };
    
    let periods = vec![5, 10, 14, 20, 50, 100, 200];
    
    println!("Building EngineCache...");
    let precompute_start = std::time::Instant::now();
    let cache = Arc::new(EngineCache::new(&raw_data, &periods));
    println!("EngineCache Built in {:?}", precompute_start.elapsed());
    
    // 1. Setup AST Generator
    let periods = [7, 14, 21, 50, 100, 200];
    let indicators = tb_indicators::templates::default_blueprints();
    let ast_gen = AstGenerator::new(5, &periods, &indicators); // max_depth 5
    let pop_size = 5000;
    
    println!("Building TargetOutcomes...");
    let targets_start = std::time::Instant::now();
    let targets = TargetOutcomes::generate_advanced_stop(
        &raw_data, 
        &tb_core::stops::StopType::FixedBarHold { bars: 5 },
        &tb_core::stops::TakeProfit::None,
        None,
        0.0
    );
    let targets = Arc::new(targets);
    println!("TargetOutcomes Built in {:?}", targets_start.elapsed());
    
    let engine = Arc::new(BitwiseEngine::new(cache, targets));
    
    let archive = tb_bitwise::archive::MapArchive::new(
        tb_core::archive::ArchiveTrait::WinRate,
        tb_core::archive::ArchiveTrait::MarketExposure,
        tb_core::fitness::FitnessFunction::ProfitFactor,
        20,
        data_len as u32,
        10,
        1,
        100.0,
        0.0
    );
    
    let mut ga = GeneticAlgorithm::new(
        engine.clone(),
        pop_size,
        1, // We just want to run 1 generation
        TradeDirection::Long,
        archive,
        0.0, // Benchmark percentile
        ast_gen
    );
    
    println!("Running GA Benchmark (1 Generation, Pop: {})...", pop_size);
    let start = std::time::Instant::now();
    ga.run(|_, _| {});
    let gen_time_ms = start.elapsed().as_millis() as u64;
    println!("Benchmark completed in {} ms", gen_time_ms);
    
    let hits = engine.cache_hits.load(std::sync::atomic::Ordering::Relaxed);
    let misses = engine.cache_misses.load(std::sync::atomic::Ordering::Relaxed);
    let total = hits + misses;
    let hit_rate = if total > 0 { (hits as f64 / total as f64) * 100.0 } else { 0.0 };
    
    println!("JIT Telemetry: {} Hits, {} Misses (Hit Rate: {:.2}%)", hits, misses, hit_rate);
    
    // Dump JSON Telemetry
    let mut nodes_map = HashMap::new();
    for entry in engine.telemetry.iter() {
        let name = entry.key().to_string();
        let record = entry.value();
        let calls = record.calls.load(std::sync::atomic::Ordering::Relaxed);
        let time_us = record.total_time_us.load(std::sync::atomic::Ordering::Relaxed);
        let time_ms = time_us as f64 / 1000.0;
        
        nodes_map.insert(name, NodeTelemetry { calls, total_time_ms: time_ms });
    }
    
    let dump = TelemetryDump {
        total_cache_hits: hits,
        total_cache_misses: misses,
        hit_rate_percent: hit_rate,
        generation_time_ms: gen_time_ms,
        pipeline_evaluating_ms: ga.time_evaluating_ms,
        pipeline_archiving_ms: ga.time_archiving_ms,
        pipeline_breeding_ms: ga.time_breeding_ms,
        pipeline_deduping_ms: ga.time_deduping_ms,
        nodes: nodes_map,
    };
    
    let json = serde_json::to_string_pretty(&dump).unwrap();
    std::fs::write("benchmark_results.json", json).unwrap();
    println!("Wrote granular node execution telemetry to benchmark_results.json");
}
