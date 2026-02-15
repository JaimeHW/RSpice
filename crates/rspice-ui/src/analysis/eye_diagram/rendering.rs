//! Eye Diagram Rendering
//!
//! Commercial-grade egui rendering for eye diagram visualization.
//! Supports overlay, persistence, and single-trace display modes.

use egui::{
    Color32, CursorIcon, FontId, Painter, Pos2, Rect, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2,
};

#[cfg(test)]
use super::data::EyeDataBuilder;
use super::data::EyeTrace;
use super::state::{
    ColorMap, EyeDiagramState, EyeDisplayMode, EyePersistenceCache, EyePersistenceCacheKey,
};
use crate::common::app::AppState;
use crate::common::viewer_style::{viewer_chart_bg_color, viewer_header_bg_color};

// =============================================================================
// Constants
// =============================================================================

fn chart_bg_color() -> Color32 {
    viewer_chart_bg_color()
}

fn grid_color() -> Color32 {
    Color32::from_rgb(40, 45, 55)
}

fn trace_color() -> Color32 {
    Color32::from_rgba_unmultiplied(50, 220, 100, 180)
}

fn mask_pass_color() -> Color32 {
    Color32::from_rgba_unmultiplied(0, 150, 0, 100)
}

fn mask_fail_color() -> Color32 {
    Color32::from_rgba_unmultiplied(200, 0, 0, 100)
}

fn mask_outline_color() -> Color32 {
    Color32::from_rgb(255, 200, 0)
}

fn center_line_color() -> Color32 {
    Color32::from_rgb(80, 85, 95)
}

fn text_color() -> Color32 {
    Color32::from_rgb(180, 185, 195)
}

fn panel_bg_color() -> Color32 {
    Color32::from_rgb(30, 33, 40)
}

fn panel_border_color() -> Color32 {
    Color32::from_rgb(60, 65, 75)
}

fn highlight_color() -> Color32 {
    Color32::from_rgb(100, 200, 255)
}

fn cursor1_color() -> Color32 {
    Color32::from_rgb(255, 235, 59)
}

fn cursor2_color() -> Color32 {
    Color32::from_rgb(76, 175, 80)
}

fn marker_color(index: usize) -> Color32 {
    const PALETTE: [Color32; 8] = [
        Color32::from_rgb(59, 130, 246),
        Color32::from_rgb(16, 185, 129),
        Color32::from_rgb(249, 115, 22),
        Color32::from_rgb(139, 92, 246),
        Color32::from_rgb(236, 72, 153),
        Color32::from_rgb(234, 179, 8),
        Color32::from_rgb(20, 184, 166),
        Color32::from_rgb(239, 68, 68),
    ];
    PALETTE[index % PALETTE.len()]
}

// =============================================================================
// Main Rendering Entry Point
// =============================================================================

/// Render the eye diagram viewer panel
pub fn render_eye_diagram_viewer(ui: &mut Ui, app_state: &mut AppState) {
    let available_rect = ui.available_rect_before_wrap();
    // Claim full available space so parent resizable panels keep user-set size
    // instead of snapping back to a content-driven natural size.
    let (_id, _rect) = ui.allocate_space(available_rect.size());
    let auto_width = preferred_measurements_pane_width(ui, &app_state.eye_diagram_state);
    app_state
        .eye_diagram_state
        .measurements_pane_auto_width_hint = auto_width;
    let measurements_width = resolve_measurements_pane_width(
        available_rect,
        app_state.eye_diagram_state.measurements_pane_width,
        auto_width,
    );
    if app_state
        .eye_diagram_state
        .measurements_pane_width
        .is_some()
    {
        app_state.eye_diagram_state.measurements_pane_width = Some(measurements_width);
    }

    // Calculate layout
    let layout = calculate_layout_with_measurements_width(
        available_rect,
        app_state.eye_diagram_state.show_measurements,
        measurements_width,
    );

    let close_requested = {
        let state = &mut app_state.eye_diagram_state;
        let close_requested = render_header(ui, &layout, state);
        handle_measurements_splitter(ui, &layout, state);
        render_chart_area(ui, &layout, state);
        render_measurements_panel(ui, &layout, state);
        close_requested
    };
    if close_requested {
        app_state.close_active_viewer();
    }
}

/// Public render function for external use
pub fn render_eye_diagram(ui: &mut Ui, state: &EyeDiagramState) {
    let available_rect = ui.available_rect_before_wrap();
    let (_id, _rect) = ui.allocate_space(available_rect.size());
    let auto_width = preferred_measurements_pane_width(ui, state);
    let measurements_width =
        resolve_measurements_pane_width(available_rect, state.measurements_pane_width, auto_width);
    let layout = calculate_layout_with_measurements_width(
        available_rect,
        state.show_measurements,
        measurements_width,
    );

    let mut state_copy = state.clone();
    render_chart_core(ui, &layout, &mut state_copy);
}

fn measure_text_width(painter: &Painter, text: &str, font: FontId, color: Color32) -> f32 {
    painter
        .layout_no_wrap(text.to_owned(), font, color)
        .size()
        .x
}

