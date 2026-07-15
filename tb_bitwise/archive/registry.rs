use crate::precompute::{BaseArray, SemanticType};

pub struct PriceData<'a> {
    pub open: &'a [f64],
    pub high: &'a [f64],
    pub low: &'a [f64],
    pub close: &'a [f64],
    pub volume: &'a [f64],
}

pub struct IndicatorSpec {
    pub name: &'static str,
    pub category: &'static str,
}

/// Returns a list of all supported indicators by the Bitwise Engine.
pub fn get_available_indicators() -> Vec<IndicatorSpec> {
    vec![
        IndicatorSpec { name: "SMA", category: "Trend" },
        IndicatorSpec { name: "EMA", category: "Trend" },
        IndicatorSpec { name: "WMA", category: "Trend" },
        IndicatorSpec { name: "HMA", category: "Trend" },
        IndicatorSpec { name: "DEMA", category: "Trend" },
        IndicatorSpec { name: "TEMA", category: "Trend" },
        IndicatorSpec { name: "KAMA", category: "Trend" },
        IndicatorSpec { name: "SMMA", category: "Trend" },
        IndicatorSpec { name: "ALMA", category: "Trend" },
        IndicatorSpec { name: "Keltner", category: "Trend" },
        IndicatorSpec { name: "Donchian", category: "Trend" },
        IndicatorSpec { name: "Supertrend", category: "Trend" },
        IndicatorSpec { name: "PSAR", category: "Trend" },
        IndicatorSpec { name: "ADX", category: "Trend" },
        IndicatorSpec { name: "RSI", category: "Oscillators" },
        IndicatorSpec { name: "MACD", category: "Oscillators" },
        IndicatorSpec { name: "Stochastic", category: "Oscillators" },
        IndicatorSpec { name: "StochRSI", category: "Oscillators" },
        IndicatorSpec { name: "WilliamsR", category: "Oscillators" },
        IndicatorSpec { name: "CCI", category: "Oscillators" },
        IndicatorSpec { name: "MFI", category: "Oscillators" },
        IndicatorSpec { name: "ROC", category: "Oscillators" },
        IndicatorSpec { name: "AwesomeOscillator", category: "Oscillators" },
        IndicatorSpec { name: "TSI", category: "Oscillators" },
        IndicatorSpec { name: "UltimateOscillator", category: "Oscillators" },
        IndicatorSpec { name: "DPO", category: "Oscillators" },
        IndicatorSpec { name: "KST", category: "Oscillators" },
        IndicatorSpec { name: "Fisher", category: "Oscillators" },
        IndicatorSpec { name: "ConnorsRSI", category: "Oscillators" },
        IndicatorSpec { name: "CMO", category: "Oscillators" },
        IndicatorSpec { name: "RVI", category: "Oscillators" },
        IndicatorSpec { name: "SMI", category: "Oscillators" },
        IndicatorSpec { name: "TRIX", category: "Oscillators" },
        IndicatorSpec { name: "EOM", category: "Oscillators" },
        IndicatorSpec { name: "VORTEX", category: "Oscillators" },
        IndicatorSpec { name: "DSS", category: "Oscillators" },
        IndicatorSpec { name: "PPO", category: "Oscillators" },
        IndicatorSpec { name: "CHOP", category: "Oscillators" },
        IndicatorSpec { name: "QQE", category: "Oscillators" },
        IndicatorSpec { name: "STC", category: "Oscillators" },
        IndicatorSpec { name: "Bollinger", category: "Volatility" },
        IndicatorSpec { name: "ATR", category: "Volatility" },
        IndicatorSpec { name: "CHAIKIN_VOL", category: "Volatility" },
        IndicatorSpec { name: "HV", category: "Volatility" },
        IndicatorSpec { name: "ULCER", category: "Volatility" },
        IndicatorSpec { name: "STDDEV", category: "Volatility" },
        IndicatorSpec { name: "BB_WIDTH", category: "Volatility" },
        IndicatorSpec { name: "BB_PERCENT_B", category: "Volatility" },
        IndicatorSpec { name: "KC_WIDTH", category: "Volatility" },
        IndicatorSpec { name: "VIX_SYNTH", category: "Volatility" },
        IndicatorSpec { name: "OBV", category: "Volume" },
        IndicatorSpec { name: "VWAP", category: "Volume" },
        IndicatorSpec { name: "AD_LINE", category: "Volume" },
        IndicatorSpec { name: "CMF", category: "Volume" },
        IndicatorSpec { name: "CHAIKIN_OSC", category: "Volume" },
        IndicatorSpec { name: "PVT", category: "Volume" },
        IndicatorSpec { name: "NVI", category: "Volume" },
        IndicatorSpec { name: "PVI", category: "Volume" },
        IndicatorSpec { name: "FORCE_INDEX", category: "Volume" },
        IndicatorSpec { name: "VFI", category: "Volume" },
        IndicatorSpec { name: "VOL_OSC", category: "Volume" },
        IndicatorSpec { name: "KLINGER", category: "Volume" },
        IndicatorSpec { name: "MVWAP", category: "Volume" },
        IndicatorSpec { name: "TWAP", category: "Volume" },
        IndicatorSpec { name: "LINREG_SLOPE", category: "Statistical" },
        IndicatorSpec { name: "LINREG_INTERCEPT", category: "Statistical" },
        IndicatorSpec { name: "LINREG_R2", category: "Statistical" },
        IndicatorSpec { name: "LINREG_CURVE", category: "Statistical" },
        IndicatorSpec { name: "STD_ERROR_BANDS", category: "Statistical" },
        IndicatorSpec { name: "Z_SCORE", category: "Statistical" },
        IndicatorSpec { name: "LOG_RETURN", category: "Statistical" },
        IndicatorSpec { name: "MEDIAN_PRICE", category: "Statistical" },
        IndicatorSpec { name: "TYPICAL_PRICE", category: "Statistical" },
        IndicatorSpec { name: "WEIGHTED_CLOSE", category: "Statistical" },
        IndicatorSpec { name: "HURST_EXPONENT", category: "Statistical" },
        IndicatorSpec { name: "PIVOT_POINTS", category: "Statistical" },
        IndicatorSpec { name: "FIB_RETRACEMENTS", category: "Statistical" },
        IndicatorSpec { name: "HEIKIN_ASHI", category: "Statistical" },
        IndicatorSpec { name: "EHLERS_SUPER_SMOOTHER", category: "Advanced DSP" },
        IndicatorSpec { name: "EHLERS_DECYCLER", category: "Advanced DSP" },
        IndicatorSpec { name: "EHLERS_CYBER_CYCLE", category: "Advanced DSP" },
        IndicatorSpec { name: "EHLERS_MAMA_FAMA", category: "Advanced DSP" },
        IndicatorSpec { name: "EHLERS_SINE_WAVE", category: "Advanced DSP" },
        IndicatorSpec { name: "EHLERS_DECYCLER_OSCILLATOR", category: "Advanced DSP" },
        IndicatorSpec { name: "EHLERS_ROOFING_FILTER", category: "Advanced DSP" },
        IndicatorSpec { name: "EHLERS_DOMINANT_CYCLE_PERIOD", category: "Advanced DSP" },
        IndicatorSpec { name: "EHLERS_AUTOCORRELATION_PERIODOGRAM", category: "Advanced DSP" },
        IndicatorSpec { name: "EHLERS_EMD", category: "Advanced DSP" },
        IndicatorSpec { name: "MARKET_MEANNESS_INDEX", category: "Advanced DSP" },
        IndicatorSpec { name: "ZERO_LAG_MACD", category: "Advanced DSP" },
        IndicatorSpec { name: "GATOR_OSCILLATOR", category: "Advanced DSP" },
        IndicatorSpec { name: "KALMAN_FILTER", category: "Advanced DSP" },
        IndicatorSpec { name: "BULLISH_ENGULFING", category: "Patterns" },
        IndicatorSpec { name: "BEARISH_ENGULFING", category: "Patterns" },
        IndicatorSpec { name: "DOJI", category: "Patterns" },
        IndicatorSpec { name: "HAMMER", category: "Patterns" },
        IndicatorSpec { name: "SHOOTING_STAR", category: "Patterns" },
        IndicatorSpec { name: "MORNING_STAR", category: "Patterns" },
        IndicatorSpec { name: "EVENING_STAR", category: "Patterns" },
    ]
}

