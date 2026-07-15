use eframe::egui;
use crate::state::TradingApp;
use tb_core::ast::{Expr, IndicatorBlueprint, SemanticType};
use crate::node_graph;
use egui_plot::{Plot, Line, PlotPoints, Points};

pub fn render(app: &mut TradingApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Blueprint Name:");
            ui.text_edit_singleline(&mut app.forge_active_blueprint_name);
            
            ui.add_space(20.0);
            ui.label("Export Type:");
            egui::ComboBox::from_id_salt("forge_type_combo")
                .selected_text(format!("{:?}", app.forge_active_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.forge_active_type, SemanticType::Price, "Price");
                    ui.selectable_value(&mut app.forge_active_type, SemanticType::Ratio, "Ratio");
                    ui.selectable_value(&mut app.forge_active_type, SemanticType::Volume, "Volume");
                });
                
            ui.add_space(20.0);
            if ui.button("Save & Export Blueprint").clicked() {
                match node_graph::compiler::compile_graph(&app.forge_node_graph, &app.forge_blueprints) {
                    Ok(ast) => {
                        let blueprint = IndicatorBlueprint {
                            name: app.forge_active_blueprint_name.clone(),
                            semantic_type: app.forge_active_type,
                            is_custom: true,
                            outputs: vec![("Output".to_string(), ast)],
                        };
                        app.forge_blueprints.push(blueprint);
                        println!("Blueprint saved!");
                    }
                    Err(e) => {
                        println!("Compiler Error: {}", e);
                    }
                }
            }
            
            ui.add_space(20.0);
            if ui.button("+ Create New Blueprint").clicked() {
                app.forge_node_graph = node_graph::NodeGraphState::default();
                app.forge_active_blueprint_name = "New_Blueprint".to_string();
            }
            
            ui.add_space(20.0);
            ui.toggle_value(&mut app.forge_show_chart, "📈 Preview Chart");
        });
        
        ui.add_space(8.0);
        
        // Node Toolbox Dropdowns
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Add Node:").strong());
            
            ui.menu_button("Data Sources", |ui| {
                if ui.button("Close").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Source("Close".to_string())); ui.close_menu(); }
                if ui.button("Open").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Source("Open".to_string())); ui.close_menu(); }
                if ui.button("High").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Source("High".to_string())); ui.close_menu(); }
                if ui.button("Low").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Source("Low".to_string())); ui.close_menu(); }
                if ui.button("Volume").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Source("Volume".to_string())); ui.close_menu(); }
            });
            
            ui.menu_button("Math Operations", |ui| {
                if ui.button("Add (+)").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Operator("Add".to_string())); ui.close_menu(); }
                if ui.button("Sub (-)").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Operator("Sub".to_string())); ui.close_menu(); }
                if ui.button("Mul (*)").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Operator("Mul".to_string())); ui.close_menu(); }
                if ui.button("Div (/)").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Operator("Div".to_string())); ui.close_menu(); }
                if ui.button("Abs").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Indicator("Abs".to_string())); ui.close_menu(); }
                if ui.button("Constant").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Constant(1.0)); ui.close_menu(); }
            });
            
            ui.menu_button("Math Primitives", |ui| {
                if ui.button("StdDev").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Indicator("StdDev".to_string())); ui.close_menu(); }
                if ui.button("Delay").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Indicator("Delay".to_string())); ui.close_menu(); }
                if ui.button("TsMax").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Indicator("TsMax".to_string())); ui.close_menu(); }
                if ui.button("TsMin").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Indicator("TsMin".to_string())); ui.close_menu(); }
            });
            
            ui.menu_button("Indicators", |ui| {
                if ui.button("SMA").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Indicator("SMA".to_string())); ui.close_menu(); }
                if ui.button("EMA").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Indicator("EMA".to_string())); ui.close_menu(); }
                if ui.button("WMA").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Indicator("WMA".to_string())); ui.close_menu(); }
                if ui.button("RMA").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Indicator("RMA".to_string())); ui.close_menu(); }
                if ui.button("MACD (Template)").clicked() { spawn_macd_template(&mut app.forge_node_graph); ui.close_menu(); }
            });
            
            ui.menu_button("Blueprints", |ui| {
                ui.menu_button("Pre-Built Templates", |ui| {
                    for bp in app.forge_blueprints.iter().filter(|b| !b.is_custom) {
                        if ui.button(&bp.name).clicked() {
                            spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Blueprint(bp.name.clone()));
                            ui.close_menu();
                        }
                    }
                });
                
                ui.menu_button("Custom Indicators", |ui| {
                    let custom_bps: Vec<_> = app.forge_blueprints.iter().filter(|b| b.is_custom).collect();
                    if custom_bps.is_empty() {
                        ui.label("No custom indicators yet.");
                    } else {
                        for bp in custom_bps {
                            if ui.button(&bp.name).clicked() {
                                spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Blueprint(bp.name.clone()));
                                ui.close_menu();
                            }
                        }
                    }
                });
            });
            
            ui.menu_button("Logic & Triggers", |ui| {
                if ui.button("Cross Above").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Condition("Crossover".to_string())); ui.close_menu(); }
                if ui.button("Cross Below").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Condition("Crossunder".to_string())); ui.close_menu(); }
                if ui.button("Greater Than").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Condition("GreaterThan".to_string())); ui.close_menu(); }
                if ui.button("Less Than").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::Condition("LessThan".to_string())); ui.close_menu(); }
            });
            
            ui.menu_button("Outputs", |ui| {
                if ui.button("Output (Math)").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::OutputExpr); ui.close_menu(); }
                if ui.button("Output (Trigger)").clicked() { spawn_node(&mut app.forge_node_graph, node_graph::NodeKind::OutputCondition); ui.close_menu(); }
            });
        });
        
        ui.separator();
        
        // Breadcrumbs
        if !app.forge_graph_history.is_empty() {
            ui.horizontal(|ui| {
                if ui.button("⬅ Back to Parent").clicked() {
                    if let Some((parent_name, parent_graph)) = app.forge_graph_history.pop() {
                        app.forge_active_blueprint_name = parent_name;
                        app.forge_node_graph = parent_graph;
                    }
                }
                ui.label(egui::RichText::new(" > ").strong());
                for (name, _) in &app.forge_graph_history {
                    ui.label(name);
                    ui.label(egui::RichText::new(" > ").strong());
                }
                ui.label(egui::RichText::new(&app.forge_active_blueprint_name).color(egui::Color32::LIGHT_BLUE));
            });
            ui.separator();
        }
        
        // Render the node graph canvas
        if let Some(drill_bp) = node_graph::render_node_graph(ui, &mut app.forge_node_graph, &app.forge_blueprints) {
            // User double-clicked a blueprint node! Time to decompile and drill down.
            if let Some(bp) = app.forge_blueprints.iter().find(|b| b.name == drill_bp) {
                // Save current graph state
                let current_name = app.forge_active_blueprint_name.clone();
                let current_graph = app.forge_node_graph.clone();
                app.forge_graph_history.push((current_name, current_graph));
                
                // Decompile AST into new node graph
                let is_trigger = bp.semantic_type == SemanticType::Boolean;
                app.forge_node_graph = node_graph::decompiler::decompile_ast(&bp.outputs[0].1, is_trigger);
                app.forge_active_blueprint_name = bp.name.clone();
                app.forge_active_type = bp.semantic_type.clone();
            }
        }
        
    });

    if app.forge_show_chart {
        let mut show_chart = app.forge_show_chart;
        egui::Window::new("📈 Preview Chart")
            .open(&mut show_chart)
            .default_size(egui::vec2(800.0, 600.0))
            .show(ctx, |ui| {
                if let Some(engine) = &app.bitwise_engine {
                    let close_prices = &engine.cache.close;
                    if close_prices.is_empty() {
                        ui.label("No data available in sandbox.");
                        return;
                    }

                    // Compile the graph dynamically to determine how to plot it
                    let compiled_ast = node_graph::compiler::compile_graph(&app.forge_node_graph, &app.forge_blueprints).ok();
                    let current_ast_str = format!("{:?}", compiled_ast);
                    
                    if app.forge_last_ast_str != current_ast_str {
                        app.forge_last_ast_str = current_ast_str.clone();
                        app.main_chart.indicators.clear();
                        
                        if let Some(ast) = &compiled_ast {
                            let st = ast.semantic_type();
                            let mut unrolled = ast.clone();
                            tb_bitwise::engine::BitwiseEngine::unroll_macros(&mut unrolled);
                            
                            use egui_charts::studies::CustomIndicator;
                            use egui_charts::studies::IndicatorValue;
                            
                            if st == SemanticType::Boolean {
                                let bitset = engine.eval_boolean(&unrolled);
                                let mut ind = CustomIndicator::new("Trigger", Box::new(move |bars| {
                                    bars.iter().enumerate().map(|(i, _)| {
                                        let block = i / 64;
                                        let bit = i % 64;
                                        if block < bitset.len() && (bitset[block] & (1 << bit)) != 0 {
                                            IndicatorValue::Single(bars[i].low - (bars[i].close * 0.005)) // Draw point slightly below low
                                        } else {
                                            IndicatorValue::None
                                        }
                                    }).collect()
                                }))
                                .with_overlay(true)
                                .with_color(egui::Color32::YELLOW);
                                app.main_chart.indicators.add_indicator(Box::new(ind));
                            } else {
                                let vals = engine.eval_float(&unrolled);
                                let overlay = st == SemanticType::Price || st == SemanticType::Scalar;
                                let name = if overlay { "Forge Overlay" } else { "Forge Oscillator" };
                                let color = if overlay { egui::Color32::from_rgb(255, 100, 100) } else { egui::Color32::from_rgb(100, 200, 255) };
                                
                                let mut ind = CustomIndicator::new(name, Box::new(move |bars| {
                                    bars.iter().enumerate().map(|(i, _)| {
                                        if i < vals.len() {
                                            IndicatorValue::Single(vals[i])
                                        } else {
                                            IndicatorValue::None
                                        }
                                    }).collect()
                                }))
                                .with_overlay(overlay)
                                .with_color(color);
                                app.main_chart.indicators.add_indicator(Box::new(ind));
                            }
                            
                            // Re-calculate the new indicators on existing bar data
                            let bars = app.main_chart.chart.data().bars.clone();
                            app.main_chart.indicators.calculate_all(&bars);
                        }
                    }
                    
                    app.main_chart.update();
                    app.main_chart.show(ui);
                } else {
                    ui.label("Waiting for data. Please go to the Data Sandbox to generate or load data first.");
                }
            });
        app.forge_show_chart = show_chart;
    }
}

