//! Existing requirement-evaluation regressions over canonical retained data.

use super::*;
use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_type::AnalysisType;
use crate::floquet::{
    FloquetOrbitKindEvidence, FloquetSpectrumEvidence, FloquetStabilityVerdictEvidence,
};
use crate::provenance::AnalysisResultProvenance;
use crate::specification::{SpecEntry, SpecPointScope, SpecificationDefinition};
use rspice_app_types::product::{ContentDigest, ObjectRevision, SimulationPlanId};
type AnalysisResult = crate::analysis_result::AnalysisResult;

fn result(sequence: u64, source_id: AnalysisInstanceId, value: f64) -> AnalysisResult {
    typed_result(sequence, source_id, AnalysisType::Ac, value)
}

fn typed_result(
    sequence: u64,
    source_id: AnalysisInstanceId,
    analysis_type: AnalysisType,
    value: f64,
) -> AnalysisResult {
    AnalysisResult::new(sequence, analysis_type, format!("{analysis_type:?}"), 0.0)
        .with_measurements(vec![rspice_core::MeasureResult::success("gain", value)])
        .with_provenance(
            AnalysisResultProvenance::new(
                source_id,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x91; 32]),
                Vec::new(),
            )
            .expect("valid prepared provenance"),
        )
}

#[test]
fn governed_source_binding_and_guard_band_control_the_verdict() {
    let producing_analysis = AnalysisInstanceId::new();
    let unrelated_analysis = AnalysisInstanceId::new();
    let projection = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "param='gain'".to_owned(),
        min: None,
        max: Some(10.0),
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    };
    let mut definition =
        SpecificationDefinition::from_legacy(SimulationPlanId::new(), 0, &projection);
    definition.requirement_key = "REQ-GAIN-1".to_owned();
    definition.producing_analysis = Some(producing_analysis);
    definition.guard_band = Some(1.0);
    let specifications = vec![
        PreparedSpecification::from_definition(definition.clone())
            .expect("valid governed requirement"),
    ];

    let verdicts = evaluate_specifications(
        &specifications,
        &[
            result(1, unrelated_analysis, 20.0),
            result(2, producing_analysis, 9.5),
        ],
    );

    assert_eq!(verdicts.len(), 1);
    assert_eq!(
        verdicts[0].status(),
        SpecificationVerdictStatus::BoundFailure,
        "the 1 dB guard band tightens the 10 dB maximum to an effective 9 dB"
    );
    assert_eq!(verdicts[0].signed_margin(), Some(-0.5));
    assert_eq!(verdicts[0].evidence_count(), 1);
    assert_eq!(verdicts[0].source_instance_id(), Some(producing_analysis));
    assert_eq!(verdicts[0].specification_id(), Some(definition.id));
    assert_eq!(verdicts[0].requirement_key(), Some("REQ-GAIN-1"));
    assert!(acceptance_is_blocked(
        &specifications,
        &SpecificationPolicy::default(),
        &verdicts,
        &[
            result(1, unrelated_analysis, 20.0),
            result(2, producing_analysis, 9.5),
        ],
    ));

    let yield_policy = SpecificationPolicy {
        monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 90.0 },
        ..SpecificationPolicy::default()
    };
    assert!(acceptance_is_blocked(
        &specifications,
        &yield_policy,
        &verdicts,
        &[result(2, producing_analysis, 9.5)],
    ));
    assert_eq!(verdicts[0].passing_evidence_count(), 0);

    definition.role = SpecificationRole::Review;
    let review_specifications =
        vec![PreparedSpecification::from_definition(definition).expect("valid review requirement")];
    let review_verdicts = evaluate_specifications(
        &review_specifications,
        &[result(3, producing_analysis, 9.5)],
    );
    assert!(!acceptance_is_blocked(
        &review_specifications,
        &SpecificationPolicy::default(),
        &review_verdicts,
        &[result(3, producing_analysis, 9.5)],
    ));
}

