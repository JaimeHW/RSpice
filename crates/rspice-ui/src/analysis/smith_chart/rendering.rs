//! Smith Chart Rendering
//!
//! Commercial-grade egui rendering for Smith chart visualization.
//! Renders constant-R circles, constant-X arcs, VSWR circles,
//! S-parameter traces, and markers.

use egui::{
    Color32, FontId, Painter, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2,
};
use std::f64::consts::PI;

use super::complex::Complex;
use super::impedance::{Impedance, Z0_DEFAULT};
use super::state::{
    constant_r_circle, constant_x_circle, standard_r_values, standard_x_values, SmithChartMode,
    SmithChartState,
};
use crate::common::app::AppState;

// =============================================================================
// Constants
// =============================================================================

/// Chart background color
fn chart_bg_color() -> Color32 {
    Color32::from_rgb(18, 20, 24)
}

/// Unit circle color
fn unit_circle_color() -> Color32 {
    Color32::from_rgb(100, 105, 115)
}

/// R-circle color
fn r_circle_color() -> Color32 {
    Color32::from_rgb(60, 65, 75)
}

/// X-circle color
fn x_circle_color() -> Color32 {
    Color32::from_rgb(50, 55, 65)
}

/// VSWR circle color
fn vswr_circle_color() -> Color32 {
    Color32::from_rgba_unmultiplied(255, 200, 50, 128)
}

/// Marker color
fn marker_color() -> Color32 {
    Color32::from_rgb(255, 100, 100)
}

/// Trace colors - static array
const TRACE_COLORS: [Color32; 8] = [
    Color32::from_rgb(59, 130, 246), // Blue
    Color32::from_rgb(239, 68, 68),  // Red
    Color32::from_rgb(34, 197, 94),  // Green
    Color32::from_rgb(168, 85, 247), // Purple
    Color32::from_rgb(249, 115, 22), // Orange
    Color32::from_rgb(236, 72, 153), // Pink
    Color32::from_rgb(20, 184, 166), // Teal
    Color32::from_rgb(234, 179, 8),  // Yellow
];

/// Get trace colors
fn trace_colors() -> &'static [Color32] {
    &TRACE_COLORS
}

// =============================================================================
// Main Rendering Entry Point
// =============================================================================

/// Render the Smith chart viewer panel
pub fn render_smith_chart_viewer(ui: &mut Ui, app_state: &mut AppState) {
    // Get or create Smith chart state
    // For now we create a demo state
    let mut state = SmithChartState::new();

    // Load demo data if available from simulation
    load_demo_data(&mut state);

    // Calculate layout
    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect);

    // Render sections
    render_header(ui, &layout, &mut state, app_state);
    render_chart_area(ui, &layout, &mut state);
    render_readout_panel(ui, &layout, &state);
}

/// Public render function for external use
pub fn render_smith_chart(ui: &mut Ui, state: &mut SmithChartState) {
    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect);

    render_chart_core(ui, &layout, state);
}

// =============================================================================
// Layout
// =============================================================================

/// Layout regions for Smith chart viewer
#[derive(Debug, Clone)]
struct SmithChartLayout {
    /// Full available rectangle
    total: Rect,
    /// Header bar
    header: Rect,
    /// Main chart area (square)
    chart: Rect,
    /// Readout panel (right side)
    readout: Rect,
}

/// Header height
const HEADER_HEIGHT: f32 = 32.0;
/// Readout panel width
const READOUT_WIDTH: f32 = 180.0;
/// Chart padding
const CHART_PADDING: f32 = 20.0;

