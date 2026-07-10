use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExprType {
    PriceOverlay, // Scales with asset price (Close, SMA, EMA)
    Oscillator,   // Bounded or mean-reverting (RSI, MACD)
    Constant,     // Static thresholds (30.0, 70.0)
    Boolean,      // Logical nodes
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Expr {
    // 1. Data Sources (Leaves)
    Close,
    Open,
    High,
    Low,
    Volume,
    
    // 2. Constants (Leaves)
    Constant { value: f64 },

    // 3. Indicators (Nodes)
    Sma { source: Box<Expr>, period: u32 },
    Ema { source: Box<Expr>, period: u32 },
    Rsi { source: Box<Expr>, period: u32 },
    Macd { source: Box<Expr>, fast: u32, slow: u32, signal: u32 },
    Atr { period: u32 },

    // 4. Mathematical Operators
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    
    // 5. Logical Operators (Return Booleans)
    CrossAbove { lhs: Box<Expr>, rhs: Box<Expr> },
    CrossBelow { lhs: Box<Expr>, rhs: Box<Expr> },
    GreaterThan { lhs: Box<Expr>, rhs: Box<Expr> },
    LessThan { lhs: Box<Expr>, rhs: Box<Expr> },
    
    // 6. Conjunctions (Combine Booleans)
    And { lhs: Box<Expr>, rhs: Box<Expr> },
    Or { lhs: Box<Expr>, rhs: Box<Expr> },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TradeDirection {
    Long,
    Short,
}

/// A Strategy Sketch is just an entry condition (an Expr that evaluates to a boolean)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sketch {
    pub name: String,
    pub direction: TradeDirection,
    pub entry: Expr,
    pub exit: Option<Expr>,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Close => write!(f, "Close"),
            Expr::Open => write!(f, "Open"),
            Expr::High => write!(f, "High"),
            Expr::Low => write!(f, "Low"),
            Expr::Volume => write!(f, "Volume"),
            Expr::Constant { value } => write!(f, "{}", value),
            Expr::Sma { source, period } => write!(f, "SMA({}, {})", source, period),
            Expr::Ema { source, period } => write!(f, "EMA({}, {})", source, period),
            Expr::Rsi { source, period } => write!(f, "RSI({}, {})", source, period),
            Expr::Macd { source, fast, slow, signal } => write!(f, "MACD({}, {}, {}, {})", source, fast, slow, signal),
            Expr::Atr { period } => write!(f, "ATR({})", period),
            Expr::Add { lhs, rhs } => write!(f, "({} + {})", lhs, rhs),
            Expr::Sub { lhs, rhs } => write!(f, "({} - {})", lhs, rhs),
            Expr::CrossAbove { lhs, rhs } => write!(f, "{} crosses above {}", lhs, rhs),
            Expr::CrossBelow { lhs, rhs } => write!(f, "{} crosses below {}", lhs, rhs),
            Expr::GreaterThan { lhs, rhs } => write!(f, "{} > {}", lhs, rhs),
            Expr::LessThan { lhs, rhs } => write!(f, "{} < {}", lhs, rhs),
            Expr::And { lhs, rhs } => write!(f, "({} AND {})", lhs, rhs),
            Expr::Or { lhs, rhs } => write!(f, "({} OR {})", lhs, rhs),
        }
    }
}

impl Expr {
    pub fn return_type(&self) -> ExprType {
        match self {
            Expr::Close | Expr::Open | Expr::High | Expr::Low | Expr::Volume => ExprType::PriceOverlay,
            Expr::Sma { .. } | Expr::Ema { .. } => ExprType::PriceOverlay,
            Expr::Rsi { .. } | Expr::Macd { .. } | Expr::Atr { .. } => ExprType::Oscillator,
            Expr::Constant { .. } => ExprType::Constant,
            Expr::Add { .. } | Expr::Sub { .. } => ExprType::PriceOverlay,
            Expr::CrossAbove { .. } | Expr::CrossBelow { .. } | Expr::GreaterThan { .. } | Expr::LessThan { .. } => ExprType::Boolean,
            Expr::And { .. } | Expr::Or { .. } => ExprType::Boolean,
        }
    }

