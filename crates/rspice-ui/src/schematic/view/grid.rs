use egui::{Painter, Pos2, Rect, Stroke};

use crate::common::app::AppState;

/// Draw the schematic grid
pub(super) fn draw_grid(painter: &Painter, bounds: Rect, state: &AppState) {
    let grid_size = state.schematic.grid_size as f32;
    let zoom = state.schematic.zoom as f32;
    let pan_x = state.schematic.pan.0 as f32;
    let pan_y = state.schematic.pan.1 as f32;

    let base_grid_spacing = grid_size * zoom;

    // Dynamically adjust grid step to ensure reasonable visual spacing
    // At very low zoom, show fewer grid lines (every 10th or 100th)
    let (grid_step, is_all_major) = if base_grid_spacing >= 5.0 {
        // Normal: draw every grid line, with every 10th as major
        (1, false)
    } else if base_grid_spacing * 10.0 >= 5.0 {
        // Medium zoom out: draw every 10th grid line (all appear as major)
        (10, true)
    } else if base_grid_spacing * 100.0 >= 5.0 {
        // Very zoomed out: draw every 100th grid line
        (100, true)
    } else {
        // Extremely zoomed out: skip grid entirely
        return;
    };

    let grid_spacing = base_grid_spacing * grid_step as f32;

    let minor_color = state.theme.grid_minor;
    let major_color = state.theme.grid_major;

    // Grid lines are at schematic coordinates that are multiples of grid_size.
    // In screen space: screen_x = bounds.min.x + pan_x + (schematic_x * zoom)
    // For grid line at schematic_x = gx * grid_size:
    //   screen_x = bounds.min.x + pan_x + gx * grid_size * zoom
    //            = bounds.min.x + pan_x + gx * grid_spacing

    // Calculate visible grid index range (now in terms of grid_step multiples)
    let left_grid = ((0.0 - pan_x) / grid_spacing).floor() as i32 - 1;
    let right_grid = ((bounds.width() - pan_x) / grid_spacing).ceil() as i32 + 1;
    let top_grid = ((0.0 - pan_y) / grid_spacing).floor() as i32 - 1;
    let bottom_grid = ((bounds.height() - pan_y) / grid_spacing).ceil() as i32 + 1;

    // Draw vertical lines (for each visible grid x-coordinate)
    for gx in left_grid..=right_grid {
        // Convert grid index to screen coordinate (include bounds.min.x offset)
        let screen_x = bounds.min.x + pan_x + (gx as f32) * grid_spacing;
        if screen_x < bounds.min.x || screen_x > bounds.max.x {
            continue;
        }
        // In stepped mode, determine if this is a "super major" line
        let actual_gx = gx * grid_step;
        let is_major = is_all_major || (actual_gx % 10 == 0);
        let color = if is_major { major_color } else { minor_color };
        painter.line_segment(
            [
                Pos2::new(screen_x, bounds.min.y),
                Pos2::new(screen_x, bounds.max.y),
            ],
            Stroke::new(if is_major { 1.0 } else { 0.5 }, color),
        );
    }

    // Draw horizontal lines (for each visible grid y-coordinate)
    for gy in top_grid..=bottom_grid {
        // Convert grid index to screen coordinate (include bounds.min.y offset)
        let screen_y = bounds.min.y + pan_y + (gy as f32) * grid_spacing;
        if screen_y < bounds.min.y || screen_y > bounds.max.y {
            continue;
        }
        let actual_gy = gy * grid_step;
        let is_major = is_all_major || (actual_gy % 10 == 0);
        let color = if is_major { major_color } else { minor_color };
        painter.line_segment(
            [
                Pos2::new(bounds.min.x, screen_y),
                Pos2::new(bounds.max.x, screen_y),
            ],
            Stroke::new(if is_major { 1.0 } else { 0.5 }, color),
        );
    }
}
