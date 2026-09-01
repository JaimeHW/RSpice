//! Migration coverage for retained simulation-result history and corrupted payload recovery.

use super::*;

#[test]
fn project_simulation_results_omits_empty_history_after_cleared_runs() {
    let mut simulation = SimulationState::default();
    simulation.start_run();
    simulation.clear_runs();

    let results = ProjectSimulationResults::from_state(&simulation);

    assert!(results.is_empty());
}

#[test]
fn project_text_load_drops_invalid_simulation_results_without_rejecting_workspace() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut simulation = SimulationState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
    simulation.runs = vec![run];
    simulation.next_run_id = 1;
    simulation.active_run_idx = Some(0);
    simulation.active_analysis_idx = Some(0);
    let project = ProjectFile::new_with_simulation_results(
        workspace,
        libraries,
        ProjectSimulationResults::from_state(&simulation),
    );
    let mut value = serde_json::to_value(project).expect("project converts to JSON");
    value["simulation_results"]["schema_version"] = serde_json::Value::from(999);
    let json = serde_json::to_string_pretty(&value).expect("fixture serializes");

    let loaded = load_project_text(&json, None).expect("workspace still loads");

    assert!(loaded.simulation_results.is_empty());
    assert!(
        loaded
            .simulation_results_warning
            .as_deref()
            .unwrap_or_default()
            .contains("unsupported simulation results schema version")
    );
}

#[test]
fn project_load_clears_legacy_regression_baseline_after_result_migration() {
    let mut project = project_with_execution_context();
    let plan_id = project
        .execution_context
        .as_ref()
        .expect("execution context")
        .simulation_plan
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let mut run = SimulationRun::new(71);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "legacy AC"));
    seal_legacy_unattributed(&mut run);
    let baseline_id = run.run_id;
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 71;
    project.simulation_results = ProjectSimulationResults::from_state(&simulation);
    project
        .workspace
        .plan_data_mut(plan_id)
        .expect("active plan payload")
        .regression_baseline_run = Some(baseline_id);

    let json = serde_json::to_string_pretty(&project).expect("legacy baseline fixture");
    let loaded = load_project_text(&json, None).expect("project remains loadable");

    assert_eq!(loaded.simulation_results.runs.len(), 1);
    assert!(
        loaded
            .workspace
            .plan_data(plan_id)
            .expect("active plan payload")
            .regression_baseline_run
            .is_none()
    );
    assert!(
        loaded
            .simulation_results_warning
            .as_deref()
            .unwrap_or_default()
            .contains("not eligible")
    );
    serialize_project_file(&loaded).expect("cleaned project reserializes");
}

#[test]
fn project_load_authenticates_v11_noise_and_preserves_eligible_regression_baseline() {
    let mut project = project_with_execution_context();
    let (plan_id, source_id, source_revision, dependencies) = {
        let plan = project
            .execution_context
            .as_ref()
            .expect("execution context")
            .simulation_plan
            .stable_analysis_plan()
            .expect("stable plan");
        let source = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::Noise)
            .expect("noise fixture instance");
        (
            plan.id(),
            source.id(),
            source.modified_revision(),
            source
                .dependencies()
                .iter()
                .map(|dependency| dependency.target())
                .collect::<Vec<_>>(),
        )
    };
    let snapshot = ContentDigest::from_bytes([0xd1; 32]);
    let summary = NoiseSummary {
        rows: vec![NoiseContributorRow {
            device: "R1".to_owned(),
            mechanism: "thermal".to_owned(),
            power: 2.5e-12,
            share_pct: 100.0,
        }],
        total_rms: Some(1.25e-6),
        input_rms: None,
        band: (1.0, 1.0e6),
    };
    let mut run = SimulationRun::new(72);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
            .with_noise_summary(summary.clone())
            .with_provenance(
                AnalysisResultProvenance::new(source_id, source_revision, snapshot, dependencies)
                    .expect("noise provenance"),
            ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        project.workspace.project.revision(),
        ContentDigest::from_bytes([0xd2; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xd3; 32])),
        &[analysis_kind_tag_for_plan_kind(AnalysisKind::Noise)],
    );
    let baseline_id = run.run_id;
    let legacy_analysis_digest = run.analyses[0].legacy_v4_result_data_digest();
    let legacy_dataset_digest = run.legacy_v4_dataset_content_digest();
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 72;
    simulation.active_run_idx = Some(0);
    simulation.active_analysis_idx = Some(0);
    assert!(simulation.set_run_retention(baseline_id, RunRetention::GoldenBaseline));
    project.simulation_results = ProjectSimulationResults::from_state(&simulation);
    assert_eq!(
        project.simulation_results.runs[0].retention,
        RunRetention::GoldenBaseline,
        "the persisted fixture must retain the runtime baseline pin",
    );
    project
        .workspace
        .plan_data_mut(plan_id)
        .expect("active plan payload")
        .regression_baseline_run = Some(baseline_id);

    let current_json = serialize_project_file(&project).expect("current project serializes");
    let mut v11: serde_json::Value = serde_json::from_str(&current_json).expect("project JSON");
    v11["simulation_results"]["schema_version"] =
        serde_json::json!(TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION);
    v11["simulation_results"]["runs"][0]["analyses"][0]["result_data_digest"] =
        serde_json::to_value(legacy_analysis_digest).expect("legacy analysis digest");
    v11["simulation_results"]["runs"][0]["dataset_content_digest"] =
        serde_json::to_value(legacy_dataset_digest).expect("legacy dataset digest");
    v11["simulation_results"]["runs"][0]["analyses"][0]["noise_summary"]
        .as_object_mut()
        .expect("noise summary object")
        .remove("input_rms");
    v11["simulation_results"]["runs"][0]
        .as_object_mut()
        .expect("persisted run object")
        .remove("retention");
    clear_v14_specification_fields_json(&mut v11["simulation_results"]);

    let loaded = load_project_text(&v11.to_string(), None)
        .expect("authentic schema-v11 project remains loadable");

    assert_eq!(
        loaded.simulation_results.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    assert_eq!(loaded.simulation_results.runs.len(), 1);
    assert!(loaded.simulation_results_warning.is_none());
    assert_eq!(
        loaded
            .workspace
            .plan_data(plan_id)
            .expect("active plan payload")
            .regression_baseline_run,
        Some(baseline_id)
    );
    let restored = loaded.simulation_results.runs[0]
        .clone()
        .into_run()
        .expect("migrated result restores");
    assert!(
        restored.retention().is_pinned(),
        "a restored regression baseline must be repaired into a non-pruneable retention class"
    );
    assert_eq!(restored.analyses[0].noise_summary, Some(summary));
    assert_ne!(
        restored.analyses[0].result_data_digest(),
        legacy_analysis_digest,
        "v11 evidence must be resealed in the current digest domain"
    );
    serialize_project_file(&loaded).expect("migrated project reserializes");

    let mut tampered_v11 = v11;
    tampered_v11["simulation_results"]["runs"][0]["analyses"][0]["noise_summary"]["total_rms"] =
        serde_json::json!(1.250_000_000_000_000_3e-6_f64);
    let rejected = load_project_text(&tampered_v11.to_string(), None)
        .expect("a bad result digest must not reject unrelated project documents");
    assert!(rejected.simulation_results.is_empty());
    assert!(
        rejected
            .workspace
            .plan_data(plan_id)
            .expect("active plan payload")
            .regression_baseline_run
            .is_none()
    );
    assert!(
        rejected
            .simulation_results_warning
            .as_deref()
            .unwrap_or_default()
            .contains("schema-v11 analysis 1 result data digest does not match retained content")
    );
}

