//! Opening a symbol definition through the whole application: the import and form actions
//! open a project session and read the platform's file access.

use crate::workbench::app::dialogs::symbol_definition::{
    open_symbol_import_dialog, open_symbol_parameter_form_dialog,
};
use crate::workbench::app_state::AppState;
use rspice_app_state::state::CellViewRef;
use rspice_app_state::state::{ModelBoundSymbolDefinition, SymbolDefinitionImport};

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

/// Importing a symbol opens what it made in the symbol editor, as Create
/// symbol does: the dialog's primary button publishes it and the symbol is
/// the view in front of the Design workspace.
#[test]
fn importing_a_symbol_opens_it_in_the_symbol_editor() {
    use crate::feature_tests::replay::Replay;
    use crate::workbench::app::dialogs::symbol_definition::state::SymbolImportBindingChoice;
    use crate::workbench::state::Workspace;
    use egui::accesskit::Role;

    let mut replay = Replay::new();
    let state = &mut replay.app.state;
    open_symbol_import_dialog(
        &mut state.design.symbol_import,
        &state.model,
        &mut state.workbench,
        &state.platform,
    );
    let library = state.design.symbol_import.target_library.clone();
    state.design.symbol_import.accept_source(
        "review.svg".to_owned(),
        r#"<svg><rect x="0" y="0" width="40" height="20"/></svg>"#.to_owned(),
    );
    state.design.symbol_import.binding_choice = SymbolImportBindingChoice::UnboundForReview;
    replay.click("Import into project library", Role::Button);

    let state = &replay.app.state;
    assert!(!state.design.symbol_import.open, "the import finished");
    assert_eq!(
        state.model.workspace.content.active_view,
        CellViewRef::new(&library, "review", "symbol"),
        "the imported symbol is in front"
    );
    assert_eq!(state.workbench.workspace, Workspace::Design);
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
    rspice_app_state::state::prepare_symbol_construction(&imported.definition, library)
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
