use super::{
    AXIS_TICK_X_OFFSET, AXIS_TICK_Y_OFFSET, AXIS_TITLE_BOTTOM_INSET, AXIS_TITLE_MIN_LEFT_INSET,
    AXIS_TITLE_TO_VALUE_LABEL_GAP, CHART_TOP_GAP, HEADER_HEIGHT, LEGEND_FIND_EDIT_MIN_WIDTH,
    LEGEND_FIND_RIGHT_GUARD, LEGEND_INSET_X, LEGEND_MIN_PLOT_WIDTH, LEGEND_SCROLLBAR_ALLOWANCE,
    LEGEND_TEXT_TRUNCATION_PADDING, LEGEND_TRACE_CONTROL_WIDTH, LEGEND_TRACE_SOLO_WIDTH,
    LEGEND_TRACE_SWATCH_WIDTH, LEGEND_WIDTH_FRACTION, LEGEND_WIDTH_MAX, LEGEND_WIDTH_MAX_FRACTION,
    LEGEND_WIDTH_MIN, X_AXIS_HEIGHT, Y_AXIS_WIDTH,
};
use crate::waveform::{
    axis,
    state::{DataBounds, ViewTransform, WaveformViewerState},
};
use egui::{Color32, FontId, Painter, Pos2, Rect, Ui, Vec2};

/// Layout regions for the waveform viewer.
#[derive(Debug, Clone)]
pub(super) struct ViewerLayout {
    pub total: Rect,
    pub header: Rect,
    pub y_axis: Rect,
    pub plot: Rect,
    pub x_axis: Rect,
    pub legend: Rect,
}

#[allow(dead_code)]
pub(super) fn calculate_layout(available: Rect) -> ViewerLayout {
    let legend_width = clamp_waveform_right_pane_width(
        available,
        (available.width() * LEGEND_WIDTH_FRACTION).clamp(LEGEND_WIDTH_MIN, LEGEND_WIDTH_MAX),
    );
    calculate_layout_with_legend_width(available, legend_width)
}

pub(super) fn calculate_layout_with_legend_width(
    available: Rect,
    legend_width: f32,
) -> ViewerLayout {
    let total = available;
    let legend_width = clamp_waveform_right_pane_width(total, legend_width);

    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));
    let content_top = header.max.y + CHART_TOP_GAP;
    let content_height = (total.height() - HEADER_HEIGHT - CHART_TOP_GAP).max(0.0);
    let chart_height = (content_height - X_AXIS_HEIGHT).max(0.0);

    let legend = Rect::from_min_size(
        Pos2::new(total.max.x - legend_width, content_top),
        Vec2::new(legend_width, content_height),
    );
    let x_axis = Rect::from_min_size(
        Pos2::new(total.min.x + Y_AXIS_WIDTH, total.max.y - X_AXIS_HEIGHT),
        Vec2::new(
            (total.width() - Y_AXIS_WIDTH - legend_width).max(0.0),
            X_AXIS_HEIGHT,
        ),
    );
    let y_axis = Rect::from_min_size(
        Pos2::new(total.min.x, content_top),
        Vec2::new(Y_AXIS_WIDTH, chart_height),
    );
    let plot = Rect::from_min_size(
        Pos2::new(total.min.x + Y_AXIS_WIDTH, content_top),
        Vec2::new(
            (total.width() - Y_AXIS_WIDTH - legend_width).max(0.0),
            chart_height,
        ),
    );

    ViewerLayout {
        total,
        header,
        y_axis,
        plot,
        x_axis,
        legend,
    }
}

pub(super) fn x_axis_title_position(layout: &ViewerLayout) -> Pos2 {
    Pos2::new(
        layout.x_axis.center().x,
        layout.x_axis.max.y - AXIS_TITLE_BOTTOM_INSET,
    )
}

pub(super) fn y_axis_title_position(
    layout: &ViewerLayout,
    max_y_tick_label_width: f32,
    y_title_width: f32,
) -> Pos2 {
    let y_tick_anchor_x = y_tick_label_position(layout, layout.plot.center().y).x;
    let y_tick_left_edge = y_tick_anchor_x - max_y_tick_label_width.max(0.0);
    let title_left = (y_tick_left_edge - AXIS_TITLE_TO_VALUE_LABEL_GAP - y_title_width)
        .max(layout.y_axis.min.x + AXIS_TITLE_MIN_LEFT_INSET);
    Pos2::new(title_left, layout.plot.center().y)
}

pub(super) fn x_tick_label_position(layout: &ViewerLayout, x: f32) -> Pos2 {
    Pos2::new(x, layout.plot.max.y + AXIS_TICK_Y_OFFSET)
}

pub(super) fn y_tick_label_position(layout: &ViewerLayout, y: f32) -> Pos2 {
    Pos2::new(layout.plot.min.x - AXIS_TICK_X_OFFSET, y)
}

pub(super) fn measure_text_width(
    painter: &Painter,
    text: &str,
    font: FontId,
    color: Color32,
) -> f32 {
    painter
        .layout_no_wrap(text.to_owned(), font, color)
        .size()
        .x
}

pub(super) fn measure_text_size(
    painter: &Painter,
    text: &str,
    font: FontId,
    color: Color32,
) -> Vec2 {
    painter.layout_no_wrap(text.to_owned(), font, color).size()
}

