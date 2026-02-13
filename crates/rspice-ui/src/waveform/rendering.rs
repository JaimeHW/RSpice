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
    Color32, FontId, Painter, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2,
};

use super::axis::{self, GridLineType};
use super::export::{calculate_export_stats, export_to_csv, export_to_spice_raw, ExportFormat};
use super::legend::{self, LegendSortOrder};
use super::measurements::{self, TraceMeasurements};
use super::state::{MeasurementScope, TraceData, ViewTransform, WaveformViewerState};
use crate::common::app::AppState;
use crate::common::viewer_style::{viewer_chart_bg_color, viewer_header_bg_color};
use crate::utils::vertical_label_layout::{
    place_vertical_line_labels, LabelSide, VerticalLabelLayoutConfig, VerticalLabelPlacement,
    VerticalLabelRequest,
};

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
const LEGEND_WIDTH_MAX: f32 = 220.0;
const LEGEND_WIDTH_FRACTION: f32 = 0.18;
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
const LEGEND_INSET_X: f32 = 4.0;
const LEGEND_INSET_Y: f32 = 8.0;

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
        log::info!(
            "Waveform data changed: {} -> {}, reloading {} traces",
            app_state.waveform_viewer.data_version,
            sim_data_version,
            app_state.simulation.waveforms.len()
        );
        // Clone waveforms to avoid borrow issues
        let waveforms: Vec<_> = app_state.simulation.waveforms.clone();
        app_state.waveform_viewer.load_from_simulation(&waveforms);
        app_state.waveform_viewer.data_version = sim_data_version;

        // Always fit on data reload
        app_state.waveform_viewer.fit_to_data_bounds();

        // Set axis labels based on current analysis type
        if let Some(run_idx) = app_state.simulation.active_run_idx {
            if let Some(analysis_idx) = app_state.simulation.active_analysis_idx {
                if let Some(run) = app_state.simulation.runs.get(run_idx) {
                    if let Some(analysis) = run.analyses.get(analysis_idx) {
                        let (x_label, x_unit, y_label, y_unit) = analysis.analysis_type.axis_info();
                        app_state.waveform_viewer.x_axis_label = x_label.to_string();
                        app_state.waveform_viewer.x_axis_unit = x_unit.to_string();
                        app_state.waveform_viewer.y_axis_label = y_label.to_string();
                        app_state.waveform_viewer.y_axis_unit = y_unit.to_string();
                    }
                }
            }
        }

        log::info!(
            "Data bounds: x=[{:.6e}, {:.6e}], y=[{:.3e}, {:.3e}], valid={}",
            app_state.waveform_viewer.data_bounds.x_min,
            app_state.waveform_viewer.data_bounds.x_max,
            app_state.waveform_viewer.data_bounds.y_min,
            app_state.waveform_viewer.data_bounds.y_max,
            app_state.waveform_viewer.data_bounds.valid
        );
        log::info!(
            "View after fit: x=[{:.6e}, {:.6e}], y=[{:.3e}, {:.3e}]",
            app_state.waveform_viewer.view.x_min,
            app_state.waveform_viewer.view.x_max,
            app_state.waveform_viewer.view.y_min,
            app_state.waveform_viewer.view.y_max,
        );
    }

    // Clamp view to data bounds every frame to enforce limits
    let bounds = app_state.waveform_viewer.data_bounds.clone();
    if bounds.valid {
        app_state.waveform_viewer.view.clamp_to_bounds(&bounds);
    }

    // Calculate layout regions and CLAIM the full available space
    // This is crucial for the panel to maintain its size properly
    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect);

    // Allocate the total available space to claim it
    // This tells egui we're using all the space
    let (_id, _rect) = ui.allocate_space(available_rect.size());

    // Render each section
    // Use split borrows to avoid borrow checker issues
    render_header(ui, &layout, &mut app_state.waveform_viewer);
    render_y_axis(ui, &layout, &app_state.waveform_viewer);
    render_plot_area(ui, &layout, &mut app_state.waveform_viewer);
    render_x_axis(ui, &layout, &app_state.waveform_viewer);
    render_legend(ui, &layout, &mut app_state.waveform_viewer);
}

// =============================================================================
// Layout Calculation
// =============================================================================

/// Layout regions for the waveform viewer
#[derive(Debug, Clone)]
pub struct ViewerLayout {
    /// Full available rectangle
    pub total: Rect,
    /// Header bar region
    pub header: Rect,
    /// Y-axis labels region
    pub y_axis: Rect,
    /// Main plot canvas region (this is where waveforms are drawn)
    pub plot: Rect,
    /// X-axis labels region
    pub x_axis: Rect,
    /// Legend region
    pub legend: Rect,
}

