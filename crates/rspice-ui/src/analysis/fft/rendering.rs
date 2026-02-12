//! FFT Viewer Rendering
//!
//! Commercial-grade egui rendering for FFT/spectrum visualization.

use egui::{Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2};
use std::f64::consts::PI;

use super::data::{FftData, FftPoint, SpectrumAnalysis};
use super::state::{FftState, FrequencyScale, MagnitudeScale};
use super::window::WindowFunction;
use crate::common::app::AppState;

// =============================================================================
// Constants
// =============================================================================

fn chart_bg_color() -> Color32 {
    Color32::from_rgb(15, 17, 21)
}

fn surface_bg_color() -> Color32 {
    Color32::from_rgb(25, 27, 33)
}

fn header_bg_color() -> Color32 {
    Color32::from_rgb(30, 33, 40)
}

fn grid_color() -> Color32 {
    Color32::from_rgb(40, 45, 55)
}

fn trace_color() -> Color32 {
    Color32::from_rgb(100, 180, 255)
}

fn peak_color() -> Color32 {
    Color32::from_rgb(255, 200, 100)
}

fn harmonic_color() -> Color32 {
    Color32::from_rgb(255, 100, 150)
}

fn fundamental_color() -> Color32 {
    Color32::from_rgb(100, 255, 100)
}

fn text_color() -> Color32 {
    Color32::from_rgb(180, 185, 195)
}

// =============================================================================
// Main Rendering Entry Point
// =============================================================================

/// Render the FFT viewer panel
pub fn render_fft_viewer(ui: &mut Ui, app_state: &mut AppState) {
    let available_rect = ui.available_rect_before_wrap();
    // Claim full available space so the parent resizable panel keeps user height
    // instead of collapsing to a content-driven "natural" size.
    let (_id, _rect) = ui.allocate_space(available_rect.size());
    let state = &mut app_state.fft_state;
    let layout = calculate_layout(available_rect);

    render_header(ui, &layout, state);
    render_spectrum(ui, &layout, state);
    render_info_panel(ui, &layout, state);
}

/// Public render function
pub fn render_fft_plot(ui: &mut Ui, state: &FftState) {
    let available_rect = ui.available_rect_before_wrap();
    let (_id, _rect) = ui.allocate_space(available_rect.size());
    let layout = calculate_layout(available_rect);

    render_spectrum_core(ui, &layout, state);
}

// =============================================================================
// Layout
// =============================================================================

#[derive(Debug, Clone)]
struct FftLayout {
    total: Rect,
    header: Rect,
    spectrum: Rect,
    info: Rect,
}

const HEADER_HEIGHT: f32 = 32.0;
const INFO_WIDTH: f32 = 150.0;
const CHART_SIDE_PADDING: f32 = 8.0;
const CHART_TOP_GAP: f32 = 0.0;
const CHART_BOTTOM_PADDING: f32 = 8.0;
const HEADER_CONTROL_HEIGHT: f32 = 24.0;
const HEADER_WINDOW_WIDTH: f32 = 176.0;
const HEADER_SCALE_WIDTH: f32 = 116.0;
const AXIS_LEFT_GUTTER: f32 = 26.0;
const AXIS_RIGHT_GUTTER: f32 = 4.0;
const AXIS_TOP_GUTTER: f32 = 2.0;
const AXIS_BOTTOM_GUTTER: f32 = 16.0;

fn calculate_layout(available: Rect) -> FftLayout {
    let total = available;

    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));

    let info = Rect::from_min_size(
        Pos2::new(total.max.x - INFO_WIDTH, header.max.y),
        Vec2::new(INFO_WIDTH, total.height() - HEADER_HEIGHT),
    );

    let spectrum = Rect::from_min_max(
        Pos2::new(
            total.min.x + CHART_SIDE_PADDING,
            header.max.y + CHART_TOP_GAP,
        ),
        Pos2::new(
            info.min.x - CHART_SIDE_PADDING,
            total.max.y - CHART_BOTTOM_PADDING,
        ),
    );

    FftLayout {
        total,
        header,
        spectrum,
        info,
    }
}

