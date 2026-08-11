//! Tests for snapshot identity and authorization.
//!
//! A snapshot must name exactly the netlist and task graph it was taken from,
//! so the cases here concentrate on refusal: a digest that does not match its
//! task, and a PVT change that must produce a new identity.

use super::*;
use crate::simulation::AnalysisConfig;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::runner::SpecExecutionOptions;

fn task() -> QueuedAnalysis {
    QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::dc_op(),
        config: Some(AnalysisConfig::dc_op()),
        spec_options: SpecExecutionOptions::default(),
        analysis_line: ".op".to_owned(),
    }
}

fn configured_op_task(config: crate::simulation::dialog::OpConfig) -> QueuedAnalysis {
    QueuedAnalysis {
        numeric_override: None,
        spec: operating_point_spec(&config),
        config: Some(AnalysisConfig::DcOp(config)),
        spec_options: SpecExecutionOptions::default(),
        analysis_line: ".op".to_owned(),
    }
}

fn transient_task() -> QueuedAnalysis {
    let config = crate::simulation::config::TransientAnalysisConfig {
        stop_time: 1.0e-6,
        step_time: 1.0e-9,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    };
    QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::Transient {
            stop_time: config.stop_time,
            step_time: config.step_time,
            start_time: config.start_time,
            max_timestep: config.max_timestep,
            uic: config.uic,
        },
        config: Some(AnalysisConfig::Transient(config)),
        spec_options: SpecExecutionOptions::default(),
        analysis_line: ".tran 1n 1u".to_owned(),
    }
}

fn temperature_task(temperatures_c: Vec<f64>) -> QueuedAnalysis {
    QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::Parametric,
        config: None,
        spec_options: SpecExecutionOptions {
            temp: Some(crate::services::simulation_runner::TempRunConfig {
                temperatures_c,
                base_mode: crate::services::simulation_runner::CornerBaseMode::Op,
            }),
            ..SpecExecutionOptions::default()
        },
        analysis_line: ".temp".to_owned(),
    }
}

fn corner_task(
    process_corners: Vec<crate::services::simulation_runner::CornerProcess>,
    voltages: Vec<f64>,
    temperatures_c: Vec<f64>,
    full_matrix: bool,
) -> QueuedAnalysis {
    QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::Corner,
        config: None,
        spec_options: SpecExecutionOptions {
            corner: Some(crate::services::simulation_runner::CornerRunConfig {
                process_corners,
                voltages,
                temperatures_c,
                full_matrix,
                nominal_voltage: Some(1.0),
                base_mode: crate::services::simulation_runner::CornerBaseMode::Op,
                model_bindings: Vec::new(),
                points: Vec::new(),
            }),
            ..SpecExecutionOptions::default()
        },
        analysis_line: ".corner".to_owned(),
    }
}

fn corner_binding(
    process: crate::services::simulation_runner::CornerProcess,
    source_label: &str,
    saturation_current: &str,
) -> crate::services::simulation_runner::CornerModelBinding {
    crate::services::simulation_runner::CornerModelBinding {
        process,
        source_label: source_label.to_owned(),
        section: Some(process.as_keyword().to_owned()),
        materialized_model_cards: format!(".model DPROCESS D (IS={saturation_current})"),
    }
}

fn instance_id(name: &str) -> AnalysisInstanceId {
    const TEST_NAMESPACE: uuid::Uuid =
        uuid::Uuid::from_u128(0x3ce2_b258_0c75_55f0_96d4_8fc1_cadf_1384);
    AnalysisInstanceId::from_namespace(TEST_NAMESPACE, name.as_bytes())
}

fn prepared(name: &str, label: &str, task: QueuedAnalysis) -> PreparedTask {
    PreparedTask::new(
        instance_id(name),
        ObjectRevision::INITIAL,
        Vec::new(),
        label,
        task,
    )
}

fn prepared_with(
    name: &str,
    revision: ObjectRevision,
    dependencies: Vec<AnalysisInstanceId>,
    label: &str,
    task: QueuedAnalysis,
) -> PreparedTask {
    PreparedTask::new(instance_id(name), revision, dependencies, label, task)
}

fn parts() -> SnapshotParts {
    const TEST_NAMESPACE: uuid::Uuid =
        uuid::Uuid::from_u128(0xe6bc_c27a_6103_5327_b2ec_c759_b58a_8598);
    SnapshotParts {
        intent: SimulationRunIntent::SimulateRunSet,
        simulation_plan_id: Some(SimulationPlanId::from_namespace(
            TEST_NAMESPACE,
            b"snapshot-test-plan",
        )),
        project_revision: 3,
        topology_revision: 4,
        source_digest: ContentDigest::from_bytes([1; 32]),
        reference_process: ProcessCorner::TT,
        reference_temperature_celsius: 27.0,
        tasks: vec![prepared("op", "DC Operating Point", task())],
        executable_netlist: "deck\n.op\n.end\n".to_owned(),
        save_policy: SavePolicy::RetainEngineProducedResults,
        model_identities: Vec::new(),
        project_model_sources: Vec::new(),
        project_veriloga_runtimes: Default::default(),
        target: ExecutionTargetCapabilities::current(),
        receipt: RunSourceReceipt::SchematicDrc(ContentDigest::from_bytes([4; 32])),
        advisories: Vec::new(),
        manual_source: None,
        cross_probe: None,
        touchstone_export: TouchstoneExportPolicy::disabled(),
        sealed_source_dependencies: Vec::new(),
    }
}

fn project_runtime() -> crate::simulation::veriloga::PreparedVerilogARuntime {
    let project_id = crate::product::ProjectId::new();
    let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::code_workspace(
                crate::state::ProjectSourceLanguage::VerilogA,
            ),
            crate::state::ProjectSourceLanguage::VerilogA,
            "file_name_differs.va",
            "module snapshot_owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
            [],
            [],
        )
        .unwrap();
    let receipt = crate::workbench::documents::code_workspace::compile_project_bundle_receipt(
        project_id,
        &bundle,
        Some("snapshot_owned"),
    )
    .unwrap();
    crate::simulation::veriloga::PreparedVerilogARuntime::try_from_current_bundle_receipt(
        project_id, &bundle, &receipt,
    )
    .unwrap()
}

