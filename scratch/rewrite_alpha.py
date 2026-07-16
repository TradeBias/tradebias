import re

with open('tb_ui/src/tabs/alpha_foundry.rs', 'r', encoding='utf-8') as f:
    code = f.read()

# We want to replace the SidePanel completely.
# It starts at: egui::SidePanel::left("alpha_foundry_controls").default_width(300.0).show(ctx, |ui| {
# and ends at the closing brace before: if let Some(metrics) = &app.latest_metrics {

# Let's find the start and end index.
start_idx = code.find('    egui::SidePanel::left("alpha_foundry_controls")')
end_idx = code.find('    if let Some(metrics) = &app.latest_metrics {')

if start_idx == -1 or end_idx == -1:
    print("Could not find start or end index")
    exit(1)

old_block = code[start_idx:end_idx]
print(f"Replacing {len(old_block)} characters")
