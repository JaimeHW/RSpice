use egui::{Color32, RichText, Ui};

use super::data::format_engineering_for_spice;
use super::state::PwlEditorState;

/// Result of PWL editor interaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PwlEditorResult {
    /// No change.
    None,
    /// Data was modified.
    Modified,
}

/// Render the PWL editor widget.
///
/// Returns `PwlEditorResult::Modified` if the data changed.
pub fn render_pwl_editor(ui: &mut Ui, state: &mut PwlEditorState) -> PwlEditorResult {
    let mut result = PwlEditorResult::None;

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("PWL Points").strong());
            ui.separator();
            if ui.button("➕ Add Point").clicked() {
                state.adding_point = true;
            }
            if state.selected_row.is_some() && ui.button("🗑 Delete").clicked() {
                state.delete_selected();
                result = PwlEditorResult::Modified;
            }
        });

        ui.separator();

        if state.adding_point {
            render_new_point_row(ui, state, &mut result);
            ui.separator();
        }

        render_table_header(ui, state);
        ui.separator();
        render_points_table(ui, state, &mut result);
        render_validation_error(ui, state);
        render_summary(ui, state);
    });

    result
}

fn render_new_point_row(ui: &mut Ui, state: &mut PwlEditorState, result: &mut PwlEditorResult) {
    ui.horizontal(|ui| {
        ui.label("Time:");
        ui.add(egui::TextEdit::singleline(&mut state.new_time).desired_width(80.0));
        ui.label("s");
        ui.separator();
        ui.label("Value:");
        ui.add(egui::TextEdit::singleline(&mut state.new_value).desired_width(80.0));
        ui.label(&state.value_unit);

        if ui.button("✓ Add").clicked() {
            state.add_point();
            *result = PwlEditorResult::Modified;
        }
        if ui.button("✗ Cancel").clicked() {
            state.adding_point = false;
            state.new_time.clear();
            state.new_value.clear();
        }
    });
}

fn render_table_header(ui: &mut Ui, state: &PwlEditorState) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("#").monospace().weak());
        ui.add_space(20.0);
        ui.label(RichText::new("Time (s)").monospace());
        ui.add_space(60.0);
        ui.label(RichText::new(format!("Value ({})", state.value_unit)).monospace());
    });
}

fn render_points_table(ui: &mut Ui, state: &mut PwlEditorState, result: &mut PwlEditorResult) {
    egui::ScrollArea::vertical()
        .max_height(200.0)
        .show(ui, |ui| {
            let mut data_changed = false;

            for i in 0..state.edit_buffers.len() {
                let is_selected = state.selected_row == Some(i);
                let row_response = ui.horizontal(|ui| {
                    let num_text = RichText::new(format!("{}", i + 1)).monospace();
                    if is_selected {
                        ui.label(num_text.color(Color32::YELLOW));
                    } else {
                        ui.label(num_text.weak());
                    }

                    ui.add_space(10.0);

                    let time_response = ui.add(
                        egui::TextEdit::singleline(&mut state.edit_buffers[i].0)
                            .desired_width(80.0)
                            .font(egui::TextStyle::Monospace),
                    );
                    if time_response.changed() {
                        data_changed = true;
                    }

                    ui.add_space(10.0);

                    let value_response = ui.add(
                        egui::TextEdit::singleline(&mut state.edit_buffers[i].1)
                            .desired_width(80.0)
                            .font(egui::TextStyle::Monospace),
                    );
                    if value_response.changed() {
                        data_changed = true;
                    }
                });

                if row_response.response.clicked() {
                    state.selected_row = Some(i);
                }
            }

            if data_changed {
                match state.sync_data_from_buffers() {
                    Ok(()) => {
                        state.validation_error = None;
                        *result = PwlEditorResult::Modified;
                    }
                    Err(e) => {
                        state.validation_error = Some(e.to_string());
                    }
                }
            }
        });
}

fn render_validation_error(ui: &mut Ui, state: &PwlEditorState) {
    if let Some(error) = &state.validation_error {
        ui.separator();
        ui.colored_label(Color32::RED, format!("⚠ {}", error));
    }
}

fn render_summary(ui: &mut Ui, state: &PwlEditorState) {
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("{} points", state.data.len())).weak());
        if let Some((t_min, t_max)) = state.data.time_range() {
            ui.separator();
            ui.label(
                RichText::new(format!(
                    "Time: {} → {}",
                    format_engineering_for_spice(t_min),
                    format_engineering_for_spice(t_max)
                ))
                .weak(),
            );
        }
        if let Some((v_min, v_max)) = state.data.value_range() {
            ui.separator();
            ui.label(
                RichText::new(format!(
                    "Value: {} → {} {}",
                    format_engineering_for_spice(v_min),
                    format_engineering_for_spice(v_max),
                    state.value_unit
                ))
                .weak(),
            );
        }
    });
}
