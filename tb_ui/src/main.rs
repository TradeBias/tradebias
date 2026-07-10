#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod data_parser;
mod components;
mod tabs;
mod app;

use app::TradingApp;
use polars::prelude::*;

fn main() -> eframe::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0]),
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