fn spawn_node(state: &mut node_graph::NodeGraphState, kind: node_graph::NodeKind) {
    let mut inline_params = Vec::new();
    
    // Auto-initialize parameters for indicators that need them
    if let node_graph::NodeKind::Indicator(ref name) = kind {
        if name == "SMA" || name == "EMA" || name == "WMA" || name == "RMA" {
            inline_params.push(("LEN".to_string(), 14.0));
        }
    } else if let node_graph::NodeKind::Blueprint(ref name) = kind {
        if name == "BOLL" {
            inline_params.push(("LEN".to_string(), 20.0));
            inline_params.push(("MULT".to_string(), 2.0));
        } else if name == "MACD" {
            inline_params.push(("FAST".to_string(), 12.0));
            inline_params.push(("SLOW".to_string(), 26.0));
        } else if name == "RSI" || name == "ATR" {
            inline_params.push(("LEN".to_string(), 14.0));
        }
    }

    state.nodes.push(node_graph::Node {
        id: state.next_node_id,
        kind,
        pos: egui::vec2(100.0, 100.0).to_pos2() - state.pan / state.zoom, // Spawn at center-ish
        inline_params,
    });
    state.next_node_id += 1;
}

fn spawn_macd_template(state: &mut node_graph::NodeGraphState) {
    let ema1_id = state.next_node_id;
    state.nodes.push(node_graph::Node {
        id: ema1_id,
        kind: node_graph::NodeKind::Indicator("EMA".to_string()),
        pos: egui::vec2(50.0, 50.0).to_pos2() - state.pan / state.zoom,
        inline_params: vec![("LEN".to_string(), 12.0)],
    });
    state.next_node_id += 1;

    let ema2_id = state.next_node_id;
    state.nodes.push(node_graph::Node {
        id: ema2_id,
        kind: node_graph::NodeKind::Indicator("EMA".to_string()),
        pos: egui::vec2(50.0, 150.0).to_pos2() - state.pan / state.zoom,
        inline_params: vec![("LEN".to_string(), 26.0)],
    });
    state.next_node_id += 1;

    let sub_id = state.next_node_id;
    state.nodes.push(node_graph::Node {
        id: sub_id,
        kind: node_graph::NodeKind::Operator("Sub".to_string()),
        pos: egui::vec2(250.0, 100.0).to_pos2() - state.pan / state.zoom,
        inline_params: vec![],
    });
    state.next_node_id += 1;

    let out_id = state.next_node_id;
    state.nodes.push(node_graph::Node {
        id: out_id,
        kind: node_graph::NodeKind::OutputExpr,
        pos: egui::vec2(400.0, 100.0).to_pos2() - state.pan / state.zoom,
        inline_params: vec![],
    });
    state.next_node_id += 1;

    state.wires.push(node_graph::Wire {
        from_node: ema1_id,
        from_pin: 0,
        to_node: sub_id,
        to_pin: 0,
    });

    state.wires.push(node_graph::Wire {
        from_node: ema2_id,
        from_pin: 0,
        to_node: sub_id,
        to_pin: 1,
    });
}
