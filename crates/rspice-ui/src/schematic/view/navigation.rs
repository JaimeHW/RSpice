use egui::{Pos2, Rect, Response, Ui, Vec2};

use crate::common::app::AppState;

pub(super) fn handle_viewport_navigation(
    ui: &Ui,
    response: &Response,
    available: Rect,
    state: &mut AppState,
) {
    // Middle-button pan follows the raw pointer delta from the very first
    // event: egui's click-vs-drag threshold would swallow the first few
    // pixels as a dead zone, and the canvas has no competing middle-click
    // action. Shift+primary keeps the threshold so clicks stay clicks.
    let middle_pan = response.is_pointer_button_down_on() && ui.input(|i| i.pointer.middle_down());
    let shift_pan =
        response.dragged_by(egui::PointerButton::Primary) && ui.input(|i| i.modifiers.shift);
    if middle_pan || shift_pan {
        let delta = if middle_pan {
            ui.input(|i| i.pointer.delta())
        } else {
            response.drag_delta()
        };
        apply_pan_delta(&mut state.schematic.pan, delta);
    }

    if let Some(touch) = ui.input(|i| i.multi_touch())
        && available.contains(touch.start_pos)
    {
        apply_pan_delta(&mut state.schematic.pan, touch.translation_delta);
        if (touch.zoom_delta - 1.0).abs() > f32::EPSILON {
            let focus = response.hover_pos().unwrap_or(touch.start_pos);
            apply_zoom_about(
                &mut state.schematic.zoom,
                &mut state.schematic.pan,
                available,
                focus,
                touch.zoom_delta as f64,
            );
        }
    }

    // Cursor-centered zoom, matching professional CAD tools.
    if response.hovered() {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0
            && let Some(cursor_pos) = response.hover_pos()
        {
            let zoom_factor = if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
            apply_zoom_about(
                &mut state.schematic.zoom,
                &mut state.schematic.pan,
                available,
                cursor_pos,
                zoom_factor,
            );
        }
    }
}

fn apply_pan_delta(pan: &mut (f64, f64), delta: Vec2) {
    pan.0 += delta.x as f64;
    pan.1 += delta.y as f64;
}

fn apply_zoom_about(
    zoom: &mut f64,
    pan: &mut (f64, f64),
    available: Rect,
    focus_pos: Pos2,
    zoom_factor: f64,
) {
    if !zoom_factor.is_finite() || zoom_factor <= 0.0 {
        return;
    }

    let old_zoom = *zoom;
    let new_zoom = (old_zoom * zoom_factor).clamp(0.1, 10.0);

    let focus_schematic_x = (focus_pos.x as f64 - available.min.x as f64 - pan.0) / old_zoom;
    let focus_schematic_y = (focus_pos.y as f64 - available.min.y as f64 - pan.1) / old_zoom;

    *zoom = new_zoom;
    pan.0 = focus_pos.x as f64 - available.min.x as f64 - focus_schematic_x * new_zoom;
    pan.1 = focus_pos.y as f64 - available.min.y as f64 - focus_schematic_y * new_zoom;
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{pos2, vec2};

    #[test]
    fn pan_delta_moves_viewport_by_screen_delta() {
        let mut pan = (2.0, -3.0);
        apply_pan_delta(&mut pan, vec2(12.5, -4.0));

        assert_eq!(pan, (14.5, -7.0));
    }

    #[test]
    fn zoom_about_screen_position_keeps_world_point_under_focus() {
        let available = Rect::from_min_size(pos2(100.0, 50.0), vec2(400.0, 300.0));
        let focus = pos2(260.0, 180.0);
        let mut zoom = 2.0;
        let mut pan = (20.0, 30.0);

        let before = (
            (focus.x as f64 - available.min.x as f64 - pan.0) / zoom,
            (focus.y as f64 - available.min.y as f64 - pan.1) / zoom,
        );
        apply_zoom_about(&mut zoom, &mut pan, available, focus, 1.5);
        let after = (
            (focus.x as f64 - available.min.x as f64 - pan.0) / zoom,
            (focus.y as f64 - available.min.y as f64 - pan.1) / zoom,
        );

        assert_eq!(zoom, 3.0);
        assert!((before.0 - after.0).abs() < 1e-12);
        assert!((before.1 - after.1).abs() < 1e-12);
    }
}
