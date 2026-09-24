//! Tests for correlation review gates and bounded state.
//!
//! Advisory failures stay visible without failing the gate, a dataset-only
//! suite is a valid draft that cannot produce evidence, and serde failures
//! leave the existing history unrewritten.

use super::*;
use crate::product::ModelSourceId;

fn source(byte: u8) -> ModelSourceEvidenceBinding {
    ModelSourceEvidenceBinding::try_new_project_bound(
        "dut",
        ModelSourceId::new(),
        ContentDigest::from_bytes([byte; 32]),
        ObjectRevision::INITIAL,
    )
    .expect("source fixture")
}

fn reference_csv() -> Vec<u8> {
    b"id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz],condition:temperature[degC]\n\
r1,gain,0,dB,0.05,1,10,27\n\
r2,gain,-1,dB,0.05,1,100,27\n\
r3,gain,-2,dB,0.05,1,1000,27\n"
            .to_vec()
}

fn simulation_csv(offset: f64) -> Vec<u8> {
    format!(
            "id,quantity,value,unit,uncertainty,weight,condition:frequency[kHz],condition:temperature[K]\n\
s1,gain,{offset},dB,0.04,1,0.01,300.15\n\
s2,gain,{},dB,0.04,1,0.1,300.15\n\
s3,gain,{},dB,0.04,1,1,300.15\n",
            -1.0 + offset,
            -2.0 + offset,
        )
        .into_bytes()
}

fn dataset(
    id: &str,
    class: CorrelationDatasetClass,
    bytes: Vec<u8>,
    source: Option<ModelSourceEvidenceBinding>,
) -> CorrelationDatasetRevision {
    let provenance = class
        .is_simulation()
        .then(|| CorrelationSimulationProvenance {
            run_id: format!("{id}-run"),
            run_dataset_id: format!("{id}-run-dataset"),
            analysis_id: 1,
            analysis_result_digest: ContentDigest::from_bytes([0x43; 32]),
            plan_id: "correlation-plan".to_owned(),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: ContentDigest::from_bytes([0x44; 32]),
            source_content_digest: ContentDigest::from_bytes([0x45; 32]),
            task_config_digest: ContentDigest::from_bytes([0x55; 32]),
            execution_target: "test-platform".to_owned(),
            export_digest: digest(&bytes),
            model_source: source.clone().expect("simulation source fixture"),
            executed_at_unix_ms: 1,
        });
    CorrelationDatasetRevision::try_from_csv_with_provenance(
        id,
        ObjectRevision::INITIAL,
        id,
        class,
        "qualified test authority",
        "lot-1",
        "fixture-1",
        "calibration-1",
        format!("{id}.csv"),
        bytes,
        source,
        provenance,
    )
    .expect("dataset fixture")
}

fn metric(
    aggregation: CorrelationAggregation,
    alignment: CorrelationAlignmentPolicy,
) -> CorrelationMetricDefinition {
    CorrelationMetricDefinition::try_new(
        "gain-error",
        "Gain error",
        "reference",
        "simulation",
        "gain",
        CorrelationCalculation::AbsoluteDecibels,
        None,
        0.25,
        1.0,
        0.5,
        aggregation,
        alignment,
        CorrelationReleaseRole::Review,
    )
    .expect("metric fixture")
}

fn suite_with(
    source: ModelSourceEvidenceBinding,
    simulation: Vec<u8>,
    metric: CorrelationMetricDefinition,
    dispositions: Vec<CorrelationOutlierDisposition>,
) -> CorrelationSuite {
    CorrelationSuite::try_new(
        "correlation",
        ObjectRevision::INITIAL,
        "DUT correlation",
        "model-owner",
        source.clone(),
        vec![
            dataset(
                "reference",
                CorrelationDatasetClass::BenchMeasurement,
                reference_csv(),
                None,
            ),
            dataset(
                "simulation",
                CorrelationDatasetClass::ModelSimulation,
                simulation,
                Some(source),
            ),
        ],
        vec![metric],
        dispositions,
    )
    .expect("suite fixture")
}

