use eframe::egui;
use crate::state::TradingApp;

pub fn render(app: &mut TradingApp, ui: &mut egui::Ui) {
    let frame = egui::Frame::none()
        .fill(crate::theme::RAIL_FILL)
        .rounding(egui::CornerRadius { nw: 8, sw: 8, ne: 0, se: 0 });
        
    egui::SidePanel::left("nav_rail")
        .exact_width(64.0)
        .resizable(false)
        .frame(frame)
        .show_inside(ui, |ui| {
            ui.add_space(16.0);
            
            ui.vertical_centered(|ui| {
                ui.add(egui::Image::new(egui::include_image!("../../../assets/logo.svg")).max_height(32.0));
                ui.add_space(32.0);
                
                let icon_size = 24.0;
                
                if ui.selectable_label(app.selected_tab == 0, egui::RichText::new(egui_phosphor::regular::DATABASE).size(icon_size))
                    .on_hover_text("Data Sandbox")
                    .clicked() 
                {
                    app.selected_tab = 0;
                }
                
                ui.add_space(16.0);
                
                if ui.selectable_label(app.selected_tab == 1, egui::RichText::new(egui_phosphor::regular::FLASK).size(icon_size))
                    .on_hover_text("Alpha Foundry")
                    .clicked() 
                {
                    app.selected_tab = 1;
                }
                
                ui.add_space(16.0);

                if ui.selectable_label(app.selected_tab == 2, egui::RichText::new(egui_phosphor::regular::GAME_CONTROLLER).size(icon_size))
                    .on_hover_text("Simulator")
                    .clicked() 
                {
                    app.selected_tab = 2;
                }
                
                ui.add_space(16.0);
                
                if ui.selectable_label(app.selected_tab == 3, egui::RichText::new(egui_phosphor::regular::BOOKS).size(icon_size))
                    .on_hover_text("Strategy Library")
                    .clicked() 
                {
                    app.selected_tab = 3;
                }
                
                ui.add_space(16.0);
                
                if ui.selectable_label(app.selected_tab == 4, egui::RichText::new(egui_phosphor::regular::WRENCH).size(icon_size))
                    .on_hover_text("Indicator Forge")
                    .clicked() 
                {
                    app.selected_tab = 4;
                }
            });
        });
}
