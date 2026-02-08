//! Nyquist Plot Rendering
//!
//! Commercial-grade egui rendering for Nyquist plot visualization.

use egui::{Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2};
use std::f64::consts::PI;

use super::data::{NyquistData, NyquistPoint};
use super::state::{NyquistOverlay, NyquistState};
use crate::common::app::AppState;

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
    Color32::from_rgb(100, 180, 255)
}

fn unity_circle_color() -> Color32 {
    Color32::from_rgba_unmultiplied(255, 200, 100, 100)
}

fn critical_point_color() -> Color32 {
    Color32::from_rgb(255, 80, 80)
}

fn stable_color() -> Color32 {
    Color32::from_rgb(80, 200, 80)
}

fn unstable_color() -> Color32 {
    Color32::from_rgb(255, 80, 80)
}

fn text_color() -> Color32 {
    Color32::from_rgb(180, 185, 195)
}

fn axis_color() -> Color32 {
    Color32::from_rgb(100, 105, 115)
}

// =============================================================================
// Main Rendering Entry Point
// =============================================================================

/// Render the Nyquist plot viewer panel
pub fn render_nyquist_viewer(ui: &mut Ui, _app_state: &mut AppState) {
    let mut state = NyquistState::new();
    load_demo_data(&mut state);

    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect);

    render_header(ui, &layout, &mut state);
    render_plot(ui, &layout, &state);
    render_info_panel(ui, &layout, &state);
}

/// Public render function
pub fn render_nyquist_plot(ui: &mut Ui, state: &NyquistState) {
    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect);

    render_plot_core(ui, &layout, state);
}

// =============================================================================
// Layout
// =============================================================================

#[derive(Debug, Clone)]
struct NyquistLayout {
    total: Rect,
    header: Rect,
    plot: Rect,
    info: Rect,
}

const HEADER_HEIGHT: f32 = 32.0;
const INFO_WIDTH: f32 = 140.0;
const CHART_PADDING: f32 = 8.0;

fn calculate_layout(available: Rect) -> NyquistLayout {
    let total = available;

    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));

    let info = Rect::from_min_size(
        Pos2::new(total.max.x - INFO_WIDTH, header.max.y),
        Vec2::new(INFO_WIDTH, total.height() - HEADER_HEIGHT),
    );

    let plot = Rect::from_min_max(
        Pos2::new(total.min.x + CHART_PADDING, header.max.y + CHART_PADDING),
        Pos2::new(info.min.x - CHART_PADDING, total.max.y - CHART_PADDING),
    );

    NyquistLayout {
        total,
        header,
        plot,
        info,
    }
}

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(ui: &mut Ui, layout: &NyquistLayout, state: &mut NyquistState) {
    ui.painter()
        .rect_filled(layout.header, Rounding::ZERO, Color32::from_rgb(30, 33, 40));

    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.horizontal(|ui| {
            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Nyquist Plot")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(16.0);

            // Overlay selector
            egui::ComboBox::from_label("")
                .selected_text(state.overlay.display_name())
                .show_ui(ui, |ui| {
                    for overlay in NyquistOverlay::all() {
                        ui.selectable_value(&mut state.overlay, *overlay, overlay.display_name());
                    }
                });

            ui.separator();

            // Toggle buttons
            if ui
                .small_button(if state.show_grid { "Grid ✓" } else { "Grid" })
                .clicked()
            {
                state.toggle_grid();
            }

            if ui
                .small_button(if state.show_critical_point {
                    "Critical ✓"
                } else {
                    "Critical"
                })
                .clicked()
            {
                state.toggle_critical_point();
            }

            if ui
                .small_button(if state.equal_axes { "1:1 ✓" } else { "1:1" })
                .clicked()
            {
                state.toggle_equal_axes();
            }
        });
    });
}

// =============================================================================
// Plot Rendering
// =============================================================================

fn render_plot(ui: &mut Ui, layout: &NyquistLayout, state: &NyquistState) {
    render_plot_core(ui, layout, state);
    ui.allocate_rect(layout.plot, Sense::click());
}

