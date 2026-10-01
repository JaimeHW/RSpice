//! Tests for the prepare-then-permit run path.
//!
//! A dispatched run must execute exactly the sources it was prepared against,
//! so these cases mutate files after prepare and assert the run fails closed
//! rather than silently reopening the changed source.

use super::*;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::workbench::{
    documents::code_workspace::compile_project_bundle_receipt, examples::load_example,
    lifecycle::project_lifecycle, workflows::netlist_workflow::apply_imported_netlist,
};
use rspice_design::drc::DrcResult;
static FIXTURE_NONCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn standalone_connection_directive_is_an_authenticated_prepared_dependency() {
    let (sources, deck) =
        crate::simulation::veriloga_tests::test_support::standalone_connection_fixture();
    reject_deferred_external_sources_with_project_runtimes(&deck, &sources, &Default::default())
        .unwrap();
    let altered = deck.replace(" UI_CONNECTIONS", " OTHER_CONNECTIONS");
    assert!(
        reject_deferred_external_sources_with_project_runtimes(
            &altered,
            &sources,
            &Default::default()
        )
        .is_err()
    );
}

#[test]
fn exhausted_run_sequence_blocks_dispatch_without_starting_a_batch() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source =
        Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    state.simulation.next_run_id = u64::MAX;
    let baseline = crate::io::capture_simulation_results(&state.simulation);
    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare manual run");
    controller
        .run_authorization
        .retain(snapshot)
        .expect("authorize run");

    controller.start_authorized_snapshot(&mut state);

    assert_eq!(state.simulation.status, "Run blocked");
    assert!(!controller.has_active_batch());
    assert!(controller.runner.can_accept_prepared_task());
    assert!(controller.cached_netlist.is_none());
    assert!(state.simulation.active_execution.is_none());
    assert!(state.ui.netlist.pending_manual_run_id.is_none());
    assert_eq!(state.simulation.next_run_id, u64::MAX);
    assert_eq!(
        crate::io::capture_simulation_results(&state.simulation),
        baseline
    );
    assert!(
        state
            .log_buffer
            .entries()
            .any(|message| { message.message.contains("run sequence is exhausted") })
    );
}

#[test]
fn spectre_model_library_ahdl_is_compiled_and_emitted_as_a_sealed_runtime_directive() {
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    let library_name = manager
        .load_library_bundle(
            "spectre-runtime.scs",
            vec![
                (
                    "models.scs".to_owned(),
                    b"simulator lang=spectre\nahdl_include \"va/device.va\"\nmodel native_d diode is=2e-14\n"
                        .to_vec(),
                ),
                (
                    "va/device.va".to_owned(),
                    b"`include \"../shared/value.vh\"\nmodule retained_device(p, n); inout p, n; electrical p, n; analog I(p, n) <+ V(p, n) / `DEVICE_R; endmodule\n"
                        .to_vec(),
                ),
                ("shared/value.vh".to_owned(), b"`define DEVICE_R 1k\n".to_vec()),
            ],
            None,
        )
        .expect("Spectre model bundle imports");
    let binding = manager
        .simulation_plan_binding(&library_name)
        .expect("imported library is bindable");
    let sealed = manager
        .seal_execution_sources_for_plan(&[binding])
        .expect("model-library sources seal");

    let runtimes = prepared_model_library_veriloga_runtimes(&sealed)
        .expect("prepared-run compilation succeeds");
    assert_eq!(runtimes.len(), 1);
    let runtime = runtimes.device_runtimes().next().expect("one AHDL runtime");
    let mut executable = "prepared Spectre AHDL\n.end\n".to_owned();
    rspice_simulation::netlist_preparation::append_project_veriloga_directive(
        &mut executable,
        runtime.source_key(),
        runtime.netlist_alias(),
    );

    assert!(executable.contains(".veriloga \"__rspice_model_library__/"));
    assert!(executable.contains(" retained_device"));
    assert!(!executable.contains("va/device.va"));
    let parsed = rspice_core::Netlist::parse(&executable).expect("prepared directive parses");
    assert_eq!(parsed.veriloga_includes.len(), 1);
}

fn fixture_dir(label: &str) -> PathBuf {
    let nonce = FIXTURE_NONCE.fetch_add(1, Ordering::Relaxed);
    let directory = std::env::temp_dir().join(format!(
        "rspice-prepared-run-{label}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).expect("create prepared-run fixture");
    directory
}

/// A runnable design whose project owns no technology. The contract admits
/// this: a technology is required only when the project has one or the plan
/// demands one.
fn technology_free_runnable_state() -> AppState {
    let mut state = AppState::default();
    for dimension in &mut state.sim_setup.run_set.dimensions {
        if dimension.kind == crate::simulation::run_set::RunSetDimensionKind::ProcessSection {
            dimension.enabled = false;
        }
    }
    load_example("Voltage Divider", &mut state.schematic);
    let mut drc = DrcResult::new();
    drc.completed = true;
    state.dialogs.drc_results = Some(drc);
    state.dialogs.drc_checked_version = state.schematic.topology_version();
    state
}

pub(in crate::simulation::controller) fn runnable_state() -> AppState {
    let mut state = technology_free_runnable_state();
    state.provision_test_project_technology_contract();
    state
}

#[test]
fn frozen_hierarchy_rejects_stale_instance_model_library_metadata() {
    use crate::state::model_library::{DeviceModel, ModelConsumerScope, ModelLibrary, ModelType};
    use crate::state::{ComponentType, Point};

    let mut state = technology_free_runnable_state();
    state.model_library_manager.clear();
    for name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(name);
        library.add_model(DeviceModel::new("shared_diode", ModelType::Diode));
        state.model_library_manager.add_library(library);
    }
    state
        .model_library_manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "shared_diode",
            "alpha",
            "preflight authority fixture",
        )
        .expect("resolve the contested global model provider");

    let id = state
        .schematic
        .add_component(ComponentType::Diode, Point::new(160, 80));
    let component = state
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("placed diode");
    component.name = "D_AUTH".to_owned();
    component.value = "shared_diode".to_owned();
    component.params = "model_library=beta".to_owned();

    let projection = state
        .workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .expect("freeze schematic hierarchy");
    let error = validate_projected_model_binding_authority(
        state.model_library_manager.catalog(),
        state.model_library_manager.resolution_records(),
        state.library_manager.catalog(),
        state.workspace.content.project.technology_binding(),
        state.pdk_config.technology_registry.validated_packages(),
        &projection,
    )
    .expect_err("stale per-instance provider metadata must fail preflight");
    assert!(
        error
            .to_string()
            .contains("project-global provider 'alpha'")
    );

    state
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("placed diode")
        .params = "model_library=alpha".to_owned();
    let projection = state
        .workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .expect("freeze rebound schematic hierarchy");
    validate_projected_model_binding_authority(
        state.model_library_manager.catalog(),
        state.model_library_manager.resolution_records(),
        state.library_manager.catalog(),
        state.workspace.content.project.technology_binding(),
        state.pdk_config.technology_registry.validated_packages(),
        &projection,
    )
    .expect("matching provider metadata is executable");
}

#[test]
fn prepared_project_run_materializes_exact_signed_pdk_reference_models() {
    let state = runnable_state();
    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("project run seals signed PDK model sources");
    controller
        .run_authorization
        .retain(snapshot)
        .expect("authorize exact project snapshot");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("freeze project dispatch");
    let executable = dispatch.executable_netlist();
    assert!(executable.contains(".model nmos_demo nmos level=1 vto=0.55"));
    assert!(!executable.contains("vto=0.60"));
    assert!(!executable.to_ascii_lowercase().contains(".lib tt"));
    assert!(executable.contains("demo180 2.3.1"));
}

