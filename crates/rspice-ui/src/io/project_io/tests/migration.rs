//! Schema-migration and provenance tests for the persisted project format.
//!
//! These cover loading older schema versions and the rules that keep a
//! restored project honest: an execution field that a v5 file never carried is
//! migrated to an explicit legacy state rather than defaulted, and a save
//! requires result provenance to be closed over its plan or its tombstones.

use super::*;

#[test]
fn schema_v5_migrates_to_explicit_legacy_execution_state() {
    let mut run = SimulationRun::new(14);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 14;
    let mut persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.schema_version = SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION;
    clear_v6_execution_fields(&mut persisted);

    persisted
        .migrate_to_current(ProjectId::new())
        .expect("schema v5 migrates without inventing execution evidence");
    persisted.validate().expect("migrated schema validates");

    assert_eq!(
        persisted.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    let migrated = &persisted.runs[0];
    assert_eq!(
        migrated.lifecycle,
        Some(SimulationRunLifecycle::LegacyUnknown)
    );
    assert_eq!(migrated.job_id, None);
    assert_eq!(migrated.execution_target, None);
    assert!(migrated.success, "legacy outcome evidence is preserved");
}

fn persisted_measurement_at_schema_v17() -> ProjectSimulationResults {
    let mut run = SimulationRun::new(18);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_measurements(vec![rspice_core::MeasureResult::success("peak", 12.0)]),
    );
    seal_legacy_unattributed(&mut run);

    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 18;
    let mut persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.schema_version = PERIODIC_STABILITY_RESULTS_SCHEMA_VERSION;
    let measurement = &mut persisted.runs[0].analyses[0].measurements[0];
    measurement.raw_value = None;
    measurement.failure_limit = None;
    measurement.failure_limit_exceeded = false;
    downgrade_result_digests_to_v8(&mut persisted);
    persisted
}

#[test]
fn schema_v17_is_authenticated_then_restores_measurement_verification_defaults() {
    let mut persisted = persisted_measurement_at_schema_v17();
    let legacy_digest = persisted.runs[0].analyses[0]
        .result_data_digest
        .as_ref()
        .copied()
        .expect("schema-v17 fixture carries a digest");

    persisted
        .migrate_to_current(ProjectId::new())
        .expect("authentic schema-v17 measurement history migrates");

    assert_eq!(
        persisted.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    let measurement = &persisted.runs[0].analyses[0].measurements[0];
    assert_eq!(measurement.raw_value, measurement.value);
    assert_eq!(measurement.failure_limit, None);
    assert!(!measurement.failure_limit_exceeded);
    let current_digest = persisted.runs[0].analyses[0]
        .result_data_digest
        .as_ref()
        .copied()
        .expect("migrated fixture carries a current digest");
    assert_ne!(legacy_digest, current_digest);
    persisted.validate().expect("migrated schema validates");
}

#[test]
fn schema_v17_rejects_tampering_before_resealing() {
    let mut persisted = persisted_measurement_at_schema_v17();
    persisted.runs[0].analyses[0].measurements[0].value = Some(13.0);

    let error = persisted
        .migrate_to_current(ProjectId::new())
        .expect_err("schema-v17 content must match its retained V8 digest");

    assert!(
        error.contains("schema-v17 analysis 1 result data digest"),
        "{error}"
    );

    let mut dataset_digest = persisted_measurement_at_schema_v17();
    dataset_digest.runs[0].dataset_content_digest =
        PersistedField::Value(ContentDigest::from_bytes([0x5a; 32]));
    let error = dataset_digest
        .migrate_to_current(ProjectId::new())
        .expect_err("schema-v17 dataset identity is authenticated before resealing");
    assert!(
        error.contains("schema-v17 simulation run 18 dataset content digest"),
        "{error}"
    );
}

#[test]
fn schema_v17_rejects_smuggled_measurement_verification_fields() {
    let source = persisted_measurement_at_schema_v17();

    let mut raw_value = source.clone();
    raw_value.runs[0].analyses[0].measurements[0].raw_value = Some(12.0);
    let error = raw_value
        .migrate_to_current(ProjectId::new())
        .expect_err("schema-v17 did not authenticate raw measurement values");
    assert!(
        error.contains("measurement verification evidence introduced by schema v18"),
        "{error}"
    );

    let mut failure_limit = source.clone();
    failure_limit.runs[0].analyses[0].measurements[0].failure_limit = Some(10.0);
    let error = failure_limit
        .migrate_to_current(ProjectId::new())
        .expect_err("schema-v17 did not authenticate FAILVALUE limits");
    assert!(
        error.contains("measurement verification evidence introduced by schema v18"),
        "{error}"
    );

    let mut verdict = source;
    verdict.runs[0].analyses[0].measurements[0].failure_limit_exceeded = true;
    let error = verdict
        .migrate_to_current(ProjectId::new())
        .expect_err("schema-v17 did not authenticate FAILVALUE verdicts");
    assert!(
        error.contains("measurement verification evidence introduced by schema v18"),
        "{error}"
    );
}

#[test]
fn schema_v18_digest_rejects_measurement_verification_tampering() {
    let mut current = persisted_measurement_at_schema_v17();
    current
        .migrate_to_current(ProjectId::new())
        .expect("fixture migrates to schema v18");

    let mut raw_value = current.clone();
    raw_value.runs[0].analyses[0].measurements[0].raw_value = Some(13.0);
    let error = raw_value
        .validate()
        .expect_err("schema-v18 authenticates raw measurement values");
    assert!(
        error.contains("result_data_digest does not match retained analysis content"),
        "{error}"
    );

    let mut raw_value_removed = current.clone();
    raw_value_removed.runs[0].analyses[0].measurements[0].raw_value = None;
    let error = raw_value_removed
        .validate()
        .expect_err("schema-v18 requires explicit raw measurement values");
    assert!(
        error.contains("raw value exactly when it carries a published value"),
        "{error}"
    );

    let mut failure_limit = current;
    failure_limit.runs[0].analyses[0].measurements[0].failure_limit = Some(20.0);
    let error = failure_limit
        .validate()
        .expect_err("schema-v18 authenticates FAILVALUE limits");
    assert!(
        error.contains("result_data_digest does not match retained analysis content"),
        "{error}"
    );
}

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
    simulation.runs = vec![run];
    simulation.next_run_id = 19;
    ProjectSimulationResults::from_state(&simulation)
}

