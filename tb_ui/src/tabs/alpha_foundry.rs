use eframe::egui;
use egui_plot::{Plot, Points, PlotPoints};
use crate::state::TradingApp;

pub fn render(app: &mut TradingApp, ctx: &egui::Context) {
    // 1. Non-blocking receiver check for Phase 1
    if let Some(rx) = &app.foundry_rx {
        while let Ok(metrics) = rx.try_recv() {
            app.latest_metrics = Some(metrics);
        }
        ctx.request_repaint();
    }
    
    // 2. Non-blocking receiver check for Phase 2 Live Backtest
    if let Some(rx) = &app.wfo_rx {
        while let Ok(res) = rx.try_recv() {
            if let Ok((elite, tearsheet)) = res {
                app.wfo_results.push((elite, tearsheet));
                app.wfo_results.sort_by(|a, b| b.1.net_pnl.partial_cmp(&a.1.net_pnl).unwrap_or(std::cmp::Ordering::Equal));
            }
        }
    }

    egui::SidePanel::left("alpha_foundry_controls").default_width(300.0).show(ctx, |ui| {
        ui.heading("Alpha Generation Settings");
        ui.add_space(16.0);
        
        egui::CollapsingHeader::new("Engine Architecture")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Select Evaluation Model:");
                ui.radio_value(&mut app.config.phase1.architecture_mode, tb_core::config::ArchitectureMode::ContinuousAst, "Continuous AST (Current)");
                ui.radio_value(&mut app.config.phase1.architecture_mode, tb_core::config::ArchitectureMode::DynamicLazyCache, "Dynamic Lazy Cache (Hybrid)");
                ui.radio_value(&mut app.config.phase1.architecture_mode, tb_core::config::ArchitectureMode::DiscretePrecomputed, "Discrete Pre-computed (BuildAlpha)");
            });

        ui.add_space(8.0);
        
        egui::CollapsingHeader::new("Risk Appetite")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Select Risk Profile:");
                let _ = ui.radio(true, "Conservative");
                let _ = ui.radio(false, "Aggressive");
            });

        ui.add_space(8.0);
        
        egui::CollapsingHeader::new("Hard Constraints (Death Penalties)")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut app.config.phase1.min_trades, 5..=100).text("Min Trades"));
                ui.add(egui::Slider::new(&mut app.config.phase1.max_exposure, 0.1..=1.0).text("Max Exposure (%)").custom_formatter(|n, _| format!("{:.0}%", n * 100.0)));
                ui.add(egui::Slider::new(&mut app.config.phase1.slippage_penalty, 0.0..=50.0).text("Slippage Penalty"));
                ui.add(egui::Slider::new(&mut app.config.phase1.in_sample_pct, 0.1..=0.9).text("In-Sample Data Split (%)").custom_formatter(|n, _| format!("{:.0}%", n * 100.0)));
                ui.add(egui::Slider::new(&mut app.config.phase1.long_strategy_pct, 0.0..=1.0).text("Long Bias (0=Shorts, 1=Longs)").custom_formatter(|n, _| format!("{:.0}% Longs", n * 100.0)));
            });

        ui.add_space(16.0);
        
        if ui.button("🚀 Start Evolution").clicked() {
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
                
                // Set up the WFO Receiver
                let (wfo_tx, wfo_rx) = crossbeam_channel::unbounded();
                app.wfo_rx = Some(wfo_rx);
                app.wfo_running = true;
                
                // Spawn Phase 2 Listener (Uses Out-Of-Sample Data)
                let simulator = tb_simulator::engine::ExecutionSimulator::new(phase2_config, elite_rx, oos_data);
                simulator.start_live_backtesting_listener(wfo_tx);
                
                // Spawn Phase 1 Evolutionary Engine (Uses In-Sample Data)
                std::thread::spawn(move || {
                    let foundry = tb_foundry::AlphaFoundry::new(phase1_config, elite_tx, Some(ui_tx));
                    if let Err(e) = foundry.run_generations(50, 100, is_data) {
                        eprintln!("Foundry error: {:?}", e);
                    }
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
                        let mut csv_content = String::from("AST Structure,Edge (Hit Rate),CPCV Variance (Risk),Exposure (%),Complexity\n");
                        for strategy in &metrics.strategies {
                            let ast = strategy.sketch.to_string().replace("\"", "\"\"");
                            csv_content.push_str(&format!("\"{}\",{:.4},{:.4},{:.1},{}\n",
                                ast,
                                strategy.fitness,
                                strategy.risk,
                                strategy.trade_frequency,
                                strategy.indicator_count
                            ));
                        }
                        if let Err(e) = std::fs::write(&path, csv_content) {
                            eprintln!("Failed to write CSV: {}", e);
                        }
                    }
                }
            });
            ui.add_space(16.0);
            
            // Split the central panel: Top half for Leaderboard, Bottom half for Live Backtest
            egui::TopBottomPanel::bottom("live_backtest_panel")
                .resizable(true)
                .min_height(300.0)
                .show_inside(ui, |ui| {
                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        ui.heading("Phase 2 - Live Out-Of-Sample Backtests");
                        
                        if !app.wfo_results.is_empty() {
                            if ui.button("💾 Export WFO to CSV").clicked() {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("CSV", &["csv"])
                                    .set_file_name("phase2_wfo_results.csv")
                                    .save_file() 
                                {
                                    let mut csv_content = String::from("AST Structure (Logic),Net PnL ($),Win Rate (%),Max Drawdown (%),Total Trades,Sharpe Ratio\n");
                                    for (elite, tearsheet) in &app.wfo_results {
                                        let ast = elite.sketch.to_string().replace("\"", "\"\"");
                                        csv_content.push_str(&format!("\"{}\",{:.2},{:.2},{:.2},{},{:.2}\n",
                                            ast,
                                            tearsheet.net_pnl,
                                            tearsheet.win_rate * 100.0,
                                            tearsheet.max_drawdown * 100.0,
                                            tearsheet.total_trades,
                                            tearsheet.sharpe_ratio
                                        ));
                                    }
                                    if let Err(e) = std::fs::write(&path, csv_content) {
                                        eprintln!("Failed to write Phase 2 CSV: {}", e);
                                    }
                                }
                            }
                        }
                    });
                    
                    ui.add_space(8.0);
                    
                    if !app.wfo_results.is_empty() {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::Grid::new("backtest_grid")
                                .striped(true)
                                .spacing([20.0, 8.0])
                                .show(ui, |ui| {
                                    // Header
                                    ui.label(egui::RichText::new("AST Structure (Logic)").strong());
                                    ui.label(egui::RichText::new("Net PnL ($)").strong().color(egui::Color32::GREEN));
                                    ui.label(egui::RichText::new("Win Rate (%)").strong());
                                    ui.label(egui::RichText::new("Max DD (%)").strong().color(egui::Color32::LIGHT_RED));
                                    ui.label(egui::RichText::new("Trades").strong());
                                    ui.label(egui::RichText::new("Sharpe").strong());
                                    ui.end_row();

                                    // Rows
                                    for (elite, tearsheet) in &app.wfo_results {
                                        ui.label(elite.sketch.to_string());
                                        
                                        ui.colored_label(
                                            if tearsheet.net_pnl >= 0.0 { egui::Color32::GREEN } else { egui::Color32::RED },
                                            format!("{:.2}", tearsheet.net_pnl)
                                        );
                                        
                                        ui.label(format!("{:.1}%", tearsheet.win_rate * 100.0));
                                        ui.label(format!("{:.1}%", tearsheet.max_drawdown * 100.0));
                                        ui.label(format!("{}", tearsheet.total_trades));
                                        ui.label(format!("{:.2}", tearsheet.sharpe_ratio));
                                        
                                        ui.end_row();
                                    }
                                });
                        });
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label(egui::RichText::new("Waiting for first Elite Strategy to compile and backtest...").italics());
                        });
                    }
                });
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("leaderboard_grid")
                    .striped(true)
                    .spacing([40.0, 8.0])
                    .show(ui, |ui| {
                        // Header
                        ui.label(egui::RichText::new("AST Structure (Logic)").strong());
                        ui.label(egui::RichText::new("Edge (Hit Rate)").strong().color(egui::Color32::GREEN));
                        ui.label(egui::RichText::new("CPCV Variance (Risk)").strong().color(egui::Color32::LIGHT_RED));
                        ui.label(egui::RichText::new("Exposure (%)").strong());
                        ui.label(egui::RichText::new("Complexity").strong());
                        ui.end_row();

                        // Rows
                        for strategy in &metrics.strategies {
                            // 1. AST Structure
                            ui.label(strategy.sketch.to_string());
                            
                            // 2. Edge (Fitness)
                            ui.colored_label(
                                if strategy.fitness >= 0.0 { egui::Color32::GREEN } else { egui::Color32::RED },
                                format!("{:.2}", strategy.fitness)
                            );
                            
                            // 3. CPCV Variance (Risk)
                            ui.label(format!("{:.4}", strategy.risk));
                            
                            // 4. Exposure
                            ui.label(format!("{:.1}%", strategy.trade_frequency));
                            
                            // 5. Complexity
                            ui.label(format!("{}", strategy.indicator_count));
                            
                            ui.end_row();
                        }
                    });
            });
        } else {
            ui.label("Evolution metrics and generated strategies will appear here.");
            if app.loaded_data.is_none() {
                ui.label(egui::RichText::new("⚠️ Please load a dataset in the Data Sandbox first.").color(egui::Color32::RED));
            }
        }
    });
}