#[test]
fn receipt_backed_manual_project_deck_uses_signed_pdk_models_without_host_paths() {
    let mut state = AppState::default();
    state.provision_test_project_technology_contract();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source =
        Some("signed project deck\nV1 d 0 1\nM1 d d 0 0 nmos_demo\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("governed manual project deck seals signed PDK sources");
    controller
        .run_authorization
        .retain(snapshot)
        .expect("authorize exact governed manual deck");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("freeze governed manual dispatch");
    let executable = dispatch.executable_netlist();
    assert!(executable.contains(".model nmos_demo nmos level=1 vto=0.55"));
    assert!(!executable.contains("vto=0.60"));
    assert!(!contains_external_include_directive(executable));
    assert!(!executable.contains("/rspice-pdk/"));
}

#[test]
fn governed_manual_deck_dispatches_signed_pdk_veriloga_runtime_without_host_paths() {
    let mut state = AppState::default();
    state.provision_test_project_veriloga_technology_contract();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source =
        Some("signed Verilog-A project deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("governed manual deck prepares the signed Verilog-A runtime");
    controller
        .run_authorization
        .retain(snapshot)
        .expect("authorize exact signed Verilog-A snapshot");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("freeze signed Verilog-A dispatch");
    let executable = dispatch.executable_netlist();
    assert!(executable.contains(".veriloga \"__rspice_pdk__/"));
    assert!(executable.contains(" pdk_resistor_model"));
    assert!(!executable.contains("veriloga/pdk_resistor.va"));
    assert!(!contains_external_include_directive(executable));
}

fn edit_frozen_transient_stop(state: &mut AppState, stop: &str) {
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("test state owns a stable plan");
    let transient = plan
        .instances()
        .iter()
        .find(|instance| {
            instance.kind() == crate::simulation::plan::AnalysisKind::Transient
                && instance.enabled()
        })
        .expect("enabled transient instance")
        .id();
    plan.edit(transient, |draft| {
        let crate::simulation::plan::AnalysisDraft::Transient(draft) = draft else {
            panic!("expected transient draft");
        };
        draft.stop = stop.to_owned();
    })
    .expect("transient edit commits");
}

#[test]
fn prepared_snapshot_detects_analysis_mutation_without_revision_change() {
    let mut state = runnable_state();
    let prepared =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare first snapshot");
    let revision = state.workspace.content.project.revision().get();
    edit_frozen_transient_stop(&mut state, "2m");
    let changed =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare changed snapshot");
    assert_eq!(state.workspace.content.project.revision().get(), revision);
    assert_ne!(prepared.digest(), changed.digest());
}

#[test]
fn prepared_snapshot_detects_non_topology_source_mutation() {
    let mut state = runnable_state();
    let prepared =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare first snapshot");
    let topology = state.schematic.topology_version();
    state.schematic.document_mut_for_test().components[0].value = "2k".to_owned();
    let changed =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare changed snapshot");
    assert_eq!(state.schematic.topology_version(), topology);
    assert_ne!(prepared.digest(), changed.digest());
}

#[test]
fn prepared_snapshot_authenticates_plan_owned_saved_outputs() {
    let mut state = runnable_state();
    state.sim_setup.save_policy.output_selection_mode =
        crate::state::OutputSelectionMode::ExplicitOnly;
    let without_output =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare baseline snapshot");
    assert_eq!(without_output.metadata().saved_output_contract_count, 0);

    let plan_id = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let output = crate::state::SavedOutput::new(
        crate::state::SavedOutputKind::RawVoltageOrCurrent,
        "output_voltage",
        "V(1)",
        crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
        crate::state::SavedOutputPolicy::SelectedAndFinalPoints,
        crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
        crate::state::SavedOutputStreaming::StoreOnly,
    )
    .expect("valid output");
    state
        .workspace
        .content
        .add_saved_output(plan_id, output)
        .expect("plan owns output");

    let with_output =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare output snapshot");
    assert_eq!(with_output.metadata().saved_output_contract_count, 1);
    assert_ne!(without_output.digest(), with_output.digest());
}

#[test]
fn automatic_touchstone_export_policy_captures_live_dialog_and_path_once() {
    let mut state = AppState::default();
    state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(
        &crate::simulation::dialog::SpConfig::default(),
    );
    state.schematic.session.current_file = Some(PathBuf::from("designs").join("amp.rsch"));
    let tasks = vec![QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::SParameter {
            do_noise: false,
            start_freq: 1.0e6,
            stop_freq: 1.0e9,
            points_per_unit: 20,
            sweep: FrequencySweep::Decade,
            z0: 50.0,
            ports: vec![SpPort {
                node_pos: "in".to_owned(),
                node_neg: "0".to_owned(),
                z0: None,
            }],
        },
        config: None,
        spec_options: SpecExecutionOptions::default(),
        analysis_line: ".sp dec 20 1Meg 1Gig".to_owned(),
    }];

    let prepared = touchstone_export_policy(
        &state,
        &tasks,
        state.schematic.session.current_file.as_deref(),
    )
    .expect("capture enabled policy");
    let prepared_path = prepared.output_path(7, 1, 2).expect("enabled export path");

    state.schematic.session.current_file = Some(PathBuf::from("redirect").join("changed.rsch"));
    let mut disabled = crate::simulation::dialog::SpConfig::default();
    disabled.touchstone_export = false;
    state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(&disabled);
    let current = touchstone_export_policy(
        &state,
        &tasks,
        state.schematic.session.current_file.as_deref(),
    )
    .expect("capture disabled policy");

    assert!(current.output_path(7, 1, 2).is_none());
    assert!(prepared_path.ends_with("designs/amp_run0007_sp01.s2p"));
}

#[test]
fn manual_touchstone_export_uses_imported_deck_origin_not_stale_schematic_path() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.schematic.session.current_file = Some(PathBuf::from("stale").join("schematic.rsch"));
    state.workspace.content.netlist_source_path =
        Some(PathBuf::from("imported").join("rf_fixture.cir"));
    state.workspace.content.netlist_source = Some(
            "deck\nV2 out 0 dc 0 ac 1 portnum 2 z0 75\nV1 in 0 dc 0 ac 1 portnum 1 z0 50\nR1 in out 100\n.sp lin 3 1Meg 3Meg\n.end\n"
                .to_owned(),
        );
    state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(
        &crate::simulation::dialog::SpConfig::default(),
    );

    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare imported RF deck");
    controller
        .run_authorization
        .retain(snapshot)
        .expect("authorize imported RF deck");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("dispatch imported RF deck");
    let policy = dispatch
        .tasks()
        .next()
        .expect("manual S-parameter task")
        .touchstone_export_policy();

    let path = policy.output_path(4, 1, 2).expect("export is enabled");
    assert!(path.ends_with("imported/rf_fixture_run0004_sp01.s2p"));
    assert!(!path.to_string_lossy().contains("schematic"));
}

#[test]
fn manual_fourier_is_topologically_bound_to_its_exact_transient_task() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(
        "Fourier deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k V(out)\n.tran 10u 5m\n.end\n"
            .to_owned(),
    );

    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare manual Fourier dependency graph");
    controller
        .run_authorization
        .retain(snapshot)
        .expect("authorize manual Fourier dependency graph");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("dispatch manual Fourier dependency graph");
    let tasks = dispatch.tasks().collect::<Vec<_>>();

    assert_eq!(tasks.len(), 2);
    assert!(matches!(tasks[0].spec(), AnalysisSpec::Transient { .. }));
    assert!(matches!(tasks[1].spec(), AnalysisSpec::Fourier { .. }));
    assert_eq!(tasks[1].dependencies(), &[tasks[0].instance_id()]);
}

