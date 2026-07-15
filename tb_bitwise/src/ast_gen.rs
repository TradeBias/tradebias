use rand::Rng;
use rand::seq::SliceRandom;
use tb_core::ast::{Expr, SemanticType};

pub struct AstGenerator {
    pub max_depth: usize,
    pub permitted_periods: Vec<u32>,
    pub permitted_indicators: Vec<tb_core::ast::IndicatorBlueprint>,
}

impl AstGenerator {
    pub fn new(max_depth: usize, permitted_periods: &[u32], permitted_indicators: &[tb_core::ast::IndicatorBlueprint]) -> Self {
        Self {
            max_depth,
            permitted_periods: permitted_periods.to_vec(),
            permitted_indicators: permitted_indicators.to_vec(),
        }
    }

    pub fn generate_boolean_trigger(&self) -> Expr {
        let mut rng = rand::thread_rng();
        // The root node MUST be a relational operator to produce a boolean
        let op = rng.gen_range(0..4);
        
        // Randomly decide the target semantic type for this comparison
        let target_type = if rng.gen_bool(0.5) { SemanticType::Price } else { SemanticType::Ratio };
        
        let lhs = self.generate_node(target_type, 1);
        let rhs = self.generate_node(target_type, 1);

        let expr = match op {
            0 => Expr::GreaterThan { lhs: Box::new(lhs), rhs: Box::new(rhs) },
            1 => Expr::LessThan { lhs: Box::new(lhs), rhs: Box::new(rhs) },
            2 => Expr::CrossAbove { lhs: Box::new(lhs), rhs: Box::new(rhs) },
            _ => Expr::CrossBelow { lhs: Box::new(lhs), rhs: Box::new(rhs) },
        };
        tb_core::ast_simplifier::simplify(&expr)
    }

