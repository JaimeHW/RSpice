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
                    && let Some(analysis) = run.analyses.get(analysis_idx) {
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

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(ui: &mut Ui, layout: &ViewerLayout, viewer_state: &mut WaveformViewerState) {
    let painter = ui.painter();

    // Header background
    painter.rect_filled(layout.header, Rounding::ZERO, viewer_header_bg_color());

    // Shrink uniformly so all controls sit centered within the header band.
    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            // Drive all widget heights via the spacing system so egui
            // places every control on a single vertically-centered row.
            ui.spacing_mut().interact_size.y = HEADER_HEIGHT - 8.0;
            ui.spacing_mut().button_padding.y = 2.0;

            ui.add_space(4.0);

            // ui.label(
            //     egui::RichText::new("Waveform Viewer")
            //         .size(13.0)
            //         .strong()
            //         .color(Color32::from_rgb(200, 200, 210)),
            // );

            // ui.add_space(12.0);

            if ui
                .add(egui::Button::new("Fit All").min_size(egui::vec2(58.0, HEADER_HEIGHT - 8.0)))
                .clicked()
            {
                viewer_state.view.fit_to_traces(&viewer_state.traces);
            }
            if ui
                .add(egui::Button::new("Fit X").min_size(egui::vec2(48.0, HEADER_HEIGHT - 8.0)))
                .clicked()
            {
                viewer_state.view.fit_x_to_traces(&viewer_state.traces);
            }
            if ui
                .add(egui::Button::new("Fit Y").min_size(egui::vec2(48.0, HEADER_HEIGHT - 8.0)))
                .clicked()
            {
                viewer_state.view.fit_y_to_traces(&viewer_state.traces);
            }

            ui.separator();

            if ui
                .add(egui::Button::new("−").min_size(egui::vec2(28.0, HEADER_HEIGHT - 8.0)))
                .clicked()
            {
                viewer_state.view.zoom(1.25, 0.5, 0.5);
            }
            if ui
                .add(egui::Button::new("+").min_size(egui::vec2(28.0, HEADER_HEIGHT - 8.0)))
                .clicked()
            {
                viewer_state.view.zoom(0.8, 0.5, 0.5);
            }

            ui.separator();

            if ui
                .add(
                    egui::Button::new("Clear Cursors")
                        .min_size(egui::vec2(94.0, HEADER_HEIGHT - 8.0)),
                )
                .clicked()
            {
                viewer_state.cursors.clear();
            }

            if ui
                .add(
                    egui::Button::new("Clear Markers")
                        .min_size(egui::vec2(94.0, HEADER_HEIGHT - 8.0)),
                )
                .clicked()
            {
                viewer_state.clear_markers();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(4.0);

                let meas_text = if viewer_state.show_measurements {
                    "Meas On"
                } else {
                    "Meas Off"
                };
                if ui
                    .add(
                        egui::Button::new(meas_text)
                            .min_size(egui::vec2(68.0, HEADER_HEIGHT - 8.0)),
                    )
                    .on_hover_text("Measurements")
                    .clicked()
                {
                    viewer_state.show_measurements = !viewer_state.show_measurements;
                }

                let export_text = if viewer_state.show_export {
                    "Export On"
                } else {
                    "Export Off"
                };
                if ui
                    .add(
                        egui::Button::new(export_text)
                            .min_size(egui::vec2(76.0, HEADER_HEIGHT - 8.0)),
                    )
                    .on_hover_text("Export")
                    .clicked()
                {
                    viewer_state.show_export = !viewer_state.show_export;
                }
            });
        });
    });
}

// =============================================================================
// Y-Axis Rendering
// =============================================================================

fn render_y_axis(ui: &mut Ui, layout: &ViewerLayout, viewer_state: &WaveformViewerState) {
    let painter = ui.painter();

    // Background
    painter.rect_filled(layout.y_axis, Rounding::ZERO, Color32::from_rgb(25, 27, 33));

    // Calculate ticks
    let y_ticks = axis::calculate_ticks(viewer_state.view.y_min, viewer_state.view.y_max, 6);

    // Generate tick labels
    let labels = axis::generate_tick_labels(&y_ticks.major_ticks, y_ticks.scale, y_ticks.precision);

    // Render tick labels (numeric values) - properly aligned with plot grid
    let font = FontId::proportional(10.0);
    let text_color = Color32::from_rgb(160, 165, 175);
    let mut max_y_tick_label_width = 0.0f32;

    for (i, &tick) in y_ticks.major_ticks.iter().enumerate() {
        let y_frac = (viewer_state.view.y_max - tick) / viewer_state.view.y_range();
        let screen_y = layout.y_axis.min.y + y_frac as f32 * layout.y_axis.height();

        if let Some(label) = labels.get(i) {
            let width = measure_text_width(painter, label, font.clone(), text_color);
            max_y_tick_label_width = max_y_tick_label_width.max(width);
            let text_pos = y_tick_label_position(layout, screen_y);
            painter.text(
                text_pos,
                egui::Align2::RIGHT_CENTER,
                label,
                font.clone(),
                text_color,
            );
        }
    }

    // Y-axis title in outer lane (farther from tick values, matching FFT layout model).
    let unit_label = y_axis_title_text(viewer_state, y_ticks.prefix);
    let title_font = FontId::proportional(10.0);
    let title_color = Color32::from_rgb(130, 135, 145);
    let title_width = measure_text_width(painter, &unit_label, title_font.clone(), title_color);
    painter.text(
        y_axis_title_position(layout, max_y_tick_label_width, title_width),
        egui::Align2::LEFT_CENTER,
        &unit_label,
        title_font,
        title_color,
    );
}

// =============================================================================
// X-Axis Rendering
// =============================================================================

fn render_x_axis(ui: &mut Ui, layout: &ViewerLayout, viewer_state: &WaveformViewerState) {
    let painter = ui.painter();

    // Background
    painter.rect_filled(layout.x_axis, Rounding::ZERO, Color32::from_rgb(25, 27, 33));

    // Calculate ticks
    let x_ticks = axis::calculate_ticks(viewer_state.view.x_min, viewer_state.view.x_max, 8);

    let labels = axis::generate_tick_labels(&x_ticks.major_ticks, x_ticks.scale, x_ticks.precision);

    // Render labels
    let font = FontId::proportional(10.0);
    let text_color = Color32::from_rgb(160, 165, 175);

    for (i, &tick) in x_ticks.major_ticks.iter().enumerate() {
        let x_frac = (tick - viewer_state.view.x_min) / viewer_state.view.x_range();
        let screen_x = layout.x_axis.min.x + x_frac as f32 * layout.x_axis.width();

        if let Some(label) = labels.get(i) {
            let text_pos = x_tick_label_position(layout, screen_x);
            painter.text(
                text_pos,
                egui::Align2::CENTER_TOP,
                label,
                font.clone(),
                text_color,
            );
        }
    }

    // Unit label (analysis-aware: Time/Frequency/Voltage)
    let label = if viewer_state.x_axis_label.is_empty() {
        "Time"
    } else {
        &viewer_state.x_axis_label
    };
    let unit = if viewer_state.x_axis_unit.is_empty() {
        "s"
    } else {
        &viewer_state.x_axis_unit
    };
    let unit_label = format!("{} ({}{})", label, x_ticks.prefix, unit);
    painter.text(
        x_axis_title_position(layout),
        egui::Align2::CENTER_BOTTOM,
        unit_label,
        FontId::proportional(9.0),
        Color32::from_rgb(120, 125, 135),
    );
}

// =============================================================================
// Plot Area Rendering
// =============================================================================

fn render_plot_area(ui: &mut Ui, layout: &ViewerLayout, viewer_state: &mut WaveformViewerState) {
    let painter = ui.painter().clone();

    // Update view dimensions
    viewer_state.view.plot_width = layout.plot.width() as f64;
    viewer_state.view.plot_height = layout.plot.height() as f64;

    // Background
    painter.rect_filled(layout.plot, Rounding::ZERO, viewer_chart_bg_color());

    // Clip to plot area for all subsequent rendering
    let clip_rect = layout.plot;

    // Grid lines
    render_grid(&painter, layout, viewer_state, clip_rect);

    // Spec overlays (render before traces so they appear as background)
    for overlay in viewer_state.spec_overlays.iter().filter(|o| o.visible) {
        overlay.render(
            &painter,
            layout.plot,
            viewer_state.view.y_min,
            viewer_state.view.y_max,
        );
    }

    // Waveform traces
    for trace in viewer_state.traces.iter().filter(|t| t.visible) {
        render_trace(&painter, layout, viewer_state, trace, clip_rect);
    }

    // Cursors
    if viewer_state.cursors.is_active() {
        render_cursors(&painter, layout, viewer_state, clip_rect);
    }

    // Box selection overlay
    if viewer_state.box_selection.is_selecting {
        render_box_selection(&painter, layout, viewer_state, clip_rect);
    }

    // Handle mouse interactions
    let response = ui.allocate_rect(layout.plot, Sense::click_and_drag());
    handle_plot_interactions(response, layout, viewer_state);

    // Plot border (match FFT viewer chrome)
    painter.rect_stroke(
        layout.plot,
        Rounding::ZERO,
        Stroke::new(1.0, plot_border_color()),
    );
}