#[test]
fn project_load_clears_dangling_regression_baseline_without_rejecting_project() {
    let mut project = project_with_execution_context();
    let plan_id = project
        .execution_context
        .as_ref()
        .expect("execution context")
        .simulation_plan
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    project
        .workspace
        .plan_data_mut(plan_id)
        .expect("active plan payload")
        .regression_baseline_run = Some(crate::product::RunId::new());

    let json = serde_json::to_string_pretty(&project).expect("dangling baseline fixture");
    let loaded = load_project_text(&json, None).expect("project remains loadable");

    assert!(loaded.simulation_results.is_empty());
    assert!(
        loaded
            .workspace
            .plan_data(plan_id)
            .expect("active plan payload")
            .regression_baseline_run
            .is_none()
    );
    assert!(
        loaded
            .simulation_results_warning
            .as_deref()
            .unwrap_or_default()
            .contains("absent from retained result history")
    );
    serialize_project_file(&loaded).expect("cleaned project reserializes");
}

#[test]
fn project_text_load_drops_unknown_analysis_type_results_without_parse_failure() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut simulation = SimulationState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC"));
    seal_legacy_unattributed(&mut run);
    simulation.runs = vec![run];
    simulation.next_run_id = 1;
    simulation.active_run_idx = Some(0);
    simulation.active_analysis_idx = Some(0);
    let project = ProjectFile::new_with_simulation_results(
        workspace,
        libraries,
        ProjectSimulationResults::from_state(&simulation),
    );
    let json = serialize_project_file(&project)
        .expect("project serializes")
        .replace(
            "\"analysis_type\": \"Ac\"",
            "\"analysis_type\": \"FutureAnalysis\"",
        );

    let loaded = load_project_text(&json, None).expect("workspace still loads");

    assert!(loaded.simulation_results.is_empty());
    assert!(
        loaded
            .simulation_results_warning
            .as_deref()
            .unwrap_or_default()
            .contains("unknown analysis type")
    );
}

#[test]
fn project_results_restore_rejects_invalid_overlay_references() {
    let mut run_one = SimulationRun::new(1);
    let mut run_two = SimulationRun::new(2);
    seal_legacy_unattributed(&mut run_one);
    seal_legacy_unattributed(&mut run_two);
    let results = ProjectSimulationResults {
        retained_dataset_limit: None,
        schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
        runs: vec![
            ProjectSimulationRun::from(&run_one),
            ProjectSimulationRun::from(&run_two),
        ],
        next_run_id: 2,
        active_run_stable_id: Some(run_one.run_id),
        active_dataset_id: Some(run_one.dataset_id),
        active_analysis_sequence: None,
        overlay_dataset_ids: vec![
            run_two.dataset_id,
            run_two.dataset_id,
            run_one.dataset_id,
            DatasetId::new(),
        ],
        ..ProjectSimulationResults::default()
    };

    let error = results
        .into_simulation_state()
        .expect_err("invalid overlay references fail closed");

    assert!(error.contains("duplicate overlay dataset id"));
}

#[test]
fn project_results_validation_rejects_duplicate_run_ids() {
    let mut run_one = SimulationRun::new(1);
    let mut run_duplicate = SimulationRun::new(1);
    seal_legacy_unattributed(&mut run_one);
    seal_legacy_unattributed(&mut run_duplicate);
    let results = ProjectSimulationResults {
        retained_dataset_limit: None,
        schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
        runs: vec![
            ProjectSimulationRun::from(&run_one),
            ProjectSimulationRun::from(&run_duplicate),
        ],
        next_run_id: 1,
        active_run_stable_id: Some(run_one.run_id),
        active_dataset_id: Some(run_one.dataset_id),
        active_analysis_sequence: None,
        overlay_dataset_ids: Vec::new(),
        ..ProjectSimulationResults::default()
    };

    let error = results.validate().expect_err("duplicate run ids fail");

    assert!(error.contains("duplicate simulation run id 1"));
}

#[test]
fn project_results_v2_requires_unique_stable_run_and_dataset_ids() {
    let mut run_one = SimulationRun::new(1);
    run_one.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN one"));
    let mut run_two = SimulationRun::new(2);
    run_two.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN two"));
    seal_legacy_unattributed(&mut run_one);
    seal_legacy_unattributed(&mut run_two);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run_one, run_two];
    simulation.next_run_id = 2;

    let baseline = ProjectSimulationResults::from_state(&simulation);

    let mut missing_run_identity = baseline.clone();
    missing_run_identity.runs[0].run_id = None;
    let error = missing_run_identity
        .validate()
        .expect_err("schema v2 must not regenerate a missing run id");
    assert!(error.contains("run_id is required"));

    let mut missing_dataset_identity = baseline.clone();
    missing_dataset_identity.runs[0].dataset_id = None;
    let error = missing_dataset_identity
        .validate()
        .expect_err("schema v2 must not regenerate a missing dataset id");
    assert!(error.contains("dataset_id is required"));

    let mut duplicate_run_identity = baseline.clone();
    duplicate_run_identity.runs[1].run_id = duplicate_run_identity.runs[0].run_id;
    let error = duplicate_run_identity
        .validate()
        .expect_err("stable run ids are globally unique");
    assert!(error.contains("duplicate stable simulation run id"));

    let mut duplicate_dataset_identity = baseline;
    duplicate_dataset_identity.runs[1].dataset_id = duplicate_dataset_identity.runs[0].dataset_id;
    let error = duplicate_dataset_identity
        .validate()
        .expect_err("dataset ids are globally unique");
    assert!(error.contains("duplicate immutable dataset id"));
}

