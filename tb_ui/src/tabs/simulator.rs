use eframe::egui;
use crate::state::TradingApp;

pub fn render(app: &mut TradingApp, ctx: &egui::Context) {
    egui::SidePanel::right("simulator_config_panel")
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.heading("Execution Engine Config");
            ui.separator();
            
            ui.label("Walk-Forward Engine (Phase 2) Settings");
            ui.add_space(10.0);
            
            ui.collapsing("Risk Management Exits", |ui| {
                let mut tp_pct = app.config.phase2.take_profit_pct.unwrap_or(0.05) * 100.0;
                let mut sl_pct = app.config.phase2.stop_loss_pct.unwrap_or(0.02) * 100.0;
                let mut trail_pct = app.config.phase2.trailing_stop_pct.unwrap_or(0.02) * 100.0;
                
                ui.add(egui::Slider::new(&mut tp_pct, 0.0..=20.0).text("Take Profit (%)"));
                ui.add(egui::Slider::new(&mut sl_pct, 0.0..=10.0).text("Stop Loss (%)"));
                ui.add(egui::Slider::new(&mut trail_pct, 0.0..=10.0).text("Trailing Stop (%)"));
                
                app.config.phase2.take_profit_pct = if tp_pct > 0.0 { Some(tp_pct / 100.0) } else { None };
                app.config.phase2.stop_loss_pct = if sl_pct > 0.0 { Some(sl_pct / 100.0) } else { None };
                app.config.phase2.trailing_stop_pct = if trail_pct > 0.0 { Some(trail_pct / 100.0) } else { None };
            });
            
            if ui.button("Run Walk-Forward Backtest").clicked() && !app.wfo_running {
                if let Some(data) = &app.loaded_data {
                    let _data_clone = data.clone();
                    let _phase2_config = app.config.phase2.clone();
                    let _phase1_config = app.config.phase1.clone();
                    let _wfo_config = tb_simulator::wfo::WfoConfig {
                        anchored: false,
                        is_window_size: 1000,
                        oos_window_size: 200,
                    }; 
                    let _generations = 5;
                    let _population_size = 10;
                    
                    let (tx, rx) = crossbeam_channel::unbounded();
                    app.simulator_wfo_rx = Some(rx);
                    app.wfo_running = true;
                    app.latest_simulator_tearsheet = None; // Reset previous results

                    std::thread::spawn(move || {
                        // WFO relies on the deprecated tb_foundry. 
                        // To be rewritten with tb_bitwise.
                        let _ = tx.send(Err("Walk-Forward Optimization is currently disabled pending Bitwise integration.".to_string()));
                    });
                } else {
                    tracing::warn!("No data loaded! Please load data in the Sandbox first.");
                }
            }
        });

    // Check for results from background thread
    if let Some(rx) = &app.simulator_wfo_rx {
        if let Ok(result) = rx.try_recv() {
            app.wfo_running = false;
            match result {
                Ok(tear_sheet) => app.latest_simulator_tearsheet = Some(tear_sheet),
                Err(e) => tracing::error!("WFO Error: {}", e),
            }
        }
    }

    let central_frame = egui::Frame::default().fill(ctx.style().visuals.panel_fill).inner_margin(16.0);
    egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
        ui.heading("Walk-Forward Equity Curves (Phase 2)");
        ui.add_space(10.0);
        
        if app.wfo_running {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Running Walk-Forward Optimization (Phase 1 & 2)...");
            });
        } else if let Some(ts) = &app.latest_simulator_tearsheet {
            ui.group(|ui| {
                ui.heading("Tear Sheet");
                ui.separator();
                
                egui::Grid::new("tearsheet_grid").num_columns(2).spacing([40.0, 4.0]).show(ui, |ui| {
                    ui.label("Total Trades:");
                    ui.label(format!("{}", ts.total_trades));
                    ui.end_row();
                    
                    ui.label("Win Rate:");
                    ui.label(format!("{:.2}%", ts.win_rate * 100.0));
                    ui.end_row();
                    
                    ui.label("Profit Factor:");
                    ui.label(format!("{:.2}", ts.profit_factor));
                    ui.end_row();
                    
                    ui.label("Net Profit:");
                    let color = if ts.net_profit >= 0.0 { egui::Color32::GREEN } else { egui::Color32::RED };
                    ui.colored_label(color, format!("${:.2}", ts.net_profit));
                    ui.end_row();
                    
                });
            });
        } else {
            ui.label("Out-of-sample execution results will be visualized here.");
        }
    });
}
