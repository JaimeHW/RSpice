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
    let layout = calculate_layout(available_rect);
    let source_names = collect_fft_source_names(app_state);

    let header_actions = {
        let state = &mut app_state.fft_state;
        render_header(ui, &layout, state, &source_names)
    };

    if let Some(source_name) = header_actions.select_source {
        refresh_fft_from_source_waveform(app_state, &source_name);
    }

    let state = &mut app_state.fft_state;
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

const HEADER_HEIGHT: f32 = 62.0;
const INFO_WIDTH: f32 = 150.0;
const CHART_SIDE_PADDING: f32 = 8.0;
const CHART_TOP_GAP: f32 = 0.0;
const CHART_BOTTOM_PADDING: f32 = 8.0;
const HEADER_CONTROL_HEIGHT: f32 = 24.0;
const HEADER_SECOND_ROW_HEIGHT: f32 = 22.0;
const HEADER_SOURCE_WIDTH: f32 = 180.0;
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

#[derive(Debug, Default)]
struct HeaderActions {
    select_source: Option<String>,
}

fn collect_fft_source_names(app_state: &AppState) -> Vec<String> {
    let mut names: Vec<String> = app_state
        .simulation
        .waveforms
        .iter()
        .map(|wf| wf.name.clone())
        .collect();
    names.sort();
    names.dedup();
    names
}

fn refresh_fft_from_source_waveform(app_state: &mut AppState, source_name: &str) {
    app_state
        .fft_state
        .set_selected_source(Some(source_name.to_string()));
    let Some(waveform) = app_state
        .simulation
        .waveforms
        .iter()
        .find(|wf| wf.name == source_name)
    else {
        app_state.fft_state.clear();
        return;
    };

    if let Some(prepared) = crate::analysis::fft::prepare_fft_input(
        &waveform.name,
        &waveform.x,
        &waveform.y,
        crate::analysis::fft::DEFAULT_MAX_FFT_POINTS,
    ) {
        app_state.fft_state.load_prepared_input(prepared);
    } else {
        app_state.fft_state.clear();
    }
}

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(
    ui: &mut Ui,
    layout: &FftLayout,
    state: &mut FftState,
    source_names: &[String],
) -> HeaderActions {
    let mut actions = HeaderActions::default();
    ui.painter()
        .rect_filled(layout.header, Rounding::ZERO, header_bg_color());

    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 4.0);

        ui.horizontal(|ui| {
            ui.spacing_mut().interact_size.y = HEADER_CONTROL_HEIGHT;
            ui.spacing_mut().button_padding.y = 2.0;
            ui.add_space(6.0);

            ui.label(
                egui::RichText::new("FFT Spectrum")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(8.0);

            let mut selected_source = state
                .selected_source
                .clone()
                .or_else(|| state.source_cache.as_ref().map(|src| src.name.clone()));
            egui::ComboBox::from_id_salt("fft_source")
                .width(HEADER_SOURCE_WIDTH)
                .selected_text(
                    selected_source
                        .as_deref()
                        .unwrap_or("Source waveform")
                        .to_string(),
                )
                .show_ui(ui, |ui| {
                    for name in source_names {
                        ui.selectable_value(&mut selected_source, Some(name.clone()), name);
                    }
                });
            if selected_source != state.selected_source {
                state.set_selected_source(selected_source.clone());
                actions.select_source = selected_source;
            }

            let mut window = state.window;
            egui::ComboBox::from_id_salt("fft_window")
                .width(HEADER_WINDOW_WIDTH)
                .selected_text(window.display_name())
                .show_ui(ui, |ui| {
                    for w in WindowFunction::all() {
                        ui.selectable_value(&mut window, *w, w.display_name());
                    }
                });
            if window != state.window {
                state.set_window(window);
            }

            let mut mag_scale = state.mag_scale;
            egui::ComboBox::from_id_salt("fft_mag_scale")
                .width(HEADER_SCALE_WIDTH)
                .selected_text(mag_scale.display_name())
                .show_ui(ui, |ui| {
                    for s in MagnitudeScale::all() {
                        ui.selectable_value(&mut mag_scale, *s, s.display_name());
                    }
                });
            if mag_scale != state.mag_scale {
                state.set_mag_scale(mag_scale);
            }

            let mut freq_scale = state.freq_scale;
            egui::ComboBox::from_id_salt("fft_freq_scale")
                .width(HEADER_SCALE_WIDTH)
                .selected_text(freq_scale.display_name())
                .show_ui(ui, |ui| {
                    for s in FrequencyScale::all() {
                        ui.selectable_value(&mut freq_scale, *s, s.display_name());
                    }
                });
            if freq_scale != state.freq_scale {
                state.set_freq_scale(freq_scale);
            }
        });

        ui.horizontal(|ui| {
            ui.spacing_mut().interact_size.y = HEADER_SECOND_ROW_HEIGHT;
            ui.add_space(6.0);

            let peaks_label = if state.show_peaks {
                "Peaks [on]"
            } else {
                "Peaks"
            };
            if ui.button(peaks_label).clicked() {
                state.toggle_peaks();
            }

            let harmonics_label = if state.show_harmonics {
                "Harm [on]"
            } else {
                "Harm"
            };
            if ui.button(harmonics_label).clicked() {
                state.toggle_harmonics();
            }

            if ui.checkbox(&mut state.freq_auto, "Auto Freq").changed() && state.freq_auto {
                state.update_auto_scale();
            }
            if ui.checkbox(&mut state.mag_auto, "Auto Mag").changed() && state.mag_auto {
                state.update_auto_scale();
            }

            ui.add(
                egui::Slider::new(&mut state.peak_threshold_db, -180.0..=20.0)
                    .text("Peak Th (dB)")
                    .fixed_decimals(0),
            );

            let mut harmonics = state.num_harmonics as u32;
            if ui
                .add(egui::Slider::new(&mut harmonics, 1..=64).text("Harmonics"))
                .changed()
            {
                state.set_num_harmonics(harmonics as usize);
            }

            if !state.freq_auto {
                ui.label("fmin");
                ui.add(egui::DragValue::new(&mut state.freq_min).speed(10.0));
                ui.label("fmax");
                ui.add(egui::DragValue::new(&mut state.freq_max).speed(10.0));
                if state.freq_scale == FrequencyScale::Log {
                    state.freq_min = state.freq_min.max(1e-12);
                }
                if state.freq_max <= state.freq_min {
                    state.freq_max = state.freq_min * 1.01;
                }
            }

            if !state.mag_auto {
                ui.label("mmin");
                ui.add(egui::DragValue::new(&mut state.mag_min).speed(0.5));
                ui.label("mmax");
                ui.add(egui::DragValue::new(&mut state.mag_max).speed(0.5));
                if state.mag_max <= state.mag_min {
                    state.mag_max = state.mag_min + 1.0;
                }
            }
        });
    });

    actions
}