fn calculate_layout(available: Rect) -> SmithChartLayout {
    let total = available;

    // Header at top
    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));

    // Readout panel on right
    let readout = Rect::from_min_size(
        Pos2::new(total.max.x - READOUT_WIDTH, header.max.y),
        Vec2::new(READOUT_WIDTH, total.height() - HEADER_HEIGHT),
    );

    // Chart takes remaining space, keeping it square
    let chart_max_width = total.width() - READOUT_WIDTH - CHART_PADDING * 2.0;
    let chart_max_height = total.height() - HEADER_HEIGHT - CHART_PADDING * 2.0;
    let chart_size = chart_max_width.min(chart_max_height);

    let chart_x = total.min.x + CHART_PADDING + (chart_max_width - chart_size) / 2.0;
    let chart_y = header.max.y + CHART_PADDING + (chart_max_height - chart_size) / 2.0;

    let chart = Rect::from_min_size(
        Pos2::new(chart_x, chart_y),
        Vec2::new(chart_size, chart_size),
    );

    SmithChartLayout {
        total,
        header,
        chart,
        readout,
    }
}

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(
    ui: &mut Ui,
    layout: &SmithChartLayout,
    state: &mut SmithChartState,
    app_state: &mut AppState,
) {
    let painter = ui.painter();

    // Background
    painter.rect_filled(layout.header, Rounding::ZERO, Color32::from_rgb(30, 33, 40));

    // Create UI area for header controls
    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.horizontal(|ui| {
            ui.add_space(8.0);

            // Title
            ui.label(
                egui::RichText::new("Smith Chart")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(16.0);

            // Mode selector
            egui::ComboBox::from_label("")
                .selected_text(state.mode.display_name())
                .show_ui(ui, |ui| {
                    for mode in SmithChartMode::all() {
                        ui.selectable_value(&mut state.mode, *mode, mode.display_name());
                    }
                });

            ui.separator();

            // Z0 input
            ui.label("Z₀:");
            let mut z0_str = format!("{:.0}", state.z0);
            let response = ui.add(egui::TextEdit::singleline(&mut z0_str).desired_width(50.0));
            if response.changed() {
                if let Ok(z0) = z0_str.parse::<f64>() {
                    state.set_z0(z0);
                }
            }
            ui.label("Ω");

            ui.separator();

            // Toggle buttons
            if ui
                .small_button(if state.show_grid { "Grid ✓" } else { "Grid" })
                .clicked()
            {
                state.show_grid = !state.show_grid;
            }

            if ui
                .small_button(if state.normalized { "Norm ✓" } else { "Norm" })
                .clicked()
            {
                state.normalized = !state.normalized;
            }

            // Right-aligned close button
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                if ui.small_button("✕").clicked() {
                    app_state.close_active_viewer();
                }
            });
        });
    });
}

// =============================================================================
// Chart Rendering
// =============================================================================

fn render_chart_area(ui: &mut Ui, layout: &SmithChartLayout, state: &mut SmithChartState) {
    render_chart_core(ui, layout, state);

    // Handle mouse interactions
    let response = ui.allocate_rect(layout.chart, Sense::click_and_drag());
    handle_chart_interactions(response, layout, state);
}

fn render_chart_core(ui: &mut Ui, layout: &SmithChartLayout, state: &SmithChartState) {
    let painter = ui.painter();
    let center = layout.chart.center();
    let radius = layout.chart.width() / 2.0 * 0.95; // Slight margin

    // Background
    painter.rect_filled(layout.chart, Rounding::ZERO, chart_bg_color());

    // Unit circle (outer boundary)
    if state.show_unit_circle {
        painter.circle_stroke(center, radius, Stroke::new(2.0, unit_circle_color()));
    }

    // R circles (constant resistance)
    if state.show_r_circles {
        render_r_circles(painter, center, radius, state);
    }

    // X circles (constant reactance)
    if state.show_x_circles {
        render_x_circles(painter, center, radius, state);
    }

    // VSWR circles
    for vswr_circle in state.visible_vswr_circles() {
        let vswr_radius = vswr_circle.radius() as f32 * radius;
        painter.circle_stroke(center, vswr_radius, Stroke::new(1.5, vswr_circle_color()));
    }

    // S-parameter traces
    let colors = trace_colors();
    for (i, trace) in state.traces.iter().enumerate().filter(|(_, t)| t.visible) {
        let color = colors[i % colors.len()];
        render_trace(painter, center, radius, trace, color);
    }

    // Marker
    if state.marker.active {
        let marker_x = center.x + state.marker.x as f32 * radius;
        let marker_y = center.y - state.marker.y as f32 * radius; // Y inverted

        painter.circle_filled(Pos2::new(marker_x, marker_y), 6.0, marker_color());
        painter.circle_stroke(
            Pos2::new(marker_x, marker_y),
            6.0,
            Stroke::new(2.0, Color32::WHITE),
        );
    }

    // Center point
    painter.circle_filled(center, 3.0, Color32::from_rgb(100, 100, 100));
}

