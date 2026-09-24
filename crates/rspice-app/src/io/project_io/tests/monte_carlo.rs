//! Monte Carlo draft and retained statistical evidence persistence.

use super::*;

#[test]
fn monte_carlo_checkpoint_retention_project_round_trip_integrity_and_legacy_absence() {
    use crate::simulation::runner::monte_carlo_checkpoint::tests::completed_checkpoint_fixture;
    use crate::state::MonteCarloCheckpointEvidence;

    let (_, bytes, _) = completed_checkpoint_fixture();
    let checkpoint = MonteCarloCheckpointEvidence::from_bytes(bytes.clone()).unwrap();
    let provenance = AnalysisResultProvenance::new_with_source_domain(
        AnalysisResultSourceDomain::ManualDeck,
        crate::product::manual_deck_analysis_instance_id_from_tag(
            ContentDigest::from_bytes([82; 32]),
            crate::state::CanonicalAnalysisKind::MonteCarlo.tag(),
            0,
        ),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([81; 32]),
        vec![],
    )
    .unwrap();
    let mut analysis = AnalysisResult::failed(1, AnalysisType::MonteCarlo, "MC", "Aborted")
        .with_provenance(provenance);
    analysis.monte_carlo_checkpoint = Some(checkpoint.clone());
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::ManualDeck,
        None,
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([82; 32]),
        PreparedSourceCheckReceipt::ManualSourceCheck(ContentDigest::from_bytes([83; 32])),
        &[crate::state::CanonicalAnalysisKind::MonteCarlo.tag()],
    );
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Aborted)
        .unwrap();
    let mut state = SimulationState::default();
    state.runs = vec![run].into();
    state.next_run_id = 1;
    state.active_run_idx = Some(0);
    state.active_analysis_idx = Some(0);
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut project = ProjectFile::new_with_simulation_results(
        workspace,
        libraries,
        ProjectSimulationResults::from_state(&state),
    );
    let json = serialize_project_file(&project).unwrap();
    let loaded = load_project_text(&json, None).unwrap();
    assert!(
        loaded.simulation_results_warning.is_none(),
        "{:?}",
        loaded.simulation_results_warning
    );
    let restored = loaded.simulation_results.into_simulation_state().unwrap();
    let analysis = restored.active_analysis().unwrap();
    assert!(!analysis.success);
    assert_eq!(analysis.monte_carlo_checkpoint.as_ref(), Some(&checkpoint));
    assert_eq!(
        analysis.monte_carlo_checkpoint.as_ref().unwrap().bytes(),
        &*bytes
    );
    assert_eq!(
        analysis.result_data_digest(),
        state.runs[0].analyses[0].result_data_digest()
    );

    for mutation in 0..5 {
        let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let analysis = &mut value["simulation_results"]["runs"][0]["analyses"][0];
        match mutation {
            0 => analysis["monte_carlo_checkpoint"] = serde_json::Value::Null,
            1 => analysis["monte_carlo_checkpoint"]["data"] = serde_json::json!("AAAA"),
            2 => {
                analysis
                    .as_object_mut()
                    .unwrap()
                    .remove("monte_carlo_checkpoint");
            }
            3 => {
                analysis["monte_carlo_checkpoint"]["digest"] =
                    serde_json::to_value(ContentDigest::from_bytes([84; 32])).unwrap()
            }
            _ => value["simulation_results"]["schema_version"] = serde_json::json!(34),
        }
        let loaded = load_project_text(&value.to_string(), None);
        if matches!(mutation, 1 | 3) {
            // Structural corruption is refused by the bounded checkpoint
            // decoder; semantic result errors are quarantined below.
            assert!(loaded.is_err(), "corrupt checkpoint {mutation} decoded");
            continue;
        }
        let loaded = loaded.unwrap();
        assert!(
            loaded.simulation_results.runs.is_empty(),
            "mutation {mutation} survived"
        );
        assert!(loaded.simulation_results_warning.is_some());
    }

    let mut wrong_type = state.runs[0].analyses[0].clone();
    wrong_type.analysis_type = AnalysisType::Transient;
    assert!(wrong_type.validate_retained_evidence().is_err());
    wrong_type.analysis_type = AnalysisType::MonteCarlo;
    wrong_type.provenance = None;
    assert!(wrong_type.validate_retained_evidence().is_err());

    // Autosave may capture a still-running task. Authenticate its original
    // bytes first, then restore an interrupted outcome with the same journal.
    let mut active = state.clone();
    active.runs[0].lifecycle = SimulationRunLifecycle::Running;
    active.runs[0].analyses[0] = AnalysisResult::live_monte_carlo_partial("MC", checkpoint.clone())
        .with_provenance(state.runs[0].analyses[0].provenance.clone().unwrap());
    project.simulation_results = ProjectSimulationResults::from_state(&active);
    let live_json = serialize_project_file(&project).unwrap();
    let loaded = load_project_text(&live_json, None).unwrap();
    assert!(
        loaded.simulation_results_warning.is_none(),
        "{:?}",
        loaded.simulation_results_warning
    );
    let restored = loaded.simulation_results.into_simulation_state().unwrap();
    assert_eq!(
        restored.runs[0].lifecycle,
        SimulationRunLifecycle::Interrupted
    );
    let partial = &restored.runs[0].analyses[0];
    assert!(!partial.success);
    assert!(!partial.is_live_partial());
    assert!(
        partial
            .error_message
            .as_deref()
            .unwrap()
            .contains("committed trials")
    );
    assert_eq!(partial.monte_carlo_checkpoint.as_ref(), Some(&checkpoint));

    // Pre-checkpoint projects preserve both absence and the old digest.
    state.runs[0].analyses[0].monte_carlo_checkpoint = None;
    let mut legacy = ProjectSimulationResults::from_state(&state);
    let digest = state.runs[0].dataset_content_digest();
    legacy.schema_version = NOISE_INPUT_QUANTITY_RESULTS_SCHEMA_VERSION;
    legacy
        .migrate_to_current(project.workspace.project.id())
        .unwrap();
    let restored = legacy.into_simulation_state().unwrap();
    assert!(
        restored.runs[0].analyses[0]
            .monte_carlo_checkpoint
            .is_none()
    );
    assert_eq!(restored.runs[0].dataset_content_digest(), digest);
}

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
        member_measurements: [Some(0.975), None, Some(1.025)]
            .into_iter()
            .enumerate()
            .map(|(index, value)| {
                crate::state::FamilyMemberMeasurements::new(
                    crate::state::FamilyMemberId::MonteCarloSequenceTrial {
                        index,
                        seed: 42,
                        policy: "parameter-xoroshiro128plus-2018-v1".into(),
                    },
                    vec![crate::state::FamilyMeasurementEvidence {
                        unit: None,
                        name: "V(out)".into(),
                        value,
                        passed: value.is_some(),
                        error: value.is_none().then(|| "did not converge".into()),
                    }],
                )
            })
            .collect(),
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
        serde_json::Value::from(MONTE_CARLO_CONFIDENCE_RESULTS_SCHEMA_VERSION);
    let rejected = load_project_text(&serde_json::to_string(&stale).unwrap(), None).unwrap();
    assert!(rejected.simulation_results.runs.is_empty());
    assert!(
        rejected
            .simulation_results_warning
            .unwrap()
            .contains("before v31")
    );

    // A schema-29 document also cannot claim confidence evidence introduced in 30.
    stale["simulation_results"]["schema_version"] =
        serde_json::Value::from(SENSITIVITY_STUDY_RESULTS_SCHEMA_VERSION);
    stale["simulation_results"]["runs"][0]["analyses"][0]["family_metadata"]["member_measurements"] =
        serde_json::json!([]);
    let rejected = load_project_text(&stale.to_string(), None).unwrap();
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
