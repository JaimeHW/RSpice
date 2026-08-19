//! What deleting and copying a cell may not leave behind.
//!
//! A deleted cell must take every document it owned with it, and leave every
//! drawing that placed it honest about the fact that it is gone. A copied cell
//! must take its drawings and leave its review history behind.

use super::*;

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