#[test]
fn current_project_results_preserve_projected_failvalue_evidence_without_synthesis() {
    let persisted = current_projected_failvalue_results();
    persisted
        .validate()
        .expect("distinct projected and raw values are valid current evidence");

    let restored = persisted
        .into_simulation_state()
        .expect("current result restores after validation");
    let measurement = &restored.runs[0].analyses[0].measurements[0];
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
    let restored = persisted
        .into_simulation_state()
        .expect("unevaluated FAILVALUE metadata restores");
    let measurement = &restored.runs[0].analyses[0].measurements[0];
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
        crate::state::WaveformData::new("I(V1)", vec![0.0, 1.0], vec![0.0, 1.0e-3], "#00aaff");
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

fn persisted_at_schema_v12(run: &SimulationRun) -> ProjectSimulationResults {
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run.clone()];
    simulation.next_run_id = run.id;
    let mut persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.schema_version = OPERATING_POINT_RESULTS_SCHEMA_VERSION;
    persisted.runs[0].analyses[0].result_data_digest =
        PersistedField::Value(run.analyses[0].legacy_v5_result_data_digest());
    persisted.runs[0].dataset_content_digest =
        PersistedField::Value(run.legacy_v5_dataset_content_digest());
    persisted
}

#[test]
fn schema_v12_is_authenticated_with_its_unit_free_encoding_then_resealed() {
    let run = run_with_waveform_unit(31, None);
    let mut persisted = persisted_at_schema_v12(&run);

    persisted
        .migrate_to_current(ProjectId::new())
        .expect("an authentic schema-v12 result history migrates");

    assert_eq!(
        persisted.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    let migrated = persisted.runs[0]
        .clone()
        .into_run()
        .expect("migrated result restores");
    assert_eq!(
        migrated.analyses[0].waveforms[0].unit, None,
        "a unit absent from a v12 file is never invented during migration"
    );
    assert_eq!(
        persisted.runs[0].analyses[0].result_data_digest.as_ref(),
        Some(&migrated.analyses[0].result_data_digest()),
        "migrated results are resealed in the current digest domain"
    );
    assert_ne!(
        migrated.analyses[0].result_data_digest(),
        run.analyses[0].legacy_v5_result_data_digest()
    );
}

#[test]
fn schema_v12_rejects_a_waveform_unit_its_own_digest_never_covered() {
    let mut persisted = persisted_at_schema_v12(&run_with_waveform_unit(32, None));
    persisted.runs[0].analyses[0].waveforms[0].unit = Some("A".to_owned());

    let error = persisted
        .migrate_to_current(ProjectId::new())
        .expect_err("a v12 file cannot carry a field introduced by v13");

    assert!(
        error.contains("waveform unit introduced by schema v13"),
        "{error}"
    );
}

#[test]
fn schema_v12_rejects_samples_that_do_not_match_its_retained_digest() {
    let mut persisted = persisted_at_schema_v12(&run_with_waveform_unit(33, None));
    persisted.runs[0].analyses[0].waveforms[0].y[1] = 1.000_000_000_000_000_2e-3;

    let error = persisted
        .migrate_to_current(ProjectId::new())
        .expect_err("tampered v12 samples fail their own digest");

    assert!(
        error.contains("schema-v12 analysis 1 result data digest does not match"),
        "{error}"
    );
}

#[test]
fn a_retained_waveform_unit_survives_the_current_schema_round_trip() {
    let run = run_with_waveform_unit(34, Some("A"));
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 34;

    let persisted = ProjectSimulationResults::from_state(&simulation);
    assert_eq!(
        persisted.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    let restored = persisted
        .into_simulation_state()
        .expect("current result history restores");

    assert_eq!(
        restored.runs[0].analyses[0].waveforms[0].unit.as_deref(),
        Some("A")
    );
}

fn persisted_pole_zero_at_schema_v15() -> ProjectSimulationResults {
    let mut run = SimulationRun::new(35);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::PoleZero, "PZ").with_result_payload(
            AnalysisResultPayload::PoleZero {
                poles: vec![crate::state::ComplexResultValue {
                    real: -1.0,
                    imaginary: 0.0,
                }],
                zeros: Vec::new(),
                pole_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
                zero_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
                gain: Some(2.0),
            },
        ),
    );
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 35;
    let mut persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.schema_version = EXECUTED_DECK_RESULTS_SCHEMA_VERSION;
    downgrade_result_digests_to_v6(&mut persisted);
    persisted
}

