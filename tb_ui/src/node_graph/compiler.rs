use tb_core::ast::{Expr, IndicatorBlueprint};
use super::state::{NodeGraphState, NodeKind, Wire, Node};

pub fn compile_graph(state: &NodeGraphState, blueprints: &[IndicatorBlueprint]) -> Result<Expr, String> {
    // Find output node
    let output_nodes: Vec<&Node> = state.nodes.iter()
        .filter(|n| matches!(n.kind, NodeKind::OutputExpr | NodeKind::OutputCondition))
        .collect();
        
    if output_nodes.is_empty() {
        return Err("No Output node found. Please add an Output (Math) or Output (Trigger) node to compile the graph.".to_string());
    }
    
    if output_nodes.len() > 1 {
        return Err("Multiple Output nodes found. A blueprint can only have one output.".to_string());
    }
    
    let root = output_nodes[0];
    
    // Find what is wired into the root
    let root_inputs: Vec<&Wire> = state.wires.iter().filter(|w| w.to_node == root.id).collect();
    if root_inputs.is_empty() {
        return Err("Output node has no inputs connected.".to_string());
    }
    
    let source_wire = root_inputs[0];
    let source_node = state.nodes.iter().find(|n| n.id == source_wire.from_node).unwrap();
    
    match root.kind {
        NodeKind::OutputExpr => {
            let expr = compile_expr(source_node, source_wire.from_pin, state, blueprints)?;
            Ok(expr)
        },
        NodeKind::OutputCondition => {
            let expr = compile_expr(source_node, source_wire.from_pin, state, blueprints)?;
            Ok(expr)
        },
        _ => unreachable!()
    }
}

