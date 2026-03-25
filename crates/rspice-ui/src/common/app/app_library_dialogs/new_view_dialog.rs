use super::shared::{DialogActionOutcome, validate_lcv_name};
use super::{ConsoleMessage, Context, RSpiceApp, VERILOGA_LIBRARY_NAME};

impl RSpiceApp {
    pub(in crate::common::app) fn process_new_view_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.new_view_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;
        let mut persist_global_veriloga = false;

        egui::Window::new("Create New View")
            .collapsible(false)
            .resizable(false)
            .default_width(350.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;

                ui.horizontal(|ui| {
                    ui.label("Library:");
                    ui.add_space(16.0);
                    ui.label(&self.state.dialogs.new_view_library);
                });
                ui.horizontal(|ui| {
                    ui.label("Cell:");
                    ui.add_space(38.0);
                    ui.label(&self.state.dialogs.new_view_cell);
                });

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label("View Name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.state.dialogs.new_view_name)
                            .hint_text("e.g., schematic")
                            .desired_width(150.0),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("View Type:");
                    ui.add_space(4.0);
                    egui::ComboBox::from_id_salt("view_type_combo")
                        .selected_text(self.state.dialogs.new_view_type.display_name())
                        .width(150.0)
                        .show_ui(ui, |ui| {
                            use crate::state::ViewType;
                            for view_type in ViewType::ALL.iter() {
                                ui.selectable_value(
                                    &mut self.state.dialogs.new_view_type,
                                    *view_type,
                                    view_type.display_name(),
                                );
                            }
                        });
                });

                if let Some(ref error) = self.state.dialogs.new_view_error {
                    ui.add_space(4.0);
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Create").clicked() {
                        should_create = true;
                    }
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                });
            });

        if should_create {
            let outcome = self.handle_new_view_create_action();
            should_close |= outcome.close;
            persist_global_veriloga |= outcome.persist_global_veriloga;
        }

        if persist_global_veriloga {
            self.persist_global_veriloga_library_with_feedback();
        }

        if should_close {
            self.state.dialogs.new_view_dialog = false;
            self.state.dialogs.new_view_name.clear();
            self.state.dialogs.new_view_error = None;
        }
    }

    pub(super) fn handle_new_view_create_action(&mut self) -> DialogActionOutcome {
        let mut outcome = DialogActionOutcome::default();
        let view_name = self.state.dialogs.new_view_name.trim();
        let library = self.state.dialogs.new_view_library.clone();
        let cell = self.state.dialogs.new_view_cell.clone();

        if let Some(error) = validate_lcv_name(view_name, "View name") {
            self.state.dialogs.new_view_error = Some(error);
            return outcome;
        }
        if library.is_empty() {
            self.state.dialogs.new_view_error = Some("Library cannot be empty".to_string());
            return outcome;
        }
        if cell.is_empty() {
            self.state.dialogs.new_view_error = Some("Cell cannot be empty".to_string());
            return outcome;
        }

        let Some(lib_ro) = self.state.library_manager.get_library(&library) else {
            self.state.dialogs.new_view_error = Some(format!("Library '{}' not found", library));
            return outcome;
        };
        let Some(cell_ro) = lib_ro.get_cell(&cell) else {
            self.state.dialogs.new_view_error = Some(format!(
                "Cell '{}' not found in library '{}'",
                cell, library
            ));
            return outcome;
        };
        if cell_ro.get_view(view_name).is_some() {
            self.state.dialogs.new_view_error = Some(format!(
                "View '{}' already exists in cell '{}'",
                view_name, cell
            ));
            return outcome;
        }

        use crate::state::View;

        if let Some(lib) = self.state.library_manager.get_library_mut(&library) {
            if let Some(cell_ref) = lib.get_cell_mut(&cell) {
                cell_ref.add_view(View::new(view_name, self.state.dialogs.new_view_type));
                self.state.push_user_message(ConsoleMessage::info(format!(
                    "Created view '{}' in cell '{}'",
                    view_name, cell
                )));
                self.state.dialogs.new_view_error = None;
                outcome.close = true;
                outcome.persist_global_veriloga = library == VERILOGA_LIBRARY_NAME;
            } else {
                self.state.dialogs.new_view_error = Some(format!(
                    "Cell '{}' not found in library '{}'",
                    cell, library
                ));
            }
        } else {
            self.state.dialogs.new_view_error = Some(format!("Library '{}' not found", library));
        }

        outcome
    }
}