fn preferred_measurements_pane_width(ui: &Ui, state: &EyeDiagramState) -> f32 {
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
struct EyeLayout {
    total: Rect,
    header: Rect,
    chart: Rect,
    measurements: Option<Rect>,
}

const HEADER_HEIGHT: f32 = 32.0;
const MEASUREMENTS_WIDTH_MIN: f32 = 180.0;
const MEASUREMENTS_WIDTH_MAX: f32 = 420.0;
const MEASUREMENTS_WIDTH_FRACTION: f32 = 0.22;
const MEASUREMENTS_WIDTH_MAX_FRACTION: f32 = 0.45;
const EYE_CHART_MIN_WIDTH: f32 = 220.0;
const MEASUREMENTS_SPLITTER_HIT_WIDTH: f32 = 8.0;
const MEASUREMENTS_SPLITTER_STROKE_WIDTH: f32 = 1.0;
const MEASUREMENTS_SCROLLBAR_ALLOWANCE: f32 = 14.0;
const CHART_LEFT_PADDING: f32 = 8.0;
const CHART_RIGHT_PADDING: f32 = 0.0;
const CHART_TOP_GAP: f32 = 0.0;
const CHART_BOTTOM_PADDING: f32 = 8.0;
const MEASUREMENTS_PANEL_PADDING: f32 = 8.0;
const AXIS_LEFT_GUTTER: f32 = 52.0;
const AXIS_RIGHT_GUTTER: f32 = 2.0;
const AXIS_TOP_GUTTER: f32 = 2.0;
const AXIS_BOTTOM_GUTTER: f32 = 30.0;
const AXIS_TITLE_MIN_LEFT_INSET: f32 = 2.0;
const AXIS_TITLE_TO_VALUE_LABEL_GAP: f32 = 6.0;
const AXIS_TITLE_BOTTOM_INSET: f32 = 2.0;
const AXIS_TICK_X_OFFSET: f32 = 2.0;
const AXIS_TICK_Y_OFFSET: f32 = 2.0;
const MAX_MAJOR_TICKS: usize = 50;
const MAX_MINOR_TICKS: usize = 250;

#[allow(dead_code)]
fn calculate_layout(available: Rect) -> EyeLayout {
    let width = clamp_measurements_pane_width(
        available,
        (available.width() * MEASUREMENTS_WIDTH_FRACTION)
            .clamp(MEASUREMENTS_WIDTH_MIN, MEASUREMENTS_WIDTH_MAX),
    );
    calculate_layout_with_measurements_width(available, true, width)
}

fn calculate_layout_with_measurements_width(
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

fn measurements_pane_width_bounds(total: Rect) -> (f32, f32) {
    let min = MEASUREMENTS_WIDTH_MIN;
    let max_by_fraction = (total.width() * MEASUREMENTS_WIDTH_MAX_FRACTION).max(min);
    let max_by_chart = (total.width() - CHART_LEFT_PADDING - EYE_CHART_MIN_WIDTH).max(min);
    let max = max_by_fraction
        .min(max_by_chart)
        .min(MEASUREMENTS_WIDTH_MAX)
        .max(min);
    (min, max)
}

fn eye_plot_rect(chart_rect: Rect) -> Rect {
    let min_x = (chart_rect.min.x + AXIS_LEFT_GUTTER).min(chart_rect.max.x - 1.0);
    let max_x = (chart_rect.max.x - AXIS_RIGHT_GUTTER).max(min_x + 1.0);
    let min_y = (chart_rect.min.y + AXIS_TOP_GUTTER).min(chart_rect.max.y - 1.0);
    let max_y = (chart_rect.max.y - AXIS_BOTTOM_GUTTER).max(min_y + 1.0);
    Rect::from_min_max(Pos2::new(min_x, min_y), Pos2::new(max_x, max_y))
}

fn x_tick_label_position(x: f32, plot_rect: Rect) -> Pos2 {
    Pos2::new(x, plot_rect.max.y + AXIS_TICK_Y_OFFSET)
}

fn y_tick_label_position(y: f32, plot_rect: Rect) -> Pos2 {
    Pos2::new(plot_rect.min.x - AXIS_TICK_X_OFFSET, y)
}

fn x_axis_title_position(chart_rect: Rect, plot_rect: Rect) -> Pos2 {
    Pos2::new(
        plot_rect.center().x,
        chart_rect.max.y - AXIS_TITLE_BOTTOM_INSET,
    )
}

fn y_axis_title_position(
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

fn clamp_measurements_pane_width(total: Rect, width: f32) -> f32 {
    let (min, max) = measurements_pane_width_bounds(total);
    width.clamp(min, max)
}

fn resolve_measurements_pane_width(total: Rect, manual_width: Option<f32>, auto_width: f32) -> f32 {
    let base = (total.width() * MEASUREMENTS_WIDTH_FRACTION)
        .clamp(MEASUREMENTS_WIDTH_MIN, MEASUREMENTS_WIDTH_MAX);
    let desired = manual_width.unwrap_or_else(|| base.max(auto_width));
    clamp_measurements_pane_width(total, desired)
}

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(ui: &mut Ui, layout: &EyeLayout, state: &mut EyeDiagramState) -> bool {
    let painter = ui.painter();
    let mut close_requested = false;

    painter.rect_filled(layout.header, Rounding::ZERO, viewer_header_bg_color());

    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().interact_size.y = 24.0;
            ui.spacing_mut().button_padding.y = 2.0;

            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Eye Diagram")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(16.0);

            let mut mode = state.mode;
            egui::ComboBox::from_id_salt("eye_mode")
                .width(112.0)
                .selected_text(mode.display_name())
                .show_ui(ui, |ui| {
                    for candidate in EyeDisplayMode::all() {
                        ui.selectable_value(&mut mode, *candidate, candidate.display_name());
                    }
                });
            if mode != state.mode {
                state.set_mode(mode);
            }

            let mut color_map = state.color_map;
            ui.add_enabled_ui(state.mode == EyeDisplayMode::Persistence, |ui| {
                egui::ComboBox::from_id_salt("eye_colormap")
                    .width(96.0)
                    .selected_text(color_map.display_name())
                    .show_ui(ui, |ui| {
                        for map in ColorMap::all() {
                            ui.selectable_value(&mut color_map, *map, map.display_name());
                        }
                    });
            });
            state.color_map = color_map;

            if state.mode == EyeDisplayMode::SingleTrace {
                let mut selected = state.selected_trace;
                let selected_text = selected
                    .map(|idx| format!("Trace {}", idx + 1))
                    .unwrap_or_else(|| "Auto".to_string());
                egui::ComboBox::from_id_salt("eye_trace_select")
                    .width(112.0)
                    .selected_text(selected_text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected, None, "Auto");
                        for idx in 0..state.data.traces.len() {
                            ui.selectable_value(
                                &mut selected,
                                Some(idx),
                                format!("Trace {}", idx + 1),
                            );
                        }
                    });
                if selected != state.selected_trace {
                    state.select_trace(selected);
                }
            }

            ui.separator();

            if ui.small_button("Fit").clicked() {
                state.reset_view_to_data();
                state.invalidate_persistence_cache();
            }

            let mut h_scale = state.h_scale;
            let h_resp = ui.add(
                egui::DragValue::new(&mut h_scale)
                    .speed((state.h_scale.abs() * 0.05).max(1e-18))
                    .range(1e-18..=1e9)
                    .suffix(" s/div"),
            );
            if h_resp.changed() {
                state.h_scale = h_scale.max(1e-18);
                state.apply_scale_controls();
                state.invalidate_persistence_cache();
            }

            let mut v_scale = state.v_scale;
            let v_resp = ui.add(
                egui::DragValue::new(&mut v_scale)
                    .speed((state.v_scale.abs() * 0.05).max(1e-9))
                    .range(1e-9..=1e9)
                    .suffix(" V/div"),
            );
            if v_resp.changed() {
                state.v_scale = v_scale.max(1e-9);
                state.apply_scale_controls();
                state.invalidate_persistence_cache();
            }

            let mut decay = state.persistence_decay;
            let decay_resp = ui.add(
                egui::Slider::new(&mut decay, 0.50..=0.999)
                    .text("Decay")
                    .clamping(egui::SliderClamping::Always),
            );
            if decay_resp.changed() {
                state.persistence_decay = decay;
                state.invalidate_persistence_cache();
            }

            ui.separator();

            if ui
                .small_button(if state.show_measurements {
                    "Meas [on]"
                } else {
                    "Meas"
                })
                .clicked()
            {
                state.toggle_measurements();
            }

            if ui
                .small_button(if state.show_mask { "Mask [on]" } else { "Mask" })
                .clicked()
            {
                state.toggle_mask();
            }

            if ui
                .small_button(if state.show_grid { "Grid [on]" } else { "Grid" })
                .clicked()
            {
                state.show_grid = !state.show_grid;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                if ui.small_button("Close").clicked() {
                    close_requested = true;
                }
            });
        });
    });
    close_requested
}