/// Calculate layout regions from available space
fn calculate_layout(available: Rect) -> ViewerLayout {
    let total = available;
    let legend_width =
        (total.width() * LEGEND_WIDTH_FRACTION).clamp(LEGEND_WIDTH_MIN, LEGEND_WIDTH_MAX);

    // Header at top
    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));
    let content_top = header.max.y + CHART_TOP_GAP;
    let content_height = (total.height() - HEADER_HEIGHT - CHART_TOP_GAP).max(0.0);
    let chart_height = (content_height - X_AXIS_HEIGHT).max(0.0);

    // Legend on right side (below header)
    let legend = Rect::from_min_size(
        Pos2::new(total.max.x - legend_width, content_top),
        Vec2::new(legend_width, content_height),
    );

    // X-axis at bottom (excluding legend)
    let x_axis = Rect::from_min_size(
        Pos2::new(total.min.x + Y_AXIS_WIDTH, total.max.y - X_AXIS_HEIGHT),
        Vec2::new((total.width() - Y_AXIS_WIDTH - legend_width).max(0.0), X_AXIS_HEIGHT),
    );

    // Y-axis on left side (between header and x-axis)
    let y_axis = Rect::from_min_size(
        Pos2::new(total.min.x, content_top),
        Vec2::new(Y_AXIS_WIDTH, chart_height),
    );

    // Plot area in the center
    let plot = Rect::from_min_size(
        Pos2::new(total.min.x + Y_AXIS_WIDTH, content_top),
        Vec2::new(
            (total.width() - Y_AXIS_WIDTH - legend_width).max(0.0),
            chart_height,
        ),
    );

    ViewerLayout {
        total,
        header,
        y_axis,
        plot,
        x_axis,
        legend,
    }
}

fn x_axis_title_position(layout: &ViewerLayout) -> Pos2 {
    Pos2::new(
        layout.x_axis.center().x,
        layout.x_axis.max.y - AXIS_TITLE_BOTTOM_INSET,
    )
}

fn y_axis_title_position(
    layout: &ViewerLayout,
    max_y_tick_label_width: f32,
    y_title_width: f32,
) -> Pos2 {
    let y_tick_anchor_x = y_tick_label_position(layout, layout.plot.center().y).x;
    let y_tick_left_edge = y_tick_anchor_x - max_y_tick_label_width.max(0.0);
    let title_left = (y_tick_left_edge - AXIS_TITLE_TO_VALUE_LABEL_GAP - y_title_width)
        .max(layout.y_axis.min.x + AXIS_TITLE_MIN_LEFT_INSET);
    Pos2::new(title_left, layout.plot.center().y)
}

fn x_tick_label_position(layout: &ViewerLayout, x: f32) -> Pos2 {
    Pos2::new(x, layout.plot.max.y + AXIS_TICK_Y_OFFSET)
}

fn y_tick_label_position(layout: &ViewerLayout, y: f32) -> Pos2 {
    Pos2::new(layout.plot.min.x - AXIS_TICK_X_OFFSET, y)
}

fn measure_text_width(painter: &Painter, text: &str, font: FontId, color: Color32) -> f32 {
    painter
        .layout_no_wrap(text.to_owned(), font, color)
        .size()
        .x
}

fn measure_text_size(painter: &Painter, text: &str, font: FontId, color: Color32) -> Vec2 {
    painter.layout_no_wrap(text.to_owned(), font, color).size()
}

