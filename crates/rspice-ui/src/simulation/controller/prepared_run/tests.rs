//! Tests for the prepare-then-permit run path.
//!
//! A dispatched run must execute exactly the sources it was prepared against,
//! so these cases mutate files after prepare and assert the run fails closed
//! rather than silently reopening the changed source.

use super::*;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::services::drc::DrcResult;
static FIXTURE_NONCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn prepared_model_receipts_authenticate_exact_executable_model_definition() {
    let mut state = AppState::default();
    state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let definition = prepared_model_receipt_definition();
    state
        .model_library_manager
        .create_project_model("models-a", &definition)
        .unwrap();
    let source = definition.canonical_source().unwrap();
    let used = prepared_project_model_sources(
        &state,
        &format!("receipt fixture\n{source}M1 d g 0 0 nch_receipt\n.op\n.end\n"),
    )
    .unwrap();
    assert_eq!(used.len(), 1);
    assert_eq!(used[0].model_name(), "nch_receipt");
}

#[test]
fn prepared_model_receipts_reject_modified_same_name_manual_card() {
    let mut state = AppState::default();
    state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let definition = prepared_model_receipt_definition();
    state
        .model_library_manager
        .create_project_model("models-a", &definition)
        .unwrap();

    let modified = prepared_project_model_sources(
            &state,
            "receipt fixture\n.model nch_receipt NMOS (level=1 vth0=0.51)\nM1 d g 0 0 nch_receipt\n.op\n.end\n",
        )
        .unwrap();
    assert!(
        modified.is_empty(),
        "a same-name card with different executable parameters must not inherit project provenance"
    );
}

#[test]
fn prepared_model_receipts_ignore_exact_but_unused_model() {
    let mut state = AppState::default();
    state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let definition = prepared_model_receipt_definition();
    state
        .model_library_manager
        .create_project_model("models-a", &definition)
        .unwrap();
    let source = definition.canonical_source().unwrap();
    let unused = prepared_project_model_sources(
        &state,
        &format!("receipt fixture\n{source}V1 out 0 1\nR1 out 0 1k\n.op\n.end\n"),
    )
    .unwrap();
    assert!(unused.is_empty());
}

#[test]
fn prepared_model_receipts_reject_duplicate_project_model_name() {
    let mut state = AppState::default();
    state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let definition = prepared_model_receipt_definition();
    state
        .model_library_manager
        .create_project_model("models-a", &definition)
        .unwrap();
    state
        .model_library_manager
        .create_project_model("models-b", &definition)
        .unwrap();
    let source = definition.canonical_source().unwrap();
    let ambiguous = prepared_project_model_sources(
        &state,
        &format!("receipt fixture\n{source}M1 d g 0 0 nch_receipt\n.op\n.end\n"),
    )
    .unwrap();
    assert!(
        ambiguous.is_empty(),
        "an executable model name shared by multiple project sources cannot authenticate either source"
    );
}

fn prepared_model_receipt_definition() -> crate::state::model_library::ProjectModelDefinition {
    crate::state::model_library::ProjectModelDefinition {
        name: "nch_receipt".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Prepared receipt fixture".to_owned(),
        numeric_parameters: std::collections::BTreeMap::from([
            ("level".to_owned(), 1.0),
            ("vth0".to_owned(), 0.48),
        ]),
        string_parameters: std::collections::BTreeMap::new(),
    }
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
    crate::workbench::examples::load_example("Voltage Divider", &mut state.schematic);
    let mut drc = DrcResult::new();
    drc.completed = true;
    state.dialogs.drc_results = Some(drc);
    state.dialogs.drc_checked_version = state.schematic.topology_version();
    state
}

fn runnable_state() -> AppState {
    let mut state = technology_free_runnable_state();
    state.provision_test_project_technology_contract();
    state
}