fn render_grid(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    _clip: Rect,
) {
    let view = &viewer_state.view;

    // X-axis grid
    let x_ticks = axis::calculate_ticks(view.x_min, view.x_max, 8);
    let x_grid = axis::generate_grid_lines(&x_ticks);

    for line in x_grid {
        let x_frac = (line.position - view.x_min) / view.x_range();
        let screen_x = layout.plot.min.x + x_frac as f32 * layout.plot.width();

        let color = match line.line_type {
            GridLineType::Major => grid_major_color(),
            GridLineType::Minor => grid_minor_color(),
        };
        let width = match line.line_type {
            GridLineType::Major => 1.0,
            GridLineType::Minor => 0.5,
        };

        painter.line_segment(
            [
                Pos2::new(screen_x, layout.plot.min.y),
                Pos2::new(screen_x, layout.plot.max.y),
            ],
            Stroke::new(width, color),
        );
    }

    // Y-axis grid
    let y_ticks = axis::calculate_ticks(view.y_min, view.y_max, 6);
    let y_grid = axis::generate_grid_lines(&y_ticks);

    for line in y_grid {
        let y_frac = (view.y_max - line.position) / view.y_range();
        let screen_y = layout.plot.min.y + y_frac as f32 * layout.plot.height();

        let color = match line.line_type {
            GridLineType::Major => grid_major_color(),
            GridLineType::Minor => grid_minor_color(),
        };
        let width = match line.line_type {
            GridLineType::Major => 1.0,
            GridLineType::Minor => 0.5,
        };

        painter.line_segment(
            [
                Pos2::new(layout.plot.min.x, screen_y),
                Pos2::new(layout.plot.max.x, screen_y),
            ],
            Stroke::new(width, color),
        );
    }
}

fn render_trace(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    trace: &TraceData,
    clip: Rect,
) {
    if trace.is_empty() {
        return;
    }

    let view = &viewer_state.view;
    TRACE_RENDER_SCRATCH.with(|scratch| {
        let mut scratch = scratch.borrow_mut();
        let visible_samples = build_trace_polyline_in_scratch(layout, view, trace, &mut scratch);
        if scratch.points.len() < 2 {
            return;
        }

        let color = trace.style.to_color32();
        let width = if trace.highlighted {
            trace.style.width * 2.0
        } else {
            trace.style.width
        };
        let stroke = Stroke::new(width, color);
        let clipped_painter = painter.with_clip_rect(clip);
        clipped_painter.add(Shape::line(scratch.points.clone(), stroke));

        if trace.style.show_markers && visible_samples <= 200 {
            for point in scratch.points.iter().copied() {
                clipped_painter.circle_filled(point, trace.style.marker_size / 2.0, color);
            }
        }
    });
}

#[derive(Debug, Clone, Copy)]
struct TraceScreenSample {
    sample_index: usize,
    data_y: f64,
    pos: Pos2,
}

#[derive(Debug, Clone, Default)]
struct TraceBucket {
    first: Option<TraceScreenSample>,
    last: Option<TraceScreenSample>,
    min: Option<TraceScreenSample>,
    max: Option<TraceScreenSample>,
}

#[cfg(test)]
#[derive(Debug, Clone, Default)]
struct TracePolyline {
    points: Vec<Pos2>,
    visible_samples: usize,
}

#[derive(Debug, Default)]
struct TraceRenderScratch {
    points: Vec<Pos2>,
    buckets: Vec<TraceBucket>,
}

thread_local! {
    static TRACE_RENDER_SCRATCH: RefCell<TraceRenderScratch> =
        RefCell::new(TraceRenderScratch::default());
}

fn trace_screen_pos(
    layout: &ViewerLayout,
    view: &ViewTransform,
    data_x: f64,
    data_y: f64,
) -> Option<Pos2> {
    if !data_x.is_finite() || !data_y.is_finite() {
        return None;
    }
    if view.x_range() <= 0.0 || view.y_range() <= 0.0 {
        return None;
    }
    let screen_x =
        layout.plot.min.x + ((data_x - view.x_min) / view.x_range()) as f32 * layout.plot.width();
    let screen_y =
        layout.plot.min.y + ((view.y_max - data_y) / view.y_range()) as f32 * layout.plot.height();
    if !screen_x.is_finite() || !screen_y.is_finite() {
        return None;
    }
    Some(Pos2::new(screen_x, screen_y))
}

fn visible_trace_index_window(trace: &TraceData, view: &ViewTransform) -> Option<(usize, usize)> {
    if trace.is_empty() || trace.x.is_empty() || view.x_max <= view.x_min {
        return None;
    }

    let first_x = *trace.x.first()?;
    let last_x = *trace.x.last()?;
    if !first_x.is_finite() || !last_x.is_finite() {
        return Some((0, trace.len()));
    }
    if view.x_max < first_x || view.x_min > last_x {
        return None;
    }

    let start = trace
        .x
        .partition_point(|x| *x < view.x_min)
        .saturating_sub(1);
    let end = (trace.x.partition_point(|x| *x <= view.x_max) + 1).min(trace.len());
    if end <= start {
        return None;
    }
    Some((start, end))
}

fn should_render_trace_directly(plot_width_px: usize, visible_samples: usize) -> bool {
    let direct_budget =
        (plot_width_px * DIRECT_RENDER_POINTS_PER_PIXEL).max(DIRECT_RENDER_MIN_SAMPLES);
    visible_samples <= direct_budget.min(DECIMATION_THRESHOLD)
}

fn push_unique_point(points: &mut Vec<Pos2>, point: Pos2) {
    if points.last().copied() == Some(point) {
        return;
    }
    points.push(point);
}

fn update_trace_bucket(bucket: &mut TraceBucket, sample: TraceScreenSample) {
    if bucket
        .first
        .map(|existing| sample.sample_index < existing.sample_index)
        .unwrap_or(true)
    {
        bucket.first = Some(sample);
    }
    if bucket
        .last
        .map(|existing| sample.sample_index > existing.sample_index)
        .unwrap_or(true)
    {
        bucket.last = Some(sample);
    }
    if bucket
        .min
        .map(|existing| sample.data_y < existing.data_y)
        .unwrap_or(true)
    {
        bucket.min = Some(sample);
    }
    if bucket
        .max
        .map(|existing| sample.data_y > existing.data_y)
        .unwrap_or(true)
    {
        bucket.max = Some(sample);
    }
}

fn collect_bucket_points(points: &mut Vec<Pos2>, bucket: &TraceBucket) {
    let mut pending = [bucket.first, bucket.min, bucket.max, bucket.last];
    let mut last_index = None;

    for _ in 0..pending.len() {
        let mut selected_slot = None;
        for (slot, sample) in pending.iter().enumerate() {
            let Some(sample) = sample else {
                continue;
            };
            let replace = selected_slot
                .map(|existing: usize| {
                    sample.sample_index < pending[existing].unwrap().sample_index
                })
                .unwrap_or(true);
            if replace {
                selected_slot = Some(slot);
            }
        }

        let Some(slot) = selected_slot else {
            break;
        };
        let Some(sample) = pending[slot].take() else {
            continue;
        };
        if last_index == Some(sample.sample_index) {
            continue;
        }
        last_index = Some(sample.sample_index);
        push_unique_point(points, sample.pos);
    }
}

#[cfg(test)]
fn build_trace_polyline(
    layout: &ViewerLayout,
    view: &ViewTransform,
    trace: &TraceData,
) -> TracePolyline {
    let mut scratch = TraceRenderScratch::default();
    let visible_samples = build_trace_polyline_in_scratch(layout, view, trace, &mut scratch);
    TracePolyline {
        points: scratch.points,
        visible_samples,
    }
}

