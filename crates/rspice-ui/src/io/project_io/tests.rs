//! Round-trip and rejection tests for the persisted project format.
//!
//! The suite pins two things that a project file must never do: silently drop
//! a field it did not understand, and load a record whose stored digest does
//! not match its contents. Most cases here are therefore negative — they
//! assert a specific refusal, not just a successful round trip.

use super::*;
use crate::state::{
    AnalysisResult, AnalysisType, Cell, CellViewRef, LayoutEdit, LayoutInstance, LayoutObjectId,
    LayoutOrientation, LayoutPoint, LayoutTransform, OpenCellView, OperatingPointValue,
    PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRun,
    SimulationRunProvenance, SimulationState, View, ViewType, WaveformData,
};
use crate::workbench::app_state::AppState;

#[test]
fn in_memory_project_text_is_size_checked_before_parsing() {
    assert!(validate_project_text_size(MAX_PROJECT_FILE_BYTES as usize).is_ok());
    let error = validate_project_text_size(MAX_PROJECT_FILE_BYTES as usize + 1)
        .expect_err("oversized project text is rejected");
    assert!(matches!(error, ProjectIoError::InvalidData(_)));
    assert!(error.to_string().contains("supported maximum"));
    assert!(validate_legacy_project_text_size(MAX_LEGACY_PROJECT_FILE_BYTES as usize).is_ok());
    let legacy_error =
        validate_legacy_project_text_size(MAX_LEGACY_PROJECT_FILE_BYTES as usize + 1)
            .expect_err("oversized legacy materialization is rejected");
    assert!(matches!(legacy_error, ProjectIoError::InvalidData(_)));
    assert!(legacy_error.to_string().contains("identity injection"));
}

#[test]
fn current_project_text_routes_to_direct_deserialization() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectFile::new(workspace, libraries);
    let json = serialize_project_file(&project).expect("current project serializes");

    assert_eq!(
        project_text_load_route(&json).expect("current route probes"),
        ProjectTextLoadRoute::Direct
    );
    assert_eq!(
        project_text_load_route(r#"{"workspace":{"project":{"schema_version":null,"id":null}}}"#)
            .expect("present null fields still probe"),
        ProjectTextLoadRoute::Direct,
        "legacy ID injection is permitted only when both keys are absent"
    );
}

fn project_with_two_authoritative_layout_documents() -> (ProjectFile, CellViewRef, CellViewRef) {
    let mut state = AppState::default();
    state.provision_test_project_technology_contract();
    let top = CellViewRef::new("user", "top", "layout");
    let child = CellViewRef::new("user", "child", "layout");
    {
        let library = state
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

fn insert_layout_instance(project: &mut ProjectFile, owner: &CellViewRef, master: CellViewRef) {
    let mut document = project
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
        .workspace
        .commit_physical_layout_document(document)
        .expect("layout document commits");
}

#[test]
fn project_validation_rejects_layout_hierarchy_without_authoritative_master_document() {
    let (mut project, top, child) = project_with_two_authoritative_layout_documents();
    insert_layout_instance(&mut project, &top, child.clone());
    assert!(project.workspace.remove_physical_layout_document(&child));

    let error = project
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
        .validate()
        .expect_err("recursive layout hierarchy must fail closed");
    assert!(error.to_string().contains("recursive cycle"), "{error}");
}

fn seal_legacy_unattributed(run: &mut SimulationRun) {
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy fixture seals explicitly");
}

fn operating_point_payload_fixture() -> AnalysisResultPayload {
    AnalysisResultPayload::OperatingPoint {
        temperature_mode: crate::state::OperatingPointTemperatureEvidence::PvtRunSet,
        temperature_celsius: 27.0,
        initial_guess: crate::state::OperatingPointInitialGuessEvidence::Automatic,
        node_initialization:
            crate::state::OperatingPointNodeInitializationEvidence::UseIcAndNodeset,
        homotopy: crate::state::OperatingPointHomotopyEvidence::Adaptive,
        annotation: crate::state::OperatingPointAnnotationEvidence::VoltagesAndCurrents,
        device_detail: crate::state::OperatingPointDeviceDetailEvidence::SelectedAndViolations,
        save_device_op: crate::state::OperatingPointSaveDeviceEvidence::Enabled,
        accuracy: crate::state::OperatingPointAccuracyEvidence::Balanced,
        selected_devices: Vec::new(),
        violation_devices: Vec::new(),
        violation_source_content_digest: None,
        validated_startup_directives: 0,
        mna_node_names: vec!["out".to_owned()],
        mna_branch_names: Vec::new(),
        mna_solution: vec![1.0],
        effective_source_content_digest: Some(ContentDigest::from_bytes([0x70; 32])),
        run_point_index: 0,
        run_point_count: 1,
        run_point_process: crate::state::OperatingPointProcessEvidence::TT,
        run_point_supply_voltage: None,
        run_point_nominal_supply_voltage: None,
    }
}

fn clear_v6_execution_fields(results: &mut ProjectSimulationResults) {
    for run in &mut results.runs {
        run.job_id = None;
        run.execution_target = None;
        run.lifecycle = None;
        run.dataset_content_digest = PersistedField::Missing;
        for analysis in &mut run.analyses {
            analysis.result_data_digest = PersistedField::Missing;
        }
    }
}

fn clear_v6_execution_fields_json(results: &mut serde_json::Value) {
    for run in results["runs"]
        .as_array_mut()
        .expect("simulation result run array")
    {
        let run = run.as_object_mut().expect("simulation result run object");
        run.remove("job_id");
        run.remove("execution_target");
        run.remove("lifecycle");
        run.remove("dataset_content_digest");
        for analysis in run
            .get_mut("analyses")
            .and_then(serde_json::Value::as_array_mut)
            .expect("simulation analysis array")
        {
            analysis
                .as_object_mut()
                .expect("simulation analysis object")
                .remove("result_data_digest");
        }
    }
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
    let receipt = PreparedRunReceipt::new(
        source_domain,
        simulation_plan_id,
        project_revision,
        prepared_snapshot_digest,
        source_content_digest,
        source_check_receipt,
        tasks,
    )
    .expect("prepared run receipt");
    run.restore_provenance(SimulationRunProvenance::Prepared(receipt))
        .expect("prepared fixture seals explicitly");
}

#[test]
fn prepared_run_receipt_round_trip_retains_exact_project_model_sources() {
    let source_id = ModelSourceId::new();
    let identity = PreparedModelSourceIdentity::new(
        source_id,
        "nch_receipt",
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x51; 32]),
    )
    .unwrap();
    let task = PreparedRunTaskReceipt::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        Vec::new(),
        2,
        ContentDigest::from_bytes([0x52; 32]),
    )
    .unwrap();
    let receipt = PreparedRunReceipt::new_with_project_model_sources(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x53; 32]),
        ContentDigest::from_bytes([0x54; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x55; 32])),
        vec![identity],
        vec![task],
    )
    .unwrap();

    let wire = ProjectPreparedRunReceipt::from(&receipt);
    let encoded = serde_json::to_string(&wire).unwrap();
    let decoded = serde_json::from_str::<ProjectPreparedRunReceipt>(&encoded).unwrap();
    let restored = decoded.into_receipt().unwrap();

    assert_eq!(restored.project_model_sources().len(), 1);
    let restored_source = &restored.project_model_sources()[0];
    assert_eq!(restored_source.source_id(), source_id);
    assert_eq!(restored_source.model_name(), "nch_receipt");
    assert_eq!(
        restored_source.content_digest(),
        ContentDigest::from_bytes([0x51; 32])
    );
}

fn project_with_execution_context() -> ProjectFile {
    use crate::simulation::dialog::{DampingStrategy, IntegrationMethod, MatrixSolver};
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelLibraryManager, ModelType};
    use crate::workbench::simulation_analysis_tabs::{TAB_AC, TAB_NOISE, TAB_TRANSIENT};

    let mut design_libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut design_libraries);

    let mut setup = crate::workbench::app_state::SimSetupState::new();
    setup.enabled.extend([TAB_AC, TAB_NOISE]);
    setup.analysis_order = vec![TAB_NOISE, TAB_TRANSIENT, TAB_AC];
    setup.listed.extend([TAB_AC, TAB_NOISE]);
    setup
        .set_reference_pvt(crate::simulation::dialog::corner::ProcessCorner::FF, -40.0)
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
        .migrate_legacy_analysis_plan(workspace.project.id())
        .expect("legacy execution fixture migrates at the load boundary");
    let execution_context = crate::io::ProjectExecutionContext::from_state(
        workspace.project.id(),
        &setup,
        &model_manager,
    )
    .expect("execution fixture validates");
    ProjectFile::new_with_execution_context(
        workspace,
        design_libraries,
        ProjectSimulationResults::default(),
        execution_context,
    )
}

fn serialized_analysis_instance_mut<'a>(
    project: &'a mut serde_json::Value,
    kind: &str,
) -> &'a mut serde_json::Value {
    project["execution_context"]["simulation_plan"]["analysis_plan"]["instances"]
        .as_array_mut()
        .expect("stable analysis instances serialize as an array")
        .iter_mut()
        .find(|instance| instance["kind"] == kind)
        .unwrap_or_else(|| panic!("fixture contains {kind} analysis"))
}