#[test]
fn schema_v15_pole_zero_results_migrate_only_as_legacy_unknown() {
    let mut authentic = persisted_pole_zero_at_schema_v15();
    authentic
        .migrate_to_current(ProjectId::new())
        .expect("an authentic schema-v15 pole-zero result migrates");
    let PersistedField::Value(AnalysisResultPayload::PoleZero {
        pole_evidence,
        zero_evidence,
        gain,
        ..
    }) = &authentic.runs[0].analyses[0].result_payload
    else {
        panic!("migrated pole-zero payload remains present")
    };
    assert_eq!(*gain, Some(2.0));
    assert!(matches!(
        pole_evidence,
        crate::state::PoleZeroRootSetEvidence::LegacyUnknown
    ));
    assert!(matches!(
        zero_evidence,
        crate::state::PoleZeroRootSetEvidence::LegacyUnknown
    ));

    let mut injected = persisted_pole_zero_at_schema_v15();
    let PersistedField::Value(AnalysisResultPayload::PoleZero { pole_evidence, .. }) =
        &mut injected.runs[0].analyses[0].result_payload
    else {
        panic!("pole-zero fixture")
    };
    *pole_evidence = crate::state::PoleZeroRootSetEvidence::Qualified {
        certificate: crate::state::PoleZeroSpectrumCertificate {
            problem_order: 1,
            infinite_count: 0,
            max_backward_error: 0.0,
            qualification_tolerance:
                crate::state::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1)
                    .unwrap(),
        },
    };
    assert!(
        injected
            .migrate_to_current(ProjectId::new())
            .expect_err("schema-v15 cannot smuggle unauthenticated root evidence")
            .contains("root evidence introduced by schema v16")
    );

    let mut missing_gain = persisted_pole_zero_at_schema_v15();
    let PersistedField::Value(AnalysisResultPayload::PoleZero { gain, .. }) =
        &mut missing_gain.runs[0].analyses[0].result_payload
    else {
        panic!("pole-zero fixture")
    };
    *gain = None;
    assert!(
        missing_gain
            .migrate_to_current(ProjectId::new())
            .expect_err("schema-v15 required a numeric gain authenticated by V6")
            .contains("result data digest does not match")
    );
}

fn persisted_periodic_at_schema_v16(analysis_type: AnalysisType) -> ProjectSimulationResults {
    let mut run = SimulationRun::new(351);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(AnalysisResult::new(
        1,
        analysis_type,
        match analysis_type {
            AnalysisType::Pss => "PSS",
            AnalysisType::Pstb => "PSTB",
            _ => unreachable!(),
        },
    ));
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 351;
    let mut persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.schema_version = POLE_ZERO_EVIDENCE_RESULTS_SCHEMA_VERSION;
    downgrade_result_digests_to_v7(&mut persisted);
    persisted
}

