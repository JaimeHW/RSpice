//! Copy Cell and Rename Cell — small modals on the dialog primitive
//! The forms are one field
//! deep; the semantics live in `AppState::copy_cell` / `rename_cell`.

use super::shared::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use super::{Context, RSpiceApp};
use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, input_row, kv_row};
use crate::workbench::AppState;

impl AppState {
    /// Open Copy Cell pre-filled from a source cell, targeting the first
    /// writable library (preferring the source's own when writable).
    pub(crate) fn open_copy_cell_dialog(
        &mut self,
        library: &str,
        cell: &str,
    ) -> Result<(), String> {
        let source = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        if source.get_cell(cell).is_none() {
            return Err(format!("Cell '{library}/{cell}' no longer exists."));
        }
        let writable: Vec<String> = self
            .library_manager
            .libraries_sorted()
            .iter()
            .filter(|lib| !lib.read_only)
            .map(|lib| lib.name.clone())
            .collect();
        let target = if writable.iter().any(|name| name == library) {
            library.to_owned()
        } else {
            writable
                .first()
                .cloned()
                .ok_or_else(|| "No writable destination library is available.".to_owned())?
        };

        self.dialogs.copy_cell_source_library = library.to_owned();
        self.dialogs.copy_cell_source_cell = cell.to_owned();
        self.dialogs.copy_cell_target_library = target;
        self.dialogs.copy_cell_name = format!("{cell}_copy");
        self.dialogs.copy_cell_error = None;
        self.dialogs.copy_cell_library_revision = self.library_manager.revision();
        self.dialogs.copy_cell_dialog = true;
        Ok(())
    }

    /// Open Rename Cell pre-filled with the current name.
    pub(crate) fn open_rename_cell_dialog(
        &mut self,
        library: &str,
        cell: &str,
    ) -> Result<(), String> {
        let target = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        if target.read_only {
            return Err(format!(
                "Library '{library}' is read-only; its cells cannot be renamed."
            ));
        }
        if target.get_cell(cell).is_none() {
            return Err(format!("Cell '{library}/{cell}' no longer exists."));
        }

        self.dialogs.rename_cell_library = library.to_owned();
        self.dialogs.rename_cell_current = cell.to_owned();
        self.dialogs.rename_cell_name = cell.to_owned();
        self.dialogs.rename_cell_error = None;
        self.dialogs.rename_cell_library_revision = self.library_manager.revision();
        self.dialogs.rename_cell_dialog = true;
        Ok(())
    }

