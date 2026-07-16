use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticType {
    Price,
    Volume,
    Oscillator,
    Scalar,
    Boolean,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IndicatorBlueprint {
    pub name: String,
    pub semantic_type: SemanticType,
    pub is_custom: bool,
    pub outputs: Vec<(String, Expr)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Expr {
    // 4a. Data Sources (Leaves)
    Close,
    Open,
    High,
    Low,
    Volume,
    Constant { value: f64 },
    Placeholder,
    ParamPlaceholder { name: String },
    Macro {
        name: String,
        output: String,
        source: Box<Expr>,
        params: Vec<(String, f64)>,
    },

    // 4b. Core Arithmetic (Binary Nodes)
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    Mul { lhs: Box<Expr>, rhs: Box<Expr> },
    Div { lhs: Box<Expr>, rhs: Box<Expr> },
    Abs { source: Box<Expr> },

    // 4c. Time-Series Aggregation
    Delay { source: Box<Expr>, period: u32 },
    TsMax { source: Box<Expr>, period: u32 },
    TsMin { source: Box<Expr>, period: u32 },
    TsSum { source: Box<Expr>, period: u32 },

    // 4d. Smoothing Primitives
    Sma { source: Box<Expr>, period: u32 },
    Ema { source: Box<Expr>, period: u32 },
    Wma { source: Box<Expr>, period: u32 },
    Rma { source: Box<Expr>, period: u32 },

    // 4e. Statistical Operators
    StdDev { source: Box<Expr>, period: u32 },
    LinRegSlope { source: Box<Expr>, period: u32 },

    // 4f. Market Internals
    TrueRange,

    // 4g. Logical / Relational (Produce Boolean)
    GreaterThan { lhs: Box<Expr>, rhs: Box<Expr> },
    LessThan { lhs: Box<Expr>, rhs: Box<Expr> },
    CrossAbove { lhs: Box<Expr>, rhs: Box<Expr> },
    CrossBelow { lhs: Box<Expr>, rhs: Box<Expr> },
    And { lhs: Box<Expr>, rhs: Box<Expr> },
    Or { lhs: Box<Expr>, rhs: Box<Expr> },

    // 4h. Hardcoded Exception Nodes (Black Boxes)
    Psar { af_step: f64, af_max: f64 },
    KalmanFilter { r: f64, q: f64 },
    EhlersSuperSmoother { period: u32 },
    EhlersDecycler { period: u32 },
    EhlersCyberCycle { alpha: f64 },
}

impl Expr {
    pub fn replace_placeholder(&mut self, target: &Expr) {
        match self {
            Expr::Placeholder => *self = target.clone(),
            Expr::Add { lhs, rhs } |
            Expr::Sub { lhs, rhs } |
            Expr::Mul { lhs, rhs } |
            Expr::Div { lhs, rhs } |
            Expr::GreaterThan { lhs, rhs } |
            Expr::LessThan { lhs, rhs } |
            Expr::CrossAbove { lhs, rhs } |
            Expr::CrossBelow { lhs, rhs } |
            Expr::And { lhs, rhs } |
            Expr::Or { lhs, rhs } => {
                lhs.replace_placeholder(target);
                rhs.replace_placeholder(target);
            },
            Expr::Sma { source, .. } |
            Expr::Ema { source, .. } |
            Expr::Wma { source, .. } |
            Expr::Rma { source, .. } |
            Expr::TsMax { source, .. } |
            Expr::TsMin { source, .. } |
            Expr::TsSum { source, .. } |
            Expr::StdDev { source, .. } |
            Expr::Abs { source } |
            Expr::LinRegSlope { source, .. } |
            Expr::Delay { source, .. } |
            Expr::Macro { source, .. } => {
                source.replace_placeholder(target);
            },
            _ => {},
        }
    }
}

impl Eq for Expr {}

impl std::hash::Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Expr::Close | Expr::Open | Expr::High | Expr::Low | Expr::Volume | Expr::TrueRange | Expr::Placeholder => {}
            Expr::Constant { value } => value.to_bits().hash(state),
            Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } | Expr::Mul { lhs, rhs } | Expr::Div { lhs, rhs } |
            Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } | Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
            Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => {
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Abs { source } => source.hash(state),
            Expr::Delay { source, period } | Expr::TsMax { source, period } | Expr::TsMin { source, period } | Expr::TsSum { source, period } |
            Expr::Sma { source, period } | Expr::Ema { source, period } | Expr::Wma { source, period } | Expr::Rma { source, period } |
            Expr::StdDev { source, period } | Expr::LinRegSlope { source, period } => {
                source.hash(state);
                period.hash(state);
            }
            Expr::Psar { af_step, af_max } => {
                af_step.to_bits().hash(state);
                af_max.to_bits().hash(state);
            }
            Expr::KalmanFilter { r, q } => {
                r.to_bits().hash(state);
                q.to_bits().hash(state);
            }
            Expr::EhlersSuperSmoother { period } | Expr::EhlersDecycler { period } => period.hash(state),
            Expr::EhlersCyberCycle { alpha } => alpha.to_bits().hash(state),
            Expr::ParamPlaceholder { name } => name.hash(state),
            Expr::Macro { name, output, source, params } => {
                name.hash(state);
                output.hash(state);
                source.hash(state);
                for (k, v) in params {
                    k.hash(state);
                    v.to_bits().hash(state);
                }
            },
        }
    }
}

