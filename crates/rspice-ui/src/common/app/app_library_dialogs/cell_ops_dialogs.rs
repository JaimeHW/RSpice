//! Copy Cell and Rename Cell — small modals on the dialog primitive
//! (`design/app/volta-library-manager.html` §07). The forms are one field
//! deep; the semantics live in `AppState::copy_cell` / `rename_cell`.

use super::shared::validate_lcv_name;
use super::{ConsoleMessage, Context, RSpiceApp};
use crate::common::AppState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, input_row, kv_row};

impl AppState {
    /// Open Copy Cell pre-filled from a source cell, targeting the first
    /// writable library (preferring the source's own when writable).
    pub(crate) fn open_copy_cell_dialog(&mut self, library: &str, cell: &str) {
        self.dialogs.copy_cell_dialog = true;
        self.dialogs.copy_cell_source_library = library.to_owned();
        self.dialogs.copy_cell_source_cell = cell.to_owned();
        self.dialogs.copy_cell_name = format!("{cell}_copy");
        self.dialogs.copy_cell_error = None;
        let writable: Vec<String> = self
            .library_manager
            .libraries_sorted()
            .iter()
            .filter(|lib| !lib.read_only)
            .map(|lib| lib.name.clone())
            .collect();
        self.dialogs.copy_cell_target_library = if writable.iter().any(|l| l == library) {
            library.to_owned()
        } else {
            writable.first().cloned().unwrap_or_default()
        };
    }

    /// Open Rename Cell pre-filled with the current name.
    pub(crate) fn open_rename_cell_dialog(&mut self, library: &str, cell: &str) {
        self.dialogs.rename_cell_dialog = true;
        self.dialogs.rename_cell_library = library.to_owned();
        self.dialogs.rename_cell_current = cell.to_owned();
        self.dialogs.rename_cell_name = cell.to_owned();
        self.dialogs.rename_cell_error = None;
    }
}

impl RSpiceApp {
    pub(in crate::common::app) fn process_copy_cell_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.copy_cell_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_copy = false;

        let lib_names: Vec<String> = self
            .state
            .library_manager
            .libraries_sorted()
            .iter()
            .filter(|lib| !lib.read_only)
            .map(|lib| lib.name.clone())
            .collect();

        let view_count = self
            .state
            .library_manager
            .get_library(&self.state.dialogs.copy_cell_source_library)
            .and_then(|lib| lib.get_cell(&self.state.dialogs.copy_cell_source_cell))
            .map(|cell| cell.views.len())
            .unwrap_or(0);
        let hint = format!("copies all {view_count} views and their content");

        let dialogs = &mut self.state.dialogs;
        let source = format!(
            "{} / {}",
            dialogs.copy_cell_source_library, dialogs.copy_cell_source_cell
        );
        let choice = Dialog::new("Library", "Copy cell", "Copy")
            .size(DialogSize::Sm)
            .ghost("Cancel")
            .hint(&hint)
            .primary_enabled(!dialogs.copy_cell_name.trim().is_empty())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                kv_row(ui, "Source", &source);

                // Target library — writable only.
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
                            "To library",
                            theme::sans(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                        egui::ComboBox::from_id_salt("copy_cell_target_combo")
                            .selected_text(&dialogs.copy_cell_target_library)
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                for name in &lib_names {
                                    ui.selectable_value(
                                        &mut dialogs.copy_cell_target_library,
                                        name.clone(),
                                        name,
                                    );
                                }
                            });
                    },
                );

                input_row(ui, "New name", &mut dialogs.copy_cell_name);

                if let Some(error) = dialogs.copy_cell_error.clone() {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(error)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.err),
                    );
                }
            });

        match choice {
            DialogChoice::Primary => should_copy = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_copy {
            let name = self.state.dialogs.copy_cell_name.trim().to_string();
            if let Some(error) = validate_lcv_name(&name, "Cell name") {
                self.state.dialogs.copy_cell_error = Some(error);
            } else if self.state.dialogs.copy_cell_target_library.is_empty() {
                self.state.dialogs.copy_cell_error = Some("Please select a library".to_string());
            } else {
                let source_library = self.state.dialogs.copy_cell_source_library.clone();
                let source_cell = self.state.dialogs.copy_cell_source_cell.clone();
                let target = self.state.dialogs.copy_cell_target_library.clone();
                match self.state.copy_cell(&source_library, &source_cell, &target, &name) {
                    Ok(views) => {
                        self.state.push_user_message(ConsoleMessage::info(format!(
                            "Copied '{source_cell}' to '{target}/{name}' ({views} views)"
                        )));
                        self.state.dialogs.copy_cell_error = None;
                        should_close = true;
                    }
                    Err(error) => self.state.dialogs.copy_cell_error = Some(error),
                }
            }
        }

        if should_close {
            let dialogs = &mut self.state.dialogs;
            dialogs.copy_cell_dialog = false;
            dialogs.copy_cell_name.clear();
            dialogs.copy_cell_error = None;
        }
    }

    pub(in crate::common::app) fn process_rename_cell_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.rename_cell_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_rename = false;

        let dialogs = &mut self.state.dialogs;
        let source = format!(
            "{} / {}",
            dialogs.rename_cell_library, dialogs.rename_cell_current
        );
        let choice = Dialog::new("Library", "Rename cell", "Rename")
            .size(DialogSize::Sm)
            .ghost("Cancel")
            .hint("remaps buffers, tabs, and instance bindings")
            .primary_enabled(!dialogs.rename_cell_name.trim().is_empty())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                kv_row(ui, "Cell", &source);
                input_row(ui, "New name", &mut dialogs.rename_cell_name);

                if let Some(error) = dialogs.rename_cell_error.clone() {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(error)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.err),
                    );
                }
            });

        match choice {
            DialogChoice::Primary => should_rename = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_rename {
            let name = self.state.dialogs.rename_cell_name.trim().to_string();
            let library = self.state.dialogs.rename_cell_library.clone();
            let current = self.state.dialogs.rename_cell_current.clone();
            if name == current {
                // Nothing to do — not an error.
                should_close = true;
            } else if let Some(error) = validate_lcv_name(&name, "Cell name") {
                self.state.dialogs.rename_cell_error = Some(error);
            } else {
                match self.state.rename_cell(&library, &current, &name) {
                    Ok(remapped) => {
                        self.state.push_user_message(ConsoleMessage::info(format!(
                            "Renamed '{current}' to '{name}' — {remapped} instance \
                             references remapped"
                        )));
                        self.state.dialogs.rename_cell_error = None;
                        should_close = true;
                    }
                    Err(error) => self.state.dialogs.rename_cell_error = Some(error),
                }
            }
        }

        if should_close {
            let dialogs = &mut self.state.dialogs;
            dialogs.rename_cell_dialog = false;
            dialogs.rename_cell_name.clear();
            dialogs.rename_cell_error = None;
        }
    }
}
