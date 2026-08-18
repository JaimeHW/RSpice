//! New Library, Rename Library, and Delete Library.
//!
//! Each form is bound to the library-catalog revision it opened against, so a
//! catalog that moved underneath the modal invalidates the transaction rather
//! than applying it to a different library. The semantics live in
//! `AppState::create_library` / `rename_library` / `delete_library`; deletion
//! additionally states, before it is confirmed, every reason the transaction
//! would refuse it.

use super::shared::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use super::{Context, RSpiceApp};
use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, input_row, kv_row};
use crate::workbench::AppState;

impl AppState {
    /// Open New Library with an empty name.
    pub(crate) fn open_new_library_dialog(&mut self) {
        self.dialogs.new_library_name.clear();
        self.dialogs.new_library_error = None;
        self.dialogs.new_library_library_revision = self.library_manager.revision();
        self.dialogs.new_library_dialog = true;
    }

    /// Open Rename Library pre-filled with the current name.
    pub(crate) fn open_rename_library_dialog(&mut self, library: &str) -> Result<(), String> {
        let target = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        if target.read_only {
            return Err(format!(
                "Library '{library}' is read-only; it cannot be renamed."
            ));
        }

        self.dialogs.rename_library_current = library.to_owned();
        self.dialogs.rename_library_name = library.to_owned();
        self.dialogs.rename_library_error = None;
        self.dialogs.rename_library_library_revision = self.library_manager.revision();
        self.dialogs.rename_library_dialog = true;
        Ok(())
    }

    /// Open the Delete Library review for one existing library.
    pub(crate) fn open_delete_library_review(&mut self, library: &str) -> Result<(), String> {
        if self.library_manager.get_library(library).is_none() {
            return Err(format!("Library '{library}' no longer exists."));
        }

        self.dialogs.delete_library_target = library.to_owned();
        self.dialogs.delete_library_error = None;
        self.dialogs.delete_library_library_revision = self.library_manager.revision();
        self.dialogs.delete_library_dialog = true;
        Ok(())
    }

    fn commit_new_library_dialog(&mut self) -> Result<String, String> {
        if self.library_manager.revision() != self.dialogs.new_library_library_revision {
            return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let name = self.dialogs.new_library_name.trim().to_owned();
        if let Some(error) = validate_lcv_name(&name, "Library name") {
            return Err(error);
        }
        self.create_library(&name).map(|()| name)
    }

    fn commit_rename_library_dialog(&mut self) -> Result<Option<usize>, String> {
        if self.library_manager.revision() != self.dialogs.rename_library_library_revision {
            return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let name = self.dialogs.rename_library_name.trim().to_owned();
        let current = self.dialogs.rename_library_current.clone();
        if name == current {
            return Ok(None);
        }
        if let Some(error) = validate_lcv_name(&name, "Library name") {
            return Err(error);
        }
        self.rename_library(&current, &name).map(Some)
    }

    fn commit_delete_library_review(&mut self) -> Result<usize, String> {
        if self.library_manager.revision() != self.dialogs.delete_library_library_revision {
            return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let library = self.dialogs.delete_library_target.clone();
        self.delete_library(&library)
    }
}

/// Everything the reader loses with the library, counted before the review is
/// confirmed.
struct LibraryDeletionImpact {
    cells: usize,
    views: usize,
    open_views: usize,
    instance_references: usize,
    source_bundles: usize,
    configuration_roots: usize,
    project_root: bool,
}

fn library_deletion_impact(state: &AppState, library: &str) -> LibraryDeletionImpact {
    let owned = state.library_manager.get_library(library);
    LibraryDeletionImpact {
        cells: owned.map_or(0, crate::state::Library::cell_count),
        views: owned.map_or(0, crate::state::Library::total_view_count),
        open_views: state
            .workspace
            .open_views
            .iter()
            .filter(|open| open.reference.library == library)
            .count(),
        instance_references: state.external_instance_references_to_library(library),
        source_bundles: state
            .workspace
            .project_sources
            .iter_bundles()
            .filter(|bundle| {
                matches!(
                    bundle.owner(),
                    crate::state::ProjectSourceOwner::CellView { reference }
                        if reference.library == library
                )
            })
            .count(),
        configuration_roots: state
            .workspace
            .configuration_sets
            .configurations()
            .iter()
            .filter(|configuration| configuration.root().library == library)
            .count(),
        project_root: state.workspace.project.root_library == library,
    }
}

fn error_label(ui: &mut egui::Ui, error: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(error)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.err),
    );
}

impl RSpiceApp {
    pub(in crate::workbench) fn process_new_library_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.new_library_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;

        let catalog_current = self.state.library_manager.revision()
            == self.state.dialogs.new_library_library_revision;
        if !catalog_current {
            self.state.dialogs.new_library_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut self.state.dialogs;
        let choice = Dialog::new("Library", "New library", "Create")
            .description(
                "Create an empty writable library in this project. Cells and views are added to it afterwards.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint("creates an empty writable library")
            .primary_enabled(catalog_current && !dialogs.new_library_name.trim().is_empty())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                input_row(ui, "Name", &mut dialogs.new_library_name);
                if let Some(error) = dialogs.new_library_error.clone() {
                    error_label(ui, &error);
                }
            });