fn y_axis_title_text(viewer_state: &WaveformViewerState, prefix: &str) -> String {
    let unit = if viewer_state.y_axis_unit.is_empty() {
        "V"
    } else {
        &viewer_state.y_axis_unit
    };
    axis::format_axis_unit(unit, prefix)
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

            ui.label(
                egui::RichText::new("Waveform Viewer")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(12.0);

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
    let polyline = build_trace_polyline(layout, view, trace);
    let points = polyline.points;

    if points.len() < 2 {
        return;
    }

    // Draw the line with clipping
    let color = trace.style.to_color32();
    let width = if trace.highlighted {
        trace.style.width * 2.0
    } else {
        trace.style.width
    };
    let stroke = Stroke::new(width, color);

    // Use a clipped painter to ensure lines don't extend beyond plot area
    let clipped_painter = painter.with_clip_rect(clip);

    // Draw line segments with clipping
    for window in points.windows(2) {
        clipped_painter.line_segment([window[0], window[1]], stroke);
    }

    // Draw markers if enabled
    if trace.style.show_markers && polyline.visible_samples <= 200 {
        for point in &points {
            painter.circle_filled(*point, trace.style.marker_size / 2.0, color);
        }
    }
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

#[derive(Debug, Clone, Default)]
struct TracePolyline {
    points: Vec<Pos2>,
    visible_samples: usize,
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

    let start = trace.x.partition_point(|x| *x < view.x_min).saturating_sub(1);
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
    let mut bucket_samples = [
        bucket.first,
        bucket.min,
        bucket.max,
        bucket.last,
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    if bucket_samples.is_empty() {
        return;
    }
    bucket_samples.sort_by_key(|sample| sample.sample_index);
    bucket_samples.dedup_by_key(|sample| sample.sample_index);
    for sample in bucket_samples {
        push_unique_point(points, sample.pos);
    }
}

fn build_trace_polyline(
    layout: &ViewerLayout,
    view: &ViewTransform,
    trace: &TraceData,
) -> TracePolyline {
    let Some((start, end)) = visible_trace_index_window(trace, view) else {
        return TracePolyline::default();
    };

    let visible_samples = end.saturating_sub(start);
    if visible_samples == 0 {
        return TracePolyline::default();
    }

    let plot_width_px = layout.plot.width().max(1.0).ceil() as usize;
    let mut points = Vec::new();

    if should_render_trace_directly(plot_width_px, visible_samples) {
        points.reserve(visible_samples);
        for idx in start..end {
            let Some((&x, &y)) = trace.x.get(idx).zip(trace.y.get(idx)) else {
                continue;
            };
            if let Some(pos) = trace_screen_pos(layout, view, x, y) {
                push_unique_point(&mut points, pos);
            }
        }
        return TracePolyline {
            points,
            visible_samples,
        };
    }

    let bucket_count = plot_width_px.max(1);
    let mut buckets = vec![TraceBucket::default(); bucket_count];

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
            &mut buckets[bucket_index],
            TraceScreenSample {
                sample_index: idx,
                data_y: y,
                pos,
            },
        );
    }

    points.reserve(bucket_count * 2);
    for bucket in &buckets {
        collect_bucket_points(&mut points, bucket);
    }

    TracePolyline {
        points,
        visible_samples,
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

    // Delta readout (if both cursors active)
    if let (Some(x1), Some(x2)) = (cursors.cursor1_x, cursors.cursor2_x) {
        let delta = (x2 - x1).abs();
        let delta_str = axis::format_time_delta(delta);
        let freq_str = format!(
            "f = {}",
            axis::format_frequency(axis::period_to_frequency(delta))
        );

        // Draw in bottom-right of plot
        let box_rect = Rect::from_min_size(
            Pos2::new(layout.plot.max.x - 140.0, layout.plot.max.y - 50.0),
            Vec2::new(130.0, 44.0),
        );

        painter.rect_filled(
            box_rect,
            Rounding::same(4.0),
            Color32::from_rgba_unmultiplied(30, 33, 40, 220),
        );
        painter.rect_stroke(
            box_rect,
            Rounding::same(4.0),
            Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
        );

        painter.text(
            Pos2::new(box_rect.min.x + 8.0, box_rect.min.y + 8.0),
            egui::Align2::LEFT_TOP,
            delta_str,
            FontId::proportional(11.0),
            Color32::from_rgb(200, 200, 210),
        );

        painter.text(
            Pos2::new(box_rect.min.x + 8.0, box_rect.min.y + 26.0),
            egui::Align2::LEFT_TOP,
            freq_str,
            FontId::proportional(11.0),
            Color32::from_rgb(160, 165, 175),
        );
    }
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
                    viewer_state.view.zoom_x_only(factor as f64, x_frac);
                    viewer_state.view.clamp_to_bounds(&data_bounds);
                } else if modifiers.ctrl {
                    viewer_state.view.zoom_y_only(factor as f64, y_frac);
                    viewer_state.view.clamp_to_bounds(&data_bounds);
                } else {
                    viewer_state
                        .view
                        .zoom_clamped(factor as f64, x_frac, y_frac, &data_bounds);
                }
            }
        }
    }

    // Click to place cursor or marker
    if response.clicked() && !viewer_state.view.did_drag {
        if let Some(pos) = response.hover_pos() {
            let x_frac = (pos.x - layout.plot.min.x) / layout.plot.width();
            let data_x = viewer_state.view.x_min + x_frac as f64 * viewer_state.view.x_range();
            let modifiers = response.ctx.input(|i| i.modifiers);
            if modifiers.alt {
                viewer_state.add_marker(data_x);
            } else {
                viewer_state.cursors.place(data_x);
            }
        }
    }

    // Alt + right click removes nearest marker.
    if response.secondary_clicked() {
        if let Some(pos) = response.hover_pos() {
            let modifiers = response.ctx.input(|i| i.modifiers);
            if modifiers.alt {
                let x_frac = (pos.x - layout.plot.min.x) / layout.plot.width();
                let data_x = viewer_state.view.x_min + x_frac as f64 * viewer_state.view.x_range();
                let tolerance = viewer_state.view.x_range() * 0.01;
                viewer_state.remove_nearest_marker(data_x, tolerance);
            }
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
        if viewer_state.box_selection.is_selecting {
            if let Some((x_min, x_max, y_min, y_max)) = viewer_state.box_selection.finish() {
                viewer_state.view.x_min = x_min;
                viewer_state.view.x_max = x_max;
                viewer_state.view.y_min = y_min;
                viewer_state.view.y_max = y_max;
                // Enforce minimum zoom to prevent numerical issues
                viewer_state.view.enforce_minimum_range();
            }
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

    // Border on left
    painter.line_segment(
        [
            Pos2::new(layout.legend.min.x, layout.legend.min.y),
            Pos2::new(layout.legend.min.x, layout.legend.max.y),
        ],
        Stroke::new(1.0, Color32::from_rgb(50, 52, 58)),
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

fn truncate_legend_trace_name(painter: &Painter, text: &str, font: FontId, max_width: f32) -> String {
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
        let mid = (low + high + 1) / 2;
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

fn render_trace_list_section(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
    ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
    ui.spacing_mut().interact_size.y = LEGEND_ROW_HEIGHT;

    ui.label(
        egui::RichText::new("Traces")
            .size(11.0)
            .strong()
            .color(Color32::from_rgb(160, 165, 175)),
    );
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        if ui.small_button("All").clicked() {
            legend::show_all_traces(&mut viewer_state.traces);
        }
        if ui.small_button("None").clicked() {
            legend::hide_all_traces(&mut viewer_state.traces);
        }
    });

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Sort").size(9.0).color(Color32::from_rgb(120, 125, 135)));
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
        ui.label(egui::RichText::new("Find").size(9.0).color(Color32::from_rgb(120, 125, 135)));
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
                ui.painter().rect_stroke(
                    swatch_rect,
                    Rounding::same(2.0),
                    Stroke::new(1.0, color),
                );
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
        let label = egui::RichText::new(&display_name).size(10.0).color(text_color);
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
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .size(10.0)
                    .color(Color32::from_rgb(200, 205, 215)),
            );
        });
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
    measurement_row(ui, "PkPk", &format_optional_value(measurements.pk_pk, y_unit));
    measurement_row(ui, "Mean", &format_optional_value(measurements.mean, y_unit));
    measurement_row(ui, "RMS", &format_optional_value(measurements.rms, y_unit));
    measurement_row(ui, "Std", &format_optional_value(measurements.std_dev, y_unit));
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
    for (idx, trace_idx) in trace_indices.iter().enumerate() {
        if let Some(trace) = viewer_state.traces.get(*trace_idx) {
            if idx > 0 {
                ui.add_space(6.0);
            }
            let measurements = if let Some((start, end)) = cursor_range {
                measurements::calculate_measurements_in_range(trace, start, end)
            } else {
                measurements::calculate_all_measurements(trace)
            };
            render_trace_measurements(ui, trace, &measurements, y_unit, x_unit);
        }
    }
}