fn cell_source_bundle(reference: CellViewRef) -> crate::state::ProjectSourceBundle {
    crate::state::ProjectSourceBundle::try_new(
        crate::state::ProjectSourceOwner::cell_view(reference),
        crate::state::ProjectSourceLanguage::VerilogA,
        "behavior.va",
        "module behavior(p, n); inout p, n; endmodule",
        std::iter::empty(),
        std::iter::empty(),
    )
    .expect("valid cell source bundle")
}

#[test]
fn project_validation_requires_cell_source_owner_to_be_an_exact_veriloga_view() {
    let mut valid = project_with_execution_context();
    let reference = CellViewRef::new(
        valid.workspace.project.root_library.clone(),
        valid.workspace.project.top_cell.clone(),
        "behavior",
    );
    valid
        .libraries
        .get_library_mut(&reference.library)
        .and_then(|library| library.get_cell_mut(&reference.cell))
        .expect("top cell")
        .add_view(crate::state::View::new(
            reference.view.as_str(),
            ViewType::VerilogA,
        ));
    valid
        .workspace
        .project_sources
        .insert_bundle(cell_source_bundle(reference.clone()))
        .expect("unique source owner");
    valid.validate().expect("exact Verilog-A owner is valid");

    let mut missing_source = valid.clone();
    missing_source.workspace.project_sources = Default::default();
    assert!(
        missing_source
            .validate()
            .expect_err("a Verilog-A view without its source must fail")
            .to_string()
            .contains("has no project source bundle")
    );

    let mut canonical_alias = valid.clone();
    canonical_alias.workspace.project_sources = Default::default();
    canonical_alias
        .workspace
        .project_sources
        .insert_bundle(cell_source_bundle(CellViewRef::new(
            reference.library.to_uppercase(),
            reference.cell.to_uppercase(),
            reference.view.to_uppercase(),
        )))
        .expect("registry accepts one canonical owner until tree validation");
    assert!(
        canonical_alias
            .validate()
            .expect_err("canonical aliases must retain exact library-tree spelling")
            .to_string()
            .contains("does not match the canonical library/view identity")
    );

    let mut missing = valid.clone();
    missing.workspace.project_sources = Default::default();
    missing
        .workspace
        .project_sources
        .insert_bundle(cell_source_bundle(CellViewRef::new(
            &reference.library,
            "missing_cell",
            "behavior",
        )))
        .expect("registry permits unresolved owner until project validation");
    assert!(
        missing
            .validate()
            .expect_err("missing owner must fail")
            .to_string()
            .contains("owns missing cell view")
    );

    let mut wrong_type = valid;
    wrong_type.workspace.project_sources = Default::default();
    let schematic = CellViewRef::new(
        &wrong_type.workspace.project.root_library,
        &wrong_type.workspace.project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    wrong_type
        .workspace
        .project_sources
        .insert_bundle(cell_source_bundle(schematic))
        .expect("registry validates owner shape");
    assert!(
        wrong_type
            .validate()
            .expect_err("schematic owner must fail")
            .to_string()
            .contains("requires a Verilog-A view")
    );
}

#[test]
fn expected_digest_gate_rejects_replaced_bytes_before_parsing() {
    let path = std::env::temp_dir().join(format!(
        "rspice-expected-project-digest-{}-{}.rspiceproj",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let project = project_with_execution_context();
    let contents = serialize_project_file(&project).expect("serialize fixture");
    std::fs::write(&path, contents).expect("write accepted fixture");
    let (_, accepted) =
        load_project_file_with_digest(&path).expect("load accepted fixture identity");

    assert!(
        load_project_file_with_expected_digest(&path, accepted)
            .expect("matching project loads")
            .is_some()
    );

    std::fs::write(&path, b"replacement bytes are intentionally not JSON")
        .expect("replace fixture");
    assert!(
        load_project_file_with_expected_digest(&path, accepted)
            .expect("digest mismatch is rejected without parsing")
            .is_none()
    );

    std::fs::remove_file(path).expect("remove isolated fixture");
}

#[test]
fn suggested_project_save_path_defaults_and_enforces_extension() {
    assert_eq!(
        suggested_project_save_path(None),
        PathBuf::from("untitled.rspiceproj")
    );
    assert_eq!(
        suggested_project_save_path(Some("amp")),
        PathBuf::from("amp.rspiceproj")
    );
    assert_eq!(
        suggested_project_save_path(Some("amp.rspiceproj")),
        PathBuf::from("amp.rspiceproj")
    );
}

#[test]
fn project_file_serializes_to_versioned_json() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectFile::new(workspace, libraries);

    let json = serialize_project_file(&project).expect("project serializes");

    assert!(json.contains("\"version\""));
    assert!(json.contains("\"workspace\""));
    assert!(json.contains("\"libraries\""));
    assert!(json.ends_with('\n'));
}

#[test]
fn project_file_round_trips_configuration_execution_authority() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Browser-qualified release".to_owned(),
            root: workspace.active_view.clone(),
            dut_path: "/top/XDUT".to_owned(),
            executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
            stop_views: vec!["spice".to_owned()],
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![crate::state::ConfigurationSetOverride {
                instance_path: "/top/XDUT/*".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: Some("tt".to_owned()),
                eligible_platforms: vec![crate::state::ConfigurationPlatform::Browser],
            }],
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Verification".to_owned(),
        })
        .expect("configuration fixture");
    let expected = workspace.configuration_sets.clone();
    let project = ProjectFile::new(workspace, libraries);

    let json = serialize_project_file(&project).expect("configuration project serializes");
    let loaded = load_project_text(&json, None).expect("configuration project loads");

    assert_eq!(loaded.workspace.configuration_sets, expected);
}

#[test]
fn project_file_round_trips_design_management_authority() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let owner = workspace.active_view.key();
    workspace
        .design_management
        .bootstrap_for_cell_view(&owner, "Main", [11, 12])
        .expect("design-management fixture");
    let expected = workspace.design_management.clone();
    let project = ProjectFile::new(workspace, libraries);

    let json = serialize_project_file(&project).expect("design project serializes");
    let loaded = load_project_text(&json, None).expect("design project loads");

    assert_eq!(loaded.workspace.design_management, expected);
    assert_eq!(
        loaded
            .workspace
            .design_management
            .semantic_digest()
            .expect("loaded semantic digest"),
        expected
            .semantic_digest()
            .expect("expected semantic digest")
    );
}

#[test]
fn project_file_rejects_unsupported_design_management_schema() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    workspace
        .design_management
        .bootstrap_for_cell_view(&workspace.active_view.key(), "Main", [11])
        .expect("design-management fixture");
    let project = ProjectFile::new(workspace, libraries);
    let mut value = serde_json::to_value(project).expect("project JSON value");
    value["workspace"]["design_management"]["schema_version"] = serde_json::Value::from(999);
    let json = serde_json::to_string(&value).expect("malformed project JSON");

    let error = load_project_text(&json, None).expect_err("unsupported schema is rejected");
    assert!(error.to_string().contains("schema 999 is unsupported"));
}

#[test]
fn project_file_round_trips_project_owned_report_documents() {
    use crate::results::report_document::{
        ReportBlockedGateTextPolicy, ReportDocument, ReportEdit, ReportPageEvidenceBinding,
        ReportPageInclusion, ReportPageUpdatePolicy, ReportTemplate,
    };

    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut report =
        ReportDocument::new_with_template("Verification report", ReportTemplate::DesignReview)
            .expect("report document");
    report
        .transact(
            report.revision(),
            vec![ReportEdit::AddPage {
                title: "PVT and yield".to_owned(),
            }],
            10,
        )
        .expect("add page");
    let page = report.pages()[0].clone();
    let exact_binding = crate::product::DatasetBinding::new(
        crate::product::DatasetId::new(),
        crate::product::ContentDigest::from_bytes([0x6a; 32]),
    );
    let second_page_revision = page.revision().next().expect("page revision");
    let third_page_revision = second_page_revision.next().expect("page revision");
    let fourth_page_revision = third_page_revision.next().expect("page revision");
    report
        .transact(
            report.revision(),
            vec![
                ReportEdit::SetPageUpdatePolicy {
                    page_id: page.id(),
                    expected_page_revision: page.revision(),
                    update_policy: ReportPageUpdatePolicy::FreezeSelectedRevision,
                },
                ReportEdit::SetPageInclusion {
                    page_id: page.id(),
                    expected_page_revision: second_page_revision,
                    inclusion: ReportPageInclusion::AppendixOnly,
                },
                ReportEdit::SetPageEvidenceBinding {
                    page_id: page.id(),
                    expected_page_revision: third_page_revision,
                    evidence_binding: ReportPageEvidenceBinding::ExactDataset {
                        binding: exact_binding,
                    },
                },
                ReportEdit::SetPageBlockedGateTextPolicy {
                    page_id: page.id(),
                    expected_page_revision: fourth_page_revision,
                    policy: ReportBlockedGateTextPolicy::SummarizeWithLink,
                },
            ],
            11,
        )
        .expect("set page publication policies");
    workspace.report_documents.push(report.clone());
    workspace.report_documents_dirty = true;
    let project = ProjectFile::new(workspace, libraries);

    let json = serialize_project_file(&project).expect("project serializes");
    let restored = load_project_text(&json, None).expect("project reloads");

    assert_eq!(restored.workspace.report_documents, vec![report]);
    let restored_report = &restored.workspace.report_documents[0];
    assert_eq!(restored_report.revision_history().records().len(), 3);
    assert_eq!(
        restored_report
            .reconstruct_revision(restored_report.id(), ObjectRevision::INITIAL)
            .expect("initial report source is reconstructable")
            .title(),
        "Verification report"
    );
    assert!(
        restored_report
            .reconstruct_revision(restored_report.id(), ObjectRevision::INITIAL)
            .expect("initial report source is reconstructable")
            .pages()
            .is_empty()
    );
    assert!(!restored.workspace.report_documents_dirty);
}