fn build_trace_polyline_in_scratch(
    layout: &ViewerLayout,
    view: &ViewTransform,
    trace: &TraceData,
    scratch: &mut TraceRenderScratch,
) -> usize {
    let Some((start, end)) = visible_trace_index_window(trace, view) else {
        scratch.points.clear();
        return 0;
    };

    let visible_samples = end.saturating_sub(start);
    if visible_samples == 0 {
        scratch.points.clear();
        return 0;
    }

    let plot_width_px = layout.plot.width().max(1.0).ceil() as usize;
    scratch.points.clear();

    if should_render_trace_directly(plot_width_px, visible_samples) {
        scratch.points.reserve(visible_samples);
        for idx in start..end {
            let Some((&x, &y)) = trace.x.get(idx).zip(trace.y.get(idx)) else {
                continue;
            };
            if let Some(pos) = trace_screen_pos(layout, view, x, y) {
                push_unique_point(&mut scratch.points, pos);
            }
        }
        return visible_samples;
    }

    let bucket_count = plot_width_px.max(1);
    scratch.buckets.clear();
    scratch.buckets.resize(bucket_count, TraceBucket::default());

    for idx in start..end {
        let Some((&x, &y)) = trace.x.get(idx).zip(trace.y.get(idx)) else {
            continue;
        };
        let Some(pos) = trace_screen_pos(layout, view, x, y) else {
            continue;
        };

        // Ignore samples far outside the clip lane to avoid skewing bucket picks.
        if pos.x < layout.plot.min.x - 1.0 || pos.x > layout.plot.max.x + 1.0 {
            continue;
        }

        let bucket_index = ((pos.x - layout.plot.min.x).floor() as isize)
            .clamp(0, bucket_count as isize - 1) as usize;
        update_trace_bucket(
            &mut scratch.buckets[bucket_index],
            TraceScreenSample {
                sample_index: idx,
                data_y: y,
                pos,
            },
        );
    }

    scratch.points.reserve(bucket_count * 2);
    for bucket in &scratch.buckets {
        collect_bucket_points(&mut scratch.points, bucket);
    }

    visible_samples
}

#[derive(Debug, Clone, PartialEq)]
struct CursorTraceReadoutRow {
    trace_index: usize,
    trace_color: Color32,
    c1_value: Option<f64>,
    c2_value: Option<f64>,
}

fn trace_value_at_cursor(trace: &TraceData, cursor_x: f64) -> Option<f64> {
    if !cursor_x.is_finite() || trace.is_empty() {
        return None;
    }
    let (Some(x_min), Some(x_max)) = (trace.x_min(), trace.x_max()) else {
        return None;
    };
    if cursor_x < x_min || cursor_x > x_max {
        return None;
    }
    trace.interpolate_at(cursor_x).filter(|v| v.is_finite())
}

fn collect_cursor_trace_readouts(
    viewer_state: &WaveformViewerState,
    c1_x: Option<f64>,
    c2_x: Option<f64>,
) -> Vec<CursorTraceReadoutRow> {
    viewer_state
        .traces
        .iter()
        .enumerate()
        .filter(|(_, trace)| trace.visible && !trace.is_empty())
        .map(|(trace_index, trace)| CursorTraceReadoutRow {
            trace_index,
            trace_color: trace.style.to_color32(),
            c1_value: c1_x.and_then(|x| trace_value_at_cursor(trace, x)),
            c2_value: c2_x.and_then(|x| trace_value_at_cursor(trace, x)),
        })
        .collect()
}

fn render_cursor_info_panel(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    c1_x: Option<f64>,
    c2_x: Option<f64>,
) {
    let show_c1 = c1_x.is_some();
    let show_c2 = c2_x.is_some();
    if !show_c1 && !show_c2 {
        return;
    }

    let readouts = collect_cursor_trace_readouts(viewer_state, c1_x, c2_x);
    let value_cols = (show_c1 as usize) + (show_c2 as usize);
    let max_width_available = (layout.plot.width() - 2.0 * CURSOR_INFO_PANEL_MARGIN).max(120.0);
    let computed_width = CURSOR_INFO_PANEL_PADDING * 2.0
        + 10.0
        + CURSOR_INFO_PANEL_BASE_NAME_WIDTH
        + CURSOR_INFO_PANEL_VALUE_WIDTH * value_cols as f32;
    let panel_width = computed_width
        .clamp(CURSOR_INFO_PANEL_MIN_WIDTH, CURSOR_INFO_PANEL_MAX_WIDTH)
        .min(max_width_available);
    let name_col_width = (panel_width
        - CURSOR_INFO_PANEL_PADDING * 2.0
        - 10.0
        - CURSOR_INFO_PANEL_VALUE_WIDTH * value_cols as f32)
        .max(44.0);

    let meta_rows =
        1usize + (show_c1 as usize) + (show_c2 as usize) + if show_c1 && show_c2 { 2 } else { 0 };
    let max_panel_height = (layout.plot.height() - 2.0 * CURSOR_INFO_PANEL_MARGIN).max(120.0);
    let fixed_height = CURSOR_INFO_PANEL_PADDING * 2.0
        + CURSOR_INFO_PANEL_ROW_HEIGHT * (meta_rows as f32 + 1.0)
        + 6.0;
    let trace_row_capacity = ((max_panel_height - fixed_height) / CURSOR_INFO_PANEL_ROW_HEIGHT)
        .floor()
        .max(1.0) as usize;

    let mut visible_trace_rows = if readouts.is_empty() {
        0
    } else {
        readouts.len().min(trace_row_capacity)
    };
    let mut overflow_count = readouts.len().saturating_sub(visible_trace_rows);
    if overflow_count > 0 && visible_trace_rows > 0 {
        visible_trace_rows = visible_trace_rows.saturating_sub(1);
        overflow_count = readouts.len().saturating_sub(visible_trace_rows);
    }
    let rendered_trace_rows = if readouts.is_empty() {
        1
    } else {
        visible_trace_rows + usize::from(overflow_count > 0)
    };

    let panel_height = CURSOR_INFO_PANEL_PADDING * 2.0
        + CURSOR_INFO_PANEL_ROW_HEIGHT * (meta_rows + rendered_trace_rows + 1) as f32
        + 6.0;
    let panel_rect = Rect::from_min_size(
        Pos2::new(
            layout.plot.max.x - CURSOR_INFO_PANEL_MARGIN - panel_width,
            layout.plot.min.y + CURSOR_INFO_PANEL_MARGIN,
        ),
        Vec2::new(panel_width, panel_height.min(max_panel_height)),
    );

    painter.rect_filled(
        panel_rect,
        Rounding::same(4.0),
        Color32::from_rgba_unmultiplied(30, 33, 40, 224),
    );
    painter.rect_stroke(
        panel_rect,
        Rounding::same(4.0),
        Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
    );

    let text_left = panel_rect.min.x + CURSOR_INFO_PANEL_PADDING;
    let row_top = panel_rect.min.y + CURSOR_INFO_PANEL_PADDING;
    let label_color = Color32::from_rgb(130, 136, 148);
    let value_color = Color32::from_rgb(198, 204, 216);
    let header_color = Color32::from_rgb(168, 174, 186);
    let title_font = FontId::proportional(10.5);
    let label_font = FontId::proportional(9.0);
    let value_font = FontId::proportional(10.0);

    let mut y = row_top;
    let draw_meta = |painter: &Painter, y: f32, label: &str, value: String| {
        painter.text(
            Pos2::new(text_left, y),
            egui::Align2::LEFT_TOP,
            label,
            label_font.clone(),
            label_color,
        );
        painter.text(
            Pos2::new(text_left + 58.0, y),
            egui::Align2::LEFT_TOP,
            value,
            value_font.clone(),
            value_color,
        );
    };

    painter.text(
        Pos2::new(text_left, y),
        egui::Align2::LEFT_TOP,
        "Cursor Readout",
        title_font,
        Color32::from_rgb(200, 206, 218),
    );
    y += CURSOR_INFO_PANEL_ROW_HEIGHT;

    if let Some(x1) = c1_x {
        draw_meta(painter, y, "C1", axis::format_time(x1));
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
    }
    if let Some(x2) = c2_x {
        draw_meta(painter, y, "C2", axis::format_time(x2));
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
    }
    if let (Some(x1), Some(x2)) = (c1_x, c2_x) {
        let delta = (x2 - x1).abs();
        draw_meta(painter, y, "dT", axis::format_time_delta(delta));
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
        draw_meta(
            painter,
            y,
            "f",
            axis::format_frequency(axis::period_to_frequency(delta)),
        );
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
    }

    let separator_y = y - 2.0;
    painter.line_segment(
        [
            Pos2::new(panel_rect.min.x + 4.0, separator_y),
            Pos2::new(panel_rect.max.x - 4.0, separator_y),
        ],
        Stroke::new(1.0, Color32::from_rgb(52, 56, 65)),
    );
    y += 4.0;

    let swatch_x = text_left;
    let name_x = swatch_x + 12.0;
    let c1_col_x = if show_c1 && show_c2 {
        panel_rect.max.x - CURSOR_INFO_PANEL_PADDING - CURSOR_INFO_PANEL_VALUE_WIDTH * 2.0
    } else {
        panel_rect.max.x - CURSOR_INFO_PANEL_PADDING - CURSOR_INFO_PANEL_VALUE_WIDTH
    };
    let c2_col_x = panel_rect.max.x - CURSOR_INFO_PANEL_PADDING - CURSOR_INFO_PANEL_VALUE_WIDTH;

    painter.text(
        Pos2::new(name_x, y),
        egui::Align2::LEFT_TOP,
        "Trace",
        label_font.clone(),
        header_color,
    );
    if show_c1 {
        painter.text(
            Pos2::new(c1_col_x, y),
            egui::Align2::LEFT_TOP,
            "C1",
            label_font.clone(),
            header_color,
        );
    }
    if show_c2 {
        painter.text(
            Pos2::new(c2_col_x, y),
            egui::Align2::LEFT_TOP,
            "C2",
            label_font.clone(),
            header_color,
        );
    }
    y += CURSOR_INFO_PANEL_ROW_HEIGHT;

    let y_unit = if viewer_state.y_axis_unit.is_empty() {
        "V"
    } else {
        viewer_state.y_axis_unit.as_str()
    };
    let name_font = FontId::proportional(10.0);

    if readouts.is_empty() {
        painter.text(
            Pos2::new(name_x, y),
            egui::Align2::LEFT_TOP,
            "No visible traces",
            value_font.clone(),
            label_color,
        );
        return;
    }

    for row in readouts.iter().take(visible_trace_rows) {
        let swatch_rect = Rect::from_min_size(
            Pos2::new(swatch_x, y + 3.0),
            Vec2::new(8.0, CURSOR_INFO_PANEL_ROW_HEIGHT - 6.0),
        );
        painter.rect_filled(swatch_rect, Rounding::same(1.0), row.trace_color);

        let trace_name = viewer_state
            .traces
            .get(row.trace_index)
            .map(|trace| trace.name.as_str())
            .unwrap_or("?");
        let display_name =
            truncate_legend_trace_name(painter, trace_name, name_font.clone(), name_col_width);
        painter.text(
            Pos2::new(name_x, y),
            egui::Align2::LEFT_TOP,
            display_name,
            name_font.clone(),
            value_color,
        );

        if show_c1 {
            painter.text(
                Pos2::new(c1_col_x, y),
                egui::Align2::LEFT_TOP,
                format_optional_value(row.c1_value, y_unit),
                value_font.clone(),
                value_color,
            );
        }
        if show_c2 {
            painter.text(
                Pos2::new(c2_col_x, y),
                egui::Align2::LEFT_TOP,
                format_optional_value(row.c2_value, y_unit),
                value_font.clone(),
                value_color,
            );
        }
        y += CURSOR_INFO_PANEL_ROW_HEIGHT;
    }

    if overflow_count > 0 {
        painter.text(
            Pos2::new(name_x, y),
            egui::Align2::LEFT_TOP,
            format!("+{} more", overflow_count),
            label_font,
            label_color,
        );
    }
}

