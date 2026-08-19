//! What deleting and copying a cell may not leave behind.
//!
//! A deleted cell must take every document it owned with it, and leave every
//! drawing that placed it honest about the fact that it is gone. A copied cell
//! must take its drawings and leave its review history behind.

use super::*;

use crate::state::ValidatedRevisionJournal;

/// A placement that carries a copy of its master's netlist identity, which is
/// what makes a dangling instance look resolved when it is not.
fn amp_placement_with_netlist_identity(state: &mut AppState) {
    let binding = state.schematic.components[0]
        .library_cell
        .as_mut()
        .expect("the fixture places amp");
    binding.module_name = Some("amp".to_owned());
    binding.source_path = Some(std::path::PathBuf::from("cells/amp.sp"));
}

#[test]
fn copy_cell_resets_the_validated_revision_journal() {
    let mut state = state_with_work_cell("amp");
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(10, 10));
    let project_id = state.workspace.project.id().to_string();
    let accepted = state.schematic.clone();
    state
        .schematic
        .seed_accepted_revision_baseline(&accepted, &project_id, 1, "work/amp/schematic")
        .expect("the source cell has been reviewed once");

    state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("a reviewed cell can be copied");

    let source = state
        .workspace
        .schematic_buffers
        .get(&CellViewRef::new("work", "amp", "schematic").key())
        .expect("the source buffer survives the copy");
    assert_eq!(
        source.validated_revisions.records().len(),
        1,
        "copying must not disturb the original's review history"
    );
    let copy = state
        .workspace
        .schematic_buffers
        .get(&CellViewRef::new("work", "amp_copy", "schematic").key())
        .expect("the copy carries the drawn content");
    assert_eq!(
        copy.components.len(),
        source.components.len(),
        "the copy is the same drawing"
    );
    assert_eq!(
        copy.validated_revisions,
        ValidatedRevisionJournal::default(),
        "a copy has been reviewed by nobody and starts its own history"
    );
}

#[test]
fn deleting_a_cell_removes_the_physical_layout_of_every_view_it_owned() {
    let mut state = state_with_work_cell("amp");
    if let Some(library) = state.library_manager.get_library_mut("work") {
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
            .initialize_physical_layout_document(owner.clone())
            .expect("layout initializes from the exact project PDK pin");
    }

    state
        .library_manager
        .get_library_mut("work")
        .expect("work library")
        .remove_cell("amp");
    state.prune_workspace_after_cell_deleted("work", "amp");

    for owner in &owned {
        assert!(
            state.workspace.physical_layout_document(owner).is_none(),
            "{} outlived the cell that owned it",
            owner.display_path()
        );
    }
    crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("a project whose deleted cell left no orphan layout still saves");
}

#[test]
fn undo_revalidates_instance_bindings() {
    let mut state = state_with_populated_user_library();
    amp_placement_with_netlist_identity(&mut state);
    let placement = state.schematic.components[0].id;
    state.schematic.selection.select_component(placement);
    assert!(
        state.schematic.delete_selection(),
        "the reader takes the placement out of the drawing"
    );

    // The master is deleted while its only placement is out of the drawing,
    // so nothing warns the reader at deletion time.
    state
        .library_manager
        .get_library_mut("user")
        .expect("user library")
        .remove_cell("amp");

    assert!(state.schematic.undo(), "undo puts the placement back");
    state.sync_active_schematic_to_workspace();

    let binding = state.schematic.components[0]
        .library_cell
        .as_ref()
        .expect("the restored placement keeps its binding");
    assert_eq!(
        binding.cell, "amp",
        "it still names the master it wants, so the reader can see what is missing"
    );
    assert!(
        binding.module_name.is_none() && binding.source_path.is_none(),
        "an undo may not resurrect a netlist identity whose master is gone"
    );
    let resolution = state.workspace.resolve_hierarchy(&state.library_manager);
    let placement = resolution
        .bindings
        .iter()
        .find(|binding| binding.reference.cell == "amp")
        .expect("the restored placement is still in the hierarchy receipt");
    assert!(
        !placement.status.is_resolved(),
        "the restored placement reads as unresolved: {}",
        placement.status.label()
    );
}
