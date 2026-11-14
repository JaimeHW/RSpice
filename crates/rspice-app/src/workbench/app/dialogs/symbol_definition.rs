//! Import and typed parameter-form workflows for project-owned symbol
//! definitions.
//!
//! The mockup owns two entry transactions on Models > Symbols & CDF. This
//! module keeps their retained drafts, file-picker authority, validation and
//! atomic project-library publication together so neither native nor browser
//! builds can bypass the same contract.

pub(crate) mod import_workflow;
mod parameter_form_workflow;
mod render;
mod source_picker;
pub(crate) mod state;

pub(in crate::workbench) use render::render_symbol_definition_dialogs;
pub(crate) use state::{SymbolImportDialogState, SymbolParameterFormDialogState};

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::state::WorkbenchSession;

pub(crate) fn open_symbol_import_dialog(
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
pub(crate) fn open_symbol_import_dialog_for(
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

pub(crate) fn open_symbol_parameter_form_dialog(
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
pub(crate) fn open_symbol_parameter_form_dialog_for(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ModelBoundSymbolDefinition, SymbolDefinitionImport};
    use crate::workbench::app_state::AppState;

    #[test]
    fn import_action_opens_an_isolated_draft_in_the_writable_project_library() {
        let mut state = AppState::default();
        state.open_project_session();

        open_symbol_import_dialog(
            &mut state.design.symbol_import,
            &state.model,
            &mut state.workbench,
            &state.platform,
        );

        assert!(state.design.symbol_import.open);
        assert_eq!(
            state.design.symbol_import.target_library,
            state.model.workspace.content.active_view.library
        );
        assert!(state.design.symbol_import.source_text.is_empty());
        assert!(!state.design.symbol_import.dirty);
    }

    #[test]
    fn form_action_loads_the_selected_typed_definition_as_the_next_revision() {
        let mut state = AppState::default();
        state.open_project_session();
        let library_name = state.model.workspace.content.active_view.library.clone();
        let cell_name = "review_form";
        let template = ModelBoundSymbolDefinition::review_only(&library_name, cell_name);
        let imported = SymbolDefinitionImport::from_bytes(
            br#"<svg><rect x="0" y="0" width="40" height="20"/></svg>"#,
            "review.svg",
            Some(template),
        )
        .expect("typed review definition");
        let library = state
            .model
            .library_manager
            .get_library_mut(&library_name)
            .expect("project library");
        crate::state::prepare_symbol_construction(&imported.definition, library)
            .expect("construction plan")
            .commit(library)
            .expect("commit definition");
        state
            .model
            .library_manager
            .select_view(&library_name, cell_name, "symbol");

        open_symbol_parameter_form_dialog(
            &mut state.design.symbol_parameter_form,
            &state.model,
            &mut state.workbench,
            &state.platform,
        );

        assert!(state.design.symbol_parameter_form.open);
        assert_eq!(
            state.design.symbol_parameter_form.target,
            Some(CellViewRef::new(&library_name, cell_name, "symbol"))
        );
        assert_eq!(
            state
                .design
                .symbol_parameter_form
                .draft_form
                .as_ref()
                .expect("form draft")
                .revision,
            2
        );
    }
}