#[test]
fn csv_import_is_bounded_utf8_bom_aware_and_content_addressed() {
    let mut csv = b"\xEF\xBB\xBF".to_vec();
    csv.extend(reference_csv());
    let dataset = dataset(
        "reference",
        CorrelationDatasetClass::BenchMeasurement,
        csv.clone(),
        None,
    );
    assert_eq!(dataset.observations.len(), 3);
    assert_eq!(dataset.raw_digest, digest(&csv));
    assert_eq!(
        dataset.observations[0].coordinates[0].dimension,
        "frequency"
    );
    assert_eq!(
        convert_value(
            dataset.observations[0].coordinates[1].value.get(),
            "degC",
            "K",
        )
        .unwrap(),
        300.15
    );

    let invalid = CorrelationDatasetRevision::try_from_csv(
        "bad",
        ObjectRevision::INITIAL,
        "bad",
        CorrelationDatasetClass::BenchMeasurement,
        "authority",
        "lot",
        "fixture",
        "calibration",
        "bad.csv",
        b"id,quantity,value,unit\nx,q,NaN,V\n".to_vec(),
        None,
    )
    .unwrap_err();
    assert_eq!(invalid.code, CorrelationErrorCode::InvalidNumber);
}

#[test]
fn dataset_validation_detects_raw_or_normalized_tampering() {
    let mut tampered_raw = dataset(
        "reference",
        CorrelationDatasetClass::BenchMeasurement,
        reference_csv(),
        None,
    );
    tampered_raw.raw_source.push(b' ');
    assert_eq!(
        tampered_raw.validate("dataset").unwrap_err().code,
        CorrelationErrorCode::SourceDigestMismatch
    );

    let mut tampered_normalized = dataset(
        "reference",
        CorrelationDatasetClass::BenchMeasurement,
        reference_csv(),
        None,
    );
    tampered_normalized.observations[0].value = FiniteValue::new(9.0).unwrap();
    assert_eq!(
        tampered_normalized.validate("dataset").unwrap_err().code,
        CorrelationErrorCode::SourceDigestMismatch
    );

    let mut tampered_provenance = dataset(
        "simulation",
        CorrelationDatasetClass::ModelSimulation,
        simulation_csv(0.0),
        Some(source(9)),
    );
    tampered_provenance
        .simulation_provenance
        .as_mut()
        .unwrap()
        .export_digest = ContentDigest::from_bytes([0x99; 32]);
    assert_eq!(
        tampered_provenance.validate("dataset").unwrap_err().code,
        CorrelationErrorCode::SourceDigestMismatch
    );
}

#[test]
fn exact_alignment_converts_condition_units_and_evaluates_uncertainty() {
    let source = source(1);
    let suite = suite_with(
        source,
        simulation_csv(0.1),
        metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        ),
        Vec::new(),
    );
    let evaluation = CorrelationEvaluation::evaluate(&suite).expect("evaluation");
    assert!(evaluation.passed);
    let outcome = &evaluation.metric_outcomes[0];
    assert_eq!(outcome.evaluated_points, 3);
    assert_eq!(outcome.excluded_points, 0);
    assert!(outcome.aggregate_error.get() > 0.09);
    assert!(outcome.aggregate_normalized_error.get() < 1.0);
}

#[test]
fn absolute_linear_metrics_support_compound_noise_density_units() {
    let source = source(21);
    let reference = dataset(
        "reference",
        CorrelationDatasetClass::BenchMeasurement,
        "id,quantity,value,unit,uncertainty,condition:frequency[Hz]\n\
r1,noise_density,5,nV/√Hz,0.1,1000\n"
            .as_bytes()
            .to_vec(),
        None,
    );
    let simulation = dataset(
        "simulation",
        CorrelationDatasetClass::ModelSimulation,
        "id,quantity,value,unit,uncertainty,condition:frequency[kHz]\n\
s1,noise_density,0.0051,uV/√Hz,0.00005,1\n"
            .as_bytes()
            .to_vec(),
        Some(source.clone()),
    );
    let metric = CorrelationMetricDefinition::try_new(
        "noise-density-error",
        "Noise density error",
        "reference",
        "simulation",
        "noise_density",
        CorrelationCalculation::AbsoluteLinear,
        None,
        0.5,
        1.0,
        1.0,
        CorrelationAggregation::EveryPoint,
        CorrelationAlignmentPolicy::ExactOnly,
        CorrelationReleaseRole::Review,
    )
    .unwrap();
    let suite = CorrelationSuite::try_new(
        "noise-correlation",
        ObjectRevision::INITIAL,
        "Noise density correlation",
        "model-owner",
        source,
        vec![reference, simulation],
        vec![metric],
        Vec::new(),
    )
    .unwrap();
    let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
    let outcome = &evaluation.metric_outcomes[0];
    assert!(evaluation.passed);
    assert!((outcome.aggregate_error.get() - 0.1).abs() < 1.0e-12);
    assert!(outcome.aggregate_normalized_error.get() < 1.0);
}

