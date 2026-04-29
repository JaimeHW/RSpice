//! Waveform Rendering Engine
//!
//! This module handles all visual rendering of the waveform viewer,
//! including traces, grid, axes, cursors, and overlays.
//!
//! # Rendering Architecture
//!
//! The rendering follows a layered approach (bottom to top):
//! 1. Background fill
//! 2. Grid lines (minor then major)
//! 3. Axis labels
//! 4. Waveform traces (with line decimation for performance)
//! 5. Cursors and cursor readouts
//! 6. Selection overlays (box zoom, highlight)
//! 7. Header/toolbar
//!
//! # Performance
//!
//! For large waveforms (>10k points), line decimation is used to
//! maintain 60fps rendering. This is a standard technique used by
//! commercial tools like Cadence ViVA.

use egui::{
    Color32, CursorIcon, FontId, Painter, Pos2, Rect, Response, Rounding, Sense, Shape, Stroke, Ui,
    UiBuilder, Vec2,
};
use std::cell::RefCell;

use super::axis::{self, GridLineType};
use super::export::{
    ExportFormat, build_export_payload, calculate_export_stats, export_format_display_name,
    save_export_payload_with_native_dialog,
};
use super::legend::{self, LegendSortOrder};
use super::measurements::TraceMeasurements;
use super::state::{MeasurementScope, TraceData, ViewTransform, WaveformViewerState};
use crate::common::app::AppState;
use crate::common::viewer_style::{viewer_chart_bg_color, viewer_header_bg_color};
use crate::utils::vertical_label_layout::{
    LabelSide, VerticalLabelLayoutConfig, VerticalLabelPlacement, VerticalLabelRequest,
    place_vertical_line_labels,
};

mod layout;

use layout::*;
mod axes;
mod cursors;
mod formatting;
mod header;
mod interactions;
mod legend_panel;
mod plot;
mod traces;

use axes::{render_x_axis, render_y_axis};
use header::render_header;
use legend_panel::{handle_waveform_right_pane_splitter, render_legend};
use plot::render_plot_area;

// =============================================================================
// Constants
// =============================================================================

/// Margin for Y-axis labels (pixels)
const Y_AXIS_WIDTH: f32 = 52.0;

/// Margin for X-axis labels (pixels)
const X_AXIS_HEIGHT: f32 = 30.0;

/// Header height (pixels)
const HEADER_HEIGHT: f32 = 32.0;

/// Legend width policy (pixels)
const LEGEND_WIDTH_MIN: f32 = 140.0;
const LEGEND_WIDTH_MAX: f32 = 420.0;
const LEGEND_WIDTH_FRACTION: f32 = 0.18;
const LEGEND_WIDTH_MAX_FRACTION: f32 = 0.45;
const LEGEND_MIN_PLOT_WIDTH: f32 = 220.0;
const LEGEND_SPLITTER_HIT_WIDTH: f32 = 8.0;
const LEGEND_SPLITTER_STROKE_WIDTH: f32 = 1.0;
const LEGEND_SCROLLBAR_ALLOWANCE: f32 = 14.0;
const CHART_TOP_GAP: f32 = 2.0;

/// Maximum points to render before decimation
const DECIMATION_THRESHOLD: usize = 2000;
const DIRECT_RENDER_MIN_SAMPLES: usize = 256;
const DIRECT_RENDER_POINTS_PER_PIXEL: usize = 2;
const AXIS_TITLE_MIN_LEFT_INSET: f32 = 2.0;
const AXIS_TITLE_TO_VALUE_LABEL_GAP: f32 = 6.0;
const AXIS_TITLE_BOTTOM_INSET: f32 = 2.0;
const AXIS_TICK_X_OFFSET: f32 = 2.0;
const AXIS_TICK_Y_OFFSET: f32 = 2.0;
const CURSOR_LABEL_FONT_SIZE: f32 = 9.0;
const CURSOR_LABEL_TEXT_PADDING_X: f32 = 5.0;
const CURSOR_LABEL_TEXT_PADDING_Y: f32 = 2.0;
const CURSOR_LABEL_BG_ALPHA: u8 = 220;
const CURSOR_LABEL_CORNER_RADIUS: f32 = 3.0;
const LEGEND_SECTION_SPACING: f32 = 8.0;
const LEGEND_ROW_HEIGHT: f32 = 18.0;
const LEGEND_TRACE_SWATCH_WIDTH: f32 = 10.0;
const LEGEND_TRACE_CONTROL_WIDTH: f32 = 22.0;
const LEGEND_TRACE_SOLO_WIDTH: f32 = LEGEND_TRACE_CONTROL_WIDTH;
const LEGEND_TRACE_LABEL_MIN_WIDTH: f32 = 28.0;
const LEGEND_TRACE_SHOW_SWATCH_MIN_WIDTH: f32 = 96.0;
const LEGEND_TRACE_SHOW_SOLO_MIN_WIDTH: f32 = 132.0;
const LEGEND_TEXT_TRUNCATION_PADDING: f32 = 8.0;
const LEGEND_FIND_EDIT_MIN_WIDTH: f32 = 40.0;
const LEGEND_FIND_RIGHT_GUARD: f32 = 6.0;
const LEGEND_CONTROL_LABEL_WIDTH: f32 = 28.0;
const LEGEND_INSET_X: f32 = 4.0;
const LEGEND_INSET_Y: f32 = 8.0;
const CURSOR_INFO_PANEL_MARGIN: f32 = 8.0;
const CURSOR_INFO_PANEL_PADDING: f32 = 8.0;
const CURSOR_INFO_PANEL_ROW_HEIGHT: f32 = 16.0;
const CURSOR_INFO_PANEL_BASE_NAME_WIDTH: f32 = 96.0;
const CURSOR_INFO_PANEL_VALUE_WIDTH: f32 = 78.0;
const CURSOR_INFO_PANEL_MIN_WIDTH: f32 = 170.0;
const CURSOR_INFO_PANEL_MAX_WIDTH: f32 = 340.0;

