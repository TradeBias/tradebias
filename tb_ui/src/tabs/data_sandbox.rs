use eframe::egui;
use crate::state::TradingApp;
use crate::data_parser::parse_dataframe_to_bars;
use polars::prelude::*;
use std::io::BufRead;

pub fn render(app: &mut TradingApp, ctx: &egui::Context, outer_ui: &mut egui::Ui) {
    // Data Sandbox Specific SidePanel
    egui::SidePanel::left("data_sandbox_controls").default_width(300.0).show_inside(outer_ui, |ui| {
        ui.heading("Data Configuration");
        ui.add_space(16.0);
        
        if ui.button("📁 Load Parquet Data").clicked() {
            if let Some(path) = rfd::FileDialog::new().add_filter("Parquet", &["parquet"]).pick_file() {
                if let Ok(lf) = tb_data::ingestion::load_parquet(&path) {
                    if let Ok(df) = lf.clone().collect() {
                        match parse_dataframe_to_bars(&df) {
                            Ok(bar_data) => {
                                app.raw_dataframe = Some(df.clone());
                                app.full_bar_data = bar_data.bars.clone();
                                
                                let total_bars = bar_data.bars.len();
                                let max_points = 2000;
                                let minimap_points: Vec<egui_plot::PlotPoint> = if total_bars > max_points {
                                    let chunk_size = (total_bars as f64 / max_points as f64).ceil() as usize;
                                    bar_data.bars.chunks(chunk_size).enumerate().map(|(chunk_idx, chunk)| {
                                        let x_center = chunk_idx * chunk_size + chunk.len() / 2;
                                        let y_val = chunk.last().unwrap().close;
                                        egui_plot::PlotPoint::new(x_center as f64, y_val)
                                    }).collect()
                                } else {
                                    bar_data.bars.iter().enumerate().map(|(i, b)| egui_plot::PlotPoint::new(i as f64, b.close)).collect()
                                };
                                app.minimap_line_cache = Some(minimap_points);
                                
                                let y_min = bar_data.bars.iter().filter(|b| !b.close.is_nan()).fold(f64::INFINITY, |a, b| a.min(b.close));
                                let y_max = bar_data.bars.iter().filter(|b| !b.close.is_nan()).fold(f64::NEG_INFINITY, |a, b| a.max(b.close));
                                app.minimap_bounds_cache = Some((y_min, y_max));
                                
                                app.scrubber_index = 0;
                                
                                let slice_end = app.scrubber_window.min(bar_data.bars.len());
                                let window_data = bar_data.bars[0..slice_end].to_vec();
                                let window_bar_data = egui_charts::model::BarData { bars: window_data };
                                
                                let symbol = path.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                                app.sandbox_chart = egui_charts::ChartBuilder::new()
                                    .with_symbol(&symbol)
                                    .with_timeframe(egui_charts::model::Timeframe::Hour1)
                                    .with_theme(crate::theme::get_charts_theme())
                                    .build();
                                app.sandbox_chart.chart.update_data(window_bar_data.clone());
                                
                                app.forge_chart = egui_charts::ChartBuilder::new()
                                    .with_symbol(&symbol)
                                    .with_timeframe(egui_charts::model::Timeframe::Hour1)
                                    .with_theme(crate::theme::get_charts_theme())
                                    .build();
                                app.forge_chart.chart.update_data(window_bar_data);
                            },
                            Err(e) => println!("ERROR Parsing DataFrame to Bars: {}", e),
                        }
                    }
                    app.loaded_data = Some(lf);
                    println!("Data loaded successfully from {:?}", path);
                    app.rebuild_engine_if_data_loaded();
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
              ui.label(egui::RichText::new("📊 Dataset Active").color(egui::Color32::GREEN));
          }

    });

    let central_frame = egui::Frame::default().fill(ctx.style().visuals.panel_fill).inner_margin(16.0)
        .rounding(egui::CornerRadius { nw: 0, ne: 0, sw: 0, se: 8 });
    
    if let Some(minimap_points) = &app.minimap_line_cache {
        egui::TopBottomPanel::bottom("minimap_panel")
            .exact_height(80.0)
            .frame(egui::Frame::default().fill(ctx.style().visuals.panel_fill).inner_margin(4.0))
            .show_inside(outer_ui, |ui| {
                let mut changed = false;
                
                let mut w = ui.available_width();
                if w <= 0.0 { w = ui.ctx().screen_rect().width(); }
                let h = ui.available_height();
                let final_h = if h.is_infinite() || h <= 0.0 { 72.0 } else { h };
                
                let (response, painter) = ui.allocate_painter(egui::vec2(w, final_h), egui::Sense::click_and_drag());
                let rect = response.rect;
                
                if let Some((y_min, y_max)) = app.minimap_bounds_cache {
                    let x_scale = rect.width() / app.full_bar_data.len().max(1) as f32;
                    let y_range = (y_max - y_min) as f32;
                    
                    // 1. Draw Minimap Line
                    let mut shape_points = Vec::with_capacity(minimap_points.len());
                    for p in minimap_points.iter() {
                        if p.y.is_nan() || p.x.is_nan() { continue; }
                        let x = rect.left() + (p.x as f32 * x_scale);
                        let y_norm = if y_range > 0.0 { 1.0 - ((p.y as f32 - y_min as f32) / y_range) } else { 0.5 };
                        let y = rect.top() + (y_norm * rect.height());
                        shape_points.push(egui::pos2(x, y));
                    }
                    if !shape_points.is_empty() {
                        painter.add(egui::Shape::line(shape_points, egui::Stroke::new(1.5, crate::theme::ACCENT_INDIGO)));
                    }
                    
                    // 2. Draw Window Box
                    let window_start_x = rect.left() + (app.scrubber_index as f32 * x_scale);
                    let window_end_x = rect.left() + ((app.scrubber_index + app.scrubber_window) as f32 * x_scale);
                    
                    let window_rect = egui::Rect::from_min_max(
                        egui::pos2(window_start_x, rect.top()),
                        egui::pos2(window_end_x, rect.bottom())
                    );
                    
                    let accent = crate::theme::ACCENT_INDIGO;
                    let window_bg = egui::Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 40);
                    painter.rect_filled(window_rect, 0.0, window_bg);
                    
                    // 3. Draw Handles
                    let handle_w = 6.0;
                    let left_handle = egui::Rect::from_min_max(
                        egui::pos2(window_start_x - handle_w, rect.top() + 4.0),
                        egui::pos2(window_start_x, rect.bottom() - 4.0)
                    );
                    let right_handle = egui::Rect::from_min_max(
                        egui::pos2(window_end_x, rect.top() + 4.0),
                        egui::pos2(window_end_x + handle_w, rect.bottom() - 4.0)
                    );
                    
                    painter.rect_filled(left_handle, 4.0, accent);
                    painter.rect_filled(right_handle, 4.0, accent);
                    
                    // 4. Interaction Logic
                    if response.drag_started() {
                        if let Some(pos) = response.interact_pointer_pos() {
                            if left_handle.expand(10.0).contains(pos) {
                                app.scrubber_drag_mode = 1;
                            } else if right_handle.expand(10.0).contains(pos) {
                                app.scrubber_drag_mode = 2;
                            } else {
                                app.scrubber_drag_mode = 3;
                            }
                        }
                    } else if response.drag_stopped() {
                        app.scrubber_drag_mode = 0;
                    }
                    
                    if (response.dragged() && app.scrubber_drag_mode != 0) || (response.clicked() && app.scrubber_drag_mode == 0) {
                        if let Some(pos) = response.interact_pointer_pos() {
                            let target_idx = ((pos.x - rect.left()) / x_scale).clamp(0.0, app.full_bar_data.len() as f32) as usize;
                            let max_idx = app.full_bar_data.len();
                            let min_window = 100;
                            let max_window = 5000;
                            
                            let mode = if response.clicked() { 3 } else { app.scrubber_drag_mode };
                            
                            match mode {
                                1 => {
                                    let current_end = app.scrubber_index + app.scrubber_window;
                                    let safe_target = target_idx.min(current_end.saturating_sub(min_window));
                                    let new_start = if current_end.saturating_sub(safe_target) > max_window {
                                        current_end.saturating_sub(max_window)
                                    } else {
                                        safe_target
                                    };
                                    app.scrubber_window = current_end - new_start;
                                    app.scrubber_index = new_start;
                                    changed = true;
                                },
                                2 => {
                                    let current_start = app.scrubber_index;
                                    let safe_target = target_idx.max(current_start + min_window).min(max_idx);
                                    let new_end = if safe_target.saturating_sub(current_start) > max_window {
                                        current_start + max_window
                                    } else {
                                        safe_target
                                    };
                                    app.scrubber_window = new_end - current_start;
                                    changed = true;
                                },
                                3 => {
                                    let half = app.scrubber_window / 2;
                                    let new_start = target_idx.saturating_sub(half);
                                    app.scrubber_index = new_start.min(max_idx.saturating_sub(app.scrubber_window));
                                    changed = true;
                                },
                                _ => {}
                            }
                        }
                    }
                }
                
                if changed && !app.full_bar_data.is_empty() {
                    let end_idx = (app.scrubber_index + app.scrubber_window).min(app.full_bar_data.len());
                    let window_data = app.full_bar_data[app.scrubber_index..end_idx].to_vec();
                    let window_bar_data = egui_charts::model::BarData { bars: window_data };
                    app.sandbox_chart.chart.update_data(window_bar_data.clone());
                    app.forge_chart.chart.update_data(window_bar_data);
                }
            });
    }

    egui::CentralPanel::default().frame(central_frame).show_inside(outer_ui, |ui| {
        app.sandbox_chart.show(ui); 
        
        // CPCV dragging logic removed
    });
}