#[test]
fn project_execution_context_round_trips_every_persisted_input() {
    let project = project_with_execution_context();
    let expected = serde_json::to_value(
        project
            .execution_context
            .as_ref()
            .expect("fixture has execution context"),
    )
    .expect("context serializes");

    let json = serialize_project_file(&project).expect("project serializes");
    let loaded = load_project_text(&json, None).expect("project reloads");
    let actual = serde_json::to_value(
        loaded
            .execution_context
            .as_ref()
            .expect("execution context restored"),
    )
    .expect("restored context serializes");

    assert_eq!(actual, expected);
    let plan = &actual["simulation_plan"];
    for retired in [
        "enabled",
        "analysis_order",
        "listed",
        "op",
        "tran",
        "ac",
        "noise",
    ] {
        assert!(
            plan.get(retired).is_none(),
            "retired field {retired} leaked"
        );
    }
    let instances = plan["analysis_plan"]["instances"]
        .as_array()
        .expect("stable instances serialize");
    assert_eq!(instances[0]["kind"], "noise");
    assert_eq!(instances[1]["kind"], "tran");
    assert_eq!(instances[2]["kind"], "ac");
    assert!(
        instances[..3]
            .iter()
            .all(|instance| instance["enabled"] == true)
    );
    assert!(plan.get("options_draft").is_none());
    assert!(plan.get("options_open").is_none());
    assert_eq!(actual["model_libraries"][0]["name"], "fixture_models");
    assert!(actual["model_libraries"][0].get("expanded").is_none());
}

#[test]
fn legacy_project_without_execution_context_remains_compatible() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectFile::new(workspace, libraries);
    let json = serialize_project_file(&project).expect("legacy-compatible project serializes");

    let loaded = load_project_text(&json, None).expect("legacy project opens");

    assert!(loaded.execution_context.is_none());
}

#[test]
fn unversioned_execution_context_migrates_to_sorted_legacy_order() {
    let project = project_with_execution_context();
    let json = serialize_project_file(&project).expect("project serializes");
    let mut value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    let context = value["execution_context"]
        .as_object_mut()
        .expect("execution object");
    context.remove("schema_version");
    let simulation_plan = context["simulation_plan"]
        .as_object_mut()
        .expect("simulation plan");
    simulation_plan.remove("analysis_plan");
    simulation_plan.insert("enabled".to_owned(), serde_json::json!([4, 1, 2]));

    let loaded = load_project_text(
        &serde_json::to_string(&value).expect("fixture serializes"),
        None,
    )
    .expect("legacy context migrates");
    let context = loaded.execution_context.expect("context retained");

    assert_eq!(
        context.schema_version,
        crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
    );
    let enabled = context
        .simulation_plan
        .stable_analysis_plan()
        .expect("legacy singleton plan migrates")
        .instances()
        .iter()
        .filter(|instance| instance.enabled())
        .map(|instance| instance.kind())
        .collect::<Vec<_>>();
    assert_eq!(
        enabled,
        vec![
            crate::simulation::plan::AnalysisKind::Transient,
            crate::simulation::plan::AnalysisKind::Ac,
            crate::simulation::plan::AnalysisKind::Noise,
        ]
    );
}

#[test]
fn malformed_execution_context_is_never_silently_defaulted() {
    let project = project_with_execution_context();
    let json = serialize_project_file(&project).expect("project serializes");
    let valid: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");

    let mut future = valid.clone();
    future["execution_context"]["schema_version"] = serde_json::json!(999);
    let error = load_project_text(&future.to_string(), None)
        .expect_err("future schema must fail")
        .to_string();
    assert!(error.contains("unsupported execution-context schema version 999"));

    let mut duplicate_identity = valid.clone();
    let instances =
        duplicate_identity["execution_context"]["simulation_plan"]["analysis_plan"]["instances"]
            .as_array_mut()
            .expect("stable instances");
    let first_id = instances[0]["id"].clone();
    instances[1]["id"] = first_id;
    let error = load_project_text(&duplicate_identity.to_string(), None)
        .expect_err("duplicate stable identity must fail")
        .to_string();
    assert!(error.contains("appears more than once"), "{error}");

    let mut mismatched_draft = valid.clone();
    serialized_analysis_instance_mut(&mut mismatched_draft, "noise")["kind"] =
        serde_json::json!("ac");
    let error = load_project_text(&mismatched_draft.to_string(), None)
        .expect_err("declared kind and draft kind must agree")
        .to_string();
    assert!(error.contains("declared as ac"), "{error}");

    let mut unsupported = valid.clone();
    serialized_analysis_instance_mut(&mut unsupported, "tran")["kind"] =
        serde_json::json!("future-analysis");
    let error = load_project_text(&unsupported.to_string(), None)
        .expect_err("unsupported stable analysis kind must fail")
        .to_string();
    assert!(
        error.contains("unknown variant `future-analysis`"),
        "{error}"
    );

    let mut mismatched_pvt = valid.clone();
    mismatched_pvt["execution_context"]["simulation_plan"]["options"]["temp"] =
        serde_json::json!(125.0);
    let error = load_project_text(&mismatched_pvt.to_string(), None)
        .expect_err("conflicting execution temperatures must fail")
        .to_string();
    assert!(error.contains("disagrees with solver option temp"));

    let mut unknown_input = valid.clone();
    serialized_analysis_instance_mut(&mut unknown_input, "tran")["draft"]["draft"]["future_mode"] =
        serde_json::json!(true);
    let error = load_project_text(&unknown_input.to_string(), None)
        .expect_err("unknown execution input must not be ignored")
        .to_string();
    assert!(error.contains("unknown field `future_mode`"));

    let mut invalid_model = valid;
    invalid_model["execution_context"]["model_libraries"][0]["selected_corner"] =
        serde_json::json!("missing");
    let error = load_project_text(&invalid_model.to_string(), None)
        .expect_err("invalid model binding must fail")
        .to_string();
    assert!(error.contains("selected_corner 'missing' does not exist"));

    let mut invalid_digest = serde_json::from_str::<serde_json::Value>(&json).expect("valid JSON");
    let absolute_source = std::env::temp_dir().join("rspice-digest-shape.lib");
    invalid_digest["execution_context"]["model_libraries"][0]["root_path"] =
        serde_json::to_value(&absolute_source).expect("path serializes");
    invalid_digest["execution_context"]["model_libraries"][0]["source_closure"] = serde_json::json!([{
        "path": absolute_source,
        "digest": "not-a-sha-256-digest"
    }]);
    let error = load_project_text(&invalid_digest.to_string(), None)
        .expect_err("malformed digest must fail")
        .to_string();
    assert!(error.contains("SHA-256 digest must contain 64 hexadecimal characters"));

    let mut digest_without_source =
        serde_json::from_str::<serde_json::Value>(&json).expect("valid JSON");
    digest_without_source["execution_context"]["model_libraries"][0]["source_closure"] = serde_json::json!([{
        "path": std::env::temp_dir().join("rspice-orphan-pin.lib"),
        "digest": "00".repeat(32)
    }]);
    let error = load_project_text(&digest_without_source.to_string(), None)
        .expect_err("digest without source path must fail")
        .to_string();
    assert!(
        error.contains("source_authority built_in cannot own a root path or source closure"),
        "{error}"
    );
}

