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
        AstExpr::Wma { source, period: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Err(CoreError::Compilation("WMA not supported in Phase 2 Polars yet".into()))
        }
        AstExpr::Hma { source, period: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Err(CoreError::Compilation("HMA not supported in Phase 2 Polars yet".into()))
        }
        AstExpr::Dema { source, period: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Err(CoreError::Compilation("DEMA not supported in Phase 2 Polars yet".into()))
        }
        AstExpr::Tema { source, period: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Err(CoreError::Compilation("TEMA not supported in Phase 2 Polars yet".into()))
        }
        AstExpr::Kama { source, period: _, fast: _, slow: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Err(CoreError::Compilation("KAMA not supported in Phase 2 Polars yet".into()))
        }
        AstExpr::Smma { source, period: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Err(CoreError::Compilation("SMMA not supported in Phase 2 Polars yet".into()))
        }
        AstExpr::Alma { source, period: _, offset: _, sigma: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Err(CoreError::Compilation("ALMA not supported in Phase 2 Polars yet".into()))
        }
        AstExpr::Rsi { source, period } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(crate::indicators::compute_rsi(inner, *period))
        }
        AstExpr::MacdLine { source, fast, slow } => {
            // Placeholder: Polars native MACD is tricky. We'll return the source for now since this is mostly stubbed for Phase 2.
            let _ = fast; let _ = slow;
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(inner)
        }
        AstExpr::MacdSignal { source, fast, slow, signal } => {
            let _ = fast; let _ = slow; let _ = signal;
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(inner)
        }
        AstExpr::MacdHistogram { source, fast, slow, signal } => {
            let _ = fast; let _ = slow; let _ = signal;
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(inner)
        }
        AstExpr::BollingerUpper { source, period: _, std_dev: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(inner)
        }
        AstExpr::BollingerLower { source, period: _, std_dev: _ } => {
            let inner = compile_ast_to_polars(source)?.cast(DataType::Float64);
            Ok(inner)
        }
        AstExpr::KeltnerUpper { period: _, multiplier: _ } => Err(CoreError::Compilation("KeltnerUpper not implemented for Polars".into())),
        AstExpr::KeltnerLower { period: _, multiplier: _ } => Err(CoreError::Compilation("KeltnerLower not implemented for Polars".into())),
        AstExpr::DonchianUpper { period: _ } => Err(CoreError::Compilation("DonchianUpper not implemented for Polars".into())),
        AstExpr::DonchianLower { period: _ } => Err(CoreError::Compilation("DonchianLower not implemented for Polars".into())),
        AstExpr::DonchianMid { period: _ } => Err(CoreError::Compilation("DonchianMid not implemented for Polars".into())),
        AstExpr::Supertrend { period: _, multiplier: _ } => Err(CoreError::Compilation("Supertrend not implemented for Polars".into())),
        AstExpr::SupertrendDir { period: _, multiplier: _ } => Err(CoreError::Compilation("SupertrendDir not implemented for Polars".into())),
        AstExpr::Psar { af_step: _, af_max: _ } => Err(CoreError::Compilation("PSAR not implemented for Polars".into())),
        AstExpr::Adx { period: _ } => Err(CoreError::Compilation("ADX not implemented for Polars".into())),
        AstExpr::DiPlus { period: _ } => Err(CoreError::Compilation("DiPlus not implemented for Polars".into())),
        AstExpr::DiMinus { period: _ } => Err(CoreError::Compilation("DiMinus not implemented for Polars".into())),
        AstExpr::StochasticK { .. } => Err(CoreError::Compilation("StochasticK not implemented for Polars".into())),
        AstExpr::StochasticD { .. } => Err(CoreError::Compilation("StochasticD not implemented for Polars".into())),
        AstExpr::StochRsiK { .. } => Err(CoreError::Compilation("StochRsiK not implemented for Polars".into())),
        AstExpr::StochRsiD { .. } => Err(CoreError::Compilation("StochRsiD not implemented for Polars".into())),
        AstExpr::WilliamsR { .. } => Err(CoreError::Compilation("WilliamsR not implemented for Polars".into())),
        AstExpr::Cci { .. } => Err(CoreError::Compilation("CCI not implemented for Polars".into())),
        AstExpr::Mfi { .. } => Err(CoreError::Compilation("MFI not implemented for Polars".into())),
        AstExpr::Roc { .. } => Err(CoreError::Compilation("ROC not implemented for Polars".into())),
        AstExpr::AwesomeOscillator => Err(CoreError::Compilation("AwesomeOscillator not implemented for Polars".into())),
        AstExpr::Tsi { .. } => Err(CoreError::Compilation("TSI not implemented for Polars".into())),
        AstExpr::UltimateOscillator { .. } => Err(CoreError::Compilation("UltimateOscillator not implemented for Polars".into())),
        AstExpr::Dpo { .. } => Err(CoreError::Compilation("DPO not implemented for Polars".into())),
        AstExpr::Kst { .. } => Err(CoreError::Compilation("KST not implemented for Polars".into())),
        AstExpr::KstSignal { .. } => Err(CoreError::Compilation("KSTSignal not implemented for Polars".into())),
        AstExpr::FisherTransform { .. } => Err(CoreError::Compilation("FisherTransform not implemented for Polars".into())),
        AstExpr::FisherTrigger { .. } => Err(CoreError::Compilation("FisherTrigger not implemented for Polars".into())),
        AstExpr::ConnorsRsi { .. } => Err(CoreError::Compilation("ConnorsRSI not implemented for Polars".into())),
        AstExpr::Cmo { .. } => Err(CoreError::Compilation("CMO not implemented for Polars".into())),
        AstExpr::Rvi { .. } => Err(CoreError::Compilation("RVI not implemented for Polars".into())),
        AstExpr::RviSignal { .. } => Err(CoreError::Compilation("RVISignal not implemented for Polars".into())),
        AstExpr::Smi { .. } => Err(CoreError::Compilation("SMI not implemented for Polars".into())),
        AstExpr::Trix { .. } => Err(CoreError::Compilation("TRIX not implemented for Polars".into())),
        AstExpr::Eom { .. } => Err(CoreError::Compilation("EOM not implemented for Polars".into())),
        AstExpr::VortexPlus { .. } => Err(CoreError::Compilation("VortexPlus not implemented for Polars".into())),
        AstExpr::VortexMinus { .. } => Err(CoreError::Compilation("VortexMinus not implemented for Polars".into())),
        AstExpr::DssBressert { .. } => Err(CoreError::Compilation("DssBressert not implemented for Polars".into())),
        AstExpr::Ppo { .. } => Err(CoreError::Compilation("PPO not implemented for Polars".into())),
        AstExpr::PpoSignal { .. } => Err(CoreError::Compilation("PPOSignal not implemented for Polars".into())),
        AstExpr::PpoHist { .. } => Err(CoreError::Compilation("PPOHist not implemented for Polars".into())),
        AstExpr::ChoppinessIndex { .. } => Err(CoreError::Compilation("ChoppinessIndex not implemented for Polars".into())),
        AstExpr::QqeFast { .. } => Err(CoreError::Compilation("QQEFast not implemented for Polars".into())),
        AstExpr::QqeSlow { .. } => Err(CoreError::Compilation("QQESlow not implemented for Polars".into())),
        AstExpr::Stc { .. } => Err(CoreError::Compilation("STC not implemented for Polars".into())),
        AstExpr::ChaikinVolatility { .. } => Err(CoreError::Compilation("ChaikinVolatility not implemented for Polars".into())),
        AstExpr::HistoricalVolatility { .. } => Err(CoreError::Compilation("HistoricalVolatility not implemented for Polars".into())),
        AstExpr::UlcerIndex { .. } => Err(CoreError::Compilation("UlcerIndex not implemented for Polars".into())),
        AstExpr::StandardDeviation { .. } => Err(CoreError::Compilation("StandardDeviation not implemented for Polars".into())),
        AstExpr::BollingerBandWidth { .. } => Err(CoreError::Compilation("BollingerBandWidth not implemented for Polars".into())),
        AstExpr::BollingerPercentB { .. } => Err(CoreError::Compilation("BollingerPercentB not implemented for Polars".into())),
        AstExpr::KeltnerChannelWidth { .. } => Err(CoreError::Compilation("KeltnerChannelWidth not implemented for Polars".into())),
        AstExpr::VixSynthetic { .. } => Err(CoreError::Compilation("VixSynthetic not implemented for Polars".into())),
        AstExpr::Obv => Err(CoreError::Compilation("OBV not implemented for Polars".into())),
        AstExpr::Vwap => Err(CoreError::Compilation("VWAP not implemented for Polars".into())),
        AstExpr::AdLine => Err(CoreError::Compilation("ADLine not implemented for Polars".into())),
        AstExpr::ChaikinMoneyFlow { .. } => Err(CoreError::Compilation("ChaikinMoneyFlow not implemented for Polars".into())),
        AstExpr::ChaikinOscillator { .. } => Err(CoreError::Compilation("ChaikinOscillator not implemented for Polars".into())),
        AstExpr::Pvt => Err(CoreError::Compilation("PVT not implemented for Polars".into())),
        AstExpr::Nvi => Err(CoreError::Compilation("NVI not implemented for Polars".into())),
        AstExpr::Pvi => Err(CoreError::Compilation("PVI not implemented for Polars".into())),
        AstExpr::ForceIndex { .. } => Err(CoreError::Compilation("ForceIndex not implemented for Polars".into())),
        AstExpr::Vfi { .. } => Err(CoreError::Compilation("VFI not implemented for Polars".into())),
        AstExpr::VolumeOscillator { .. } => Err(CoreError::Compilation("VolumeOscillator not implemented for Polars".into())),
        AstExpr::KlingerOscillator { .. } => Err(CoreError::Compilation("KlingerOscillator not implemented for Polars".into())),
        AstExpr::Mvwap { .. } => Err(CoreError::Compilation("MVWAP not implemented for Polars".into())),
        AstExpr::Twap { .. } => Err(CoreError::Compilation("TWAP not implemented for Polars".into())),
        AstExpr::LinRegSlope { .. } => Err(CoreError::Compilation("LinRegSlope not implemented for Polars".into())),
        AstExpr::LinRegIntercept { .. } => Err(CoreError::Compilation("LinRegIntercept not implemented for Polars".into())),
        AstExpr::LinRegRSquared { .. } => Err(CoreError::Compilation("LinRegRSquared not implemented for Polars".into())),
        AstExpr::LinRegCurve { .. } => Err(CoreError::Compilation("LinRegCurve not implemented for Polars".into())),
        AstExpr::StdErrorBandUpper { .. } => Err(CoreError::Compilation("StdErrorBandUpper not implemented for Polars".into())),
        AstExpr::StdErrorBandLower { .. } => Err(CoreError::Compilation("StdErrorBandLower not implemented for Polars".into())),
        AstExpr::ZScore { .. } => Err(CoreError::Compilation("ZScore not implemented for Polars".into())),
        AstExpr::LogReturn => Err(CoreError::Compilation("LogReturn not implemented for Polars".into())),
        AstExpr::MedianPrice => Err(CoreError::Compilation("MedianPrice not implemented for Polars".into())),
        AstExpr::TypicalPrice => Err(CoreError::Compilation("TypicalPrice not implemented for Polars".into())),
        AstExpr::WeightedClose => Err(CoreError::Compilation("WeightedClose not implemented for Polars".into())),
        AstExpr::HurstExponent { .. } => Err(CoreError::Compilation("HurstExponent not implemented for Polars".into())),
        AstExpr::PivotPointsP { .. } => Err(CoreError::Compilation("PivotPointsP not implemented for Polars".into())),
        AstExpr::PivotPointsR1 { .. } => Err(CoreError::Compilation("PivotPointsR1 not implemented for Polars".into())),
        AstExpr::PivotPointsS1 { .. } => Err(CoreError::Compilation("PivotPointsS1 not implemented for Polars".into())),
        AstExpr::FibLevel236 { .. } => Err(CoreError::Compilation("FibLevel236 not implemented for Polars".into())),
        AstExpr::FibLevel382 { .. } => Err(CoreError::Compilation("FibLevel382 not implemented for Polars".into())),
        AstExpr::FibLevel500 { .. } => Err(CoreError::Compilation("FibLevel500 not implemented for Polars".into())),
        AstExpr::FibLevel618 { .. } => Err(CoreError::Compilation("FibLevel618 not implemented for Polars".into())),
        AstExpr::FibLevel786 { .. } => Err(CoreError::Compilation("FibLevel786 not implemented for Polars".into())),
        AstExpr::HeikinAshiClose => Err(CoreError::Compilation("HeikinAshiClose not implemented for Polars".into())),
        AstExpr::EhlersSuperSmoother { .. } => Err(CoreError::Compilation("EhlersSuperSmoother not implemented for Polars".into())),
        AstExpr::EhlersDecycler { .. } => Err(CoreError::Compilation("EhlersDecycler not implemented for Polars".into())),
        AstExpr::EhlersCyberCycle { .. } => Err(CoreError::Compilation("EhlersCyberCycle not implemented for Polars".into())),
        AstExpr::EhlersMama { .. } => Err(CoreError::Compilation("EhlersMama not implemented for Polars".into())),
        AstExpr::EhlersFama { .. } => Err(CoreError::Compilation("EhlersFama not implemented for Polars".into())),
        AstExpr::EhlersSine => Err(CoreError::Compilation("EhlersSine not implemented for Polars".into())),
        AstExpr::EhlersLeadSine => Err(CoreError::Compilation("EhlersLeadSine not implemented for Polars".into())),
        AstExpr::EhlersDecyclerOscillator { .. } => Err(CoreError::Compilation("EhlersDecyclerOscillator not implemented for Polars".into())),
        AstExpr::EhlersRoofingFilter { .. } => Err(CoreError::Compilation("EhlersRoofingFilter not implemented for Polars".into())),
        AstExpr::EhlersDominantCyclePeriod => Err(CoreError::Compilation("EhlersDominantCyclePeriod not implemented for Polars".into())),
        AstExpr::EhlersAutocorrelationPeriodogram => Err(CoreError::Compilation("EhlersAutocorrelationPeriodogram not implemented for Polars".into())),
        AstExpr::EhlersEmd { .. } => Err(CoreError::Compilation("EhlersEmd not implemented for Polars".into())),
        AstExpr::MarketMeannessIndex { .. } => Err(CoreError::Compilation("MarketMeannessIndex not implemented for Polars".into())),
        AstExpr::ZeroLagMacdLine { .. } => Err(CoreError::Compilation("ZeroLagMacdLine not implemented for Polars".into())),
        AstExpr::ZeroLagMacdSignal { .. } => Err(CoreError::Compilation("ZeroLagMacdSignal not implemented for Polars".into())),
        AstExpr::ZeroLagMacdHist { .. } => Err(CoreError::Compilation("ZeroLagMacdHist not implemented for Polars".into())),
        AstExpr::GatorTop => Err(CoreError::Compilation("GatorTop not implemented for Polars".into())),
        AstExpr::GatorBottom => Err(CoreError::Compilation("GatorBottom not implemented for Polars".into())),
        AstExpr::KalmanFilter { .. } => Err(CoreError::Compilation("KalmanFilter not implemented for Polars".into())),
        AstExpr::BullishEngulfing => Err(CoreError::Compilation("BullishEngulfing not implemented for Polars".into())),
        AstExpr::BearishEngulfing => Err(CoreError::Compilation("BearishEngulfing not implemented for Polars".into())),
        AstExpr::Doji => Err(CoreError::Compilation("Doji not implemented for Polars".into())),
        AstExpr::Hammer => Err(CoreError::Compilation("Hammer not implemented for Polars".into())),
        AstExpr::ShootingStar => Err(CoreError::Compilation("ShootingStar not implemented for Polars".into())),
        AstExpr::MorningStar => Err(CoreError::Compilation("MorningStar not implemented for Polars".into())),
        AstExpr::EveningStar => Err(CoreError::Compilation("EveningStar not implemented for Polars".into())),
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
        crate::ast::Expr::Sma { .. } | crate::ast::Expr::Ema { .. } | crate::ast::Expr::Wma { .. } | crate::ast::Expr::Hma { .. } | crate::ast::Expr::Dema { .. } | crate::ast::Expr::Tema { .. } | crate::ast::Expr::Kama { .. } | crate::ast::Expr::Smma { .. } | crate::ast::Expr::Alma { .. } | crate::ast::Expr::Rsi { .. } | 
        crate::ast::Expr::MacdLine { .. } | crate::ast::Expr::MacdSignal { .. } | crate::ast::Expr::MacdHistogram { .. } |
        crate::ast::Expr::BollingerUpper { .. } | crate::ast::Expr::BollingerLower { .. } | crate::ast::Expr::KeltnerUpper { .. } | crate::ast::Expr::KeltnerLower { .. } | crate::ast::Expr::DonchianUpper { .. } | crate::ast::Expr::DonchianLower { .. } | crate::ast::Expr::DonchianMid { .. } | crate::ast::Expr::Supertrend { .. } | crate::ast::Expr::SupertrendDir { .. } | crate::ast::Expr::Psar { .. } | crate::ast::Expr::Adx { .. } | crate::ast::Expr::DiPlus { .. } | crate::ast::Expr::DiMinus { .. } | crate::ast::Expr::StochasticK { .. } | crate::ast::Expr::StochasticD { .. } | crate::ast::Expr::StochRsiK { .. } | crate::ast::Expr::StochRsiD { .. } | crate::ast::Expr::WilliamsR { .. } | crate::ast::Expr::Cci { .. } | crate::ast::Expr::Mfi { .. } | crate::ast::Expr::Roc { .. } | crate::ast::Expr::AwesomeOscillator { .. } | crate::ast::Expr::Tsi { .. } | crate::ast::Expr::UltimateOscillator { .. } | crate::ast::Expr::Dpo { .. } | crate::ast::Expr::Kst { .. } | crate::ast::Expr::KstSignal { .. } | crate::ast::Expr::FisherTransform { .. } | crate::ast::Expr::FisherTrigger { .. } | crate::ast::Expr::ConnorsRsi { .. } | crate::ast::Expr::Cmo { .. } | crate::ast::Expr::Rvi { .. } | crate::ast::Expr::RviSignal { .. } | crate::ast::Expr::Smi { .. } | crate::ast::Expr::Trix { .. } | crate::ast::Expr::Eom { .. } | crate::ast::Expr::VortexPlus { .. } | crate::ast::Expr::VortexMinus { .. } | crate::ast::Expr::DssBressert { .. } | crate::ast::Expr::Ppo { .. } | crate::ast::Expr::PpoSignal { .. } | crate::ast::Expr::PpoHist { .. } | crate::ast::Expr::ChoppinessIndex { .. } | crate::ast::Expr::QqeFast { .. } | crate::ast::Expr::QqeSlow { .. } | crate::ast::Expr::Stc { .. } | crate::ast::Expr::ChaikinVolatility { .. } | crate::ast::Expr::HistoricalVolatility { .. } | crate::ast::Expr::UlcerIndex { .. } | crate::ast::Expr::StandardDeviation { .. } | crate::ast::Expr::BollingerBandWidth { .. } | crate::ast::Expr::BollingerPercentB { .. } | crate::ast::Expr::KeltnerChannelWidth { .. } | crate::ast::Expr::VixSynthetic { .. } |        crate::ast::Expr::Obv | crate::ast::Expr::Vwap | crate::ast::Expr::AdLine | crate::ast::Expr::ChaikinMoneyFlow { .. } | crate::ast::Expr::ChaikinOscillator { .. } | crate::ast::Expr::Pvt | crate::ast::Expr::Nvi | crate::ast::Expr::Pvi | crate::ast::Expr::ForceIndex { .. } | crate::ast::Expr::Vfi { .. } | crate::ast::Expr::VolumeOscillator { .. } | crate::ast::Expr::KlingerOscillator { .. } | crate::ast::Expr::Mvwap { .. } | crate::ast::Expr::Twap { .. } | crate::ast::Expr::LinRegSlope { .. } | crate::ast::Expr::LinRegIntercept { .. } | crate::ast::Expr::LinRegRSquared { .. } | crate::ast::Expr::LinRegCurve { .. } | crate::ast::Expr::StdErrorBandUpper { .. } | crate::ast::Expr::StdErrorBandLower { .. } | crate::ast::Expr::ZScore { .. } | crate::ast::Expr::LogReturn | crate::ast::Expr::MedianPrice | crate::ast::Expr::TypicalPrice | crate::ast::Expr::WeightedClose | crate::ast::Expr::HurstExponent { .. } | crate::ast::Expr::PivotPointsP { .. } | crate::ast::Expr::PivotPointsR1 { .. } | crate::ast::Expr::PivotPointsS1 { .. } | crate::ast::Expr::FibLevel236 { .. } | crate::ast::Expr::FibLevel382 { .. } | crate::ast::Expr::FibLevel500 { .. } | crate::ast::Expr::FibLevel618 { .. } | crate::ast::Expr::FibLevel786 { .. } | crate::ast::Expr::HeikinAshiClose | crate::ast::Expr::EhlersSuperSmoother { .. } | crate::ast::Expr::EhlersDecycler { .. } | crate::ast::Expr::EhlersCyberCycle { .. } | crate::ast::Expr::EhlersMama { .. } | crate::ast::Expr::EhlersFama { .. } | crate::ast::Expr::EhlersSine | crate::ast::Expr::EhlersLeadSine | crate::ast::Expr::EhlersDecyclerOscillator { .. } | crate::ast::Expr::EhlersRoofingFilter { .. } | crate::ast::Expr::EhlersDominantCyclePeriod | crate::ast::Expr::EhlersAutocorrelationPeriodogram | crate::ast::Expr::EhlersEmd { .. } | crate::ast::Expr::MarketMeannessIndex { .. } | crate::ast::Expr::ZeroLagMacdLine { .. } | crate::ast::Expr::ZeroLagMacdSignal { .. } | crate::ast::Expr::ZeroLagMacdHist { .. } | crate::ast::Expr::GatorTop | crate::ast::Expr::GatorBottom | crate::ast::Expr::KalmanFilter { .. } | crate::ast::Expr::BullishEngulfing | crate::ast::Expr::BearishEngulfing | crate::ast::Expr::Doji | crate::ast::Expr::Hammer | crate::ast::Expr::ShootingStar | crate::ast::Expr::MorningStar | crate::ast::Expr::EveningStar | crate::ast::Expr::Atr { .. } => {
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