#[test]
fn native_scalar_units_convert_periodic_limits_without_changing_history() {
    let source = AnalysisInstanceId::new();
    let mut analysis = AnalysisResult::new(1, AnalysisType::Pss, "PSS", 0.0)
        .with_result_payload(AnalysisResultPayload::PssFloquet {
            period_s: Some(2.0),
            fundamental_frequency_hz: Some(0.5),
            iterations: Some(2),
            residual_norm: Some(1.0e-12),
            multipliers: Vec::new(),
            floquet_evidence: FloquetSpectrumEvidence::NoDynamicModes,
            orbit_kind: FloquetOrbitKindEvidence::Driven,
            trivial_multiplier_index: None,
            stability_verdict: FloquetStabilityVerdictEvidence::Stable,
        })
        .with_provenance(
            AnalysisResultProvenance::new(
                source,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x71; 32]),
                Vec::new(),
            )
            .unwrap(),
        );
    let specification = PreparedSpecification::new(SpecEntry {
        measurement: "pss_mode_count".to_owned(),
        expression: "authenticated Floquet spectrum order".to_owned(),
        min: Some(0.0),
        max: Some(0.0),
        unit: "count".to_owned(),
        scope: SpecPointScope::AllPoints,
    })
    .unwrap();

    let historical = analysis.clone();
    analysis.retain_native_scalar_units();
    let verdicts = evaluate_specifications(&[specification], &[analysis.clone()]);

    assert_eq!(verdicts.len(), 1);
    assert_eq!(verdicts[0].status(), SpecificationVerdictStatus::Pass);
    assert_eq!(verdicts[0].worst_value(), Some(0.0));
    assert_eq!(verdicts[0].evidence_count(), 1);
    let spec = |unit: &str| {
        PreparedSpecification::new(SpecEntry {
            measurement: "PsS_PeRiOd".into(),
            expression: "period".into(),
            min: Some(1900.0),
            max: Some(2100.0),
            unit: unit.into(),
            scope: SpecPointScope::AllPoints,
        })
        .unwrap()
    };
    let verdict = evaluate_specifications(&[spec("ms")], &[analysis.clone()]).remove(0);
    assert_eq!(verdict.status(), SpecificationVerdictStatus::Pass);
    assert_eq!(verdict.worst_value(), Some(2000.0));
    assert_eq!(verdict.signed_margin(), Some(100.0));
    let old = evaluate_specifications(&[spec("ms")], &[historical]).remove(0);
    assert_eq!(old.status(), SpecificationVerdictStatus::BoundFailure);
    assert_eq!(old.worst_value(), Some(2.0));
    assert_eq!(
        evaluate_specifications(&[spec("V")], &[analysis.clone()])[0].status(),
        SpecificationVerdictStatus::MeasurementFailure
    );
    // An authored measurement still owns its name even when a native
    // payload provides a scalar with the same name.
    let mut authored = rspice_core::MeasureResult::success("pss_period", 0.003);
    authored.units = Some(rspice_core::analysis::MeasurementUnits {
        value: rspice_core::analysis::MeasurementUnit::Known("A".into()),
        raw_value: rspice_core::analysis::MeasurementUnit::Known("A".into()),
        axis: rspice_core::analysis::MeasurementUnit::Known("s".into()),
    });
    analysis.measurements.push(authored);
    assert_eq!(
        analysis.scalar_evidence("pss_period")[0]
            .value_in_unit("mA")
            .unwrap(),
        Some(3.0)
    );
}

