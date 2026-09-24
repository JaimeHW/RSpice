//! Copy and rename paths for the project's physical layout documents.
//!
//! A layout is keyed by the exact Library/Cell/View identity that owns it, so
//! every identity move has to carry it: the document's own owner, and every
//! master that a layout elsewhere in the project places from that identity. A
//! move leaving either behind orphans a layout no surface can address again.

use super::*;

#[test]
fn copy_and_rename_cell_preserve_layout_documents_and_remap_hierarchical_masters() {
    let mut state = state_with_work_cell("amp");
    let amp_layout = CellViewRef::new("work", "amp", "layout");
    let wrapper_layout = CellViewRef::new("work", "wrapper", "layout");
    {
        let library = state
            .library_manager
            .get_library_mut("work")
            .expect("work library");
        library
            .get_cell_mut("amp")
            .expect("amp cell")
            .add_view(View::new("layout", ViewType::Layout));
        let mut wrapper = Cell::new("wrapper");
        wrapper.add_view(View::new("schematic", ViewType::Schematic));
        wrapper.add_view(View::new("layout", ViewType::Layout));
        library.add_cell(wrapper);
    }
    state.provision_test_project_technology_contract();
    state
        .initialize_physical_layout_document(amp_layout.clone())
        .expect("amp layout initializes from exact project PDK pin");
    state
        .initialize_physical_layout_document(wrapper_layout.clone())
        .expect("wrapper layout initializes from exact project PDK pin");

    let mut wrapper = state
        .workspace
        .physical_layout_document(&wrapper_layout)
        .expect("wrapper layout document")
        .clone();
    let wrapper_revision = wrapper.revision();
    wrapper
        .apply_transaction(
            wrapper_revision,
            &[LayoutEdit::InsertInstance {
                id: LayoutObjectId::new(),
                value: LayoutInstance {
                    master: amp_layout.clone(),
                    transform: LayoutTransform {
                        origin: LayoutPoint::new(1_000, 2_000),
                        orientation: LayoutOrientation::R90,
                    },
                    array: None,
                    terminal_bindings: Default::default(),
                    properties: Default::default(),
                },
            }],
        )
        .expect("hierarchical instance transaction");
    state
        .workspace
        .commit_physical_layout_document(wrapper)
        .expect("hierarchical wrapper layout commits");

    state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("layout-bearing cell copy succeeds");
    let copied_layout = CellViewRef::new("work", "amp_copy", "layout");
    let copied = state
        .workspace
        .physical_layout_document(&copied_layout)
        .expect("copied authoritative layout document");
    assert_eq!(copied.owner(), &copied_layout);
    assert_eq!(
        copied.technology(),
        state
            .workspace
            .physical_layout_document(&amp_layout)
            .expect("source layout remains")
            .technology()
    );
    assert_eq!(
        state
            .workspace
            .physical_layout_document(&wrapper_layout)
            .expect("wrapper remains")
            .instances()
            .values()
            .next()
            .expect("wrapper instance")
            .master,
        amp_layout,
        "copy must not retarget existing hierarchy"
    );

    state
        .rename_cell("work", "amp", "amp_renamed")
        .expect("layout-bearing cell rename succeeds");
    let renamed_layout = CellViewRef::new("work", "amp_renamed", "layout");
    assert!(
        state
            .workspace
            .physical_layout_document(&CellViewRef::new("work", "amp", "layout"))
            .is_none()
    );
    assert_eq!(
        state
            .workspace
            .physical_layout_document(&renamed_layout)
            .expect("renamed authoritative layout document")
            .owner(),
        &renamed_layout
    );
    assert_eq!(
        state
            .workspace
            .physical_layout_document(&wrapper_layout)
            .expect("wrapper remains after rename")
            .instances()
            .values()
            .next()
            .expect("wrapper instance after rename")
            .master,
        renamed_layout,
        "rename must retarget every hierarchical layout reference"
    );
    assert!(
        state
            .workspace
            .physical_layout_document(&copied_layout)
            .is_some(),
        "independent copied layout remains authoritative"
    );
    crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("post-copy/rename project snapshot validates end to end");
}

#[test]
fn rename_library_carries_the_physical_layouts_that_name_it() {
    let mut state = state_with_populated_user_library();
    state.provision_test_project_technology_contract();
    if let Some(library) = state.library_manager.get_library_mut("user") {
        library
            .get_cell_mut("amp")
            .expect("fixture amp cell")
            .add_view(View::new("layout", ViewType::Layout));
    }
    state
        .initialize_physical_layout_document(CellViewRef::new("user", "amp", "layout"))
        .expect("layout initializes from the exact project PDK pin");

    state
        .rename_library("user", "project_lib")
        .expect("a layout follows its library the way it follows its cell");

    assert!(state.library_manager.get_library("user").is_none());
    assert!(
        state
            .workspace
            .physical_layout_document(&CellViewRef::new("user", "amp", "layout"))
            .is_none()
    );
    let moved = CellViewRef::new("project_lib", "amp", "layout");
    assert_eq!(
        state
            .workspace
            .physical_layout_document(&moved)
            .expect("the layout is addressable under the new library identity")
            .owner(),
        &moved
    );
}

#[test]
fn rename_view_carries_the_physical_layouts_that_name_it() {
    let mut state = state_with_populated_user_library();
    state.provision_test_project_technology_contract();
    if let Some(library) = state.library_manager.get_library_mut("user") {
        library
            .get_cell_mut("amp")
            .expect("fixture amp cell")
            .add_view(View::new("layout", ViewType::Layout));
    }
    state
        .initialize_physical_layout_document(CellViewRef::new("user", "amp", "layout"))
        .expect("layout initializes from the exact project PDK pin");

    state
        .rename_view("user", "amp", "layout", "layout_v2")
        .expect("a layout follows the view that owns it");

    assert!(
        state
            .workspace
            .physical_layout_document(&CellViewRef::new("user", "amp", "layout"))
            .is_none()
    );
    let moved = CellViewRef::new("user", "amp", "layout_v2");
    assert_eq!(
        state
            .workspace
            .physical_layout_document(&moved)
            .expect("the layout is addressable under the new view identity")
            .owner(),
        &moved
    );
}
