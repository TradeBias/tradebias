use eframe::egui;
use super::state::{NodeGraphState, NodeKind};

pub fn draw_background(painter: &egui::Painter, rect: egui::Rect, pan: egui::Vec2, zoom: f32) {
    painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(20, 20, 20));
    
    // Draw Grid
    let grid_size = 50.0 * zoom;
    let offset = egui::vec2(pan.x % grid_size, pan.y % grid_size);
    
    for i in 0..=(rect.width() / grid_size) as i32 + 1 {
        let x = rect.left() + offset.x + (i as f32) * grid_size;
        painter.vline(x, rect.y_range(), egui::Stroke::new(1.0, egui::Color32::from_white_alpha(10)));
    }
    
    for i in 0..=(rect.height() / grid_size) as i32 + 1 {
        let y = rect.top() + offset.y + (i as f32) * grid_size;
        painter.hline(rect.x_range(), y, egui::Stroke::new(1.0, egui::Color32::from_white_alpha(10)));
    }
}

pub fn draw_wires(painter: &egui::Painter, rect: egui::Rect, state: &NodeGraphState) {
    for wire in &state.wires {
        if let Some(from_node) = state.nodes.iter().find(|n| n.id == wire.from_node) {
            if let Some(to_node) = state.nodes.iter().find(|n| n.id == wire.to_node) {
                let from_screen = rect.min + from_node.pos.to_vec2() * state.zoom + state.pan;
                let to_screen = rect.min + to_node.pos.to_vec2() * state.zoom + state.pan;
                
                let from_pos = get_pin_pos(from_screen, wire.from_pin, true, state.zoom);
                let to_pos = get_pin_pos(to_screen, wire.to_pin, false, state.zoom);
                draw_bezier(painter, from_pos, to_pos, egui::Color32::LIGHT_BLUE);
            }
        }
    }
    
    if let Some((from_node_id, from_pin)) = state.dragging_wire_from {
        if let Some(pos) = painter.ctx().pointer_latest_pos() {
            if let Some(from_node) = state.nodes.iter().find(|n| n.id == from_node_id) {
                let from_screen = rect.min + from_node.pos.to_vec2() * state.zoom + state.pan;
                let from_pos = get_pin_pos(from_screen, from_pin, true, state.zoom);
                draw_bezier(painter, from_pos, pos, egui::Color32::YELLOW);
            }
        }
    }
}

fn draw_bezier(painter: &egui::Painter, from: egui::Pos2, to: egui::Pos2, color: egui::Color32) {
    let control_dist = ((to.x - from.x).abs() * 0.5).max(50.0);
    let cp1 = from + egui::vec2(control_dist, 0.0);
    let cp2 = to - egui::vec2(control_dist, 0.0);
    
    let shape = egui::Shape::CubicBezier(egui::epaint::CubicBezierShape {
        points: [from, cp1, cp2, to],
        closed: false,
        fill: egui::Color32::TRANSPARENT,
        stroke: egui::Stroke::new(3.0, color).into(),
    });
    painter.add(shape);
}

fn get_pin_pos(node_pos_screen: egui::Pos2, pin_idx: usize, is_output: bool, zoom: f32) -> egui::Pos2 {
    let node_width = 150.0 * zoom;
    if is_output {
        node_pos_screen + egui::vec2(node_width, 20.0 * zoom)
    } else {
        node_pos_screen + egui::vec2(0.0, 20.0 * zoom + (pin_idx as f32 * 20.0 * zoom))
    }
}