#[test]
fn campaign_freezes_distinct_plan_members_without_switching_the_live_editor() {
    let mut state = runnable_state();
    let first_plan_id = state.sim_setup.stable_analysis_plan().unwrap().id();
    state
        .workspace
        .content
        .migrate_active_plan_data(first_plan_id);
    let second_plan_id = state
        .sim_setup
        .create_plan("Second campaign plan")
        .expect("create campaign plan");
    state
        .workspace
        .content
        .migrate_inactive_plan_data(second_plan_id);
    state
        .workspace
        .content
        .sync_legacy_specs_projection(second_plan_id);
    let live_plan_before_dispatch = state.sim_setup.stable_analysis_plan().unwrap().id();
    let mut controller = SimulationController::new();

    let receipt = controller
        .prepare_and_start_campaign(
            &mut state,
            "Nightly characterization",
            &[first_plan_id, second_plan_id],
        )
        .expect("freeze and dispatch campaign");

    assert_eq!(receipt.member_count, 2);
    assert_eq!(
        state.sim_setup.stable_analysis_plan().unwrap().id(),
        live_plan_before_dispatch
    );
    let run = state
        .simulation
        .active_run()
        .expect("first member dispatched");
    assert_eq!(
        run.prepared_receipt().unwrap().simulation_plan_id(),
        Some(first_plan_id)
    );
    let membership = run
        .campaign_membership()
        .expect("campaign identity retained");
    assert_eq!(membership.campaign_id(), receipt.campaign_id);
    assert_eq!(membership.member_index(), 1);
    assert_eq!(membership.member_count(), 2);
    assert_eq!(
        controller
            .active_campaign
            .as_ref()
            .expect("campaign remains active")
            .pending
            .len(),
        1
    );

    let project = project_lifecycle::snapshot(&state).expect("campaign member project snapshot");
    let json = crate::io::project_io::serialize_project_file(&project)
        .expect("campaign member serializes");
    let loaded =
        crate::io::project_io::load_project_text(&json, None).expect("campaign member reloads");
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("campaign result history restores");
    let restored_membership = restored.runs[0]
        .campaign_membership()
        .expect("campaign identity survives project round trip");
    assert_eq!(restored_membership.campaign_id(), receipt.campaign_id);
    assert_eq!(restored_membership.member_index(), 1);

    controller.abort();
}

#[test]
fn prepared_snapshot_authenticates_and_enforces_plan_owned_save_policy() {
    let mut state = runnable_state();
    let baseline =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("baseline save policy prepares");

    state.sim_setup.save_policy.live_streaming_enabled = false;
    state.sim_setup.save_policy.retain_failure_diagnostics = false;
    state.sim_setup.save_policy.retained_dataset_limit = 5;
    let changed =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("changed save policy prepares");
    assert_ne!(baseline.digest(), changed.digest());
    assert_eq!(
        changed.metadata().save_policy,
        "Plan-owned save and streaming policy"
    );

    let plan_id = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    state
        .workspace
        .content
        .add_saved_output(
            plan_id,
            crate::state::SavedOutput::new(
                crate::state::SavedOutputKind::RawVoltageOrCurrent,
                "output_voltage",
                "V(1)",
                crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
                crate::state::SavedOutputPolicy::SelectedAndFinalPoints,
                crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
                crate::state::SavedOutputStreaming::StoreOnly,
            )
            .expect("valid output"),
        )
        .expect("plan owns output");
    state.sim_setup.save_policy.maximum_storage_bytes = 1;
    let error =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect_err("one-byte plan budget must refuse retained output");
    assert!(error.message().contains("storage budget"), "{error}");
}

#[test]
fn deferred_outputs_share_one_sealed_engine_source_budget_per_analysis() {
    let mut state = runnable_state();
    let plan_id = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    for name in ["deferred_voltage", "deferred_voltage_copy"] {
        state
            .workspace
            .content
            .add_saved_output(
                plan_id,
                crate::state::SavedOutput::new(
                    crate::state::SavedOutputKind::RawVoltageOrCurrent,
                    name,
                    "V(1)",
                    crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
                    crate::state::SavedOutputPolicy::OnDemandFromRetainedState,
                    crate::state::SavedOutputPrecision::FullSourcePrecision,
                    crate::state::SavedOutputStreaming::StoreOnly,
                )
                .expect("valid deferred output"),
            )
            .expect("plan owns deferred output");
    }
    let one_source =
        rspice_simulation::output_contract::retained_engine_source_upper_bound_bytes(1);
    state.sim_setup.save_policy.maximum_storage_bytes = one_source;

    SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect("two deferred outputs share the same prepared analysis source state");

    state.sim_setup.save_policy.maximum_storage_bytes = one_source - 1;
    let error =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect_err("the shared source-state ceiling is still enforced");
    assert!(error.message().contains("storage budget"), "{error}");
}

#[test]
fn manual_periodic_analyses_are_topologically_bound_to_seed_and_pss() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(
        "Periodic deck\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\nLPROBE out sensed 1n\nR2 sensed 0 1k\n.pss fund=1Meg points=128 harms=8\n.pac dec 20 1k 100Meg input=V1 out=out\n.pnoise dec 10 1 1Meg out=out\n.pxf dec 10 1k 10Meg input=V1 out=out outsideband=1\n.pstb probe=LPROBE maxharm=8 nmults=6\n.end\n"
            .to_owned(),
    );

    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare manual periodic dependency graph");
    controller
        .run_authorization
        .retain(snapshot)
        .expect("authorize manual periodic dependency graph");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("dispatch manual periodic dependency graph");
    let tasks = dispatch.tasks().collect::<Vec<_>>();

    assert_eq!(tasks.len(), 7);
    assert!(matches!(tasks[0].spec(), AnalysisSpec::DcOp { .. }));
    assert!(matches!(tasks[1].spec(), AnalysisSpec::Pss { .. }));
    assert_eq!(tasks[1].dependencies(), &[tasks[0].instance_id()]);
    for consumer in &tasks[2..] {
        assert!(matches!(
            consumer.spec(),
            AnalysisSpec::PssSpectrum { .. }
                | AnalysisSpec::Pac
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pstb
        ));
        assert_eq!(consumer.dependencies(), &[tasks[1].instance_id()]);
    }
}

#[test]
fn dispatch_rejects_mutation_after_explicit_preflight() {
    let mut state = runnable_state();
    let mut controller = SimulationController::new();
    controller
        .prepare_run_set_for_preflight(&state)
        .expect("preflight");
    edit_frozen_transient_stop(&mut state, "3m");
    let error = controller
        .consume_snapshot_for_dispatch(&state)
        .expect_err("changed input must fail closed");
    assert_eq!(error.stage(), PreparationStage::Authorization);
    assert!(error.message().contains("expired"));
}

#[test]
fn governed_snapshot_check_rejects_in_flight_pvt_change() {
    let mut state = runnable_state();
    let mut controller = SimulationController::new();
    let metadata = controller
        .prepare_run_set_for_preflight(&state)
        .expect("preflight");

    controller
        .ensure_run_set_snapshot_current(&state, metadata.snapshot_digest, metadata.source_digest)
        .expect("unchanged contract remains current");

    state
        .sim_setup
        .set_reference_pvt(crate::product::ProcessCorner::TT, 125.0)
        .expect("physical temperature");
    let error = controller
        .ensure_run_set_snapshot_current(&state, metadata.snapshot_digest, metadata.source_digest)
        .expect_err("PVT mutation must invalidate evidence authority");
    assert_eq!(error.stage(), PreparationStage::Authorization);
    assert!(error.message().contains("no longer matches"));
}