#[test]
fn interpolation_is_deterministic_and_extrapolation_fails_closed() {
    let source_binding = source(2);
    let simulation = b"id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz],condition:temperature[degC]\n\
s1,gain,0,dB,0,1,10,27\n\
s2,gain,-2,dB,0,1,190,27\n\
s3,gain,-2,dB,0,1,1000,27\n"
            .to_vec();
    let suite = suite_with(
        source_binding,
        simulation,
        metric(
            CorrelationAggregation::WorstCondition,
            CorrelationAlignmentPolicy::MonotoneInterpolation {
                axis: "frequency".to_owned(),
                extrapolation: CorrelationExtrapolationPolicy::Forbid,
            },
        ),
        Vec::new(),
    );
    let evaluation = CorrelationEvaluation::evaluate(&suite).expect("interpolation");
    assert!(evaluation.passed);
    let middle = evaluation.metric_outcomes[0]
        .residuals
        .iter()
        .find(|point| point.reference_observation_id == "r2")
        .unwrap();
    assert!(middle.metric_error.get() < 0.1);

    let mut outside = suite.clone();
    outside.datasets[0] = dataset(
            "reference",
            CorrelationDatasetClass::BenchMeasurement,
            b"id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz],condition:temperature[degC]\n\
r0,gain,0,dB,0,1,1,27\n"
                .to_vec(),
            None,
        );
    assert_eq!(
        CorrelationEvaluation::evaluate(&outside).unwrap_err().code,
        CorrelationErrorCode::ExtrapolationForbidden
    );

    let mut missing_axis_metric = metric(
        CorrelationAggregation::EveryPoint,
        CorrelationAlignmentPolicy::ExactOnly,
    );
    missing_axis_metric.domain = Some(CorrelationMetricDomain {
        axis: "supply".to_owned(),
        unit: "V".to_owned(),
        minimum: FiniteValue::new(0.0).unwrap(),
        maximum: FiniteValue::new(2.0).unwrap(),
    });
    let missing_axis = suite_with(
        source(22),
        simulation_csv(0.0),
        missing_axis_metric,
        Vec::new(),
    );
    assert_eq!(
        CorrelationEvaluation::evaluate(&missing_axis)
            .unwrap_err()
            .code,
        CorrelationErrorCode::MetricInvalid
    );
}

#[test]
fn indexed_alignment_propagates_condition_unit_mismatches() {
    let simulation = b"id,quantity,value,unit,uncertainty,weight,condition:frequency[kHz],condition:temperature[V]\n\
s1,gain,0,dB,0.04,1,0.01,300.15\n\
s2,gain,-1,dB,0.04,1,0.1,300.15\n\
s3,gain,-2,dB,0.04,1,1,300.15\n"
            .to_vec();
    let suite = suite_with(
        source(23),
        simulation,
        metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        ),
        Vec::new(),
    );
    assert_eq!(
        CorrelationEvaluation::evaluate(&suite).unwrap_err().code,
        CorrelationErrorCode::UnitMismatch
    );
}

