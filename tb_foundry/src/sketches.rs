use tb_core::{Expr, Sketch, TradeDirection};
use rand::seq::SliceRandom;
use rand::Rng;

/// Generates a seed population of valid strategy sketches (Generation 0).
pub fn generate_seed_population(population_size: usize, long_strategy_pct: f64) -> Vec<Sketch> {
    let mut rng = rand::thread_rng();
    let mut population = Vec::with_capacity(population_size);
    
    for i in 0..population_size {
        // Decide direction based on user configuration
        let direction = if rng.gen_bool(long_strategy_pct) {
            TradeDirection::Long
        } else {
            TradeDirection::Short
        };

        let template_idx = i % 3;
        
        let sketch = match template_idx {
            0 => generate_trend_following(i, &mut rng, direction),
            1 => generate_mean_reversion(i, &mut rng, direction),
            _ => generate_macd_cross(i, &mut rng, direction),
        };
        
        population.push(sketch);
    }
    
    population
}

/// Template 1: Trend Following (Fast MA vs Slow MA)
fn generate_trend_following(id: usize, rng: &mut rand::rngs::ThreadRng, direction: TradeDirection) -> Sketch {
    let fast_periods = [5, 10, 15, 20];
    let slow_periods = [50, 100, 200];
    
    let fast = *fast_periods.choose(rng).unwrap();
    let slow = *slow_periods.choose(rng).unwrap();
    
    let entry = match direction {
        TradeDirection::Long => Expr::CrossAbove {
            lhs: Box::new(Expr::Sma { source: Box::new(Expr::Close), period: fast }),
            rhs: Box::new(Expr::Sma { source: Box::new(Expr::Close), period: slow }),
        },
        TradeDirection::Short => Expr::CrossBelow {
            lhs: Box::new(Expr::Sma { source: Box::new(Expr::Close), period: fast }),
            rhs: Box::new(Expr::Sma { source: Box::new(Expr::Close), period: slow }),
        }
    };
    
    Sketch {
        name: format!("gen0_trend_{}", id),
        direction,
        entry,
        exit: None,
    }
}

/// Template 2: Mean Reversion (RSI Thresholds)
fn generate_mean_reversion(id: usize, rng: &mut rand::rngs::ThreadRng, direction: TradeDirection) -> Sketch {
    let rsi_periods = [7, 10, 14, 21];
    let thresholds = [20.0, 25.0, 30.0, 35.0];
    
    let period = *rsi_periods.choose(rng).unwrap();
    let threshold = *thresholds.choose(rng).unwrap();
    
    let entry = match direction {
        TradeDirection::Long => Expr::CrossBelow {
            lhs: Box::new(Expr::Rsi { source: Box::new(Expr::Close), period }),
            rhs: Box::new(Expr::Constant { value: threshold }),
        },
        TradeDirection::Short => Expr::CrossAbove {
            lhs: Box::new(Expr::Rsi { source: Box::new(Expr::Close), period }),
            rhs: Box::new(Expr::Constant { value: 100.0 - threshold }),
        }
    };
    
    Sketch {
        name: format!("gen0_meanrev_{}", id),
        direction,
        entry,
        exit: None,
    }
}

/// Template 3: MACD Crossover (Zero Line)
fn generate_macd_cross(id: usize, rng: &mut rand::rngs::ThreadRng, direction: TradeDirection) -> Sketch {
    let fast = *[8, 12, 14].choose(rng).unwrap();
    let slow = *[21, 26, 30].choose(rng).unwrap();
    
    let entry = match direction {
        TradeDirection::Long => Expr::CrossAbove {
            lhs: Box::new(Expr::MacdLine { source: Box::new(Expr::Close), fast, slow }),
            rhs: Box::new(Expr::Constant { value: 0.0 }),
        },
        TradeDirection::Short => Expr::CrossBelow {
            lhs: Box::new(Expr::MacdLine { source: Box::new(Expr::Close), fast, slow }),
            rhs: Box::new(Expr::Constant { value: 0.0 }),
        }
    };
    
    Sketch {
        name: format!("gen0_macd_{}", id),
        direction,
        entry,
        exit: None,
    }
}