fn export_format_display_name(format: ExportFormat) -> &'static str {
    match format {
        ExportFormat::Csv => "CSV",
        ExportFormat::Tsv => "TSV",
        ExportFormat::SpiceRaw => "SPICE RAW",
    }
}

fn build_export_payload(traces: &[TraceData], options: &super::export::ExportOptions) -> String {
    match options.format {
        ExportFormat::SpiceRaw => export_to_spice_raw(traces, "RSpice Waveforms"),
        ExportFormat::Csv | ExportFormat::Tsv => export_to_csv(traces, options),
    }
}

fn save_export_payload_with_dialog(
    payload: &str,
    format: ExportFormat,
) -> Result<std::path::PathBuf, String> {
    let extension = format.extension();
    let filter_name = export_format_display_name(format);
    let dialog = rfd::FileDialog::new()
        .add_filter(filter_name, &[extension])
        .set_file_name(format!("waveforms.{}", extension))
        .set_title("Export Waveforms");
    let Some(path) = dialog.save_file() else {
        return Err("Export canceled".to_string());
    };
    std::fs::write(&path, payload).map_err(|err| err.to_string())?;
    Ok(path)
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
            .selected_text(export_format_display_name(viewer_state.export_options.format))
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
            if ui.add(egui::DragValue::new(&mut value).speed(1e-9)).changed() {
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
            if ui.add(egui::DragValue::new(&mut value).speed(1e-9)).changed() {
                viewer_state.export_options.x_end = Some(value);
            }
        }
    });

    if let (Some(start), Some(end)) = (
        viewer_state.export_options.x_start,
        viewer_state.export_options.x_end,
    ) {
        if end < start {
            viewer_state.export_options.x_end = Some(start);
        }
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
            viewer_state.export_status = match save_export_payload_with_dialog(
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
mod tests {
    use super::*;

    #[test]
    fn test_calculate_layout() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
        let layout = calculate_layout(rect);

        // Verify layout regions don't overlap incorrectly
        assert!(layout.header.max.y <= layout.plot.min.y);
        assert!(layout.y_axis.max.x <= layout.plot.min.x);
        assert!(layout.plot.max.x <= layout.legend.min.x);
        assert!(layout.plot.max.y <= layout.x_axis.max.y);
    }

    #[test]
    fn test_calculate_layout_small() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(300.0, 200.0));
        let layout = calculate_layout(rect);

        // Should still produce valid layout even if cramped
        assert!(layout.plot.width() >= 0.0);
        assert!(layout.plot.height() >= 0.0);
    }

    #[test]
    fn test_layout_uses_fft_matched_chart_top_gap() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
        let layout = calculate_layout(rect);

        assert!((layout.plot.min.y - layout.header.max.y - CHART_TOP_GAP).abs() < f32::EPSILON);
        assert!((layout.y_axis.min.y - layout.header.max.y - CHART_TOP_GAP).abs() < f32::EPSILON);
    }

    #[test]
    fn test_visible_trace_index_window_expands_one_sample_past_view_bounds() {
        let trace = TraceData::new(
            "T",
            vec![0.0, 1.0, 2.0, 3.0, 4.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0],
        );
        let mut view = ViewTransform::default();
        view.x_min = 1.5;
        view.x_max = 2.5;

        let window = visible_trace_index_window(&trace, &view).expect("window");
        assert_eq!(window, (1, 4));
    }

    #[test]
    fn test_visible_trace_index_window_returns_none_when_trace_is_outside_view() {
        let trace = TraceData::new(
            "T",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 0.0, 0.0],
        );
        let mut view = ViewTransform::default();
        view.x_min = 10.0;
        view.x_max = 11.0;

        assert!(visible_trace_index_window(&trace, &view).is_none());
    }

    #[test]
    fn test_should_render_trace_directly_uses_visible_density_not_total_trace_length() {
        assert!(should_render_trace_directly(1000, 900));
        assert!(!should_render_trace_directly(1000, 5000));
    }

    #[test]
    fn test_measurement_cursor_range_requires_dual_cursor_and_flag() {
        let mut state = WaveformViewerState::new();
        state.measurement_use_cursor_range = true;
        assert!(measurement_cursor_range(&state).is_none());

        state.cursors.place(4.0);
        state.cursors.place(1.0);
        assert_eq!(measurement_cursor_range(&state), Some((1.0, 4.0)));

        state.measurement_use_cursor_range = false;
        assert!(measurement_cursor_range(&state).is_none());
    }

    #[test]
    fn test_measurement_trace_indices_respect_scope() {
        let mut state = WaveformViewerState::new();
        let mut t0 = TraceData::new("A", vec![0.0, 1.0], vec![0.0, 1.0]);
        let mut t1 = TraceData::new("B", vec![0.0, 1.0], vec![1.0, 2.0]);
        t0.visible = true;
        t1.visible = false;
        state.traces = vec![t0, t1];

        state.measurement_scope = MeasurementScope::Visible;
        assert_eq!(measurement_trace_indices(&state), vec![0]);

        state.measurement_scope = MeasurementScope::All;
        assert_eq!(measurement_trace_indices(&state), vec![0, 1]);

        state.measurement_scope = MeasurementScope::Selected;
        state.selected_trace = Some("B".to_string());
        assert_eq!(measurement_trace_indices(&state), vec![1]);

        state.selected_trace = Some("Missing".to_string());
        assert!(measurement_trace_indices(&state).is_empty());
    }

    #[test]
    fn test_build_export_payload_routes_by_format() {
        let traces = vec![TraceData::new(
            "V(out)",
            vec![0.0, 1e-6],
            vec![0.0, 1.0],
        )];

        let mut csv_opts = super::super::export::ExportOptions::default();
        csv_opts.format = ExportFormat::Csv;
        let csv = build_export_payload(&traces, &csv_opts);
        assert!(csv.contains("Time,"));

        let mut tsv_opts = super::super::export::ExportOptions::default();
        tsv_opts.format = ExportFormat::Tsv;
        let tsv = build_export_payload(&traces, &tsv_opts);
        assert!(tsv.contains('\t'));

        let mut raw_opts = super::super::export::ExportOptions::default();
        raw_opts.format = ExportFormat::SpiceRaw;
        let raw = build_export_payload(&traces, &raw_opts);
        assert!(raw.contains("Title: RSpice Waveforms"));
        assert!(raw.contains("Values:"));
    }

    fn screen_to_data_y(layout: &ViewerLayout, view: &ViewTransform, screen_y: f32) -> f64 {
        let y_frac = (screen_y - layout.plot.min.y) as f64 / layout.plot.height() as f64;
        view.y_max - y_frac * view.y_range()
    }

    #[test]
    fn test_build_trace_polyline_uses_visible_window_density() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1200.0, 600.0));
        let layout = calculate_layout(rect);
        let mut view = ViewTransform::default();
        view.x_min = 10.0;
        view.x_max = 10.1;
        view.y_min = -1.2;
        view.y_max = 1.2;

        let n = 200_000usize;
        let dt = 20.0 / (n as f64 - 1.0);
        let x: Vec<f64> = (0..n).map(|i| i as f64 * dt).collect();
        let y: Vec<f64> = x
            .iter()
            .map(|t| (2.0 * std::f64::consts::PI * 5_000.0 * t).sin())
            .collect();
        let trace = TraceData::new("HF", x, y);

        let polyline = build_trace_polyline(&layout, &view, &trace);
        assert!(polyline.visible_samples > 900);
        assert!(polyline.visible_samples < 1100);
        assert!(polyline.points.len() > 200);
    }

    #[test]
    fn test_build_trace_polyline_bucket_decimation_preserves_extrema() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(900.0, 500.0));
        let layout = calculate_layout(rect);
        let mut view = ViewTransform::default();
        view.x_min = 0.0;
        view.x_max = 1.0;
        view.y_min = -1.2;
        view.y_max = 1.2;

        let n = 200_000usize;
        let x: Vec<f64> = (0..n).map(|i| i as f64 / (n as f64 - 1.0)).collect();
        let y: Vec<f64> = x
            .iter()
            .map(|t| (2.0 * std::f64::consts::PI * 250.0 * t).sin())
            .collect();
        let trace = TraceData::new("HF", x, y);

        let polyline = build_trace_polyline(&layout, &view, &trace);
        assert!(polyline.points.len() >= layout.plot.width() as usize);

        let y_values: Vec<f64> = polyline
            .points
            .iter()
            .map(|p| screen_to_data_y(&layout, &view, p.y))
            .collect();
        let max_y = y_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let min_y = y_values.iter().copied().fold(f64::INFINITY, f64::min);
        assert!(max_y > 0.95);
        assert!(min_y < -0.95);
    }

    #[test]
    fn test_build_trace_polyline_ignores_non_finite_samples() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 480.0));
        let layout = calculate_layout(rect);
        let mut view = ViewTransform::default();
        view.x_min = 0.0;
        view.x_max = 4.0;
        view.y_min = -2.0;
        view.y_max = 2.0;

        let trace = TraceData::new(
            "NF",
            vec![0.0, 1.0, 2.0, 3.0, 4.0],
            vec![0.0, f64::NAN, 1.0, f64::INFINITY, -1.0],
        );

        let polyline = build_trace_polyline(&layout, &view, &trace);
        assert!(polyline.points.len() >= 2);
        assert!(polyline.points.iter().all(|p| p.x.is_finite() && p.y.is_finite()));
    }

    #[test]
    fn test_color_constants() {
        // Verify color functions return distinguishable colors
        assert_ne!(grid_major_color(), grid_minor_color());
        assert_ne!(cursor1_color(), cursor2_color());
        assert_ne!(box_select_fill(), box_select_stroke());
    }

    #[test]
    fn test_layout_dimensions() {
        assert!(Y_AXIS_WIDTH > 0.0);
        assert!(X_AXIS_HEIGHT > 0.0);
        assert!(HEADER_HEIGHT > 0.0);
        assert!(LEGEND_WIDTH_MIN > 0.0);
        assert!(LEGEND_WIDTH_MAX >= LEGEND_WIDTH_MIN);
        assert!(LEGEND_WIDTH_FRACTION > 0.0);
        assert!((LEGEND_TRACE_SOLO_WIDTH - LEGEND_TRACE_CONTROL_WIDTH).abs() < f32::EPSILON);
    }

    #[test]
    fn test_active_solo_trace_index_detects_single_visible_trace() {
        let mut traces = vec![
            TraceData::new("A", vec![0.0], vec![0.0]),
            TraceData::new("B", vec![0.0], vec![0.0]),
            TraceData::new("C", vec![0.0], vec![0.0]),
        ];
        traces[0].visible = false;
        traces[1].visible = true;
        traces[2].visible = false;
        assert_eq!(active_solo_trace_index(&traces), Some(1));
    }

    #[test]
    fn test_active_solo_trace_index_returns_none_for_ambiguous_visibility() {
        let mut traces = vec![
            TraceData::new("A", vec![0.0], vec![0.0]),
            TraceData::new("B", vec![0.0], vec![0.0]),
        ];
        traces[0].visible = true;
        traces[1].visible = true;
        assert_eq!(active_solo_trace_index(&traces), None);

        traces[0].visible = false;
        traces[1].visible = false;
        assert_eq!(active_solo_trace_index(&traces), None);
    }

    #[test]
    fn test_layout_legend_width_tracks_dynamic_policy() {
        let wide = Rect::from_min_size(Pos2::ZERO, Vec2::new(1800.0, 700.0));
        let wide_layout = calculate_layout(wide);
        let wide_legend_width = wide_layout.legend.width();
        assert!(wide_legend_width <= LEGEND_WIDTH_MAX + f32::EPSILON);
        assert!(wide_legend_width >= LEGEND_WIDTH_MIN - f32::EPSILON);

        let narrow = Rect::from_min_size(Pos2::ZERO, Vec2::new(340.0, 260.0));
        let narrow_layout = calculate_layout(narrow);
        let narrow_legend_width = narrow_layout.legend.width();
        assert!(narrow_legend_width <= LEGEND_WIDTH_MAX + f32::EPSILON);
        assert!(narrow_legend_width >= LEGEND_WIDTH_MIN - f32::EPSILON);

        // Width should increase for wider layouts (up to max clamp).
        assert!(wide_legend_width >= narrow_legend_width);
    }

    #[test]
    fn test_legend_inner_rect_uses_tighter_horizontal_inset() {
        let legend = Rect::from_min_size(Pos2::new(10.0, 20.0), Vec2::new(200.0, 300.0));
        let inner = legend_inner_rect(legend);
        assert!((inner.min.x - (legend.min.x + LEGEND_INSET_X)).abs() < f32::EPSILON);
        assert!((inner.max.x - (legend.max.x - LEGEND_INSET_X)).abs() < f32::EPSILON);
        assert!((inner.min.y - (legend.min.y + LEGEND_INSET_Y)).abs() < f32::EPSILON);
        assert!((inner.max.y - (legend.max.y - LEGEND_INSET_Y)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_calculate_legend_trace_row_layout_wide_keeps_all_columns_stable() {
        let layout = calculate_legend_trace_row_layout(180.0, 4.0);
        assert!(layout.show_swatch);
        assert!(layout.show_solo);
        assert!(layout.name_width >= LEGEND_TRACE_LABEL_MIN_WIDTH);
        // 180 - (swatch+spacing=14) - (checkbox+spacing=26) - (solo+spacing=26) = 114
        assert!((layout.name_width - 114.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_calculate_legend_trace_row_layout_narrow_hides_optional_columns() {
        let layout = calculate_legend_trace_row_layout(88.0, 4.0);
        assert!(!layout.show_swatch);
        assert!(!layout.show_solo);
        assert!(layout.name_width >= LEGEND_TRACE_LABEL_MIN_WIDTH);
    }

    #[test]
    fn test_calculate_legend_find_row_layout_hides_clear_when_narrow() {
        let layout = calculate_legend_find_row_layout(48.0, 4.0);
        assert!(!layout.show_clear);
        assert!((layout.edit_width - 48.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_calculate_legend_find_row_layout_shows_clear_when_wide() {
        let layout = calculate_legend_find_row_layout(96.0, 4.0);
        assert!(layout.show_clear);
        assert!(
            (layout.edit_width
                - (96.0 - LEGEND_TRACE_SOLO_WIDTH - 4.0 - LEGEND_FIND_RIGHT_GUARD))
            .abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn test_legend_row_rect_is_exact_width_and_height() {
        let rect = legend_row_rect(12.0, 24.0, 140.0);
        assert!((rect.min.x - 12.0).abs() < f32::EPSILON);
        assert!((rect.min.y - 24.0).abs() < f32::EPSILON);
        assert!((rect.width() - 140.0).abs() < f32::EPSILON);
        assert!((rect.height() - LEGEND_ROW_HEIGHT).abs() < f32::EPSILON);
    }

    #[test]
    fn test_truncate_legend_trace_name_applies_ellipsis_when_needed() {
        let mut truncated = String::new();
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                truncated = truncate_legend_trace_name(
                    ui.painter(),
                    "NET_SUPER_LONG_HIERARCHICAL_NAME_OUT",
                    FontId::proportional(10.0),
                    30.0,
                );
            });
        });
        assert!(truncated.ends_with("..."));
        assert!(truncated.len() < "NET_SUPER_LONG_HIERARCHICAL_NAME_OUT".len());
    }

    #[test]
    fn test_truncate_legend_trace_name_keeps_short_names_intact() {
        let mut rendered = String::new();
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                rendered = truncate_legend_trace_name(
                    ui.painter(),
                    "NET1",
                    FontId::proportional(10.0),
                    120.0,
                );
            });
        });
        assert_eq!(rendered, "NET1");
    }

    #[test]
    fn test_apply_legend_selection_sets_selected_trace_and_highlight() {
        let mut state = WaveformViewerState::new();
        state.traces = vec![
            TraceData::new("NET1", vec![0.0, 1.0], vec![0.0, 1.0]),
            TraceData::new("NET2", vec![0.0, 1.0], vec![1.0, 0.0]),
        ];
        state.traces[0].highlighted = true;

        apply_legend_selection(&mut state, Some("NET2".to_string()));

        assert_eq!(state.selected_trace.as_deref(), Some("NET2"));
        assert!(!state.traces[0].highlighted);
        assert!(state.traces[1].highlighted);
    }

    #[test]
    fn test_axis_positions_follow_fft_lane_model() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
        let layout = calculate_layout(rect);

        let x_tick = x_tick_label_position(&layout, layout.plot.center().x);
        let x_title = x_axis_title_position(&layout);
        assert!(x_tick.y > layout.plot.max.y);
        assert!(x_title.y > x_tick.y);

        let y_tick = y_tick_label_position(&layout, layout.plot.center().y);
        let y_title = y_axis_title_position(&layout, 28.0, 14.0);
        assert!(y_tick.x < layout.plot.min.x);
        assert!(y_title.x < y_tick.x);
    }

    #[test]
    fn test_y_axis_title_position_tracks_y_value_label_width() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 600.0));
        let layout = calculate_layout(rect);
        let narrow = y_axis_title_position(&layout, 14.0, 12.0);
        let wide = y_axis_title_position(&layout, 40.0, 12.0);
        assert!(wide.x < narrow.x);
    }

    #[test]
    fn test_y_axis_title_text_has_no_square_brackets() {
        let mut state = WaveformViewerState::new();
        state.y_axis_unit = "V".to_string();

        let title = y_axis_title_text(&state, "m");
        assert_eq!(title, "mV");
        assert!(!title.contains('['));
        assert!(!title.contains(']'));
    }

    #[test]
    fn test_layout_waveform_cursor_labels_avoids_line_collisions_and_overlap() {
        let layout = calculate_layout(Rect::from_min_size(Pos2::ZERO, Vec2::new(920.0, 520.0)));
        let state = WaveformViewerState::new();
        let requests = vec![
            VerticalLabelRequest {
                anchor_x: layout.plot.center().x - 8.0,
                size: Vec2::new(84.0, 16.0),
            },
            VerticalLabelRequest {
                anchor_x: layout.plot.center().x + 8.0,
                size: Vec2::new(84.0, 16.0),
            },
        ];
        let lines = vec![layout.plot.center().x - 8.0, layout.plot.center().x + 8.0];

        let placements = layout_waveform_cursor_labels(&layout, &state, &requests, &lines);
        assert_eq!(placements.len(), requests.len());
        assert!(!placements[0].rect.intersects(placements[1].rect));
        for placement in &placements {
            for x in &lines {
                assert!(!(*x >= placement.rect.min.x && *x <= placement.rect.max.x));
            }
        }
    }

    #[test]
    fn test_layout_waveform_cursor_labels_moves_below_dense_top_trace_band() {
        let layout = calculate_layout(Rect::from_min_size(Pos2::ZERO, Vec2::new(920.0, 520.0)));
        let mut state = WaveformViewerState::new();
        state.view.x_min = 0.0;
        state.view.x_max = 1.0;
        state.view.y_min = -1.0;
        state.view.y_max = 1.0;

        let mut x = Vec::new();
        let mut y = Vec::new();
        for i in 0..=120 {
            let t = i as f64 / 120.0;
            x.push(t);
            y.push(0.95);
        }
        state.traces.push(TraceData::new("TopBand", x, y));

        let requests = vec![VerticalLabelRequest {
            anchor_x: layout.plot.center().x,
            size: Vec2::new(90.0, 16.0),
        }];
        let lines = vec![layout.plot.center().x];
        let placements = layout_waveform_cursor_labels(&layout, &state, &requests, &lines);

        assert_eq!(placements.len(), 1);
        assert!(placements[0].rect.min.y > layout.plot.min.y + 2.0);
    }
}

