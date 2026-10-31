//! New Library, Rename Library, and Delete Library.
//!
//! Each form is bound to the library-catalog revision it opened against, so a
//! catalog that moved underneath the modal invalidates the transaction rather
//! than applying it to a different library. The semantics live in
//! `Model::create_library` / `rename_library` / `delete_library`; deletion
//! additionally states, before it is confirmed, every reason the transaction
//! would refuse it.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, input_row, kv_row};
use crate::workbench::app::dialogs::library::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::{LibraryRemoval, WorkspacePruned};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::{ProjectRequest, ReservedLibraryRevision};
use crate::workbench::slots::ProjectState;

impl ProjectState {
    /// Open New Library with an empty name.
    pub(crate) fn open_new_library_dialog(&mut self, model: &Model) {
        self.new_library_name.clear();
        self.new_library_error = None;
        self.new_library_library_revision = model.library_manager.revision();
        self.new_library_dialog = true;
    }

    /// Open Rename Library pre-filled with the current name.
    pub(crate) fn open_rename_library_dialog(
        &mut self,
        model: &Model,
        library: &str,
    ) -> Result<(), String> {
        let target = model
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        if target.read_only {
            return Err(format!(
                "Library '{library}' is read-only; it cannot be renamed."
            ));
        }

        self.rename_library_current = library.to_owned();
        self.rename_library_name = library.to_owned();
        self.rename_library_error = None;
        self.rename_library_library_revision = model.library_manager.revision();
        self.rename_library_dialog = true;
        Ok(())
    }

    /// Open the Delete Library review for one existing library.
    pub(crate) fn open_delete_library_review(
        &mut self,
        model: &Model,
        library: &str,
    ) -> Result<(), String> {
        if model.library_manager.get_library(library).is_none() {
            return Err(format!("Library '{library}' no longer exists."));
        }

        self.delete_library_target = library.to_owned();
        self.delete_library_error = None;
        self.delete_library_library_revision = model.library_manager.revision();
        self.delete_library_dialog = true;
        Ok(())
    }
}

/// Create the library the dialog names, and its name.
fn commit_new_library_dialog(cx: &mut Cx<'_, ProjectFeature>) -> Result<String, String> {
    if cx.model.library_manager.revision() != cx.state.new_library_library_revision {
        return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
    }
    let name = cx.state.new_library_name.trim().to_owned();
    if let Some(error) = validate_lcv_name(&name, "Library name") {
        return Err(error);
    }
    cx.model.create_library(&name).map(|()| name)
}

/// Rename the library the dialog names: how many instance references now
/// name it, and what the rename left outside the model, or `None` when the
/// name did not change.
fn commit_rename_library_dialog(
    cx: &mut Cx<'_, ProjectFeature>,
) -> Result<Option<(usize, WorkspacePruned)>, String> {
    if cx.model.library_manager.revision() != cx.state.rename_library_library_revision {
        return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
    }
    let name = cx.state.rename_library_name.trim().to_owned();
    let current = cx.state.rename_library_current.clone();
    if name == current {
        return Ok(None);
    }
    if let Some(error) = validate_lcv_name(&name, "Library name") {
        return Err(error);
    }
    cx.model.rename_library(&current, &name).map(Some)
}

