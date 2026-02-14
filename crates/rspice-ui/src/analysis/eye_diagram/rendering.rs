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
use super::state::{ColorMap, EyeDiagramState, EyeDisplayMode};
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

    render_chart_core(ui, &layout, state);
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
const CHART_PADDING: f32 = 16.0;
const MEASUREMENTS_PANEL_PADDING: f32 = 8.0;

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

    let measurements = if show_measurements {
        let width = clamp_measurements_pane_width(total, measurements_width);
        Some(Rect::from_min_size(
            Pos2::new(total.max.x - width, header.max.y),
            Vec2::new(width, (total.height() - HEADER_HEIGHT).max(0.0)),
        ))
    } else {
        None
    };

    let chart_right = measurements
        .as_ref()
        .map(|rect| rect.min.x - CHART_PADDING)
        .unwrap_or(total.max.x - CHART_PADDING);

    let chart = Rect::from_min_max(
        Pos2::new(total.min.x + CHART_PADDING, header.max.y + CHART_PADDING),
        Pos2::new(chart_right, total.max.y - CHART_PADDING),
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
    let max_by_chart = (total.width() - CHART_PADDING * 2.0 - EYE_CHART_MIN_WIDTH).max(min);
    let max = max_by_fraction
        .min(max_by_chart)
        .min(MEASUREMENTS_WIDTH_MAX)
        .max(min);
    (min, max)
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

            egui::ComboBox::from_id_salt("eye_mode")
                .width(112.0)
                .selected_text(state.mode.display_name())
                .show_ui(ui, |ui| {
                    for mode in EyeDisplayMode::all() {
                        ui.selectable_value(&mut state.mode, *mode, mode.display_name());
                    }
                });

            let mut color_map = state.color_map;
            egui::ComboBox::from_id_salt("eye_colormap")
                .width(96.0)
                .selected_text(color_map.display_name())
                .show_ui(ui, |ui| {
                    for map in ColorMap::all() {
                        ui.selectable_value(&mut color_map, *map, map.display_name());
                    }
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

fn render_chart_area(ui: &mut Ui, layout: &EyeLayout, state: &EyeDiagramState) {
    render_chart_core(ui, layout, state);

    let _response = ui.allocate_rect(layout.chart, Sense::click());
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

fn render_chart_core(ui: &mut Ui, layout: &EyeLayout, state: &EyeDiagramState) {
    let painter = ui.painter();
    let rect = layout.chart;

    // Background
    painter.rect_filled(rect, Rounding::ZERO, chart_bg_color());

    // Grid
    if state.show_grid {
        render_grid(painter, rect, state);
    }

    // Mask (if enabled)
    if state.show_mask && state.mask.enabled {
        render_mask(painter, rect, state);
    }

    // Center lines
    render_center_lines(painter, rect, state);

    // Eye traces
    match state.mode {
        EyeDisplayMode::Overlay => render_traces_overlay(painter, rect, state),
        EyeDisplayMode::Persistence => render_traces_persistence(painter, rect, state),
        EyeDisplayMode::SingleTrace => render_single_trace(painter, rect, state),
    }

    // Border
    painter.rect_stroke(rect, Rounding::ZERO, Stroke::new(1.0, panel_border_color()));
}

fn render_grid(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
    let stroke = Stroke::new(0.5, grid_color());

    // Vertical lines (UI divisions)
    let ui_count = state.ui_count.max(1) as usize;
    let divisions_per_ui = 4;
    let total_divisions = ui_count * divisions_per_ui;

    for i in 1..total_divisions {
        let x = rect.min.x + (i as f32 / total_divisions as f32) * rect.width();
        painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], stroke);
    }

    // Horizontal lines (voltage divisions)
    let v_divisions = 8;
    for i in 1..v_divisions {
        let y = rect.min.y + (i as f32 / v_divisions as f32) * rect.height();
        painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], stroke);
    }
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

fn render_mask(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
    let mask = &state.mask;

    if mask.inner.points.is_empty() {
        return;
    }

    let ui_count = state.ui_count.max(1) as f32;
    // Convert mask polygon to screen coordinates
    let screen_points: Vec<Pos2> = mask
        .inner
        .points
        .iter()
        .map(|&(t, v)| {
            let x = rect.min.x + t as f32 * rect.width() / ui_count;
            let y = rect.center().y - v as f32 * rect.height() / 2.0;
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

fn render_traces_persistence(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
    // For persistence mode, we'd build a density histogram
    // For now, use overlay with fading
    let data = &state.data;
    let n = data.traces.len();
    if n == 0 {
        return;
    }

    for (i, trace) in data.traces.iter().enumerate() {
        let intensity = ((i + 1) as f32 / n as f32).powf(0.5);
        let (r, g, b) = state.color_map.map(intensity);
        let color = Color32::from_rgba_unmultiplied(r, g, b, 200);
        let stroke = Stroke::new(1.0, color);
        render_single_eye_trace(painter, rect, state, trace, stroke);
    }
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
    let data = &state.data;
    let ui_count = state.ui_count.max(1) as f64;
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

        let t_norm = t / ui_count;
        let v_norm = if data.swing > 0.0 {
            (v - data.v_cross) / data.swing
        } else {
            0.0
        };

        let point = Pos2::new(
            rect.min.x + t_norm as f32 * rect.width(),
            rect.center().y - v_norm as f32 * rect.height(),
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

// =============================================================================
// Measurements Panel
// =============================================================================

fn render_measurements_panel(ui: &mut Ui, layout: &EyeLayout, state: &EyeDiagramState) {
    let Some(measurements_rect) = layout.measurements else {
        return;
    };
    let painter = ui.painter();

    painter.rect_filled(measurements_rect, Rounding::ZERO, panel_bg_color());
    painter.rect_stroke(
        measurements_rect,
        Rounding::ZERO,
        Stroke::new(1.0, panel_border_color()),
    );

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
                });
            });
    });
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
        assert!(layout.chart.max.x <= measurements.min.x);
    }

    #[test]
    fn test_layout_without_measurements_reclaims_right_width() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let layout = calculate_layout_with_measurements_width(rect, false, 200.0);
        assert!(layout.measurements.is_none());
        assert!((layout.chart.max.x - (rect.max.x - CHART_PADDING)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_measurements_pane_width_bounds_preserve_chart_minimum() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(900.0, 600.0));
        let (_min, max) = measurements_pane_width_bounds(rect);
        let remaining = rect.width() - CHART_PADDING * 2.0 - max;
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
