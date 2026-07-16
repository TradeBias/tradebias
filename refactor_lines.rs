use std::fs;

fn main() {
    let code = fs::read_to_string("tb_ui/src/tabs/alpha_foundry_temp.rs").unwrap();
    let lines: Vec<&str> = code.lines().collect();
    
    let top_actions = lines[0..56].join("\n");
    let settings_code = lines[57..334].join("\n");
    let start_logic = lines[335..506].join("\n");
    let stats_code = lines[507..567].join("\n");
    let leaderboard_code = lines[587..902].join("\n");
    
    let settings_inner = settings_code.replace("egui::SidePanel::left(\"alpha_settings_drawer\")", "if false {")
                                      .replace(".resizable(true)", "")
                                      .replace(".default_width(350.0)", "")
                                      .replace(".frame(drawer_frame)", "")
                                      .replace(".show_inside(outer_ui, |ui| {", "    egui::ScrollArea::vertical().show(ui, |ui| {");
                                      
    let stats_inner = stats_code.replace("egui::TopBottomPanel::bottom(\"foundry_stats_panel\").show_inside(outer_ui, |ui| {", "if true {");
    
    let leaderboard_inner = leaderboard_code.replace("egui::CentralPanel::default().frame(central_frame).show_inside(outer_ui, |ui| {", "if true {");
    
    let mut new_code = String::new();
    new_code.push_str(&top_actions);
    new_code.push_str("\n    let mut tree = std::mem::replace(&mut app.foundry_tree, egui_tiles::Tree::empty(\"tmp\"));\n");
    new_code.push_str("    let mut behavior = crate::tabs::foundry_layout::FoundryBehavior { app };\n");
    new_code.push_str("    tree.ui(&mut behavior, outer_ui);\n");
    new_code.push_str("    app.foundry_tree = tree;\n");
    new_code.push_str("}\n\n");
    
    new_code.push_str("pub fn render_settings(app: &mut crate::state::TradingApp, ui: &mut egui::Ui, start_clicked: &mut bool) {\n");
    new_code.push_str(&settings_inner);
    new_code.push_str("\n");
    new_code.push_str(&start_logic.replace("if start_clicked {", "if *start_clicked {"));
    new_code.push_str("\n}\n\n");
    
    new_code.push_str("pub fn render_stats(app: &mut crate::state::TradingApp, ui: &mut egui::Ui) {\n");
    new_code.push_str(&stats_inner.replace("outer_ui", "ui"));
    new_code.push_str("\n}\n\n");
    
    new_code.push_str("pub fn render_leaderboard(app: &mut crate::state::TradingApp, ui: &mut egui::Ui) {\n");
    new_code.push_str(&leaderboard_inner.replace("outer_ui", "ui"));
    new_code.push_str("\n}\n");
    
    fs::write("tb_ui/src/tabs/alpha_foundry.rs", new_code).unwrap();
}
