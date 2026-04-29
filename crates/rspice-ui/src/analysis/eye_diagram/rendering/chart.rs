use egui::{Color32, CursorIcon, Pos2, Rect, Rounding, Sense, Stroke, Ui};

use super::super::state::{EyeDiagramState, EyeDisplayMode};
use super::axes::{
    GridLabelMetrics, eye_time_ticks, eye_voltage_ticks, render_axes, render_center_lines,
    render_grid, x_to_eye_time, y_to_eye_voltage,
};
use super::cursors::render_eye_cursors_and_markers;
use super::layout::{
    EyeLayout, MEASUREMENTS_SPLITTER_HIT_WIDTH, MEASUREMENTS_SPLITTER_STROKE_WIDTH,
    clamp_measurements_pane_width, eye_plot_rect,
};
use super::mask::render_mask;
use super::style::{chart_bg_color, panel_bg_color, panel_border_color};
use super::traces::{render_single_trace, render_traces_overlay, render_traces_persistence};

pub(super) fn render_chart_area(ui: &mut Ui, layout: &EyeLayout, state: &mut EyeDiagramState) {
    let response = ui.allocate_rect(layout.chart, Sense::click_and_drag());
    render_chart_core(ui, layout, state);
    handle_eye_chart_interactions(ui, response, eye_plot_rect(layout.chart), state);
}

pub(super) fn handle_measurements_splitter(
    ui: &mut Ui,
    layout: &EyeLayout,
    state: &mut EyeDiagramState,
) {
    let Some(measurements_rect) = layout.measurements else {
        return;
    };
    let half_hit = MEASUREMENTS_SPLITTER_HIT_WIDTH * 0.5;
    let splitter_rect = Rect::from_min_max(
        Pos2::new(measurements_rect.min.x - half_hit, measurements_rect.min.y),
        Pos2::new(measurements_rect.min.x + half_hit, measurements_rect.max.y),
    );

    let splitter_id = ui.id().with("eye_measurements_splitter");
    let mut response = ui.interact(splitter_rect, splitter_id, Sense::click_and_drag());
    response = response.on_hover_cursor(CursorIcon::ResizeHorizontal);

    if response.double_clicked() {
        state.measurements_pane_width = None;
    }

    if response.dragged() {
        let delta_x = ui.ctx().input(|i| i.pointer.delta().x);
        let next = next_measurements_pane_width(
            state.measurements_pane_width,
            measurements_rect.width(),
            delta_x,
            layout.total,
        );
        state.measurements_pane_width = Some(next);
    }

    let stroke_color = if response.dragged() {
        Color32::from_rgb(115, 150, 220)
    } else if response.hovered() {
        Color32::from_rgb(90, 115, 165)
    } else {
        panel_border_color()
    };
    ui.painter().line_segment(
        [
            Pos2::new(measurements_rect.min.x, measurements_rect.min.y),
            Pos2::new(measurements_rect.min.x, measurements_rect.max.y),
        ],
        Stroke::new(MEASUREMENTS_SPLITTER_STROKE_WIDTH, stroke_color),
    );
}

pub(super) fn next_measurements_pane_width(
    current_width: Option<f32>,
    fallback_layout_width: f32,
    drag_delta_x: f32,
    total: Rect,
) -> f32 {
    let base = current_width.unwrap_or(fallback_layout_width);
    clamp_measurements_pane_width(total, base - drag_delta_x)
}

