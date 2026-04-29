use egui::{Painter, Pos2, Rect, Stroke};

use super::super::state::EyeDiagramState;
use super::axes::{eye_full_time_range_seconds, eye_time_to_x, eye_voltage_to_y};
use super::style::{mask_fail_color, mask_outline_color, mask_pass_color};

pub(super) fn render_mask(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
    let mask = &state.mask;

    if mask.inner.points.is_empty() {
        return;
    }

    // Convert mask polygon to screen coordinates
    let screen_points: Vec<Pos2> = mask
        .inner
        .points
        .iter()
        .map(|&(t, v)| {
            let time_seconds = t * eye_full_time_range_seconds(state);
            let voltage = state.data.v_cross + v * state.data.swing;
            let x = eye_time_to_x(time_seconds, rect, state);
            let y = eye_voltage_to_y(voltage, rect, state);
            Pos2::new(x, y)
        })
        .collect();

    // Fill mask region
    let fill_color = if mask.is_passing() {
        mask_pass_color()
    } else {
        mask_fail_color()
    };

    if screen_points.len() >= 3 {
        // Draw as triangles from centroid
        let centroid = Pos2::new(
            screen_points.iter().map(|p| p.x).sum::<f32>() / screen_points.len() as f32,
            screen_points.iter().map(|p| p.y).sum::<f32>() / screen_points.len() as f32,
        );

        for i in 0..screen_points.len() {
            let j = (i + 1) % screen_points.len();
            painter.add(egui::Shape::convex_polygon(
                vec![centroid, screen_points[i], screen_points[j]],
                fill_color,
                Stroke::NONE,
            ));
        }

        // Outline
        for i in 0..screen_points.len() {
            let j = (i + 1) % screen_points.len();
            painter.line_segment(
                [screen_points[i], screen_points[j]],
                Stroke::new(1.5, mask_outline_color()),
            );
        }
    }
}
