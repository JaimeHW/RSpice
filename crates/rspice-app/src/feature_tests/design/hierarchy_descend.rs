//! Descending into a placed instance through the whole application: the descent dialog's
//! transaction is answered as the shell answers it.

use crate::workbench::app_state::AppState;
use rspice_app_design::workbench::app::dialogs::hierarchy::descend::*;
use rspice_app_state::state::{
    Cell, CellViewRef, Component, ComponentType, Library, LibraryCellInstance, Point, View,
    ViewType,
};
use rspice_app_state::workbench::preferences::TogglePreference;

/// Apply the choice as the dialog's step and the shell's answer do,
/// straight after one another.
fn commit_descend_context(state: &mut AppState) -> Result<(), String> {
    let request = descend_request(&state.design.descend_hierarchy, &state.model)?;
    state.design.descend_hierarchy.close();
    answer_descend_hierarchy(state.navigation(), request);
    Ok(())
}

pub(crate) fn hierarchy_state() -> (AppState, CellViewRef, CellViewRef) {
    let mut state = AppState::default();
    let parent = state.model.workspace.content.active_view.clone();
    let child = CellViewRef::new("hierarchy_test", "afe_core", "schematic");
    let mut cell = Cell::new("afe_core");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    let mut library = Library::new("hierarchy_test");
    library.add_cell(cell);
    state.model.library_manager.add_library(library);

    let mut instance =
        Component::new(91, ComponentType::CellInstance, Point::new(100, 100)).with_library_cell(
            LibraryCellInstance::new("hierarchy_test", "afe_core", "schematic"),
        );
    instance.name = "XAFE".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(instance);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(91);
    (state, parent, child)
}

#[test]
fn menu_transaction_opens_without_navigating_and_rejects_selection_drift() {
    let (mut state, parent, _) = hierarchy_state();
    assert!(open_descend_hierarchy_dialog(
        &mut state.design.descend_hierarchy,
        &state.model,
        &mut state.workbench,
    ));
    assert!(state.design.descend_hierarchy.open);
    assert_eq!(state.model.workspace.content.active_view, parent);

    state.model.schematic.session.editor.selection.clear();
    let error = commit_descend_context(&mut state).expect_err("selection drift must block");
    assert!(error.contains("selected-object set changed"));
    assert_eq!(state.model.workspace.content.active_view, parent);
    assert!(state.design.descend_hierarchy.open);
}

#[test]
fn preference_sets_the_workflow_default_without_removing_explicit_choices() {
    let (mut state, _, child) = hierarchy_state();
    state
        .model
        .preferences
        .set_toggle(TogglePreference::HierarchicalEditInPlace, false);
    assert!(open_descend_hierarchy_dialog(
        &mut state.design.descend_hierarchy,
        &state.model,
        &mut state.workbench,
    ));
    assert_eq!(
        state.design.descend_hierarchy.edit_mode,
        HierarchyDescendEditMode::OpenIsolated
    );

    state.design.descend_hierarchy.edit_mode = HierarchyDescendEditMode::EditInPlace;
    commit_descend_context(&mut state).expect("explicit edit-in-place choice remains valid");

    assert_eq!(state.model.workspace.content.active_view, child);
    assert_eq!(
        state
            .model
            .workspace
            .content
            .occurrence_labels()
            .last()
            .unwrap(),
        "XAFE"
    );
}

#[test]
fn the_symbol_mode_edits_the_instance_symbol_on_this_sheet() {
    let (mut state, parent, _) = hierarchy_state();
    assert!(open_descend_hierarchy_dialog(
        &mut state.design.descend_hierarchy,
        &state.model,
        &mut state.workbench,
    ));
    state.design.descend_hierarchy.edit_mode = HierarchyDescendEditMode::EditSymbolInPlace;
    let error = commit_descend_context(&mut state).expect_err("afe_core has no symbol view");
    assert!(error.contains("the cell has no symbol view"), "{error}");
    assert_eq!(state.model.workspace.content.active_view, parent);

    state
        .model
        .library_manager
        .get_library_mut("hierarchy_test")
        .and_then(|library| library.get_cell_mut("afe_core"))
        .expect("the instance's cell")
        .add_view(View::new("symbol", ViewType::Symbol));
    commit_descend_context(&mut state).expect("the symbol is editable");

    assert!(!state.design.descend_hierarchy.open);
    assert_eq!(
        state.model.workspace.content.active_view,
        CellViewRef::new("hierarchy_test", "afe_core", "symbol")
    );
    assert_eq!(
        rspice_app_design::workbench::app::actions::workspace::symbol_in_place::in_place_session(
            &state.design.symbol.in_place,
            &state.model
        )
        .map(|session| &session.parent),
        Some(&parent),
        "the symbol is edited on the sheet, not descended into"
    );
}

#[test]
fn read_only_reference_belongs_to_its_document_and_never_mutates_library_ownership() {
    let (mut state, parent, child) = hierarchy_state();
    assert!(open_descend_hierarchy_dialog(
        &mut state.design.descend_hierarchy,
        &state.model,
        &mut state.workbench,
    ));
    state.design.descend_hierarchy.edit_mode = HierarchyDescendEditMode::ReadOnlyReference;
    commit_descend_context(&mut state).expect("valid reference context");

    assert_eq!(state.model.workspace.content.active_view, child);
    assert!(state.model.workspace.content.active_read_only_reference());
    assert!(state.model.active_view_read_only());
    assert!(!state.model.schematic.session.read_only);
    assert!(
        !state
            .model
            .library_manager
            .get_library("hierarchy_test")
            .expect("library")
            .read_only
    );

    state.open_workspace_view(parent);
    assert!(
        !state.model.workspace.content.active_read_only_reference(),
        "the parent was never opened as a reference"
    );
    assert!(!state.model.active_view_read_only());

    state.open_workspace_view(child);
    assert!(
        state.model.workspace.content.active_read_only_reference(),
        "returning to the reference document still refuses writes"
    );
    assert!(
        !rspice_app_design::workbench::app::actions::workspace::symbol_document::symbol_editor_copy_available(&state.model),
        "the symbol editor's escape hatch follows the same document"
    );
}
