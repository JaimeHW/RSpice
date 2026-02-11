use egui::Context;

use super::{save_global_veriloga_library, ConsoleMessage, RSpiceApp, VERILOGA_LIBRARY_NAME};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct DialogActionOutcome {
    close: bool,
    persist_global_veriloga: bool,
}

fn validate_lcv_name(name: &str, field_label: &str) -> Option<String> {
    if name.is_empty() {
        return Some(format!("{field_label} cannot be empty"));
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Some(format!(
            "{field_label} must contain only letters, numbers, and underscores"
        ));
    }
    None
}

impl RSpiceApp {
    fn persist_global_veriloga_library_with_feedback(&mut self) {
        if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
            log::warn!("Failed to persist global Verilog-A library: {}", err);
            self.state
                .push_user_message(ConsoleMessage::warning(format!(
                    "Failed to persist Verilog-A library: {}",
                    err
                )));
        }
    }

    pub(super) fn process_new_cell_dialog(&mut self, ctx: &Context) {
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

                // Library selection
                ui.horizontal(|ui| {
                    ui.label("Library:");
                    ui.add_space(20.0);

                    // Get editable library names (non-readonly)
                    let lib_names: Vec<String> = self
                        .state
                        .library_manager
                        .libraries_sorted()
                        .iter()
                        .filter(|lib| !lib.read_only)
                        .map(|lib| lib.name.clone())
                        .collect();

                    // Default to first editable library if empty
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

                // Cell name input
                ui.horizontal(|ui| {
                    ui.label("Cell Name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.state.dialogs.new_cell_name)
                            .hint_text("e.g., my_opamp")
                            .desired_width(200.0),
                    );
                });

                // Description input
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

                // View types to create
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

                // Error message display
                if let Some(ref error) = self.state.dialogs.new_cell_error {
                    ui.add_space(4.0);
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Action buttons
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
            // Reset dialog state
            self.state.dialogs.new_cell_dialog = false;
            self.state.dialogs.new_cell_name.clear();
            self.state.dialogs.new_cell_description.clear();
            self.state.dialogs.new_cell_error = None;
            self.state.dialogs.new_cell_create_schematic = true;
            self.state.dialogs.new_cell_create_symbol = false;
            self.state.dialogs.new_cell_create_testbench = false;
        }
    }

    fn handle_new_cell_create_action(&mut self) -> DialogActionOutcome {
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

    pub(super) fn process_new_view_dialog(&mut self, ctx: &Context) {
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

                // Show target library and cell (read-only)
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

                // View name input
                ui.horizontal(|ui| {
                    ui.label("View Name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.state.dialogs.new_view_name)
                            .hint_text("e.g., schematic")
                            .desired_width(150.0),
                    );
                });

                // View type selection
                ui.horizontal(|ui| {
                    ui.label("View Type:");
                    ui.add_space(4.0);
                    egui::ComboBox::from_id_salt("view_type_combo")
                        .selected_text(self.state.dialogs.new_view_type.display_name())
                        .width(150.0)
                        .show_ui(ui, |ui| {
                            use crate::state::ViewType;
                            for vt in ViewType::ALL.iter() {
                                ui.selectable_value(
                                    &mut self.state.dialogs.new_view_type,
                                    *vt,
                                    vt.display_name(),
                                );
                            }
                        });
                });

                // Error message display
                if let Some(ref error) = self.state.dialogs.new_view_error {
                    ui.add_space(4.0);
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Action buttons
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

    fn handle_new_view_create_action(&mut self) -> DialogActionOutcome {
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

    pub(super) fn process_pending_library_deletions(&mut self) {
        // Process pending cell deletion
        if let Some((lib_name, cell_name)) = self.state.pending_delete_cell.take() {
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name) {
                deleted = lib.remove_cell(&cell_name);
                if deleted {
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Deleted cell '{}' from library '{}'",
                        cell_name, lib_name
                    )));
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                self.persist_global_veriloga_library_with_feedback();
            }
        }

        // Process pending view deletion
        if let Some((lib_name, cell_name, view_name)) = self.state.pending_delete_view.take() {
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name) {
                if let Some(cell) = lib.get_cell_mut(&cell_name) {
                    deleted = cell.remove_view(&view_name);
                    if deleted {
                        self.state.push_user_message(ConsoleMessage::info(format!(
                            "Deleted view '{}' from cell '{}'",
                            view_name, cell_name
                        )));
                    }
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                self.persist_global_veriloga_library_with_feedback();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, Library, View, ViewType};

    fn make_test_app() -> RSpiceApp {
        RSpiceApp {
            state: super::super::AppState::default(),
            first_frame: false,
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
        }
    }

    #[test]
    fn test_handle_new_view_create_action_reports_missing_library() {
        let mut app = make_test_app();
        app.state.dialogs.new_view_name = "schematic".to_string();
        app.state.dialogs.new_view_library = "missing_lib".to_string();
        app.state.dialogs.new_view_cell = "my_cell".to_string();

        let outcome = app.handle_new_view_create_action();

        assert_eq!(outcome, DialogActionOutcome::default());
        assert_eq!(
            app.state.dialogs.new_view_error.as_deref(),
            Some("Library 'missing_lib' not found")
        );
    }

    #[test]
    fn test_handle_new_view_create_action_reports_missing_cell() {
        let mut app = make_test_app();
        app.state.library_manager.add_library(Library::new("work"));
        app.state.dialogs.new_view_name = "schematic".to_string();
        app.state.dialogs.new_view_library = "work".to_string();
        app.state.dialogs.new_view_cell = "missing_cell".to_string();

        let outcome = app.handle_new_view_create_action();

        assert_eq!(outcome, DialogActionOutcome::default());
        assert_eq!(
            app.state.dialogs.new_view_error.as_deref(),
            Some("Cell 'missing_cell' not found in library 'work'")
        );
    }

    #[test]
    fn test_handle_new_view_create_action_reports_duplicate_view() {
        let mut app = make_test_app();
        let mut lib = Library::new("work");
        let mut cell = Cell::new("my_cell");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        lib.add_cell(cell);
        app.state.library_manager.add_library(lib);

        app.state.dialogs.new_view_name = "schematic".to_string();
        app.state.dialogs.new_view_library = "work".to_string();
        app.state.dialogs.new_view_cell = "my_cell".to_string();

        let outcome = app.handle_new_view_create_action();

        assert_eq!(outcome, DialogActionOutcome::default());
        assert_eq!(
            app.state.dialogs.new_view_error.as_deref(),
            Some("View 'schematic' already exists in cell 'my_cell'")
        );
    }

    #[test]
    fn test_handle_new_view_create_action_adds_view_and_requests_close() {
        let mut app = make_test_app();
        let mut lib = Library::new("work");
        lib.add_cell(Cell::new("my_cell"));
        app.state.library_manager.add_library(lib);

        app.state.dialogs.new_view_name = "symbol".to_string();
        app.state.dialogs.new_view_library = "work".to_string();
        app.state.dialogs.new_view_cell = "my_cell".to_string();
        app.state.dialogs.new_view_type = ViewType::Symbol;

        let outcome = app.handle_new_view_create_action();

        assert!(outcome.close);
        assert!(!outcome.persist_global_veriloga);
        assert!(app.state.dialogs.new_view_error.is_none());
        let created = app
            .state
            .library_manager
            .get_library("work")
            .and_then(|lib| lib.get_cell("my_cell"))
            .and_then(|cell| cell.get_view("symbol"))
            .is_some();
        assert!(created);
    }

    #[test]
    fn test_handle_new_cell_create_action_reports_missing_library() {
        let mut app = make_test_app();
        app.state.dialogs.new_cell_name = "my_cell".to_string();
        app.state.dialogs.new_cell_library = "missing".to_string();

        let outcome = app.handle_new_cell_create_action();

        assert_eq!(outcome, DialogActionOutcome::default());
        assert_eq!(
            app.state.dialogs.new_cell_error.as_deref(),
            Some("Library 'missing' not found")
        );
    }

    #[test]
    fn test_handle_new_cell_create_action_adds_cell_and_views() {
        let mut app = make_test_app();
        app.state.library_manager.add_library(Library::new("work"));
        app.state.dialogs.new_cell_name = "my_cell".to_string();
        app.state.dialogs.new_cell_library = "work".to_string();
        app.state.dialogs.new_cell_create_schematic = true;
        app.state.dialogs.new_cell_create_symbol = true;
        app.state.dialogs.new_cell_create_testbench = false;

        let outcome = app.handle_new_cell_create_action();

        assert!(outcome.close);
        assert!(!outcome.persist_global_veriloga);
        assert!(app.state.dialogs.new_cell_error.is_none());
        let created = app
            .state
            .library_manager
            .get_library("work")
            .and_then(|lib| lib.get_cell("my_cell"));
        assert!(created.is_some());
        let cell = created.unwrap();
        assert!(cell.get_view("schematic").is_some());
        assert!(cell.get_view("symbol").is_some());
        assert!(cell.get_view("testbench").is_none());
    }

    #[test]
    fn test_process_pending_library_deletions_removes_cell_and_view() {
        let mut app = make_test_app();
        let mut lib = Library::new("work");
        let mut cell = Cell::new("my_cell");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        lib.add_cell(cell);
        app.state.library_manager.add_library(lib);

        app.state.pending_delete_view = Some((
            "work".to_string(),
            "my_cell".to_string(),
            "schematic".to_string(),
        ));
        app.process_pending_library_deletions();
        let view_exists = app
            .state
            .library_manager
            .get_library("work")
            .and_then(|lib| lib.get_cell("my_cell"))
            .and_then(|cell| cell.get_view("schematic"))
            .is_some();
        assert!(!view_exists);

        app.state.pending_delete_cell = Some(("work".to_string(), "my_cell".to_string()));
        app.process_pending_library_deletions();
        let cell_exists = app
            .state
            .library_manager
            .get_library("work")
            .and_then(|lib| lib.get_cell("my_cell"))
            .is_some();
        assert!(!cell_exists);
    }
}