fn render_cursors(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    _clip: Rect,
) {
    let view = &viewer_state.view;
    let cursors = &viewer_state.cursors;
    let marker_count = viewer_state.markers.len();
    let mut labels: Vec<WaveformCursorLabelSpec> = Vec::with_capacity(2 + marker_count);
    let mut line_x_positions: Vec<f32> = Vec::with_capacity(2 + marker_count);

    // Cursor 1
    if let Some(x1) = cursors.cursor1_x {
        let screen_x =
            layout.plot.min.x + ((x1 - view.x_min) / view.x_range()) as f32 * layout.plot.width();

        // Vertical line
        painter.line_segment(
            [
                Pos2::new(screen_x, layout.plot.min.y),
                Pos2::new(screen_x, layout.plot.max.y),
            ],
            Stroke::new(1.5, cursor1_color()),
        );
        line_x_positions.push(screen_x);
        labels.push(WaveformCursorLabelSpec {
            anchor_x: screen_x,
            text: format!("C1 {}", axis::format_time(x1)),
            color: cursor1_color(),
            font: FontId::proportional(CURSOR_LABEL_FONT_SIZE),
        });
    }

    // Cursor 2
    if let Some(x2) = cursors.cursor2_x {
        let screen_x =
            layout.plot.min.x + ((x2 - view.x_min) / view.x_range()) as f32 * layout.plot.width();

        painter.line_segment(
            [
                Pos2::new(screen_x, layout.plot.min.y),
                Pos2::new(screen_x, layout.plot.max.y),
            ],
            Stroke::new(1.5, cursor2_color()),
        );
        line_x_positions.push(screen_x);
        labels.push(WaveformCursorLabelSpec {
            anchor_x: screen_x,
            text: format!("C2 {}", axis::format_time(x2)),
            color: cursor2_color(),
            font: FontId::proportional(CURSOR_LABEL_FONT_SIZE),
        });
    }

    for (idx, marker_x) in viewer_state.markers.iter().enumerate() {
        let screen_x = layout.plot.min.x
            + ((*marker_x - view.x_min) / view.x_range()) as f32 * layout.plot.width();
        if !screen_x.is_finite() || screen_x < layout.plot.min.x || screen_x > layout.plot.max.x {
            continue;
        }
        let color = marker_color(idx);
        painter.line_segment(
            [
                Pos2::new(screen_x, layout.plot.min.y),
                Pos2::new(screen_x, layout.plot.max.y),
            ],
            Stroke::new(1.0, color),
        );
        line_x_positions.push(screen_x);
        labels.push(WaveformCursorLabelSpec {
            anchor_x: screen_x,
            text: format!("M{} {}", idx + 1, axis::format_time(*marker_x)),
            color,
            font: FontId::proportional(CURSOR_LABEL_FONT_SIZE),
        });
    }

    if !labels.is_empty() {
        render_waveform_cursor_labels(painter, layout, viewer_state, &labels, &line_x_positions);
    }
    render_cursor_info_panel(
        painter,
        layout,
        viewer_state,
        cursors.cursor1_x,
        cursors.cursor2_x,
    );
}

#[derive(Debug, Clone)]
struct WaveformCursorLabelSpec {
    anchor_x: f32,
    text: String,
    color: Color32,
    font: FontId,
}

fn render_waveform_cursor_labels(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    labels: &[WaveformCursorLabelSpec],
    line_x_positions: &[f32],
) {
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

    let placements =
        layout_waveform_cursor_labels(layout, viewer_state, &requests, line_x_positions);
    for (label, placement) in labels.iter().zip(placements.iter()) {
        draw_waveform_cursor_label(painter, label, placement);
    }
}

fn layout_waveform_cursor_labels(
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    requests: &[VerticalLabelRequest],
    line_x_positions: &[f32],
) -> Vec<VerticalLabelPlacement> {
    let config = VerticalLabelLayoutConfig {
        line_clearance: 4.0,
        top_margin: 2.0,
        row_gap: 3.0,
        preferred_rows: 5,
        nudge_step: 8.0,
        nudge_steps: 10,
        label_gap: 2.0,
    };
    let max_h = requests.iter().fold(0.0f32, |acc, r| acc.max(r.size.y));
    let search_band_bottom = layout.plot.min.y
        + config.top_margin
        + (max_h + config.row_gap) * config.preferred_rows as f32
        + 4.0;
    let obstacles =
        collect_waveform_cursor_label_obstacles(layout, viewer_state, search_band_bottom);
    place_vertical_line_labels(layout.plot, requests, line_x_positions, &obstacles, config)
}

