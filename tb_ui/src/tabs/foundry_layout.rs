use egui_tiles::{Behavior, TileId, UiResponse};
use crate::state::TradingApp;

pub struct FoundryBehavior<'a> {
    pub app: &'a mut TradingApp,
}

impl<'a> Behavior<&'static str> for FoundryBehavior<'a> {
    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: TileId, pane: &mut &'static str) -> UiResponse {
        let frame = egui::Frame::default()
            .fill(crate::theme::WINDOW_FILL)
            .inner_margin(16.0)
            .corner_radius(8)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(51, 51, 51)));

        frame.show(ui, |ui| {
            ui.heading(*pane);
            ui.add_space(8.0);
            match *pane {
                "Settings" => {
                    crate::tabs::alpha_foundry::render_settings(self.app, ui);
                }
                "Leaderboard" => {
                    crate::tabs::alpha_foundry::render_leaderboard(self.app, ui);
                }
                "Stats" => {
                    crate::tabs::alpha_foundry::render_stats(self.app, ui);
                }
                "Robustness" => {
                    crate::components::modals::render_robustness_ui(self.app, ui);
                }
                _ => {}
            }
        });
        
        UiResponse::None
    }
    
    fn tab_title_for_pane(&mut self, pane: &&'static str) -> egui::WidgetText {
        (*pane).into()
    }
}
