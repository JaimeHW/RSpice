use egui::{Pos2, Rect};

use crate::common::app::AppState;
use crate::state::Point;

use super::viewport::Viewport;

pub(super) fn viewport_from_state(
    state: &AppState,
    bounds: Rect,
    pixels_per_point: f32,
) -> Viewport {
    // Snap the camera offset to whole physical pixels: feathered strokes
    // shimmering across fractional positions read as judder during pans.
    // The authoritative f64 pan is untouched, so motion never accumulates
    // rounding error.
    let ppp = pixels_per_point.max(0.5);
    let snap = |v: f64| ((v as f32) * ppp).round() / ppp;
    Viewport {
        offset: Pos2::new(snap(state.schematic.pan.0), snap(state.schematic.pan.1)),
        zoom: state.schematic.zoom as f32,
        bounds,
    }
}

pub(super) fn screen_to_grid(viewport: &Viewport, grid_size: i32, screen_pos: Pos2) -> Point {
    let zoom = viewport.zoom as f64;
    let pan_x = viewport.offset.x as f64;
    let pan_y = viewport.offset.y as f64;
    let bounds_min_x = viewport.bounds.min.x as f64;
    let bounds_min_y = viewport.bounds.min.y as f64;

    let schematic_x = (screen_pos.x as f64 - bounds_min_x - pan_x) / zoom;
    let schematic_y = (screen_pos.y as f64 - bounds_min_y - pan_y) / zoom;
    let grid_x = (schematic_x / grid_size as f64).round() as i32;
    let grid_y = (schematic_y / grid_size as f64).round() as i32;
    Point::new(grid_x * grid_size, grid_y * grid_size)
}

/// Convert a screen coordinate to the nearest integer schematic coordinate
/// without applying grid snapping. Hit-testing uses this projection so its
/// tolerance can be expressed in screen pixels independently of grid cadence.
pub(super) fn screen_to_schematic(viewport: &Viewport, screen_pos: Pos2) -> Point {
    let zoom = viewport.zoom as f64;
    let pan_x = viewport.offset.x as f64;
    let pan_y = viewport.offset.y as f64;
    let bounds_min_x = viewport.bounds.min.x as f64;
    let bounds_min_y = viewport.bounds.min.y as f64;

    let schematic_x = (screen_pos.x as f64 - bounds_min_x - pan_x) / zoom;
    let schematic_y = (screen_pos.y as f64 - bounds_min_y - pan_y) / zoom;
    Point::new(schematic_x.round() as i32, schematic_y.round() as i32)
}

pub(super) fn screen_to_wire_grid(viewport: &Viewport, grid_size: i32, screen_pos: Pos2) -> Point {
    // Wires intentionally snap to the same full-grid cadence as components.
    screen_to_grid(viewport, grid_size, screen_pos)
}
