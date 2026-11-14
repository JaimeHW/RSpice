//! Opening a symbol definition through the whole application: the import and form actions
//! open a project session and read the platform's file access.

use crate::state::CellViewRef;
use crate::state::{ModelBoundSymbolDefinition, SymbolDefinitionImport};
use crate::workbench::app::dialogs::symbol_definition::{
    open_symbol_import_dialog, open_symbol_parameter_form_dialog,
};
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
