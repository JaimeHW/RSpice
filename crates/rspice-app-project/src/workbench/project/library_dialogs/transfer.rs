//! Copy library, Copy view and Move cell: one command form, over the
//! catalog operations `Model::copy_library`, `Model::copy_view` and
//! `Model::move_cell`, each one step of Undo.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::CellViewRef;
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::dialogs::library::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::slots::ProjectState;

/// Which of the three operations the form carries out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferKind {
    CopyLibrary,
    CopyView,
    MoveCell,
}

/// The form's state: what it copies or moves, where to, and under what
/// name. Closed while `kind` is `None`.
#[derive(Debug, Clone, Default)]
pub struct LibraryTransferDialog {
    pub kind: Option<TransferKind>,
    pub source_library: String,
    pub source_cell: String,
    pub source_view: String,
    /// The library the copy or the moved cell goes to; a library copy names
    /// a new one in `name`.
    pub target_library: String,
    /// The cell a view copy goes to.
    pub target_cell: String,
    /// The new library's, the view copy's or the moved cell's name.
    pub name: String,
    pub error: Option<String>,
    /// The catalog revision the form was opened on.
    pub(crate) library_revision: u64,
}

impl LibraryTransferDialog {
    pub fn is_open(&self) -> bool {
        self.kind.is_some()
    }
}

/// The libraries a copy or a move can go to, by name.
fn writable_libraries(model: &Model, except: Option<&str>) -> Vec<String> {
    model
        .library_manager
        .libraries_sorted()
        .iter()
        .filter(|library| !library.read_only && Some(library.name.as_str()) != except)
        .map(|library| library.name.clone())
        .collect()
}

impl ProjectState {
    /// Open Copy library on `library`, the copy named after it.
    pub fn open_copy_library_dialog(&mut self, model: &Model, library: &str) -> Result<(), String> {
        model
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        *self.library_transfer = LibraryTransferDialog {
            kind: Some(TransferKind::CopyLibrary),
            source_library: library.to_owned(),
            name: format!("{library}_copy"),
            library_revision: model.library_manager.revision(),
            ..LibraryTransferDialog::default()
        };
        Ok(())
    }

    /// Open Copy view on `reference`: into its own cell when its library
    /// is writable, else into the same cell of the first writable library.
    pub fn open_copy_view_dialog(
        &mut self,
        model: &Model,
        reference: &CellViewRef,
    ) -> Result<(), String> {
        model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .ok_or_else(|| format!("View '{}' no longer exists.", reference.display_path()))?;
        let writable = writable_libraries(model, None);
        let target_library = if writable.contains(&reference.library) {
            reference.library.clone()
        } else {
            writable
                .first()
                .cloned()
                .ok_or_else(|| "No writable library can take the copy.".to_owned())?
        };
        *self.library_transfer = LibraryTransferDialog {
            kind: Some(TransferKind::CopyView),
            source_library: reference.library.clone(),
            source_cell: reference.cell.clone(),
            source_view: reference.view.clone(),
            target_library,
            target_cell: reference.cell.clone(),
            name: format!("{}_copy", reference.view),
            library_revision: model.library_manager.revision(),
            ..LibraryTransferDialog::default()
        };
        Ok(())
    }

    /// Open Move cell on `library`/`cell`, to the first other writable
    /// library, under its own name.
    pub fn open_move_cell_dialog(
        &mut self,
        model: &Model,
        library: &str,
        cell: &str,
    ) -> Result<(), String> {
        let source = model
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        if source.read_only {
            return Err(format!(
                "Library '{library}' is read-only; its cells cannot be moved."
            ));
        }
        if source.get_cell(cell).is_none() {
            return Err(format!("Cell '{library}/{cell}' no longer exists."));
        }
        let target_library = writable_libraries(model, Some(library))
            .into_iter()
            .next()
            .ok_or_else(|| "No other writable library can take the cell.".to_owned())?;
        *self.library_transfer = LibraryTransferDialog {
            kind: Some(TransferKind::MoveCell),
            source_library: library.to_owned(),
            source_cell: cell.to_owned(),
            target_library,
            name: cell.to_owned(),
            library_revision: model.library_manager.revision(),
            ..LibraryTransferDialog::default()
        };
        Ok(())
    }
}