// =============================================================================
// Chart Rendering
// =============================================================================

fn render_chart_area(ui: &mut Ui, layout: &EyeLayout, state: &mut EyeDiagramState) {
    let response = ui.allocate_rect(layout.chart, Sense::click_and_drag());
    render_chart_core(ui, layout, state);
    handle_eye_chart_interactions(ui, response, eye_plot_rect(layout.chart), state);
}

fn handle_measurements_splitter(ui: &mut Ui, layout: &EyeLayout, state: &mut EyeDiagramState) {
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

fn next_measurements_pane_width(
    current_width: Option<f32>,
    fallback_layout_width: f32,
    drag_delta_x: f32,
    total: Rect,
) -> f32 {
    let base = current_width.unwrap_or(fallback_layout_width);
    clamp_measurements_pane_width(total, base - drag_delta_x)
}

fn handle_eye_chart_interactions(
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

    if response.clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            if plot_rect.contains(pos) {
                let modifiers = ui.input(|i| i.modifiers);
                let time_s = x_to_eye_time(pos.x, plot_rect, state);
                if modifiers.alt {
                    state.add_marker(time_s);
                } else {
                    state.cursors.place(time_s);
                }
            }
        }
    }

    if response.secondary_clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            if plot_rect.contains(pos) {
                let modifiers = ui.input(|i| i.modifiers);
                if modifiers.alt {
                    let time_s = x_to_eye_time(pos.x, plot_rect, state);
                    let tolerance = eye_marker_removal_tolerance_s(state, plot_rect, pos.x);
                    state.remove_nearest_marker(time_s, tolerance);
                }
            }
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

