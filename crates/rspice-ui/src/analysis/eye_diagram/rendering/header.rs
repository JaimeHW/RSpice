use egui::{Color32, Rounding, Ui, UiBuilder};

use super::super::state::{ColorMap, EyeDiagramState, EyeDisplayMode};
use super::layout::EyeLayout;
use crate::common::viewer_style::viewer_header_bg_color;

pub(super) fn render_header(ui: &mut Ui, layout: &EyeLayout, state: &mut EyeDiagramState) -> bool {
    let painter = ui.painter();
    let mut close_requested = false;

    painter.rect_filled(layout.header, Rounding::ZERO, viewer_header_bg_color());

    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().interact_size.y = 24.0;
            ui.spacing_mut().button_padding.y = 2.0;

            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Eye Diagram")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(16.0);

            let mut mode = state.mode;
            egui::ComboBox::from_id_salt("eye_mode")
                .width(112.0)
                .selected_text(mode.display_name())
                .show_ui(ui, |ui| {
                    for candidate in EyeDisplayMode::all() {
                        ui.selectable_value(&mut mode, *candidate, candidate.display_name());
                    }
                });
            if mode != state.mode {
                state.set_mode(mode);
            }

            let mut color_map = state.color_map;
            ui.add_enabled_ui(state.mode == EyeDisplayMode::Persistence, |ui| {
                egui::ComboBox::from_id_salt("eye_colormap")
                    .width(96.0)
                    .selected_text(color_map.display_name())
                    .show_ui(ui, |ui| {
                        for map in ColorMap::all() {
                            ui.selectable_value(&mut color_map, *map, map.display_name());
                        }
                    });
            });
            state.color_map = color_map;

            if state.mode == EyeDisplayMode::SingleTrace {
                let mut selected = state.selected_trace;
                let selected_text = selected
                    .map(|idx| format!("Trace {}", idx + 1))
                    .unwrap_or_else(|| "Auto".to_string());
                egui::ComboBox::from_id_salt("eye_trace_select")
                    .width(112.0)
                    .selected_text(selected_text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected, None, "Auto");
                        for idx in 0..state.data.traces.len() {
                            ui.selectable_value(
                                &mut selected,
                                Some(idx),
                                format!("Trace {}", idx + 1),
                            );
                        }
                    });
                if selected != state.selected_trace {
                    state.select_trace(selected);
                }
            }

            ui.separator();

            if ui.small_button("Fit").clicked() {
                state.reset_view_to_data();
                state.invalidate_persistence_cache();
            }

            let mut h_scale = state.h_scale;
            let h_resp = ui.add(
                egui::DragValue::new(&mut h_scale)
                    .speed((state.h_scale.abs() * 0.05).max(1e-18))
                    .range(1e-18..=1e9)
                    .suffix(" s/div"),
            );
            if h_resp.changed() {
                state.h_scale = h_scale.max(1e-18);
                state.apply_scale_controls();
                state.invalidate_persistence_cache();
            }

            let mut v_scale = state.v_scale;
            let v_resp = ui.add(
                egui::DragValue::new(&mut v_scale)
                    .speed((state.v_scale.abs() * 0.05).max(1e-9))
                    .range(1e-9..=1e9)
                    .suffix(" V/div"),
            );
            if v_resp.changed() {
                state.v_scale = v_scale.max(1e-9);
                state.apply_scale_controls();
                state.invalidate_persistence_cache();
            }

            let mut decay = state.persistence_decay;
            let decay_resp = ui.add(
                egui::Slider::new(&mut decay, 0.50..=0.999)
                    .text("Decay")
                    .clamping(egui::SliderClamping::Always),
            );
            if decay_resp.changed() {
                state.persistence_decay = decay;
                state.invalidate_persistence_cache();
            }

            ui.separator();

            if ui
                .small_button(if state.show_measurements {
                    "Meas [on]"
                } else {
                    "Meas"
                })
                .clicked()
            {
                state.toggle_measurements();
            }

            if ui
                .small_button(if state.show_mask { "Mask [on]" } else { "Mask" })
                .clicked()
            {
                state.toggle_mask();
            }

            if ui
                .small_button(if state.show_grid { "Grid [on]" } else { "Grid" })
                .clicked()
            {
                state.show_grid = !state.show_grid;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                if ui.small_button("Close").clicked() {
                    close_requested = true;
                }
            });
        });
    });
    close_requested
}

// =============================================================================