pub(super) fn y_axis_title_text(viewer_state: &WaveformViewerState, prefix: &str) -> String {
    let unit = if viewer_state.y_axis_unit.is_empty() {
        "V"
    } else {
        &viewer_state.y_axis_unit
    };
    axis::format_axis_unit(unit, prefix)
}

pub(super) fn waveform_right_pane_width_bounds(total: Rect) -> (f32, f32) {
    let min = LEGEND_WIDTH_MIN;
    let max_by_fraction = (total.width() * LEGEND_WIDTH_MAX_FRACTION).max(min);
    let max_by_plot = (total.width() - Y_AXIS_WIDTH - LEGEND_MIN_PLOT_WIDTH).max(min);
    let max = max_by_fraction
        .min(max_by_plot)
        .min(LEGEND_WIDTH_MAX)
        .max(min);
    (min, max)
}

pub(super) fn clamp_waveform_right_pane_width(total: Rect, width: f32) -> f32 {
    let (min, max) = waveform_right_pane_width_bounds(total);
    width.clamp(min, max)
}

pub(super) fn resolve_waveform_right_pane_width(
    total: Rect,
    manual_width: Option<f32>,
    auto_width: f32,
) -> f32 {
    let base = (total.width() * LEGEND_WIDTH_FRACTION).clamp(LEGEND_WIDTH_MIN, LEGEND_WIDTH_MAX);
    let desired = manual_width.unwrap_or_else(|| base.max(auto_width));
    clamp_waveform_right_pane_width(total, desired)
}

pub(super) fn button_width_for_text(
    painter: &Painter,
    text: &str,
    font: FontId,
    color: Color32,
) -> f32 {
    measure_text_width(painter, text, font, color) + 16.0
}

pub(super) fn preferred_waveform_right_pane_width(
    ui: &Ui,
    viewer_state: &WaveformViewerState,
) -> f32 {
    let painter = ui.painter();
    let label_color = Color32::from_rgb(120, 125, 135);
    let body_color = Color32::from_rgb(200, 205, 215);
    let label_font = FontId::proportional(10.0);
    let body_font = FontId::proportional(11.0);
    let spacing = 4.0;

    let show_row = measure_text_width(painter, "Show", label_font.clone(), label_color)
        + spacing
        + button_width_for_text(painter, "All", body_font.clone(), body_color)
        + spacing
        + button_width_for_text(painter, "Clear", body_font.clone(), body_color);
    let sort_row =
        measure_text_width(painter, "Sort", label_font.clone(), label_color) + spacing + 120.0;
    let find_row = measure_text_width(painter, "Find", label_font.clone(), label_color)
        + spacing
        + LEGEND_FIND_EDIT_MIN_WIDTH.max(76.0)
        + spacing
        + LEGEND_TRACE_SOLO_WIDTH
        + LEGEND_FIND_RIGHT_GUARD;

    let max_trace_name_width = viewer_state
        .traces
        .iter()
        .map(|trace| measure_text_width(painter, &trace.name, body_font.clone(), body_color))
        .fold(72.0f32, f32::max)
        .clamp(72.0, 240.0);
    let trace_row = LEGEND_TRACE_SWATCH_WIDTH
        + spacing
        + LEGEND_TRACE_CONTROL_WIDTH
        + spacing
        + LEGEND_TRACE_SOLO_WIDTH
        + spacing
        + max_trace_name_width
        + LEGEND_TEXT_TRUNCATION_PADDING;

    let marker_button_text = viewer_state
        .markers
        .iter()
        .copied()
        .last()
        .map(axis::format_time)
        .unwrap_or_else(|| "1.00 us".to_string());
    let marker_row = measure_text_width(painter, "M16", body_font.clone(), body_color)
        + spacing
        + button_width_for_text(painter, &marker_button_text, body_font.clone(), body_color)
        + spacing
        + button_width_for_text(painter, "x", body_font.clone(), body_color);
    let markers_hint = measure_text_width(
        painter,
        "Alt+LMB add, Alt+RMB remove",
        FontId::proportional(9.0),
        label_color,
    );

    let content_width = show_row
        .max(sort_row)
        .max(find_row)
        .max(trace_row)
        .max(marker_row)
        .max(markers_hint);
    content_width + LEGEND_SCROLLBAR_ALLOWANCE + LEGEND_INSET_X * 2.0
}

pub(super) fn next_waveform_right_pane_width(
    current_width: Option<f32>,
    fallback_layout_width: f32,
    drag_delta_x: f32,
    total: Rect,
) -> f32 {
    let base = current_width.unwrap_or(fallback_layout_width);
    clamp_waveform_right_pane_width(total, base - drag_delta_x)
}

pub(super) fn center_waveform_view_x_on_marker(
    view: &mut ViewTransform,
    bounds: &DataBounds,
    marker_x: f64,
) {
    if !marker_x.is_finite() {
        return;
    }
    let range = view.x_range();
    if !range.is_finite() || range <= 0.0 {
        return;
    }
    let half = range * 0.5;
    view.x_min = marker_x - half;
    view.x_max = marker_x + half;
    view.clamp_to_bounds(bounds);
}