fn compile_expr(node: &Node, requested_pin: usize, state: &NodeGraphState, blueprints: &[IndicatorBlueprint]) -> Result<Expr, String> {
    match &node.kind {
        NodeKind::Source(s) => {
            match s.as_str() {
                "Close" => Ok(Expr::Close),
                "Open" => Ok(Expr::Open),
                "High" => Ok(Expr::High),
                "Low" => Ok(Expr::Low),
                "Volume" => Ok(Expr::Volume),
                _ => Err(format!("Unknown source type: {}", s))
            }
        },
        NodeKind::Constant(c) => Ok(Expr::Constant{ value: *c }),
        NodeKind::Indicator(ind_type) => {
            let inputs: Vec<&Wire> = state.wires.iter().filter(|w| w.to_node == node.id).collect();
            if inputs.is_empty() {
                return Err(format!("Indicator {} is missing an input wire.", ind_type));
            }
            
            let input_wire = inputs[0];
            let input_node = state.nodes.iter().find(|n| n.id == input_wire.from_node).unwrap();
            let child_expr = compile_expr(input_node, input_wire.from_pin, state, blueprints)?;
            
            // For now, hardcode var names based on node ID to ensure they are unique
            let var_name = format!("Node{}_Param", node.id);
            let period = node.inline_params.iter().find(|(k, _)| k == "LEN").map(|(_, v)| *v as u32).unwrap_or(14);
            
            match ind_type.as_str() {
                "SMA" => Ok(Expr::Sma { source: Box::new(child_expr), period }),
                "EMA" => Ok(Expr::Ema { source: Box::new(child_expr), period }),
                "WMA" => Ok(Expr::Wma { source: Box::new(child_expr), period }),
                "RMA" => Ok(Expr::Rma { source: Box::new(child_expr), period }),
                "StdDev" => Ok(Expr::StdDev { source: Box::new(child_expr), period }),
                "Delay" => Ok(Expr::Delay { source: Box::new(child_expr), period }),
                "TsMax" => Ok(Expr::TsMax { source: Box::new(child_expr), period }),
                "TsMin" => Ok(Expr::TsMin { source: Box::new(child_expr), period }),
                "Abs" => Ok(Expr::Abs { source: Box::new(child_expr) }),
                // Templates (RSI, MACD)
                "MACD" => Ok(Expr::Sub { 
                    lhs: Box::new(Expr::Ema { source: Box::new(child_expr.clone()), period: 12 }), 
                    rhs: Box::new(Expr::Ema { source: Box::new(child_expr), period: 26 })
                }), 
                "RSI" => {
                    // Approximate RSI using RMA (which is what TradingView uses)
                    // We don't have native RSI in AST, so we use placeholders or math if possible.
                    // For now, let's keep it simple or add an RSI node to AST.
                    // Actually tb_core doesn't have RSI yet. We'd have to construct it or fail.
                    Err(format!("RSI macro not fully implemented in compiler yet"))
                },
                _ => Err(format!("Unknown indicator type: {}", ind_type))
            }
        },
        NodeKind::Operator(op) => {
            // Need two inputs
            let mut inputs: Vec<&Wire> = state.wires.iter().filter(|w| w.to_node == node.id).collect();
            inputs.sort_by_key(|w| w.to_pin); // to_pin 0 is left, to_pin 1 is right
            
            if inputs.len() < 2 {
                return Err(format!("Operator {} is missing input wires.", op));
            }
            
            let left_node = state.nodes.iter().find(|n| n.id == inputs[0].from_node).unwrap();
            let right_node = state.nodes.iter().find(|n| n.id == inputs[1].from_node).unwrap();
            
            let left_expr = compile_expr(left_node, inputs[0].from_pin, state, blueprints)?;
            let right_expr = compile_expr(right_node, inputs[1].from_pin, state, blueprints)?;
            
            match op.as_str() {
                "Add" => Ok(Expr::Add{ lhs: Box::new(left_expr), rhs: Box::new(right_expr) }),
                "Sub" => Ok(Expr::Sub{ lhs: Box::new(left_expr), rhs: Box::new(right_expr) }),
                "Mul" => Ok(Expr::Mul{ lhs: Box::new(left_expr), rhs: Box::new(right_expr) }),
                "Div" => Ok(Expr::Div{ lhs: Box::new(left_expr), rhs: Box::new(right_expr) }),
                _ => Err(format!("Unknown operator: {}", op))
            }
        },
        NodeKind::Condition(cond_type) => {
            let mut inputs: Vec<&Wire> = state.wires.iter().filter(|w| w.to_node == node.id).collect();
            inputs.sort_by_key(|w| w.to_pin);
            
            if inputs.len() < 2 {
                return Err(format!("Condition {} is missing input wires.", cond_type));
            }
            
            let left_node = state.nodes.iter().find(|n| n.id == inputs[0].from_node).unwrap();
            let right_node = state.nodes.iter().find(|n| n.id == inputs[1].from_node).unwrap();
            
            let left_expr = compile_expr(left_node, inputs[0].from_pin, state, blueprints)?;
            let right_expr = compile_expr(right_node, inputs[1].from_pin, state, blueprints)?;
            
            match cond_type.as_str() {
                "Crossover" => Ok(Expr::CrossAbove { lhs: Box::new(left_expr), rhs: Box::new(right_expr) }),
                "Crossunder" => Ok(Expr::CrossBelow { lhs: Box::new(left_expr), rhs: Box::new(right_expr) }),
                "GreaterThan" => Ok(Expr::GreaterThan { lhs: Box::new(left_expr), rhs: Box::new(right_expr) }),
                "LessThan" => Ok(Expr::LessThan { lhs: Box::new(left_expr), rhs: Box::new(right_expr) }),
                _ => Err(format!("Unknown condition: {}", cond_type))
            }
        },
        NodeKind::Blueprint(bp_name) => {
            let inputs: Vec<&Wire> = state.wires.iter().filter(|w| w.to_node == node.id).collect();
            if inputs.is_empty() {
                return Err(format!("Blueprint {} is missing an input wire.", bp_name));
            }
            
            let input_wire = inputs[0];
            let input_node = state.nodes.iter().find(|n| n.id == input_wire.from_node).unwrap();
            let child_expr = compile_expr(input_node, input_wire.from_pin, state, blueprints)?;
            
            let bp_template = blueprints.iter().find(|b| &b.name == bp_name)
                .ok_or_else(|| format!("Blueprint {} not found in registry", bp_name))?;
                
            let output_name = bp_template.outputs.get(requested_pin)
                .map(|(n, _)| n.clone())
                .unwrap_or_else(|| bp_template.outputs[0].0.clone());
            
            // If it's a custom indicator (not a built-in Macro), we could unroll it here, 
            // but for now let's just make it a Macro node and unroll in the engine.
            
            Ok(Expr::Macro {
                name: bp_name.clone(),
                output: output_name,
                source: Box::new(child_expr),
                params: node.inline_params.clone(),
            })
        },
        _ => Err(format!("Node type {:?} cannot be evaluated as an Expression.", node.kind))
    }
}
