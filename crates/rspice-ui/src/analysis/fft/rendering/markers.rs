use super::interactions::{freq_to_x, freq_to_x_for_trace, mag_to_y};
use super::*;

#[derive(Debug, Clone)]
pub(super) struct PlotCursorLabelSpec {
    pub(super) anchor_x: f32,
    pub(super) text: String,
    pub(super) color: Color32,
    pub(super) font: FontId,
}

pub(super) fn render_fundamental_marker(
    painter: &egui::Painter,
    rect: Rect,
    freq: f64,
    state: &FftState,
) -> Option<f32> {
    let x = freq_to_x(freq, rect, state);
    if x < rect.min.x || x > rect.max.x {
        return None;
    }

    painter.line_segment(
        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
        Stroke::new(1.0, fundamental_color()),
    );
    Some(x)
}

pub(super) fn render_harmonic_marker(
    painter: &egui::Painter,
    rect: Rect,
    freq: f64,
    _db: f64,
    state: &FftState,
) -> Option<f32> {
    let x = freq_to_x(freq, rect, state);
    if x < rect.min.x || x > rect.max.x {
        return None;
    }

    // Short vertical tick
    painter.line_segment(
        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.min.y + 15.0)],
        Stroke::new(1.0, harmonic_color()),
    );
    Some(x)
}

pub(super) fn render_peak_marker(
    painter: &egui::Painter,
    rect: Rect,
    peak: &FftPoint,
    state: &FftState,
) {
    let x = freq_to_x(peak.frequency, rect, state);
    let y = mag_to_y(peak, rect, state);

    if x < rect.min.x || x > rect.max.x {
        return;
    }

    // Small circle at peak
    painter.circle_filled(
        Pos2::new(x, y.clamp(rect.min.y, rect.max.y)),
        3.0,
        peak_color(),
    );
}

pub(super) fn render_user_marker(
    plot_rect: Rect,
    marker_freq: f64,
    data: &FftData,
    state: &FftState,
    painter: &egui::Painter,
    slot_index: usize,
) -> Option<PlotCursorLabelSpec> {
    let x = freq_to_x(marker_freq, plot_rect, state);
    if !x.is_finite() || x < plot_rect.min.x || x > plot_rect.max.x {
        return None;
    }
    let marker_name = format!("M{}", slot_index + 1);
    let marker_color = marker_color_for_slot(slot_index);
    painter.line_segment(
        [Pos2::new(x, plot_rect.min.y), Pos2::new(x, plot_rect.max.y)],
        Stroke::new(1.0, marker_color),
    );

    data.interpolate(marker_freq).map(|point| {
        let text = match state.mag_scale {
            MagnitudeScale::Linear => {
                format!(
                    "{}: {} | {:.4}",
                    marker_name,
                    format_freq(marker_freq),
                    state.display_magnitude(&point)
                )
            }
            MagnitudeScale::DB => format!(
                "{}: {} | {:.2} dB",
                marker_name,
                format_freq(marker_freq),
                state.display_magnitude(&point)
            ),
            MagnitudeScale::DBc => format!(
                "{}: {} | {:.2} dBc",
                marker_name,
                format_freq(marker_freq),
                state.display_magnitude(&point)
            ),
            MagnitudeScale::DBm => {
                format!(
                    "{}: {} | {:.2} dBm",
                    marker_name,
                    format_freq(marker_freq),
                    state.display_magnitude(&point)
                )
            }
        };
        PlotCursorLabelSpec {
            anchor_x: x,
            text,
            color: marker_color,
            font: FontId::proportional(CURSOR_LABEL_FONT_SIZE),
        }
    })
}