fn render_plot_core(ui: &mut Ui, layout: &NyquistLayout, state: &NyquistState) {
    let painter = ui.painter().clone();
    let rect = layout.plot;

    // Background
    painter.rect_filled(rect, Rounding::ZERO, chart_bg_color());

    // Grid
    if state.show_grid {
        render_grid(&painter, rect, state);
    }

    // Axes
    render_axes(&painter, rect, state);

    // Unity circle overlay
    if state.overlay.show_unity_circle() {
        render_unity_circle(&painter, rect, state);
    }

    // Critical point
    if state.show_critical_point {
        render_critical_point(&painter, rect, state);
    }

    // Curves
    for curve in &state.curves {
        render_curve(&painter, rect, curve, state);
    }

    // Border
    painter.rect_stroke(
        rect,
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
    );
}

fn render_grid(painter: &egui::Painter, rect: Rect, _state: &NyquistState) {
    let stroke = Stroke::new(0.5, grid_color());

    // Vertical lines (real axis)
    let num_lines = 8;
    for i in 0..=num_lines {
        let t = i as f32 / num_lines as f32;
        let x = rect.min.x + t * rect.width();
        painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], stroke);
    }

    // Horizontal lines (imaginary axis)
    for i in 0..=num_lines {
        let t = i as f32 / num_lines as f32;
        let y = rect.min.y + t * rect.height();
        painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], stroke);
    }
}

fn render_axes(painter: &egui::Painter, rect: Rect, state: &NyquistState) {
    let stroke = Stroke::new(1.0, axis_color());

    // Real axis (y = 0)
    let zero_y = map_to_screen(0.0, 0.0, rect, state).y;
    if zero_y >= rect.min.y && zero_y <= rect.max.y {
        painter.line_segment(
            [Pos2::new(rect.min.x, zero_y), Pos2::new(rect.max.x, zero_y)],
            stroke,
        );
    }

    // Imaginary axis (x = 0)
    let zero_x = map_to_screen(0.0, 0.0, rect, state).x;
    if zero_x >= rect.min.x && zero_x <= rect.max.x {
        painter.line_segment(
            [Pos2::new(zero_x, rect.min.y), Pos2::new(zero_x, rect.max.y)],
            stroke,
        );
    }

    // Labels
    painter.text(
        Pos2::new(rect.max.x - 5.0, zero_y + 12.0),
        egui::Align2::RIGHT_TOP,
        "Re",
        FontId::proportional(10.0),
        text_color(),
    );

    painter.text(
        Pos2::new(zero_x + 5.0, rect.min.y + 5.0),
        egui::Align2::LEFT_TOP,
        "Im",
        FontId::proportional(10.0),
        text_color(),
    );
}

fn render_unity_circle(painter: &egui::Painter, rect: Rect, state: &NyquistState) {
    // Circle centered at origin with radius 1
    let center = map_to_screen(0.0, 0.0, rect, state);

    // Calculate radius in screen coordinates
    let edge = map_to_screen(1.0, 0.0, rect, state);
    let radius = (edge.x - center.x).abs();

    painter.circle_stroke(center, radius, Stroke::new(1.0, unity_circle_color()));
}

fn render_critical_point(painter: &egui::Painter, rect: Rect, state: &NyquistState) {
    let pos = map_to_screen(-1.0, 0.0, rect, state);

    // Draw X marker
    let size = 5.0;
    let stroke = Stroke::new(2.0, critical_point_color());

    painter.line_segment(
        [
            Pos2::new(pos.x - size, pos.y - size),
            Pos2::new(pos.x + size, pos.y + size),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x + size, pos.y - size),
            Pos2::new(pos.x - size, pos.y + size),
        ],
        stroke,
    );

    // Label
    painter.text(
        Pos2::new(pos.x, pos.y + 12.0),
        egui::Align2::CENTER_TOP,
        "(-1, 0)",
        FontId::proportional(9.0),
        critical_point_color(),
    );
}

fn render_curve(painter: &egui::Painter, rect: Rect, curve: &NyquistData, state: &NyquistState) {
    if curve.is_empty() {
        return;
    }

    let stroke = Stroke::new(1.5, trace_color());

    let points: Vec<Pos2> = curve
        .points
        .iter()
        .filter_map(|p| {
            let screen = map_to_screen(p.real, p.imag, rect, state);
            if screen.x >= rect.min.x
                && screen.x <= rect.max.x
                && screen.y >= rect.min.y
                && screen.y <= rect.max.y
            {
                Some(screen)
            } else {
                None
            }
        })
        .collect();

    for window in points.windows(2) {
        painter.line_segment([window[0], window[1]], stroke);
    }

    // Start/end markers
    if let Some(first) = curve.points.first() {
        let pos = map_to_screen(first.real, first.imag, rect, state);
        painter.circle_filled(pos, 4.0, trace_color());
    }
}

