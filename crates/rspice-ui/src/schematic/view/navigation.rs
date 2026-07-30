//! Pan and zoom.
//!
//! Viewport movement: wheel and pinch zoom about the pointer, drag panning,
//! and the fit/zoom-to-selection commands.

use egui::{Event, MouseWheelUnit, Pos2, Rect, Response, TouchPhase, Ui, Vec2};

use crate::workbench::app_state::AppState;

const SCHEMATIC_ZOOM_MIN: f64 = 0.25;
const SCHEMATIC_ZOOM_MAX: f64 = 8.0;
/// Convert normalized raw wheel/trackpad points into a continuous zoom.
///
/// Egui deliberately replays discrete wheel notches through its smoothed
/// scrolling stream. That feels good in a document, but it makes a CAD camera
/// continue zooming after the physical input and compounds one notch across
/// several frames. The schematic consumes raw events instead: pixel-precise
/// trackpads remain proportional, discrete units are normalized once, and a
/// pathological driver spike is bounded before the exponential curve.
const WHEEL_ZOOM_EXPONENT_PER_POINT: f64 = 0.0006;
const WHEEL_ZOOM_MAX_DELTA_PER_EVENT: f32 = 240.0;
const WHEEL_ZOOM_LINE_POINTS: f32 = 40.0;
const WHEEL_ZOOM_PAGE_POINTS: f32 = 240.0;

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
        let (scroll, shift) = raw_wheel_navigation_delta(ui);
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

fn raw_wheel_navigation_delta(ui: &Ui) -> (Vec2, bool) {
    ui.input(|input| {
        let mut normalized = Vec2::ZERO;
        let mut shift = input.modifiers.shift;
        for event in &input.raw.events {
            let Event::MouseWheel {
                unit,
                delta,
                phase: TouchPhase::Move,
                modifiers,
            } = event
            else {
                continue;
            };
            shift |= modifiers.shift;
            normalized.x += canonical_wheel_points(*unit, delta.x);
            normalized.y += canonical_wheel_points(*unit, delta.y);
        }
        (normalized, shift)
    })
}

fn canonical_wheel_points(unit: MouseWheelUnit, delta: f32) -> f32 {
    let points = match unit {
        MouseWheelUnit::Point => delta,
        MouseWheelUnit::Line => delta * WHEEL_ZOOM_LINE_POINTS,
        MouseWheelUnit::Page => delta.signum() * WHEEL_ZOOM_PAGE_POINTS,
    };
    points.clamp(
        -WHEEL_ZOOM_MAX_DELTA_PER_EVENT,
        WHEEL_ZOOM_MAX_DELTA_PER_EVENT,
    )
}

fn wheel_zoom_factor(scroll_delta_y: f32) -> f64 {
    (f64::from(scroll_delta_y) * WHEEL_ZOOM_EXPONENT_PER_POINT).exp()
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
            (1.07..1.08).contains(&notch_in),
            "one wheel notch should be a controlled step, got {notch_in}"
        );
        assert!((notch_in * notch_out - 1.0).abs() < 1e-12);

        let small_delta = wheel_zoom_factor(1.0);
        assert!(
            (1.0..1.001).contains(&small_delta),
            "precision scrolling should remain proportional, got {small_delta}"
        );
    }

    #[test]
    fn raw_wheel_units_are_normalized_once_and_driver_spikes_are_bounded() {
        assert_eq!(canonical_wheel_points(MouseWheelUnit::Point, 1.0), 1.0);
        assert_eq!(
            canonical_wheel_points(MouseWheelUnit::Point, 10_000.0),
            WHEEL_ZOOM_MAX_DELTA_PER_EVENT
        );
        assert_eq!(
            canonical_wheel_points(MouseWheelUnit::Line, 1.0),
            WHEEL_ZOOM_LINE_POINTS
        );
        assert_eq!(
            canonical_wheel_points(MouseWheelUnit::Page, -3.0),
            -WHEEL_ZOOM_PAGE_POINTS
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
