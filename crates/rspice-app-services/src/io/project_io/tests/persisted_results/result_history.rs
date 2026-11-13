//! Retained simulation-result history through the project file, and recovery from corrupted payloads.

use super::*;
use crate::state::{SensitivityResultMode, SensitivityResultRow};

fn sensitivity_history(
    normalized: rspice_core::analysis::sensitivity::SensitivityValue<f64>,
) -> (SimulationRun, ProjectSimulationResults) {
    let mut run = SimulationRun::new(1);
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS").with_result_payload(
            AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows: vec![SensitivityResultRow {
                    parameter: "gain".to_owned(),
                    raw: 0.0.into(),
                    normalized,
                }],
            },
        ),
    );
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run.clone()].into();
    simulation.retained.next_run_id = 1;
    (run, crate::io::capture_simulation_results(&simulation))
}

#[test]
fn sensitivity_availability_survives_project_results_serialization_and_authentication() {
    use rspice_core::analysis::sensitivity::{SensitivityUnavailability, SensitivityValue};
    let (run, saved) = sensitivity_history(SensitivityValue::unavailable(
        SensitivityUnavailability::ZeroOutput,
    ));
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectSnapshot::new_with_simulation_results(workspace, libraries, saved.clone());
    let text = serialize_project_file(&project).unwrap();
    let loaded = load_project_text(&text, None).unwrap();
    assert!(loaded.file.simulation_results_warning.is_none());
    let restored =
        crate::io::simulation_state_from_results(loaded.file.simulation_results).unwrap();
    assert_eq!(
        restored.retained.runs[0].analyses[0].result_payload,
        run.analyses[0].result_payload
    );
    for replacement in [
        0.0.into(),
        SensitivityValue::unavailable(SensitivityUnavailability::OutOfRange),
    ] {
        let mut tampered = saved.clone();
        let PersistedField::Value(AnalysisResultPayload::Sensitivity { rows, .. }) =
            &mut tampered.runs[0].analyses[0].result_payload
        else {
            panic!("sensitivity payload")
        };
        rows[0].normalized = replacement;
        assert!(
            tampered.validate().is_err(),
            "changing availability changes authenticated evidence"
        );
    }
}

#[test]
fn cleared_run_sequence_survives_project_round_trip() {
    for last_sequence in [1, 42, u64::MAX] {
        let mut simulation = SimulationState::default();
        simulation.retained.next_run_id = last_sequence - 1;
        assert_eq!(simulation.start_run().id, last_sequence);
        simulation.clear_runs();
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectSnapshot::new_with_simulation_results(
            workspace,
            libraries,
            crate::io::capture_simulation_results(&simulation),
        );
        let text = serialize_project_file(&project).expect("cleared history publishes");
        let loaded = load_project_text(&text, None).expect("cleared history reopens");
        assert!(loaded.file.simulation_results_warning.is_none());
        assert_eq!(loaded.file.simulation_results.next_run_id, last_sequence);
        let mut restored =
            crate::io::simulation_state_from_results(loaded.file.simulation_results).unwrap();
        assert!(restored.retained.runs.is_empty());
        assert_eq!(restored.retained.next_run_id, last_sequence);
        if last_sequence < u64::MAX {
            assert_eq!(restored.start_run().id, last_sequence + 1);
        }
    }
}

#[test]
fn project_text_load_drops_invalid_simulation_results_without_rejecting_workspace() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut simulation = SimulationState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 1;
    simulation.view.active_run_idx = Some(0);
    simulation.view.active_analysis_idx = Some(0);
    let project = ProjectSnapshot::new_with_simulation_results(
        workspace,
        libraries,
        crate::io::capture_simulation_results(&simulation),
    );
    let mut value = serde_json::to_value(project).expect("project converts to JSON");
    value["simulation_results"]["schema_version"] = serde_json::Value::from(999);
    let json = serde_json::to_string_pretty(&value).expect("fixture serializes");

    let loaded = load_project_text(&json, None).expect("workspace still loads");

    assert!(loaded.file.simulation_results.is_empty());
    assert!(
        loaded
            .file
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
        .file
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
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 71;
    project.file.simulation_results = crate::io::capture_simulation_results(&simulation);
    project
        .file
        .workspace
        .plan_data_mut(plan_id)
        .expect("active plan payload")
        .regression_baseline_run = Some(baseline_id);

    let json = serde_json::to_string_pretty(&project).expect("legacy baseline fixture");
    let loaded = load_project_text(&json, None).expect("project remains loadable");

    assert_eq!(loaded.file.simulation_results.runs.len(), 1);
    assert!(
        loaded
            .file
            .workspace
            .plan_data(plan_id)
            .expect("active plan payload")
            .regression_baseline_run
            .is_none()
    );
    assert!(
        loaded
            .file
            .simulation_results_warning
            .as_deref()
            .unwrap_or_default()
            .contains("not eligible")
    );
    serialize_project_file(&loaded).expect("cleaned project reserializes");
}