#[test]
fn projects_written_before_golden_baselines_restore_every_run_pruneable() {
    let mut run_one = SimulationRun::new(1);
    run_one.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN one"));
    let mut run_two = SimulationRun::new(2);
    run_two.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC two"));
    seal_legacy_unattributed(&mut run_one);
    seal_legacy_unattributed(&mut run_two);
    let baseline_run_id = run_one.run_id;
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run_two, run_one];
    simulation.next_run_id = 2;

    let historical = serde_json::to_string(&ProjectSimulationResults::from_state(&simulation))
        .expect("results serialize");
    assert!(
        !historical.contains("retention"),
        "a project with no baselines writes exactly what it wrote before they existed"
    );
    let restored = serde_json::from_str::<ProjectSimulationResults>(&historical)
        .expect("historical results decode")
        .into_simulation_state()
        .expect("historical results restore");
    assert_eq!(restored.pinned_run_count(), 0);
    assert!(
        restored
            .runs
            .iter()
            .all(|run| run.retention() == RunRetention::Pruneable)
    );

    assert!(simulation.set_run_retention(baseline_run_id, RunRetention::GoldenBaseline));
    let current = serde_json::to_string(&ProjectSimulationResults::from_state(&simulation))
        .expect("results with a baseline serialize");
    let reloaded = serde_json::from_str::<ProjectSimulationResults>(&current)
        .expect("results with a baseline decode")
        .into_simulation_state()
        .expect("results with a baseline restore");
    assert_eq!(reloaded.pinned_run_count(), 1);
    assert!(
        reloaded
            .run_by_stable_id(baseline_run_id)
            .expect("the baseline is retained")
            .retention()
            .is_pinned()
    );
}

#[test]
fn project_results_v2_rejects_cross_bound_selection_and_active_overlay() {
    let mut run_one = SimulationRun::new(1);
    run_one.add_analysis(AnalysisResult::new(3, AnalysisType::Transient, "TRAN one"));
    let mut run_two = SimulationRun::new(2);
    run_two.add_analysis(AnalysisResult::new(8, AnalysisType::Ac, "AC two"));
    seal_legacy_unattributed(&mut run_one);
    seal_legacy_unattributed(&mut run_two);
    let run_one_dataset_id = run_one.dataset_id;
    let run_two_dataset_id = run_two.dataset_id;
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run_one, run_two];
    simulation.next_run_id = 2;
    simulation.active_run_idx = Some(0);
    simulation.active_analysis_idx = Some(0);
    let baseline = ProjectSimulationResults::from_state(&simulation);

    let mut cross_bound = baseline.clone();
    cross_bound.active_dataset_id = Some(run_two_dataset_id);
    let error = cross_bound
        .validate()
        .expect_err("a dataset cannot be rebound to a different active run");
    assert!(error.contains("does not belong to active run"));

    let mut missing_analysis = baseline.clone();
    missing_analysis.active_analysis_sequence = Some(999);
    let error = missing_analysis
        .validate()
        .expect_err("the selected analysis must belong to the active dataset");
    assert!(error.contains("does not exist in active dataset"));

    let mut active_overlay = baseline;
    active_overlay.overlay_dataset_ids = vec![run_one_dataset_id];
    let error = active_overlay
        .validate()
        .expect_err("the active dataset cannot also be an overlay");
    assert!(error.contains("cannot also be an overlay"));
}

