//! Pan and zoom.
//!
//! Viewport movement: wheel and pinch zoom about the pointer, drag panning,
//! and the fit/zoom-to-selection commands.

use egui::{Pos2, Rect, Response, Ui, Vec2};

use crate::workbench::app_state::AppState;

const SCHEMATIC_ZOOM_MIN: f64 = 0.25;
const SCHEMATIC_ZOOM_MAX: f64 = 8.0;
/// Convert high-resolution wheel/trackpad deltas into a continuous zoom.
///
/// A fixed multiplier per frame makes a precision trackpad emit the same
/// ten-percent jump for a one-pixel gesture as for a full mouse-wheel notch,
/// then compounds those jumps while the OS coalesces events. This exponential
/// curve preserves direction and accumulated motion without allowing one
/// physical gesture—often delivered as several platform events—to traverse
/// most of the zoom range. The per-frame cap rejects pathological driver
/// spikes.
const WHEEL_ZOOM_EXPONENT_PER_POINT: f64 = 0.00005;
const WHEEL_ZOOM_MAX_DELTA_PER_FRAME: f64 = 240.0;

pub(super) fn primary_pan_modifier_down(ui: &Ui) -> bool {
    ui.input(|input| input.modifiers.alt || input.key_down(egui::Key::Space))
}

pub(super) fn primary_pan_gesture_active(ui: &Ui, response: &Response) -> bool {
    let (alt, space, primary_down) = ui.input(|input| {
        (
            input.modifiers.alt,
            input.key_down(egui::Key::Space),
            input.pointer.primary_down(),
        )
    });
    let primary_owned = (response.is_pointer_button_down_on() && primary_down)
        || response.dragged_by(egui::PointerButton::Primary);
    (alt && primary_owned)
        || (space && (primary_owned || response.clicked_by(egui::PointerButton::Primary)))
}

pub(super) fn handle_viewport_navigation(
    ui: &Ui,
    response: &Response,
    available: Rect,
    state: &mut AppState,
) {
    // Middle-button pan follows the raw pointer delta from the very first
    // event: egui's click-vs-drag threshold would swallow the first few
    // pixels as a dead zone, and the canvas has no competing middle-click
    // action. Space+primary and Alt+primary are the mockup's modeless pan
    // escape hatches while an authoring tool remains armed.
    let middle_pan = response.is_pointer_button_down_on() && ui.input(|i| i.pointer.middle_down());
    let modified_primary_pan = response.is_pointer_button_down_on()
        && ui.input(|input| input.pointer.primary_down())
        && primary_pan_modifier_down(ui);
    if middle_pan || modified_primary_pan {
        let delta = ui.input(|i| i.pointer.delta());
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
        let (scroll, shift) = ui.input(|i| (i.smooth_scroll_delta, i.modifiers.shift));
        if shift {
            let horizontal = if scroll.y != 0.0 { scroll.y } else { scroll.x };
            if horizontal != 0.0 {
                apply_horizontal_scroll_pan(&mut state.schematic.pan, horizontal);
            }
        } else if scroll.y != 0.0
            && let Some(cursor_pos) = response.hover_pos()
        {
            apply_zoom_about(
                &mut state.schematic.zoom,
                &mut state.schematic.pan,
                available,
                cursor_pos,
                wheel_zoom_factor(scroll.y),
            );
        }
    }
}

fn wheel_zoom_factor(scroll_delta_y: f32) -> f64 {
    let delta = f64::from(scroll_delta_y).clamp(
        -WHEEL_ZOOM_MAX_DELTA_PER_FRAME,
        WHEEL_ZOOM_MAX_DELTA_PER_FRAME,
    );
    (delta * WHEEL_ZOOM_EXPONENT_PER_POINT).exp()
}

fn apply_horizontal_scroll_pan(pan: &mut (f64, f64), delta: f32) {
    pan.0 += f64::from(delta);
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
    let new_zoom = (old_zoom * zoom_factor).clamp(SCHEMATIC_ZOOM_MIN, SCHEMATIC_ZOOM_MAX);

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

    #[test]
    fn shifted_wheel_changes_horizontal_pan_only() {
        let mut pan = (4.0, -8.0);
        apply_horizontal_scroll_pan(&mut pan, 12.5);
        assert_eq!(pan, (16.5, -8.0));
    }

    #[test]
    fn wheel_zoom_is_continuous_bounded_and_reversible() {
        assert_eq!(wheel_zoom_factor(0.0), 1.0);

        let notch_in = wheel_zoom_factor(120.0);
        let notch_out = wheel_zoom_factor(-120.0);
        assert!(
            (1.0..1.007).contains(&notch_in),
            "one wheel notch should be a controlled step, got {notch_in}"
        );
        assert!((notch_in * notch_out - 1.0).abs() < 1e-12);

        let small_delta = wheel_zoom_factor(1.0);
        assert!(
            (1.0..1.001).contains(&small_delta),
            "precision scrolling should remain proportional, got {small_delta}"
        );

        assert_eq!(
            wheel_zoom_factor(10_000.0),
            wheel_zoom_factor(WHEEL_ZOOM_MAX_DELTA_PER_FRAME as f32),
        );
    }

    #[test]
    fn schematic_zoom_uses_the_mockup_twenty_five_to_eight_hundred_percent_contract() {
        let available = Rect::from_min_size(pos2(0.0, 0.0), vec2(400.0, 300.0));
        let focus = available.center();
        let mut pan = (0.0, 0.0);
        let mut zoom = 1.0;

        apply_zoom_about(&mut zoom, &mut pan, available, focus, 100.0);
        assert_eq!(zoom, SCHEMATIC_ZOOM_MAX);

        apply_zoom_about(&mut zoom, &mut pan, available, focus, 0.001);
        assert_eq!(zoom, SCHEMATIC_ZOOM_MIN);
    }
}