fn signed_pdk_runtime() -> crate::simulation::veriloga::PreparedVerilogARuntime {
    let (archive, trust, authority) =
        crate::state::pdk_config::signed_veriloga_technology_test_fixture();
    let mut registry = crate::state::pdk_config::PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(
            &archive,
            &trust,
            &authority,
            "Prepare signed PDK runtime snapshot fixture",
        )
        .unwrap();
    let package = registry.validated_packages()[0].clone();
    let sealed = registry
        .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
        .unwrap();
    crate::simulation::veriloga::compile_signed_pdk_source_runtime(
        &sealed.binding,
        sealed.archive_digest,
        &sealed.veriloga_artifacts,
        &sealed.veriloga_bindings[0],
    )
    .unwrap()
}

#[test]
fn snapshot_rejects_positive_area_model_bin_overlap_before_dispatch() {
    let mut ambiguous = parts();
    ambiguous.executable_netlist = "ambiguous binned mosfet\n\
         V1 d 0 1\n\
         M1 d d 0 0 NCH W=1u L=0.5u\n\
         .model NCH.0 NMOS LEVEL=1 LMIN=0.28u LMAX=0.7u WMIN=0.5u WMAX=2u VTO=0.4\n\
         .model NCH.1 NMOS LEVEL=1 LMIN=0.4u LMAX=1.2u WMIN=0.5u WMAX=2u VTO=0.9\n\
         .op\n\
         .end\n"
        .to_owned();

    let error = PreparedRunSnapshot::new(ambiguous)
        .expect_err("an ambiguous model-bin instance must never reach dispatch");
    assert_eq!(error.stage(), PreparationStage::ModelBindings);
    assert!(error.message().contains("MOSFET 'M1'"));
    assert!(error.message().contains("model family 'NCH' is ambiguous"));
}

#[test]
fn snapshot_requires_the_exact_aliased_project_runtime_directive() {
    let runtime = project_runtime();
    let mut missing = parts();
    missing.project_veriloga_runtimes =
        crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime.clone()])
            .unwrap();
    assert!(matches!(
        PreparedRunSnapshot::new(missing),
        Err(PreparationError {
            stage: PreparationStage::ModelBindings,
            ..
        })
    ));

    let directive = crate::simulation::veriloga::project_veriloga_directive(
        runtime.source_key(),
        runtime.netlist_alias(),
    );
    let mut suffixed = parts();
    suffixed.executable_netlist = format!("deck\n{directive} unexpected\n.op\n.end\n");
    suffixed.project_veriloga_runtimes =
        crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime.clone()])
            .unwrap();
    assert!(PreparedRunSnapshot::new(suffixed).is_err());

    let mut exact = parts();
    exact.executable_netlist = format!("deck\n{directive}\n.op\n.end\n");
    exact.project_veriloga_runtimes =
        crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime]).unwrap();
    assert!(PreparedRunSnapshot::new(exact).is_ok());
}

#[test]
fn snapshot_binds_signed_pdk_veriloga_runtime_and_archive_provenance() {
    let runtime = signed_pdk_runtime();
    assert!(
        runtime
            .provenance_label()
            .starts_with("signed-pdk-veriloga:__rspice_pdk__/")
    );
    let directive = crate::simulation::veriloga::project_veriloga_directive(
        runtime.source_key(),
        runtime.netlist_alias(),
    );
    let mut prepared = parts();
    prepared.executable_netlist = format!("signed PDK runtime\n{directive}\n.op\n.end\n");
    prepared.project_veriloga_runtimes =
        crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime]).unwrap();
    let snapshot = PreparedRunSnapshot::new(prepared).expect("signed runtime snapshot validates");
    assert_eq!(snapshot.metadata().model_identity_count, 1);
}

#[test]
fn snapshot_rejects_unsealed_or_duplicate_veriloga_directives() {
    let runtime = project_runtime();
    let directive = crate::simulation::veriloga::project_veriloga_directive(
        runtime.source_key(),
        runtime.netlist_alias(),
    );
    let runtime_set =
        crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime]).unwrap();

    let mut unsealed = parts();
    unsealed.executable_netlist = format!(
        "deck\n{directive}\n.veriloga \"__rspice_project__/foreign.va\" FOREIGN\n.op\n.end\n"
    );
    unsealed.project_veriloga_runtimes = runtime_set.clone();
    assert!(matches!(
        PreparedRunSnapshot::new(unsealed),
        Err(PreparationError {
            stage: PreparationStage::ModelBindings,
            ..
        })
    ));

    let mut duplicated = parts();
    duplicated.executable_netlist = format!("deck\n{directive}\n{directive}\n.op\n.end\n");
    duplicated.project_veriloga_runtimes = runtime_set;
    assert!(matches!(
        PreparedRunSnapshot::new(duplicated),
        Err(PreparationError {
            stage: PreparationStage::ModelBindings,
            ..
        })
    ));
}

#[test]
fn task_order_changes_snapshot_identity() {
    let mut two = parts();
    two.tasks
        .push(prepared("tran", "Transient", transient_task()));
    let ordered = PreparedRunSnapshot::new(two).expect("ordered snapshot");

    let mut reversed = parts();
    reversed.tasks = vec![
        prepared("tran", "Transient", transient_task()),
        prepared("op", "DC Operating Point", task()),
    ];
    let reversed = PreparedRunSnapshot::new(reversed).expect("reversed snapshot");

    assert_ne!(ordered.digest(), reversed.digest());
}