#[test]
fn extrapolation_uses_a_conservative_uncertainty_bound() {
    let aligned = |id: &str, uncertainty: f64| AlignedObservation {
        value: 0.0,
        uncertainty,
        weight: 1.0,
        observation_ids: vec![id.to_owned()],
        evidence: CorrelationAlignmentEvidence::Exact,
    };
    let extrapolated = interpolate(
        (0.0, aligned("left", 1.0)),
        (1.0, aligned("right", 2.0)),
        -1.0,
        CorrelationAlignmentEvidence::Extrapolated,
    )
    .unwrap();
    assert_eq!(extrapolated.uncertainty, 4.0);
    assert_eq!(extrapolated.weight, 1.0);

    let interpolated = interpolate(
        (0.0, aligned("left", 1.0)),
        (1.0, aligned("right", 2.0)),
        0.5,
        CorrelationAlignmentEvidence::Interpolated,
    )
    .unwrap();
    assert_eq!(interpolated.uncertainty, 1.5);
}

#[test]
fn phase_wrapping_and_nearest_rank_percentile_are_exact() {
    let reference = CorrelationObservation {
        id: "phase".to_owned(),
        quantity: "phase".to_owned(),
        value: FiniteValue::new(179.0).unwrap(),
        unit: "deg".to_owned(),
        uncertainty: NonNegativeFinite::new(0.0).unwrap(),
        weight: NonNegativeFinite::new(1.0).unwrap(),
        coordinates: Vec::new(),
    };
    let simulated = AlignedObservation {
        value: -179.0,
        uncertainty: 0.0,
        weight: 1.0,
        observation_ids: vec!["phase-simulation".to_owned()],
        evidence: CorrelationAlignmentEvidence::Exact,
    };
    let (error, _) = metric_error(
        CorrelationCalculation::PhaseWrappedDegrees,
        &reference,
        &simulated,
    )
    .unwrap();
    assert_eq!(error, 2.0);

    let residual = |id: usize, normalized: f64| CorrelationResidualPoint {
        id: id.to_string(),
        metric_id: "metric".to_owned(),
        reference_observation_id: id.to_string(),
        reference_value: FiniteValue::new(0.0).unwrap(),
        simulated_value: FiniteValue::new(0.0).unwrap(),
        simulation_observation_ids: vec![format!("sim-{id}")],
        alignment_evidence: CorrelationAlignmentEvidence::Exact,
        metric_error: NonNegativeFinite::new(normalized).unwrap(),
        effective_limit: NonNegativeFinite::new(1.0).unwrap(),
        normalized_error: NonNegativeFinite::new(normalized).unwrap(),
        weight: NonNegativeFinite::new(1.0).unwrap(),
        condition_group: "all-conditions".to_owned(),
        excluded: false,
        exclusion_disposition_id: None,
    };
    let residuals = (1..=20)
        .map(|value| residual(value, value as f64))
        .collect::<Vec<_>>();
    assert_eq!(
        aggregate_residuals(
            &residuals,
            CorrelationCalculation::Relative,
            CorrelationAggregation::Percentile95,
        ),
        (19.0, 19.0)
    );

    let mut unequal = vec![residual(21, 0.0), residual(22, 2.0)];
    let rms = aggregate_residuals(
        &unequal,
        CorrelationCalculation::Relative,
        CorrelationAggregation::RootMeanSquare,
    );
    assert!((rms.0 - 2.0_f64.sqrt()).abs() < 1.0e-12);
    assert_eq!(
        aggregate_residuals(
            &unequal,
            CorrelationCalculation::Relative,
            CorrelationAggregation::EveryPoint,
        ),
        (2.0, 2.0)
    );
    let worst_condition = aggregate_residuals(
        &unequal,
        CorrelationCalculation::Relative,
        CorrelationAggregation::WorstCondition,
    );
    assert!((worst_condition.0 - 2.0_f64.sqrt()).abs() < 1.0e-12);

    unequal[0].weight = NonNegativeFinite::new(1.0).unwrap();
    unequal[1].weight = NonNegativeFinite::new(3.0).unwrap();
    let weighted = aggregate_residuals(
        &unequal,
        CorrelationCalculation::WeightedRelative,
        CorrelationAggregation::RootMeanSquare,
    );
    assert!((weighted.0 - 3.0_f64.sqrt()).abs() < 1.0e-12);
}

