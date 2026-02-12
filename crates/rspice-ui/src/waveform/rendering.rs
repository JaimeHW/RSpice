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
use super::state::{TraceData, ViewTransform, WaveformViewerState};
use crate::common::app::AppState;

// =============================================================================
// Constants
// =============================================================================

/// Margin for Y-axis labels (pixels)
const Y_AXIS_WIDTH: f32 = 60.0;

/// Margin for X-axis labels (pixels)
const X_AXIS_HEIGHT: f32 = 30.0;

/// Header height (pixels)
const HEADER_HEIGHT: f32 = 32.0;

/// Legend width (pixels)
const LEGEND_WIDTH: f32 = 120.0;

/// Maximum points to render before decimation
const DECIMATION_THRESHOLD: usize = 2000;

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

// Box selection colors
fn box_select_fill() -> Color32 {
    Color32::from_rgba_unmultiplied(59, 130, 246, 50)
}
fn box_select_stroke() -> Color32 {
    Color32::from_rgb(59, 130, 246)
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

    // Header at top
    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));

    // Legend on right side (below header)
    let legend = Rect::from_min_size(
        Pos2::new(total.max.x - LEGEND_WIDTH, header.max.y),
        Vec2::new(LEGEND_WIDTH, total.height() - HEADER_HEIGHT),
    );

    // X-axis at bottom (excluding legend)
    let x_axis = Rect::from_min_size(
        Pos2::new(total.min.x + Y_AXIS_WIDTH, total.max.y - X_AXIS_HEIGHT),
        Vec2::new(total.width() - Y_AXIS_WIDTH - LEGEND_WIDTH, X_AXIS_HEIGHT),
    );

    // Y-axis on left side (between header and x-axis)
    let y_axis = Rect::from_min_size(
        Pos2::new(total.min.x, header.max.y),
        Vec2::new(Y_AXIS_WIDTH, total.height() - HEADER_HEIGHT - X_AXIS_HEIGHT),
    );

    // Plot area in the center
    let plot = Rect::from_min_size(
        Pos2::new(total.min.x + Y_AXIS_WIDTH, header.max.y),
        Vec2::new(
            total.width() - Y_AXIS_WIDTH - LEGEND_WIDTH,
            total.height() - HEADER_HEIGHT - X_AXIS_HEIGHT,
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

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(ui: &mut Ui, layout: &ViewerLayout, viewer_state: &mut WaveformViewerState) {
    let painter = ui.painter();

    // Header background
    painter.rect_filled(layout.header, Rounding::ZERO, Color32::from_rgb(30, 33, 40));

    // Create a UI area for header controls
    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        let control_height = (ui.available_height() - 2.0)
            .max(ui.spacing().interact_size.y)
            .max(18.0);
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), ui.available_height()),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.add_space(8.0);

                // Title
                let (title_rect, _title_response) = ui.allocate_exact_size(
                    egui::vec2(128.0, control_height),
                    egui::Sense::hover(),
                );
                ui.painter().text(
                    title_rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    "Waveform Viewer",
                    FontId::proportional(13.0),
                    Color32::from_rgb(200, 200, 210),
                );

                ui.add_space(16.0);

                // Fit buttons
                if ui
                    .add_sized(
                        egui::vec2(58.0, control_height),
                        egui::Button::new("Fit All"),
                    )
                    .clicked()
                {
                    viewer_state.view.fit_to_traces(&viewer_state.traces);
                }
                if ui
                    .add_sized(egui::vec2(48.0, control_height), egui::Button::new("Fit X"))
                    .clicked()
                {
                    viewer_state.view.fit_x_to_traces(&viewer_state.traces);
                }
                if ui
                    .add_sized(egui::vec2(48.0, control_height), egui::Button::new("Fit Y"))
                    .clicked()
                {
                    viewer_state.view.fit_y_to_traces(&viewer_state.traces);
                }

                ui.separator();

                // Zoom buttons
                if ui
                    .add_sized(egui::vec2(28.0, control_height), egui::Button::new("-"))
                    .clicked()
                {
                    viewer_state.view.zoom(1.25, 0.5, 0.5);
                }
                if ui
                    .add_sized(egui::vec2(28.0, control_height), egui::Button::new("+"))
                    .clicked()
                {
                    viewer_state.view.zoom(0.8, 0.5, 0.5);
                }

                ui.separator();

                // Cursor clear
                if ui
                    .add_sized(
                        egui::vec2(94.0, control_height),
                        egui::Button::new("Clear Cursors"),
                    )
                    .clicked()
                {
                    viewer_state.cursors.clear();
                }

                // Right-aligned controls
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);

                    // Toggle buttons
                    let meas_text = if viewer_state.show_measurements {
                        "Meas On"
                    } else {
                        "Meas Off"
                    };
                    if ui
                        .add_sized(egui::vec2(68.0, control_height), egui::Button::new(meas_text))
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
                        .add_sized(
                            egui::vec2(76.0, control_height),
                            egui::Button::new(export_text),
                        )
                        .on_hover_text("Export")
                        .clicked()
                    {
                        viewer_state.show_export = !viewer_state.show_export;
                    }
                });
            },
        );
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

    for (i, &tick) in y_ticks.major_ticks.iter().enumerate() {
        let y_frac = (viewer_state.view.y_max - tick) / viewer_state.view.y_range();
        let screen_y = layout.y_axis.min.y + y_frac as f32 * layout.y_axis.height();

        if let Some(label) = labels.get(i) {
            let text_pos = Pos2::new(layout.y_axis.max.x - 4.0, screen_y);
            painter.text(
                text_pos,
                egui::Align2::RIGHT_CENTER,
                label,
                font.clone(),
                text_color,
            );
        }
    }

    // Y-axis unit label at top-left corner (compact format like professional tools)
    let unit = if viewer_state.y_axis_unit.is_empty() {
        "V"
    } else {
        &viewer_state.y_axis_unit
    };
    let unit_label = format!("[{}{}]", y_ticks.prefix, unit);
    painter.text(
        Pos2::new(layout.y_axis.min.x + 2.0, layout.y_axis.min.y + 2.0),
        egui::Align2::LEFT_TOP,
        &unit_label,
        FontId::proportional(9.0),
        Color32::from_rgb(130, 135, 145),
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
            let text_pos = Pos2::new(screen_x, layout.x_axis.min.y + 8.0);
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
        Pos2::new(layout.x_axis.center().x, layout.x_axis.max.y - 4.0),
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
    let painter = ui.painter();

    // Update view dimensions
    viewer_state.view.plot_width = layout.plot.width() as f64;
    viewer_state.view.plot_height = layout.plot.height() as f64;

    // Background
    painter.rect_filled(layout.plot, Rounding::ZERO, Color32::from_rgb(18, 20, 24));

    // Clip to plot area for all subsequent rendering
    let clip_rect = layout.plot;

    // Grid lines
    render_grid(painter, layout, viewer_state, clip_rect);

    // Spec overlays (render before traces so they appear as background)
    for overlay in viewer_state.spec_overlays.iter().filter(|o| o.visible) {
        overlay.render(
            painter,
            layout.plot,
            viewer_state.view.y_min,
            viewer_state.view.y_max,
        );
    }

    // Waveform traces
    for trace in viewer_state.traces.iter().filter(|t| t.visible) {
        render_trace(painter, layout, viewer_state, trace, clip_rect);
    }

    // Cursors
    if viewer_state.cursors.is_active() {
        render_cursors(painter, layout, viewer_state, clip_rect);
    }

    // Box selection overlay
    if viewer_state.box_selection.is_selecting {
        render_box_selection(painter, layout, viewer_state, clip_rect);
    }

    // Handle mouse interactions
    let response = ui.allocate_rect(layout.plot, Sense::click_and_drag());
    handle_plot_interactions(response, layout, viewer_state);
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
    let n = trace.len();

    // Decimate if too many points
    let step = if n > DECIMATION_THRESHOLD {
        (n / DECIMATION_THRESHOLD).max(1)
    } else {
        1
    };

    // Build screen coordinate points
    let mut points: Vec<Pos2> = Vec::with_capacity(n / step + 1);

    for i in (0..n).step_by(step) {
        let data_x = trace.x[i];
        let data_y = trace.y[i];

        // Skip invalid values
        if !data_x.is_finite() || !data_y.is_finite() {
            continue;
        }

        let screen_x = layout.plot.min.x
            + ((data_x - view.x_min) / view.x_range()) as f32 * layout.plot.width();
        let screen_y = layout.plot.min.y
            + ((view.y_max - data_y) / view.y_range()) as f32 * layout.plot.height();

        points.push(Pos2::new(screen_x, screen_y));
    }

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
    if trace.style.show_markers && n <= 200 {
        for point in &points {
            painter.circle_filled(*point, trace.style.marker_size / 2.0, color);
        }
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

        // Label at top
        painter.text(
            Pos2::new(screen_x, layout.plot.min.y + 4.0),
            egui::Align2::CENTER_TOP,
            "C1",
            FontId::proportional(10.0),
            cursor1_color(),
        );

        // Time readout
        let time_str = axis::format_time(x1);
        painter.text(
            Pos2::new(screen_x + 4.0, layout.plot.min.y + 18.0),
            egui::Align2::LEFT_TOP,
            time_str,
            FontId::proportional(9.0),
            cursor1_color(),
        );
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

        painter.text(
            Pos2::new(screen_x, layout.plot.min.y + 4.0),
            egui::Align2::CENTER_TOP,
            "C2",
            FontId::proportional(10.0),
            cursor2_color(),
        );

        let time_str = axis::format_time(x2);
        painter.text(
            Pos2::new(screen_x + 4.0, layout.plot.min.y + 18.0),
            egui::Align2::LEFT_TOP,
            time_str,
            FontId::proportional(9.0),
            cursor2_color(),
        );
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

    // Click to place cursor
    if response.clicked() && !viewer_state.view.did_drag {
        if let Some(pos) = response.hover_pos() {
            let x_frac = (pos.x - layout.plot.min.x) / layout.plot.width();
            let data_x = viewer_state.view.x_min + x_frac as f64 * viewer_state.view.x_range();
            viewer_state.cursors.place(data_x);
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

    // Collect clicked trace indices (can't mutate during immutable iteration)
    let mut clicked_indices: Vec<usize> = Vec::new();

    // Create UI area for legend items
    let legend_inner = layout.legend.shrink(8.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(legend_inner), |ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Traces")
                    .size(11.0)
                    .strong()
                    .color(Color32::from_rgb(160, 165, 175)),
            );

            ui.add_space(4.0);

            // Render each trace as a legend item
            for (i, trace) in viewer_state.traces.iter().enumerate() {
                let color = trace.style.to_color32();
                let text_color = if trace.visible {
                    Color32::from_rgb(200, 200, 210)
                } else {
                    Color32::from_rgb(100, 105, 115)
                };

                ui.horizontal(|ui| {
                    // Color swatch
                    let swatch_rect = ui.allocate_space(Vec2::new(12.0, 12.0)).1;
                    if trace.visible {
                        ui.painter()
                            .rect_filled(swatch_rect, Rounding::same(2.0), color);
                    } else {
                        ui.painter().rect_stroke(
                            swatch_rect,
                            Rounding::same(2.0),
                            Stroke::new(1.0, color),
                        );
                    }

                    // Trace name (clickable to toggle visibility)
                    let name_response = ui.add(
                        egui::Label::new(
                            egui::RichText::new(&trace.name)
                                .size(10.0)
                                .color(text_color),
                        )
                        .sense(Sense::click()),
                    );

                    if name_response.clicked() {
                        clicked_indices.push(i);
                    }

                    if name_response.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                });
            }

            // If no traces, show placeholder
            if viewer_state.traces.is_empty() {
                ui.label(
                    egui::RichText::new("No waveforms")
                        .size(10.0)
                        .color(Color32::from_rgb(100, 105, 115)),
                );
            }
        });
    });

    // Apply visibility toggles after the iteration
    for idx in clicked_indices {
        if let Some(trace) = viewer_state.traces.get_mut(idx) {
            trace.visible = !trace.visible;
            log::info!(
                "Toggled trace '{}' visibility to {}",
                trace.name,
                trace.visible
            );
        }
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
    fn test_decimation_threshold() {
        // A trace with many points should decimate
        let n = 10000;
        let step = if n > DECIMATION_THRESHOLD {
            (n / DECIMATION_THRESHOLD).max(1)
        } else {
            1
        };

        assert!(step > 1, "Large traces should be decimated");
        assert!(
            n / step <= DECIMATION_THRESHOLD * 2,
            "Decimation should stay near threshold"
        );
    }

    #[test]
    fn test_decimation_small_trace() {
        // A small trace should not decimate
        let n = 500;
        let step = if n > DECIMATION_THRESHOLD {
            (n / DECIMATION_THRESHOLD).max(1)
        } else {
            1
        };

        assert_eq!(step, 1, "Small traces should not decimate");
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
        assert!(LEGEND_WIDTH > 0.0);
    }
}