// =============================================================================
// Spectrum Rendering
// =============================================================================

fn render_spectrum(ui: &mut Ui, layout: &FftLayout, state: &mut FftState) {
    let response = ui.allocate_rect(layout.spectrum, Sense::click_and_drag());
    render_spectrum_core(ui, layout, state);
    handle_spectrum_interactions(ui, response, spectrum_plot_rect(layout.spectrum), state);
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
        render_grid(&painter, rect, plot_rect, state);
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

        if let Some(marker_freq) = state.marker_frequency {
            render_user_marker(&painter, rect, plot_rect, marker_freq, data, state);
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

#[derive(Debug, Clone)]
struct AxisTick {
    value: f64,
    label: String,
    major: bool,
}

fn render_grid(painter: &egui::Painter, spectrum_rect: Rect, plot_rect: Rect, state: &FftState) {
    let freq_ticks = frequency_ticks(state, 10);
    let mag_ticks = magnitude_ticks(state, 8);

    for tick in &freq_ticks {
        let x = freq_to_x(tick.value, plot_rect, state);
        if !x.is_finite() || x < plot_rect.min.x || x > plot_rect.max.x {
            continue;
        }
        let stroke = if tick.major {
            Stroke::new(0.7, grid_color())
        } else {
            Stroke::new(0.4, Color32::from_rgb(34, 38, 46))
        };
        painter.line_segment(
            [Pos2::new(x, plot_rect.min.y), Pos2::new(x, plot_rect.max.y)],
            stroke,
        );
        if tick.major {
            painter.text(
                Pos2::new(x, spectrum_rect.max.y - 4.0),
                egui::Align2::CENTER_BOTTOM,
                &tick.label,
                FontId::proportional(9.0),
                text_color(),
            );
        }
    }

    for tick in &mag_ticks {
        let point = FftPoint::new(0.0, magnitude_to_linear(tick.value, state), 0.0);
        let y = mag_to_y(&point, plot_rect, state);
        if !y.is_finite() || y < plot_rect.min.y || y > plot_rect.max.y {
            continue;
        }
        let stroke = if tick.major {
            Stroke::new(0.7, grid_color())
        } else {
            Stroke::new(0.4, Color32::from_rgb(34, 38, 46))
        };
        painter.line_segment(
            [Pos2::new(plot_rect.min.x, y), Pos2::new(plot_rect.max.x, y)],
            stroke,
        );
        if tick.major {
            painter.text(
                Pos2::new(spectrum_rect.min.x + 2.0, y),
                egui::Align2::LEFT_CENTER,
                &tick.label,
                FontId::proportional(9.0),
                text_color(),
            );
        }
    }
}

fn frequency_ticks(state: &FftState, approx: usize) -> Vec<AxisTick> {
    match state.freq_scale {
        FrequencyScale::Linear => linear_ticks(state.freq_min, state.freq_max, approx, |v| {
            format_freq_tick(v)
        }),
        FrequencyScale::Log => log_ticks(state.freq_min.max(1e-12), state.freq_max),
    }
}

fn magnitude_ticks(state: &FftState, approx: usize) -> Vec<AxisTick> {
    linear_ticks(state.mag_min, state.mag_max, approx, |v| {
        match state.mag_scale {
            MagnitudeScale::Linear => {
                let value = v.max(0.0);
                if value >= 1e3 || (value > 0.0 && value < 1e-3) {
                    format!("{:.2e}", value)
                } else {
                    format!("{:.3}", value)
                }
            }
            _ => format!("{:.0}", v),
        }
    })
}

fn linear_ticks<F>(min: f64, max: f64, approx: usize, mut labeler: F) -> Vec<AxisTick>
where
    F: FnMut(f64) -> String,
{
    if !min.is_finite() || !max.is_finite() || max <= min {
        return Vec::new();
    }
    let target = approx.max(2) as f64;
    let raw_step = (max - min) / target;
    let step = nice_step(raw_step);
    if !step.is_finite() || step <= 0.0 {
        return Vec::new();
    }

    let mut ticks = Vec::new();
    let start = (min / step).floor() as i64 - 1;
    let end = (max / step).ceil() as i64 + 1;
    for idx in start..=end {
        let value = idx as f64 * step;
        if value < min - step * 0.25 || value > max + step * 0.25 {
            continue;
        }
        ticks.push(AxisTick {
            value,
            label: labeler(value),
            major: true,
        });
    }
    ticks
}

fn log_ticks(min: f64, max: f64) -> Vec<AxisTick> {
    if !min.is_finite() || !max.is_finite() || max <= min || min <= 0.0 {
        return Vec::new();
    }
    let min_dec = min.log10().floor() as i32;
    let max_dec = max.log10().ceil() as i32;
    let mut ticks = Vec::new();
    for dec in min_dec..=max_dec {
        let decade = 10.0_f64.powi(dec);
        for (mult, major) in [(1.0, true), (2.0, false), (5.0, false)] {
            let value = decade * mult;
            if value < min || value > max {
                continue;
            }
            ticks.push(AxisTick {
                value,
                label: if major {
                    format_freq_tick(value)
                } else {
                    String::new()
                },
                major,
            });
        }
    }
    ticks
}

fn nice_step(raw_step: f64) -> f64 {
    if !raw_step.is_finite() || raw_step <= 0.0 {
        return 1.0;
    }
    let exponent = raw_step.log10().floor();
    let base = 10.0_f64.powf(exponent);
    let fraction = raw_step / base;
    let nice_fraction = if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice_fraction * base
}

fn format_freq_tick(freq: f64) -> String {
    if freq >= 1e9 {
        format!("{:.2}G", freq / 1e9)
    } else if freq >= 1e6 {
        format!("{:.2}M", freq / 1e6)
    } else if freq >= 1e3 {
        format!("{:.2}k", freq / 1e3)
    } else if freq >= 1.0 {
        format!("{:.2}", freq)
    } else if freq > 0.0 {
        format!("{:.2e}", freq)
    } else {
        "0".to_string()
    }
}

fn magnitude_to_linear(value: f64, state: &FftState) -> f64 {
    match state.mag_scale {
        MagnitudeScale::Linear => value,
        MagnitudeScale::DB => 10.0_f64.powf(value / 20.0),
        MagnitudeScale::DBm => {
            let power_w = 1e-3 * 10.0_f64.powf(value / 10.0);
            (power_w * state.z0.max(1e-9)).sqrt()
        }
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

fn render_user_marker(
    painter: &egui::Painter,
    spectrum_rect: Rect,
    plot_rect: Rect,
    marker_freq: f64,
    data: &FftData,
    state: &FftState,
) {
    let x = freq_to_x(marker_freq, plot_rect, state);
    if !x.is_finite() || x < plot_rect.min.x || x > plot_rect.max.x {
        return;
    }
    painter.line_segment(
        [Pos2::new(x, plot_rect.min.y), Pos2::new(x, plot_rect.max.y)],
        Stroke::new(1.0, Color32::from_rgb(220, 220, 120)),
    );

    if let Some(point) = data.interpolate(marker_freq) {
        let label = match state.mag_scale {
            MagnitudeScale::Linear => {
                format!("M: {} | {:.4}", format_freq(marker_freq), point.magnitude)
            }
            MagnitudeScale::DB => format!(
                "M: {} | {:.2} dB",
                format_freq(marker_freq),
                point.magnitude_db()
            ),
            MagnitudeScale::DBm => {
                format!(
                    "M: {} | {:.2} dBm",
                    format_freq(marker_freq),
                    point.magnitude_dbm(state.z0)
                )
            }
        };
        painter.text(
            Pos2::new(x, spectrum_rect.min.y + 2.0),
            egui::Align2::CENTER_TOP,
            label,
            FontId::proportional(9.0),
            Color32::from_rgb(220, 220, 120),
        );
    }
}

fn handle_spectrum_interactions(
    ui: &Ui,
    response: egui::Response,
    plot_rect: Rect,
    state: &mut FftState,
) {
    if response.double_clicked() {
        state.freq_auto = true;
        state.mag_auto = true;
        state.marker_frequency = None;
        state.update_auto_scale();
        return;
    }

    if response.clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            if plot_rect.contains(pos) {
                let freq = x_to_freq(pos.x, plot_rect, state);
                if freq.is_finite() {
                    state.set_marker_frequency(Some(freq));
                }
            }
        }
    }

    if response.hovered() {
        let scroll_y = ui.input(|i| i.raw_scroll_delta.y);
        if scroll_y.abs() > f32::EPSILON {
            state.freq_auto = false;
            state.mag_auto = false;

            let zoom = (1.0f64 - (scroll_y as f64) * 0.0015).clamp(0.5, 1.5);
            let pointer = response
                .hover_pos()
                .unwrap_or(Pos2::new(plot_rect.center().x, plot_rect.center().y));
            let x_frac = ((pointer.x - plot_rect.min.x) / plot_rect.width()).clamp(0.0, 1.0) as f64;
            let y_frac =
                ((pointer.y - plot_rect.min.y) / plot_rect.height()).clamp(0.0, 1.0) as f64;

            zoom_frequency_range(state, zoom, x_frac);
            zoom_magnitude_range(state, zoom, y_frac);
        }
    }

    if response.dragged() {
        let delta = ui.input(|i| i.pointer.delta());
        if delta.length_sq() > 0.0 {
            state.freq_auto = false;
            state.mag_auto = false;

            pan_frequency_range(state, delta.x as f64, plot_rect.width() as f64);
            pan_magnitude_range(state, delta.y as f64, plot_rect.height() as f64);
        }
    }
}

fn zoom_frequency_range(state: &mut FftState, factor: f64, center_frac: f64) {
    match state.freq_scale {
        FrequencyScale::Linear => {
            let range = state.freq_max - state.freq_min;
            if !range.is_finite() || range <= 0.0 {
                return;
            }
            let center = state.freq_min + center_frac * range;
            let new_range = (range * factor).max(1e-12);
            state.freq_min = center - center_frac * new_range;
            state.freq_max = state.freq_min + new_range;
        }
        FrequencyScale::Log => {
            let min = state.freq_min.max(1e-12);
            let max = state.freq_max.max(min * 1.000_001);
            let log_min = min.log10();
            let log_max = max.log10();
            let log_range = log_max - log_min;
            if !log_range.is_finite() || log_range <= 0.0 {
                return;
            }
            let center = log_min + center_frac * log_range;
            let new_range = (log_range * factor).max(1e-9);
            let new_min = center - center_frac * new_range;
            let new_max = new_min + new_range;
            state.freq_min = 10.0_f64.powf(new_min);
            state.freq_max = 10.0_f64.powf(new_max);
        }
    }
    if state.freq_max <= state.freq_min {
        state.freq_max = state.freq_min * 1.01;
    }
}

fn zoom_magnitude_range(state: &mut FftState, factor: f64, y_frac_from_top: f64) {
    let range = state.mag_max - state.mag_min;
    if !range.is_finite() || range <= 0.0 {
        return;
    }
    let center = state.mag_max - y_frac_from_top * range;
    let below_frac = 1.0 - y_frac_from_top;
    let above_frac = y_frac_from_top;
    let new_range = (range * factor).max(1e-12);

    state.mag_min = center - below_frac * new_range;
    state.mag_max = center + above_frac * new_range;
    if state.mag_scale == MagnitudeScale::Linear {
        state.mag_min = state.mag_min.max(0.0);
    }
    if state.mag_max <= state.mag_min {
        state.mag_max = state.mag_min + 1.0;
    }
}

fn pan_frequency_range(state: &mut FftState, delta_x_pixels: f64, width_pixels: f64) {
    if width_pixels <= 0.0 {
        return;
    }
    match state.freq_scale {
        FrequencyScale::Linear => {
            let range = state.freq_max - state.freq_min;
            if !range.is_finite() || range <= 0.0 {
                return;
            }
            let shift = -delta_x_pixels / width_pixels * range;
            state.freq_min += shift;
            state.freq_max += shift;
        }
        FrequencyScale::Log => {
            let min = state.freq_min.max(1e-12);
            let max = state.freq_max.max(min * 1.000_001);
            let log_range = max.log10() - min.log10();
            if !log_range.is_finite() || log_range <= 0.0 {
                return;
            }
            let shift_log = -delta_x_pixels / width_pixels * log_range;
            let ratio = 10.0_f64.powf(shift_log);
            state.freq_min = min * ratio;
            state.freq_max = max * ratio;
        }
    }
}

fn pan_magnitude_range(state: &mut FftState, delta_y_pixels: f64, height_pixels: f64) {
    if height_pixels <= 0.0 {
        return;
    }
    let range = state.mag_max - state.mag_min;
    if !range.is_finite() || range <= 0.0 {
        return;
    }
    let shift = delta_y_pixels / height_pixels * range;
    state.mag_min += shift;
    state.mag_max += shift;
    if state.mag_scale == MagnitudeScale::Linear && state.mag_min < 0.0 {
        let adjust = -state.mag_min;
        state.mag_min += adjust;
        state.mag_max += adjust;
    }
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

fn x_to_freq(x: f32, rect: Rect, state: &FftState) -> f64 {
    let t = ((x - rect.min.x) / rect.width()).clamp(0.0, 1.0) as f64;
    match state.freq_scale {
        FrequencyScale::Linear => state.freq_min + t * (state.freq_max - state.freq_min),
        FrequencyScale::Log => {
            let min = state.freq_min.max(1e-12);
            let max = state.freq_max.max(min * 1.000_001);
            let log_val = min.log10() + t * (max.log10() - min.log10());
            10.0_f64.powf(log_val)
        }
    }
}

fn mag_to_y(point: &FftPoint, rect: Rect, state: &FftState) -> f32 {
    let value = match state.mag_scale {
        MagnitudeScale::DB => point.magnitude_db(),
        MagnitudeScale::DBm => point.magnitude_dbm(state.z0),
        MagnitudeScale::Linear => point.magnitude,
    };

    let (min, max) = (state.mag_min, state.mag_max);

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

                if let Some(sinad) = analysis.sinad_db {
                    info_row(ui, "SINAD", &format!("{:.1} dB", sinad));
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
            info_row(
                ui,
                "ENBW",
                &format!("{:.2} bins", state.window.noise_bandwidth()),
            );

            if let Some(ref source) = state.source_cache {
                ui.add_space(6.0);
                ui.label(egui::RichText::new("Source").size(10.0).color(text_color()));
                info_row(ui, "Trace", &source.name);
                info_row(ui, "Input N", &format!("{}", source.original_count));
                info_row(ui, "Samples", &format!("{}", source.samples.len()));
                if source.decimation_factor > 1 {
                    info_row(ui, "Decim", &format!("x{}", source.decimation_factor));
                }
                info_row(ui, "Fs", &format_freq(source.sample_rate));
            }

            if let Some(marker_freq) = state.marker_frequency {
                ui.add_space(6.0);
                ui.label(egui::RichText::new("Marker").size(10.0).color(text_color()));
                info_row(ui, "Freq", &format_freq(marker_freq));
                if let Some(ref data) = state.data {
                    if let Some(point) = data.interpolate(marker_freq) {
                        match state.mag_scale {
                            MagnitudeScale::Linear => {
                                info_row(ui, "Mag", &format!("{:.5}", point.magnitude))
                            }
                            MagnitudeScale::DB => {
                                info_row(ui, "Mag", &format!("{:.2} dB", point.magnitude_db()))
                            }
                            MagnitudeScale::DBm => info_row(
                                ui,
                                "Mag",
                                &format!("{:.2} dBm", point.magnitude_dbm(state.z0)),
                            ),
                        }
                    }
                }
            }
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
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();

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

    if let Some(prepared) = crate::analysis::fft::prepare_fft_input(
        "Demo Signal",
        &time,
        &data,
        crate::analysis::fft::DEFAULT_MAX_FFT_POINTS,
    ) {
        state.load_prepared_input(prepared);
    }
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
        assert!(state.source_cache.is_some());
    }

    #[test]
    fn test_x_to_freq_linear_inverse() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(200.0, 100.0));
        let mut state = FftState::new();
        state.freq_scale = FrequencyScale::Linear;
        state.freq_min = 10.0;
        state.freq_max = 1010.0;

        let f = 610.0;
        let x = freq_to_x(f, rect, &state);
        let back = x_to_freq(x, rect, &state);
        assert!((back - f).abs() < 1e-3);
    }

    #[test]
    fn test_x_to_freq_log_inverse() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(300.0, 100.0));
        let mut state = FftState::new();
        state.freq_scale = FrequencyScale::Log;
        state.freq_min = 10.0;
        state.freq_max = 10_000.0;

        let f = 500.0;
        let x = freq_to_x(f, rect, &state);
        let back = x_to_freq(x, rect, &state);
        assert!((back - f).abs() / f < 1e-6);
    }

    #[test]
    fn test_frequency_ticks_log_has_major_decades() {
        let mut state = FftState::new();
        state.freq_scale = FrequencyScale::Log;
        state.freq_min = 10.0;
        state.freq_max = 1_000_000.0;

        let ticks = frequency_ticks(&state, 10);
        assert!(ticks
            .iter()
            .any(|t| t.major && (t.value - 10.0).abs() < 1e-9));
        assert!(ticks
            .iter()
            .any(|t| t.major && (t.value - 1000.0).abs() < 1e-9));
        assert!(ticks
            .iter()
            .any(|t| t.major && (t.value - 100000.0).abs() < 1e-9));
    }

    #[test]
    fn test_magnitude_to_linear_dbm_conversion() {
        let mut state = FftState::new();
        state.mag_scale = MagnitudeScale::DBm;
        state.z0 = 50.0;
        // 13.0103 dBm ~= 1 Vrms into 50 ohm
        let v = magnitude_to_linear(13.0103, &state);
        assert!((v - 1.0).abs() < 1e-2);
    }
}
