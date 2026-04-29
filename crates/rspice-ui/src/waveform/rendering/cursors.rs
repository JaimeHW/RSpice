use super::formatting::{format_optional_value, truncate_legend_trace_name};
use super::*;

#[derive(Debug, Clone, PartialEq)]
struct CursorTraceReadoutRow {
    trace_index: usize,
    trace_color: Color32,
    c1_value: Option<f64>,
    c2_value: Option<f64>,
}

fn trace_value_at_cursor(trace: &TraceData, cursor_x: f64) -> Option<f64> {
    if !cursor_x.is_finite() || trace.is_empty() {
        return None;
    }
    let (Some(x_min), Some(x_max)) = (trace.x_min(), trace.x_max()) else {
        return None;
    };
    if cursor_x < x_min || cursor_x > x_max {
        return None;
    }
    trace.interpolate_at(cursor_x).filter(|v| v.is_finite())
}

fn collect_cursor_trace_readouts(
    viewer_state: &WaveformViewerState,
    c1_x: Option<f64>,
    c2_x: Option<f64>,
) -> Vec<CursorTraceReadoutRow> {
    viewer_state
        .traces
        .iter()
        .enumerate()
        .filter(|(_, trace)| trace.visible && !trace.is_empty())
        .map(|(trace_index, trace)| CursorTraceReadoutRow {
            trace_index,
            trace_color: trace.style.to_color32(),
            c1_value: c1_x.and_then(|x| trace_value_at_cursor(trace, x)),
            c2_value: c2_x.and_then(|x| trace_value_at_cursor(trace, x)),
        })
        .collect()
}

