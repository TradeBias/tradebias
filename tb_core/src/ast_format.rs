use crate::ast::Expr;
use std::fmt;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Leaf Nodes
            Expr::Close => write!(f, "Close"),
            Expr::Open => write!(f, "Open"),
            Expr::High => write!(f, "High"),
            Expr::Low => write!(f, "Low"),
            Expr::Volume => write!(f, "Volume"),
            Expr::Constant { value } => write!(f, "{}", value),
            Expr::TrueRange => write!(f, "TrueRange"),
            Expr::Placeholder => write!(f, "<Placeholder>"),
            Expr::ParamPlaceholder { name } => write!(f, "<Param: {}>", name),
            Expr::Macro { name, output, source, params } => {
                let params_str: Vec<String> = params.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
                if params_str.is_empty() {
                    write!(f, "{}.{}({})", name, output, source)
                } else {
                    write!(f, "{}.{}({}, {})", name, output, source, params_str.join(", "))
                }
            },

            // Core Arithmetic (Algebraic Formatting)
            Expr::Add { lhs, rhs } => write!(f, "({} + {})", lhs, rhs),
            Expr::Sub { lhs, rhs } => write!(f, "({} - {})", lhs, rhs),
            Expr::Mul { lhs, rhs } => write!(f, "({} * {})", lhs, rhs),
            Expr::Div { lhs, rhs } => write!(f, "({} / {})", lhs, rhs),
            Expr::Abs { source } => write!(f, "Abs({})", source),

            // Time-Series Aggregation
            Expr::Delay { source, period } => write!(f, "{}[-{}]", source, period),
            Expr::TsMax { source, period } => write!(f, "Max({}, {})", source, period),
            Expr::TsMin { source, period } => write!(f, "Min({}, {})", source, period),
            Expr::TsSum { source, period } => write!(f, "Sum({}, {})", source, period),

            // Smoothers
            Expr::Sma { source, period } => write!(f, "SMA({}, {})", source, period),
            Expr::Ema { source, period } => write!(f, "EMA({}, {})", source, period),
            Expr::Wma { source, period } => write!(f, "WMA({}, {})", source, period),
            Expr::Rma { source, period } => write!(f, "RMA({}, {})", source, period),

            // Statistical
            Expr::StdDev { source, period } => write!(f, "StdDev({}, {})", source, period),
            Expr::LinRegSlope { source, period } => write!(f, "LinRegSlope({}, {})", source, period),

            // Logical / Relational
            Expr::GreaterThan { lhs, rhs } => write!(f, "{} > {}", lhs, rhs),
            Expr::LessThan { lhs, rhs } => write!(f, "{} < {}", lhs, rhs),
            Expr::CrossAbove { lhs, rhs } => write!(f, "{} crosses above {}", lhs, rhs),
            Expr::CrossBelow { lhs, rhs } => write!(f, "{} crosses below {}", lhs, rhs),
            Expr::And { lhs, rhs } => write!(f, "({} AND {})", lhs, rhs),
            Expr::Or { lhs, rhs } => write!(f, "({} OR {})", lhs, rhs),

            // Black Boxes
            Expr::Psar { af_step, af_max } => write!(f, "PSAR({}, {})", af_step, af_max),
            Expr::KalmanFilter { r, q } => write!(f, "Kalman({}, {})", r, q),
            Expr::EhlersSuperSmoother { period } => write!(f, "SuperSmoother({})", period),
            Expr::EhlersDecycler { period } => write!(f, "Decycler({})", period),
            Expr::EhlersCyberCycle { alpha } => write!(f, "CyberCycle({})", alpha),
        }
    }
}
