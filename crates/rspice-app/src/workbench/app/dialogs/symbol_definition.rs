//! Import and typed parameter-form workflows for project-owned symbol
//! definitions.
//!
//! The mockup owns two entry transactions on Models > Symbols & CDF. This
//! module keeps their retained drafts, file-picker authority, validation and
//! atomic project-library publication together so neither native nor browser
//! builds can bypass the same contract.

mod import_workflow;
mod parameter_form_workflow;
mod render;
mod source_picker;
mod state;

pub(crate) use state::{SymbolImportDialogState, SymbolParameterFormDialogState};

use crate::diagnostics::ConsoleMessage;
use crate::state::{CellViewRef, ModelBoundSymbolDefinition, ViewType};
use crate::workbench::app_state::AppState;

pub(crate) fn open_symbol_import_dialog(state: &mut AppState) {
    open_symbol_import_dialog_for(state, None);
}

/// Open the importer with an explicit project-owned symbol as the preferred
/// electrical binding source. Callers that present their own shared
/// Library/Cell/View selection use this entry point so an unrelated active
/// Design document cannot silently retarget the transaction.
pub(crate) fn open_symbol_import_dialog_for(
    state: &mut AppState,
    preferred_binding_source: Option<&CellViewRef>,
) {
    let Some(target_library) = writable_library_name(state) else {
        state.push_user_message(ConsoleMessage::warning(
            "Import symbol requires an open project with a writable design library.",
        ));
        return;
    };
    let binding_source = preferred_binding_source
        .cloned()
        .filter(|reference| reference_has_typed_definition(state, reference))
        .or_else(|| {
            selected_symbol_reference(state)
                .filter(|reference| reference_has_typed_definition(state, reference))
        })
        .or_else(|| first_model_bound_symbol_reference(state));
    state.dialogs.symbol_import = SymbolImportDialogState {
        open: true,
        target_library,
        binding_source,
        ..SymbolImportDialogState::default()
    };
}

pub(crate) fn open_symbol_parameter_form_dialog(state: &mut AppState) {
    let Some(target) = selected_symbol_reference(state).or_else(|| first_symbol_reference(state))
    else {
        state.push_user_message(ConsoleMessage::warning(
            "Open form designer requires a symbol definition in the project library.",
        ));
        return;
    };
    open_symbol_parameter_form_dialog_for(state, target);
}

/// Open the form transaction for one exact symbol reference.
///
/// This explicit form is required by the Library/Cellview specialist route:
/// its project-library selection is authoritative even when another symbol
/// remains open in the Design workspace.
pub(crate) fn open_symbol_parameter_form_dialog_for(state: &mut AppState, target: CellViewRef) {
    let writable = state
        .library_manager
        .get_library(&target.library)
        .is_some_and(|library| !library.read_only);
    if !state.project_lifecycle.project_open
        || state.workbench.safe_mode.project_read_only()
        || !writable
    {
        state.push_user_message(ConsoleMessage::warning(
            "The selected symbol parameter form is unavailable because its project library is read-only.",
        ));
        return;
    }
    let Some(view) = state
        .library_manager
        .get_library(&target.library)
        .and_then(|library| library.get_cell(&target.cell))
        .and_then(|cell| cell.get_view(&target.view))
    else {
        state.push_user_message(ConsoleMessage::warning(
            "The selected symbol view is no longer available.",
        ));
        return;
    };
    if view.view_type != ViewType::Symbol {
        state.push_user_message(ConsoleMessage::warning(
            "The selected cellview is not a symbol and has no component-form contract.",
        ));
        return;
    }
    let definition = match crate::state::ModelBoundSymbolDefinition::load_from_view(view) {
        Ok(Some(definition)) => definition,
        Ok(None) => {
            state.push_user_message(ConsoleMessage::warning(
                "The selected legacy symbol has no typed component-form contract. Create or import a model-bound symbol first.",
            ));
            return;
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "The selected symbol definition is invalid: {error}"
            )));
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
    state.dialogs.symbol_parameter_form = SymbolParameterFormDialogState {
        open: true,
        target: Some(target),
        original_definition: Some(definition),
        draft_form: Some(draft_form),
        selected_section,
        selected_field,
        ..SymbolParameterFormDialogState::default()
    };
}

