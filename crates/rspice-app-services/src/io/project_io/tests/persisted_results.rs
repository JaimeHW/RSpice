//! Result evidence and provenance tests for the persisted project format.
//!
//! These cover the rules that keep a restored project honest: lifecycle and
//! execution identity must agree, evidence cannot be forged or synthesized, and
//! a save requires result provenance to be closed over its plan or its
//! tombstones.

use super::*;

fn current_projected_failvalue_results() -> ProjectSimulationResults {
    let measurement = rspice_core::MeasureResult {
        name: "peak_at".to_owned(),
        value: Some(20.0),
        raw_value: Some(3.0),
        error: None,
        passed: true,
        expected: None,
        tolerance: None,
        failure_limit: Some(4.0),
        failure_limit_exceeded: false,
        event_axis: Some(20.0),
        units: None,
    };
    let mut run = SimulationRun::new(19);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_measurements(vec![measurement]),
    );
    seal_legacy_unattributed(&mut run);

    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 19;
    crate::io::capture_simulation_results(&simulation)
}

#[test]
fn current_project_results_preserve_projected_failvalue_evidence_without_synthesis() {
    let persisted = current_projected_failvalue_results();
    persisted
        .validate()
        .expect("distinct projected and raw values are valid current evidence");

    let restored = crate::io::simulation_state_from_results(persisted)
        .expect("current result restores after validation");
    let measurement = &restored.retained.runs[0].analyses[0].measurements[0];
    assert_eq!(measurement.value, Some(20.0));
    assert_eq!(measurement.raw_value, Some(3.0));
    assert_eq!(measurement.failure_limit, Some(4.0));
    assert!(!measurement.failure_limit_exceeded);
}

#[test]
fn current_project_results_preserve_unevaluated_failvalue_metadata_without_raw_synthesis() {
    let mut persisted = current_projected_failvalue_results();
    let measurement = &mut persisted.runs[0].analyses[0].measurements[0];
    measurement.value = None;
    measurement.raw_value = None;
    measurement.event_axis = None;
    measurement.passed = false;
    measurement.error = Some("signal was unavailable".to_owned());
    seal_project_result_digests(&mut persisted.runs[0]).expect("fixture digests reseal");

    persisted
        .validate()
        .expect("an early failure retains the authored limit without raw evidence");
    let restored = crate::io::simulation_state_from_results(persisted)
        .expect("unevaluated FAILVALUE metadata restores");
    let measurement = &restored.retained.runs[0].analyses[0].measurements[0];
    assert_eq!(measurement.value, None);
    assert_eq!(measurement.raw_value, None);
    assert_eq!(measurement.failure_limit, Some(4.0));
    assert!(!measurement.failure_limit_exceeded);
    assert!(!measurement.passed);
}

#[test]
fn current_project_results_reject_incomplete_or_forged_failvalue_evidence() {
    let source = current_projected_failvalue_results();

    let mut missing_raw = source.clone();
    missing_raw.runs[0].analyses[0].measurements[0].raw_value = None;
    let error = missing_raw
        .validate()
        .expect_err("current results never infer raw evidence from a published value");
    assert!(error.contains("raw value"), "{error}");

    let mut nonfinite_raw = source.clone();
    nonfinite_raw.runs[0].analyses[0].measurements[0].raw_value = Some(f64::NAN);
    assert!(nonfinite_raw.validate().is_err());

    let mut nonfinite_limit = source.clone();
    nonfinite_limit.runs[0].analyses[0].measurements[0].failure_limit = Some(f64::INFINITY);
    assert!(nonfinite_limit.validate().is_err());

    let mut false_positive = source.clone();
    let measurement = &mut false_positive.runs[0].analyses[0].measurements[0];
    measurement.failure_limit_exceeded = true;
    measurement.passed = false;
    let error = false_positive
        .validate()
        .expect_err("FAILVALUE verdicts are recomputed from raw evidence");
    assert!(error.contains("FAILVALUE verdict"), "{error}");

    let mut false_negative = source.clone();
    let measurement = &mut false_negative.runs[0].analyses[0].measurements[0];
    measurement.raw_value = Some(-4.0);
    measurement.passed = false;
    let error = false_negative
        .validate()
        .expect_err("inclusive FAILVALUE equality must be retained as exceeded");
    assert!(error.contains("FAILVALUE verdict"), "{error}");

    let mut passed_after_exceeded = source;
    let measurement = &mut passed_after_exceeded.runs[0].analyses[0].measurements[0];
    measurement.raw_value = Some(4.0);
    measurement.failure_limit_exceeded = true;
    let error = passed_after_exceeded
        .validate()
        .expect_err("an exceeded FAILVALUE measurement cannot be marked passed");
    assert!(error.contains("cannot pass"), "{error}");
}