#[test]
fn manual_include_without_origin_fails_closed() {
    let mut state = AppState::default();
    state.workspace.content.netlist_source =
        Some("deck\n.include models.lib\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let error =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect_err("unbound include must fail");
    assert_eq!(error.stage(), PreparationStage::SourceChecks);
    assert!(error.message().contains("origin"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn generated_model_section_is_expanded_and_retained_before_dispatch() {
    let directory = fixture_dir("configured-model-section");
    let model = directory.join("device.lib");
    fs::write(
            &model,
            ".lib tt\n.subckt owned in out\nRsrc in out 9k\n.ends owned\n.endl tt\n.lib ff\n.subckt owned in out\nRsrc in out 4k\n.ends owned\n.endl ff\n",
        )
        .expect("write model-section fixture");
    let source = format!(
        "configured deck\n.lib \"{}\" tt\nX1 in out owned\n.end\n",
        model.display()
    );

    let (expanded, dependencies) = expand_generated_dependencies(
        &source,
        Some(&directory.join("generated.cir")),
        &rspice_simulation::netlist_preparation::IncludeSearchChain::default(),
        &rspice_model_library::ModelCatalog::default(),
        &rspice_model_library::ModelResolutionRecords::default(),
    )
    .expect("configured dependency seals");

    assert!(expanded.contains("Rsrc in out 9k"));
    assert!(!expanded.contains("Rsrc in out 4k"));
    reject_deferred_external_sources(&expanded).expect("expanded deck has no deferred source");
    assert_eq!(dependencies.len(), 1);
    assert_eq!(dependencies[0].selected_section(), Some("tt"));
    assert_eq!(
        dependencies[0].source(),
        fs::read_to_string(&model).unwrap()
    );

    fs::remove_dir_all(directory).expect("remove model-section fixture");
}

#[test]
fn prepared_waveforms_distinguish_inline_names_from_file_inputs() {
    let inline = [
        "V1 file 0 PWL(0 0 1 1)\nR1 file 0 1k",
        ".param file=1 simulation=2\nV1 out 0 PWL(0 0 1 {file})\nR1 out 0 1k",
        ".param pwl=1 file=2\nV1 out 0 PWL 0 0 pwl file\nR1 out 0 1k",
        ".subckt source out params: file=1\nV1 out 0 PWL(0 0 1 {file})\n.ends\nX1 out source file=2\nR1 out 0 1k",
    ];
    for body in inline {
        let source = format!("PWL FILE notes\n{body}\n.op\n.end\n");
        let state = manual_deck_state(&source);
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .unwrap_or_else(|error| panic!("{source}: {error}"));
    }
    for body in [
        "V1 out 0 PWL FILE \"unsealed.csv\"",
        "V1 out 0 DC 1 AC 1 PWL(FILE=\"unsealed.csv\") DISTOF1 1",
        ".subckt source out params: dcval=1\nV1 out 0 DC {dcval} PWL\n+ FILE=\"unsealed.csv\"\n.ends\nX1 out source dcval=2",
    ] {
        let source = format!("deck\n{body}\nR1 out 0 1k\n.op\n.end\n");
        let state = manual_deck_state(&source);
        let error =
            SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
                .expect_err("a real file dependency cannot reach dispatch");
        assert_eq!(
            error.stage(),
            PreparationStage::SourceChecks,
            "{source}: {error}"
        );
        assert!(
            error.message().contains("file-backed PWL source"),
            "{source}: {error}"
        );
    }
}

#[test]
fn case_altered_project_veriloga_key_is_rejected_before_dispatch() {
    let mut state = AppState::default();
    state
        .workspace
        .content
        .replace_imported_project_source(
            crate::state::ProjectSourceLanguage::VerilogA,
            "model.va".to_owned(),
            "module owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
                .to_owned(),
        )
        .expect("replace bootstrapped project Verilog-A source");
    let bundle = state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
            crate::state::ProjectSourceLanguage::VerilogA,
        ))
        .expect("installed project source bundle");
    let receipt =
        compile_project_bundle_receipt(state.workspace.content.project.id(), bundle, None)
            .expect("compile project Verilog-A source");
    state.ui.code_workspace.veriloga.receipt = Some(receipt);

    let bundle = state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
            crate::state::ProjectSourceLanguage::VerilogA,
        ))
        .expect("installed project source bundle");
    let source_key = rspice_design::project_sources::project_veriloga_bundle_source_key(
        state.workspace.content.project.id(),
        bundle,
        "owned",
    )
    .expect("derive exact project source key");
    let exact_directive =
        rspice_simulation::netlist_preparation::project_veriloga_directive(&source_key, "owned");
    let exact_runtimes = project_veriloga_runtimes_referenced_by(
        state.workspace.content.project.id(),
        &state.workspace.content.project_sources,
        state
            .ui
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .map(|receipt| &receipt.compilation),
        &exact_directive,
    )
    .expect("inspect exact project directive");
    assert_eq!(exact_runtimes.len(), 1);
    reject_deferred_external_sources_with_project_runtimes(
        &exact_directive,
        &exact_runtimes,
        &Default::default(),
    )
    .expect("exact project identity is permitted");

    let altered_key = source_key.replacen("__rspice_project__", "__RSPICE_PROJECT__", 1);
    let altered_directive =
        rspice_simulation::netlist_preparation::project_veriloga_directive(&altered_key, "owned");
    let altered_runtimes = project_veriloga_runtimes_referenced_by(
        state.workspace.content.project.id(),
        &state.workspace.content.project_sources,
        state
            .ui
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .map(|receipt| &receipt.compilation),
        &altered_directive,
    )
    .expect("inspect altered project directive");
    assert!(
        altered_runtimes.is_empty(),
        "case-altered virtual paths must not acquire the exact project runtime"
    );
    let error = reject_deferred_external_sources_with_project_runtimes(
        &altered_directive,
        &altered_runtimes,
        &Default::default(),
    )
    .expect_err("case-altered project key must remain an external dependency");
    assert_eq!(error.stage(), PreparationStage::SourceChecks);
    assert!(error.message().contains("unsealed external dependency"));
}