#[test]
fn unfinished_analysis_drafts_are_project_data_not_file_corruption() {
    let project = project_with_execution_context();
    let json = serialize_project_file(&project).expect("project serializes");
    let mut value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    serialized_analysis_instance_mut(&mut value, "tran")["draft"]["draft"]["stop"] =
        serde_json::json!("unfinished(");
    serialized_analysis_instance_mut(&mut value, "mc")["draft"]["draft"]["seed"] =
        serde_json::json!("not-an-integer-yet");

    let loaded = load_project_text(&value.to_string(), None)
        .expect("draft syntax is validated by run preflight, not project loading");
    let plan = &loaded
        .execution_context
        .expect("context retained")
        .simulation_plan;

    let stable = plan.stable_analysis_plan().expect("stable plan restored");
    let transient = stable
        .instances()
        .iter()
        .find(|instance| instance.kind() == crate::simulation::plan::AnalysisKind::Transient)
        .expect("transient instance");
    let crate::simulation::plan::AnalysisDraft::Transient(transient) = transient.draft() else {
        panic!("transient instance owns transient draft");
    };
    assert_eq!(transient.stop, "unfinished(");
    let monte_carlo = stable
        .instances()
        .iter()
        .find(|instance| instance.kind() == crate::simulation::plan::AnalysisKind::MonteCarlo)
        .expect("Monte Carlo instance");
    let crate::simulation::plan::AnalysisDraft::MonteCarlo(monte_carlo) = monte_carlo.draft()
    else {
        panic!("Monte Carlo instance owns Monte Carlo draft");
    };
    assert_eq!(monte_carlo.seed, "not-an-integer-yet");
    assert!(
        plan.analysis_draft_validation_error(&crate::simulation::plan::AnalysisDraft::Transient(
            transient.clone(),
        ))
        .is_some()
    );
    assert!(
        plan.analysis_draft_validation_error(&crate::simulation::plan::AnalysisDraft::MonteCarlo(
            monte_carlo.clone()
        ),)
            .is_some()
    );
}

#[test]
fn project_file_round_trips_persisted_simulation_results() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut simulation = SimulationState::default();
    let waveform = WaveformData::new(
        "|V(out)|",
        vec![1.0, 10.0, 100.0],
        vec![2.0, 3.0, 4.0],
        "#00aaff",
    )
    .with_complex_components("V(out)", vec![2.0, 3.0, 4.0], vec![0.1, 0.2, 0.3]);
    let mut run = SimulationRun::new(12);
    run.timestamp = 1234.5;
    run.label = "Run 12 (fixture)".to_string();
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.set_elapsed_time(0.125);
    run.add_analysis(
        AnalysisResult::new(7, AnalysisType::Ac, "AC fixture")
            .with_waveforms(vec![waveform])
            .with_dc_op(crate::state::DcOpResult {
                node_voltages: vec![OperatingPointValue {
                    name: "V(out)".to_string(),
                    value: 1.25,
                    unit: "V".to_string(),
                }],
                branch_currents: Vec::new(),
                power_dissipation: Vec::new(),
            })
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 3.0)]),
    );
    seal_legacy_unattributed(&mut run);
    let expected_run_id = run.run_id;
    let expected_dataset_id = run.dataset_id;
    simulation.runs = vec![run];
    simulation.next_run_id = 12;
    simulation.active_run_idx = Some(0);
    simulation.active_analysis_idx = Some(0);

    let project = ProjectFile::new_with_simulation_results(
        workspace,
        libraries,
        ProjectSimulationResults::from_state(&simulation),
    );
    let json = serialize_project_file(&project).expect("project serializes with results");

    assert!(json.contains("\"simulation_results\""));
    let loaded = load_project_text(&json, None).expect("project reloads");
    let restored = loaded
        .simulation_results
        .into_simulation_state()
        .expect("validated project results restore");

    assert_eq!(restored.run_count(), 1);
    assert_eq!(
        restored.active_run().map(|run| run.run_id),
        Some(expected_run_id)
    );
    assert_eq!(
        restored.active_run().map(|run| run.dataset_id),
        Some(expected_dataset_id)
    );
    assert_eq!(
        restored.active_run().expect("active run").label,
        "Run 12 (fixture)"
    );
    let restored_run = restored.active_run().expect("active run");
    assert!(restored_run.job_id.is_some());
    assert_eq!(
        restored_run.execution_target,
        Some(ExecutionTarget::current())
    );
    assert_eq!(restored_run.lifecycle, SimulationRunLifecycle::Completed);
    let analysis = restored.active_analysis().expect("active analysis");
    assert_eq!(analysis.id, 7);
    assert_eq!(analysis.analysis_type, AnalysisType::Ac);
    assert_eq!(analysis.measurements[0].name, "gain");
    assert_eq!(analysis.waveforms[0].complex.as_ref().unwrap().imag[2], 0.3);
    assert_eq!(restored.waveforms[0].name, "|V(out)|");

    let mut unversioned_value: serde_json::Value =
        serde_json::from_str(&json).expect("current project parses as JSON");
    let legacy_results = unversioned_value["simulation_results"]
        .as_object_mut()
        .expect("simulation result object");
    legacy_results.remove("schema_version");
    legacy_results.remove("active_run_stable_id");
    legacy_results.remove("active_dataset_id");
    legacy_results.remove("active_analysis_sequence");
    legacy_results.insert("active_run_id".to_owned(), serde_json::json!(12));
    legacy_results.insert("active_analysis_id".to_owned(), serde_json::json!(7));
    let legacy_run = legacy_results["runs"][0]
        .as_object_mut()
        .expect("legacy run object");
    legacy_run.remove("run_id");
    legacy_run.remove("dataset_id");
    legacy_run.remove("job_id");
    legacy_run.remove("execution_target");
    legacy_run.remove("lifecycle");
    legacy_run.remove("dataset_content_digest");
    for analysis in legacy_run["analyses"]
        .as_array_mut()
        .expect("legacy analysis array")
    {
        analysis
            .as_object_mut()
            .expect("legacy analysis object")
            .remove("result_data_digest");
    }
    legacy_run.remove("provenance_mode");
    let unversioned_json =
        serde_json::to_string(&unversioned_value).expect("unversioned project serializes");
    let unversioned =
        load_project_text(&unversioned_json, None).expect("unversioned project migrates");
    let migrated_run = &unversioned.simulation_results.runs[0];
    assert_eq!(
        unversioned.simulation_results.active_run_stable_id,
        migrated_run.run_id
    );
    assert_eq!(
        unversioned.simulation_results.active_dataset_id,
        migrated_run.dataset_id
    );
    assert_ne!(migrated_run.run_id, Some(expected_run_id));
    assert_ne!(migrated_run.dataset_id, Some(expected_dataset_id));
    assert_eq!(
        unversioned.simulation_results.active_analysis_sequence,
        Some(7)
    );
    assert_eq!(
        migrated_run.lifecycle,
        Some(SimulationRunLifecycle::LegacyUnknown)
    );
}

#[test]
fn project_file_round_trips_exact_result_family_metadata_and_migrates_v6_absence() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let metadata = crate::state::AnalysisResultFamilyMetadata::MonteCarlo {
        seed: 42,
        runs_requested: 3,
        runs_completed: 2,
        failures: 1,
        all_converged: false,
        variables: vec![crate::state::MonteCarloVariableMetadata {
            name: "V(out)".to_owned(),
            samples: vec![0.975, 1.025],
            mean: 1.0,
            std_dev: 0.025,
            min: 0.975,
            max: 1.025,
        }],
    };
    let mut run = SimulationRun::new(1);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC")
            .with_family_metadata(metadata.clone()),
    );
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 1;
    simulation.active_run_idx = Some(0);
    simulation.active_analysis_idx = Some(0);

    let project = ProjectFile::new_with_simulation_results(
        workspace,
        libraries,
        ProjectSimulationResults::from_state(&simulation),
    );
    let json = serialize_project_file(&project).expect("family metadata serializes");
    let loaded = load_project_text(&json, None).expect("family metadata reloads");
    let restored = loaded
        .simulation_results
        .into_simulation_state()
        .expect("family metadata restores");
    assert_eq!(
        restored
            .active_analysis()
            .and_then(|analysis| analysis.family_metadata.as_ref()),
        Some(&metadata)
    );

    let mut v6: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
    v6["simulation_results"]["schema_version"] =
        serde_json::Value::from(EXECUTION_IDENTITY_RESULTS_SCHEMA_VERSION);
    v6["simulation_results"]["runs"][0]
        .as_object_mut()
        .expect("run object")
        .remove("dataset_content_digest");
    v6["simulation_results"]["runs"][0]["analyses"][0]
        .as_object_mut()
        .expect("analysis object")
        .remove("family_metadata");
    v6["simulation_results"]["runs"][0]["analyses"][0]
        .as_object_mut()
        .expect("analysis object")
        .remove("result_data_digest");
    let migrated = load_project_text(&v6.to_string(), None).expect("v6 project migrates");
    assert_eq!(
        migrated.simulation_results.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    let migrated = migrated
        .simulation_results
        .into_simulation_state()
        .expect("migrated v6 results restore");
    assert!(
        migrated
            .active_analysis()
            .expect("migrated analysis")
            .family_metadata
            .is_none(),
        "legacy absence must remain explicit instead of being inferred from waveforms"
    );
}

