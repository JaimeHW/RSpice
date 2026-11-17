//! Copying and renaming cells, libraries and views carries their physical layouts, and
//! creating and editing a layout are steps of the one Undo, run through the whole
//! application.

use super::{state_with_populated_user_library, state_with_work_cell};
use rspice_app_state::state::{
    Cell, CellViewRef, LayoutEdit, LayoutInstance, LayoutObjectId, LayoutOrientation, LayoutPoint,
    LayoutTransform, View, ViewType,
};

#[test]
fn copy_and_rename_cell_preserve_layout_documents_and_remap_hierarchical_masters() {
    let mut state = state_with_work_cell("amp");
    let amp_layout = CellViewRef::new("work", "amp", "layout");
    let wrapper_layout = CellViewRef::new("work", "wrapper", "layout");
    {
        let library = state
            .model
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
        .model
        .initialize_physical_layout_document(amp_layout.clone())
        .expect("amp layout initializes from exact project PDK pin");
    state
        .model
        .initialize_physical_layout_document(wrapper_layout.clone())
        .expect("wrapper layout initializes from exact project PDK pin");

    let mut wrapper = state
        .model
        .workspace
        .content
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
        .model
        .workspace
        .content
        .commit_physical_layout_document(wrapper)
        .expect("hierarchical wrapper layout commits");

    state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("layout-bearing cell copy succeeds");
    let copied_layout = CellViewRef::new("work", "amp_copy", "layout");
    let copied = state
        .model
        .workspace
        .content
        .physical_layout_document(&copied_layout)
        .expect("copied authoritative layout document");
    assert_eq!(copied.owner(), &copied_layout);
    assert_eq!(
        copied.technology(),
        state
            .model
            .workspace
            .content
            .physical_layout_document(&amp_layout)
            .expect("source layout remains")
            .technology()
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
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
            .model
            .workspace
            .content
            .physical_layout_document(&CellViewRef::new("work", "amp", "layout"))
            .is_none()
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
            .physical_layout_document(&renamed_layout)
            .expect("renamed authoritative layout document")
            .owner(),
        &renamed_layout
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
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
            .model
            .workspace
            .content
            .physical_layout_document(&copied_layout)
            .is_some(),
        "independent copied layout remains authoritative"
    );
    rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("post-copy/rename project snapshot validates end to end");
}

#[test]
fn rename_library_carries_the_physical_layouts_that_name_it() {
    let mut state = state_with_populated_user_library();
    state.provision_test_project_technology_contract();
    if let Some(library) = state.model.library_manager.get_library_mut("user") {
        library
            .get_cell_mut("amp")
            .expect("fixture amp cell")
            .add_view(View::new("layout", ViewType::Layout));
    }
    state
        .model
        .initialize_physical_layout_document(CellViewRef::new("user", "amp", "layout"))
        .expect("layout initializes from the exact project PDK pin");

    state
        .rename_library("user", "project_lib")
        .expect("a layout follows its library the way it follows its cell");

    assert!(state.model.library_manager.get_library("user").is_none());
    assert!(
        state
            .model
            .workspace
            .content
            .physical_layout_document(&CellViewRef::new("user", "amp", "layout"))
            .is_none()
    );
    let moved = CellViewRef::new("project_lib", "amp", "layout");
    assert_eq!(
        state
            .model
            .workspace
            .content
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
    if let Some(library) = state.model.library_manager.get_library_mut("user") {
        library
            .get_cell_mut("amp")
            .expect("fixture amp cell")
            .add_view(View::new("layout", ViewType::Layout));
    }
    state
        .model
        .initialize_physical_layout_document(CellViewRef::new("user", "amp", "layout"))
        .expect("layout initializes from the exact project PDK pin");

    state
        .rename_view("user", "amp", "layout", "layout_v2")
        .expect("a layout follows the view that owns it");

    assert!(
        state
            .model
            .workspace
            .content
            .physical_layout_document(&CellViewRef::new("user", "amp", "layout"))
            .is_none()
    );
    let moved = CellViewRef::new("user", "amp", "layout_v2");
    assert_eq!(
        state
            .model
            .workspace
            .content
            .physical_layout_document(&moved)
            .expect("the layout is addressable under the new view identity")
            .owner(),
        &moved
    );
}

/// Creating a Layout view's physical document and each edit to it are steps
/// of the one Undo, behind the library step taken before them: Undo takes
/// the shape back, then the document, then the library, and Redo makes each
/// again in turn.
#[test]
fn creating_and_editing_a_layout_are_steps_of_the_one_undo() {
    use rspice_app_state::product::ObjectRevision;
    use rspice_app_state::state::{
        LayoutGeometry, LayoutLayerPurpose, LayoutShape, PhysicalLayoutDocument,
    };
    use rspice_app_workbench::workbench::framework::command_registry::ids;

    let mut app = crate::workbench::RSpiceApp::test_instance();
    let state = &mut app.state;
    state.provision_test_project_technology_contract();
    let owner = CellViewRef::new("user", "top", "layout");
    state
        .model
        .library_manager
        .get_library_mut("user")
        .expect("the project's library")
        .get_or_create_cell("top")
        .add_view(View::new("layout", ViewType::Layout));
    state
        .model
        .workspace
        .open_view(owner.clone(), ViewType::Layout);
    state.open_project_session();
    state
        .model
        .create_library("extra")
        .expect("the library is created");
    let start = state
        .model
        .initialize_physical_layout_document(owner.clone())
        .expect("the layout is created");
    let shape = LayoutObjectId::new();
    state
        .model
        .apply_physical_layout_transaction(
            &owner,
            start,
            &[LayoutEdit::InsertShape {
                id: shape,
                value: LayoutShape {
                    layer_purpose: LayoutLayerPurpose::try_new("metal1", "drawing").unwrap(),
                    geometry: LayoutGeometry::Rectangle {
                        lower_left: LayoutPoint::new(0, 0),
                        upper_right: LayoutPoint::new(100, 200),
                    },
                    net: None,
                    properties: std::collections::BTreeMap::new(),
                },
            }],
        )
        .expect("the shape is drawn");

    let layout = |app: &crate::workbench::RSpiceApp| -> Option<(ObjectRevision, usize)> {
        app.state
            .model
            .workspace
            .content
            .physical_layout_document(&owner)
            .map(|document: &PhysicalLayoutDocument| (document.revision(), document.shapes().len()))
    };
    let extra = |app: &crate::workbench::RSpiceApp| {
        app.state
            .model
            .library_manager
            .get_library("extra")
            .is_some()
    };
    let drawn = layout(&app);
    assert_eq!(drawn.map(|(_, shapes)| shapes), Some(1));

    app.execute_command_key(ids::UNDO);
    assert_eq!(layout(&app), Some((start, 0)), "Undo takes the shape back");
    app.execute_command_key(ids::UNDO);
    assert_eq!(layout(&app), None, "then the layout");
    assert!(extra(&app), "the library step stands behind them");
    app.execute_command_key(ids::UNDO);
    assert!(!extra(&app), "and undoes in its turn");

    for _ in 0..3 {
        app.execute_command_key(ids::REDO);
    }
    assert!(extra(&app));
    assert_eq!(layout(&app), drawn, "Redo makes each again");
    assert!(
        app.state
            .model
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == owner && open.dirty),
        "the layout's tab is unsaved"
    );
}