#[test]
fn owning_plan_identity_is_authenticated_and_required_for_plan_dispatch() {
    const OTHER_PLAN_NAMESPACE: uuid::Uuid =
        uuid::Uuid::from_u128(0xb77a_31fd_d31e_54f4_9507_1b1a_26ce_53fa);
    let first = PreparedRunSnapshot::new(parts()).expect("first plan snapshot");
    let mut changed = parts();
    changed.simulation_plan_id = Some(SimulationPlanId::from_namespace(
        OTHER_PLAN_NAMESPACE,
        b"other-plan",
    ));
    let changed = PreparedRunSnapshot::new(changed).expect("changed plan snapshot");
    assert_ne!(first.digest(), changed.digest());

    let mut missing = parts();
    missing.simulation_plan_id = None;
    let error = PreparedRunSnapshot::new(missing)
        .expect_err("plan dispatch without an owning plan must fail closed");
    assert_eq!(error.stage(), PreparationStage::Authorization);
    assert!(error.message().contains("simulation plan identity"));
}

#[test]
fn task_revision_and_exact_dependency_graph_change_snapshot_identity() {
    let mut independent = parts();
    independent
        .tasks
        .push(prepared("tran", "Transient", transient_task()));
    let independent = PreparedRunSnapshot::new(independent).expect("independent task snapshot");

    let mut dependent = parts();
    dependent.tasks.push(prepared_with(
        "tran",
        ObjectRevision::INITIAL,
        vec![instance_id("op")],
        "Transient",
        transient_task(),
    ));
    let dependent = PreparedRunSnapshot::new(dependent).expect("dependent task snapshot");
    assert_ne!(independent.digest(), dependent.digest());

    let base_revision = PreparedRunSnapshot::new(parts()).expect("base revision snapshot");
    let mut revised = parts();
    revised.tasks[0] = prepared_with(
        "op",
        ObjectRevision::new(2).expect("revision two"),
        Vec::new(),
        "DC Operating Point",
        task(),
    );
    let revised = PreparedRunSnapshot::new(revised).expect("revised task snapshot");
    assert_ne!(base_revision.digest(), revised.digest());
}

#[test]
fn snapshot_rejects_mixed_task_source_revisions() {
    let mut mixed = parts();
    mixed.tasks.push(prepared_with(
        "tran",
        ObjectRevision::new(2).expect("revision two"),
        Vec::new(),
        "Transient",
        transient_task(),
    ));

    let error =
        PreparedRunSnapshot::new(mixed).expect_err("one frozen run cannot mix source revisions");
    assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
    assert!(error.message().contains("cannot mix source revisions"));
}

#[test]
fn snapshot_rejects_duplicate_ids_and_duplicate_dependency_edges() {
    let mut empty = parts();
    empty.tasks.clear();
    let error = PreparedRunSnapshot::new(empty).expect_err("empty graph must fail");
    assert!(error.message().contains("at least one analysis"));

    let mut duplicate_id = parts();
    duplicate_id
        .tasks
        .push(prepared("op", "Duplicate OP", task()));
    let error = PreparedRunSnapshot::new(duplicate_id).expect_err("duplicate ID must fail");
    assert!(error.message().contains("duplicate analysis instance"));

    let mut duplicate_edge = parts();
    duplicate_edge.tasks.push(prepared_with(
        "tran",
        ObjectRevision::INITIAL,
        vec![instance_id("op"), instance_id("op")],
        "Transient",
        transient_task(),
    ));
    let error = PreparedRunSnapshot::new(duplicate_edge).expect_err("duplicate edge must fail");
    assert!(error.message().contains("duplicate dependency"));
}

#[test]
fn snapshot_rejects_self_dangling_later_and_cyclic_dependencies() {
    let mut self_edge = parts();
    self_edge.tasks[0].dependencies = vec![instance_id("op")];
    let error = PreparedRunSnapshot::new(self_edge).expect_err("self edge must fail");
    assert!(error.message().contains("depend on itself"));

    let mut dangling = parts();
    dangling.tasks[0].dependencies = vec![instance_id("missing")];
    let error = PreparedRunSnapshot::new(dangling).expect_err("dangling edge must fail");
    assert!(error.message().contains("missing dependency"));

    let mut later = parts();
    later.tasks = vec![
        prepared_with(
            "tran",
            ObjectRevision::INITIAL,
            vec![instance_id("op")],
            "Transient",
            transient_task(),
        ),
        prepared("op", "DC Operating Point", task()),
    ];
    let error = PreparedRunSnapshot::new(later).expect_err("later edge must fail");
    assert!(error.message().contains("must appear earlier"));

    let mut cycle = parts();
    cycle.tasks = vec![
        prepared_with(
            "op",
            ObjectRevision::INITIAL,
            vec![instance_id("tran")],
            "DC Operating Point",
            task(),
        ),
        prepared_with(
            "tran",
            ObjectRevision::INITIAL,
            vec![instance_id("op")],
            "Transient",
            transient_task(),
        ),
    ];
    let error = PreparedRunSnapshot::new(cycle).expect_err("cycle must fail");
    assert!(error.message().contains("dependency cycle"));
}

#[test]
fn source_content_change_changes_snapshot_identity_without_revision_change() {
    let first = PreparedRunSnapshot::new(parts()).expect("first snapshot");
    let mut changed = parts();
    changed.executable_netlist = "deck\n.op\nR1 out 0 1k\n.end\n".to_owned();
    changed.source_digest = ContentDigest::from_bytes([9; 32]);
    let changed = PreparedRunSnapshot::new(changed).expect("changed snapshot");
    assert_ne!(first.digest(), changed.digest());
}