fn collect_waveform_cursor_label_obstacles(
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    band_bottom: f32,
) -> Vec<Rect> {
    let mut obstacles = Vec::new();
    if !band_bottom.is_finite() || band_bottom <= layout.plot.min.y {
        return obstacles;
    }

    let view = &viewer_state.view;
    if view.x_range() <= 0.0 || view.y_range() <= 0.0 {
        return obstacles;
    }

    let band_bottom = band_bottom.min(layout.plot.max.y);
    for trace in &viewer_state.traces {
        if !trace.visible || trace.is_empty() {
            continue;
        }
        let n = trace.len();
        let step = (n / 600).max(1);
        for idx in (0..n).step_by(step) {
            let (Some(&x), Some(&y)) = (trace.x.get(idx), trace.y.get(idx)) else {
                continue;
            };
            if !x.is_finite() || !y.is_finite() || x < view.x_min || x > view.x_max {
                continue;
            }

            let screen_x = layout.plot.min.x
                + ((x - view.x_min) / view.x_range()) as f32 * layout.plot.width();
            let screen_y = layout.plot.min.y
                + ((view.y_max - y) / view.y_range()) as f32 * layout.plot.height();
            if !screen_x.is_finite() || !screen_y.is_finite() {
                continue;
            }
            if screen_x < layout.plot.min.x
                || screen_x > layout.plot.max.x
                || screen_y < layout.plot.min.y
                || screen_y > band_bottom
            {
                continue;
            }
            obstacles.push(Rect::from_center_size(
                Pos2::new(screen_x, screen_y),
                Vec2::splat(3.0),
            ));
        }
    }
    obstacles
}

