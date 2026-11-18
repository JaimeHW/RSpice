//! Library properties and Cell properties: what a library or a cell is for,
//! and whether a library the project keeps takes changes, edited on one
//! command form over `Model::edit_library_properties` and
//! `Model::edit_cell_properties`, each edit one step of Undo.

use egui::Context;

use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::dialogs::library::LIBRARY_CATALOG_STALE_MESSAGE;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::slots::ProjectState;

/// The form's state. Closed while `library` is empty; a cell's properties
/// while `cell` names one.
#[derive(Debug, Clone, Default)]
pub struct LibraryPropertiesDialog {
    pub library: String,
    pub cell: Option<String>,
    pub description: String,
    pub read_only: bool,
    pub error: Option<String>,
    /// The catalog revision the form was opened on.
    pub(crate) library_revision: u64,
}

impl LibraryPropertiesDialog {
    pub fn is_open(&self) -> bool {
        !self.library.is_empty()
    }
}

impl ProjectState {
    /// Open the properties of `library`, or with `cell` of that cell.
    pub fn open_library_properties_dialog(
        &mut self,
        model: &Model,
        library: &str,
        cell: Option<&str>,
    ) -> Result<(), String> {
        let found = model
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        let description = match cell {
            Some(cell) => found
                .get_cell(cell)
                .ok_or_else(|| format!("Cell '{library}/{cell}' no longer exists."))?
                .description
                .clone(),
            None => found.description().to_owned(),
        };
        *self.library_properties = LibraryPropertiesDialog {
            library: library.to_owned(),
            cell: cell.map(str::to_owned),
            description,
            read_only: found.read_only,
            error: None,
            library_revision: model.library_manager.revision(),
        };
        Ok(())
    }
}

/// Apply what the open form holds, each edit one step of Undo, or the
/// refusal the form shows.
pub fn commit_library_properties(cx: &mut Cx<'_, ProjectFeature>) -> Result<(), String> {
    let form = cx.state.library_properties.clone();
    if cx.model.library_manager.revision() != form.library_revision {
        return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
    }
    match &form.cell {
        Some(cell) => cx
            .model
            .edit_cell_properties(&form.library, cell, &form.description),
        None => cx
            .model
            .edit_library_properties(&form.library, &form.description, form.read_only),
    }
}

impl ProjectFeature {
    /// Draw Library properties or Cell properties, whichever is open.
    pub(in crate::workbench) fn process_library_properties_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        if !cx.state.library_properties.is_open() {
            return;
        }
        let catalog_current =
            cx.model.library_manager.revision() == cx.state.library_properties.library_revision;
        let form = &mut *cx.state.library_properties;
        let Some(library) = cx.model.library_manager.get_library(&form.library) else {
            *form = LibraryPropertiesDialog::default();
            return;
        };
        // Only a library the project keeps has an access of its own.
        let reference = cx.model.library_manager.reference(&form.library);
        let access_editable = reference.is_none() && form.cell.is_none();
        let location = reference.map_or_else(
            || "In this project".to_owned(),
            |reference| {
                reference
                    .path
                    .shown(rspice_design::library::folder::HostPlatform::current())
            },
        );
        let technology = library.technology.trim().to_owned();
        let views = form
            .cell
            .as_deref()
            .and_then(|cell| library.get_cell(cell))
            .map(|cell| cell.views.len());
        let title = match form.cell {
            Some(_) => "Cell properties",
            None => "Library properties",
        };
        let error = (!catalog_current)
            .then(|| LIBRARY_CATALOG_STALE_MESSAGE.to_owned())
            .or_else(|| form.error.clone());
        let mut edited = false;
        let hint = match form.cell {
            Some(_) => "What the cell is for",
            None => "What the library is for",
        };
        // A description is prose: Enter breaks its line, as in the project's
        // own properties.
        let choice = CommandForm::new(title, "Apply")
            .reading_column()
            .primary_on_enter(false)
            .primary_enabled(catalog_current)
            .status(error.as_deref().map_or(Ok(None), Err))
            .show(ctx, |rows| {
                match &form.cell {
                    Some(cell) => rows.value("Cell", &format!("{}/{cell}", form.library)),
                    None => rows.value("Library", &form.library),
                }
                rows.value("Location", &location);
                if form.cell.is_none() && !technology.is_empty() {
                    rows.value("Technology", &technology);
                }
                if let Some(views) = views {
                    rows.value("Views", &views.to_string());
                }
                let description = egui::Id::new("library-properties-description");
                edited |= rows
                    .text_multiline("Description", description, &mut form.description, hint, 3)
                    .changed();
                if access_editable {
                    edited |= rows.checks("Access", &mut [("Read-only", &mut form.read_only)]);
                } else if form.cell.is_none() {
                    rows.value(
                        "Access",
                        if form.read_only {
                            "Read-only"
                        } else {
                            "Writable"
                        },
                    );
                }
                Some(description)
            });
        if edited && catalog_current {
            cx.state.library_properties.error = None;
        }
        let close = match choice {
            DialogChoice::Primary => match commit_library_properties(cx) {
                Ok(()) => true,
                Err(error) => {
                    cx.state.library_properties.error = Some(error);
                    false
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => true,
            DialogChoice::Secondary | DialogChoice::None => false,
        };
        if close {
            *cx.state.library_properties = LibraryPropertiesDialog::default();
        }
    }
}