fn writable_library_name(state: &AppState) -> Option<String> {
    if !state.project_lifecycle.project_open || state.workbench.safe_mode.project_read_only() {
        return None;
    }
    state
        .library_manager
        .selected_library
        .as_deref()
        .and_then(|name| {
            state
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        })
        .or_else(|| {
            state
                .library_manager
                .libraries_sorted()
                .into_iter()
                .find(|library| !library.read_only)
                .map(|library| library.name.clone())
        })
}

fn selected_symbol_reference(state: &AppState) -> Option<CellViewRef> {
    let active = &state.workspace.active_view;
    if state
        .library_manager
        .get_library(&active.library)
        .and_then(|library| library.get_cell(&active.cell))
        .and_then(|cell| cell.get_view(&active.view))
        .is_some_and(|view| view.view_type == ViewType::Symbol)
    {
        return Some(active.clone());
    }
    let library = state.library_manager.selected_library.as_deref()?;
    let cell = state.library_manager.selected_cell.as_deref()?;
    let view = state.library_manager.selected_view.as_deref()?;
    (state
        .library_manager
        .get_library(library)?
        .get_cell(cell)?
        .get_view(view)?
        .view_type
        == ViewType::Symbol)
        .then(|| CellViewRef::new(library, cell, view))
}

fn first_symbol_reference(state: &AppState) -> Option<CellViewRef> {
    state
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

fn first_model_bound_symbol_reference(state: &AppState) -> Option<CellViewRef> {
    state
        .library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            library.cells_sorted().into_iter().flat_map(move |cell| {
                cell.views_sorted()
                    .into_iter()
                    .filter(|view| {
                        view.view_type == ViewType::Symbol
                            && matches!(
                                ModelBoundSymbolDefinition::load_from_view(view),
                                Ok(Some(_))
                            )
                    })
                    .map(move |view| CellViewRef::new(&library.name, &cell.name, &view.name))
            })
        })
        .next()
}

fn reference_has_typed_definition(state: &AppState, reference: &CellViewRef) -> bool {
    state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .is_some_and(|view| {
            matches!(
                ModelBoundSymbolDefinition::load_from_view(view),
                Ok(Some(_))
            )
        })
}

/// Maximum UTF-8 source accepted by the symbol-definition importer.
///
/// Symbol definitions are compact vector documents. A four MiB ceiling is
/// deliberately independent from the much larger project/deck limit and is
/// applied before parsing on both native and browser builds.
pub(crate) const MAX_SYMBOL_DEFINITION_IMPORT_BYTES: u64 = 4 * 1024 * 1024;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ModelBoundSymbolDefinition, SymbolDefinitionImport};

    #[test]
    fn import_action_opens_an_isolated_draft_in_the_writable_project_library() {
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;

        open_symbol_import_dialog(&mut state);

        assert!(state.dialogs.symbol_import.open);
        assert_eq!(
            state.dialogs.symbol_import.target_library,
            state.workspace.active_view.library
        );
        assert!(state.dialogs.symbol_import.source_text.is_empty());
        assert!(!state.dialogs.symbol_import.dirty);
    }

    #[test]
    fn form_action_loads_the_selected_typed_definition_as_the_next_revision() {
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;
        let library_name = state.workspace.active_view.library.clone();
        let cell_name = "review_form";
        let template = ModelBoundSymbolDefinition::review_only(&library_name, cell_name);
        let imported = SymbolDefinitionImport::from_bytes(
            br#"<svg><rect x="0" y="0" width="40" height="20"/></svg>"#,
            "review.svg",
            Some(template),
        )
        .expect("typed review definition");
        let library = state
            .library_manager
            .get_library_mut(&library_name)
            .expect("project library");
        imported
            .definition
            .build_plan(library)
            .expect("construction plan")
            .commit(library)
            .expect("commit definition");
        state
            .library_manager
            .select_view(&library_name, cell_name, "symbol");

        open_symbol_parameter_form_dialog(&mut state);

        assert!(state.dialogs.symbol_parameter_form.open);
        assert_eq!(
            state.dialogs.symbol_parameter_form.target,
            Some(CellViewRef::new(&library_name, cell_name, "symbol"))
        );
        assert_eq!(
            state
                .dialogs
                .symbol_parameter_form
                .draft_form
                .as_ref()
                .expect("form draft")
                .revision,
            2
        );
    }
}