fn draw_waveform_cursor_label(
    painter: &Painter,
    label: &WaveformCursorLabelSpec,
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
        Stroke::new(1.0, label.color.gamma_multiply(0.75)),
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

fn render_box_selection(
    painter: &Painter,
    layout: &ViewerLayout,
    viewer_state: &WaveformViewerState,
    _clip: Rect,
) {
    let view = &viewer_state.view;
    let box_sel = &viewer_state.box_selection;

    // Convert data coords to screen coords
    let x1_screen = layout.plot.min.x
        + ((box_sel.start_x - view.x_min) / view.x_range()) as f32 * layout.plot.width();
    let x2_screen = layout.plot.min.x
        + ((box_sel.end_x - view.x_min) / view.x_range()) as f32 * layout.plot.width();
    let y1_screen = layout.plot.min.y
        + ((view.y_max - box_sel.start_y) / view.y_range()) as f32 * layout.plot.height();
    let y2_screen = layout.plot.min.y
        + ((view.y_max - box_sel.end_y) / view.y_range()) as f32 * layout.plot.height();

    let rect = Rect::from_two_pos(
        Pos2::new(x1_screen, y1_screen),
        Pos2::new(x2_screen, y2_screen),
    );

    painter.rect_filled(rect, Rounding::ZERO, box_select_fill());
    painter.rect_stroke(rect, Rounding::ZERO, Stroke::new(2.0, box_select_stroke()));
}

// =============================================================================
// Mouse Interaction Handling
// =============================================================================

fn handle_plot_interactions(
    response: Response,
    layout: &ViewerLayout,
    viewer_state: &mut WaveformViewerState,
) {
    // Clone data_bounds upfront to avoid borrow conflicts
    let data_bounds = viewer_state.data_bounds.clone();

    // Scroll wheel zoom
    if response.hovered() {
        let scroll_delta = response.ctx.input(|i| i.raw_scroll_delta);
        if scroll_delta.y.abs() > 0.1 {
            let factor = if scroll_delta.y > 0.0 { 0.9 } else { 1.1 };

            // Get mouse position as fraction
            if let Some(pos) = response.hover_pos() {
                let x_frac =
                    ((pos.x - layout.plot.min.x) / layout.plot.width()).clamp(0.0, 1.0) as f64;
                let y_frac =
                    ((pos.y - layout.plot.min.y) / layout.plot.height()).clamp(0.0, 1.0) as f64;

                // Check modifier keys - all zoom operations get clamped
                let modifiers = response.ctx.input(|i| i.modifiers);
                if modifiers.shift {
                    viewer_state.view.zoom_x_only(factor, x_frac);
                    viewer_state.view.clamp_to_bounds(&data_bounds);
                } else if modifiers.ctrl {
                    viewer_state.view.zoom_y_only(factor, y_frac);
                    viewer_state.view.clamp_to_bounds(&data_bounds);
                } else {
                    viewer_state
                        .view
                        .zoom_clamped(factor, x_frac, y_frac, &data_bounds);
                }
            }
        }
    }

    // Click to place cursor or marker
    if response.clicked() && !viewer_state.view.did_drag
        && let Some(pos) = response.hover_pos() {
            let x_frac = (pos.x - layout.plot.min.x) / layout.plot.width();
            let data_x = viewer_state.view.x_min + x_frac as f64 * viewer_state.view.x_range();
            let modifiers = response.ctx.input(|i| i.modifiers);
            if modifiers.alt {
                viewer_state.add_marker(data_x);
            } else {
                viewer_state.cursors.place(data_x);
            }
        }

    // Alt + right click removes nearest marker.
    if response.secondary_clicked()
        && let Some(pos) = response.hover_pos() {
            let modifiers = response.ctx.input(|i| i.modifiers);
            if modifiers.alt {
                let x_frac = (pos.x - layout.plot.min.x) / layout.plot.width();
                let data_x = viewer_state.view.x_min + x_frac as f64 * viewer_state.view.x_range();
                let tolerance = viewer_state.view.x_range() * 0.01;
                viewer_state.remove_nearest_marker(data_x, tolerance);
            }
        }

    // Drag handling
    if response.dragged() {
        viewer_state.view.did_drag = true;

        let modifiers = response.ctx.input(|i| i.modifiers);

        if modifiers.shift {
            // Shift+drag = box selection
            if !viewer_state.box_selection.is_selecting {
                if let Some(pos) = response.hover_pos() {
                    let x_frac = (pos.x - layout.plot.min.x) / layout.plot.width();
                    let y_frac = (pos.y - layout.plot.min.y) / layout.plot.height();
                    let data_x =
                        viewer_state.view.x_min + x_frac as f64 * viewer_state.view.x_range();
                    let data_y =
                        viewer_state.view.y_max - y_frac as f64 * viewer_state.view.y_range();
                    viewer_state.box_selection.start(
                        data_x,
                        data_y,
                        pos.x as f64,
                        pos.y as f64,
                        (
                            layout.plot.min.x as f64,
                            layout.plot.min.y as f64,
                            layout.plot.width() as f64,
                            layout.plot.height() as f64,
                        ),
                    );
                }
            } else if let Some(pos) = response.hover_pos() {
                let x_frac = (pos.x - layout.plot.min.x) / layout.plot.width();
                let y_frac = (pos.y - layout.plot.min.y) / layout.plot.height();
                let data_x = viewer_state.view.x_min + x_frac as f64 * viewer_state.view.x_range();
                let data_y = viewer_state.view.y_max - y_frac as f64 * viewer_state.view.y_range();
                viewer_state.box_selection.update(data_x, data_y);
            }
        } else {
            // Regular drag = pan
            let delta = response.drag_delta();
            let data_dx =
                -(delta.x as f64 / viewer_state.view.plot_width) * viewer_state.view.x_range();
            let data_dy =
                (delta.y as f64 / viewer_state.view.plot_height) * viewer_state.view.y_range();
            viewer_state
                .view
                .pan_clamped(data_dx, data_dy, &data_bounds);
        }
    }

    // Drag released
    if response.drag_stopped() {
        if viewer_state.box_selection.is_selecting
            && let Some((x_min, x_max, y_min, y_max)) = viewer_state.box_selection.finish() {
                viewer_state.view.x_min = x_min;
                viewer_state.view.x_max = x_max;
                viewer_state.view.y_min = y_min;
                viewer_state.view.y_max = y_max;
                // Enforce minimum zoom to prevent numerical issues
                viewer_state.view.enforce_minimum_range();
            }
        viewer_state.view.did_drag = false;
    }

    // Keyboard shortcuts when focused
    if response.has_focus() || response.hovered() {
        response.ctx.input(|i| {
            if i.key_pressed(egui::Key::F) {
                viewer_state.view.fit_to_traces(&viewer_state.traces);
            }
            if i.key_pressed(egui::Key::H) {
                viewer_state.view.fit_x_to_traces(&viewer_state.traces);
            }
            if i.key_pressed(egui::Key::V) {
                viewer_state.view.fit_y_to_traces(&viewer_state.traces);
            }
            if i.key_pressed(egui::Key::Escape) {
                viewer_state.cursors.clear();
                viewer_state.clear_markers();
                viewer_state.box_selection.cancel();
            }
        });
    }
}

// =============================================================================
// Legend Rendering
// =============================================================================

fn render_legend(ui: &mut Ui, layout: &ViewerLayout, viewer_state: &mut WaveformViewerState) {
    let painter = ui.painter();

    // Background
    painter.rect_filled(layout.legend, Rounding::ZERO, Color32::from_rgb(30, 33, 40));

    // Match plot chrome so panel/graph top and edges align visually.
    painter.rect_stroke(
        layout.legend,
        Rounding::ZERO,
        Stroke::new(1.0, plot_border_color()),
    );

    // Create UI area for legend items
    let legend_inner = legend_inner_rect(layout.legend);
    ui.allocate_new_ui(UiBuilder::new().max_rect(legend_inner), |ui| {
        ui.set_min_width(legend_inner.width());
        ui.set_max_width(legend_inner.width());
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .id_salt("waveform_legend_scroll")
            .show(ui, |ui| {
                let content_width = ui.available_width().max(0.0);
                ui.set_min_width(content_width);
                ui.set_max_width(content_width);
                render_trace_list_section(ui, viewer_state);
                ui.add_space(LEGEND_SECTION_SPACING);
                render_markers_panel(ui, viewer_state);
                if viewer_state.show_measurements {
                    ui.add_space(LEGEND_SECTION_SPACING);
                    render_measurements_panel(ui, viewer_state);
                }
                if viewer_state.show_export {
                    ui.add_space(LEGEND_SECTION_SPACING);
                    render_export_panel(ui, viewer_state);
                }
            });
    });
}

fn legend_inner_rect(legend_rect: Rect) -> Rect {
    legend_rect.shrink2(Vec2::new(LEGEND_INSET_X, LEGEND_INSET_Y))
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LegendTraceRowLayout {
    show_swatch: bool,
    show_solo: bool,
    name_width: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LegendFindRowLayout {
    show_clear: bool,
    edit_width: f32,
}

fn legend_row_rect(row_left: f32, row_top: f32, row_width: f32) -> Rect {
    Rect::from_min_size(
        Pos2::new(row_left, row_top),
        Vec2::new(row_width.max(0.0), LEGEND_ROW_HEIGHT),
    )
}

fn calculate_legend_trace_row_layout(row_width: f32, item_spacing_x: f32) -> LegendTraceRowLayout {
    let available = row_width.max(0.0);
    let show_swatch = available >= LEGEND_TRACE_SHOW_SWATCH_MIN_WIDTH;
    let show_solo = available >= LEGEND_TRACE_SHOW_SOLO_MIN_WIDTH;

    let swatch_width = if show_swatch {
        LEGEND_TRACE_SWATCH_WIDTH + item_spacing_x
    } else {
        0.0
    };
    let solo_width = if show_solo {
        LEGEND_TRACE_SOLO_WIDTH + item_spacing_x
    } else {
        0.0
    };
    let fixed_width = swatch_width + LEGEND_TRACE_CONTROL_WIDTH + item_spacing_x + solo_width;
    let name_width = (available - fixed_width).max(LEGEND_TRACE_LABEL_MIN_WIDTH);

    LegendTraceRowLayout {
        show_swatch,
        show_solo,
        name_width,
    }
}

fn calculate_legend_find_row_layout(row_width: f32, item_spacing_x: f32) -> LegendFindRowLayout {
    let available = row_width.max(0.0);
    let required_for_clear = LEGEND_FIND_EDIT_MIN_WIDTH + LEGEND_TRACE_SOLO_WIDTH + item_spacing_x;
    let show_clear = available >= required_for_clear;
    let edit_width = if show_clear {
        (available - LEGEND_TRACE_SOLO_WIDTH - item_spacing_x - LEGEND_FIND_RIGHT_GUARD).max(0.0)
    } else {
        available
    };
    LegendFindRowLayout {
        show_clear,
        edit_width,
    }
}

fn truncate_legend_trace_name(
    painter: &Painter,
    text: &str,
    font: FontId,
    max_width: f32,
) -> String {
    const ELLIPSIS: &str = "...";
    if text.is_empty() || max_width <= 0.0 {
        return String::new();
    }
    let text_width = measure_text_width(painter, text, font.clone(), Color32::WHITE);
    if text_width <= max_width {
        return text.to_owned();
    }
    let ellipsis_width = measure_text_width(painter, ELLIPSIS, font.clone(), Color32::WHITE);
    if ellipsis_width >= max_width {
        return ELLIPSIS.to_owned();
    }

    let chars: Vec<char> = text.chars().collect();
    let mut low = 0usize;
    let mut high = chars.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let prefix: String = chars.iter().take(mid).collect();
        let candidate = format!("{prefix}{ELLIPSIS}");
        let width = measure_text_width(painter, &candidate, font.clone(), Color32::WHITE);
        if width <= max_width {
            low = mid;
        } else {
            high = mid.saturating_sub(1);
        }
    }

    let prefix: String = chars.iter().take(low).collect();
    format!("{prefix}{ELLIPSIS}")
}

fn active_solo_trace_index(traces: &[TraceData]) -> Option<usize> {
    let mut solo: Option<usize> = None;
    for (idx, trace) in traces.iter().enumerate() {
        if !trace.visible {
            continue;
        }
        if solo.is_some() {
            return None;
        }
        solo = Some(idx);
    }
    solo
}

fn render_solo_control(ui: &mut Ui, rect: Rect, is_active: bool) -> Response {
    ui.put(rect, egui::RadioButton::new(is_active, ""))
}

fn render_legend_control_label(ui: &mut Ui, text: &str) {
    ui.allocate_ui_with_layout(
        Vec2::new(LEGEND_CONTROL_LABEL_WIDTH, LEGEND_ROW_HEIGHT),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.label(
                egui::RichText::new(text)
                    .size(9.0)
                    .color(Color32::from_rgb(120, 125, 135)),
            );
        },
    );
}

fn render_trace_list_section(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
    ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
    ui.spacing_mut().interact_size.y = LEGEND_ROW_HEIGHT;

    ui.label(
        egui::RichText::new("Traces")
            .size(11.0)
            .strong()
            .color(Color32::from_rgb(160, 165, 175)),
    );
    ui.horizontal(|ui| {
        render_legend_control_label(ui, "Show");
        if ui.small_button("All").clicked() {
            legend::show_all_traces(&mut viewer_state.traces);
        }
        if ui.small_button("Clear").clicked() {
            legend::hide_all_traces(&mut viewer_state.traces);
        }
    });

    ui.horizontal(|ui| {
        render_legend_control_label(ui, "Sort");
        let combo_width = ui.available_width().clamp(60.0, 140.0);
        egui::ComboBox::from_id_salt("waveform_legend_sort")
            .selected_text(match viewer_state.legend_state.sort_by {
                LegendSortOrder::Index => "Index",
                LegendSortOrder::Name => "Name",
                LegendSortOrder::Visibility => "Visible",
            })
            .width(combo_width)
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut viewer_state.legend_state.sort_by,
                    LegendSortOrder::Index,
                    "Index",
                );
                ui.selectable_value(
                    &mut viewer_state.legend_state.sort_by,
                    LegendSortOrder::Name,
                    "Name",
                );
                ui.selectable_value(
                    &mut viewer_state.legend_state.sort_by,
                    LegendSortOrder::Visibility,
                    "Visible",
                );
            });
    });

    ui.horizontal(|ui| {
        render_legend_control_label(ui, "Find");
        let find_layout =
            calculate_legend_find_row_layout(ui.available_width(), ui.spacing().item_spacing.x);
        let edit_rect = ui
            .allocate_space(Vec2::new(find_layout.edit_width, LEGEND_ROW_HEIGHT))
            .1;
        ui.put(
            edit_rect,
            egui::TextEdit::singleline(&mut viewer_state.legend_state.filter).hint_text("trace"),
        );
        if find_layout.show_clear {
            let clear_rect = ui
                .allocate_space(Vec2::new(LEGEND_TRACE_SOLO_WIDTH, edit_rect.height()))
                .1;
            let clear_clicked = ui
                .scope(|ui| {
                    ui.spacing_mut().button_padding = Vec2::ZERO;
                    ui.put(clear_rect, egui::Button::new("x")).clicked()
                })
                .inner;
            if clear_clicked {
                viewer_state.legend_state.clear_filter();
            }
            ui.add_space(LEGEND_FIND_RIGHT_GUARD);
        }
    });
    ui.add_space(4.0);

    let items = legend::build_legend_items(&viewer_state.traces, &viewer_state.legend_state);
    let mut visibility_updates: Vec<(usize, bool)> = Vec::new();
    let mut solo_trace_idx: Option<usize> = None;
    let mut selected_trace_name: Option<String> = None;
    let solo_active_idx = active_solo_trace_index(&viewer_state.traces);
    let trace_rows_width = ui.max_rect().width().max(0.0);
    let trace_rows_left = ui.max_rect().min.x;
    let item_spacing_x = ui.spacing().item_spacing.x;

    for item in &items {
        let color = Color32::from_rgba_unmultiplied(
            item.color[0],
            item.color[1],
            item.color[2],
            item.color[3],
        );
        let selected = viewer_state
            .selected_trace
            .as_deref()
            .is_some_and(|name| name == item.name);

        let row_top = ui.cursor().min.y;
        let row_rect = legend_row_rect(trace_rows_left, row_top, trace_rows_width);
        ui.allocate_rect(row_rect, Sense::hover());
        let row_layout = calculate_legend_trace_row_layout(trace_rows_width, item_spacing_x);
        let mut left = row_rect.min.x;

        if row_layout.show_swatch {
            let swatch_rect = Rect::from_center_size(
                Pos2::new(
                    left + (LEGEND_TRACE_SWATCH_WIDTH * 0.5),
                    row_rect.center().y,
                ),
                Vec2::new(LEGEND_TRACE_SWATCH_WIDTH, LEGEND_TRACE_SWATCH_WIDTH),
            );
            if item.visible {
                ui.painter()
                    .rect_filled(swatch_rect, Rounding::same(2.0), color);
            } else {
                ui.painter()
                    .rect_stroke(swatch_rect, Rounding::same(2.0), Stroke::new(1.0, color));
            }
            left += LEGEND_TRACE_SWATCH_WIDTH + item_spacing_x;
        }

        let checkbox_rect = Rect::from_min_size(
            Pos2::new(left, row_rect.min.y),
            Vec2::new(LEGEND_TRACE_CONTROL_WIDTH, LEGEND_ROW_HEIGHT),
        );
        let mut visible = item.visible;
        if ui
            .put(checkbox_rect, egui::Checkbox::without_text(&mut visible))
            .changed()
        {
            visibility_updates.push((item.index, visible));
        }
        left += LEGEND_TRACE_CONTROL_WIDTH + item_spacing_x;

        if row_layout.show_solo {
            let solo_rect = Rect::from_min_size(
                Pos2::new(left, row_rect.min.y),
                Vec2::new(LEGEND_TRACE_SOLO_WIDTH, LEGEND_ROW_HEIGHT),
            );
            let is_active = solo_active_idx == Some(item.index);
            if render_solo_control(ui, solo_rect, is_active)
                .on_hover_text("Solo trace")
                .clicked()
            {
                solo_trace_idx = Some(item.index);
                selected_trace_name = Some(item.name.clone());
            }
            left += LEGEND_TRACE_SOLO_WIDTH + item_spacing_x;
        }

        let name_slot_width = (row_rect.max.x - left).max(LEGEND_TRACE_LABEL_MIN_WIDTH);
        let name_rect = Rect::from_min_size(
            Pos2::new(left, row_rect.min.y),
            Vec2::new(name_slot_width, LEGEND_ROW_HEIGHT),
        );
        let text_color = if item.visible {
            Color32::from_rgb(200, 205, 215)
        } else {
            Color32::from_rgb(110, 115, 125)
        };
        let display_name = truncate_legend_trace_name(
            ui.painter(),
            &item.name,
            FontId::proportional(10.0),
            (name_slot_width - LEGEND_TEXT_TRUNCATION_PADDING).max(0.0),
        );
        let label = egui::RichText::new(&display_name)
            .size(10.0)
            .color(text_color);
        let label_response = ui
            .allocate_new_ui(UiBuilder::new().max_rect(name_rect), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.selectable_label(selected, label)
                })
                .inner
            })
            .inner;
        let label_response = if display_name != item.name {
            label_response.on_hover_text(&item.name)
        } else {
            label_response
        };
        if label_response.clicked() {
            selected_trace_name = Some(item.name.clone());
        }
    }

    for (idx, visible) in visibility_updates {
        if let Some(trace) = viewer_state.traces.get_mut(idx) {
            trace.visible = visible;
        }
    }
    if let Some(idx) = solo_trace_idx {
        legend::solo_trace(&mut viewer_state.traces, idx);
    }
    apply_legend_selection(viewer_state, selected_trace_name);

    if items.is_empty() {
        ui.label(
            egui::RichText::new("No traces in filter")
                .size(10.0)
                .color(Color32::from_rgb(100, 105, 115)),
        );
    }
}

