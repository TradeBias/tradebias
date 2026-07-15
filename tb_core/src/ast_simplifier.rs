use crate::ast::Expr;

pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        // Base leaves
        Expr::Close | Expr::Open | Expr::High | Expr::Low | Expr::Volume | Expr::TrueRange | Expr::Constant { .. } | Expr::Placeholder | Expr::ParamPlaceholder { .. } => expr.clone(),

        // Binary Arithmetic
        Expr::Add { lhs, rhs } => {
            let l = simplify(lhs);
            let r = simplify(rhs);
            match (&l, &r) {
                (Expr::Constant { value: v1 }, Expr::Constant { value: v2 }) => Expr::Constant { value: v1 + v2 },
                (Expr::Constant { value: 0.0 }, _) => r,
                (_, Expr::Constant { value: 0.0 }) => l,
                _ => Expr::Add { lhs: Box::new(l), rhs: Box::new(r) },
            }
        }
        Expr::Sub { lhs, rhs } => {
            let l = simplify(lhs);
            let r = simplify(rhs);
            match (&l, &r) {
                (Expr::Constant { value: v1 }, Expr::Constant { value: v2 }) => Expr::Constant { value: v1 - v2 },
                (_, Expr::Constant { value: 0.0 }) => l,
                _ if l == r => Expr::Constant { value: 0.0 },
                _ => Expr::Sub { lhs: Box::new(l), rhs: Box::new(r) },
            }
        }
        Expr::Mul { lhs, rhs } => {
            let l = simplify(lhs);
            let r = simplify(rhs);
            match (&l, &r) {
                (Expr::Constant { value: v1 }, Expr::Constant { value: v2 }) => Expr::Constant { value: v1 * v2 },
                (Expr::Constant { value: 0.0 }, _) | (_, Expr::Constant { value: 0.0 }) => Expr::Constant { value: 0.0 },
                (Expr::Constant { value: 1.0 }, _) => r,
                (_, Expr::Constant { value: 1.0 }) => l,
                _ => Expr::Mul { lhs: Box::new(l), rhs: Box::new(r) },
            }
        }
        Expr::Div { lhs, rhs } => {
            let l = simplify(lhs);
            let r = simplify(rhs);
            match (&l, &r) {
                (Expr::Constant { value: v1 }, Expr::Constant { value: v2 }) if *v2 != 0.0 => Expr::Constant { value: v1 / v2 },
                (_, Expr::Constant { value: 1.0 }) => l,
                _ if l == r => Expr::Constant { value: 1.0 },
                (Expr::Constant { value: 0.0 }, _) => Expr::Constant { value: 0.0 },
                _ => Expr::Div { lhs: Box::new(l), rhs: Box::new(r) },
            }
        }
        Expr::Abs { source } => {
            let s = simplify(source);
            match s {
                Expr::Constant { value } => Expr::Constant { value: value.abs() },
                Expr::Abs { source: inner } => Expr::Abs { source: inner }, // Abs(Abs(x)) == Abs(x)
                _ => Expr::Abs { source: Box::new(s) },
            }
        }

        // Smoothing (Collapse moving averages over constants)
        Expr::Sma { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { .. } = s { return s; }
            Expr::Sma { source: Box::new(s), period: *period }
        }
        Expr::Ema { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { .. } = s { return s; }
            Expr::Ema { source: Box::new(s), period: *period }
        }
        Expr::Wma { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { .. } = s { return s; }
            Expr::Wma { source: Box::new(s), period: *period }
        }
        Expr::Rma { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { .. } = s { return s; }
            Expr::Rma { source: Box::new(s), period: *period }
        }
        
        // Ts aggregations
        Expr::Delay { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { .. } = s { return s; }
            if *period == 0 { return s; } // Delay 0 is identity
            Expr::Delay { source: Box::new(s), period: *period }
        }
        Expr::TsMax { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { .. } = s { return s; }
            Expr::TsMax { source: Box::new(s), period: *period }
        }
        Expr::TsMin { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { .. } = s { return s; }
            Expr::TsMin { source: Box::new(s), period: *period }
        }
        Expr::TsSum { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { value } = s { return Expr::Constant { value: value * (*period as f64) }; }
            Expr::TsSum { source: Box::new(s), period: *period }
        }

        // Statistical
        Expr::StdDev { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { .. } = s { return Expr::Constant { value: 0.0 }; }
            Expr::StdDev { source: Box::new(s), period: *period }
        }
        Expr::LinRegSlope { source, period } => {
            let s = simplify(source);
            if let Expr::Constant { .. } = s { return Expr::Constant { value: 0.0 }; }
            Expr::LinRegSlope { source: Box::new(s), period: *period }
        }

        // Logical
        Expr::GreaterThan { lhs, rhs } => Expr::GreaterThan { lhs: Box::new(simplify(lhs)), rhs: Box::new(simplify(rhs)) },
        Expr::LessThan { lhs, rhs } => Expr::LessThan { lhs: Box::new(simplify(lhs)), rhs: Box::new(simplify(rhs)) },
        Expr::CrossAbove { lhs, rhs } => Expr::CrossAbove { lhs: Box::new(simplify(lhs)), rhs: Box::new(simplify(rhs)) },
        Expr::CrossBelow { lhs, rhs } => Expr::CrossBelow { lhs: Box::new(simplify(lhs)), rhs: Box::new(simplify(rhs)) },
        
        Expr::And { lhs, rhs } => {
            let l = simplify(lhs);
            let r = simplify(rhs);
            if l == r { return l; } // Idempotency
            Expr::And { lhs: Box::new(l), rhs: Box::new(r) }
        }
        Expr::Or { lhs, rhs } => {
            let l = simplify(lhs);
            let r = simplify(rhs);
            if l == r { return l; } // Idempotency
            Expr::Or { lhs: Box::new(l), rhs: Box::new(r) }
        }

        Expr::Psar { af_step, af_max } => Expr::Psar { af_step: *af_step, af_max: *af_max },
        Expr::KalmanFilter { r, q } => Expr::KalmanFilter { r: *r, q: *q },
        Expr::EhlersSuperSmoother { period } => Expr::EhlersSuperSmoother { period: *period },
        Expr::EhlersDecycler { period } => Expr::EhlersDecycler { period: *period },
        Expr::EhlersCyberCycle { alpha } => Expr::EhlersCyberCycle { alpha: *alpha },
        Expr::Macro { name, output, source, params } => Expr::Macro {
            name: name.clone(),
            output: output.clone(),
            source: Box::new(simplify(source)),
            params: params.clone(),
        },
    }
}
