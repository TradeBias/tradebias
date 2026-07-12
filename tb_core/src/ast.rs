use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExprType {
    PriceOverlay, // Scales with asset price (Close, SMA, EMA)
    Oscillator,   // Bounded or mean-reverting (RSI, MACD)
    Constant,     // Static thresholds (30.0, 70.0)
    Boolean,      // Logical nodes
    Pattern,      // Binary pattern (1.0 or 0.0)
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
    Wma { source: Box<Expr>, period: u32 },
    Hma { source: Box<Expr>, period: u32 },
    Dema { source: Box<Expr>, period: u32 },
    Tema { source: Box<Expr>, period: u32 },
    Kama { source: Box<Expr>, period: u32, fast: u32, slow: u32 },
    Smma { source: Box<Expr>, period: u32 },
    Rsi { source: Box<Expr>, period: u32 },
    MacdLine { source: Box<Expr>, fast: u32, slow: u32 },
    MacdSignal { source: Box<Expr>, fast: u32, slow: u32, signal: u32 },
    MacdHistogram { source: Box<Expr>, fast: u32, slow: u32, signal: u32 },
    BollingerUpper { source: Box<Expr>, period: u32, std_dev: f64 },
    BollingerLower { source: Box<Expr>, period: u32, std_dev: f64 },
    Atr { period: u32 },
    KeltnerUpper { period: u32, multiplier: f64 },
    KeltnerLower { period: u32, multiplier: f64 },
    DonchianUpper { period: u32 },
    DonchianLower { period: u32 },
    DonchianMid { period: u32 },
    Supertrend { period: u32, multiplier: f64 },
    SupertrendDir { period: u32, multiplier: f64 },
    Psar { af_step: f64, af_max: f64 },
    Alma { source: Box<Expr>, period: u32, offset: f64, sigma: f64 },
    Adx { period: u32 },
    DiPlus { period: u32 },
    DiMinus { period: u32 },
    StochasticK { period: u32, k_period: u32, d_period: u32 },
    StochasticD { period: u32, k_period: u32, d_period: u32 },
    StochRsiK { rsi_period: u32, stoch_period: u32, k_period: u32, d_period: u32 },
    StochRsiD { rsi_period: u32, stoch_period: u32, k_period: u32, d_period: u32 },
    WilliamsR { period: u32 },
    Cci { period: u32 },
    Mfi { period: u32 },
    Roc { period: u32 },
    AwesomeOscillator,
    Tsi { long_period: u32, short_period: u32 },
    UltimateOscillator { p1: u32, p2: u32, p3: u32 },
    Dpo { period: u32 },
    Kst { r1: u32, r2: u32, r3: u32, r4: u32, s1: u32, s2: u32, s3: u32, s4: u32 },
    KstSignal { r1: u32, r2: u32, r3: u32, r4: u32, s1: u32, s2: u32, s3: u32, s4: u32 },
    FisherTransform { period: u32 },
    FisherTrigger { period: u32 },
    ConnorsRsi { rsi_period: u32, streak_rsi_period: u32, percent_rank_period: u32 },
    Cmo { period: u32 },
    Rvi { period: u32 },
    RviSignal { period: u32 },
    Smi { period: u32, ema1_p: u32, ema2_p: u32 },
    Trix { period: u32 },
    Eom { period: u32 },
    VortexPlus { period: u32 },
    VortexMinus { period: u32 },
    DssBressert { period: u32, ema_period: u32 },
    Ppo { fast: u32, slow: u32, signal: u32 },
    PpoSignal { fast: u32, slow: u32, signal: u32 },
    PpoHist { fast: u32, slow: u32, signal: u32 },
    ChoppinessIndex { period: u32 },
    QqeFast { period: u32, sf: u32 },
    QqeSlow { period: u32, sf: u32 },
    Stc { fast: u32, slow: u32, period: u32, ema_period: u32 },
    ChaikinVolatility { ema_period: u32, roc_period: u32 },
    HistoricalVolatility { period: u32, annual_factor: f64 },
    UlcerIndex { period: u32 },
    StandardDeviation { period: u32 },
    BollingerBandWidth { period: u32, std_dev: f64 },
    BollingerPercentB { period: u32, std_dev: f64 },
    KeltnerChannelWidth { period: u32, multiplier: f64 },
    VixSynthetic { period: u32 },
    Obv,
    Vwap,
    AdLine,
    ChaikinMoneyFlow { period: u32 },
    ChaikinOscillator { fast: u32, slow: u32 },
    Pvt,
    Nvi,
    Pvi,
    ForceIndex { period: u32 },
    Vfi { period: u32 },
    VolumeOscillator { fast: u32, slow: u32 },
    KlingerOscillator { fast: u32, slow: u32 },
    Mvwap { period: u32 },
    Twap { period: u32 },
    LinRegSlope { period: u32 },
    LinRegIntercept { period: u32 },
    LinRegRSquared { period: u32 },
    LinRegCurve { period: u32 },
    StdErrorBandUpper { period: u32, multiplier: f64 },
    StdErrorBandLower { period: u32, multiplier: f64 },
    ZScore { period: u32 },
    LogReturn,
    MedianPrice,
    TypicalPrice,
    WeightedClose,
    HurstExponent { period: u32 },
    PivotPointsP { period: u32 },
    PivotPointsR1 { period: u32 },
    PivotPointsS1 { period: u32 },
    FibLevel236 { period: u32 },
    FibLevel382 { period: u32 },
    FibLevel500 { period: u32 },
    FibLevel618 { period: u32 },
    FibLevel786 { period: u32 },
    HeikinAshiClose,
    EhlersSuperSmoother { period: u32 },
    EhlersDecycler { period: u32 },
    EhlersCyberCycle { alpha: f64 },
    EhlersMama { fast_limit: f64, slow_limit: f64 },
    EhlersFama { fast_limit: f64, slow_limit: f64 },
    EhlersSine,
    EhlersLeadSine,
    EhlersDecyclerOscillator { hp_period1: u32, hp_period2: u32 },
    EhlersRoofingFilter { hp_period: u32, lp_period: u32 },
    EhlersDominantCyclePeriod,
    EhlersAutocorrelationPeriodogram,
    EhlersEmd { period: u32, fraction: f64 },
    MarketMeannessIndex { period: u32 },
    ZeroLagMacdLine { fast: u32, slow: u32, signal: u32 },
    ZeroLagMacdSignal { fast: u32, slow: u32, signal: u32 },
    ZeroLagMacdHist { fast: u32, slow: u32, signal: u32 },
    GatorTop,
    GatorBottom,
    KalmanFilter { r: f64, q: f64 },

    // 7. Candlestick Patterns
    BullishEngulfing,
    BearishEngulfing,
    Doji,
    Hammer,
    ShootingStar,
    MorningStar,
    EveningStar,

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
    pub expectancy: f64,
    pub trade_frequency: f64,
    pub indicator_count: u8,
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
            Expr::Wma { source, period } => write!(f, "WMA({}, {})", source, period),
            Expr::Hma { source, period } => write!(f, "HMA({}, {})", source, period),
            Expr::Dema { source, period } => write!(f, "DEMA({}, {})", source, period),
            Expr::Tema { source, period } => write!(f, "TEMA({}, {})", source, period),
            Expr::Kama { source, period, fast, slow } => write!(f, "KAMA({}, {}, {}, {})", source, period, fast, slow),
            Expr::Smma { source, period } => write!(f, "SMMA({}, {})", source, period),
            Expr::Rsi { source, period } => write!(f, "RSI({}, {})", source, period),
            Expr::MacdLine { source, fast, slow } => write!(f, "MACD_Line({}, {}, {})", source, fast, slow),
            Expr::MacdSignal { source, fast, slow, signal } => write!(f, "MACD_Signal({}, {}, {}, {})", source, fast, slow, signal),
            Expr::MacdHistogram { source, fast, slow, signal } => write!(f, "MACD_Hist({}, {}, {}, {})", source, fast, slow, signal),
            Expr::BollingerUpper { source, period, std_dev } => write!(f, "BB_Upper({}, {}, {})", source, period, std_dev),
            Expr::BollingerLower { source, period, std_dev } => write!(f, "BB_Lower({}, {}, {})", source, period, std_dev),
            Expr::Atr { period } => write!(f, "ATR({})", period),
            Expr::KeltnerUpper { period, multiplier } => write!(f, "KeltnerUpper({}, {})", period, multiplier),
            Expr::KeltnerLower { period, multiplier } => write!(f, "KeltnerLower({}, {})", period, multiplier),
            Expr::DonchianUpper { period } => write!(f, "DonchianUpper({})", period),
            Expr::DonchianLower { period } => write!(f, "DonchianLower({})", period),
            Expr::DonchianMid { period } => write!(f, "DonchianMid({})", period),
            Expr::Supertrend { period, multiplier } => write!(f, "Supertrend({}, {})", period, multiplier),
            Expr::SupertrendDir { period, multiplier } => write!(f, "SupertrendDir({}, {})", period, multiplier),
            Expr::Psar { af_step, af_max } => write!(f, "PSAR({}, {})", af_step, af_max),
            Expr::Alma { source, period, offset, sigma } => write!(f, "ALMA({}, {}, {}, {})", source, period, offset, sigma),
            Expr::Adx { period } => write!(f, "ADX({})", period),
            Expr::DiPlus { period } => write!(f, "DI+({})", period),
            Expr::DiMinus { period } => write!(f, "DI-({})", period),
            Expr::StochasticK { period, k_period, d_period } => write!(f, "StochK({}, {}, {})", period, k_period, d_period),
            Expr::StochasticD { period, k_period, d_period } => write!(f, "StochD({}, {}, {})", period, k_period, d_period),
            Expr::StochRsiK { rsi_period, stoch_period, k_period, d_period } => write!(f, "StochRsiK({}, {}, {}, {})", rsi_period, stoch_period, k_period, d_period),
            Expr::StochRsiD { rsi_period, stoch_period, k_period, d_period } => write!(f, "StochRsiD({}, {}, {}, {})", rsi_period, stoch_period, k_period, d_period),
            Expr::WilliamsR { period } => write!(f, "WilliamsR({})", period),
            Expr::Cci { period } => write!(f, "CCI({})", period),
            Expr::Mfi { period } => write!(f, "MFI({})", period),
            Expr::Roc { period } => write!(f, "ROC({})", period),
            Expr::AwesomeOscillator => write!(f, "AwesomeOscillator"),
            Expr::Tsi { long_period, short_period } => write!(f, "TSI({}, {})", long_period, short_period),
            Expr::UltimateOscillator { p1, p2, p3 } => write!(f, "UltimateOscillator({}, {}, {})", p1, p2, p3),
            Expr::Dpo { period } => write!(f, "DPO({})", period),
            Expr::Kst { r1, r2, r3, r4, s1, s2, s3, s4 } => write!(f, "KST({}_{}_{}_{}_{}_{}_{}_{})", r1, r2, r3, r4, s1, s2, s3, s4),
            Expr::KstSignal { r1, r2, r3, r4, s1, s2, s3, s4 } => write!(f, "KST_Signal({}_{}_{}_{}_{}_{}_{}_{})", r1, r2, r3, r4, s1, s2, s3, s4),
            Expr::FisherTransform { period } => write!(f, "Fisher({})", period),
            Expr::FisherTrigger { period } => write!(f, "FisherTrigger({})", period),
            Expr::ConnorsRsi { rsi_period, streak_rsi_period, percent_rank_period } => write!(f, "CRSI({}, {}, {})", rsi_period, streak_rsi_period, percent_rank_period),
            Expr::Cmo { period } => write!(f, "CMO({})", period),
            Expr::Rvi { period } => write!(f, "RVI({})", period),
            Expr::RviSignal { period } => write!(f, "RVI_Signal({})", period),
            Expr::Smi { period, ema1_p, ema2_p } => write!(f, "SMI({}, {}, {})", period, ema1_p, ema2_p),
            Expr::Trix { period } => write!(f, "TRIX({})", period),
            Expr::Eom { period } => write!(f, "EOM({})", period),
            Expr::VortexPlus { period } => write!(f, "VortexPlus({})", period),
            Expr::VortexMinus { period } => write!(f, "VortexMinus({})", period),
            Expr::DssBressert { period, ema_period } => write!(f, "DSS({}, {})", period, ema_period),
            Expr::Ppo { fast, slow, signal } => write!(f, "PPO({}, {}, {})", fast, slow, signal),
            Expr::PpoSignal { fast, slow, signal } => write!(f, "PPOSig({}, {}, {})", fast, slow, signal),
            Expr::PpoHist { fast, slow, signal } => write!(f, "PPOHist({}, {}, {})", fast, slow, signal),
            Expr::ChoppinessIndex { period } => write!(f, "CHOP({})", period),
            Expr::QqeFast { period, sf } => write!(f, "QQEFast({}, {})", period, sf),
            Expr::QqeSlow { period, sf } => write!(f, "QQESlow({}, {})", period, sf),
            Expr::Stc { fast, slow, period, ema_period } => write!(f, "STC({}, {}, {}, {})", fast, slow, period, ema_period),
            Expr::ChaikinVolatility { ema_period, roc_period } => write!(f, "ChaikinVol({}, {})", ema_period, roc_period),
            Expr::HistoricalVolatility { period, annual_factor } => write!(f, "HV({}, {})", period, annual_factor),
            Expr::UlcerIndex { period } => write!(f, "Ulcer({})", period),
            Expr::StandardDeviation { period } => write!(f, "StdDev({})", period),
            Expr::BollingerBandWidth { period, std_dev } => write!(f, "BBWidth({}, {})", period, std_dev),
            Expr::BollingerPercentB { period, std_dev } => write!(f, "BB%B({}, {})", period, std_dev),
            Expr::KeltnerChannelWidth { period, multiplier } => write!(f, "KCWidth({}, {})", period, multiplier),
            Expr::VixSynthetic { period } => write!(f, "VixSynth({})", period),
            Expr::Obv => write!(f, "OBV"),
            Expr::Vwap => write!(f, "VWAP"),
            Expr::AdLine => write!(f, "AD_LINE"),
            Expr::ChaikinMoneyFlow { period } => write!(f, "CMF({})", period),
            Expr::ChaikinOscillator { fast, slow } => write!(f, "ChaikinOsc({}, {})", fast, slow),
            Expr::Pvt => write!(f, "PVT"),
            Expr::Nvi => write!(f, "NVI"),
            Expr::Pvi => write!(f, "PVI"),
            Expr::ForceIndex { period } => write!(f, "ForceIndex({})", period),
            Expr::Vfi { period } => write!(f, "VFI({})", period),
            Expr::VolumeOscillator { fast, slow } => write!(f, "VolOsc({}, {})", fast, slow),
            Expr::KlingerOscillator { fast, slow } => write!(f, "KO({}, {})", fast, slow),
            Expr::Mvwap { period } => write!(f, "MVWAP({})", period),
            Expr::Twap { period } => write!(f, "TWAP({})", period),
            Expr::LinRegSlope { period } => write!(f, "LinRegSlope({})", period),
            Expr::LinRegIntercept { period } => write!(f, "LinRegIntercept({})", period),
            Expr::LinRegRSquared { period } => write!(f, "LinRegRSquared({})", period),
            Expr::LinRegCurve { period } => write!(f, "LinRegCurve({})", period),
            Expr::StdErrorBandUpper { period, multiplier } => write!(f, "StdErrorUp({}, {})", period, multiplier),
            Expr::StdErrorBandLower { period, multiplier } => write!(f, "StdErrorDn({}, {})", period, multiplier),
            Expr::ZScore { period } => write!(f, "ZScore({})", period),
            Expr::LogReturn => write!(f, "LogReturn"),
            Expr::MedianPrice => write!(f, "MedianPrice"),
            Expr::TypicalPrice => write!(f, "TypicalPrice"),
            Expr::WeightedClose => write!(f, "WeightedClose"),
            Expr::HurstExponent { period } => write!(f, "HurstExp({})", period),
            Expr::PivotPointsP { period } => write!(f, "PivotP({})", period),
            Expr::PivotPointsR1 { period } => write!(f, "PivotR1({})", period),
            Expr::PivotPointsS1 { period } => write!(f, "PivotS1({})", period),
            Expr::FibLevel236 { period } => write!(f, "Fib236({})", period),
            Expr::FibLevel382 { period } => write!(f, "Fib382({})", period),
            Expr::FibLevel500 { period } => write!(f, "Fib500({})", period),
            Expr::FibLevel618 { period } => write!(f, "Fib618({})", period),
            Expr::FibLevel786 { period } => write!(f, "Fib786({})", period),
            Expr::HeikinAshiClose => write!(f, "HAClose"),
            Expr::EhlersSuperSmoother { period } => write!(f, "SuperSmooth({})", period),
            Expr::EhlersDecycler { period } => write!(f, "Decycler({})", period),
            Expr::EhlersCyberCycle { alpha } => write!(f, "CyberCycle({})", alpha),
            Expr::EhlersMama { fast_limit, slow_limit } => write!(f, "MAMA({},{})", fast_limit, slow_limit),
            Expr::EhlersFama { fast_limit, slow_limit } => write!(f, "FAMA({},{})", fast_limit, slow_limit),
            Expr::EhlersSine => write!(f, "Sine"),
            Expr::EhlersLeadSine => write!(f, "LeadSine"),
            Expr::EhlersDecyclerOscillator { hp_period1, hp_period2 } => write!(f, "DecyclerOsc({},{})", hp_period1, hp_period2),
            Expr::EhlersRoofingFilter { hp_period, lp_period } => write!(f, "Roofing({},{})", hp_period, lp_period),
            Expr::EhlersDominantCyclePeriod => write!(f, "DomCycle"),
            Expr::EhlersAutocorrelationPeriodogram => write!(f, "AutoCorrPeriod"),
            Expr::EhlersEmd { period, fraction } => write!(f, "EMD({},{})", period, fraction),
            Expr::MarketMeannessIndex { period } => write!(f, "MMI({})", period),
            Expr::ZeroLagMacdLine { fast, slow, signal } => write!(f, "ZLMacdLine({},{},{})", fast, slow, signal),
            Expr::ZeroLagMacdSignal { fast, slow, signal } => write!(f, "ZLMacdSig({},{},{})", fast, slow, signal),
            Expr::ZeroLagMacdHist { fast, slow, signal } => write!(f, "ZLMacdHist({},{},{})", fast, slow, signal),
            Expr::GatorTop => write!(f, "GatorTop"),
            Expr::GatorBottom => write!(f, "GatorBottom"),
            Expr::KalmanFilter { r, q } => write!(f, "Kalman({},{})", r, q),
            Expr::BullishEngulfing => write!(f, "BullishEngulfing"),
            Expr::BearishEngulfing => write!(f, "BearishEngulfing"),
            Expr::Doji => write!(f, "Doji"),
            Expr::Hammer => write!(f, "Hammer"),
            Expr::ShootingStar => write!(f, "ShootingStar"),
            Expr::MorningStar => write!(f, "MorningStar"),
            Expr::EveningStar => write!(f, "EveningStar"),
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
            Expr::Sma { .. } | Expr::Ema { .. } | Expr::Wma { .. } | Expr::Hma { .. } | Expr::Dema { .. } | Expr::Tema { .. } | Expr::Kama { .. } | Expr::Smma { .. } | Expr::Alma { .. } | Expr::BollingerUpper { .. } | Expr::BollingerLower { .. } | Expr::KeltnerUpper { .. } | Expr::KeltnerLower { .. } | Expr::DonchianUpper { .. } | Expr::DonchianLower { .. } | Expr::DonchianMid { .. } | Expr::Supertrend { .. } | Expr::Psar { .. } | Expr::Vwap | Expr::Mvwap { .. } | Expr::Twap { .. } | Expr::LinRegCurve { .. } | Expr::StdErrorBandUpper { .. } | Expr::StdErrorBandLower { .. } | Expr::MedianPrice | Expr::TypicalPrice | Expr::WeightedClose | Expr::PivotPointsP { .. } | Expr::PivotPointsR1 { .. } | Expr::PivotPointsS1 { .. } | Expr::FibLevel236 { .. } | Expr::FibLevel382 { .. } | Expr::FibLevel500 { .. } | Expr::FibLevel618 { .. } | Expr::FibLevel786 { .. } | Expr::HeikinAshiClose | Expr::EhlersSuperSmoother { .. } | Expr::EhlersMama { .. } | Expr::EhlersFama { .. } | Expr::EhlersRoofingFilter { .. } | Expr::KalmanFilter { .. } => ExprType::PriceOverlay,
            Expr::Rsi { .. } | Expr::MacdLine { .. } | Expr::MacdSignal { .. } | Expr::MacdHistogram { .. } | Expr::Atr { .. } | Expr::Adx { .. } | Expr::DiPlus { .. } | Expr::DiMinus { .. } | Expr::SupertrendDir { .. } | Expr::StochasticK { .. } | Expr::StochasticD { .. } | Expr::StochRsiK { .. } | Expr::StochRsiD { .. } | Expr::WilliamsR { .. } | Expr::Cci { .. } | Expr::Mfi { .. } | Expr::Roc { .. } | Expr::AwesomeOscillator { .. } | Expr::Tsi { .. } | Expr::UltimateOscillator { .. } | Expr::Dpo { .. } | Expr::Kst { .. } | Expr::KstSignal { .. } | Expr::FisherTransform { .. } | Expr::FisherTrigger { .. } | Expr::ConnorsRsi { .. } | Expr::Cmo { .. } | Expr::Rvi { .. } | Expr::RviSignal { .. } | Expr::Smi { .. } | Expr::Trix { .. } | Expr::Eom { .. } | Expr::VortexPlus { .. } | Expr::VortexMinus { .. } | Expr::DssBressert { .. } | Expr::Ppo { .. } | Expr::PpoSignal { .. } | Expr::PpoHist { .. } | Expr::ChoppinessIndex { .. } | Expr::QqeFast { .. } | Expr::QqeSlow { .. } | Expr::Stc { .. } | Expr::ChaikinVolatility { .. } | Expr::HistoricalVolatility { .. } | Expr::UlcerIndex { .. } | Expr::StandardDeviation { .. } | Expr::BollingerBandWidth { .. } | Expr::BollingerPercentB { .. } | Expr::KeltnerChannelWidth { .. } | Expr::VixSynthetic { .. } | Expr::Obv | Expr::AdLine | Expr::ChaikinMoneyFlow { .. } | Expr::ChaikinOscillator { .. } | Expr::Pvt | Expr::Nvi | Expr::Pvi | Expr::ForceIndex { .. } | Expr::Vfi { .. } | Expr::VolumeOscillator { .. } | Expr::KlingerOscillator { .. } | Expr::LinRegSlope { .. } | Expr::LinRegIntercept { .. } | Expr::LinRegRSquared { .. } | Expr::ZScore { .. } | Expr::LogReturn | Expr::HurstExponent { .. } | Expr::EhlersDecycler { .. } | Expr::EhlersCyberCycle { .. } | Expr::EhlersSine | Expr::EhlersLeadSine | Expr::EhlersDecyclerOscillator { .. } | Expr::EhlersDominantCyclePeriod | Expr::EhlersAutocorrelationPeriodogram | Expr::EhlersEmd { .. } | Expr::MarketMeannessIndex { .. } | Expr::ZeroLagMacdLine { .. } | Expr::ZeroLagMacdSignal { .. } | Expr::ZeroLagMacdHist { .. } | Expr::GatorTop | Expr::GatorBottom => ExprType::Oscillator,
            Expr::Constant { .. } => ExprType::Constant,
            Expr::BullishEngulfing | Expr::BearishEngulfing | Expr::Doji | Expr::Hammer | Expr::ShootingStar | Expr::MorningStar | Expr::EveningStar => ExprType::Pattern,
            Expr::Add { .. } | Expr::Sub { .. } => ExprType::PriceOverlay,
            Expr::CrossAbove { .. } | Expr::CrossBelow { .. } | Expr::GreaterThan { .. } | Expr::LessThan { .. } => ExprType::Boolean,
            Expr::And { .. } | Expr::Or { .. } => ExprType::Boolean,
        }
    }

    pub fn indicator_count(&self) -> u8 {
        match self {
            Expr::Close | Expr::Open | Expr::High | Expr::Low | Expr::Volume | Expr::Constant { .. } => 0,
            Expr::Sma { source, .. } | Expr::Ema { source, .. } | Expr::Wma { source, .. } | Expr::Hma { source, .. } | Expr::Dema { source, .. } | Expr::Tema { source, .. } | Expr::Kama { source, .. } | Expr::Smma { source, .. } | Expr::Alma { source, .. } | Expr::Rsi { source, .. } => 1 + source.indicator_count(),
            Expr::MacdLine { source, .. } | Expr::MacdSignal { source, .. } | Expr::MacdHistogram { source, .. } => 1 + source.indicator_count(),
            Expr::BollingerUpper { source, .. } | Expr::BollingerLower { source, .. } => 1 + source.indicator_count(),
            Expr::Atr { .. } | Expr::KeltnerUpper { .. } | Expr::KeltnerLower { .. } | Expr::DonchianUpper { .. } | Expr::DonchianLower { .. } | Expr::DonchianMid { .. } | Expr::Supertrend { .. } | Expr::SupertrendDir { .. } | Expr::Psar { .. } | Expr::Adx { .. } | Expr::DiPlus { .. } | Expr::DiMinus { .. } | Expr::StochasticK { .. } | Expr::StochasticD { .. } | Expr::StochRsiK { .. } | Expr::StochRsiD { .. } | Expr::WilliamsR { .. } | Expr::Cci { .. } | Expr::Mfi { .. } | Expr::Roc { .. } | Expr::AwesomeOscillator | Expr::Tsi { .. } | Expr::UltimateOscillator { .. } | Expr::Dpo { .. } | Expr::Kst { .. } | Expr::KstSignal { .. } | Expr::FisherTransform { .. } | Expr::FisherTrigger { .. } | Expr::ConnorsRsi { .. } | Expr::Cmo { .. } | Expr::Rvi { .. } | Expr::RviSignal { .. } | Expr::Smi { .. } | Expr::Trix { .. } | Expr::Eom { .. } | Expr::VortexPlus { .. } | Expr::VortexMinus { .. } | Expr::DssBressert { .. } | Expr::Ppo { .. } | Expr::PpoSignal { .. } | Expr::PpoHist { .. } | Expr::ChoppinessIndex { .. } | Expr::QqeFast { .. } | Expr::QqeSlow { .. } | Expr::Stc { .. } | Expr::ChaikinVolatility { .. } | Expr::HistoricalVolatility { .. } | Expr::UlcerIndex { .. } | Expr::StandardDeviation { .. } | Expr::BollingerBandWidth { .. } | Expr::BollingerPercentB { .. } | Expr::KeltnerChannelWidth { .. } | Expr::VixSynthetic { .. } | Expr::Obv | Expr::Vwap | Expr::AdLine | Expr::ChaikinMoneyFlow { .. } | Expr::ChaikinOscillator { .. } | Expr::Pvt | Expr::Nvi | Expr::Pvi | Expr::ForceIndex { .. } | Expr::Vfi { .. } | Expr::VolumeOscillator { .. } | Expr::KlingerOscillator { .. } | Expr::Mvwap { .. } | Expr::Twap { .. } | Expr::LinRegSlope { .. } | Expr::LinRegIntercept { .. } | Expr::LinRegRSquared { .. } | Expr::LinRegCurve { .. } | Expr::StdErrorBandUpper { .. } | Expr::StdErrorBandLower { .. } | Expr::ZScore { .. } | Expr::LogReturn | Expr::MedianPrice | Expr::TypicalPrice | Expr::WeightedClose | Expr::HurstExponent { .. } | Expr::PivotPointsP { .. } | Expr::PivotPointsR1 { .. } | Expr::PivotPointsS1 { .. } | Expr::FibLevel236 { .. } | Expr::FibLevel382 { .. } | Expr::FibLevel500 { .. } | Expr::FibLevel618 { .. } | Expr::FibLevel786 { .. } | Expr::HeikinAshiClose | Expr::EhlersSuperSmoother { .. } | Expr::EhlersDecycler { .. } | Expr::EhlersCyberCycle { .. } | Expr::EhlersMama { .. } | Expr::EhlersFama { .. } | Expr::EhlersSine | Expr::EhlersLeadSine | Expr::EhlersDecyclerOscillator { .. } | Expr::EhlersRoofingFilter { .. } | Expr::EhlersDominantCyclePeriod | Expr::EhlersAutocorrelationPeriodogram | Expr::EhlersEmd { .. } | Expr::MarketMeannessIndex { .. } | Expr::ZeroLagMacdLine { .. } | Expr::ZeroLagMacdSignal { .. } | Expr::ZeroLagMacdHist { .. } | Expr::GatorTop | Expr::GatorBottom | Expr::KalmanFilter { .. } => 1,
            Expr::BullishEngulfing | Expr::BearishEngulfing | Expr::Doji | Expr::Hammer | Expr::ShootingStar | Expr::MorningStar | Expr::EveningStar => 1,
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
            Expr::Wma { source, .. } => format!("WMA({})", source.structural_signature()),
            Expr::Hma { source, .. } => format!("HMA({})", source.structural_signature()),
            Expr::Dema { source, .. } => format!("DEMA({})", source.structural_signature()),
            Expr::Tema { source, .. } => format!("TEMA({})", source.structural_signature()),
            Expr::Kama { source, .. } => format!("KAMA({})", source.structural_signature()),
            Expr::Smma { source, .. } => format!("SMMA({})", source.structural_signature()),
            Expr::Rsi { source, .. } => format!("RSI({})", source.structural_signature()),
            Expr::MacdLine { source, .. } => format!("MACDL({})", source.structural_signature()),
            Expr::MacdSignal { source, .. } => format!("MACDS({})", source.structural_signature()),
            Expr::MacdHistogram { source, .. } => format!("MACDH({})", source.structural_signature()),
            Expr::BollingerUpper { source, .. } => format!("BBU({})", source.structural_signature()),
            Expr::BollingerLower { source, .. } => format!("BBL({})", source.structural_signature()),
            Expr::Atr { .. } => "ATR".to_string(),
            Expr::KeltnerUpper { .. } => "KeltnerUpper".to_string(),
            Expr::KeltnerLower { .. } => "KeltnerLower".to_string(),
            Expr::DonchianUpper { .. } => "DonchianUpper".to_string(),
            Expr::DonchianLower { .. } => "DonchianLower".to_string(),
            Expr::DonchianMid { .. } => "DonchianMid".to_string(),
            Expr::Supertrend { .. } => "Supertrend".to_string(),
            Expr::SupertrendDir { .. } => "SupertrendDir".to_string(),
            Expr::Psar { .. } => "PSAR".to_string(),
            Expr::Alma { source, .. } => format!("ALMA({})", source.structural_signature()),
            Expr::Adx { .. } => "ADX".to_string(),
            Expr::DiPlus { .. } => "DI+".to_string(),
            Expr::DiMinus { .. } => "DI-".to_string(),
            Expr::StochasticK { .. } => "StochK".to_string(),
            Expr::StochasticD { .. } => "StochD".to_string(),
            Expr::StochRsiK { .. } => "StochRsiK".to_string(),
            Expr::StochRsiD { .. } => "StochRsiD".to_string(),
            Expr::WilliamsR { .. } => "WilliamsR".to_string(),
            Expr::Cci { .. } => "CCI".to_string(),
            Expr::Mfi { .. } => "MFI".to_string(),
            Expr::Roc { .. } => "ROC".to_string(),
            Expr::AwesomeOscillator => "AwesomeOscillator".to_string(),
            Expr::Tsi { .. } => "TSI".to_string(),
            Expr::UltimateOscillator { .. } => "UltimateOscillator".to_string(),
            Expr::Dpo { .. } => "DPO".to_string(),
            Expr::Kst { .. } => "KST".to_string(),
            Expr::KstSignal { .. } => "KST_Signal".to_string(),
            Expr::FisherTransform { .. } => "Fisher".to_string(),
            Expr::FisherTrigger { .. } => "FisherTrigger".to_string(),
            Expr::ConnorsRsi { .. } => "ConnorsRSI".to_string(),
            Expr::Cmo { .. } => "CMO".to_string(),
            Expr::Rvi { .. } => "RVI".to_string(),
            Expr::RviSignal { .. } => "RVI_Signal".to_string(),
            Expr::Smi { .. } => "SMI".to_string(),
            Expr::Trix { .. } => "TRIX".to_string(),
            Expr::Eom { .. } => "EOM".to_string(),
            Expr::VortexPlus { .. } => "VortexPlus".to_string(),
            Expr::VortexMinus { .. } => "VortexMinus".to_string(),
            Expr::DssBressert { .. } => "DSS".to_string(),
            Expr::Ppo { .. } => "PPO".to_string(),
            Expr::PpoSignal { .. } => "PPOSig".to_string(),
            Expr::PpoHist { .. } => "PPOHist".to_string(),
            Expr::ChoppinessIndex { .. } => "CHOP".to_string(),
            Expr::QqeFast { .. } => "QQEFast".to_string(),
            Expr::QqeSlow { .. } => "QQESlow".to_string(),
            Expr::Stc { .. } => "STC".to_string(),
            Expr::ChaikinVolatility { .. } => "ChaikinVol".to_string(),
            Expr::HistoricalVolatility { .. } => "HV".to_string(),
            Expr::UlcerIndex { .. } => "Ulcer".to_string(),
            Expr::StandardDeviation { .. } => "StdDev".to_string(),
            Expr::BollingerBandWidth { .. } => "BBWidth".to_string(),
            Expr::BollingerPercentB { .. } => "BB%B".to_string(),
            Expr::KeltnerChannelWidth { .. } => "KCWidth".to_string(),
            Expr::VixSynthetic { .. } => "VixSynth".to_string(),
            Expr::Obv => "OBV".to_string(),
            Expr::Vwap => "VWAP".to_string(),
            Expr::AdLine => "AD_LINE".to_string(),
            Expr::ChaikinMoneyFlow { .. } => "CMF".to_string(),
            Expr::ChaikinOscillator { .. } => "ChaikinOsc".to_string(),
            Expr::Pvt => "PVT".to_string(),
            Expr::Nvi => "NVI".to_string(),
            Expr::Pvi => "PVI".to_string(),
            Expr::ForceIndex { .. } => "ForceIndex".to_string(),
            Expr::Vfi { .. } => "VFI".to_string(),
            Expr::VolumeOscillator { .. } => "VolOsc".to_string(),
            Expr::KlingerOscillator { .. } => "KO".to_string(),
            Expr::Mvwap { .. } => "MVWAP".to_string(),
            Expr::Twap { .. } => "TWAP".to_string(),
            Expr::LinRegSlope { .. } => "LinRegSlope".to_string(),
            Expr::LinRegIntercept { .. } => "LinRegIntercept".to_string(),
            Expr::LinRegRSquared { .. } => "LinRegRSquared".to_string(),
            Expr::LinRegCurve { .. } => "LinRegCurve".to_string(),
            Expr::StdErrorBandUpper { .. } => "StdErrorUp".to_string(),
            Expr::StdErrorBandLower { .. } => "StdErrorDn".to_string(),
            Expr::ZScore { .. } => "ZScore".to_string(),
            Expr::LogReturn => "LogReturn".to_string(),
            Expr::MedianPrice => "MedianPrice".to_string(),
            Expr::TypicalPrice => "TypicalPrice".to_string(),
            Expr::WeightedClose => "WeightedClose".to_string(),
            Expr::HurstExponent { .. } => "HurstExp".to_string(),
            Expr::PivotPointsP { .. } => "PivotP".to_string(),
            Expr::PivotPointsR1 { .. } => "PivotR1".to_string(),
            Expr::PivotPointsS1 { .. } => "PivotS1".to_string(),
            Expr::FibLevel236 { .. } => "Fib236".to_string(),
            Expr::FibLevel382 { .. } => "Fib382".to_string(),
            Expr::FibLevel500 { .. } => "Fib500".to_string(),
            Expr::FibLevel618 { .. } => "Fib618".to_string(),
            Expr::FibLevel786 { .. } => "Fib786".to_string(),
            Expr::HeikinAshiClose => "HAClose".to_string(),
            Expr::EhlersSuperSmoother { .. } => "SuperSmooth".to_string(),
            Expr::EhlersDecycler { .. } => "Decycler".to_string(),
            Expr::EhlersCyberCycle { .. } => "CyberCycle".to_string(),
            Expr::EhlersMama { .. } => "MAMA".to_string(),
            Expr::EhlersFama { .. } => "FAMA".to_string(),
            Expr::EhlersSine => "Sine".to_string(),
            Expr::EhlersLeadSine => "LeadSine".to_string(),
            Expr::EhlersDecyclerOscillator { .. } => "DecyclerOsc".to_string(),
            Expr::EhlersRoofingFilter { .. } => "Roofing".to_string(),
            Expr::EhlersDominantCyclePeriod => "DomCycle".to_string(),
            Expr::EhlersAutocorrelationPeriodogram => "AutoCorrPeriod".to_string(),
            Expr::EhlersEmd { .. } => "EMD".to_string(),
            Expr::MarketMeannessIndex { .. } => "MMI".to_string(),
            Expr::ZeroLagMacdLine { .. } => "ZLMacdLine".to_string(),
            Expr::ZeroLagMacdSignal { .. } => "ZLMacdSig".to_string(),
            Expr::ZeroLagMacdHist { .. } => "ZLMacdHist".to_string(),
            Expr::GatorTop => "GatorTop".to_string(),
            Expr::GatorBottom => "GatorBottom".to_string(),
            Expr::KalmanFilter { .. } => "Kalman".to_string(),
            Expr::BullishEngulfing => "BullEngulf".to_string(),
            Expr::BearishEngulfing => "BearEngulf".to_string(),
            Expr::Doji => "Doji".to_string(),
            Expr::Hammer => "Hammer".to_string(),
            Expr::ShootingStar => "ShootingStar".to_string(),
            Expr::MorningStar => "MorningStar".to_string(),
            Expr::EveningStar => "EveningStar".to_string(),
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
            Expr::Sma { .. } | Expr::Ema { .. } | Expr::Wma { .. } | Expr::Hma { .. } | Expr::Dema { .. } | Expr::Tema { .. } | Expr::Kama { .. } | Expr::Smma { .. } | Expr::Alma { .. } | Expr::Rsi { .. } | Expr::MacdLine { .. } | Expr::MacdSignal { .. } | Expr::MacdHistogram { .. } | Expr::BollingerUpper { .. } | Expr::BollingerLower { .. } | Expr::Atr { .. } | Expr::KeltnerUpper { .. } | Expr::KeltnerLower { .. } | Expr::DonchianUpper { .. } | Expr::DonchianLower { .. } | Expr::DonchianMid { .. } | Expr::Supertrend { .. } | Expr::SupertrendDir { .. } | Expr::Psar { .. } | Expr::Adx { .. } | Expr::DiPlus { .. } | Expr::DiMinus { .. } | Expr::StochasticK { .. } | Expr::StochasticD { .. } | Expr::StochRsiK { .. } | Expr::StochRsiD { .. } | Expr::WilliamsR { .. } | Expr::Cci { .. } | Expr::Mfi { .. } | Expr::Roc { .. } | Expr::AwesomeOscillator { .. } | Expr::Tsi { .. } | Expr::UltimateOscillator { .. } | Expr::Dpo { .. } | Expr::Kst { .. } | Expr::KstSignal { .. } | Expr::FisherTransform { .. } | Expr::FisherTrigger { .. } | Expr::ConnorsRsi { .. } | Expr::Cmo { .. } | Expr::Rvi { .. } | Expr::RviSignal { .. } | Expr::Smi { .. } | Expr::Trix { .. } | Expr::Eom { .. } | Expr::VortexPlus { .. } | Expr::VortexMinus { .. } | Expr::DssBressert { .. } | Expr::Ppo { .. } | Expr::PpoSignal { .. } | Expr::PpoHist { .. } | Expr::ChoppinessIndex { .. } | Expr::QqeFast { .. } | Expr::QqeSlow { .. } | Expr::Stc { .. } | Expr::ChaikinVolatility { .. } | Expr::HistoricalVolatility { .. } | Expr::UlcerIndex { .. } | Expr::StandardDeviation { .. } | Expr::BollingerBandWidth { .. } | Expr::BollingerPercentB { .. } | Expr::KeltnerChannelWidth { .. } | Expr::VixSynthetic { .. } | Expr::Obv | Expr::Vwap | Expr::AdLine | Expr::ChaikinMoneyFlow { .. } | Expr::ChaikinOscillator { .. } | Expr::Pvt | Expr::Nvi | Expr::Pvi | Expr::ForceIndex { .. } | Expr::Vfi { .. } | Expr::VolumeOscillator { .. } | Expr::KlingerOscillator { .. } | Expr::Mvwap { .. } | Expr::Twap { .. } | Expr::LinRegSlope { .. } | Expr::LinRegIntercept { .. } | Expr::LinRegRSquared { .. } | Expr::LinRegCurve { .. } | Expr::StdErrorBandUpper { .. } | Expr::StdErrorBandLower { .. } | Expr::ZScore { .. } | Expr::LogReturn | Expr::MedianPrice | Expr::TypicalPrice | Expr::WeightedClose | Expr::HurstExponent { .. } | Expr::PivotPointsP { .. } | Expr::PivotPointsR1 { .. } | Expr::PivotPointsS1 { .. } | Expr::FibLevel236 { .. } | Expr::FibLevel382 { .. } | Expr::FibLevel500 { .. } | Expr::FibLevel618 { .. } | Expr::FibLevel786 { .. } | Expr::HeikinAshiClose | Expr::EhlersSuperSmoother { .. } | Expr::EhlersDecycler { .. } | Expr::EhlersCyberCycle { .. } | Expr::EhlersMama { .. } | Expr::EhlersFama { .. } | Expr::EhlersSine | Expr::EhlersLeadSine | Expr::EhlersDecyclerOscillator { .. } | Expr::EhlersRoofingFilter { .. } | Expr::EhlersDominantCyclePeriod | Expr::EhlersAutocorrelationPeriodogram | Expr::EhlersEmd { .. } | Expr::MarketMeannessIndex { .. } | Expr::ZeroLagMacdLine { .. } | Expr::ZeroLagMacdSignal { .. } | Expr::ZeroLagMacdHist { .. } | Expr::GatorTop | Expr::GatorBottom | Expr::KalmanFilter { .. } | Expr::BullishEngulfing | Expr::BearishEngulfing | Expr::Doji | Expr::Hammer | Expr::ShootingStar | Expr::MorningStar | Expr::EveningStar => {
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
            Expr::Sma { period, source } | Expr::Ema { period, source } | Expr::Wma { period, source } | Expr::Hma { period, source } | Expr::Dema { period, source } | Expr::Tema { period, source } | Expr::Smma { period, source } | Expr::Rsi { period, source } => {
                *period = snap(*period);
                source.snap_to_grid(grid);
            }
            Expr::Alma { period, source, .. } => {
                *period = snap(*period);
                source.snap_to_grid(grid);
            }
            Expr::Kama { period, fast, slow, source } => {
                *period = snap(*period);
                *fast = snap(*fast);
                *slow = snap(*slow);
                source.snap_to_grid(grid);
            }
            Expr::Atr { period } | Expr::KeltnerUpper { period, .. } | Expr::KeltnerLower { period, .. } | Expr::DonchianUpper { period } | Expr::DonchianLower { period } | Expr::DonchianMid { period } | Expr::Supertrend { period, .. } | Expr::SupertrendDir { period, .. } | Expr::Adx { period } | Expr::DiPlus { period } | Expr::DiMinus { period } | Expr::WilliamsR { period } | Expr::Cci { period } | Expr::Mfi { period } | Expr::Roc { period } | Expr::Dpo { period } | Expr::FisherTransform { period } | Expr::FisherTrigger { period } | Expr::Cmo { period } | Expr::Rvi { period } | Expr::RviSignal { period } | Expr::Trix { period } | Expr::Eom { period } | Expr::VortexPlus { period } | Expr::VortexMinus { period } | Expr::ChoppinessIndex { period } => {
                *period = snap(*period);
            }
            Expr::AwesomeOscillator => {}
            Expr::DssBressert { period, ema_period } => {
                *period = snap(*period);
                *ema_period = snap(*ema_period);
            }
            Expr::Ppo { fast, slow, signal } | Expr::PpoSignal { fast, slow, signal } | Expr::PpoHist { fast, slow, signal } => {
                *fast = snap(*fast);
                *slow = snap(*slow);
                *signal = snap(*signal);
            }
            Expr::QqeFast { period, sf } | Expr::QqeSlow { period, sf } => {
                *period = snap(*period);
                *sf = snap(*sf);
            }
            Expr::Stc { fast, slow, period, ema_period } => {
                *fast = snap(*fast);
                *slow = snap(*slow);
                *period = snap(*period);
                *ema_period = snap(*ema_period);
            }
            Expr::ChaikinVolatility { ema_period, roc_period } => {
                *ema_period = snap(*ema_period);
                *roc_period = snap(*roc_period);
            }
            Expr::ChaikinOscillator { fast, slow } | Expr::VolumeOscillator { fast, slow } | Expr::KlingerOscillator { fast, slow } => {
                *fast = snap(*fast);
                *slow = snap(*slow);
            }
            Expr::HistoricalVolatility { period, .. } | Expr::UlcerIndex { period } | Expr::StandardDeviation { period } | Expr::BollingerBandWidth { period, .. } | Expr::BollingerPercentB { period, .. } | Expr::KeltnerChannelWidth { period, .. } | Expr::VixSynthetic { period } | Expr::ChaikinMoneyFlow { period } | Expr::ForceIndex { period } | Expr::Vfi { period } | Expr::Mvwap { period } | Expr::Twap { period } | Expr::LinRegSlope { period } | Expr::LinRegIntercept { period } | Expr::LinRegRSquared { period } | Expr::LinRegCurve { period } | Expr::StdErrorBandUpper { period, .. } | Expr::StdErrorBandLower { period, .. } | Expr::ZScore { period } | Expr::HurstExponent { period } | Expr::PivotPointsP { period } | Expr::PivotPointsR1 { period } | Expr::PivotPointsS1 { period } | Expr::FibLevel236 { period } | Expr::FibLevel382 { period } | Expr::FibLevel500 { period } | Expr::FibLevel618 { period } | Expr::FibLevel786 { period } | Expr::EhlersSuperSmoother { period } | Expr::EhlersDecycler { period } | Expr::EhlersEmd { period, .. } | Expr::MarketMeannessIndex { period } => {
                *period = snap(*period);
            }
            Expr::EhlersDecyclerOscillator { hp_period1, hp_period2 } => {
                *hp_period1 = snap(*hp_period1);
                *hp_period2 = snap(*hp_period2);
            }
            Expr::EhlersRoofingFilter { hp_period, lp_period } => {
                *hp_period = snap(*hp_period);
                *lp_period = snap(*lp_period);
            }
            Expr::ZeroLagMacdLine { fast, slow, signal } | Expr::ZeroLagMacdSignal { fast, slow, signal } | Expr::ZeroLagMacdHist { fast, slow, signal } => {
                *fast = snap(*fast);
                *slow = snap(*slow);
                *signal = snap(*signal);
            }
            Expr::Obv | Expr::Vwap | Expr::AdLine | Expr::Pvt | Expr::Nvi | Expr::Pvi | Expr::LogReturn | Expr::MedianPrice | Expr::TypicalPrice | Expr::WeightedClose | Expr::HeikinAshiClose | Expr::EhlersCyberCycle { .. } | Expr::EhlersMama { .. } | Expr::EhlersFama { .. } | Expr::EhlersSine | Expr::EhlersLeadSine | Expr::EhlersDominantCyclePeriod | Expr::EhlersAutocorrelationPeriodogram | Expr::GatorTop | Expr::GatorBottom | Expr::KalmanFilter { .. } | Expr::BullishEngulfing | Expr::BearishEngulfing | Expr::Doji | Expr::Hammer | Expr::ShootingStar | Expr::MorningStar | Expr::EveningStar => {}
            Expr::ConnorsRsi { rsi_period, streak_rsi_period, percent_rank_period } => {
                *rsi_period = snap(*rsi_period);
                *streak_rsi_period = snap(*streak_rsi_period);
                *percent_rank_period = snap(*percent_rank_period);
            }
            Expr::Smi { period, ema1_p, ema2_p } => {
                *period = snap(*period);
                *ema1_p = snap(*ema1_p);
                *ema2_p = snap(*ema2_p);
            }
            Expr::Tsi { long_period, short_period } => {
                *long_period = snap(*long_period);
                *short_period = snap(*short_period);
            }
            Expr::UltimateOscillator { p1, p2, p3 } => {
                *p1 = snap(*p1);
                *p2 = snap(*p2);
                *p3 = snap(*p3);
            }
            Expr::Kst { r1, r2, r3, r4, s1, s2, s3, s4 } | Expr::KstSignal { r1, r2, r3, r4, s1, s2, s3, s4 } => {
                *r1 = snap(*r1); *r2 = snap(*r2); *r3 = snap(*r3); *r4 = snap(*r4);
                *s1 = snap(*s1); *s2 = snap(*s2); *s3 = snap(*s3); *s4 = snap(*s4);
            }
            Expr::StochasticK { period, k_period, d_period } | Expr::StochasticD { period, k_period, d_period } => {
                *period = snap(*period);
                *k_period = snap(*k_period);
                *d_period = snap(*d_period);
            }
            Expr::StochRsiK { rsi_period, stoch_period, k_period, d_period } | Expr::StochRsiD { rsi_period, stoch_period, k_period, d_period } => {
                *rsi_period = snap(*rsi_period);
                *stoch_period = snap(*stoch_period);
                *k_period = snap(*k_period);
                *d_period = snap(*d_period);
            }
            Expr::MacdLine { fast, slow, source } => {
                *fast = snap(*fast);
                *slow = snap(*slow);
                if *fast >= *slow {
                    if *fast == *slow {
                        if grid.len() > 1 && *fast == grid[0] { *slow = grid[1]; } else { *fast = grid[0]; }
                    } else { std::mem::swap(fast, slow); }
                }
                source.snap_to_grid(grid);
            }
            Expr::MacdSignal { fast, slow, signal, source } | Expr::MacdHistogram { fast, slow, signal, source } => {
                *fast = snap(*fast);
                *slow = snap(*slow);
                *signal = snap(*signal);
                if *fast >= *slow {
                    if *fast == *slow {
                        if grid.len() > 1 && *fast == grid[0] { *slow = grid[1]; } else { *fast = grid[0]; }
                    } else { std::mem::swap(fast, slow); }
                }
                source.snap_to_grid(grid);
            }
            Expr::BollingerUpper { period, source, .. } | Expr::BollingerLower { period, source, .. } => {
                *period = snap(*period);
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
            "WMA" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Wma { source: Box::new(src.clone()), period: p }); }
                }
            }
            "HMA" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Hma { source: Box::new(src.clone()), period: p }); }
                }
            }
            "DEMA" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Dema { source: Box::new(src.clone()), period: p }); }
                }
            }
            "TEMA" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Tema { source: Box::new(src.clone()), period: p }); }
                }
            }
            "KAMA" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Kama { source: Box::new(src.clone()), period: p, fast: 2, slow: 30 }); }
                }
            }
            "SMMA" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Smma { source: Box::new(src.clone()), period: p }); }
                }
            }
            "ALMA" => {
                for src in sources {
                    for &p in grid { exprs.push(Expr::Alma { source: Box::new(src.clone()), period: p, offset: 0.85, sigma: 6.0 }); }
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
                            exprs.push(Expr::MacdLine { source: Box::new(src.clone()), fast, slow });
                            for &signal in grid {
                                exprs.push(Expr::MacdSignal { source: Box::new(src.clone()), fast, slow, signal });
                                exprs.push(Expr::MacdHistogram { source: Box::new(src.clone()), fast, slow, signal });
                            }
                        }
                    }
                }
            }
            "BOLLINGER" => {
                for src in sources {
                    for &p in grid { 
                        exprs.push(Expr::BollingerUpper { source: Box::new(src.clone()), period: p, std_dev: 2.0 });
                        exprs.push(Expr::BollingerLower { source: Box::new(src.clone()), period: p, std_dev: 2.0 });
                    }
                }
            }
            "KELTNER" => {
                for &p in grid {
                    exprs.push(Expr::KeltnerUpper { period: p, multiplier: 2.0 });
                    exprs.push(Expr::KeltnerLower { period: p, multiplier: 2.0 });
                }
            }
            "DONCHIAN" => {
                for &p in grid {
                    exprs.push(Expr::DonchianUpper { period: p });
                    exprs.push(Expr::DonchianLower { period: p });
                    exprs.push(Expr::DonchianMid { period: p });
                }
            }
            "SUPERTREND" => {
                for &p in grid {
                    exprs.push(Expr::Supertrend { period: p, multiplier: 3.0 });
                    exprs.push(Expr::SupertrendDir { period: p, multiplier: 3.0 });
                }
            }
            "PSAR" => {
                exprs.push(Expr::Psar { af_step: 0.02, af_max: 0.2 });
            }
            "ADX" => {
                for &p in grid {
                    exprs.push(Expr::Adx { period: p });
                    exprs.push(Expr::DiPlus { period: p });
                    exprs.push(Expr::DiMinus { period: p });
                }
            }
            "STOCHASTIC" => {
                for &p in grid {
                    exprs.push(Expr::StochasticK { period: p, k_period: 3, d_period: 3 });
                    exprs.push(Expr::StochasticD { period: p, k_period: 3, d_period: 3 });
                }
            }
            "STOCHRSI" => {
                for &p in grid {
                    exprs.push(Expr::StochRsiK { rsi_period: p, stoch_period: p, k_period: 3, d_period: 3 });
                    exprs.push(Expr::StochRsiD { rsi_period: p, stoch_period: p, k_period: 3, d_period: 3 });
                }
            }
            "WILLIAMS_R" => {
                for &p in grid { exprs.push(Expr::WilliamsR { period: p }); }
            }
            "CCI" => {
                for &p in grid { exprs.push(Expr::Cci { period: p }); }
            }
            "MFI" => {
                for &p in grid { exprs.push(Expr::Mfi { period: p }); }
            }
            "ROC" => {
                for &p in grid { exprs.push(Expr::Roc { period: p }); }
            }
            "AWESOME_OSCILLATOR" => {
                exprs.push(Expr::AwesomeOscillator);
            }
            "TSI" => {
                for &p in grid { exprs.push(Expr::Tsi { long_period: p, short_period: p.max(3) }); }
            }
            "ULTIMATE_OSCILLATOR" => {
                exprs.push(Expr::UltimateOscillator { p1: 7, p2: 14, p3: 28 });
            }
            "DPO" => {
                for &p in grid { exprs.push(Expr::Dpo { period: p }); }
            }
            "KST" => {
                exprs.push(Expr::Kst { r1: 10, r2: 15, r3: 20, r4: 30, s1: 10, s2: 10, s3: 10, s4: 15 });
                exprs.push(Expr::KstSignal { r1: 10, r2: 15, r3: 20, r4: 30, s1: 10, s2: 10, s3: 10, s4: 15 });
            }
            "FISHER" => {
                for &p in grid {
                    exprs.push(Expr::FisherTransform { period: p });
                    exprs.push(Expr::FisherTrigger { period: p });
                }
            }
            "CRSI" => {
                for &p in grid { exprs.push(Expr::ConnorsRsi { rsi_period: 3, streak_rsi_period: 2, percent_rank_period: 100 }); }
            }
            "CMO" => {
                for &p in grid { exprs.push(Expr::Cmo { period: p }); }
            }
            "RVI" => {
                for &p in grid {
                    exprs.push(Expr::Rvi { period: p });
                    exprs.push(Expr::RviSignal { period: p });
                }
            }
            "SMI" => {
                for &p in grid { exprs.push(Expr::Smi { period: p, ema1_p: 3, ema2_p: 3 }); }
            }
            "TRIX" => {
                for &p in grid { exprs.push(Expr::Trix { period: p }); }
            }
            "EOM" => {
                for &p in grid { exprs.push(Expr::Eom { period: p }); }
            }
            "VORTEX" => {
                for &p in grid {
                    exprs.push(Expr::VortexPlus { period: p });
                    exprs.push(Expr::VortexMinus { period: p });
                }
            }
            "DSS" => {
                for &p in grid { exprs.push(Expr::DssBressert { period: p, ema_period: 3 }); }
            }
            "PPO" => {
                exprs.push(Expr::Ppo { fast: 12, slow: 26, signal: 9 });
                exprs.push(Expr::PpoSignal { fast: 12, slow: 26, signal: 9 });
                exprs.push(Expr::PpoHist { fast: 12, slow: 26, signal: 9 });
            }
            "CHOP" => {
                for &p in grid { exprs.push(Expr::ChoppinessIndex { period: p }); }
            }
            "QQE" => {
                for &p in grid {
                    exprs.push(Expr::QqeFast { period: p, sf: 5 });
                    exprs.push(Expr::QqeSlow { period: p, sf: 5 });
                }
            }
            "STC" => {
                for &p in grid { exprs.push(Expr::Stc { fast: 23, slow: 50, period: p, ema_period: 3 }); }
            }
            "CHAIKIN_VOL" => {
                for &p in grid { exprs.push(Expr::ChaikinVolatility { ema_period: p, roc_period: p }); }
            }
            "HV" => {
                for &p in grid { exprs.push(Expr::HistoricalVolatility { period: p, annual_factor: 365.0 }); }
            }
            "ULCER" => {
                for &p in grid { exprs.push(Expr::UlcerIndex { period: p }); }
            }
            "STDDEV" => {
                for &p in grid { exprs.push(Expr::StandardDeviation { period: p }); }
            }
            "BB_WIDTH" => {
                for &p in grid { exprs.push(Expr::BollingerBandWidth { period: p, std_dev: 2.0 }); }
            }
            "BB_PERCENT_B" => {
                for &p in grid { exprs.push(Expr::BollingerPercentB { period: p, std_dev: 2.0 }); }
            }
            "KC_WIDTH" => {
                for &p in grid { exprs.push(Expr::KeltnerChannelWidth { period: p, multiplier: 2.0 }); }
            }
            "VIX_SYNTH" => {
                for &p in grid { exprs.push(Expr::VixSynthetic { period: p }); }
            }
            "OBV" => {
                exprs.push(Expr::Obv);
            }
            "VWAP" => {
                exprs.push(Expr::Vwap);
            }
            "AD_LINE" => {
                exprs.push(Expr::AdLine);
            }
            "CMF" => {
                for &p in grid { exprs.push(Expr::ChaikinMoneyFlow { period: p }); }
            }
            "CHAIKIN_OSC" => {
                exprs.push(Expr::ChaikinOscillator { fast: 3, slow: 10 });
            }
            "PVT" => {
                exprs.push(Expr::Pvt);
            }
            "NVI" => {
                exprs.push(Expr::Nvi);
            }
            "PVI" => {
                exprs.push(Expr::Pvi);
            }
            "FORCE_INDEX" => {
                for &p in grid { exprs.push(Expr::ForceIndex { period: p }); }
            }
            "VFI" => {
                for &p in grid { exprs.push(Expr::Vfi { period: p }); }
            }
            "VOL_OSC" => {
                exprs.push(Expr::VolumeOscillator { fast: 12, slow: 26 });
            }
            "KLINGER" => {
                exprs.push(Expr::KlingerOscillator { fast: 34, slow: 55 });
            }
            "MVWAP" => {
                for &p in grid { exprs.push(Expr::Mvwap { period: p }); }
            }
            "TWAP" => {
                for &p in grid { exprs.push(Expr::Twap { period: p }); }
            }
            "LINREG_SLOPE" => { for &p in grid { exprs.push(Expr::LinRegSlope { period: p }); } }
            "LINREG_INTERCEPT" => { for &p in grid { exprs.push(Expr::LinRegIntercept { period: p }); } }
            "LINREG_R2" => { for &p in grid { exprs.push(Expr::LinRegRSquared { period: p }); } }
            "LINREG_CURVE" => { for &p in grid { exprs.push(Expr::LinRegCurve { period: p }); } }
            "STD_ERROR_BANDS" => {
                for &p in grid {
                    exprs.push(Expr::StdErrorBandUpper { period: p, multiplier: 2.0 });
                    exprs.push(Expr::StdErrorBandLower { period: p, multiplier: 2.0 });
                }
            }
            "Z_SCORE" => { for &p in grid { exprs.push(Expr::ZScore { period: p }); } }
            "LOG_RETURN" => { exprs.push(Expr::LogReturn); }
            "MEDIAN_PRICE" => { exprs.push(Expr::MedianPrice); }
            "TYPICAL_PRICE" => { exprs.push(Expr::TypicalPrice); }
            "WEIGHTED_CLOSE" => { exprs.push(Expr::WeightedClose); }
            "HURST_EXPONENT" => { for &p in grid { exprs.push(Expr::HurstExponent { period: p }); } }
            "PIVOT_POINTS" => {
                for &p in grid {
                    exprs.push(Expr::PivotPointsP { period: p });
                    exprs.push(Expr::PivotPointsR1 { period: p });
                    exprs.push(Expr::PivotPointsS1 { period: p });
                }
            }
            "FIB_RETRACEMENTS" => {
                for &p in grid {
                    exprs.push(Expr::FibLevel236 { period: p });
                    exprs.push(Expr::FibLevel382 { period: p });
                    exprs.push(Expr::FibLevel500 { period: p });
                    exprs.push(Expr::FibLevel618 { period: p });
                    exprs.push(Expr::FibLevel786 { period: p });
                }
            }
            "HEIKIN_ASHI" => { exprs.push(Expr::HeikinAshiClose); }
            "EHLERS_SUPER_SMOOTHER" => { for &p in grid { exprs.push(Expr::EhlersSuperSmoother { period: p }); } }
            "EHLERS_DECYCLER" => { for &p in grid { exprs.push(Expr::EhlersDecycler { period: p }); } }
            "EHLERS_CYBER_CYCLE" => { exprs.push(Expr::EhlersCyberCycle { alpha: 0.07 }); }
            "EHLERS_MAMA_FAMA" => { 
                exprs.push(Expr::EhlersMama { fast_limit: 0.5, slow_limit: 0.05 });
                exprs.push(Expr::EhlersFama { fast_limit: 0.5, slow_limit: 0.05 });
            }
            "EHLERS_SINE_WAVE" => {
                exprs.push(Expr::EhlersSine);
                exprs.push(Expr::EhlersLeadSine);
            }
            "EHLERS_DECYCLER_OSCILLATOR" => {
                for &p in grid {
                    exprs.push(Expr::EhlersDecyclerOscillator { hp_period1: p, hp_period2: p * 2 });
                }
            }
            "EHLERS_ROOFING_FILTER" => {
                for &p in grid {
                    exprs.push(Expr::EhlersRoofingFilter { hp_period: 40, lp_period: p });
                }
            }
            "EHLERS_DOMINANT_CYCLE_PERIOD" => { exprs.push(Expr::EhlersDominantCyclePeriod); }
            "EHLERS_AUTOCORRELATION_PERIODOGRAM" => { exprs.push(Expr::EhlersAutocorrelationPeriodogram); }
            "EHLERS_EMD" => {
                for &p in grid {
                    exprs.push(Expr::EhlersEmd { period: p, fraction: 0.1 });
                }
            }
            "MARKET_MEANNESS_INDEX" => {
                for &p in grid { exprs.push(Expr::MarketMeannessIndex { period: p }); }
            }
            "ZERO_LAG_MACD" => {
                let macd_params = [(12, 26, 9), (8, 21, 5), (10, 20, 5)];
                for (fast, slow, signal) in macd_params {
                    exprs.push(Expr::ZeroLagMacdLine { fast, slow, signal });
                    exprs.push(Expr::ZeroLagMacdSignal { fast, slow, signal });
                    exprs.push(Expr::ZeroLagMacdHist { fast, slow, signal });
                }
            }
            "GATOR_OSCILLATOR" => {
                exprs.push(Expr::GatorTop);
                exprs.push(Expr::GatorBottom);
            }
            "KALMAN_FILTER" => {
                exprs.push(Expr::KalmanFilter { r: 1.0, q: 0.1 });
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
