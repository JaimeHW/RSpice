//! Bode Plot Rendering
//!
//! Commercial-grade egui rendering for Bode plot visualization.

use egui::{
    Color32, FontId, Painter, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2,
};
use std::f64::consts::PI;

use super::data::{BodeData, FrequencyPoint, FrequencyResponse};
use super::state::{BodeDisplayMode, BodePlotState};
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

fn mag_trace_color() -> Color32 {
    Color32::from_rgb(100, 180, 255)
}

fn phase_trace_color() -> Color32 {
    Color32::from_rgb(255, 150, 100)
}

fn crossover_color() -> Color32 {
    Color32::from_rgb(255, 255, 100)
}

fn margin_color() -> Color32 {
    Color32::from_rgba_unmultiplied(100, 255, 100, 150)
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

/// Render the Bode plot viewer panel
pub fn render_bode_viewer(ui: &mut Ui, app_state: &mut AppState) {
    let available_rect = ui.available_rect_before_wrap();
    let close_requested = {
        let state = &mut app_state.bode_plot_state;
        let layout = calculate_layout(available_rect, state);
        let close_requested = render_header(ui, &layout, state);
        render_plots(ui, &layout, state);
        render_info_panel(ui, &layout, state);
        close_requested
    };
    if close_requested {
        app_state.active_viewer = crate::viewers::ActiveViewer::Waveform;
    }
}

/// Public render function for external use
pub fn render_bode_plot(ui: &mut Ui, state: &BodePlotState) {
    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect, state);

    render_plots(ui, &layout, state);
}

// =============================================================================
// Layout
// =============================================================================

#[derive(Debug, Clone)]
struct BodeLayout {
    total: Rect,
    header: Rect,
    magnitude: Option<Rect>,
    phase: Option<Rect>,
    info: Rect,
}

const HEADER_HEIGHT: f32 = 32.0;
const INFO_WIDTH: f32 = 160.0;
const CHART_PADDING: f32 = 8.0;
const PLOT_GAP: f32 = 4.0;