#[test]
fn retained_result_data_digests_round_trip_and_reject_sample_tampering() {
    let mut run = SimulationRun::new(2);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new("V(out)", vec![1.0, 10.0], vec![0.25, 0.5], "#00aaff")
                .with_complex_components("V(out)", vec![0.25, 0.5], vec![-0.75, -0.5]),
        ]),
    );
    seal_legacy_unattributed(&mut run);
    let expected_analysis_digest = run.analyses[0].result_data_digest();
    let expected_dataset_digest = run.dataset_content_digest();
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 2;

    let persisted = ProjectSimulationResults::from_state(&simulation);
    assert_eq!(
        persisted.runs[0].analyses[0]
            .result_data_digest
            .as_ref()
            .copied(),
        Some(expected_analysis_digest)
    );
    assert_eq!(
        persisted.runs[0].dataset_content_digest.as_ref().copied(),
        Some(expected_dataset_digest)
    );

    let json = serde_json::to_string(&persisted).expect("result data serializes");
    let restored: ProjectSimulationResults =
        serde_json::from_str(&json).expect("result data deserializes");
    restored.validate().expect("retained digests validate");
    let restored_run = restored
        .into_simulation_state()
        .expect("retained result data restores")
        .runs
        .remove(0);
    assert_eq!(
        restored_run.analyses[0].result_data_digest(),
        expected_analysis_digest
    );
    assert_eq!(
        restored_run.dataset_content_digest(),
        expected_dataset_digest
    );

    let mut tampered: serde_json::Value =
        serde_json::from_str(&json).expect("result document JSON");
    tampered["runs"][0]["analyses"][0]["waveforms"][0]["complex"]["imag"][1] =
        serde_json::json!(-0.499_999_999_999_999_94_f64);
    let tampered: ProjectSimulationResults =
        serde_json::from_value(tampered).expect("tampered result remains structurally valid");
    assert!(
        tampered
            .validate()
            .expect_err("a changed complex sample invalidates its retained digest")
            .contains("result_data_digest does not match retained analysis content")
    );
}

#[test]
fn typed_result_payloads_round_trip_and_reject_payload_tampering() {
    let mut run = SimulationRun::new(31);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::PoleZero, "PZ").with_result_payload(
            AnalysisResultPayload::PoleZero {
                poles: vec![crate::state::ComplexResultValue {
                    real: -1.0,
                    imaginary: 2.0,
                }],
                zeros: vec![crate::state::ComplexResultValue {
                    real: -3.0,
                    imaginary: 0.0,
                }],
                gain: 4.0,
            },
        ),
    );
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Sensitivity, "SENS").with_result_payload(
            AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: crate::state::SensitivityResultMode::Ac {
                    frequency_hz: 10_000.0,
                },
                rows: vec![crate::state::SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 2.0,
                    normalized: 0.5,
                }],
            },
        ),
    );
    run.add_analysis(
        AnalysisResult::new(3, AnalysisType::Tf, "TF").with_result_payload(
            AnalysisResultPayload::TransferFunction {
                input_source: "VIN".to_owned(),
                output_expression: "V(OUT)".to_owned(),
                input_quantity: crate::state::TransferFunctionQuantityEvidence::Voltage,
                output_quantity: crate::state::TransferFunctionQuantityEvidence::Voltage,
                input_unit: "V".to_owned(),
                output_unit: "V".to_owned(),
                normalization: crate::state::TransferFunctionNormalizationEvidence::None,
                accuracy: crate::state::TransferFunctionAccuracyEvidence::Balanced,
                gain: Some(crate::state::TransferFunctionScalarEvidence::Finite(10.0)),
                input_resistance: Some(
                    crate::state::TransferFunctionScalarEvidence::PositiveInfinity,
                ),
                output_resistance: Some(crate::state::TransferFunctionScalarEvidence::Finite(50.0)),
                nominal_input: None,
                nominal_output: None,
            },
        ),
    );
    run.add_analysis(
        AnalysisResult::new(4, AnalysisType::Reliability, "Reliability")
            .with_family_metadata(AnalysisResultFamilyMetadata::Reliability { years: vec![10.0] })
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![crate::state::ReliabilityDeviceEvidence {
                    device_id: "M1".to_owned(),
                    stress: crate::state::ReliabilityStressEvidence {
                        average_gate_stress_v: 1.2,
                        average_drain_stress_v: 1.8,
                        average_temperature_k: 358.15,
                        duration_s: 3_600.0,
                    },
                    checkpoints: vec![crate::state::ReliabilityCheckpointEvidence {
                        years: 10.0,
                        shift: crate::state::ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 0.03,
                            mobility_shift: -0.004,
                            drain_source_resistance_shift: 0.0015,
                        },
                    }],
                }],
            }),
    );
    run.add_analysis(
        AnalysisResult::new(5, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 1.0],
            })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![crate::state::SoaEvaluationEvidence {
                    device_id: "M1".to_owned(),
                    parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
                    limit_value: 3.3,
                    worst_actual_value: 3.2,
                    worst_time_s: 1.0,
                    sample_count: 2,
                    unit: "V".to_owned(),
                    description: "Maximum drain-source voltage".to_owned(),
                    verdict: crate::state::SoaRuleVerdictEvidence::Warning,
                }],
                violations: vec![crate::state::SoaViolationEvidence {
                    device_id: "M1".to_owned(),
                    parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
                    limit_value: 3.3,
                    actual_value: 3.2,
                    time_s: 1.0,
                    severity: crate::state::SoaViolationSeverityEvidence::Warning,
                }],
            }),
    );
    run.add_analysis(
        AnalysisResult::new(6, AnalysisType::DcOp, "OP").with_result_payload(
            AnalysisResultPayload::OperatingPoint {
                temperature_mode: crate::state::OperatingPointTemperatureEvidence::PvtRunSet,
                temperature_celsius: 27.0,
                initial_guess: crate::state::OperatingPointInitialGuessEvidence::PreviousConverged,
                node_initialization:
                    crate::state::OperatingPointNodeInitializationEvidence::UseIcAndNodeset,
                homotopy: crate::state::OperatingPointHomotopyEvidence::Adaptive,
                annotation: crate::state::OperatingPointAnnotationEvidence::VoltagesAndDeviceOp,
                device_detail: crate::state::OperatingPointDeviceDetailEvidence::ViolationsOnly,
                save_device_op: crate::state::OperatingPointSaveDeviceEvidence::FinalPointOnly,
                accuracy: crate::state::OperatingPointAccuracyEvidence::Robust,
                selected_devices: vec!["M1".to_owned()],
                violation_devices: vec!["M1".to_owned()],
                violation_source_content_digest: Some(crate::product::ContentDigest::from_bytes(
                    [0x61; 32],
                )),
                validated_startup_directives: 2,
                mna_node_names: vec!["in".to_owned(), "out".to_owned()],
                mna_branch_names: vec!["V1".to_owned()],
                mna_solution: vec![1.0, 0.5, -0.5e-3],
                effective_source_content_digest: Some(crate::product::ContentDigest::from_bytes(
                    [0x62; 32],
                )),
                run_point_index: 1,
                run_point_count: 2,
                run_point_process: crate::state::OperatingPointProcessEvidence::SS,
                run_point_supply_voltage: Some(0.9),
                run_point_nominal_supply_voltage: Some(1.0),
            },
        ),
    );
    seal_legacy_unattributed(&mut run);
    let dataset_id = run.dataset_id;
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 31;
    simulation.active_run_idx = Some(0);
    simulation.active_analysis_idx = Some(1);

    let persisted = ProjectSimulationResults::from_state(&simulation);
    let json = serde_json::to_string(&persisted).expect("typed payloads serialize");
    let decoded: ProjectSimulationResults =
        serde_json::from_str(&json).expect("typed payloads deserialize");
    decoded.validate().expect("typed payload digests validate");
    let restored = decoded
        .into_simulation_state()
        .expect("typed payloads restore");
    assert_eq!(
        restored.active_run().map(|run| run.dataset_id),
        Some(dataset_id)
    );
    assert_eq!(
        restored.active_analysis().map(|analysis| analysis.id),
        Some(2)
    );
    assert_eq!(
        restored.runs[0].analyses[0].result_payload,
        simulation.runs[0].analyses[0].result_payload
    );
    assert_eq!(
        restored.runs[0].analyses[1].result_payload,
        simulation.runs[0].analyses[1].result_payload
    );
    assert_eq!(
        restored.runs[0].analyses[2].result_payload,
        simulation.runs[0].analyses[2].result_payload
    );
    assert!(matches!(
        restored.runs[0].analyses[2].result_payload.as_ref(),
        Some(AnalysisResultPayload::TransferFunction {
            input_resistance: Some(crate::state::TransferFunctionScalarEvidence::PositiveInfinity),
            ..
        })
    ));
    assert_eq!(
        restored.runs[0].analyses[3].result_payload,
        simulation.runs[0].analyses[3].result_payload
    );
    assert_eq!(
        restored.runs[0].analyses[4].result_payload,
        simulation.runs[0].analyses[4].result_payload
    );
    assert_eq!(
        restored.runs[0].analyses[5].result_payload,
        simulation.runs[0].analyses[5].result_payload
    );

    let mut tampered: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
    tampered["runs"][0]["analyses"][0]["result_payload"]["gain"] =
        serde_json::json!(4.000_000_000_000_001_f64);
    let tampered: ProjectSimulationResults =
        serde_json::from_value(tampered).expect("tampered payload remains structural");
    assert!(
        tampered
            .validate()
            .expect_err("payload tampering invalidates the result digest")
            .contains("result_data_digest does not match retained analysis content")
    );

    let mut op_tampered: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
    op_tampered["runs"][0]["analyses"][5]["result_payload"]["mna_solution"][1] =
        serde_json::json!(0.500_000_000_000_000_1_f64);
    let op_tampered: ProjectSimulationResults =
        serde_json::from_value(op_tampered).expect("tampered OP payload remains structural");
    assert!(
        op_tampered
            .validate()
            .expect_err("OP MNA tampering invalidates the result digest")
            .contains("result_data_digest does not match retained analysis content")
    );

    let mut reliability_tampered: serde_json::Value =
        serde_json::from_str(&json).expect("project JSON");
    reliability_tampered["runs"][0]["analyses"][3]["result_payload"]["devices"][0]["checkpoints"]
        [0]["shift"]["mobility_shift"] = serde_json::json!(-0.004_000_000_000_000_001_f64);
    let reliability_tampered: ProjectSimulationResults =
        serde_json::from_value(reliability_tampered)
            .expect("tampered reliability payload remains structural");
    assert!(
        reliability_tampered
            .validate()
            .expect_err("reliability field tampering invalidates the result digest")
            .contains("result_data_digest does not match retained analysis content")
    );

    let mut tf_tampered: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
    tf_tampered["runs"][0]["analyses"][2]["result_payload"]["gain"]["value"] =
        serde_json::json!(10.000_000_000_000_002_f64);
    let tf_tampered: ProjectSimulationResults = serde_json::from_value(tf_tampered)
        .expect("tampered transfer-function payload remains structural");
    assert!(
        tf_tampered
            .validate()
            .expect_err("transfer-function field tampering invalidates the result digest")
            .contains("result_data_digest does not match retained analysis content")
    );

    let mut soa_tampered: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
    soa_tampered["runs"][0]["analyses"][4]["result_payload"]["evaluations"][0]["description"] =
        serde_json::json!("Changed rule description");
    let soa_tampered: ProjectSimulationResults =
        serde_json::from_value(soa_tampered).expect("tampered SOA payload remains structural");
    assert!(
        soa_tampered
            .validate()
            .expect_err("SOA field tampering invalidates the result digest")
            .contains("result_data_digest does not match retained analysis content")
    );

    let mut null_payload: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
    null_payload["runs"][0]["analyses"][0]["result_payload"] = serde_json::Value::Null;
    let null_payload: ProjectSimulationResults =
        serde_json::from_value(null_payload).expect("null remains presence-aware");
    assert!(
        null_payload
            .validate()
            .expect_err("current payload cannot be explicitly null")
            .contains("result_payload must not be null")
    );
}