/// Carry out what the open form holds: the line the Console states, or the
/// refusal the form shows.
pub fn commit_library_transfer(cx: &mut Cx<'_, ProjectFeature>) -> Result<(), String> {
    let form = cx.state.library_transfer.clone();
    let kind = form
        .kind
        .ok_or_else(|| "Nothing is open to copy or move.".to_owned())?;
    if cx.model.library_manager.revision() != form.library_revision {
        return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
    }
    let name = form.name.trim().to_owned();
    let label = match kind {
        TransferKind::CopyLibrary => "Library name",
        TransferKind::CopyView => "View name",
        TransferKind::MoveCell => "Cell name",
    };
    if let Some(error) = validate_lcv_name(&name, label) {
        return Err(error);
    }
    match kind {
        TransferKind::CopyLibrary => {
            let (outcome, prompt) = cx.model.copy_library(&form.source_library, &name);
            if let Some(prompt) = prompt {
                cx.wb.prompt(prompt);
            }
            let cells = outcome?;
            cx.model.record(
                LogSeverity::Info,
                LogSource::Studio,
                format!(
                    "Copied library '{}' to '{name}' ({cells} cells).",
                    form.source_library
                ),
            );
        }
        TransferKind::CopyView => {
            let cell = form.target_cell.trim().to_owned();
            if let Some(error) = validate_lcv_name(&cell, "Cell") {
                return Err(error);
            }
            let source =
                CellViewRef::new(&form.source_library, &form.source_cell, &form.source_view);
            let target = CellViewRef::new(&form.target_library, &cell, &name);
            let (outcome, prompt) = cx.model.copy_view(&source, &target);
            if let Some(prompt) = prompt {
                cx.wb.prompt(prompt);
            }
            outcome?;
            cx.model.record(
                LogSeverity::Info,
                LogSource::Studio,
                format!(
                    "Copied view '{}' to '{}'.",
                    source.display_path(),
                    target.display_path()
                ),
            );
        }
        TransferKind::MoveCell => {
            let (remapped, pruned) = cx.model.move_cell(
                &form.source_library,
                &form.source_cell,
                &form.target_library,
                &name,
            )?;
            cx.wb.outbox.push(ProjectRequest::CatalogChanged {
                pruned,
                mutation: None,
                notice: format!(
                    "Moved cell '{}/{}' to '{}/{name}'; {remapped} instance references now name it.",
                    form.source_library, form.source_cell, form.target_library
                ),
            });
        }
    }
    Ok(())
}

impl ProjectFeature {
    /// Draw Copy library, Copy view or Move cell, whichever is open.
    pub(in crate::workbench) fn process_library_transfer_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        let Some(kind) = cx.state.library_transfer.kind else {
            return;
        };
        let catalog_current =
            cx.model.library_manager.revision() == cx.state.library_transfer.library_revision;
        let except = (kind == TransferKind::MoveCell)
            .then(|| cx.state.library_transfer.source_library.clone());
        let targets = writable_libraries(cx.model, except.as_deref());
        let form = &mut *cx.state.library_transfer;
        let (title, primary, description, source) = match kind {
            TransferKind::CopyLibrary => (
                "Copy library",
                "Copy",
                "Copy every cell and view of the library, with their drawings, sources and layouts, into a new writable library of this project.",
                form.source_library.clone(),
            ),
            TransferKind::CopyView => (
                "Copy view",
                "Copy",
                "Copy the view and its content into a cell of a writable library. A cell that does not exist is created.",
                format!(
                    "{}/{}/{}",
                    form.source_library, form.source_cell, form.source_view
                ),
            ),
            TransferKind::MoveCell => (
                "Move cell",
                "Move",
                "Move the cell to another library. Open tabs, every placement and the project top follow it.",
                format!("{}/{}", form.source_library, form.source_cell),
            ),
        };
        let error = (!catalog_current)
            .then(|| LIBRARY_CATALOG_STALE_MESSAGE.to_owned())
            .or_else(|| form.error.clone());
        let status = match &error {
            Some(error) => Err(error.as_str()),
            None if form.name.trim().is_empty() => Ok(Some("Enter a name.")),
            None => Ok(None),
        };
        let needs_target = kind != TransferKind::CopyLibrary;
        let mut edited = false;
        let choice = CommandForm::new(title, primary)
            .describe(description)
            .primary_enabled(
                catalog_current
                    && !form.name.trim().is_empty()
                    && (!needs_target || !targets.is_empty()),
            )
            .status(status)
            .show(ctx, |rows| {
                rows.value_mono("Source", &source);
                if needs_target
                    && let Some(index) = rows.select(
                        "Library",
                        "library-transfer-target",
                        &form.target_library,
                        &targets,
                    )
                {
                    form.target_library.clone_from(&targets[index]);
                    edited = true;
                }
                if kind == TransferKind::CopyView {
                    edited |= rows
                        .text(
                            "Cell",
                            egui::Id::new("library-transfer-cell"),
                            &mut form.target_cell,
                            "",
                        )
                        .changed();
                }
                let label = match kind {
                    TransferKind::CopyLibrary => "Library name",
                    TransferKind::CopyView => "View name",
                    TransferKind::MoveCell => "Cell name",
                };
                let name = egui::Id::new("library-transfer-name");
                edited |= rows.text(label, name, &mut form.name, "").changed();
                Some(name)
            });
        // A refusal belongs to what it refused; an edited form has not been
        // refused yet.
        if edited && catalog_current {
            cx.state.library_transfer.error = None;
        }
        let close = match choice {
            DialogChoice::Primary => match commit_library_transfer(cx) {
                Ok(()) => true,
                Err(error) => {
                    cx.state.library_transfer.error = Some(error);
                    false
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => true,
            DialogChoice::Secondary | DialogChoice::None => false,
        };
        if close {
            *cx.state.library_transfer = LibraryTransferDialog::default();
        }
    }
}
