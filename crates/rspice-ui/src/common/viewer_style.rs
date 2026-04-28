use egui::Color32;

/// Shared background fill for viewer control header bars.
///
/// Keep this aligned with the viewer workspace strip/chip surface so control
/// bars blend with the surrounding panel frame.
pub fn viewer_header_bg_color() -> Color32 {
    Color32::from_rgb(23, 26, 32)
}

/// Shared chart plotting surface fill for viewer graphs.
pub fn viewer_chart_bg_color() -> Color32 {
    Color32::from_rgb(15, 17, 21)
}