fn render_cursor_info_panel(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    c1_x: Option<f64>,
    c2_x: Option<f64>,
) {
    let show_c1 = c1_x.is_some();
    let show_c2 = c2_x.is_some();
    if !show_c1 && !show_c2 {
        return;
    }

    let readouts = collect_cursor_trace_readouts(viewer_state, c1_x, c2_x);
    let value_cols = (show_c1 as usize) + (show_c2 as usize);
    let max_width_available = (layout.plot.width() - 2.0 * CURSOR_INFO_PANEL_MARGIN).max(120.0);
    let computed_width = CURSOR_INFO_PANEL_PADDING * 2.0
        + 10.0
        + CURSOR_INFO_PANEL_BASE_NAME_WIDTH
        + CURSOR_INFO_PANEL_VALUE_WIDTH * value_cols as f32;
    let panel_width = computed_width
        .clamp(CURSOR_INFO_PANEL_MIN_WIDTH, CURSOR_INFO_PANEL_MAX_WIDTH)
        .min(max_width_available);
    let name_col_width = (panel_width
        - CURSOR_INFO_PANEL_PADDING * 2.0
        - 10.0
        - CURSOR_INFO_PANEL_VALUE_WIDTH * value_cols as f32)
        .max(44.0);

    let meta_rows =
        1usize + (show_c1 as usize) + (show_c2 as usize) + if show_c1 && show_c2 { 2 } else { 0 };
    let max_panel_height = (layout.plot.height() - 2.0 * CURSOR_INFO_PANEL_MARGIN).max(120.0);
    let fixed_height = CURSOR_INFO_PANEL_PADDING * 2.0
        + CURSOR_INFO_PANEL_ROW_HEIGHT * (meta_rows as f32 + 1.0)
        + 6.0;
    let trace_row_capacity = ((max_panel_height - fixed_height) / CURSOR_INFO_PANEL_ROW_HEIGHT)
        .floor()
        .max(1.0) as usize;

    let mut visible_trace_rows = if readouts.is_empty() {
        0
    } else {
        readouts.len().min(trace_row_capacity)
    };
    let mut overflow_count = readouts.len().saturating_sub(visible_trace_rows);
    if overflow_count > 0 && visible_trace_rows > 0 {
        visible_trace_rows = visible_trace_rows.saturating_sub(1);
        overflow_count = readouts.len().saturating_sub(visible_trace_rows);
    }
    let rendered_trace_rows = if readouts.is_empty() {
        1
    } else {
        visible_trace_rows + usize::from(overflow_count > 0)
    };

    let panel_height = CURSOR_INFO_PANEL_PADDING * 2.0
        + CURSOR_INFO_PANEL_ROW_HEIGHT * (meta_rows + rendered_trace_rows + 1) as f32
        + 6.0;
    let panel_rect = Rect::from_min_size(
        Pos2::new(
            layout.plot.max.x - CURSOR_INFO_PANEL_MARGIN - panel_width,
            layout.plot.min.y + CURSOR_INFO_PANEL_MARGIN,
        ),
        Vec2::new(panel_width, panel_height.min(max_panel_height)),
    );

    painter.rect_filled(
        panel_rect,
        Rounding::same(4.0),
        Color32::from_rgba_unmultiplied(30, 33, 40, 224),
    );
    painter.rect_stroke(
        panel_rect,
        Rounding::same(4.0),
        Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
    );

    let text_left = panel_rect.min.x + CURSOR_INFO_PANEL_PADDING;
    let row_top = panel_rect.min.y + CURSOR_INFO_PANEL_PADDING;
    let label_color = Color32::from_rgb(130, 136, 148);
    let value_color = Color32::from_rgb(198, 204, 216);
    let header_color = Color32::from_rgb(168, 174, 186);
    let title_font = FontId::proportional(10.5);
    let label_font = FontId::proportional(9.0);
    let value_font = FontId::proportional(10.0);

    let mut y = row_top;
    let draw_meta = |painter: &Painter, y: f32, label: &str, value: String| {
        painter.text(
            Pos2::new(text_left, y),
            egui::Align2::LEFT_TOP,
            label,
            label_font.clone(),
            label_color,
        );
        painter.text(
            Pos2::new(text_left + 58.0, y),
            egui::Align2::LEFT_TOP,
            value,
            value_font.clone(),
            value_color,
        );
    };

    painter.text(
        Pos2::new(text_left, y),
        egui::Align2::LEFT_TOP,
        "Cursor Readout",
        title_font,
        Color32::from_rgb(200, 206, 218),
    );
    y += CURSOR_INFO_PANEL_ROW_HEIGHT;

    if let Some(x1) = c1_x {
        draw_meta(painter, y, "C1", axis::format_time(x1));
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
    }
    if let Some(x2) = c2_x {
        draw_meta(painter, y, "C2", axis::format_time(x2));
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
    }
    if let (Some(x1), Some(x2)) = (c1_x, c2_x) {
        let delta = (x2 - x1).abs();
        draw_meta(painter, y, "dT", axis::format_time_delta(delta));
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
        draw_meta(
            painter,
            y,
            "f",
            axis::format_frequency(axis::period_to_frequency(delta)),
        );
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
    }

    let separator_y = y - 2.0;
    painter.line_segment(
        [
            Pos2::new(panel_rect.min.x + 4.0, separator_y),
            Pos2::new(panel_rect.max.x - 4.0, separator_y),
        ],
        Stroke::new(1.0, Color32::from_rgb(52, 56, 65)),
    );
    y += 4.0;

    let swatch_x = text_left;
    let name_x = swatch_x + 12.0;
    let c1_col_x = if show_c1 && show_c2 {
        panel_rect.max.x - CURSOR_INFO_PANEL_PADDING - CURSOR_INFO_PANEL_VALUE_WIDTH * 2.0
    } else {
        panel_rect.max.x - CURSOR_INFO_PANEL_PADDING - CURSOR_INFO_PANEL_VALUE_WIDTH
    };
    let c2_col_x = panel_rect.max.x - CURSOR_INFO_PANEL_PADDING - CURSOR_INFO_PANEL_VALUE_WIDTH;

    painter.text(
        Pos2::new(name_x, y),
        egui::Align2::LEFT_TOP,
        "Trace",
        label_font.clone(),
        header_color,
    );
    if show_c1 {
        painter.text(
            Pos2::new(c1_col_x, y),
            egui::Align2::LEFT_TOP,
            "C1",
            label_font.clone(),
            header_color,
        );
    }
    if show_c2 {
        painter.text(
            Pos2::new(c2_col_x, y),
            egui::Align2::LEFT_TOP,
            "C2",
            label_font.clone(),
            header_color,
        );
    }
    y += CURSOR_INFO_PANEL_ROW_HEIGHT;

    let y_unit = if viewer_state.y_axis_unit.is_empty() {
        "V"
    } else {
        viewer_state.y_axis_unit.as_str()
    };
    let name_font = FontId::proportional(10.0);

    if readouts.is_empty() {
        painter.text(
            Pos2::new(name_x, y),
            egui::Align2::LEFT_TOP,
            "No visible traces",
            value_font.clone(),
            label_color,
        );
        return;
    }

    for row in readouts.iter().take(visible_trace_rows) {
        let swatch_rect = Rect::from_min_size(
            Pos2::new(swatch_x, y + 3.0),
            Vec2::new(8.0, CURSOR_INFO_PANEL_ROW_HEIGHT - 6.0),
        );
        painter.rect_filled(swatch_rect, Rounding::same(1.0), row.trace_color);

        let trace_name = viewer_state
            .traces
            .get(row.trace_index)
            .map(|trace| trace.name.as_str())
            .unwrap_or("?");
        let display_name =
            truncate_legend_trace_name(painter, trace_name, name_font.clone(), name_col_width);
        painter.text(
            Pos2::new(name_x, y),
            egui::Align2::LEFT_TOP,
            display_name,
            name_font.clone(),
            value_color,
        );

        if show_c1 {
            painter.text(
                Pos2::new(c1_col_x, y),
                egui::Align2::LEFT_TOP,
                format_optional_value(row.c1_value, y_unit),
                value_font.clone(),
                value_color,
            );
        }
        if show_c2 {
            painter.text(
                Pos2::new(c2_col_x, y),
                egui::Align2::LEFT_TOP,
                format_optional_value(row.c2_value, y_unit),
                value_font.clone(),
                value_color,
            );
        }
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
    }

    if overflow_count > 0 {
        painter.text(
            Pos2::new(name_x, y),
            egui::Align2::LEFT_TOP,
            format!("+{} more", overflow_count),
            label_font,
            label_color,
        );
    }
}