/// Builds all permutations of BaseArrays for the given permitted indicators.
pub fn build_base_arrays(permitted_indicators: &[String], data: &PriceData) -> Vec<BaseArray> {
    let mut base_arrays = Vec::new();

    // 1. Core Price Data (Always generated)
    base_arrays.push(BaseArray {
        name: "Close".to_string(),
        semantic_type: SemanticType::Price,
        ast: tb_core::ast::Expr::Close,
        data: data.close.to_vec(),
    });
    base_arrays.push(BaseArray {
        name: "High".to_string(),
        semantic_type: SemanticType::Price,
        ast: tb_core::ast::Expr::High,
        data: data.high.to_vec(),
    });
    base_arrays.push(BaseArray {
        name: "Low".to_string(),
        semantic_type: SemanticType::Price,
        ast: tb_core::ast::Expr::Low,
        data: data.low.to_vec(),
    });

    // 2. Indicators
    if permitted_indicators.contains(&"SMA".to_string()) {
        for period in [5, 10, 20, 50, 200] {
            base_arrays.push(BaseArray {
                name: format!("SMA_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Sma { source: Box::new(tb_core::ast::Expr::Close), period },
                data: tb_math::indicators::sma(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"EMA".to_string()) {
        for period in [9, 21, 50] {
            base_arrays.push(BaseArray {
                name: format!("EMA_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Ema { source: Box::new(tb_core::ast::Expr::Close), period },
                data: tb_math::indicators::ema(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"WMA".to_string()) {
        for period in [9u32, 21u32, 50u32] {
            base_arrays.push(BaseArray {
                name: format!("WMA_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Wma { source: Box::new(tb_core::ast::Expr::Close), period },
                data: tb_math::indicators::wma(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"HMA".to_string()) {
        for period in [14u32, 21u32, 55u32] {
            base_arrays.push(BaseArray {
                name: format!("HMA_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Hma { source: Box::new(tb_core::ast::Expr::Close), period },
                data: tb_math::indicators::hma(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"DEMA".to_string()) {
        for period in [9u32, 21u32, 50u32] {
            base_arrays.push(BaseArray {
                name: format!("DEMA_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Dema { source: Box::new(tb_core::ast::Expr::Close), period },
                data: tb_math::indicators::dema(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"TEMA".to_string()) {
        for period in [9u32, 21u32, 50u32] {
            base_arrays.push(BaseArray {
                name: format!("TEMA_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Tema { source: Box::new(tb_core::ast::Expr::Close), period },
                data: tb_math::indicators::tema(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"KAMA".to_string()) {
        for period in [10u32, 21u32] {
            base_arrays.push(BaseArray {
                name: format!("KAMA_{}_2_30", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Kama { source: Box::new(tb_core::ast::Expr::Close), period, fast: 2, slow: 30 },
                data: tb_math::indicators::kama(data.close, period as usize, 2, 30),
            });
        }
    }

    if permitted_indicators.contains(&"SMMA".to_string()) {
        for period in [20u32, 50u32] {
            base_arrays.push(BaseArray {
                name: format!("SMMA_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Smma { source: Box::new(tb_core::ast::Expr::Close), period },
                data: tb_math::indicators::smma(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"ALMA".to_string()) {
        for period in [9u32, 21u32] {
            base_arrays.push(BaseArray {
                name: format!("ALMA_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Alma { source: Box::new(tb_core::ast::Expr::Close), period, offset: 0.85, sigma: 6.0 },
                data: tb_math::indicators::alma(data.close, period as usize, 0.85, 6.0),
            });
        }
    }

    if permitted_indicators.contains(&"Keltner".to_string()) {
        for period in [20u32] {
            let (upper, lower, mid) = tb_math::indicators::keltner_channels(data.high, data.low, data.close, period as usize, 2.0);
            base_arrays.push(BaseArray {
                name: format!("KeltnerUpper_{}_2.0", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::KeltnerUpper { period, multiplier: 2.0 },
                data: upper,
            });
            base_arrays.push(BaseArray {
                name: format!("KeltnerLower_{}_2.0", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::KeltnerLower { period, multiplier: 2.0 },
                data: lower,
            });
        }
    }

    if permitted_indicators.contains(&"Donchian".to_string()) {
        for period in [20u32] {
            let (upper, mid, lower) = tb_math::indicators::donchian_channels(data.high, data.low, period as usize);
            base_arrays.push(BaseArray {
                name: format!("DonchianUpper_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::DonchianUpper { period },
                data: upper,
            });
            base_arrays.push(BaseArray {
                name: format!("DonchianLower_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::DonchianLower { period },
                data: lower,
            });
            base_arrays.push(BaseArray {
                name: format!("DonchianMid_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::DonchianMid { period },
                data: mid,
            });
        }
    }

    if permitted_indicators.contains(&"Supertrend".to_string()) {
        for period in [10u32] {
            let (st, dir) = tb_math::indicators::supertrend(data.high, data.low, data.close, period as usize, 3.0);
            base_arrays.push(BaseArray {
                name: format!("Supertrend_{}_3.0", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Supertrend { period, multiplier: 3.0 },
                data: st,
            });
            base_arrays.push(BaseArray {
                name: format!("SupertrendDir_{}_3.0", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::SupertrendDir { period, multiplier: 3.0 },
                data: dir,
            });
        }
    }

    if permitted_indicators.contains(&"PSAR".to_string()) {
        base_arrays.push(BaseArray {
            name: "PSAR_0.02_0.2".to_string(),
            semantic_type: SemanticType::Price,
            ast: tb_core::ast::Expr::Psar { af_step: 0.02, af_max: 0.2 },
            data: tb_math::indicators::psar(data.high, data.low, 0.02, 0.2),
        });
    }

    if permitted_indicators.contains(&"ADX".to_string()) {
        for period in [14u32] {
            let (adx_line, plus_di, minus_di) = tb_math::indicators::adx(data.high, data.low, data.close, period as usize);
            base_arrays.push(BaseArray {
                name: format!("ADX_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Adx { period },
                data: adx_line,
            });
            base_arrays.push(BaseArray {
                name: format!("DI+_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::DiPlus { period },
                data: plus_di,
            });
            base_arrays.push(BaseArray {
                name: format!("DI-_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::DiMinus { period },
                data: minus_di,
            });
        }
    }

    if permitted_indicators.contains(&"RSI".to_string()) {
        for period in [9, 14, 21] {
            base_arrays.push(BaseArray {
                name: format!("RSI_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Rsi { source: Box::new(tb_core::ast::Expr::Close), period },
                data: tb_math::indicators::rsi(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"Stochastic".to_string()) {
        for period in [14u32] {
            let (k, d) = tb_math::indicators::stochastic(data.high, data.low, data.close, period as usize, 3, 3);
            base_arrays.push(BaseArray {
                name: format!("StochK_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::StochasticK { period, k_period: 3, d_period: 3 },
                data: k,
            });
            base_arrays.push(BaseArray {
                name: format!("StochD_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::StochasticD { period, k_period: 3, d_period: 3 },
                data: d,
            });
        }
    }

    if permitted_indicators.contains(&"StochRSI".to_string()) {
        for period in [14u32] {
            let (k, d) = tb_math::indicators::stoch_rsi(data.close, period as usize, period as usize, 3, 3);
            base_arrays.push(BaseArray {
                name: format!("StochRsiK_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::StochRsiK { rsi_period: period, stoch_period: period, k_period: 3, d_period: 3 },
                data: k,
            });
            base_arrays.push(BaseArray {
                name: format!("StochRsiD_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::StochRsiD { rsi_period: period, stoch_period: period, k_period: 3, d_period: 3 },
                data: d,
            });
        }
    }

    if permitted_indicators.contains(&"WilliamsR".to_string()) {
        for period in [14u32] {
            base_arrays.push(BaseArray {
                name: format!("WilliamsR_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::WilliamsR { period },
                data: tb_math::indicators::williams_r(data.high, data.low, data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"CCI".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("CCI_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Cci { period },
                data: tb_math::indicators::cci(data.high, data.low, data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"MFI".to_string()) {
        for period in [14u32] {
            base_arrays.push(BaseArray {
                name: format!("MFI_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Mfi { period },
                data: tb_math::indicators::mfi(data.high, data.low, data.close, data.volume, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"ROC".to_string()) {
        for period in [9u32] {
            base_arrays.push(BaseArray {
                name: format!("ROC_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Roc { period },
                data: tb_math::indicators::roc(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"AwesomeOscillator".to_string()) {
        base_arrays.push(BaseArray {
            name: "AwesomeOscillator".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::AwesomeOscillator,
            data: tb_math::indicators::awesome_oscillator(data.high, data.low),
        });
    }

    if permitted_indicators.contains(&"TSI".to_string()) {
        for period in [25u32] {
            base_arrays.push(BaseArray {
                name: format!("TSI_{}_13", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Tsi { long_period: period, short_period: 13 },
                data: tb_math::indicators::tsi(data.close, period as usize, 13),
            });
        }
    }

    if permitted_indicators.contains(&"UltimateOscillator".to_string()) {
        base_arrays.push(BaseArray {
            name: "UltimateOscillator_7_14_28".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::UltimateOscillator { p1: 7, p2: 14, p3: 28 },
            data: tb_math::indicators::ultimate_oscillator(data.high, data.low, data.close, 7, 14, 28),
        });
    }

    if permitted_indicators.contains(&"DPO".to_string()) {
        for period in [21u32] {
            base_arrays.push(BaseArray {
                name: format!("DPO_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Dpo { period },
                data: tb_math::indicators::dpo(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"KST".to_string()) {
        let (kst, kst_sig) = tb_math::indicators::kst(data.close, 10, 15, 20, 30, 10, 10, 10, 15);
        base_arrays.push(BaseArray {
            name: "KST_10_15_20_30_10_10_10_15".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::Kst { r1: 10, r2: 15, r3: 20, r4: 30, s1: 10, s2: 10, s3: 10, s4: 15 },
            data: kst,
        });
        base_arrays.push(BaseArray {
            name: "KST_Signal_10_15_20_30_10_10_10_15".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::KstSignal { r1: 10, r2: 15, r3: 20, r4: 30, s1: 10, s2: 10, s3: 10, s4: 15 },
            data: kst_sig,
        });
    }

    if permitted_indicators.contains(&"Fisher".to_string()) {
        for period in [9u32] {
            let (fisher, trigger) = tb_math::indicators::fisher_transform(data.high, data.low, period as usize);
            base_arrays.push(BaseArray {
                name: format!("Fisher_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::FisherTransform { period },
                data: fisher,
            });
            base_arrays.push(BaseArray {
                name: format!("FisherTrigger_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::FisherTrigger { period },
                data: trigger,
            });
        }
    }

    if permitted_indicators.contains(&"ConnorsRSI".to_string()) {
        base_arrays.push(BaseArray {
            name: "CRSI_3_2_100".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::ConnorsRsi { rsi_period: 3, streak_rsi_period: 2, percent_rank_period: 100 },
            data: tb_math::indicators::connors_rsi(data.close, 3, 2, 100),
        });
    }

    if permitted_indicators.contains(&"CMO".to_string()) {
        for period in [9u32] {
            base_arrays.push(BaseArray {
                name: format!("CMO_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Cmo { period },
                data: tb_math::indicators::cmo(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"RVI".to_string()) {
        for period in [10u32] {
            let (rvi, rvi_sig) = tb_math::indicators::rvi(data.open, data.high, data.low, data.close, period as usize);
            base_arrays.push(BaseArray {
                name: format!("RVI_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Rvi { period },
                data: rvi,
            });
            base_arrays.push(BaseArray {
                name: format!("RVISignal_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::RviSignal { period },
                data: rvi_sig,
            });
        }
    }

    if permitted_indicators.contains(&"SMI".to_string()) {
        for period in [10u32] {
            base_arrays.push(BaseArray {
                name: format!("SMI_{}_3_3", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Smi { period, ema1_p: 3, ema2_p: 3 },
                data: tb_math::indicators::smi(data.high, data.low, data.close, period as usize, 3, 3),
            });
        }
    }

    if permitted_indicators.contains(&"TRIX".to_string()) {
        for period in [18u32] {
            base_arrays.push(BaseArray {
                name: format!("TRIX_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Trix { period },
                data: tb_math::indicators::trix(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"EOM".to_string()) {
        for period in [14u32] {
            base_arrays.push(BaseArray {
                name: format!("EOM_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Eom { period },
                data: tb_math::indicators::eom(data.high, data.low, data.volume, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"VORTEX".to_string()) {
        for period in [14u32] {
            let (v_plus, v_minus) = tb_math::indicators::vortex(data.high, data.low, data.close, period as usize);
            base_arrays.push(BaseArray {
                name: format!("VortexPlus_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::VortexPlus { period },
                data: v_plus,
            });
            base_arrays.push(BaseArray {
                name: format!("VortexMinus_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::VortexMinus { period },
                data: v_minus,
            });
        }
    }

    if permitted_indicators.contains(&"DSS".to_string()) {
        for period in [10u32] {
            base_arrays.push(BaseArray {
                name: format!("DSS_{}_3", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::DssBressert { period, ema_period: 3 },
                data: tb_math::indicators::dss_bressert(data.high, data.low, data.close, period as usize, 3),
            });
        }
    }

    if permitted_indicators.contains(&"PPO".to_string()) {
        let (ppo, ppo_sig, ppo_hist) = tb_math::indicators::ppo(data.close, 12, 26, 9);
        base_arrays.push(BaseArray {
            name: "PPO_12_26".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::Ppo { fast: 12, slow: 26, signal: 9 },
            data: ppo,
        });
        base_arrays.push(BaseArray {
            name: "PPOSig_12_26_9".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::PpoSignal { fast: 12, slow: 26, signal: 9 },
            data: ppo_sig,
        });
        base_arrays.push(BaseArray {
            name: "PPOHist_12_26_9".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::PpoHist { fast: 12, slow: 26, signal: 9 },
            data: ppo_hist,
        });
    }

    if permitted_indicators.contains(&"CHOP".to_string()) {
        for period in [14u32] {
            base_arrays.push(BaseArray {
                name: format!("CHOP_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::ChoppinessIndex { period },
                data: tb_math::indicators::choppiness_index(data.high, data.low, data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"QQE".to_string()) {
        for period in [14u32] {
            let (qqe_fast, qqe_slow) = tb_math::indicators::qqe(data.close, period as usize, 5, 4.236);
            base_arrays.push(BaseArray {
                name: format!("QQEFast_{}_5", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::QqeFast { period, sf: 5 },
                data: qqe_fast,
            });
            base_arrays.push(BaseArray {
                name: format!("QQESlow_{}_5", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::QqeSlow { period, sf: 5 },
                data: qqe_slow,
            });
        }
    }

    if permitted_indicators.contains(&"STC".to_string()) {
        for period in [10u32] {
            base_arrays.push(BaseArray {
                name: format!("STC_23_50_{}_3", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Stc { fast: 23, slow: 50, period, ema_period: 3 },
                data: tb_math::indicators::stc(data.close, 23, 50, period as usize, 3),
            });
        }
    }

    if permitted_indicators.contains(&"CHAIKIN_VOL".to_string()) {
        for period in [10u32] {
            base_arrays.push(BaseArray {
                name: format!("ChaikinVol_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::ChaikinVolatility { ema_period: period, roc_period: period },
                data: tb_math::indicators::chaikin_volatility(data.high, data.low, period as usize, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"HV".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("HV_{}_365", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::HistoricalVolatility { period, annual_factor: 365.0 },
                data: tb_math::indicators::historical_volatility(data.close, period as usize, 365.0),
            });
        }
    }

    if permitted_indicators.contains(&"ULCER".to_string()) {
        for period in [14u32] {
            base_arrays.push(BaseArray {
                name: format!("Ulcer_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::UlcerIndex { period },
                data: tb_math::indicators::ulcer_index(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"STDDEV".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("StdDev_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::StandardDeviation { period },
                data: tb_math::indicators::standard_deviation(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"BB_WIDTH".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("BBWidth_{}_2.0", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::BollingerBandWidth { period, std_dev: 2.0 },
                data: tb_math::indicators::bollinger_band_width(data.close, period as usize, 2.0),
            });
        }
    }

    if permitted_indicators.contains(&"BB_PERCENT_B".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("BBPercentB_{}_2.0", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::BollingerPercentB { period, std_dev: 2.0 },
                data: tb_math::indicators::bollinger_percent_b(data.close, period as usize, 2.0),
            });
        }
    }

    if permitted_indicators.contains(&"KC_WIDTH".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("KCWidth_{}_2.0", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::KeltnerChannelWidth { period, multiplier: 2.0 },
                data: tb_math::indicators::keltner_channel_width(data.high, data.low, data.close, period as usize, 2.0),
            });
        }
    }

    if permitted_indicators.contains(&"VIX_SYNTH".to_string()) {
        for period in [22u32] {
            base_arrays.push(BaseArray {
                name: format!("VixSynth_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::VixSynthetic { period },
                data: tb_math::indicators::vix_synthetic(data.close, data.low, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"OBV".to_string()) {
        base_arrays.push(BaseArray {
            name: "OBV".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::Obv,
            data: tb_math::indicators::obv(data.close, data.volume),
        });
    }

    if permitted_indicators.contains(&"VWAP".to_string()) {
        base_arrays.push(BaseArray {
            name: "VWAP".to_string(),
            semantic_type: SemanticType::Price,
            ast: tb_core::ast::Expr::Vwap,
            data: tb_math::indicators::vwap(data.high, data.low, data.close, data.volume),
        });
    }

    if permitted_indicators.contains(&"AD_LINE".to_string()) {
        base_arrays.push(BaseArray {
            name: "ADLine".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::AdLine,
            data: tb_math::indicators::ad_line(data.high, data.low, data.close, data.volume),
        });
    }

    if permitted_indicators.contains(&"CMF".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("CMF_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::ChaikinMoneyFlow { period },
                data: tb_math::indicators::chaikin_money_flow(data.high, data.low, data.close, data.volume, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"CHAIKIN_OSC".to_string()) {
        base_arrays.push(BaseArray {
            name: "ChaikinOsc_3_10".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::ChaikinOscillator { fast: 3, slow: 10 },
            data: tb_math::indicators::chaikin_oscillator(data.high, data.low, data.close, data.volume, 3, 10),
        });
    }

    if permitted_indicators.contains(&"PVT".to_string()) {
        base_arrays.push(BaseArray {
            name: "PVT".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::Pvt,
            data: tb_math::indicators::pvt(data.close, data.volume),
        });
    }

    if permitted_indicators.contains(&"NVI".to_string()) {
        base_arrays.push(BaseArray {
            name: "NVI".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::Nvi,
            data: tb_math::indicators::nvi(data.close, data.volume),
        });
    }

    if permitted_indicators.contains(&"PVI".to_string()) {
        base_arrays.push(BaseArray {
            name: "PVI".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::Pvi,
            data: tb_math::indicators::pvi(data.close, data.volume),
        });
    }

    if permitted_indicators.contains(&"FORCE_INDEX".to_string()) {
        for period in [13u32] {
            base_arrays.push(BaseArray {
                name: format!("ForceIndex_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::ForceIndex { period },
                data: tb_math::indicators::force_index(data.close, data.volume, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"VFI".to_string()) {
        for period in [130u32] {
            base_arrays.push(BaseArray {
                name: format!("VFI_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::Vfi { period },
                data: tb_math::indicators::vfi(data.high, data.low, data.close, data.volume, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"VOL_OSC".to_string()) {
        base_arrays.push(BaseArray {
            name: "VolOsc_12_26".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::VolumeOscillator { fast: 12, slow: 26 },
            data: tb_math::indicators::volume_oscillator(data.volume, 12, 26),
        });
    }

    if permitted_indicators.contains(&"KLINGER".to_string()) {
        base_arrays.push(BaseArray {
            name: "Klinger_34_55".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::KlingerOscillator { fast: 34, slow: 55 },
            data: tb_math::indicators::klinger_oscillator(data.high, data.low, data.close, data.volume, 34, 55),
        });
    }

    if permitted_indicators.contains(&"MVWAP".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("MVWAP_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Mvwap { period },
                data: tb_math::indicators::mvwap(data.high, data.low, data.close, data.volume, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"TWAP".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("TWAP_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::Twap { period },
                data: tb_math::indicators::twap(data.high, data.low, data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"LINREG_SLOPE".to_string()) {
        for period in [14u32, 20] {
            base_arrays.push(BaseArray {
                name: format!("LinRegSlope_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::LinRegSlope { period },
                data: tb_math::indicators::linreg_slope(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"LINREG_INTERCEPT".to_string()) {
        for period in [14u32, 20] {
            base_arrays.push(BaseArray {
                name: format!("LinRegIntercept_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::LinRegIntercept { period },
                data: tb_math::indicators::linreg_intercept(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"LINREG_R2".to_string()) {
        for period in [14u32, 20] {
            base_arrays.push(BaseArray {
                name: format!("LinRegRSquared_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::LinRegRSquared { period },
                data: tb_math::indicators::linreg_r_squared(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"LINREG_CURVE".to_string()) {
        for period in [14u32, 20] {
            base_arrays.push(BaseArray {
                name: format!("LinRegCurve_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::LinRegCurve { period },
                data: tb_math::indicators::linreg_curve(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"STD_ERROR_BANDS".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("StdErrorUp_{}_2.0", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::StdErrorBandUpper { period, multiplier: 2.0 },
                data: tb_math::indicators::std_error_bands_upper(data.close, period as usize, 2.0),
            });
            base_arrays.push(BaseArray {
                name: format!("StdErrorDn_{}_2.0", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::StdErrorBandLower { period, multiplier: 2.0 },
                data: tb_math::indicators::std_error_bands_lower(data.close, period as usize, 2.0),
            });
        }
    }

    if permitted_indicators.contains(&"Z_SCORE".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("ZScore_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::ZScore { period },
                data: tb_math::indicators::z_score(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"LOG_RETURN".to_string()) {
        base_arrays.push(BaseArray {
            name: "LogReturn".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::LogReturn,
            data: tb_math::indicators::log_return(data.close),
        });
    }

    if permitted_indicators.contains(&"MEDIAN_PRICE".to_string()) {
        base_arrays.push(BaseArray {
            name: "MedianPrice".to_string(),
            semantic_type: SemanticType::Price,
            ast: tb_core::ast::Expr::MedianPrice,
            data: tb_math::indicators::median_price(data.high, data.low),
        });
    }

    if permitted_indicators.contains(&"TYPICAL_PRICE".to_string()) {
        base_arrays.push(BaseArray {
            name: "TypicalPrice".to_string(),
            semantic_type: SemanticType::Price,
            ast: tb_core::ast::Expr::TypicalPrice,
            data: tb_math::indicators::typical_price(data.high, data.low, data.close),
        });
    }

    if permitted_indicators.contains(&"WEIGHTED_CLOSE".to_string()) {
        base_arrays.push(BaseArray {
            name: "WeightedClose".to_string(),
            semantic_type: SemanticType::Price,
            ast: tb_core::ast::Expr::WeightedClose,
            data: tb_math::indicators::weighted_close(data.high, data.low, data.close),
        });
    }

    if permitted_indicators.contains(&"HURST_EXPONENT".to_string()) {
        for period in [50u32, 100] {
            base_arrays.push(BaseArray {
                name: format!("HurstExp_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::HurstExponent { period },
                data: tb_math::indicators::hurst_exponent(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"PIVOT_POINTS".to_string()) {
        for period in [20u32] { // Rolling 20-bar pivots
            base_arrays.push(BaseArray { name: format!("PivotP_{}", period), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::PivotPointsP { period }, data: tb_math::indicators::pivot_points_p(data.high, data.low, data.close, period as usize) });
            base_arrays.push(BaseArray { name: format!("PivotR1_{}", period), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::PivotPointsR1 { period }, data: tb_math::indicators::pivot_points_r1(data.high, data.low, data.close, period as usize) });
            base_arrays.push(BaseArray { name: format!("PivotS1_{}", period), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::PivotPointsS1 { period }, data: tb_math::indicators::pivot_points_s1(data.high, data.low, data.close, period as usize) });
        }
    }

    if permitted_indicators.contains(&"FIB_RETRACEMENTS".to_string()) {
        for period in [50u32] { // Rolling 50-bar Fib levels
            base_arrays.push(BaseArray { name: format!("Fib236_{}", period), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::FibLevel236 { period }, data: tb_math::indicators::fib_level_236(data.high, data.low, period as usize) });
            base_arrays.push(BaseArray { name: format!("Fib382_{}", period), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::FibLevel382 { period }, data: tb_math::indicators::fib_level_382(data.high, data.low, period as usize) });
            base_arrays.push(BaseArray { name: format!("Fib500_{}", period), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::FibLevel500 { period }, data: tb_math::indicators::fib_level_500(data.high, data.low, period as usize) });
            base_arrays.push(BaseArray { name: format!("Fib618_{}", period), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::FibLevel618 { period }, data: tb_math::indicators::fib_level_618(data.high, data.low, period as usize) });
            base_arrays.push(BaseArray { name: format!("Fib786_{}", period), semantic_type: SemanticType::Price, ast: tb_core::ast::Expr::FibLevel786 { period }, data: tb_math::indicators::fib_level_786(data.high, data.low, period as usize) });
        }
    }

    if permitted_indicators.contains(&"HEIKIN_ASHI".to_string()) {
        base_arrays.push(BaseArray {
            name: "HAClose".to_string(),
            semantic_type: SemanticType::Price,
            ast: tb_core::ast::Expr::HeikinAshiClose,
            data: tb_math::indicators::heikin_ashi_close(data.open, data.high, data.low, data.close),
        });
    }

    if permitted_indicators.contains(&"EHLERS_SUPER_SMOOTHER".to_string()) {
        for period in [10u32, 20] {
            base_arrays.push(BaseArray {
                name: format!("SuperSmooth_{}", period),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::EhlersSuperSmoother { period },
                data: tb_math::indicators::ehlers_super_smoother(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"EHLERS_DECYCLER".to_string()) {
        for period in [20u32, 40] {
            base_arrays.push(BaseArray {
                name: format!("Decycler_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::EhlersDecycler { period },
                data: tb_math::indicators::ehlers_decycler(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"EHLERS_CYBER_CYCLE".to_string()) {
        base_arrays.push(BaseArray {
            name: "CyberCycle_07".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::EhlersCyberCycle { alpha: 0.07 },
            data: tb_math::indicators::ehlers_cyber_cycle(data.close, 0.07),
        });
    }

    if permitted_indicators.contains(&"EHLERS_MAMA_FAMA".to_string()) {
        base_arrays.push(BaseArray {
            name: "MAMA".to_string(),
            semantic_type: SemanticType::Price,
            ast: tb_core::ast::Expr::EhlersMama { fast_limit: 0.5, slow_limit: 0.05 },
            data: tb_math::indicators::ehlers_mama(data.close, 0.5, 0.05),
        });
        base_arrays.push(BaseArray {
            name: "FAMA".to_string(),
            semantic_type: SemanticType::Price,
            ast: tb_core::ast::Expr::EhlersFama { fast_limit: 0.5, slow_limit: 0.05 },
            data: tb_math::indicators::ehlers_fama(data.close, 0.5, 0.05),
        });
    }

    if permitted_indicators.contains(&"EHLERS_SINE_WAVE".to_string()) {
        base_arrays.push(BaseArray {
            name: "Sine".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::EhlersSine,
            data: tb_math::indicators::ehlers_sine(data.close),
        });
        base_arrays.push(BaseArray {
            name: "LeadSine".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::EhlersLeadSine,
            data: tb_math::indicators::ehlers_lead_sine(data.close),
        });
    }

    if permitted_indicators.contains(&"EHLERS_DECYCLER_OSCILLATOR".to_string()) {
        for period in [10u32, 20] {
            base_arrays.push(BaseArray {
                name: format!("DecyclerOsc_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::EhlersDecyclerOscillator { hp_period1: period, hp_period2: period * 2 },
                data: tb_math::indicators::ehlers_decycler_oscillator(data.close, period as usize, (period * 2) as usize),
            });
        }
    }

    if permitted_indicators.contains(&"EHLERS_ROOFING_FILTER".to_string()) {
        for period in [10u32, 20] {
            base_arrays.push(BaseArray {
                name: format!("Roofing_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::EhlersRoofingFilter { hp_period: 40, lp_period: period },
                data: tb_math::indicators::ehlers_roofing_filter(data.close, 40, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"EHLERS_DOMINANT_CYCLE_PERIOD".to_string()) {
        base_arrays.push(BaseArray {
            name: "DomCycle".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::EhlersDominantCyclePeriod,
            data: tb_math::indicators::ehlers_dominant_cycle_period(data.close),
        });
    }

    if permitted_indicators.contains(&"EHLERS_AUTOCORRELATION_PERIODOGRAM".to_string()) {
        base_arrays.push(BaseArray {
            name: "AutoCorrPeriod".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::EhlersAutocorrelationPeriodogram,
            data: tb_math::indicators::ehlers_dominant_cycle_period(data.close), // mapped to same function for now
        });
    }

    if permitted_indicators.contains(&"EHLERS_EMD".to_string()) {
        for period in [20u32] {
            base_arrays.push(BaseArray {
                name: format!("EMD_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::EhlersEmd { period, fraction: 0.1 },
                data: tb_math::indicators::ehlers_emd(data.close, period as usize, 0.1),
            });
        }
    }

    if permitted_indicators.contains(&"MARKET_MEANNESS_INDEX".to_string()) {
        for period in [20u32, 50] {
            base_arrays.push(BaseArray {
                name: format!("MMI_{}", period),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::MarketMeannessIndex { period },
                data: tb_math::indicators::market_meanness_index(data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"ZERO_LAG_MACD".to_string()) {
        let macd_params = [(12, 26, 9), (8, 21, 5)];
        for (fast, slow, signal) in macd_params {
            base_arrays.push(BaseArray { name: format!("ZLMacdLine_{}_{}_{}", fast, slow, signal), semantic_type: SemanticType::Oscillator, ast: tb_core::ast::Expr::ZeroLagMacdLine { fast, slow, signal }, data: tb_math::indicators::zl_macd_line(data.close, fast as usize, slow as usize, signal as usize) });
            base_arrays.push(BaseArray { name: format!("ZLMacdSig_{}_{}_{}", fast, slow, signal), semantic_type: SemanticType::Oscillator, ast: tb_core::ast::Expr::ZeroLagMacdSignal { fast, slow, signal }, data: tb_math::indicators::zl_macd_signal(data.close, fast as usize, slow as usize, signal as usize) });
            base_arrays.push(BaseArray { name: format!("ZLMacdHist_{}_{}_{}", fast, slow, signal), semantic_type: SemanticType::Oscillator, ast: tb_core::ast::Expr::ZeroLagMacdHist { fast, slow, signal }, data: tb_math::indicators::zl_macd_hist(data.close, fast as usize, slow as usize, signal as usize) });
        }
    }

    if permitted_indicators.contains(&"GATOR_OSCILLATOR".to_string()) {
        base_arrays.push(BaseArray {
            name: "GatorTop".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::GatorTop,
            data: tb_math::indicators::gator_top(data.close),
        });
        base_arrays.push(BaseArray {
            name: "GatorBottom".to_string(),
            semantic_type: SemanticType::Oscillator,
            ast: tb_core::ast::Expr::GatorBottom,
            data: tb_math::indicators::gator_bottom(data.close),
        });
    }

    if permitted_indicators.contains(&"KALMAN_FILTER".to_string()) {
        base_arrays.push(BaseArray {
            name: "Kalman".to_string(),
            semantic_type: SemanticType::Price,
            ast: tb_core::ast::Expr::KalmanFilter { r: 1.0, q: 0.1 },
            data: tb_math::indicators::kalman_filter(data.close, 1.0, 0.1),
        });
    }

    if permitted_indicators.contains(&"MACD".to_string()) {
        let macd_params = [(12, 26, 9), (8, 21, 5), (10, 20, 5)];
        for (fast, slow, signal) in macd_params {
            let (macd_line, sig_line, hist_line) = tb_math::indicators::macd(data.close, fast as usize, slow as usize, signal as usize);
            base_arrays.push(BaseArray {
                name: format!("MACD_Line_{}_{}", fast, slow),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::MacdLine { source: Box::new(tb_core::ast::Expr::Close), fast, slow },
                data: macd_line,
            });
            base_arrays.push(BaseArray {
                name: format!("MACD_Signal_{}_{}_{}", fast, slow, signal),
                semantic_type: SemanticType::Oscillator,
                ast: tb_core::ast::Expr::MacdSignal { source: Box::new(tb_core::ast::Expr::Close), fast, slow, signal },
                data: sig_line,
            });
            base_arrays.push(BaseArray {
                name: format!("MACD_Hist_{}_{}_{}", fast, slow, signal),
                semantic_type: SemanticType::Momentum,
                ast: tb_core::ast::Expr::MacdHistogram { source: Box::new(tb_core::ast::Expr::Close), fast, slow, signal },
                data: hist_line,
            });
        }
    }

    if permitted_indicators.contains(&"Bollinger".to_string()) {
        let bb_params = [(20, 2.0), (20, 2.5), (50, 2.0), (50, 2.5)];
        for (period, std_dev) in bb_params {
            let (upper, _mid, lower) = tb_math::indicators::bollinger_bands(data.close, period as usize, std_dev);
            base_arrays.push(BaseArray {
                name: format!("BB_Upper_{}_{:.1}", period, std_dev),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::BollingerUpper { source: Box::new(tb_core::ast::Expr::Close), period, std_dev },
                data: upper,
            });
            base_arrays.push(BaseArray {
                name: format!("BB_Lower_{}_{:.1}", period, std_dev),
                semantic_type: SemanticType::Price,
                ast: tb_core::ast::Expr::BollingerLower { source: Box::new(tb_core::ast::Expr::Close), period, std_dev },
                data: lower,
            });
        }
    }

    if permitted_indicators.contains(&"ATR".to_string()) {
        for period in [14, 20] {
            base_arrays.push(BaseArray {
                name: format!("ATR_{}", period),
                semantic_type: SemanticType::Price, // Changed to Price since it measures absolute volatility magnitude
                ast: tb_core::ast::Expr::Atr { period },
                data: tb_math::indicators::atr(data.high, data.low, data.close, period as usize),
            });
        }
    }

    if permitted_indicators.contains(&"BULLISH_ENGULFING".to_string()) {
        base_arrays.push(BaseArray {
            name: "BullishEngulfing".to_string(),
            semantic_type: SemanticType::Pattern,
            ast: tb_core::ast::Expr::BullishEngulfing,
            data: tb_math::patterns::bullish_engulfing(data.open, data.close),
        });
    }

    if permitted_indicators.contains(&"BEARISH_ENGULFING".to_string()) {
        base_arrays.push(BaseArray {
            name: "BearishEngulfing".to_string(),
            semantic_type: SemanticType::Pattern,
            ast: tb_core::ast::Expr::BearishEngulfing,
            data: tb_math::patterns::bearish_engulfing(data.open, data.close),
        });
    }

    if permitted_indicators.contains(&"DOJI".to_string()) {
        base_arrays.push(BaseArray {
            name: "Doji".to_string(),
            semantic_type: SemanticType::Pattern,
            ast: tb_core::ast::Expr::Doji,
            data: tb_math::patterns::doji(data.open, data.high, data.low, data.close),
        });
    }

    if permitted_indicators.contains(&"HAMMER".to_string()) {
        base_arrays.push(BaseArray {
            name: "Hammer".to_string(),
            semantic_type: SemanticType::Pattern,
            ast: tb_core::ast::Expr::Hammer,
            data: tb_math::patterns::hammer(data.open, data.high, data.low, data.close),
        });
    }

    if permitted_indicators.contains(&"SHOOTING_STAR".to_string()) {
        base_arrays.push(BaseArray {
            name: "ShootingStar".to_string(),
            semantic_type: SemanticType::Pattern,
            ast: tb_core::ast::Expr::ShootingStar,
            data: tb_math::patterns::shooting_star(data.open, data.high, data.low, data.close),
        });
    }

    if permitted_indicators.contains(&"MORNING_STAR".to_string()) {
        base_arrays.push(BaseArray {
            name: "MorningStar".to_string(),
            semantic_type: SemanticType::Pattern,
            ast: tb_core::ast::Expr::MorningStar,
            data: tb_math::patterns::morning_star(data.open, data.close),
        });
    }

    if permitted_indicators.contains(&"EVENING_STAR".to_string()) {
        base_arrays.push(BaseArray {
            name: "EveningStar".to_string(),
            semantic_type: SemanticType::Pattern,
            ast: tb_core::ast::Expr::EveningStar,
            data: tb_math::patterns::evening_star(data.open, data.close),
        });
    }

    base_arrays
}
