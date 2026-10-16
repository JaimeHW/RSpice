//! Pan and zoom.
//!
//! Viewport movement: wheel and pinch zoom about the pointer, drag panning,
//! and the fit/zoom-to-selection commands.

use egui::{Event, MouseWheelUnit, Pos2, Rect, Response, TouchPhase, Ui, Vec2};

/// The lowest zoom offered whatever the sheet: 25 %.
pub const SCHEMATIC_ZOOM_MIN: f64 = 0.25;
pub const SCHEMATIC_ZOOM_MAX: f64 = 8.0;

/// The lowest zoom the canvas offers: 25 %, or less when that is what it takes
/// to show the whole sheet on this canvas.
pub fn zoom_floor(sheet_fit_zoom: f64) -> f64 {
    if sheet_fit_zoom.is_finite() && sheet_fit_zoom > 0.0 {
        sheet_fit_zoom.min(SCHEMATIC_ZOOM_MIN)
    } else {
        SCHEMATIC_ZOOM_MIN
    }
}

/// A change of view a command asks for. Only the canvas knows its own
/// rectangle, so the canvas carries the request out on its next frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoomRequest {
    /// Multiply the zoom by this factor, pivoting on the middle of the canvas.
    By(f64),
    /// Set this zoom, pivoting on the middle of the canvas.
    To(f64),
    /// Frame this world rectangle, `(min_x, min_y, max_x, max_y)`.
    Frame((f64, f64, f64, f64)),
}

/// The zoom box: armed by its command, it frames the next rectangle dragged
/// on the canvas and then lets go, as Virtuoso's `z` does.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ZoomBox {
    pub armed: bool,
    /// The world corners of the box being dragged: where the drag started and
    /// where the pointer is.
    pub drag: Option<((f64, f64), (f64, f64))>,
}

impl ZoomBox {
    pub fn arm(&mut self) {
        *self = Self {
            armed: true,
            drag: None,
        };
    }

    pub fn disarm(&mut self) {
        *self = Self::default();
    }

    /// The dragged box as `(min_x, min_y, max_x, max_y)`.
    pub fn world_rect(&self) -> Option<(f64, f64, f64, f64)> {
        let ((ax, ay), (bx, by)) = self.drag?;
        Some((ax.min(bx), ay.min(by), ax.max(bx), ay.max(by)))
    }
}
/// Convert normalized raw wheel/trackpad points into a continuous zoom.
///
/// Egui deliberately replays discrete wheel notches through its smoothed
/// scrolling stream. That feels good in a document, but it makes a CAD camera
/// continue zooming after the physical input and compounds one notch across
/// several frames. The schematic consumes raw events instead: pixel-precise
/// trackpads remain proportional, discrete units are normalized once, and a
/// pathological driver spike is bounded before the exponential curve.
const WHEEL_ZOOM_EXPONENT_PER_POINT: f64 = 0.00065;
const WHEEL_ZOOM_MAX_DELTA_PER_EVENT: f32 = 240.0;
const WHEEL_ZOOM_LINE_POINTS: f32 = 40.0;
const WHEEL_ZOOM_PAGE_POINTS: f32 = 240.0;

pub fn primary_pan_modifier_down(ui: &Ui) -> bool {
    ui.input(|input| input.modifiers.alt || input.key_down(egui::Key::Space))
}

