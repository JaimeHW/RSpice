//! Opening a project whose schematic hierarchy no longer holds together.
//!
//! A placement outlives the master it names by design, so neither a missing
//! master nor a loop through one may cost the reader their project. Both are
//! repaired on the way in and both are reported, which is the whole difference
//! between this pass and the layout one it mirrors.

use super::*;

use crate::state::{LibraryCellInstance, Point, SchematicState};

#[test]
fn load_repairs_missing_instance_master() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let top = workspace.active_view.clone();
    let mut binding = LibraryCellInstance::new("user", "ghost", "schematic");
    binding.module_name = Some("ghost".to_owned());
    binding.source_path = Some(std::path::PathBuf::from("cells/ghost.sp"));
    workspace
        .schematic_buffers
        .get_mut(&top.key())
        .expect("the bootstrapped top buffer")
        .add_library_cell_component(Point::new(10, 10), binding);
    let json = serialize_project_file(&ProjectFile::new(workspace, libraries))
        .expect("a project may name a master it has lost");

    let loaded =
        load_project_text(&json, None).expect("a project whose master vanished still opens");

    let restored = loaded
        .workspace
        .schematic_buffers
        .get(&top.key())
        .expect("the parent drawing loads");
    assert_eq!(
        restored.components.len(),
        1,
        "the placement is repaired, not dropped"
    );
    let binding = restored.components[0]
        .library_cell
        .as_ref()
        .expect("it still names the master it wants");
    assert_eq!(binding.cell, "ghost");
    assert!(
        binding.module_name.is_none() && binding.source_path.is_none(),
        "nothing may netlist a master the project no longer holds"
    );
    let warning = loaded
        .workspace_migration_warning
        .expect("a silent repair is not a repair");
    assert!(
        warning.contains("user/ghost/schematic"),
        "the warning has to name the missing master: {warning}"
    );
}

#[test]
fn load_breaks_an_instantiation_cycle_and_names_both_ends() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let top = workspace.active_view.clone();
    let amp = CellViewRef::new("user", "amp", "schematic");
    {
        let library = libraries
            .get_library_mut("user")
            .expect("the default project library");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
    }
    let mut amp_schematic = SchematicState::default();
    amp_schematic.add_library_cell_component(
        Point::new(0, 0),
        LibraryCellInstance::new("user", "top", "schematic"),
    );
    workspace.schematic_buffers.insert(amp.key(), amp_schematic);
    workspace
        .schematic_buffers
        .get_mut(&top.key())
        .expect("the bootstrapped top buffer")
        .add_library_cell_component(
            Point::new(10, 10),
            LibraryCellInstance::new("user", "amp", "schematic"),
        );
    let json = serialize_project_file(&ProjectFile::new(workspace, libraries))
        .expect("a recursive hierarchy can be written");

    let loaded = load_project_text(&json, None).expect("a recursive hierarchy still opens");

    let placements = loaded.workspace.schematic_buffers[&top.key()]
        .components
        .len()
        + loaded.workspace.schematic_buffers[&amp.key()]
            .components
            .len();
    assert_eq!(
        placements, 1,
        "exactly the edge that closes the loop is cut"
    );
    let warning = loaded
        .workspace_migration_warning
        .expect("a silent repair is not a repair");
    assert!(
        warning.contains("user/top/schematic") && warning.contains("user/amp/schematic"),
        "a broken loop has to name both of its ends: {warning}"
    );
}

#[test]
#[cfg(feature = "generated-veriloga-catalog")]
fn unresolved_generated_veriloga_bindings_survive_project_save_and_reload() {
    use crate::state::{
        PersistedGeneratedIdentity, generated_veriloga_library_binding,
        validate_generated_veriloga_binding,
    };
    use rspice_core::device::veriloga_builtins::{
        GENERATED_VERILOGA_COMPATIBILITY_CATALOG, generated_veriloga_model_descriptor,
    };

    let descriptor = generated_veriloga_model_descriptor("vbic13").unwrap();
    let historical = GENERATED_VERILOGA_COMPATIBILITY_CATALOG
        .iter()
        .find(|entry| entry.public_model_name == "vbic13")
        .unwrap();
    assert_ne!(descriptor.checkpoint_identity, historical.semantic_identity);
    for explicit_null in [false, true] {
        let mut binding = generated_veriloga_library_binding(descriptor).unwrap();
        let contract = binding.generated_veriloga.as_mut().unwrap();
        contract.schema_revision = 1;
        contract.descriptor_abi_version = 2;
        contract.source_identity = if explicit_null {
            PersistedGeneratedIdentity::Null
        } else {
            PersistedGeneratedIdentity::Missing
        };
        contract.accepted_state_shape_identity = PersistedGeneratedIdentity::Missing;
        contract.checkpoint_identity = historical
            .wire_v26_combined_identity_alias
            .unwrap()
            .to_owned();
        contract.descriptor_signature = historical
            .wire_ui_v1_descriptor_signature_alias
            .unwrap()
            .to_owned();

        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let top = workspace.active_view.key();
        let schematic = workspace.schematic_buffers.get_mut(&top).unwrap();
        schematic.add_library_cell_component(Point::new(10, 20), binding);
        let before = serde_json::to_value(&schematic.components[0]).unwrap();
        let json = serde_json::to_string(&ProjectFile::new(workspace, libraries)).unwrap();

        let loaded =
            load_project_text(&json, None).expect("an unresolved model must not cost the project");
        let restored = &loaded.workspace.schematic_buffers[&top].components[0];
        assert_eq!(serde_json::to_value(restored).unwrap(), before);
        assert!(
            validate_generated_veriloga_binding(restored.library_cell.as_ref().unwrap()).is_err()
        );
        assert!(
            loaded
                .workspace_migration_warning
                .as_deref()
                .unwrap()
                .contains("Preserved 1 generated Verilog-A binding(s) as unresolved")
        );

        let saved = serialize_project_file(&loaded).expect("unresolved evidence remains saveable");
        let reloaded =
            load_project_text(&saved, None).expect("unresolved evidence remains loadable");
        assert_eq!(
            serde_json::to_value(&reloaded.workspace.schematic_buffers[&top].components[0])
                .unwrap(),
            before,
            "saving must preserve both absent and explicitly null identity fields"
        );
    }
}
