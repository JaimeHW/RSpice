//! Canvas dot grid.
//!
//! The schematic well uses a dot grid (one dot per snap point), batched into
//! a single mesh so even dense views cost one draw call. At low zoom the dot
//! pitch steps up ×10 / ×100 to stay legible, and below that the grid fades
//! out entirely.

use egui::{Painter, Rect, Shape, pos2, vec2};

use crate::common::app::AppState;

/// Minimum on-screen dot pitch before stepping up to a coarser grid.
const MIN_PITCH: f32 = 9.0;
/// Dot half-extent in points (dots render as 2 × 2 quads, ≈ the design's
/// 0.9 px-radius circles, at a fraction of the tessellation cost).
const DOT_HALF: f32 = 1.0;

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

    let left = ((0.0 - pan_x) / pitch).floor() as i32;
    let right = ((bounds.width() - pan_x) / pitch).ceil() as i32;
    let top = ((0.0 - pan_y) / pitch).floor() as i32;
    let bottom = ((bounds.height() - pan_y) / pitch).ceil() as i32;

    let mut mesh = egui::Mesh::default();
    for gx in left..=right {
        let x = bounds.min.x + pan_x + gx as f32 * pitch;
        if x < bounds.min.x || x > bounds.max.x {
            continue;
        }
        for gy in top..=bottom {
            let y = bounds.min.y + pan_y + gy as f32 * pitch;
            if y < bounds.min.y || y > bounds.max.y {
                continue;
            }
            mesh.add_colored_rect(
                Rect::from_center_size(pos2(x, y), vec2(DOT_HALF * 2.0, DOT_HALF * 2.0)),
                color,
            );
        }
    }
    painter.add(Shape::mesh(mesh));
}
