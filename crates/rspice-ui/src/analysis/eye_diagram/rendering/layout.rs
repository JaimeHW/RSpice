use egui::{Color32, FontId, Painter, Pos2, Rect, Ui, Vec2};

use super::super::state::EyeDiagramState;

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

pub(super) fn preferred_measurements_pane_width(ui: &Ui, state: &EyeDiagramState) -> f32 {
    let painter = ui.painter();
    let label_font = FontId::proportional(10.0);
    let value_font = FontId::proportional(11.0);
    let label_color = Color32::from_rgb(120, 125, 135);
    let value_color = Color32::from_rgb(200, 205, 215);

    let labels = [
        "Data Rate",
        "UI",
        "Eye Height",
        "Eye Width",
        "Jitter (p-p)",
        "Rise Time",
        "Fall Time",
        "Q-Factor",
        "Est. BER",
        "SNR",
    ];
    let mut max_label_width = 0.0f32;
    for label in labels {
        let width = measure_text_width(
            painter,
            &format!("{}:", label),
            label_font.clone(),
            label_color,
        );
        max_label_width = max_label_width.max(width);
    }

    let m = &state.measurements;
    let mut values = vec![
        m.format_data_rate(),
        format!("{:.2} ps", m.unit_interval * 1e12),
        m.format_height(),
        m.format_width(),
        m.format_jitter(),
        m.format_rise_time(),
        m.format_fall_time(),
        format!("{:.2}", m.q_factor),
        m.format_ber(),
        format!("{:.1} dB", m.snr_db),
        "No data".to_string(),
    ];
    if state.show_mask {
        values.push(format!("Mask: {}", state.mask_result_string()));
    }

    let mut max_value_width = 0.0f32;
    for value in &values {
        let width = measure_text_width(painter, value, value_font.clone(), value_color);
        max_value_width = max_value_width.max(width);
    }

    max_label_width
        + 8.0
        + max_value_width
        + MEASUREMENTS_PANEL_PADDING * 2.0
        + MEASUREMENTS_SCROLLBAR_ALLOWANCE
}

// =============================================================================
// Layout
// =============================================================================

#[derive(Debug, Clone)]
pub(super) struct EyeLayout {
    pub(super) total: Rect,
    pub(super) header: Rect,
    pub(super) chart: Rect,
    pub(super) measurements: Option<Rect>,
}

pub(super) const HEADER_HEIGHT: f32 = 32.0;
pub(super) const MEASUREMENTS_WIDTH_MIN: f32 = 180.0;
pub(super) const MEASUREMENTS_WIDTH_MAX: f32 = 420.0;
pub(super) const MEASUREMENTS_WIDTH_FRACTION: f32 = 0.22;
pub(super) const MEASUREMENTS_WIDTH_MAX_FRACTION: f32 = 0.45;
pub(super) const EYE_CHART_MIN_WIDTH: f32 = 220.0;
pub(super) const MEASUREMENTS_SPLITTER_HIT_WIDTH: f32 = 8.0;
pub(super) const MEASUREMENTS_SPLITTER_STROKE_WIDTH: f32 = 1.0;
pub(super) const MEASUREMENTS_SCROLLBAR_ALLOWANCE: f32 = 14.0;
pub(super) const CHART_LEFT_PADDING: f32 = 8.0;
pub(super) const CHART_RIGHT_PADDING: f32 = 0.0;
pub(super) const CHART_TOP_GAP: f32 = 0.0;
pub(super) const CHART_BOTTOM_PADDING: f32 = 8.0;
pub(super) const MEASUREMENTS_PANEL_PADDING: f32 = 8.0;
pub(super) const AXIS_LEFT_GUTTER: f32 = 52.0;
pub(super) const AXIS_RIGHT_GUTTER: f32 = 2.0;
pub(super) const AXIS_TOP_GUTTER: f32 = 2.0;
pub(super) const AXIS_BOTTOM_GUTTER: f32 = 30.0;
pub(super) const AXIS_TITLE_MIN_LEFT_INSET: f32 = 2.0;
pub(super) const AXIS_TITLE_TO_VALUE_LABEL_GAP: f32 = 6.0;
pub(super) const AXIS_TITLE_BOTTOM_INSET: f32 = 2.0;
pub(super) const AXIS_TICK_X_OFFSET: f32 = 2.0;
pub(super) const AXIS_TICK_Y_OFFSET: f32 = 2.0;
pub(super) const MAX_MAJOR_TICKS: usize = 50;
pub(super) const MAX_MINOR_TICKS: usize = 250;

