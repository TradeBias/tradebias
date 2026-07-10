use std::collections::HashMap;
use crate::engine::EvaluatedStrategy;

pub struct Archive {
    /// Map of AST Structural Signature -> Best strategy with that signature
    pub grid: HashMap<String, EvaluatedStrategy>,
    /// Quality threshold: only accept strategies with fitness above this
    pub min_fitness: f64,
}

impl Archive {
    pub fn new(min_fitness: f64) -> Self {
        Self {
            grid: HashMap::new(),
            min_fitness,
        }
    }

    /// Try to add a strategy. Returns true if it was added.
    pub fn try_insert(&mut self, strategy: EvaluatedStrategy) -> bool {
        if strategy.fitness < self.min_fitness {
            return false;
        }

        let cell = strategy.sketch.structural_signature();

        match self.grid.get(&cell) {
            None => {
                // Empty cell - this is a novel behaviour, always accept
                self.grid.insert(cell, strategy);
                true
            }
            Some(existing) if strategy.fitness > existing.fitness => {
                // Better than current occupant - replace
                self.grid.insert(cell, strategy);
                true
            }
            _ => false, // cell occupied by better strategy
        }
    }

    /// Get all elite strategies (one per cell)
    pub fn elites(&self) -> Vec<&EvaluatedStrategy> {
        self.grid.values().collect()
    }
}
