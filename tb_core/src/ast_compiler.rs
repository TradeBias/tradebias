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

        // 3. Indicators (Nodes)
        AstExpr::Sma { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_sma(inner, *period))
        }
        AstExpr::Ema { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_ema(inner, *period))
        }
        AstExpr::Rsi { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_rsi(inner, *period))
        }
        AstExpr::Macd { source, fast, slow, signal: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_macd(inner, *fast, *slow))
        }
        AstExpr::Atr { period: _ } => {
            Err(CoreError::Compilation("ATR compilation not fully implemented in mock".into()))
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

        // 5. Logical Operators (Return Booleans)
        AstExpr::CrossAbove { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            // CrossAbove logic: lhs[t-1] < rhs[t-1] AND lhs[t] > rhs[t]
            let cross_above = l.clone().shift(lit(1))
                .lt(r.clone().shift(lit(1)))
                .and(l.gt(r));
            Ok(cross_above)
        }
        AstExpr::CrossBelow { lhs, rhs } => {
            let l = compile_ast_to_polars(lhs)?.cast(DataType::Float64);
            let r = compile_ast_to_polars(rhs)?.cast(DataType::Float64);
            // CrossBelow logic: lhs[t-1] > rhs[t-1] AND lhs[t] < rhs[t]
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
    }
}

pub fn compile_ast_with_cache(expr: &crate::ast::Expr) -> Result<polars::lazy::dsl::Expr, CoreError> {
    use polars::lazy::dsl::*;
    match expr {
        crate::ast::Expr::Close => Ok(col("close")),
        crate::ast::Expr::Open => Ok(col("open")),
        crate::ast::Expr::High => Ok(col("high")),
        crate::ast::Expr::Low => Ok(col("low")),
        crate::ast::Expr::Volume => Ok(col("volume")),
        crate::ast::Expr::Constant { value } => Ok(lit(*value)),
        crate::ast::Expr::Sma { .. } | crate::ast::Expr::Ema { .. } | crate::ast::Expr::Rsi { .. } | crate::ast::Expr::Macd { .. } | crate::ast::Expr::Atr { .. } => {
            Ok(col(&expr.to_string()))
        }
        crate::ast::Expr::Add { lhs, rhs } => Ok(compile_ast_with_cache(lhs)? + compile_ast_with_cache(rhs)?),
        crate::ast::Expr::Sub { lhs, rhs } => Ok(compile_ast_with_cache(lhs)? - compile_ast_with_cache(rhs)?),
        crate::ast::Expr::CrossAbove { lhs, rhs } => {
            let l = compile_ast_with_cache(lhs)?;
            let r = compile_ast_with_cache(rhs)?;
            Ok(l.clone().gt(r.clone()).and(l.shift(lit(1)).lt_eq(r.shift(lit(1)))))
        }
        crate::ast::Expr::CrossBelow { lhs, rhs } => {
            let l = compile_ast_with_cache(lhs)?;
            let r = compile_ast_with_cache(rhs)?;
            Ok(l.clone().lt(r.clone()).and(l.shift(lit(1)).gt_eq(r.shift(lit(1)))))
        }
        crate::ast::Expr::GreaterThan { lhs, rhs } => Ok(compile_ast_with_cache(lhs)?.gt(compile_ast_with_cache(rhs)?)),
        crate::ast::Expr::LessThan { lhs, rhs } => Ok(compile_ast_with_cache(lhs)?.lt(compile_ast_with_cache(rhs)?)),
        crate::ast::Expr::And { lhs, rhs } => Ok(compile_ast_with_cache(lhs)?.and(compile_ast_with_cache(rhs)?)),
        crate::ast::Expr::Or { lhs, rhs } => Ok(compile_ast_with_cache(lhs)?.or(compile_ast_with_cache(rhs)?)),
    }
}