fn eye_marker_removal_tolerance_s(state: &EyeDiagramState, plot_rect: Rect, pointer_x: f32) -> f64 {
    let x_radius = (plot_rect.width() * 0.01).max(4.0);
    let x0 = (pointer_x - x_radius).clamp(plot_rect.min.x, plot_rect.max.x);
    let x1 = (pointer_x + x_radius).clamp(plot_rect.min.x, plot_rect.max.x);
    (x_to_eye_time(x1, plot_rect, state) - x_to_eye_time(x0, plot_rect, state))
        .abs()
        .max(1e-18)
}

fn render_chart_core(ui: &mut Ui, layout: &EyeLayout, state: &mut EyeDiagramState) {
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

#[derive(Debug, Clone)]
struct AxisTick {
    value: f64,
    label: String,
    major: bool,
}

#[derive(Debug, Clone, Default)]
struct AxisTicks {
    ticks: Vec<AxisTick>,
    unit: String,
}

#[derive(Debug, Clone, Copy, Default)]
struct GridLabelMetrics {
    max_y_tick_label_width: f32,
}

fn render_grid(
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

fn render_axes(
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

fn render_center_lines(painter: &Painter, rect: Rect, _state: &EyeDiagramState) {
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

fn eye_time_ticks(state: &EyeDiagramState, target_major_ticks: usize) -> AxisTicks {
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

fn eye_voltage_ticks(state: &EyeDiagramState, target_major_ticks: usize) -> AxisTicks {
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

fn eye_full_time_range_seconds(state: &EyeDiagramState) -> f64 {
    state.full_time_span_seconds()
}

fn eye_view_time_bounds(state: &EyeDiagramState) -> (f64, f64) {
    (state.view.time_min_s, state.view.time_max_s)
}

fn eye_view_voltage_bounds(state: &EyeDiagramState) -> (f64, f64) {
    (state.view.voltage_min, state.view.voltage_max)
}

fn trace_time_to_seconds(state: &EyeDiagramState, time_ui: f64) -> f64 {
    time_ui * state.data.bit_period.max(1e-18)
}

fn eye_time_to_x(time_seconds: f64, plot_rect: Rect, state: &EyeDiagramState) -> f32 {
    let (min, max) = eye_view_time_bounds(state);
    let range = max - min;
    if !range.is_finite() || range <= 0.0 {
        return plot_rect.center().x;
    }
    let t = ((time_seconds - min) / range).clamp(0.0, 1.0);
    plot_rect.min.x + (t as f32) * plot_rect.width()
}

fn eye_voltage_to_y(voltage: f64, plot_rect: Rect, state: &EyeDiagramState) -> f32 {
    let (v_min, v_max) = eye_view_voltage_bounds(state);
    let range = v_max - v_min;
    if !range.is_finite() || range <= 0.0 {
        return plot_rect.center().y;
    }
    let t = ((voltage - v_min) / range).clamp(0.0, 1.0);
    plot_rect.max.y - (t as f32) * plot_rect.height()
}

fn x_to_eye_time(x: f32, plot_rect: Rect, state: &EyeDiagramState) -> f64 {
    if plot_rect.width() <= 0.0 {
        return eye_view_time_bounds(state).0;
    }
    let t = ((x - plot_rect.min.x) / plot_rect.width()).clamp(0.0, 1.0) as f64;
    let (min, max) = eye_view_time_bounds(state);
    min + t * (max - min)
}

fn y_to_eye_voltage(y: f32, plot_rect: Rect, state: &EyeDiagramState) -> f64 {
    if plot_rect.height() <= 0.0 {
        return eye_view_voltage_bounds(state).0;
    }
    let t = ((y - plot_rect.min.y) / plot_rect.height()).clamp(0.0, 1.0) as f64;
    let (min, max) = eye_view_voltage_bounds(state);
    max - t * (max - min)
}

fn render_mask(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
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

fn render_traces_overlay(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
    let data = &state.data;
    let stroke = Stroke::new(1.0, trace_color());

    for trace in &data.traces {
        render_single_eye_trace(painter, rect, state, trace, stroke);
    }
}

fn render_traces_persistence(painter: &Painter, rect: Rect, state: &mut EyeDiagramState) {
    let color_map = state.color_map;
    let intensity_exponent = persistence_intensity_exponent(state.persistence_decay);
    let Some(grid) = ensure_persistence_grid(rect, state) else {
        return;
    };
    let cell_width = rect.width() / grid.width as f32;
    let cell_height = rect.height() / grid.height as f32;
    if !cell_width.is_finite()
        || !cell_height.is_finite()
        || cell_width <= 0.0
        || cell_height <= 0.0
    {
        return;
    }

    for (x, y, count) in &grid.nonzero_bins {
        let base = *count as f32 / grid.max_count.max(1) as f32;
        let intensity = base.powf(intensity_exponent).clamp(0.0, 1.0);
        let (r, g, b) = color_map.map(intensity);
        let alpha = (32.0 + intensity * 223.0).clamp(0.0, 255.0) as u8;
        let color = Color32::from_rgba_unmultiplied(r, g, b, alpha);
        let min = Pos2::new(
            rect.min.x + *x as f32 * cell_width,
            rect.min.y + *y as f32 * cell_height,
        );
        painter.rect_filled(
            Rect::from_min_size(min, Vec2::new(cell_width + 0.5, cell_height + 0.5)),
            Rounding::ZERO,
            color,
        );
    }
}

#[derive(Debug, Clone)]
struct PersistenceGrid {
    width: usize,
    height: usize,
    nonzero_bins: Vec<(usize, usize, u32)>,
    max_count: u32,
}

fn persistence_intensity_exponent(decay: f32) -> f32 {
    // Higher decay => longer persistence tail => brighter low-density bins.
    let clamped = decay.clamp(0.50, 0.999);
    0.35 + (1.0 - clamped) * 3.3
}

fn ensure_persistence_grid<'a>(
    plot_rect: Rect,
    state: &'a mut EyeDiagramState,
) -> Option<&'a EyePersistenceCache> {
    let key = persistence_cache_key(plot_rect, state)?;
    let cache_hit = state
        .persistence_cache
        .as_ref()
        .map(|cache| cache.key == key)
        .unwrap_or(false);
    if !cache_hit {
        let rebuilt = build_persistence_cache(state, key)?;
        state.persistence_cache = Some(rebuilt);
    }
    state.persistence_cache.as_ref()
}

fn persistence_cache_key(
    plot_rect: Rect,
    state: &EyeDiagramState,
) -> Option<EyePersistenceCacheKey> {
    let width = (plot_rect.width().round() as usize).clamp(96, 480);
    let height = (plot_rect.height().round() as usize).clamp(72, 320);
    if width == 0 || height == 0 {
        return None;
    }

    Some(EyePersistenceCacheKey {
        width,
        height,
        trace_count: state.data.traces.len(),
        total_points: state.data.total_points(),
        ui_count: state.ui_count.max(1),
        time_min_s: state.view.time_min_s,
        time_max_s: state.view.time_max_s,
        voltage_min: state.view.voltage_min,
        voltage_max: state.view.voltage_max,
        decay_quantized: (state.persistence_decay.clamp(0.0, 1.0) * 1000.0).round() as u16,
    })
}

fn build_persistence_cache(
    state: &EyeDiagramState,
    key: EyePersistenceCacheKey,
) -> Option<EyePersistenceCache> {
    let grid = build_persistence_grid(state, key.width, key.height)?;
    Some(EyePersistenceCache {
        key,
        width: grid.width,
        height: grid.height,
        nonzero_bins: grid.nonzero_bins,
        max_count: grid.max_count,
    })
}

fn build_persistence_grid(
    state: &EyeDiagramState,
    width: usize,
    height: usize,
) -> Option<PersistenceGrid> {
    let data = &state.data;
    if data.traces.is_empty() {
        return None;
    }

    let mut counts = vec![0u32; width * height];
    let mut max_count = 0u32;
    let (time_min, time_max) = eye_view_time_bounds(state);
    let time_range = (time_max - time_min).max(1e-18);
    if !time_range.is_finite() || time_range <= 0.0 {
        return None;
    }
    let (v_min, v_max) = eye_view_voltage_bounds(state);
    let v_range = (v_max - v_min).max(1e-18);

    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 0..n {
            let t = trace.time[i];
            let v = trace.amplitude[i];
            if !t.is_finite() || !v.is_finite() {
                continue;
            }

            let time_seconds = trace_time_to_seconds(state, t);
            let x_norm = ((time_seconds - time_min) / time_range).clamp(0.0, 1.0);
            let y_norm = ((v_max - v) / v_range).clamp(0.0, 1.0);

            let xi = ((x_norm * (width as f64 - 1.0)).round() as usize).min(width - 1);
            let yi = ((y_norm * (height as f64 - 1.0)).round() as usize).min(height - 1);
            let idx = yi * width + xi;
            counts[idx] = counts[idx].saturating_add(1);
            max_count = max_count.max(counts[idx]);
        }
    }

    if max_count == 0 {
        return None;
    }

    let mut nonzero_bins = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let count = counts[y * width + x];
            if count > 0 {
                nonzero_bins.push((x, y, count));
            }
        }
    }

    Some(PersistenceGrid {
        width,
        height,
        nonzero_bins,
        max_count,
    })
}

fn render_single_trace(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
    let data = &state.data;

    if let Some(idx) = state.selected_trace {
        if let Some(trace) = data.traces.get(idx) {
            let stroke = Stroke::new(2.0, highlight_color());
            render_single_eye_trace(painter, rect, state, trace, stroke);
        }
    } else if let Some(trace) = data.traces.first() {
        let stroke = Stroke::new(2.0, trace_color());
        render_single_eye_trace(painter, rect, state, trace, stroke);
    }
}

fn render_single_eye_trace(
    painter: &Painter,
    rect: Rect,
    state: &EyeDiagramState,
    trace: &EyeTrace,
    stroke: Stroke,
) {
    let n = trace.time.len().min(trace.amplitude.len());

    if n < 2 {
        return;
    }

    let mut prev: Option<Pos2> = None;
    for i in 0..n {
        let t = trace.time[i];
        let v = trace.amplitude[i];
        if !t.is_finite() || !v.is_finite() {
            prev = None;
            continue;
        }

        let time_seconds = trace_time_to_seconds(state, t);
        let point = Pos2::new(
            eye_time_to_x(time_seconds, rect, state),
            eye_voltage_to_y(v, rect, state),
        );

        if let Some(start) = prev {
            if let Some([clipped_start, clipped_end]) =
                clip_line_segment_to_rect(start, point, rect)
            {
                painter.line_segment([clipped_start, clipped_end], stroke);
            }
        }
        prev = Some(point);
    }
}

fn clip_line_segment_to_rect(start: Pos2, end: Pos2, rect: Rect) -> Option<[Pos2; 2]> {
    let mut t0 = 0.0f32;
    let mut t1 = 1.0f32;
    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let checks = [
        (-dx, start.x - rect.min.x),
        (dx, rect.max.x - start.x),
        (-dy, start.y - rect.min.y),
        (dy, rect.max.y - start.y),
    ];

    for (p, q) in checks {
        if p.abs() <= f32::EPSILON {
            if q < 0.0 {
                return None;
            }
            continue;
        }

        let r = q / p;
        if p < 0.0 {
            if r > t1 {
                return None;
            }
            if r > t0 {
                t0 = r;
            }
        } else if p > 0.0 {
            if r < t0 {
                return None;
            }
            if r < t1 {
                t1 = r;
            }
        }
    }

    if t0 > t1 {
        return None;
    }

    Some([
        Pos2::new(start.x + t0 * dx, start.y + t0 * dy),
        Pos2::new(start.x + t1 * dx, start.y + t1 * dy),
    ])
}

fn render_eye_cursors_and_markers(painter: &Painter, plot_rect: Rect, state: &EyeDiagramState) {
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

fn draw_eye_axis_label(
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
// Measurements Panel
// =============================================================================

fn render_measurements_panel(ui: &mut Ui, layout: &EyeLayout, state: &mut EyeDiagramState) {
    let Some(measurements_rect) = layout.measurements else {
        return;
    };
    let painter = ui.painter();

    painter.rect_filled(measurements_rect, Rounding::ZERO, panel_bg_color());
    if let Some(outline_rect) = measurements_outline_rect(measurements_rect) {
        painter.rect_stroke(
            outline_rect,
            Rounding::ZERO,
            Stroke::new(1.0, panel_border_color()),
        );
    }

    let panel_rect = measurements_rect.shrink(MEASUREMENTS_PANEL_PADDING);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        egui::ScrollArea::vertical()
            .id_salt("eye_measurements_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Measurements")
                            .size(11.0)
                            .color(text_color()),
                    );
                    ui.add_space(8.0);

                    if state.data.trace_count() == 0 {
                        ui.label(
                            egui::RichText::new("No data")
                                .size(10.0)
                                .color(Color32::from_rgb(100, 105, 115)),
                        );
                        return;
                    }

                    let m = &state.measurements;

                    measurement_row(ui, "Data Rate", &m.format_data_rate());
                    measurement_row(ui, "UI", &format!("{:.2} ps", m.unit_interval * 1e12));

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    measurement_row(ui, "Eye Height", &m.format_height());
                    measurement_row(ui, "Eye Width", &m.format_width());

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    measurement_row(ui, "Jitter (p-p)", &m.format_jitter());
                    measurement_row(ui, "Rise Time", &m.format_rise_time());
                    measurement_row(ui, "Fall Time", &m.format_fall_time());

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    measurement_row(ui, "Q-Factor", &format!("{:.2}", m.q_factor));
                    measurement_row(ui, "Est. BER", &m.format_ber());
                    measurement_row(ui, "SNR", &format!("{:.1} dB", m.snr_db));

                    if state.show_mask {
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(4.0);

                        let result = state.mask_result_string();
                        let color = if state.mask.is_passing() {
                            Color32::from_rgb(100, 200, 100)
                        } else {
                            Color32::from_rgb(255, 100, 100)
                        };

                        ui.label(
                            egui::RichText::new(format!("Mask: {}", result))
                                .size(11.0)
                                .color(color),
                        );
                    }

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);
                    render_cursor_marker_manager(ui, state);
                });
            });
    });
}