pub fn draw_nodes(ui: &mut egui::Ui, painter: &egui::Painter, rect: egui::Rect, response: &egui::Response, state: &mut NodeGraphState, blueprints: &[tb_core::ast::IndicatorBlueprint]) -> Option<String> {
    let mut node_to_drag = None;
    let mut wire_start_drag = None;
    let mut hovered_input_pin = None;
    let mut double_clicked_blueprint = None;
    
    // We need to use indexing so we can mutate the nodes
    for i in 0..state.nodes.len() {
        let node_id = state.nodes[i].id;
        let node_pos_base = state.nodes[i].pos;
        let node_kind = state.nodes[i].kind.clone();
        
        let screen_pos = rect.min + node_pos_base.to_vec2() * state.zoom + state.pan;
        
        // Calculate dynamic height based on number of inline params
        let param_count = state.nodes[i].inline_params.len();
        let dynamic_height = 50.0 + (param_count as f32 * 25.0);
        let node_size = egui::vec2(150.0, dynamic_height) * state.zoom;
        let node_rect = egui::Rect::from_min_size(screen_pos, node_size);
        
        let in_pos = get_pin_pos(screen_pos, 0, false, state.zoom);
        
        // Find how many outputs this node has
        let output_names: Vec<String> = if let NodeKind::Blueprint(bp_name) = &node_kind {
            if let Some(bp) = blueprints.iter().find(|b| &b.name == bp_name) {
                bp.outputs.iter().map(|(n, _)| n.clone()).collect()
            } else {
                vec!["Output".to_string()]
            }
        } else {
            vec!["".to_string()]
        };
        let num_outputs = output_names.len();
        
        let node_response = ui.allocate_rect(node_rect, egui::Sense::click_and_drag());
        if node_response.double_clicked() {
            if let NodeKind::Blueprint(bp_name) = &node_kind {
                double_clicked_blueprint = Some(bp_name.clone());
            }
        }
        
        if node_response.clicked() {
            state.selected_node = Some(node_id);
        } else if response.clicked() && !ui.rect_contains_pointer(node_rect) {
            // Clicked outside, we could clear selection but it might clear it too eagerly if clicking another node.
            // A better way is to only clear if we didn't click any node. We'll handle this in interaction.rs.
        }
        
        let mut node_deleted = false;
        node_response.context_menu(|ui| {
            if ui.button("🗑 Delete Node").clicked() {
                node_deleted = true;
                ui.close_menu();
            }
        });
        
        if node_deleted {
            // we will handle deletion below after the loop
            state.selected_node = Some(node_id); // Mark it so we can delete it easily
            ui.input_mut(|i| i.events.push(egui::Event::Key {
                key: egui::Key::Delete,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
                repeat: false,
                physical_key: None,
            })); // Hack to trigger deletion
        }
        
        let mut is_dragging_pin = false;
        let pointer_pos = ui.ctx().pointer_hover_pos();
        if let Some(pos) = pointer_pos {
            // Check all valid input pins
            let num_inputs = match node_kind {
                NodeKind::Source(_) | NodeKind::Constant(_) => 0,
                NodeKind::Operator(_) | NodeKind::Condition(_) => 2,
                _ => 1,
            };
            
            for pin_idx in 0..num_inputs {
                let pin_in_pos = get_pin_pos(screen_pos, pin_idx, false, state.zoom);
                if pin_in_pos.distance(pos) < 15.0 * state.zoom {
                    hovered_input_pin = Some((node_id, pin_idx));
                }
            }
            
            if node_response.drag_started() || response.drag_started() || ui.input(|i| i.pointer.primary_down()) {
                for pin_idx in 0..num_outputs {
                    let out_pos = get_pin_pos(screen_pos, pin_idx, true, state.zoom);
                    if out_pos.distance(pos) < 15.0 * state.zoom {
                        wire_start_drag = Some((node_id, pin_idx));
                        is_dragging_pin = true;
                        break;
                    }
                }
            }
        }
        
        if node_response.dragged() && !is_dragging_pin && state.dragging_wire_from.is_none() {
            state.nodes[i].pos += node_response.drag_delta() / state.zoom;
            node_to_drag = Some(node_id);
        }
        
        let header_color = match node_kind {
            NodeKind::Source(_) => egui::Color32::from_rgb(40, 80, 40),
            NodeKind::Indicator(_) => egui::Color32::from_rgb(80, 40, 40),
            NodeKind::Operator(_) => egui::Color32::from_rgb(40, 40, 80),
            NodeKind::Condition(_) => egui::Color32::from_rgb(80, 80, 40),
            NodeKind::Constant(_) => egui::Color32::from_rgb(60, 100, 60),
            NodeKind::OutputExpr | NodeKind::OutputCondition => egui::Color32::from_rgb(100, 60, 100),
            NodeKind::Blueprint(_) => egui::Color32::from_rgb(120, 80, 30),
        };
        
        let stroke_color = if state.selected_node == Some(node_id) {
            egui::Color32::YELLOW
        } else {
            egui::Color32::from_white_alpha(50)
        };
        
        let bg_color = ui.visuals().window_fill();
        painter.rect(node_rect, 8.0 * state.zoom, bg_color, egui::Stroke::new(if state.selected_node == Some(node_id) { 2.0 } else { 1.0 }, stroke_color), egui::StrokeKind::Inside);
        
        // Node Header Box
        let mut header_rect = node_rect;
        header_rect.set_height(25.0 * state.zoom);
        let rounding = egui::CornerRadius {
            nw: (8.0 * state.zoom) as u8,
            ne: (8.0 * state.zoom) as u8,
            sw: 0,
            se: 0,
        };
        painter.rect(header_rect, rounding, header_color, egui::Stroke::NONE, egui::StrokeKind::Inside);
        painter.hline(node_rect.x_range(), header_rect.bottom(), egui::Stroke::new(1.0, stroke_color));
        
        let title = match &node_kind {
            NodeKind::Source(s) => s.clone(),
            NodeKind::Indicator(s) => s.clone(),
            NodeKind::Operator(s) => s.clone(),
            NodeKind::Condition(s) => s.clone(),
            NodeKind::Constant(c) => format!("{}", c),
            NodeKind::OutputExpr => "Output (Math)".to_string(),
            NodeKind::OutputCondition => "Output (Trigger)".to_string(),
            NodeKind::Blueprint(s) => s.clone(),
        };
        
        painter.text(
            header_rect.center(),
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(12.0 * state.zoom),
            egui::Color32::WHITE,
        );
        
        // Render Inline Params
        if !state.nodes[i].inline_params.is_empty() {
            let inner_rect = node_rect.shrink(10.0 * state.zoom);
            let mut ui_rect = inner_rect;
            ui_rect.min.y += 20.0 * state.zoom; // Push below header
            
            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(ui_rect).layout(egui::Layout::top_down(egui::Align::Min)));
            
            for (p_name, p_val) in &mut state.nodes[i].inline_params {
                child_ui.horizontal(|p_ui| {
                    p_ui.label(egui::RichText::new(&*p_name).size(12.0 * state.zoom));
                    p_ui.add(egui::DragValue::new(p_val).speed(1.0));
                });
            }
        }
        
        // Pins (Inputs)
        let num_inputs = match node_kind {
            NodeKind::Source(_) | NodeKind::Constant(_) => 0,
            NodeKind::Operator(_) | NodeKind::Condition(_) => 2,
            _ => 1,
        };
        
        for pin_idx in 0..num_inputs {
            let pin_in_pos = get_pin_pos(screen_pos, pin_idx, false, state.zoom);
            let in_color = if hovered_input_pin == Some((node_id, pin_idx)) { egui::Color32::WHITE } else { egui::Color32::LIGHT_GREEN };
            painter.circle_filled(pin_in_pos, 6.0 * state.zoom, in_color);
            painter.circle_stroke(pin_in_pos, 6.0 * state.zoom, egui::Stroke::new(1.0, egui::Color32::BLACK));
        }
        
        // Pins (Outputs)
        if !matches!(node_kind, NodeKind::OutputExpr | NodeKind::OutputCondition) {
            for pin_idx in 0..num_outputs {
                let out_pos = get_pin_pos(screen_pos, pin_idx, true, state.zoom);
                painter.circle_filled(out_pos, 6.0 * state.zoom, egui::Color32::LIGHT_BLUE);
                painter.circle_stroke(out_pos, 6.0 * state.zoom, egui::Stroke::new(1.0, egui::Color32::BLACK));
                
                if !output_names[pin_idx].is_empty() && output_names.len() > 1 {
                    painter.text(
                        out_pos - egui::vec2(15.0 * state.zoom, 0.0),
                        egui::Align2::RIGHT_CENTER,
                        &output_names[pin_idx],
                        egui::FontId::proportional(10.0 * state.zoom),
                        egui::Color32::LIGHT_GRAY,
                    );
                }
            }
        }
    }
    
    // Process interactions after loop
    if let Some(pin) = wire_start_drag {
        state.dragging_wire_from = Some(pin);
    } else if let Some(id) = node_to_drag {
        state.dragging_node = Some(id);
    }
    
    if let Some(id) = state.dragging_node {
        // Node position is updated inline above now
    }
    
    if response.drag_stopped() || ui.input(|i| i.pointer.any_released()) {
        if let (Some((from_node, from_pin)), Some((to_node, to_pin))) = (state.dragging_wire_from, hovered_input_pin) {
            // Prevent cycles and duplicate connections (basic)
            if from_node != to_node {
                // Remove existing wire to this input pin (only 1 connection per input)
                state.wires.retain(|w| !(w.to_node == to_node && w.to_pin == to_pin));
                
                state.wires.push(crate::node_graph::Wire {
                    from_node,
                    from_pin,
                    to_node,
                    to_pin,
                });
            }
        }
        state.dragging_node = None;
        state.dragging_wire_from = None;
    }
    
    double_clicked_blueprint
}