/// Build a completed one-analysis run whose single waveform optionally
/// states a unit, sealed as legacy-unattributed so no plan fixture is needed.
fn run_with_waveform_unit(sequence: u64, unit: Option<&str>) -> SimulationRun {
    let mut waveform =
        crate::state::RetainedTrace::new("I(V1)", vec![0.0, 1.0], vec![0.0, 1.0e-3], "#00aaff");
    waveform.unit = unit.map(str::to_owned);
    let mut run = SimulationRun::new(sequence);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform]),
    );
    seal_legacy_unattributed(&mut run);
    run
}

#[test]
fn a_retained_waveform_unit_survives_the_current_schema_round_trip() {
    let run = run_with_waveform_unit(34, Some("A"));
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 34;

    let persisted = crate::io::capture_simulation_results(&simulation);
    assert_eq!(persisted.schema_version, 46);
    let restored = crate::io::simulation_state_from_results(persisted)
        .expect("current result history restores");

    assert_eq!(
        restored.retained.runs[0].analyses[0].waveforms[0]
            .unit
            .as_deref(),
        Some("A")
    );
}

#[test]
fn current_schema_rejects_successful_periodic_results_without_the_typed_payload() {
    for (analysis_type, label) in [(AnalysisType::Pss, "PSS"), (AnalysisType::Pstb, "PSTB")] {
        let mut run = SimulationRun::new(351);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.add_analysis(AnalysisResult::new(1, analysis_type, label));
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.retained.runs = vec![run].into();
        simulation.retained.next_run_id = 351;
        let current = crate::io::capture_simulation_results(&simulation);
        let error = current
            .validate()
            .expect_err("current periodic success cannot omit its typed evidence");
        assert!(
            error.contains("must match successful periodic analysis type"),
            "{error}"
        );
    }
}

#[test]
fn current_schema_requires_coherent_lifecycle_and_execution_identity() {
    let mut run = SimulationRun::new(15);
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 15;
    let current = crate::io::capture_simulation_results(&simulation);
    current
        .validate()
        .expect("current preparing snapshot is explicit");

    let mut missing_lifecycle = current.clone();
    missing_lifecycle.runs[0].lifecycle = None;
    assert!(
        missing_lifecycle
            .validate()
            .expect_err("schema v6 requires lifecycle evidence")
            .contains("lifecycle is required by simulation results schema v6")
    );

    let mut missing_job = current.clone();
    missing_job.runs[0].job_id = None;
    assert!(
        missing_job
            .validate()
            .expect_err("current execution requires job identity")
            .contains("job_id is required for a non-legacy lifecycle")
    );

    let mut legacy_with_target = current.clone();
    legacy_with_target.runs[0].lifecycle = Some(SimulationRunLifecycle::LegacyUnknown);
    assert!(
        legacy_with_target
            .validate()
            .expect_err("legacy execution cannot claim current identity")
            .contains("legacy_unknown but carries current execution job/target identity")
    );

    let mut false_completion = current;
    false_completion.runs[0].lifecycle = Some(SimulationRunLifecycle::Completed);
    assert!(
        false_completion
            .validate()
            .expect_err("completed outcome must be successful")
            .contains("completed but its success outcome is false")
    );
}