#[test]
fn prepared_project_run_materializes_exact_signed_pdk_reference_models() {
    let mut state = runnable_state();
    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect("project run seals signed PDK model sources");
    controller
        .authorize_snapshot(snapshot)
        .expect("authorize exact project snapshot");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&mut state)
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
    state.workspace.netlist_source =
        Some("signed project deck\nV1 d 0 1\nM1 d d 0 0 nmos_demo\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
        .expect("governed manual project deck seals signed PDK sources");
    controller
        .authorize_snapshot(snapshot)
        .expect("authorize exact governed manual deck");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&mut state)
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
    state.workspace.netlist_source =
        Some("signed Verilog-A project deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
        .expect("governed manual deck prepares the signed Verilog-A runtime");
    controller
        .authorize_snapshot(snapshot)
        .expect("authorize exact signed Verilog-A snapshot");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&mut state)
        .expect("freeze signed Verilog-A dispatch");
    let executable = dispatch.executable_netlist();
    assert!(executable.contains(".veriloga \"__rspice_pdk__/"));
    assert!(executable.contains(" pdk_resistor_model"));
    assert!(!executable.contains("veriloga/pdk_resistor.va"));
    assert!(!contains_external_include_directive(executable));
}

fn prepared_pss_task(
    tone_sources: impl IntoIterator<Item = &'static str>,
    oscillator_mode: bool,
) -> PreparedTask {
    PreparedTask::new(
        crate::product::AnalysisInstanceId::new(),
        crate::product::ObjectRevision::INITIAL,
        Vec::new(),
        "PSS",
        QueuedAnalysis {
            numeric_override: None,
            spec: AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                fundamental_freq: 1.0e3,
                tone_sources: tone_sources.into_iter().map(str::to_owned).collect(),
                tstab_periods: 20,
                points_per_period: 512,
                tolerance: 1.0e-7,
                oscillator_mode,
                oscillator_node: oscillator_mode.then(|| "out".to_owned()),
                num_harmonics: 20,
            },
            config: None,
            spec_options: SpecExecutionOptions::default(),
            analysis_line: ".pss 1k".to_owned(),
        },
    )
}

#[test]
fn prepared_pss_authenticates_the_complete_executable_source_set() {
    let deck = "periodic sources\nVLO lo 0 SIN(0 1 1k)\nVCLK clk 0 PULSE(0 1 0 1u 1u 200u 500u)\nR1 lo 0 1k\nR2 clk 0 1k\n.end\n";
    validate_prepared_periodic_sources(&[prepared_pss_task(["vclk", "VLO"], false)], deck)
        .expect("the complete commensurate source set is accepted");

    let error = validate_prepared_periodic_sources(&[prepared_pss_task(["VLO"], false)], deck)
        .expect_err("an omitted periodic source fails preflight");
    assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
    assert!(error.message().contains("omitted: VCLK"));
}

#[test]
fn prepared_autonomous_pss_accepts_an_exact_empty_driven_source_set() {
    let deck = "autonomous oscillator\nR1 out 0 1k\nC1 out 0 1n\n.end\n";
    validate_prepared_periodic_sources(&[prepared_pss_task([], true)], deck)
        .expect("a source-free autonomous circuit has an exact empty source set");
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
    let controller = SimulationController::new();
    let prepared = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect("prepare first snapshot");
    let revision = state.workspace.project.revision().get();
    edit_frozen_transient_stop(&mut state, "2m");
    let changed = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect("prepare changed snapshot");
    assert_eq!(state.workspace.project.revision().get(), revision);
    assert_ne!(prepared.digest(), changed.digest());
}

#[test]
fn prepared_snapshot_detects_non_topology_source_mutation() {
    let mut state = runnable_state();
    let controller = SimulationController::new();
    let prepared = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect("prepare first snapshot");
    let topology = state.schematic.topology_version();
    state.schematic.components[0].value = "2k".to_owned();
    let changed = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect("prepare changed snapshot");
    assert_eq!(state.schematic.topology_version(), topology);
    assert_ne!(prepared.digest(), changed.digest());
}