#[test]
fn schema_v16_periodic_results_migrate_to_explicit_unknown_without_inference() {
    for analysis_type in [AnalysisType::Pss, AnalysisType::Pstb] {
        let mut persisted = persisted_periodic_at_schema_v16(analysis_type);
        assert!(persisted.runs[0].analyses[0].waveforms.is_empty());
        persisted
            .migrate_to_current(ProjectId::new())
            .expect("authentic zero-waveform schema-v16 periodic result migrates");
        assert_eq!(
            persisted.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        let payload = persisted.runs[0].analyses[0]
            .result_payload
            .as_ref()
            .expect("migration adds an explicit periodic marker");
        let json = serde_json::to_value(payload).expect("marker serializes");
        assert_eq!(json["floquet_evidence"]["status"], "legacy_unknown");
        assert_eq!(json["orbit_kind"], "legacy_unknown");
        assert_eq!(json["stability_verdict"], "indeterminate");
        assert_eq!(json["multipliers"].as_array().map(Vec::len).unwrap_or(0), 0);
        assert_eq!(json["modes"].as_array().map(Vec::len).unwrap_or(0), 0);
        assert!(json["period_s"].is_null());
        assert!(json["fundamental_frequency_hz"].is_null());
        persisted
            .validate()
            .expect("migrated marker is sealed by the V8 current contract");
    }
}

#[test]
fn schema_v16_cannot_inject_periodic_evidence_or_overwrite_an_existing_payload() {
    let mut injected = persisted_periodic_at_schema_v16(AnalysisType::Pss);
    injected.runs[0].analyses[0].result_payload = PersistedField::Value(
        AnalysisResultPayload::legacy_periodic_marker(AnalysisType::Pss).unwrap(),
    );
    let error = injected
        .migrate_to_current(ProjectId::new())
        .expect_err("schema-v16 cannot inject a payload that its V7 contract never admitted");
    assert!(
        error.contains("periodic stability evidence introduced by schema v17"),
        "{error}"
    );

    let mut nonperiodic = persisted_periodic_at_schema_v16(AnalysisType::Pstb);
    nonperiodic.runs[0].analyses[0].result_payload =
        PersistedField::Value(AnalysisResultPayload::ScalarMeasurements {
            values: std::collections::BTreeMap::from([("legacy".to_owned(), 1.0)]),
        });
    downgrade_result_digests_to_v7(&mut nonperiodic);
    let original = nonperiodic.clone();
    let error = nonperiodic
        .migrate_to_current(ProjectId::new())
        .expect_err("migration must not replace an authenticated nonperiodic payload");
    assert!(
        error.contains("must match successful periodic analysis type"),
        "{error}"
    );
    assert_eq!(nonperiodic, original, "failed migration is transactional");
}

#[test]
fn current_schema_rejects_successful_periodic_results_without_the_typed_payload() {
    for analysis_type in [AnalysisType::Pss, AnalysisType::Pstb] {
        let mut current = persisted_periodic_at_schema_v16(analysis_type);
        current.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
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
fn schema_v13_migrates_prepared_receipts_to_an_explicit_default_specification_policy() {
    let plan_id = SimulationPlanId::new();
    let mut run = SimulationRun::new(35);
    let snapshot = ContentDigest::from_bytes([0xa0; 32]);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_provenance(
            AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                snapshot,
                Vec::new(),
            )
            .expect("prepared provenance"),
        ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0xa1; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xa2; 32])),
        &[analysis_kind_tag_for_plan_kind(AnalysisKind::Ac)],
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 35;
    let mut persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.schema_version = WAVEFORM_UNIT_RESULTS_SCHEMA_VERSION;
    downgrade_result_digests_to_v6(&mut persisted);
    let PersistedField::Value(receipt) = &mut persisted.runs[0].prepared_receipt else {
        panic!("prepared fixture has a receipt");
    };
    receipt.specification_policy = PersistedField::Missing;

    persisted
        .migrate_to_current(ProjectId::new())
        .expect("an authentic schema-v13 prepared receipt migrates");

    assert_eq!(
        persisted.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    let PersistedField::Value(receipt) = &persisted.runs[0].prepared_receipt else {
        panic!("migrated prepared receipt remains present");
    };
    assert!(matches!(
        receipt.specification_policy,
        PersistedField::Value(ref policy) if policy.bitwise_eq(&SpecificationPolicy::default())
    ));
}

#[test]
fn schema_v13_rejects_governed_specification_fields_from_a_later_schema() {
    let mut run = SimulationRun::new(36);
    let snapshot = ContentDigest::from_bytes([0xa5; 32]);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_provenance(
            AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                snapshot,
                Vec::new(),
            )
            .expect("prepared provenance"),
        ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0xa3; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xa4; 32])),
        &[analysis_kind_tag_for_plan_kind(AnalysisKind::Ac)],
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 36;
    let mut smuggled = ProjectSimulationResults::from_state(&simulation);
    smuggled.schema_version = WAVEFORM_UNIT_RESULTS_SCHEMA_VERSION;

    let error = smuggled
        .migrate_to_current(ProjectId::new())
        .expect_err("schema-v13 cannot carry a schema-v14 policy field");
    assert!(
        error.contains("governed specification fields introduced by schema v14"),
        "{error}"
    );
}

