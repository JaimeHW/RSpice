//! The persisted project format, through the session that builds and restores
//! it and the runs that produce its evidence.
//!
//! Each case here needs the whole application: a layout made through the
//! session, a session restored from its own serialization, or a Monte Carlo
//! checkpoint a run completed.

use crate::io::project_io::*;
use crate::product::{ObjectRevision, SimulationPlanId};
use crate::simulation::dialog::{DampingStrategy, IntegrationMethod, MatrixSolver};
use crate::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisType, Cell, CellViewRef, LayoutEdit,
    LayoutInstance, LayoutObjectId, LayoutOrientation, LayoutPoint, LayoutTransform,
    PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRun,
    SimulationRunLifecycle, SimulationRunProvenance, SimulationState, View, ViewType,
};
use crate::state::{LibraryManager, ProjectWorkspace};
use crate::workbench::app_state::AppState;
use rspice_results::provenance::AnalysisResultSourceDomain;
use rspice_simulation_contract::analysis_kind::AnalysisKind;
use rspice_vocabulary::hierarchy_path::InstancePath;
use rspice_vocabulary::product::ContentDigest;

mod annotation;
mod monte_carlo;

fn project_with_execution_context() -> ProjectSnapshot {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelLibraryManager, ModelType};
    let ac = AnalysisKind::Ac.legacy_index();
    let noise = AnalysisKind::Noise.legacy_index();
    let transient = AnalysisKind::Transient.legacy_index();

    let mut design_libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut design_libraries);

    let mut setup = crate::workbench::app_state::SimSetupState::new();
    setup.enabled.extend([ac, noise]);
    setup.analysis_order = vec![noise, transient, ac];
    setup.listed.extend([ac, noise]);
    setup
        .set_reference_pvt(crate::product::ProcessCorner::FF, -40.0)
        .expect("fixture PVT is valid");
    setup.tran.stop = "25u".to_owned();
    setup.tran.step = "2n".to_owned();
    setup.tran.start = "1u".to_owned();
    setup.tran.max_step = "100n".to_owned();
    setup.tran.uic = true;
    setup.ac.fstart = "10".to_owned();
    setup.ac.fstop = "8G".to_owned();
    setup.ac.points = "77".to_owned();
    setup.ac.sweep = 1;
    setup.noise.output = "vout".to_owned();
    setup.noise.reference = "vref".to_owned();
    setup.noise.input = "VIN".to_owned();
    setup.noise.fstart = "10".to_owned();
    setup.noise.fstop = "5G".to_owned();
    setup.disto_f2_over_f1 = "0.91".to_owned();
    setup.options.reltol = 2e-4;
    setup.options.residual_reltol = 3e-4;
    setup.options.vntol = 4e-7;
    setup.options.abstol = 5e-13;
    setup.options.iabstol = 6e-13;
    setup.options.chgtol = 7e-15;
    setup.options.pivrel = 8e-4;
    setup.options.pivtol = 9e-14;
    setup.options.itl1 = 80;
    setup.options.itl4 = 12;
    setup.options.gmin_stepping = false;
    setup.options.source_stepping = false;
    setup.options.pseudo_transient = false;
    setup.options.arc_length = true;
    setup.options.gmin = 2e-12;
    setup.options.damping = DampingStrategy::Combined;
    setup.options.method = IntegrationMethod::Gear2;
    setup.options.solver = MatrixSolver::SparseLu;
    setup.options.bypass_enabled = true;
    setup.options.bypass_reltol = 5e-4;
    setup.options.bypass_abstol = 5e-7;
    setup.options.min_timestep = 2e-15;
    setup.options.max_timestep = 2e-3;
    setup.options.tnom = 25.0;

    let mut model_manager = ModelLibraryManager::new();
    let mut model_library = ModelLibrary::new("fixture_models");
    model_library.pdk_name = "fixture_pdk".to_owned();
    model_library.technology_node = "90nm".to_owned();
    model_library.version = "2.1".to_owned();
    let mut model = DeviceModel::new("nch_fixture", ModelType::Nmos);
    model.add_parameter("kp", 1.25e-3);
    model_library.add_model(model);
    model_manager.add_library(model_library);

    // Exercise the deterministic singleton-to-instance migration while
    // retaining every legacy fixture edit above.
    setup.analysis_plan = None;
    setup
        .migrate_legacy_analysis_plan(workspace.content.project.id())
        .expect("legacy execution fixture migrates at the load boundary");
    let execution_context = crate::io::capture_execution_context(&setup, &model_manager)
        .expect("execution fixture validates");
    ProjectSnapshot::new_with_execution_context(
        workspace,
        design_libraries,
        ProjectSimulationResults::default(),
        execution_context,
    )
}

fn seal_prepared_run(
    run: &mut SimulationRun,
    source_domain: AnalysisResultSourceDomain,
    simulation_plan_id: Option<SimulationPlanId>,
    project_revision: ObjectRevision,
    source_content_digest: ContentDigest,
    source_check_receipt: PreparedSourceCheckReceipt,
    analysis_kind_tags: &[u8],
) {
    assert_eq!(run.analyses.len(), analysis_kind_tags.len());
    let prepared_snapshot_digest = run
        .analyses
        .first()
        .and_then(|analysis| analysis.provenance.as_ref())
        .expect("prepared fixture has provenance")
        .prepared_snapshot_digest();
    let tasks = run
        .analyses
        .iter()
        .zip(analysis_kind_tags)
        .enumerate()
        .map(|(index, (analysis, kind_tag))| {
            let provenance = analysis.provenance.as_ref().expect("prepared provenance");
            PreparedRunTaskReceipt::new(
                provenance.source_instance_id(),
                provenance.source_revision(),
                provenance.dependency_ids().to_vec(),
                *kind_tag,
                ContentDigest::from_bytes([0xc0_u8.wrapping_add(index as u8); 32]),
            )
            .expect("prepared task receipt")
        })
        .collect::<Vec<_>>();
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain,
        simulation_plan_id,
        project_revision,
        prepared_snapshot_digest,
        source_content_digest,
        source_check_receipt,
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks,
    })
    .expect("prepared run receipt");
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("prepared fixture seals explicitly");
}

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
