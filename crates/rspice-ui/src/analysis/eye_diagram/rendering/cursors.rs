use egui::{Color32, FontId, Painter, Pos2, Rect, Rounding, Stroke, Vec2};

use super::super::state::EyeDiagramState;
use super::axes::eye_time_to_x;
use super::style::{cursor1_color, cursor2_color, marker_color};

pub(super) fn render_eye_cursors_and_markers(
    painter: &Painter,
    plot_rect: Rect,
    state: &EyeDiagramState,
) {
    let label_font = FontId::proportional(9.0);

    if let Some(t) = state.cursors.cursor1_time_s {
        let x = eye_time_to_x(t, plot_rect, state);
        if x.is_finite() {
            painter.line_segment(
                [Pos2::new(x, plot_rect.min.y), Pos2::new(x, plot_rect.max.y)],
                Stroke::new(1.4, cursor1_color()),
            );
            draw_eye_axis_label(
                painter,
                Pos2::new(x, plot_rect.min.y + 3.0),
                format!("C1 {}", crate::waveform::axis::format_time(t)),
                cursor1_color(),
                &label_font,
            );
        }
    }

    if let Some(t) = state.cursors.cursor2_time_s {
        let x = eye_time_to_x(t, plot_rect, state);
        if x.is_finite() {
            painter.line_segment(
                [Pos2::new(x, plot_rect.min.y), Pos2::new(x, plot_rect.max.y)],
                Stroke::new(1.4, cursor2_color()),
            );
            draw_eye_axis_label(
                painter,
                Pos2::new(x, plot_rect.min.y + 15.0),
                format!("C2 {}", crate::waveform::axis::format_time(t)),
                cursor2_color(),
                &label_font,
            );
        }
    }

    for (idx, marker_t) in state.markers.iter().copied().enumerate() {
        let x = eye_time_to_x(marker_t, plot_rect, state);
        if !x.is_finite() {
            continue;
        }
        let color = marker_color(idx);
        painter.line_segment(
            [Pos2::new(x, plot_rect.min.y), Pos2::new(x, plot_rect.max.y)],
            Stroke::new(1.0, color),
        );
        draw_eye_axis_label(
            painter,
            Pos2::new(x, plot_rect.min.y + 27.0),
            format!(
                "M{} {}",
                idx + 1,
                crate::waveform::axis::format_time(marker_t)
            ),
            color,
            &label_font,
        );
    }
}

pub(super) fn draw_eye_axis_label(
    painter: &Painter,
    anchor: Pos2,
    text: String,
    color: Color32,
    font: &FontId,
) {
    let galley = painter.layout_no_wrap(text, font.clone(), color);
    let size = galley.size();
    let rect = Rect::from_center_size(anchor, Vec2::new(size.x + 8.0, size.y + 4.0));
    painter.rect_filled(
        rect,
        Rounding::same(3.0),
        Color32::from_rgba_unmultiplied(20, 22, 28, 220),
    );
    painter.rect_stroke(
        rect,
        Rounding::same(3.0),
        Stroke::new(1.0, color.gamma_multiply(0.8)),
    );
    painter.galley(
        Pos2::new(rect.min.x + 4.0, rect.min.y + 2.0),
        galley,
        Color32::TRANSPARENT,
    );
}

// =============================================================================