#[test]
fn current_schema_requires_coherent_lifecycle_and_execution_identity() {
    let mut run = SimulationRun::new(15);
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 15;
    let current = ProjectSimulationResults::from_state(&simulation);
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
        simulation.runs = vec![run];
        simulation.next_run_id = sequence;

        let persisted = ProjectSimulationResults::from_state(&simulation);
        assert!(!persisted.runs[0].success);
        persisted
            .validate()
            .expect("nonterminal snapshot validates");
        let restored = persisted
            .into_simulation_state()
            .expect("nonterminal snapshot restores fail-closed");
        let restored = &restored.runs[0];

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
    simulation.runs = vec![run];
    simulation.next_run_id = 21;

    let persisted = ProjectSimulationResults::from_state(&simulation);
    let json = serde_json::to_string(&persisted).expect("results serialize");
    let decoded: ProjectSimulationResults =
        serde_json::from_str(&json).expect("results deserialize");
    let restored = decoded
        .into_simulation_state()
        .expect("provenance validates and restores");
    let restored_run = &restored.runs[0];

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
    let source_id =
        crate::product::manual_deck_analysis_instance_id_from_tag(source_content_digest, 5, 0);
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
    simulation.runs = vec![run];
    simulation.next_run_id = 29;

    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project = ProjectFile::new_with_simulation_results(
        workspace,
        libraries,
        ProjectSimulationResults::from_state(&simulation),
    );

    let json = serialize_project_file(&project)
        .expect("manual-deck result does not require a simulation-plan owner");
    let loaded = load_project_text(&json, None).expect("manual-deck project reloads");
    let restored = loaded
        .simulation_results
        .into_simulation_state()
        .expect("manual-deck result history restores");
    let provenance = restored.runs[0].analyses[0]
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
fn schema_v4_provenance_migrates_without_guessing_its_source_domain() {
    let source_id = AnalysisInstanceId::new();
    let mut run = SimulationRun::new(30);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "Legacy prepared AC").with_provenance(
            AnalysisResultProvenance::new(
                source_id,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x94; 32]),
                Vec::new(),
            )
            .expect("legacy prepared provenance fixture"),
        ),
    );
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x93; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x92; 32])),
        &[2],
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 30;
    let mut persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.schema_version = EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION;
    clear_v6_execution_fields(&mut persisted);
    persisted.runs[0].prepared_receipt = PersistedField::Missing;
    for provenance in persisted.runs[0]
        .analyses
        .iter_mut()
        .filter_map(|analysis| analysis.provenance.as_mut())
    {
        provenance.source_domain = PersistedField::Missing;
    }
    let v4_json = serde_json::to_string(&persisted).expect("schema-v4 fixture serializes");
    assert!(!v4_json.contains("source_domain"));
    let mut persisted: ProjectSimulationResults =
        serde_json::from_str(&v4_json).expect("schema-v4 fixture deserializes");

    persisted
        .migrate_to_current(ProjectId::new())
        .expect("schema v4 migrates");
    persisted.validate().expect("migrated schema validates");

    assert_eq!(
        persisted.runs[0].provenance_mode,
        PersistedField::Value(ProjectRunProvenanceMode::LegacyPreparedUnclassified)
    );
    assert_eq!(
        persisted.runs[0].analyses[0]
            .provenance
            .as_ref()
            .expect("provenance retained")
            .source_domain,
        PersistedField::Value(AnalysisResultSourceDomain::LegacyUnclassified)
    );
}

#[test]
fn legacy_result_schema_truth_cannot_be_repaired_or_downgraded() {
    let source_id = AnalysisInstanceId::new();
    let mut run = SimulationRun::new(33);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "Prepared AC").with_provenance(
            AnalysisResultProvenance::new(
                source_id,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0xa3; 32]),
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
        ContentDigest::from_bytes([0xa2; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xa1; 32])),
        &[2],
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 33;
    let mut v4 = ProjectSimulationResults::from_state(&simulation);
    v4.schema_version = EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION;
    clear_v6_execution_fields(&mut v4);
    v4.runs[0].prepared_receipt = PersistedField::Missing;
    v4.runs[0].provenance_mode = PersistedField::Value(ProjectRunProvenanceMode::PreparedTaskBound);
    v4.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_domain = PersistedField::Missing;

    let mut stripped = v4.clone();
    stripped.runs[0].analyses[0].provenance = None;
    assert!(
        stripped
            .migrate_to_current(ProjectId::new())
            .expect_err("v4 prepared mode cannot be laundered after provenance stripping")
            .contains("complete provenance is missing")
    );

    let mut contradictory = v4.clone();
    contradictory.runs[0].provenance_mode =
        PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed);
    assert!(
        contradictory
            .migrate_to_current(ProjectId::new())
            .expect_err("v4 legacy mode cannot contain prepared provenance")
            .contains("legacy_unattributed")
    );

    let mut missing_mode = v4.clone();
    missing_mode.runs[0].provenance_mode = PersistedField::Missing;
    assert!(
        missing_mode
            .migrate_to_current(ProjectId::new())
            .expect_err("v4 mode is authoritative and required")
            .contains("provenance_mode is required")
    );

    let mut downgraded = v4.clone();
    downgraded.schema_version = PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION;
    downgraded.runs[0].provenance_mode = PersistedField::Missing;
    downgraded.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_domain = PersistedField::Value(AnalysisResultSourceDomain::SimulationPlan);
    assert!(
        downgraded
            .migrate_to_current(ProjectId::new())
            .expect_err("new source-domain data cannot masquerade as schema v3")
            .contains("introduced after schema v3")
    );

    for schema_version in [
        LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION,
        STABLE_DATASET_RESULTS_SCHEMA_VERSION,
    ] {
        let mut impossible = v4.clone();
        impossible.schema_version = schema_version;
        impossible.runs[0].provenance_mode = PersistedField::Missing;
        impossible.runs[0].analyses[0]
            .provenance
            .as_mut()
            .expect("provenance")
            .source_domain = PersistedField::Missing;
        if schema_version == LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION {
            impossible.runs[0].run_id = None;
            impossible.runs[0].dataset_id = None;
            impossible.active_run_stable_id = None;
            impossible.active_dataset_id = None;
            impossible.active_analysis_sequence = None;
        }
        assert!(
            impossible
                .migrate_to_current(ProjectId::new())
                .expect_err("v1/v2 cannot contain prepared provenance")
                .contains("prepared provenance introduced after")
        );
    }
}