#[test]
fn retained_op_state_must_match_the_prepared_executable_source() {
    use crate::simulation::dialog::{OpInitialGuess, OpNodeInitialization, OpPreviousState};

    let mut matching = parts();
    let previous = OpPreviousState {
        source_content_digest: super::super::canonical::operating_point_effective_source_digest(
            &matching.executable_netlist,
            crate::simulation::dialog::OpRunPointContext::default(),
        ),
        producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
        producer_result_digest: ContentDigest::from_bytes([3; 32]),
        node_names: vec!["out".to_owned()],
        branch_names: Vec::new(),
        solution: vec![1.25],
    };
    let mut config = crate::simulation::dialog::OpConfig::default();
    config.initial_guess = OpInitialGuess::PreviousConverged;
    config.node_initialization = OpNodeInitialization::IgnoreIcAndNodeset;
    config.previous_state = Some(previous.clone());
    let mut spec = AnalysisSpec::dc_op();
    let AnalysisSpec::DcOp {
        initial_guess,
        node_initialization,
        previous_state,
        ..
    } = &mut spec
    else {
        unreachable!("current OP constructor returns the configured variant");
    };
    *initial_guess = OpInitialGuess::PreviousConverged;
    *node_initialization = OpNodeInitialization::IgnoreIcAndNodeset;
    *previous_state = Some(previous);
    let retained_task = QueuedAnalysis {
        numeric_override: None,
        spec,
        config: Some(AnalysisConfig::DcOp(config)),
        spec_options: SpecExecutionOptions::default(),
        analysis_line: ".op".to_owned(),
    };

    matching.tasks = vec![prepared("op", "DC Operating Point", retained_task.clone())];
    PreparedRunSnapshot::new(matching).expect("matching source-bound state");

    let mut changed = parts();
    changed.executable_netlist = "deck\nR1 out 0 1k\n.op\n.end\n".to_owned();
    changed.source_digest = ContentDigest::from_bytes([9; 32]);
    changed.tasks = vec![prepared("op", "DC Operating Point", retained_task)];
    let error = PreparedRunSnapshot::new(changed)
        .expect_err("stale retained state must fail before dispatch");
    assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
    assert!(error.message().contains("different executable source"));
}

#[test]
fn retained_soa_context_must_match_the_prepared_executable_source() {
    use crate::simulation::dialog::OpDeviceDetail;

    let soa_source = ContentDigest::from_bytes([1; 32]);
    let mut config = crate::simulation::dialog::OpConfig::default();
    config.device_detail = OpDeviceDetail::ViolationsOnly;
    config.violation_devices = vec!["M1".to_owned()];
    config.violation_source_content_digest = Some(soa_source);
    let mut spec = AnalysisSpec::dc_op();
    let AnalysisSpec::DcOp {
        device_detail,
        violation_devices,
        violation_source_content_digest,
        ..
    } = &mut spec
    else {
        unreachable!("current OP constructor returns the configured variant");
    };
    *device_detail = OpDeviceDetail::ViolationsOnly;
    *violation_devices = vec!["M1".to_owned()];
    *violation_source_content_digest = Some(soa_source);
    let retained_task = QueuedAnalysis {
        numeric_override: None,
        spec,
        config: Some(AnalysisConfig::DcOp(config)),
        spec_options: SpecExecutionOptions::default(),
        analysis_line: ".op".to_owned(),
    };

    let mut matching = parts();
    matching.tasks = vec![prepared("op", "DC Operating Point", retained_task.clone())];
    PreparedRunSnapshot::new(matching).expect("matching source-bound SOA context");

    let mut changed = parts();
    changed.source_digest = ContentDigest::from_bytes([9; 32]);
    changed.tasks = vec![prepared("op", "DC Operating Point", retained_task)];
    let error = PreparedRunSnapshot::new(changed)
        .expect_err("stale retained SOA evidence must fail before dispatch");
    assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
    assert!(error.message().contains("SOA violation evidence"));
}

#[test]
fn target_capability_change_changes_snapshot_identity() {
    let first = PreparedRunSnapshot::new(parts()).expect("first snapshot");
    let mut changed = parts();
    changed.target.cancellable = !changed.target.cancellable;
    let changed = PreparedRunSnapshot::new(changed).expect("changed target snapshot");
    assert_ne!(first.digest(), changed.digest());
}

#[test]
fn every_execution_target_advertises_verified_cancellation() {
    let target = ExecutionTargetCapabilities::current();
    assert!(target.cancellable);
    assert_eq!(execution_target_supports_cancellation(), target.cancellable);
}

#[test]
fn automatic_export_policy_is_authenticated_by_snapshot_identity() {
    let first = PreparedRunSnapshot::new(parts()).expect("disabled export snapshot");
    let mut changed = parts();
    changed.touchstone_export =
        TouchstoneExportPolicy::enabled(2, PathBuf::from("sealed-output"), OsString::from("amp"))
            .expect("valid export policy");
    let changed = PreparedRunSnapshot::new(changed).expect("enabled export snapshot");

    assert_ne!(first.digest(), changed.digest());
    assert_eq!(
        changed
            .touchstone_export
            .output_path(12, 3, 4)
            .expect("enabled output path"),
        PathBuf::from("sealed-output").join("amp_run0012_sp03.s4p")
    );
}

#[test]
fn automatic_export_identity_is_derived_from_the_exact_output_prefix() {
    let first =
        TouchstoneExportPolicy::enabled(2, PathBuf::from("sealed-output-a"), OsString::from("amp"))
            .expect("first output policy");
    let second =
        TouchstoneExportPolicy::enabled(2, PathBuf::from("sealed-output-b"), OsString::from("amp"))
            .expect("second output policy");
    assert_ne!(first, second);

    assert!(
        TouchstoneExportPolicy::enabled(
            2,
            PathBuf::from("sealed-output"),
            OsString::from("../redirect"),
        )
        .is_err(),
        "a stem must not be able to redirect the captured directory"
    );
}

#[test]
fn model_identity_set_order_does_not_change_snapshot_identity() {
    let first_identity = ModelSourceIdentity::new("first", ContentDigest::from_bytes([0x11; 32]));
    let second_identity = ModelSourceIdentity::new("second", ContentDigest::from_bytes([0x22; 32]));
    let mut forward = parts();
    forward.model_identities = vec![first_identity.clone(), second_identity.clone()];
    let forward = PreparedRunSnapshot::new(forward).expect("forward snapshot");
    let mut reverse = parts();
    reverse.model_identities = vec![second_identity, first_identity];
    let reverse = PreparedRunSnapshot::new(reverse).expect("reverse snapshot");
    assert_eq!(forward.digest(), reverse.digest());
}

