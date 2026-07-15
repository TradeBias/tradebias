use eframe::egui;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    Source(String),         // e.g. Close, Open
    Indicator(String),      // e.g. SMA, EMA
    Operator(String),       // e.g. Add, Sub
    Condition(String),      // e.g. Crossover, GreaterThan
    Constant(f64),
    OutputExpr,             // Terminal node for Indicator Graph
    OutputCondition,        // Terminal node for Strategy Graph
    Blueprint(String),      // A complete subgraph acting as a single node
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub kind: NodeKind,
    pub pos: egui::Pos2,
    // Inline Configuration Parameters (e.g., LEN, MULT)
    pub inline_params: Vec<(String, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Wire {
    pub from_node: usize,
    pub from_pin: usize, // output pin index
    pub to_node: usize,
    pub to_pin: usize, // input pin index
}

#[derive(Clone, Debug)]
pub struct NodeGraphState {
    pub nodes: Vec<Node>,
    pub wires: Vec<Wire>,
    pub next_node_id: usize,
    
    // Pan and Zoom
    pub pan: egui::Vec2,
    pub zoom: f32,
    pub selected_node: Option<usize>,
    
    // Interaction state
    pub dragging_node: Option<usize>,
    pub dragging_wire_from: Option<(usize, usize)>, // (node_id, pin_index)
}

impl Default for NodeGraphState {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            wires: Vec::new(),
            next_node_id: 1,
            pan: egui::Vec2::ZERO,
            zoom: 1.0,
            selected_node: None,
            dragging_node: None,
            dragging_wire_from: None,
        }
    }
}