#[test]
fn persisted_running_and_cancelling_runs_restore_as_interrupted() {
    for (sequence, cancelling) in [(16, false), (17, true)] {
        let mut run = SimulationRun::new(sequence);
        run.mark_running().expect("fixture run starts");
        if cancelling {
            run.mark_cancelling().expect("fixture cancellation starts");
        }
        seal_legacy_unattributed(&mut run);
        let expected_job_id = run.job_id;
        let expected_target = run.execution_target;
        let mut simulation = SimulationState::default();
        simulation.retained.runs = vec![run].into();
        simulation.retained.next_run_id = sequence;

        let persisted = crate::io::capture_simulation_results(&simulation);
        assert!(!persisted.runs[0].success);
        persisted
            .validate()
            .expect("nonterminal snapshot validates");
        let restored = crate::io::simulation_state_from_results(persisted)
            .expect("nonterminal snapshot restores fail-closed");
        let restored = &restored.retained.runs[0];

        assert_eq!(restored.lifecycle, SimulationRunLifecycle::Interrupted);
        assert!(!restored.success);
        assert_eq!(restored.job_id, expected_job_id);
        assert_eq!(restored.execution_target, expected_target);
    }
}

#[test]
fn prepared_result_provenance_round_trips_two_same_kind_analyses_exactly() {
    let first_id = AnalysisInstanceId::new();
    let second_id = AnalysisInstanceId::new();
    let snapshot = ContentDigest::from_bytes([0xa7; 32]);
    let revision = ObjectRevision::new(12).expect("fixture revision");
    let mut run = SimulationRun::new(21);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC low band").with_provenance(
            AnalysisResultProvenance::new(first_id, revision, snapshot, Vec::new())
                .expect("first provenance"),
        ),
    );
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Ac, "AC high band").with_provenance(
            AnalysisResultProvenance::new(second_id, revision, snapshot, vec![first_id])
                .expect("second provenance"),
        ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0xa6; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xa5; 32])),
        &[2, 2],
    );
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 21;

    let persisted = crate::io::capture_simulation_results(&simulation);
    let json = serde_json::to_string(&persisted).expect("results serialize");
    let decoded: ProjectSimulationResults =
        serde_json::from_str(&json).expect("results deserialize");
    let restored = crate::io::simulation_state_from_results(decoded)
        .expect("provenance validates and restores");
    let restored_run = &restored.retained.runs[0];

    assert_eq!(restored_run.analyses.len(), 2);
    assert_eq!(
        restored_run
            .find_analysis_by_source_instance(first_id)
            .expect("first exact source")
            .label,
        "AC low band"
    );
    let second = restored_run
        .find_analysis_by_source_instance(second_id)
        .expect("second exact source");
    let second_provenance = second.provenance.as_ref().expect("current provenance");
    assert_eq!(second.label, "AC high band");
    assert_eq!(second_provenance.source_revision(), revision);
    assert_eq!(second_provenance.prepared_snapshot_digest(), snapshot);
    assert_eq!(second_provenance.dependency_ids(), &[first_id]);
}

#[test]
fn manual_deck_result_provenance_round_trips_without_a_simulation_plan() {
    let source_content_digest = ContentDigest::from_bytes([0x8c; 32]);
    let source_id = rspice_vocabulary::product::manual_deck_analysis_instance_id_from_tag(
        source_content_digest,
        5,
        0,
    );
    let snapshot = ContentDigest::from_bytes([0x8d; 32]);
    let mut run = SimulationRun::new(29);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Manual TRAN").with_provenance(
            AnalysisResultProvenance::new_with_source_domain(
                AnalysisResultSourceDomain::ManualDeck,
                source_id,
                ObjectRevision::INITIAL,
                snapshot,
                Vec::new(),
            )
            .expect("manual-deck provenance"),
        ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::ManualDeck,
        None,
        ObjectRevision::INITIAL,
        source_content_digest,
        PreparedSourceCheckReceipt::ManualSourceCheck(ContentDigest::from_bytes([0x8b; 32])),
        &[5],
    );
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 29;

    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectSnapshot::new_with_simulation_results(
        workspace,
        libraries,
        crate::io::capture_simulation_results(&simulation),
    );

    let json = serialize_project_file(&project)
        .expect("manual-deck result does not require a simulation-plan owner");
    let loaded = load_project_text(&json, None).expect("manual-deck project reloads");
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("manual-deck result history restores");
    let provenance = restored.retained.runs[0].analyses[0]
        .provenance
        .as_ref()
        .expect("provenance retained");

    assert_eq!(
        provenance.source_domain(),
        AnalysisResultSourceDomain::ManualDeck
    );
    assert_eq!(provenance.source_instance_id(), source_id);
    assert_eq!(provenance.prepared_snapshot_digest(), snapshot);
}