fn calculate_layout(available: Rect, state: &BodePlotState) -> BodeLayout {
    let total = available;

    // Ensure minimum dimensions
    let min_width = CHART_PADDING * 4.0 + INFO_WIDTH;
    let min_height = HEADER_HEIGHT + CHART_PADDING * 2.0 + 50.0;

    // If too small, return minimal layout with no plots
    if total.width() < min_width || total.height() < min_height {
        let header = Rect::from_min_size(
            total.min,
            Vec2::new(total.width(), HEADER_HEIGHT.min(total.height())),
        );
        return BodeLayout {
            total,
            header,
            magnitude: None,
            phase: None,
            info: Rect::NOTHING,
        };
    }

    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));

    let info = Rect::from_min_size(
        Pos2::new(total.max.x - INFO_WIDTH, header.max.y),
        Vec2::new(INFO_WIDTH, total.height() - HEADER_HEIGHT),
    );

    let plot_left = total.min.x + CHART_PADDING;
    let plot_right = (info.min.x - CHART_PADDING).max(plot_left + 50.0);
    let plot_top = header.max.y + CHART_PADDING;
    let plot_bottom = (total.max.y - CHART_PADDING).max(plot_top + 50.0);

    let plot_area = Rect::from_min_max(
        Pos2::new(plot_left, plot_top),
        Pos2::new(plot_right, plot_bottom),
    );

    let (magnitude, phase) = match state.mode {
        BodeDisplayMode::Both => {
            let mid_y = plot_area.min.y + (plot_area.height() - PLOT_GAP) / 2.0;
            (
                Some(Rect::from_min_max(
                    plot_area.min,
                    Pos2::new(plot_area.max.x, mid_y),
                )),
                Some(Rect::from_min_max(
                    Pos2::new(plot_area.min.x, mid_y + PLOT_GAP),
                    plot_area.max,
                )),
            )
        }
        BodeDisplayMode::MagnitudeOnly => (Some(plot_area), None),
        BodeDisplayMode::PhaseOnly => (None, Some(plot_area)),
    };

    BodeLayout {
        total,
        header,
        magnitude,
        phase,
        info,
    }
}

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(
    ui: &mut Ui,
    layout: &BodeLayout,
    state: &mut BodePlotState,
) -> bool {
    let painter = ui.painter();
    let mut close_requested = false;

    painter.rect_filled(layout.header, Rounding::ZERO, Color32::from_rgb(30, 33, 40));

    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.horizontal(|ui| {
            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Bode Plot")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(16.0);

            // Mode selector
            egui::ComboBox::from_label("")
                .selected_text(state.mode.display_name())
                .show_ui(ui, |ui| {
                    for mode in BodeDisplayMode::all() {
                        ui.selectable_value(&mut state.mode, *mode, mode.display_name());
                    }
                });

            ui.separator();

            // Toggle buttons
            if ui
                .small_button(if state.show_margins {
                    "Margins ✓"
                } else {
                    "Margins"
                })
                .clicked()
            {
                state.toggle_margins();
            }

            if ui
                .small_button(if state.show_grid { "Grid ✓" } else { "Grid" })
                .clicked()
            {
                state.toggle_grid();
            }

            if ui
                .small_button(if state.show_cursor {
                    "Cursor ✓"
                } else {
                    "Cursor"
                })
                .clicked()
            {
                state.toggle_cursor();
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
// Plot Rendering
// =============================================================================

fn render_plots(ui: &mut Ui, layout: &BodeLayout, state: &BodePlotState) {
    // Clone painter to avoid borrow conflict with ui.allocate_rect
    let painter = ui.painter().clone();

    // Magnitude plot
    if let Some(mag_rect) = layout.magnitude {
        render_magnitude_plot(&painter, mag_rect, state);
    }

    // Phase plot
    if let Some(phase_rect) = layout.phase {
        render_phase_plot(&painter, phase_rect, state);
    }

    // Allocate rects after painting
    if let Some(mag_rect) = layout.magnitude {
        ui.allocate_rect(mag_rect, Sense::click());
    }
    if let Some(phase_rect) = layout.phase {
        ui.allocate_rect(phase_rect, Sense::click());
    }
}

fn render_magnitude_plot(painter: &Painter, rect: Rect, state: &BodePlotState) {
    // Background
    painter.rect_filled(rect, Rounding::ZERO, chart_bg_color());

    // Grid
    if state.show_grid {
        render_log_grid(painter, rect, state.mag_min, state.mag_max);
    }

    // 0 dB line
    let zero_y = map_to_y(0.0, state.mag_min, state.mag_max, rect);
    if zero_y >= rect.min.y && zero_y <= rect.max.y {
        painter.line_segment(
            [Pos2::new(rect.min.x, zero_y), Pos2::new(rect.max.x, zero_y)],
            Stroke::new(1.0, crossover_color()),
        );
    }

    // Traces
    if let Some(resp) = state.data.primary() {
        if let Some((f_min, f_max)) = resp.frequency_range() {
            render_magnitude_trace(painter, rect, resp, f_min, f_max, state);
        }
    }

    // Label
    painter.text(
        Pos2::new(rect.min.x + 5.0, rect.min.y + 5.0),
        egui::Align2::LEFT_TOP,
        "Magnitude (dB)",
        FontId::proportional(10.0),
        text_color(),
    );

    // Border
    painter.rect_stroke(
        rect,
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
    );
}

fn render_phase_plot(painter: &Painter, rect: Rect, state: &BodePlotState) {
    // Background
    painter.rect_filled(rect, Rounding::ZERO, chart_bg_color());

    // Grid
    if state.show_grid {
        render_log_grid(painter, rect, state.phase_min, state.phase_max);
    }

    // -180° line (for stability)
    let minus180_y = map_to_y(-180.0, state.phase_min, state.phase_max, rect);
    if minus180_y >= rect.min.y && minus180_y <= rect.max.y {
        painter.line_segment(
            [
                Pos2::new(rect.min.x, minus180_y),
                Pos2::new(rect.max.x, minus180_y),
            ],
            Stroke::new(1.0, crossover_color()),
        );
    }

    // Traces
    if let Some(resp) = state.data.primary() {
        if let Some((f_min, f_max)) = resp.frequency_range() {
            render_phase_trace(painter, rect, resp, f_min, f_max, state);
        }
    }

    // Label
    painter.text(
        Pos2::new(rect.min.x + 5.0, rect.min.y + 5.0),
        egui::Align2::LEFT_TOP,
        "Phase (°)",
        FontId::proportional(10.0),
        text_color(),
    );

    // Border
    painter.rect_stroke(
        rect,
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
    );
}

fn render_log_grid(painter: &Painter, rect: Rect, _y_min: f64, _y_max: f64) {
    let stroke = Stroke::new(0.5, grid_color());

    // Vertical lines (log decades)
    let num_decades = 5;
    for i in 0..=num_decades {
        let x = rect.min.x + (i as f32 / num_decades as f32) * rect.width();
        painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], stroke);
    }

    // Horizontal lines
    let y_divisions = 5;
    for i in 0..=y_divisions {
        let y = rect.min.y + (i as f32 / y_divisions as f32) * rect.height();
        painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], stroke);
    }
}

