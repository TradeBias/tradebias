use eframe::egui::{self, Color32, Rounding, Stroke, Margin};

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // 1. Shapes & Spacing
    style.visuals.window_corner_radius = egui::CornerRadius::same(8);
    style.visuals.menu_corner_radius = egui::CornerRadius::same(6);
    style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(4);
    style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(4);
    style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(4);
    style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(4);
    style.visuals.widgets.open.corner_radius = egui::CornerRadius::same(4);
    
    style.spacing.window_margin = Margin::same(12);
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);

    // 2. Colors - Backgrounds & Panels
    // #121212
    let bg_color = Color32::from_rgb(18, 18, 18);
    // #242424
    let panel_color = Color32::from_rgb(36, 36, 36);
    // #333333
    let border_color = Color32::from_rgb(51, 51, 51);
    
    style.visuals.window_fill = Color32::from_rgb(30, 30, 32); // Slightly distinct modal background
    style.visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(94, 106, 210)); // Indigo border for modals!
    style.visuals.panel_fill = bg_color;
    style.visuals.extreme_bg_color = Color32::from_rgb(26, 26, 26); // text edits, etc.

    // 3. Colors - Accents & Text
    // #5E6AD2
    let accent_color = Color32::from_rgb(94, 106, 210);
    let hover_accent = Color32::from_rgb(114, 126, 230);
    
    let text_primary = Color32::from_rgb(234, 234, 234);
    let text_secondary = Color32::from_rgb(136, 136, 136);

    style.visuals.selection.bg_fill = accent_color;
    style.visuals.selection.stroke = Stroke::new(1.0, accent_color);
    
    style.visuals.override_text_color = Some(text_primary);
    
    // Default widget looks
    style.visuals.widgets.noninteractive.bg_fill = panel_color;
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border_color);
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text_secondary);

    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(42, 42, 42);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, border_color);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text_primary);

    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(52, 52, 52);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, hover_accent);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, text_primary);

    style.visuals.widgets.active.bg_fill = hover_accent;
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.0, accent_color);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    
    // 4. Custom colors for positive/negative (could be used around the app)
    style.visuals.error_fg_color = Color32::from_rgb(251, 113, 133); // Soft rose
    style.visuals.warn_fg_color = Color32::from_rgb(251, 191, 36);  // Soft amber

    ctx.set_style(style);
}

// Global constants for custom plotting / node graph elements
pub const SUCCESS_GREEN: Color32 = Color32::from_rgb(52, 211, 153);
pub const ERROR_RED: Color32 = Color32::from_rgb(251, 113, 133);
pub const ACCENT_INDIGO: Color32 = Color32::from_rgb(94, 106, 210);
pub const WINDOW_FILL: Color32 = Color32::from_rgb(30, 30, 32);
pub const RAIL_FILL: Color32 = Color32::from_rgb(26, 26, 29);

pub fn get_charts_theme() -> egui_charts::theme::Theme {
    let mut theme = egui_charts::theme::Theme::dark();
    
    // Modify semantic tokens
    theme.semantic.chart.bg = Color32::from_rgb(18, 18, 18);
    theme.semantic.chart.bg_axis = Color32::from_rgb(18, 18, 18);
    theme.semantic.chart.grid_line = Color32::from_rgb(51, 51, 51);
    theme.semantic.chart.grid_line_major = Color32::from_rgb(61, 61, 61);
    theme.semantic.chart.axis_text = Color32::from_rgb(136, 136, 136);
    theme.semantic.chart.crosshair_line = Color32::from_rgba_unmultiplied(94, 106, 210, 150);
    
    theme.semantic.chart.candle_up = SUCCESS_GREEN;
    theme.semantic.chart.candle_down = ERROR_RED;
    
    // Volume colors
    theme.semantic.chart.volume_up = SUCCESS_GREEN;
    theme.semantic.chart.volume_down = ERROR_RED;
    
    // Rebuild components from semantic tokens
    theme.components = egui_charts::theme::ComponentStyles::from_semantic(&theme.semantic);
    
    theme
}