// Grid line colors (using runtime values since Color32 constructors aren't const)
fn grid_major_color() -> Color32 {
    Color32::from_rgb(50, 52, 58)
}
fn grid_minor_color() -> Color32 {
    Color32::from_rgb(35, 37, 42)
}

// Cursor colors
fn cursor1_color() -> Color32 {
    Color32::from_rgb(255, 200, 50)
}
fn cursor2_color() -> Color32 {
    Color32::from_rgb(50, 200, 255)
}

fn marker_color(index: usize) -> Color32 {
    const MARKER_COLORS: [Color32; 6] = [
        Color32::from_rgb(255, 155, 95),
        Color32::from_rgb(180, 235, 120),
        Color32::from_rgb(255, 220, 120),
        Color32::from_rgb(205, 170, 255),
        Color32::from_rgb(130, 215, 255),
        Color32::from_rgb(255, 135, 180),
    ];
    MARKER_COLORS[index % MARKER_COLORS.len()]
}

// Box selection colors
fn box_select_fill() -> Color32 {
    Color32::from_rgba_unmultiplied(59, 130, 246, 50)
}
fn box_select_stroke() -> Color32 {
    Color32::from_rgb(59, 130, 246)
}

fn plot_border_color() -> Color32 {
    Color32::from_rgb(60, 65, 75)
}

// =============================================================================
// Main Rendering Entry Point
// =============================================================================

/// Render the complete waveform viewer
///
/// Uses persistent state from `AppState.waveform_viewer` to maintain pan/zoom
/// across frames. Only reloads trace data when simulation data changes.
pub fn render_waveform_viewer(ui: &mut Ui, app_state: &mut AppState) {
    // Get simulation data version to detect changes
    let sim_data_version = app_state.simulation.data_version;
    let has_waveforms = !app_state.simulation.waveforms.is_empty();
    let has_traces = !app_state.waveform_viewer.traces.is_empty();

    // Reload traces when:
    // 1. Data version changed, OR
    // 2. There are waveforms but no traces loaded yet (initial load case)
    let needs_reload = app_state.waveform_viewer.data_version != sim_data_version
        || (has_waveforms && !has_traces);

    if needs_reload {
        // Clone waveforms to avoid borrow issues
        let waveforms: Vec<_> = app_state.simulation.waveforms.clone();
        app_state.waveform_viewer.load_from_simulation(&waveforms);
        app_state.waveform_viewer.data_version = sim_data_version;

        // Always fit on data reload
        app_state.waveform_viewer.fit_to_data_bounds();

        // Set axis labels based on current analysis type
        if let Some(run_idx) = app_state.simulation.active_run_idx
            && let Some(analysis_idx) = app_state.simulation.active_analysis_idx
            && let Some(run) = app_state.simulation.runs.get(run_idx)
            && let Some(analysis) = run.analyses.get(analysis_idx)
        {
            let (x_label, x_unit, y_label, y_unit) = analysis.analysis_type.axis_info();
            app_state.waveform_viewer.x_axis_label = x_label.to_string();
            app_state.waveform_viewer.x_axis_unit = x_unit.to_string();
            app_state.waveform_viewer.y_axis_label = y_label.to_string();
            app_state.waveform_viewer.y_axis_unit = y_unit.to_string();
        }
    }

    // Clamp view to data bounds every frame to enforce limits
    let bounds = app_state.waveform_viewer.data_bounds.clone();
    if bounds.valid {
        app_state.waveform_viewer.view.clamp_to_bounds(&bounds);
    }

    // Calculate layout regions and CLAIM the full available space
    // This is crucial for the panel to maintain its size properly
    let available_rect = ui.available_rect_before_wrap();
    let auto_width = preferred_waveform_right_pane_width(ui, &app_state.waveform_viewer);
    app_state.waveform_viewer.right_pane_auto_width_hint = auto_width;
    let legend_width = resolve_waveform_right_pane_width(
        available_rect,
        app_state.waveform_viewer.right_pane_width,
        auto_width,
    );
    if app_state.waveform_viewer.right_pane_width.is_some() {
        app_state.waveform_viewer.right_pane_width = Some(legend_width);
    }
    let layout = calculate_layout_with_legend_width(available_rect, legend_width);

    // Allocate the total available space to claim it
    // This tells egui we're using all the space
    let (_id, _rect) = ui.allocate_space(available_rect.size());

    // Render each section
    // Use split borrows to avoid borrow checker issues
    render_header(ui, &layout, &mut app_state.waveform_viewer);
    render_y_axis(ui, &layout, &app_state.waveform_viewer);
    handle_waveform_right_pane_splitter(ui, &layout, &mut app_state.waveform_viewer);
    render_plot_area(ui, &layout, &mut app_state.waveform_viewer);
    render_x_axis(ui, &layout, &app_state.waveform_viewer);
    render_legend(ui, &layout, &mut app_state.waveform_viewer);
}
