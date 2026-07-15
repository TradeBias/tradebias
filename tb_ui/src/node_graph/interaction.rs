use eframe::egui;
use super::state::NodeGraphState;
use super::painter;

pub fn handle_interaction(ui: &mut egui::Ui, state: &mut NodeGraphState, blueprints: &[tb_core::ast::IndicatorBlueprint]) -> Option<String> {
    let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
    let rect = response.rect;
    
    // Pan and Zoom
    if response.dragged_by(egui::PointerButton::Secondary) || response.dragged_by(egui::PointerButton::Middle) {
        state.pan += response.drag_delta();
    }
    
    if ui.rect_contains_pointer(rect) {
        let scroll = ui.input(|i| i.raw_scroll_delta.y);
        if scroll != 0.0 {
            let zoom_delta = (scroll * 0.005).exp();
            state.zoom *= zoom_delta;
            state.zoom = state.zoom.clamp(0.2, 5.0);
        }
    }
    
    if response.clicked() {
        // If clicked on background, clear selection
        state.selected_node = None;
    }
    
    // Handle Delete Node
    if ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) {
        if let Some(selected_id) = state.selected_node {
            // Delete node
            state.nodes.retain(|n| n.id != selected_id);
            // Delete associated wires
            state.wires.retain(|w| w.from_node != selected_id && w.to_node != selected_id);
            state.selected_node = None;
        }
    }
    
    // Draw Background Grid
    painter::draw_background(&painter, rect, state.pan, state.zoom);
    
    // Draw Wires
    painter::draw_wires(&painter, rect, state);
    
    // Draw Nodes
    painter::draw_nodes(ui, &painter, rect, &response, state, blueprints)
}