#[test]
fn schema_v8_digests_are_authenticated_before_v9_resealing() {
    let mut run = SimulationRun::new(32);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 32;
    let mut v8 = ProjectSimulationResults::from_state(&simulation);
    v8.schema_version = CONTENT_DIGEST_RESULTS_SCHEMA_VERSION;
    for analysis in &mut v8.runs[0].analyses {
        analysis.result_payload = PersistedField::Missing;
        analysis.result_data_digest = PersistedField::Value(
            analysis
                .clone()
                .into_analysis()
                .expect("v8 analysis fixture")
                .legacy_v1_result_data_digest(),
        );
    }
    v8.runs[0].dataset_content_digest = PersistedField::Value(
        v8.runs[0]
            .clone()
            .into_run()
            .expect("v8 run fixture")
            .legacy_v1_dataset_content_digest(),
    );

    let mut migrated = v8.clone();
    migrated
        .migrate_to_current(ProjectId::new())
        .expect("authentic v8 results migrate");
    assert_eq!(
        migrated.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    migrated
        .validate()
        .expect("resealed current results validate");
    assert_ne!(
        migrated.runs[0].dataset_content_digest, v8.runs[0].dataset_content_digest,
        "current results use the new canonical digest domain"
    );

    let mut tampered = v8.clone();
    tampered.runs[0].analyses[0].waveforms[0].y[1] = 1.000_000_000_000_000_2;
    assert!(
        tampered
            .migrate_to_current(ProjectId::new())
            .expect_err("v8 tampering is rejected before resealing")
            .contains("schema-v8 analysis 1 result data digest")
    );

    let mut injected = v8;
    injected.runs[0].analyses[0].result_payload =
        PersistedField::Value(AnalysisResultPayload::ScalarMeasurements {
            values: std::collections::BTreeMap::from([("gain".to_owned(), 1.0)]),
        });
    assert!(
        injected
            .migrate_to_current(ProjectId::new())
            .expect_err("schema-v8 cannot inject a v9 payload")
            .contains("typed result payload introduced by schema v9")
    );
}

#[test]
fn schema_v9_digests_are_authenticated_before_current_resealing() {
    let mut run = SimulationRun::new(33);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Disto, "DISTO").with_result_payload(
            AnalysisResultPayload::ScalarMeasurements {
                values: std::collections::BTreeMap::from([("gain".to_owned(), 10.0)]),
            },
        ),
    );
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Reliability, "Reliability").with_family_metadata(
            AnalysisResultFamilyMetadata::Reliability {
                years: vec![1.0, 5.0, 10.0],
            },
        ),
    );
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 33;
    let mut v9 = ProjectSimulationResults::from_state(&simulation);
    v9.schema_version = TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION;
    for analysis in &mut v9.runs[0].analyses {
        analysis.result_data_digest = PersistedField::Value(
            analysis
                .clone()
                .into_analysis()
                .expect("v9 analysis fixture")
                .legacy_v2_result_data_digest(),
        );
    }
    v9.runs[0].dataset_content_digest = PersistedField::Value(
        v9.runs[0]
            .clone()
            .into_run()
            .expect("v9 run fixture")
            .legacy_v2_dataset_content_digest(),
    );

    let mut migrated = v9.clone();
    migrated
        .migrate_to_current(ProjectId::new())
        .expect("authentic v9 results migrate");
    assert_eq!(
        migrated.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    migrated
        .validate()
        .expect("resealed current results validate");
    assert!(
        migrated.runs[0].analyses[1].result_payload.is_missing(),
        "migration preserves the absence of v10 reliability evidence"
    );
    assert_ne!(
        migrated.runs[0].dataset_content_digest, v9.runs[0].dataset_content_digest,
        "current results use the v4 canonical digest domain"
    );

    let mut tampered = v9.clone();
    let Some(AnalysisResultPayload::ScalarMeasurements { values }) =
        tampered.runs[0].analyses[0].result_payload.as_mut()
    else {
        panic!("schema-v9 scalar payload")
    };
    values.insert("gain".to_owned(), 10.000_000_000_000_002);
    assert!(
        tampered
            .migrate_to_current(ProjectId::new())
            .expect_err("v9 tampering is rejected before resealing")
            .contains("schema-v9 analysis 1 result data digest")
    );

    let mut injected = v9.clone();
    injected.runs[0].analyses[0].result_payload =
        PersistedField::Value(AnalysisResultPayload::Reliability {
            devices: vec![crate::state::ReliabilityDeviceEvidence {
                device_id: "M1".to_owned(),
                stress: crate::state::ReliabilityStressEvidence {
                    average_gate_stress_v: 1.0,
                    average_drain_stress_v: 1.0,
                    average_temperature_k: 300.0,
                    duration_s: 1.0,
                },
                checkpoints: Vec::new(),
            }],
        });
    assert!(
        injected
            .migrate_to_current(ProjectId::new())
            .expect_err("schema-v9 cannot inject v10 evidence")
            .contains("Reliability/SOA evidence introduced by schema v10")
    );

    let mut injected_op = v9;
    injected_op.runs[0].analyses[0].result_payload =
        PersistedField::Value(operating_point_payload_fixture());
    assert!(
        injected_op
            .migrate_to_current(ProjectId::new())
            .expect_err("schema-v9 cannot inject v12 operating-point evidence")
            .contains("operating-point evidence introduced by schema v12")
    );
}

