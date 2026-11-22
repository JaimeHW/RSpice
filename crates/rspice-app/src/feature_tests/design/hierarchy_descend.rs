//! Descending into a placed instance through the whole application: the descent dialog's
//! transaction is answered as the shell answers it.

use crate::workbench::app_state::AppState;
use rspice_app_design::workbench::app::dialogs::hierarchy::descend::*;
use rspice_app_state::state::{
    Cell, CellViewRef, Component, ComponentType, Library, LibraryCellInstance, Point, View,
    ViewType,
};
use rspice_app_state::workbench::preferences::TogglePreference;
use rspice_app_state::workbench::workspace::Workspace;

use crate::feature_tests::replay::Replay;
use crate::workbench::RSpiceApp;
use egui::accesskit::Role;

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

/// A read-only reference was the reader's own choice, so the banner over its
/// canvas makes the document editable in place.
#[test]
fn make_editable_on_the_banner_lifts_a_read_only_reference() {
    let (mut state, _, child) = hierarchy_state();
    assert!(open_descend_hierarchy_dialog(
        &mut state.design.descend_hierarchy,
        &state.model,
        &mut state.workbench,
    ));
    state.design.descend_hierarchy.edit_mode = HierarchyDescendEditMode::ReadOnlyReference;
    commit_descend_context(&mut state).expect("a read-only look is navigation");
    assert!(state.model.active_view_read_only());

    let mut app = RSpiceApp::test_instance();
    app.state = state;
    app.state.workbench.activate(Workspace::Design);
    let mut replay = Replay::of(app);
    replay.frames(vec![]);
    replay.click("Make editable", Role::Button);

    let state = &replay.app.state;
    assert_eq!(state.model.workspace.content.active_view, child);
    assert!(!state.model.active_view_read_only());
}

/// A port added inside a block leaves both of its instances in the parent
/// behind. Returning to the parent names them and the step that repairs
/// them, and that step brings both up to date as one undo step.
#[test]
fn returning_from_a_block_whose_ports_changed_names_its_instances_and_one_step_updates_them() {
    use crate::workbench::examples::hierarchy_reference::{self, AMP_INSTANCES};

    let reference = hierarchy_reference::build();
    let amp = reference.amp_schematic.clone();
    let mut state = reference.state;
    state.open_workspace_view(reference.top.clone());
    state
        .navigation()
        .descend_into_instance(Some(AMP_INSTANCES[0].to_owned()), amp);
    let port = state
        .model
        .schematic
        .add_component(ComponentType::Port, Point::new(900, 900));
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port)
        .expect("the placed port is retained")
        .value = "trim".to_owned();

    state.navigation().ascend_workspace_level();

    let notice = state.prompt_text().unwrap_or_default().to_owned();
    assert!(
        notice.contains("X1, X2") && notice.contains("Update all instances"),
        "{notice}"
    );
    let stale = |state: &AppState| {
        state.model.schematic.stale_instance_interfaces(
            &state.model.library_manager,
            &state.model.workspace.content.schematic_buffers,
        )
    };
    assert_eq!(stale(&state), AMP_INSTANCES);

    let summary = state
        .model
        .schematic
        .update_all_instance_interfaces(
            &state.model.library_manager,
            &state.model.workspace.content.schematic_buffers,
        )
        .expect("both instances update");
    assert!(summary.contains("Updated 2 instances"), "{summary}");
    assert!(stale(&state).is_empty());
    for name in AMP_INSTANCES {
        let binding = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.name == name)
            .and_then(|component| component.library_cell.as_ref())
            .expect("the instance stays bound");
        assert!(
            binding
                .terminal_order
                .iter()
                .any(|terminal| terminal == "trim"),
            "{name} presents the new port: {:?}",
            binding.terminal_order
        );
    }

    assert!(state.model.schematic.undo());
    assert_eq!(
        stale(&state),
        AMP_INSTANCES,
        "one undo puts both instances back as they were"
    );
}