#[test]
fn project_load_clears_dangling_regression_baseline_without_rejecting_project() {
    let mut project = project_with_execution_context();
    let plan_id = project
        .file
        .execution_context
        .as_ref()
        .expect("execution context")
        .simulation_plan
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    project
        .file
        .workspace
        .plan_data_mut(plan_id)
        .expect("active plan payload")
        .regression_baseline_run = Some(crate::product::RunId::new());

    let json = serde_json::to_string_pretty(&project).expect("dangling baseline fixture");
    let loaded = load_project_text(&json, None).expect("project remains loadable");

    assert!(loaded.file.simulation_results.is_empty());
    assert!(
        loaded
            .file
            .workspace
            .plan_data(plan_id)
            .expect("active plan payload")
            .regression_baseline_run
            .is_none()
    );
    assert!(
        loaded
            .file
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
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 1;
    simulation.view.active_run_idx = Some(0);
    simulation.view.active_analysis_idx = Some(0);
    let project = ProjectSnapshot::new_with_simulation_results(
        workspace,
        libraries,
        crate::io::capture_simulation_results(&simulation),
    );
    let json = serialize_project_file(&project)
        .expect("project serializes")
        .replace(
            "\"analysis_type\": \"Ac\"",
            "\"analysis_type\": \"FutureAnalysis\"",
        );

    let loaded = load_project_text(&json, None).expect("workspace still loads");

    assert!(loaded.file.simulation_results.is_empty());
    assert!(
        loaded
            .file
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
    let results: ProjectSimulationResults = ProjectSimulationResultsData {
        retained_dataset_limit: None,
        schema_version: ProjectSimulationResultsData::default().schema_version,
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
        ..ProjectSimulationResultsData::default()
    }
    .into();

    let error = crate::io::simulation_state_from_results(results)
        .expect_err("invalid overlay references fail closed");

    assert!(error.contains("duplicate overlay dataset id"));
}

#[test]
fn project_results_validation_rejects_duplicate_run_ids() {
    let mut run_one = SimulationRun::new(1);
    let mut run_duplicate = SimulationRun::new(1);
    seal_legacy_unattributed(&mut run_one);
    seal_legacy_unattributed(&mut run_duplicate);
    let results: ProjectSimulationResults = ProjectSimulationResultsData {
        retained_dataset_limit: None,
        schema_version: ProjectSimulationResultsData::default().schema_version,
        runs: vec![
            ProjectSimulationRun::from(&run_one),
            ProjectSimulationRun::from(&run_duplicate),
        ],
        next_run_id: 1,
        active_run_stable_id: Some(run_one.run_id),
        active_dataset_id: Some(run_one.dataset_id),
        active_analysis_sequence: None,
        overlay_dataset_ids: Vec::new(),
        ..ProjectSimulationResultsData::default()
    }
    .into();

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
    simulation.retained.runs = vec![run_one, run_two].into();
    simulation.retained.next_run_id = 2;

    let baseline = crate::io::capture_simulation_results(&simulation);

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
    simulation.retained.runs = vec![run_two, run_one].into();
    simulation.retained.next_run_id = 2;

    let historical = serde_json::to_string(&crate::io::capture_simulation_results(&simulation))
        .expect("results serialize");
    assert!(
        !historical.contains("retention"),
        "a project with no baselines writes exactly what it wrote before they existed"
    );
    let restored = crate::io::simulation_state_from_results(
        serde_json::from_str::<ProjectSimulationResults>(&historical)
            .expect("historical results decode"),
    )
    .expect("historical results restore");
    assert_eq!(restored.retained.pinned_run_count(), 0);
    assert!(
        restored
            .retained
            .runs
            .iter()
            .all(|run| run.retention() == RunRetention::Pruneable)
    );

    assert!(simulation.set_run_retention(baseline_run_id, RunRetention::GoldenBaseline));
    let current = serde_json::to_string(&crate::io::capture_simulation_results(&simulation))
        .expect("results with a baseline serialize");
    let reloaded = crate::io::simulation_state_from_results(
        serde_json::from_str::<ProjectSimulationResults>(&current)
            .expect("results with a baseline decode"),
    )
    .expect("results with a baseline restore");
    assert_eq!(reloaded.retained.pinned_run_count(), 1);
    assert!(
        reloaded
            .retained
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
    simulation.retained.runs = vec![run_one, run_two].into();
    simulation.retained.next_run_id = 2;
    simulation.view.active_run_idx = Some(0);
    simulation.view.active_analysis_idx = Some(0);
    let baseline = crate::io::capture_simulation_results(&simulation);

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
fn project_results_validation_rejects_duplicate_waveform_names_in_analysis() {
    let run_id = RunId::new();
    let dataset_id = DatasetId::new();
    let results: ProjectSimulationResults = ProjectSimulationResultsData {
        retained_dataset_limit: None,
        schema_version: ProjectSimulationResultsData::default().schema_version,
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
                convergence: PersistedField::Missing,
                id: 1,
                analysis_type: "Transient".to_string(),
                label: "TRAN".to_string(),
                timestamp: 1.0,
                result_data_digest: PersistedField::Missing,
                waveforms: vec![
                    ProjectWaveformData {
                        name: "V(out)".to_string(),
                        x: vec![0.0].into(),
                        y: vec![1.0].into(),
                        color: "#00aaff".to_string(),
                        visible: true,
                        unit: None,
                        complex: None,
                    },
                    ProjectWaveformData {
                        name: "V(out)".to_string(),
                        x: vec![0.0].into(),
                        y: vec![2.0].into(),
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
                native_scalar_units: PersistedField::Missing,
                monte_carlo_checkpoint: PersistedField::Missing,
                transient_checkpoint: PersistedField::Missing,
                measurements: Vec::new(),
                checks: PersistedField::Missing,
                saved_output_receipts: Vec::new(),
                success: true,
                error_message: None,
                failure_attribution: None,
                failure_capability: None,
                provenance: None,
                import_source: PersistedField::Missing,
            }],
            dataset_content_digest: PersistedField::Missing,
            provenance_mode: PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed),
            prepared_receipt: PersistedField::Missing,
            campaign_membership: None,
            retention: RunRetention::Pruneable,
            elapsed_time: 0.1,
            success: true,
            specification_verdicts: None,
            log: Default::default(),
        }],
        next_run_id: 1,
        active_run_stable_id: Some(run_id),
        active_dataset_id: Some(dataset_id),
        active_analysis_sequence: Some(1),
        overlay_dataset_ids: Vec::new(),
        ..ProjectSimulationResultsData::default()
    }
    .into();

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
    let results: ProjectSimulationResults = ProjectSimulationResultsData {
        retained_dataset_limit: None,
        schema_version: ProjectSimulationResultsData::default().schema_version,
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
                convergence: PersistedField::Missing,
                id: 1,
                analysis_type: "Transient".to_string(),
                label: "TRAN".to_string(),
                timestamp: 1.0,
                result_data_digest: PersistedField::Missing,
                waveforms: vec![ProjectWaveformData {
                    name: "V(out)".to_string(),
                    x: vec![0.0, 2.0, 1.0, 3.0].into(),
                    y: vec![0.0, 1.0, 2.0, 3.0].into(),
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
                native_scalar_units: PersistedField::Missing,
                monte_carlo_checkpoint: PersistedField::Missing,
                transient_checkpoint: PersistedField::Missing,
                measurements: Vec::new(),
                checks: PersistedField::Missing,
                saved_output_receipts: Vec::new(),
                success: true,
                error_message: None,
                failure_attribution: None,
                failure_capability: None,
                provenance: None,
                import_source: PersistedField::Missing,
            }],
            dataset_content_digest: PersistedField::Missing,
            provenance_mode: PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed),
            prepared_receipt: PersistedField::Missing,
            campaign_membership: None,
            retention: RunRetention::Pruneable,
            elapsed_time: 0.1,
            success: true,
            specification_verdicts: None,
            log: Default::default(),
        }],
        next_run_id: 1,
        active_run_stable_id: Some(run_id),
        active_dataset_id: Some(dataset_id),
        active_analysis_sequence: Some(1),
        overlay_dataset_ids: Vec::new(),
        ..ProjectSimulationResultsData::default()
    }
    .into();

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
    let mut results: ProjectSimulationResults = ProjectSimulationResultsData {
        retained_dataset_limit: None,
        schema_version: ProjectSimulationResultsData::default().schema_version,
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
                convergence: PersistedField::Missing,
                id: 1,
                analysis_type: "Noise".to_string(),
                label: "NOISE".to_string(),
                timestamp: 1.0,
                result_data_digest: PersistedField::Missing,
                waveforms: Vec::new(),
                dc_op: None,
                device_op: None,
                noise_summary: Some(ProjectNoiseSummary {
                    input_quantity: None,
                    conversion: None,
                    noise_figure: None,
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
                native_scalar_units: PersistedField::Missing,
                monte_carlo_checkpoint: PersistedField::Missing,
                transient_checkpoint: PersistedField::Missing,
                measurements: Vec::new(),
                checks: PersistedField::Missing,
                saved_output_receipts: Vec::new(),
                success: true,
                error_message: None,
                failure_attribution: None,
                failure_capability: None,
                provenance: None,
                import_source: PersistedField::Missing,
            }],
            dataset_content_digest: PersistedField::Missing,
            provenance_mode: PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed),
            prepared_receipt: PersistedField::Missing,
            campaign_membership: None,
            retention: RunRetention::Pruneable,
            elapsed_time: 0.1,
            success: true,
            specification_verdicts: None,
            log: Default::default(),
        }],
        next_run_id: 1,
        active_run_stable_id: Some(run_id),
        active_dataset_id: Some(dataset_id),
        active_analysis_sequence: Some(1),
        overlay_dataset_ids: Vec::new(),
        ..ProjectSimulationResultsData::default()
    }
    .into();

    seal_project_result_digests(&mut results.runs[0]).expect("fixture digests seal");

    results.validate().expect("core noise labels are valid");

    let restored =
        crate::io::simulation_state_from_results(results).expect("valid result history restores");
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
        .content
        .project
        .set_path(PathBuf::from("stale-native-path.rspiceproj"));
    let project = ProjectSnapshot::new(workspace, libraries);
    let json = serialize_project_file(&project).expect("project serializes");

    let loaded = load_project_text(&json, Some(Path::new("browser-import.rspiceproj")))
        .expect("project text loads");

    assert_eq!(
        loaded.file.workspace.project.path.as_deref(),
        Some(Path::new("browser-import.rspiceproj"))
    );
    assert_eq!(
        loaded.file.workspace.project.display_name(),
        "stale-native-path",
        "moving a project file must not silently rename its logical identity"
    );
}