fn measurements_outline_rect(measurements_rect: Rect) -> Option<Rect> {
    let top = (measurements_rect.min.y + AXIS_TOP_GUTTER).min(measurements_rect.max.y);
    if top >= measurements_rect.max.y {
        return None;
    }
    Some(Rect::from_min_max(
        Pos2::new(measurements_rect.min.x, top),
        measurements_rect.max,
    ))
}

fn measurement_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}:", label))
                .size(10.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .size(11.0)
                    .color(Color32::from_rgb(200, 205, 215)),
            );
        });
    });
}

fn render_cursor_marker_manager(ui: &mut Ui, state: &mut EyeDiagramState) {
    ui.label(
        egui::RichText::new("Cursors / Markers")
            .size(11.0)
            .color(text_color()),
    );
    ui.add_space(4.0);

    let c1 = state
        .cursors
        .cursor1_time_s
        .map(crate::waveform::axis::format_time)
        .unwrap_or_else(|| "—".to_string());
    let c2 = state
        .cursors
        .cursor2_time_s
        .map(crate::waveform::axis::format_time)
        .unwrap_or_else(|| "—".to_string());
    measurement_row(ui, "C1", &c1);
    measurement_row(ui, "C2", &c2);
    if let Some(dt) = state.cursors.delta_time() {
        measurement_row(ui, "ΔT", &crate::waveform::axis::format_time(dt));
    } else {
        measurement_row(ui, "ΔT", "—");
    }

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        if ui.small_button("Clear Cursors").clicked() {
            state.cursors.clear();
        }
        if ui.small_button("Clear Markers").clicked() {
            state.clear_markers();
        }
    });

    if state.markers.is_empty() {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("No markers")
                .size(10.0)
                .color(Color32::from_rgb(100, 105, 115)),
        );
        return;
    }

    ui.add_space(4.0);
    let mut remove_idx: Option<usize> = None;
    for (idx, marker) in state.markers.iter().copied().enumerate() {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("M{}:", idx + 1))
                    .size(10.0)
                    .color(Color32::from_rgb(120, 125, 135)),
            );
            ui.label(
                egui::RichText::new(crate::waveform::axis::format_time(marker))
                    .size(11.0)
                    .color(Color32::from_rgb(200, 205, 215)),
            );
            if ui.small_button("x").clicked() {
                remove_idx = Some(idx);
            }
        });
    }

    if let Some(idx) = remove_idx {
        state.remove_marker_at(idx);
    }
}