#[test]
fn configured_cell_view_compiles_the_exact_sealed_veriloga_bundle() {
    let mut state = AppState::default();
    let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");

    let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "sealed_gain".to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut cell = crate::state::Cell::new("gain");
    cell.add_view(view);
    let mut library = crate::state::Library::new("behavioral");
    library.add_cell(cell);
    state.library_manager.add_library(library);

    let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(reference),
            crate::state::ProjectSourceLanguage::VerilogA,
            "behavioral/gain.va",
            "`include \"behavioral/gain_constants.va\"\nmodule sealed_gain(p, n); inout p, n; electrical p, n; analog I(p,n) <+ `RSPICE_GAIN * V(p,n); endmodule\n",
            [crate::state::ProjectSourceFile::try_new(
                "behavioral/gain_constants.va",
                "`define RSPICE_GAIN 1.0\n",
            )
            .expect("valid included source")],
            [crate::state::ProjectSourceDependency::try_new(
                "behavioral/gain.va",
                "behavioral/gain_constants.va",
            )
            .expect("valid dependency edge")],
        )
        .expect("valid sealed Verilog-A bundle");
    let expected_digest = bundle.closure_digest();
    state
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("attach cell-view source");

    let mut placed = crate::state::LibraryCellInstance::new("behavioral", "gain", "schematic");
    placed.terminal_order = vec!["p".to_owned(), "n".to_owned()];
    state
        .schematic
        .add_library_cell_component(crate::state::Point::new(20, 20), placed);
    state
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Mixed-signal".to_owned(),
            root: crate::state::CellViewRef::default_top(),
            dut_path: "/top/X1".to_owned(),
            executable_view_policy: vec!["veriloga".to_owned()],
            stop_views: vec!["veriloga".to_owned()],
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Mixed-signal design".to_owned(),
        })
        .expect("create executable mixed-signal configuration");

    let projection = state
        .workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .expect("resolve configured behavioral view");
    let runtimes = prepared_configuration_veriloga_runtimes(
        state.workspace.content.project.id(),
        &state.workspace.content.project_sources,
        &projection,
    )
    .expect("compile exact configured source closure");

    let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
        state.library_manager.catalog(),
        &projection,
    );
    let generated = rspice_simulation::netlist_gen::generate_netlist_hierarchical(
        projection.root_schematic().expect("materialized root"),
        &[],
        &hierarchy,
        &rspice_simulation::netlist_gen::NetlistSourceData::new(
            &crate::simulation::table_route::SourceFiles,
        ),
    );
    assert!(generated.errors.is_empty(), "{:?}", generated.errors);

    assert_eq!(runtimes.len(), 1);
    let runtime = runtimes.device_runtimes().next().expect("prepared runtime");
    assert_eq!(runtime.source_digest(), expected_digest);
    assert_eq!(runtime.module_name(), "sealed_gain");
    assert_eq!(runtime.terminal_names().unwrap(), ["p", "n"]);
    assert!(runtime.source_key().starts_with("__rspice_project__/"));
    assert_eq!(
        generated
            .netlist
            .lines()
            .filter(|line| line.trim().eq_ignore_ascii_case(
                &rspice_simulation::netlist_preparation::project_veriloga_directive(
                    runtime.source_key(),
                    runtime.netlist_alias(),
                )
            ))
            .count(),
        1,
        "configured netlist must reference the exact prepared runtime once"
    );
    rspice_simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime.clone()])
        .unwrap()
        .install()
        .expect("sealed configured runtime installs in the session cache");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn statistical_deferred_file_waveforms_are_rejected_before_dispatch() {
    let directory = fixture_dir("statistical-waveform-files");
    fs::write(
        directory.join("statistics.scs"),
        "simulator lang=spectre\nstatistics {\nprocess {\nvary level dist=gauss std=0.1\n}\n}\n",
    )
    .unwrap();
    let path = directory.join("deck.cir");
    for (waveform, file_backed) in [
        ("PWL(0 0 1 {level})", false),
        ("PWL FILE \"unsealed.csv\"", true),
    ] {
        let source = format!(
            "deck\n.param level=1\n.include \"statistics.scs\"\nV1 out 0 DC {{level}} {waveform}\nR1 out 0 1k\n.op\n.end\n"
        );
        fs::write(&path, &source).unwrap();
        let mut state = manual_deck_state(&source);
        state.workspace.content.netlist_source_path = Some(path.clone());
        let result =
            SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck);
        if file_backed {
            let error = result.expect_err("statistical deferral cannot hide an input file");
            assert_eq!(error.stage(), PreparationStage::SourceChecks);
            assert!(
                error.message().contains("file-backed PWL source"),
                "{error}"
            );
        } else {
            result.expect("statistical inline waveforms remain executable");
        }
    }
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn corner_waveforms_are_checked_after_materialization_and_scope_resolution() {
    use rspice_app_types::product::ProcessCorner;
    use rspice_model_library::CornerModelBinding;
    use rspice_simulation::sweeps::CornerRunConfig;

    let root = "deck\n.param file=2\nX1 out source level=3\nR1 out 0 1k\n.op\n.end\n";
    for (waveform, file_backed) in [
        ("PWL(0 0 1 {file})", false),
        ("PWL\n+ FILE=\"late.csv\"", true),
    ] {
        let task = QueuedAnalysis {
            numeric_override: None,
            spec: AnalysisSpec::Corner,
            config: None,
            spec_options: SpecExecutionOptions {
                corner: Some(CornerRunConfig {
                    process_corners: vec![ProcessCorner::FF],
                    model_bindings: vec![CornerModelBinding {
                        process: ProcessCorner::FF,
                        source_label: "foundry.lib [FF]".to_owned(),
                        section: Some("FF".to_owned()),
                        materialized_model_cards: format!(
                            ".subckt source out params: level=1\nV1 out 0 DC {{level}} {waveform}\n.ends\n"
                        ),
                    }],
                    ..CornerRunConfig::default()
                }),
                ..SpecExecutionOptions::default()
            },
            analysis_line: ".corner".to_owned(),
        };
        let result = reject_deferred_corner_model_sources(&[task], root);
        if file_backed {
            let error = result.expect_err("corner-local data files must be sealed");
            assert_eq!(error.stage(), PreparationStage::ModelBindings);
            assert!(
                error.message().contains("file-backed PWL source"),
                "{error}"
            );
        } else {
            result.expect("inline corner waveforms may use root and instance parameters");
        }
    }
}

#[test]
fn every_materialized_corner_binding_is_audited_before_dispatch() {
    use rspice_app_types::product::ProcessCorner;
    use rspice_model_library::CornerModelBinding;
    use rspice_simulation::sweeps::CornerRunConfig;

    let task = QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::Corner,
        config: None,
        spec_options: SpecExecutionOptions {
            corner: Some(CornerRunConfig {
                process_corners: vec![ProcessCorner::FF],
                model_bindings: vec![CornerModelBinding {
                    process: ProcessCorner::FF,
                    source_label: "foundry.lib [FF]".to_owned(),
                    section: Some("FF".to_owned()),
                    materialized_model_cards:
                        ".model external d_source (input_file\n+ = \"C:/late/stimulus.txt\")"
                            .to_owned(),
                }],
                ..CornerRunConfig::default()
            }),
            ..SpecExecutionOptions::default()
        },
        analysis_line: ".corner".to_owned(),
    };

    let error = reject_deferred_corner_model_sources(&[task], "deck\n.op\n.end\n")
        .expect_err("non-reference corner source must be sealed");
    assert_eq!(error.stage(), PreparationStage::ModelBindings);
    assert!(error.message().contains("foundry.lib [FF]"));
    assert!(error.message().contains("unsealed external dependency"));
}

#[test]
fn active_batch_reentry_preserves_prepared_authorization_and_batch_metadata() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source =
        Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare replacement request");
    let prepared_digest = snapshot.digest();
    controller
        .run_authorization
        .retain(snapshot)
        .expect("authorize replacement request");

    let active_run_id = state.simulation.start_run().id;
    controller.current_run_id = Some(active_run_id);
    controller.current_spec = Some(AnalysisSpec::dc_op());
    controller.current_analysis_idx = 1;
    controller.total_analyses = 2;
    controller.cached_netlist = Some("existing sealed batch".to_owned());

    controller.start_authorized_snapshot(&mut state);

    assert_eq!(controller.current_run_id, Some(active_run_id));
    assert_eq!(controller.current_spec, Some(AnalysisSpec::dc_op()));
    assert_eq!(controller.current_analysis_idx, 1);
    assert_eq!(controller.total_analyses, 2);
    assert_eq!(
        controller.cached_netlist.as_deref(),
        Some("existing sealed batch")
    );
    assert_eq!(state.simulation.runs.len(), 1);
    assert_eq!(
        controller
            .run_authorization
            .retained_snapshot()
            .map(|snapshot| snapshot.digest()),
        Some(prepared_digest)
    );
}

