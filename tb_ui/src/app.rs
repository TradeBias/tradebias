use eframe::egui;

pub use crate::state::TradingApp;

impl eframe::App for TradingApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make the outer OS window transparent
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Layout is now natively responsive, no complex state tree to serialize
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::components::modals::render_mapping_modal(self, ctx);
        crate::components::modals::render_robustness_modal(self, ctx);

        // 0. Borderless Window Resize Handles
        egui::Area::new(egui::Id::new("resize_edges"))
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                let screen_rect = ctx.screen_rect();
                let edge_thickness = 6.0;
                let corner_size = 16.0;
                
                let edges = [
                    (egui::Rect::from_min_max(screen_rect.min, egui::pos2(screen_rect.min.x + corner_size, screen_rect.min.y + corner_size)), egui::ResizeDirection::NorthWest, egui::CursorIcon::ResizeNwSe),
                    (egui::Rect::from_min_max(egui::pos2(screen_rect.max.x - corner_size, screen_rect.min.y), egui::pos2(screen_rect.max.x, screen_rect.min.y + corner_size)), egui::ResizeDirection::NorthEast, egui::CursorIcon::ResizeNeSw),
                    (egui::Rect::from_min_max(egui::pos2(screen_rect.min.x, screen_rect.max.y - corner_size), egui::pos2(screen_rect.min.x + corner_size, screen_rect.max.y)), egui::ResizeDirection::SouthWest, egui::CursorIcon::ResizeNeSw),
                    (egui::Rect::from_min_max(egui::pos2(screen_rect.max.x - corner_size, screen_rect.max.y - corner_size), screen_rect.max), egui::ResizeDirection::SouthEast, egui::CursorIcon::ResizeNwSe),
                    (egui::Rect::from_min_max(egui::pos2(screen_rect.min.x + corner_size, screen_rect.min.y), egui::pos2(screen_rect.max.x - corner_size, screen_rect.min.y + edge_thickness)), egui::ResizeDirection::North, egui::CursorIcon::ResizeVertical),
                    (egui::Rect::from_min_max(egui::pos2(screen_rect.min.x + corner_size, screen_rect.max.y - edge_thickness), egui::pos2(screen_rect.max.x - corner_size, screen_rect.max.y)), egui::ResizeDirection::South, egui::CursorIcon::ResizeVertical),
                    (egui::Rect::from_min_max(egui::pos2(screen_rect.min.x, screen_rect.min.y + corner_size), egui::pos2(screen_rect.min.x + edge_thickness, screen_rect.max.y - corner_size)), egui::ResizeDirection::West, egui::CursorIcon::ResizeHorizontal),
                    (egui::Rect::from_min_max(egui::pos2(screen_rect.max.x - edge_thickness, screen_rect.min.y + corner_size), egui::pos2(screen_rect.max.x, screen_rect.max.y - corner_size)), egui::ResizeDirection::East, egui::CursorIcon::ResizeHorizontal),
                ];
                
                for (i, (rect, dir, cursor)) in edges.into_iter().enumerate() {
                    let response = ui.interact(rect, ui.id().with(format!("resize_edge_{i}")), egui::Sense::drag());
                    if response.hovered() {
                        ctx.set_cursor_icon(cursor);
                    }
                    if response.drag_started() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(dir));
                    }
                }
            });

        let is_max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        let margin = if is_max { 0.0 } else { 16.0 };

        let outer_frame = egui::Frame::none()
            .fill(crate::theme::WINDOW_FILL)
            .rounding(if is_max { 0.0 } else { 8.0 })
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(51, 51, 51)))
            .shadow(if is_max { egui::epaint::Shadow::default() } else { egui::epaint::Shadow { offset: [0, 8], blur: 16, spread: 0, color: egui::Color32::from_black_alpha(150) } })
            .inner_margin(0.0)
            .outer_margin(margin);

        egui::CentralPanel::default().frame(outer_frame).show(ctx, |ui| {
            // 1.5 Custom Title Bar
            let title_frame = egui::Frame::none()
                .fill(crate::theme::WINDOW_FILL)
                .rounding(egui::CornerRadius { nw: 8, ne: 8, sw: 0, se: 0 });
                
            egui::TopBottomPanel::top("title_bar").exact_height(32.0).frame(title_frame).show_inside(ui, |ui| {
                let title_bar_rect = ui.max_rect();
                let title_bar_response = ui.interact(title_bar_rect, ui.id().with("title_bar"), egui::Sense::click());
                if title_bar_response.is_pointer_button_down_on() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }

                ui.horizontal_centered(|ui| {
                    ui.add_space(16.0);
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        if ui.add(egui::Button::new(egui_phosphor::regular::X).frame(false)).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.add(egui::Button::new(egui_phosphor::regular::SQUARE).frame(false)).clicked() {
                            let is_max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_max));
                        }
                        if ui.add(egui::Button::new(egui_phosphor::regular::MINUS).frame(false)).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                    });
                });
            });

            // 2. Global Left Navigation Rail
            crate::components::left_nav::render(self, ui);

            // 3. Tab Routing
            match self.selected_tab {
                0 => crate::tabs::data_sandbox::render(self, ctx, ui),
                1 => crate::tabs::alpha_foundry::render(self, ctx, ui),
                2 => crate::tabs::simulator::render(self, ctx, ui),
                3 => crate::tabs::library::render(self, ctx, ui),
                4 => crate::tabs::indicator_forge::render(self, ctx, ui),
                _ => { 
                    egui::CentralPanel::default().show_inside(ui, |ui| {
                        ui.heading("Select a Tab"); 
                    });
                }
            }
        });
    }
}
