//! Histogram Rendering
//!
//! Commercial-grade egui rendering for histogram visualization.

use egui::{Color32, Painter, Pos2, Rect, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2};

use super::data::Histogram;
use super::state::{AxisScale, DistributionOverlay, HistogramDisplayMode, HistogramState};
use super::statistics::{LogNormalParams, NormalParams};
use crate::common::app::AppState;
use crate::common::viewer_style::viewer_header_bg_color;

// =============================================================================
// Constants
// =============================================================================

fn chart_bg_color() -> Color32 {
    Color32::from_rgb(15, 17, 21)
}

fn grid_color() -> Color32 {
    Color32::from_rgb(40, 45, 55)
}

fn bar_color() -> Color32 {
    Color32::from_rgba_unmultiplied(70, 130, 220, 200)
}

fn bar_outline_color() -> Color32 {
    Color32::from_rgb(100, 150, 230)
}

fn overlay_color() -> Color32 {
    Color32::from_rgb(255, 150, 50)
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

/// Render the histogram viewer panel
pub fn render_histogram_viewer(ui: &mut Ui, app_state: &mut AppState) {
    let available_rect = ui.available_rect_before_wrap();
    let close_requested = {
        let state = &mut app_state.analysis.histogram_state;
        let layout = calculate_layout(available_rect, state.show_stats);
        let close_requested = render_header(ui, &layout, state);
        render_chart_area(ui, &layout, state);
        if state.show_stats {
            render_stats_panel(ui, &layout, state);
        }
        close_requested
    };
    if close_requested {
        app_state.close_active_viewer();
    }
}

/// Public render function for external use
pub fn render_histogram(ui: &mut Ui, state: &HistogramState) {
    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect, state.show_stats);

    render_chart_core(ui, &layout, state);
}

// =============================================================================
// Layout
// =============================================================================

#[derive(Debug, Clone)]
struct HistogramLayout {
    header: Rect,
    chart: Rect,
    stats: Option<Rect>,
}

const HEADER_HEIGHT: f32 = 32.0;
const STATS_WIDTH: f32 = 180.0;
const CHART_PADDING: f32 = 16.0;

fn calculate_layout(available: Rect, show_stats: bool) -> HistogramLayout {
    let total = available;

    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));

    let stats = if show_stats {
        Some(Rect::from_min_size(
            Pos2::new(total.max.x - STATS_WIDTH, header.max.y),
            Vec2::new(STATS_WIDTH, total.height() - HEADER_HEIGHT),
        ))
    } else {
        None
    };

    let chart_right = if show_stats {
        total.max.x - STATS_WIDTH - CHART_PADDING
    } else {
        total.max.x - CHART_PADDING
    };

    let chart = Rect::from_min_max(
        Pos2::new(total.min.x + CHART_PADDING, header.max.y + CHART_PADDING),
        Pos2::new(chart_right, total.max.y - CHART_PADDING),
    );

    HistogramLayout {
        header,
        chart,
        stats,
    }
}

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(ui: &mut Ui, layout: &HistogramLayout, state: &mut HistogramState) -> bool {
    let painter = ui.painter();
    let mut close_requested = false;

    painter.rect_filled(layout.header, Rounding::ZERO, viewer_header_bg_color());

    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.horizontal(|ui| {
            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Histogram")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(16.0);

            // Mode selector
            egui::ComboBox::from_label("")
                .selected_text(state.mode.display_name())
                .show_ui(ui, |ui| {
                    for mode in HistogramDisplayMode::all() {
                        ui.selectable_value(&mut state.mode, *mode, mode.display_name());
                    }
                });

            ui.separator();

            // Scale toggle
            if ui.small_button(state.y_scale.display_name()).clicked() {
                state.toggle_log_scale();
            }

            // Overlay selector
            egui::ComboBox::from_id_salt("overlay")
                .selected_text(state.overlay.display_name())
                .show_ui(ui, |ui| {
                    for overlay in DistributionOverlay::all() {
                        ui.selectable_value(&mut state.overlay, *overlay, overlay.display_name());
                    }
                });

            // Toggle buttons
            if ui
                .small_button(if state.show_stats {
                    "Stats âœ“"
                } else {
                    "Stats"
                })
                .clicked()
            {
                state.toggle_stats();
            }

            if ui
                .small_button(if state.show_grid {
                    "Grid âœ“"
                } else {
                    "Grid"
                })
                .clicked()
            {
                state.toggle_grid();
            }

            // Right-aligned close button
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                if ui.small_button("âœ•").clicked() {
                    close_requested = true;
                }
            });
        });
    });
    close_requested
}

// =============================================================================
// Chart Rendering
// =============================================================================

