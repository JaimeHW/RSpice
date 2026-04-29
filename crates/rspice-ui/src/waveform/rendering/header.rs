use super::*;

// =============================================================================
// Header Rendering
// =============================================================================

pub(super) fn render_header(
    ui: &mut Ui,
    layout: &ViewerLayout,
    viewer_state: &mut WaveformViewerState,
) {
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