#[test]
fn prepared_snapshot_authenticates_plan_owned_saved_outputs() {
    let mut state = runnable_state();
    let controller = SimulationController::new();
    let without_output = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
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
        .add_saved_output(plan_id, output)
        .expect("plan owns output");

    let with_output = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
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
    state.schematic.current_file = Some(PathBuf::from("designs").join("amp.rsch"));
    let tasks = vec![QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::SParameter {
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

    let prepared =
        touchstone_export_policy(&state, &tasks, state.schematic.current_file.as_deref())
            .expect("capture enabled policy");
    let prepared_path = prepared.output_path(7, 1, 2).expect("enabled export path");

    state.schematic.current_file = Some(PathBuf::from("redirect").join("changed.rsch"));
    let mut disabled = crate::simulation::dialog::SpConfig::default();
    disabled.touchstone_export = false;
    state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(&disabled);
    let current = touchstone_export_policy(&state, &tasks, state.schematic.current_file.as_deref())
        .expect("capture disabled policy");

    assert!(current.output_path(7, 1, 2).is_none());
    assert!(prepared_path.ends_with("designs/amp_run0007_sp01.s2p"));
}

#[test]
fn manual_touchstone_export_uses_imported_deck_origin_not_stale_schematic_path() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.schematic.current_file = Some(PathBuf::from("stale").join("schematic.rsch"));
    state.workspace.netlist_source_path = Some(PathBuf::from("imported").join("rf_fixture.cir"));
    state.workspace.netlist_source = Some(
            "deck\nV2 out 0 dc 0 ac 1 portnum 2 z0 75\nV1 in 0 dc 0 ac 1 portnum 1 z0 50\nR1 in out 100\n.sp lin 3 1Meg 3Meg\n.end\n"
                .to_owned(),
        );
    state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(
        &crate::simulation::dialog::SpConfig::default(),
    );

    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
        .expect("prepare imported RF deck");
    controller
        .authorize_snapshot(snapshot)
        .expect("authorize imported RF deck");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&mut state)
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
    state.workspace.netlist_source = Some(
        "Fourier deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k V(out)\n.tran 10u 5m\n.end\n"
            .to_owned(),
    );

    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
        .expect("prepare manual Fourier dependency graph");
    controller
        .authorize_snapshot(snapshot)
        .expect("authorize manual Fourier dependency graph");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&mut state)
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
    state.workspace.migrate_active_plan_data(first_plan_id);
    let second_plan_id = state
        .sim_setup
        .create_plan("Second campaign plan")
        .expect("create campaign plan");
    state.workspace.migrate_inactive_plan_data(second_plan_id);
    state.workspace.sync_legacy_specs_projection(second_plan_id);
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

    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("campaign member project snapshot");
    let json = crate::io::project_io::serialize_project_file(&project)
        .expect("campaign member serializes");
    let loaded =
        crate::io::project_io::load_project_text(&json, None).expect("campaign member reloads");
    let restored = loaded
        .simulation_results
        .into_simulation_state()
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
    let controller = SimulationController::new();
    let baseline = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect("baseline save policy prepares");

    state.sim_setup.save_policy.live_streaming_enabled = false;
    state.sim_setup.save_policy.retain_failure_diagnostics = false;
    state.sim_setup.save_policy.retained_dataset_limit = 5;
    let changed = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
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
    let error = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect_err("one-byte plan budget must refuse retained output");
    assert!(error.message().contains("storage budget"), "{error}");
}

