import re

with open(r'E:\repo\app-main\tb_ui\src\tabs\alpha_foundry.rs', 'r', encoding='utf-8') as f:
    code = f.read()

# We need to turn the main block into:
# pub fn render(app: &mut TradingApp, ctx: &egui::Context, outer_ui: &mut egui::Ui) {
#    // 1. Non-blocking receiver check for Phase 1
#    ...
#    // Top Bar for Actions
#    ...
#    // Rendering tree
#    let mut tree = std::mem::replace(&mut app.foundry_tree, egui_tiles::Tree::empty("tmp"));
#    let mut behavior = crate::tabs::foundry_layout::FoundryBehavior { app };
#    tree.ui(&mut behavior, outer_ui);
#    app.foundry_tree = tree;
# }
# 
# pub fn render_settings(app: &mut TradingApp, ui: &mut egui::Ui) { ... }
# pub fn render_leaderboard(app: &mut TradingApp, ui: &mut egui::Ui) { ... }
# pub fn render_stats(app: &mut TradingApp, ui: &mut egui::Ui) { ... }

# Find the start of the `if app.show_alpha_settings {` block. This is the settings drawer.
settings_start = code.find('if app.show_alpha_settings {')
settings_end = code.find('    if start_clicked {')

settings_code_block = code[settings_start:settings_end]

# Extract the inner part of settings grid
settings_inner_start = settings_code_block.find('egui::Grid::new("alpha_settings_grid")')
settings_inner_end = settings_code_block.rfind('            }); // End SidePanel Drawer')

settings_inner = settings_code_block[settings_inner_start:settings_inner_end]
settings_inner = "        egui::ScrollArea::vertical().show(ui, |ui| {\n            " + settings_inner + "    });"


# Find the start of the start logic (line 332 approx)
start_logic_start = code.find('    if start_clicked {')
start_logic_end = code.find('    if let Some(metrics) = &app.latest_metrics {')
start_logic = code[start_logic_start:start_logic_end]
start_logic = start_logic.replace('if start_clicked {', 'if true {')


# Find Stats rendering
stats_start = code.find('    if let Some(metrics) = &app.latest_metrics {')
stats_end = code.find('    let is_wide = ctx.screen_rect().width() > 1200.0;')
stats_code_block = code[stats_start:stats_end]
stats_inner_start = stats_code_block.find('            ui.vertical(|ui| {')
stats_inner_end = stats_code_block.rfind('        });\n    }')
stats_inner = stats_code_block[:stats_inner_start-1] + stats_code_block[stats_inner_start:stats_inner_end] + "    }\n"
stats_inner = stats_inner.replace('egui::TopBottomPanel::bottom("foundry_stats_panel").show_inside(outer_ui, |ui| {', 'if true {')


# Find Leaderboard rendering
leaderboard_start = code.find('    let central_frame = egui::Frame::default().fill(ctx.style().visuals.panel_fill).inner_margin(16.0);')
leaderboard_end = len(code)
leaderboard_code_block = code[leaderboard_start:leaderboard_end]
leaderboard_inner_start = leaderboard_code_block.find('        ui.heading("Alpha Foundry - Leaderboard (Phase 1)");')
leaderboard_inner_end = leaderboard_code_block.rfind('    });\n}')
leaderboard_inner = leaderboard_code_block[leaderboard_inner_start:leaderboard_inner_end]

# Keep Top Actions
top_actions_start = code.find('    let top_frame = egui::Frame::default()')
top_actions_end = code.find('    outer_ui.add_space(8.0);')
top_actions = code[top_actions_start:top_actions_end]

# Generate the new code
new_code = code[:top_actions_start] + top_actions + """
    
    let mut tree = std::mem::replace(&mut app.foundry_tree, egui_tiles::Tree::empty("tmp"));
    let mut behavior = crate::tabs::foundry_layout::FoundryBehavior { app };
    tree.ui(&mut behavior, outer_ui);
    app.foundry_tree = tree;
}

pub fn render_settings(app: &mut crate::state::TradingApp, ui: &mut egui::Ui) {
""" + settings_inner + """
    
""" + start_logic + """
}

pub fn render_stats(app: &mut crate::state::TradingApp, ui: &mut egui::Ui) {
""" + stats_inner + """
}

pub fn render_leaderboard(app: &mut crate::state::TradingApp, ui: &mut egui::Ui) {
""" + leaderboard_inner + """
}
"""

with open(r'E:\repo\app-main\tb_ui\src\tabs\alpha_foundry.rs', 'w', encoding='utf-8') as f:
    f.write(new_code)