    fn generate_node(&self, target_type: SemanticType, current_depth: usize) -> Expr {
        let mut rng = rand::thread_rng();
        
        // If we hit max depth, we must generate a leaf node or a simple smoother of a leaf
        if current_depth >= self.max_depth {
            return self.generate_leaf(target_type);
        }

        // 50% chance to early exit to a leaf to keep trees shallow and realistic
        if rng.gen_bool(0.5) {
            return self.generate_leaf(target_type);
        }

        // Otherwise generate a branch node
        match target_type {
            SemanticType::Price => {
                let op = rng.gen_range(0..4);
                match op {
                    0 => { // Add Price + Price
                        let lhs = self.generate_node(SemanticType::Price, current_depth + 1);
                        let rhs = self.generate_node(SemanticType::Price, current_depth + 1);
                        Expr::Add { lhs: Box::new(lhs), rhs: Box::new(rhs) }
                    },
                    1 => { // Sub Price - Price
                        let lhs = self.generate_node(SemanticType::Price, current_depth + 1);
                        let rhs = self.generate_node(SemanticType::Price, current_depth + 1);
                        Expr::Sub { lhs: Box::new(lhs), rhs: Box::new(rhs) }
                    },
                    2 => { // Mul Price * Scalar
                        let lhs = self.generate_node(SemanticType::Price, current_depth + 1);
                        let rhs = self.generate_node(SemanticType::Scalar, current_depth + 1);
                        Expr::Mul { lhs: Box::new(lhs), rhs: Box::new(rhs) }
                    },
                    _ => { // Smoother / Indicator
                        let source = self.generate_node(SemanticType::Price, current_depth + 1);
                        let p = *self.permitted_periods.choose(&mut rng).unwrap();
                        
                        if !self.permitted_indicators.is_empty() && rng.gen_bool(0.7) {
                            // Filter for price-domain indicators
                            let price_inds: Vec<&tb_core::ast::IndicatorBlueprint> = self.permitted_indicators.iter().filter(|i| i.semantic_type == SemanticType::Price).collect();
                            if !price_inds.is_empty() {
                                let ind = price_inds.choose(&mut rng).unwrap();
                                let (output_name, _) = ind.outputs.choose(&mut rng).unwrap();
                                let mut params = vec![];
                                if ind.name == "BOLL" {
                                    params.push(("LEN".to_string(), p as f64));
                                    params.push(("MULT".to_string(), 2.0));
                                } else {
                                    params.push(("LEN".to_string(), p as f64));
                                }
                                Expr::Macro {
                                    name: ind.name.clone(),
                                    output: output_name.clone(),
                                    source: Box::new(source),
                                    params
                                }
                            } else {
                                Expr::Sma { source: Box::new(source), period: p }
                            }
                        } else {
                            let smooth_op = rng.gen_range(0..5);
                            match smooth_op {
                                0 => Expr::Sma { source: Box::new(source), period: p },
                                1 => Expr::Ema { source: Box::new(source), period: p },
                                2 => Expr::TsMax { source: Box::new(source), period: p },
                                3 => Expr::TsMin { source: Box::new(source), period: p },
                                _ => Expr::Delay { source: Box::new(source), period: p },
                            }
                        }
                    }
                }
            },
            SemanticType::Ratio => {
                let op = rng.gen_range(0..3);
                match op {
                    0 => { // Div Price / Price
                        let lhs = self.generate_node(SemanticType::Price, current_depth + 1);
                        let rhs = self.generate_node(SemanticType::Price, current_depth + 1);
                        Expr::Div { lhs: Box::new(lhs), rhs: Box::new(rhs) }
                    },
                    1 => { // LinRegSlope
                        let source = self.generate_node(SemanticType::Price, current_depth + 1);
                        let p = *self.permitted_periods.choose(&mut rng).unwrap();
                        Expr::LinRegSlope { source: Box::new(source), period: p }
                    },
                    _ => { // Ratio/Oscillator Indicators (MACD, RSI, ATR)
                        let source = self.generate_node(SemanticType::Price, current_depth + 1);
                        let p = *self.permitted_periods.choose(&mut rng).unwrap();
                        
                        if !self.permitted_indicators.is_empty() && rng.gen_bool(0.7) {
                            let ratio_inds: Vec<&tb_core::ast::IndicatorBlueprint> = self.permitted_indicators.iter().filter(|i| i.semantic_type == SemanticType::Ratio).collect();
                            if !ratio_inds.is_empty() {
                                let ind = ratio_inds.choose(&mut rng).unwrap();
                                let (output_name, _) = ind.outputs.choose(&mut rng).unwrap();
                                let mut params = vec![];
                                if ind.name == "MACD" {
                                    params.push(("FAST".to_string(), 12.0));
                                    params.push(("SLOW".to_string(), 26.0));
                                    params.push(("SIGNAL".to_string(), 9.0));
                                } else {
                                    params.push(("LEN".to_string(), p as f64));
                                }
                                Expr::Macro {
                                    name: ind.name.clone(),
                                    output: output_name.clone(),
                                    source: Box::new(source),
                                    params
                                }
                            } else {
                                Expr::StdDev { source: Box::new(source), period: p }
                            }
                        } else {
                            Expr::StdDev { source: Box::new(source), period: p }
                        }
                    }
                }
            },
            SemanticType::Scalar => {
                let val: f64 = rng.gen_range(0.1..5.0);
                Expr::Constant { value: (val * 10.0).round() / 10.0 }
            },
            _ => self.generate_leaf(target_type),
        }
    }

    fn generate_leaf(&self, target_type: SemanticType) -> Expr {
        let mut rng = rand::thread_rng();
        match target_type {
            SemanticType::Price => {
                let leaf = rng.gen_range(0..4);
                match leaf {
                    0 => Expr::Close,
                    1 => Expr::Open,
                    2 => Expr::High,
                    _ => Expr::Low,
                }
            },
            SemanticType::Volume => Expr::Volume,
            SemanticType::Scalar => {
                let val: f64 = rng.gen_range(0.1..5.0);
                Expr::Constant { value: (val * 10.0).round() / 10.0 }
            },
            _ => Expr::Constant { value: 1.0 }, // fallback
        }
    }
}
