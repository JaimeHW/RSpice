//! FFT Viewer Rendering
//!
//! Commercial-grade egui rendering for FFT/spectrum visualization.

use egui::{
    Color32, CursorIcon, FontId, Pos2, Rect, Rounding, Sense, Shape, Stroke, Ui, UiBuilder, Vec2,
};
use std::cell::RefCell;

use super::data::{FftData, FftPoint, SpectrumNormalization};
use super::state::{FftState, FrequencyScale, InputFidelity, MagnitudeScale};
use super::window::WindowFunction;
use crate::common::app::AppState;
use crate::common::viewer_style::{viewer_chart_bg_color, viewer_header_bg_color};
use crate::state::AnalysisType;
use crate::utils::vertical_label_layout::{
    LabelSide, VerticalLabelLayoutConfig, VerticalLabelPlacement, VerticalLabelRequest,
    place_vertical_line_labels,
};

mod layout;

use layout::*;
mod header;
mod info_panel;
mod interactions;
mod markers;
mod source;
mod spectrum;
mod trace;

use header::{render_header, render_time_controls_header};
use info_panel::render_info_panel;
use interactions::handle_fft_info_splitter;
use source::{
    collect_fft_source_names, current_fft_source_time_bounds, fft_supported_for_active_analysis,
    refresh_fft_from_source_waveform,
};
use spectrum::{render_spectrum, render_spectrum_core};

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

fn panel_bg_color() -> Color32 {
    // Match waveform viewer right-side legend panel fill.
    Color32::from_rgb(30, 33, 40)
}

fn panel_border_color() -> Color32 {
    Color32::from_rgb(60, 65, 75)
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

fn marker_color_for_slot(slot_index: usize) -> Color32 {
    match slot_index {
        0 => marker_primary_color(),
        1 => marker_secondary_color(),
        _ => {
            const EXTRA_COLORS: &[(u8, u8, u8)] = &[
                (146, 196, 255),
                (255, 214, 102),
                (162, 230, 135),
                (255, 145, 112),
                (201, 168, 255),
                (119, 221, 219),
            ];
            let (r, g, b) = EXTRA_COLORS[(slot_index - 2) % EXTRA_COLORS.len()];
            Color32::from_rgb(r, g, b)
        }
    }
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
        app_state.analysis.fft_state.clear();
    }

    let available_rect = ui.available_rect_before_wrap();
    // Claim full available space so the parent resizable panel keeps user height
    // instead of collapsing to a content-driven "natural" size.
    let (_id, _rect) = ui.allocate_space(available_rect.size());
    let auto_info_width = preferred_fft_info_pane_width(ui, &app_state.analysis.fft_state);
    app_state.analysis.fft_state.info_pane_auto_width_hint = auto_info_width;
    let info_width = resolve_fft_info_pane_width(
        available_rect,
        app_state.analysis.fft_state.info_pane_width,
        auto_info_width,
    );
    if app_state.analysis.fft_state.info_pane_width.is_some() {
        app_state.analysis.fft_state.info_pane_width = Some(info_width);
    }
    let layout = calculate_layout_with_info_width(available_rect, info_width);
    let source_names = collect_fft_source_names(app_state);
    let source_time_bounds = current_fft_source_time_bounds(app_state);

    let header_actions = {
        let state = &mut app_state.analysis.fft_state;
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

    let state = &mut app_state.analysis.fft_state;
    handle_fft_info_splitter(ui, &layout, state);
    render_spectrum(ui, &layout, state);
    render_info_panel(ui, &layout, state);
}

/// Public render function
pub fn render_fft_plot(ui: &mut Ui, state: &mut FftState) {
    let available_rect = ui.available_rect_before_wrap();
    let (_id, _rect) = ui.allocate_space(available_rect.size());
    let layout = calculate_layout(available_rect);

    render_spectrum_core(ui, &layout, state);
}

const HEADER_ROW_HEIGHT: f32 = 34.0;
const HEADER_TOP_HEIGHT: f32 = HEADER_ROW_HEIGHT;
const HEADER_MAIN_HEIGHT: f32 = HEADER_ROW_HEIGHT;
const INFO_WIDTH_MIN: f32 = 150.0;
const INFO_WIDTH_MAX: f32 = 420.0;
const INFO_WIDTH_FRACTION: f32 = 0.20;
const INFO_WIDTH_MAX_FRACTION: f32 = 0.45;
const FFT_MIN_PLOT_WIDTH: f32 = 220.0;
const INFO_SPLITTER_HIT_WIDTH: f32 = 8.0;
const INFO_SPLITTER_STROKE_WIDTH: f32 = 1.0;
const INFO_SCROLLBAR_ALLOWANCE: f32 = 14.0;
const CHART_LEFT_PADDING: f32 = 8.0;
const CHART_RIGHT_PADDING: f32 = 0.0;
const CHART_TOP_GAP: f32 = 0.0;
const CHART_BOTTOM_PADDING: f32 = 8.0;
const HEADER_CONTROL_HEIGHT: f32 = 24.0;
const HEADER_DROPDOWN_MIN_WIDTH: f32 = 82.0;
const HEADER_DROPDOWN_MAX_WIDTH: f32 = 220.0;
const HEADER_DROPDOWN_TEXT_PADDING: f32 = 28.0;
const INFO_PANEL_PADDING: f32 = 8.0;
const AXIS_LEFT_GUTTER: f32 = 52.0;
const AXIS_RIGHT_GUTTER: f32 = 0.0;
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
const SPECTRUM_DECIMATION_THRESHOLD: usize = 2000;
const SPECTRUM_DIRECT_MIN_BINS: usize = 256;
const SPECTRUM_DIRECT_BINS_PER_PIXEL: usize = 2;

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

// =============================================================================
// Tests
// =============================================================================
