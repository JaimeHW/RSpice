use super::*;

pub(super) fn render_box_selection(
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

pub(super) fn handle_plot_interactions(
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
    if response.clicked()
        && !viewer_state.view.did_drag
        && let Some(pos) = response.hover_pos()
    {
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
        && let Some(pos) = response.hover_pos()
    {
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
            && let Some((x_min, x_max, y_min, y_max)) = viewer_state.box_selection.finish()
        {
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
