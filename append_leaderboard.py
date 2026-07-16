import sys

with open("tb_ui/src/tabs/alpha_foundry.rs", "a", encoding="utf-8") as f:
    f.write("""
pub fn render_leaderboard(app: &mut crate::state::TradingApp, ui: &mut eframe::egui::Ui) {
    ui.heading("Alpha Foundry - Leaderboard (Phase 1)");
    
    if let Some(metrics) = &app.latest_metrics {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label(eframe::egui::RichText::new(format!("Generation: {}/50", metrics.generation)).strong().color(eframe::egui::Color32::GREEN));
            
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
                ui.label(eframe::egui::RichText::new("Precomputing Indicators...").size(16.0));
            });
        } else {
            eframe::egui::ScrollArea::both().show(ui, |ui| {
            eframe::egui::Grid::new("leaderboard_grid")
                .striped(true)
                .spacing([40.0, 8.0])
                .show(ui, |ui| {
                    // Header
                    ui.label(eframe::egui::RichText::new("AST Structure (Logic)").strong());
                    ui.label(eframe::egui::RichText::new("Total PnL").strong().color(eframe::egui::Color32::LIGHT_GREEN));
                    ui.label(eframe::egui::RichText::new("Trades").strong());
                    ui.label(eframe::egui::RichText::new("Avg Trade").strong()).on_hover_text("Expected $ per trade");
                    ui.label(eframe::egui::RichText::new("Win %").strong());
                    ui.label(eframe::egui::RichText::new("Exposure (%)").strong());
                    
                    ui.label(eframe::egui::RichText::new("Ratio W/L").strong());
                    ui.label(eframe::egui::RichText::new("Profit Factor").strong());
                    
                    ui.label(eframe::egui::RichText::new("Max DD").strong()).on_hover_text("Max Drawdown");
                    ui.label(eframe::egui::RichText::new("Max Cons. Loss").strong());
                    ui.label(eframe::egui::RichText::new("Largest Win/Loss").strong());
                    ui.label(eframe::egui::RichText::new("PnL/DD").strong());
                    
                    ui.label(eframe::egui::RichText::new("Sharpe").strong());
                    ui.label(eframe::egui::RichText::new("Sortino").strong());
                    ui.label(eframe::egui::RichText::new("Corr Coef").strong()).on_hover_text("Equity curve straightness (1.0 = perfect)");
                    ui.label(eframe::egui::RichText::new("Complexity").strong());
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
                            
                            if app.show_robustness_modal {
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
                            }
                        }
                        
                        // 2. Total PnL
                        ui.colored_label(
                            if strategy.pnl >= 0.0 { eframe::egui::Color32::LIGHT_GREEN } else { eframe::egui::Color32::LIGHT_RED },
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
                            if strategy.profit_factor >= 1.5 { eframe::egui::Color32::LIGHT_GREEN } else { ui.visuals().text_color() },
                            format!("{:.2}", strategy.profit_factor)
                        );
                        
                        // 8. Max DD
                        ui.colored_label(eframe::egui::Color32::LIGHT_RED, format!("{:.2}", strategy.max_drawdown));
                        
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
            ui.label(eframe::egui::RichText::new("⚠️ Please load a dataset in the Data Sandbox first.").color(eframe::egui::Color32::RED));
        }
    }
}
""")
