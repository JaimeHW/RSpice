//! Eye Diagram Rendering
//!
//! Commercial-grade egui rendering for eye diagram visualization.
//! Supports overlay, persistence, and single-trace display modes.

use egui::{
    Color32, FontId, Painter, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2,
};
use std::f64::consts::PI;

use super::data::{EyeData, EyeDataBuilder, EyeTrace};
use super::measurements::{calculate_eye_measurements, EyeMeasurements};
use super::state::{ColorMap, EyeDiagramState, EyeDisplayMode, EyeMask};
use crate::common::app::AppState;
use crate::common::viewer_style::viewer_header_bg_color;

// =============================================================================
// Constants
// =============================================================================

fn chart_bg_color() -> Color32 {
    Color32::from_rgb(15, 17, 21)
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

fn highlight_color() -> Color32 {
    Color32::from_rgb(100, 200, 255)
}

// =============================================================================
// Main Rendering Entry Point
// =============================================================================

/// Render the eye diagram viewer panel
pub fn render_eye_diagram_viewer(ui: &mut Ui, app_state: &mut AppState) {
    // Calculate layout
    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect);

    let close_requested = {
        let state = &mut app_state.eye_diagram_state;
        let close_requested = render_header(ui, &layout, state);
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
    let layout = calculate_layout(available_rect);

    render_chart_core(ui, &layout, state);
}

// =============================================================================
// Layout
// =============================================================================

#[derive(Debug, Clone)]
struct EyeLayout {
    total: Rect,
    header: Rect,
    chart: Rect,
    measurements: Rect,
}

const HEADER_HEIGHT: f32 = 32.0;
const MEASUREMENTS_WIDTH: f32 = 200.0;
const CHART_PADDING: f32 = 16.0;

fn calculate_layout(available: Rect) -> EyeLayout {
    let total = available;

    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));

    let measurements = Rect::from_min_size(
        Pos2::new(total.max.x - MEASUREMENTS_WIDTH, header.max.y),
        Vec2::new(MEASUREMENTS_WIDTH, total.height() - HEADER_HEIGHT),
    );

    let chart = Rect::from_min_max(
        Pos2::new(total.min.x + CHART_PADDING, header.max.y + CHART_PADDING),
        Pos2::new(
            measurements.min.x - CHART_PADDING,
            total.max.y - CHART_PADDING,
        ),
    );

    EyeLayout {
        total,
        header,
        chart,
        measurements,
    }
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
        ui.horizontal(|ui| {
            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Eye Diagram")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(16.0);

            // Mode selector
            egui::ComboBox::from_label("")
                .selected_text(state.mode.display_name())
                .show_ui(ui, |ui| {
                    for mode in EyeDisplayMode::all() {
                        ui.selectable_value(&mut state.mode, *mode, mode.display_name());
                    }
                });

            ui.separator();

            // Toggle buttons
            if ui
                .small_button(if state.show_measurements {
                    "Meas ✓"
                } else {
                    "Meas"
                })
                .clicked()
            {
                state.toggle_measurements();
            }

            if ui
                .small_button(if state.show_mask { "Mask ✓" } else { "Mask" })
                .clicked()
            {
                state.toggle_mask();
            }

            if ui
                .small_button(if state.show_grid { "Grid ✓" } else { "Grid" })
                .clicked()
            {
                state.show_grid = !state.show_grid;
            }

            // Right-aligned close button
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                if ui.small_button("✕").clicked() {
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
    painter.rect_stroke(
        rect,
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
    );
}

fn render_grid(painter: &Painter, rect: Rect, state: &EyeDiagramState) {
    let stroke = Stroke::new(0.5, grid_color());

    // Vertical lines (UI divisions)
    let ui_count = state.ui_count as usize;
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

    // Convert mask polygon to screen coordinates
    let screen_points: Vec<Pos2> = mask
        .inner
        .points
        .iter()
        .map(|&(t, v)| {
            let x = rect.min.x + t as f32 * rect.width() / state.ui_count as f32;
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

    for (i, trace) in data.traces.iter().enumerate() {
        let intensity = (i as f32 / n as f32).powf(0.5);
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
    let n = trace.time.len().min(trace.amplitude.len());

    if n < 2 {
        return;
    }

    // Convert to screen coordinates
    let points: Vec<Pos2> = (0..n)
        .filter_map(|i| {
            let t = trace.time[i];
            let v = trace.amplitude[i];

            if !t.is_finite() || !v.is_finite() {
                return None;
            }

            // Normalize time to 0..ui_count
            let t_norm = t / state.ui_count as f64;

            // Normalize voltage
            let v_norm = if data.swing > 0.0 {
                (v - data.v_cross) / data.swing
            } else {
                0.0
            };

            let x = rect.min.x + t_norm as f32 * rect.width();
            let y = rect.center().y - v_norm as f32 * rect.height();

            // Clamp to rect
            let x = x.clamp(rect.min.x, rect.max.x);
            let y = y.clamp(rect.min.y, rect.max.y);

            Some(Pos2::new(x, y))
        })
        .collect();

    // Draw line segments
    for window in points.windows(2) {
        painter.line_segment([window[0], window[1]], stroke);
    }
}

// =============================================================================
// Measurements Panel
// =============================================================================

fn render_measurements_panel(ui: &mut Ui, layout: &EyeLayout, state: &EyeDiagramState) {
    let painter = ui.painter();

    painter.rect_filled(
        layout.measurements,
        Rounding::ZERO,
        Color32::from_rgb(25, 27, 33),
    );

    let panel_rect = layout.measurements.shrink(8.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Measurements")
                    .size(11.0)
                    .color(text_color()),
            );
            ui.add_space(8.0);

            let m = &state.measurements;

            // Data rate
            measurement_row(ui, "Data Rate", &m.format_data_rate());
            measurement_row(ui, "UI", &format!("{:.2} ps", m.unit_interval * 1e12));

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // Eye opening
            measurement_row(ui, "Eye Height", &m.format_height());
            measurement_row(ui, "Eye Width", &m.format_width());

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // Jitter
            measurement_row(ui, "Jitter (p-p)", &m.format_jitter());
            measurement_row(ui, "Rise Time", &m.format_rise_time());
            measurement_row(ui, "Fall Time", &m.format_fall_time());

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // Quality metrics
            measurement_row(ui, "Q-Factor", &format!("{:.2}", m.q_factor));
            measurement_row(ui, "Est. BER", &m.format_ber());
            measurement_row(ui, "SNR", &format!("{:.1} dB", m.snr_db));

            // Mask result
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
        assert!(layout.chart.max.x <= layout.measurements.min.x);
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
