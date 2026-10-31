//! Rename View.
//!
//! A view name is half of a Library/Cell/View identity, so the rename is a
//! project transaction rather than a label edit: the form is bound to the
//! library-catalog revision it opened against, and the semantics live in
//! `Model::rename_view`.

use egui::Context;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, input_row, kv_row};
use crate::workbench::app::dialogs::library::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::WorkspacePruned;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::slots::ProjectState;

impl ProjectState {
    /// Open Rename View pre-filled with the current name.
    pub(crate) fn open_rename_view_dialog(
        &mut self,
        model: &Model,
        library: &str,
        cell: &str,
        view: &str,
    ) -> Result<(), String> {
        let target = model
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

        self.rename_view_library = library.to_owned();
        self.rename_view_cell = cell.to_owned();
        self.rename_view_current = view.to_owned();
        self.rename_view_name = view.to_owned();
        self.rename_view_error = None;
        self.rename_view_library_revision = model.library_manager.revision();
        self.rename_view_dialog = true;
        Ok(())
    }
}

/// Rename the view the dialog names: how many instance references now name
/// it, and what the rename left outside the model, or `None` when the name
/// did not change.
fn commit_rename_view_dialog(
    cx: &mut Cx<'_, ProjectFeature>,
) -> Result<Option<(usize, WorkspacePruned)>, String> {
    if cx.model.library_manager.revision() != cx.state.rename_view_library_revision {
        return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
    }
    let name = cx.state.rename_view_name.trim().to_owned();
    let library = cx.state.rename_view_library.clone();
    let cell = cx.state.rename_view_cell.clone();
    let current = cx.state.rename_view_current.clone();
    if name == current {
        return Ok(None);
    }
    if let Some(error) = validate_lcv_name(&name, "View name") {
        return Err(error);
    }
    cx.model
        .rename_view(&library, &cell, &current, &name)
        .map(Some)
}

impl ProjectFeature {
    /// Draw Rename View. What a rename leaves outside the model goes to the
    /// shell with the rename's own line in the log.
    pub(in crate::workbench) fn process_rename_view_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if !cx.state.rename_view_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_rename = false;

        let catalog_current =
            cx.model.library_manager.revision() == cx.state.rename_view_library_revision;
        if !catalog_current {
            cx.state.rename_view_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut *cx.state;
        let source = format!(
            "{} / {} / {}",
            dialogs.rename_view_library, dialogs.rename_view_cell, dialogs.rename_view_current
        );
        let choice = Dialog::new("Rename view", "Rename")
            .description(
                "Rename the selected view and remap its buffer, open tabs, owned source, and every instance bound to exactly this view.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint("remaps the buffer, tabs, and instances bound to this view")
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
            let current = cx.state.rename_view_current.clone();
            let name = cx.state.rename_view_name.trim().to_owned();
            match commit_rename_view_dialog(cx) {
                Ok(Some((remapped, pruned))) => {
                    cx.wb.outbox.push(ProjectRequest::CatalogChanged {
                        pruned,
                        mutation: None,
                        notice: format!(
                            "Renamed view '{current}' to '{name}'; {remapped} instance \
                             references now name it."
                        ),
                    });
                    cx.state.rename_view_error = None;
                    should_close = true;
                }
                Ok(None) => should_close = true,
                Err(error) => cx.state.rename_view_error = Some(error),
            }
        }

        if should_close {
            let dialogs = &mut *cx.state;
            dialogs.rename_view_dialog = false;
            dialogs.rename_view_name.clear();
            dialogs.rename_view_error = None;
        }
    }
}

#[cfg(test)]
mod tests;