#[test]
fn project_text_migrates_v1_result_sequences_once_to_stable_identities() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectFile::new(workspace, libraries);
    let project_value = serde_json::to_value(project).expect("project converts to JSON");

    // A schema-v1 golden record is authored from the fields that actually
    // existed in that era.  Do not downgrade a current record by deleting a
    // hand-maintained subset: that silently leaves newly introduced evidence
    // (for example measurement-verification fields) in an anachronistic file.
    let versioned_v1_results = serde_json::json!({
        "schema_version": LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION,
        "runs": [
            {
                "id": 1,
                "label": "Run 1 (v1 golden)",
                "timestamp": 11.0,
                "analyses": [{
                    "id": 4,
                    "analysis_type": "Transient",
                    "label": "TRAN legacy one",
                    "timestamp": 11.25,
                    "waveforms": [{
                        "name": "V(out)",
                        "x": [0.0, 1.0e-6],
                        "y": [0.0, 1.0],
                        "color": "#00aaff",
                        "visible": true
                    }],
                    "measurements": [],
                    "success": true
                }],
                "elapsed_time": 0.01,
                "success": true
            },
            {
                "id": 2,
                "label": "Run 2 (v1 golden)",
                "timestamp": 12.0,
                "analyses": [{
                    "id": 9,
                    "analysis_type": "Ac",
                    "label": "AC legacy two",
                    "timestamp": 12.25,
                    "waveforms": [],
                    "measurements": [],
                    "success": true
                }],
                "elapsed_time": 0.02,
                "success": true
            }
        ],
        "next_run_id": 2,
        "active_run_id": 2,
        "active_analysis_id": 9,
        "overlay_run_ids": [1]
    });

    let mut migrated_matrix = Vec::new();
    for versioned in [true, false] {
        let mut value = project_value.clone();
        let mut results = versioned_v1_results.clone();
        if !versioned {
            results
                .as_object_mut()
                .expect("v1 golden result object")
                .remove("schema_version");
        }
        value["simulation_results"] = results;
        let json = serde_json::to_string_pretty(&value).expect("v1 golden fixture serializes");
        migrated_matrix.push(load_project_text(&json, None).unwrap_or_else(|error| {
            panic!(
                "{} v1 golden project migrates: {error}",
                if versioned {
                    "versioned"
                } else {
                    "unversioned"
                }
            )
        }));
    }
    assert_eq!(
        migrated_matrix[0].simulation_results, migrated_matrix[1].simulation_results,
        "an omitted schema tag is exactly the schema-v1 wire contract"
    );
    let migrated = &migrated_matrix[0];

    assert!(migrated.simulation_results_warning.is_none());
    assert_eq!(
        migrated.simulation_results.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    assert!(
        migrated
            .simulation_results
            .runs
            .iter()
            .all(|run| run.run_id.is_some() && run.dataset_id.is_some())
    );
    let active_run_id = migrated.simulation_results.runs[1]
        .run_id
        .expect("migrated active run id");
    let active_dataset_id = migrated.simulation_results.runs[1]
        .dataset_id
        .expect("migrated active dataset id");
    let migrated_overlay_id = migrated.simulation_results.runs[0]
        .dataset_id
        .expect("migrated overlay id");
    assert_eq!(
        migrated.simulation_results.active_run_stable_id,
        Some(active_run_id)
    );
    assert_eq!(
        migrated.simulation_results.active_dataset_id,
        Some(active_dataset_id)
    );
    assert_eq!(
        migrated.simulation_results.active_analysis_sequence,
        Some(9)
    );
    assert_eq!(
        migrated.simulation_results.overlay_dataset_ids,
        vec![migrated_overlay_id]
    );
    assert!(migrated.simulation_results.active_run_id.is_none());
    assert!(migrated.simulation_results.active_analysis_id.is_none());
    assert!(migrated.simulation_results.overlay_run_ids.is_empty());

    let current_json = serialize_project_file(migrated).expect("migration persists");
    let reloaded = load_project_text(&current_json, None).expect("migrated project reloads");
    assert_eq!(
        reloaded.simulation_results.active_run_stable_id,
        Some(active_run_id)
    );
    assert_eq!(
        reloaded.simulation_results.active_dataset_id,
        Some(active_dataset_id)
    );
    assert_eq!(
        reloaded.simulation_results.active_analysis_sequence,
        Some(9)
    );
    assert_eq!(
        reloaded.simulation_results.overlay_dataset_ids,
        vec![migrated_overlay_id]
    );
}

#[test]
fn project_results_validation_rejects_duplicate_waveform_names_in_analysis() {
    let run_id = RunId::new();
    let dataset_id = DatasetId::new();
    let results = ProjectSimulationResults {
        retained_dataset_limit: None,
        schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
        runs: vec![ProjectSimulationRun {
            job_id: None,
            run_id: Some(run_id),
            dataset_id: Some(dataset_id),
            execution_target: None,
            lifecycle: Some(SimulationRunLifecycle::LegacyUnknown),
            id: 1,
            label: "Run 1".to_string(),
            timestamp: 1.0,
            analyses: vec![ProjectAnalysisResult {
                id: 1,
                analysis_type: "Transient".to_string(),
                label: "TRAN".to_string(),
                timestamp: 1.0,
                result_data_digest: PersistedField::Missing,
                waveforms: vec![
                    ProjectWaveformData {
                        name: "V(out)".to_string(),
                        x: vec![0.0],
                        y: vec![1.0],
                        color: "#00aaff".to_string(),
                        visible: true,
                        unit: None,
                        complex: None,
                    },
                    ProjectWaveformData {
                        name: "V(out)".to_string(),
                        x: vec![0.0],
                        y: vec![2.0],
                        color: "#ffaa00".to_string(),
                        visible: true,
                        unit: None,
                        complex: None,
                    },
                ],
                dc_op: None,
                device_op: None,
                noise_summary: None,
                family_metadata: None,
                result_payload: PersistedField::Missing,
                measurements: Vec::new(),
                saved_output_receipts: Vec::new(),
                success: true,
                error_message: None,
                failure_attribution: None,
                provenance: None,
            }],
            dataset_content_digest: PersistedField::Missing,
            provenance_mode: PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed),
            prepared_receipt: PersistedField::Missing,
            campaign_membership: None,
            retention: RunRetention::Pruneable,
            elapsed_time: 0.1,
            success: true,
            specification_verdicts: None,
        }],
        next_run_id: 1,
        active_run_stable_id: Some(run_id),
        active_dataset_id: Some(dataset_id),
        active_analysis_sequence: Some(1),
        overlay_dataset_ids: Vec::new(),
        ..ProjectSimulationResults::default()
    };

    let error = results
        .validate()
        .expect_err("duplicate waveform names in an analysis fail");

    assert!(error.contains("runs[0].analyses[0]"));
    assert!(error.contains("duplicate waveform name 'V(out)'"));
}

#[test]
fn project_results_validation_rejects_non_monotonic_waveform_x() {
    let run_id = RunId::new();
    let dataset_id = DatasetId::new();
    let results = ProjectSimulationResults {
        retained_dataset_limit: None,
        schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
        runs: vec![ProjectSimulationRun {
            job_id: None,
            run_id: Some(run_id),
            dataset_id: Some(dataset_id),
            execution_target: None,
            lifecycle: Some(SimulationRunLifecycle::LegacyUnknown),
            id: 1,
            label: "Run 1".to_string(),
            timestamp: 1.0,
            analyses: vec![ProjectAnalysisResult {
                id: 1,
                analysis_type: "Transient".to_string(),
                label: "TRAN".to_string(),
                timestamp: 1.0,
                result_data_digest: PersistedField::Missing,
                waveforms: vec![ProjectWaveformData {
                    name: "V(out)".to_string(),
                    x: vec![0.0, 2.0, 1.0, 3.0],
                    y: vec![0.0, 1.0, 2.0, 3.0],
                    color: "#00aaff".to_string(),
                    visible: true,
                    unit: None,
                    complex: None,
                }],
                dc_op: None,
                device_op: None,
                noise_summary: None,
                family_metadata: None,
                result_payload: PersistedField::Missing,
                measurements: Vec::new(),
                saved_output_receipts: Vec::new(),
                success: true,
                error_message: None,
                failure_attribution: None,
                provenance: None,
            }],
            dataset_content_digest: PersistedField::Missing,
            provenance_mode: PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed),
            prepared_receipt: PersistedField::Missing,
            campaign_membership: None,
            retention: RunRetention::Pruneable,
            elapsed_time: 0.1,
            success: true,
            specification_verdicts: None,
        }],
        next_run_id: 1,
        active_run_stable_id: Some(run_id),
        active_dataset_id: Some(dataset_id),
        active_analysis_sequence: Some(1),
        overlay_dataset_ids: Vec::new(),
        ..ProjectSimulationResults::default()
    };

    let error = results
        .validate()
        .expect_err("non-monotonic waveform x data must fail");

    assert!(error.contains("runs[0].analyses[0].waveforms[0].x"));
    assert!(error.contains("monotonic"));
}