#[test]
fn outlier_decisions_are_append_only_reviewed_and_scope_visible() {
    let source = source(3);
    let first = CorrelationOutlierDisposition {
        id: "disp-1".to_owned(),
        metric_id: "gain-error".to_owned(),
        reference_observation_id: "r3".to_owned(),
        decision: CorrelationOutlierDecision::ExcludeFixtureFault,
        reason: "Independent fixture inspection found leakage.".to_owned(),
        owner_id: "model-owner".to_owned(),
        reviewer_id: "characterization-reviewer".to_owned(),
        decided_at_unix_ms: 1,
        supersedes: None,
    };
    let suite = suite_with(
        source,
        simulation_csv(0.1),
        metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        ),
        vec![first.clone()],
    );
    let outcome = &CorrelationEvaluation::evaluate(&suite)
        .expect("reviewed exclusion")
        .metric_outcomes[0];
    assert_eq!(outcome.evaluated_points, 2);
    assert_eq!(outcome.excluded_points, 1);
    assert_eq!(outcome.residuals.len(), 3);
    assert!((outcome.coverage.get() - 2.0 / 3.0).abs() < 1.0e-12);
    let excluded = outcome
        .residuals
        .iter()
        .find(|residual| residual.reference_observation_id == "r3")
        .unwrap();
    assert!(excluded.excluded);
    assert_eq!(excluded.exclusion_disposition_id.as_deref(), Some("disp-1"));

    let mut invalid = suite.clone();
    invalid.dispositions.push(CorrelationOutlierDisposition {
        id: "disp-2".to_owned(),
        decision: CorrelationOutlierDecision::Retain,
        decided_at_unix_ms: 2,
        supersedes: None,
        ..first
    });
    assert_eq!(
        invalid.validate().unwrap_err().code,
        CorrelationErrorCode::DispositionInvalid
    );
}

#[test]
fn declared_minimum_coverage_is_a_numerical_gate_and_is_revalidated() {
    let disposition = CorrelationOutlierDisposition {
        id: "coverage-exclusion".to_owned(),
        metric_id: "gain-error".to_owned(),
        reference_observation_id: "r3".to_owned(),
        decision: CorrelationOutlierDecision::ExcludeFixtureFault,
        reason: "Verified fixture fault.".to_owned(),
        owner_id: "model-owner".to_owned(),
        reviewer_id: "independent-reviewer".to_owned(),
        decided_at_unix_ms: 1,
        supersedes: None,
    };
    let mut gated_metric = metric(
        CorrelationAggregation::EveryPoint,
        CorrelationAlignmentPolicy::ExactOnly,
    );
    gated_metric.minimum_coverage = NonNegativeFinite::new(0.75).unwrap();
    let suite = suite_with(
        source(33),
        simulation_csv(0.1),
        gated_metric,
        vec![disposition],
    );
    let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
    let outcome = &evaluation.metric_outcomes[0];
    assert!(outcome.aggregate_normalized_error.get() <= 1.0);
    assert!((outcome.coverage.get() - 2.0 / 3.0).abs() < 1.0e-12);
    assert_eq!(outcome.minimum_coverage.get(), 0.75);
    assert!(!outcome.passed);
    assert!(!evaluation.passed);

    let mut tampered = outcome.clone();
    tampered.passed = true;
    assert_eq!(
        validate_metric_outcomes(&[tampered], "tampered")
            .unwrap_err()
            .code,
        CorrelationErrorCode::EvidenceStale
    );
}

