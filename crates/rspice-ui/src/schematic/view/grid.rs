//! Canvas grid.
//!
//! The schematic well draws its grid in one of two styles — a dot per
//! snap point (default) or hairline rules — batched into a single mesh
//! so even dense views cost one draw call. At low zoom the pitch steps
//! up ×10 / ×100 to stay legible, and below that the grid fades out
//! entirely.
//!
//! The lattice is built once in grid-local space and cached; the pattern
//! repeats with period `pitch`, so pans and idle frames reuse the mesh
//! (a vertex memcpy + translate) instead of re-emitting tens of
//! thousands of quads. Only zoom (pitch), style, and resize rebuild it.

use std::cell::RefCell;

use egui::{Color32, Mesh, Painter, Rect, Shape, pos2, vec2};

use crate::common::app::AppState;
use crate::workbench::GridStyle;

/// Minimum on-screen pitch before stepping up to a coarser grid.
const MIN_PITCH: f32 = 9.0;
/// Dot half-extent in points (dots render as 2 × 2 quads, ≈ the design's
/// 0.9 px-radius circles, at a fraction of the tessellation cost).
const DOT_HALF: f32 = 1.0;
/// Rule half-thickness in points (1 pt hairlines).
const LINE_HALF: f32 = 0.5;

struct CachedGrid {
    pitch_bits: u32,
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

/// Draw the schematic grid in the workbench's active style.
pub(super) fn draw_grid(painter: &Painter, bounds: Rect, state: &AppState) {
    let style = state.ui.grid;
    if !style.visible() {
        return;
    }

    let grid_size = state.schematic.grid_size as f32;
    let zoom = state.schematic.zoom as f32;
    let pan_x = state.schematic.pan.0 as f32;
    let pan_y = state.schematic.pan.1 as f32;

    let base_pitch = grid_size * zoom;
    let pitch = if base_pitch >= MIN_PITCH {
        base_pitch
    } else if base_pitch * 10.0 >= MIN_PITCH {
        base_pitch * 10.0
    } else if base_pitch * 100.0 >= MIN_PITCH {
        base_pitch * 100.0
    } else {
        return;
    };

    // Rules sweep the full canvas where dots only mark crossings, so the
    // line style takes a quieter tint of the same grid token.
    let color = match style {
        GridStyle::Lines => crate::ui::tokens::active_palette()
            .canvas_grid
            .gamma_multiply(0.55),
        _ => crate::ui::tokens::active_palette().canvas_grid,
    };

    // Lattice extent: cover the bounds plus one period on each side; the
    // painter's clip rect trims the overhang.
    let cols = (bounds.width() / pitch).ceil() as i32 + 2;
    let rows = (bounds.height() / pitch).ceil() as i32 + 2;

    // Phase: the lattice column/row at or before the top-left corner.
    let origin = bounds.min
        + vec2(
            pan_x.rem_euclid(pitch) - pitch,
            pan_y.rem_euclid(pitch) - pitch,
        );

    GRID_MESH.with(|cache| {
        let mut cache = cache.borrow_mut();
        let stale = match cache.as_ref() {
            Some(c) => {
                c.pitch_bits != pitch.to_bits()
                    || c.style != style
                    || c.color != color
                    || c.cols != cols
                    || c.rows != rows
            }
            None => true,
        };
        if stale {
            let mesh = match style {
                GridStyle::Lines => line_mesh(pitch, cols, rows, color),
                _ => dot_mesh(pitch, cols, rows, color),
            };
            *cache = Some(CachedGrid {
                pitch_bits: pitch.to_bits(),
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
            painter.add(Shape::mesh(mesh));
        }
    });
}

/// One 2 × 2 pt quad per lattice crossing.
fn dot_mesh(pitch: f32, cols: i32, rows: i32, color: Color32) -> Mesh {
    let quads = (cols as usize) * (rows as usize);
    let mut mesh = Mesh::default();
    mesh.reserve_vertices(quads * 4);
    mesh.reserve_triangles(quads * 2);
    for gx in 0..cols {
        let x = gx as f32 * pitch;
        for gy in 0..rows {
            let y = gy as f32 * pitch;
            mesh.add_colored_rect(
                Rect::from_center_size(pos2(x, y), vec2(DOT_HALF * 2.0, DOT_HALF * 2.0)),
                color,
            );
        }
    }
    mesh
}

/// One hairline quad per lattice column and row, spanning the extent.
fn line_mesh(pitch: f32, cols: i32, rows: i32, color: Color32) -> Mesh {
    let quads = (cols + rows) as usize;
    let mut mesh = Mesh::default();
    mesh.reserve_vertices(quads * 4);
    mesh.reserve_triangles(quads * 2);
    let height = rows as f32 * pitch;
    let width = cols as f32 * pitch;
    for gx in 0..cols {
        let x = gx as f32 * pitch;
        mesh.add_colored_rect(
            Rect::from_min_max(pos2(x - LINE_HALF, 0.0), pos2(x + LINE_HALF, height)),
            color,
        );
    }
    for gy in 0..rows {
        let y = gy as f32 * pitch;
        mesh.add_colored_rect(
            Rect::from_min_max(pos2(0.0, y - LINE_HALF), pos2(width, y + LINE_HALF)),
            color,
        );
    }
    mesh
}