#[test]
fn project_save_requires_result_provenance_to_be_closed_over_plan_or_tombstones() {
    let mut project = project_with_execution_context();
    let plan = project
        .file
        .execution_context
        .as_ref()
        .expect("execution context")
        .simulation_plan
        .stable_analysis_plan()
        .expect("stable plan");
    let source = plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Ac)
        .expect("AC fixture instance");
    let source_id = source.id();
    let source_revision = plan.revision();
    let snapshot = ContentDigest::from_bytes([0x5c; 32]);

    let mut run = SimulationRun::new(31);
    let run_id = run.run_id;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_provenance(
            AnalysisResultProvenance::new(source_id, source_revision, snapshot, Vec::new())
                .expect("prepared provenance"),
        ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan.id()),
        project.file.workspace.project.revision(),
        ContentDigest::from_bytes([0x5b; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x5a; 32])),
        &[2],
    );
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 31;
    project.file.simulation_results = crate::io::capture_simulation_results(&simulation);

    serialize_project_file(&project).expect("current plan owns result source");

    let mut future_revision = project.clone();
    let future_source_revision =
        ObjectRevision::new(source_revision.get() + 1).expect("future fixture revision");
    future_revision.file.simulation_results.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_revision = future_source_revision;
    future_revision.file.simulation_results.runs[0]
        .prepared_receipt
        .as_mut()
        .expect("receipt")
        .tasks[0]
        .source_revision = future_source_revision;
    assert!(
        serialize_project_file(&future_revision)
            .expect_err("result revision beyond the retained plan must block save")
            .to_string()
            .contains("outside the retained analysis revision interval")
    );

    let mut missing = project.clone();
    let orphaned_source_id = AnalysisInstanceId::new();
    missing.file.simulation_results.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_instance_id = orphaned_source_id;
    missing.file.simulation_results.runs[0]
        .prepared_receipt
        .as_mut()
        .expect("receipt")
        .tasks[0]
        .source_instance_id = orphaned_source_id;
    assert!(
        serialize_project_file(&missing)
            .expect_err("orphaned prepared result must block save")
            .to_string()
            .contains("absent from the persisted plan and its tombstones")
    );

    let mut retained = project.clone();
    retained
        .file
        .execution_context
        .as_mut()
        .expect("execution context")
        .simulation_plan
        .stable_analysis_plan_mut()
        .expect("stable plan")
        .remove(source_id, vec![run_id])
        .expect("remove with exact retained run");
    serialize_project_file(&retained).expect("tombstone closes retained result reference");

    let removed_revision = retained
        .file
        .execution_context
        .as_ref()
        .expect("execution context")
        .simulation_plan
        .stable_analysis_plan()
        .expect("stable plan")
        .tombstones()
        .iter()
        .find(|tombstone| tombstone.id() == source_id)
        .expect("source tombstone")
        .removed_revision();
    let mut at_removal = retained.clone();
    at_removal.file.simulation_results.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_revision = removed_revision;
    at_removal.file.simulation_results.runs[0]
        .prepared_receipt
        .as_mut()
        .expect("receipt")
        .tasks[0]
        .source_revision = removed_revision;
    assert!(
        serialize_project_file(&at_removal)
            .expect_err("a result cannot be produced at the removal revision")
            .to_string()
            .contains("outside the retained analysis revision interval")
    );

    let mut unretained = project;
    unretained
        .file
        .execution_context
        .as_mut()
        .expect("execution context")
        .simulation_plan
        .stable_analysis_plan_mut()
        .expect("stable plan")
        .remove(source_id, Vec::new())
        .expect("remove without retained run");
    assert!(
        serialize_project_file(&unretained)
            .expect_err("tombstone must retain the exact historical run")
            .to_string()
            .contains("is not retained by its tombstone")
    );
}

