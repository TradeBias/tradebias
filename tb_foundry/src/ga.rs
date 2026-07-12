use tb_core::{Expr, Sketch, ExprType};
use rand::Rng; // We will need rand for mutations
use rand::seq::SliceRandom;
use std::boxed::Box;

/// A Tree-Aware Mutation Engine for the GA
pub struct MutationEngine {
    pub mutation_rate: f64,
    pub permitted_indicators: Vec<String>,
}

impl MutationEngine {
    pub fn new(mutation_rate: f64, permitted_indicators: Vec<String>) -> Self {
        Self { mutation_rate, permitted_indicators }
    }

    /// Recursively mutates the AST tree, strictly enforcing ExprType
    pub fn mutate_expr(&self, expr: &mut Expr) {
        let mut rng = rand::thread_rng();
        
        // Only mutate if we hit the probability
        if rng.gen_bool(self.mutation_rate) {
            match expr {
                // FLOAT MUTATIONS
                Expr::Constant { value } => {
                    let shift = rng.gen_range(-0.1..=0.1);
                    *value *= 1.0 + shift;
                    *value = (*value * 10.0).round() / 10.0; // Round to 1 decimal place
                }
                Expr::Sma { source, period } | Expr::Ema { source, period } | Expr::Rsi { source, period } => {
                    if rng.gen_bool(0.2) && !self.permitted_indicators.is_empty() {
                        let new_ind = self.permitted_indicators.choose(&mut rng).unwrap().as_str();
                        match new_ind {
                            "SMA" => *expr = Expr::Sma { source: source.clone(), period: *period },
                            "EMA" => *expr = Expr::Ema { source: source.clone(), period: *period },
                            "RSI" => *expr = Expr::Rsi { source: source.clone(), period: *period },
                            "ATR" => *expr = Expr::Atr { period: *period },
                            "MACD" => *expr = Expr::MacdLine { source: source.clone(), fast: 12, slow: 26 },
                            _ => {}
                        }
                        return;
                    }
                    
                    if rng.gen_bool(0.3) {
                        *period = rng.gen_range(5..=200); // Jump to prevent getting stuck
                    } else {
                        let shift: i32 = rng.gen_range(-10..=10);
                        *period = (*period as i32 + shift).clamp(5, 200) as u32; 
                    }
                }
                Expr::MacdLine { fast, slow, .. } => {
                    let shift: i32 = rng.gen_range(-2..=2);
                    *fast = (*fast as i32 + shift).clamp(3, 50) as u32;
                    *slow = (*slow as i32 + shift).clamp(*fast as i32 + 1, 200) as u32;
                }
                Expr::MacdSignal { fast, slow, signal, .. } | Expr::MacdHistogram { fast, slow, signal, .. } => {
                    let shift: i32 = rng.gen_range(-2..=2);
                    *fast = (*fast as i32 + shift).clamp(3, 50) as u32;
                    *slow = (*slow as i32 + shift).clamp(*fast as i32 + 1, 200) as u32;
                    *signal = (*signal as i32 + shift).clamp(3, 30) as u32;
                }
                Expr::BollingerUpper { period, .. } | Expr::BollingerLower { period, .. } => {
                    let shift: i32 = rng.gen_range(-2..=2);
                    *period = (*period as i32 + shift).clamp(5, 100) as u32;
                }
                Expr::Atr { period } => {
                    let shift: i32 = rng.gen_range(-2..=2);
                    *period = (*period as i32 + shift).clamp(5, 100) as u32;
                }
                
                // BOOLEAN MUTATIONS
                Expr::CrossAbove { lhs, rhs } => {
                    if rng.gen_bool(0.2) {
                        *expr = Expr::CrossBelow { lhs: lhs.clone(), rhs: rhs.clone() };
                        return;
                    } else if rng.gen_bool(0.2) {
                        *expr = Expr::GreaterThan { lhs: lhs.clone(), rhs: rhs.clone() };
                        return;
                    }
                }
                Expr::CrossBelow { lhs, rhs } => {
                    if rng.gen_bool(0.2) {
                        *expr = Expr::CrossAbove { lhs: lhs.clone(), rhs: rhs.clone() };
                        return;
                    } else if rng.gen_bool(0.2) {
                        *expr = Expr::LessThan { lhs: lhs.clone(), rhs: rhs.clone() };
                        return;
                    }
                }
                Expr::GreaterThan { lhs, rhs } => {
                    if rng.gen_bool(0.2) {
                        *expr = Expr::CrossAbove { lhs: lhs.clone(), rhs: rhs.clone() };
                        return;
                    }
                }
                Expr::LessThan { lhs, rhs } => {
                    if rng.gen_bool(0.2) {
                        *expr = Expr::CrossBelow { lhs: lhs.clone(), rhs: rhs.clone() };
                        return;
                    }
                }
                Expr::And { lhs, rhs } => {
                    if rng.gen_bool(0.2) {
                        *expr = Expr::Or { lhs: lhs.clone(), rhs: rhs.clone() };
                        return;
                    }
                }
                Expr::Or { lhs, rhs } => {
                    if rng.gen_bool(0.2) {
                        *expr = Expr::And { lhs: lhs.clone(), rhs: rhs.clone() };
                        return;
                    }
                }
                _ => {}
            }
        }
        
        // Recursively attempt to mutate children
        match expr {
            Expr::Sma { source, .. } | Expr::Ema { source, .. } | Expr::Rsi { source, .. } | Expr::MacdLine { source, .. } | Expr::MacdSignal { source, .. } | Expr::MacdHistogram { source, .. } | Expr::BollingerUpper { source, .. } | Expr::BollingerLower { source, .. } => {
                self.mutate_expr(source);
            }
            Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } | 
            Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
            Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } |
            Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => {
                self.mutate_expr(lhs);
                self.mutate_expr(rhs);
            }
            _ => {}
        }
    }

    pub fn mutate_sketch(&self, sketch: &mut Sketch) {
        self.mutate_expr(&mut sketch.entry);
        if let Some(ex) = &mut sketch.exit {
            self.mutate_expr(ex);
        }
    }
}