fn render_chart_area(ui: &mut Ui, layout: &HistogramLayout, state: &HistogramState) {
    render_chart_core(ui, layout, state);
    let _response = ui.allocate_rect(layout.chart, Sense::click());
}

fn render_chart_core(ui: &mut Ui, layout: &HistogramLayout, state: &HistogramState) {
    let painter = ui.painter();
    let rect = layout.chart;

    // Background
    painter.rect_filled(rect, Rounding::ZERO, chart_bg_color());

    // Grid
    if state.show_grid {
        render_grid(painter, rect);
    }

    // Bars
    if let Some(hist) = state.current_histogram() {
        render_histogram_bars(painter, rect, hist, state);

        // Distribution overlay
        if state.overlay != DistributionOverlay::None {
            render_distribution_overlay(painter, rect, hist, state);
        }
    }

    // Axes
    render_axes(painter, rect, state);

    // Border
    painter.rect_stroke(
        rect,
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
    );
}

fn render_grid(painter: &Painter, rect: Rect) {
    let stroke = Stroke::new(0.5, grid_color());

    // Vertical lines
    let x_divisions = 10;
    for i in 1..x_divisions {
        let x = rect.min.x + (i as f32 / x_divisions as f32) * rect.width();
        painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], stroke);
    }

    // Horizontal lines
    let y_divisions = 5;
    for i in 1..y_divisions {
        let y = rect.min.y + (i as f32 / y_divisions as f32) * rect.height();
        painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], stroke);
    }
}

fn render_histogram_bars(painter: &Painter, rect: Rect, hist: &Histogram, state: &HistogramState) {
    let n = hist.bin_count();
    if n == 0 {
        return;
    }

    let (range_min, range_max) = hist.range();
    let range = range_max - range_min;
    if range <= 0.0 {
        return;
    }

    // Get values based on display mode
    let values: Vec<f64> = match state.mode {
        HistogramDisplayMode::Count => hist.bins.iter().map(|b| b.count as f64).collect(),
        HistogramDisplayMode::Pdf => hist.pdf(),
        HistogramDisplayMode::Cdf => hist.cdf(),
        HistogramDisplayMode::Percent => {
            let total = hist.total_count as f64;
            hist.bins
                .iter()
                .map(|b| {
                    if total > 0.0 {
                        b.count as f64 / total * 100.0
                    } else {
                        0.0
                    }
                })
                .collect()
        }
    };

    let max_value = values.iter().copied().fold(0.0_f64, f64::max);
    if max_value <= 0.0 {
        return;
    }

    let bar_width = rect.width() / n as f32;
    let gap = 1.0; // Pixel gap between bars

    for (i, &value) in values.iter().enumerate() {
        let height_ratio = if state.y_scale == AxisScale::Log {
            if value > 0.0 && max_value > 0.0 {
                (1.0 + value.log10()) / (1.0 + max_value.log10())
            } else {
                0.0
            }
        } else {
            value / max_value
        };

        let bar_height = height_ratio as f32 * (rect.height() - 20.0);
        if bar_height < 1.0 {
            continue;
        }

        let x = rect.min.x + i as f32 * bar_width + gap;
        let y = rect.max.y - 15.0 - bar_height;
        let bar_rect = Rect::from_min_size(
            Pos2::new(x, y),
            Vec2::new((bar_width - gap * 2.0).max(1.0), bar_height),
        );

        painter.rect_filled(bar_rect, Rounding::ZERO, bar_color());
        painter.rect_stroke(
            bar_rect,
            Rounding::ZERO,
            Stroke::new(0.5, bar_outline_color()),
        );
    }
}

fn render_distribution_overlay(
    painter: &Painter,
    rect: Rect,
    hist: &Histogram,
    state: &HistogramState,
) {
    if state.mode == HistogramDisplayMode::Cdf {
        return; // Skip overlay for CDF mode
    }

    match state.overlay {
        DistributionOverlay::Normal => {
            let params = NormalParams::fit(hist);
            render_normal_curve(painter, rect, hist, &params, state);
        }
        DistributionOverlay::LogNormal => {
            if let Some(params) = LogNormalParams::fit(hist) {
                render_lognormal_curve(painter, rect, hist, &params, state);
            }
        }
        DistributionOverlay::None => {}
    }
}

