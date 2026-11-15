//! Copying a cell leaves its review history behind and deleting one takes its layouts,
//! run through the whole application.

use super::state_with_work_cell;
use rspice_app_state::state::{
    CellViewRef, ComponentType, Point, ValidatedRevisionJournal, View, ViewType,
};

#[test]
fn copy_cell_resets_the_validated_revision_journal() {
    let mut state = state_with_work_cell("amp");
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(10, 10));
    let project_id = state.model.workspace.content.project.id().to_string();
    let accepted = state.model.schematic.clone();
    state
        .model
        .schematic
        .seed_accepted_revision_baseline(&accepted, &project_id, 1, "work/amp/schematic")
        .expect("the source cell has been reviewed once");

    state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("a reviewed cell can be copied");

    let source = state
        .model
        .workspace
        .content
        .schematic_buffers
        .get(&CellViewRef::new("work", "amp", "schematic").key())
        .expect("the source buffer survives the copy");
    assert_eq!(
        source.document().validated_revisions.records().len(),
        1,
        "copying must not disturb the original's review history"
    );
    let copy = state
        .model
        .workspace
        .content
        .schematic_buffers
        .get(&CellViewRef::new("work", "amp_copy", "schematic").key())
        .expect("the copy carries the drawn content");
    assert_eq!(
        copy.document().components.len(),
        source.document().components.len(),
        "the copy is the same drawing"
    );
    assert_eq!(
        copy.document().validated_revisions,
        ValidatedRevisionJournal::default(),
        "a copy has been reviewed by nobody and starts its own history"
    );
}

#[test]
fn deleting_a_cell_removes_the_physical_layout_of_every_view_it_owned() {
    let mut state = state_with_work_cell("amp");
    if let Some(library) = state.model.library_manager.get_library_mut("work") {
        let cell = library.get_cell_mut("amp").expect("fixture amp cell");
        cell.add_view(View::new("layout", ViewType::Layout));
        cell.add_view(View::new("layout_alt", ViewType::Layout));
    }
    state.provision_test_project_technology_contract();
    let owned = [
        CellViewRef::new("work", "amp", "layout"),
        CellViewRef::new("work", "amp", "layout_alt"),
    ];
    for owner in &owned {
        state
            .model
            .initialize_physical_layout_document(owner.clone())
            .expect("layout initializes from the exact project PDK pin");
    }

    state
        .model
        .library_manager
        .get_library_mut("work")
        .expect("work library")
        .remove_cell("amp");
    state.prune_workspace_after_cell_deleted("work", "amp");

    for owner in &owned {
        assert!(
            state
                .model
                .workspace
                .content
                .physical_layout_document(owner)
                .is_none(),
            "{} outlived the cell that owned it",
            owner.display_path()
        );
    }
    rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("a project whose deleted cell left no orphan layout still saves");
}
