use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FitnessFunction {
    Pnl,
    Drawdown,
    PnlOverDd,
    RatioWl,
    ProfitFactor,
    Sharpe,
    Sortino,
    AvgTrade,
    WinPercentage,
    CpcIndex,
    CorrCoef,
}

impl Default for FitnessFunction {
    fn default() -> Self {
        FitnessFunction::PnlOverDd
    }
}

impl FitnessFunction {
    /// Registry of all available fitness functions for the UI.
    pub fn available() -> Vec<(Self, &'static str)> {
        vec![
            (Self::Pnl, "Total PnL"),
            (Self::Drawdown, "Drawdown"),
            (Self::PnlOverDd, "PnL / DD"),
            (Self::RatioWl, "Win/Loss Ratio"),
            (Self::ProfitFactor, "Profit Factor"),
            (Self::Sharpe, "Sharpe Ratio"),
            (Self::Sortino, "Sortino Ratio"),
            (Self::AvgTrade, "Average Trade"),
            (Self::WinPercentage, "Win Percentage"),
            (Self::CpcIndex, "CPC Index"),
            (Self::CorrCoef, "Correlation Coef"),
        ]
    }
}