pub(super) fn render_fft_cursor_labels(
    painter: &egui::Painter,
    plot_rect: Rect,
    data: &FftData,
    state: &FftState,
    labels: &[PlotCursorLabelSpec],
    line_x_positions: &[f32],
) {
    if labels.is_empty() {
        return;
    }

    let requests: Vec<VerticalLabelRequest> = labels
        .iter()
        .map(|label| {
            let text_size =
                measure_text_size(painter, &label.text, label.font.clone(), label.color);
            VerticalLabelRequest {
                anchor_x: label.anchor_x,
                size: Vec2::new(
                    text_size.x + CURSOR_LABEL_TEXT_PADDING_X * 2.0,
                    text_size.y + CURSOR_LABEL_TEXT_PADDING_Y * 2.0,
                ),
            }
        })
        .collect();

    let placements = layout_fft_cursor_labels(plot_rect, &requests, line_x_positions, data, state);
    for (label, placement) in labels.iter().zip(placements.iter()) {
        draw_cursor_label(painter, label, placement);
    }
}

fn layout_fft_cursor_labels(
    plot_rect: Rect,
    requests: &[VerticalLabelRequest],
    line_x_positions: &[f32],
    data: &FftData,
    state: &FftState,
) -> Vec<VerticalLabelPlacement> {
    let max_h = requests.iter().fold(0.0f32, |acc, r| acc.max(r.size.y));
    let config = VerticalLabelLayoutConfig {
        line_clearance: 4.0,
        top_margin: 2.0,
        row_gap: 3.0,
        preferred_rows: 6,
        nudge_step: 8.0,
        nudge_steps: 10,
        label_gap: 2.0,
    };
    let search_band_bottom = plot_rect.min.y
        + config.top_margin
        + (max_h + config.row_gap) * config.preferred_rows as f32
        + 4.0;
    let obstacles = collect_fft_cursor_label_obstacles(plot_rect, data, state, search_band_bottom);

    place_vertical_line_labels(plot_rect, requests, line_x_positions, &obstacles, config)
}

fn collect_fft_cursor_label_obstacles(
    plot_rect: Rect,
    data: &FftData,
    state: &FftState,
    band_bottom: f32,
) -> Vec<Rect> {
    let mut obstacles = Vec::new();
    if !band_bottom.is_finite() || band_bottom <= plot_rect.min.y {
        return obstacles;
    }

    let band_bottom = band_bottom.min(plot_rect.max.y);
    let n = data.points.len();
    if n == 0 {
        return obstacles;
    }
    let step = (n / 600).max(1);
    obstacles.reserve((n / step).min(800));

    for point in data.points.iter().step_by(step) {
        let Some(x) = freq_to_x_for_trace(point.frequency, plot_rect, state) else {
            continue;
        };
        let y = mag_to_y(point, plot_rect, state);
        if !x.is_finite() || !y.is_finite() {
            continue;
        }
        if x < plot_rect.min.x || x > plot_rect.max.x || y < plot_rect.min.y || y > band_bottom {
            continue;
        }
        obstacles.push(Rect::from_center_size(Pos2::new(x, y), Vec2::splat(3.0)));
    }

    obstacles
}

fn draw_cursor_label(
    painter: &egui::Painter,
    label: &PlotCursorLabelSpec,
    placement: &VerticalLabelPlacement,
) {
    let bg = Color32::from_rgba_unmultiplied(20, 22, 28, CURSOR_LABEL_BG_ALPHA);
    painter.rect_filled(
        placement.rect,
        Rounding::same(CURSOR_LABEL_CORNER_RADIUS),
        bg,
    );
    painter.rect_stroke(
        placement.rect,
        Rounding::same(CURSOR_LABEL_CORNER_RADIUS),
        Stroke::new(1.0, label.color.gamma_multiply(0.8)),
    );

    let connector_y = placement.rect.center().y;
    let connector_x = match placement.side {
        LabelSide::Right => placement.rect.min.x,
        LabelSide::Left => placement.rect.max.x,
    };
    painter.line_segment(
        [
            Pos2::new(label.anchor_x, connector_y),
            Pos2::new(connector_x, connector_y),
        ],
        Stroke::new(CURSOR_LABEL_LINE_STROKE, label.color.gamma_multiply(0.75)),
    );

    painter.text(
        Pos2::new(
            placement.rect.min.x + CURSOR_LABEL_TEXT_PADDING_X,
            placement.rect.min.y + CURSOR_LABEL_TEXT_PADDING_Y,
        ),
        egui::Align2::LEFT_TOP,
        &label.text,
        label.font.clone(),
        label.color,
    );
}