#[test]
fn monte_carlo_yield_gate_applies_only_to_monte_carlo_evidence() {
    let source = AnalysisInstanceId::new();
    let projection = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "param='gain'".to_owned(),
        min: None,
        max: Some(10.0),
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    };
    let definition = SpecificationDefinition::from_legacy(SimulationPlanId::new(), 0, &projection);
    let specifications = vec![
        PreparedSpecification::from_definition(definition).expect("valid governed requirement"),
    ];
    let policy = SpecificationPolicy {
        monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 40.0 },
        ..SpecificationPolicy::default()
    };
    let monte_carlo = vec![
        typed_result(1, source, AnalysisType::MonteCarlo, 9.0),
        typed_result(2, source, AnalysisType::MonteCarlo, 11.0),
    ];
    let monte_carlo_verdicts = evaluate_specifications(&specifications, &monte_carlo);
    assert_eq!(
        monte_carlo_verdicts[0].status(),
        SpecificationVerdictStatus::BoundFailure
    );
    assert!(
        !acceptance_is_blocked(
            &specifications,
            &policy,
            &monte_carlo_verdicts,
            &monte_carlo,
        ),
        "50% Monte Carlo yield satisfies a 40% statistical gate"
    );

    let nominal = vec![result(1, source, 9.0), result(2, source, 11.0)];
    let nominal_verdicts = evaluate_specifications(&specifications, &nominal);
    assert!(
        acceptance_is_blocked(&specifications, &policy, &nominal_verdicts, &nominal),
        "a Monte Carlo policy must not waive an ordinary analysis failure"
    );
}

/// Build a Monte Carlo analysis whose retained trials each measured `gain`.
///
/// `trials` is `(requested trial index, value)`, so a test can leave gaps
/// exactly as a driver does when a trial fails to converge.
fn monte_carlo_trials(source_id: AnalysisInstanceId, trials: &[(usize, f64)]) -> AnalysisResult {
    use crate::family_measurements::{
        FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
    };
    use crate::family_metadata::AnalysisResultFamilyMetadata;

    let members = trials
        .iter()
        .map(|(index, value)| {
            FamilyMemberMeasurements::new(
                FamilyMemberId::MonteCarloTrial {
                    index: *index,
                    seed: 4_000 + *index as u64,
                },
                vec![FamilyMeasurementEvidence {
                    unit: None,
                    name: "gain".to_owned(),
                    value: Some(*value),
                    passed: true,
                    error: None,
                }],
            )
        })
        .collect();

    AnalysisResult::new(1, AnalysisType::MonteCarlo, "Monte Carlo", 0.0)
        .with_family_metadata(AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 4_000,
            runs_requested: trials.len(),
            runs_completed: trials.len(),
            failures: 0,
            all_converged: true,
            variables: Vec::new(),
            member_measurements: members,
        })
        .with_provenance(
            AnalysisResultProvenance::new(
                source_id,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x44; 32]),
                Vec::new(),
            )
            .expect("valid prepared provenance"),
        )
}

fn gain_floor_entry(minimum: f64) -> SpecEntry {
    SpecEntry {
        measurement: "gain".to_owned(),
        expression: "param=gain".to_owned(),
        min: Some(minimum),
        max: None,
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    }
}

fn gain_at_least(minimum: f64) -> Vec<PreparedSpecification> {
    vec![PreparedSpecification::new(gain_floor_entry(minimum)).expect("valid specification")]
}

fn governed_gain_at_least(minimum: f64) -> Vec<PreparedSpecification> {
    let mut definition = SpecificationDefinition::from_legacy(
        SimulationPlanId::new(),
        0,
        &gain_floor_entry(minimum),
    );
    definition.requirement_key = "REQ-GAIN-MC".to_owned();
    vec![PreparedSpecification::from_definition(definition).expect("valid requirement")]
}

/// A specification bound to a Monte Carlo measurement is judged against the
/// distribution, and reports the trial that produced the worst value.
///
/// Before trials carried their own measurements, a Monte Carlo analysis
/// contributed no candidate at all and this specification came back as
/// missing evidence — a limit no run could ever fail.
#[test]
fn a_spec_bound_to_a_monte_carlo_measurement_names_its_worst_trial() {
    use crate::family_measurements::FamilyMemberId;

    let source_id = AnalysisInstanceId::new();
    let analyses = [monte_carlo_trials(
        source_id,
        &[(0, 12.0), (2, 8.5), (4, 11.0), (5, 10.5)],
    )];

    let verdicts = evaluate_specifications(&gain_at_least(10.0), &analyses);

    assert_eq!(
        verdicts[0].status(),
        SpecificationVerdictStatus::BoundFailure
    );
    assert_eq!(verdicts[0].worst_value(), Some(8.5));
    assert_eq!(
        verdicts[0].worst_member(),
        Some(&FamilyMemberId::MonteCarloTrial {
            index: 2,
            seed: 4_002
        }),
        "the verdict must name the trial that produced the worst value, by \
         the index the driver requested it under and the seed that \
         reproduces it"
    );
    assert_eq!(
        verdicts[0].evidence_count(),
        4,
        "every retained trial is evidence"
    );
}