// =============================================================================
// Demo Data
// =============================================================================

#[cfg(test)]
fn load_demo_data(state: &mut EyeDiagramState) {
    // Generate demo eye diagram for 10 Gbps signal
    let bit_period = 100e-12; // 100ps = 10 Gbps
    let pattern = generate_prbs7_pattern();
    let samples_per_bit = 32;

    let mut time = Vec::new();
    let mut signal = Vec::new();

    // Generate waveform with transitions and some noise
    for (bit_idx, &bit) in pattern.iter().take(64).enumerate() {
        for s in 0..samples_per_bit {
            let t = (bit_idx * samples_per_bit + s) as f64 * bit_period / samples_per_bit as f64;
            let phase = s as f64 / samples_per_bit as f64;

            // Get current and next bit for transition
            let next_bit = pattern.get(bit_idx + 1).copied().unwrap_or(bit);

            // Smooth transition
            let v = if bit == next_bit {
                if bit == 1 {
                    0.35
                } else {
                    -0.35
                }
            } else {
                let transition = if phase < 0.3 {
                    0.0
                } else if phase > 0.7 {
                    1.0
                } else {
                    let x = (phase - 0.3) / 0.4;
                    x * x * (3.0 - 2.0 * x) // Smooth step
                };

                if bit == 0 {
                    -0.35 + 0.7 * transition
                } else {
                    0.35 - 0.7 * transition
                }
            };

            // Add small noise
            let noise = ((t * 1e12 * 7.0).sin() * 0.01) + ((t * 1e12 * 13.0).cos() * 0.005);

            time.push(t);
            signal.push(v + noise);
        }
    }

    let builder = EyeDataBuilder::new()
        .bit_period(bit_period)
        .ui_count(2)
        .skip_initial(4);

    let data = builder.build(&time, &signal);
    state.load_data(data);
}

