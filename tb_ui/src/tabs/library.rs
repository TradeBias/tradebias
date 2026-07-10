use eframe::egui;
use crate::state::TradingApp;

pub fn render(_app: &mut TradingApp, ctx: &egui::Context) {
    let central_frame = egui::Frame::default().fill(ctx.style().visuals.panel_fill).inner_margin(16.0);
    egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📚 Strategy Library");
            ui.add_space(16.0);
            
            // Mock Tear Sheet Cards
            for i in 1..=5 {
                egui::Frame::default()
                    .fill(egui::Color32::from_gray(30))
                    .corner_radius(4.0)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(50)))
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.heading(format!("Elite Strategy #{}", i));
                                ui.label(egui::RichText::new("CrossAbove(SMA 10, SMA 50) + RSI Filter").color(egui::Color32::from_gray(180)));
                            });
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Deploy").clicked() {
                                    // deployment logic
                                }
                                if ui.button("View Equity Curve").clicked() {
                                    // view chart logic
                                }
                            });
                        });
                        
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);
                        
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format!("CAGR: {:.2}%", 18.0 + (i as f32 * 2.5))).strong().color(egui::Color32::GREEN));
                            ui.add_space(16.0);
                            ui.label(egui::RichText::new(format!("Max DD: -{:.1}%", 8.0 - (i as f32 * 0.5))).strong().color(egui::Color32::RED));
                            ui.add_space(16.0);
                            ui.label(egui::RichText::new(format!("Win Rate: {:.1}%", 52.0 + (i as f32))));
                            ui.add_space(16.0);
                            ui.label(egui::RichText::new("CPCV Variance: Low").color(egui::Color32::LIGHT_BLUE));
                        });
                    });
                ui.add_space(12.0);
            }
        });
    });
}