#[test]
fn pvt_metadata_counts_the_exact_full_corner_matrix_inside_one_task() {
    use crate::services::simulation_runner::CornerProcess;
    let mut matrix = parts();
    let mut corner = corner_task(
        vec![CornerProcess::TT, CornerProcess::FF],
        vec![0.9, 1.1],
        vec![-40.0, 125.0],
        true,
    );
    corner
        .spec_options
        .corner
        .as_mut()
        .expect("corner config")
        .model_bindings = vec![corner_binding(CornerProcess::FF, "ff.lib", "1e-11")];
    matrix.tasks = vec![prepared("corner", "Corner", corner)];

    let snapshot = PreparedRunSnapshot::new(matrix).expect("full corner matrix snapshot");
    assert_eq!(snapshot.pvt_points.len(), 8);
    assert_eq!(snapshot.tasks.len(), 1);
    assert_eq!(snapshot.metadata().pvt_point_count, 8);
}

/// A diagonal sweep pairs the axes index by index, and axes of different
/// non-scalar lengths have no such pairing.
///
/// This used to expand by modular arithmetic: two processes against three
/// voltages produced a third point whose process silently wrapped back to the
/// first, so the manifest recorded a corner the plan never declared. The
/// expansion now refuses it, and a scalar axis — which is genuinely shared by
/// every point — still pairs with any length.
#[test]
fn a_diagonal_corner_sweep_refuses_unequal_axes_and_shares_a_scalar_one() {
    use crate::services::simulation_runner::CornerProcess;

    let mut unequal = parts();
    let mut corner = corner_task(
        vec![CornerProcess::SS, CornerProcess::FF],
        vec![0.9, 1.0, 1.1],
        vec![-40.0, 125.0],
        false,
    );
    corner
        .spec_options
        .corner
        .as_mut()
        .expect("corner config")
        .model_bindings = vec![
        corner_binding(CornerProcess::SS, "ss.lib", "1e-13"),
        corner_binding(CornerProcess::FF, "ff.lib", "1e-11"),
    ];
    unequal.tasks = vec![prepared("corner", "Corner", corner)];

    let error = PreparedRunSnapshot::new(unequal)
        .expect_err("2 processes, 3 voltages and 2 temperatures have no diagonal pairing");
    assert!(
        format!("{error}").contains("equal non-scalar axis lengths"),
        "{error}"
    );

    let mut paired = parts();
    let mut corner = corner_task(
        vec![CornerProcess::SS, CornerProcess::FF],
        vec![1.0],
        vec![-40.0, 125.0],
        false,
    );
    corner
        .spec_options
        .corner
        .as_mut()
        .expect("corner config")
        .model_bindings = vec![
        corner_binding(CornerProcess::SS, "ss.lib", "1e-13"),
        corner_binding(CornerProcess::FF, "ff.lib", "1e-11"),
    ];
    paired.tasks = vec![prepared("corner", "Corner", corner)];

    let snapshot = PreparedRunSnapshot::new(paired).expect("diagonal corner snapshot");
    assert_eq!(snapshot.pvt_points.len(), 2);
    assert_eq!(snapshot.pvt_points[0].process, ProcessCorner::SS);
    assert_eq!(snapshot.pvt_points[0].voltage, Some(1.0));
    assert_eq!(snapshot.pvt_points[0].temperature_celsius, -40.0);
    assert_eq!(snapshot.pvt_points[1].process, ProcessCorner::FF);
    assert_eq!(snapshot.pvt_points[1].voltage, Some(1.0));
    assert_eq!(snapshot.pvt_points[1].temperature_celsius, 125.0);
}

#[test]
fn pvt_point_set_is_ordered_and_deduplicated_across_tasks() {
    let mut swept = parts();
    swept.tasks = vec![
        prepared("op", "DC Operating Point", task()),
        prepared(
            "temperature",
            "Temperature",
            temperature_task(vec![27.0, 85.0, 85.0]),
        ),
    ];

    let snapshot = PreparedRunSnapshot::new(swept).expect("temperature snapshot");
    assert_eq!(snapshot.pvt_points.len(), 2);
    assert_eq!(snapshot.pvt_points[0].temperature_celsius, 27.0);
    assert_eq!(snapshot.pvt_points[1].temperature_celsius, 85.0);
    assert_eq!(snapshot.tasks.len(), 3);
    assert_eq!(snapshot.metadata().pvt_point_count, 2);
}