fn spectrum_plot_rect(spectrum_rect: Rect) -> Rect {
    let min_x = (spectrum_rect.min.x + AXIS_LEFT_GUTTER).min(spectrum_rect.max.x - 1.0);
    let max_x = (spectrum_rect.max.x - AXIS_RIGHT_GUTTER).max(min_x + 1.0);
    let min_y = (spectrum_rect.min.y + AXIS_TOP_GUTTER).min(spectrum_rect.max.y - 1.0);
    let max_y = (spectrum_rect.max.y - AXIS_BOTTOM_GUTTER).max(min_y + 1.0);
    Rect::from_min_max(Pos2::new(min_x, min_y), Pos2::new(max_x, max_y))
}

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(ui: &mut Ui, layout: &FftLayout, state: &mut FftState) {
    ui.painter()
        .rect_filled(layout.header, Rounding::ZERO, header_bg_color());

    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().interact_size.y = HEADER_CONTROL_HEIGHT;
            ui.spacing_mut().button_padding.y = 2.0;
            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("FFT Spectrum")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(16.0);

            // Window selector
            egui::ComboBox::from_id_salt("window")
                .width(HEADER_WINDOW_WIDTH)
                .selected_text(state.window.display_name())
                .show_ui(ui, |ui| {
                    for w in WindowFunction::all() {
                        ui.selectable_value(&mut state.window, *w, w.display_name());
                    }
                });

            // Magnitude scale
            egui::ComboBox::from_id_salt("mag_scale")
                .width(HEADER_SCALE_WIDTH)
                .selected_text(state.mag_scale.display_name())
                .show_ui(ui, |ui| {
                    for s in MagnitudeScale::all() {
                        ui.selectable_value(&mut state.mag_scale, *s, s.display_name());
                    }
                });

            // Frequency scale
            egui::ComboBox::from_id_salt("freq_scale")
                .width(HEADER_SCALE_WIDTH)
                .selected_text(state.freq_scale.display_name())
                .show_ui(ui, |ui| {
                    for s in FrequencyScale::all() {
                        ui.selectable_value(&mut state.freq_scale, *s, s.display_name());
                    }
                });

            ui.separator();
            let peaks_label = if state.show_peaks {
                "Peaks [on]"
            } else {
                "Peaks"
            };
            if ui
                .add(
                    egui::Button::new(peaks_label)
                        .min_size(egui::vec2(72.0, HEADER_CONTROL_HEIGHT)),
                )
                .clicked()
            {
                state.toggle_peaks();
            }
            let harmonics_label = if state.show_harmonics {
                "Harm [on]"
            } else {
                "Harm"
            };
            if ui
                .add(
                    egui::Button::new(harmonics_label)
                        .min_size(egui::vec2(72.0, HEADER_CONTROL_HEIGHT)),
                )
                .clicked()
            {
                state.toggle_harmonics();
            }
        });
    });
}

// =============================================================================
// Spectrum Rendering
// =============================================================================

fn render_spectrum(ui: &mut Ui, layout: &FftLayout, state: &FftState) {
    render_spectrum_core(ui, layout, state);
    ui.allocate_rect(layout.spectrum, Sense::click());
}

fn render_spectrum_core(ui: &mut Ui, layout: &FftLayout, state: &FftState) {
    let painter = ui.painter().clone();
    let rect = layout.spectrum;
    let plot_rect = spectrum_plot_rect(rect);

    // Draw non-plot surface and plot area separately so axis gutters
    // never share the same background as waveform data.
    painter.rect_filled(rect, Rounding::ZERO, surface_bg_color());
    painter.rect_filled(plot_rect, Rounding::ZERO, chart_bg_color());

    // Grid
    if state.show_grid {
        render_grid(&painter, plot_rect, state);
    }

    // Spectrum trace
    if let Some(ref data) = state.data {
        render_trace(&painter, plot_rect, data, state);

        // Fundamental marker
        if let Some(ref analysis) = state.analysis {
            if let Some(fund_freq) = analysis.fundamental_frequency {
                render_fundamental_marker(&painter, plot_rect, fund_freq, data, state);
            }

            // Harmonic markers
            if state.show_harmonics {
                for (freq, db) in &analysis.harmonics {
                    render_harmonic_marker(&painter, plot_rect, *freq, *db, state);
                }
            }
        }

        // Peak markers
        if state.show_peaks {
            let peaks = data.find_peaks(state.peak_threshold_db);
            for (_, peak) in peaks.iter().take(10) {
                render_peak_marker(&painter, plot_rect, peak, state);
            }
        }
    }

    // Axis labels rendered in gutters so they never overlap trace data.
    painter.text(
        Pos2::new(plot_rect.center().x, rect.max.y - 2.0),
        egui::Align2::CENTER_BOTTOM,
        "Frequency",
        FontId::proportional(10.0),
        text_color(),
    );

    painter.text(
        Pos2::new(rect.min.x + 2.0, plot_rect.center().y),
        egui::Align2::LEFT_CENTER,
        state.mag_scale.display_name(),
        FontId::proportional(10.0),
        text_color(),
    );

    // Plot border
    painter.rect_stroke(
        plot_rect,
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
    );
}

