use egui::{Rect, Response, Ui};

use crate::common::app::AppState;

pub(super) fn handle_viewport_navigation(
    ui: &Ui,
    response: &Response,
    available: Rect,
    state: &mut AppState,
) {
    if response.dragged_by(egui::PointerButton::Middle)
        || (response.dragged_by(egui::PointerButton::Primary) && ui.input(|i| i.modifiers.shift))
    {
        let delta = response.drag_delta();
        state.schematic.pan.0 += delta.x as f64;
        state.schematic.pan.1 += delta.y as f64;
    }

    // Cursor-centered zoom, matching professional CAD tools.
    if response.hovered() {
        let scroll = ui.input(|i| i.raw_scroll_delta.y);
        if scroll != 0.0
            && let Some(cursor_pos) = response.hover_pos()
        {
            let zoom_factor = if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
            let old_zoom = state.schematic.zoom;
            let new_zoom = (old_zoom * zoom_factor).clamp(0.1, 10.0);

            let cursor_schematic_x =
                (cursor_pos.x as f64 - available.min.x as f64 - state.schematic.pan.0) / old_zoom;
            let cursor_schematic_y =
                (cursor_pos.y as f64 - available.min.y as f64 - state.schematic.pan.1) / old_zoom;

            state.schematic.zoom = new_zoom;
            state.schematic.pan.0 =
                cursor_pos.x as f64 - available.min.x as f64 - cursor_schematic_x * new_zoom;
            state.schematic.pan.1 =
                cursor_pos.y as f64 - available.min.y as f64 - cursor_schematic_y * new_zoom;
        }
    }
}