#[test]
fn manual_periodic_analyses_are_topologically_bound_to_seed_and_pss() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.netlist_source = Some(
        "Periodic deck\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\nLPROBE out sensed 1n\nR2 sensed 0 1k\n.pss 1Meg tones=V1 points_per_period=128 save_harmonics=8\n.pac dec 20 1k 100Meg input=V1 output=out\n.pnoise dec 10 1 1Meg output=out noiseref=output\n.pxf dec 10 1k 10Meg input=V1 output=out outsideband=1\n.pstb probe=LPROBE maxharm=8 nmults=6\n.end\n"
            .to_owned(),
    );

    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
        .expect("prepare manual periodic dependency graph");
    controller
        .authorize_snapshot(snapshot)
        .expect("authorize manual periodic dependency graph");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&mut state)
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
        .consume_snapshot_for_dispatch(&mut state)
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
        .set_reference_pvt(crate::simulation::dialog::corner::ProcessCorner::TT, 125.0)
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
    state.workspace.netlist_source =
        Some("deck\n.include models.lib\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let controller = SimulationController::new();
    let error = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
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
        &crate::state::ModelLibraryManager::default(),
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
fn every_runtime_external_input_category_fails_closed() {
    let cases = [
        ".include model.lib",
        ".lib model.lib TT",
        ".spef_include parasitics.spef",
        ".VERILOGA compact_model.va",
        ".VA compact_model.va",
        ".ahdl_include compact_model.va",
        ".hdl compact_model.va",
        ".verilog compact_model.va",
        ".load prior.raw",
        "Vstim in 0 PWL FILE \"wave.csv\"",
        ".measure tran fit ERROR V(out) FILE reference.prn DEPVARCOL 2",
        ".model touchstone transfer (file = \"network.s2p\")",
        ".model source d_source (input_file = \"stimulus.txt\")",
        ".model state d_state (state_file = \"state.tbl\")",
        ".model process d_process (process_file = worker)",
        ".model cosim d_cosim (simulation = \"payload.dll\")",
        "Aco [in] [out] null cosim simulation = provider",
    ];

    for line in cases {
        let Err(error) = reject_deferred_external_sources(line) else {
            panic!("unsealed runtime input must be rejected: {line}");
        };
        assert_eq!(error.stage(), PreparationStage::SourceChecks, "{line}");
        assert!(
            error.message().contains("unsealed external dependency"),
            "{line}"
        );
    }
}

#[test]
fn case_altered_project_veriloga_key_is_rejected_before_dispatch() {
    let mut state = AppState::default();
    state
        .workspace
        .replace_imported_project_source(
            crate::state::ProjectSourceLanguage::VerilogA,
            "model.va".to_owned(),
            "module owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
                .to_owned(),
        )
        .expect("replace bootstrapped project Verilog-A source");
    let bundle = state
        .workspace
        .project_sources
        .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
            crate::state::ProjectSourceLanguage::VerilogA,
        ))
        .expect("installed project source bundle");
    let receipt = crate::workbench::documents::code_workspace::compile_project_bundle_receipt(
        state.workspace.project.id(),
        bundle,
        None,
    )
    .expect("compile project Verilog-A source");
    state.ui.code_workspace.veriloga.receipt = Some(receipt);

    let bundle = state
        .workspace
        .project_sources
        .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
            crate::state::ProjectSourceLanguage::VerilogA,
        ))
        .expect("installed project source bundle");
    let source_key = crate::state::project_veriloga_bundle_source_key(
        state.workspace.project.id(),
        bundle,
        "owned",
    )
    .expect("derive exact project source key");
    let exact_directive =
        crate::simulation::veriloga::project_veriloga_directive(&source_key, "owned");
    let exact_runtimes = project_veriloga_runtimes_referenced_by(&state, &exact_directive)
        .expect("inspect exact project directive");
    assert_eq!(exact_runtimes.len(), 1);
    reject_deferred_external_sources_with_project_runtimes(&exact_directive, &exact_runtimes)
        .expect("exact project identity is permitted");

    let altered_key = source_key.replacen("__rspice_project__", "__RSPICE_PROJECT__", 1);
    let altered_directive =
        crate::simulation::veriloga::project_veriloga_directive(&altered_key, "owned");
    let altered_runtimes = project_veriloga_runtimes_referenced_by(&state, &altered_directive)
        .expect("inspect altered project directive");
    assert!(
        altered_runtimes.is_empty(),
        "case-altered virtual paths must not acquire the exact project runtime"
    );
    let error = reject_deferred_external_sources_with_project_runtimes(
        &altered_directive,
        &altered_runtimes,
    )
    .expect_err("case-altered project key must remain an external dependency");
    assert_eq!(error.stage(), PreparationStage::SourceChecks);
    assert!(error.message().contains("unsealed external dependency"));
}

