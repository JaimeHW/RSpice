use super::shared::{DialogActionOutcome, validate_lcv_name};
use super::{ConsoleMessage, Context, RSpiceApp, VERILOGA_LIBRARY_NAME};

impl RSpiceApp {
    pub(in crate::common::app) fn process_new_cell_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.new_cell_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;
        let mut persist_global_veriloga = false;

        egui::Window::new("Create New Cell")
            .collapsible(false)
            .resizable(false)
            .default_width(400.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;

                ui.horizontal(|ui| {
                    ui.label("Library:");
                    ui.add_space(20.0);

                    let lib_names: Vec<String> = self
                        .state
                        .library_manager
                        .libraries_sorted()
                        .iter()
                        .filter(|lib| !lib.read_only)
                        .map(|lib| lib.name.clone())
                        .collect();

                    if self.state.dialogs.new_cell_library.is_empty() && !lib_names.is_empty() {
                        self.state.dialogs.new_cell_library = lib_names[0].clone();
                    }

                    egui::ComboBox::from_id_salt("cell_library_combo")
                        .selected_text(&self.state.dialogs.new_cell_library)
                        .width(200.0)
                        .show_ui(ui, |ui| {
                            for name in &lib_names {
                                ui.selectable_value(
                                    &mut self.state.dialogs.new_cell_library,
                                    name.clone(),
                                    name,
                                );
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Cell Name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.state.dialogs.new_cell_name)
                            .hint_text("e.g., my_opamp")
                            .desired_width(200.0),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Description:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.state.dialogs.new_cell_description)
                            .hint_text("Optional description")
                            .desired_width(200.0),
                    );
                });

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                ui.label("Views to Create:");
                ui.indent("views_indent", |ui| {
                    ui.checkbox(
                        &mut self.state.dialogs.new_cell_create_schematic,
                        "Schematic",
                    );
                    ui.checkbox(&mut self.state.dialogs.new_cell_create_symbol, "Symbol");
                    ui.checkbox(
                        &mut self.state.dialogs.new_cell_create_testbench,
                        "Testbench",
                    );
                });

                if let Some(ref error) = self.state.dialogs.new_cell_error {
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
            let outcome = self.handle_new_cell_create_action();
            should_close |= outcome.close;
            persist_global_veriloga |= outcome.persist_global_veriloga;
        }

        if persist_global_veriloga {
            self.persist_global_veriloga_library_with_feedback();
        }

        if should_close {
            self.state.dialogs.new_cell_dialog = false;
            self.state.dialogs.new_cell_name.clear();
            self.state.dialogs.new_cell_description.clear();
            self.state.dialogs.new_cell_error = None;
            self.state.dialogs.new_cell_create_schematic = true;
            self.state.dialogs.new_cell_create_symbol = false;
            self.state.dialogs.new_cell_create_testbench = false;
        }
    }

    pub(super) fn handle_new_cell_create_action(&mut self) -> DialogActionOutcome {
        let mut outcome = DialogActionOutcome::default();
        let name = self.state.dialogs.new_cell_name.trim();
        let library = self.state.dialogs.new_cell_library.clone();

        if let Some(error) = validate_lcv_name(name, "Cell name") {
            self.state.dialogs.new_cell_error = Some(error);
            return outcome;
        }
        if library.is_empty() {
            self.state.dialogs.new_cell_error = Some("Please select a library".to_string());
            return outcome;
        }

        let Some(lib_ro) = self.state.library_manager.get_library(&library) else {
            self.state.dialogs.new_cell_error = Some(format!("Library '{}' not found", library));
            return outcome;
        };
        if lib_ro.get_cell(name).is_some() {
            self.state.dialogs.new_cell_error = Some(format!(
                "Cell '{}' already exists in library '{}'",
                name, library
            ));
            return outcome;
        }

        use crate::state::{Cell, View, ViewType};

        let mut cell = Cell::new(name);
        cell.description = self.state.dialogs.new_cell_description.clone();

        if self.state.dialogs.new_cell_create_schematic {
            cell.add_view(View::new("schematic", ViewType::Schematic));
        }
        if self.state.dialogs.new_cell_create_symbol {
            cell.add_view(View::new("symbol", ViewType::Symbol));
        }
        if self.state.dialogs.new_cell_create_testbench {
            cell.add_view(View::new("testbench", ViewType::Testbench));
        }

        if let Some(lib) = self.state.library_manager.get_library_mut(&library) {
            lib.add_cell(cell);
            self.state.push_user_message(ConsoleMessage::info(format!(
                "Created cell '{}' in library '{}'",
                name, library
            )));
            self.state.dialogs.new_cell_error = None;
            outcome.close = true;
            outcome.persist_global_veriloga = library == VERILOGA_LIBRARY_NAME;
        } else {
            self.state.dialogs.new_cell_error = Some(format!("Library '{}' not found", library));
        }

        outcome
    }
}