impl Expr {
    /// Evaluates the semantic type of an expression node according to strict dimensional analysis.
    pub fn semantic_type(&self) -> SemanticType {
        match self {
            // Leaf Nodes
            Expr::Close | Expr::Open | Expr::High | Expr::Low => SemanticType::Price,
            Expr::Volume => SemanticType::Volume,
            Expr::Constant { .. } => SemanticType::Scalar,
            Expr::TrueRange => SemanticType::Price,
            Expr::Placeholder => SemanticType::Price, // default fallback
            Expr::ParamPlaceholder { .. } => SemanticType::Scalar,

            // Operations that preserve the type of their input
            Expr::Abs { source } 
            | Expr::Delay { source, .. } 
            | Expr::TsMax { source, .. } 
            | Expr::TsMin { source, .. } 
            | Expr::TsSum { source, .. } 
            | Expr::Sma { source, .. } 
            | Expr::Ema { source, .. } 
            | Expr::Wma { source, .. } 
            | Expr::Rma { source, .. } 
            | Expr::StdDev { source, .. } 
            | Expr::Macro { source, .. } => source.semantic_type(),

            Expr::LinRegSlope { .. } => SemanticType::Oscillator,

            // Relational & Logical always produce Boolean
            Expr::GreaterThan { .. } 
            | Expr::LessThan { .. } 
            | Expr::CrossAbove { .. } 
            | Expr::CrossBelow { .. } 
            | Expr::And { .. } 
            | Expr::Or { .. } => SemanticType::Boolean,

            // Black Boxes (mostly price overlays)
            Expr::Psar { .. } 
            | Expr::KalmanFilter { .. } 
            | Expr::EhlersSuperSmoother { .. } 
            | Expr::EhlersDecycler { .. } 
            | Expr::EhlersCyberCycle { .. } => SemanticType::Price,

            // Binary Arithmetic Type Algebra
            Expr::Add { lhs, rhs } => {
                let l = lhs.semantic_type();
                let r = rhs.semantic_type();
                if l == r && (l == SemanticType::Price || l == SemanticType::Oscillator || l == SemanticType::Volume || l == SemanticType::Scalar) {
                    l
                } else if (l == SemanticType::Oscillator && r == SemanticType::Scalar) || (l == SemanticType::Scalar && r == SemanticType::Oscillator) {
                    SemanticType::Oscillator
                } else {
                    SemanticType::Unknown
                }
            },
            Expr::Sub { lhs, rhs } => {
                let l = lhs.semantic_type();
                let r = rhs.semantic_type();
                if l == SemanticType::Price && r == SemanticType::Price {
                    SemanticType::Oscillator
                } else if l == r && (l == SemanticType::Oscillator || l == SemanticType::Volume || l == SemanticType::Scalar) {
                    l
                } else if (l == SemanticType::Oscillator && r == SemanticType::Scalar) || (l == SemanticType::Scalar && r == SemanticType::Oscillator) {
                    SemanticType::Oscillator
                } else if l == SemanticType::Price && r == SemanticType::Scalar {
                    SemanticType::Price
                } else {
                    SemanticType::Unknown
                }
            },

            Expr::Mul { lhs, rhs } => {
                let l = lhs.semantic_type();
                let r = rhs.semantic_type();
                match (l, r) {
                    (SemanticType::Price, SemanticType::Scalar) | (SemanticType::Scalar, SemanticType::Price) => SemanticType::Price,
                    (SemanticType::Volume, SemanticType::Scalar) | (SemanticType::Scalar, SemanticType::Volume) => SemanticType::Volume,
                    (SemanticType::Oscillator, SemanticType::Scalar) | (SemanticType::Scalar, SemanticType::Oscillator) => SemanticType::Oscillator,
                    (SemanticType::Scalar, SemanticType::Scalar) => SemanticType::Scalar,
                    (SemanticType::Price, SemanticType::Oscillator) | (SemanticType::Oscillator, SemanticType::Price) => SemanticType::Price,
                    (SemanticType::Volume, SemanticType::Oscillator) | (SemanticType::Oscillator, SemanticType::Volume) => SemanticType::Volume,
                    _ => SemanticType::Unknown,
                }
            },

            Expr::Div { lhs, rhs } => {
                let l = lhs.semantic_type();
                let r = rhs.semantic_type();
                match (l, r) {
                    (SemanticType::Price, SemanticType::Price) => SemanticType::Oscillator,
                    (SemanticType::Volume, SemanticType::Volume) => SemanticType::Oscillator,
                    (SemanticType::Price, SemanticType::Scalar) => SemanticType::Price,
                    (SemanticType::Volume, SemanticType::Scalar) => SemanticType::Volume,
                    (SemanticType::Oscillator, SemanticType::Scalar) => SemanticType::Oscillator,
                    (SemanticType::Oscillator, SemanticType::Oscillator) => SemanticType::Oscillator,
                    (SemanticType::Scalar, SemanticType::Scalar) => SemanticType::Scalar,
                    _ => SemanticType::Unknown,
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TradeDirection {
    Long,
    Short,
    LongAndShort,
}

/// A Strategy Sketch is just an entry condition (an Expr that evaluates to a boolean)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sketch {
    pub name: String,
    pub direction: TradeDirection,
    pub entry: Expr,
    pub exit: Option<Expr>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EliteStrategy {
    pub sketch: Sketch,
    pub fitness: f64,
    pub pnl: f64,
    pub max_drawdown: f64,
    pub pnl_over_dd: f64,
    pub sharpe: f64,
    pub sortino: f64,
    pub profit_factor: f64,
    pub cpc_index: f64,
    pub corr_coef: f64,
    pub avg_trade: f64,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub std_win: f64,
    pub std_loss: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
    pub max_consecutive_losses: u32,
    pub exposure_pct: f64,
    pub indicator_count: u8,
    pub num_trades: u32,
    pub conditions: Vec<Expr>,
}