fn render_r_circles(painter: &Painter, center: Pos2, radius: f32, _state: &SmithChartState) {
    let stroke = Stroke::new(0.5, r_circle_color());

    for &r in standard_r_values() {
        let (cx, _cy, cr) = constant_r_circle(r);
        let screen_cx = center.x + cx as f32 * radius;
        let screen_cy = center.y; // R circles are centered on real axis
        let screen_cr = cr as f32 * radius;

        // Only draw the part inside the unit circle
        draw_clipped_circle(
            painter, center, radius, screen_cx, screen_cy, screen_cr, stroke,
        );

        // Label
        if r > 0.0 && r <= 5.0 {
            let label_x = center.x + (cx + cr * 0.7) as f32 * radius;
            painter.text(
                Pos2::new(label_x, center.y - 8.0),
                egui::Align2::CENTER_BOTTOM,
                format!("{}", r),
                FontId::proportional(9.0),
                Color32::from_rgb(100, 105, 115),
            );
        }
    }
}

fn render_x_circles(painter: &Painter, center: Pos2, radius: f32, _state: &SmithChartState) {
    let stroke = Stroke::new(0.5, x_circle_color());

    for &x in standard_x_values() {
        if let Some((cx, cy, cr)) = constant_x_circle(x) {
            let screen_cx = center.x + cx as f32 * radius;
            let screen_cy = center.y - cy as f32 * radius; // Y inverted
            let screen_cr = cr as f32 * radius;

            // Draw arc clipped to unit circle
            draw_clipped_arc(
                painter,
                center,
                radius,
                screen_cx,
                screen_cy,
                screen_cr,
                x > 0.0,
                stroke,
            );
        }
    }

    // X = 0 line (real axis)
    painter.line_segment(
        [
            Pos2::new(center.x - radius, center.y),
            Pos2::new(center.x + radius, center.y),
        ],
        Stroke::new(0.5, x_circle_color()),
    );
}

/// Draw a circle clipped to the unit circle
fn draw_clipped_circle(
    painter: &Painter,
    _unit_center: Pos2,
    _unit_radius: f32,
    cx: f32,
    cy: f32,
    cr: f32,
    stroke: Stroke,
) {
    // For now, just draw the whole circle - clipping is visual only
    // In a production implementation, we'd calculate arc segments
    painter.circle_stroke(Pos2::new(cx, cy), cr, stroke);
}

/// Draw an arc of an X circle clipped to unit circle
fn draw_clipped_arc(
    painter: &Painter,
    unit_center: Pos2,
    unit_radius: f32,
    cx: f32,
    cy: f32,
    cr: f32,
    is_upper: bool,
    stroke: Stroke,
) {
    // Calculate arc points
    const ARC_SEGMENTS: usize = 32;
    let mut points = Vec::with_capacity(ARC_SEGMENTS);

    // X circles are tangent to right edge of unit circle
    // Arc from roughly -90° to +90° (adjusted for position)
    let angle_start = if is_upper { -PI / 2.0 } else { PI / 2.0 };
    let angle_span = PI;

    for i in 0..=ARC_SEGMENTS {
        let t = i as f64 / ARC_SEGMENTS as f64;
        let angle = angle_start + t * angle_span;

        let px = cx + cr * angle.cos() as f32;
        let py = cy + cr * angle.sin() as f32;

        // Check if point is inside unit circle
        let dx = px - unit_center.x;
        let dy = py - unit_center.y;
        if dx * dx + dy * dy <= unit_radius * unit_radius * 1.01 {
            points.push(Pos2::new(px, py));
        }
    }

    // Draw connected line segments
    if points.len() >= 2 {
        for window in points.windows(2) {
            painter.line_segment([window[0], window[1]], stroke);
        }
    }
}

fn render_trace(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    trace: &super::state::SParamTrace,
    color: Color32,
) {
    if trace.points.len() < 2 {
        return;
    }

    let stroke = Stroke::new(2.0, color);

    // Convert S-parameter to screen coordinates
    let screen_points: Vec<Pos2> = trace
        .points
        .iter()
        .map(|p| {
            let x = center.x + p.s.re as f32 * radius;
            let y = center.y - p.s.im as f32 * radius; // Y inverted
            Pos2::new(x, y)
        })
        .collect();

    // Draw connected segments
    for window in screen_points.windows(2) {
        painter.line_segment([window[0], window[1]], stroke);
    }

    // Draw markers at start and end
    if let Some(first) = screen_points.first() {
        painter.circle_filled(*first, 4.0, color);
    }
    if let Some(last) = screen_points.last() {
        painter.circle_stroke(*last, 4.0, Stroke::new(2.0, color));
    }
}

// =============================================================================
// Readout Panel
// =============================================================================

