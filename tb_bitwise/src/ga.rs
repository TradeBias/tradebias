use tracing::info;
use crate::engine::BitwiseEngine;
use crate::ast_gen::AstGenerator;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use ahash::AHasher;
use tb_core::ast::Expr;

#[derive(Clone, Debug)]
pub struct Genome {
    /// The mathematical boolean expressions that make up this strategy
    pub conditions: Vec<Expr>,
    pub metrics: crate::metrics::StrategyResult,
}

// ---------------------------------------------------------
// The MAP-Elites Genetic Algorithm
// ---------------------------------------------------------

pub struct GeneticAlgorithm {
    pub engine: Arc<BitwiseEngine>,
    pub population_size: usize,
    pub generations: usize,
    pub direction: tb_core::ast::TradeDirection,
    pub archive: crate::archive::MapArchive,
    pub random_benchmark_percentile: f64,
    pub hash_cache: HashSet<u64>,
    pub ast_gen: AstGenerator,
    
    // Telemetry
    pub time_evaluating_ms: u128,
    pub time_archiving_ms: u128,
    pub time_breeding_ms: u128,
    pub time_deduping_ms: u128,
}

impl GeneticAlgorithm {
    pub fn new(
        engine: Arc<BitwiseEngine>, 
        population_size: usize, 
        generations: usize, 
        direction: tb_core::ast::TradeDirection, 
        archive: crate::archive::MapArchive, 
        random_benchmark_percentile: f64,
        ast_gen: AstGenerator
    ) -> Self {
        Self {
            engine,
            population_size,
            generations,
            direction,
            archive,
            random_benchmark_percentile,
            hash_cache: HashSet::new(),
            ast_gen,
            time_evaluating_ms: 0,
            time_archiving_ms: 0,
            time_breeding_ms: 0,
            time_deduping_ms: 0,
        }
    }