#[cfg(test)]
fn generate_prbs7_pattern() -> Vec<u8> {
    // Generate PRBS-7 pattern (2^7 - 1 = 127 bits)
    let mut pattern = Vec::with_capacity(127);
    let mut lfsr: u8 = 0x7F; // Initial state

    for _ in 0..127 {
        let bit = (lfsr >> 6) & 1;
        pattern.push(bit);

        // PRBS-7 polynomial: x^7 + x^6 + 1
        let new_bit = ((lfsr >> 6) ^ (lfsr >> 5)) & 1;
        lfsr = ((lfsr << 1) | new_bit) & 0x7F;
    }

    pattern
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_calculation() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let layout = calculate_layout(rect);

        assert!(layout.chart.width() > 0.0);
        assert!(layout.chart.height() > 0.0);
        assert!(layout.measurements.is_some());
        let measurements = layout.measurements.expect("measurements pane");
        assert!(layout.chart.max.x <= measurements.min.x + f32::EPSILON);
    }

    #[test]
    fn test_layout_chart_and_measurements_share_top_edge() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(960.0, 560.0));
        let layout = calculate_layout(rect);
        let measurements = layout.measurements.expect("measurements pane");
        assert!((layout.chart.min.y - measurements.min.y).abs() < f32::EPSILON);
    }

    #[test]
    fn test_layout_chart_touches_measurements_without_gap() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(960.0, 560.0));
        let layout = calculate_layout(rect);
        let measurements = layout.measurements.expect("measurements pane");
        assert!((layout.chart.max.x - measurements.min.x).abs() < f32::EPSILON);
    }

    #[test]
    fn test_layout_without_measurements_reclaims_right_width() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let layout = calculate_layout_with_measurements_width(rect, false, 200.0);
        assert!(layout.measurements.is_none());
        assert!((layout.chart.max.x - (rect.max.x - CHART_RIGHT_PADDING)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_measurements_pane_width_bounds_preserve_chart_minimum() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(900.0, 600.0));
        let (_min, max) = measurements_pane_width_bounds(rect);
        let remaining = rect.width() - CHART_LEFT_PADDING - max;
        assert!(remaining >= EYE_CHART_MIN_WIDTH - f32::EPSILON);
    }

    #[test]
    fn test_resolve_measurements_pane_width_clamps_manual_width() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(740.0, 540.0));
        let resolved = resolve_measurements_pane_width(rect, Some(9999.0), 0.0);
        let (_min, max) = measurements_pane_width_bounds(rect);
        assert!((resolved - max).abs() < f32::EPSILON);
    }

    #[test]
    fn test_next_measurements_pane_width_follows_drag_direction() {
        let total = Rect::from_min_size(Pos2::ZERO, Vec2::new(900.0, 600.0));
        let base = 220.0;
        let shrink = next_measurements_pane_width(Some(base), base, 10.0, total);
        let grow = next_measurements_pane_width(Some(base), base, -10.0, total);
        assert!(shrink < base);
        assert!(grow > base);
    }

    #[test]
    fn test_clip_line_segment_inside_rect_is_unchanged() {
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
        let clipped = clip_line_segment_to_rect(Pos2::new(2.0, 3.0), Pos2::new(8.0, 9.0), rect)
            .expect("segment should remain visible");
        assert!((clipped[0].x - 2.0).abs() < 1e-6);
        assert!((clipped[0].y - 3.0).abs() < 1e-6);
        assert!((clipped[1].x - 8.0).abs() < 1e-6);
        assert!((clipped[1].y - 9.0).abs() < 1e-6);
    }

    #[test]
    fn test_clip_line_segment_crossing_bottom_is_trimmed() {
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
        let clipped = clip_line_segment_to_rect(Pos2::new(2.0, 8.0), Pos2::new(8.0, 14.0), rect)
            .expect("segment should intersect bottom boundary");
        assert!((clipped[0].x - 2.0).abs() < 1e-6);
        assert!((clipped[0].y - 8.0).abs() < 1e-6);
        assert!((clipped[1].x - 4.0).abs() < 1e-6);
        assert!((clipped[1].y - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_clip_line_segment_fully_outside_rect_is_rejected() {
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
        let clipped = clip_line_segment_to_rect(Pos2::new(-5.0, -2.0), Pos2::new(-1.0, -8.0), rect);
        assert!(clipped.is_none());
    }

    #[test]
    fn test_eye_ticks_generate_major_labels() {
        let mut state = EyeDiagramState::new();
        load_demo_data(&mut state);
        let x_ticks = eye_time_ticks(&state, 10);
        let y_ticks = eye_voltage_ticks(&state, 8);
        assert!(x_ticks.ticks.iter().any(|t| t.major && !t.label.is_empty()));
        assert!(y_ticks.ticks.iter().any(|t| t.major && !t.label.is_empty()));
    }

    #[test]
    fn test_build_persistence_grid_with_demo_data() {
        let mut state = EyeDiagramState::new();
        state.set_mode(EyeDisplayMode::Persistence);
        load_demo_data(&mut state);
        let grid = build_persistence_grid(&state, 400, 260).expect("persistence grid");
        assert!(grid.width >= 96);
        assert!(grid.height >= 72);
        assert!(grid.max_count > 0);
        assert!(!grid.nonzero_bins.is_empty());
    }

    #[test]
    fn test_persistence_cache_reuses_existing_grid_until_key_changes() {
        let mut state = EyeDiagramState::new();
        state.set_mode(EyeDisplayMode::Persistence);
        load_demo_data(&mut state);
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(420.0, 280.0));

        let first_ptr = {
            let cache = ensure_persistence_grid(rect, &mut state).expect("cache");
            cache.nonzero_bins.as_ptr()
        };
        let first_key = state.persistence_cache.as_ref().expect("cache").key;

        let second_ptr = {
            let cache = ensure_persistence_grid(rect, &mut state).expect("cache");
            cache.nonzero_bins.as_ptr()
        };
        assert_eq!(first_ptr, second_ptr);

        let center_t = (state.view.time_min_s + state.view.time_max_s) * 0.5;
        let center_v = (state.view.voltage_min + state.view.voltage_max) * 0.5;
        state.zoom_view(0.8, center_t, center_v);
        let _ = ensure_persistence_grid(rect, &mut state).expect("cache rebuilt");
        let second_key = state.persistence_cache.as_ref().expect("cache").key;
        assert_ne!(first_key.time_max_s, second_key.time_max_s);
    }

    #[test]
    fn test_prbs7_pattern() {
        let pattern = generate_prbs7_pattern();
        assert_eq!(pattern.len(), 127);

        // All bits should be 0 or 1
        assert!(pattern.iter().all(|&b| b == 0 || b == 1));

        // Should have both 0s and 1s
        assert!(pattern.iter().any(|&b| b == 0));
        assert!(pattern.iter().any(|&b| b == 1));
    }

    #[test]
    fn test_load_demo_data() {
        let mut state = EyeDiagramState::new();
        load_demo_data(&mut state);

        assert!(state.data.trace_count() > 0);
        assert!(state.measurements.data_rate > 0.0);
    }
}