#[test]
fn project_save_rejects_result_revision_before_source_creation() {
    let mut project = project_with_execution_context();
    let (source_id, plan_id, created_revision) = {
        let plan = project
            .file
            .execution_context
            .as_mut()
            .expect("execution context")
            .simulation_plan
            .stable_analysis_plan_mut()
            .expect("stable plan");
        let (source_id, _) = plan
            .insert(AnalysisKind::Ac)
            .expect("independent AC source inserts");
        let source = plan.instance(source_id).expect("inserted source");
        (source_id, plan.id(), source.created_revision())
    };
    assert!(created_revision.get() > ObjectRevision::INITIAL.get());
    let before_creation = ObjectRevision::new(created_revision.get() - 1)
        .expect("revision immediately before creation exists");
    let snapshot = ContentDigest::from_bytes([0xc3; 32]);
    let mut run = SimulationRun::new(36);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "Premature AC").with_provenance(
            AnalysisResultProvenance::new(source_id, before_creation, snapshot, Vec::new())
                .expect("prepared provenance"),
        ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        project.file.workspace.project.revision(),
        ContentDigest::from_bytes([0xc2; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xc1; 32])),
        &[2],
    );
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 36;
    project.file.simulation_results = crate::io::capture_simulation_results(&simulation);

    let error = serialize_project_file(&project)
        .expect_err("source cannot own results from before it existed")
        .to_string();
    assert!(
        error.contains("outside the retained analysis revision interval"),
        "{error}"
    );
}

#[test]
fn prepared_result_provenance_validation_rejects_aliases_and_partial_history() {
    let first_id = AnalysisInstanceId::new();
    let second_id = AnalysisInstanceId::new();
    let snapshot = ContentDigest::from_bytes([0x61; 32]);
    let mut run = SimulationRun::new(23);
    for (sequence, source_id, label) in [(1, first_id, "AC one"), (2, second_id, "AC two")] {
        run.add_analysis(
            AnalysisResult::new(sequence, AnalysisType::Ac, label).with_provenance(
                AnalysisResultProvenance::new(
                    source_id,
                    ObjectRevision::INITIAL,
                    snapshot,
                    Vec::new(),
                )
                .expect("fixture provenance"),
            ),
        );
    }
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x60; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x5f; 32])),
        &[2, 2],
    );
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run].into();
    simulation.retained.next_run_id = 23;
    let baseline = crate::io::capture_simulation_results(&simulation);

    let mut aliased = baseline.clone();
    aliased.runs[0].analyses[1]
        .provenance
        .as_mut()
        .expect("second provenance")
        .source_instance_id = first_id;
    assert!(
        aliased
            .validate()
            .expect_err("source aliases fail closed")
            .contains("duplicates prepared analysis instance")
    );

    let mut forward_dependency = baseline.clone();
    forward_dependency.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("first provenance")
        .dependency_ids = vec![second_id];
    assert!(
        forward_dependency
            .validate()
            .expect_err("dependencies must follow frozen execution order")
            .contains("before that result appears")
    );

    let mut partial = baseline.clone();
    partial.runs[0].analyses[1].provenance = None;
    assert!(
        partial
            .validate()
            .expect_err("legacy/current mixing fails closed")
            .contains("is prepared_task_bound")
    );

    let mut stripped = baseline;
    for analysis in &mut stripped.runs[0].analyses {
        analysis.provenance = None;
    }
    assert!(
        stripped
            .validate()
            .expect_err("current-schema provenance cannot disappear wholesale")
            .contains("is prepared_task_bound")
    );
}

mod result_history;

/// A file that says it was written by a later build is refused rather than
/// read as if this build understood it.
#[test]
fn a_results_schema_from_the_future_is_refused_by_number() {
    let mut ahead = crate::io::capture_simulation_results(&SimulationState::default());
    ahead.schema_version = ProjectSimulationResultsData::default().schema_version + 1;
    let error = ahead
        .validate()
        .expect_err("a forward schema version is not read");
    assert!(
        error.contains(&format!(
            "unsupported simulation results schema version {}",
            ahead.schema_version
        )),
        "{error}"
    );
}