#[test]
fn unpolled_completion_reentry_does_not_consume_or_replace_authorization() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source =
        Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare replacement request");
    let prepared_digest = snapshot.digest();
    controller
        .run_authorization
        .retain(snapshot)
        .expect("authorize replacement request");
    controller.runner = crate::simulation::controller::test_execution::start_manual_deck(
        state.workspace.content.netlist_source.as_deref().unwrap(),
    );
    crate::simulation::controller::test_execution::wait_until_finished_unpolled(&controller.runner);

    controller.start_authorized_snapshot(&mut state);

    assert!(state.simulation.runs.is_empty());
    assert_eq!(controller.total_analyses, 0);
    assert_eq!(
        controller
            .run_authorization
            .retained_snapshot()
            .map(|snapshot| snapshot.digest()),
        Some(prepared_digest)
    );
    assert!(!controller.runner.can_accept_prepared_task());
}

#[test]
fn direct_manual_run_cannot_bypass_internal_prepare_and_permit_consumption() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source =
        Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();

    controller.start_simulation(&mut state);

    assert!(controller.run_authorization.retained_snapshot().is_none());
    assert_eq!(controller.total_analyses, 0);
    assert!(controller.cached_netlist.is_none());

    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");
    controller.start_simulation(&mut state);

    assert!(controller.run_authorization.retained_snapshot().is_none());
    assert_eq!(controller.total_analyses, 1);
    assert!(
        controller
            .cached_netlist
            .as_deref()
            .is_some_and(|netlist| netlist.contains(".op"))
    );
    controller.abort();
}

#[test]
fn included_source_mutation_after_prepare_is_rejected() {
    let directory = fixture_dir("include-mutation");
    let origin = directory.join("deck.cir");
    let include = directory.join("device.inc");
    fs::write(&include, "R1 out 0 1k\n").expect("write include");
    let source = "deck\n.include device.inc\nV1 out 0 1\n.op\n.end\n";
    fs::write(&origin, source).expect("write deck origin");

    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(source.to_owned());
    state.workspace.content.netlist_source_path = Some(origin);
    let mut controller = SimulationController::new();
    let metadata = controller
        .validate_manual_deck_document(&state)
        .expect("validate and retain first include closure");

    fs::write(&include, "R1 out 0 2k\n").expect("mutate include");
    let changed =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare changed include closure")
            .metadata();
    assert_ne!(metadata.source_digest, changed.source_digest);
    let error = controller
        .consume_snapshot_for_dispatch(&state)
        .expect_err("changed include must invalidate authorization");
    assert_eq!(error.stage(), PreparationStage::Authorization);

    fs::remove_dir_all(directory).expect("remove include fixture");
}

#[test]
fn dispatched_include_closure_never_reopens_mutated_source_files() {
    let directory = fixture_dir("dispatched-include-closure");
    let origin = directory.join("deck.cir");
    let include = directory.join("device.inc");
    fs::write(&include, "R1 out 0 1k\n").expect("write include");
    let source = "deck\n.include device.inc\nV1 out 0 1\n.op\n.end\n";
    fs::write(&origin, source).expect("write deck origin");

    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(source.to_owned());
    state.workspace.content.netlist_source_path = Some(origin);
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("validate the exact include closure");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("freeze the authorized dispatch");

    fs::write(&include, "R1 out 0 2k\n").expect("mutate source after dispatch");

    assert!(dispatch.executable_netlist().contains("R1 out 0 1k"));
    assert!(!dispatch.executable_netlist().contains("R1 out 0 2k"));
    assert!(!contains_external_include_directive(
        dispatch.executable_netlist()
    ));
    for task in dispatch.tasks() {
        assert_eq!(
            task.executable_netlist().as_ref(),
            dispatch.executable_netlist()
        );
    }

    fs::remove_dir_all(directory).expect("remove include fixture");
}

#[test]
fn accepted_model_file_mutation_after_prepare_fails_closed() {
    let directory = fixture_dir("model-mutation");
    let model = directory.join("foundry.lib");
    fs::write(
        &model,
        ".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n",
    )
    .expect("write model");
    let mut state = runnable_state();
    let library_name = state
        .model_library_manager
        .load_library_file(&model, None)
        .expect("load model library");
    state.sim_setup.model_bindings.push(
        state
            .model_library_manager
            .simulation_plan_binding(&library_name)
            .expect("explicitly bind the loaded library to this plan"),
    );
    let mut controller = SimulationController::new();
    controller
        .prepare_run_set_for_preflight(&state)
        .expect("prepare model-bound run");

    fs::write(
        &model,
        ".lib TT\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl TT\n",
    )
    .expect("mutate model");
    let error = controller
        .consume_snapshot_for_dispatch(&state)
        .expect_err("changed accepted model bytes must block dispatch");
    assert_eq!(error.stage(), PreparationStage::ModelBindings);
    assert!(error.message().contains("changed"));

    fs::remove_dir_all(directory).expect("remove model fixture");
}

#[test]
fn rebuilt_prepared_snapshots_are_deterministic() {
    let state = runnable_state();
    let first =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("baseline snapshot");
    let baseline_netlist = first.executable_netlist().to_owned();
    let baseline = first.metadata();

    // Dispatch rebuilds the snapshot and refuses to run when the digest moves,
    // so unchanged state has to seal to one identity every time. Anything
    // sampled from the clock or from a per-instance hash seed shows up here as
    // drift between two builds that saw exactly the same inputs.
    for attempt in 0..16 {
        let snapshot = SimulationController::build_prepared_snapshot(
            &state,
            SimulationRunIntent::SimulateRunSet,
        )
        .expect("rebuilt snapshot");
        assert_eq!(
            snapshot.executable_netlist(),
            baseline_netlist,
            "attempt {attempt}: executable source drifted with no state change"
        );
        let rebuilt = snapshot.metadata();
        assert_eq!(
            rebuilt.source_digest, baseline.source_digest,
            "attempt {attempt}: executable source digest drifted"
        );
        assert_eq!(
            rebuilt.receipt_digest, baseline.receipt_digest,
            "attempt {attempt}: source-check receipt drifted"
        );
        assert_eq!(
            rebuilt.advisories, baseline.advisories,
            "attempt {attempt}: advisories drifted"
        );
        assert_eq!(
            rebuilt.snapshot_digest, baseline.snapshot_digest,
            "attempt {attempt}: prepared snapshot digest drifted"
        );
    }
}

#[test]
fn repeated_authorize_and_dispatch_cycles_never_expire_an_unchanged_run() {
    for attempt in 0..16 {
        let state = runnable_state();
        let mut controller = SimulationController::new();
        let snapshot = SimulationController::build_prepared_snapshot(
            &state,
            SimulationRunIntent::SimulateRunSet,
        )
        .expect("project run seals signed PDK model sources");
        controller
            .run_authorization
            .retain(snapshot)
            .expect("authorize exact project snapshot");
        controller
            .consume_snapshot_for_dispatch(&state)
            .unwrap_or_else(|error| {
                panic!("attempt {attempt}: unchanged run was rejected: {error:?}")
            });
    }
}

/// Insert an enabled analysis draft at the end of the state's stable plan.
fn insert_enabled_draft(
    state: &mut AppState,
    draft: crate::simulation::plan::AnalysisDraft,
) -> crate::product::AnalysisInstanceId {
    let plan = state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("test state owns a stable plan");
    let position = plan.instances().len();
    plan.insert_draft_with_id(
        crate::product::AnalysisInstanceId::new(),
        draft,
        true,
        position,
    )
    .expect("an appended analysis has no prerequisites")
    .0
}

fn manual_deck_state(deck: &str) -> AppState {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(deck.to_owned());
    state
}

