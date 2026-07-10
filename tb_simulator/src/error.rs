use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimulatorError {
    #[error("Simulation engine failed: {0}")]
    Engine(String),

    #[error("Missing data column: {0}")]
    MissingData(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Evaluation error: {0}")]
    Evaluation(String),
}