fn render_grid(painter: &egui::Painter, rect: Rect, _state: &FftState) {
    let stroke = Stroke::new(0.5, grid_color());

    // Vertical lines (frequency)
    let num_freq = 10;
    for i in 0..=num_freq {
        let t = i as f32 / num_freq as f32;
        let x = rect.min.x + t * rect.width();
        painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], stroke);
    }

    // Horizontal lines (magnitude)
    let num_mag = 10;
    for i in 0..=num_mag {
        let t = i as f32 / num_mag as f32;
        let y = rect.min.y + t * rect.height();
        painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], stroke);
    }
}

fn render_trace(painter: &egui::Painter, rect: Rect, data: &FftData, state: &FftState) {
    if data.is_empty() {
        return;
    }

    let stroke = Stroke::new(1.5, trace_color());

    let points: Vec<Pos2> = data
        .points
        .iter()
        .filter_map(|p| {
            let x = freq_to_x(p.frequency, rect, state);
            let y = mag_to_y(p, rect, state);
            if x.is_finite() && y.is_finite() && x >= rect.min.x && x <= rect.max.x {
                Some(Pos2::new(x, y.clamp(rect.min.y, rect.max.y)))
            } else {
                None
            }
        })
        .collect();

    // Draw as filled area
    if points.len() >= 2 {
        // Draw fill (subtle)
        let mut fill_points = points.clone();
        if let Some(first) = fill_points.first() {
            fill_points.insert(0, Pos2::new(first.x, rect.max.y));
        }
        if let Some(last) = fill_points.last() {
            fill_points.push(Pos2::new(last.x, rect.max.y));
        }

        // Draw line
        for window in points.windows(2) {
            painter.line_segment([window[0], window[1]], stroke);
        }
    }
}

fn render_fundamental_marker(
    painter: &egui::Painter,
    rect: Rect,
    freq: f64,
    _data: &FftData,
    state: &FftState,
) {
    let x = freq_to_x(freq, rect, state);
    if x < rect.min.x || x > rect.max.x {
        return;
    }

    // Vertical line
    painter.line_segment(
        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
        Stroke::new(1.0, fundamental_color()),
    );

    // Label
    painter.text(
        Pos2::new(x, rect.min.y + 5.0),
        egui::Align2::CENTER_TOP,
        "f0",
        FontId::proportional(10.0),
        fundamental_color(),
    );
}

fn render_harmonic_marker(
    painter: &egui::Painter,
    rect: Rect,
    freq: f64,
    _db: f64,
    state: &FftState,
) {
    let x = freq_to_x(freq, rect, state);
    if x < rect.min.x || x > rect.max.x {
        return;
    }

    // Short vertical tick
    painter.line_segment(
        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.min.y + 15.0)],
        Stroke::new(1.0, harmonic_color()),
    );
}

fn render_peak_marker(painter: &egui::Painter, rect: Rect, peak: &FftPoint, state: &FftState) {
    let x = freq_to_x(peak.frequency, rect, state);
    let y = mag_to_y(peak, rect, state);

    if x < rect.min.x || x > rect.max.x {
        return;
    }

    // Small circle at peak
    painter.circle_filled(
        Pos2::new(x, y.clamp(rect.min.y, rect.max.y)),
        3.0,
        peak_color(),
    );
}

fn freq_to_x(freq: f64, rect: Rect, state: &FftState) -> f32 {
    match state.freq_scale {
        FrequencyScale::Linear => {
            let range = state.freq_max - state.freq_min;
            if range <= 0.0 {
                return rect.center().x;
            }
            let t = (freq - state.freq_min) / range;
            rect.min.x + t as f32 * rect.width()
        }
        FrequencyScale::Log => {
            let f_min = state.freq_min.max(1.0);
            let f_max = state.freq_max;
            if f_max <= f_min || freq <= 0.0 {
                return rect.min.x;
            }
            let log_range = f_max.log10() - f_min.log10();
            let t = (freq.log10() - f_min.log10()) / log_range;
            rect.min.x + t as f32 * rect.width()
        }
    }
}

fn mag_to_y(point: &FftPoint, rect: Rect, state: &FftState) -> f32 {
    let value = match state.mag_scale {
        MagnitudeScale::DB => point.magnitude_db(),
        MagnitudeScale::DBm => point.magnitude_dbm(state.z0),
        MagnitudeScale::Linear => point.magnitude,
    };

    let (min, max) = match state.mag_scale {
        MagnitudeScale::Linear => (0.0, 1.0),
        _ => (state.mag_min, state.mag_max),
    };

    let range = max - min;
    if range <= 0.0 {
        return rect.center().y;
    }

    let t = (value - min) / range;
    rect.max.y - t as f32 * rect.height()
}

// =============================================================================
// Info Panel
// =============================================================================

