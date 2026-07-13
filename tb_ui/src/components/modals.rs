use eframe::egui;
use crate::state::TradingApp;
use crate::data_parser::parse_dataframe_to_bars;
use polars::prelude::*;
use egui_plot::{Plot, Line, PlotPoints};

pub fn render_mapping_modal(app: &mut TradingApp, ctx: &egui::Context) {
    if !app.show_mapping_modal {
        return;
    }

    egui::Window::new("Map CSV Columns")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label("Please map your CSV columns to the required schema.");
            ui.add_space(8.0);
            
            let mut render_combo = |label: &str, current: &mut String| {
                ui.horizontal(|ui| {
                    ui.label(label);
                    egui::ComboBox::from_id_source(label)
                        .selected_text(current.clone())
                        .show_ui(ui, |ui| {
                            for col in &app.available_columns {
                                ui.selectable_value(current, col.clone(), col);
                            }
                        });
                });
            };

            render_combo("Timestamp", &mut app.column_mapping.time);
            render_combo("Open", &mut app.column_mapping.open);
            render_combo("High", &mut app.column_mapping.high);
            render_combo("Low", &mut app.column_mapping.low);
            render_combo("Close", &mut app.column_mapping.close);
            render_combo("Volume", &mut app.column_mapping.volume);

            ui.add_space(16.0);
            ui.horizontal(|ui| {
                if ui.button("Apply & Load").clicked() {
                    if let Some((mut df, path)) = app.raw_df_cache.take() {
                        let _ = df.rename(app.column_mapping.time.as_str(), "timestamp".into());
                        let _ = df.rename(app.column_mapping.open.as_str(), "open".into());
                        let _ = df.rename(app.column_mapping.high.as_str(), "high".into());
                        let _ = df.rename(app.column_mapping.low.as_str(), "low".into());
                        let _ = df.rename(app.column_mapping.close.as_str(), "close".into());
                        let _ = df.rename(app.column_mapping.volume.as_str(), "volume".into());

                        let parquet_path = path.with_extension("parquet");
                        if let Ok(file) = std::fs::File::create(&parquet_path) {
                            if ParquetWriter::new(file).finish(&mut df).is_ok() {
                                if let Ok(lf) = tb_data::ingestion::load_parquet(&parquet_path) {
                                    if let Ok(df) = lf.clone().collect() {
                                        match parse_dataframe_to_bars(&df) {
                                            Ok(bar_data) => {
                                                let symbol = parquet_path.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                                                app.main_chart = egui_charts::ChartBuilder::new()
                                                    .with_symbol(&symbol)
                                                    .with_timeframe(egui_charts::model::Timeframe::Hour1)
                                                    .with_theme(egui_charts::theme::Theme::dark())
                                                    .build();
                                                app.main_chart.chart.update_data(bar_data);
                                                println!("Chart data injected successfully!");
                                            },
                                            Err(e) => println!("ERROR Parsing DataFrame to Bars: {}", e),
                                        }
                                    }
                                    app.loaded_data = Some(lf);
                                    println!("CSV mapped, converted, and loaded!");
                                }
                            }
                        }
                    }
                    app.show_mapping_modal = false;
                }
                if ui.button("Cancel").clicked() {
                    app.show_mapping_modal = false;
                    app.raw_df_cache = None;
                }
            });
        });
}