#[test]
fn project_text_load_without_source_path_clears_stale_file_identity() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    workspace
        .content
        .project
        .set_path(PathBuf::from("stale-native-path.rspiceproj"));
    let project = ProjectSnapshot::new(workspace, libraries);
    let json = serialize_project_file(&project).expect("project serializes");

    let loaded = load_project_text(&json, None).expect("project text loads");

    assert!(loaded.file.workspace.project.path.is_none());
    assert_eq!(
        loaded.file.workspace.project.display_name(),
        "stale-native-path"
    );
}

#[test]
fn project_load_rejects_unsupported_descriptor_schema() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectSnapshot::new(workspace, libraries);
    let mut value = serde_json::to_value(project).expect("project converts to JSON");
    value["workspace"]["project"]["schema_version"] =
        serde_json::Value::from(crate::state::PROJECT_DESCRIPTOR_SCHEMA_VERSION + 1);
    let contents = serde_json::to_string_pretty(&value).expect("fixture serializes");

    let error = load_project_text(&contents, None)
        .expect_err("future project descriptor schema must fail closed");

    assert!(matches!(error, ProjectIoError::ParseError(_)));
    assert!(
        error
            .to_string()
            .contains("unsupported project descriptor schema version")
    );
}

#[test]
fn project_load_rejects_a_descriptor_without_its_identity_or_schema() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectSnapshot::new(workspace, libraries);
    let value = serde_json::to_value(project).expect("project converts to JSON");

    for field in ["id", "schema_version"] {
        let mut missing = value.clone();
        missing["workspace"]["project"]
            .as_object_mut()
            .expect("project descriptor object")
            .remove(field);
        let error = load_project_text(&missing.to_string(), None)
            .expect_err("a descriptor cannot lose a required field");
        assert!(
            error
                .to_string()
                .contains(&format!("missing field `{field}`")),
            "{error}"
        );

        let mut null = value.clone();
        null["workspace"]["project"][field] = serde_json::Value::Null;
        assert!(
            load_project_text(&null.to_string(), None).is_err(),
            "an explicit null {field} is refused"
        );
    }
}

