//! Canvas dot grid.
//!
//! The schematic well uses a dot grid (one dot per snap point), batched into
//! a single mesh so even dense views cost one draw call. At low zoom the dot
//! pitch steps up ×10 / ×100 to stay legible, and below that the grid fades
//! out entirely.
//!
//! The lattice is built once in grid-local space and cached; the dot
//! pattern repeats with period `pitch`, so pans and idle frames reuse the
//! mesh (a vertex memcpy + translate) instead of re-emitting tens of
//! thousands of quads. Only zoom (pitch) and resize rebuild it.

use std::cell::RefCell;

use egui::{Color32, Mesh, Painter, Rect, Shape, pos2, vec2};

use crate::common::app::AppState;

/// Minimum on-screen dot pitch before stepping up to a coarser grid.
const MIN_PITCH: f32 = 9.0;
/// Dot half-extent in points (dots render as 2 × 2 quads, ≈ the design's
/// 0.9 px-radius circles, at a fraction of the tessellation cost).
const DOT_HALF: f32 = 1.0;

struct CachedGrid {
    pitch_bits: u32,
    color: Color32,
    cols: i32,
    rows: i32,
    /// Dots at `(i · pitch, j · pitch)` with origin (0, 0).
    mesh: Mesh,
}

thread_local! {
    /// Painting is single-threaded; one canvas grid is live at a time.
    static GRID_MESH: RefCell<Option<CachedGrid>> = const { RefCell::new(None) };
}

/// Draw the schematic grid as dots.
pub(super) fn draw_grid(painter: &Painter, bounds: Rect, state: &AppState) {
    if !state.shell.show_grid {
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

    let color = state.theme.grid_minor;

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
                    || c.color != color
                    || c.cols != cols
                    || c.rows != rows
            }
            None => true,
        };
        if stale {
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
            *cache = Some(CachedGrid {
                pitch_bits: pitch.to_bits(),
                color,
                cols,
                rows,
                mesh,
            });
        }

        let cached = cache.as_ref().expect("grid mesh built above");
        let mut mesh = cached.mesh.clone();
        mesh.translate(origin.to_vec2());
        painter.add(Shape::mesh(mesh));
    });
}
