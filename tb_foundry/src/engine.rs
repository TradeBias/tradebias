use crossbeam_channel::Sender;
use polars::prelude::*;
use tb_core::{Sketch, Phase1Config};
use tracing::{info, warn};
use crate::error::FoundryError;
use crate::archive::Archive;

#[derive(Debug, Clone)]
pub struct EvaluatedStrategy {
    pub name: String,
    pub sketch: Sketch,
    pub fitness: f64,
    pub risk: f64,
    pub trade_frequency: f64,
    pub indicator_count: u8,
    pub sketch_id: u8,
}

#[derive(Debug, Clone)]
pub struct GenerationMetrics {
    pub generation: usize,
    pub strategies: Vec<EvaluatedStrategy>, 
    pub total_generated: usize,
    pub total_discarded: usize,
    pub elapsed_seconds: f64,
}

pub use tb_core::ast::EliteStrategy;

pub struct AlphaFoundry {
    config: Phase1Config,
    elite_tx: Sender<EliteStrategy>,
    ui_tx: Option<Sender<GenerationMetrics>>,
}

impl AlphaFoundry {
    pub fn new(config: Phase1Config, elite_tx: Sender<EliteStrategy>, ui_tx: Option<Sender<GenerationMetrics>>) -> Self {
        Self { config, elite_tx, ui_tx }
    }

    /// Runs the entire Generational GA Loop for Phase 1
    pub fn run_generations(&self, generations: usize, population_size: usize, data: LazyFrame) -> Result<EliteStrategy, FoundryError> {
        match self.config.architecture_mode {
            tb_core::config::ArchitectureMode::ContinuousAst => {
                crate::engines::continuous::run(&self.config, generations, population_size, data, &self.elite_tx, &self.ui_tx)
            }
            tb_core::config::ArchitectureMode::DiscretePrecomputed => {
                crate::engines::discrete::run(&self.config, generations, population_size, data, &self.elite_tx, &self.ui_tx)
            }
            tb_core::config::ArchitectureMode::DynamicLazyCache => {
                crate::engines::hybrid::run(&self.config, generations, population_size, data, &self.elite_tx, &self.ui_tx)
            }
        }
    }
}