/// Take the library the review names out of the catalog, its revision not
/// yet published.
fn commit_delete_library_review(cx: &mut Cx<'_, ProjectFeature>) -> Result<LibraryRemoval, String> {
    if cx.model.library_manager.revision() != cx.state.delete_library_library_revision {
        return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
    }
    let library = cx.state.delete_library_target.clone();
    cx.model.delete_library(&library)
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

fn library_deletion_impact(model: &Model, library: &str) -> LibraryDeletionImpact {
    let owned = model.library_manager.get_library(library);
    LibraryDeletionImpact {
        cells: owned.map_or(0, crate::state::Library::cell_count),
        views: owned.map_or(0, crate::state::Library::total_view_count),
        open_views: model
            .workspace
            .content
            .open_views
            .iter()
            .filter(|open| open.reference.library == library)
            .count(),
        instance_references: model.external_instance_references_to_library(library),
        source_bundles: model
            .workspace
            .content
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
        configuration_roots: model
            .workspace
            .content
            .configuration_sets
            .configurations()
            .iter()
            .filter(|configuration| configuration.root().library == library)
            .count(),
        project_root: model.workspace.content.project.root_library == library,
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

impl ProjectFeature {
    pub(in crate::workbench) fn process_new_library_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if !cx.state.new_library_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;

        let catalog_current =
            cx.model.library_manager.revision() == cx.state.new_library_library_revision;
        if !catalog_current {
            cx.state.new_library_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut *cx.state;
        let choice = Dialog::new("New library", "Create")
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
            match commit_new_library_dialog(cx) {
                Ok(name) => {
                    cx.model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        format!("Created library '{name}'."),
                    );
                    cx.state.new_library_error = None;
                    should_close = true;
                }
                Err(error) => cx.state.new_library_error = Some(error),
            }
        }

        if should_close {
            let dialogs = &mut *cx.state;
            dialogs.new_library_dialog = false;
            dialogs.new_library_name.clear();
            dialogs.new_library_error = None;
        }
    }

    /// Draw Rename Library. What a rename leaves outside the model goes to
    /// the shell with the rename's own line in the log.
    pub(in crate::workbench) fn process_rename_library_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        if !cx.state.rename_library_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_rename = false;

        let catalog_current =
            cx.model.library_manager.revision() == cx.state.rename_library_library_revision;
        if !catalog_current {
            cx.state.rename_library_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut *cx.state;
        let current = dialogs.rename_library_current.clone();
        let choice = Dialog::new("Rename library", "Rename")
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
            let current = cx.state.rename_library_current.clone();
            let name = cx.state.rename_library_name.trim().to_owned();
            match commit_rename_library_dialog(cx) {
                Ok(Some((remapped, pruned))) => {
                    cx.wb.outbox.push(ProjectRequest::CatalogChanged {
                        pruned,
                        mutation: None,
                        notice: format!(
                            "Renamed library '{current}' to '{name}'; {remapped} instance \
                             references now name it."
                        ),
                    });
                    cx.state.rename_library_error = None;
                    should_close = true;
                }
                Ok(None) => should_close = true,
                Err(error) => cx.state.rename_library_error = Some(error),
            }
        }

        if should_close {
            let dialogs = &mut *cx.state;
            dialogs.rename_library_dialog = false;
            dialogs.rename_library_name.clear();
            dialogs.rename_library_error = None;
        }
    }

    /// Draw the Delete Library review. What a deletion leaves outside the
    /// model, the revision it reserved and its own line in the log go to the
    /// shell, which carries them out in that order.
    pub(in crate::workbench) fn process_delete_library_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        if !cx.state.delete_library_dialog {
            return;
        }

        let library = cx.state.delete_library_target.clone();
        let impact = library_deletion_impact(cx.model, &library);
        let catalog_current =
            cx.model.library_manager.revision() == cx.state.delete_library_library_revision;
        let blocker = cx.model.library_deletion_blocker(&library);
        let displayed_error = if catalog_current {
            blocker
                .clone()
                .or_else(|| cx.state.delete_library_error.clone())
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

        let choice = Dialog::new("Delete library", "Delete")
            .description(
                "Permanently remove this library, every cell and view it owns, and every loaded workspace buffer beneath it.",
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
            DialogChoice::Primary => match commit_delete_library_review(cx) {
                Ok(LibraryRemoval {
                    cells,
                    pruned,
                    mutation,
                }) => {
                    cx.wb.outbox.push(ProjectRequest::CatalogChanged {
                        pruned,
                        mutation: Some(ReservedLibraryRevision::new(mutation)),
                        notice: format!(
                            "Deleted library '{library}' and its {cells} cell{}.",
                            if cells == 1 { "" } else { "s" }
                        ),
                    });
                    cx.state.delete_library_error = None;
                    should_close = true;
                }
                Err(error) => cx.state.delete_library_error = Some(error),
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_close {
            let dialogs = &mut *cx.state;
            dialogs.delete_library_dialog = false;
            dialogs.delete_library_target.clear();
            dialogs.delete_library_error = None;
        }
    }
}

#[cfg(test)]
mod tests;
