use super::super::*;
use super::measurements::measurement_row;

pub(super) fn render_export_panel(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
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
    ) && end < start
    {
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