/// The yield fraction a statistical specification is judged by.
#[test]
fn a_monte_carlo_verdict_reports_the_fraction_of_trials_that_held() {
    let source_id = AnalysisInstanceId::new();
    let analyses = [monte_carlo_trials(
        source_id,
        &[(0, 12.0), (1, 8.5), (2, 11.0), (3, 10.5)],
    )];

    let verdicts = evaluate_specifications(&governed_gain_at_least(10.0), &analyses);

    assert_eq!(verdicts[0].evidence_count(), 4);
    assert_eq!(
        verdicts[0].passing_evidence_count(),
        3,
        "three of four trials held the 10 dB floor"
    );
}

/// The Monte Carlo yield gate was unreachable while Monte Carlo produced no
/// measurements: it filters candidates to the ones an MC analysis supplied,
/// and that set was always empty, so `YieldAtLeast` could never block.
#[test]
fn the_monte_carlo_yield_gate_now_has_a_population_to_judge() {
    let source_id = AnalysisInstanceId::new();
    let analyses = [monte_carlo_trials(
        source_id,
        &[(0, 12.0), (1, 8.5), (2, 11.0), (3, 10.5)],
    )];
    let specifications = governed_gain_at_least(10.0);
    let verdicts = evaluate_specifications(&specifications, &analyses);

    let demanding = SpecificationPolicy {
        monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 90.0 },
        ..SpecificationPolicy::default()
    };
    assert!(
        acceptance_is_blocked(&specifications, &demanding, &verdicts, &analyses),
        "75% yield must not clear a 90% gate"
    );

    let tolerant = SpecificationPolicy {
        monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 70.0 },
        ..SpecificationPolicy::default()
    };
    assert!(
        !acceptance_is_blocked(&specifications, &tolerant, &verdicts, &analyses),
        "75% yield clears a 70% gate"
    );
}

/// A passing yield does not clear the corner evidence beside it.
///
/// The yield gate judges the trials. A specification is routinely governed
/// by corner and parametric evidence as well, and that evidence gets no
/// statistical treatment — one point out of bound is out of bound. The gate
/// used to `return` as soon as the yield held, so a blocking specification
/// whose corner run failed outright read as "not blocked".
#[test]
fn monte_carlo_yield_counts_failed_solves_in_the_requested_population() {
    let mut result =
        monte_carlo_trials(AnalysisInstanceId::new(), &[(0, 12.0), (1, 0.0), (2, 11.0)]);
    let Some(crate::family_metadata::AnalysisResultFamilyMetadata::MonteCarlo {
        runs_completed,
        failures,
        all_converged,
        member_measurements,
        ..
    }) = &mut result.family_metadata
    else {
        panic!("MC")
    };
    *runs_completed = 2;
    *failures = 1;
    *all_converged = false;
    let failed = &mut member_measurements[1].measurements[0];
    failed.value = None;
    failed.passed = false;
    failed.error = Some("did not converge".into());
    let analyses = [result];
    let specifications = governed_gain_at_least(10.0);
    let verdicts = evaluate_specifications(&specifications, &analyses);
    let policy = SpecificationPolicy {
        monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 90.0 },
        ..Default::default()
    };
    assert!(
        acceptance_is_blocked(&specifications, &policy, &verdicts, &analyses),
        "two passing solves among three attempted trials are not 100% yield"
    );
}