#[test]
fn pvt_operating_point_dispatches_three_exact_temperatures_and_retains_only_final_report() {
    use crate::simulation::dialog::{OpConfig, OpSaveDevice};

    let mut swept = parts();
    swept.executable_netlist =
        "diode\nV1 in 0 0.7\nD1 in 0 DTEST\n.model DTEST D\n.op\n.end\n".to_owned();
    swept.tasks = vec![
        prepared(
            "op",
            "DC Operating Point",
            configured_op_task(OpConfig {
                save_device_op: OpSaveDevice::FinalPointOnly,
                ..OpConfig::default()
            }),
        ),
        prepared(
            "temperature",
            "Temperature",
            temperature_task(vec![-40.0, 27.0, 85.0]),
        ),
    ];

    let snapshot = PreparedRunSnapshot::new(swept).expect("three-point OP snapshot");
    let op_configs = snapshot
        .tasks
        .iter()
        .filter_map(|task| match task.queued_analysis().config.as_ref() {
            Some(AnalysisConfig::DcOp(config)) => Some(config.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(op_configs.len(), 3);
    assert_eq!(
        op_configs
            .iter()
            .map(|config| config.temperature_celsius)
            .collect::<Vec<_>>(),
        vec![-40.0, 27.0, 85.0]
    );

    for (index, config) in op_configs.into_iter().enumerate() {
        assert_eq!(config.run_point.index, index);
        assert_eq!(config.run_point.count, 3);
        let result = crate::simulation::EngineBridge::new()
            .run(
                &AnalysisConfig::DcOp(config),
                "diode\nV1 in 0 0.7\nD1 in 0 DTEST\n.model DTEST D\n.op\n.end\n",
            )
            .expect("exact PVT OP point solves");
        let crate::simulation::SimulationResult::DcOp(result) = result else {
            panic!("OP result")
        };
        assert_eq!(result.device_report.is_some(), index == 2);
    }
}

#[test]
fn op_plus_ss_corner_has_no_unrequested_reference_point() {
    use crate::services::simulation_runner::CornerProcess;

    let mut ss_corner = corner_task(vec![CornerProcess::SS], vec![0.9], vec![125.0], true);
    ss_corner
        .spec_options
        .corner
        .as_mut()
        .expect("corner config")
        .model_bindings = vec![corner_binding(CornerProcess::SS, "ss.lib", "1e-13")];
    let mut ss_only = parts();
    ss_only.tasks.push(prepared("ss", "SS Corner", ss_corner));

    let snapshot = PreparedRunSnapshot::new(ss_only).expect("SS-only PVT snapshot");
    assert_eq!(snapshot.pvt_points.len(), 1);
    assert_eq!(snapshot.pvt_points[0].process, ProcessCorner::SS);
    assert_eq!(snapshot.pvt_points[0].temperature_celsius, 125.0);
    let config = snapshot.tasks[0]
        .queued_analysis()
        .config
        .as_ref()
        .and_then(|config| match config {
            AnalysisConfig::DcOp(config) => Some(config),
            _ => None,
        })
        .expect("expanded OP config");
    assert_eq!(config.run_point.process, ProcessCorner::SS);
    assert_eq!(config.run_point.count, 1);
}

#[test]
fn process_and_voltage_axes_change_the_authorized_op_execution_contract() {
    use crate::services::simulation_runner::CornerProcess;

    let mut corner = corner_task(
        vec![CornerProcess::TT, CornerProcess::SS],
        vec![1.0, 1.2],
        vec![27.0],
        true,
    );
    corner
        .spec_options
        .corner
        .as_mut()
        .expect("corner config")
        .model_bindings = vec![
        corner_binding(CornerProcess::TT, "tt.lib", "1e-12"),
        corner_binding(CornerProcess::SS, "ss.lib", "1e-13"),
    ];
    let mut pvt = parts();
    pvt.executable_netlist = "pvt\nVDD in 0 1\nR1 in 0 1k\n.op\n.end\n".to_owned();
    pvt.tasks.push(prepared("corner", "Corner", corner));
    let snapshot = PreparedRunSnapshot::new(pvt).expect("PVT snapshot");
    assert_eq!(snapshot.pvt_points.len(), 4);
    assert_eq!(
        snapshot
            .tasks
            .iter()
            .filter(|task| { matches!(task.task.config.as_ref(), Some(AnalysisConfig::DcOp(_))) })
            .count(),
        4
    );

    let digest = snapshot.digest();
    let permit = crate::simulation::execution::ExecutionPermitIssuer::default()
        .issue(digest)
        .expect("permit");
    let proof = permit.consume(digest, digest).expect("consume permit");
    let dispatch = snapshot.authorize_dispatch(proof).expect("authorize PVT");
    let mut seen_contracts = HashSet::new();
    let mut seen_process_sources = HashMap::new();
    let mut seen_supplies = HashSet::new();
    for task in dispatch.into_tasks().into_iter().take(4) {
        assert_eq!(task.authored_instance_id(), instance_id("op"));
        let config_digest = task.config_digest();
        let resolved = task
            .resolve_dependency_artifacts(&HashMap::new())
            .expect("OP has no typed dependencies");
        let (queued, source, _, _) = resolved.into_runner_parts();
        let Some(AnalysisConfig::DcOp(config)) = queued.config else {
            panic!("OP config")
        };
        seen_contracts.insert(config_digest);
        seen_supplies.insert(config.run_point.supply_voltage.unwrap().to_bits());
        seen_process_sources
            .entry(config.run_point.process)
            .or_insert_with(|| source.to_string());
        let result = crate::simulation::EngineBridge::new()
            .run(&AnalysisConfig::DcOp(config.clone()), &source)
            .expect("corner OP solve");
        let crate::simulation::SimulationResult::DcOp(result) = result else {
            panic!("OP result")
        };
        assert!(
            (result.voltage("in").expect("supply node") - config.run_point.supply_voltage.unwrap())
                .abs()
                <= 1.0e-10
        );
    }
    assert_eq!(seen_contracts.len(), 4);
    assert_eq!(seen_supplies.len(), 2);
    assert!(seen_process_sources[&ProcessCorner::TT].contains("tt.lib"));
    assert!(seen_process_sources[&ProcessCorner::SS].contains("ss.lib"));
    assert_ne!(
        seen_process_sources[&ProcessCorner::TT],
        seen_process_sources[&ProcessCorner::SS]
    );
}

#[test]
fn downstream_ordering_dependency_targets_the_final_expanded_op_point() {
    let mut graph = parts();
    let original_op = instance_id("op");
    graph.tasks = vec![
        prepared("op", "DC Operating Point", task()),
        prepared(
            "temperature",
            "Temperature",
            temperature_task(vec![-40.0, 85.0]),
        ),
        prepared_with(
            "tran",
            ObjectRevision::INITIAL,
            vec![original_op],
            "Transient",
            transient_task(),
        ),
    ];

    let snapshot = PreparedRunSnapshot::new(graph).expect("expanded dependency graph");
    let op_tasks = snapshot
        .tasks
        .iter()
        .filter(|task| matches!(task.task.config.as_ref(), Some(AnalysisConfig::DcOp(_))))
        .collect::<Vec<_>>();
    assert_eq!(op_tasks.len(), 2);
    assert_eq!(op_tasks[1].dependencies(), &[op_tasks[0].instance_id()]);
    let transient = snapshot
        .tasks
        .iter()
        .find(|task| matches!(task.task.spec, AnalysisSpec::Transient { .. }))
        .expect("transient task");
    assert_eq!(transient.dependencies(), &[op_tasks[1].instance_id()]);
}

#[test]
fn identical_coordinates_with_different_model_contracts_do_not_deduplicate() {
    use crate::services::simulation_runner::CornerProcess;

    let corner_with = |label: &str, saturation_current: &str| {
        let mut corner = corner_task(vec![CornerProcess::TT], vec![1.0], vec![27.0], true);
        corner
            .spec_options
            .corner
            .as_mut()
            .expect("corner config")
            .model_bindings = vec![corner_binding(CornerProcess::TT, label, saturation_current)];
        corner
    };
    let mut ambiguous_coordinates = parts();
    ambiguous_coordinates.tasks.push(prepared(
        "corner-a",
        "Corner A",
        corner_with("a.lib", "1e-12"),
    ));
    ambiguous_coordinates.tasks.push(prepared(
        "corner-b",
        "Corner B",
        corner_with("b.lib", "2e-12"),
    ));

    let snapshot = PreparedRunSnapshot::new(ambiguous_coordinates)
        .expect("distinct source contracts remain distinct points");
    assert_eq!(snapshot.pvt_points.len(), 2);
    let op_sources = snapshot
        .tasks
        .iter()
        .filter_map(|task| {
            matches!(task.task.config.as_ref(), Some(AnalysisConfig::DcOp(_)))
                .then(|| task.executable_netlist_override.as_deref())
                .flatten()
        })
        .collect::<Vec<_>>();
    assert_eq!(op_sources.len(), 2);
    assert_ne!(op_sources[0], op_sources[1]);
}

#[test]
fn pvt_change_changes_snapshot_identity_without_revision_change() {
    let first = PreparedRunSnapshot::new(parts()).expect("reference snapshot");
    let mut changed = parts();
    changed.reference_temperature_celsius = 125.0;
    let changed = PreparedRunSnapshot::new(changed).expect("changed PVT snapshot");
    assert_ne!(first.digest(), changed.digest());
}

#[test]
fn authorized_tasks_own_the_exact_snapshot_netlist_after_permit_consumption() {
    let snapshot = PreparedRunSnapshot::new(parts()).expect("prepared snapshot");
    let digest = snapshot.digest();
    let issuer = crate::simulation::execution::ExecutionPermitIssuer::default();
    let permit = issuer.issue(digest).expect("issue permit");
    let proof = permit
        .consume(digest, digest)
        .expect("consume exact permit");
    let dispatch = snapshot
        .authorize_dispatch(proof)
        .expect("authorize exact snapshot");

    assert_eq!(dispatch.executable_netlist(), "deck\n.op\n.end\n");
    assert_eq!(dispatch.task_count(), 1);
    let mut tasks = dispatch.into_tasks();
    let authorized = tasks.pop_front().expect("authorized task");
    assert_eq!(authorized.snapshot_digest(), digest);
    assert_eq!(authorized.instance_id(), instance_id("op"));
    assert_eq!(authorized.source_revision(), ObjectRevision::INITIAL);
    assert!(authorized.dependencies().is_empty());
    assert_eq!(authorized.label(), "DC Operating Point");
    assert_eq!(authorized.config_digest(), parts().tasks[0].config_digest());
    let resolved = authorized
        .resolve_dependency_artifacts(&HashMap::new())
        .expect("artifact-free task resolves");
    let (_, netlist, runtimes, dependencies) = resolved.into_runner_parts();
    assert_eq!(&*netlist, "deck\n.op\n.end\n");
    assert!(runtimes.is_empty());
    dependencies
        .validate_for_config()
        .expect("OP has no typed dependencies");
    assert!(tasks.is_empty());
}

#[test]
fn authorization_preserves_exact_task_graph_and_source_revision() {
    let revision = ObjectRevision::new(7).expect("revision seven");
    let mut frozen = parts();
    frozen.tasks[0] = prepared_with("op", revision, Vec::new(), "DC Operating Point", task());
    frozen.tasks.push(prepared_with(
        "tran",
        revision,
        vec![instance_id("op")],
        "Transient",
        transient_task(),
    ));
    let snapshot = PreparedRunSnapshot::new(frozen).expect("prepared graph");
    let digest = snapshot.digest();
    let issuer = crate::simulation::execution::ExecutionPermitIssuer::default();
    let proof = issuer
        .issue(digest)
        .expect("issue permit")
        .consume(digest, digest)
        .expect("consume permit");
    let dispatch = snapshot.authorize_dispatch(proof).expect("authorize graph");
    let tasks = dispatch.tasks().collect::<Vec<_>>();

    assert_eq!(tasks[0].instance_id(), instance_id("op"));
    assert!(tasks[0].dependencies().is_empty());
    assert_eq!(tasks[1].instance_id(), instance_id("tran"));
    assert_eq!(tasks[1].dependencies(), &[instance_id("op")]);
    assert_eq!(tasks[1].source_revision(), revision);
    assert_eq!(tasks[1].snapshot_digest(), digest);
}

#[test]
fn snapshot_rejects_digest_that_does_not_match_actual_task() {
    let mut mismatched = parts();
    mismatched.tasks[0].config_digest = ContentDigest::from_bytes([0xff; 32]);
    let error = PreparedRunSnapshot::new(mismatched)
        .expect_err("digest cannot authenticate a different task");
    assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
    assert!(error.message().contains("actual dispatch payload"));
}

#[test]
fn pvt_plan_capacity_check_handles_overflow_and_limit() {
    let error = ensure_pvt_point_capacity(9, 2, 10).expect_err("eleven runs exceed ten");
    assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
    assert!(error.message().contains("11 runs"));

    let overflow = ensure_pvt_point_capacity(usize::MAX, 1, usize::MAX - 1)
        .expect_err("overflow must fail closed");
    assert!(overflow.message().contains(&usize::MAX.to_string()));
}

/// A per-analysis solver override only exists if the bytes the engine parses
/// carry it. This walks the whole seam: the record reaches the task's own deck
/// as a second options card, that card wins over the one already in the deck,
/// and the two tasks do not share a configuration identity.
#[test]
fn an_authored_iteration_budget_reaches_the_task_deck_and_changes_its_identity() {
    use crate::simulation::plan::{AnalysisKind, AnalysisNumericOverride, NumericOverrideOption};

    const DECK: &str = "override deck\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         .options ITL1=40\n\
         .tran 1n 1u\n\
         .end\n";

    let deck_parts = |task| {
        let mut parts = parts();
        parts.executable_netlist = DECK.to_owned();
        parts.tasks = vec![prepared("tran", "Transient", task)];
        parts
    };

    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Transient, NumericOverrideOption::Itl1, "321")
        .expect("a transient may carry its own Newton budget");
    let mut authored_task = transient_task();
    authored_task.numeric_override = Some(record);

    let inheriting = PreparedRunSnapshot::new(deck_parts(transient_task()))
        .expect("an inheriting transient prepares");
    let authored = PreparedRunSnapshot::new(deck_parts(authored_task))
        .expect("an authored transient prepares");

    assert!(
        inheriting.tasks[0].executable_netlist_override.is_none(),
        "an analysis that states nothing must leave its deck alone"
    );
    assert_ne!(
        inheriting.tasks[0].config_digest, authored.tasks[0].config_digest,
        "two tasks that run different numerics cannot share one configuration identity"
    );

    let deck = authored.tasks[0]
        .executable_netlist_override
        .as_deref()
        .expect("the authored budget rewrote this task's deck");
    assert_eq!(
        deck.matches(".OPTIONS").count() + deck.matches(".options").count(),
        2,
        "the authored card must join the deck's own, not replace it:\n{deck}"
    );
    assert!(
        deck.find("ITL1=321")
            .is_some_and(|authored| deck.find(".end").is_some_and(|end| authored < end)),
        "a card after the terminal .end is never read:\n{deck}"
    );

    let parsed = rspice_core::netlist::parse_netlist(deck).expect("the spliced deck still parses");
    assert_eq!(
        parsed.options.itl1,
        Some(321),
        "the later options card must win per key, or the override resolves to the deck's value"
    );
}

/// Attribution is what lets a specification be a claim about one corner, so it
/// has to be exact in both directions: an expanded point names the point it
/// solved, and a task that runs once for the whole space names nothing. A
/// fabricated nominal on the second kind would let a scoped limit pass on
/// evidence that never belonged to a point.
#[test]
fn expanded_op_points_are_attributed_and_an_unexpanded_task_is_not() {
    let mut swept = parts();
    swept.executable_netlist =
        "diode\nV1 in 0 0.7\nD1 in 0 DTEST\n.model DTEST D\n.op\n.end\n".to_owned();
    swept.tasks = vec![
        prepared("op", "DC Operating Point", task()),
        prepared(
            "temperature",
            "Temperature",
            temperature_task(vec![-40.0, 27.0, 85.0]),
        ),
        prepared("tran", "Transient", transient_task()),
    ];

    let snapshot = PreparedRunSnapshot::new(swept).expect("three-point OP snapshot");
    let attributed: Vec<(String, f64, bool)> = snapshot
        .tasks
        .iter()
        .filter_map(|task| task.pvt_point())
        .map(|point| {
            (
                point.process().to_owned(),
                point.temperature_celsius(),
                point.is_nominal(),
            )
        })
        .collect();

    assert_eq!(
        attributed,
        vec![
            ("TT".to_owned(), -40.0, false),
            ("TT".to_owned(), 27.0, true),
            ("TT".to_owned(), 85.0, false),
        ],
        "each expanded point names its own process and temperature, and only the \
         reference temperature is the run's nominal point"
    );
    assert_eq!(
        snapshot
            .tasks
            .iter()
            .filter(|task| task.pvt_point().is_none())
            .count(),
        2,
        "the temperature axis task and the transient run once for the whole space"
    );
    for point in snapshot.tasks.iter().filter_map(PreparedTask::pvt_point) {
        assert_eq!(
            point.supply_voltage(),
            None,
            "a temperature-only axis leaves the deck's own supply standing"
        );
        assert_eq!(
            point.corner_contract(),
            None,
            "a temperature-only axis has no corner contract to name"
        );
    }
}

/// A supply axis makes exactly one point nominal — the reference process at
/// the reference temperature and the contract's nominal supply — so a limit
/// scoped to nominal cannot be answered by a derated point.
#[test]
fn a_corner_axis_marks_exactly_one_point_nominal_and_names_its_contract() {
    use crate::services::simulation_runner::CornerProcess;

    let mut corner = corner_task(
        vec![CornerProcess::TT, CornerProcess::SS],
        vec![1.0, 1.2],
        vec![27.0],
        true,
    );
    corner
        .spec_options
        .corner
        .as_mut()
        .expect("corner config")
        .model_bindings = vec![
        corner_binding(CornerProcess::TT, "tt.lib", "1e-12"),
        corner_binding(CornerProcess::SS, "ss.lib", "1e-13"),
    ];
    let mut pvt = parts();
    pvt.executable_netlist = "pvt\nVDD in 0 1\nR1 in 0 1k\n.op\n.end\n".to_owned();
    pvt.tasks.push(prepared("corner", "Corner", corner));

    let snapshot = PreparedRunSnapshot::new(pvt).expect("PVT snapshot");
    let points: Vec<_> = snapshot
        .tasks
        .iter()
        .filter_map(PreparedTask::pvt_point)
        .collect();

    assert_eq!(points.len(), 4);
    let nominal: Vec<_> = points.iter().filter(|point| point.is_nominal()).collect();
    assert_eq!(nominal.len(), 1);
    assert_eq!(nominal[0].process(), "TT");
    assert_eq!(nominal[0].supply_voltage(), Some(1.0));
    assert_eq!(nominal[0].temperature_celsius(), 27.0);
    assert!(
        points.iter().all(|point| point.corner_contract().is_some()),
        "a corner axis binds process models through a contract, and the point names it"
    );
    assert_eq!(
        points
            .iter()
            .filter(|point| point.process() == "SS")
            .count(),
        2
    );
}