#[test]
fn legacy_result_schema_rejects_present_or_null_later_era_fields() {
    let mut run = SimulationRun::new(34);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "Prepared AC").with_provenance(
            AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0xb3; 32]),
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
        ContentDigest::from_bytes([0xb2; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xb1; 32])),
        &[2],
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 34;
    let current = ProjectSimulationResults::from_state(&simulation);

    for schema_version in [
        LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION,
        STABLE_DATASET_RESULTS_SCHEMA_VERSION,
        PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION,
        EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION,
    ] {
        let mut with_receipt = current.clone();
        with_receipt.schema_version = schema_version;
        clear_v6_execution_fields(&mut with_receipt);
        clear_v14_specification_fields(&mut with_receipt);
        assert!(
            with_receipt
                .migrate_to_current(ProjectId::new())
                .expect_err("schemas v1-v4 cannot carry a v5 prepared receipt")
                .contains("prepared run receipt introduced after")
        );

        let mut null_receipt_json =
            serde_json::to_value(&current).expect("current result document serializes");
        null_receipt_json["schema_version"] = serde_json::json!(schema_version);
        clear_v6_execution_fields_json(&mut null_receipt_json);
        null_receipt_json["runs"][0]["prepared_receipt"] = serde_json::Value::Null;
        let mut with_null_receipt: ProjectSimulationResults =
            serde_json::from_value(null_receipt_json)
                .expect("explicit null receipt remains parseable evidence");
        assert!(with_null_receipt.runs[0].prepared_receipt.is_null());
        assert!(
            with_null_receipt
                .migrate_to_current(ProjectId::new())
                .expect_err("an explicit null receipt is still an anachronistic field")
                .contains("prepared run receipt introduced after")
        );
    }

    let mut null_v3_mode_json =
        serde_json::to_value(&current).expect("current result document serializes");
    null_v3_mode_json["schema_version"] =
        serde_json::json!(PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION);
    clear_v6_execution_fields_json(&mut null_v3_mode_json);
    null_v3_mode_json["runs"][0]["provenance_mode"] = serde_json::Value::Null;
    let mut null_v3_mode: ProjectSimulationResults =
        serde_json::from_value(null_v3_mode_json).expect("null mode evidence deserializes");
    null_v3_mode.runs[0].prepared_receipt = PersistedField::Missing;
    assert!(null_v3_mode.runs[0].provenance_mode.is_null());
    null_v3_mode.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_domain = PersistedField::Missing;
    assert!(
        null_v3_mode
            .migrate_to_current(ProjectId::new())
            .expect_err("an explicit null mode is present in schema v3")
            .contains("provenance_mode introduced after schema v3")
    );

    let mut null_v4_mode = current.clone();
    null_v4_mode.schema_version = EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION;
    clear_v6_execution_fields(&mut null_v4_mode);
    null_v4_mode.runs[0].prepared_receipt = PersistedField::Missing;
    null_v4_mode.runs[0].provenance_mode = PersistedField::Null;
    null_v4_mode.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_domain = PersistedField::Missing;
    assert!(
        null_v4_mode
            .migrate_to_current(ProjectId::new())
            .expect_err("schema v4 requires a non-null authoritative mode")
            .contains("provenance_mode is required by schema v4")
    );

    let mut null_v4_source_json =
        serde_json::to_value(&current).expect("current result document serializes");
    null_v4_source_json["schema_version"] =
        serde_json::json!(EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION);
    clear_v6_execution_fields_json(&mut null_v4_source_json);
    null_v4_source_json["runs"][0]["analyses"][0]["provenance"]["source_domain"] =
        serde_json::Value::Null;
    let mut null_v4_source_domain: ProjectSimulationResults =
        serde_json::from_value(null_v4_source_json)
            .expect("null source-domain evidence deserializes");
    null_v4_source_domain.runs[0].prepared_receipt = PersistedField::Missing;
    assert!(
        null_v4_source_domain.runs[0].analyses[0]
            .provenance
            .as_ref()
            .expect("provenance")
            .source_domain
            .is_null()
    );
    assert!(
        null_v4_source_domain
            .migrate_to_current(ProjectId::new())
            .expect_err("an explicit null source domain is still a v5 field")
            .contains("source_domain was introduced after schema v4")
    );
}

