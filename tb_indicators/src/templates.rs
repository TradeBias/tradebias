use tb_core::ast::{Expr, IndicatorBlueprint, SemanticType};

/// Builds a MACD Line template: EMA(fast) - EMA(slow)
pub fn macd_line(source: Expr, fast: u32, slow: u32) -> Expr {
    Expr::Sub {
        lhs: Box::new(Expr::Ema { source: Box::new(source.clone()), period: fast }),
        rhs: Box::new(Expr::Ema { source: Box::new(source), period: slow }),
    }
}

/// Builds a MACD Signal template: EMA(MACD Line, signal)
pub fn macd_signal(source: Expr, fast: u32, slow: u32, signal: u32) -> Expr {
    Expr::Ema {
        source: Box::new(macd_line(source, fast, slow)),
        period: signal,
    }
}

/// Builds a Bollinger Bands Upper template: SMA(period) + (StdDev(period) * 2.0)
pub fn bollinger_upper(source: Expr, period: u32, dev: f64) -> Expr {
    Expr::Add {
        lhs: Box::new(Expr::Sma { source: Box::new(source.clone()), period }),
        rhs: Box::new(Expr::Mul {
            lhs: Box::new(Expr::StdDev { source: Box::new(source), period }),
            rhs: Box::new(Expr::Constant { value: dev }),
        }),
    }
}

/// Builds a Bollinger Bands Lower template: SMA(period) - (StdDev(period) * 2.0)
pub fn bollinger_lower(source: Expr, period: u32, dev: f64) -> Expr {
    Expr::Sub {
        lhs: Box::new(Expr::Sma { source: Box::new(source.clone()), period }),
        rhs: Box::new(Expr::Mul {
            lhs: Box::new(Expr::StdDev { source: Box::new(source), period }),
            rhs: Box::new(Expr::Constant { value: dev }),
        }),
    }
}

/// Builds a Z-Score template (similar to CCI): (Price - SMA(period)) / StdDev(period)
pub fn z_score(source: Expr, period: u32) -> Expr {
    Expr::Div {
        lhs: Box::new(Expr::Sub {
            lhs: Box::new(source.clone()),
            rhs: Box::new(Expr::Sma { source: Box::new(source.clone()), period }),
        }),
        rhs: Box::new(Expr::StdDev { source: Box::new(source), period }),
    }
}

/// Builds a Shallow RSI-Equivalent Momentum Oscillator:
/// (Close - Min(14)) / (Max(14) - Min(14)) -> Stochastics style normalized ratio
pub fn normalized_momentum(source: Expr, period: u32) -> Expr {
    Expr::Div {
        lhs: Box::new(Expr::Sub {
            lhs: Box::new(source.clone()),
            rhs: Box::new(Expr::TsMin { source: Box::new(source.clone()), period }),
        }),
        rhs: Box::new(Expr::Sub {
            lhs: Box::new(Expr::TsMax { source: Box::new(source.clone()), period }),
            rhs: Box::new(Expr::TsMin { source: Box::new(source), period }),
        }),
    }
}

pub fn default_blueprints() -> Vec<IndicatorBlueprint> {
    vec![
        IndicatorBlueprint {
            name: "SMA".to_string(),
            semantic_type: SemanticType::Price,
            is_custom: false,
            outputs: vec![("Value".to_string(), Expr::Sma { source: Box::new(Expr::Placeholder), period: 14 })],
        },
        IndicatorBlueprint {
            name: "EMA".to_string(),
            semantic_type: SemanticType::Price,
            is_custom: false,
            outputs: vec![("Value".to_string(), Expr::Ema { source: Box::new(Expr::Placeholder), period: 14 })],
        },
        IndicatorBlueprint {
            name: "BOLL".to_string(),
            semantic_type: SemanticType::Price,
            is_custom: false,
            outputs: vec![
                ("Upper".to_string(), bollinger_upper(Expr::Placeholder, 20, 2.0)),
                ("Basis".to_string(), Expr::Sma { source: Box::new(Expr::Placeholder), period: 20 }),
                ("Lower".to_string(), bollinger_lower(Expr::Placeholder, 20, 2.0)),
            ],
        },
        IndicatorBlueprint {
            name: "MACD".to_string(),
            semantic_type: SemanticType::Oscillator,
            is_custom: false,
            outputs: vec![
                ("Line".to_string(), macd_line(Expr::Placeholder, 12, 26)),
                ("Signal".to_string(), macd_signal(Expr::Placeholder, 12, 26, 9)),
            ],
        },
        IndicatorBlueprint {
            name: "RSI".to_string(),
            semantic_type: SemanticType::Oscillator,
            is_custom: false,
            outputs: vec![("Value".to_string(), normalized_momentum(Expr::Placeholder, 14))],
        },
        IndicatorBlueprint {
            name: "ATR".to_string(),
            semantic_type: SemanticType::Scalar,
            is_custom: false,
            outputs: vec![("Value".to_string(), Expr::Rma { source: Box::new(Expr::Placeholder), period: 14 })],
        },
    ]
}
