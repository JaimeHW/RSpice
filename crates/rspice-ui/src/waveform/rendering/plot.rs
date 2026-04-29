use super::*;
use super::{
    cursors::render_cursors,
    interactions::{handle_plot_interactions, render_box_selection},
    traces::render_trace,
};

// =============================================================================
// Plot Area Rendering
// =============================================================================

pub(super) fn render_plot_area(
    ui: &mut Ui,
    layout: &ViewerLayout,
    viewer_state: &mut WaveformViewerState,
) {
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
