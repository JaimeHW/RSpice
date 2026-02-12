//! FFT Viewer Rendering
//!
//! Commercial-grade egui rendering for FFT/spectrum visualization.

use egui::{Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2};
use std::f64::consts::PI;

use super::data::{FftData, FftPoint, SpectrumAnalysis, SpectrumNormalization};
use super::state::{FftState, FrequencyScale, InputFidelity, MagnitudeScale, MarkerSlot};
use super::window::WindowFunction;
use crate::common::app::AppState;
use crate::common::viewer_style::{viewer_chart_bg_color, viewer_header_bg_color};
use crate::state::AnalysisType;
use crate::utils::vertical_label_layout::{
    place_vertical_line_labels, LabelSide, VerticalLabelLayoutConfig, VerticalLabelPlacement,
    VerticalLabelRequest,
};

// =============================================================================
// Constants
// =============================================================================

fn chart_bg_color() -> Color32 {
    viewer_chart_bg_color()
}

fn surface_bg_color() -> Color32 {
    viewer_header_bg_color()
}

fn header_bg_color() -> Color32 {
    viewer_header_bg_color()
}

fn grid_major_color() -> Color32 {
    Color32::from_rgb(50, 52, 58)
}

fn grid_minor_color() -> Color32 {
    Color32::from_rgb(35, 37, 42)
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

fn marker_primary_color() -> Color32 {
    Color32::from_rgb(220, 220, 120)
}

fn marker_secondary_color() -> Color32 {
    Color32::from_rgb(255, 175, 95)
}

fn text_color() -> Color32 {
    Color32::from_rgb(180, 185, 195)
}

// =============================================================================
// Main Rendering Entry Point
// =============================================================================

/// Render the FFT viewer panel
pub fn render_fft_viewer(ui: &mut Ui, app_state: &mut AppState) {
    if !fft_supported_for_active_analysis(app_state) {
        app_state.fft_state.clear();
    }

    let available_rect = ui.available_rect_before_wrap();
    // Claim full available space so the parent resizable panel keeps user height
    // instead of collapsing to a content-driven "natural" size.
    let (_id, _rect) = ui.allocate_space(available_rect.size());
    let layout = calculate_layout(available_rect);
    let source_names = collect_fft_source_names(app_state);
    let source_time_bounds = current_fft_source_time_bounds(app_state);

    let header_actions = {
        let state = &mut app_state.fft_state;
        let mut actions = HeaderActions::default();
        actions.merge(render_time_controls_header(
            ui,
            &layout,
            state,
            source_time_bounds,
        ));
        actions.merge(render_header(ui, &layout, state, &source_names));
        actions
    };

    if let Some(source_name) = header_actions.refresh_source {
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
    header_top: Rect,
    header_main: Rect,
    spectrum: Rect,
    info: Rect,
}

const HEADER_ROW_HEIGHT: f32 = 34.0;
const HEADER_TOP_HEIGHT: f32 = HEADER_ROW_HEIGHT;
const HEADER_MAIN_HEIGHT: f32 = HEADER_ROW_HEIGHT;
const INFO_WIDTH: f32 = 150.0;
const CHART_SIDE_PADDING: f32 = 8.0;
const CHART_TOP_GAP: f32 = 0.0;
const CHART_BOTTOM_PADDING: f32 = 8.0;
const HEADER_CONTROL_HEIGHT: f32 = 24.0;
const HEADER_DROPDOWN_MIN_WIDTH: f32 = 82.0;
const HEADER_DROPDOWN_MAX_WIDTH: f32 = 220.0;
const HEADER_DROPDOWN_TEXT_PADDING: f32 = 28.0;
const INFO_PANEL_PADDING: f32 = 8.0;
const AXIS_LEFT_GUTTER: f32 = 52.0;
const AXIS_RIGHT_GUTTER: f32 = 4.0;
const AXIS_TOP_GUTTER: f32 = 2.0;
const AXIS_BOTTOM_GUTTER: f32 = 30.0;
const AXIS_TITLE_MIN_LEFT_INSET: f32 = 2.0;
const AXIS_TITLE_TO_VALUE_LABEL_GAP: f32 = 6.0;
const AXIS_TITLE_BOTTOM_INSET: f32 = 2.0;
const AXIS_TICK_X_OFFSET: f32 = 2.0;
const AXIS_TICK_Y_OFFSET: f32 = 2.0;
const MAX_LINEAR_MAJOR_TICKS: usize = 50;
const MAX_LINEAR_MINOR_TICKS: usize = 250;
const LINEAR_MINOR_SUBDIVISIONS: usize = 5;
const CURSOR_LABEL_TEXT_PADDING_X: f32 = 5.0;
const CURSOR_LABEL_TEXT_PADDING_Y: f32 = 2.0;
const CURSOR_LABEL_CORNER_RADIUS: f32 = 3.0;
const CURSOR_LABEL_LINE_STROKE: f32 = 1.0;
const CURSOR_LABEL_FONT_SIZE: f32 = 9.0;
const CURSOR_LABEL_BG_ALPHA: u8 = 220;

fn calculate_layout(available: Rect) -> FftLayout {
    let total = available;
    let total_header_height = (HEADER_TOP_HEIGHT + HEADER_MAIN_HEIGHT).min(total.height());
    let top_height = HEADER_TOP_HEIGHT.min(total_header_height);
    let main_height = (total_header_height - top_height).max(0.0);

    let header_top = Rect::from_min_size(total.min, Vec2::new(total.width(), top_height));
    let header_main = Rect::from_min_size(
        Pos2::new(total.min.x, header_top.max.y),
        Vec2::new(total.width(), main_height),
    );
    let content_top = header_main.max.y;

    let info = Rect::from_min_size(
        Pos2::new(total.max.x - INFO_WIDTH, content_top),
        Vec2::new(INFO_WIDTH, (total.max.y - content_top).max(0.0)),
    );

    let spectrum = Rect::from_min_max(
        Pos2::new(
            total.min.x + CHART_SIDE_PADDING,
            content_top + CHART_TOP_GAP,
        ),
        Pos2::new(
            info.min.x - CHART_SIDE_PADDING,
            total.max.y - CHART_BOTTOM_PADDING,
        ),
    );

    FftLayout {
        total,
        header_top,
        header_main,
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

fn x_axis_title_position(spectrum_rect: Rect, plot_rect: Rect) -> Pos2 {
    Pos2::new(
        plot_rect.center().x,
        spectrum_rect.max.y - AXIS_TITLE_BOTTOM_INSET,
    )
}

fn y_axis_title_position(
    spectrum_rect: Rect,
    plot_rect: Rect,
    max_y_tick_label_width: f32,
    y_title_width: f32,
) -> Pos2 {
    let y_tick_anchor_x = y_tick_label_position(plot_rect.center().y, plot_rect).x;
    let y_tick_left_edge = y_tick_anchor_x - max_y_tick_label_width.max(0.0);
    let title_left = (y_tick_left_edge - AXIS_TITLE_TO_VALUE_LABEL_GAP - y_title_width)
        .max(spectrum_rect.min.x + AXIS_TITLE_MIN_LEFT_INSET);
    Pos2::new(title_left, plot_rect.center().y)
}

fn info_content_rect(layout: &FftLayout) -> Rect {
    // The visual right-side lane is bounded by the spectrum edge and window edge.
    // Center content inside that lane so it does not appear right-shifted.
    let lane = Rect::from_min_max(
        Pos2::new(layout.spectrum.max.x, layout.info.min.y),
        layout.info.max,
    );
    lane.shrink(INFO_PANEL_PADDING)
}

fn x_tick_label_position(x: f32, plot_rect: Rect) -> Pos2 {
    Pos2::new(x, plot_rect.max.y + AXIS_TICK_Y_OFFSET)
}

fn y_tick_label_position(y: f32, plot_rect: Rect) -> Pos2 {
    Pos2::new(plot_rect.min.x - AXIS_TICK_X_OFFSET, y)
}

fn measure_text_width(painter: &egui::Painter, text: &str, font: FontId, color: Color32) -> f32 {
    painter
        .layout_no_wrap(text.to_owned(), font, color)
        .size()
        .x
}

fn measure_text_size(painter: &egui::Painter, text: &str, font: FontId, color: Color32) -> Vec2 {
    painter.layout_no_wrap(text.to_owned(), font, color).size()
}

fn combo_width_from_texts<'a, I>(
    ui: &Ui,
    selected_text: &str,
    options: I,
    min_width: f32,
    max_width: f32,
) -> f32
where
    I: IntoIterator<Item = &'a str>,
{
    let painter = ui.painter();
    let font = FontId::proportional(12.0);
    let color = text_color();

    let mut max_text_width = measure_text_width(painter, selected_text, font.clone(), color);
    for option in options {
        let width = measure_text_width(painter, option, font.clone(), color);
        max_text_width = max_text_width.max(width);
    }

    (max_text_width + HEADER_DROPDOWN_TEXT_PADDING).clamp(min_width, max_width)
}

#[derive(Debug, Default)]
struct HeaderActions {
    refresh_source: Option<String>,
}

impl HeaderActions {
    fn merge(&mut self, other: HeaderActions) {
        if other.refresh_source.is_some() {
            self.refresh_source = other.refresh_source;
        }
    }
}

fn queue_fft_refresh(actions: &mut HeaderActions, state: &FftState) {
    if actions.refresh_source.is_none() {
        actions.refresh_source = state
            .selected_source
            .clone()
            .or_else(|| state.source_cache.as_ref().map(|src| src.name.clone()));
    }
}

fn collect_fft_source_names(app_state: &AppState) -> Vec<String> {
    if !fft_supported_for_active_analysis(app_state) {
        return Vec::new();
    }

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

fn current_fft_source_time_bounds(app_state: &AppState) -> Option<(f64, f64)> {
    let selected = app_state.fft_state.selected_source.as_ref().or_else(|| {
        app_state
            .fft_state
            .source_cache
            .as_ref()
            .map(|src| &src.name)
    })?;
    let waveform = app_state
        .simulation
        .waveforms
        .iter()
        .find(|wf| wf.name == *selected)?;
    waveform_time_bounds(waveform)
}

fn waveform_time_bounds(waveform: &crate::state::WaveformData) -> Option<(f64, f64)> {
    let start = waveform.x.iter().copied().find(|x| x.is_finite())?;
    let end = waveform.x.iter().copied().rfind(|x| x.is_finite())?;
    if end > start {
        Some((start, end))
    } else {
        None
    }
}

fn fft_supported_for_active_analysis(app_state: &AppState) -> bool {
    matches!(
        app_state
            .simulation
            .active_analysis()
            .map(|analysis| analysis.analysis_type),
        Some(
            AnalysisType::Transient
                | AnalysisType::Pss
                | AnalysisType::Envelope
                | AnalysisType::Soa
        )
    )
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

    let input_options = app_state.fft_state.input_options_for_waveform(&waveform.x);
    if let Some(prepared) = crate::analysis::fft::prepare_fft_input_with_options(
        source_name,
        &waveform.x,
        &waveform.y,
        input_options,
    ) {
        app_state.fft_state.load_prepared_input(prepared);
    } else {
        app_state.fft_state.clear();
    }
}

// =============================================================================
// Header Rendering
// =============================================================================

fn sync_manual_fft_time_window(state: &mut FftState, source_time_bounds: Option<(f64, f64)>) {
    let Some((min_t, max_t)) = source_time_bounds else {
        return;
    };

    if state.time_window_auto {
        state.time_window_start = min_t;
        state.time_window_end = max_t;
        return;
    }

    state.time_window_start = state.time_window_start.clamp(min_t, max_t);
    state.time_window_end = state.time_window_end.clamp(min_t, max_t);
    if state.time_window_end <= state.time_window_start {
        state.time_window_start = min_t;
        state.time_window_end = max_t;
    }
}

fn render_time_controls_header(
    ui: &mut Ui,
    layout: &FftLayout,
    state: &mut FftState,
    source_time_bounds: Option<(f64, f64)>,
) -> HeaderActions {
    let mut actions = HeaderActions::default();
    ui.painter()
        .rect_filled(layout.header_top, Rounding::ZERO, header_bg_color());

    sync_manual_fft_time_window(state, source_time_bounds);

    let header_rect = layout.header_top.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 4.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().interact_size.y = HEADER_CONTROL_HEIGHT;
                ui.spacing_mut().button_padding.y = 2.0;
                ui.add_space(4.0);

                ui.label("Auto Time");
                let mut time_changed = ui.checkbox(&mut state.time_window_auto, "").changed();
                let time_speed = source_time_bounds
                    .map(|(min_t, max_t)| ((max_t - min_t).abs() / 1000.0).max(1e-15))
                    .unwrap_or(1e-9);
                ui.add_enabled_ui(!state.time_window_auto, |ui| {
                    ui.label("Start");
                    if ui
                        .add(
                            egui::DragValue::new(&mut state.time_window_start)
                                .speed(time_speed)
                                .max_decimals(12),
                        )
                        .changed()
                    {
                        time_changed = true;
                    }
                    ui.label("End");
                    if ui
                        .add(
                            egui::DragValue::new(&mut state.time_window_end)
                                .speed(time_speed)
                                .max_decimals(12),
                        )
                        .changed()
                    {
                        time_changed = true;
                    }
                });
                sync_manual_fft_time_window(state, source_time_bounds);
                if time_changed {
                    queue_fft_refresh(&mut actions, state);
                }

                ui.separator();

                ui.label("Auto N");
                let mut sample_changed = ui.checkbox(&mut state.sample_count_auto, "").changed();
                ui.add_enabled_ui(!state.sample_count_auto, |ui| {
                    ui.label("N");
                    let mut sample_count = state.sample_count as u64;
                    if ui
                        .add(
                            egui::DragValue::new(&mut sample_count)
                                .range(
                                    crate::analysis::fft::MIN_FFT_SAMPLES as u64
                                        ..=crate::analysis::fft::MAX_REFERENCE_RESAMPLE_POINTS
                                            as u64,
                                )
                                .speed(1.0),
                        )
                        .changed()
                    {
                        state.sample_count = sample_count as usize;
                        sample_changed = true;
                    }
                });
                state.sync_sample_count_control_value();
                if sample_changed {
                    queue_fft_refresh(&mut actions, state);
                }
            });
        });
    });

    actions
}

