//! Canvas grid.
//!
//! The schematic well draws its grid in one of two styles — a dot per
//! snap point (default) or hairline rules — batched into a single mesh
//! so even dense views cost one draw call. At low zoom the authored pitch
//! advances through the conventional 1-2-5 engineering sequence so the grid
//! remains legible without losing the operator's sense of scale.
//!
//! The lattice is built once in grid-local space and cached; the pattern
//! repeats with period `pitch`, so pans and idle frames reuse the mesh
//! (a vertex memcpy + translate) instead of re-emitting tens of
//! thousands of quads. Only zoom (pitch), style, and resize rebuild it.

use std::cell::RefCell;

use egui::{Color32, Mesh, Painter, Rect, Shape, pos2, vec2};

use crate::state::GridStyle;
use crate::workbench::app_state::AppState;

/// Minimum cell pitch in device pixels before selecting the next coarser
/// engineering step.
const MIN_DEVICE_PITCH: f32 = 5.0;
/// Dot and rule geometry is specified in device pixels, then converted to
/// egui points for the active display scale.
const DEVICE_HAIRLINE: f32 = 1.0;

struct CachedGrid {
    pitch_bits: u32,
    pixels_per_point_bits: u32,
    style: GridStyle,
    color: Color32,
    cols: i32,
    rows: i32,
    /// Pattern anchored at `(i · pitch, j · pitch)` with origin (0, 0).
    mesh: Mesh,
}

thread_local! {
    /// Painting is single-threaded; one canvas grid is live at a time.
    static GRID_MESH: RefCell<Option<CachedGrid>> = const { RefCell::new(None) };
}

/// Draw the schematic grid in the workbench's active style, clipped to the
/// authored drawing area while retaining the world-origin lattice phase.
pub(super) fn draw_grid(
    painter: &Painter,
    canvas_bounds: Rect,
    clip_bounds: Rect,
    state: &AppState,
) {
    let style = state.ui.grid;
    if !style.visible() || !clip_bounds.is_positive() {
        return;
    }

    let grid_size = state.schematic.grid_size as f32;
    let zoom = state.schematic.zoom as f32;
    let pan_x = state.schematic.pan.0 as f32;
    let pan_y = state.schematic.pan.1 as f32;

    let pixels_per_point = painter.ctx().pixels_per_point().max(f32::EPSILON);
    let base_pitch = grid_size * zoom;
    let multiplier = engineering_grid_multiplier(base_pitch * pixels_per_point, MIN_DEVICE_PITCH);
    let pitch = base_pitch * multiplier;
    if !pitch.is_finite() || pitch <= 0.0 {
        return;
    }

    // Dots and rules share the authored canvas-grid token. The style changes
    // geometry only; silently dimming one mode makes the same visibility
    // preference produce two different contrast contracts.
    let color = crate::ui::tokens::active_palette().canvas_grid;

    // Lattice extent: cover the bounds plus one period on each side; the
    // painter's clip rect trims the overhang.
    let cols = (clip_bounds.width() / pitch).ceil() as i32 + 2;
    let rows = (clip_bounds.height() / pitch).ceil() as i32 + 2;

    // Phase is anchored to the viewport's world origin, then advanced to the
    // first lattice point at or before the drawing-area clip.
    let world_origin = canvas_bounds.min + vec2(pan_x, pan_y);
    let origin = pos2(
        clip_bounds.left() - (clip_bounds.left() - world_origin.x).rem_euclid(pitch) - pitch,
        clip_bounds.top() - (clip_bounds.top() - world_origin.y).rem_euclid(pitch) - pitch,
    );

    GRID_MESH.with(|cache| {
        let mut cache = cache.borrow_mut();
        let stale = match cache.as_ref() {
            Some(c) => {
                c.pitch_bits != pitch.to_bits()
                    || c.pixels_per_point_bits != pixels_per_point.to_bits()
                    || c.style != style
                    || c.color != color
                    || c.cols != cols
                    || c.rows != rows
            }
            None => true,
        };
        if stale {
            let hairline = DEVICE_HAIRLINE / pixels_per_point;
            let mesh = match style {
                GridStyle::Lines => line_mesh(pitch, cols, rows, color, hairline),
                _ => dot_mesh(pitch, cols, rows, color, hairline),
            };
            *cache = Some(CachedGrid {
                pitch_bits: pitch.to_bits(),
                pixels_per_point_bits: pixels_per_point.to_bits(),
                style,
                color,
                cols,
                rows,
                mesh,
            });
        }

        // Rebuilt above when stale; skip a frame rather than panic if not.
        if let Some(cached) = cache.as_ref() {
            let mut mesh = cached.mesh.clone();
            mesh.translate(origin.to_vec2());
            painter.with_clip_rect(clip_bounds).add(Shape::mesh(mesh));
        }
    });
}

