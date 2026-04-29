use super::super::formatting::{format_optional_freq, format_optional_time, format_optional_value};
use super::super::*;

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

pub(super) fn measurement_row(ui: &mut Ui, label: &str, value: &str) {
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

pub(super) fn render_measurements_panel(ui: &mut Ui, viewer_state: &mut WaveformViewerState) {
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