#[test]
fn project_text_load_rejects_missing_active_schematic_buffer() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut project = ProjectSnapshot::new(workspace, libraries);
    let active_key = project.file.workspace.active_key();
    project.remove_schematic_editor(&active_key);
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
        .content
        .schematic_buffers
        .keys()
        .next()
        .and_then(|key| workspace.clone_schematic_editor(key))
        .expect("default project has a schematic buffer");
    workspace.content.active_view = ghost.clone();
    workspace.content.open_views = vec![OpenCellView::new(ghost.clone(), ViewType::Schematic)];
    workspace.content.hierarchy_stack = vec![ghost.clone()];
    workspace.content.schematic_buffers.clear();
    workspace.session.schematic_sessions.clear();
    workspace.insert_schematic_editor(ghost_key.clone(), schematic);
    let project = ProjectSnapshot::new(workspace, libraries);
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
    let active = workspace.content.active_view.clone();
    workspace.content.open_views = vec![OpenCellView::new(active.clone(), ViewType::Symbol)];
    let active_key = active.key();
    let project = ProjectSnapshot::new(workspace, libraries);
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
    workspace.content.active_view = active.clone();
    workspace.content.hierarchy_stack = vec![active.clone()];
    let project = ProjectSnapshot::new(workspace, libraries);
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
        let active = workspace.content.active_view.clone();
        let project = ProjectSnapshot::new(workspace, libraries);
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

        let project = ProjectSnapshot::new(workspace, libraries);
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

    let project = ProjectSnapshot::new(workspace, libraries);
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
    let project = ProjectSnapshot::new(workspace, libraries);
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
            .content
            .schematic_buffers
            .keys()
            .next()
            .and_then(|key| workspace.clone_schematic_editor(key))
            .expect("default schematic buffer");
        workspace.insert_schematic_editor(key.to_owned(), buffer);
        let project = ProjectSnapshot::new(workspace, libraries);
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
        .content
        .schematic_buffers
        .keys()
        .next()
        .and_then(|key| workspace.clone_schematic_editor(key))
        .expect("default schematic buffer");
    workspace.insert_schematic_editor("user/top/symbol".to_owned(), buffer);
    let project = ProjectSnapshot::new(workspace, libraries);
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
    workspace
        .content
        .open_views
        .push(workspace.content.open_views[0].clone());
    let project = ProjectSnapshot::new(workspace, libraries);
    let json = serde_json::to_string_pretty(&project).expect("duplicate fixture serializes");

    let error =
        load_project_text(&json, None).expect_err("duplicate open-view keys must fail closed");
    assert!(
        error.to_string().contains("duplicate cell-view key")
            && error.to_string().contains("workspace.open_views")
    );
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
        .map(|kind| kind.canonical_kind().tag())
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(tags.len(), AnalysisKind::ALL.len());
    assert_eq!(AnalysisKind::Qpss.canonical_kind().tag(), 26);
    assert_eq!(AnalysisKind::DcMismatch.canonical_kind().tag(), 34);
}

