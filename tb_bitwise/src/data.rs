use anyhow::Result;
use tracing::info;

/// The core data structure for the Bitwise Engine.
/// This completely replaces Polars DataFrames with pure, contiguous Float arrays.
pub struct RawData {
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Vec<f64>,
}

impl RawData {
    /// Loads a CSV file into pure contiguous vectors.
    pub fn from_csv(path: &str) -> Result<Self> {
        info!("Loading dataset into contiguous arrays from {}", path);
        
        let mut rdr = csv::Reader::from_path(path)?;
        
        let mut open = Vec::new();
        let mut high = Vec::new();
        let mut low = Vec::new();
        let mut close = Vec::new();
        let mut volume = Vec::new();

        // Assuming headers are Open, High, Low, Close, Volume
        // We will implement robust parsing later.
        for result in rdr.records() {
            let record = result?;
            // Dummy parsing for now. Real implementation will target correct column indexes.
            if let (Ok(o), Ok(h), Ok(l), Ok(c), Ok(v)) = (
                record[1].parse::<f64>(),
                record[2].parse::<f64>(),
                record[3].parse::<f64>(),
                record[4].parse::<f64>(),
                record[5].parse::<f64>(),
            ) {
                open.push(o);
                high.push(h);
                low.push(l);
                close.push(c);
                volume.push(v);
            }
        }
        
        info!("Loaded {} bars into raw arrays.", close.len());
        Ok(Self { open, high, low, close, volume })
    }

    /// Loads dataset directly from memory arrays (e.g. pulled from PostgreSQL via tb_data).
    /// Ensures all vectors are perfectly aligned for Bitwise intersection.
    pub fn from_arrays(open: Vec<f64>, high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, volume: Vec<f64>) -> Result<Self> {
        let len = close.len();
        
        if open.len() != len || high.len() != len || low.len() != len || volume.len() != len {
            anyhow::bail!("Data Misalignment Error: All arrays must be exactly the same length. (Close = {}, Open = {}, High = {}, Low = {}, Vol = {})", len, open.len(), high.len(), low.len(), volume.len());
        }

        info!("Loaded {} live/historical bars directly from memory.", len);
        Ok(Self { open, high, low, close, volume })
    }
}
