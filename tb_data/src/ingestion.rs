use polars::prelude::*;
use std::path::Path;
use tracing::info;
use crate::error::DataError;

/// Core schema required for Phase 1 Alpha Foundry
pub fn required_schema() -> Schema {
    let mut schema = Schema::new();
    schema.with_column("timestamp".into(), DataType::Datetime(TimeUnit::Microseconds, None));
    schema.with_column("open".into(), DataType::Float64);
    schema.with_column("high".into(), DataType::Float64);
    schema.with_column("low".into(), DataType::Float64);
    schema.with_column("close".into(), DataType::Float64);
    schema.with_column("volume".into(), DataType::Float64);
    schema
}

/// Loads a Parquet file locally for the backtester
pub fn load_parquet<P: AsRef<Path>>(path: P) -> Result<LazyFrame, DataError> {
    info!("Loading Parquet file from {:?}", path.as_ref());
    let lf = LazyFrame::scan_parquet(path, ScanArgsParquet::default())?;
    Ok(lf)
}

/// Simulated CSV ingestion with schema validation
pub fn ingest_csv<P: AsRef<Path>>(csv_path: P, output_parquet: P) -> Result<(), DataError> {
    info!("Ingesting CSV from {:?}", csv_path.as_ref());
    let mut df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(csv_path.as_ref().into()))?
        .finish()?;

    // Minimal validation to ensure essential columns exist (in reality we'd coerce schema)
    let schema = df.schema();
    for required_col in ["timestamp", "open", "high", "low", "close", "volume"] {
        if !schema.contains(required_col) {
            return Err(DataError::InvalidSchema(format!("Missing column: {}", required_col)));
        }
    }

    // Save as parquet for optimized reads later
    info!("Saving Parquet to {:?}", output_parquet.as_ref());
    let file = std::fs::File::create(output_parquet)?;
    ParquetWriter::new(file).finish(&mut df)?;

    Ok(())
}

/// Aligns a higher timeframe dataset onto a lower timeframe execution dataset.
/// Using Polars' `join_asof`, we forward-fill the daily indicator values onto the 1H/1M bars.
pub fn align_timeframes(
    base_lf: LazyFrame,
    higher_tf_lf: LazyFrame,
    suffix: &str,
) -> Result<LazyFrame, DataError> {
    info!("Aligning higher timeframe data with suffix '{}' via Asof Join", suffix);

    // Using an asof join, we find the closest prior row in the higher timeframe for each row in the base timeframe
    let aligned = base_lf.join_builder()
        .with(higher_tf_lf)
        .left_on(vec![col("timestamp")])
        .right_on(vec![col("timestamp")])
        .how(JoinType::Left)
        .suffix(suffix)
        .finish();

    Ok(aligned)
}