#[test]
fn a_passing_yield_does_not_waive_the_non_monte_carlo_evidence() {
    let source_id = AnalysisInstanceId::new();
    let specifications = governed_gain_at_least(10.0);
    let tolerant = SpecificationPolicy {
        monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 70.0 },
        ..SpecificationPolicy::default()
    };

    // 75% of the trials hold the floor, which clears the 70% gate.
    let trials = monte_carlo_trials(source_id, &[(0, 12.0), (1, 8.5), (2, 11.0), (3, 10.5)]);
    let monte_carlo_only = [trials.clone()];
    let verdicts = evaluate_specifications(&specifications, &monte_carlo_only);
    assert!(
        !acceptance_is_blocked(&specifications, &tolerant, &verdicts, &monte_carlo_only),
        "75% yield clears a 70% gate"
    );

    // The same trials, beside a corner that measured 7 dB against a 10 dB
    // floor. The yield is unchanged and the corner is out of bound.
    let mixed = [trials, result(7, source_id, 7.0)];
    let verdicts = evaluate_specifications(&specifications, &mixed);
    assert!(
        acceptance_is_blocked(&specifications, &tolerant, &verdicts, &mixed),
        "a corner out of bound blocks acceptance however the trials distributed"
    );

    // And the corner passing puts it back: the OR must not have become an
    // unconditional block.
    let passing = [
        monte_carlo_trials(source_id, &[(0, 12.0), (1, 8.5), (2, 11.0), (3, 10.5)]),
        result(7, source_id, 12.5),
    ];
    let verdicts = evaluate_specifications(&specifications, &passing);
    assert!(
        !acceptance_is_blocked(&specifications, &tolerant, &verdicts, &passing),
        "a passing corner beside a passing yield blocks nothing"
    );
}

/// An in-analysis sweep answers a limit over every point it solved.
#[test]
fn an_in_analysis_sweep_is_judged_over_its_points_not_its_last_one() {
    use crate::family_measurements::{
        FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
    };
    use crate::family_metadata::AnalysisResultFamilyMetadata;

    let source_id = AnalysisInstanceId::new();
    let members = [(0, 25.0, 12.0), (1, 75.0, 7.25), (2, 125.0, 11.0)]
        .into_iter()
        .map(|(index, coordinate, value)| {
            FamilyMemberMeasurements::new(
                FamilyMemberId::SweepPoint {
                    index,
                    value: coordinate,
                },
                vec![FamilyMeasurementEvidence {
                    unit: None,
                    name: "gain".to_owned(),
                    value: Some(value),
                    passed: true,
                    error: None,
                }],
            )
        })
        .collect();
    let analyses = [
        AnalysisResult::new(1, AnalysisType::Parametric, "Parametric", 0.0)
            .with_family_metadata(AnalysisResultFamilyMetadata::Parametric {
                target: "TEMP".to_owned(),
                sweep_values: vec![25.0, 75.0, 125.0],
                failed_points: 0,
                member_measurements: members,
            })
            .with_provenance(
                AnalysisResultProvenance::new(
                    source_id,
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0x55; 32]),
                    Vec::new(),
                )
                .expect("valid prepared provenance"),
            ),
    ];

    let verdicts = evaluate_specifications(&gain_at_least(10.0), &analyses);

    assert_eq!(
        verdicts[0].status(),
        SpecificationVerdictStatus::BoundFailure,
        "the sweep's worst point breaks the floor even though its last does not"
    );
    assert_eq!(verdicts[0].worst_value(), Some(7.25));
    assert_eq!(
        verdicts[0].worst_member(),
        Some(&FamilyMemberId::SweepPoint {
            index: 1,
            value: 75.0
        })
    );
}