pub(super) fn handle_eye_chart_interactions(
    ui: &Ui,
    response: egui::Response,
    plot_rect: Rect,
    state: &mut EyeDiagramState,
) {
    if response.double_clicked() {
        state.reset_view_to_data();
        state.cursors.clear();
        state.clear_markers();
        state.invalidate_persistence_cache();
        return;
    }

    if response.clicked()
        && let Some(pos) = response.interact_pointer_pos()
        && plot_rect.contains(pos)
    {
        let modifiers = ui.input(|i| i.modifiers);
        let time_s = x_to_eye_time(pos.x, plot_rect, state);
        if modifiers.alt {
            state.add_marker(time_s);
        } else {
            state.cursors.place(time_s);
        }
    }

    if response.secondary_clicked()
        && let Some(pos) = response.interact_pointer_pos()
        && plot_rect.contains(pos)
    {
        let modifiers = ui.input(|i| i.modifiers);
        if modifiers.alt {
            let time_s = x_to_eye_time(pos.x, plot_rect, state);
            let tolerance = eye_marker_removal_tolerance_s(state, plot_rect, pos.x);
            state.remove_nearest_marker(time_s, tolerance);
        }
    }

    if response.hovered() {
        let scroll_y = ui.input(|i| i.raw_scroll_delta.y);
        if scroll_y.abs() > f32::EPSILON {
            let zoom = (1.0f64 - (scroll_y as f64) * 0.0015).clamp(0.5, 1.5);
            let pointer = response.hover_pos().unwrap_or(plot_rect.center());
            let center_time_s = x_to_eye_time(pointer.x, plot_rect, state);
            let center_voltage = y_to_eye_voltage(pointer.y, plot_rect, state);
            state.zoom_view(zoom, center_time_s, center_voltage);
            state.invalidate_persistence_cache();
        }
    }

    if response.dragged_by(egui::PointerButton::Primary) {
        if !response
            .hover_pos()
            .map(|pos| plot_rect.contains(pos))
            .unwrap_or(false)
        {
            return;
        }
        if ui.input(|i| i.modifiers.alt) {
            return;
        }
        let delta = ui.input(|i| i.pointer.delta());
        if delta.length_sq() > 0.0 {
            let dt = -(delta.x as f64 / plot_rect.width() as f64) * state.view.time_span();
            let dv = (delta.y as f64 / plot_rect.height() as f64) * state.view.voltage_span();
            state.pan_view(dt, dv);
            state.invalidate_persistence_cache();
        }
    }
}

pub(super) fn eye_marker_removal_tolerance_s(
    state: &EyeDiagramState,
    plot_rect: Rect,
    pointer_x: f32,
) -> f64 {
    let x_radius = (plot_rect.width() * 0.01).max(4.0);
    let x0 = (pointer_x - x_radius).clamp(plot_rect.min.x, plot_rect.max.x);
    let x1 = (pointer_x + x_radius).clamp(plot_rect.min.x, plot_rect.max.x);
    (x_to_eye_time(x1, plot_rect, state) - x_to_eye_time(x0, plot_rect, state))
        .abs()
        .max(1e-18)
}

pub(super) fn render_chart_core(ui: &mut Ui, layout: &EyeLayout, state: &mut EyeDiagramState) {
    let painter = ui.painter().clone();
    let chart_rect = layout.chart;
    let plot_rect = eye_plot_rect(chart_rect);
    let x_ticks = eye_time_ticks(state, 10);
    let y_ticks = eye_voltage_ticks(state, 8);

    painter.rect_filled(chart_rect, Rounding::ZERO, panel_bg_color());
    painter.rect_filled(plot_rect, Rounding::ZERO, chart_bg_color());

    let grid_metrics = if state.show_grid {
        render_grid(&painter, plot_rect, state, &x_ticks, &y_ticks)
    } else {
        GridLabelMetrics::default()
    };

    if state.show_mask && state.mask.enabled {
        render_mask(&painter, plot_rect, state);
    }

    render_center_lines(&painter, plot_rect, state);

    match state.mode {
        EyeDisplayMode::Overlay => render_traces_overlay(&painter, plot_rect, state),
        EyeDisplayMode::Persistence => render_traces_persistence(&painter, plot_rect, state),
        EyeDisplayMode::SingleTrace => render_single_trace(&painter, plot_rect, state),
    }

    render_eye_cursors_and_markers(&painter, plot_rect, state);

    render_axes(
        &painter,
        chart_rect,
        plot_rect,
        &x_ticks,
        &y_ticks,
        grid_metrics,
    );

    painter.rect_stroke(
        plot_rect,
        Rounding::ZERO,
        Stroke::new(1.0, panel_border_color()),
    );
}
