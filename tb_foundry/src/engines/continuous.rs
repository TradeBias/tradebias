use polars::prelude::*;
use crossbeam_channel::Sender;
use tb_core::{Phase1Config, Sketch};
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
    info!("Starting Alpha Foundry (Continuous AST Engine): {} Generations, Pop Size: {}", generations, population_size);
    
    // Prepare base data (e.g. forward_returns) and materialize into memory
    let prepared_data = crate::metrics::prepare_data_for_evaluation(data)
        .collect()
        .map_err(|e| FoundryError::Evaluation(format!("Failed to materialize data: {}", e)))?;

    // 1. Initialize seeded population using the Sketch Library
    let mut population: Vec<Sketch> = crate::sketches::generate_seed_population(population_size, config.long_strategy_pct);

    let mutation_engine = crate::ga::MutationEngine::new(0.25, config.permitted_indicators.clone()); // 25% mutation rate
    let mut final_elite: Option<EliteStrategy> = None;
    let mut archive = Archive::new(0.0); // Minimum fitness threshold 0.0
    let mut cumulative_deaths = 0;

    let start_time = std::time::Instant::now();
    let total_bars = prepared_data.height() as f64;

    for gen_idx in 1..=generations {
        info!("--- Generation {} ---", gen_idx);
        
        // 2. Evaluate fitness (Compile AST to Polars Expressions)
        let mut signal_exprs = Vec::with_capacity(population_size);
        let mut valid_sketches = Vec::new();

        for (i, mut sketch) in population.into_iter().enumerate() {
            // Guarantee unique name for Polars column
            sketch.name = format!("gen{}_idx{}", gen_idx, i);
            // Compile AST to Polars Expressions based on direction
            let entry_expr = tb_core::ast_compiler::compile_ast_to_polars(&sketch.entry).map_err(|e| FoundryError::Evaluation(e.to_string()))?;
            let signal_expr = match sketch.direction {
                tb_core::TradeDirection::Long => when(entry_expr).then(lit(1.0)).otherwise(lit(0.0)),
                tb_core::TradeDirection::Short => when(entry_expr).then(lit(-1.0)).otherwise(lit(0.0)),
            };
            signal_exprs.push(signal_expr.alias(&sketch.name));
            valid_sketches.push(sketch);
        }

        // Execute the bulk Polars queries in small chunks to avoid Optimizer explosion
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

        // Fetch the target vector from the prepared data
        let target_series = prepared_data.column("forward_return")
            .map_err(|e| FoundryError::Evaluation(e.to_string()))?.clone();

        let min_trades = config.min_trades as f64;
        let min_exposure = config.min_exposure;
        let max_exposure = config.max_exposure;

        use rayon::prelude::*;
        
        // Phase 1 Matrix * Vector dot product evaluation!
        let evaluated_results: Vec<EvaluatedStrategy> = valid_sketches.into_par_iter()
            .zip(all_signal_series.into_par_iter())
            .filter_map(|(sketch, signal_series)| {
                // Cast boolean signal to f64 mask (0.0 or 1.0)
                let signal_f64 = signal_series.cast(&DataType::Float64).ok()?;
                
                // 1. Calculate trade frequency & exposure first
                let raw_freq = signal_f64.f64().ok()?.sum().unwrap_or(0.0);
                let exposure = raw_freq / total_bars;
                let trade_frequency = exposure * 100.0;
                let indicator_count = sketch.indicator_count();
                let sketch_id = if sketch.name.contains("trend") { 0 }
                                else if sketch.name.contains("meanrev") { 1 }
                                else if sketch.name.contains("macd") { 2 }
                                else { 3 };

                // 2. Enforce Hard Constraints (Death Penalties)
                if raw_freq.is_nan() || exposure.is_nan() || raw_freq < min_trades || exposure < min_exposure || exposure > max_exposure {
                    return Some(EvaluatedStrategy {
                        name: sketch.name.clone(),
                        sketch,
                        fitness: -9999.0, // Death penalty bypasses expensive metrics
                        risk: 0.0,
                        trade_frequency,
                        indicator_count,
                        sketch_id,
                    });
                }

                // 3. Element-wise dot product (Signal * Target)
                let strat_ret = (&signal_f64 * &target_series).ok()?;
                let strat_ret_f64 = strat_ret.f64().ok()?;
                
                let raw_fitness = strat_ret_f64.sum().unwrap_or(0.0) * 100.0;
                
                let num_trades = {
                    let mut count = 0;
                    let mut prev = 0.0;
                    for val in signal_f64.f64().unwrap().into_no_null_iter() {
                        if (val - prev).abs() > 0.1 { count += 1; }
                        prev = val;
                    }
                    (count as f64) / 2.0
                };
                
                let complexity_penalty = (indicator_count as f64) * 200.0;
                let slippage_penalty = num_trades * config.slippage_penalty;
                
                let fitness = raw_fitness - complexity_penalty - slippage_penalty;
                
                // Risk (CPCV Regime Robustness / Variance Across Folds)
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

        // 3. Pareto Sort (NSGA-II 3-Objective Sort)
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

        // 4. Record the Elite (Rank 0)
        if let Some((best_sketch, best_fitness)) = evaluated_pop.first() {
            info!("Generation {} Elite: Fitness {:.4}", gen_idx, best_fitness);
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

        // 5. Build Next Generation (Tournament Selection + MAP-Elites Injection)
        let mut next_generation = Vec::with_capacity(population_size);
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        // Re-inject top archive elites
        let mut archive_elites = archive.elites();
        archive_elites.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        
        let elites_to_inject = (population_size / 10).min(archive_elites.len());
        for i in 0..elites_to_inject {
            next_generation.push(archive_elites[i].sketch.clone());
        }

        // Fill remainder with tournament selection
        let tournament_size = 3;
        let breeding_pool = if archive_elites.len() >= tournament_size {
            archive_elites.iter().map(|e| (e.sketch.clone(), e.fitness)).collect::<Vec<_>>()
        } else {
            evaluated_pop.clone()
        };

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
                // Crossover 
                let mut child = crate::ga::crossover_sketch(&a, &b);
                // Mutate
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
    
    info!("Alpha Foundry optimization complete.");
    final_elite.ok_or_else(|| FoundryError::Evaluation("No valid strategies found to form an Elite".into()))
}
