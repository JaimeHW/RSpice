use super::super::*;

pub(super) fn render_markers_panel(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
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