#[test]
fn reviewed_foreign_profile_is_applied_when_the_owned_deck_is_prepared() {
    let source = "reviewed export\nsimulator lang=spice\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let mut state = manual_deck_state(source);
    assert!(apply_imported_netlist(
        &mut state,
        source.to_owned(),
        None,
        "reviewed.cir",
    ));
    let descriptor = state.workspace.content.netlist_descriptor.as_mut().unwrap();
    descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spectre);
    descriptor.execution_profile = Some(crate::state::NetlistExecutionProfile::SpectreSpiceV1);
    descriptor.compatibility_reviewed = true;
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("the accepted adapter must also reach run preparation");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("validated source dispatches");
    assert_eq!(dispatch.manual_source(), Some(source));
    let parsed =
        rspice_core::Netlist::parse(dispatch.executable_netlist()).expect("prepared source parses");
    assert_eq!(parsed.elements.len(), 2);
    assert_eq!(parsed.analyses.len(), 1);
    assert!(
        dispatch
            .executable_netlist()
            .contains("presentation directive: simulator lang=spice")
    );
}

#[test]
fn an_unreviewed_owned_profile_cannot_acquire_a_manual_execution_permit() {
    let source = "quarantined\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let mut state = manual_deck_state(source);
    assert!(apply_imported_netlist(
        &mut state,
        source.to_owned(),
        None,
        "quarantined.cir",
    ));
    let descriptor = state.workspace.content.netlist_descriptor.as_mut().unwrap();
    descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spice3Ngspice);
    descriptor.execution_profile = None;
    descriptor.compatibility_reviewed = false;
    let mut controller = SimulationController::new();
    assert!(controller.validate_manual_deck_document(&state).is_err());
    assert!(controller.run_authorization.retained_snapshot().is_none());
}

#[test]
fn reviewed_ngspice_profile_binds_engine_defaults_and_revalidates_edits() {
    let source = "reviewed control\nV1 out 0 1\nR1 out 0 1k\n.control; analyses\nop; harmless note\n.endc\n.end\n";
    let mut state = manual_deck_state(source);
    assert!(apply_imported_netlist(
        &mut state,
        source.to_owned(),
        None,
        "reviewed.cir"
    ));
    let descriptor = state.workspace.content.netlist_descriptor.as_mut().unwrap();
    descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spice3Ngspice);
    descriptor.execution_profile = Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2);
    descriptor.compatibility_reviewed = true;
    let mut controller = SimulationController::new();
    controller.validate_manual_deck_document(&state).unwrap();
    let dispatch = controller.consume_snapshot_for_dispatch(&state).unwrap();
    assert_eq!(dispatch.manual_source(), Some(source));
    assert!(
        dispatch
            .executable_netlist()
            .contains("RSpice execution profile: spice3-ngspice/2")
    );
    let parsed = rspice_core::Netlist::parse(dispatch.executable_netlist()).unwrap();
    let config = crate::simulation::dialog::SimulationOptions::default()
        .resolve_simulation_config(Some(&parsed.options));
    assert_eq!(config.spice_dialect, rspice_core::SpiceDialect::Ngspice);
    assert_eq!(
        config.resolved_jfet_level2_model(),
        rspice_core::SpiceDialect::Ngspice.default_jfet_level2_model()
    );
    for edit in [
        source.replace("op; harmless note", "wrdata out.txt v(out)"),
        source.replace(
            ".control; analyses",
            ".OPTIONS RSPICE_DIALECT=XYCE\n.control",
        ),
        source.replace(".control; analyses", ".unsupported_card\n.control"),
    ] {
        state.workspace.content.netlist_source = Some(edit);
        assert!(controller.validate_manual_deck_document(&state).is_err());
        assert!(controller.run_authorization.retained_snapshot().is_none());
    }
}

#[test]
fn a_project_without_a_technology_prepares_its_run_set_against_the_plain_model_library() {
    let state = technology_free_runnable_state();
    let mut controller = SimulationController::new();
    let metadata = controller
        .prepare_run_set_for_preflight(&state)
        .expect("a project owing nothing to a technology prepares its run set");
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("the same technology-free contract rebuilds");
    let executable = snapshot.executable_netlist();
    assert!(!executable.contains("nmos_demo"), "{executable}");
    assert!(!executable.contains("demo180"), "{executable}");

    let attached = runnable_state();
    let with_technology = SimulationController::build_prepared_snapshot(
        &attached,
        SimulationRunIntent::SimulateRunSet,
    )
    .expect("the same design prepares with a technology attached");
    assert!(with_technology.executable_netlist().contains("demo180"));
    assert!(
        metadata.model_identity_count < with_technology.metadata().model_identity_count,
        "the signed PDK identity must not reach a run set prepared without a technology"
    );
}

#[test]
fn an_enabled_corner_analysis_without_a_technology_blocks_preparation() {
    let mut state = technology_free_runnable_state();
    // The sections are declared by the plan, which is the only place a run
    // space is declared now. The corner instance reads them.
    state.sim_setup.run_set = crate::simulation::run_set::RunSetState::default();
    for dimension in &mut state.sim_setup.run_set.dimensions {
        if dimension.kind == crate::simulation::run_set::RunSetDimensionKind::Supply {
            dimension.source = format!(
                "{}VDD",
                crate::simulation::run_set::NETLIST_SUPPLY_SOURCE_PREFIX
            );
        }
    }
    let corner = crate::simulation::dialog::corner::CornerDialogState::default();
    insert_enabled_draft(
        &mut state,
        crate::simulation::plan::AnalysisDraft::Corner(corner),
    );

    let mut controller = SimulationController::new();
    let error = controller
        .prepare_run_set_for_preflight(&state)
        .expect_err("non-typical corner sections demand an attached technology");
    assert_eq!(error.stage(), PreparationStage::ModelBindings);
    assert!(
        error
            .message()
            .contains("requires an attached project technology"),
        "{error}"
    );
    // The sections are named by the declaration that asks for them. They used
    // to be attributed to the corner instance, back when the instance carried
    // its own space; naming an instance now would attribute the plan's
    // declaration to whichever analysis happened to read it first.
    assert!(
        error.message().contains("global Run Set requests SS, FF"),
        "{error}"
    );
}

#[test]
fn a_technology_binding_without_an_audit_receipt_blocks_preparation() {
    let mut state = runnable_state();
    // A copy keeps the binding and starts a fresh audit history, which is
    // exactly the shape the reattach contract refuses.
    state.workspace.content.project = state
        .workspace
        .content
        .project
        .fork_copy_at(PathBuf::from("forked").join("copy.rsproj"));
    assert!(
        state
            .workspace
            .content
            .project
            .technology_binding()
            .is_some()
    );
    assert!(!state.project_technology_in_effect());

    let mut controller = SimulationController::new();
    let error = controller
        .prepare_run_set_for_preflight(&state)
        .expect_err("a binding without receipts is an invalid technology, not an absent one");
    assert_eq!(error.stage(), PreparationStage::ModelBindings);
    assert!(
        error
            .message()
            .contains("predates checkpoint-backed authority receipts"),
        "{error}"
    );
}

#[test]
fn an_unresolved_device_model_is_named_at_prepare_rather_than_at_dispatch() {
    let state = manual_deck_state(
        "manual model check\nV1 d 0 1\nM1 d g 0 0 nch_missing\nR1 g 0 1k\n.op\n.end\n",
    );
    let mut controller = SimulationController::new();
    let error = controller
        .validate_manual_deck_document(&state)
        .expect_err("a device naming an undefined model cannot be prepared");
    assert_eq!(error.stage(), PreparationStage::ModelBindings);
    let named = error.message().to_ascii_lowercase();
    assert!(named.contains("nch_missing"), "{error}");
    assert!(named.contains("m1"), "{error}");
    assert!(named.contains("mosfet"), "{error}");
    assert!(
        error
            .message()
            .contains("No project technology is attached"),
        "{error}"
    );
}

