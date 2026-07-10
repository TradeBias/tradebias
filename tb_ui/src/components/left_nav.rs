use eframe::egui;
use crate::state::TradingApp;

pub fn render(app: &mut TradingApp, ctx: &egui::Context) {
    egui::SidePanel::left("nav_rail").default_width(180.0).show(ctx, |ui| {
        ui.heading("tb_ui");
        ui.add_space(16.0);
        
        ui.vertical(|ui| {
            if ui.selectable_label(app.selected_tab == 0, "📊 Data Sandbox").clicked() {
                app.selected_tab = 0;
            }
            if ui.selectable_label(app.selected_tab == 1, "🧬 Alpha Foundry").clicked() {
                app.selected_tab = 1;
            }
            if ui.selectable_label(app.selected_tab == 2, "⚔️ Execution Simulator").clicked() {
                app.selected_tab = 2;
            }
            if ui.selectable_label(app.selected_tab == 3, "📚 Strategy Library").clicked() {
                app.selected_tab = 3;
            }
        });
    });
}