fn render_normal_curve(
    painter: &Painter,
    rect: Rect,
    hist: &Histogram,
    params: &NormalParams,
    _state: &HistogramState,
) {
    let (range_min, range_max) = hist.range();
    let n_points = 100;

    let mut points = Vec::with_capacity(n_points);

    // Scale factor to match histogram height
    let max_pdf = params.pdf(params.mean);
    let max_count = hist.max_count() as f64;
    let scale = if max_pdf > 0.0 {
        max_count / max_pdf
    } else {
        1.0
    };

    for i in 0..n_points {
        let t = i as f64 / (n_points - 1) as f64;
        let x = range_min + t * (range_max - range_min);
        let y = params.pdf(x) * scale;

        let screen_x = rect.min.x + t as f32 * rect.width();
        let height_ratio = if max_count > 0.0 { y / max_count } else { 0.0 };
        let screen_y = rect.max.y - 15.0 - height_ratio as f32 * (rect.height() - 20.0);

        if screen_y >= rect.min.y && screen_y <= rect.max.y {
            points.push(Pos2::new(screen_x, screen_y));
        }
    }

    // Draw as connected line segments
    for window in points.windows(2) {
        painter.line_segment([window[0], window[1]], Stroke::new(2.0, overlay_color()));
    }
}

fn render_lognormal_curve(
    painter: &Painter,
    rect: Rect,
    hist: &Histogram,
    params: &LogNormalParams,
    _state: &HistogramState,
) {
    let (range_min, range_max) = hist.range();
    let n_points = 100;

    // Skip if data range includes non-positive values (log-normal undefined)
    if range_min <= 0.0 {
        return;
    }

    let mut points = Vec::with_capacity(n_points);

    // Scale factor to match histogram height
    // Mode of log-normal is exp(mu - sigma^2)
    let mode = (params.mu - params.sigma * params.sigma).exp();
    let max_pdf = params.pdf(mode);
    let max_count = hist.max_count() as f64;
    let scale = if max_pdf > 0.0 {
        max_count / max_pdf
    } else {
        1.0
    };

    for i in 0..n_points {
        let t = i as f64 / (n_points - 1) as f64;
        let x = range_min + t * (range_max - range_min);
        let y = params.pdf(x) * scale;

        let screen_x = rect.min.x + t as f32 * rect.width();
        let height_ratio = if max_count > 0.0 { y / max_count } else { 0.0 };
        let screen_y = rect.max.y - 15.0 - height_ratio as f32 * (rect.height() - 20.0);

        if screen_y >= rect.min.y && screen_y <= rect.max.y {
            points.push(Pos2::new(screen_x, screen_y));
        }
    }

    // Draw as connected line segments (using distinct color for log-normal)
    let lognormal_color = Color32::from_rgb(255, 100, 100); // Red-ish for log-normal
    for window in points.windows(2) {
        painter.line_segment([window[0], window[1]], Stroke::new(2.0, lognormal_color));
    }
}

fn render_axes(painter: &Painter, rect: Rect, _state: &HistogramState) {
    let stroke = Stroke::new(1.0, axis_color());

    // X-axis
    let x_axis_y = rect.max.y - 15.0;
    painter.line_segment(
        [
            Pos2::new(rect.min.x, x_axis_y),
            Pos2::new(rect.max.x, x_axis_y),
        ],
        stroke,
    );

    // Y-axis
    painter.line_segment(
        [
            Pos2::new(rect.min.x, rect.min.y),
            Pos2::new(rect.min.x, x_axis_y),
        ],
        stroke,
    );
}

// =============================================================================
// Stats Panel
// =============================================================================

fn render_stats_panel(ui: &mut Ui, layout: &HistogramLayout, state: &HistogramState) {
    let Some(stats_rect) = layout.stats else {
        return;
    };

    let painter = ui.painter();
    painter.rect_filled(stats_rect, Rounding::ZERO, Color32::from_rgb(25, 27, 33));

    let panel_rect = stats_rect.shrink(8.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Statistics")
                    .size(11.0)
                    .color(text_color()),
            );
            ui.add_space(8.0);

            if let Some(stats) = state.current_stats() {
                stat_row(ui, "Count", &format!("{}", stats.count));
                stat_row(ui, "Mean", &stats.format_mean());
                stat_row(ui, "Std Dev", &stats.format_std_dev());

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                stat_row(ui, "Min", &format!("{:.4}", stats.min));
                stat_row(ui, "Max", &format!("{:.4}", stats.max));
                stat_row(ui, "Median", &format!("{:.4}", stats.median));

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                stat_row(ui, "Q1", &format!("{:.4}", stats.q1));
                stat_row(ui, "Q3", &format!("{:.4}", stats.q3));
                stat_row(ui, "IQR", &format!("{:.4}", stats.iqr));

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                stat_row(ui, "Skewness", &format!("{:.3}", stats.skewness));
                stat_row(ui, "Kurtosis", &format!("{:.3}", stats.kurtosis));
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

fn stat_row(ui: &mut Ui, label: &str, value: &str) {
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

// =============================================================================
// Tests
// =============================================================================
