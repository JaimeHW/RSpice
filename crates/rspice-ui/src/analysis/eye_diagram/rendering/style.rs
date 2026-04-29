use egui::Color32;

use crate::common::viewer_style::viewer_chart_bg_color;

pub(super) fn chart_bg_color() -> Color32 {
    viewer_chart_bg_color()
}

pub(super) fn grid_color() -> Color32 {
    Color32::from_rgb(40, 45, 55)
}

pub(super) fn trace_color() -> Color32 {
    Color32::from_rgba_unmultiplied(50, 220, 100, 180)
}

pub(super) fn mask_pass_color() -> Color32 {
    Color32::from_rgba_unmultiplied(0, 150, 0, 100)
}

pub(super) fn mask_fail_color() -> Color32 {
    Color32::from_rgba_unmultiplied(200, 0, 0, 100)
}

pub(super) fn mask_outline_color() -> Color32 {
    Color32::from_rgb(255, 200, 0)
}

pub(super) fn center_line_color() -> Color32 {
    Color32::from_rgb(80, 85, 95)
}

pub(super) fn text_color() -> Color32 {
    Color32::from_rgb(180, 185, 195)
}

pub(super) fn panel_bg_color() -> Color32 {
    Color32::from_rgb(30, 33, 40)
}

pub(super) fn panel_border_color() -> Color32 {
    Color32::from_rgb(60, 65, 75)
}

pub(super) fn highlight_color() -> Color32 {
    Color32::from_rgb(100, 200, 255)
}

pub(super) fn cursor1_color() -> Color32 {
    Color32::from_rgb(255, 235, 59)
}

pub(super) fn cursor2_color() -> Color32 {
    Color32::from_rgb(76, 175, 80)
}

pub(super) fn marker_color(index: usize) -> Color32 {
    const PALETTE: [Color32; 8] = [
        Color32::from_rgb(59, 130, 246),
        Color32::from_rgb(16, 185, 129),
        Color32::from_rgb(249, 115, 22),
        Color32::from_rgb(139, 92, 246),
        Color32::from_rgb(236, 72, 153),
        Color32::from_rgb(234, 179, 8),
        Color32::from_rgb(20, 184, 166),
        Color32::from_rgb(239, 68, 68),
    ];
    PALETTE[index % PALETTE.len()]
}

// =============================================================================
