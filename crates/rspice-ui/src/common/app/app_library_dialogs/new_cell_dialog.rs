//! New Cell — small modal on the dialog primitive: target library, name,
//! description, and the views to seed the cell with.

use super::shared::{DialogActionOutcome, validate_lcv_name};
use super::{ConsoleMessage, Context, RSpiceApp, VERILOGA_LIBRARY_NAME};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, check_row, input_row};

impl RSpiceApp {
    pub(in crate::common::app) fn process_new_cell_dialog(&mut self, ctx: &Context) {
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

        let dialogs = &mut self.state.dialogs;
        let choice = Dialog::new("Library", "New cell", "Create")
            .size(DialogSize::Sm)
            .ghost("Cancel")
            .primary_enabled(!dialogs.new_cell_name.trim().is_empty())
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
        if lib_ro.get_cell(&name).is_some() {
            self.state.dialogs.new_cell_error = Some(format!(
                "Cell '{}' already exists in library '{}'",
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
