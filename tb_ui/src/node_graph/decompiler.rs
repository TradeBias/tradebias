use tb_core::ast::Expr;
use super::state::{NodeGraphState, NodeKind, Wire, Node};
use eframe::egui;

pub fn decompile_ast(ast: &Expr, is_trigger: bool) -> NodeGraphState {
    let mut state = NodeGraphState::default();
    
    let out_kind = if is_trigger { NodeKind::OutputCondition } else { NodeKind::OutputExpr };
    
    // Create output node
    let out_node = Node {
        id: state.next_node_id,
        kind: out_kind,
        pos: egui::pos2(700.0, 300.0),
        inline_params: vec![],
    };
    state.nodes.push(out_node.clone());
    state.next_node_id += 1;
    
    let child_id = decompile_expr(ast, &mut state, 450.0, 300.0);
    state.wires.push(Wire {
        from_node: child_id,
        from_pin: 0,
        to_node: out_node.id,
        to_pin: 0,
    });
    
    // Pan graph to show nodes cleanly
    state.pan = egui::vec2(-100.0, -100.0);
    
    state
}

fn decompile_expr(expr: &Expr, state: &mut NodeGraphState, x: f32, y: f32) -> usize {
    let id = state.next_node_id;
    state.next_node_id += 1;
    
    let mut kind = NodeKind::Constant(0.0);
    let mut inline_params = Vec::new();
    
    let mut children_to_process = Vec::new(); // (child_expr, pin_idx)
    
    match expr {
        Expr::Close => kind = NodeKind::Source("Close".to_string()),
        Expr::Open => kind = NodeKind::Source("Open".to_string()),
        Expr::High => kind = NodeKind::Source("High".to_string()),
        Expr::Low => kind = NodeKind::Source("Low".to_string()),
        Expr::Volume => kind = NodeKind::Source("Volume".to_string()),
        Expr::Placeholder => kind = NodeKind::Source("Placeholder".to_string()),
        Expr::Constant { value } => kind = NodeKind::Constant(*value),
        
        Expr::Add { lhs, rhs } => {
            kind = NodeKind::Operator("Add".to_string());
            children_to_process.push((&**lhs, 0));
            children_to_process.push((&**rhs, 1));
        }
        Expr::Sub { lhs, rhs } => {
            kind = NodeKind::Operator("Sub".to_string());
            children_to_process.push((&**lhs, 0));
            children_to_process.push((&**rhs, 1));
        }
        Expr::Mul { lhs, rhs } => {
            kind = NodeKind::Operator("Mul".to_string());
            children_to_process.push((&**lhs, 0));
            children_to_process.push((&**rhs, 1));
        }
        Expr::Div { lhs, rhs } => {
            kind = NodeKind::Operator("Div".to_string());
            children_to_process.push((&**lhs, 0));
            children_to_process.push((&**rhs, 1));
        }
        
        Expr::Sma { source, period } => {
            kind = NodeKind::Indicator("SMA".to_string());
            inline_params.push(("LEN".to_string(), *period as f64));
            children_to_process.push((&**source, 0));
        }
        Expr::Ema { source, period } => {
            kind = NodeKind::Indicator("EMA".to_string());
            inline_params.push(("LEN".to_string(), *period as f64));
            children_to_process.push((&**source, 0));
        }
        Expr::Wma { source, period } => {
            kind = NodeKind::Indicator("WMA".to_string());
            inline_params.push(("LEN".to_string(), *period as f64));
            children_to_process.push((&**source, 0));
        }
        Expr::Rma { source, period } => {
            kind = NodeKind::Indicator("RMA".to_string());
            inline_params.push(("LEN".to_string(), *period as f64));
            children_to_process.push((&**source, 0));
        }
        Expr::StdDev { source, period } => {
            kind = NodeKind::Indicator("StdDev".to_string());
            inline_params.push(("LEN".to_string(), *period as f64));
            children_to_process.push((&**source, 0));
        }
        Expr::TsMax { source, period } => {
            kind = NodeKind::Indicator("TsMax".to_string());
            inline_params.push(("LEN".to_string(), *period as f64));
            children_to_process.push((&**source, 0));
        }
        Expr::TsMin { source, period } => {
            kind = NodeKind::Indicator("TsMin".to_string());
            inline_params.push(("LEN".to_string(), *period as f64));
            children_to_process.push((&**source, 0));
        }
        Expr::Delay { source, period } => {
            kind = NodeKind::Indicator("Delay".to_string());
            inline_params.push(("LEN".to_string(), *period as f64));
            children_to_process.push((&**source, 0));
        }
        Expr::Abs { source } => {
            kind = NodeKind::Indicator("Abs".to_string());
            children_to_process.push((&**source, 0));
        }
        Expr::CrossAbove { lhs, rhs } => {
            kind = NodeKind::Condition("Crossover".to_string());
            children_to_process.push((&**lhs, 0));
            children_to_process.push((&**rhs, 1));
        }
        Expr::CrossBelow { lhs, rhs } => {
            kind = NodeKind::Condition("Crossunder".to_string());
            children_to_process.push((&**lhs, 0));
            children_to_process.push((&**rhs, 1));
        }
        Expr::GreaterThan { lhs, rhs } => {
            kind = NodeKind::Condition("GreaterThan".to_string());
            children_to_process.push((&**lhs, 0));
            children_to_process.push((&**rhs, 1));
        }
        Expr::LessThan { lhs, rhs } => {
            kind = NodeKind::Condition("LessThan".to_string());
            children_to_process.push((&**lhs, 0));
            children_to_process.push((&**rhs, 1));
        }
        
        _ => {
            kind = NodeKind::Constant(0.0);
        }
    }
    
    state.nodes.push(Node {
        id,
        kind,
        pos: egui::pos2(x, y),
        inline_params,
    });
    
    let x_offset = 220.0;
    let y_offset = 150.0;
    
    for (i, (child, pin)) in children_to_process.into_iter().enumerate() {
        let child_y = if i == 0 { y - y_offset } else { y + y_offset };
        let child_x = x - x_offset;
        
        let child_id = decompile_expr(child, state, child_x, child_y);
        
        state.wires.push(Wire {
            from_node: child_id,
            from_pin: 0,
            to_node: id,
            to_pin: pin,
        });
    }
    
    id
}
