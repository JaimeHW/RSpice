use egui::{Color32, FontId, Painter, Pos2, Rect, Stroke};

use super::super::state::EyeDiagramState;
use super::layout::{
    MAX_MAJOR_TICKS, MAX_MINOR_TICKS, measure_text_width, x_axis_title_position,
    x_tick_label_position, y_axis_title_position, y_tick_label_position,
};
use super::style::{center_line_color, grid_color, text_color};

#[derive(Debug, Clone)]
struct AxisTick {
    value: f64,
    label: String,
    major: bool,
}

#[derive(Debug, Clone, Default)]
pub(super) struct AxisTicks {
    ticks: Vec<AxisTick>,
    unit: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct GridLabelMetrics {
    max_y_tick_label_width: f32,
}

pub(super) fn render_grid(
    painter: &Painter,
    plot_rect: Rect,
    state: &EyeDiagramState,
    x_ticks: &AxisTicks,
    y_ticks: &AxisTicks,
) -> GridLabelMetrics {
    let mut metrics = GridLabelMetrics::default();
    let tick_font = FontId::proportional(9.0);

    for tick in &x_ticks.ticks {
        let x = eye_time_to_x(tick.value, plot_rect, state);
        if !x.is_finite() || x < plot_rect.min.x || x > plot_rect.max.x {
            continue;
        }
        let stroke = if tick.major {
            Stroke::new(1.0, grid_color())
        } else {
            Stroke::new(0.5, Color32::from_rgb(30, 34, 44))
        };
        painter.line_segment(
            [Pos2::new(x, plot_rect.min.y), Pos2::new(x, plot_rect.max.y)],
            stroke,
        );
        if tick.major {
            painter.text(
                x_tick_label_position(x, plot_rect),
                egui::Align2::CENTER_TOP,
                &tick.label,
                tick_font.clone(),
                text_color(),
            );
        }
    }

    for tick in &y_ticks.ticks {
        let y = eye_voltage_to_y(tick.value, plot_rect, state);
        if !y.is_finite() || y < plot_rect.min.y || y > plot_rect.max.y {
            continue;
        }
        let stroke = if tick.major {
            Stroke::new(1.0, grid_color())
        } else {
            Stroke::new(0.5, Color32::from_rgb(30, 34, 44))
        };
        painter.line_segment(
            [Pos2::new(plot_rect.min.x, y), Pos2::new(plot_rect.max.x, y)],
            stroke,
        );
        if tick.major {
            let width = measure_text_width(painter, &tick.label, tick_font.clone(), text_color());
            metrics.max_y_tick_label_width = metrics.max_y_tick_label_width.max(width);
            painter.text(
                y_tick_label_position(y, plot_rect),
                egui::Align2::RIGHT_CENTER,
                &tick.label,
                tick_font.clone(),
                text_color(),
            );
        }
    }

    metrics
}

pub(super) fn render_axes(
    painter: &Painter,
    chart_rect: Rect,
    plot_rect: Rect,
    x_ticks: &AxisTicks,
    y_ticks: &AxisTicks,
    metrics: GridLabelMetrics,
) {
    let x_title = format!("Time ({})", x_ticks.unit);
    let y_title = format!("Amplitude ({})", y_ticks.unit);
    let y_font = FontId::proportional(10.0);
    let y_title_width = measure_text_width(painter, &y_title, y_font.clone(), text_color());

    painter.text(
        x_axis_title_position(chart_rect, plot_rect),
        egui::Align2::CENTER_BOTTOM,
        x_title,
        FontId::proportional(10.0),
        text_color(),
    );
    painter.text(
        y_axis_title_position(
            chart_rect,
            plot_rect,
            metrics.max_y_tick_label_width,
            y_title_width,
        ),
        egui::Align2::LEFT_CENTER,
        y_title,
        y_font,
        text_color(),
    );
}

pub(super) fn render_center_lines(painter: &Painter, rect: Rect, _state: &EyeDiagramState) {
    let stroke = Stroke::new(1.0, center_line_color());

    // Vertical center (mid-UI)
    let cx = rect.center().x;
    painter.line_segment(
        [Pos2::new(cx, rect.min.y), Pos2::new(cx, rect.max.y)],
        stroke,
    );

    // Horizontal center (crossing level)
    let cy = rect.center().y;
    painter.line_segment(
        [Pos2::new(rect.min.x, cy), Pos2::new(rect.max.x, cy)],
        stroke,
    );
}

pub(super) fn eye_time_ticks(state: &EyeDiagramState, target_major_ticks: usize) -> AxisTicks {
    let (time_min, time_max) = eye_view_time_bounds(state);
    if !time_min.is_finite() || !time_max.is_finite() || time_max <= time_min {
        return AxisTicks {
            ticks: Vec::new(),
            unit: "s".to_string(),
        };
    }

    let tick_spec = crate::waveform::axis::calculate_ticks(time_min, time_max, target_major_ticks);
    let unit = crate::waveform::axis::format_axis_unit("s", tick_spec.prefix);
    let mut ticks = Vec::with_capacity(
        tick_spec
            .major_ticks
            .len()
            .saturating_add(tick_spec.minor_ticks.len())
            .min(MAX_MAJOR_TICKS + MAX_MINOR_TICKS),
    );

    for value in tick_spec.major_ticks.iter().copied().take(MAX_MAJOR_TICKS) {
        ticks.push(AxisTick {
            value,
            label: crate::waveform::axis::format_axis_value(
                value,
                tick_spec.scale,
                tick_spec.precision,
            ),
            major: true,
        });
    }
    for value in tick_spec.minor_ticks.iter().copied().take(MAX_MINOR_TICKS) {
        ticks.push(AxisTick {
            value,
            label: String::new(),
            major: false,
        });
    }
    ticks.sort_by(|a, b| a.value.total_cmp(&b.value));

    AxisTicks { ticks, unit }
}

pub(super) fn eye_voltage_ticks(state: &EyeDiagramState, target_major_ticks: usize) -> AxisTicks {
    let (v_min, v_max) = eye_view_voltage_bounds(state);
    if !v_min.is_finite() || !v_max.is_finite() || v_max <= v_min {
        return AxisTicks {
            ticks: Vec::new(),
            unit: "V".to_string(),
        };
    }

    let tick_spec = crate::waveform::axis::calculate_ticks(v_min, v_max, target_major_ticks);
    let unit = crate::waveform::axis::format_axis_unit("V", tick_spec.prefix);
    let mut ticks = Vec::with_capacity(
        tick_spec
            .major_ticks
            .len()
            .saturating_add(tick_spec.minor_ticks.len())
            .min(MAX_MAJOR_TICKS + MAX_MINOR_TICKS),
    );
    for value in tick_spec.major_ticks.iter().copied().take(MAX_MAJOR_TICKS) {
        ticks.push(AxisTick {
            value,
            label: crate::waveform::axis::format_axis_value(
                value,
                tick_spec.scale,
                tick_spec.precision,
            ),
            major: true,
        });
    }
    for value in tick_spec.minor_ticks.iter().copied().take(MAX_MINOR_TICKS) {
        ticks.push(AxisTick {
            value,
            label: String::new(),
            major: false,
        });
    }
    ticks.sort_by(|a, b| a.value.total_cmp(&b.value));

    AxisTicks { ticks, unit }
}

pub(super) fn eye_full_time_range_seconds(state: &EyeDiagramState) -> f64 {
    state.full_time_span_seconds()
}

pub(super) fn eye_view_time_bounds(state: &EyeDiagramState) -> (f64, f64) {
    (state.view.time_min_s, state.view.time_max_s)
}

pub(super) fn eye_view_voltage_bounds(state: &EyeDiagramState) -> (f64, f64) {
    (state.view.voltage_min, state.view.voltage_max)
}

pub(super) fn trace_time_to_seconds(state: &EyeDiagramState, time_ui: f64) -> f64 {
    time_ui * state.data.bit_period.max(1e-18)
}

pub(super) fn eye_time_to_x(time_seconds: f64, plot_rect: Rect, state: &EyeDiagramState) -> f32 {
    let (min, max) = eye_view_time_bounds(state);
    let range = max - min;
    if !range.is_finite() || range <= 0.0 {
        return plot_rect.center().x;
    }
    let t = ((time_seconds - min) / range).clamp(0.0, 1.0);
    plot_rect.min.x + (t as f32) * plot_rect.width()
}

pub(super) fn eye_voltage_to_y(voltage: f64, plot_rect: Rect, state: &EyeDiagramState) -> f32 {
    let (v_min, v_max) = eye_view_voltage_bounds(state);
    let range = v_max - v_min;
    if !range.is_finite() || range <= 0.0 {
        return plot_rect.center().y;
    }
    let t = ((voltage - v_min) / range).clamp(0.0, 1.0);
    plot_rect.max.y - (t as f32) * plot_rect.height()
}

pub(super) fn x_to_eye_time(x: f32, plot_rect: Rect, state: &EyeDiagramState) -> f64 {
    if plot_rect.width() <= 0.0 {
        return eye_view_time_bounds(state).0;
    }
    let t = ((x - plot_rect.min.x) / plot_rect.width()).clamp(0.0, 1.0) as f64;
    let (min, max) = eye_view_time_bounds(state);
    min + t * (max - min)
}

pub(super) fn y_to_eye_voltage(y: f32, plot_rect: Rect, state: &EyeDiagramState) -> f64 {
    if plot_rect.height() <= 0.0 {
        return eye_view_voltage_bounds(state).0;
    }
    let t = ((y - plot_rect.min.y) / plot_rect.height()).clamp(0.0, 1.0) as f64;
    let (min, max) = eye_view_voltage_bounds(state);
    max - t * (max - min)
}