pub fn render_robustness_modal(app: &mut TradingApp, ctx: &egui::Context) {
    if !app.show_robustness_modal {
        return;
    }

    let mut open = app.show_robustness_modal;
    let mut needs_regen = false;

    egui::Window::new("🔬 Interactive Robustness Playground")
        .open(&mut open)
        .default_width(900.0)
        .default_height(600.0)
        .show(ctx, |ui| {
            if let Some(report) = &app.robustness_report {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(250.0);
                        ui.heading("Robustness Tests");
                        ui.separator();

                        ui.label(egui::RichText::new("A. Feature Ablation").strong());
                        ui.label("Toggle logic nodes to see curve impact:");
                        if let (Some(engine), Some(idx)) = (&app.bitwise_engine, app.selected_strategy_idx) {
                            if let Some(metrics) = &app.latest_metrics {
                                if idx < metrics.strategies.len() {
                                    let strategy = &metrics.strategies[idx];
                                    for &c_idx in &strategy.condition_indexes {
                                        let name = &engine.grid.conditions[c_idx].name;
                                        let mut is_enabled = !app.robustness_disabled_conditions.contains(&c_idx);
                                        if ui.checkbox(&mut is_enabled, name).changed() {
                                            if is_enabled {
                                                app.robustness_disabled_conditions.remove(&c_idx);
                                            } else {
                                                app.robustness_disabled_conditions.insert(c_idx);
                                            }
                                            needs_regen = true;
                                        }
                                    }
                                }
                            }
                        }
                        ui.add_space(10.0);

                        ui.label(egui::RichText::new("B. IS/OOS Decay").strong());
                        ui.label("IS/OOS boundary is marked by a vertical dashed line.");
                        ui.add_space(10.0);

                        ui.label(egui::RichText::new("C. Top Trade Deletion").strong());
                        ui.checkbox(&mut app.robustness_show_deletion, "Show Deletion Curve");
                        ui.horizontal(|ui| {
                            ui.label("Drop Top N Trades:");
                            if ui.add(egui::Slider::new(&mut app.robustness_top_n_drop, 0..=10)).changed() {
                                needs_regen = true;
                            }
                        });
                        ui.add_space(10.0);

                        ui.label(egui::RichText::new("D. Slippage Test").strong());
                        ui.checkbox(&mut app.robustness_show_noise, "Show Worst-Case Slippage Line");
                        ui.horizontal(|ui| {
                            ui.label("Fixed Penalty per Trade:");
                            if ui.add(egui::Slider::new(&mut app.robustness_noise_pct, 0.0..=5.0).text("pts")).changed() {
                                needs_regen = true;
                            }
                        });
                        ui.add_space(10.0);

                        ui.label(egui::RichText::new("E. Monte Carlo").strong());
                        ui.checkbox(&mut app.robustness_show_mc, "Show Spaghetti Cloud");
                        ui.label("Generated via Block Bootstrapping");
                    });
                    
                    ui.separator();
                    
                    Plot::new("robustness_equity_curve")
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            // B. IS/OOS Boundary
                            let is_len = (report.baseline_curve.len() as f64 * (app.config.phase1.in_sample_pct / 100.0)) as f64;
                            plot_ui.vline(egui_plot::VLine::new("IS/OOS Boundary", is_len)
                                .color(egui::Color32::from_gray(120))
                                .style(egui_plot::LineStyle::Dashed { length: 5.0 }));
                            if app.robustness_show_noise {
                                let step = (report.slippage_curve.len() / 2000).max(1);
                                let slippage_pts: PlotPoints = report.slippage_curve
                                    .iter()
                                    .enumerate()
                                    .step_by(step)
                                    .map(|(i, &pnl)| [i as f64, pnl])
                                    .collect();
                                plot_ui.line(Line::new("Slippage", slippage_pts).color(egui::Color32::from_rgb(255, 140, 0)).width(2.0));
                            }
                            
                            if app.robustness_show_mc {
                                for (i, mc_curve) in report.monte_carlo_curves.iter().enumerate() {
                                    let step = (mc_curve.len() / 500).max(1);
                                    let mc_pts: PlotPoints = mc_curve
                                        .iter()
                                        .enumerate()
                                        .step_by(step)
                                        .map(|(i, &pnl)| [i as f64, pnl])
                                        .collect();
                                    plot_ui.line(Line::new(format!("MC_{}", i), mc_pts).color(egui::Color32::from_white_alpha(15)).width(1.0));
                                }
                            }

                            if app.robustness_show_deletion {
                                let step = (report.deletion_curve.len() / 2000).max(1);
                                let del_pts: PlotPoints = report.deletion_curve
                                    .iter()
                                    .enumerate()
                                    .step_by(step)
                                    .map(|(i, &pnl)| [i as f64, pnl])
                                    .collect();
                                plot_ui.line(Line::new("Trade Deletion", del_pts).color(egui::Color32::from_rgb(255, 100, 100)).style(egui_plot::LineStyle::Dashed { length: 5.0 }).width(2.0));
                            }

                            let step = (report.baseline_curve.len() / 2000).max(1);
                            let baseline_pts: PlotPoints = report.baseline_curve
                                .iter()
                                .enumerate()
                                .step_by(step)
                                .map(|(i, &pnl)| [i as f64, pnl])
                                .collect();
                            
                            plot_ui.line(Line::new("Baseline", baseline_pts).color(egui::Color32::from_rgb(100, 150, 250)).width(3.0));
                        });
                });
            } else {
                ui.label("Generating report...");
            }
        });

    if needs_regen {
        if let (Some(engine), Some(idx)) = (&app.bitwise_engine, app.selected_strategy_idx) {
            if let Some(metrics) = &app.latest_metrics {
                if idx < metrics.strategies.len() {
                    let strategy = &metrics.strategies[idx];
                    let active_conditions: Vec<usize> = strategy.condition_indexes.iter().copied()
                        .filter(|&c| !app.robustness_disabled_conditions.contains(&c))
                        .collect();
                        
                    let new_report = tb_bitwise::robustness::generate_report(
                        engine,
                        &active_conditions,
                        &app.config.phase1.trade_direction,
                        app.robustness_noise_pct / 100.0,
                        app.robustness_top_n_drop,
                    );
                    app.robustness_report = Some(new_report);
                }
            }
        }
    }

    if !open {
        app.show_robustness_modal = false;
        app.robustness_report = None;
    }
}
