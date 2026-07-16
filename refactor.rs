use std::fs;

fn main() {
    let code = fs::read_to_string("tb_ui/src/tabs/alpha_foundry_temp.rs").unwrap();
    
    // Find the settings block
    let settings_start = code.find("    if app.show_alpha_settings {").unwrap();
    let start_logic_start = code.find("    if start_clicked {").unwrap();
    
    let settings_code = &code[settings_start..start_logic_start];
    let grid_start = settings_code.find("egui::Grid::new").unwrap();
    let grid_end = settings_code.rfind("}); // End SidePanel Drawer").unwrap();
    let settings_inner = format!("        egui::ScrollArea::vertical().show(ui, |ui| {{\n            {}    }});", &settings_code[grid_start..grid_end]);
    
    let stats_start = code.find("    if let Some(metrics) = &app.latest_metrics {").unwrap();
    let start_logic = &code[start_logic_start..stats_start];
    let start_logic_inner = start_logic.replace("if start_clicked {", "if *start_clicked {");
    
    let is_wide_start = code.find("    let is_wide = ctx.screen_rect().width() > 1200.0;").unwrap();
    let stats_code = &code[stats_start..is_wide_start];
    let stats_inner = stats_code.replace("egui::TopBottomPanel::bottom(\"foundry_stats_panel\").show_inside(outer_ui, |ui| {", "if true {");
    
    let leaderboard_start = code.find("    let central_frame = egui::Frame::default()").unwrap();
    let leaderboard_code = &code[leaderboard_start..];
    let leaderboard_inner_start = leaderboard_code.find("        ui.heading(\"Alpha Foundry - Leaderboard (Phase 1)\");").unwrap();
    let leaderboard_inner_end = leaderboard_code.rfind("    });\n}").unwrap();
    let leaderboard_inner = &leaderboard_code[leaderboard_inner_start..leaderboard_inner_end];
    
    let top_actions_start = code.find("    // Top Bar for Actions").unwrap();
    let top_actions_end_idx = code.find("outer_ui.add_space(8.0);").unwrap();
    let top_actions_end = code[top_actions_end_idx..].find('\n').unwrap() + top_actions_end_idx + 1;
    let top_actions = &code[top_actions_start..top_actions_end];
    
    let head = &code[..top_actions_start];
    
    let mut new_code = String::new();
    new_code.push_str(head);
    new_code.push_str(top_actions);
    new_code.push_str("\n    let mut tree = std::mem::replace(&mut app.foundry_tree, egui_tiles::Tree::empty(\"tmp\"));\n");
    new_code.push_str("    let mut behavior = crate::tabs::foundry_layout::FoundryBehavior { app };\n");
    new_code.push_str("    tree.ui(&mut behavior, outer_ui);\n");
    new_code.push_str("    app.foundry_tree = tree;\n");
    new_code.push_str("}\n\n");
    
    new_code.push_str("pub fn render_settings(app: &mut crate::state::TradingApp, ui: &mut egui::Ui, start_clicked: &mut bool) {\n");
    new_code.push_str(&settings_inner);
    new_code.push_str("\n");
    new_code.push_str(&start_logic_inner);
    new_code.push_str("}\n\n");
    
    new_code.push_str("pub fn render_stats(app: &mut crate::state::TradingApp, ui: &mut egui::Ui) {\n");
    new_code.push_str(&stats_inner.replace("outer_ui", "ui"));
    new_code.push_str("}\n\n");
    
    new_code.push_str("pub fn render_leaderboard(app: &mut crate::state::TradingApp, ui: &mut egui::Ui) {\n");
    new_code.push_str(&leaderboard_inner.replace("outer_ui", "ui"));
    new_code.push_str("\n}\n");
    
    fs::write("tb_ui/src/tabs/alpha_foundry.rs", new_code).unwrap();
}