#[test]
fn schema_v10_digests_are_authenticated_before_v11_tf_resealing() {
    let mut run = SimulationRun::new(34);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_family_metadata(AnalysisResultFamilyMetadata::Reliability { years: vec![10.0] })
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![crate::state::ReliabilityDeviceEvidence {
                    device_id: "M1".to_owned(),
                    stress: crate::state::ReliabilityStressEvidence {
                        average_gate_stress_v: 1.2,
                        average_drain_stress_v: 1.8,
                        average_temperature_k: 358.15,
                        duration_s: 3_600.0,
                    },
                    checkpoints: vec![crate::state::ReliabilityCheckpointEvidence {
                        years: 10.0,
                        shift: crate::state::ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 0.03,
                            mobility_shift: -0.004,
                            drain_source_resistance_shift: 0.0015,
                        },
                    }],
                }],
            }),
    );
    run.add_analysis(AnalysisResult::new(2, AnalysisType::Tf, "TF"));
    seal_legacy_unattributed(&mut run);

    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 34;
    let mut v10 = ProjectSimulationResults::from_state(&simulation);
    v10.schema_version = RELIABILITY_SOA_RESULTS_SCHEMA_VERSION;
    for analysis in &mut v10.runs[0].analyses {
        analysis.result_data_digest = PersistedField::Value(
            analysis
                .clone()
                .into_analysis()
                .expect("v10 analysis fixture")
                .legacy_v3_result_data_digest(),
        );
    }
    v10.runs[0].dataset_content_digest = PersistedField::Value(
        v10.runs[0]
            .clone()
            .into_run()
            .expect("v10 run fixture")
            .legacy_v3_dataset_content_digest(),
    );

    let mut migrated = v10.clone();
    migrated
        .migrate_to_current(ProjectId::new())
        .expect("authentic v10 results migrate");
    assert_eq!(
        migrated.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    migrated.validate().expect("resealed v11 results validate");
    assert!(
        migrated.runs[0].analyses[1].result_payload.is_missing(),
        "migration preserves the absence of v11 transfer-function evidence"
    );
    assert_ne!(
        migrated.runs[0].dataset_content_digest, v10.runs[0].dataset_content_digest,
        "v11 uses the v4 canonical digest domain"
    );

    let mut tampered = v10.clone();
    let Some(AnalysisResultPayload::Reliability { devices }) =
        tampered.runs[0].analyses[0].result_payload.as_mut()
    else {
        panic!("schema-v10 reliability payload")
    };
    devices[0].stress.duration_s = 3_600.000_000_000_000_5;
    assert!(
        tampered
            .migrate_to_current(ProjectId::new())
            .expect_err("v10 tampering is rejected before resealing")
            .contains("schema-v10 analysis 1 result data digest")
    );

    let mut injected = v10.clone();
    injected.runs[0].analyses[1].result_payload =
        PersistedField::Value(AnalysisResultPayload::TransferFunction {
            input_source: "VIN".to_owned(),
            output_expression: "V(OUT)".to_owned(),
            input_quantity: crate::state::TransferFunctionQuantityEvidence::Voltage,
            output_quantity: crate::state::TransferFunctionQuantityEvidence::Voltage,
            input_unit: "V".to_owned(),
            output_unit: "V".to_owned(),
            normalization: crate::state::TransferFunctionNormalizationEvidence::None,
            accuracy: crate::state::TransferFunctionAccuracyEvidence::Balanced,
            gain: Some(crate::state::TransferFunctionScalarEvidence::Finite(0.5)),
            input_resistance: Some(crate::state::TransferFunctionScalarEvidence::PositiveInfinity),
            output_resistance: Some(crate::state::TransferFunctionScalarEvidence::Finite(50.0)),
            nominal_input: None,
            nominal_output: None,
        });
    assert!(
        injected
            .migrate_to_current(ProjectId::new())
            .expect_err("schema-v10 cannot inject v11 transfer-function evidence")
            .contains("transfer-function evidence introduced by schema v11")
    );

    let mut injected_op = v10;
    injected_op.runs[0].analyses[0].result_payload =
        PersistedField::Value(operating_point_payload_fixture());
    assert!(
        injected_op
            .migrate_to_current(ProjectId::new())
            .expect_err("schema-v10 cannot inject v12 operating-point evidence")
            .contains("operating-point evidence introduced by schema v12")
    );
}

#[test]
fn schema_v7_digest_migration_is_deterministic_and_rejects_anachronistic_fields() {
    let mut run = SimulationRun::new(3);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 3;
    let mut legacy = ProjectSimulationResults::from_state(&simulation);
    legacy.schema_version = FAMILY_METADATA_RESULTS_SCHEMA_VERSION;
    legacy.runs[0].dataset_content_digest = PersistedField::Missing;
    legacy.runs[0].analyses[0].result_data_digest = PersistedField::Missing;

    let mut first = legacy.clone();
    let mut second = legacy.clone();
    let project_id = ProjectId::new();
    first
        .migrate_to_current(project_id)
        .expect("first schema-v7 migration succeeds");
    second
        .migrate_to_current(project_id)
        .expect("identical schema-v7 migration succeeds");
    assert_eq!(
        first.runs[0].analyses[0].result_data_digest,
        second.runs[0].analyses[0].result_data_digest
    );
    assert_eq!(
        first.runs[0].dataset_content_digest,
        second.runs[0].dataset_content_digest
    );
    first.validate().expect("migrated digests validate");

    let mut injected_payload = legacy;
    injected_payload.runs[0].analyses[0].result_payload =
        PersistedField::Value(AnalysisResultPayload::ScalarMeasurements {
            values: std::collections::BTreeMap::from([("gain".to_owned(), 1.0)]),
        });
    let injected_before = injected_payload.clone();
    assert!(
        injected_payload
            .migrate_to_current(project_id)
            .expect_err("schema v7 cannot carry schema-v9 typed evidence")
            .contains("typed result payload introduced by schema v9")
    );
    assert_eq!(
        injected_payload, injected_before,
        "failed typed-payload migration is transactional"
    );

    let mut relabeled = first;
    relabeled.schema_version = FAMILY_METADATA_RESULTS_SCHEMA_VERSION;
    let before = relabeled.clone();
    assert!(
        relabeled
            .migrate_to_current(project_id)
            .expect_err("schema v7 cannot carry schema-v8 digest fields")
            .contains("introduced by schema v8")
    );
    assert_eq!(relabeled, before, "failed migration is transactional");
}

#[test]
fn legacy_schema_field_gate_rejects_relabelled_typed_evidence() {
    let mut run = SimulationRun::new(4);
    run.add_analysis(AnalysisResult::new(
        1,
        AnalysisType::Reliability,
        "Reliability",
    ));
    seal_legacy_unattributed(&mut run);
    let mut persisted_run = ProjectSimulationRun::from(&run);
    persisted_run.analyses[0].result_data_digest = PersistedField::Missing;
    persisted_run.dataset_content_digest = PersistedField::Missing;
    persisted_run.analyses[0].result_payload =
        PersistedField::Value(AnalysisResultPayload::Reliability {
            devices: vec![crate::state::ReliabilityDeviceEvidence {
                device_id: "M1".to_owned(),
                stress: crate::state::ReliabilityStressEvidence {
                    average_gate_stress_v: 1.0,
                    average_drain_stress_v: 1.0,
                    average_temperature_k: 300.0,
                    duration_s: 1.0,
                },
                checkpoints: vec![crate::state::ReliabilityCheckpointEvidence {
                    years: 1.0,
                    shift: crate::state::ReliabilityShiftEvidence {
                        threshold_voltage_shift_v: 0.01,
                        mobility_shift: -0.001,
                        drain_source_resistance_shift: 0.0001,
                    },
                }],
            }],
        });

    for source_schema in
        LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION..=CONTENT_DIGEST_RESULTS_SCHEMA_VERSION
    {
        assert!(
            validate_result_fields_for_source_schema(&persisted_run, source_schema)
                .expect_err("pre-v9 schema cannot carry a typed payload")
                .contains("typed result payload introduced by schema v9")
        );
    }

    persisted_run.analyses[0].result_payload = PersistedField::Missing;
    persisted_run.analyses[0].family_metadata =
        Some(AnalysisResultFamilyMetadata::Reliability { years: vec![1.0] });
    for source_schema in
        LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION..FAMILY_METADATA_RESULTS_SCHEMA_VERSION
    {
        assert!(
            validate_result_fields_for_source_schema(&persisted_run, source_schema)
                .expect_err("pre-v7 schema cannot carry family metadata")
                .contains("family metadata introduced by schema v7")
        );
    }
}

#[test]
fn missing_persisted_lifecycle_restores_as_explicit_legacy_unknown() {
    let mut run = SimulationRun::new(13);
    seal_legacy_unattributed(&mut run);
    let expected_run_id = run.run_id;
    let mut persisted = ProjectSimulationRun::from(&run);
    persisted.job_id = None;
    persisted.execution_target = None;
    persisted.lifecycle = None;

    let restored = persisted.into_run().expect("legacy run restores");

    assert_eq!(restored.run_id, expected_run_id);
    assert_eq!(restored.job_id, None);
    assert_eq!(restored.execution_target, None);
    assert_eq!(restored.lifecycle, SimulationRunLifecycle::LegacyUnknown);
}

/// Solve a deck and hand back the operating-point report the engine really
/// produced, so the round trip below is judged against emitted labels rather
/// than against labels a fixture chose.
fn solved_device_op_report(deck: &str) -> rspice_core::circuit::DeviceOpReport {
    let netlist = rspice_core::netlist::Netlist::parse(deck).expect("op deck parses");
    let (_, report) =
        rspice_core::engine::Engine::new(rspice_core::engine::SimulationConfig::default())
            .run_dc_op_with_report(&netlist)
            .expect("operating point solves");
    report
}

fn run_retaining(report: rspice_core::circuit::DeviceOpReport) -> SimulationRun {
    let mut run = SimulationRun::new(61);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcOp, "Operating point").with_device_op(report),
    );
    seal_legacy_unattributed(&mut run);
    run
}