fn map_to_screen(real: f64, imag: f64, rect: Rect, state: &NyquistState) -> Pos2 {
    let real_range = state.real_max - state.real_min;
    let imag_range = state.imag_max - state.imag_min;

    let x = if real_range > 0.0 {
        rect.min.x + ((real - state.real_min) / real_range) as f32 * rect.width()
    } else {
        rect.center().x
    };

    let y = if imag_range > 0.0 {
        rect.max.y - ((imag - state.imag_min) / imag_range) as f32 * rect.height()
    } else {
        rect.center().y
    };

    Pos2::new(x, y)
}

// =============================================================================
// Info Panel
// =============================================================================

fn render_info_panel(ui: &mut Ui, layout: &NyquistLayout, state: &NyquistState) {
    ui.painter()
        .rect_filled(layout.info, Rounding::ZERO, Color32::from_rgb(25, 27, 33));

    let panel_rect = layout.info.shrink(8.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Stability")
                    .size(11.0)
                    .color(text_color()),
            );
            ui.add_space(8.0);

            if let Some(curve) = state.current_curve() {
                info_row(ui, "Points", &format!("{}", curve.len()));

                if let Some(min_dist) = curve.min_distance_from_critical() {
                    info_row(ui, "Min Dist", &format!("{:.3}", min_dist));
                }

                let enc = curve.count_encirclements();
                info_row(ui, "Encircl.", &format!("{}", enc));

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                let is_stable = curve.is_stable_open_loop();
                let stability_text = if is_stable { "STABLE" } else { "UNSTABLE" };
                let stability_color = if is_stable {
                    stable_color()
                } else {
                    unstable_color()
                };

                ui.label(
                    egui::RichText::new(stability_text)
                        .size(12.0)
                        .strong()
                        .color(stability_color),
                );

                // Margins
                if let Some(gm) = curve.gain_margin() {
                    let gm_db = 20.0 * gm.log10();
                    info_row(ui, "GM", &format!("{:.1} dB", gm_db));
                }

                if let Some(pm) = curve.phase_margin() {
                    info_row(ui, "PM", &format!("{:.1}°", pm));
                }
            } else {
                ui.label(
                    egui::RichText::new("No data")
                        .size(10.0)
                        .color(Color32::from_rgb(100, 105, 115)),
                );
            }
        });
    });
}

fn info_row(ui: &mut Ui, label: &str, value: &str) {
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

fn load_demo_data(state: &mut NyquistState) {
    // Create demo loop gain: second-order system
    let mut data = NyquistData::new("Demo Loop Gain");
    let fc = 1000.0;
    let q = 0.707;

    for i in 0..101 {
        let f = 10.0_f64.powf(i as f64 / 20.0 - 1.0); // 0.1 to 100kHz
        let omega = 2.0 * PI * f;
        let omega_c = 2.0 * PI * fc;
        let s = omega / omega_c;

        // H(s) = K / (s^2 + s/Q + 1), with K = 2 for moderate gain
        let denom_re = 1.0 - s * s;
        let denom_im = s / q;
        let denom_mag_sq = denom_re * denom_re + denom_im * denom_im;

        let k = 2.0;
        let real = k * denom_re / denom_mag_sq;
        let imag = -k * denom_im / denom_mag_sq;

        data.add_point(NyquistPoint::new(f, real, imag));
    }

    state.load_data(data);
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

        assert!(layout.plot.width() > 0.0);
        assert!(layout.plot.height() > 0.0);
    }

    #[test]
    fn test_map_to_screen() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
        let mut state = NyquistState::new();
        state.real_min = -1.0;
        state.real_max = 1.0;
        state.imag_min = -1.0;
        state.imag_max = 1.0;

        // Origin should map to center
        let center = map_to_screen(0.0, 0.0, rect, &state);
        assert!((center.x - 50.0).abs() < 0.1);
        assert!((center.y - 50.0).abs() < 0.1);

        // (-1, 0) should be on left
        let left = map_to_screen(-1.0, 0.0, rect, &state);
        assert!((left.x - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_load_demo_data() {
        let mut state = NyquistState::new();
        load_demo_data(&mut state);

        assert!(!state.is_empty());
        assert!(state.current_curve().is_some());
    }
}