pub fn primary_pan_gesture_active(ui: &Ui, response: &Response) -> bool {
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

/// Wheel, pinch and drag navigation. `min_zoom` is this frame's
/// [`zoom_floor`].
pub fn handle_viewport_navigation(
    ui: &Ui,
    response: &Response,
    available: Rect,
    zoom: &mut f64,
    pan: &mut (f64, f64),
    min_zoom: f64,
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
        apply_pan_delta(pan, delta);
    }

    if let Some(touch) = ui.input(|i| i.multi_touch())
        && available.contains(touch.start_pos)
    {
        apply_pan_delta(pan, touch.translation_delta);
        if (touch.zoom_delta - 1.0).abs() > f32::EPSILON {
            let focus = response.hover_pos().unwrap_or(touch.start_pos);
            apply_zoom_about(
                zoom,
                pan,
                available,
                focus,
                touch.zoom_delta as f64,
                min_zoom,
            );
        }
    }

    // Cursor-centered zoom, matching professional CAD tools.
    if response.hovered() {
        let (scroll, shift) = raw_wheel_navigation_delta(ui);
        if shift {
            let horizontal = if scroll.y != 0.0 { scroll.y } else { scroll.x };
            if horizontal != 0.0 {
                apply_horizontal_scroll_pan(pan, horizontal);
            }
        } else if scroll.y != 0.0
            && let Some(cursor_pos) = response.hover_pos()
        {
            apply_zoom_about(
                zoom,
                pan,
                available,
                cursor_pos,
                wheel_zoom_factor(scroll.y),
                min_zoom,
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

/// Scale the zoom by `zoom_factor`, keeping the world point under
/// `focus_pos` where it is on screen.
pub fn apply_zoom_about(
    zoom: &mut f64,
    pan: &mut (f64, f64),
    available: Rect,
    focus_pos: Pos2,
    zoom_factor: f64,
    min_zoom: f64,
) {
    if !zoom_factor.is_finite() || zoom_factor <= 0.0 {
        return;
    }
    let target = *zoom * zoom_factor;
    let focus = (
        f64::from(focus_pos.x) - f64::from(available.min.x),
        f64::from(focus_pos.y) - f64::from(available.min.y),
    );
    zoom_to_about(zoom, pan, focus, target, min_zoom);
}

/// Carry out a [`ZoomRequest::By`] or [`ZoomRequest::To`] about the middle of
/// a canvas `canvas_size` pixels across. A [`ZoomRequest::Frame`] needs the
/// fit policy the caller owns and is left to it.
pub fn apply_zoom_request(
    zoom: &mut f64,
    pan: &mut (f64, f64),
    canvas_size: (f64, f64),
    request: ZoomRequest,
    min_zoom: f64,
) {
    let target = match request {
        ZoomRequest::By(factor) if factor.is_finite() && factor > 0.0 => *zoom * factor,
        ZoomRequest::To(target) if target.is_finite() && target > 0.0 => target,
        ZoomRequest::By(_) | ZoomRequest::To(_) | ZoomRequest::Frame(_) => return,
    };
    let middle = (canvas_size.0 * 0.5, canvas_size.1 * 0.5);
    zoom_to_about(zoom, pan, middle, target, min_zoom);
}

/// Set the zoom to `target`, clamped to the range, keeping the world point
/// at `focus` (canvas pixels from its top-left corner) where it is.
fn zoom_to_about(
    zoom: &mut f64,
    pan: &mut (f64, f64),
    focus: (f64, f64),
    target: f64,
    min_zoom: f64,
) {
    let floor = if min_zoom.is_finite() && min_zoom > 0.0 {
        min_zoom.min(SCHEMATIC_ZOOM_MAX)
    } else {
        SCHEMATIC_ZOOM_MIN
    };
    let old_zoom = *zoom;
    let new_zoom = target.clamp(floor, SCHEMATIC_ZOOM_MAX);

    let focus_schematic_x = (focus.0 - pan.0) / old_zoom;
    let focus_schematic_y = (focus.1 - pan.1) / old_zoom;

    *zoom = new_zoom;
    pan.0 = focus.0 - focus_schematic_x * new_zoom;
    pan.1 = focus.1 - focus_schematic_y * new_zoom;
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
        apply_zoom_about(
            &mut zoom,
            &mut pan,
            available,
            focus,
            1.5,
            SCHEMATIC_ZOOM_MIN,
        );
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
            (1.08..1.082).contains(&notch_in),
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

        apply_zoom_about(&mut zoom, &mut pan, available, focus, 100.0, 0.25);
        assert_eq!(zoom, SCHEMATIC_ZOOM_MAX);

        apply_zoom_about(&mut zoom, &mut pan, available, focus, 0.001, 0.25);
        assert_eq!(zoom, SCHEMATIC_ZOOM_MIN);
    }

    /// A sheet too large for the canvas at 25 % lowers the floor to the zoom
    /// that fits it; a sheet that fits leaves the floor at 25 %.
    #[test]
    fn the_zoom_floor_is_the_lower_of_a_quarter_and_the_sheet_fit() {
        assert_eq!(zoom_floor(0.1), 0.1);
        assert_eq!(zoom_floor(0.6), SCHEMATIC_ZOOM_MIN);
        assert_eq!(zoom_floor(f64::NAN), SCHEMATIC_ZOOM_MIN);

        let available = Rect::from_min_size(pos2(0.0, 0.0), vec2(400.0, 300.0));
        let mut pan = (0.0, 0.0);
        let mut zoom = 0.25;
        apply_zoom_about(&mut zoom, &mut pan, available, available.center(), 0.001, 0.1);
        assert_eq!(zoom, 0.1);
    }

    /// A command's zoom pivots on the middle of the canvas.
    #[test]
    fn a_zoom_request_keeps_the_middle_of_the_canvas_in_place() {
        let world_at_centre =
            |zoom: f64, pan: (f64, f64)| ((200.0 - pan.0) / zoom, (150.0 - pan.1) / zoom);
        let mut zoom = 0.5;
        let mut pan = (-800.0, 40.0);
        let before = world_at_centre(zoom, pan);
        for request in [ZoomRequest::By(1.2), ZoomRequest::To(1.0), ZoomRequest::By(0.5)] {
            apply_zoom_request(&mut zoom, &mut pan, (400.0, 300.0), request, 0.25);
            let after = world_at_centre(zoom, pan);
            assert!((after.0 - before.0).abs() < 1e-9 && (after.1 - before.1).abs() < 1e-9);
        }
        assert_eq!(zoom, 0.5);
    }
}
