//! Import and typed parameter-form workflows for project-owned symbol
//! definitions.
//!
//! The mockup owns two entry transactions on Models > Symbols & CDF. This
//! module keeps their retained drafts, file-picker authority, validation and
//! atomic project-library publication together so neither native nor browser
//! builds can bypass the same contract.

pub mod import_workflow;
mod parameter_form_workflow;
mod render;
mod source_picker;
pub mod state;

pub(in crate::workbench) use render::render_symbol_definition_dialogs;
pub use state::{SymbolImportDialogState, SymbolParameterFormDialogState};

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::state::WorkbenchSession;

pub fn open_symbol_import_dialog(
    dialog: &mut SymbolImportDialogState,
    model: &Model,
    wb: &mut WorkbenchSession,
    platform: &PlatformState,
) {
    open_symbol_import_dialog_for(dialog, model, wb, platform, None);
}

/// Open the importer with an explicit project-owned symbol as the preferred
/// electrical binding source. Callers that present their own shared
/// Library/Cell/View selection use this entry point so an unrelated active
/// Design document cannot silently retarget the transaction.
pub fn open_symbol_import_dialog_for(
    dialog: &mut SymbolImportDialogState,
    model: &Model,
    wb: &mut WorkbenchSession,
    platform: &PlatformState,
    preferred_binding_source: Option<&CellViewRef>,
) {
    let Some(target_library) = writable_library_name(model, platform) else {
        wb.prompt("Import symbol needs an open project with a writable design library.");
        return;
    };
    let binding_source = preferred_binding_source
        .cloned()
        .filter(|reference| reference_has_typed_definition(model, reference))
        .or_else(|| {
            selected_symbol_reference(model)
                .filter(|reference| reference_has_typed_definition(model, reference))
        })
        .or_else(|| first_model_bound_symbol_reference(model));
    *dialog = SymbolImportDialogState {
        open: true,
        target_library,
        binding_source,
        ..SymbolImportDialogState::default()
    };
}

pub fn open_symbol_parameter_form_dialog(
    form: &mut SymbolParameterFormDialogState,
    model: &Model,
    wb: &mut WorkbenchSession,
    platform: &PlatformState,
) {
    let Some(target) = selected_symbol_reference(model).or_else(|| first_symbol_reference(model))
    else {
        wb.prompt("Open form designer needs a symbol in the project library.");
        return;
    };
    open_symbol_parameter_form_dialog_for(form, model, wb, platform, target);
}

/// Open the form transaction for one exact symbol reference.
///
/// This explicit form is required by the Library/Cellview specialist route:
/// its project-library selection is authoritative even when another symbol
/// remains open in the Design workspace.
pub fn open_symbol_parameter_form_dialog_for(
    form: &mut SymbolParameterFormDialogState,
    model: &Model,
    wb: &mut WorkbenchSession,
    platform: &PlatformState,
    target: CellViewRef,
) {
    let writable = model
        .library_manager
        .get_library(&target.library)
        .is_some_and(|library| !library.read_only);
    if !platform.project_lifecycle.is_open() || model.safe_mode.project_read_only() || !writable {
        wb.prompt("The symbol's parameter form cannot be edited: its library is read-only.");
        return;
    }
    let Some(view) = model
        .library_manager
        .get_library(&target.library)
        .and_then(|library| library.get_cell(&target.cell))
        .and_then(|cell| cell.get_view(&target.view))
    else {
        wb.prompt("The selected symbol view is no longer available.");
        return;
    };
    if view.view_type != ViewType::Symbol {
        wb.prompt("The selected cellview is not a symbol, so it has no parameter form.");
        return;
    }
    let definition = match crate::state::load_model_bound_symbol(view) {
        Ok(Some(definition)) => definition,
        Ok(None) => {
            wb.prompt(
                "The selected symbol has no parameter form. Create or import a model-bound \
                 symbol first.",
            );
            return;
        }
        Err(error) => {
            wb.prompt(format!(
                "The selected symbol definition is invalid: {error}"
            ));
            return;
        }
    };
    let mut draft_form = definition.parameter_form.clone();
    draft_form.revision = draft_form.revision.saturating_add(1);
    let selected_section = draft_form
        .sections
        .first()
        .map(|section| section.key.clone());
    let selected_field = draft_form
        .sections
        .first()
        .and_then(|section| section.fields.first())
        .map(|field| field.key.clone());
    *form = SymbolParameterFormDialogState {
        open: true,
        target: Some(target),
        original_definition: Some(definition),
        draft_form: Some(draft_form),
        selected_section,
        selected_field,
        ..SymbolParameterFormDialogState::default()
    };
}

fn writable_library_name(model: &Model, platform: &PlatformState) -> Option<String> {
    if !platform.project_lifecycle.is_open() || model.safe_mode.project_read_only() {
        return None;
    }
    model
        .library_manager
        .selected_library
        .as_deref()
        .and_then(|name| {
            model
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        })
        .or_else(|| {
            model
                .library_manager
                .libraries_sorted()
                .into_iter()
                .find(|library| !library.read_only)
                .map(|library| library.name.clone())
        })
}

fn selected_symbol_reference(model: &Model) -> Option<CellViewRef> {
    let active = &model.workspace.content.active_view;
    if model
        .library_manager
        .get_library(&active.library)
        .and_then(|library| library.get_cell(&active.cell))
        .and_then(|cell| cell.get_view(&active.view))
        .is_some_and(|view| view.view_type == ViewType::Symbol)
    {
        return Some(active.clone());
    }
    let library = model.library_manager.selected_library.as_deref()?;
    let cell = model.library_manager.selected_cell.as_deref()?;
    let view = model.library_manager.selected_view.as_deref()?;
    (model
        .library_manager
        .get_library(library)?
        .get_cell(cell)?
        .get_view(view)?
        .view_type
        == ViewType::Symbol)
        .then(|| CellViewRef::new(library, cell, view))
}

fn first_symbol_reference(model: &Model) -> Option<CellViewRef> {
    model
        .library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            library.cells_sorted().into_iter().flat_map(move |cell| {
                cell.views_sorted()
                    .into_iter()
                    .filter(|view| view.view_type == ViewType::Symbol)
                    .map(move |view| CellViewRef::new(&library.name, &cell.name, &view.name))
            })
        })
        .next()
}

fn first_model_bound_symbol_reference(model: &Model) -> Option<CellViewRef> {
    model
        .library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            library.cells_sorted().into_iter().flat_map(move |cell| {
                cell.views_sorted()
                    .into_iter()
                    .filter(|view| {
                        view.view_type == ViewType::Symbol
                            && matches!(crate::state::load_model_bound_symbol(view), Ok(Some(_)))
                    })
                    .map(move |view| CellViewRef::new(&library.name, &cell.name, &view.name))
            })
        })
        .next()
}

fn reference_has_typed_definition(model: &Model, reference: &CellViewRef) -> bool {
    model
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .is_some_and(|view| matches!(crate::state::load_model_bound_symbol(view), Ok(Some(_))))
}