#[test]
fn immutable_evidence_rejects_stale_suite_or_source() {
    let source = source(4);
    let suite = suite_with(
        source,
        simulation_csv(0.1),
        metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        ),
        Vec::new(),
    );
    let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
    let evidence = CorrelationEvidence::try_new(
        "evidence-1",
        &evaluation,
        "independent-reviewer",
        CorrelationReviewDecision::Accept,
        "The retained comparison is accepted for model review.",
        10,
    )
    .unwrap();
    evidence.validate_current(&suite).unwrap();
    assert!(evidence.approved());
    assert_eq!(
        evidence.content_digest().unwrap(),
        evidence.content_digest().unwrap()
    );
    let self_review = CorrelationEvidence::try_new(
        "self-review",
        &evaluation,
        "model-owner",
        CorrelationReviewDecision::Accept,
        "The suite owner attempted to approve their own evidence.",
        9,
    )
    .unwrap();
    assert_eq!(
        self_review.validate_current(&suite).unwrap_err().code,
        CorrelationErrorCode::ReviewInvalid
    );

    let rejected = CorrelationEvidence::try_new(
        "evidence-rejected",
        &evaluation,
        "independent-reviewer",
        CorrelationReviewDecision::Reject,
        "Numerical gates pass, but the retained evidence is rejected pending fixture review.",
        11,
    )
    .unwrap();
    assert!(rejected.passed);
    assert!(!rejected.approved());

    let mut revised = suite.clone();
    revised.metrics[0].limit = NonNegativeFinite::new(0.2).unwrap();
    assert_eq!(
        evidence.validate_current(&revised).unwrap_err().code,
        CorrelationErrorCode::EvidenceStale
    );

    let mut tampered = evidence.clone();
    tampered.metric_outcomes[0].residuals[0].metric_error = NonNegativeFinite::new(0.0).unwrap();
    assert_eq!(
        tampered.validate_current(&suite).unwrap_err().code,
        CorrelationErrorCode::EvidenceStale
    );

    let orphan = ModelCorrelationState {
        schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
        suites: Vec::new(),
        evidence: vec![evidence],
    };
    assert_eq!(
        orphan.validate().unwrap_err().code,
        CorrelationErrorCode::EvidenceStale
    );
}

#[test]
fn release_approval_gate_is_optional_until_configured_and_then_fails_closed() {
    let current_source = source(44);
    ModelCorrelationState::default()
        .require_release_approval("dut", &current_source)
        .unwrap();

    let suite = suite_with(
        current_source.clone(),
        simulation_csv(0.1),
        metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        ),
        Vec::new(),
    );
    let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
    let mut state = ModelCorrelationState::try_new(vec![suite], Vec::new()).unwrap();
    assert_eq!(
        state
            .require_release_approval("dut", &current_source)
            .unwrap_err()
            .code,
        CorrelationErrorCode::ReviewInvalid
    );

    let accepted = CorrelationEvidence::try_new(
        "accepted-review",
        &evaluation,
        "independent-reviewer",
        CorrelationReviewDecision::Accept,
        "Current retained evidence is accepted.",
        10,
    )
    .unwrap();
    state.add_evidence(accepted).unwrap();
    state
        .require_release_approval("dut", &current_source)
        .unwrap();

    let rejected = CorrelationEvidence::try_new(
        "later-rejection",
        &evaluation,
        "independent-reviewer",
        CorrelationReviewDecision::Reject,
        "A later review rejects the current retained evidence.",
        11,
    )
    .unwrap();
    state.add_evidence(rejected).unwrap();
    assert_eq!(
        state
            .require_release_approval("dut", &current_source)
            .unwrap_err()
            .code,
        CorrelationErrorCode::ReviewInvalid
    );
    assert_eq!(
        state
            .require_release_approval("dut", &source(45))
            .unwrap_err()
            .code,
        CorrelationErrorCode::SourceBindingMismatch
    );
}

#[test]
fn reviewer_acceptance_cannot_override_failed_numerical_gates() {
    let mut failing_metric = metric(
        CorrelationAggregation::EveryPoint,
        CorrelationAlignmentPolicy::ExactOnly,
    );
    failing_metric.limit = NonNegativeFinite::new(0.01).unwrap();
    let suite = suite_with(source(34), simulation_csv(0.1), failing_metric, Vec::new());
    let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
    assert!(!evaluation.passed);
    assert_eq!(
        CorrelationEvidence::try_new(
            "invalid-acceptance",
            &evaluation,
            "reviewer",
            CorrelationReviewDecision::Accept,
            "Attempted acceptance.",
            1,
        )
        .unwrap_err()
        .code,
        CorrelationErrorCode::ReviewInvalid
    );
    let rejected = CorrelationEvidence::try_new(
        "valid-rejection",
        &evaluation,
        "reviewer",
        CorrelationReviewDecision::Reject,
        "Numerical gates failed.",
        1,
    )
    .unwrap();
    assert!(!rejected.passed);
    assert!(!rejected.approved());
}