#[test]
fn legacy_noise_total_rms_migrates_losslessly_to_optional_evidence() {
    let legacy = r#"{"rows":[],"total_rms":1.25e-6,"band":[1.0,1000.0]}"#;
    let restored: ProjectNoiseSummary =
        serde_json::from_str(legacy).expect("legacy noise summary decodes");
    assert_eq!(restored.total_rms, Some(1.25e-6));
    assert_eq!(restored.input_rms, None);
    assert_eq!(restored.into_noise_summary().total_rms, Some(1.25e-6));
}

#[test]
fn project_results_preserve_core_noise_mechanism_labels() {
    let run_id = RunId::new();
    let dataset_id = DatasetId::new();
    let mut results = ProjectSimulationResults {
        retained_dataset_limit: None,
        schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
        runs: vec![ProjectSimulationRun {
            job_id: None,
            run_id: Some(run_id),
            dataset_id: Some(dataset_id),
            execution_target: None,
            lifecycle: Some(SimulationRunLifecycle::LegacyUnknown),
            id: 1,
            label: "Run 1".to_string(),
            timestamp: 1.0,
            analyses: vec![ProjectAnalysisResult {
                id: 1,
                analysis_type: "Noise".to_string(),
                label: "NOISE".to_string(),
                timestamp: 1.0,
                result_data_digest: PersistedField::Missing,
                waveforms: Vec::new(),
                dc_op: None,
                device_op: None,
                noise_summary: Some(ProjectNoiseSummary {
                    rows: vec![
                        ProjectNoiseContributorRow {
                            device: "BNOISE1".to_string(),
                            mechanism: "white".to_string(),
                            power: 1.0e-18,
                            share_pct: 60.0,
                        },
                        ProjectNoiseContributorRow {
                            device: "ATABLE1".to_string(),
                            mechanism: "table".to_string(),
                            power: 2.0e-18,
                            share_pct: 40.0,
                        },
                    ],
                    total_rms: Some(1.0e-6),
                    input_rms: None,
                    band: (1.0, 1.0e6),
                }),
                family_metadata: None,
                result_payload: PersistedField::Missing,
                measurements: Vec::new(),
                saved_output_receipts: Vec::new(),
                success: true,
                error_message: None,
                failure_attribution: None,
                provenance: None,
            }],
            dataset_content_digest: PersistedField::Missing,
            provenance_mode: PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed),
            prepared_receipt: PersistedField::Missing,
            campaign_membership: None,
            retention: RunRetention::Pruneable,
            elapsed_time: 0.1,
            success: true,
            specification_verdicts: None,
        }],
        next_run_id: 1,
        active_run_stable_id: Some(run_id),
        active_dataset_id: Some(dataset_id),
        active_analysis_sequence: Some(1),
        overlay_dataset_ids: Vec::new(),
        ..ProjectSimulationResults::default()
    };

    seal_project_result_digests(&mut results.runs[0]).expect("fixture digests seal");

    results.validate().expect("core noise labels are valid");

    let restored = results
        .into_simulation_state()
        .expect("valid result history restores");
    let summary = restored
        .active_analysis()
        .and_then(|analysis| analysis.noise_summary.as_ref())
        .expect("noise summary restores");

    assert_eq!(summary.rows[0].mechanism, "white");
    assert_eq!(summary.rows[1].mechanism, "table");
}

#[test]
fn project_text_load_updates_source_path_without_renaming_identity() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    workspace
        .project
        .set_path(PathBuf::from("stale-native-path.rspiceproj"));
    let project = ProjectFile::new(workspace, libraries);
    let json = serialize_project_file(&project).expect("project serializes");

    let loaded = load_project_text(&json, Some(Path::new("browser-import.rspiceproj")))
        .expect("project text loads");

    assert_eq!(
        loaded.workspace.project.path.as_deref(),
        Some(Path::new("browser-import.rspiceproj"))
    );
    assert_eq!(
        loaded.workspace.project.display_name(),
        "stale-native-path",
        "moving a project file must not silently rename its logical identity"
    );
}

#[test]
fn project_text_load_without_source_path_clears_stale_file_identity() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    workspace
        .project
        .set_path(PathBuf::from("stale-native-path.rspiceproj"));
    let project = ProjectFile::new(workspace, libraries);
    let json = serialize_project_file(&project).expect("project serializes");

    let loaded = load_project_text(&json, None).expect("project text loads");

    assert!(loaded.workspace.project.path.is_none());
    assert_eq!(loaded.workspace.project.display_name(), "stale-native-path");
}

