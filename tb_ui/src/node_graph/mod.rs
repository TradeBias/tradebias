pub mod state;
pub mod painter;
pub mod interaction;
pub mod compiler;
pub mod decompiler;

pub use state::{NodeGraphState, Node, NodeKind, Wire};

pub fn render_node_graph(ui: &mut eframe::egui::Ui, state: &mut NodeGraphState, blueprints: &[tb_core::ast::IndicatorBlueprint]) -> Option<String> {
    interaction::handle_interaction(ui, state, blueprints)
}
