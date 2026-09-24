//! Rename View.
//!
//! A view name is half of a Library/Cell/View identity, so the rename is a
//! project transaction rather than a label edit: the form is bound to the
//! library-catalog revision it opened against, and the semantics live in
//! `AppState::rename_view`.

use super::shared::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use super::{Context, RSpiceApp};
use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, input_row, kv_row};
use crate::workbench::AppState;

impl AppState {
    /// Open Rename View pre-filled with the current name.
    pub(crate) fn open_rename_view_dialog(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
    ) -> Result<(), String> {
        let target = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        if target.read_only {
            return Err(format!(
                "Library '{library}' is read-only; its views cannot be renamed."
            ));
        }
        if target
            .get_cell(cell)
            .and_then(|cell| cell.get_view(view))
            .is_none()
        {
            return Err(format!("View '{library}/{cell}/{view}' no longer exists."));
        }

        self.dialogs.rename_view_library = library.to_owned();
        self.dialogs.rename_view_cell = cell.to_owned();
        self.dialogs.rename_view_current = view.to_owned();
        self.dialogs.rename_view_name = view.to_owned();
        self.dialogs.rename_view_error = None;
        self.dialogs.rename_view_library_revision = self.library_manager.revision();
        self.dialogs.rename_view_dialog = true;
        Ok(())
    }

    fn commit_rename_view_dialog(&mut self) -> Result<Option<usize>, String> {
        if self.library_manager.revision() != self.dialogs.rename_view_library_revision {
            return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let name = self.dialogs.rename_view_name.trim().to_owned();
        let library = self.dialogs.rename_view_library.clone();
        let cell = self.dialogs.rename_view_cell.clone();
        let current = self.dialogs.rename_view_current.clone();
        if name == current {
            return Ok(None);
        }
        if let Some(error) = validate_lcv_name(&name, "View name") {
            return Err(error);
        }
        self.rename_view(&library, &cell, &current, &name).map(Some)
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn process_rename_view_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.rename_view_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_rename = false;

        let catalog_current = self.state.library_manager.revision()
            == self.state.dialogs.rename_view_library_revision;
        if !catalog_current {
            self.state.dialogs.rename_view_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut self.state.dialogs;
        let source = format!(
            "{} / {} / {}",
            dialogs.rename_view_library, dialogs.rename_view_cell, dialogs.rename_view_current
        );
        let choice = Dialog::new("Library", "Rename view", "Rename")
            .description(
                "Rename the selected view and remap its buffer, open tabs, owned source, and every instance bound to exactly this view.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint("remaps the buffer, tabs, and view-exact bindings")
            .primary_enabled(catalog_current && !dialogs.rename_view_name.trim().is_empty())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let t = Tokens::get(ui.ctx());

                kv_row(ui, "View", &source);
                input_row(ui, "New name", &mut dialogs.rename_view_name);

                if let Some(error) = dialogs.rename_view_error.clone() {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(error)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.err),
                    );
                }
            });

        match choice {
            DialogChoice::Primary => should_rename = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_rename {
            let current = self.state.dialogs.rename_view_current.clone();
            let name = self.state.dialogs.rename_view_name.trim().to_owned();
            match self.state.commit_rename_view_dialog() {
                Ok(Some(remapped)) => {
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Renamed view '{current}' to '{name}' — {remapped} instance \
                         references remapped"
                    )));
                    self.state.dialogs.rename_view_error = None;
                    should_close = true;
                }
                Ok(None) => should_close = true,
                Err(error) => self.state.dialogs.rename_view_error = Some(error),
            }
        }

        if should_close {
            let dialogs = &mut self.state.dialogs;
            dialogs.rename_view_dialog = false;
            dialogs.rename_view_name.clear();
            dialogs.rename_view_error = None;
        }
    }
}

#[cfg(test)]
mod tests;