#[test]
fn legacy_project_migration_assigns_stable_identity_metadata() {
    let project = project_with_execution_context();
    let mut value = serde_json::to_value(project).expect("project converts to JSON");
    let descriptor = value["workspace"]["project"]
        .as_object_mut()
        .expect("project descriptor is an object");
    descriptor.remove("id");
    descriptor.remove("schema_version");
    descriptor.remove("revision");
    value["workspace"]
        .as_object_mut()
        .expect("workspace is an object")
        .remove("simulation_plan_payloads");
    value["execution_context"]["schema_version"] = serde_json::Value::from(3_u32);
    let legacy_plan = value["execution_context"]["simulation_plan"]
        .as_object_mut()
        .expect("simulation plan is an object");
    legacy_plan.remove("analysis_plan");
    legacy_plan.insert("enabled".to_owned(), serde_json::json!([1]));
    legacy_plan.insert("analysis_order".to_owned(), serde_json::json!([1]));
    let legacy = serde_json::to_string_pretty(&value).expect("legacy fixture serializes");

    assert_eq!(
        project_text_load_route(&legacy).expect("legacy route probes"),
        ProjectTextLoadRoute::LegacyProjectIdInjection
    );

    let migrated = load_project_text(&legacy, None).expect("legacy project migrates");
    let replay = load_project_text(&legacy, None).expect("identical legacy bytes migrate again");

    assert!(!migrated.workspace.project.id().as_uuid().is_nil());
    assert_eq!(
        migrated.workspace.project.schema_version(),
        crate::state::PROJECT_DESCRIPTOR_SCHEMA_VERSION
    );
    assert_eq!(migrated.workspace.project.revision().get(), 1);
    assert_eq!(
        migrated.workspace.project.id(),
        replay.workspace.project.id()
    );
    let migrated_plan = migrated
        .execution_context
        .as_ref()
        .expect("migrated execution context")
        .simulation_plan
        .stable_analysis_plan()
        .expect("migrated stable plan");
    let replay_plan = replay
        .execution_context
        .as_ref()
        .expect("replayed execution context")
        .simulation_plan
        .stable_analysis_plan()
        .expect("replayed stable plan");
    assert_eq!(migrated_plan.id(), replay_plan.id());
    assert_eq!(
        migrated_plan
            .instances()
            .iter()
            .map(|instance| instance.id())
            .collect::<Vec<_>>(),
        replay_plan
            .instances()
            .iter()
            .map(|instance| instance.id())
            .collect::<Vec<_>>()
    );

    let migrated_json =
        serialize_project_file(&migrated).expect("migrated identity persists on save");
    assert_eq!(
        project_text_load_route(&migrated_json).expect("migrated route probes"),
        ProjectTextLoadRoute::Direct
    );
    let reloaded = load_project_text(&migrated_json, None).expect("migrated project reloads");
    assert_eq!(
        reloaded.workspace.project.id(),
        migrated.workspace.project.id()
    );
    assert_eq!(
        reloaded.workspace.project.revision(),
        migrated.workspace.project.revision()
    );
}

#[test]
fn project_load_rejects_unsupported_descriptor_schema() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectFile::new(workspace, libraries);
    let mut value = serde_json::to_value(project).expect("project converts to JSON");
    value["workspace"]["project"]["schema_version"] =
        serde_json::Value::from(crate::state::PROJECT_DESCRIPTOR_SCHEMA_VERSION + 1);
    let contents = serde_json::to_string_pretty(&value).expect("fixture serializes");

    let error = load_project_text(&contents, None)
        .expect_err("future project descriptor schema must fail closed");

    assert!(matches!(error, ProjectIoError::InvalidData(_)));
    assert!(error.to_string().contains("project schema version"));
}

#[test]
fn project_load_rejects_missing_or_null_identity_on_a_versioned_descriptor() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectFile::new(workspace, libraries);
    let value = serde_json::to_value(project).expect("project converts to JSON");

    let mut missing = value.clone();
    missing["workspace"]["project"]
        .as_object_mut()
        .expect("project descriptor object")
        .remove("id");
    let missing_error = load_project_text(&missing.to_string(), None)
        .expect_err("versioned descriptor cannot lose its identity");
    assert!(
        missing_error
            .to_string()
            .contains("missing its stable identity")
    );

    let mut null = value.clone();
    null["workspace"]["project"]["id"] = serde_json::Value::Null;
    let null_error = load_project_text(&null.to_string(), None)
        .expect_err("explicit null identity is never a legacy migration");
    assert!(
        null_error
            .to_string()
            .contains("must not be explicitly null")
    );

    let mut unversioned_null_id = value.clone();
    unversioned_null_id["workspace"]["project"]
        .as_object_mut()
        .expect("project descriptor object")
        .remove("schema_version");
    unversioned_null_id["workspace"]["project"]["id"] = serde_json::Value::Null;
    let unversioned_null_error = load_project_text(&unversioned_null_id.to_string(), None)
        .expect_err("unversioned explicit null identity is not legacy absence");
    assert!(
        unversioned_null_error
            .to_string()
            .contains("must not be explicitly null")
    );

    let mut null_schema = value;
    null_schema["workspace"]["project"]["schema_version"] = serde_json::Value::Null;
    let null_schema_error = load_project_text(&null_schema.to_string(), None)
        .expect_err("explicit null schema cannot trigger legacy migration");
    assert!(
        null_schema_error
            .to_string()
            .contains("schema version must not be explicitly null")
    );
}

#[test]
fn project_text_load_rejects_missing_active_schematic_buffer() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut project = ProjectFile::new(workspace, libraries);
    let active_key = project.workspace.active_key();
    project.workspace.schematic_buffers.remove(&active_key);
    let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

    let err =
        load_project_text(&json, None).expect_err("missing active schematic buffer must fail load");

    assert!(matches!(err, ProjectIoError::InvalidData(_)));
    assert!(
        err.to_string().contains(&active_key),
        "error should name the missing buffer key"
    );
}

#[test]
fn project_text_load_rejects_workspace_references_missing_from_libraries() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let ghost = CellViewRef::new("ghost", "amp", "schematic");
    let ghost_key = ghost.key();
    let schematic = workspace
        .schematic_buffers
        .values()
        .next()
        .cloned()
        .expect("default project has a schematic buffer");
    workspace.active_view = ghost.clone();
    workspace.open_views = vec![OpenCellView::new(ghost.clone(), ViewType::Schematic)];
    workspace.hierarchy_stack = vec![ghost.clone()];
    workspace.schematic_buffers.clear();
    workspace
        .schematic_buffers
        .insert(ghost_key.clone(), schematic);
    let project = ProjectFile::new(workspace, libraries);
    let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

    let err = load_project_text(&json, None)
        .expect_err("workspace references absent from libraries must fail load");

    assert!(matches!(err, ProjectIoError::InvalidData(_)));
    assert!(
        err.to_string().contains(&ghost_key),
        "error should name the missing workspace reference"
    );
}

#[test]
fn project_text_load_rejects_workspace_view_type_mismatch() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let active = workspace.active_view.clone();
    workspace.open_views = vec![OpenCellView::new(active.clone(), ViewType::Symbol)];
    let active_key = active.key();
    let project = ProjectFile::new(workspace, libraries);
    let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

    let err =
        load_project_text(&json, None).expect_err("workspace view type mismatch must fail load");

    assert!(matches!(err, ProjectIoError::InvalidData(_)));
    assert!(
        err.to_string().contains(&active_key) && err.to_string().contains("view type"),
        "error should name the mismatched view reference"
    );
}

