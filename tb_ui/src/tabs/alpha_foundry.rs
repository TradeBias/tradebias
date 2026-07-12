use eframe::egui;
use egui_plot::{Plot, Points, PlotPoints};
use crate::state::TradingApp;

pub fn render(app: &mut TradingApp, ctx: &egui::Context) {
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
    
    // Phase 2 listener removed from Alpha Foundry tab

    egui::SidePanel::left("alpha_foundry_controls").default_width(300.0).show(ctx, |ui| {
        ui.heading("Alpha Generation Settings");
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.label("Starting Equity: $");
            ui.add(egui::DragValue::new(&mut app.config.phase1.starting_equity)
                .speed(1000.0)
                .range(100.0..=100_000_000.0)
            );
        });
        ui.add_space(16.0);
        
        egui::CollapsingHeader::new("1. Target / Exit Definitions")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Exit Logic (Pre-computed):");
                
                let mut current_stop = app.config.phase1.stop_type.clone();
                let available_types = tb_core::stops::StopType::available_types();
                
                // Identify the current type index based on variant name
                let mut stop_idx = match current_stop {
                    tb_core::stops::StopType::FixedBarHold { .. } => 0,
                    tb_core::stops::StopType::StandardStop { .. } => 1,
                    tb_core::stops::StopType::TrailingStop { .. } => 2,
                };
                
                egui::ComboBox::from_id_source("stop_type_dropdown")
                    .selected_text(available_types[stop_idx].0)
                    .show_ui(ui, |ui| {
                        for (i, (name, _)) in available_types.iter().enumerate() {
                            ui.selectable_value(&mut stop_idx, i, *name);
                        }
                    });
                    
                match stop_idx {
                    0 => {
                        let mut bars = if let tb_core::stops::StopType::FixedBarHold { bars } = current_stop { bars } else { 5 };
                        ui.add(egui::Slider::new(&mut bars, 1..=20).text("Bars"));
                        app.config.phase1.stop_type = tb_core::stops::StopType::FixedBarHold { bars };
                    }
                    1 | 2 => {
                        let mut calc = match current_stop {
                            tb_core::stops::StopType::StandardStop { calc } => calc,
                            tb_core::stops::StopType::TrailingStop { calc } => calc,
                            _ => tb_core::stops::StopCalculation::Atr { multiplier: 2.0 },
                        };
                        
                        let available_calcs = tb_core::stops::StopCalculation::available();
                        
                        ui.horizontal(|ui| {
                            ui.label("Calculation:");
                            
                            // Find current index
                            let mut calc_idx = available_calcs.iter().position(|(c, _, _)| std::mem::discriminant(c) == std::mem::discriminant(&calc)).unwrap_or(0);
                            
                            egui::ComboBox::from_id_source("calc_type_dropdown")
                                .selected_text(available_calcs[calc_idx].1)
                                .show_ui(ui, |ui| {
                                    for (i, (_, name, _)) in available_calcs.iter().enumerate() {
                                        ui.selectable_value(&mut calc_idx, i, *name);
                                    }
                                });
                                
                            // Ensure calc is set to the selected type
                            if std::mem::discriminant(&calc) != std::mem::discriminant(&available_calcs[calc_idx].0) {
                                calc = available_calcs[calc_idx].0.clone();
                            }
                            
                            let spec = &available_calcs[calc_idx].2;
                            let mut val = calc.get_val();
                            ui.add(egui::Slider::new(&mut val, spec.min..=spec.max).text(spec.label));
                            calc.update_val(val);
                        });
                        
                        if stop_idx == 1 {
                            app.config.phase1.stop_type = tb_core::stops::StopType::StandardStop { calc };
                        } else {
                            app.config.phase1.stop_type = tb_core::stops::StopType::TrailingStop { calc };
                        }
                    }
                    _ => {}
                }
                
                ui.add_space(8.0);
                
                if stop_idx != 0 { // If not Fixed Bar Hold, show Take Profit
                    ui.label("Take Profit:");
                    
                    let mut current_tp = app.config.phase1.take_profit.clone();
                    
                    // Force a Take Profit if using a Static Stop
                    if stop_idx == 1 && matches!(current_tp, tb_core::stops::TakeProfit::None) {
                        current_tp = tb_core::stops::TakeProfit::RiskReward { multiplier: 2.0 };
                    }
                    
                    let available_tps = tb_core::stops::TakeProfit::available();
                    
                    // Find current index
                    let mut tp_idx = available_tps.iter().position(|(t, _, _)| std::mem::discriminant(t) == std::mem::discriminant(&current_tp)).unwrap_or(0);
                    
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_source("tp_type_dropdown")
                            .selected_text(available_tps[tp_idx].1)
                            .show_ui(ui, |ui| {
                                for (i, (tp, name, _)) in available_tps.iter().enumerate() {
                                    if stop_idx == 1 && matches!(tp, tb_core::stops::TakeProfit::None) {
                                        continue; // Don't allow None for Static Stop
                                    }
                                    ui.selectable_value(&mut tp_idx, i, *name);
                                }
                            });
                            
                        // Ensure TP is set to the selected type
                        if std::mem::discriminant(&current_tp) != std::mem::discriminant(&available_tps[tp_idx].0) {
                            current_tp = available_tps[tp_idx].0.clone();
                        }
                        
                        if let Some(spec) = &available_tps[tp_idx].2 {
                            let mut val = current_tp.get_val();
                            ui.add(egui::Slider::new(&mut val, spec.min..=spec.max).text(spec.label));
                            current_tp.update_val(val);
                        }
                    });
                    
                    app.config.phase1.take_profit = current_tp;
                }
                
                ui.add_space(4.0);
                ui.label("Trade Direction:");
                ui.radio_value(&mut app.config.phase1.trade_direction, tb_core::ast::TradeDirection::Long, "Long Only");
                ui.radio_value(&mut app.config.phase1.trade_direction, tb_core::ast::TradeDirection::Short, "Short Only");
            });

        ui.add_space(8.0);
        
        egui::CollapsingHeader::new("2. MAP-Elites Diversity Grid")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("X-Axis (Diversity Trait 1):");
                    
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
                ui.horizontal(|ui| {
                    ui.label("Y-Axis (Diversity Trait 2):");
                    
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
                
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Grid Resolution (Size):");
                    egui::ComboBox::from_id_source("grid_size_combo")
                        .selected_text(format!("{}x{}", app.config.phase1.grid_size, app.config.phase1.grid_size))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.config.phase1.grid_size, 10, "10x10 (Low)");
                            ui.selectable_value(&mut app.config.phase1.grid_size, 25, "25x25 (Medium)");
                            ui.selectable_value(&mut app.config.phase1.grid_size, 50, "50x50 (High)");
                        });
                });
            });

        ui.add_space(8.0);
        
        egui::CollapsingHeader::new("3. Fitness Maximizer")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Optimize Cells For:");
                for (func, label) in tb_core::fitness::FitnessFunction::available() {
                    ui.radio_value(&mut app.config.phase1.fitness, func, label);
                }
            });
            
        ui.add_space(8.0);
        
        egui::CollapsingHeader::new("4. Data Validation")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut app.config.phase1.in_sample_pct, 0.1..=0.9).text("In-Sample Split (%)").custom_formatter(|n, _| format!("{:.0}%", n * 100.0)));
            });

        ui.add_space(8.0);
        
        egui::CollapsingHeader::new("5. Indicator Universe")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("{} Indicators Selected", app.config.phase1.permitted_indicators.len()));
                if ui.button("⚙ Configure Indicator Universe").clicked() {
                    app.show_indicator_modal = true;
                }
            });

        ui.add_space(16.0);
        
        if ui.button("🚀 Start Evolution (Bitwise SIMD)").clicked() {
            if let Some(loaded_data) = &app.loaded_data {
                let (ui_tx, ui_rx) = crossbeam_channel::unbounded();
                app.foundry_rx = Some(ui_rx);
                
                let elite_tx = app.elite_tx.clone().expect("Elite tx must be initialized");
                let elite_rx = app.elite_rx.clone().expect("Elite rx must be initialized");
                
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
                    use tb_bitwise::precompute::{ConditionGrid, BaseArray, SemanticType};
                    use std::collections::HashSet;
                    use tb_bitwise::targets::TargetOutcomes;
                    use tb_bitwise::engine::BitwiseEngine;
                    use tb_bitwise::ga::GeneticAlgorithm;
                    use tb_bitwise::translator::translate_to_sketch;
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

                    // 2. Dynamically Generate Base Arrays based on config
                    let price_data = tb_bitwise::registry::PriceData {
                        open: &opens,
                        high: &highs,
                        low: &lows,
                        close: &closes,
                        volume: &volumes,
                    };
                    let base_arrays = tb_bitwise::registry::build_base_arrays(&phase1_config.permitted_indicators, &price_data);

                    // 3. Generate Grid and cull
                    let mut grid = ConditionGrid::new();
                    grid.generate(&base_arrays);
                    
                    let _ = ui_tx.send(crate::state::GenerationMetrics {
                        generation: 0, elapsed_seconds: 0.0, total_generated: 0, total_discarded: 0, strategies: vec![],
                        status_msg: Some("Filtering low-quality signals...".to_string())
                    });
                    grid.cull_sparsity(closes.len(), 0.01, 0.95);
                    
                    let _ = ui_tx.send(crate::state::GenerationMetrics {
                        generation: 0, elapsed_seconds: 0.0, total_generated: 0, total_discarded: 0, strategies: vec![],
                        status_msg: Some("Removing redundant signals...".to_string())
                    });
                    grid.cull_correlated(0.95);
                    grid.cull_sparsity(closes.len(), 0.01, 0.95);
                    grid.cull_correlated(0.95);
                    let arc_grid = std::sync::Arc::new(grid);

                    // 4. Generate Target Outcomes (Advanced Stops)
                    let raw_data = tb_bitwise::data::RawData::from_arrays(opens.clone(), highs.clone(), lows.clone(), closes.clone(), volumes.clone()).unwrap();
                    
                    let atr_data = match (&phase1_config.stop_type, &phase1_config.take_profit) {
                        (tb_core::stops::StopType::StandardStop { calc: tb_core::stops::StopCalculation::Atr { .. } }, _) |
                        (tb_core::stops::StopType::TrailingStop { calc: tb_core::stops::StopCalculation::Atr { .. } }, _) |
                        (_, tb_core::stops::TakeProfit::Atr { .. }) => {
                            Some(tb_math::indicators::atr(&highs, &lows, &closes, 14))
                        }
                        _ => None,
                    };
                    
                    let targets = TargetOutcomes::generate_advanced_stop(
                        &raw_data, 
                        &phase1_config.stop_type, 
                        &phase1_config.take_profit,
                        atr_data.as_deref()
                    );
                    let arc_targets = std::sync::Arc::new(targets);

                    // 5. Run the Genetic Algorithm
                    let engine = std::sync::Arc::new(BitwiseEngine::new(arc_grid.clone(), arc_targets));
                    let max_complexity = 5; // Hardcoded fallback max for now based on AST max
                    let archive = tb_bitwise::archive::MapArchive::new(phase1_config.map_x.clone(), phase1_config.map_y.clone(), phase1_config.fitness.clone(), phase1_config.grid_size, closes.len() as u32, max_complexity);
                    let mut ga = GeneticAlgorithm::new(engine, 5_000, 50, phase1_config.trade_direction.clone(), archive); // 50 Generations
                    
                    let _ = ui_tx.send(crate::state::GenerationMetrics {
                        generation: 0, elapsed_seconds: 0.0, total_generated: 0, total_discarded: 0, strategies: vec![],
                        status_msg: Some("Running Genetic Algorithm...".to_string())
                    });

                    let start_eval = std::time::Instant::now();
                    ga.run();
                    let elapsed = start_eval.elapsed();

                    // 6. Extract Kings and send to UI
                    let final_kings: Vec<_> = ga.archive.grid.iter().flat_map(|col| col.iter()).filter_map(|c| c.clone()).collect();
                    let mut elite_strategies = Vec::new();

                    for king in final_kings {
                        if let Some(sketch) = translate_to_sketch(&king, &arc_grid, phase1_config.trade_direction.clone()) {
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
                                expectancy: king.metrics.avg_trade,
                                trade_frequency: (king.metrics.total_trades as f64 / closes.len() as f64) * 100.0,
                                indicator_count: king.conditions.len() as u8,
                            };
                            elite_strategies.push(elite.clone());
                            let _ = elite_tx.send(elite); // Send to Simulator
                        }
                    }

                    // Send Metrics to UI
                    let _ = ui_tx.send(crate::state::GenerationMetrics {
                        generation: 50,
                        elapsed_seconds: elapsed.as_secs_f64(),
                        total_generated: 250_000, // 5k * 50
                        total_discarded: 0,
                        strategies: elite_strategies,
                        status_msg: None,
                    });
                });
            } else {
                println!("Cannot start evolution: No data loaded in Sandbox.");
            }
        }
    });

    if let Some(metrics) = &app.latest_metrics {
        egui::TopBottomPanel::bottom("foundry_stats_panel").show(ctx, |ui| {
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
        });
    }

    let central_frame = egui::Frame::default().fill(ctx.style().visuals.panel_fill).inner_margin(16.0);
    egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
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
            });
            ui.add_space(16.0);
            
            // Leaderboard Only
            if app.foundry_rx.is_some() {
                ui.add_space(80.0);
                ui.vertical_centered(|ui| {
                    ui.spinner();
                    ui.add_space(20.0);
                    let mut display_text = "Running Genetic Algorithm...".to_string();
                    if let Some(metrics) = &app.latest_metrics {
                        if let Some(msg) = &metrics.status_msg {
                            display_text = msg.clone();
                        }
                    }
                    ui.label(egui::RichText::new(display_text).size(16.0));
                });
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("leaderboard_grid")
                    .striped(true)
                    .spacing([40.0, 8.0])
                    .show(ui, |ui| {
                        // Header
                        ui.label(egui::RichText::new("AST Structure (Logic)").strong());
                        ui.label(egui::RichText::new("Total PnL").strong().color(egui::Color32::LIGHT_GREEN));
                        ui.label(egui::RichText::new("Win %").strong());
                        ui.label(egui::RichText::new("Drawdown").strong());
                        ui.label(egui::RichText::new("Sharpe").strong());
                        ui.label(egui::RichText::new("Sortino").strong());
                        ui.label(egui::RichText::new("Profit Factor").strong());
                        ui.label(egui::RichText::new("Exposure (%)").strong());
                        ui.label(egui::RichText::new("Complexity").strong());
                        ui.end_row();

                        // Rows
                        for strategy in &metrics.strategies {
                            // 1. AST Structure
                            ui.label(strategy.sketch.to_string());
                            
                            // 2. Total PnL
                            ui.colored_label(
                                if strategy.pnl >= 0.0 { egui::Color32::LIGHT_GREEN } else { egui::Color32::LIGHT_RED },
                                format!("{:.2}", strategy.pnl)
                            );
                            
                            // 3. Win %
                            ui.label(format!("{:.1}%", strategy.fitness)); // Using fitness field which is populated with win_rate

                            // 4. Drawdown
                            ui.colored_label(egui::Color32::LIGHT_RED, format!("{:.2}", strategy.max_drawdown));

                            // 5. Sharpe
                            ui.label(format!("{:.2}", strategy.sharpe));

                            // 6. Sortino
                            ui.label(format!("{:.2}", strategy.sortino));

                            // 7. Profit Factor
                            ui.label(format!("{:.2}", strategy.profit_factor));
                            
                            // 8. Exposure
                            ui.label(format!("{:.1}%", strategy.trade_frequency));
                            
                            // 5. Complexity
                            ui.label(format!("{}", strategy.indicator_count));
                            
                            ui.end_row();
                        }
                    });
                });
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
                        let all_indicators = tb_bitwise::registry::get_available_indicators();
                        app.config.phase1.permitted_indicators = all_indicators.iter().map(|s| s.name.to_string()).collect();
                    }
                    if ui.button("Disable All").clicked() {
                        app.config.phase1.permitted_indicators.clear();
                    }
                });
                ui.add_space(8.0);
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let all_indicators = tb_bitwise::registry::get_available_indicators();
                    
                    // Group by category
                    let mut categories: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
                    for spec in &all_indicators {
                        categories.entry(spec.category).or_default().push(spec.name);
                    }
                    
                    let mut sorted_cats: Vec<_> = categories.keys().cloned().collect();
                    sorted_cats.sort();
                    
                    for cat in sorted_cats {
                        egui::CollapsingHeader::new(cat)
                            .default_open(false)
                            .show(ui, |ui| {
                                if let Some(names) = categories.get(cat) {
                                    for &ind in names {
                                        let mut is_enabled = app.config.phase1.permitted_indicators.contains(&ind.to_string());
                                        if ui.checkbox(&mut is_enabled, ind).changed() {
                                            if is_enabled {
                                                app.config.phase1.permitted_indicators.push(ind.to_string());
                                            } else {
                                                app.config.phase1.permitted_indicators.retain(|x| x != ind);
                                            }
                                        }
                                    }
                                }
                            });
                    }
                });
            });
        app.show_indicator_modal = is_open;
    }
}
