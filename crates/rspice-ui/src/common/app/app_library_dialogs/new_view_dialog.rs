//! New View — small modal on the dialog primitive: the target
//! library/cell context, a view name, and the view type as chips.

use super::shared::{DialogActionOutcome, validate_lcv_name};
use super::{ConsoleMessage, Context, RSpiceApp, VERILOGA_LIBRARY_NAME};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, chip, input_row, kv_row};

impl RSpiceApp {
    pub(in crate::common::app) fn process_new_view_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.new_view_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;
        let mut persist_global_veriloga = false;

        let dialogs = &mut self.state.dialogs;
        let can_create = !dialogs.new_view_name.trim().is_empty()
            && !dialogs.new_view_library.is_empty()
            && !dialogs.new_view_cell.is_empty();

        let choice = Dialog::new("Library", "New view", "Create")
            .size(DialogSize::Sm)
            .ghost("Cancel")
            .primary_enabled(can_create)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                kv_row(ui, "Library", &dialogs.new_view_library);
                kv_row(ui, "Cell", &dialogs.new_view_cell);
                ui.add_space(4.0);

                input_row(ui, "View name", &mut dialogs.new_view_name);

                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), t.metrics.row_h),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        let (label_rect, _) = ui.allocate_exact_size(
                            egui::vec2(96.0, t.metrics.row_h),
                            egui::Sense::hover(),
                        );
                        ui.painter().text(
                            egui::pos2(label_rect.left(), label_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            "Type",
                            theme::sans(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                        use crate::state::ViewType;
                        for view_type in ViewType::ALL.iter() {
                            if chip(
                                ui,
                                view_type.display_name(),
                                dialogs.new_view_type == *view_type,
                            )
                            .clicked()
                            {
                                dialogs.new_view_type = *view_type;
                            }
                        }
                    },
                );

                if let Some(error) = dialogs.new_view_error.clone() {
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
        let view_name = self.state.dialogs.new_view_name.trim().to_string();
        let library = self.state.dialogs.new_view_library.clone();
        let cell = self.state.dialogs.new_view_cell.clone();

        if let Some(error) = validate_lcv_name(&view_name, "View name") {
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
        if cell_ro.get_view(&view_name).is_some() {
            self.state.dialogs.new_view_error = Some(format!(
                "View '{}' already exists in cell '{}'",
                view_name, cell
            ));
            return outcome;
        }

        use crate::state::View;

        if let Some(lib) = self.state.library_manager.get_library_mut(&library) {
            if let Some(cell_ref) = lib.get_cell_mut(&cell) {
                let view_type = self.state.dialogs.new_view_type;
                cell_ref.add_view(View::new(&view_name, view_type));
                self.state.push_user_message(ConsoleMessage::info(format!(
                    "Created view '{}' in cell '{}'",
                    view_name, cell
                )));
                if matches!(
                    view_type,
                    crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
                ) {
                    self.state
                        .open_workspace_view(crate::state::CellViewRef::new(
                            library.clone(),
                            cell.clone(),
                            view_name.clone(),
                        ));
                }
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