#[test]
fn project_text_load_rejects_active_view_missing_from_open_views() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    add_top_symbol_view(&mut libraries);
    let active = CellViewRef::new("user", "top", "symbol");
    workspace.active_view = active.clone();
    workspace.hierarchy_stack = vec![active.clone()];
    let project = ProjectFile::new(workspace, libraries);
    let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

    let err = load_project_text(&json, None)
        .expect_err("active view absent from open views must fail load");

    assert!(matches!(err, ProjectIoError::InvalidData(_)));
    assert!(
        err.to_string().contains(&active.key()) && err.to_string().contains("open_views"),
        "error should name the missing active open view"
    );
}

#[test]
fn project_text_load_rejects_library_tree_key_name_mismatch() {
    type ProjectJsonMutation = fn(&mut serde_json::Value, &CellViewRef);

    let cases: [(&str, ProjectJsonMutation); 3] = [
        (
            "library",
            |value: &mut serde_json::Value, active: &CellViewRef| {
                value["libraries"]["libraries"][&active.library]["name"] =
                    serde_json::Value::String("ghost".to_string());
            },
        ),
        (
            "cell",
            |value: &mut serde_json::Value, active: &CellViewRef| {
                value["libraries"]["libraries"][&active.library]["cells"][&active.cell]["name"] =
                    serde_json::Value::String("ghost".to_string());
            },
        ),
        (
            "view",
            |value: &mut serde_json::Value, active: &CellViewRef| {
                value["libraries"]["libraries"][&active.library]["cells"][&active.cell]["views"]
                    [&active.view]["name"] = serde_json::Value::String("ghost".to_string());
            },
        ),
    ];

    for (case, mutate) in cases {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let active = workspace.active_view.clone();
        let project = ProjectFile::new(workspace, libraries);
        let mut value = serde_json::to_value(&project).expect("project converts to json value");
        mutate(&mut value, &active);
        let json = serde_json::to_string_pretty(&value).expect("corrupt fixture serializes");

        let err = load_project_text(&json, None)
            .expect_err("library tree key/name mismatches must fail load");

        assert!(matches!(err, ProjectIoError::InvalidData(_)));
        assert!(
            err.to_string().contains(case) && err.to_string().contains("map key"),
            "unexpected {case} mismatch error: {err}"
        );
    }
}

#[test]
fn project_load_rejects_unicode_canonical_library_cell_and_view_collisions() {
    for scope in ["library", "cell", "view"] {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        match scope {
            "library" => {
                libraries.add_library(crate::state::Library::new("\u{c9}tage"));
                libraries.add_library(crate::state::Library::new("\u{e9}TAGE"));
            }
            "cell" => {
                let library = libraries.get_library_mut("user").expect("user library");
                library.add_cell(crate::state::Cell::new("\u{c9}tage"));
                library.add_cell(crate::state::Cell::new("\u{e9}TAGE"));
            }
            "view" => {
                let cell = libraries
                    .get_library_mut("user")
                    .expect("user library")
                    .get_cell_mut("top")
                    .expect("top cell");
                cell.add_view(crate::state::View::new("Mod\u{e8}le", ViewType::Symbol));
                cell.add_view(crate::state::View::new("MOD\u{c8}LE", ViewType::Symbol));
            }
            _ => unreachable!(),
        }

        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("fixture serializes");
        let error = load_project_text(&json, None)
            .expect_err("canonical library identities must be unique");

        assert!(
            error.to_string().contains("canonical")
                && error.to_string().contains(scope)
                && error.to_string().contains("collision"),
            "unexpected {scope} collision error: {error}"
        );
    }
}

#[test]
fn project_load_rejects_slash_alias_triples_before_key_lookup() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);

    let mut first_cell = crate::state::Cell::new("b");
    first_cell.add_view(crate::state::View::new("c/d", ViewType::Symbol));
    let mut first_library = crate::state::Library::new("a");
    first_library.add_cell(first_cell);
    libraries.add_library(first_library);

    let mut second_cell = crate::state::Cell::new("c");
    second_cell.add_view(crate::state::View::new("d", ViewType::Symbol));
    let mut second_library = crate::state::Library::new("a/b");
    second_library.add_cell(second_cell);
    libraries.add_library(second_library);

    let project = ProjectFile::new(workspace, libraries);
    let json = serde_json::to_string_pretty(&project).expect("alias fixture serializes");
    let error = load_project_text(&json, None)
        .expect_err("two distinct triples must not alias one generated key");

    assert!(matches!(error, ProjectIoError::InvalidData(_)));
    assert!(
        error
            .to_string()
            .contains("duplicate cell-view key 'a/b/c/d'")
            && error.to_string().contains("injective"),
        "unexpected alias error: {error}"
    );
}

#[test]
fn project_load_rejects_lcv_names_outside_the_ui_contract() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut invalid_cell = crate::state::Cell::new("bad-name");
    invalid_cell.add_view(crate::state::View::new("schematic", ViewType::Schematic));
    libraries
        .get_library_mut("user")
        .expect("default library")
        .add_cell(invalid_cell);
    let project = ProjectFile::new(workspace, libraries);
    let json = serde_json::to_string_pretty(&project).expect("invalid fixture serializes");

    let error = load_project_text(&json, None)
        .expect_err("persisted names outside the UI contract must fail closed");

    assert!(error.to_string().contains("bad-name"));
    assert!(error.to_string().contains("cell-view name contract"));
}

#[test]
fn project_load_rejects_orphan_and_malformed_schematic_buffers() {
    for (key, expected) in [
        ("ghost/cell/schematic", "orphaned"),
        ("user/top/schematic/alias", "malformed"),
    ] {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let buffer = workspace
            .schematic_buffers
            .values()
            .next()
            .cloned()
            .expect("default schematic buffer");
        workspace.schematic_buffers.insert(key.to_owned(), buffer);
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("buffer fixture serializes");

        let error = load_project_text(&json, None)
            .expect_err("unowned or malformed persisted buffers must fail closed");
        assert!(
            error.to_string().contains(key) && error.to_string().contains(expected),
            "unexpected buffer error for {key}: {error}"
        );
    }
}

