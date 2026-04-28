//! Waveform Viewer Mouse Interactions
//!
//! Handles pan, zoom, drag, and selection interactions for the waveform viewer.
//! This module provides the interaction logic that is used by the rendering module.

use super::state::{BoxSelection, CursorState, ViewTransform};

// =============================================================================
// Interaction State
// =============================================================================

/// Pan/zoom interaction state
#[derive(Debug, Clone, Default)]
pub struct InteractionState {
    /// Whether panning is in progress
    pub is_panning: bool,
    /// Pan start X (screen coords)
    pub pan_start_x: f32,
    /// Pan start Y (screen coords)
    pub pan_start_y: f32,
}

// =============================================================================
// Zoom Operations
// =============================================================================

/// Apply scroll wheel zoom to view transform
///
/// # Arguments
/// * `view` - View transform to modify
/// * `delta_y` - Scroll wheel delta (positive = zoom in)
/// * `mouse_x_frac` - Mouse X as fraction of plot width (0-1)
/// * `mouse_y_frac` - Mouse Y as fraction of plot height (0-1)
/// * `shift_held` - Shift modifier for X-only zoom
/// * `ctrl_held` - Ctrl modifier for Y-only zoom
pub fn apply_scroll_zoom(
    view: &mut ViewTransform,
    delta_y: f32,
    mouse_x_frac: f64,
    mouse_y_frac: f64,
    shift_held: bool,
    ctrl_held: bool,
) {
    let factor = if delta_y > 0.0 { 0.9 } else { 1.1 };

    if shift_held {
        view.zoom_x_only(factor, mouse_x_frac);
    } else if ctrl_held {
        view.zoom_y_only(factor, mouse_y_frac);
    } else {
        view.zoom(factor, mouse_x_frac, mouse_y_frac);
    }
}

/// Apply button zoom (for toolbar buttons)
pub fn apply_button_zoom(view: &mut ViewTransform, zoom_in: bool) {
    let factor = if zoom_in { 0.8 } else { 1.25 };
    view.zoom(factor, 0.5, 0.5);
}

// =============================================================================
// Pan Operations
// =============================================================================

/// Apply drag delta to pan the view
///
/// # Arguments
/// * `view` - View transform to modify
/// * `delta_x` - Screen delta X (pixels)
/// * `delta_y` - Screen delta Y (pixels)
pub fn apply_pan_drag(view: &mut ViewTransform, delta_x: f32, delta_y: f32) {
    let data_dx = -(delta_x as f64 / view.plot_width) * view.x_range();
    let data_dy = (delta_y as f64 / view.plot_height) * view.y_range();
    view.pan(data_dx, data_dy);
}

// =============================================================================
// Cursor Operations
// =============================================================================

/// Place cursor at screen position
///
/// # Arguments
/// * `cursors` - Cursor state to modify
/// * `view` - View transform for coordinate conversion
/// * `screen_x` - Screen X coordinate (relative to plot origin)
/// * `plot_width` - Plot width in pixels
pub fn place_cursor_at_screen(
    cursors: &mut CursorState,
    view: &ViewTransform,
    screen_x: f32,
    plot_width: f32,
) {
    let x_frac = (screen_x / plot_width).clamp(0.0, 1.0) as f64;
    let data_x = view.x_min + x_frac * view.x_range();
    cursors.place(data_x);
}

// =============================================================================
// Box Selection Operations
// =============================================================================

/// Start box selection at screen position
pub fn start_box_selection(
    box_sel: &mut BoxSelection,
    view: &ViewTransform,
    screen_x: f32,
    screen_y: f32,
    plot_rect: (f64, f64, f64, f64),
) {
    let data_x = screen_to_data_x(view, screen_x, plot_rect.2 as f32);
    let data_y = screen_to_data_y(view, screen_y, plot_rect.3 as f32);
    box_sel.start(data_x, data_y, screen_x as f64, screen_y as f64, plot_rect);
}

/// Update box selection endpoint
pub fn update_box_selection(
    box_sel: &mut BoxSelection,
    view: &ViewTransform,
    screen_x: f32,
    screen_y: f32,
    plot_width: f32,
    plot_height: f32,
) {
    let data_x = screen_to_data_x(view, screen_x, plot_width);
    let data_y = screen_to_data_y(view, screen_y, plot_height);
    box_sel.update(data_x, data_y);
}

/// Finish box selection and zoom to region
pub fn finish_box_selection(box_sel: &mut BoxSelection, view: &mut ViewTransform) {
    if let Some((x_min, x_max, y_min, y_max)) = box_sel.finish() {
        view.x_min = x_min;
        view.x_max = x_max;
        view.y_min = y_min;
        view.y_max = y_max;
    }
}

// =============================================================================
// Coordinate Conversion Helpers
// =============================================================================

/// Convert screen X to data X
fn screen_to_data_x(view: &ViewTransform, screen_x: f32, plot_width: f32) -> f64 {
    let x_frac = (screen_x / plot_width.max(1.0)).clamp(0.0, 1.0) as f64;
    view.x_min + x_frac * view.x_range()
}

/// Convert screen Y to data Y
fn screen_to_data_y(view: &ViewTransform, screen_y: f32, plot_height: f32) -> f64 {
    let y_frac = (screen_y / plot_height.max(1.0)).clamp(0.0, 1.0) as f64;
    view.y_max - y_frac * view.y_range()
}

// =============================================================================
// Tests
// =============================================================================