#[test]
fn advisory_failures_remain_visible_without_failing_the_review_gate() {
    let source = source(31);
    let mut suite = suite_with(
        source,
        simulation_csv(0.1),
        metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        ),
        Vec::new(),
    );
    let mut advisory = suite.metrics[0].clone();
    advisory.id = "advisory-gain".to_owned();
    advisory.name = "Advisory gain".to_owned();
    advisory.limit = NonNegativeFinite::new(0.01).unwrap();
    advisory.release_role = CorrelationReleaseRole::Advisory;
    suite.metrics.push(advisory);
    suite.validate().unwrap();

    let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
    assert!(evaluation.passed);
    assert!(
        evaluation
            .metric_outcomes
            .iter()
            .any(
                |outcome| outcome.release_role == CorrelationReleaseRole::Advisory
                    && !outcome.passed
            )
    );

    suite.metrics[0].limit = NonNegativeFinite::new(0.01).unwrap();
    assert!(!CorrelationEvaluation::evaluate(&suite).unwrap().passed);
}

#[test]
fn a_dataset_only_suite_is_a_valid_draft_but_cannot_produce_evidence() {
    let source = source(32);
    let suite = CorrelationSuite::try_new(
        "draft",
        ObjectRevision::INITIAL,
        "Dataset import draft",
        "model-owner",
        source,
        vec![dataset(
            "reference",
            CorrelationDatasetClass::BenchMeasurement,
            reference_csv(),
            None,
        )],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(
        CorrelationEvaluation::evaluate(&suite).unwrap_err().code,
        CorrelationErrorCode::MissingValue
    );
}

#[test]
fn aggregate_suite_and_state_collections_are_bounded() {
    let suite = suite_with(
        source(35),
        simulation_csv(0.0),
        metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        ),
        Vec::new(),
    );
    let mut oversized_suite = suite.clone();
    oversized_suite.metrics = (0..=MAX_CORRELATION_METRICS)
        .map(|index| {
            let mut metric = oversized_suite.metrics[0].clone();
            metric.id = format!("metric-{index}");
            metric.name = format!("Metric {index}");
            metric
        })
        .collect();
    assert_eq!(
        oversized_suite.validate().unwrap_err().code,
        CorrelationErrorCode::ResourceLimit
    );

    let oversized_state = ModelCorrelationState {
        schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
        suites: vec![suite; MAX_CORRELATION_SUITE_REVISIONS + 1],
        evidence: Vec::new(),
    };
    assert_eq!(
        oversized_state.validate().unwrap_err().code,
        CorrelationErrorCode::ResourceLimit
    );
}

#[test]
fn state_and_serde_fail_closed_without_rewriting_history() {
    let source = source(5);
    let suite = suite_with(
        source,
        simulation_csv(0.0),
        metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        ),
        Vec::new(),
    );
    let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
    let evidence = CorrelationEvidence::try_new(
        "evidence",
        &evaluation,
        "reviewer",
        CorrelationReviewDecision::Accept,
        "Accepted exact retained evidence.",
        12,
    )
    .unwrap();
    let mut state = ModelCorrelationState::try_new(vec![suite], Vec::new()).unwrap();
    state.add_evidence(evidence.clone()).unwrap();
    assert_eq!(
        state.add_evidence(evidence).unwrap_err().code,
        CorrelationErrorCode::ImmutableRecord
    );

    let mut json = serde_json::to_value(&state).unwrap();
    json.as_object_mut()
        .unwrap()
        .insert("unknown".to_owned(), serde_json::json!(true));
    assert!(serde_json::from_value::<ModelCorrelationState>(json).is_err());
    let mut json = serde_json::to_value(&state).unwrap();
    json["schema_version"] = serde_json::json!(99);
    let restored = serde_json::from_value::<ModelCorrelationState>(json).unwrap();
    assert_eq!(
        restored.validate().unwrap_err().code,
        CorrelationErrorCode::UnsupportedSchema
    );
}
