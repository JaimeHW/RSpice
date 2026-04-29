use egui::{Color32, Pos2, Rect, Rounding, Stroke, Ui, UiBuilder};

use super::super::state::EyeDiagramState;
use super::layout::{AXIS_TOP_GUTTER, EyeLayout, MEASUREMENTS_PANEL_PADDING};
use super::style::{panel_bg_color, panel_border_color, text_color};

pub(super) fn render_measurements_panel(
    ui: &mut Ui,
    layout: &EyeLayout,
    state: &mut EyeDiagramState,
) {
    let Some(measurements_rect) = layout.measurements else {
        return;
    };
    let painter = ui.painter();

    painter.rect_filled(measurements_rect, Rounding::ZERO, panel_bg_color());
    if let Some(outline_rect) = measurements_outline_rect(measurements_rect) {
        painter.rect_stroke(
            outline_rect,
            Rounding::ZERO,
            Stroke::new(1.0, panel_border_color()),
        );
    }

    let panel_rect = measurements_rect.shrink(MEASUREMENTS_PANEL_PADDING);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        egui::ScrollArea::vertical()
            .id_salt("eye_measurements_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Measurements")
                            .size(11.0)
                            .color(text_color()),
                    );
                    ui.add_space(8.0);

                    if state.data.trace_count() == 0 {
                        ui.label(
                            egui::RichText::new("No data")
                                .size(10.0)
                                .color(Color32::from_rgb(100, 105, 115)),
                        );
                        return;
                    }

                    let m = &state.measurements;

                    measurement_row(ui, "Data Rate", &m.format_data_rate());
                    measurement_row(ui, "UI", &format!("{:.2} ps", m.unit_interval * 1e12));

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    measurement_row(ui, "Eye Height", &m.format_height());
                    measurement_row(ui, "Eye Width", &m.format_width());

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    measurement_row(ui, "Jitter (p-p)", &m.format_jitter());
                    measurement_row(ui, "Rise Time", &m.format_rise_time());
                    measurement_row(ui, "Fall Time", &m.format_fall_time());

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    measurement_row(ui, "Q-Factor", &format!("{:.2}", m.q_factor));
                    measurement_row(ui, "Est. BER", &m.format_ber());
                    measurement_row(ui, "SNR", &format!("{:.1} dB", m.snr_db));

                    if state.show_mask {
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(4.0);

                        let result = state.mask_result_string();
                        let color = if state.mask.is_passing() {
                            Color32::from_rgb(100, 200, 100)
                        } else {
                            Color32::from_rgb(255, 100, 100)
                        };

                        ui.label(
                            egui::RichText::new(format!("Mask: {}", result))
                                .size(11.0)
                                .color(color),
                        );
                    }

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);
                    render_cursor_marker_manager(ui, state);
                });
            });
    });
}

pub(super) fn measurements_outline_rect(measurements_rect: Rect) -> Option<Rect> {
    let top = (measurements_rect.min.y + AXIS_TOP_GUTTER).min(measurements_rect.max.y);
    if top >= measurements_rect.max.y {
        return None;
    }
    Some(Rect::from_min_max(
        Pos2::new(measurements_rect.min.x, top),
        measurements_rect.max,
    ))
}

pub(super) fn measurement_row(ui: &mut Ui, label: &str, value: &str) {
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

pub(super) fn render_cursor_marker_manager(ui: &mut Ui, state: &mut EyeDiagramState) {
    ui.label(
        egui::RichText::new("Cursors / Markers")
            .size(11.0)
            .color(text_color()),
    );
    ui.add_space(4.0);

    let c1 = state
        .cursors
        .cursor1_time_s
        .map(crate::waveform::axis::format_time)
        .unwrap_or_else(|| "â€”".to_string());
    let c2 = state
        .cursors
        .cursor2_time_s
        .map(crate::waveform::axis::format_time)
        .unwrap_or_else(|| "â€”".to_string());
    measurement_row(ui, "C1", &c1);
    measurement_row(ui, "C2", &c2);
    if let Some(dt) = state.cursors.delta_time() {
        measurement_row(ui, "Î”T", &crate::waveform::axis::format_time(dt));
    } else {
        measurement_row(ui, "Î”T", "â€”");
    }

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        if ui.small_button("Clear Cursors").clicked() {
            state.cursors.clear();
        }
        if ui.small_button("Clear Markers").clicked() {
            state.clear_markers();
        }
    });

    if state.markers.is_empty() {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("No markers")
                .size(10.0)
                .color(Color32::from_rgb(100, 105, 115)),
        );
        return;
    }

    ui.add_space(4.0);
    let mut remove_idx: Option<usize> = None;
    for (idx, marker) in state.markers.iter().copied().enumerate() {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("M{}:", idx + 1))
                    .size(10.0)
                    .color(Color32::from_rgb(120, 125, 135)),
            );
            ui.label(
                egui::RichText::new(crate::waveform::axis::format_time(marker))
                    .size(11.0)
                    .color(Color32::from_rgb(200, 205, 215)),
            );
            if ui.small_button("x").clicked() {
                remove_idx = Some(idx);
            }
        });
    }

    if let Some(idx) = remove_idx {
        state.remove_marker_at(idx);
    }
}

// =============================================================================