/// A retained operating-point report is engine output, and every label in it
/// has to be one the reader can restore, because a label it cannot resolve
/// fails validation and refuses the whole project.
///
/// Both families here were refused: the diode reports a junction capacitance
/// and the VDMOS reports a device family, an operating region and a dissipation
/// figure that the reader's vocabulary never carried, so an operating point
/// containing either could not be written at all.
#[test]
fn a_run_retaining_a_device_operating_point_saves_and_reopens() {
    let diode = "\
* junction diode operating point
v1 in 0 dc 5
r1 in a 1k
d1 a 0 dmod
.model dmod D IS=1e-14 N=1.5
.op
.end
";
    let vdmos = "\
* power VDMOS operating point
vd d 0 dc 10
vg g 0 dc 5
m1 d g 0 0 irfmod W=0.386 L=2.5u
.MODEL irfmod NMOS LEVEL=18 VTO=3.5 RS=0.005 M=3
.op
.end
";

    for (family, deck, quantity) in [("DIODE", diode, "cd"), ("VDMOS", vdmos, "power")] {
        let report = solved_device_op_report(deck);
        let emitted = report
            .entries
            .iter()
            .find(|entry| entry.device_kind == family)
            .unwrap_or_else(|| panic!("the {family} deck reports that family"));
        assert!(
            emitted.params.iter().any(|(name, _)| *name == quantity),
            "the {family} entry reports {quantity}, the quantity the reader used to refuse"
        );

        let mut simulation = SimulationState::default();
        simulation.next_run_id = 61;
        simulation.runs = vec![run_retaining(report.clone())];

        let persisted = ProjectSimulationResults::from_state(&simulation);
        persisted
            .validate()
            .unwrap_or_else(|error| panic!("a {family} operating point is persistable: {error}"));

        let mut reloaded = SimulationState::default();
        persisted
            .apply_to_state(&mut reloaded)
            .unwrap_or_else(|error| panic!("a saved {family} operating point reopens: {error}"));

        let restored = reloaded.runs[0].analyses[0]
            .device_op
            .as_ref()
            .expect("the reopened run still carries its operating-point report");
        assert_eq!(restored.entries.len(), report.entries.len());
        for (before, after) in report.entries.iter().zip(&restored.entries) {
            assert_eq!(before.name, after.name);
            assert_eq!(before.device_kind, after.device_kind);
            assert_eq!(before.region, after.region);
            assert_eq!(
                before
                    .params
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>(),
                after
                    .params
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>(),
                "{family} quantity names survive the round trip"
            );
        }
    }
}

/// The reader's accept set is the engine's own label vocabulary, so nothing the
/// engine can name is refused on write. This is the property that failed: the
/// reader carried a hand-copied subset, and every family that had outgrown it
/// wrote a project the reader would not take back.
#[test]
fn the_project_reader_accepts_every_label_the_engine_can_emit() {
    for label in rspice_core::circuit::OP_LABELS {
        require_static_label(label.as_str(), "device_op label")
            .unwrap_or_else(|error| panic!("{error}"));
    }
}

/// Completing the vocabulary is not the same as dropping the check: text the
/// engine has no label for still refuses the write, because restoring it would
/// invent a quantity the report never carried.
#[test]
fn an_operating_point_label_outside_the_vocabulary_is_still_refused() {
    let report = rspice_core::circuit::DeviceOpReport {
        entries: vec![rspice_core::circuit::DeviceOpEntry {
            name: "XU1".to_owned(),
            device_kind: "DIODE",
            region: None,
            params: vec![("vd", 0.7), ("vendor-quantity", 1.0)],
        }],
    };
    let mut simulation = SimulationState::default();
    simulation.next_run_id = 61;
    simulation.runs = vec![run_retaining(report)];

    let error = ProjectSimulationResults::from_state(&simulation)
        .validate()
        .expect_err("an unrecognized quantity refuses the write");
    assert!(
        error.contains("unknown static label 'vendor-quantity'"),
        "{error}"
    );
}

/// Solve a deck's noise analysis and hand back the ranked contributor summary
/// the engine really produced, so the round trip below is judged against
/// emitted mechanisms rather than against mechanisms a fixture chose.
fn solved_noise_summary(deck: &str, output: &str, input: &str) -> crate::state::NoiseSummary {
    let netlist = rspice_core::netlist::Netlist::parse(deck).expect("noise deck parses");
    let frequencies = [1.0e1, 1.0e2, 1.0e3, 1.0e4, 1.0e5];
    let results =
        rspice_core::engine::Engine::new(rspice_core::engine::SimulationConfig::default())
            .run_noise_named_with_input_source(&netlist, output, None, input, &frequencies, 300.15)
            .expect("noise analysis runs");
    let integrated = rspice_core::analysis::IntegratedNoise::new(results);
    crate::state::NoiseSummary {
        rows: integrated
            .contribution_summary()
            .into_iter()
            .map(|contribution| crate::state::NoiseContributorRow {
                device: contribution.device_name,
                mechanism: contribution.mechanism,
                power: contribution.integrated_power,
                share_pct: contribution.percentage,
            })
            .collect(),
        total_rms: Some(integrated.total_output_noise()),
        input_rms: Some(integrated.total_input_referred_noise()),
        band: (frequencies[0], frequencies[frequencies.len() - 1]),
    }
}

fn run_retaining_noise(summary: crate::state::NoiseSummary) -> SimulationRun {
    let mut run = SimulationRun::new(62);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Noise, "Noise").with_noise_summary(summary),
    );
    seal_legacy_unattributed(&mut run);
    run
}

/// A ranked noise summary is engine output, and its mechanisms come from the
/// device model rather than from the broad physical class the reader used to
/// accept. A MOSFET names its channel and series-resistance sources, a bipolar
/// names its transport, base and parasitic-resistance sources, and retaining
/// the top contributors is the default, so a run containing either refused the
/// whole project on save.
#[test]
fn a_run_retaining_a_noise_summary_saves_and_reopens() {
    let mosfet = "\
* classic level-1 MOSFET noise
vdd dd 0 dc 5
rl dd d 10k
vin g 0 dc 2 ac 1
m1 d g 0 0 nmod w=10u l=1u
.model nmod NMOS (LEVEL=1 VTO=1 KP=100u RD=10 RS=10 KF=1e-24 AF=1)
.end
";
    let bipolar = "\
* Gummel-Poon bipolar noise
vcc cc 0 dc 10
rl cc c 10k
vin bb 0 dc 0.75 ac 1
rb bb b 1k
q1 c b 0 qmod
.model qmod NPN (IS=1e-16 BF=100 RB=100 RC=10 RE=1 KF=1e-14 AF=1)
.end
";

    for (family, deck, output, mechanisms) in [
        ("MOSFET", mosfet, "d", ["ID", "FN", "RD", "RS"].as_slice()),
        (
            "BJT",
            bipolar,
            "c",
            ["IC", "IB", "FN", "RB", "RC", "RE"].as_slice(),
        ),
    ] {
        let summary = solved_noise_summary(deck, output, "vin");
        let emitted = summary
            .rows
            .iter()
            .map(|row| row.mechanism.as_str())
            .collect::<Vec<_>>();
        for mechanism in mechanisms {
            assert!(
                emitted.contains(mechanism),
                "the {family} deck contributes {mechanism}, one of the mechanisms the reader refused; got {emitted:?}"
            );
        }

        let mut simulation = SimulationState::default();
        simulation.next_run_id = 62;
        simulation.runs = vec![run_retaining_noise(summary.clone())];

        let persisted = ProjectSimulationResults::from_state(&simulation);
        persisted
            .validate()
            .unwrap_or_else(|error| panic!("a {family} noise summary is persistable: {error}"));

        let mut reloaded = SimulationState::default();
        persisted
            .apply_to_state(&mut reloaded)
            .unwrap_or_else(|error| panic!("a saved {family} noise summary reopens: {error}"));

        let restored = reloaded.runs[0].analyses[0]
            .noise_summary
            .as_ref()
            .expect("the reopened run still carries its noise summary");
        assert_eq!(
            restored
                .rows
                .iter()
                .map(|row| (row.device.as_str(), row.mechanism.as_str()))
                .collect::<Vec<_>>(),
            summary
                .rows
                .iter()
                .map(|row| (row.device.as_str(), row.mechanism.as_str()))
                .collect::<Vec<_>>(),
            "{family} contributor identities survive the round trip"
        );
    }
}

/// Bounding the mechanism is not the same as dropping the check: text no
/// emitter can compose still refuses the write, because a summary is not a
/// place to put free prose or unbounded input.
#[test]
fn a_noise_mechanism_outside_the_persistable_shape_is_still_refused() {
    for mechanism in [
        String::new(),
        "channel thermal".to_owned(),
        "a".repeat(rspice_core::analysis::NOISE_MECHANISM_MAX_BYTES + 1),
    ] {
        let summary = crate::state::NoiseSummary {
            rows: vec![crate::state::NoiseContributorRow {
                device: "M1".to_owned(),
                mechanism,
                power: 1e-18,
                share_pct: 100.0,
            }],
            total_rms: Some(1e-9),
            input_rms: Some(1e-9),
            band: (1.0, 1.0e5),
        };
        let mut simulation = SimulationState::default();
        simulation.next_run_id = 62;
        simulation.runs = vec![run_retaining_noise(summary)];

        let error = ProjectSimulationResults::from_state(&simulation)
            .validate()
            .expect_err("a mechanism outside the shape refuses the write");
        assert!(error.contains("is not a noise mechanism"), "{error}");
    }
}

mod migration;
