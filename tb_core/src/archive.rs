use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ArchiveTrait {
    WinRate,
    MarketExposure,
    Complexity,
    MaxDrawdown,
}

impl ArchiveTrait {
    /// Registry of all available traits for MAP-Elites axes.
    pub fn available() -> Vec<(Self, &'static str)> {
        vec![
            (Self::WinRate, "Win Rate (%)"),
            (Self::MarketExposure, "Market Exposure (%)"),
            (Self::Complexity, "Complexity (Indicators)"),
            (Self::MaxDrawdown, "Max Drawdown (%)"),
        ]
    }
}