fn render_magnitude_trace(
    painter: &Painter,
    rect: Rect,
    resp: &FrequencyResponse,
    f_min: f64,
    f_max: f64,
    state: &BodePlotState,
) {
    let stroke = Stroke::new(1.5, mag_trace_color());

    let points: Vec<Pos2> = resp
        .points
        .iter()
        .filter_map(|p| {
            let x = map_log_to_x(p.frequency, f_min, f_max, rect);
            let y = map_to_y(p.magnitude_db(), state.mag_min, state.mag_max, rect);
            if x.is_finite() && y.is_finite() {
                Some(Pos2::new(x, y.clamp(rect.min.y, rect.max.y)))
            } else {
                None
            }
        })
        .collect();

    for window in points.windows(2) {
        painter.line_segment([window[0], window[1]], stroke);
    }
}

fn render_phase_trace(
    painter: &Painter,
    rect: Rect,
    resp: &FrequencyResponse,
    f_min: f64,
    f_max: f64,
    state: &BodePlotState,
) {
    let stroke = Stroke::new(1.5, phase_trace_color());

    let points: Vec<Pos2> = resp
        .points
        .iter()
        .filter_map(|p| {
            let x = map_log_to_x(p.frequency, f_min, f_max, rect);
            let y = map_to_y(p.phase_deg(), state.phase_min, state.phase_max, rect);
            if x.is_finite() && y.is_finite() {
                Some(Pos2::new(x, y.clamp(rect.min.y, rect.max.y)))
            } else {
                None
            }
        })
        .collect();

    for window in points.windows(2) {
        painter.line_segment([window[0], window[1]], stroke);
    }
}

fn map_log_to_x(freq: f64, f_min: f64, f_max: f64, rect: Rect) -> f32 {
    if freq <= 0.0 || f_min <= 0.0 || f_max <= 0.0 {
        return rect.min.x;
    }
    let log_range = f_max.log10() - f_min.log10();
    if log_range <= 0.0 {
        return rect.center().x;
    }
    let t = (freq.log10() - f_min.log10()) / log_range;
    rect.min.x + t as f32 * rect.width()
}

fn map_to_y(value: f64, v_min: f64, v_max: f64, rect: Rect) -> f32 {
    let range = v_max - v_min;
    if range <= 0.0 {
        return rect.center().y;
    }
    let t = (value - v_min) / range;
    rect.max.y - t as f32 * rect.height()
}

// =============================================================================
// Info Panel
// =============================================================================