#[allow(dead_code)]
pub(super) fn calculate_layout(available: Rect) -> EyeLayout {
    let width = clamp_measurements_pane_width(
        available,
        (available.width() * MEASUREMENTS_WIDTH_FRACTION)
            .clamp(MEASUREMENTS_WIDTH_MIN, MEASUREMENTS_WIDTH_MAX),
    );
    calculate_layout_with_measurements_width(available, true, width)
}

pub(super) fn calculate_layout_with_measurements_width(
    available: Rect,
    show_measurements: bool,
    measurements_width: f32,
) -> EyeLayout {
    let total = available;

    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));
    let content_top = header.max.y;
    let content_height = (total.height() - HEADER_HEIGHT).max(0.0);

    let measurements = if show_measurements {
        let width = clamp_measurements_pane_width(total, measurements_width);
        Some(Rect::from_min_size(
            Pos2::new(total.max.x - width, content_top),
            Vec2::new(width, content_height),
        ))
    } else {
        None
    };

    let chart_right = measurements
        .as_ref()
        .map(|rect| rect.min.x - CHART_RIGHT_PADDING)
        .unwrap_or(total.max.x - CHART_RIGHT_PADDING);

    let chart = Rect::from_min_max(
        Pos2::new(
            total.min.x + CHART_LEFT_PADDING,
            content_top + CHART_TOP_GAP,
        ),
        Pos2::new(chart_right, total.max.y - CHART_BOTTOM_PADDING),
    );

    EyeLayout {
        total,
        header,
        chart,
        measurements,
    }
}

pub(super) fn measurements_pane_width_bounds(total: Rect) -> (f32, f32) {
    let min = MEASUREMENTS_WIDTH_MIN;
    let max_by_fraction = (total.width() * MEASUREMENTS_WIDTH_MAX_FRACTION).max(min);
    let max_by_chart = (total.width() - CHART_LEFT_PADDING - EYE_CHART_MIN_WIDTH).max(min);
    let max = max_by_fraction
        .min(max_by_chart)
        .min(MEASUREMENTS_WIDTH_MAX)
        .max(min);
    (min, max)
}

pub(super) fn eye_plot_rect(chart_rect: Rect) -> Rect {
    let min_x = (chart_rect.min.x + AXIS_LEFT_GUTTER).min(chart_rect.max.x - 1.0);
    let max_x = (chart_rect.max.x - AXIS_RIGHT_GUTTER).max(min_x + 1.0);
    let min_y = (chart_rect.min.y + AXIS_TOP_GUTTER).min(chart_rect.max.y - 1.0);
    let max_y = (chart_rect.max.y - AXIS_BOTTOM_GUTTER).max(min_y + 1.0);
    Rect::from_min_max(Pos2::new(min_x, min_y), Pos2::new(max_x, max_y))
}

pub(super) fn x_tick_label_position(x: f32, plot_rect: Rect) -> Pos2 {
    Pos2::new(x, plot_rect.max.y + AXIS_TICK_Y_OFFSET)
}

pub(super) fn y_tick_label_position(y: f32, plot_rect: Rect) -> Pos2 {
    Pos2::new(plot_rect.min.x - AXIS_TICK_X_OFFSET, y)
}

pub(super) fn x_axis_title_position(chart_rect: Rect, plot_rect: Rect) -> Pos2 {
    Pos2::new(
        plot_rect.center().x,
        chart_rect.max.y - AXIS_TITLE_BOTTOM_INSET,
    )
}

pub(super) fn y_axis_title_position(
    chart_rect: Rect,
    plot_rect: Rect,
    max_y_tick_label_width: f32,
    y_title_width: f32,
) -> Pos2 {
    let y_tick_anchor_x = y_tick_label_position(plot_rect.center().y, plot_rect).x;
    let y_tick_left_edge = y_tick_anchor_x - max_y_tick_label_width.max(0.0);
    let title_left = (y_tick_left_edge - AXIS_TITLE_TO_VALUE_LABEL_GAP - y_title_width)
        .max(chart_rect.min.x + AXIS_TITLE_MIN_LEFT_INSET);
    Pos2::new(title_left, plot_rect.center().y)
}

pub(super) fn clamp_measurements_pane_width(total: Rect, width: f32) -> f32 {
    let (min, max) = measurements_pane_width_bounds(total);
    width.clamp(min, max)
}

pub(super) fn resolve_measurements_pane_width(
    total: Rect,
    manual_width: Option<f32>,
    auto_width: f32,
) -> f32 {
    let base = (total.width() * MEASUREMENTS_WIDTH_FRACTION)
        .clamp(MEASUREMENTS_WIDTH_MIN, MEASUREMENTS_WIDTH_MAX);
    let desired = manual_width.unwrap_or_else(|| base.max(auto_width));
    clamp_measurements_pane_width(total, desired)
}

// =============================================================================