#[test]
fn current_impulse_project_history_round_trips_and_authenticates_charge() {
    let mut run = SimulationRun::new(1);
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
            AnalysisResultPayload::TransientEvents {
                voltage_impulses: None,
                digital_traces: vec![],
                real_traces: vec![],
                digital_buses: vec![],
                current_impulses: Some(crate::state::current_impulse_history_fixture()),
            },
        ),
    );
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run.clone()].into();
    simulation.retained.next_run_id = 1;
    let saved = crate::io::capture_simulation_results(&simulation);
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectSnapshot::new_with_simulation_results(workspace, libraries, saved.clone());
    let loaded = load_project_text(&serialize_project_file(&project).unwrap(), None).unwrap();
    assert!(loaded.file.simulation_results_warning.is_none());
    let restored =
        crate::io::simulation_state_from_results(loaded.file.simulation_results).unwrap();
    assert_eq!(
        restored.retained.runs[0].analyses[0].result_payload,
        run.analyses[0].result_payload
    );
    let mut tampered = saved;
    let PersistedField::Value(AnalysisResultPayload::TransientEvents {
        current_impulses: Some(history),
        ..
    }) = &mut tampered.runs[0].analyses[0].result_payload
    else {
        panic!("impulses")
    };
    history.traces[0].points[0].charge_coulombs *= -1.0;
    assert!(tampered.validate().is_err());
}

/// A run's point attribution round-trips exactly.
#[test]
fn point_attribution_round_trips_exactly() {
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
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 42;

    let current = crate::io::capture_simulation_results(&simulation);
    let json = serde_json::to_string(&current).expect("serialize attributed results");
    let reloaded: ProjectSimulationResults =
        serde_json::from_str(&json).expect("attributed results restore");
    let restored =
        crate::io::simulation_state_from_results(reloaded).expect("attributed results apply");
    assert_eq!(
        restored.retained.runs[0].analyses[0]
            .provenance
            .as_ref()
            .expect("provenance")
            .pvt_point(),
        Some(&point)
    );
}
