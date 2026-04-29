use egui::{Color32, Painter, Pos2, Rect, Stroke};

use crate::common::app::AppState;

/// Draw an in-canvas status bar overlay at the bottom of the schematic area
pub(super) fn draw_canvas_status_bar(
    painter: &Painter,
    bounds: Rect,
    state: &AppState,
    hover_pos: Option<Pos2>,
) {
    let bar_height = 22.0;
    let bar_rect = Rect::from_min_max(
        Pos2::new(bounds.min.x, bounds.max.y - bar_height),
        bounds.max,
    );

    // Semi-transparent background
    painter.rect_filled(
        bar_rect,
        0.0,
        Color32::from_rgba_unmultiplied(30, 30, 35, 220),
    );

    // Top border line
    painter.line_segment(
        [bar_rect.left_top(), bar_rect.right_top()],
        Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
    );

    let text_color = Color32::from_rgb(180, 180, 190);
    let font = egui::FontId::proportional(11.0);
    let mut x = bounds.min.x + 10.0;
    let y = bounds.max.y - bar_height / 2.0;

    // Cursor position in schematic coordinates
    if let Some(cursor) = hover_pos {
        let schematic_x =
            ((cursor.x - bounds.min.x) as f64 - state.schematic.pan.0) / state.schematic.zoom;
        let schematic_y =
            ((cursor.y - bounds.min.y) as f64 - state.schematic.pan.1) / state.schematic.zoom;

        let cursor_text = format!("({:.0}, {:.0})", schematic_x, schematic_y);
        painter.text(
            Pos2::new(x, y),
            egui::Align2::LEFT_CENTER,
            &cursor_text,
            font.clone(),
            text_color,
        );
        x += 100.0;
    }

    // Separator
    painter.line_segment(
        [
            Pos2::new(x, bar_rect.min.y + 4.0),
            Pos2::new(x, bar_rect.max.y - 4.0),
        ],
        Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
    );
    x += 12.0;

    // Zoom level
    let zoom_pct = (state.schematic.zoom * 100.0) as i32;
    let zoom_text = format!("{}%", zoom_pct);
    painter.text(
        Pos2::new(x, y),
        egui::Align2::LEFT_CENTER,
        &zoom_text,
        font.clone(),
        text_color,
    );
    x += 60.0;

    // Separator
    painter.line_segment(
        [
            Pos2::new(x, bar_rect.min.y + 4.0),
            Pos2::new(x, bar_rect.max.y - 4.0),
        ],
        Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
    );
    x += 12.0;

    // Grid size
    let grid_text = format!("Grid: {}", state.schematic.grid_size);
    painter.text(
        Pos2::new(x, y),
        egui::Align2::LEFT_CENTER,
        &grid_text,
        font.clone(),
        text_color,
    );

    // Right side: selection info
    let comp_count = state.schematic.selection.components.len();
    let wire_count = state.schematic.selection.wires.len();
    let selection_text = if comp_count > 0 || wire_count > 0 {
        format!("{} comp, {} wire", comp_count, wire_count)
    } else {
        "Ready".to_string()
    };

    painter.text(
        Pos2::new(bounds.max.x - 10.0, y),
        egui::Align2::RIGHT_CENTER,
        &selection_text,
        font,
        text_color,
    );
}
