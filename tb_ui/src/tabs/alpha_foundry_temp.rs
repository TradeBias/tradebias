use eframe::egui;
use egui_plot::{Plot, Points, PlotPoints};
use crate::state::TradingApp;

pub fn render(app: &mut TradingApp, ctx: &egui::Context, outer_ui: &mut egui::Ui) {
    // 1. Non-blocking receiver check for Phase 1
    if let Some(rx) = &app.foundry_rx {
        match rx.try_recv() {
            Ok(metrics) => {
                app.latest_metrics = Some(metrics);
            }
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                app.foundry_rx = None;
            }
            Err(crossbeam_channel::TryRecvError::Empty) => {}
        }
        ctx.request_repaint();
    }
    
    if let Some(rx) = &app.engine_rx {
        if let Ok(engine) = rx.try_recv() {
            app.bitwise_engine = Some(engine);
            app.engine_rx = None; // We only expect one engine per run
        }
    }
    
    // Phase 2 listener removed from Alpha Foundry tab

    let mut start_clicked = false;
    
    // Top Bar for Actions
    let top_frame = egui::Frame::default()
        .fill(crate::theme::WINDOW_FILL)
        .inner_margin(12.0)
        .corner_radius(8)
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(51, 51, 51)));

    egui::TopBottomPanel::top("alpha_action_bar")
        .frame(top_frame)
        .show_inside(outer_ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Alpha Foundry");
                ui.add_space(16.0);
                let btn_text = if app.show_alpha_settings { "⏴ Hide Configuration" } else { "⚙ Show Configuration" };
                if ui.button(btn_text).clicked() {
                    app.show_alpha_settings = !app.show_alpha_settings;
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    start_clicked = ui.button("🚀 Start Evolution (Bitwise SIMD)").clicked();
                });
            });
        });
        
    outer_ui.add_space(8.0);
        
    // Conditional Settings Drawer
    if app.show_alpha_settings {
        let drawer_frame = egui::Frame::default().fill(crate::theme::WINDOW_FILL).inner_margin(16.0).corner_radius(8);
        egui::SidePanel::left("alpha_settings_drawer")
            .resizable(true)
            .default_width(350.0)
            .frame(drawer_frame)
            .show_inside(outer_ui, |ui| {
                ui.heading("Configuration");
                ui.add_space(12.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("alpha_settings_grid")
                        .num_columns(2)
                        .spacing([40.0, 12.0])
                        .show(ui, |ui| {
                // --- Starting Equity ---
                ui.label("Starting Equity: $");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::DragValue::new(&mut app.config.phase1.starting_equity)
                        .speed(1000.0)
                        .range(100.0..=100_000_000.0)
                    );
                });
                ui.end_row();

                // --- Target / Exit Definitions ---
                ui.label(egui::RichText::new("Target / Exit Definitions").strong().size(14.0));
                ui.label(""); // empty cell
                ui.end_row();

                ui.label("Exit Logic:");
                
                let mut current_stop = app.config.phase1.stop_type.clone();
                let available_types = tb_core::stops::StopType::available_types();
                let mut stop_idx = match current_stop {
                    tb_core::stops::StopType::FixedBarHold { .. } => 0,
                    tb_core::stops::StopType::StandardStop { .. } => 1,
                    tb_core::stops::StopType::TrailingStop { .. } => 2,
                };
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::from_id_source("stop_type_dropdown")
                        .selected_text(available_types[stop_idx].0)
                        .show_ui(ui, |ui| {
                            for (i, (name, _)) in available_types.iter().enumerate() {
                                ui.selectable_value(&mut stop_idx, i, *name);
                            }
                        });
                });
                ui.end_row();
                    
                match stop_idx {
                    0 => {
                        ui.label("Bars to Hold:");
                        let mut bars = if let tb_core::stops::StopType::FixedBarHold { bars } = current_stop { bars } else { 5 };
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(egui::Slider::new(&mut bars, 1..=20).text("Bars"));
                        });
                        ui.end_row();
                        app.config.phase1.stop_type = tb_core::stops::StopType::FixedBarHold { bars };
                    }
                    1 | 2 => {
                        let mut calc = match current_stop {
                            tb_core::stops::StopType::StandardStop { calc } => calc,
                            tb_core::stops::StopType::TrailingStop { calc } => calc,
                            _ => tb_core::stops::StopCalculation::Atr { multiplier: 2.0 },
                        };
                        
                        let available_calcs = tb_core::stops::StopCalculation::available();
                        
                        ui.label("Calculation:");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut calc_idx = available_calcs.iter().position(|(c, _, _)| std::mem::discriminant(c) == std::mem::discriminant(&calc)).unwrap_or(0);
                            egui::ComboBox::from_id_source("calc_type_dropdown")
                                .selected_text(available_calcs[calc_idx].1)
                                .show_ui(ui, |ui| {
                                    for (i, (_, name, _)) in available_calcs.iter().enumerate() {
                                        ui.selectable_value(&mut calc_idx, i, *name);
                                    }
                                });
                                
                            if std::mem::discriminant(&calc) != std::mem::discriminant(&available_calcs[calc_idx].0) {
                                calc = available_calcs[calc_idx].0.clone();
                            }
                            
                            let spec = &available_calcs[calc_idx].2;
                            let mut val = calc.get_val();
                            ui.add(egui::Slider::new(&mut val, spec.min..=spec.max));
                            calc.update_val(val);
                        });
                        ui.end_row();
                        
                        if stop_idx == 1 {
                            app.config.phase1.stop_type = tb_core::stops::StopType::StandardStop { calc };
                        } else {
                            app.config.phase1.stop_type = tb_core::stops::StopType::TrailingStop { calc };
                        }
                    }
                    _ => {}
                }
                
                if stop_idx != 0 {
                    ui.label("Take Profit:");
                    let mut current_tp = app.config.phase1.take_profit.clone();
                    if stop_idx == 1 && matches!(current_tp, tb_core::stops::TakeProfit::None) {
                        current_tp = tb_core::stops::TakeProfit::RiskReward { multiplier: 2.0 };
                    }
                    let available_tps = tb_core::stops::TakeProfit::available();
                    let mut tp_idx = available_tps.iter().position(|(t, _, _)| std::mem::discriminant(t) == std::mem::discriminant(&current_tp)).unwrap_or(0);
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::ComboBox::from_id_source("tp_type_dropdown")
                            .selected_text(available_tps[tp_idx].1)
                            .show_ui(ui, |ui| {
                                for (i, (tp, name, _)) in available_tps.iter().enumerate() {
                                    if stop_idx == 1 && matches!(tp, tb_core::stops::TakeProfit::None) {
                                        continue;
                                    }
                                    ui.selectable_value(&mut tp_idx, i, *name);
                                }
                            });
                            
                        if std::mem::discriminant(&current_tp) != std::mem::discriminant(&available_tps[tp_idx].0) {
                            current_tp = available_tps[tp_idx].0.clone();
                        }
                        
                        if let Some(spec) = &available_tps[tp_idx].2 {
                            let mut val = current_tp.get_val();
                            ui.add(egui::Slider::new(&mut val, spec.min..=spec.max));
                            current_tp.update_val(val);
                        }
                    });
                    ui.end_row();
                    app.config.phase1.take_profit = current_tp;
                }
                
                ui.label("Trade Direction:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let current_dir = match app.config.phase1.trade_direction {
                        tb_core::ast::TradeDirection::Long => "Long Only",
                        tb_core::ast::TradeDirection::Short => "Short Only",
                        tb_core::ast::TradeDirection::LongAndShort => "Long & Short",
                    };
                    
                    egui::ComboBox::from_id_source("trade_dir_dropdown")
                        .selected_text(current_dir)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.config.phase1.trade_direction, tb_core::ast::TradeDirection::Long, "Long Only");
                            ui.selectable_value(&mut app.config.phase1.trade_direction, tb_core::ast::TradeDirection::Short, "Short Only");
                            ui.selectable_value(&mut app.config.phase1.trade_direction, tb_core::ast::TradeDirection::LongAndShort, "Long & Short");
                        });
                });
                ui.end_row();

                // --- Diversity Grid ---
                ui.label(egui::RichText::new("Diversity Grid").strong().size(14.0));
                ui.label("");
                ui.end_row();

                ui.label("Diversity Trait 1:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let x_options = tb_core::archive::ArchiveTrait::available();
                    let current_x_label = x_options.iter().find(|(t, _)| *t == app.config.phase1.map_x).map(|(_, l)| *l).unwrap_or("Unknown");
                    egui::ComboBox::from_id_source("map_x_combo")
                        .selected_text(current_x_label)
                        .show_ui(ui, |ui| {
                            for (trait_val, label) in &x_options {
                                ui.selectable_value(&mut app.config.phase1.map_x, trait_val.clone(), *label);
                            }
                        });
                });
                ui.end_row();

                ui.label("Diversity Trait 2:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let y_options = tb_core::archive::ArchiveTrait::available();
                    let current_y_label = y_options.iter().find(|(t, _)| *t == app.config.phase1.map_y).map(|(_, l)| *l).unwrap_or("Unknown");
                    egui::ComboBox::from_id_source("map_y_combo")
                        .selected_text(current_y_label)
                        .show_ui(ui, |ui| {
                            for (trait_val, label) in &y_options {
                                ui.selectable_value(&mut app.config.phase1.map_y, trait_val.clone(), *label);
                            }
                        });
                });
                ui.end_row();

                ui.label("Grid Resolution:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::from_id_source("grid_size_combo")
                        .selected_text(format!("{}x{}", app.config.phase1.grid_size, app.config.phase1.grid_size))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.config.phase1.grid_size, 10, "10x10 (Low)");
                            ui.selectable_value(&mut app.config.phase1.grid_size, 25, "25x25 (Medium)");
                            ui.selectable_value(&mut app.config.phase1.grid_size, 50, "50x50 (High)");
                        });
                });
                ui.end_row();

                // --- Fitness Metric ---
                ui.label(egui::RichText::new("Fitness Metric").strong().size(14.0));
                ui.label("");
                ui.end_row();

                ui.label("Optimize Cells For:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let available = tb_core::fitness::FitnessFunction::available();
                    let current_label = available.iter()
                        .find(|(f, _)| std::mem::discriminant(f) == std::mem::discriminant(&app.config.phase1.fitness))
                        .map(|(_, l)| *l)
                        .unwrap_or("Select");
                    
                    egui::ComboBox::from_id_source("fitness_metric_combo")
                        .selected_text(current_label)
                        .show_ui(ui, |ui| {
                            for (func, label) in available {
                                ui.selectable_value(&mut app.config.phase1.fitness, func, label);
                            }
                        });
                });
                ui.end_row();

                // --- Data Validation ---
                ui.label(egui::RichText::new("Data Validation").strong().size(14.0));
                ui.label("");
                ui.end_row();

                ui.label("In-Sample Split:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut app.config.phase1.in_sample_pct, 0.1..=0.9).text("%").custom_formatter(|n, _| format!("{:.0}%", n * 100.0)));
                });
                ui.end_row();

                // --- Robustness Filters ---
                ui.label(egui::RichText::new("Robustness Filters").strong().size(14.0));
                ui.label("");
                ui.end_row();

                ui.label("Occam's Penalty:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut app.config.phase1.occam_penalty_pct, 0.0..=0.25).text("%").custom_formatter(|n, _| format!("{:.1}%", n * 100.0)));
                });
                ui.end_row();

                ui.label("Transaction Cost:")
                    .on_hover_text("Combined Slippage & Commission points per trade.");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut app.config.phase1.slippage_penalty, 0.0..=0.0100).step_by(0.0001).text("pts"));
                });
                ui.end_row();

                ui.label("Min Trade Count:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut app.config.phase1.min_trades, 1..=500));
                });
                ui.end_row();

                ui.label("Max Exposure:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut app.config.phase1.max_exposure, 0.01..=1.0).text("%").custom_formatter(|n, _| format!("{:.0}%", n * 100.0)));
                });
                ui.end_row();

                ui.label("Dumb Luck Filter:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut app.config.phase1.random_benchmark_percentile, 0.5..=0.99).text("PCTL").custom_formatter(|n, _| format!("{:.0}th", n * 100.0)));
                });
                ui.end_row();

                // --- AST Complexity ---
                ui.label(egui::RichText::new("Genome Complexity").strong().size(14.0));
                ui.label("");
                ui.end_row();

                ui.label("Max Trigger Rules:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut app.config.phase1.max_num_rules, 1..=20));
                });
                ui.end_row();

                ui.label("Max AST Tree Depth:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Slider::new(&mut app.config.phase1.max_ast_depth, 1..=10));
                });
                ui.end_row();

                // --- Indicator Universe ---
                ui.label(egui::RichText::new("Indicator Universe").strong().size(14.0));
                ui.label("");
                ui.end_row();

                ui.label("Selected:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙ Configure").clicked() {
                        app.show_indicator_modal = true;
                    }
                    ui.label(format!("{} Indicators", app.config.phase1.permitted_indicators.len()));
                });
                ui.end_row();

            }); // End Grid
                }); // End ScrollArea
            }); // End SidePanel Drawer
    }

    if start_clicked {
            if let Some(loaded_data) = &app.loaded_data {
                let (ui_tx, ui_rx) = crossbeam_channel::unbounded();
                app.foundry_rx = Some(ui_rx);
                
                let elite_tx = app.elite_tx.clone().expect("Elite tx must be initialized");
                let elite_rx = app.elite_rx.clone().expect("Elite rx must be initialized");
                
                let (engine_tx, engine_rx) = crossbeam_channel::unbounded();
                app.engine_tx = Some(engine_tx.clone());
                app.engine_rx = Some(engine_rx);
                
                let lf = loaded_data.clone();
                let phase1_config = app.config.phase1.clone();
                let phase2_config = app.config.phase2.clone();
                
                // IS / OOS Data Split
                let df_height = match lf.clone().collect() {
                    Ok(df) => df.height() as u32,
                    Err(_) => 10000, // Fallback
                };
                let is_len = (df_height as f64 * phase1_config.in_sample_pct) as u32;
                let oos_len = df_height.saturating_sub(is_len);
                
                let is_data = lf.clone().slice(0, is_len);
                let oos_data = lf.clone().slice(is_len as i64, oos_len);
                
                // Removed Phase 2 Live Backtesting Listener spawn
                
                // Spawn Phase 1 Evolutionary Engine (Uses In-Sample Data)
                std::thread::spawn(move || {
                    use std::collections::HashSet;
                    use tb_bitwise::targets::TargetOutcomes;
                    use tb_bitwise::engine::BitwiseEngine;
                    use tb_bitwise::ga::GeneticAlgorithm;
                    use tb_core::ast::EliteStrategy;

                    // 1. Materialize Polars DataFrame to Vec<f64>
                    let df = is_data.collect().unwrap();
                    
                    let close_series = df.column("close").or_else(|_| df.column("Close")).expect("Missing Close column").cast(&polars::prelude::DataType::Float64).unwrap();
                    let closes: Vec<f64> = close_series.f64().unwrap().into_no_null_iter().collect();
                    
                    let open_series = df.column("open").or_else(|_| df.column("Open")).unwrap_or(&close_series).cast(&polars::prelude::DataType::Float64).unwrap();
                    let opens: Vec<f64> = open_series.f64().unwrap().into_no_null_iter().collect();
                    
                    let high_series = df.column("high").or_else(|_| df.column("High")).unwrap_or(&close_series).cast(&polars::prelude::DataType::Float64).unwrap();
                    let highs: Vec<f64> = high_series.f64().unwrap().into_no_null_iter().collect();
                    
                    let low_series = df.column("low").or_else(|_| df.column("Low")).unwrap_or(&close_series).cast(&polars::prelude::DataType::Float64).unwrap();
                    let lows: Vec<f64> = low_series.f64().unwrap().into_no_null_iter().collect();
                    
                    let volumes: Vec<f64> = if let Ok(vol) = df.column("volume").or_else(|_| df.column("Volume")) {
                        vol.cast(&polars::prelude::DataType::Float64).unwrap().f64().unwrap().into_no_null_iter().collect()
                    } else {
                        vec![0.0; closes.len()]
                    };
                    
                    let ui_tx = ui_tx.clone();
                    let _ = ui_tx.send(crate::state::GenerationMetrics {
                        generation: 0, elapsed_seconds: 0.0, total_generated: 0, total_discarded: 0, strategies: vec![],
                        status_msg: Some("Building strategy components...".to_string())
                    });

                    // 2. Precompute Engine Cache
                    let raw_data = tb_bitwise::data::RawData::from_arrays(opens.clone(), highs.clone(), lows.clone(), closes.clone(), volumes.clone()).unwrap();
                    let cache = tb_bitwise::precompute::EngineCache::new(&raw_data, &[7, 14, 21, 50, 100, 200]);
                    let arc_cache = std::sync::Arc::new(cache);

                    // 4. Generate Target Outcomes (Advanced Stops)
                    let atr_data = match (&phase1_config.stop_type, &phase1_config.take_profit) {
                        (tb_core::stops::StopType::StandardStop { calc: tb_core::stops::StopCalculation::Atr { .. } }, _) |
                        (tb_core::stops::StopType::TrailingStop { calc: tb_core::stops::StopCalculation::Atr { .. } }, _) |
                        (_, tb_core::stops::TakeProfit::Atr { .. }) => {
                            let tr = tb_math::primitives::true_range(&highs, &lows, &closes);
                            Some(tb_math::primitives::rma(&tr, 14))
                        }
                        _ => None,
                    };
                    
                    let targets = TargetOutcomes::generate_advanced_stop(
                        &raw_data, 
                        &phase1_config.stop_type, 
                        &phase1_config.take_profit,
                        atr_data.as_deref(),
                        phase1_config.slippage_penalty
                    );
                    let arc_targets = std::sync::Arc::new(targets);

                    // 5. Run the Genetic Algorithm
                    let engine = std::sync::Arc::new(BitwiseEngine::new(arc_cache, arc_targets));
                    let ast_gen = tb_bitwise::ast_gen::AstGenerator::new(phase1_config.max_ast_depth, &[7, 14, 21, 50, 100, 200], &phase1_config.permitted_indicators);
                    let archive = tb_bitwise::archive::MapArchive::new(phase1_config.map_x.clone(), phase1_config.map_y.clone(), phase1_config.fitness.clone(), phase1_config.grid_size, closes.len() as u32, phase1_config.max_num_rules, phase1_config.min_trades, phase1_config.max_exposure, phase1_config.occam_penalty_pct);
                    let mut ga = GeneticAlgorithm::new(engine.clone(), 5_000, 50, phase1_config.trade_direction.clone(), archive, phase1_config.random_benchmark_percentile, ast_gen); // 50 Generations
                    
                    let _ = ui_tx.send(crate::state::GenerationMetrics {
                        generation: 0, elapsed_seconds: 0.0, total_generated: 0, total_discarded: 0, strategies: vec![],
                        status_msg: Some("Running Genetic Algorithm...".to_string())
                    });

                    let start_eval = std::time::Instant::now();
                    let ui_tx_clone = ui_tx.clone();
                    let pop_size = 5_000;
                    ga.run(move |gen_idx, elapsed_secs| {
                        let _ = ui_tx_clone.send(crate::state::GenerationMetrics {
                            generation: gen_idx,
                            elapsed_seconds: elapsed_secs,
                            total_generated: gen_idx * pop_size,
                            total_discarded: 0,
                            strategies: vec![],
                            status_msg: Some("Running Genetic Algorithm...".to_string()),
                        });
                    });
                    let elapsed = start_eval.elapsed();

                    // 6. Extract Kings and send to UI
                    let final_kings: Vec<_> = ga.archive.grid.iter().flat_map(|col| col.iter()).filter_map(|c| c.clone()).collect();
                    let mut elite_strategies = Vec::new();

                    for king in final_kings {
                        if let Some(sketch) = tb_bitwise::translator::translate_to_sketch(&king, phase1_config.trade_direction.clone()) {
                            let elite = EliteStrategy {
                                sketch,
                                fitness: king.metrics.win_rate,
                                pnl: king.metrics.total_pnl,
                                max_drawdown: king.metrics.max_drawdown,
                                pnl_over_dd: king.metrics.pnl_over_dd,
                                sharpe: king.metrics.sharpe,
                                sortino: king.metrics.sortino,
                                profit_factor: king.metrics.profit_factor,
                                cpc_index: king.metrics.cpc_index,
                                corr_coef: king.metrics.corr_coef,
                                avg_trade: king.metrics.avg_trade,
                                avg_win: king.metrics.avg_win,
                                avg_loss: king.metrics.avg_loss,
                                std_win: king.metrics.std_win,
                                std_loss: king.metrics.std_loss,
                                largest_win: king.metrics.largest_win,
                                largest_loss: king.metrics.largest_loss,
                                max_consecutive_losses: king.metrics.max_consecutive_losses,
                                exposure_pct: king.metrics.exposure_pct,
                                indicator_count: king.conditions.len() as u8,
                                num_trades: king.metrics.total_trades,
                                conditions: king.conditions.clone(),
                            };
                            elite_strategies.push(elite.clone());
                            let _ = elite_tx.send(elite); // Send to Simulator
                        }
                    }

                    // Send Engine to UI
                    let _ = engine_tx.send(engine.clone());

                    // Send Metrics to UI
                    let _ = ui_tx.send(crate::state::GenerationMetrics {
                        generation: 50,
                        elapsed_seconds: elapsed.as_secs_f64(),
                        total_generated: 250_000,
                        total_discarded: 0,
                        strategies: elite_strategies,
                        status_msg: None,
                    });
                });
            } else {
                println!("Cannot start evolution: No data loaded in Sandbox.");
            }
        }


    if let Some(metrics) = &app.latest_metrics {
        let is_running = app.foundry_rx.is_some();
        let is_precomputing = metrics.status_msg.as_ref().map_or(false, |m| m.contains("Precomputing"));
        let current_gen = metrics.generation;

        egui::TopBottomPanel::bottom("foundry_stats_panel").show_inside(outer_ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let speed = if metrics.elapsed_seconds > 0.0 {
                        (metrics.total_generated as f64 / metrics.elapsed_seconds) as u32
                    } else {
                        0
                    };
                    ui.label(egui::RichText::new(format!("⚡ {} strategies/sec", speed)).strong());
                    ui.separator();
                    ui.label(format!("Total Generated: {}", metrics.total_generated));
                    ui.separator();
                    ui.label(format!("Total Discarded: {}", metrics.total_discarded));
                    ui.separator();
                    ui.label(format!("Time Elapsed: {:.1}s", metrics.elapsed_seconds));
                });

                if is_running && !is_precomputing {
                    ui.add_space(4.0);
                    let progress = current_gen as f32 / 50.0;
                    
                    ui.add(
                        egui::ProgressBar::new(progress)
                            .text(format!("Generation {} / 50", current_gen))
                            .fill(egui::Color32::from_gray(128))
                            .animate(true)
                    );
                }
            });
        });
    }

    let is_wide = ctx.screen_rect().width() > 1200.0;
    
    if is_wide {
        app.show_robustness_modal = false;
        
        let card_frame = egui::Frame::default()
            .fill(crate::theme::WINDOW_FILL)
            .inner_margin(16.0)
            .corner_radius(8)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(51, 51, 51)));
            
        egui::SidePanel::right("robustness_panel")
            .resizable(true)
            .default_width(400.0)
            .frame(card_frame)
            .show_inside(outer_ui, |ui| {
                crate::components::modals::render_robustness_ui(app, ui);
            });
    }

    let central_frame = egui::Frame::default().fill(ctx.style().visuals.panel_fill).inner_margin(16.0);
    egui::CentralPanel::default().frame(central_frame).show_inside(outer_ui, |ui| {
        ui.heading("Alpha Foundry - Leaderboard (Phase 1)");
        
        if let Some(metrics) = &app.latest_metrics {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Generation: {}/50", metrics.generation)).strong().color(egui::Color32::GREEN));
                
                if ui.button("💾 Export Leaderboard to CSV").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("CSV", &["csv"])
                        .set_file_name("phase1_results.csv")
                        .save_file() 
                    {
                        if let Err(e) = crate::export::export_leaderboard_to_csv(&path, &app.config.phase1, metrics) {
                            eprintln!("Failed to write CSV: {}", e);
                        }
                    }
                }
                
                ui.add_space(8.0);
                if ui.button("⚙ Metric Filters").clicked() {
                    app.show_metric_filters_modal = true;
                }
                
                ui.add_space(8.0);
                if ui.button("🔬 Robustness Report").clicked() {
                    if let (Some(engine), Some(idx)) = (&app.bitwise_engine, app.selected_strategy_idx) {
                        if let Some(metrics) = &app.latest_metrics {
                            if idx < metrics.strategies.len() {
                                let strategy = &metrics.strategies[idx];
                                let report = tb_bitwise::robustness::generate_report(
                                    engine,
                                    &strategy.conditions,
                                    &app.config.phase1.trade_direction,
                                    app.robustness_noise_pct / 100.0,
                                    app.robustness_top_n_drop as f64 / 100.0,
                                );
                                app.robustness_report = Some(report);
                                app.show_robustness_modal = true;
                            }
                        }
                    }
                }
            });
            ui.add_space(16.0);
            
            let is_running = app.foundry_rx.is_some();
            let mut is_precomputing = false;
            if let Some(metrics) = &app.latest_metrics {
                if let Some(msg) = &metrics.status_msg {
                    if msg.contains("Building") || msg.contains("Precomputing") {
                        is_precomputing = true;
                    }
                }
            }

            if is_running && is_precomputing {
                ui.add_space(80.0);
                ui.vertical_centered(|ui| {
                    ui.spinner();
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("Precomputing Indicators...").size(16.0));
                });
            } else {
                egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("leaderboard_grid")
                    .striped(true)
                    .spacing([40.0, 8.0])
                    .show(ui, |ui| {
                        // Header
                        ui.label(egui::RichText::new("AST Structure (Logic)").strong());
                        ui.label(egui::RichText::new("Total PnL").strong().color(egui::Color32::LIGHT_GREEN));
                        ui.label(egui::RichText::new("Trades").strong());
                        ui.label(egui::RichText::new("Avg Trade").strong()).on_hover_text("Expected $ per trade");
                        ui.label(egui::RichText::new("Win %").strong());
                        ui.label(egui::RichText::new("Exposure (%)").strong());
                        
                        ui.label(egui::RichText::new("Ratio W/L").strong());
                        ui.label(egui::RichText::new("Profit Factor").strong());
                        
                        ui.label(egui::RichText::new("Max DD").strong()).on_hover_text("Max Drawdown");
                        ui.label(egui::RichText::new("Max Cons. Loss").strong());
                        ui.label(egui::RichText::new("Largest Win/Loss").strong());
                        ui.label(egui::RichText::new("PnL/DD").strong());
                        
                        ui.label(egui::RichText::new("Sharpe").strong());
                        ui.label(egui::RichText::new("Sortino").strong());
                        ui.label(egui::RichText::new("Corr Coef").strong()).on_hover_text("Equity curve straightness (1.0 = perfect)");
                        ui.label(egui::RichText::new("Complexity").strong());
                        ui.end_row();

                        // Rows
                        let filtered_strategies: Vec<_> = metrics.strategies.iter().filter(|s| {
                            s.pnl >= app.min_pnl_filter && s.pnl <= app.max_pnl_filter &&
                            s.fitness >= app.min_win_rate_filter && s.fitness <= app.max_win_rate_filter &&
                            s.corr_coef >= app.min_corr_coef_filter && 
                            s.max_consecutive_losses <= app.max_cons_loss_filter as u32
                        })
                        .collect();
                        
                        for (idx, strategy) in filtered_strategies.iter().enumerate() {
                            // 1. AST Structure
                            let is_selected = app.selected_strategy_idx == Some(idx);
                            let full_ast = format!("{}", strategy.sketch.entry);
                            let mut display_ast = full_ast.clone();
                            if display_ast.len() > 45 {
                                display_ast.truncate(42);
                                display_ast.push_str("...");
                            }
                            
                            if ui.selectable_label(is_selected, display_ast).on_hover_text(full_ast).clicked() {
                                app.selected_strategy_idx = Some(idx);
                                app.robustness_disabled_conditions.clear();
                                
                                if let Some(engine) = &app.bitwise_engine {
                                    let report = tb_bitwise::robustness::generate_report(
                                        engine,
                                        &strategy.conditions,
                                        &app.config.phase1.trade_direction,
                                        app.robustness_noise_pct / 100.0,
                                        app.robustness_top_n_drop as f64 / 100.0,
                                    );
                                    app.robustness_report = Some(report);
                                }
                                app.show_robustness_modal = true;
                            }
                            
                            // 2. Total PnL
                            ui.colored_label(
                                if strategy.pnl >= 0.0 { egui::Color32::LIGHT_GREEN } else { egui::Color32::LIGHT_RED },
                                format!("{:.2}", strategy.pnl)
                            );
                            
                            // 2.5. Trades
                            ui.label(format!("{}", strategy.num_trades));

                            // 3. Avg Trade
                            ui.label(format!("{:.2}", strategy.avg_trade));
                            
                            // 4. Win %
                            ui.label(format!("{:.1}%", strategy.fitness)); // Using fitness field which is populated with win_rate

                            // 5. Exposure %
                            ui.label(format!("{:.1}%", strategy.exposure_pct));
                            
                            // 6. Ratio W/L
                            let ratio_wl = if strategy.avg_loss > 0.0 { strategy.avg_win / strategy.avg_loss } else { strategy.avg_win };
                            ui.label(format!("{:.2}", ratio_wl));

                            // 7. Profit Factor
                            ui.colored_label(
                                if strategy.profit_factor >= 1.5 { egui::Color32::LIGHT_GREEN } else { ui.visuals().text_color() },
                                format!("{:.2}", strategy.profit_factor)
                            );
                            
                            // 8. Max DD
                            ui.colored_label(egui::Color32::LIGHT_RED, format!("{:.2}", strategy.max_drawdown));
                            
                            // 9. Max Cons. Loss
                            ui.label(format!("{}", strategy.max_consecutive_losses));
                            
                            // 10. Largest Win/Loss
                            ui.label(format!("{:.1} / {:.1}", strategy.largest_win, strategy.largest_loss));
                            
                            // 11. PnL/DD
                            ui.label(format!("{:.2}", strategy.pnl_over_dd));

                            // 12. Sharpe
                            ui.label(format!("{:.2}", strategy.sharpe));

                            // 13. Sortino
                            ui.label(format!("{:.2}", strategy.sortino));
                            
                            // 14. Corr Coef
                            ui.label(format!("{:.3}", strategy.corr_coef));

                            // 15. Complexity
                            ui.label(format!("{}", strategy.indicator_count));
                            
                            ui.end_row();
                        }
                    });
                }); // Scroll area end
            }
        } else {
            ui.label("Evolution metrics and generated strategies will appear here.");
            if app.loaded_data.is_none() {
                ui.label(egui::RichText::new("⚠️ Please load a dataset in the Data Sandbox first.").color(egui::Color32::RED));
            }
        }
    });

    if app.show_indicator_modal {
        let mut is_open = app.show_indicator_modal;
        egui::Window::new("Indicator Universe")
            .open(&mut is_open)
            .collapsible(false)
            .resizable(true)
            .default_width(400.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                ui.label("Select indicators available for genetic mutation:");
                ui.add_space(8.0);
                
                ui.horizontal(|ui| {
                    if ui.button("Enable All").clicked() {
                        let mut all = tb_indicators::templates::default_blueprints();
                        all.extend(app.forge_blueprints.clone());
                        app.config.phase1.permitted_indicators = all;
                    }
                    if ui.button("Disable All").clicked() {
                        app.config.phase1.permitted_indicators.clear();
                    }
                });
                ui.add_space(8.0);
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let grouped_indicators = vec![
                        ("Momentum", vec!["SMA", "EMA", "RSI"]),
                        ("Trend", vec!["MACD"]),
                        ("Volatility", vec!["ATR", "BOLL"]),
                    ];
                    
                    let all_blueprints = tb_indicators::templates::default_blueprints();
                    
                    for (cat, names) in grouped_indicators {
                        ui.label(egui::RichText::new(cat).strong());
                        ui.add_space(4.0);
                        for ind in names {
                            let mut is_enabled = app.config.phase1.permitted_indicators.iter().any(|b| b.name == ind);
                            if ui.checkbox(&mut is_enabled, ind).changed() {
                                if is_enabled {
                                    if let Some(blueprint) = all_blueprints.iter().find(|b| b.name == ind) {
                                        app.config.phase1.permitted_indicators.push(blueprint.clone());
                                    }
                                } else {
                                    app.config.phase1.permitted_indicators.retain(|b| b.name != ind);
                                }
                            }
                        }
                        ui.add_space(12.0);
                    }
                    
                    if !app.forge_blueprints.is_empty() {
                        ui.label(egui::RichText::new("My Custom Indicators").strong().color(egui::Color32::LIGHT_BLUE));
                        ui.add_space(4.0);
                        for blueprint in &app.forge_blueprints {
                            let ind = &blueprint.name;
                            let mut is_enabled = app.config.phase1.permitted_indicators.iter().any(|b| &b.name == ind);
                            if ui.checkbox(&mut is_enabled, ind).changed() {
                                if is_enabled {
                                    app.config.phase1.permitted_indicators.push(blueprint.clone());
                                } else {
                                    app.config.phase1.permitted_indicators.retain(|b| &b.name != ind);
                                }
                            }
                        }
                        ui.add_space(12.0);
                    }
                });
            });
        app.show_indicator_modal = is_open;
    }

    if app.show_metric_filters_modal {
        let mut is_open = app.show_metric_filters_modal;
        egui::Window::new("Metric Filters")
            .open(&mut is_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("Filter Leaderboard Results").strong());
                ui.add_space(8.0);
                
                egui::Grid::new("metric_filters_grid")
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Net PnL:");
                        ui.horizontal(|ui| {
                            ui.label("Min:");
                            ui.add(egui::DragValue::new(&mut app.min_pnl_filter).speed(100.0));
                            ui.label("Max:");
                            ui.add(egui::DragValue::new(&mut app.max_pnl_filter).speed(100.0));
                        });
                        ui.end_row();
                        
                        ui.label("Win Rate (%):");
                        ui.horizontal(|ui| {
                            ui.label("Min:");
                            ui.add(egui::DragValue::new(&mut app.min_win_rate_filter).speed(1.0).clamp_range(0.0..=100.0));
                            ui.label("Max:");
                            ui.add(egui::DragValue::new(&mut app.max_win_rate_filter).speed(1.0).clamp_range(0.0..=100.0));
                        });
                        ui.end_row();

                        ui.label("R-Squared (Corr Coef):");
                        ui.horizontal(|ui| {
                            ui.label("Min:");
                            ui.add(egui::DragValue::new(&mut app.min_corr_coef_filter).speed(0.01).clamp_range(-1.0..=1.0));
                        });
                        ui.end_row();

                        ui.label("Max Consecutive Losses:");
                        ui.horizontal(|ui| {
                            ui.label("Max:");
                            ui.add(egui::DragValue::new(&mut app.max_cons_loss_filter).speed(1));
                        });
                        ui.end_row();
                    });
            });
        app.show_metric_filters_modal = is_open;
    }
}
