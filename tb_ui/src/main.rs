#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod data_parser;
mod components;
mod tabs;
mod app;
mod export;
mod theme;
pub mod node_graph;

use app::TradingApp;
use polars::prelude::*;

fn main() -> eframe::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let icon_img = image::load_from_memory(include_bytes!("../../assets/logo.png")).expect("Failed to load icon").to_rgba8();
    let (icon_width, icon_height) = icon_img.dimensions();
    
    // Make the actual logo 30% smaller inside the same bounding box (adds transparent padding)
    let new_width = (icon_width as f32 * 0.7) as u32;
    let new_height = (icon_height as f32 * 0.7) as u32;
    let resized = image::imageops::resize(&icon_img, new_width, new_height, image::imageops::FilterType::Lanczos3);
    
    let mut padded = image::RgbaImage::new(icon_width, icon_height);
    image::imageops::overlay(&mut padded, &resized, ((icon_width - new_width) / 2) as i64, ((icon_height - new_height) / 2) as i64);
    
    let icon_data = eframe::egui::IconData {
        rgba: padded.into_raw(),
        width: icon_width,
        height: icon_height,
    };

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(std::sync::Arc::new(icon_data))
            .with_decorations(false) // Disable standard OS title bar
            .with_transparent(true), // Enable transparent window for custom border
        ..Default::default()
    };

    // 1. Setup the inter-thread channels
    let (elite_tx, elite_rx) = crossbeam_channel::unbounded();
    
    // 2. Thread B (Execution Simulator) is now controlled via UI triggers directly

    // 3. Start the UI (Main Thread)
    eframe::run_native(
        "Greenfield Trading Engine",
        native_options,
        Box::new(|cc| {
            let mut app = TradingApp::new(cc);
            app.elite_tx = Some(elite_tx);
            app.elite_rx = Some(elite_rx);
            Ok(Box::new(app))
        }),
    )
}