/// Helper to extract all nodes of a specific ExprType
fn get_nodes_of_type(expr: &Expr, target_type: ExprType) -> Vec<Expr> {
    let mut nodes = Vec::new();
    fn walk(e: &Expr, target_type: ExprType, nodes: &mut Vec<Expr>) {
        if e.return_type() == target_type {
            nodes.push(e.clone());
        }
        match e {
            Expr::Sma { source, .. } | Expr::Ema { source, .. } | Expr::Rsi { source, .. } | Expr::MacdLine { source, .. } | Expr::MacdSignal { source, .. } | Expr::MacdHistogram { source, .. } | Expr::BollingerUpper { source, .. } | Expr::BollingerLower { source, .. } => walk(source, target_type, nodes),
            Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } |
            Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
            Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } |
            Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => {
                walk(lhs, target_type, nodes);
                walk(rhs, target_type, nodes);
            }
            _ => {}
        }
    }
    walk(expr, target_type, &mut nodes);
    nodes
}

/// Helper to count nodes in an AST tree
fn count_nodes(e: &Expr) -> usize {
    match e {
        Expr::Sma { source, .. } | Expr::Ema { source, .. } | Expr::Rsi { source, .. } | Expr::MacdLine { source, .. } | Expr::MacdSignal { source, .. } | Expr::MacdHistogram { source, .. } | Expr::BollingerUpper { source, .. } | Expr::BollingerLower { source, .. } => 1 + count_nodes(source),
        Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } |
        Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
        Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } |
        Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => 1 + count_nodes(lhs) + count_nodes(rhs),
        _ => 1
    }
}

/// Extracts the Nth node from the tree
fn extract_node_at(e: &Expr, current: &mut usize, target: usize) -> Option<Expr> {
    if *current == target { return Some(e.clone()); }
    *current += 1;
    match e {
        Expr::Sma { source, .. } | Expr::Ema { source, .. } | Expr::Rsi { source, .. } | Expr::MacdLine { source, .. } | Expr::MacdSignal { source, .. } | Expr::MacdHistogram { source, .. } | Expr::BollingerUpper { source, .. } | Expr::BollingerLower { source, .. } => extract_node_at(source, current, target),
        Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } |
        Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
        Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } |
        Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => {
            if let Some(n) = extract_node_at(lhs, current, target) { return Some(n); }
            extract_node_at(rhs, current, target)
        }
        _ => None
    }
}