    fn commit_copy_cell_dialog(&mut self) -> Result<usize, String> {
        if self.library_manager.revision() != self.dialogs.copy_cell_library_revision {
            return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let name = self.dialogs.copy_cell_name.trim().to_owned();
        if let Some(error) = validate_lcv_name(&name, "Cell name") {
            return Err(error);
        }
        if self.dialogs.copy_cell_target_library.is_empty() {
            return Err("Please select a library".to_owned());
        }
        let source_library = self.dialogs.copy_cell_source_library.clone();
        let source_cell = self.dialogs.copy_cell_source_cell.clone();
        let target = self.dialogs.copy_cell_target_library.clone();
        self.copy_cell(&source_library, &source_cell, &target, &name)
    }

    fn commit_rename_cell_dialog(&mut self) -> Result<Option<usize>, String> {
        if self.library_manager.revision() != self.dialogs.rename_cell_library_revision {
            return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let name = self.dialogs.rename_cell_name.trim().to_owned();
        let library = self.dialogs.rename_cell_library.clone();
        let current = self.dialogs.rename_cell_current.clone();
        if name == current {
            return Ok(None);
        }
        if let Some(error) = validate_lcv_name(&name, "Cell name") {
            return Err(error);
        }
        self.rename_cell(&library, &current, &name).map(Some)
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn process_copy_cell_dialog(&mut self, ctx: &Context) {
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
        let catalog_current =
            self.state.library_manager.revision() == self.state.dialogs.copy_cell_library_revision;
        if !catalog_current {
            self.state.dialogs.copy_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }

        let dialogs = &mut self.state.dialogs;
        let source = format!(
            "{} / {}",
            dialogs.copy_cell_source_library, dialogs.copy_cell_source_cell
        );
        let choice = Dialog::new("Library", "Copy cell", "Copy")
            .description(
                "Copy every view and its content from the selected cell into a writable library under a new name.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint(&hint)
            .primary_enabled(catalog_current && !dialogs.copy_cell_name.trim().is_empty())
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
            let source_cell = self.state.dialogs.copy_cell_source_cell.clone();
            let target = self.state.dialogs.copy_cell_target_library.clone();
            let name = self.state.dialogs.copy_cell_name.trim().to_owned();
            match self.state.commit_copy_cell_dialog() {
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

        if should_close {
            let dialogs = &mut self.state.dialogs;
            dialogs.copy_cell_dialog = false;
            dialogs.copy_cell_name.clear();
            dialogs.copy_cell_error = None;
        }
    }

    pub(in crate::workbench) fn process_rename_cell_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.rename_cell_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_rename = false;

        let catalog_current = self.state.library_manager.revision()
            == self.state.dialogs.rename_cell_library_revision;
        if !catalog_current {
            self.state.dialogs.rename_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut self.state.dialogs;
        let source = format!(
            "{} / {}",
            dialogs.rename_cell_library, dialogs.rename_cell_current
        );
        let choice = Dialog::new("Library", "Rename cell", "Rename")
            .description(
                "Rename the selected cell and remap open buffers, tabs, and instance bindings that reference it.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint("remaps buffers, tabs, and instance bindings")
            .primary_enabled(catalog_current && !dialogs.rename_cell_name.trim().is_empty())
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
            let current = self.state.dialogs.rename_cell_current.clone();
            let name = self.state.dialogs.rename_cell_name.trim().to_owned();
            match self.state.commit_rename_cell_dialog() {
                Ok(Some(remapped)) => {
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Renamed '{current}' to '{name}' — {remapped} instance \
                         references remapped"
                    )));
                    self.state.dialogs.rename_cell_error = None;
                    should_close = true;
                }
                Ok(None) => should_close = true,
                Err(error) => self.state.dialogs.rename_cell_error = Some(error),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, Library};

    #[test]
    fn rename_cell_dialog_starts_with_the_exact_active_cell_identity() {
        let mut state = AppState::default();
        let mut library = Library::new("work");
        library.add_cell(Cell::new("amplifier"));
        state.library_manager.add_library(library);

        state
            .open_rename_cell_dialog("work", "amplifier")
            .expect("default work/amplifier cell is writable");

        assert!(state.dialogs.rename_cell_dialog);
        assert_eq!(state.dialogs.rename_cell_library, "work");
        assert_eq!(state.dialogs.rename_cell_current, "amplifier");
        assert_eq!(state.dialogs.rename_cell_name, "amplifier");
        assert!(state.dialogs.rename_cell_error.is_none());
        assert_eq!(
            state.dialogs.rename_cell_library_revision,
            state.library_manager.revision()
        );
    }

    #[test]
    fn dialog_entry_points_reject_stale_targets_without_mutating_modal_state() {
        let mut state = AppState::default();
        let mut library = Library::new("work");
        library.add_cell(Cell::new("amplifier"));
        state.library_manager.add_library(library);
        state.dialogs.copy_cell_source_library = "sentinel".to_owned();
        state.dialogs.rename_cell_library = "sentinel".to_owned();

        assert!(state.open_copy_cell_dialog("missing", "amplifier").is_err());
        assert!(state.open_rename_cell_dialog("work", "missing").is_err());

        assert!(!state.dialogs.copy_cell_dialog);
        assert!(!state.dialogs.rename_cell_dialog);
        assert_eq!(state.dialogs.copy_cell_source_library, "sentinel");
        assert_eq!(state.dialogs.rename_cell_library, "sentinel");
    }

    #[test]
    fn rename_entry_point_rejects_read_only_library_without_mutation() {
        let mut state = AppState::default();
        let mut work = Library::new("work");
        work.add_cell(Cell::new("amplifier"));
        state.library_manager.add_library(work);
        let library = state
            .library_manager
            .get_library_mut("work")
            .expect("default work library exists");
        library.read_only = true;

        let error = state
            .open_rename_cell_dialog("work", "amplifier")
            .expect_err("read-only libraries must fail closed");

        assert!(error.contains("read-only"));
        assert!(!state.dialogs.rename_cell_dialog);
        assert!(state.dialogs.rename_cell_library.is_empty());
    }

    #[test]
    fn copy_and_rename_dialog_commits_reject_intervening_catalog_change() {
        let mut copy_state = AppState::default();
        let mut copy_library = Library::new("work");
        copy_library.add_cell(Cell::new("amplifier"));
        copy_state.library_manager.add_library(copy_library);
        copy_state
            .open_copy_cell_dialog("work", "amplifier")
            .expect("copy dialog opens");
        assert_eq!(
            copy_state.dialogs.copy_cell_library_revision,
            copy_state.library_manager.revision()
        );
        copy_state
            .library_manager
            .add_library(Library::new("intervening"));

        let copy_error = copy_state
            .commit_copy_cell_dialog()
            .expect_err("stale copy must fail closed");

        assert_eq!(copy_error, LIBRARY_CATALOG_STALE_MESSAGE);
        assert!(
            copy_state
                .library_manager
                .get_library("work")
                .is_some_and(|library| library.get_cell("amplifier_copy").is_none())
        );

        let mut rename_state = AppState::default();
        let mut rename_library = Library::new("work");
        rename_library.add_cell(Cell::new("amplifier"));
        rename_state.library_manager.add_library(rename_library);
        rename_state
            .open_rename_cell_dialog("work", "amplifier")
            .expect("rename dialog opens");
        rename_state.dialogs.rename_cell_name = "amplifier_v2".to_owned();
        rename_state
            .library_manager
            .add_library(Library::new("intervening"));

        let rename_error = rename_state
            .commit_rename_cell_dialog()
            .expect_err("stale rename must fail closed");

        assert_eq!(rename_error, LIBRARY_CATALOG_STALE_MESSAGE);
        let library = rename_state
            .library_manager
            .get_library("work")
            .expect("source library remains");
        assert!(library.get_cell("amplifier").is_some());
        assert!(library.get_cell("amplifier_v2").is_none());
    }
}