fn handle_waveform_right_pane_splitter(
    ui: &mut Ui,
    layout: &ViewerLayout,
    viewer_state: &mut WaveformViewerState,
) {
    let half_hit = LEGEND_SPLITTER_HIT_WIDTH * 0.5;
    let splitter_rect = Rect::from_min_max(
        Pos2::new(layout.legend.min.x - half_hit, layout.legend.min.y),
        Pos2::new(layout.legend.min.x + half_hit, layout.legend.max.y),
    );

    let splitter_id = ui.id().with("waveform_right_pane_splitter");
    let mut response = ui.interact(splitter_rect, splitter_id, Sense::click_and_drag());
    response = response.on_hover_cursor(CursorIcon::ResizeHorizontal);

    if response.double_clicked() {
        viewer_state.right_pane_width = None;
    }

    if response.dragged() {
        let delta_x = ui.ctx().input(|i| i.pointer.delta().x);
        let next = next_waveform_right_pane_width(
            viewer_state.right_pane_width,
            layout.legend.width(),
            delta_x,
            layout.total,
        );
        viewer_state.right_pane_width = Some(next);
    }

    let stroke_color = if response.dragged() {
        Color32::from_rgb(115, 150, 220)
    } else if response.hovered() {
        Color32::from_rgb(90, 115, 165)
    } else {
        plot_border_color()
    };
    ui.painter().line_segment(
        [
            Pos2::new(layout.legend.min.x, layout.legend.min.y),
            Pos2::new(layout.legend.min.x, layout.legend.max.y),
        ],
        Stroke::new(LEGEND_SPLITTER_STROKE_WIDTH, stroke_color),
    );
}

fn render_markers_panel(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
    ui.separator();
    ui.label(
        egui::RichText::new("Markers")
            .size(11.0)
            .strong()
            .color(Color32::from_rgb(160, 165, 175)),
    );

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}", viewer_state.markers.len()))
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.label(
            egui::RichText::new("entries")
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        if ui.small_button("Clear").clicked() {
            viewer_state.clear_markers();
        }
    });
    ui.label(
        egui::RichText::new("Alt+LMB add, Alt+RMB remove")
            .size(9.0)
            .color(Color32::from_rgb(120, 125, 135)),
    );

    if viewer_state.markers.is_empty() {
        ui.label(
            egui::RichText::new("No markers")
                .size(10.0)
                .color(Color32::from_rgb(100, 105, 115)),
        );
        return;
    }

    let mut jump_to: Option<f64> = None;
    let mut remove_idx: Option<usize> = None;
    let markers: Vec<f64> = viewer_state.markers.clone();
    for (idx, marker_x) in markers.iter().copied().enumerate() {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("M{}", idx + 1))
                    .size(10.0)
                    .color(marker_color(idx)),
            );
            if ui
                .small_button(axis::format_time(marker_x))
                .on_hover_text("Center X view on marker")
                .clicked()
            {
                jump_to = Some(marker_x);
            }
            if ui
                .small_button("x")
                .on_hover_text("Delete marker")
                .clicked()
            {
                remove_idx = Some(idx);
            }
        });
    }

    if let Some(idx) = remove_idx {
        viewer_state.remove_marker_at(idx);
    }
    if let Some(marker_x) = jump_to {
        center_waveform_view_x_on_marker(
            &mut viewer_state.view,
            &viewer_state.data_bounds,
            marker_x,
        );
    }
}

fn apply_legend_selection(
    viewer_state: &mut WaveformViewerState,
    selected_trace_name: Option<String>,
) {
    if let Some(name) = selected_trace_name {
        viewer_state.selected_trace = Some(name.clone());
        viewer_state.clear_highlights();
        viewer_state.set_trace_highlight(&name, true);
    }
}

fn measurement_cursor_range(viewer_state: &WaveformViewerState) -> Option<(f64, f64)> {
    if !viewer_state.measurement_use_cursor_range {
        return None;
    }
    let (Some(c1), Some(c2)) = (
        viewer_state.cursors.cursor1_x,
        viewer_state.cursors.cursor2_x,
    ) else {
        return None;
    };
    Some((c1.min(c2), c1.max(c2)))
}

fn measurement_trace_indices(viewer_state: &WaveformViewerState) -> Vec<usize> {
    match viewer_state.measurement_scope {
        MeasurementScope::Selected => {
            let Some(selected) = viewer_state.selected_trace.as_deref() else {
                return Vec::new();
            };
            viewer_state
                .traces
                .iter()
                .enumerate()
                .find_map(|(idx, trace)| (trace.name == selected).then_some(idx))
                .into_iter()
                .collect()
        }
        MeasurementScope::Visible => viewer_state
            .traces
            .iter()
            .enumerate()
            .filter_map(|(idx, trace)| trace.visible.then_some(idx))
            .collect(),
        MeasurementScope::All => (0..viewer_state.traces.len()).collect(),
    }
}