/// Replaces the Nth node in the tree with the donor
fn replace_at(e: &mut Expr, donor: Expr, current: &mut usize, target: usize) -> bool {
    if *current == target {
        *e = donor;
        return true;
    }
    *current += 1;
    match e {
        Expr::Sma { source, .. } | Expr::Ema { source, .. } | Expr::Wma { source, .. } | Expr::Hma { source, .. } | Expr::Dema { source, .. } | Expr::Tema { source, .. } | Expr::Kama { source, .. } | Expr::Smma { source, .. } | Expr::Alma { source, .. } | Expr::Rsi { source, .. } | Expr::MacdLine { source, .. } | Expr::MacdSignal { source, .. } | Expr::MacdHistogram { source, .. } | Expr::BollingerUpper { source, .. } | Expr::BollingerLower { source, .. } => {
            replace_at(source, donor, current, target)
        }
        Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } |
        Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
        Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } |
        Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => {
            if replace_at(lhs, donor.clone(), current, target) { return true; }
            replace_at(rhs, donor, current, target)
        }
        _ => false
    }
}

/// Helper to measure deep nesting of indicators
fn max_indicator_depth(e: &Expr) -> usize {
    match e {
        Expr::Close | Expr::Open | Expr::High | Expr::Low | Expr::Volume | Expr::Constant { .. } => 0,
        Expr::Sma { source, .. } | Expr::Ema { source, .. } | Expr::Wma { source, .. } | Expr::Hma { source, .. } | Expr::Dema { source, .. } | Expr::Tema { source, .. } | Expr::Kama { source, .. } | Expr::Smma { source, .. } | Expr::Alma { source, .. } | Expr::Rsi { source, .. } | Expr::MacdLine { source, .. } | Expr::MacdSignal { source, .. } | Expr::MacdHistogram { source, .. } | Expr::BollingerUpper { source, .. } | Expr::BollingerLower { source, .. } => {
            1 + max_indicator_depth(source)
        }
        Expr::Atr { .. } | Expr::KeltnerUpper { .. } | Expr::KeltnerLower { .. } | Expr::DonchianUpper { .. } | Expr::DonchianLower { .. } | Expr::DonchianMid { .. } | Expr::Supertrend { .. } | Expr::SupertrendDir { .. } | Expr::Psar { .. } | Expr::Adx { .. } | Expr::DiPlus { .. } | Expr::DiMinus { .. } | Expr::StochasticK { .. } | Expr::StochasticD { .. } | Expr::StochRsiK { .. } | Expr::StochRsiD { .. } | Expr::WilliamsR { .. } | Expr::Cci { .. } | Expr::Mfi { .. } | Expr::Roc { .. } | Expr::AwesomeOscillator { .. } | Expr::Tsi { .. } | Expr::UltimateOscillator { .. } | Expr::Dpo { .. } | Expr::Kst { .. } | Expr::KstSignal { .. } | Expr::FisherTransform { .. } | Expr::FisherTrigger { .. } | Expr::ConnorsRsi { .. } | Expr::Cmo { .. } | Expr::Rvi { .. } | Expr::RviSignal { .. } | Expr::Smi { .. } | Expr::Trix { .. } | Expr::Eom { .. } | Expr::VortexPlus { .. } | Expr::VortexMinus { .. } | Expr::DssBressert { .. } | Expr::Ppo { .. } | Expr::PpoSignal { .. } | Expr::PpoHist { .. } | Expr::ChoppinessIndex { .. } | Expr::QqeFast { .. } | Expr::QqeSlow { .. } | Expr::Stc { .. } | Expr::ChaikinVolatility { .. } | Expr::HistoricalVolatility { .. } | Expr::UlcerIndex { .. } | Expr::StandardDeviation { .. } | Expr::BollingerBandWidth { .. } | Expr::BollingerPercentB { .. } | Expr::KeltnerChannelWidth { .. } | Expr::VixSynthetic { .. } | Expr::Obv | Expr::Vwap | Expr::AdLine | Expr::ChaikinMoneyFlow { .. } | Expr::ChaikinOscillator { .. } | Expr::Pvt | Expr::Nvi | Expr::Pvi | Expr::ForceIndex { .. } | Expr::Vfi { .. } | Expr::VolumeOscillator { .. } | Expr::KlingerOscillator { .. } | Expr::Mvwap { .. } | Expr::Twap { .. } | Expr::LinRegSlope { .. } | Expr::LinRegIntercept { .. } | Expr::LinRegRSquared { .. } | Expr::LinRegCurve { .. } | Expr::StdErrorBandUpper { .. } | Expr::StdErrorBandLower { .. } | Expr::ZScore { .. } | Expr::LogReturn | Expr::MedianPrice | Expr::TypicalPrice | Expr::WeightedClose | Expr::HurstExponent { .. } | Expr::PivotPointsP { .. } | Expr::PivotPointsR1 { .. } | Expr::PivotPointsS1 { .. } | Expr::FibLevel236 { .. } | Expr::FibLevel382 { .. } | Expr::FibLevel500 { .. } | Expr::FibLevel618 { .. } | Expr::FibLevel786 { .. } | Expr::HeikinAshiClose | Expr::EhlersSuperSmoother { .. } | Expr::EhlersDecycler { .. } | Expr::EhlersCyberCycle { .. } | Expr::EhlersMama { .. } | Expr::EhlersFama { .. } | Expr::EhlersSine | Expr::EhlersLeadSine | Expr::EhlersDecyclerOscillator { .. } | Expr::EhlersRoofingFilter { .. } | Expr::EhlersDominantCyclePeriod | Expr::EhlersAutocorrelationPeriodogram | Expr::EhlersEmd { .. } | Expr::MarketMeannessIndex { .. } | Expr::ZeroLagMacdLine { .. } | Expr::ZeroLagMacdSignal { .. } | Expr::ZeroLagMacdHist { .. } | Expr::GatorTop | Expr::GatorBottom | Expr::KalmanFilter { .. } | Expr::BullishEngulfing | Expr::BearishEngulfing | Expr::Doji | Expr::Hammer | Expr::ShootingStar | Expr::MorningStar | Expr::EveningStar => 1,
        Expr::Add { lhs, rhs } | Expr::Sub { lhs, rhs } => {
            max_indicator_depth(lhs).max(max_indicator_depth(rhs))
        }
        Expr::CrossAbove { lhs, rhs } | Expr::CrossBelow { lhs, rhs } |
        Expr::GreaterThan { lhs, rhs } | Expr::LessThan { lhs, rhs } |
        Expr::And { lhs, rhs } | Expr::Or { lhs, rhs } => {
            max_indicator_depth(lhs).max(max_indicator_depth(rhs))
        }
    }
}