        match choice {
            DialogChoice::Primary => should_create = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_create {
            match self.state.commit_new_library_dialog() {
                Ok(name) => {
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Created library '{name}'"
                    )));
                    self.state.dialogs.new_library_error = None;
                    should_close = true;
                }
                Err(error) => self.state.dialogs.new_library_error = Some(error),
            }
        }

        if should_close {
            let dialogs = &mut self.state.dialogs;
            dialogs.new_library_dialog = false;
            dialogs.new_library_name.clear();
            dialogs.new_library_error = None;
        }
    }

    pub(in crate::workbench) fn process_rename_library_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.rename_library_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_rename = false;

        let catalog_current = self.state.library_manager.revision()
            == self.state.dialogs.rename_library_library_revision;
        if !catalog_current {
            self.state.dialogs.rename_library_error =
                Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut self.state.dialogs;
        let current = dialogs.rename_library_current.clone();
        let choice = Dialog::new("Library", "Rename library", "Rename")
            .description(
                "Rename the selected library and remap open buffers, tabs, instance bindings, owned sources, and the project root.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint("remaps buffers, tabs, bindings, and sources")
            .primary_enabled(catalog_current && !dialogs.rename_library_name.trim().is_empty())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                kv_row(ui, "Library", &current);
                input_row(ui, "New name", &mut dialogs.rename_library_name);
                if let Some(error) = dialogs.rename_library_error.clone() {
                    error_label(ui, &error);
                }
            });

        match choice {
            DialogChoice::Primary => should_rename = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_rename {
            let current = self.state.dialogs.rename_library_current.clone();
            let name = self.state.dialogs.rename_library_name.trim().to_owned();
            match self.state.commit_rename_library_dialog() {
                Ok(Some(remapped)) => {
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Renamed library '{current}' to '{name}' — {remapped} instance \
                         references remapped"
                    )));
                    self.state.dialogs.rename_library_error = None;
                    should_close = true;
                }
                Ok(None) => should_close = true,
                Err(error) => self.state.dialogs.rename_library_error = Some(error),
            }
        }

        if should_close {
            let dialogs = &mut self.state.dialogs;
            dialogs.rename_library_dialog = false;
            dialogs.rename_library_name.clear();
            dialogs.rename_library_error = None;
        }
    }

    pub(in crate::workbench) fn process_delete_library_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.delete_library_dialog {
            return;
        }

        let library = self.state.dialogs.delete_library_target.clone();
        let impact = library_deletion_impact(&self.state, &library);
        let catalog_current = self.state.library_manager.revision()
            == self.state.dialogs.delete_library_library_revision;
        let blocker = self.state.library_deletion_blocker(&library);
        let displayed_error = if catalog_current {
            blocker
                .clone()
                .or_else(|| self.state.dialogs.delete_library_error.clone())
        } else {
            Some(
                "The library catalog changed after this review opened. Cancel and review the deletion again."
                    .to_owned(),
            )
        };
        let cells = impact.cells.to_string();
        let views = impact.views.to_string();
        let open_views = impact.open_views.to_string();
        let references = impact.instance_references.to_string();
        let bundles = impact.source_bundles.to_string();
        let roots = impact.configuration_roots.to_string();
        let project_root = if impact.project_root { "yes" } else { "no" };

        let choice = Dialog::new("Library", "Delete library", "Delete")
            .description(
                "Permanently remove the exact library, every cell and view it owns, and every loaded workspace buffer beneath it.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .destructive()
            .primary_enabled(catalog_current && blocker.is_none())
            .hint("Permanent project mutation")
            .show(ctx, |ui| {
                kv_row(ui, "Library", &library);
                kv_row(ui, "Cells removed", &cells);
                kv_row(ui, "Views removed", &views);
                kv_row(ui, "Open views", &open_views);
                kv_row(ui, "External instance references", &references);
                kv_row(ui, "Owned source bundles", &bundles);
                kv_row(ui, "Configuration roots", &roots);
                kv_row(ui, "Project root", project_root);

                ui.add_space(8.0);
                let t = Tokens::get(ui.ctx());
                ui.label(
                    egui::RichText::new(
                        "This action removes the selected library from the current project. It cannot be undone from the schematic-local history.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.warn),
                );
                if let Some(error) = displayed_error {
                    error_label(ui, &error);
                }
            });

        let mut should_close = false;
        match choice {
            DialogChoice::Primary => match self.state.commit_delete_library_review() {
                Ok(cells) => {
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Deleted library '{library}' and its {cells} cell{}",
                        if cells == 1 { "" } else { "s" }
                    )));
                    self.state.dialogs.delete_library_error = None;
                    should_close = true;
                }
                Err(error) => self.state.dialogs.delete_library_error = Some(error),
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_close {
            let dialogs = &mut self.state.dialogs;
            dialogs.delete_library_dialog = false;
            dialogs.delete_library_target.clear();
            dialogs.delete_library_error = None;
        }
    }
}

#[cfg(test)]
mod tests;