fn render_info_panel(ui: &mut Ui, layout: &FftLayout, state: &FftState) {
    ui.painter()
        .rect_filled(layout.info, Rounding::ZERO, surface_bg_color());

    let panel_rect = layout.info.shrink(8.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Analysis")
                    .size(11.0)
                    .color(text_color()),
            );
            ui.add_space(8.0);

            if let Some(ref analysis) = state.analysis {
                if let Some(fund) = analysis.fundamental_frequency {
                    info_row(ui, "Fund.", &format_freq(fund));
                }

                if let Some(fund_db) = analysis.fundamental_db {
                    info_row(ui, "Level", &format!("{:.1} dB", fund_db));
                }

                ui.add_space(4.0);

                if let Some(thd) = analysis.thd_percent {
                    let color = if thd < 1.0 {
                        Color32::from_rgb(100, 200, 100)
                    } else if thd < 5.0 {
                        Color32::from_rgb(200, 200, 100)
                    } else {
                        Color32::from_rgb(255, 100, 100)
                    };
                    info_row_colored(ui, "THD", &format!("{:.3}%", thd), color);
                }

                if let Some(sfdr) = analysis.sfdr_db {
                    info_row(ui, "SFDR", &format!("{:.1} dB", sfdr));
                }

                if let Some(snr) = analysis.snr_db {
                    info_row(ui, "SNR", &format!("{:.1} dB", snr));
                }

                if let Some(noise) = analysis.noise_floor_db {
                    info_row(ui, "Noise", &format!("{:.1} dB", noise));
                }

                ui.add_space(4.0);
                info_row(ui, "Harmonics", &format!("{}", analysis.harmonics.len()));
            } else {
                ui.label(
                    egui::RichText::new("No data")
                        .size(10.0)
                        .color(Color32::from_rgb(100, 105, 115)),
                );
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // Window info
            ui.label(egui::RichText::new("Window").size(10.0).color(text_color()));
            info_row(ui, "Type", state.window.display_name());
            info_row(
                ui,
                "Sidelobe",
                &format!("{:.0} dB", state.window.sidelobe_level()),
            );
        });
    });
}

fn info_row(ui: &mut Ui, label: &str, value: &str) {
    info_row_colored(ui, label, value, Color32::from_rgb(200, 205, 215));
}

fn info_row_colored(ui: &mut Ui, label: &str, value: &str, color: Color32) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}:", label))
                .size(10.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(value).size(11.0).color(color));
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

fn load_demo_data(state: &mut FftState) {
    // Generate demo signal: 1kHz fundamental + harmonics + noise
    let fs = 44100.0;
    let n = 4096;
    let f0 = 1000.0;

    let data: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64 / fs;
            let fundamental = (2.0 * PI * f0 * t).sin();
            let h2 = 0.05 * (2.0 * PI * 2.0 * f0 * t).sin(); // 5% 2nd
            let h3 = 0.02 * (2.0 * PI * 3.0 * f0 * t).sin(); // 2% 3rd
            let noise = (i as f64 * 12345.6789).sin() * 0.001;
            fundamental + h2 + h3 + noise
        })
        .collect();

    let fft = FftData::from_time_domain("Demo Signal", &data, fs, WindowFunction::Hanning);
    state.load_data(fft);
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

        assert!(layout.spectrum.width() > 0.0);
        assert!(layout.spectrum.height() > 0.0);
        let plot_rect = spectrum_plot_rect(layout.spectrum);
        assert!(plot_rect.min.x > layout.spectrum.min.x);
        assert!(plot_rect.max.y < layout.spectrum.max.y);
    }

    #[test]
    fn test_freq_to_x_linear() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
        let mut state = FftState::new();
        state.freq_min = 0.0;
        state.freq_max = 1000.0;
        state.freq_scale = FrequencyScale::Linear;

        let x = freq_to_x(500.0, rect, &state);
        assert!((x - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_freq_to_x_log() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
        let mut state = FftState::new();
        state.freq_min = 10.0;
        state.freq_max = 10000.0;
        state.freq_scale = FrequencyScale::Log;

        // 100Hz is 1 decade from 10, which is 1/3 of 3 decades
        let x = freq_to_x(100.0, rect, &state);
        assert!((x - 100.0 / 3.0).abs() < 1.0);
    }

    #[test]
    fn test_format_freq() {
        assert!(format_freq(1000.0).contains("kHz"));
        assert!(format_freq(1e6).contains("MHz"));
        assert!(format_freq(1e9).contains("GHz"));
    }

    #[test]
    fn test_load_demo_data() {
        let mut state = FftState::new();
        load_demo_data(&mut state);

        assert!(state.has_data());
        assert!(state.analysis.is_some());
    }
}
