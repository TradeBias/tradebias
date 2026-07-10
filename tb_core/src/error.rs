use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Failed to serialize or deserialize AST: {0}")]
    AstSerde(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Compilation error: {0}")]
    Compilation(String),
}