/// A verdict from history that names no member restores as naming none.
///
/// `worst_member` was added after verdicts were already being persisted,
/// so every retained verdict from before it carries no such key. The
/// honest reading is "unnamed", never "the first member" — and the field
/// is `serde(default)` for exactly that reason. This removes the key from
/// a real serialized verdict rather than writing `null`, because `null`
/// exercises a present field and would pass even with the default gone.
#[test]
fn a_verdict_stating_no_worst_member_restores_as_naming_none() {
    use crate::family_measurements::FamilyMemberId;

    let source_id = AnalysisInstanceId::new();
    let analyses = [monte_carlo_trials(source_id, &[(0, 12.0), (2, 8.5)])];
    let verdict = evaluate_specifications(&gain_at_least(10.0), &analyses)
        .pop()
        .expect("one specification, one verdict");
    assert_eq!(
        verdict.worst_member(),
        Some(&FamilyMemberId::MonteCarloTrial {
            index: 2,
            seed: 4_002
        }),
        "the fixture must name one for its removal to mean anything"
    );

    let mut document: serde_json::Value =
        serde_json::to_value(&verdict).expect("a verdict serializes");
    assert!(
        document
            .as_object_mut()
            .expect("a verdict is written as an object")
            .remove("worst_member")
            .is_some()
    );

    let restored: SpecificationVerdict =
        serde_json::from_value(document).expect("a verdict from before members still loads");
    assert_eq!(restored.worst_member(), None);
    assert_eq!(
        restored.status(),
        verdict.status(),
        "and everything else about it is unchanged"
    );
}

/// A family that retained no member evidence must not become a spread.
#[test]
fn a_family_without_member_evidence_still_answers_from_its_analysis_measurement() {
    let source_id = AnalysisInstanceId::new();
    let analyses = [typed_result(1, source_id, AnalysisType::MonteCarlo, 11.0)];

    let verdicts = evaluate_specifications(&gain_at_least(10.0), &analyses);

    assert_eq!(verdicts[0].status(), SpecificationVerdictStatus::Pass);
    assert_eq!(verdicts[0].evidence_count(), 1);
    assert_eq!(
        verdicts[0].worst_member(),
        None,
        "an analysis-level measurement is not attributed to a member"
    );
}
#[test]
fn measurement_units_drive_guard_bands_family_limits_and_legacy_verdicts() {
    use rspice_core::analysis::{MeasurementUnit, MeasurementUnits};
    let source = AnalysisInstanceId::new();
    let entry = SpecEntry {
        measurement: "gain".into(),
        expression: String::new(),
        min: Some(200.0),
        max: Some(300.0),
        unit: "mV".into(),
        scope: SpecPointScope::AllPoints,
    };
    let mut definition = SpecificationDefinition::from_legacy(SimulationPlanId::new(), 0, &entry);
    definition.guard_band = Some(5.0);
    let specs = vec![PreparedSpecification::from_definition(definition).unwrap()];
    let mut analysis = result(1, source, 0.25);
    analysis.measurements[0].units = Some(MeasurementUnits {
        value: MeasurementUnit::Known("V".into()),
        raw_value: MeasurementUnit::Known("V".into()),
        axis: MeasurementUnit::Known("Hz".into()),
    });
    let verdict = evaluate_specifications(&specs, &[analysis.clone()]).remove(0);
    assert_eq!(verdict.status(), SpecificationVerdictStatus::Pass);
    assert_eq!(verdict.worst_value(), Some(250.0));
    assert_eq!(verdict.signed_margin(), Some(45.0));
    let mut family = monte_carlo_trials(source, &[(0, 0.25), (1, 0.299)]);
    let Some(crate::family_metadata::AnalysisResultFamilyMetadata::MonteCarlo {
        member_measurements,
        ..
    }) = &mut family.family_metadata
    else {
        panic!("MC");
    };
    for member in member_measurements {
        member.measurements[0].unit = Some(MeasurementUnit::Known("V".into()));
    }
    let verdict = evaluate_specifications(&specs, &[family]).remove(0);
    assert_eq!(verdict.status(), SpecificationVerdictStatus::BoundFailure);
    assert_eq!(verdict.worst_value(), Some(299.0));
    assert_eq!(verdict.signed_margin(), Some(-4.0));
    analysis.measurements[0].units.as_mut().unwrap().value = MeasurementUnit::Unknown;
    assert_eq!(
        evaluate_specifications(&specs, &[analysis.clone()])[0].status(),
        SpecificationVerdictStatus::MeasurementFailure
    );
    analysis.measurements[0].units = None;
    let verdict = evaluate_specifications(&specs, &[analysis]).remove(0);
    assert_eq!(
        verdict.worst_value(),
        Some(0.25),
        "historical numeric interpretation is retained"
    );
}
