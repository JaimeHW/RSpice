use super::*;

mod export;
mod markers;
mod measurements;
mod trace_list;

// =============================================================================
// Legend Rendering
// =============================================================================

pub(super) fn render_legend(
    ui: &mut Ui,
    layout: &ViewerLayout,
    viewer_state: &mut WaveformViewerState,
) {
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
                trace_list::render_trace_list_section(ui, viewer_state);
                ui.add_space(LEGEND_SECTION_SPACING);
                markers::render_markers_panel(ui, viewer_state);
                if viewer_state.show_measurements {
                    ui.add_space(LEGEND_SECTION_SPACING);
                    measurements::render_measurements_panel(ui, viewer_state);
                }
                if viewer_state.show_export {
                    ui.add_space(LEGEND_SECTION_SPACING);
                    export::render_export_panel(ui, viewer_state);
                }
            });
    });
}

fn legend_inner_rect(legend_rect: Rect) -> Rect {
    legend_rect.shrink2(Vec2::new(LEGEND_INSET_X, LEGEND_INSET_Y))
}

pub(super) fn handle_waveform_right_pane_splitter(
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