pub(super) fn render_cursors(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    _clip: Rect,
) {
    let view = &viewer_state.view;
    let cursors = &viewer_state.cursors;
    let marker_count = viewer_state.markers.len();
    let mut labels: Vec<WaveformCursorLabelSpec> = Vec::with_capacity(2 + marker_count);
    let mut line_x_positions: Vec<f32> = Vec::with_capacity(2 + marker_count);

    // Cursor 1
    if let Some(x1) = cursors.cursor1_x {
        let screen_x =
            layout.plot.min.x + ((x1 - view.x_min) / view.x_range()) as f32 * layout.plot.width();

        // Vertical line
        painter.line_segment(
            [
                Pos2::new(screen_x, layout.plot.min.y),
                Pos2::new(screen_x, layout.plot.max.y),
            ],
            Stroke::new(1.5, cursor1_color()),
        );
        line_x_positions.push(screen_x);
        labels.push(WaveformCursorLabelSpec {
            anchor_x: screen_x,
            text: format!("C1 {}", axis::format_time(x1)),
            color: cursor1_color(),
            font: FontId::proportional(CURSOR_LABEL_FONT_SIZE),
        });
    }

    // Cursor 2
    if let Some(x2) = cursors.cursor2_x {
        let screen_x =
            layout.plot.min.x + ((x2 - view.x_min) / view.x_range()) as f32 * layout.plot.width();

        painter.line_segment(
            [
                Pos2::new(screen_x, layout.plot.min.y),
                Pos2::new(screen_x, layout.plot.max.y),
            ],
            Stroke::new(1.5, cursor2_color()),
        );
        line_x_positions.push(screen_x);
        labels.push(WaveformCursorLabelSpec {
            anchor_x: screen_x,
            text: format!("C2 {}", axis::format_time(x2)),
            color: cursor2_color(),
            font: FontId::proportional(CURSOR_LABEL_FONT_SIZE),
        });
    }

    for (idx, marker_x) in viewer_state.markers.iter().enumerate() {
        let screen_x = layout.plot.min.x
            + ((*marker_x - view.x_min) / view.x_range()) as f32 * layout.plot.width();
        if !screen_x.is_finite() || screen_x < layout.plot.min.x || screen_x > layout.plot.max.x {
            continue;
        }
        let color = marker_color(idx);
        painter.line_segment(
            [
                Pos2::new(screen_x, layout.plot.min.y),
                Pos2::new(screen_x, layout.plot.max.y),
            ],
            Stroke::new(1.0, color),
        );
        line_x_positions.push(screen_x);
        labels.push(WaveformCursorLabelSpec {
            anchor_x: screen_x,
            text: format!("M{} {}", idx + 1, axis::format_time(*marker_x)),
            color,
            font: FontId::proportional(CURSOR_LABEL_FONT_SIZE),
        });
    }

    if !labels.is_empty() {
        render_waveform_cursor_labels(painter, layout, viewer_state, &labels, &line_x_positions);
    }
    render_cursor_info_panel(
        painter,
        layout,
        viewer_state,
        cursors.cursor1_x,
        cursors.cursor2_x,
    );
}

