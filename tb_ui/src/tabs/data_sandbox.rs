use eframe::egui;
use crate::state::TradingApp;
use crate::data_parser::parse_dataframe_to_bars;
use polars::prelude::*;
use std::io::BufRead;

pub fn render(app: &mut TradingApp, ctx: &egui::Context) {
    // Data Sandbox Specific SidePanel
    egui::SidePanel::left("data_sandbox_controls").default_width(300.0).show(ctx, |ui| {
        ui.heading("Data Configuration");
        ui.add_space(16.0);
        
        if ui.button("📁 Load Parquet Data").clicked() {
            if let Some(path) = rfd::FileDialog::new().add_filter("Parquet", &["parquet"]).pick_file() {
                if let Ok(lf) = tb_data::ingestion::load_parquet(&path) {
                    if let Ok(df) = lf.clone().collect() {
                        match parse_dataframe_to_bars(&df) {
                            Ok(bar_data) => {
                                let symbol = path.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                                app.main_chart = egui_charts::ChartBuilder::new()
                                    .with_symbol(&symbol)
                                    .with_timeframe(egui_charts::model::Timeframe::Hour1)
                                    .with_theme(egui_charts::theme::Theme::dark())
                                    .build();
                                app.main_chart.chart.update_data(bar_data);
                            },
                            Err(e) => println!("ERROR Parsing DataFrame to Bars: {}", e),
                        }
                    }
                    app.loaded_data = Some(lf);
                    println!("Data loaded successfully from {:?}", path);
                }
            }
        }
        ui.add_space(8.0);
        if ui.button("📄 Load CSV Data").clicked() {
            if let Some(path) = rfd::FileDialog::new().add_filter("CSV", &["csv", "tsv", "txt"]).pick_file() {
                let mut separator = b',';
                if let Ok(file) = std::fs::File::open(&path) {
                    let mut reader = std::io::BufReader::new(file);
                    let mut first_line = String::new();
                    if reader.read_line(&mut first_line).is_ok() {
                        if first_line.contains('\t') {
                            separator = b'\t';
                        }
                    }
                }

                let mut opts = CsvReadOptions::default()
                    .with_has_header(true)
                    .with_parse_options(polars::prelude::CsvParseOptions::default().with_separator(separator).with_try_parse_dates(true));
                
                if let Ok(mut df) = opts.try_into_reader_with_file_path(Some(path.clone().into())).and_then(|r| r.finish()) {
                    let cols = df.get_column_names();
                    if cols.contains(&"<DATE>") && cols.contains(&"<TIME>") {
                        let lf = df.clone().lazy().with_column(
                            (col("<DATE>").cast(DataType::String) + lit(" ") + col("<TIME>").cast(DataType::String)).alias("timestamp")
                        );
                        if let Ok(new_df) = lf.collect() {
                            df = new_df;
                        }
                    }

                    app.available_columns = df.get_column_names().into_iter().map(|s| s.to_string()).collect();
                    
                    if app.available_columns.contains(&"timestamp".to_string()) { app.column_mapping.time = "timestamp".into(); }
                    if app.available_columns.contains(&"<OPEN>".to_string()) { app.column_mapping.open = "<OPEN>".into(); }
                    if app.available_columns.contains(&"<HIGH>".to_string()) { app.column_mapping.high = "<HIGH>".into(); }
                    if app.available_columns.contains(&"<LOW>".to_string()) { app.column_mapping.low = "<LOW>".into(); }
                    if app.available_columns.contains(&"<CLOSE>".to_string()) { app.column_mapping.close = "<CLOSE>".into(); }
                    if app.available_columns.contains(&"<TICKVOL>".to_string()) { app.column_mapping.volume = "<TICKVOL>".into(); }
                    if app.available_columns.contains(&"<VOL>".to_string()) { app.column_mapping.volume = "<VOL>".into(); }
                    if app.available_columns.contains(&"time".to_string()) { app.column_mapping.time = "time".into(); }
                    if app.available_columns.contains(&"date".to_string()) { app.column_mapping.time = "date".into(); }
                    if app.available_columns.contains(&"Timestamp".to_string()) { app.column_mapping.time = "Timestamp".into(); }
                    
                    app.raw_df_cache = Some((df, path));
                    app.show_mapping_modal = true;
                }
            }
        }
        if let Some(_lf) = &app.loaded_data {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("✅ Dataset Active").color(egui::Color32::GREEN));
        }

        ui.add_space(24.0);
        ui.separator();
        ui.add_space(24.0);
        
        ui.heading("CPCV Regimes");
        ui.label(egui::RichText::new("Drag on chart to define out-of-sample regimes.").color(egui::Color32::from_gray(180)));
        ui.add_space(8.0);
        
        if app.selected_regimes.is_empty() {
            ui.label("No regimes selected.");
        } else {
            for (i, r) in app.selected_regimes.iter().enumerate() {
                ui.label(format!("Regime {}: Bars {} -> {}", i + 1, r.0, r.1));
            }
            ui.add_space(8.0);
            if ui.button("Clear Regimes").clicked() {
                app.selected_regimes.clear();
            }
        }
    });

    let central_frame = egui::Frame::default().fill(ctx.style().visuals.panel_fill).inner_margin(0.0);
    egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
        app.main_chart.show(ui); 
        
        if let Some(hover_idx_f64) = app.main_chart.chart.get_hover_bar_idx() {
            let idx = hover_idx_f64.round() as usize;
            
            if ctx.input(|i| i.pointer.primary_pressed()) {
                app.is_dragging_regime = true;
                app.current_regime_start = Some(idx);
            }
            if ctx.input(|i| i.pointer.primary_released()) {
                if app.is_dragging_regime {
                    if let Some(start) = app.current_regime_start {
                        let end = idx;
                        let min = start.min(end);
                        let max = start.max(end);
                        if max.saturating_sub(min) > 2 {
                            app.selected_regimes.push((min, max));
                        }
                    }
                }
                app.is_dragging_regime = false;
                app.current_regime_start = None;
            }
        }
    });
}
