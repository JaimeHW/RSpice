//! Shared viewer surface colors, derived from the active design tokens so
//! every analysis viewer follows the selected theme.

use egui::Color32;

use crate::ui::tokens::active_palette;

/// Shared background fill for viewer control header bars.
///
/// Matches the panel chrome so control bars blend with the shell.
pub fn viewer_header_bg_color() -> Color32 {
    active_palette().bg_panel
}

/// Shared chart plotting surface fill for viewer graphs (document well).
pub fn viewer_chart_bg_color() -> Color32 {
    active_palette().canvas_bg
}