#[test]
fn project_veriloga_path_is_exact_while_spice_identifiers_ignore_case() {
    let source_key = "__rspice_project__/project/digest/Model.va";
    assert!(project_veriloga_directive_matches_exact_identity(
        ".VERILOGA \"__rspice_project__/project/digest/Model.va\" OWNED",
        source_key,
        "owned",
    ));
    assert!(!project_veriloga_directive_matches_exact_identity(
        ".VERILOGA \"__rspice_project__/project/digest/model.va\" OWNED",
        source_key,
        "owned",
    ));
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
            &state.workspace.active_view,
            &state.schematic,
        )
        .expect("resolve configured behavioral view");
    let runtimes = prepared_configuration_veriloga_runtimes(&state, &projection)
        .expect("compile exact configured source closure");

    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
        &state.library_manager,
        &projection,
    );
    let generated = crate::simulation::netlist_gen::generate_netlist_hierarchical(
        projection.root_schematic().expect("materialized root"),
        &[],
        &hierarchy,
    );
    assert!(generated.errors.is_empty(), "{:?}", generated.errors);

    assert_eq!(runtimes.len(), 1);
    let runtime = runtimes.iter().next().expect("prepared runtime");
    assert_eq!(runtime.source_digest(), expected_digest);
    assert_eq!(runtime.module_name(), "sealed_gain");
    assert_eq!(runtime.terminal_names().unwrap(), ["p", "n"]);
    assert!(runtime.source_key().starts_with("__rspice_project__/"));
    assert_eq!(
        generated
            .netlist
            .lines()
            .filter(|line| line.trim().eq_ignore_ascii_case(
                &crate::simulation::veriloga::project_veriloga_directive(
                    runtime.source_key(),
                    runtime.netlist_alias(),
                )
            ))
            .count(),
        1,
        "configured netlist must reference the exact prepared runtime once"
    );
    runtime
        .install()
        .expect("sealed configured runtime installs in the session cache");
}

#[test]
fn continuation_folding_cannot_hide_external_dependencies() {
    for source in [
        "deck\nBlookup out 0 V=table\n+ (\"C:/curves/transfer.tbl\")\n.end\n",
        "deck\nVstim in 0 PWL(\n+ FILE = \"C:/stimulus/wave.csv\"\n+ )\n.end\n",
        "deck\n.model cosim d_cosim (simulation\n* comment between continued records\n+ = \"C:/plugins/payload.dll\")\n.end\n",
        "deck\n.model source d_source (input_file\n+ = 'C:/stimulus/input.txt')\n.end\n",
    ] {
        let error = reject_deferred_external_sources(source)
            .expect_err("continued external dependency must fail closed");
        assert_eq!(error.stage(), PreparationStage::SourceChecks, "{source}");
        assert!(
            error.message().contains("unsealed external dependency"),
            "{source}: {error}"
        );
    }
}

#[test]
fn benign_continuations_remain_accepted() {
    for source in [
        "deck\nBinline out 0 V=table(\n+ V(in), 0, 0, 1, 1)\n.end\n",
        "deck\n.model diode D(\n+ IS=1e-12\n+ N=1.1)\n.end\n",
    ] {
        reject_deferred_external_sources(source)
            .unwrap_or_else(|error| panic!("benign continuation was rejected: {source}: {error}"));
    }
}