#[test]
fn builder_resolvable_model_names_prepare_without_any_card() {
    for deck in [
        // A bare type name binds a MOSFET with no card at all.
        "manual model check\nV1 d 0 1\nM1 d g 0 0 NMOS\nR1 g 0 1k\n.op\n.end\n",
        // The embedded foundation library carries this part.
        "manual model check\nV1 a 0 1\nD1 a 0 RSPICE_DIODE\n.op\n.end\n",
        // Nothing instantiates the subcircuit, so the builder never binds it.
        "manual model check\n.subckt unused a k\nD1 a k missing_d\n.ends\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
    ] {
        let state = manual_deck_state(deck);
        let mut controller = SimulationController::new();
        if let Err(error) = controller.validate_manual_deck_document(&state) {
            panic!("a deck the builder resolves was rejected: {deck}: {error}");
        }
    }
}

#[test]
fn a_generated_run_set_names_its_unresolved_device_models() {
    let mut state = technology_free_runnable_state();
    let bound = state
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.name == "R1")
        .expect("the divider fixture owns R1");
    bound.kind = crate::state::ComponentType::Diode;
    bound.name = "D1".to_owned();
    bound.value = "d_missing".to_owned();

    let mut controller = SimulationController::new();
    let error = controller
        .prepare_run_set_for_preflight(&state)
        .expect_err("a generated deck naming an undefined model cannot be prepared");
    assert_eq!(error.stage(), PreparationStage::ModelBindings);
    let named = error.message().to_ascii_lowercase();
    assert!(named.contains("d_missing"), "{error}");
    assert!(named.contains("diode"), "{error}");
}

/// One cell instantiated twice by the active schematic: the smallest design
/// whose deck declares one master and instantiates two occurrences.
///
/// The cell declares no interface, so the two placements add no node to the
/// root and the divider fixture's own design checks still pass.
fn author_two_occurrences_of_one_cell(state: &mut AppState) {
    use crate::state::{
        CellViewRef, ComponentType, Library, LibraryCellInstance, Point, SchematicState, View,
        ViewType,
    };

    let mut master = SchematicState::default();
    master.add_component(ComponentType::Resistor, Point::new(30, 0));

    if state.library_manager.get_library("user").is_none() {
        state.library_manager.add_library(Library::new("user"));
    }
    let cell = state
        .library_manager
        .get_library_mut("user")
        .expect("the project library")
        .get_or_create_cell("pad");
    if cell.get_view("schematic").is_none() {
        cell.add_view(View::new("schematic", ViewType::Schematic));
    }
    state
        .workspace
        .insert_schematic_editor(CellViewRef::new("user", "pad", "schematic").key(), master);

    let mut binding = LibraryCellInstance::new("user", "pad", "schematic");
    binding.bind_interface(&[]);
    state
        .schematic
        .add_library_cell_component(Point::new(400, 400), binding.clone());
    state
        .schematic
        .add_library_cell_component(Point::new(600, 400), binding);
    state.dialogs.drc_checked_version = state.schematic.topology_version();
}

#[test]
fn the_prepared_snapshot_carries_the_decks_emission_map() {
    let mut state = runnable_state();
    author_two_occurrences_of_one_cell(&mut state);

    let projection = state
        .workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .expect("the authored hierarchy projects");
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
        state.library_manager.catalog(),
        &projection,
    );
    let generated = rspice_simulation::netlist_gen::generate_netlist_hierarchical(
        projection
            .root_schematic()
            .expect("the projection carries the root"),
        &[],
        &hierarchy,
        &rspice_simulation::netlist_gen::NetlistSourceData::new(
            &crate::simulation::table_route::SourceFiles,
        ),
    );
    assert!(generated.errors.is_empty(), "{:?}", generated.errors);

    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("the authored hierarchy prepares");

    let rows = |map: &[rspice_simulation::netlist_gen::EmissionRow]| {
        map.iter()
            .map(|row| {
                (
                    row.occurrence.to_string(),
                    row.master.clone(),
                    row.engine_prefix.clone(),
                )
            })
            .collect::<Vec<_>>()
    };
    let carried = rows(snapshot.emission_map());
    assert_eq!(
        carried,
        rows(&generated.emission_map),
        "the snapshot must carry the deck's own emission map, not a second derivation"
    );
    assert_eq!(carried.len(), 2, "{carried:?}");
    assert_eq!(
        carried[0].1, carried[1].1,
        "two occurrences of one cellview share one master: {carried:?}"
    );
    assert_eq!(
        carried
            .iter()
            .map(|(occurrence, _, prefix)| (occurrence.as_str(), prefix.as_str()))
            .collect::<Vec<_>>(),
        vec![("/X1", "X1"), ("/X2", "X2")],
        "{carried:?}"
    );
}

#[test]
fn the_authorized_run_receipt_seals_the_decks_hierarchy_map() {
    let mut state = runnable_state();
    author_two_occurrences_of_one_cell(&mut state);

    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("the authored hierarchy prepares");
    let emitted = snapshot
        .emission_map()
        .iter()
        .map(|row| {
            (
                row.occurrence.to_string(),
                row.master.clone(),
                row.engine_prefix.clone(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(emitted.len(), 2, "{emitted:?}");

    let receipt = crate::simulation::execution::PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .expect("the permitted snapshot authorizes")
        .prepared_run_receipt(crate::state::AnalysisResultSourceDomain::SimulationPlan)
        .expect("an authorized run states what it ran against");

    assert_eq!(
        receipt
            .hierarchy_map()
            .iter()
            .map(|row| (
                row.occurrence().to_owned(),
                row.master().to_owned(),
                row.engine_prefix().to_owned(),
            ))
            .collect::<Vec<_>>(),
        emitted,
        "the receipt seals the deck's own map rather than a second derivation"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn reviewed_spectre_header_survives_path_based_dependency_expansion() {
    let directory = fixture_dir("reviewed-spectre-header");
    let source = "simulator lang=spice\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let profile = crate::state::NetlistExecutionProfile::SpectreSpiceV1;
    for extension in ["cir", "scs"] {
        let path = directory.join(format!("export.{extension}"));
        fs::write(&path, source).unwrap();
        let adapted = profile.adapt_source(source).unwrap();
        let parsed = rspice_core::Netlist::parse_with_path(&adapted, &path).unwrap();
        profile.validate_parsed_netlist(&parsed).unwrap();
        let mut state = manual_deck_state(source);
        assert!(apply_imported_netlist(
            &mut state,
            source.to_owned(),
            Some(path),
            "export"
        ));
        let descriptor = state.workspace.content.netlist_descriptor.as_mut().unwrap();
        descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spectre);
        descriptor.execution_profile = Some(profile);
        descriptor.compatibility_reviewed = true;
        let mut controller = SimulationController::new();
        controller.validate_manual_deck_document(&state).unwrap();
        let dispatch = controller.consume_snapshot_for_dispatch(&state).unwrap();
        assert_eq!(dispatch.manual_source(), Some(source));
        let parsed = rspice_core::Netlist::parse(dispatch.executable_netlist()).unwrap();
        assert_eq!(parsed.elements.len(), 2);
        assert_eq!(parsed.analyses.len(), 1);
        assert_eq!(parsed.options.spice_dialect, Some(profile.spice_dialect()));
    }
    fs::remove_dir_all(directory).unwrap();
}