    pub fn indicator_count(&self) -> u8 {
        match self {
            Expr::Close | Expr::Open | Expr::High | Expr::Low | Expr::Volume | Expr::Constant { .. } => 0,
            Expr::Sma { source, .. } | Expr::Ema { source, .. } | Expr::Rsi { source, .. } => 1 + source.indicator_count(),
            Expr::Macd { source, .. } => 1 + source.indicator_count(),
            Expr::Atr { .. } => 1,
            Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } |
            Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
            Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } |
            Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => lhs.indicator_count() + rhs.indicator_count(),
        }
    }

    pub fn structural_signature(&self) -> String {
        match self {
            Expr::Close => "Close".to_string(),
            Expr::Open => "Open".to_string(),
            Expr::High => "High".to_string(),
            Expr::Low => "Low".to_string(),
            Expr::Volume => "Volume".to_string(),
            Expr::Constant { .. } => "Const".to_string(),
            Expr::Sma { source, .. } => format!("SMA({})", source.structural_signature()),
            Expr::Ema { source, .. } => format!("EMA({})", source.structural_signature()),
            Expr::Rsi { source, .. } => format!("RSI({})", source.structural_signature()),
            Expr::Macd { source, .. } => format!("MACD({})", source.structural_signature()),
            Expr::Atr { .. } => "ATR".to_string(),
            Expr::Add { lhs, rhs } => format!("Add({},{})", lhs.structural_signature(), rhs.structural_signature()),
            Expr::Sub { lhs, rhs } => format!("Sub({},{})", lhs.structural_signature(), rhs.structural_signature()),
            Expr::CrossAbove { lhs, rhs } => format!("XUp({},{})", lhs.structural_signature(), rhs.structural_signature()),
            Expr::CrossBelow { lhs, rhs } => format!("XDn({},{})", lhs.structural_signature(), rhs.structural_signature()),
            Expr::GreaterThan { lhs, rhs } => format!("Gt({},{})", lhs.structural_signature(), rhs.structural_signature()),
            Expr::LessThan { lhs, rhs } => format!("Lt({},{})", lhs.structural_signature(), rhs.structural_signature()),
            Expr::And { lhs, rhs } => format!("And({},{})", lhs.structural_signature(), rhs.structural_signature()),
            Expr::Or { lhs, rhs } => format!("Or({},{})", lhs.structural_signature(), rhs.structural_signature()),
        }
    }

    pub fn extract_indicator_nodes(&self, indicators: &mut std::collections::HashSet<String>, nodes: &mut Vec<Expr>) {
        match self {
            Expr::Sma { .. } | Expr::Ema { .. } | Expr::Rsi { .. } | Expr::Macd { .. } | Expr::Atr { .. } => {
                let key = self.to_string();
                if !indicators.contains(&key) {
                    indicators.insert(key);
                    nodes.push(self.clone());
                }
            }
            Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } |
            Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
            Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } |
            Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => {
                lhs.extract_indicator_nodes(indicators, nodes);
                rhs.extract_indicator_nodes(indicators, nodes);
            }
            _ => {}
        }
    }

    pub fn snap_to_grid(&mut self, grid: &[u32]) {
        if grid.is_empty() { return; }
        let snap = |val: u32| -> u32 {
            *grid.iter().min_by_key(|&&g| (g as i32 - val as i32).abs()).unwrap()
        };
        match self {
            Expr::Sma { period, source } | Expr::Ema { period, source } | Expr::Rsi { period, source } => {
                *period = snap(*period);
                source.snap_to_grid(grid);
            }
            Expr::Atr { period } => {
                *period = snap(*period);
            }
            Expr::Macd { fast, slow, signal, source } => {
                *fast = snap(*fast);
                *slow = snap(*slow);
                *signal = snap(*signal);
                if *fast >= *slow {
                    if *fast == *slow {
                        if grid.len() > 1 && *fast == grid[0] {
                            *slow = grid[1];
                        } else {
                            *fast = grid[0];
                        }
                    } else {
                        std::mem::swap(fast, slow);
                    }
                }
                source.snap_to_grid(grid);
            }
            Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } |
            Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
            Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } |
            Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => {
                lhs.snap_to_grid(grid);
                rhs.snap_to_grid(grid);
            }
            _ => {}
        }
    }

    pub fn generate_grid_permutations(indicator_name: &str, sources: &[Expr], grid: &[u32]) -> Vec<Expr> {
        let mut exprs = Vec::new();
        match indicator_name.to_uppercase().as_str() {
            "SMA" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Sma { source: Box::new(src.clone()), period: p }); }
                }
            }
            "EMA" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Ema { source: Box::new(src.clone()), period: p }); }
                }
            }
            "RSI" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Rsi { source: Box::new(src.clone()), period: p }); }
                }
            }
            "MACD" => {
                for src in sources {
                    for &fast in grid {
                        for &slow in grid {
                            if fast >= slow { continue; }
                            for &signal in grid {
                                exprs.push(Expr::Macd { source: Box::new(src.clone()), fast, slow, signal });
                            }
                        }
                    }
                }
            }
            "ATR" => {
                for &p in grid { exprs.push(Expr::Atr { period: p }); }
            }
            _ => {}
        }
        exprs
    }
}

impl Sketch {
    pub fn indicator_count(&self) -> u8 {
        let mut count = self.entry.indicator_count();
        if let Some(ex) = &self.exit { count += ex.indicator_count(); }
        count
    }

    pub fn structural_signature(&self) -> String {
        let mut sig = format!("{:?}:{}", self.direction, self.entry.structural_signature());
        if let Some(ex) = &self.exit { sig.push_str(&format!("|X:{}", ex.structural_signature())); }
        sig
    }

    pub fn extract_indicator_nodes(&self, indicators: &mut std::collections::HashSet<String>, nodes: &mut Vec<Expr>) {
        self.entry.extract_indicator_nodes(indicators, nodes);
        if let Some(ex) = &self.exit { ex.extract_indicator_nodes(indicators, nodes); }
    }

    pub fn snap_to_grid(&mut self, grid: &[u32]) {
        self.entry.snap_to_grid(grid);
        if let Some(ex) = &mut self.exit { ex.snap_to_grid(grid); }
    }
}

impl std::fmt::Display for Sketch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("IF {} THEN Enter {:?}", self.entry, self.direction);
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sketch_serialization() {
        let sketch = Sketch {
            name: "TestSketch".into(),
            direction: TradeDirection::Long,
            entry: Expr::CrossAbove {
                lhs: Box::new(Expr::Sma { source: Box::new(Expr::Close), period: 10 }),
                rhs: Box::new(Expr::Sma { source: Box::new(Expr::Close), period: 50 }),
            },
            exit: None,
        };

        let json = serde_json::to_string(&sketch).unwrap();
        let deserialized: Sketch = serde_json::from_str(&json).unwrap();
        assert_eq!(sketch, deserialized);
    }
}