fn render_readout_panel(ui: &mut Ui, layout: &SmithChartLayout, state: &SmithChartState) {
    let painter = ui.painter();

    // Background
    painter.rect_filled(
        layout.readout,
        Rounding::ZERO,
        Color32::from_rgb(25, 27, 33),
    );

    // Create UI area
    let readout_rect = layout.readout.shrink(8.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(readout_rect), |ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Marker Readout")
                    .size(11.0)
                    .color(Color32::from_rgb(160, 165, 175)),
            );
            ui.add_space(8.0);

            if state.marker.active {
                let readout = state.marker_readout();

                // Impedance
                ui.label(
                    egui::RichText::new("Impedance:")
                        .size(10.0)
                        .color(Color32::from_rgb(120, 125, 135)),
                );
                if state.normalized {
                    ui.label(
                        egui::RichText::new(format!(
                            "Z = {:.3}+j{:.3}",
                            readout.impedance_normalized.r, readout.impedance_normalized.x
                        ))
                        .size(12.0)
                        .color(Color32::from_rgb(200, 200, 210)),
                    );
                } else {
                    ui.label(
                        egui::RichText::new(format!("{}", readout.impedance))
                            .size(12.0)
                            .color(Color32::from_rgb(200, 200, 210)),
                    );
                }

                ui.add_space(4.0);

                // Gamma
                ui.label(
                    egui::RichText::new(format!(
                        "Γ = {:.3}∠{:.1}°",
                        readout.gamma.magnitude(),
                        readout.gamma.phase_deg()
                    ))
                    .size(11.0)
                    .color(Color32::from_rgb(160, 165, 175)),
                );

                ui.add_space(4.0);

                // VSWR
                ui.label(
                    egui::RichText::new(format!("VSWR = {:.2}:1", readout.vswr))
                        .size(11.0)
                        .color(Color32::from_rgb(160, 165, 175)),
                );

                // Return Loss
                ui.label(
                    egui::RichText::new(format!("RL = {:.1} dB", readout.return_loss_db))
                        .size(11.0)
                        .color(Color32::from_rgb(160, 165, 175)),
                );
            } else {
                ui.label(
                    egui::RichText::new("Click chart to place marker")
                        .size(10.0)
                        .color(Color32::from_rgb(100, 105, 115)),
                );
            }

            ui.add_space(16.0);

            // Reference impedance
            ui.label(
                egui::RichText::new(format!("Z₀ = {:.0}Ω", state.z0))
                    .size(10.0)
                    .color(Color32::from_rgb(100, 105, 115)),
            );
        });
    });
}

// =============================================================================
// Interactions
// =============================================================================

fn handle_chart_interactions(
    response: Response,
    layout: &SmithChartLayout,
    state: &mut SmithChartState,
) {
    let center = layout.chart.center();
    let radius = layout.chart.width() / 2.0 * 0.95;

    // Click to place marker
    if response.clicked() {
        if let Some(pos) = response.hover_pos() {
            let x = (pos.x - center.x) / radius;
            let y = -(pos.y - center.y) / radius; // Y inverted
            state.place_marker(x as f64, y as f64);
        }
    }

    // Right-click to clear marker
    if response.secondary_clicked() {
        state.clear_marker();
    }
}

// =============================================================================
// Demo Data
// =============================================================================

fn load_demo_data(state: &mut SmithChartState) {
    // Create a demo S11 trace showing a resonant antenna
    let mut trace = super::state::SParamTrace::new("S11 (Demo)");

    // Simulate frequency sweep with resonance
    let f_start = 100e6;
    let f_stop = 1000e6;
    let f_center = 500e6;
    let bw = 50e6;

    for i in 0..100 {
        let f = f_start + (f_stop - f_start) * i as f64 / 99.0;

        // Simple resonant impedance model
        let f_normalized = (f - f_center) / bw;
        let gamma_mag = 0.9 * (1.0 / (1.0 + f_normalized * f_normalized)).sqrt();
        let gamma_phase = -PI * f_normalized.atan();

        let s = Complex::from_polar(gamma_mag, gamma_phase);
        trace.add_point(f, s);
    }

    state.traces.push(trace);
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

        // Chart should be square
        assert!((layout.chart.width() - layout.chart.height()).abs() < 0.1);

        // Chart should fit within available space
        assert!(layout.chart.max.x <= layout.readout.min.x);
        assert!(layout.chart.min.y >= layout.header.max.y);
    }

    #[test]
    fn test_trace_colors() {
        let colors = trace_colors();
        assert!(colors.len() >= 5);
    }

    #[test]
    fn test_demo_data() {
        let mut state = SmithChartState::new();
        load_demo_data(&mut state);
        assert_eq!(state.traces.len(), 1);
        assert!(!state.traces[0].points.is_empty());
    }
}
