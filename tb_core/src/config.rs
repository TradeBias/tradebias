use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SessionConfig {
    pub phase1: Phase1Config,
    pub phase2: Phase2Config,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ArchitectureMode {
    ContinuousAst,
    DiscretePrecomputed,
    DynamicLazyCache,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Phase1Config {
    pub architecture_mode: ArchitectureMode,
    pub trading_style: TradingStyle,
    pub optimization_focus: OptimizationFocus,
    pub risk_appetite: RiskAppetite,
    pub complexity_cap: ComplexityCap,
    pub permitted_indicators: Vec<String>, // E.g. "SMA", "EMA", "RSI", "MACD"
    pub stop_type: crate::stops::StopType,
    pub take_profit: crate::stops::TakeProfit,
    pub trade_direction: crate::ast::TradeDirection,
    pub map_x: crate::archive::ArchiveTrait,
    pub map_y: crate::archive::ArchiveTrait,
    pub fitness: crate::fitness::FitnessFunction,
    pub grid_size: usize,
    // Hard Constraints
    pub min_trades: usize,
    pub min_exposure: f64,
    pub max_exposure: f64,
    pub slippage_penalty: f64,
    pub in_sample_pct: f64,
    pub long_strategy_pct: f64,
    pub starting_equity: f64,
    // Robustness Defaults
    pub occam_penalty_pct: f64,
    pub random_benchmark_percentile: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Phase2Config {
    pub exit_strategy: ExitStrategy,
    pub position_sizing: PositionSizing,
    pub frictions: Frictions,
    pub take_profit_pct: Option<f64>,
    pub stop_loss_pct: Option<f64>,
    pub trailing_stop_pct: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TradingStyle {
    Scalping,
    DayTrading,
    Swing,
    Position,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OptimizationFocus {
    MaximizeEdge,
    MaximizeWinLoss,
    MinimizeDrawdown,
    Balanced,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RiskAppetite {
    Conservative,
    Moderate,
    Aggressive,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ComplexityCap {
    HumanReadable,
    Moderate,
    BlackBox,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ExitStrategy {
    SetAndForget, // E.g., Hard TP
    TrailingStop,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PositionSizing {
    Fixed1Percent,
    VolatilityAdjusted,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Frictions {
    Low,
    Medium,
    High,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            phase1: Phase1Config {
                architecture_mode: ArchitectureMode::ContinuousAst,
                trading_style: TradingStyle::Swing,
                optimization_focus: OptimizationFocus::MaximizeEdge,
                risk_appetite: RiskAppetite::Conservative,
                complexity_cap: ComplexityCap::HumanReadable,
                permitted_indicators: vec!["SMA".into(), "EMA".into(), "RSI".into(), "MACD".into()],
                stop_type: crate::stops::StopType::FixedBarHold { bars: 5 },
                take_profit: crate::stops::TakeProfit::RiskReward { multiplier: 2.0 },
                trade_direction: crate::ast::TradeDirection::Long,
                map_x: crate::archive::ArchiveTrait::MarketExposure,
                map_y: crate::archive::ArchiveTrait::WinRate,
                fitness: crate::fitness::FitnessFunction::PnlOverDd,
                grid_size: 25,
                min_trades: 10,
                min_exposure: 0.001,
                max_exposure: 0.8,
                slippage_penalty: 0.0,
                in_sample_pct: 0.7,
                long_strategy_pct: 0.5,
                starting_equity: 100_000.0,
                occam_penalty_pct: 0.02,
                random_benchmark_percentile: 0.95,
            },
            phase2: Phase2Config {
                exit_strategy: ExitStrategy::TrailingStop,
                position_sizing: PositionSizing::Fixed1Percent,
                frictions: Frictions::Low,
                take_profit_pct: Some(0.05),
                stop_loss_pct: Some(0.02),
                trailing_stop_pct: Some(0.02),
            },
        }
    }
}