/// Smallest 1-2-5 multiplier whose resulting pitch reaches the requested
/// device-pixel floor. The sequence continues by decades instead of
/// disappearing at an arbitrary low zoom.
fn engineering_grid_multiplier(base_device_pitch: f32, minimum_device_pitch: f32) -> f32 {
    if !base_device_pitch.is_finite()
        || base_device_pitch <= 0.0
        || base_device_pitch >= minimum_device_pitch
    {
        return 1.0;
    }
    let required = minimum_device_pitch / base_device_pitch;
    let decade = 10.0_f32.powf(required.log10().floor());
    for mantissa in [1.0_f32, 2.0, 5.0, 10.0] {
        let candidate = decade * mantissa;
        if candidate >= required {
            return candidate;
        }
    }
    decade * 10.0
}

/// One one-device-pixel quad per lattice crossing.
fn dot_mesh(pitch: f32, cols: i32, rows: i32, color: Color32, hairline: f32) -> Mesh {
    let quads = (cols as usize) * (rows as usize);
    let mut mesh = Mesh::default();
    mesh.reserve_vertices(quads * 4);
    mesh.reserve_triangles(quads * 2);
    for gx in 0..cols {
        let x = gx as f32 * pitch;
        for gy in 0..rows {
            let y = gy as f32 * pitch;
            mesh.add_colored_rect(
                Rect::from_center_size(pos2(x, y), vec2(hairline, hairline)),
                color,
            );
        }
    }
    mesh
}

/// One hairline quad per lattice column and row, spanning the extent.
fn line_mesh(pitch: f32, cols: i32, rows: i32, color: Color32, hairline: f32) -> Mesh {
    let quads = (cols + rows) as usize;
    let mut mesh = Mesh::default();
    mesh.reserve_vertices(quads * 4);
    mesh.reserve_triangles(quads * 2);
    let height = rows as f32 * pitch;
    let width = cols as f32 * pitch;
    let half = hairline * 0.5;
    for gx in 0..cols {
        let x = gx as f32 * pitch;
        mesh.add_colored_rect(
            Rect::from_min_max(pos2(x - half, 0.0), pos2(x + half, height)),
            color,
        );
    }
    for gy in 0..rows {
        let y = gy as f32 * pitch;
        mesh.add_colored_rect(
            Rect::from_min_max(pos2(0.0, y - half), pos2(width, y + half)),
            color,
        );
    }
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engineering_grid_uses_one_two_five_steps_at_five_device_pixels() {
        assert_eq!(engineering_grid_multiplier(5.0, 5.0), 1.0);
        assert_eq!(engineering_grid_multiplier(3.0, 5.0), 2.0);
        assert_eq!(engineering_grid_multiplier(1.1, 5.0), 5.0);
        assert_eq!(engineering_grid_multiplier(0.6, 5.0), 10.0);
        assert_eq!(engineering_grid_multiplier(0.24, 5.0), 50.0);
        assert!(0.11 * engineering_grid_multiplier(0.11, 5.0) >= MIN_DEVICE_PITCH);
    }
}