#[test]
fn project_load_rejects_schematic_buffer_bound_to_symbol_view() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    add_top_symbol_view(&mut libraries);
    let buffer = workspace
        .schematic_buffers
        .values()
        .next()
        .cloned()
        .expect("default schematic buffer");
    workspace
        .schematic_buffers
        .insert("user/top/symbol".to_owned(), buffer);
    let project = ProjectFile::new(workspace, libraries);
    let json = serde_json::to_string_pretty(&project).expect("buffer fixture serializes");

    let error =
        load_project_text(&json, None).expect_err("symbol views cannot own schematic buffers");
    assert!(
        error.to_string().contains("user/top/symbol") && error.to_string().contains("cannot own")
    );
}

#[test]
fn project_load_rejects_duplicate_open_view_keys() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    workspace.open_views.push(workspace.open_views[0].clone());
    let project = ProjectFile::new(workspace, libraries);
    let json = serde_json::to_string_pretty(&project).expect("duplicate fixture serializes");

    let error =
        load_project_text(&json, None).expect_err("duplicate open-view keys must fail closed");
    assert!(
        error.to_string().contains("duplicate cell-view key")
            && error.to_string().contains("workspace.open_views")
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
#[allow(deprecated)]
fn legacy_public_project_save_is_create_only_and_preserves_existing_bytes() {
    let root = std::env::temp_dir().join(format!(
        "rspice-legacy-create-only-project-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&root).expect("create isolated project test directory");
    let path = root.join("design.rspiceproj");
    std::fs::write(&path, "external project bytes").expect("write existing target");
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectFile::new(workspace, libraries);

    let error = save_project_file(&project, &path)
        .expect_err("legacy public save must not overwrite an existing destination");

    assert!(error.to_string().contains("create-only"));
    assert_eq!(
        std::fs::read_to_string(&path).expect("read preserved destination"),
        "external project bytes"
    );
    std::fs::remove_dir_all(root).expect("remove isolated project test directory");
}

#[test]
fn project_text_load_reports_parse_errors_without_filesystem() {
    let err = load_project_text("{not valid json", Some(Path::new("bad.rspiceproj")))
        .expect_err("invalid project text fails");

    assert!(matches!(err, ProjectIoError::ParseError(_)));
}

#[test]
fn canonical_plan_kind_tags_cover_the_complete_manifest_without_collisions() {
    let tags = AnalysisKind::ALL
        .into_iter()
        .map(analysis_kind_tag_for_plan_kind)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(tags.len(), AnalysisKind::ALL.len());
    assert_eq!(analysis_kind_tag_for_plan_kind(AnalysisKind::Qpss), 26);
    assert_eq!(
        analysis_kind_tag_for_plan_kind(AnalysisKind::DcMismatch),
        34
    );
}

/// A project written before results were attributed to a PVT point still
/// loads, and its results stay unattributed. Inventing a nominal point for
/// them would let a specification scoped to nominal pass on evidence that
/// belongs to no point at all.
#[test]
fn results_written_before_point_attribution_load_as_unattributed() {
    use crate::state::SpecPointScope;

    let mut run = SimulationRun::new(41);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "Prepared AC").with_provenance(
            AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0xb7; 32]),
                Vec::new(),
            )
            .expect("prepared provenance fixture"),
        ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0xb6; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xb5; 32])),
        &[2],
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 41;

    let mut v5 = ProjectSimulationResults::from_state(&simulation);
    v5.schema_version = SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION;
    clear_v6_execution_fields(&mut v5);
    clear_v14_specification_fields(&mut v5);
    assert_eq!(
        v5.runs[0].analyses[0]
            .provenance
            .as_ref()
            .expect("provenance")
            .pvt_point,
        None,
        "a v5 file has no point field to write"
    );

    v5.migrate_to_current(ProjectId::new())
        .expect("a project without point attribution migrates");
    let restored = v5
        .into_simulation_state()
        .expect("migrated results restore");
    let provenance = restored.runs[0].analyses[0]
        .provenance
        .as_ref()
        .expect("provenance survives the migration");

    assert_eq!(provenance.pvt_point(), None);
    assert!(
        SpecPointScope::AllPoints.admits(provenance.pvt_point()),
        "an unscoped specification still judges restored legacy evidence"
    );
    assert!(
        !SpecPointScope::Nominal.admits(provenance.pvt_point()),
        "a nominal-scoped specification is never answered by unattributed evidence"
    );
}

/// Attribution round-trips exactly, and a legacy file cannot smuggle it in:
/// a point on a v5 record would be current-schema evidence wearing an older
/// version number.
#[test]
fn point_attribution_round_trips_and_cannot_masquerade_as_a_legacy_schema() {
    let point = crate::state::AnalysisResultPvtPoint::new(
        "SS",
        Some(1.62),
        125.0,
        Some(ContentDigest::from_bytes([0xc4; 32])),
        false,
    )
    .expect("valid attributed point");
    let mut run = SimulationRun::new(42);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "SS 125C").with_provenance(
            AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0xc3; 32]),
                Vec::new(),
            )
            .expect("prepared provenance fixture")
            .with_pvt_point(Some(point.clone())),
        ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0xc2; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xc1; 32])),
        &[2],
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 42;

    let current = ProjectSimulationResults::from_state(&simulation);
    let json = serde_json::to_string(&current).expect("serialize attributed results");
    let reloaded: ProjectSimulationResults =
        serde_json::from_str(&json).expect("attributed results restore");
    let restored = reloaded
        .into_simulation_state()
        .expect("attributed results apply");
    assert_eq!(
        restored.runs[0].analyses[0]
            .provenance
            .as_ref()
            .expect("provenance")
            .pvt_point(),
        Some(&point)
    );

    let mut smuggled = current;
    smuggled.schema_version = SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION;
    clear_v6_execution_fields(&mut smuggled);
    clear_v14_specification_fields(&mut smuggled);
    assert!(
        smuggled
            .migrate_to_current(ProjectId::new())
            .expect_err("a v5 record cannot carry point attribution")
            .contains("pvt_point was introduced after schema v5")
    );
}
