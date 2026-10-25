//! The persisted project format, through the session that builds and restores
//! it and the runs that produce its evidence.
//!
//! Each case here needs the whole application: a layout made through the
//! session, a session restored from its own serialization, or a Monte Carlo
//! checkpoint a run completed.

use crate::io::project_io::tests::{project_with_execution_context, seal_prepared_run};
use crate::io::project_io::*;
use crate::product::ObjectRevision;
use crate::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisType, Cell, CellViewRef, LayoutEdit,
    LayoutInstance, LayoutObjectId, LayoutOrientation, LayoutPoint, LayoutTransform,
    PreparedSourceCheckReceipt, SimulationRun, SimulationRunLifecycle, SimulationState, View,
    ViewType,
};
use crate::state::{LibraryManager, ProjectWorkspace};
use crate::workbench::app_state::AppState;
use rspice_results::provenance::AnalysisResultSourceDomain;
use rspice_vocabulary::hierarchy_path::InstancePath;
use rspice_vocabulary::product::ContentDigest;

mod annotation;
mod monte_carlo;

fn project_with_two_authoritative_layout_documents() -> (ProjectSnapshot, CellViewRef, CellViewRef)
{
    let mut state = AppState::default();
    state.provision_test_project_technology_contract();
    let top = CellViewRef::new("user", "top", "layout");
    let child = CellViewRef::new("user", "child", "layout");
    {
        let library = state
            .model
            .library_manager
            .get_library_mut("user")
            .expect("default project library");
        library
            .get_cell_mut("top")
            .expect("default top cell")
            .add_view(View::new("layout", ViewType::Layout));
        let mut child_cell = Cell::new("child");
        child_cell.add_view(View::new("layout", ViewType::Layout));
        library.add_cell(child_cell);
    }
    state
        .initialize_physical_layout_document(top.clone())
        .expect("top layout initializes");
    state
        .initialize_physical_layout_document(child.clone())
        .expect("child layout initializes");
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("two-layout project snapshot validates");
    (project, top, child)
}

fn insert_layout_instance(project: &mut ProjectSnapshot, owner: &CellViewRef, master: CellViewRef) {
    let mut document = project
        .file
        .workspace
        .physical_layout_document(owner)
        .expect("layout owner document")
        .clone();
    let revision = document.revision();
    document
        .apply_transaction(
            revision,
            &[LayoutEdit::InsertInstance {
                id: LayoutObjectId::new(),
                value: LayoutInstance {
                    master,
                    transform: LayoutTransform {
                        origin: LayoutPoint::new(0, 0),
                        orientation: LayoutOrientation::R0,
                    },
                    array: None,
                    terminal_bindings: Default::default(),
                    properties: Default::default(),
                },
            }],
        )
        .expect("layout instance transaction is locally valid");
    project
        .file
        .workspace
        .commit_physical_layout_document(document)
        .expect("layout document commits");
}

#[test]
fn project_validation_rejects_layout_hierarchy_without_authoritative_master_document() {
    let (mut project, top, child) = project_with_two_authoritative_layout_documents();
    insert_layout_instance(&mut project, &top, child.clone());
    assert!(
        project
            .file
            .workspace
            .remove_physical_layout_document(&child)
    );

    let error = project
        .file
        .validate()
        .expect_err("missing authoritative layout master must fail closed");
    assert!(
        error
            .to_string()
            .contains("no authoritative physical-layout document"),
        "{error}"
    );
}

#[test]
fn project_validation_rejects_recursive_physical_layout_hierarchy() {
    let (mut project, top, child) = project_with_two_authoritative_layout_documents();
    insert_layout_instance(&mut project, &top, child.clone());
    insert_layout_instance(&mut project, &child, top);

    let error = project
        .file
        .validate()
        .expect_err("recursive layout hierarchy must fail closed");
    assert!(error.to_string().contains("recursive cycle"), "{error}");
}