#[test]
fn every_materialized_corner_binding_is_audited_before_dispatch() {
    use crate::services::simulation_runner::{CornerModelBinding, CornerProcess, CornerRunConfig};

    let task = QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::Corner,
        config: None,
        spec_options: SpecExecutionOptions {
            corner: Some(CornerRunConfig {
                process_corners: vec![CornerProcess::FF],
                model_bindings: vec![CornerModelBinding {
                    process: CornerProcess::FF,
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

    let error = reject_deferred_corner_model_sources(&[task])
        .expect_err("non-reference corner source must be sealed");
    assert_eq!(error.stage(), PreparationStage::ModelBindings);
    assert!(error.message().contains("foundry.lib [FF]"));
    assert!(error.message().contains("unsealed external dependency"));
}

#[test]
fn active_batch_reentry_preserves_prepared_authorization_and_batch_metadata() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
        .expect("prepare replacement request");
    let prepared_digest = snapshot.digest();
    controller
        .authorize_snapshot(snapshot)
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
            .pending_prepared_run
            .as_ref()
            .map(|pending| pending.snapshot.digest()),
        Some(prepared_digest)
    );
}

#[test]
fn unpolled_completion_reentry_does_not_consume_or_replace_authorization() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
        .expect("prepare replacement request");
    let prepared_digest = snapshot.digest();
    controller
        .authorize_snapshot(snapshot)
        .expect("authorize replacement request");
    controller
        .runner
        .store_pending_result(Err(SimulationError::Aborted))
        .expect("seed finished, unpolled worker result");

    controller.start_authorized_snapshot(&mut state);

    assert!(state.simulation.runs.is_empty());
    assert_eq!(controller.total_analyses, 0);
    assert_eq!(
        controller
            .pending_prepared_run
            .as_ref()
            .map(|pending| pending.snapshot.digest()),
        Some(prepared_digest)
    );
    assert!(!controller.runner.can_accept_prepared_task());
}

#[test]
fn every_behavioral_file_lookup_alias_fails_closed() {
    let functions = [
        "table",
        "tablefile",
        "fasttable",
        "fasttablefile",
        "cubic",
        "cubicfile",
        "akima",
        "akimafile",
        "spline",
        "splinefile",
        "wodicka",
        "wodickafile",
        "bli",
        "blifile",
        "barycentric",
        "barycentricfile",
    ];

    for function in functions {
        let line = format!("Blookup out 0 V={function}(\"curve.dat\")");
        assert_eq!(
            deferred_external_source_reason(&line),
            Some("file-backed behavioral lookup"),
            "{function}"
        );
    }
}

#[test]
fn external_input_audit_ignores_comments_and_inline_data() {
    for line in [
        "* .include ignored.lib",
        "// .VERILOGA ignored.va",
        "R1 out 0 1k $ file=ignored.tbl",
        "R2 out 0 2k ; file=ignored.tbl",
        "R3 out 0 3k // file=ignored.tbl",
        ".data sweep_values",
        "+ 0 1 2 3",
        ".enddata",
        "Binline out 0 V=table(V(in), 0, 0, 1, 1)",
        ".param profile=1",
    ] {
        assert_eq!(deferred_external_source_reason(line), None, "{line}");
    }

    for line in [
        "Aco $G_DPWR [in] [out] null cosim simulation=provider",
        ".model source d_source (input_file='stimulus;production.txt')",
        ".model source d_source (input_file=\"stimulus\\\"production.txt\")",
    ] {
        assert!(
            deferred_external_source_reason(line).is_some(),
            "executable dependency must survive comment scanning: {line}"
        );
    }
}

#[test]
fn direct_manual_run_cannot_bypass_internal_prepare_and_permit_consumption() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let mut controller = SimulationController::new();

    controller.start_simulation(&mut state);

    assert!(controller.pending_prepared_run.is_none());
    assert_eq!(controller.total_analyses, 0);
    assert!(controller.cached_netlist.is_none());

    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");
    controller.start_simulation(&mut state);

    assert!(controller.pending_prepared_run.is_none());
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
    state.workspace.netlist_source = Some(source.to_owned());
    state.workspace.netlist_source_path = Some(origin);
    let mut controller = SimulationController::new();
    let metadata = controller
        .validate_manual_deck_document(&state)
        .expect("validate and retain first include closure");

    fs::write(&include, "R1 out 0 2k\n").expect("mutate include");
    let changed = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
        .expect("prepare changed include closure")
        .metadata();
    assert_ne!(metadata.source_digest, changed.source_digest);
    let error = controller
        .consume_snapshot_for_dispatch(&mut state)
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
    state.workspace.netlist_source = Some(source.to_owned());
    state.workspace.netlist_source_path = Some(origin);
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("validate the exact include closure");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&mut state)
        .expect("freeze the authorized dispatch");

    fs::write(&include, "R1 out 0 2k\n").expect("mutate source after dispatch");

    assert!(dispatch.executable_netlist().contains("R1 out 0 1k"));
    assert!(!dispatch.executable_netlist().contains("R1 out 0 2k"));
    assert!(!contains_external_include_directive(
        dispatch.executable_netlist()
    ));
    for task in dispatch.tasks() {
        assert_eq!(task.executable_netlist(), dispatch.executable_netlist());
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
        .consume_snapshot_for_dispatch(&mut state)
        .expect_err("changed accepted model bytes must block dispatch");
    assert_eq!(error.stage(), PreparationStage::ModelBindings);
    assert!(error.message().contains("changed"));

    fs::remove_dir_all(directory).expect("remove model fixture");
}

#[test]
fn rebuilt_prepared_snapshots_are_deterministic() {
    let state = runnable_state();
    let controller = SimulationController::new();
    let first = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect("baseline snapshot");
    let baseline_netlist = first.executable_netlist().to_owned();
    let baseline = first.metadata();

    // Dispatch rebuilds the snapshot and refuses to run when the digest moves,
    // so unchanged state has to seal to one identity every time. Anything
    // sampled from the clock or from a per-instance hash seed shows up here as
    // drift between two builds that saw exactly the same inputs.
    for attempt in 0..16 {
        let snapshot = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
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
        let mut state = runnable_state();
        let mut controller = SimulationController::new();
        let snapshot = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("project run seals signed PDK model sources");
        controller
            .authorize_snapshot(snapshot)
            .expect("authorize exact project snapshot");
        controller
            .consume_snapshot_for_dispatch(&mut state)
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
    state.workspace.netlist_source = Some(deck.to_owned());
    state
}

#[test]
fn a_project_without_a_technology_prepares_its_run_set_against_the_plain_model_library() {
    let state = technology_free_runnable_state();
    let mut controller = SimulationController::new();
    let metadata = controller
        .prepare_run_set_for_preflight(&state)
        .expect("a project owing nothing to a technology prepares its run set");
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .expect("the same technology-free contract rebuilds");
    let executable = snapshot.executable_netlist();
    assert!(!executable.contains("nmos_demo"), "{executable}");
    assert!(!executable.contains("demo180"), "{executable}");

    let attached = runnable_state();
    let with_technology = controller
        .build_prepared_snapshot(&attached, SimulationRunIntent::SimulateRunSet)
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
    let mut corner = crate::simulation::dialog::corner::CornerDialogState::default();
    for dimension in &mut corner.run_set.dimensions {
        if dimension.kind == crate::simulation::run_set::RunSetDimensionKind::Supply {
            dimension.source = format!(
                "{}VDD",
                crate::simulation::run_set::NETLIST_SUPPLY_SOURCE_PREFIX
            );
        }
    }
    let instance = insert_enabled_draft(
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
    assert!(error.message().contains(&instance.to_string()), "{error}");
}

#[test]
fn a_technology_binding_without_an_audit_receipt_blocks_preparation() {
    let mut state = runnable_state();
    // A copy keeps the binding and starts a fresh audit history, which is
    // exactly the shape the reattach contract refuses.
    state.workspace.project = state
        .workspace
        .project
        .fork_copy_at(PathBuf::from("forked").join("copy.rsproj"));
    assert!(state.workspace.project.technology_binding().is_some());
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
        // The embedded diode library carries this part.
        "manual model check\nV1 a 0 1\nD1 a 0 1N4148\n.op\n.end\n",
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
