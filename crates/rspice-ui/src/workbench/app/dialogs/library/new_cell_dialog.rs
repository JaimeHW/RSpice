//! New Cell — small modal on the dialog primitive: target library, name,
//! description, and the views to seed the cell with.

use super::shared::{DialogActionOutcome, LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use super::{Context, RSpiceApp, VERILOGA_LIBRARY_NAME};
use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, check_row, input_row};

impl RSpiceApp {
    pub(in crate::workbench) fn process_new_cell_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.new_cell_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;
        let mut persist_global_veriloga = false;

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
        let catalog_current =
            self.state.library_manager.revision() == self.state.dialogs.new_cell_library_revision;
        if !catalog_current {
            self.state.dialogs.new_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }

        let dialogs = &mut self.state.dialogs;
        let choice = Dialog::new("Library", "New cell", "Create")
            .description(
                "Create a named cell in a writable library with the selected initial views.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .primary_enabled(catalog_current && !dialogs.new_cell_name.trim().is_empty())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                // Target library.
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), t.metrics.row_h),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        let (label_rect, _) = ui.allocate_exact_size(
                            egui::vec2(92.0, t.metrics.row_h),
                            egui::Sense::hover(),
                        );
                        ui.painter().text(
                            egui::pos2(label_rect.left(), label_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            "Library",
                            theme::sans(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                        egui::ComboBox::from_id_salt("cell_library_combo")
                            .selected_text(&dialogs.new_cell_library)
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                for name in &lib_names {
                                    ui.selectable_value(
                                        &mut dialogs.new_cell_library,
                                        name.clone(),
                                        name,
                                    );
                                }
                            });
                    },
                );

                input_row(ui, "Cell name", &mut dialogs.new_cell_name);
                input_row(ui, "Description", &mut dialogs.new_cell_description);

                ui.add_space(8.0);
                let mut kicker = egui::text::LayoutJob::default();
                kicker.append(
                    "VIEWS TO CREATE",
                    0.0,
                    egui::TextFormat {
                        font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                        color: c.text_faint,
                        extra_letter_spacing: 0.08 * tokens::FS_0,
                        ..Default::default()
                    },
                );
                ui.label(kicker);
                check_row(ui, "Schematic", &mut dialogs.new_cell_create_schematic);
                check_row(ui, "Symbol", &mut dialogs.new_cell_create_symbol);
                check_row(ui, "Testbench", &mut dialogs.new_cell_create_testbench);

                if let Some(error) = dialogs.new_cell_error.clone() {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(error)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.err),
                    );
                }
            });

        match choice {
            DialogChoice::Primary => should_create = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

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
        let name = self.state.dialogs.new_cell_name.trim().to_string();
        let library = self.state.dialogs.new_cell_library.clone();

        if self.state.library_manager.revision() != self.state.dialogs.new_cell_library_revision {
            self.state.dialogs.new_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
            return outcome;
        }
        if let Some(error) = validate_lcv_name(&name, "Cell name") {
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
        if lib_ro.read_only {
            self.state.dialogs.new_cell_error = Some(format!(
                "Library '{library}' is read only; create the cell in an editable library"
            ));
            return outcome;
        }
        let requested_identity = crate::state::canonical_cell_view_owner_key(&library, &name, "");
        if lib_ro.cells.values().any(|cell| {
            crate::state::canonical_cell_view_owner_key(&library, &cell.name, "")
                == requested_identity
        }) {
            self.state.dialogs.new_cell_error = Some(format!(
                "Cell '{}' conflicts with an existing canonical cell identity in library '{}'",
                name, library
            ));
            return outcome;
        }

        use crate::state::{Cell, View, ViewType};

        let mut cell = Cell::new(&name);
        cell.description = self.state.dialogs.new_cell_description.clone();

        if self.state.dialogs.new_cell_create_schematic {
            cell.add_view(View::new("schematic", ViewType::Schematic));
        }
        if self.state.dialogs.new_cell_create_symbol {
            // A cell created here starts on an empty schematic, so its symbol
            // starts empty too and grows with the interface the user draws.
            match super::new_view_dialog::seeded_symbol_view("symbol", &[]) {
                Ok(view) => cell.add_view(view),
                Err(error) => {
                    self.state.dialogs.new_cell_error = Some(error);
                    return outcome;
                }
            }
        }
        if self.state.dialogs.new_cell_create_testbench {
            cell.add_view(View::new("testbench", ViewType::Testbench));
        }

        let project_mutation = match self.state.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::CreateCell {
                library: library.clone(),
                cell: name.clone(),
            },
        ) {
            Ok(prepared) => prepared,
            Err(error) => {
                self.state.dialogs.new_cell_error = Some(error);
                return outcome;
            }
        };
        let seeded_schematic = self.state.new_schematic_document();
        if let Some(lib) = self.state.library_manager.get_library_mut(&library) {
            lib.add_cell(cell);
            for view_name in [
                self.state
                    .dialogs
                    .new_cell_create_schematic
                    .then_some("schematic"),
                self.state
                    .dialogs
                    .new_cell_create_testbench
                    .then_some("testbench"),
            ]
            .into_iter()
            .flatten()
            {
                let reference = crate::state::CellViewRef::new(&library, &name, view_name);
                self.state
                    .workspace
                    .schematic_buffers
                    .insert(reference.key(), seeded_schematic.clone());
            }
            self.state
                .publish_project_library_mutation(project_mutation);
            self.state.push_user_message(ConsoleMessage::info(format!(
                "Created cell '{}' in library '{}'",
                name, library
            )));
            let view_to_open = if self.state.dialogs.new_cell_create_schematic {
                Some("schematic")
            } else if self.state.dialogs.new_cell_create_symbol {
                Some("symbol")
            } else if self.state.dialogs.new_cell_create_testbench {
                Some("testbench")
            } else {
                None
            };
            if let Some(view) = view_to_open {
                self.state
                    .open_workspace_view(crate::state::CellViewRef::new(
                        library.clone(),
                        name.clone(),
                        view,
                    ));
            }
            self.state.dialogs.new_cell_error = None;
            outcome.close = true;
            outcome.persist_global_veriloga = library == VERILOGA_LIBRARY_NAME;
        } else {
            self.state.dialogs.new_cell_error = Some(format!("Library '{}' not found", library));
        }

        outcome
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        CellViewRef, ComponentType, Library, OperatingPointAnnotationPolicy, Point,
        SchematicGridPitch, SymbolDocument,
    };
    use crate::workbench::ChoicePreference;

    fn cell_symbol_view(app: &RSpiceApp, library: &str, cell: &str) -> crate::state::View {
        app.state
            .library_manager
            .get_library(library)
            .and_then(|library| library.get_cell(cell))
            .and_then(|cell| cell.get_view("symbol"))
            .cloned()
            .expect("the created cell owns a symbol view")
    }

    #[test]
    fn creating_cell_seeds_every_schematic_like_view_at_creation_time() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .library_manager
            .add_library(Library::new("policy_test"));
        app.state
            .ui
            .preferences
            .set_choice(ChoicePreference::SchematicGrid, 2)
            .unwrap();
        app.state
            .ui
            .preferences
            .set_choice(ChoicePreference::OperatingPointAnnotation, 2)
            .unwrap();
        app.state.dialogs.new_cell_library = "policy_test".to_owned();
        app.state.dialogs.new_cell_name = "amp".to_owned();
        app.state.dialogs.new_cell_create_schematic = true;
        app.state.dialogs.new_cell_create_testbench = true;
        app.state.dialogs.new_cell_library_revision = app.state.library_manager.revision();
        let project_revision_before = app.state.workspace.project.revision().get();

        let outcome = app.handle_new_cell_create_action();

        assert!(outcome.close);
        assert_eq!(
            app.state.workspace.project.revision().get(),
            project_revision_before + 1
        );
        assert!(matches!(
            app.state
                .workspace
                .project
                .library_mutation_audit()
                .last()
                .map(|receipt| receipt.mutation()),
            Some(crate::state::ProjectLibraryMutation::CreateCell { library, cell })
                if library == "policy_test" && cell == "amp"
        ));
        assert!(app.state.workspace.project_metadata_dirty);
        for view in ["schematic", "testbench"] {
            let key = crate::state::CellViewRef::new("policy_test", "amp", view).key();
            let document = app
                .state
                .workspace
                .schematic_buffers
                .get(&key)
                .expect("document seeded when view is created");
            assert_eq!(
                document.document_policy.grid_pitch,
                SchematicGridPitch::Metric
            );
            assert_eq!(
                document.grid_size,
                SchematicGridPitch::Metric.canvas_grid_size()
            );
            assert_eq!(
                document.document_policy.operating_point_annotations,
                OperatingPointAnnotationPolicy::Hidden
            );
        }
    }

    #[test]
    fn new_cell_symbol_view_follows_the_schematic_interface_until_first_edited() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .library_manager
            .add_library(Library::new("tracking_test"));
        app.state.dialogs.new_cell_library = "tracking_test".to_owned();
        app.state.dialogs.new_cell_name = "amp".to_owned();
        app.state.dialogs.new_cell_create_schematic = true;
        app.state.dialogs.new_cell_create_symbol = true;
        app.state.dialogs.new_cell_library_revision = app.state.library_manager.revision();

        let outcome = app.handle_new_cell_create_action();

        assert!(outcome.close);
        assert_eq!(
            app.state.workspace.active_view,
            CellViewRef::new("tracking_test", "amp", "schematic")
        );
        let created = cell_symbol_view(&app, "tracking_test", "amp");
        assert_eq!(
            created.metadata.get("generated").map(String::as_str),
            Some("ports"),
            "a symbol created with the cell is the schematic's until hand edited"
        );
        assert!(
            SymbolDocument::load_from_view(&created)
                .expect("the seeded symbol document parses")
                .pins
                .is_empty()
        );

        let port_id = app
            .state
            .schematic
            .add_component(ComponentType::Port, Point::origin());
        app.state
            .schematic
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("the placed interface port")
            .value = "IN".to_owned();
        app.state.sync_active_schematic_to_workspace();

        let refreshed = cell_symbol_view(&app, "tracking_test", "amp");
        assert_eq!(
            refreshed.metadata.get("ports").map(String::as_str),
            Some("IN:inout")
        );
        assert_eq!(
            SymbolDocument::load_from_view(&refreshed)
                .expect("the refreshed symbol document parses")
                .pins
                .iter()
                .map(|pin| pin.name.as_str())
                .collect::<Vec<_>>(),
            ["IN"]
        );
    }

    #[test]
    fn new_cell_rejects_accented_canonical_collision_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("identity_test");
        library.add_cell(crate::state::Cell::new("\u{c9}tage"));
        app.state.library_manager.add_library(library);
        app.state.dialogs.new_cell_library = "identity_test".to_owned();
        app.state.dialogs.new_cell_name = "\u{e9}TAGE".to_owned();
        app.state.dialogs.new_cell_create_schematic = true;
        app.state.dialogs.new_cell_library_revision = app.state.library_manager.revision();
        let buffers_before = app.state.workspace.schematic_buffers.len();

        let outcome = app.handle_new_cell_create_action();

        assert!(!outcome.close);
        let library = app
            .state
            .library_manager
            .get_library("identity_test")
            .expect("identity library remains");
        assert_eq!(library.cell_count(), 1);
        assert!(library.get_cell("\u{c9}tage").is_some());
        assert_eq!(app.state.workspace.schematic_buffers.len(), buffers_before);
        assert!(
            app.state
                .dialogs
                .new_cell_error
                .as_deref()
                .is_some_and(|error| error.contains("canonical cell identity"))
        );
    }

    #[test]
    fn new_cell_rejects_read_only_library_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("read_only_test");
        library.read_only = true;
        app.state.library_manager.add_library(library);
        app.state.dialogs.new_cell_library = "read_only_test".to_owned();
        app.state.dialogs.new_cell_name = "amp".to_owned();
        app.state.dialogs.new_cell_library_revision = app.state.library_manager.revision();
        let revision_before = app.state.workspace.project.revision();

        let outcome = app.handle_new_cell_create_action();

        assert!(!outcome.close);
        assert!(
            app.state
                .library_manager
                .get_library("read_only_test")
                .is_some_and(|library| library.get_cell("amp").is_none())
        );
        assert_eq!(app.state.workspace.project.revision(), revision_before);
        assert!(
            app.state
                .dialogs
                .new_cell_error
                .as_deref()
                .is_some_and(|error| error.contains("read only"))
        );
    }

    #[test]
    fn new_cell_rejects_catalog_change_after_dialog_open_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .library_manager
            .add_library(Library::new("stale_test"));
        app.state.dialogs.new_cell_library = "stale_test".to_owned();
        app.state.dialogs.new_cell_name = "amp".to_owned();
        app.state.dialogs.new_cell_library_revision = app.state.library_manager.revision();
        app.state
            .library_manager
            .add_library(Library::new("intervening_change"));
        let project_revision_before = app.state.workspace.project.revision();

        let outcome = app.handle_new_cell_create_action();

        assert!(!outcome.close);
        assert!(
            app.state
                .library_manager
                .get_library("stale_test")
                .is_some_and(|library| library.get_cell("amp").is_none())
        );
        assert_eq!(
            app.state.workspace.project.revision(),
            project_revision_before
        );
        assert_eq!(
            app.state.dialogs.new_cell_error.as_deref(),
            Some(LIBRARY_CATALOG_STALE_MESSAGE)
        );
    }
}
