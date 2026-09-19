//! Monte Carlo draft and retained statistical evidence persistence.

use super::*;

#[test]
fn monte_carlo_project_restore_distinguishes_legacy_default_from_explicit_zero() {
    let project = project_with_execution_context();
    let serialized = serialize_project_file(&project).unwrap();
    for (field, text, expected) in [
        ("seed", "0", None),
        ("seed", "7", Some(7)),
        ("explicit_seed", "0", Some(0)),
        ("explicit_seed", "18446744073709551615", Some(u64::MAX)),
    ] {
        let mut value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        let draft = serialized_analysis_instance_mut(&mut value, "mc")["draft"]["draft"]
            .as_object_mut()
            .unwrap();
        // This fixture's inactive MC editor starts empty. Supply a runnable
        // analysis before testing which random stream survives project load.
        draft.insert("num_runs".to_owned(), serde_json::json!("2"));
        draft.insert("variation_pct".to_owned(), serde_json::json!("5"));
        draft.remove("explicit_seed");
        draft.insert(field.to_owned(), serde_json::json!(text));
        let loaded = load_project_text(&value.to_string(), None).unwrap();
        let context = loaded.execution_context.unwrap();
        let stable = context.simulation_plan.stable_analysis_plan().unwrap();
        let instance = stable
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::MonteCarlo)
            .unwrap();
        let AnalysisDraft::MonteCarlo(mut draft) = instance.draft().clone() else {
            panic!("wrong draft");
        };
        draft.ensure_initialized();
        assert_eq!(draft.to_config().unwrap().seed, expected, "{field}={text}");
    }
}

#[test]
fn project_file_round_trips_exact_result_family_metadata_and_migrates_v6_absence() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let metadata = crate::state::AnalysisResultFamilyMetadata::MonteCarlo {
        member_measurements: Vec::new(),
        seed: 42,
        runs_requested: 3,
        runs_completed: 2,
        failures: 1,
        all_converged: false,
        variables: vec![crate::state::MonteCarloVariableMetadata {
            mean_confidence: Some(crate::state::MonteCarloMeanConfidence {
                level_pct: 95.0,
                method: crate::state::MonteCarloMeanMethod::StudentT,
                successful_samples: 2,
                conditional_on_successful_trials: true,
                interval: crate::state::MonteCarloMeanInterval::Available {
                    lower: 0.9,
                    upper: 1.1,
                },
            }),
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
    simulation.runs = vec![run].into();
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

    let mut stale: serde_json::Value = serde_json::from_str(&json).unwrap();
    stale["simulation_results"]["schema_version"] =
        serde_json::Value::from(SENSITIVITY_STUDY_RESULTS_SCHEMA_VERSION);
    let rejected = load_project_text(&serde_json::to_string(&stale).unwrap(), None).unwrap();
    assert!(rejected.simulation_results.runs.is_empty());
    assert!(
        rejected
            .simulation_results_warning
            .unwrap()
            .contains("before v30")
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
