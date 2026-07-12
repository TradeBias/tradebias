use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StopCalculation {
    Atr { multiplier: f64 },
    Fixed { points: f64 },
    Percentage { pct: f64 },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StopType {
    FixedBarHold { bars: usize },
    StandardStop { calc: StopCalculation },
    TrailingStop { calc: StopCalculation },
}

impl Default for StopType {
    fn default() -> Self {
        StopType::FixedBarHold { bars: 5 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TakeProfit {
    RiskReward { multiplier: f64 },
    Atr { multiplier: f64 },
    Fixed { points: f64 },
    Percentage { pct: f64 },
    None,
}

impl Default for TakeProfit {
    fn default() -> Self {
        TakeProfit::RiskReward { multiplier: 2.0 }
    }
}

pub struct ParamSpec {
    pub label: &'static str,
    pub min: f64,
    pub max: f64,
}

impl StopCalculation {
    pub fn available() -> Vec<(Self, &'static str, ParamSpec)> {
        vec![
            (Self::Atr { multiplier: 2.0 }, "ATR", ParamSpec { label: "Multiplier", min: 1.0, max: 6.0 }),
            (Self::Fixed { points: 10.0 }, "Fixed Points", ParamSpec { label: "Points", min: 1.0, max: 100.0 }),
            (Self::Percentage { pct: 1.0 }, "Percentage", ParamSpec { label: "%", min: 0.1, max: 10.0 }),
        ]
    }
    
    pub fn update_val(&mut self, val: f64) {
        match self {
            Self::Atr { multiplier } => *multiplier = val,
            Self::Fixed { points } => *points = val,
            Self::Percentage { pct } => *pct = val,
        }
    }
    
    pub fn get_val(&self) -> f64 {
        match self {
            Self::Atr { multiplier } => *multiplier,
            Self::Fixed { points } => *points,
            Self::Percentage { pct } => *pct,
        }
    }
}

impl StopType {
    pub fn available_types() -> Vec<(&'static str, bool)> {
        // Name, and whether it requires a StopCalculation
        vec![
            ("Fixed Bar Hold", false),
            ("Standard Stop (Static)", true),
            ("Trailing Stop", true),
        ]
    }
}

impl TakeProfit {
    pub fn available() -> Vec<(Self, &'static str, Option<ParamSpec>)> {
        vec![
            (Self::RiskReward { multiplier: 2.0 }, "Risk/Reward Multiplier", Some(ParamSpec { label: "R:R", min: 0.5, max: 10.0 })),
            (Self::Atr { multiplier: 3.0 }, "ATR Multiplier", Some(ParamSpec { label: "Multiplier", min: 1.0, max: 10.0 })),
            (Self::Fixed { points: 20.0 }, "Fixed Points", Some(ParamSpec { label: "Points", min: 1.0, max: 200.0 })),
            (Self::Percentage { pct: 2.0 }, "Percentage", Some(ParamSpec { label: "%", min: 0.1, max: 20.0 })),
            (Self::None, "None (Let Run)", None),
        ]
    }
    
    pub fn update_val(&mut self, val: f64) {
        match self {
            Self::RiskReward { multiplier } => *multiplier = val,
            Self::Atr { multiplier } => *multiplier = val,
            Self::Fixed { points } => *points = val,
            Self::Percentage { pct } => *pct = val,
            Self::None => {}
        }
    }
    
    pub fn get_val(&self) -> f64 {
        match self {
            Self::RiskReward { multiplier } => *multiplier,
            Self::Atr { multiplier } => *multiplier,
            Self::Fixed { points } => *points,
            Self::Percentage { pct } => *pct,
            Self::None => 0.0,
        }
    }
}
