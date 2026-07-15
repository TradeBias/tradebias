use eframe::egui;
use crate::state::TradingApp;

pub fn render(app: &mut TradingApp, ctx: &egui::Context) {
    egui::SidePanel::left("nav_rail")
        .exact_width(64.0)
        .resizable(false)
        .show(ctx, |ui| {
            ui.add_space(16.0);
            
            ui.vertical_centered(|ui| {
                ui.heading("tb");
                ui.add_space(32.0);
                
                let icon_size = 24.0;
                
                if ui.selectable_label(app.selected_tab == 0, egui::RichText::new("📊").size(icon_size))
                    .on_hover_text("Data Sandbox")
                    .clicked() 
                {
                    app.selected_tab = 0;
                }
                
                ui.add_space(16.0);
                
                if ui.selectable_label(app.selected_tab == 1, egui::RichText::new("🧪").size(icon_size))
                    .on_hover_text("Alpha Foundry")
                    .clicked() 
                {
                    app.selected_tab = 1;
                }
                
                ui.add_space(16.0);
                
                if ui.selectable_label(app.selected_tab == 3, egui::RichText::new("📚").size(icon_size))
                    .on_hover_text("Strategy Library")
                    .clicked() 
                {
                    app.selected_tab = 3;
                }
                
                ui.add_space(16.0);
                
                if ui.selectable_label(app.selected_tab == 4, egui::RichText::new("🏗️").size(icon_size))
                    .on_hover_text("Indicator Forge")
                    .clicked() 
                {
                    app.selected_tab = 4;
                }
            });
        });
}