fn format_optional_value(value: Option<f64>, unit: &str) -> String {
    value
        .map(|v| axis::format_with_si_prefix(v, unit, 4))
        .unwrap_or_else(|| "--".to_string())
}

fn format_optional_time(value: Option<f64>) -> String {
    value
        .map(axis::format_time)
        .unwrap_or_else(|| "--".to_string())
}

fn format_optional_freq(value: Option<f64>) -> String {
    value
        .map(axis::format_frequency)
        .unwrap_or_else(|| "--".to_string())
}

fn measurement_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}:", label))
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.label(
            egui::RichText::new(value)
                .size(10.0)
                .color(Color32::from_rgb(200, 205, 215)),
        );
    });
}

fn render_trace_measurements(
    ui: &mut Ui,
    trace: &TraceData,
    measurements: &TraceMeasurements,
    y_unit: &str,
    x_unit: &str,
) {
    ui.label(
        egui::RichText::new(&trace.name)
            .size(10.0)
            .strong()
            .color(trace.style.to_color32()),
    );
    measurement_row(ui, "Min", &format_optional_value(measurements.min, y_unit));
    measurement_row(ui, "Max", &format_optional_value(measurements.max, y_unit));
    measurement_row(
        ui,
        "PkPk",
        &format_optional_value(measurements.pk_pk, y_unit),
    );
    measurement_row(
        ui,
        "Mean",
        &format_optional_value(measurements.mean, y_unit),
    );
    measurement_row(ui, "RMS", &format_optional_value(measurements.rms, y_unit));
    measurement_row(
        ui,
        "Std",
        &format_optional_value(measurements.std_dev, y_unit),
    );
    measurement_row(ui, "Rise", &format_optional_time(measurements.rise_time));
    measurement_row(ui, "Fall", &format_optional_time(measurements.fall_time));
    measurement_row(ui, "Period", &format_optional_time(measurements.period));
    measurement_row(ui, "Freq", &format_optional_freq(measurements.frequency));
    measurement_row(
        ui,
        "Duty",
        &measurements
            .duty_cycle
            .map(|v| format!("{:.2}%", v))
            .unwrap_or_else(|| "--".to_string()),
    );
    measurement_row(
        ui,
        "Integral",
        &format_optional_value(measurements.integral, &format!("{}*{}", y_unit, x_unit)),
    );
}

fn render_measurements_panel(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
    ui.separator();
    ui.label(
        egui::RichText::new("Measurements")
            .size(11.0)
            .strong()
            .color(Color32::from_rgb(160, 165, 175)),
    );
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Scope")
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        egui::ComboBox::from_id_salt("waveform_measure_scope")
            .selected_text(viewer_state.measurement_scope.display_name())
            .width(80.0)
            .show_ui(ui, |ui| {
                for scope in MeasurementScope::all() {
                    ui.selectable_value(
                        &mut viewer_state.measurement_scope,
                        *scope,
                        scope.display_name(),
                    );
                }
            });
        ui.checkbox(
            &mut viewer_state.measurement_use_cursor_range,
            "Cursor range",
        );
    });

    if let Some((start, end)) = measurement_cursor_range(viewer_state) {
        measurement_row(
            ui,
            "Range",
            &format!("{} - {}", axis::format_time(start), axis::format_time(end)),
        );
    }

    let trace_indices = measurement_trace_indices(viewer_state);
    if trace_indices.is_empty() {
        ui.label(
            egui::RichText::new("No traces in selected scope")
                .size(10.0)
                .color(Color32::from_rgb(100, 105, 115)),
        );
        return;
    }

    let y_unit = if viewer_state.y_axis_unit.is_empty() {
        "V"
    } else {
        viewer_state.y_axis_unit.as_str()
    };
    let x_unit = if viewer_state.x_axis_unit.is_empty() {
        "s"
    } else {
        viewer_state.x_axis_unit.as_str()
    };

    let cursor_range = measurement_cursor_range(viewer_state);
    let traces = &viewer_state.traces;
    let measurement_cache = &mut viewer_state.measurement_cache;
    measurement_cache.truncate_to_trace_count(traces.len());
    for (idx, trace_idx) in trace_indices.iter().enumerate() {
        if let Some(trace) = traces.get(*trace_idx) {
            if idx > 0 {
                ui.add_space(6.0);
            }
            let measurements = measurement_cache.get_or_compute(*trace_idx, trace, cursor_range);
            render_trace_measurements(ui, trace, measurements, y_unit, x_unit);
        }
    }
}

fn render_export_panel(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
    ui.separator();
    ui.label(
        egui::RichText::new("Export")
            .size(11.0)
            .strong()
            .color(Color32::from_rgb(160, 165, 175)),
    );
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Format")
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        egui::ComboBox::from_id_salt("waveform_export_format")
            .selected_text(export_format_display_name(
                viewer_state.export_options.format,
            ))
            .width(96.0)
            .show_ui(ui, |ui| {
                for format in [ExportFormat::Csv, ExportFormat::Tsv, ExportFormat::SpiceRaw] {
                    ui.selectable_value(
                        &mut viewer_state.export_options.format,
                        format,
                        export_format_display_name(format),
                    );
                }
            });
    });

    ui.horizontal(|ui| {
        ui.checkbox(&mut viewer_state.export_options.include_header, "Header");
        ui.checkbox(&mut viewer_state.export_options.include_hidden, "Hidden");
        ui.checkbox(
            &mut viewer_state.export_options.scientific_notation,
            "Scientific",
        );
    });

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Precision")
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.add(
            egui::DragValue::new(&mut viewer_state.export_options.precision)
                .range(1..=15)
                .speed(1.0),
        );
    });

    let mut use_start = viewer_state.export_options.x_start.is_some();
    let mut use_end = viewer_state.export_options.x_end.is_some();
    ui.horizontal(|ui| {
        if ui.checkbox(&mut use_start, "Start").changed() {
            viewer_state.export_options.x_start = if use_start {
                Some(viewer_state.view.x_min)
            } else {
                None
            };
        }
        if use_start {
            let mut value = viewer_state
                .export_options
                .x_start
                .unwrap_or(viewer_state.view.x_min);
            if ui
                .add(egui::DragValue::new(&mut value).speed(1e-9))
                .changed()
            {
                viewer_state.export_options.x_start = Some(value);
            }
        }
    });
    ui.horizontal(|ui| {
        if ui.checkbox(&mut use_end, "End").changed() {
            viewer_state.export_options.x_end = if use_end {
                Some(viewer_state.view.x_max)
            } else {
                None
            };
        }
        if use_end {
            let mut value = viewer_state
                .export_options
                .x_end
                .unwrap_or(viewer_state.view.x_max);
            if ui
                .add(egui::DragValue::new(&mut value).speed(1e-9))
                .changed()
            {
                viewer_state.export_options.x_end = Some(value);
            }
        }
    });

    if let (Some(start), Some(end)) = (
        viewer_state.export_options.x_start,
        viewer_state.export_options.x_end,
    )
        && end < start {
            viewer_state.export_options.x_end = Some(start);
        }

    let stats = calculate_export_stats(&viewer_state.traces, &viewer_state.export_options);
    measurement_row(ui, "Traces", &format!("{}", stats.num_traces));
    measurement_row(ui, "Points", &format!("{}", stats.num_points));
    measurement_row(
        ui,
        "Est Size",
        &axis::format_with_si_prefix(stats.estimated_size as f64, "B", 2),
    );

    ui.horizontal(|ui| {
        if ui.button("Copy").clicked() {
            let payload = build_export_payload(&viewer_state.traces, &viewer_state.export_options);
            ui.ctx().copy_text(payload.clone());
            viewer_state.export_status = Some(format!("Copied {} bytes", payload.len()));
        }
        if ui.button("Save...").clicked() {
            let payload = build_export_payload(&viewer_state.traces, &viewer_state.export_options);
            viewer_state.export_status = match save_export_payload_with_native_dialog(
                &payload,
                viewer_state.export_options.format,
            ) {
                Ok(path) => Some(format!("Saved {}", path.display())),
                Err(err) => Some(err),
            };
        }
    });

    if let Some(status) = viewer_state.export_status.as_deref() {
        ui.label(
            egui::RichText::new(status)
                .size(9.0)
                .color(Color32::from_rgb(130, 180, 220)),
        );
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests;
