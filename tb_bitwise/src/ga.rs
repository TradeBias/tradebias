use tracing::info;
use crate::engine::BitwiseEngine;
use rand::Rng;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Genome {
    /// The indexes of the conditions in the ConditionGrid that make up this strategy
    pub conditions: Vec<usize>,
    pub metrics: crate::metrics::StrategyResult,
}

// ---------------------------------------------------------
// The MAP-Elites Genetic Algorithm
// ---------------------------------------------------------

use std::sync::Arc;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct GeneticAlgorithm {
    pub engine: Arc<BitwiseEngine>,
    pub population_size: usize,
    pub generations: usize,
    pub direction: tb_core::ast::TradeDirection,
    pub archive: crate::archive::MapArchive,
    pub random_benchmark_percentile: f64,
    pub hash_cache: HashSet<u64>,
}

impl GeneticAlgorithm {
    pub fn new(engine: Arc<BitwiseEngine>, population_size: usize, generations: usize, direction: tb_core::ast::TradeDirection, archive: crate::archive::MapArchive, random_benchmark_percentile: f64) -> Self {
        Self {
            engine,
            population_size,
            generations,
            direction,
            archive,
            random_benchmark_percentile,
            hash_cache: HashSet::new(),
        }
    }

    pub fn run(&mut self) {
        info!("Starting MAP-Elites Alpha Generation");
        info!("Population: {}, Generations: {}", self.population_size, self.generations);
        
        let num_conditions = self.engine.grid.conditions.len();
        if num_conditions == 0 {
            info!("No conditions available in grid. Aborting GA.");
            return;
        }

        // 1. Initialize random population
        let mut current_population = Vec::with_capacity(self.population_size);
        for _ in 0..self.population_size {
            let mut conditions = Vec::new();
            let num_rules = rand::thread_rng().gen_range(2..=5); // Strategy complexity
            for _ in 0..num_rules {
                conditions.push(rand::thread_rng().gen_range(0..num_conditions));
            }
            current_population.push(Genome { 
                conditions, 
                metrics: crate::metrics::StrategyMetrics::new().finalize(0) 
            });
        }
            
        // 1.5 Evaluate Initial Random Population (Generation 0) and Calculate Dumb Luck Threshold
        let mut current_population: Vec<Genome> = current_population.into_par_iter().map(|mut g| {
            let metrics = match self.direction {
                tb_core::ast::TradeDirection::Long => self.engine.evaluate_long(&g.conditions),
                tb_core::ast::TradeDirection::Short => self.engine.evaluate_short(&g.conditions),
                tb_core::ast::TradeDirection::LongAndShort => self.engine.evaluate_symmetric(&g.conditions),
            };
            g.metrics = metrics;
            g
        }).collect();

        // Calculate Random Benchmarking Percentile
        let mut gen0_scores: Vec<f64> = current_population.iter()
            .map(|g| crate::fitness::get_fitness_score(g, &self.archive.fitness_metric, self.archive.occam_penalty_pct))
            .collect();
        gen0_scores.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let p_idx = (gen0_scores.len() as f64 * self.random_benchmark_percentile) as usize;
        let dumb_luck_threshold = if gen0_scores.is_empty() { 0.0 } else { gen0_scores[p_idx.min(gen0_scores.len().saturating_sub(1))] };
        
        info!("Random Benchmarking: {}th Percentile Dumb Luck Threshold = {:.4}", self.random_benchmark_percentile * 100.0, dumb_luck_threshold);

        // Submit Gen 0 to archive
        for g in current_population.clone() {
            if crate::fitness::get_fitness_score(&g, &self.archive.fitness_metric, self.archive.occam_penalty_pct) >= dumb_luck_threshold {
                self.archive.submit(g);
            }
        }

        // 2. The Evolution Loop
        for generation_idx in 1..=self.generations {
            let start_time = std::time::Instant::now();

            // Filter out already evaluated genomes using Zobrist Hash Cache
            let mut novel_population = Vec::new();
            for mut g in current_population {
                g.conditions.sort_unstable();
                g.conditions.dedup();
                
                if g.conditions.is_empty() {
                    continue;
                }

                let mut hasher = DefaultHasher::new();
                g.conditions.hash(&mut hasher);
                let hash = hasher.finish();

                if self.hash_cache.insert(hash) {
                    novel_population.push(g);
                }
            }

            let num_novel = novel_population.len();

            // Evaluate only the novel genomes in parallel
            let evaluated: Vec<Genome> = novel_population.into_par_iter().map(|mut g| {
                let metrics = match self.direction {
                    tb_core::ast::TradeDirection::Long => self.engine.evaluate_long(&g.conditions),
                    tb_core::ast::TradeDirection::Short => self.engine.evaluate_short(&g.conditions),
                    tb_core::ast::TradeDirection::LongAndShort => self.engine.evaluate_symmetric(&g.conditions),
                };
                g.metrics = metrics;
                g
            }).collect();

            // Submit evaluated genomes to the MAP-Elites Archive
            let mut new_kings = 0;
            for g in evaluated {
                let score = crate::fitness::get_fitness_score(&g, &self.archive.fitness_metric, self.archive.occam_penalty_pct);
                if score >= dumb_luck_threshold {
                    if self.archive.submit(g) {
                        new_kings += 1;
                    }
                }
            }

            // Extract the current Kings from the Archive to act as parents
            let kings: Vec<Genome> = self.archive.grid
                .iter()
                .flat_map(|col| col.iter())
                .filter_map(|cell| cell.clone())
                .collect();

            if kings.is_empty() {
                info!("Generation {}: No valid strategies found.", generation_idx);
                break;
            }

            info!("Gen {} completed in {:?}. Novel Genomes Evaluated: {}. New Kings Found: {}. Total Kings: {}. Cache Size: {}", 
                  generation_idx, start_time.elapsed(), num_novel, new_kings, kings.len(), self.hash_cache.len());

            // Generate the next population by breeding the Kings
            let mut next_population = Vec::with_capacity(self.population_size);
            for _ in 0..self.population_size {
                let king_idx = rand::thread_rng().gen_range(0..kings.len());
                let mut child = kings[king_idx].clone();
                
                // Mutation: Randomly replace one condition in the strategy
                if !child.conditions.is_empty() {
                    let mutate_idx = rand::thread_rng().gen_range(0..child.conditions.len());
                    child.conditions[mutate_idx] = rand::thread_rng().gen_range(0..num_conditions);
                }
                
                // Reset stats for the child
                child.metrics = crate::metrics::StrategyMetrics::new().finalize(0);
                next_population.push(child);
            }

            current_population = next_population;
        }
        
        info!("MAP-Elites Optimization finished.");
    }
}
