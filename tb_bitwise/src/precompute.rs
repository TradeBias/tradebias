use tracing::info;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use rayon::prelude::*;
use crate::data::RawData;
use tb_core::ast::Expr;

/// Defines the semantic category of an indicator to prevent illogical comparisons.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticType {
    Price,
    Oscillator,
    Volume,
    Momentum, // e.g., MACD Histogram
    Pattern,  // Binary float array (1.0 = true, 0.0 = false)
}

/// A pre-computed float array for a specific indicator.
pub struct BaseArray {
    pub name: String,
    pub semantic_type: SemanticType,
    pub ast: Expr,
    pub data: Vec<f64>,
}

/// Represents a single boolean condition evaluated across the entire dataset.
/// Bits are packed into `u64` integers (64 bars per integer).
pub struct BitsetCondition {
    pub id: u64, // Zobrist hash derived from the condition name
    pub name: String,
    pub ast: Expr,
    pub bits: Vec<u64>,
}

impl BitsetCondition {
    pub fn new(name: String, ast: Expr, bits: Vec<u64>) -> Self {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let id = hasher.finish();
        Self { id, name, ast, bits }
    }
}

/// The massive grid of all possible pre-computed indicator logic.
pub struct ConditionGrid {
    pub conditions: Vec<BitsetCondition>,
}

impl ConditionGrid {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// Procedurally generates bitsets based on semantic typing.
    /// This prevents mathematical garbage like `RSI > Close`.
    pub fn generate(&mut self, base_arrays: &[BaseArray]) {
        info!("Starting procedural bitset generation...");
        let start_time = std::time::Instant::now();
        
        let mut generated_conditions: Vec<BitsetCondition> = base_arrays.par_iter().enumerate().flat_map(|(i, array_a)| {
            let mut local_conditions = Vec::new();
            
            // 0. Pattern checks (Direct truth value)
            if array_a.semantic_type == SemanticType::Pattern {
                local_conditions.push(BitsetCondition::new(
                    format!("{} == True", array_a.name),
                    Expr::GreaterThan { lhs: Box::new(array_a.ast.clone()), rhs: Box::new(Expr::Constant { value: 0.5 }) },
                    build_greater_than_constant(&array_a.data, 0.5),
                ));
                return local_conditions;
            }

            // 1. Threshold checks (Oscillators against constants)
            if array_a.semantic_type == SemanticType::Oscillator {
                for threshold in [20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0] {
                    local_conditions.push(BitsetCondition::new(
                        format!("{} < {}", array_a.name, threshold),
                        Expr::LessThan { lhs: Box::new(array_a.ast.clone()), rhs: Box::new(Expr::Constant { value: threshold }) },
                        build_less_than_constant(&array_a.data, threshold),
                    ));
                    local_conditions.push(BitsetCondition::new(
                        format!("{} > {}", array_a.name, threshold),
                        Expr::GreaterThan { lhs: Box::new(array_a.ast.clone()), rhs: Box::new(Expr::Constant { value: threshold }) },
                        build_greater_than_constant(&array_a.data, threshold),
                    ));
                }
            }

            // 2. Crosses between identical semantic types (e.g. Price vs Price)
            for j in (i + 1)..base_arrays.len() {
                let array_b = &base_arrays[j];

                if array_a.semantic_type == array_b.semantic_type {
                    local_conditions.push(BitsetCondition::new(
                        format!("{} > {}", array_a.name, array_b.name),
                        Expr::GreaterThan { lhs: Box::new(array_a.ast.clone()), rhs: Box::new(array_b.ast.clone()) },
                        build_greater_than_array(&array_a.data, &array_b.data),
                    ));
                    local_conditions.push(BitsetCondition::new(
                        format!("{} < {}", array_a.name, array_b.name),
                        Expr::LessThan { lhs: Box::new(array_a.ast.clone()), rhs: Box::new(array_b.ast.clone()) },
                        build_less_than_array(&array_a.data, &array_b.data),
                    ));
                }
            }
            local_conditions
        }).collect();

        let count = generated_conditions.len();
        self.conditions.append(&mut generated_conditions);

        let elapsed = start_time.elapsed();
        info!("Successfully generated {} semantic bitset conditions in {:?}", count, elapsed);
    }

