use crossbeam_channel::Receiver;
use tb_core::{Phase2Config, Sketch};
use tracing::{info, warn};
use crate::error::SimulatorError;

use crate::metrics::TearSheet;
use tb_core::ast::EliteStrategy;

pub struct ExecutionSimulator {
    config: Phase2Config,
    elite_rx: Receiver<EliteStrategy>,
    data: polars::prelude::LazyFrame,
}

impl ExecutionSimulator {
    pub fn new(config: Phase2Config, elite_rx: Receiver<EliteStrategy>, mut data: polars::prelude::LazyFrame) -> Self {
        // Normalize column names to lowercase to prevent case issues
        if let Ok(schema) = data.schema() {
            let mut old_names = Vec::new();
            let mut new_names = Vec::new();
            for name in schema.iter_names() {
                let lower = name.to_lowercase();
                if name.as_str() != lower {
                    old_names.push(name.to_string());
                    new_names.push(lower);
                }
            }
            if !old_names.is_empty() {
                data = data.rename(old_names, new_names);
            }
        }
        Self { config, elite_rx, data }
    }

    // Removed run_wfo as tb_foundry is deprecated

    /// Spawns a background thread that constantly listens for new EliteStrategies from Phase 1,
    /// runs a full backtest simulation on them, and streams the TearSheet back to the UI.
    pub fn start_live_backtesting_listener(&self, ui_equity_tx: crossbeam_channel::Sender<Result<(EliteStrategy, TearSheet), String>>) {
        let data = self.data.clone();
        let rx = self.elite_rx.clone();
        let config = self.config.clone();
        
        std::thread::spawn(move || {
            let mut seen_signatures = std::collections::HashSet::new();
            
            for elite in rx {
                let sig = format!("{:?}", elite.sketch);
                if seen_signatures.contains(&sig) {
                    continue; // Skip duplicate elites to save WFO compute
                }
                seen_signatures.insert(sig);
                
                let mut ledger = crate::portfolio::Ledger::new(100_000.0);
                
                // Create a temporary simulator instance with the same config
                let sim = ExecutionSimulator {
                    config: config.clone(),
                    elite_rx: crossbeam_channel::unbounded().1, // dummy
                    data: data.clone(),
                };
                
                match sim.simulate_oos_slice(&elite.sketch, data.clone(), &mut ledger) {
                    Ok(_) => {
                        let tear_sheet = TearSheet::generate(&ledger);
                        let _ = ui_equity_tx.send(Ok((elite, tear_sheet)));
                    }
                    Err(e) => {
                        let _ = ui_equity_tx.send(Err(e.to_string()));
                    }
                }
            }
        });
    }

    fn simulate_oos_slice(&self, sketch: &Sketch, oos_data: polars::prelude::LazyFrame, ledger: &mut crate::portfolio::Ledger) -> Result<(), SimulatorError> {
        let entry_signal_expr = tb_core::ast_compiler::compile_ast_to_polars(&sketch.entry)
            .map_err(|e| SimulatorError::Evaluation(e.to_string()))?
            .alias("entry_signal");
            
        let exit_signal_expr = if let Some(exit_ast) = &sketch.exit {
            tb_core::ast_compiler::compile_ast_to_polars(exit_ast)
                .map_err(|e| SimulatorError::Evaluation(e.to_string()))?
                .alias("exit_signal")
        } else {
            polars::lazy::dsl::lit(false).alias("exit_signal")
        };
        
        let df = oos_data
            .with_columns(vec![entry_signal_expr, exit_signal_expr])
            .select(&[
                polars::lazy::dsl::col("close"), 
                polars::lazy::dsl::col("entry_signal"),
                polars::lazy::dsl::col("exit_signal")
            ])
            .collect()
            .map_err(|e| SimulatorError::Evaluation(e.to_string()))?;
            
        let close_series = df.column("close").map_err(|e| SimulatorError::Evaluation(e.to_string()))?;
        let entry_series = df.column("entry_signal").map_err(|e| SimulatorError::Evaluation(e.to_string()))?;
        let exit_series = df.column("exit_signal").map_err(|e| SimulatorError::Evaluation(e.to_string()))?;
        
        let closes = close_series.f64().map_err(|e| SimulatorError::Evaluation(e.to_string()))?;
        let entry_signals = entry_series.bool().map_err(|e| SimulatorError::Evaluation(e.to_string()))?;
        let exit_signals = exit_series.bool().map_err(|e| SimulatorError::Evaluation(e.to_string()))?;

        let mut active_position: Option<crate::portfolio::Position> = None;
        let mut extreme_price = 0.0;
        let trailing_stop_pct = self.config.trailing_stop_pct.unwrap_or(1.0);
        let stop_loss_pct = self.config.stop_loss_pct.unwrap_or(1.0);
        let take_profit_pct = self.config.take_profit_pct.unwrap_or(100.0);
        
        let global_offset = ledger.total_trades();
        
        for i in 0..df.height() {
            let close = closes.get(i).unwrap_or(0.0);
            let entry_signal = entry_signals.get(i).unwrap_or(false);
            let exit_signal = exit_signals.get(i).unwrap_or(false);
            let abs_index = global_offset + i;
            
            if let Some(pos) = &active_position {
                let mut risk_exit = false;
                
                match pos.side {
                    crate::portfolio::PositionSide::Long => {
                        if close > extreme_price { extreme_price = close; }
                        let trail = close <= extreme_price * (1.0 - trailing_stop_pct);
                        let sl = close <= pos.entry_price * (1.0 - stop_loss_pct);
                        let tp = close >= pos.entry_price * (1.0 + take_profit_pct);
                        risk_exit = trail || sl || tp;
                    },
                    crate::portfolio::PositionSide::Short => {
                        if close < extreme_price { extreme_price = close; }
                        let trail = close >= extreme_price * (1.0 + trailing_stop_pct);
                        let sl = close >= pos.entry_price * (1.0 + stop_loss_pct);
                        let tp = close <= pos.entry_price * (1.0 - take_profit_pct);
                        risk_exit = trail || sl || tp;
                    }
                }
                
                let mut time_exit = false;
                if sketch.exit.is_none() {
                    if abs_index - pos.entry_index >= 5 {
                        time_exit = true;
                    }
                }
                
                let alpha_exit = exit_signal || time_exit;
                
                if alpha_exit || risk_exit {
                    ledger.record_trade(pos, close, abs_index);
                    active_position = None;
                }
            }
            
            if active_position.is_none() {
                if entry_signal {
                    let side = match sketch.direction {
                        tb_core::TradeDirection::Long => crate::portfolio::PositionSide::Long,
                        tb_core::TradeDirection::Short => crate::portfolio::PositionSide::Short,
                        tb_core::TradeDirection::LongAndShort => crate::portfolio::PositionSide::Long, // TODO: Implement symmetric backtesting
                    };
                    
                    // Fixed 1% Risk Sizing
                    let risk_amount = ledger.current_balance * 0.01;
                    let risk_per_unit = close * stop_loss_pct;
                    let size = if risk_per_unit > 0.0 { risk_amount / risk_per_unit } else { 1.0 };
                    
                    active_position = Some(crate::portfolio::Position::new(side, close, size, abs_index));
                    extreme_price = close;
                }
            }
        }
        
        if let Some(pos) = &active_position {
            let last_close = closes.get(df.height() - 1).unwrap_or(0.0);
            ledger.record_trade(pos, last_close, global_offset + df.height() - 1);
        }
        
        Ok(())
    }
}