fn render_header(
    ui: &mut Ui,
    layout: &FftLayout,
    state: &mut FftState,
    source_names: &[String],
) -> HeaderActions {
    let mut actions = HeaderActions::default();
    ui.painter()
        .rect_filled(layout.header_main, Rounding::ZERO, header_bg_color());

    let header_rect = layout.header_main.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 4.0);

        ui.horizontal(|ui| {
            ui.spacing_mut().interact_size.y = HEADER_CONTROL_HEIGHT;
            ui.spacing_mut().button_padding.y = 2.0;
            ui.add_space(4.0);

            // ui.label(
            //     egui::RichText::new("FFT Spectrum")
            //         .size(13.0)
            //         .strong()
            //         .color(Color32::from_rgb(200, 200, 210)),
            // );

            // ui.add_space(8.0);

            let mut selected_source = state
                .selected_source
                .clone()
                .or_else(|| state.source_cache.as_ref().map(|src| src.name.clone()));
            let source_selected_text = selected_source.as_deref().unwrap_or("Source waveform");
            let source_width = combo_width_from_texts(
                ui,
                source_selected_text,
                source_names
                    .iter()
                    .map(String::as_str)
                    .chain(std::iter::once("Source waveform")),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_source")
                .width(source_width)
                .selected_text(source_selected_text)
                .show_ui(ui, |ui| {
                    for name in source_names {
                        ui.selectable_value(&mut selected_source, Some(name.clone()), name);
                    }
                });
            if selected_source != state.selected_source {
                state.set_selected_source(selected_source.clone());
                actions.refresh_source = selected_source;
            }

            let mut window = state.window;
            let window_width = combo_width_from_texts(
                ui,
                window.display_name(),
                WindowFunction::all().iter().map(|w| w.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_window")
                .width(window_width)
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
            let mag_scale_width = combo_width_from_texts(
                ui,
                mag_scale.display_name(),
                MagnitudeScale::all().iter().map(|s| s.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_mag_scale")
                .width(mag_scale_width)
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
            let freq_scale_width = combo_width_from_texts(
                ui,
                freq_scale.display_name(),
                FrequencyScale::all().iter().map(|s| s.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_freq_scale")
                .width(freq_scale_width)
                .selected_text(freq_scale.display_name())
                .show_ui(ui, |ui| {
                    for s in FrequencyScale::all() {
                        ui.selectable_value(&mut freq_scale, *s, s.display_name());
                    }
                });
            if freq_scale != state.freq_scale {
                state.set_freq_scale(freq_scale);
            }

            let mut normalization = state.normalization;
            let normalization_width = combo_width_from_texts(
                ui,
                normalization.display_name(),
                SpectrumNormalization::all()
                    .iter()
                    .map(|mode| mode.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_norm")
                .width(normalization_width)
                .selected_text(normalization.display_name())
                .show_ui(ui, |ui| {
                    for mode in SpectrumNormalization::all() {
                        ui.selectable_value(&mut normalization, *mode, mode.display_name());
                    }
                });
            if normalization != state.normalization {
                state.set_normalization(normalization);
            }

            let mut fidelity = state.input_fidelity;
            let fidelity_width = combo_width_from_texts(
                ui,
                fidelity.display_name(),
                InputFidelity::all().iter().map(|mode| mode.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_fidelity")
                .width(fidelity_width)
                .selected_text(fidelity.display_name())
                .show_ui(ui, |ui| {
                    for mode in InputFidelity::all() {
                        ui.selectable_value(&mut fidelity, *mode, mode.display_name());
                    }
                });
            if fidelity != state.input_fidelity {
                state.set_input_fidelity(fidelity);
                queue_fft_refresh(&mut actions, state);
            }

            ui.separator();

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

            let grid_label = if state.show_grid { "Grid [on]" } else { "Grid" };
            if ui.button(grid_label).clicked() {
                state.toggle_grid();
            }

            ui.separator();
            ui.label("Marker");
            if ui
                .selectable_label(state.active_marker_slot == MarkerSlot::M1, "M1")
                .clicked()
            {
                state.set_active_marker_slot(MarkerSlot::M1);
            }
            if ui
                .selectable_label(state.active_marker_slot == MarkerSlot::M2, "M2")
                .clicked()
            {
                state.set_active_marker_slot(MarkerSlot::M2);
            }
            if ui.small_button("Clear Mk").clicked() {
                state.clear_markers();
            }

            ui.separator();

            ui.label("Auto Freq");
            if ui.checkbox(&mut state.freq_auto, "").changed() && state.freq_auto {
                state.update_auto_scale();
            }

            ui.add_enabled_ui(!state.freq_auto, |ui| {
                ui.label("Min");
                ui.add(egui::DragValue::new(&mut state.freq_min).speed(10.0));
                ui.label("Max");
                ui.add(egui::DragValue::new(&mut state.freq_max).speed(10.0));
            });

            if state.freq_scale == FrequencyScale::Log {
                state.freq_min = state.freq_min.max(1e-12);
            }
            if state.freq_max <= state.freq_min {
                state.freq_max = state.freq_min * 1.01;
            }

            ui.separator();

            ui.label("Auto Mag");
            if ui.checkbox(&mut state.mag_auto, "").changed() && state.mag_auto {
                state.update_auto_scale();
            }

            ui.add_enabled_ui(!state.mag_auto, |ui| {
                ui.label("Min");
                ui.add(egui::DragValue::new(&mut state.mag_min).speed(0.5));
                ui.label("Max");
                ui.add(egui::DragValue::new(&mut state.mag_max).speed(0.5));
            });
            if state.mag_max <= state.mag_min {
                state.mag_max = state.mag_min + 1.0;
            }

            ui.separator();

            ui.label("Peak Th (dB)");
            ui.add(
                egui::DragValue::new(&mut state.peak_threshold_db)
                    .speed(0.5)
                    .fixed_decimals(1),
            );
            state.peak_threshold_db = state.peak_threshold_db.clamp(-180.0, 20.0);

            ui.label("Harmonics");
            let mut harmonics = state.num_harmonics as u32;
            if ui
                .add(egui::DragValue::new(&mut harmonics).speed(1.0))
                .changed()
            {
                state.set_num_harmonics(harmonics.clamp(1, 64) as usize);
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
    let grid_metrics = if state.show_grid {
        render_grid(&painter, rect, plot_rect, state)
    } else {
        GridLabelMetrics::default()
    };

    // Spectrum trace
    if let Some(ref data) = state.data {
        render_trace(&painter, plot_rect, data, state);
        let mut cursor_labels: Vec<PlotCursorLabelSpec> = Vec::with_capacity(2);
        let mut line_x_positions: Vec<f32> = Vec::new();

        // Fundamental marker
        if let Some(ref analysis) = state.analysis {
            if let Some(fund_freq) = analysis.fundamental_frequency {
                if let Some(x) = render_fundamental_marker(&painter, plot_rect, fund_freq, state) {
                    line_x_positions.push(x);
                    cursor_labels.push(PlotCursorLabelSpec {
                        anchor_x: x,
                        text: "f0".to_string(),
                        color: fundamental_color(),
                        font: FontId::proportional(10.0),
                    });
                }
            }

            // Harmonic markers
            if state.show_harmonics {
                for (freq, db) in &analysis.harmonics {
                    if let Some(x) = render_harmonic_marker(&painter, plot_rect, *freq, *db, state)
                    {
                        line_x_positions.push(x);
                    }
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
            if let Some(label) = render_user_marker(
                plot_rect,
                marker_freq,
                data,
                state,
                &painter,
                MarkerSlot::M1,
            ) {
                line_x_positions.push(label.anchor_x);
                cursor_labels.push(label);
            }
        }
        if let Some(marker_freq) = state.marker_frequency_secondary {
            if let Some(label) = render_user_marker(
                plot_rect,
                marker_freq,
                data,
                state,
                &painter,
                MarkerSlot::M2,
            ) {
                line_x_positions.push(label.anchor_x);
                cursor_labels.push(label);
            }
        }

        if !cursor_labels.is_empty() {
            render_fft_cursor_labels(
                &painter,
                plot_rect,
                data,
                state,
                &cursor_labels,
                &line_x_positions,
            );
        }
    }

    // Axis labels rendered in gutters so they never overlap trace data.
    painter.text(
        x_axis_title_position(rect, plot_rect),
        egui::Align2::CENTER_BOTTOM,
        "Frequency",
        FontId::proportional(10.0),
        text_color(),
    );

    let y_axis_title = state.mag_scale.display_name();
    let y_axis_title_font = FontId::proportional(10.0);
    let y_axis_title_width = measure_text_width(
        &painter,
        y_axis_title,
        y_axis_title_font.clone(),
        text_color(),
    );
    painter.text(
        y_axis_title_position(
            rect,
            plot_rect,
            grid_metrics.max_y_tick_label_width,
            y_axis_title_width,
        ),
        egui::Align2::LEFT_CENTER,
        y_axis_title,
        y_axis_title_font,
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

#[derive(Debug, Clone, Copy, Default)]
struct GridLabelMetrics {
    max_y_tick_label_width: f32,
}

fn render_grid(
    painter: &egui::Painter,
    _spectrum_rect: Rect,
    plot_rect: Rect,
    state: &FftState,
) -> GridLabelMetrics {
    let freq_ticks = frequency_ticks(state, 10);
    let mag_ticks = magnitude_ticks(state, 8);
    let tick_font = FontId::proportional(9.0);
    let mut metrics = GridLabelMetrics::default();

    for tick in &freq_ticks {
        let x = freq_to_x(tick.value, plot_rect, state);
        if !x.is_finite() || x < plot_rect.min.x || x > plot_rect.max.x {
            continue;
        }
        let stroke = if tick.major {
            Stroke::new(1.0, grid_major_color())
        } else {
            Stroke::new(0.5, grid_minor_color())
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

    for tick in &mag_ticks {
        let point = FftPoint::new(0.0, magnitude_to_linear(tick.value, state), 0.0);
        let y = mag_to_y(&point, plot_rect, state);
        if !y.is_finite() || y < plot_rect.min.y || y > plot_rect.max.y {
            continue;
        }
        let stroke = if tick.major {
            Stroke::new(1.0, grid_major_color())
        } else {
            Stroke::new(0.5, grid_minor_color())
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

    let epsilon = step * 1e-9;
    let start = (min / step).floor() * step;
    let end = (max / step).ceil() * step;

    let mut major_ticks = Vec::with_capacity(approx + 4);
    let mut major = start;
    while major <= end + epsilon && major_ticks.len() < MAX_LINEAR_MAJOR_TICKS {
        if major >= min - epsilon && major <= max + epsilon {
            major_ticks.push(major);
        }
        major += step;
    }

    let mut ticks = Vec::with_capacity(
        major_ticks
            .len()
            .saturating_mul(LINEAR_MINOR_SUBDIVISIONS)
            .min(MAX_LINEAR_MINOR_TICKS),
    );

    for &major_value in &major_ticks {
        ticks.push(AxisTick {
            value: major_value,
            label: labeler(major_value),
            major: true,
        });
    }

    let minor_step = step / LINEAR_MINOR_SUBDIVISIONS as f64;
    if !minor_step.is_finite() || minor_step <= 0.0 {
        ticks.sort_by(|a, b| a.value.total_cmp(&b.value));
        return ticks;
    }

    let mut minor_count = 0usize;
    let mut base = start;
    while base <= end + epsilon && minor_count < MAX_LINEAR_MINOR_TICKS {
        for i in 1..LINEAR_MINOR_SUBDIVISIONS {
            if minor_count >= MAX_LINEAR_MINOR_TICKS {
                break;
            }
            let value = base + i as f64 * minor_step;
            if value <= min + epsilon || value >= max - epsilon {
                continue;
            }
            let coincides_with_major = major_ticks.iter().any(|&m| (m - value).abs() < epsilon);
            if coincides_with_major {
                continue;
            }
            ticks.push(AxisTick {
                value,
                label: String::new(),
                major: false,
            });
            minor_count += 1;
        }
        base += step;
    }

    ticks.sort_by(|a, b| a.value.total_cmp(&b.value));
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
        for mult in 1..=9 {
            let major = mult == 1;
            let value = decade * mult as f64;
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
    let z0 = if state.z0.is_finite() && state.z0 > 0.0 {
        state.z0
    } else {
        50.0
    };
    match state.mag_scale {
        MagnitudeScale::Linear => value,
        MagnitudeScale::DB => 10.0_f64.powf(value / 20.0),
        MagnitudeScale::DBc => {
            let fundamental_db = state
                .analysis
                .as_ref()
                .and_then(|analysis| analysis.fundamental_db)
                .unwrap_or(0.0);
            10.0_f64.powf((value + fundamental_db) / 20.0)
        }
        MagnitudeScale::DBm => {
            let power_w = 1e-3 * 10.0_f64.powf(value / 10.0);
            (power_w * z0).sqrt()
        }
    }
}

fn render_trace(painter: &egui::Painter, rect: Rect, data: &FftData, state: &FftState) {
    if data.is_empty() {
        return;
    }

    let stroke = Stroke::new(1.5, trace_color());
    let clipped_painter = painter.with_clip_rect(rect);

    for window in data.points.windows(2) {
        let [start, end] = window else {
            continue;
        };

        let Some(x0) = freq_to_x_for_trace(start.frequency, rect, state) else {
            continue;
        };
        let Some(x1) = freq_to_x_for_trace(end.frequency, rect, state) else {
            continue;
        };
        let y0 = mag_to_y(start, rect, state);
        let y1 = mag_to_y(end, rect, state);
        if !(x0.is_finite() && y0.is_finite() && x1.is_finite() && y1.is_finite()) {
            continue;
        }
        if segment_is_trivially_outside_rect(x0, y0, x1, y1, rect) {
            continue;
        }

        clipped_painter.line_segment([Pos2::new(x0, y0), Pos2::new(x1, y1)], stroke);
    }
}

fn segment_is_trivially_outside_rect(x0: f32, y0: f32, x1: f32, y1: f32, rect: Rect) -> bool {
    (x0 < rect.min.x && x1 < rect.min.x)
        || (x0 > rect.max.x && x1 > rect.max.x)
        || (y0 < rect.min.y && y1 < rect.min.y)
        || (y0 > rect.max.y && y1 > rect.max.y)
}

#[cfg(test)]
fn clip_line_segment_to_rect(start: Pos2, end: Pos2, rect: Rect) -> Option<[Pos2; 2]> {
    if !(start.x.is_finite() && start.y.is_finite() && end.x.is_finite() && end.y.is_finite()) {
        return None;
    }

    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let mut t_min = 0.0f32;
    let mut t_max = 1.0f32;

    // Liang-Barsky clipping against left, right, top, bottom boundaries.
    for (p, q) in [
        (-dx, start.x - rect.min.x),
        (dx, rect.max.x - start.x),
        (-dy, start.y - rect.min.y),
        (dy, rect.max.y - start.y),
    ] {
        if p.abs() <= f32::EPSILON {
            if q < 0.0 {
                return None;
            }
            continue;
        }

        let t = q / p;
        if p < 0.0 {
            if t > t_max {
                return None;
            }
            if t > t_min {
                t_min = t;
            }
        } else {
            if t < t_min {
                return None;
            }
            if t < t_max {
                t_max = t;
            }
        }
    }

    if t_max < t_min {
        return None;
    }

    let clipped_start = Pos2::new(
        (start.x + dx * t_min).clamp(rect.min.x, rect.max.x),
        (start.y + dy * t_min).clamp(rect.min.y, rect.max.y),
    );
    let clipped_end = Pos2::new(
        (start.x + dx * t_max).clamp(rect.min.x, rect.max.x),
        (start.y + dy * t_max).clamp(rect.min.y, rect.max.y),
    );
    Some([clipped_start, clipped_end])
}

#[derive(Debug, Clone)]
struct PlotCursorLabelSpec {
    anchor_x: f32,
    text: String,
    color: Color32,
    font: FontId,
}

fn render_fundamental_marker(
    painter: &egui::Painter,
    rect: Rect,
    freq: f64,
    state: &FftState,
) -> Option<f32> {
    let x = freq_to_x(freq, rect, state);
    if x < rect.min.x || x > rect.max.x {
        return None;
    }

    painter.line_segment(
        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
        Stroke::new(1.0, fundamental_color()),
    );
    Some(x)
}

fn render_harmonic_marker(
    painter: &egui::Painter,
    rect: Rect,
    freq: f64,
    _db: f64,
    state: &FftState,
) -> Option<f32> {
    let x = freq_to_x(freq, rect, state);
    if x < rect.min.x || x > rect.max.x {
        return None;
    }

    // Short vertical tick
    painter.line_segment(
        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.min.y + 15.0)],
        Stroke::new(1.0, harmonic_color()),
    );
    Some(x)
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
    plot_rect: Rect,
    marker_freq: f64,
    data: &FftData,
    state: &FftState,
    painter: &egui::Painter,
    slot: MarkerSlot,
) -> Option<PlotCursorLabelSpec> {
    let x = freq_to_x(marker_freq, plot_rect, state);
    if !x.is_finite() || x < plot_rect.min.x || x > plot_rect.max.x {
        return None;
    }
    let (marker_name, marker_color) = match slot {
        MarkerSlot::M1 => ("M1", marker_primary_color()),
        MarkerSlot::M2 => ("M2", marker_secondary_color()),
    };
    painter.line_segment(
        [Pos2::new(x, plot_rect.min.y), Pos2::new(x, plot_rect.max.y)],
        Stroke::new(1.0, marker_color),
    );

    data.interpolate(marker_freq).map(|point| {
        let text = match state.mag_scale {
            MagnitudeScale::Linear => {
                format!(
                    "{}: {} | {:.4}",
                    marker_name,
                    format_freq(marker_freq),
                    state.display_magnitude(&point)
                )
            }
            MagnitudeScale::DB => format!(
                "{}: {} | {:.2} dB",
                marker_name,
                format_freq(marker_freq),
                state.display_magnitude(&point)
            ),
            MagnitudeScale::DBc => format!(
                "{}: {} | {:.2} dBc",
                marker_name,
                format_freq(marker_freq),
                state.display_magnitude(&point)
            ),
            MagnitudeScale::DBm => {
                format!(
                    "{}: {} | {:.2} dBm",
                    marker_name,
                    format_freq(marker_freq),
                    state.display_magnitude(&point)
                )
            }
        };
        PlotCursorLabelSpec {
            anchor_x: x,
            text,
            color: marker_color,
            font: FontId::proportional(CURSOR_LABEL_FONT_SIZE),
        }
    })
}

fn render_fft_cursor_labels(
    painter: &egui::Painter,
    plot_rect: Rect,
    data: &FftData,
    state: &FftState,
    labels: &[PlotCursorLabelSpec],
    line_x_positions: &[f32],
) {
    if labels.is_empty() {
        return;
    }

    let requests: Vec<VerticalLabelRequest> = labels
        .iter()
        .map(|label| {
            let text_size =
                measure_text_size(painter, &label.text, label.font.clone(), label.color);
            VerticalLabelRequest {
                anchor_x: label.anchor_x,
                size: Vec2::new(
                    text_size.x + CURSOR_LABEL_TEXT_PADDING_X * 2.0,
                    text_size.y + CURSOR_LABEL_TEXT_PADDING_Y * 2.0,
                ),
            }
        })
        .collect();

    let placements = layout_fft_cursor_labels(plot_rect, &requests, line_x_positions, data, state);
    for (label, placement) in labels.iter().zip(placements.iter()) {
        draw_cursor_label(painter, label, placement);
    }
}

fn layout_fft_cursor_labels(
    plot_rect: Rect,
    requests: &[VerticalLabelRequest],
    line_x_positions: &[f32],
    data: &FftData,
    state: &FftState,
) -> Vec<VerticalLabelPlacement> {
    let max_h = requests.iter().fold(0.0f32, |acc, r| acc.max(r.size.y));
    let config = VerticalLabelLayoutConfig {
        line_clearance: 4.0,
        top_margin: 2.0,
        row_gap: 3.0,
        preferred_rows: 6,
        nudge_step: 8.0,
        nudge_steps: 10,
        label_gap: 2.0,
    };
    let search_band_bottom = plot_rect.min.y
        + config.top_margin
        + (max_h + config.row_gap) * config.preferred_rows as f32
        + 4.0;
    let obstacles = collect_fft_cursor_label_obstacles(plot_rect, data, state, search_band_bottom);

    place_vertical_line_labels(plot_rect, requests, line_x_positions, &obstacles, config)
}

fn collect_fft_cursor_label_obstacles(
    plot_rect: Rect,
    data: &FftData,
    state: &FftState,
    band_bottom: f32,
) -> Vec<Rect> {
    let mut obstacles = Vec::new();
    if !band_bottom.is_finite() || band_bottom <= plot_rect.min.y {
        return obstacles;
    }

    let band_bottom = band_bottom.min(plot_rect.max.y);
    let n = data.points.len();
    if n == 0 {
        return obstacles;
    }
    let step = (n / 600).max(1);
    obstacles.reserve((n / step).min(800));

    for point in data.points.iter().step_by(step) {
        let Some(x) = freq_to_x_for_trace(point.frequency, plot_rect, state) else {
            continue;
        };
        let y = mag_to_y(point, plot_rect, state);
        if !x.is_finite() || !y.is_finite() {
            continue;
        }
        if x < plot_rect.min.x || x > plot_rect.max.x || y < plot_rect.min.y || y > band_bottom {
            continue;
        }
        obstacles.push(Rect::from_center_size(Pos2::new(x, y), Vec2::splat(3.0)));
    }

    obstacles
}

fn draw_cursor_label(
    painter: &egui::Painter,
    label: &PlotCursorLabelSpec,
    placement: &VerticalLabelPlacement,
) {
    let bg = Color32::from_rgba_unmultiplied(20, 22, 28, CURSOR_LABEL_BG_ALPHA);
    painter.rect_filled(
        placement.rect,
        Rounding::same(CURSOR_LABEL_CORNER_RADIUS),
        bg,
    );
    painter.rect_stroke(
        placement.rect,
        Rounding::same(CURSOR_LABEL_CORNER_RADIUS),
        Stroke::new(1.0, label.color.gamma_multiply(0.8)),
    );

    let connector_y = placement.rect.center().y;
    let connector_x = match placement.side {
        LabelSide::Right => placement.rect.min.x,
        LabelSide::Left => placement.rect.max.x,
    };
    painter.line_segment(
        [
            Pos2::new(label.anchor_x, connector_y),
            Pos2::new(connector_x, connector_y),
        ],
        Stroke::new(CURSOR_LABEL_LINE_STROKE, label.color.gamma_multiply(0.75)),
    );

    painter.text(
        Pos2::new(
            placement.rect.min.x + CURSOR_LABEL_TEXT_PADDING_X,
            placement.rect.min.y + CURSOR_LABEL_TEXT_PADDING_Y,
        ),
        egui::Align2::LEFT_TOP,
        &label.text,
        label.font.clone(),
        label.color,
    );
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
        state.clear_markers();
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

fn freq_to_x_for_trace(freq: f64, rect: Rect, state: &FftState) -> Option<f32> {
    if !freq.is_finite() {
        return None;
    }
    match state.freq_scale {
        FrequencyScale::Linear => {
            let range = state.freq_max - state.freq_min;
            if !range.is_finite() || range <= 0.0 {
                return None;
            }
            let t = (freq - state.freq_min) / range;
            Some(rect.min.x + t as f32 * rect.width())
        }
        FrequencyScale::Log => {
            if freq <= 0.0 {
                // Nonpositive frequencies cannot be represented on a log axis.
                return None;
            }
            let f_min = state.freq_min.max(1e-12);
            let f_max = state.freq_max.max(f_min * 1.000_001);
            let log_range = f_max.log10() - f_min.log10();
            if !log_range.is_finite() || log_range <= 0.0 {
                return None;
            }
            let t = (freq.log10() - f_min.log10()) / log_range;
            Some(rect.min.x + t as f32 * rect.width())
        }
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
            let f_min = state.freq_min.max(1e-12);
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
    let value = state.display_magnitude(point);

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

    let panel_rect = info_content_rect(layout);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        egui::ScrollArea::vertical()
            .id_salt("fft_info_panel_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Keep the right-aligned value column stable while scrolling.
                ui.set_width(ui.available_width());
                render_info_panel_content(ui, state);
            });
    });
}

fn render_info_panel_content(ui: &mut Ui, state: &FftState) {
    ui.vertical(|ui| {
        ui.label(
            egui::RichText::new("Analysis")
                .size(10.0)
                .color(text_color()),
        );
        ui.add_space(4.0);

        if let Some(ref analysis) = state.analysis {
            if let Some(fund) = analysis.fundamental_frequency {
                info_row(ui, "Fund.", &format_freq(fund));
            }

            if let Some(fund_db) = analysis.fundamental_db {
                info_row(ui, "Level", &format!("{:.1} dB", fund_db));
            }

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

            info_row(ui, "Harmonics", &format!("{}", analysis.harmonics.len()));
        } else {
            ui.label(
                egui::RichText::new("No data")
                    .size(10.0)
                    .color(Color32::from_rgb(100, 105, 115)),
            );
        }

        ui.add_space(6.0);

        // Window info
        ui.label(egui::RichText::new("Window").size(10.0).color(text_color()));
        info_row(ui, "Type", state.window.display_name());
        info_row(ui, "Norm", state.normalization.display_name());
        info_row(ui, "Fidelity", state.input_fidelity.display_name());
        if !state.time_window_auto {
            info_row(
                ui,
                "Tstart",
                &crate::waveform::axis::format_time(state.time_window_start),
            );
            info_row(
                ui,
                "Tstop",
                &crate::waveform::axis::format_time(state.time_window_end),
            );
        }
        if !state.sample_count_auto {
            info_row(ui, "N set", &format!("{}", state.sample_count));
        }
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

        if state.marker_frequency.is_some() || state.marker_frequency_secondary.is_some() {
            ui.add_space(6.0);
            ui.label(egui::RichText::new("Markers").size(10.0).color(text_color()));
            if let Some(ref data) = state.data {
                if let Some(marker_freq) = state.marker_frequency {
                    info_row(ui, "M1 F", &format_freq(marker_freq));
                    if let Some(point) = data.interpolate(marker_freq) {
                        info_row(ui, "M1 M", &format_marker_magnitude(state, &point));
                    }
                }
                if let Some(marker_freq) = state.marker_frequency_secondary {
                    info_row(ui, "M2 F", &format_freq(marker_freq));
                    if let Some(point) = data.interpolate(marker_freq) {
                        info_row(ui, "M2 M", &format_marker_magnitude(state, &point));
                    }
                }
                if let (Some(m1), Some(m2)) = (state.marker_frequency, state.marker_frequency_secondary)
                {
                    info_row(
                        ui,
                        "ΔF",
                        &format_freq((m2 - m1).abs()),
                    );
                    let m1_mag = data.interpolate(m1).map(|p| state.display_magnitude(&p));
                    let m2_mag = data.interpolate(m2).map(|p| state.display_magnitude(&p));
                    if let (Some(v1), Some(v2)) = (m1_mag, m2_mag) {
                        info_row(ui, "ΔM", &format_marker_delta(state, v2 - v1));
                    }
                }
            }
        }
    });
}

fn format_marker_magnitude(state: &FftState, point: &FftPoint) -> String {
    match state.mag_scale {
        MagnitudeScale::Linear => format!("{:.5}", state.display_magnitude(point)),
        MagnitudeScale::DB => format!("{:.2} dB", state.display_magnitude(point)),
        MagnitudeScale::DBc => format!("{:.2} dBc", state.display_magnitude(point)),
        MagnitudeScale::DBm => format!("{:.2} dBm", state.display_magnitude(point)),
    }
}

fn format_marker_delta(state: &FftState, delta: f64) -> String {
    match state.mag_scale {
        MagnitudeScale::Linear => format!("{:+.5}", delta),
        MagnitudeScale::DB => format!("{:+.2} dB", delta),
        MagnitudeScale::DBc => format!("{:+.2} dBc", delta),
        MagnitudeScale::DBm => format!("{:+.2} dBm", delta),
    }
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

    if let Some(prepared) = crate::analysis::fft::prepare_fft_input_with_options(
        "Demo Signal",
        &time,
        &data,
        state.input_options_for_waveform(&time),
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
    fn test_layout_uses_two_stacked_header_rows() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let layout = calculate_layout(rect);

        assert!((layout.header_top.height() - HEADER_TOP_HEIGHT).abs() < f32::EPSILON);
        assert!((layout.header_main.height() - HEADER_MAIN_HEIGHT).abs() < f32::EPSILON);
        assert!((layout.header_main.min.y - layout.header_top.max.y).abs() < f32::EPSILON);
        assert!((layout.info.min.y - layout.header_main.max.y).abs() < f32::EPSILON);
        assert!(layout.spectrum.min.y >= layout.header_main.max.y);
    }

    #[test]
    fn test_layout_clamps_header_rows_for_short_viewports() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 40.0));
        let layout = calculate_layout(rect);

        assert!((layout.header_top.height() - 34.0).abs() < f32::EPSILON);
        assert!((layout.header_main.height() - 6.0).abs() < f32::EPSILON);
        assert!((layout.info.height() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sync_manual_fft_time_window_auto_tracks_source_bounds() {
        let mut state = FftState::default();
        state.time_window_auto = true;
        state.time_window_start = -1.0;
        state.time_window_end = -0.5;

        sync_manual_fft_time_window(&mut state, Some((1.0, 3.0)));

        assert!((state.time_window_start - 1.0).abs() < f64::EPSILON);
        assert!((state.time_window_end - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sync_manual_fft_time_window_manual_clamps_and_recovers_invalid_range() {
        let mut state = FftState::default();
        state.time_window_auto = false;
        state.time_window_start = 10.0;
        state.time_window_end = 5.0;

        sync_manual_fft_time_window(&mut state, Some((1.0, 3.0)));

        assert!((state.time_window_start - 1.0).abs() < f64::EPSILON);
        assert!((state.time_window_end - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_fft_surface_and_header_backgrounds_match_viewer_chrome() {
        assert_eq!(surface_bg_color(), header_bg_color());
        assert_eq!(surface_bg_color(), viewer_header_bg_color());
    }

    #[test]
    fn test_fft_supported_for_active_analysis_only_time_domain() {
        let mut state = AppState::default();
        assert!(!fft_supported_for_active_analysis(&state));

        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(crate::state::AnalysisResult::new(
            1,
            AnalysisType::Transient,
            "tran",
        ));
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        assert!(fft_supported_for_active_analysis(&state));

        state.simulation.runs[0].analyses[0].analysis_type = AnalysisType::Ac;
        assert!(!fft_supported_for_active_analysis(&state));
    }

    #[test]
    fn test_spectrum_plot_rect_reserves_axis_gutters() {
        let spectrum = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 400.0));
        let plot = spectrum_plot_rect(spectrum);
        assert!((plot.min.x - spectrum.min.x - AXIS_LEFT_GUTTER).abs() < f32::EPSILON);
        assert!((plot.max.x - spectrum.max.x + AXIS_RIGHT_GUTTER).abs() < f32::EPSILON);
        assert!((plot.min.y - spectrum.min.y - AXIS_TOP_GUTTER).abs() < f32::EPSILON);
        assert!((plot.max.y - spectrum.max.y + AXIS_BOTTOM_GUTTER).abs() < f32::EPSILON);
    }

    #[test]
    fn test_axis_titles_are_farther_from_plot_than_tick_labels() {
        let spectrum = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 400.0));
        let plot = spectrum_plot_rect(spectrum);

        let x_tick = x_tick_label_position(plot.center().x, plot);
        let x_axis = x_axis_title_position(spectrum, plot);
        assert!(x_tick.y > plot.max.y);
        assert!(x_axis.y > x_tick.y);

        let y_tick = y_tick_label_position(plot.center().y, plot);
        let y_axis = y_axis_title_position(spectrum, plot, 28.0, 14.0);
        assert!(y_tick.x < plot.min.x);
        assert!(y_axis.x < y_tick.x);
    }

    #[test]
    fn test_y_axis_title_position_tracks_y_value_label_width() {
        let spectrum = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 400.0));
        let plot = spectrum_plot_rect(spectrum);

        let narrow = y_axis_title_position(spectrum, plot, 14.0, 12.0);
        let wide = y_axis_title_position(spectrum, plot, 40.0, 12.0);
        assert!(wide.x < narrow.x);
    }

    #[test]
    fn test_info_content_rect_is_centered_within_panel() {
        let total = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let layout = calculate_layout(total);
        let content = info_content_rect(&layout);
        let lane = Rect::from_min_max(
            Pos2::new(layout.spectrum.max.x, layout.info.min.y),
            layout.info.max,
        );
        let inner_lane = lane.shrink(INFO_PANEL_PADDING);

        assert!((content.center().x - inner_lane.center().x).abs() < f32::EPSILON);
        assert!((content.min.x - inner_lane.min.x).abs() < f32::EPSILON);
        assert!((content.max.x - inner_lane.max.x).abs() < f32::EPSILON);
    }

    #[test]
    fn test_layout_fft_cursor_labels_avoids_line_collisions_and_label_overlap() {
        let plot = Rect::from_min_size(Pos2::ZERO, Vec2::new(600.0, 320.0));
        let requests = vec![
            VerticalLabelRequest {
                anchor_x: 220.0,
                size: Vec2::new(66.0, 16.0),
            },
            VerticalLabelRequest {
                anchor_x: 230.0,
                size: Vec2::new(66.0, 16.0),
            },
        ];
        let line_x = vec![220.0, 230.0, 250.0];
        let data = FftData::default();
        let state = FftState::new();

        let placements = layout_fft_cursor_labels(plot, &requests, &line_x, &data, &state);
        assert_eq!(placements.len(), requests.len());
        assert!(!placements[0].rect.intersects(placements[1].rect));
        for placement in &placements {
            for x in &line_x {
                assert!(!(*x >= placement.rect.min.x && *x <= placement.rect.max.x));
            }
        }
    }

    #[test]
    fn test_collect_fft_cursor_label_obstacles_samples_top_band_trace_points() {
        let plot = Rect::from_min_size(Pos2::ZERO, Vec2::new(600.0, 320.0));
        let mut state = FftState::new();
        state.freq_scale = FrequencyScale::Linear;
        state.freq_min = 0.0;
        state.freq_max = 1000.0;
        state.mag_scale = MagnitudeScale::DB;
        state.mag_min = -120.0;
        state.mag_max = 20.0;

        let freqs = vec![0.0, 100.0, 200.0, 300.0, 400.0, 500.0];
        let mags = vec![1.0; freqs.len()];
        let phases = vec![0.0; freqs.len()];
        let data = FftData::from_spectrum("top", &freqs, &mags, &phases, 1000.0);
        let obstacles = collect_fft_cursor_label_obstacles(plot, &data, &state, plot.min.y + 64.0);

        assert!(!obstacles.is_empty());
        assert!(obstacles
            .iter()
            .all(|r| r.max.y <= plot.min.y + 64.0 + 1e-3));
    }

    #[test]
    fn test_render_info_panel_handles_small_height_with_scroll() {
        let mut state = FftState::new();
        load_demo_data(&mut state);
        state.marker_frequency = state
            .analysis
            .as_ref()
            .and_then(|a| a.fundamental_frequency);
        assert!(state.analysis.is_some());
        assert!(state.source_cache.is_some());

        let ctx = egui::Context::default();
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let layout =
                    calculate_layout(Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 96.0)));
                render_info_panel(ui, &layout, &state);
            });
        });

        assert!(
            !output.shapes.is_empty(),
            "render should produce clipped shapes for constrained-height info panels"
        );
    }

    #[test]
    fn test_render_info_panel_handles_empty_state_with_scroll_container() {
        let state = FftState::new();
        let ctx = egui::Context::default();
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let layout =
                    calculate_layout(Rect::from_min_size(Pos2::ZERO, Vec2::new(480.0, 88.0)));
                render_info_panel(ui, &layout, &state);
            });
        });

        assert!(
            !output.shapes.is_empty(),
            "empty-state info panel should still render header/body within constrained space"
        );
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
    fn test_clip_line_segment_fully_below_rect_is_rejected() {
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
        let clipped = clip_line_segment_to_rect(Pos2::new(1.0, 12.0), Pos2::new(9.0, 14.0), rect);
        assert!(clipped.is_none());
    }

    #[test]
    fn test_clip_line_segment_crossing_bottom_is_trimmed() {
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
        let clipped = clip_line_segment_to_rect(Pos2::new(2.0, 8.0), Pos2::new(8.0, 14.0), rect)
            .expect("segment should intersect bottom edge");

        // Intersects y=10 at t=1/3 -> x=4.
        assert!((clipped[0].x - 2.0).abs() < 1e-6);
        assert!((clipped[0].y - 8.0).abs() < 1e-6);
        assert!((clipped[1].x - 4.0).abs() < 1e-5);
        assert!((clipped[1].y - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_clip_line_segment_outside_left_and_right_is_clipped_to_vertical_edges() {
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
        let clipped = clip_line_segment_to_rect(Pos2::new(-5.0, 5.0), Pos2::new(15.0, 5.0), rect)
            .expect("segment crosses plotting area");
        assert!((clipped[0].x - 0.0).abs() < 1e-6);
        assert!((clipped[0].y - 5.0).abs() < 1e-6);
        assert!((clipped[1].x - 10.0).abs() < 1e-6);
        assert!((clipped[1].y - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_segment_is_trivially_outside_rect_below() {
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
        assert!(segment_is_trivially_outside_rect(
            1.0, 12.0, 9.0, 14.0, rect
        ));
    }

    #[test]
    fn test_segment_is_trivially_outside_rect_false_for_crossing_segment() {
        let rect = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(10.0, 10.0));
        assert!(!segment_is_trivially_outside_rect(
            2.0, 8.0, 8.0, 14.0, rect
        ));
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
    fn test_freq_to_x_log_supports_sub_hz_ranges() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(120.0, 100.0));
        let mut state = FftState::new();
        state.freq_min = 1e-3;
        state.freq_max = 1e3;
        state.freq_scale = FrequencyScale::Log;

        // 1 Hz is centered across six decades (1e-3..1e3).
        let x = freq_to_x(1.0, rect, &state);
        assert!((x - rect.center().x).abs() < 1.0);
    }

    #[test]
    fn test_freq_to_x_for_trace_log_rejects_nonpositive_frequency() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
        let mut state = FftState::new();
        state.freq_min = 10.0;
        state.freq_max = 10_000.0;
        state.freq_scale = FrequencyScale::Log;

        assert!(freq_to_x_for_trace(0.0, rect, &state).is_none());
        assert!(freq_to_x_for_trace(-1.0, rect, &state).is_none());
        assert!(freq_to_x_for_trace(100.0, rect, &state).is_some());
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
    fn test_refresh_fft_from_source_waveform_reference_mode_preserves_large_uniform_input() {
        let mut app_state = AppState::default();
        let fs = 2_000_000.0;
        let n = crate::analysis::fft::DEFAULT_MAX_FFT_POINTS * 3;
        let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
        let values: Vec<f64> = (0..n)
            .map(|i| (2.0 * PI * 250_000.0 * i as f64 / fs).sin())
            .collect();
        app_state
            .simulation
            .waveforms
            .push(crate::state::WaveformData::new(
                "V(out)", time, values, "#4aa3ff",
            ));
        app_state
            .fft_state
            .set_input_fidelity(InputFidelity::Reference);

        refresh_fft_from_source_waveform(&mut app_state, "V(out)");

        let source = app_state
            .fft_state
            .source_cache
            .as_ref()
            .expect("source cache");
        assert_eq!(source.decimation_factor, 1);
        assert_eq!(source.samples.len(), n);
    }

    #[test]
    fn test_refresh_fft_from_source_waveform_interactive_mode_caps_large_uniform_input() {
        let mut app_state = AppState::default();
        let fs = 2_000_000.0;
        let n = crate::analysis::fft::DEFAULT_MAX_FFT_POINTS * 3;
        let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
        let values: Vec<f64> = (0..n)
            .map(|i| (2.0 * PI * 250_000.0 * i as f64 / fs).sin())
            .collect();
        app_state
            .simulation
            .waveforms
            .push(crate::state::WaveformData::new(
                "V(out)", time, values, "#4aa3ff",
            ));
        app_state
            .fft_state
            .set_input_fidelity(InputFidelity::Interactive);

        refresh_fft_from_source_waveform(&mut app_state, "V(out)");

        let source = app_state
            .fft_state
            .source_cache
            .as_ref()
            .expect("source cache");
        assert!(source.samples.len() <= crate::analysis::fft::DEFAULT_MAX_FFT_POINTS);
        assert!(source.decimation_factor > 1);
    }

    #[test]
    fn test_refresh_fft_from_source_waveform_syncs_auto_n_control_to_effective_samples() {
        let mut app_state = AppState::default();
        let fs = 2_000_000.0;
        let n = crate::analysis::fft::DEFAULT_MAX_FFT_POINTS * 3;
        let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
        let values: Vec<f64> = (0..n)
            .map(|i| (2.0 * PI * 250_000.0 * i as f64 / fs).sin())
            .collect();
        app_state
            .simulation
            .waveforms
            .push(crate::state::WaveformData::new(
                "V(out)", time, values, "#4aa3ff",
            ));
        app_state
            .fft_state
            .set_input_fidelity(InputFidelity::Interactive);
        app_state.fft_state.sample_count_auto = true;
        app_state.fft_state.sample_count = 2048;

        refresh_fft_from_source_waveform(&mut app_state, "V(out)");

        let source = app_state
            .fft_state
            .source_cache
            .as_ref()
            .expect("source cache");
        assert_eq!(app_state.fft_state.sample_count, source.samples.len());
    }

    #[test]
    fn test_refresh_fft_from_source_waveform_applies_manual_time_window_and_sample_target() {
        let mut app_state = AppState::default();
        let fs = 100_000.0;
        let n = 100_000usize;
        let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
        let values: Vec<f64> = (0..n)
            .map(|i| (2.0 * PI * 5_000.0 * i as f64 / fs).sin())
            .collect();
        app_state
            .simulation
            .waveforms
            .push(crate::state::WaveformData::new(
                "V(out)", time, values, "#4aa3ff",
            ));
        app_state
            .fft_state
            .set_input_fidelity(InputFidelity::Reference);
        app_state.fft_state.time_window_auto = false;
        app_state.fft_state.time_window_start = 0.2;
        app_state.fft_state.time_window_end = 0.4;
        app_state.fft_state.sample_count_auto = false;
        app_state.fft_state.sample_count = 2048;

        refresh_fft_from_source_waveform(&mut app_state, "V(out)");

        let source = app_state
            .fft_state
            .source_cache
            .as_ref()
            .expect("source cache");
        assert_eq!(source.decimation_factor, 1);
        assert_eq!(source.samples.len(), 2048);
        assert!(source.original_count > 15_000);
        assert!(source.original_count < 25_000);
    }

    #[test]
    fn test_current_fft_source_time_bounds_uses_selected_source() {
        let mut app_state = AppState::default();
        app_state
            .simulation
            .waveforms
            .push(crate::state::WaveformData::new(
                "A",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 0.0, 0.0],
                "#123456",
            ));
        app_state
            .simulation
            .waveforms
            .push(crate::state::WaveformData::new(
                "B",
                vec![10.0, 11.0, 12.0],
                vec![0.0, 0.0, 0.0],
                "#abcdef",
            ));
        app_state
            .fft_state
            .set_selected_source(Some("B".to_string()));

        let bounds = current_fft_source_time_bounds(&app_state).expect("bounds");
        assert!((bounds.0 - 10.0).abs() < 1e-12);
        assert!((bounds.1 - 12.0).abs() < 1e-12);
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
    fn test_frequency_ticks_log_contains_minor_subdivisions() {
        let mut state = FftState::new();
        state.freq_scale = FrequencyScale::Log;
        state.freq_min = 10.0;
        state.freq_max = 100.0;

        let ticks = frequency_ticks(&state, 10);
        assert!(ticks
            .iter()
            .any(|t| !t.major && (t.value - 20.0).abs() < 1e-9));
        assert!(ticks
            .iter()
            .any(|t| !t.major && (t.value - 50.0).abs() < 1e-9));
        assert!(ticks
            .iter()
            .any(|t| !t.major && (t.value - 90.0).abs() < 1e-9));
    }

    #[test]
    fn test_frequency_ticks_linear_contains_minor_gridlines() {
        let mut state = FftState::new();
        state.freq_scale = FrequencyScale::Linear;
        state.freq_min = 0.0;
        state.freq_max = 10.0;

        let ticks = frequency_ticks(&state, 5);
        let major_count = ticks.iter().filter(|t| t.major).count();
        let minor_count = ticks.iter().filter(|t| !t.major).count();

        assert!(major_count >= 3);
        assert!(minor_count > 0);
        assert!(ticks
            .iter()
            .filter(|t| !t.major)
            .all(|t| t.label.is_empty()));
    }

    #[test]
    fn test_magnitude_ticks_contains_minor_gridlines() {
        let mut state = FftState::new();
        state.mag_scale = MagnitudeScale::DB;
        state.mag_min = -120.0;
        state.mag_max = 0.0;

        let ticks = magnitude_ticks(&state, 8);
        assert!(ticks.iter().any(|t| t.major));
        assert!(ticks.iter().any(|t| !t.major));
    }

    #[test]
    fn test_linear_ticks_minor_do_not_overlap_major_values() {
        let ticks = linear_ticks(-5.0, 5.0, 5, |v| format!("{v:.1}"));
        let majors: Vec<f64> = ticks.iter().filter(|t| t.major).map(|t| t.value).collect();
        let epsilon = 1e-9;
        for minor in ticks.iter().filter(|t| !t.major) {
            assert!(majors
                .iter()
                .all(|&major| (major - minor.value).abs() > epsilon));
        }
    }

    #[test]
    fn test_linear_ticks_minor_count_is_capped() {
        let ticks = linear_ticks(0.0, 1_000_000_000.0, 1_000_000, |v| format!("{v:.0}"));
        let minor_count = ticks.iter().filter(|t| !t.major).count();
        assert!(minor_count <= MAX_LINEAR_MINOR_TICKS);
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

    #[test]
    fn test_magnitude_to_linear_dbc_uses_fundamental_db_reference() {
        let mut state = FftState::new();
        state.mag_scale = MagnitudeScale::DBc;
        state.analysis = Some(SpectrumAnalysis {
            fundamental_frequency: Some(1_000.0),
            fundamental_db: Some(-6.0),
            harmonics: Vec::new(),
            thd_percent: None,
            thd_db: None,
            sfdr_db: None,
            snr_db: None,
            sinad_db: None,
            noise_floor_db: None,
        });
        // 0 dBc should map to the same absolute magnitude as -6 dB.
        let v = magnitude_to_linear(0.0, &state);
        assert!((v - 10.0_f64.powf(-6.0 / 20.0)).abs() < 1e-12);
    }

    #[test]
    fn test_format_marker_magnitude_supports_dbc_units() {
        let mut state = FftState::new();
        state.mag_scale = MagnitudeScale::DBc;
        state.analysis = Some(SpectrumAnalysis {
            fundamental_frequency: Some(1_000.0),
            fundamental_db: Some(0.0),
            harmonics: Vec::new(),
            thd_percent: None,
            thd_db: None,
            sfdr_db: None,
            snr_db: None,
            sinad_db: None,
            noise_floor_db: None,
        });
        let point = FftPoint::new(2_000.0, 0.5, 0.0);
        let text = format_marker_magnitude(&state, &point);
        assert!(text.contains("dBc"));
    }
}
