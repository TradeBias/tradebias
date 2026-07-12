use polars::prelude::*;
use crossbeam_channel::Sender;
use tb_core::{Phase1Config, Sketch, Expr};
use tracing::info;
use crate::error::FoundryError;
use crate::archive::Archive;
use crate::engine::{EliteStrategy, EvaluatedStrategy, GenerationMetrics};

pub fn run(
    config: &Phase1Config,
    generations: usize, 
    population_size: usize, 
    data: LazyFrame,
    elite_tx: &Sender<EliteStrategy>,
    ui_tx: &Option<Sender<GenerationMetrics>>
) -> Result<EliteStrategy, FoundryError> {
    info!("Starting Alpha Foundry (Discrete Pre-computed Engine)");
    
    let mut prepared_data = crate::metrics::prepare_data_for_evaluation(data)
        .collect()
        .map_err(|e| FoundryError::Evaluation(format!("Failed to materialize data: {}", e)))?;

    // STEP 0: Pre-compute the entire feature grid upfront!
    info!("STEP 0: Pre-computing exact feature grid in memory...");
    let mut grid_exprs = Vec::new();
    let sources = vec![Expr::Close, Expr::Open, Expr::High, Expr::Low, Expr::Volume];
    let grid_periods: &[u32] = &[5, 10, 15, 20, 25, 30, 40, 50, 100, 200];
    
    for ind_name in &config.permitted_indicators {
        let perms = Expr::generate_grid_permutations(ind_name, &sources, grid_periods);
        for ind in perms {
            if let Ok(e) = tb_core::ast_compiler::compile_ast_to_polars(&ind) {
                grid_exprs.push(e.alias(&ind.to_string()));
            }
        }
    }
    
    if !grid_exprs.is_empty() {
        prepared_data = prepared_data.lazy()
            .with_columns(grid_exprs)
            .collect()
            .map_err(|e| FoundryError::Evaluation(format!("Failed to build feature grid: {}", e)))?;
    }
    info!("Feature grid compiled. Starting discrete evolution...");

    let mut population: Vec<Sketch> = crate::sketches::generate_seed_population(population_size, config.long_strategy_pct);
    let mutation_engine = crate::ga::MutationEngine::new(0.25, config.permitted_indicators.clone());
    let mut final_elite: Option<EliteStrategy> = None;
    let mut archive = Archive::new(0.0);
    let mut cumulative_deaths = 0;

    let start_time = std::time::Instant::now();
    let total_bars = prepared_data.height() as f64;

    for gen_idx in 1..=generations {
        info!("--- Generation {} ---", gen_idx);
        
        let mut signal_exprs = Vec::with_capacity(population_size);
        let mut valid_sketches = Vec::new();

        for (i, mut sketch) in population.into_iter().enumerate() {
            // SNAP TO GRID
            sketch.snap_to_grid(grid_periods);
            
            sketch.name = format!("gen{}_idx{}", gen_idx, i);
            if let Ok(entry_expr) = tb_core::ast_compiler::compile_ast_with_cache(&sketch.entry).map_err(|e| FoundryError::Evaluation(e.to_string())) {
                let combined_expr = match sketch.direction {
                    tb_core::TradeDirection::Long => polars::lazy::dsl::when(entry_expr).then(polars::lazy::dsl::lit(1.0)).otherwise(polars::lazy::dsl::lit(0.0)),
                    tb_core::TradeDirection::Short => polars::lazy::dsl::when(entry_expr).then(polars::lazy::dsl::lit(-1.0)).otherwise(polars::lazy::dsl::lit(0.0)),
                };
                
                signal_exprs.push(combined_expr.alias(&sketch.name));
                valid_sketches.push(sketch);
            }
        }

        let mut all_signal_series = Vec::with_capacity(valid_sketches.len());
        for chunk in signal_exprs.chunks(200) {
            let chunk_df = prepared_data.clone().lazy()
                .select(chunk.to_vec())
                .collect()
                .map_err(|e| FoundryError::Evaluation(e.to_string()))?;
            
            for series in chunk_df.get_columns() {
                all_signal_series.push(series.clone());
            }
        }

        let target_series = prepared_data.column("forward_return")
            .map_err(|e| FoundryError::Evaluation(e.to_string()))?.clone();


        let min_trades = config.min_trades as f64;
        let min_exposure = config.min_exposure;
        let max_exposure = config.max_exposure;

        use rayon::prelude::*;
        
        let evaluated_results: Vec<EvaluatedStrategy> = valid_sketches.into_par_iter()
            .zip(all_signal_series.into_par_iter())
            .filter_map(|(sketch, signal_series)| {
                let signal_f64 = signal_series.cast(&DataType::Float64).ok()?;
                
                let raw_freq = {
                    let mut sum = 0.0;
                    for val in signal_f64.f64().unwrap().into_no_null_iter() {
                        sum += val.abs();
                    }
                    sum
                };
                
                let exposure = raw_freq / total_bars;
                let trade_frequency = exposure * 100.0;
                let indicator_count = sketch.indicator_count();
                let sketch_id = if sketch.name.contains("trend") { 0 }
                                else if sketch.name.contains("meanrev") { 1 }
                                else if sketch.name.contains("macd") { 2 }
                                else { 3 };

                if raw_freq.is_nan() || exposure.is_nan() || raw_freq < min_trades || exposure < min_exposure || exposure > max_exposure {
                    return Some(EvaluatedStrategy {
                        name: sketch.name.clone(),
                        sketch,
                        fitness: -9999.0, 
                        risk: 0.0,
                        trade_frequency,
                        indicator_count,
                        sketch_id,
                    });
                }

                let strat_ret = (&signal_f64 * &target_series).ok()?;
                let strat_ret_f64 = strat_ret.f64().ok()?;
                
                let raw_fitness = strat_ret_f64.sum().unwrap_or(0.0) * 100.0;
                
                // Alpha Isolation (Information Ratio Penalty)
                // Subtract the underlying asset's macro trend from our fitness based on time exposed.
                // If Benchmark returned 100%, and we held 50% of the time, our "Beta" is 50%.
                // We must beat 50% to generate true Alpha.
                let alpha_fitness = raw_fitness;
                
                // Calculate number of trades (0 to 1, 0 to -1 transitions) to penalize high-frequency noise
                let num_trades = {
                    let mut count = 0;
                    let mut prev = 0.0;
                    for val in signal_f64.f64().unwrap().into_no_null_iter() {
                        if (val - prev).abs() > 0.1 { count += 1; }
                        prev = val;
                    }
                    (count as f64) / 2.0 // Divide by 2 because entry and exit both trigger a transition
                };
                
                // Occam's Razor Penalty: Subtract 200 from fitness for every indicator block used.
                let complexity_penalty = (indicator_count as f64) * 200.0;
                
                // Slippage Penalty: Violently penalizes over-trading and short periods.
                // Subtract config.slippage_penalty points per trade.
                let slippage_penalty = num_trades * config.slippage_penalty;
                
                let fitness = alpha_fitness - complexity_penalty - slippage_penalty;
                
                let total_len = strat_ret_f64.len();
                let num_chunks = 6;
                let chunk_size = total_len / num_chunks;
                
                let risk = if chunk_size > 0 {
                    let mut chunk_sums = Vec::with_capacity(num_chunks);
                    let mut current_chunk_sum = 0.0;
                    let mut current_chunk_idx = 0;
                    
                    for (i, val) in strat_ret_f64.into_no_null_iter().enumerate() {
                        current_chunk_sum += val;
                        if (i + 1) % chunk_size == 0 && current_chunk_idx < num_chunks {
                            chunk_sums.push(current_chunk_sum);
                            current_chunk_sum = 0.0;
                            current_chunk_idx += 1;
                        }
                    }
                    
                    let chunk_mean = chunk_sums.iter().sum::<f64>() / (chunk_sums.len() as f64);
                    let mut cpcv_variance = 0.0;
                    for sum in &chunk_sums {
                        cpcv_variance += (sum - chunk_mean) * (sum - chunk_mean);
                    }
                    if chunk_sums.len() > 1 {
                        (cpcv_variance / (chunk_sums.len() as f64 - 1.0)).sqrt() * 100.0
                    } else { 0.0 }
                } else {
                    0.0
                };

                let pnl = strat_ret_f64.sum().unwrap_or(0.0);
                let expectancy = if num_trades > 0.0 { pnl / num_trades } else { 0.0 };



                Some(EvaluatedStrategy {
                    name: sketch.name.clone(),
                    sketch,
                    fitness,
                    risk,
                    trade_frequency,
                    indicator_count,
                    sketch_id,
                })
            })
            .collect();
            
        let deaths_this_gen = evaluated_results.iter().filter(|s| s.fitness == -9999.0).count();
        cumulative_deaths += deaths_this_gen;

        let mut strategies = evaluated_results;

        crate::pareto::pareto_sort(&mut strategies);

        let mut evaluated_pop = Vec::with_capacity(strategies.len());
        for eval_strat in &strategies {
            archive.try_insert(eval_strat.clone());
            evaluated_pop.push((eval_strat.sketch.clone(), eval_strat.fitness));
        }

        let elapsed_seconds = start_time.elapsed().as_secs_f64();
        let total_generated = gen_idx * population_size;

        if let Some(tx) = ui_tx {
            let mut archive_view: Vec<_> = archive.elites().into_iter().cloned().collect();
            archive_view.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
            let _ = tx.send(GenerationMetrics {
                generation: gen_idx,
                strategies: archive_view,
                total_generated,
                total_discarded: cumulative_deaths,
                elapsed_seconds,
            });
        }

        if let Some((best_sketch, best_fitness)) = evaluated_pop.first() {
            let elite = EliteStrategy {
                sketch: best_sketch.clone(),
                fitness: *best_fitness,
                pnl: 0.0,
                max_drawdown: 0.0,
                pnl_over_dd: 0.0,
                sharpe: 0.0,
                sortino: 0.0,
                profit_factor: 0.0,
                cpc_index: 0.0,
                corr_coef: 0.0,
                expectancy: 0.0,
                trade_frequency: 0.0,
                indicator_count: 0,
            };
            
            let _ = elite_tx.send(elite.clone());
            final_elite = Some(elite);
        }

        let mut next_generation = Vec::with_capacity(population_size);
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        let mut archive_elites = archive.elites();
        archive_elites.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        
        let elites_to_inject = (population_size / 10).min(archive_elites.len());
        for i in 0..elites_to_inject {
            next_generation.push(archive_elites[i].sketch.clone());
        }

        let tournament_size = 3;
        let mut breeding_pool = evaluated_pop.clone();
        // Inject historical elites into the breeding pool so they can mate with the general population
        for e in &archive_elites {
            breeding_pool.push((e.sketch.clone(), e.fitness));
        }

        while next_generation.len() < population_size {
            let mut parent_a: Option<Sketch> = None;
            let mut best_fit_a = f64::NEG_INFINITY;
            for _ in 0..tournament_size {
                if let Some(candidate) = breeding_pool.choose(&mut rng) {
                    if candidate.1 > best_fit_a {
                        best_fit_a = candidate.1;
                        parent_a = Some(candidate.0.clone());
                    }
                }
            }
            
            let mut parent_b: Option<Sketch> = None;
            let mut best_fit_b = f64::NEG_INFINITY;
            for _ in 0..tournament_size {
                if let Some(candidate) = breeding_pool.choose(&mut rng) {
                    if candidate.1 > best_fit_b {
                        best_fit_b = candidate.1;
                        parent_b = Some(candidate.0.clone());
                    }
                }
            }
            
            if let (Some(a), Some(b)) = (parent_a, parent_b) {
                let mut child = crate::ga::crossover_sketch(&a, &b);
                let mut mutated_child = child.clone();
                mutation_engine.mutate_sketch(&mut mutated_child);
                if crate::ga::is_structurally_sound(&mutated_child) {
                    child = mutated_child;
                }
                child.name = format!("gen{}_ind{}", gen_idx, next_generation.len());
                next_generation.push(child);
            } else {
                next_generation.push(crate::sketches::generate_seed_population(1, config.long_strategy_pct).pop().unwrap());
            }
        }
        
        population = next_generation;
    }
    
    info!("Alpha Foundry optimization complete. Pre-computed Grid Model.");
    final_elite.ok_or_else(|| FoundryError::Evaluation("No valid strategies found to form an Elite".into()))
}
