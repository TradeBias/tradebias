use polars::prelude::*;
use crate::ast::Expr as AstExpr;
use crate::error::CoreError;

/// Compiles a tb_core::Expr (AST) into a polars::lazy::dsl::Expr
pub fn compile_ast_to_polars(ast: &AstExpr) -> Result<Expr, CoreError> {
    match ast {
        // 1. Data Sources (Leaves)
        AstExpr::Close => Ok(col("close")),
        AstExpr::Open => Ok(col("open")),
        AstExpr::High => Ok(col("high")),
        AstExpr::Low => Ok(col("low")),
        AstExpr::Volume => Ok(col("volume")),

        // 2. Constants (Leaves)
        AstExpr::Constant { value } => Ok(lit(*value)),

        // 3. Primitives
        AstExpr::Sma { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_sma(inner, *period))
        }
        AstExpr::Ema { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_ema(inner, *period))
        }
        AstExpr::Wma { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_wma(inner, *period))
        }
        AstExpr::Rma { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_rma(inner, *period))
        }
        
        AstExpr::TsMax { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_ts_max(inner, *period))
        }
        AstExpr::TsMin { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_ts_min(inner, *period))
        }
        AstExpr::TsSum { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_ts_sum(inner, *period))
        }
        AstExpr::Delay { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(inner.shift(lit(*period as i64)))
        }
        AstExpr::StdDev { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_std_dev(inner, *period))
        }
        AstExpr::LinRegSlope { source: _, period: _ } => Err(CoreError::Compilation("LinRegSlope not supported in Phase 2 Polars yet".into())),
        AstExpr::TrueRange => {
            Ok(crate::indicators::compute_true_range(col("high"), col("low"), col("close")))
        }

        // 4. Mathematical Operators
        AstExpr::Add { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            Ok(l + r)
        }
        AstExpr::Sub { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            Ok(l - r)
        }
        AstExpr::Mul { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            Ok(l * r)
        }
        AstExpr::Div { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            Ok(l / r)
        }
        AstExpr::Abs { source } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(when(inner.clone().lt(lit(0.0))).then(lit(0.0) - inner.clone()).otherwise(inner))
        }

        // 5. Logical Operators (Return Booleans)
        AstExpr::CrossAbove { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            let cross_above = l.clone().shift(lit(1))
                .lt(r.clone().shift(lit(1)))
                .and(l.gt(r));
            Ok(cross_above)
        }
        AstExpr::CrossBelow { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            let cross_below = l.clone().shift(lit(1))
                .gt(r.clone().shift(lit(1)))
                .and(l.lt(r));
            Ok(cross_below)
        }
        AstExpr::GreaterThan { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            Ok(l.gt(r))
        }
        AstExpr::LessThan { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            Ok(l.lt(r))
        }

        // 6. Conjunctions (Combine Booleans)
        AstExpr::And { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Boolean);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Boolean);
            Ok(l.and(r))
        }
        AstExpr::Or { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Boolean);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Boolean);
            Ok(l.or(r))
        }
        _ => Err(CoreError::Compilation("Not supported in Phase 2 Polars yet".into())),
    }
}

pub fn compile_ast_with_cache(expr: &AstExpr) -> Result<polars::lazy::dsl::Expr, CoreError> {
    use polars::lazy::dsl::*;
    match expr {
        AstExpr::Close => Ok(col("close")),
        AstExpr::Open => Ok(col("open")),
        AstExpr::High => Ok(col("high")),
        AstExpr::Low => Ok(col("low")),
        AstExpr::Volume => Ok(col("volume")),
        AstExpr::Constant { value } => Ok(lit(*value)),
        AstExpr::Sma { .. } | AstExpr::Ema { .. } | AstExpr::Wma { .. } | AstExpr::Rma { .. } | 
        AstExpr::TsMax { .. } | AstExpr::TsMin { .. } | AstExpr::TsSum { .. } | 
        AstExpr::Delay { .. } | AstExpr::StdDev { .. } | AstExpr::LinRegSlope { .. } | 
        AstExpr::TrueRange => {
            Ok(col(&expr.to_string()))
        }
        AstExpr::Add { lhs, rhs } => Ok(compile_ast_with_cache(lhs)? + compile_ast_with_cache(rhs)?),
        AstExpr::Sub { lhs, rhs } => Ok(compile_ast_with_cache(lhs)? - compile_ast_with_cache(rhs)?),
        AstExpr::Mul { lhs, rhs } => Ok(compile_ast_with_cache(lhs)? * compile_ast_with_cache(rhs)?),
        AstExpr::Div { lhs, rhs } => Ok(compile_ast_with_cache(lhs)? / compile_ast_with_cache(rhs)?),
        AstExpr::Abs { source } => {
            let inner = compile_ast_with_cache(source)?;
            Ok(when(inner.clone().lt(lit(0.0))).then(lit(0.0) - inner.clone()).otherwise(inner))
        },
        AstExpr::CrossAbove { lhs, rhs } => {
            let l = compile_ast_with_cache(lhs)?;
            let r = compile_ast_with_cache(rhs)?;
            Ok(l.clone().gt(r.clone()).and(l.shift(lit(1)).lt_eq(r.shift(lit(1)))))
        }
        AstExpr::CrossBelow { lhs, rhs } => {
            let l = compile_ast_with_cache(lhs)?;
            let r = compile_ast_with_cache(rhs)?;
            Ok(l.clone().lt(r.clone()).and(l.shift(lit(1)).gt_eq(r.shift(lit(1)))))
        }
        AstExpr::GreaterThan { lhs, rhs } => Ok(compile_ast_with_cache(lhs)?.gt(compile_ast_with_cache(rhs)?)),
        AstExpr::LessThan { lhs, rhs } => Ok(compile_ast_with_cache(lhs)?.lt(compile_ast_with_cache(rhs)?)),
        AstExpr::And { lhs, rhs } => Ok(compile_ast_with_cache(lhs)?.and(compile_ast_with_cache(rhs)?)),
        AstExpr::Or { lhs, rhs } => Ok(compile_ast_with_cache(lhs)?.or(compile_ast_with_cache(rhs)?)),
        _ => Err(CoreError::Compilation("Not supported in Phase 2 Polars yet".into())),
    }
}