#[derive(Debug, Clone)]
struct WaveformCursorLabelSpec {
    anchor_x: f32,
    text: String,
    color: Color32,
    font: FontId,
}

fn render_waveform_cursor_labels(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    labels: &[WaveformCursorLabelSpec],
    line_x_positions: &[f32],
) {
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

    let placements =
        layout_waveform_cursor_labels(layout, viewer_state, &requests, line_x_positions);
    for (label, placement) in labels.iter().zip(placements.iter()) {
        draw_waveform_cursor_label(painter, label, placement);
    }
}

fn layout_waveform_cursor_labels(
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    requests: &[VerticalLabelRequest],
    line_x_positions: &[f32],
) -> Vec<VerticalLabelPlacement> {
    let config = VerticalLabelLayoutConfig {
        line_clearance: 4.0,
        top_margin: 2.0,
        row_gap: 3.0,
        preferred_rows: 5,
        nudge_step: 8.0,
        nudge_steps: 10,
        label_gap: 2.0,
    };
    let max_h = requests.iter().fold(0.0f32, |acc, r| acc.max(r.size.y));
    let search_band_bottom = layout.plot.min.y
        + config.top_margin
        + (max_h + config.row_gap) * config.preferred_rows as f32
        + 4.0;
    let obstacles =
        collect_waveform_cursor_label_obstacles(layout, viewer_state, search_band_bottom);
    place_vertical_line_labels(layout.plot, requests, line_x_positions, &obstacles, config)
}

fn collect_waveform_cursor_label_obstacles(
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    band_bottom: f32,
) -> Vec<Rect> {
    let mut obstacles = Vec::new();
    if !band_bottom.is_finite() || band_bottom <= layout.plot.min.y {
        return obstacles;
    }

    let view = &viewer_state.view;
    if view.x_range() <= 0.0 || view.y_range() <= 0.0 {
        return obstacles;
    }

    let band_bottom = band_bottom.min(layout.plot.max.y);
    for trace in &viewer_state.traces {
        if !trace.visible || trace.is_empty() {
            continue;
        }
        let n = trace.len();
        let step = (n / 600).max(1);
        for idx in (0..n).step_by(step) {
            let (Some(&x), Some(&y)) = (trace.x.get(idx), trace.y.get(idx)) else {
                continue;
            };
            if !x.is_finite() || !y.is_finite() || x < view.x_min || x > view.x_max {
                continue;
            }

            let screen_x = layout.plot.min.x
                + ((x - view.x_min) / view.x_range()) as f32 * layout.plot.width();
            let screen_y = layout.plot.min.y
                + ((view.y_max - y) / view.y_range()) as f32 * layout.plot.height();
            if !screen_x.is_finite() || !screen_y.is_finite() {
                continue;
            }
            if screen_x < layout.plot.min.x
                || screen_x > layout.plot.max.x
                || screen_y < layout.plot.min.y
                || screen_y > band_bottom
            {
                continue;
            }
            obstacles.push(Rect::from_center_size(
                Pos2::new(screen_x, screen_y),
                Vec2::splat(3.0),
            ));
        }
    }
    obstacles
}

fn draw_waveform_cursor_label(
    painter: &Painter,
    label: &WaveformCursorLabelSpec,
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
        Stroke::new(1.0, label.color.gamma_multiply(0.75)),
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