fn render_info_panel(ui: &mut Ui, layout: &BodeLayout, state: &BodePlotState) {
    // Paint background first
    ui.painter()
        .rect_filled(layout.info, Rounding::ZERO, Color32::from_rgb(25, 27, 33));

    let panel_rect = layout.info.shrink(8.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Info").size(11.0).color(text_color()));
            ui.add_space(8.0);

            if let Some(resp) = state.data.primary() {
                info_row(ui, "Traces", &format!("{}", state.trace_count()));

                if let Some((f_min, f_max)) = resp.frequency_range() {
                    info_row(ui, "F min", &format_freq(f_min));
                    info_row(ui, "F max", &format_freq(f_max));
                }

                if let Some(dc) = resp.dc_gain_db() {
                    info_row(ui, "DC Gain", &format!("{:.1} dB", dc));
                }

                if let Some(bw) = resp.bandwidth_3db() {
                    info_row(ui, "-3dB BW", &format_freq(bw));
                }

                // Margins
                if state.show_margins {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);

                    if let Some(margins) = &state.data.margins {
                        info_row(ui, "Gain Margin", &margins.format_gain_margin());
                        info_row(ui, "Phase Margin", &margins.format_phase_margin());

                        let stability = if margins.is_stable {
                            "Stable"
                        } else {
                            "Unstable"
                        };
                        let color = if margins.is_stable {
                            Color32::from_rgb(100, 200, 100)
                        } else {
                            Color32::from_rgb(255, 100, 100)
                        };

                        ui.label(egui::RichText::new(stability).size(11.0).color(color));
                    } else {
                        ui.label(
                            egui::RichText::new("No margins")
                                .size(10.0)
                                .color(Color32::from_rgb(100, 105, 115)),
                        );
                    }
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

fn format_freq(freq: f64) -> String {
    if freq >= 1e9 {
        format!("{:.2} GHz", freq / 1e9)
    } else if freq >= 1e6 {
        format!("{:.2} MHz", freq / 1e6)
    } else if freq >= 1e3 {
        format!("{:.2} kHz", freq / 1e3)
    } else {
        format!("{:.2} Hz", freq)
    }
}

// =============================================================================
// Demo Data
// =============================================================================

fn load_demo_data(state: &mut BodePlotState) {
    // Create a demo second-order lowpass response
    let mut resp = FrequencyResponse::new("Demo TF");
    let fc = 1000.0; // 1kHz
    let q = 0.707; // Butterworth

    for i in 0..51 {
        let f = 10.0_f64.powf(i as f64 / 10.0); // 1Hz to 100kHz
        let omega = 2.0 * PI * f;
        let omega_c = 2.0 * PI * fc;
        let s_normalized = omega / omega_c;

        // Second order: H(s) = 1 / (s^2 + s/Q + 1)
        let denom_re = 1.0 - s_normalized * s_normalized;
        let denom_im = s_normalized / q;
        let denom_mag_sq = denom_re * denom_re + denom_im * denom_im;

        let mag = 1.0 / denom_mag_sq.sqrt();
        let phase = -denom_im.atan2(denom_re);

        resp.add_point(FrequencyPoint::new(f, mag, phase));
    }

    let mut data = BodeData::new();
    data.add_response(resp);
    data.calculate_margins();

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
        let state = BodePlotState::new();
        let layout = calculate_layout(rect, &state);

        assert!(layout.magnitude.is_some());
        assert!(layout.phase.is_some());
    }

    #[test]
    fn test_layout_magnitude_only() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let mut state = BodePlotState::new();
        state.mode = BodeDisplayMode::MagnitudeOnly;

        let layout = calculate_layout(rect, &state);

        assert!(layout.magnitude.is_some());
        assert!(layout.phase.is_none());
    }

    #[test]
    fn test_map_log_to_x() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));

        // At min frequency
        let x = map_log_to_x(10.0, 10.0, 10000.0, rect);
        assert!((x - 0.0).abs() < 0.1);

        // At max frequency
        let x = map_log_to_x(10000.0, 10.0, 10000.0, rect);
        assert!((x - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_format_freq() {
        assert!(format_freq(1000.0).contains("kHz"));
        assert!(format_freq(1e6).contains("MHz"));
        assert!(format_freq(1e9).contains("GHz"));
    }

    #[test]
    fn test_load_demo_data() {
        let mut state = BodePlotState::new();
        load_demo_data(&mut state);

        assert!(!state.is_empty());
        assert!(state.data.primary().is_some());
    }
}
