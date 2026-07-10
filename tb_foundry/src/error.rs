use thiserror::Error;

#[derive(Error, Debug)]
pub enum FoundryError {
    #[error("Polars computation error: {0}")]
    Polars(#[from] polars::error::PolarsError),

    #[error("Evaluation error: {0}")]
    Evaluation(String),
}
