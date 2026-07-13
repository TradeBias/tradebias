use eframe::egui;

pub use crate::state::TradingApp;

impl eframe::App for TradingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::components::modals::render_mapping_modal(self, ctx);
        crate::components::modals::render_robustness_modal(self, ctx);

        // 1. Global Status Bar (Bottom)
        egui::TopBottomPanel::bottom("status_bar").exact_height(24.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Greenfield Trading Engine v0.1.0");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("RAM: 1.2GB | GPU: Ready");
                });
            });
        });

        // 2. Global Left Navigation Rail
        crate::components::left_nav::render(self, ctx);

        // 3. Tab Routing (Tabs provide their own inner SidePanels and CentralPanels)
        match self.selected_tab {
            0 => crate::tabs::data_sandbox::render(self, ctx),
            1 => crate::tabs::alpha_foundry::render(self, ctx),
            2 => crate::tabs::simulator::render(self, ctx),
            3 => crate::tabs::library::render(self, ctx),
            _ => { 
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Select a Tab"); 
                });
            }
        }
    }
}