pub fn is_structurally_sound(sketch: &Sketch) -> bool {
    if sketch.entry.return_type() != tb_core::ExprType::Boolean { return false; }
    if let Some(ex) = &sketch.exit { if ex.return_type() != tb_core::ExprType::Boolean { return false; } }
    
    // Hard Caps for Curve Fitting
    if count_nodes(&sketch.entry) > 7 { return false; }
    if max_indicator_depth(&sketch.entry) > 2 { return false; }
    
    true
}

/// Type-Safe AST Subtree Crossover Operator
pub fn crossover_sketch(parent1: &Sketch, parent2: &Sketch) -> Sketch {
    let mut child = parent1.clone();
    let mut rng = rand::thread_rng();
    
    // 1. Pick a random target node in the child
    let total_nodes = count_nodes(&child.entry);
    let target_idx = rng.gen_range(0..total_nodes);
    
    // 2. Identify the target node's required ExprType (Float or Boolean)
    let mut curr = 0;
    let target_node = extract_node_at(&child.entry, &mut curr, target_idx).unwrap();
    let required_type = target_node.return_type();
    
    // 3. Find all nodes in parent2 that MATCH this exact ExprType
    let donor_candidates = get_nodes_of_type(&parent2.entry, required_type);
    if donor_candidates.is_empty() {
        return parent1.clone(); // Recombination failure (rare), return clone
    }
    
    // 4. Splice the matching donor into the target location
    let donor = donor_candidates.choose(&mut rng).unwrap().clone();
    let mut curr = 0;
    replace_at(&mut child.entry, donor, &mut curr, target_idx);
    
    // Hard cap complexity to prevent exponential tree growth and overfitting
    if count_nodes(&child.entry) > 7 {
        return parent1.clone(); 
    }
    
    // Prevent deep nesting like SMA(SMA(SMA(Close)))
    if max_indicator_depth(&child.entry) > 2 {
        return parent1.clone();
    }
    
    child
}
