use eframe::egui;
use crate::state::TradingApp;
use crate::data_parser::parse_dataframe_to_bars;
use polars::prelude::*;

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