    /// Identifies and deletes conditions that are highly correlated (e.g., > 95% identical signals)
    /// This prevents premature convergence in the Genetic Algorithm.
    pub fn cull_correlated(&mut self, max_similarity: f64) {
        info!("Starting Bitwise Correlation Culling (Threshold: {:.1}%)...", max_similarity * 100.0);
        let start_time = std::time::Instant::now();
        
        let num_conditions = self.conditions.len();
        // Parallel O(N^2) comparison loop using Rayon
        let to_remove: std::collections::HashSet<usize> = (0..num_conditions)
            .into_par_iter()
            .flat_map(|i| {
                let mut local_removals = Vec::new();
                let bits_a = &self.conditions[i].bits;
                let len = bits_a.len();
                let total_bars = (len * 64) as f64;
                
                for j in (i + 1)..num_conditions {
                    let bits_b = &self.conditions[j].bits;
                    
                    // Fast bitwise identical check
                    let diff_bits: u32 = bits_a.iter().zip(bits_b.iter()).map(|(a, b)| (a ^ b).count_ones()).sum();
                    let identical_bits = (len * 64) as u32 - diff_bits;
                    
                    let similarity = identical_bits as f64 / total_bars;
                    
                    if similarity >= max_similarity {
                        local_removals.push(j);
                    }
                }
                local_removals
            })
            .collect();
        
        let original_count = self.conditions.len();
        self.conditions = std::mem::take(&mut self.conditions)
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| !to_remove.contains(idx))
            .map(|(_, val)| val)
            .collect();
            
        let culled = original_count - self.conditions.len();
        info!("Culling complete in {:?}. Removed {} redundant conditions. Remaining: {}", start_time.elapsed(), culled, self.conditions.len());
    }

    /// Identifies and deletes conditions that trigger too rarely or too often.
    /// This removes statistically insignificant edges and absolute truths (e.g. Close > 0).
    pub fn cull_sparsity(&mut self, total_bars: usize, min_trigger_pct: f64, max_trigger_pct: f64) {
        info!("Starting Sparsity Culling (Min: {:.1}%, Max: {:.1}%)...", min_trigger_pct * 100.0, max_trigger_pct * 100.0);
        let start_time = std::time::Instant::now();
        
        let min_triggers = (total_bars as f64 * min_trigger_pct) as u32;
        let max_triggers = (total_bars as f64 * max_trigger_pct) as u32;

        let original_count = self.conditions.len();
        
        self.conditions.retain(|cond| {
            let mut triggers = 0;
            for &block in &cond.bits {
                triggers += block.count_ones();
            }
            triggers >= min_triggers && triggers <= max_triggers
        });

        let culled = original_count - self.conditions.len();
        info!("Sparsity Culling complete in {:?}. Removed {} sparse/dense conditions. Remaining: {}", start_time.elapsed(), culled, self.conditions.len());
    }
}

// =====================================================================
// SIMD/Bitwise Packing Helpers
// Packs true/false outcomes into 64-bit integers.
// =====================================================================

fn build_less_than_constant(data: &[f64], threshold: f64) -> Vec<u64> {
    let num_blocks = (data.len() + 63) / 64;
    let mut bitset = vec![0u64; num_blocks];
    for (i, &val) in data.iter().enumerate() {
        if !val.is_nan() && val < threshold {
            bitset[i / 64] |= 1 << (i % 64);
        }
    }
    bitset
}

fn build_greater_than_constant(data: &[f64], threshold: f64) -> Vec<u64> {
    let num_blocks = (data.len() + 63) / 64;
    let mut bitset = vec![0u64; num_blocks];
    for (i, &val) in data.iter().enumerate() {
        if !val.is_nan() && val > threshold {
            bitset[i / 64] |= 1 << (i % 64);
        }
    }
    bitset
}

fn build_greater_than_array(data_a: &[f64], data_b: &[f64]) -> Vec<u64> {
    let num_blocks = (data_a.len() + 63) / 64;
    let mut bitset = vec![0u64; num_blocks];
    for i in 0..data_a.len() {
        let val_a = data_a[i];
        let val_b = data_b[i];
        if !val_a.is_nan() && !val_b.is_nan() && val_a > val_b {
            bitset[i / 64] |= 1 << (i % 64);
        }
    }
    bitset
}

fn build_less_than_array(data_a: &[f64], data_b: &[f64]) -> Vec<u64> {
    let num_blocks = (data_a.len() + 63) / 64;
    let mut bitset = vec![0u64; num_blocks];
    for i in 0..data_a.len() {
        let val_a = data_a[i];
        let val_b = data_b[i];
        if !val_a.is_nan() && !val_b.is_nan() && val_a < val_b {
            bitset[i / 64] |= 1 << (i % 64);
        }
    }
    bitset
}