#[test]
fn failed_legacy_result_migration_is_transactional() {
    let mut run = SimulationRun::new(35);
    run.add_analysis(AnalysisResult::new(
        1,
        AnalysisType::Transient,
        "Legacy TRAN",
    ));
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 35;
    let mut legacy = ProjectSimulationResults::from_state(&simulation);
    legacy.schema_version = LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION;
    clear_v6_execution_fields(&mut legacy);
    legacy.runs[0].run_id = None;
    legacy.runs[0].dataset_id = None;
    legacy.runs[0].provenance_mode = PersistedField::Missing;
    legacy.active_run_stable_id = None;
    legacy.active_dataset_id = None;
    legacy.active_analysis_sequence = None;
    legacy.active_run_id = Some(999);
    legacy.active_analysis_id = Some(1);
    let before = legacy.clone();

    let error = legacy
        .migrate_to_current(ProjectId::new())
        .expect_err("invalid late selection reference must abort migration");

    assert!(error.contains("run sequence 999 does not exist"));
    assert_eq!(legacy, before, "failed migration must not mutate any field");
}

#[test]
fn schema_v1_result_identity_migration_is_reproducible() {
    let mut run = SimulationRun::new(18);
    run.add_analysis(AnalysisResult::new(
        1,
        AnalysisType::Transient,
        "Legacy TRAN",
    ));
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 18;
    let mut first = ProjectSimulationResults::from_state(&simulation);
    first.schema_version = LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION;
    clear_v6_execution_fields(&mut first);
    first.runs[0].provenance_mode = PersistedField::Missing;
    first.runs[0].run_id = None;
    first.runs[0].dataset_id = None;
    first.active_run_stable_id = None;
    first.active_dataset_id = None;
    first.active_analysis_sequence = None;
    first.active_run_id = Some(18);
    first.active_analysis_id = Some(1);
    let mut second = first.clone();
    let mut other_project = first.clone();

    let project_id = ProjectId::new();
    first
        .migrate_to_current(project_id)
        .expect("first migration succeeds");
    second
        .migrate_to_current(project_id)
        .expect("identical migration succeeds");
    other_project
        .migrate_to_current(ProjectId::new())
        .expect("other project migration succeeds");

    assert_eq!(first.runs[0].run_id, second.runs[0].run_id);
    assert_eq!(first.runs[0].dataset_id, second.runs[0].dataset_id);
    assert_eq!(first.active_run_stable_id, second.active_run_stable_id);
    assert_eq!(first.active_dataset_id, second.active_dataset_id);
    assert_ne!(first.runs[0].run_id, other_project.runs[0].run_id);
    assert_ne!(first.runs[0].dataset_id, other_project.runs[0].dataset_id);
}

#[test]
fn project_save_requires_result_provenance_to_be_closed_over_plan_or_tombstones() {
    let mut project = project_with_execution_context();
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
        project.workspace.project.revision(),
        ContentDigest::from_bytes([0x5b; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x5a; 32])),
        &[2],
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 31;
    project.simulation_results = ProjectSimulationResults::from_state(&simulation);

    serialize_project_file(&project).expect("current plan owns result source");

    let mut future_revision = project.clone();
    let future_source_revision =
        ObjectRevision::new(source_revision.get() + 1).expect("future fixture revision");
    future_revision.simulation_results.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_revision = future_source_revision;
    future_revision.simulation_results.runs[0]
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
    missing.simulation_results.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_instance_id = orphaned_source_id;
    missing.simulation_results.runs[0]
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
    at_removal.simulation_results.runs[0].analyses[0]
        .provenance
        .as_mut()
        .expect("provenance")
        .source_revision = removed_revision;
    at_removal.simulation_results.runs[0]
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
        project.workspace.project.revision(),
        ContentDigest::from_bytes([0xc2; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xc1; 32])),
        &[2],
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 36;
    project.simulation_results = ProjectSimulationResults::from_state(&simulation);

    let error = serialize_project_file(&project)
        .expect_err("source cannot own results from before it existed")
        .to_string();
    assert!(
        error.contains("outside the retained analysis revision interval"),
        "{error}"
    );
}