    pub fn run<F>(&mut self, mut progress_callback: F) 
    where F: FnMut(usize, f64)
    {
        info!("Starting MAP-Elites Alpha Generation");
        info!("Population: {}, Generations: {}", self.population_size, self.generations);
        let run_start_time = std::time::Instant::now();
        
        let max_num_rules = self.archive.max_complexity; // max_complexity is mapped to max_num_rules in archive

        // 1. Initialize random population
        let mut current_population = Vec::with_capacity(self.population_size);
        for _ in 0..self.population_size {
            let mut conditions = Vec::new();
            let num_rules = rand::thread_rng().gen_range(1..=max_num_rules.max(1));
            for _ in 0..num_rules {
                conditions.push(self.ast_gen.generate_boolean_trigger());
            }
            current_population.push(Genome { 
                conditions, 
                metrics: crate::metrics::StrategyMetrics::new().finalize(0) 
            });
        }
            
        let dedup_start = std::time::Instant::now();
        let mut current_population = Self::deduplicate_genomes(current_population, &mut self.hash_cache);
        self.time_deduping_ms += dedup_start.elapsed().as_millis();

        let eval_start = std::time::Instant::now();
        let mut current_population: Vec<Genome> = current_population.into_par_iter().map(|mut g| {
            let metrics = match self.direction {
                tb_core::ast::TradeDirection::Long => self.engine.evaluate_long(&g.conditions),
                tb_core::ast::TradeDirection::Short => self.engine.evaluate_short(&g.conditions),
                tb_core::ast::TradeDirection::LongAndShort => self.engine.evaluate_symmetric(&g.conditions),
            };
            g.metrics = metrics;
            g
        }).collect();
        self.time_evaluating_ms += eval_start.elapsed().as_millis();

        // Calculate Random Benchmarking Percentile
        let mut gen0_scores: Vec<f64> = current_population.iter()
            .map(|g| crate::fitness::get_fitness_score(g, &self.archive.fitness_metric, self.archive.occam_penalty_pct))
            .collect();
        gen0_scores.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let p_idx = (gen0_scores.len() as f64 * self.random_benchmark_percentile) as usize;
        let dumb_luck_threshold = if gen0_scores.is_empty() { 0.0 } else { gen0_scores[p_idx.min(gen0_scores.len().saturating_sub(1))] };
        
        info!("Random Benchmarking: {}th Percentile Dumb Luck Threshold = {:.4}", self.random_benchmark_percentile * 100.0, dumb_luck_threshold);

        let arch_start = std::time::Instant::now();
        for g in current_population.clone() {
            if crate::fitness::get_fitness_score(&g, &self.archive.fitness_metric, self.archive.occam_penalty_pct) >= dumb_luck_threshold {
                self.archive.submit(g);
            }
        }
        self.time_archiving_ms += arch_start.elapsed().as_millis();

        // 2. The Evolution Loop
        for generation_idx in 1..=self.generations {
            let start_time = std::time::Instant::now();

            let dedup_start = std::time::Instant::now();
            let novel_population = Self::deduplicate_genomes(current_population, &mut self.hash_cache);
            self.time_deduping_ms += dedup_start.elapsed().as_millis();

            let num_novel = novel_population.len();

            let eval_start = std::time::Instant::now();
            let evaluated: Vec<Genome> = novel_population.into_par_iter().map(|mut g| {
                let metrics = match self.direction {
                    tb_core::ast::TradeDirection::Long => self.engine.evaluate_long(&g.conditions),
                    tb_core::ast::TradeDirection::Short => self.engine.evaluate_short(&g.conditions),
                    tb_core::ast::TradeDirection::LongAndShort => self.engine.evaluate_symmetric(&g.conditions),
                };
                g.metrics = metrics;
                g
            }).collect();
            self.time_evaluating_ms += eval_start.elapsed().as_millis();

            let arch_start = std::time::Instant::now();
            let mut new_kings = 0;
            for g in evaluated {
                let score = crate::fitness::get_fitness_score(&g, &self.archive.fitness_metric, self.archive.occam_penalty_pct);
                if score >= dumb_luck_threshold {
                    if self.archive.submit(g) {
                        new_kings += 1;
                    }
                }
            }

            let kings: Vec<Genome> = self.archive.grid
                .iter()
                .flat_map(|col| col.iter())
                .filter_map(|cell| cell.clone())
                .collect();
            self.time_archiving_ms += arch_start.elapsed().as_millis();

            if kings.is_empty() {
                info!("Generation {}: No valid strategies found.", generation_idx);
                break;
            }

            info!("Gen {} completed in {:?}. Novel Genomes Evaluated: {}. New Kings Found: {}. Total Kings: {}. Cache Size: {}", 
                  generation_idx, start_time.elapsed(), num_novel, new_kings, kings.len(), self.hash_cache.len());

            let breed_start = std::time::Instant::now();
            let mut next_population = Vec::with_capacity(self.population_size);
            for _ in 0..self.population_size {
                let king_idx = rand::thread_rng().gen_range(0..kings.len());
                let mut child = kings[king_idx].clone();
                
                if !child.conditions.is_empty() {
                    let mut rng = rand::thread_rng();
                    if rng.gen_bool(0.2) && child.conditions.len() < max_num_rules {
                        child.conditions.push(self.ast_gen.generate_boolean_trigger());
                    } else if rng.gen_bool(0.2) && child.conditions.len() > 1 {
                        let mutate_idx = rng.gen_range(0..child.conditions.len());
                        child.conditions.remove(mutate_idx);
                    } else {
                        let mutate_idx = rng.gen_range(0..child.conditions.len());
                        child.conditions[mutate_idx] = self.ast_gen.generate_boolean_trigger();
                    }
                } else {
                    child.conditions.push(self.ast_gen.generate_boolean_trigger());
                }
                
                child.metrics = crate::metrics::StrategyMetrics::new().finalize(0);
                next_population.push(child);
            }
            current_population = next_population;
            self.time_breeding_ms += breed_start.elapsed().as_millis();

            progress_callback(generation_idx, run_start_time.elapsed().as_secs_f64());
        }
        
        info!("MAP-Elites Optimization finished.");
    }

    fn deduplicate_genomes(population: Vec<Genome>, hash_cache: &mut HashSet<u64>) -> Vec<Genome> {
        let mut novel_population = Vec::new();
        for mut g in population {
            let mut hashed_conds: Vec<(u64, tb_core::ast::Expr)> = g.conditions.into_iter().map(|c| {
                let mut h = AHasher::default();
                c.hash(&mut h);
                (h.finish(), c)
            }).collect();
            
            hashed_conds.sort_by_key(|(h, _)| *h);
            hashed_conds.dedup_by_key(|(h, _)| *h);
            
            g.conditions = hashed_conds.into_iter().map(|(_, c)| c).collect();
            
            if g.conditions.is_empty() {
                continue;
            }

            let mut hasher = AHasher::default();
            for c in &g.conditions {
                c.hash(&mut hasher);
            }
            let hash = hasher.finish();

            if hash_cache.insert(hash) {
                novel_population.push(g);
            }
        }
        novel_population
    }
}
