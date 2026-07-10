pub mod error;
pub mod ingestion;

pub use error::DataError;
pub use ingestion::{load_parquet, ingest_csv, required_schema, align_timeframes};