#[test]
fn v2_same_kind_results_migrate_without_guessing_source_identity() {
    let mut run = SimulationRun::new(22);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC low band"));
    run.add_analysis(AnalysisResult::new(2, AnalysisType::Ac, "AC high band"));
    seal_legacy_unattributed(&mut run);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 22;
    let mut persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.schema_version = STABLE_DATASET_RESULTS_SCHEMA_VERSION;
    clear_v6_execution_fields(&mut persisted);
    persisted.runs[0].provenance_mode = PersistedField::Missing;

    persisted
        .migrate_to_current(ProjectId::new())
        .expect("v2 migrates");

    assert_eq!(
        persisted.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    assert!(
        persisted.runs[0]
            .analyses
            .iter()
            .all(|analysis| analysis.provenance.is_none())
    );
    let restored = persisted
        .into_simulation_state()
        .expect("legacy absence remains valid");
    assert!(
        restored.runs[0]
            .analyses
            .iter()
            .all(|analysis| analysis.provenance.is_none())
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
    simulation.runs = vec![run];
    simulation.next_run_id = 23;
    let baseline = ProjectSimulationResults::from_state(&simulation);

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

/// A completed transient whose retained events are two conductors, sealed the
/// way a schema-v18 file was: with the V9 encoding, which had no bus table.
fn persisted_events_at_schema_v18() -> ProjectSimulationResults {
    use crate::state::{DigitalEventPointEvidence, DigitalEventTraceEvidence};

    let trace = |name: &str| DigitalEventTraceEvidence {
        node_name: name.to_owned(),
        points: vec![
            DigitalEventPointEvidence {
                time_s: 0.0,
                value_code: 0,
            },
            DigitalEventPointEvidence {
                time_s: 5.0e-9,
                value_code: 1,
            },
        ],
    };
    let mut run = SimulationRun::new(19);
    run.mark_running().expect("fixture run starts");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("fixture run completes");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
            AnalysisResultPayload::TransientEvents {
                digital_traces: vec![trace("count#1"), trace("count#0")],
                real_traces: Vec::new(),
                digital_buses: Vec::new(),
            },
        ),
    );
    seal_legacy_unattributed(&mut run);

    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 19;
    let mut persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.schema_version = MEASUREMENT_VERIFICATION_RESULTS_SCHEMA_VERSION;
    for persisted_run in &mut persisted.runs {
        let restored = persisted_run
            .clone()
            .into_run()
            .expect("event fixture restores before the V9 digest downgrade");
        for (persisted_analysis, analysis) in
            persisted_run.analyses.iter_mut().zip(&restored.analyses)
        {
            persisted_analysis.result_data_digest =
                PersistedField::Value(analysis.legacy_v9_result_data_digest());
        }
        persisted_run.dataset_content_digest =
            PersistedField::Value(restored.legacy_v9_dataset_content_digest());
    }
    persisted
}

#[test]
fn schema_v18_is_authenticated_then_resealed_with_an_empty_bus_table() {
    let mut persisted = persisted_events_at_schema_v18();
    let legacy_digest = persisted.runs[0].analyses[0]
        .result_data_digest
        .as_ref()
        .copied()
        .expect("schema-v18 fixture carries a digest");

    persisted
        .migrate_to_current(ProjectId::new())
        .expect("an authentic schema-v18 event history migrates");

    assert_eq!(
        persisted.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    let Some(AnalysisResultPayload::TransientEvents { digital_buses, .. }) =
        persisted.runs[0].analyses[0].result_payload.as_ref()
    else {
        panic!("the migrated analysis still carries its event history");
    };
    assert!(
        digital_buses.is_empty(),
        "a file written before buses existed declares none"
    );
    let current_digest = persisted.runs[0].analyses[0]
        .result_data_digest
        .as_ref()
        .copied()
        .expect("migrated fixture carries a current digest");
    assert_ne!(legacy_digest, current_digest);
    persisted.validate().expect("migrated schema validates");
}

#[test]
fn schema_v18_rejects_tampering_and_a_smuggled_bus_table() {
    let mut tampered = persisted_events_at_schema_v18();
    let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
        tampered.runs[0].analyses[0].result_payload.as_mut()
    else {
        panic!("event payload");
    };
    digital_traces[0].points[1].value_code = 2;
    let error = tampered
        .migrate_to_current(ProjectId::new())
        .expect_err("schema-v18 content must match its retained V9 digest");
    assert!(
        error.contains("schema-v18 analysis 1 result data digest"),
        "{error}"
    );

    let mut smuggled = persisted_events_at_schema_v18();
    let Some(AnalysisResultPayload::TransientEvents { digital_buses, .. }) =
        smuggled.runs[0].analyses[0].result_payload.as_mut()
    else {
        panic!("event payload");
    };
    digital_buses.push(crate::state::DigitalBusEvidence {
        name: "count".to_owned(),
        msb: 1,
        lsb: 0,
        members: vec!["count#1".to_owned(), "count#0".to_owned()],
        source: crate::state::DigitalBusSourceEvidence::Engine,
    });
    let error = smuggled
        .migrate_to_current(ProjectId::new())
        .expect_err("a schema-v18 file cannot have declared a bus");
    assert!(
        error.contains("digital bus 'count', which was introduced by schema v19"),
        "{error}"
    );
}

/// A file that says it was written by a later build is refused rather than
/// read as if this build understood it.
#[test]
fn a_results_schema_from_the_future_is_refused_by_number() {
    let mut ahead = persisted_events_at_schema_v18();
    ahead.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION + 1;
    let error = ahead
        .migrate_to_current(ProjectId::new())
        .expect_err("a forward schema version is not migrated");
    assert!(
        error.contains("unsupported simulation results schema version 20"),
        "{error}"
    );
}
