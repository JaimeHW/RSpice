//! Immutable evaluation of the specifications sealed into a prepared run.

use crate::product::{AnalysisInstanceId, SpecificationId};

use super::{AnalysisResult, AnalysisType, PreparedSpecification};
use crate::state::{
    MissingMeasurementPolicy, MonteCarloSpecificationGate, NominalFailurePolicy, SpecPointScope,
    SpecificationPolicy, SpecificationRole,
};

/// Terminal outcome for one frozen specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecificationVerdictStatus {
    Pass,
    BoundFailure,
    MeasurementFailure,
    MissingEvidence,
}

/// Exact terminal judgment retained with a run.
///
/// Floating-point values are compared bitwise. The evaluator admits only
/// finite values, making this a true equivalence relation suitable for
/// validating a persisted verdict against a fresh deterministic evaluation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecificationVerdict {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    specification_id: Option<SpecificationId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    requirement_key: Option<String>,
    measurement: String,
    status: SpecificationVerdictStatus,
    worst_value: Option<f64>,
    signed_margin: Option<f64>,
    evidence_count: u64,
    /// Number of accepted in-bound samples across the governed evidence set.
    /// Statistical gating classifies the retained analyses by type at the
    /// acceptance boundary; legacy rows preserve zero because their
    /// historical verdict schema never claimed a passing population.
    #[serde(default)]
    passing_evidence_count: u64,
    source_instance_id: Option<AnalysisInstanceId>,
    /// Which member of a result family supplied the worst value, when the worst
    /// value came from one.
    ///
    /// `None` covers every verdict answered by an analysis-level measurement,
    /// and every verdict from history written before families attributed their
    /// members. It never means "the first member": a worst case that cannot be
    /// named is reported as unnamed rather than guessed at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    worst_member: Option<super::FamilyMemberId>,
}

impl PartialEq for SpecificationVerdict {
    fn eq(&self, other: &Self) -> bool {
        self.specification_id == other.specification_id
            && self.requirement_key == other.requirement_key
            && self.measurement == other.measurement
            && self.status == other.status
            && self.worst_value.map(f64::to_bits) == other.worst_value.map(f64::to_bits)
            && self.signed_margin.map(f64::to_bits) == other.signed_margin.map(f64::to_bits)
            && self.evidence_count == other.evidence_count
            && self.passing_evidence_count == other.passing_evidence_count
            && self.source_instance_id == other.source_instance_id
            && self.worst_member == other.worst_member
    }
}

impl Eq for SpecificationVerdict {}

impl SpecificationVerdict {
    #[must_use]
    pub const fn specification_id(&self) -> Option<SpecificationId> {
        self.specification_id
    }

    #[must_use]
    pub fn requirement_key(&self) -> Option<&str> {
        self.requirement_key.as_deref()
    }

    #[must_use]
    pub fn measurement(&self) -> &str {
        &self.measurement
    }

    #[must_use]
    pub const fn status(&self) -> SpecificationVerdictStatus {
        self.status
    }

    #[must_use]
    pub const fn worst_value(&self) -> Option<f64> {
        self.worst_value
    }

    #[must_use]
    pub const fn signed_margin(&self) -> Option<f64> {
        self.signed_margin
    }

    #[must_use]
    pub const fn evidence_count(&self) -> u64 {
        self.evidence_count
    }

    #[must_use]
    pub const fn passing_evidence_count(&self) -> u64 {
        self.passing_evidence_count
    }

    #[must_use]
    pub const fn source_instance_id(&self) -> Option<AnalysisInstanceId> {
        self.source_instance_id
    }

    /// The family member that supplied the worst value, when one did.
    #[must_use]
    pub const fn worst_member(&self) -> Option<&super::FamilyMemberId> {
        self.worst_member.as_ref()
    }
}

struct Candidate {
    value: Option<f64>,
    measurement_passed: bool,
    signed_margin: Option<f64>,
    source_instance_id: AnalysisInstanceId,
    is_monte_carlo: bool,
    /// Which family member produced this candidate, for evidence that came
    /// from one. Analysis-level measurements carry `None`.
    member: Option<super::FamilyMemberId>,
}

pub(super) fn evaluate_specifications(
    specifications: &[PreparedSpecification],
    analyses: &[AnalysisResult],
) -> Vec<SpecificationVerdict> {
    specifications
        .iter()
        .map(|specification| evaluate_specification(specification, analyses))
        .collect()
}

fn evaluate_specification(
    specification: &PreparedSpecification,
    analyses: &[AnalysisResult],
) -> SpecificationVerdict {
    let spec = specification.entry();
    let definition = specification.definition();
    let specification_id = definition.map(|definition| definition.id);
    let requirement_key = definition.map(|definition| definition.requirement_key.clone());
    let mut candidates = candidates_for(specification, analyses);
    let evidence_count = u64::try_from(candidates.len()).unwrap_or(u64::MAX);
    let passing_evidence_count = definition.map_or(0, |_| {
        u64::try_from(
            candidates
                .iter()
                .filter(|candidate| candidate_is_passing(candidate))
                .count(),
        )
        .unwrap_or(u64::MAX)
    });
    if candidates.is_empty() {
        return SpecificationVerdict {
            specification_id,
            requirement_key,
            measurement: spec.measurement.clone(),
            status: SpecificationVerdictStatus::MissingEvidence,
            worst_value: None,
            signed_margin: None,
            evidence_count,
            passing_evidence_count,
            source_instance_id: None,
            worst_member: None,
        };
    }

    candidates.sort_by(|left, right| {
        left.measurement_passed
            .cmp(&right.measurement_passed)
            .then_with(|| compare_margin(left.signed_margin, right.signed_margin))
    });
    let worst = &candidates[0];
    let status = if !worst.measurement_passed {
        SpecificationVerdictStatus::MeasurementFailure
    } else if worst.signed_margin.is_none_or(|margin| margin >= 0.0) {
        SpecificationVerdictStatus::Pass
    } else {
        SpecificationVerdictStatus::BoundFailure
    };
    SpecificationVerdict {
        specification_id,
        requirement_key,
        measurement: spec.measurement.clone(),
        status,
        worst_value: worst.value,
        signed_margin: worst.signed_margin,
        evidence_count,
        passing_evidence_count,
        source_instance_id: Some(worst.source_instance_id),
        worst_member: worst.member.clone(),
    }
}

fn candidates_for(
    specification: &PreparedSpecification,
    analyses: &[AnalysisResult],
) -> Vec<Candidate> {
    let spec = specification.entry();
    let definition = specification.definition();
    let producing_analysis = definition.and_then(|definition| definition.producing_analysis);
    let guard_band = definition.and_then(|definition| definition.guard_band);
    analyses
        .iter()
        .filter_map(|analysis| {
            let provenance = analysis.provenance()?;
            let source_instance_id = provenance.authored_source_instance_id();
            (spec.scope.admits(provenance.pvt_point())
                && producing_analysis.is_none_or(|expected| expected == source_instance_id))
            .then_some((analysis, source_instance_id))
        })
        .flat_map(|(analysis, source_instance_id)| {
            let is_monte_carlo = analysis.analysis_type == AnalysisType::MonteCarlo;
            let make =
                move |value: Option<f64>, measured: bool, member: Option<super::FamilyMemberId>| {
                    let value = value.filter(|value| value.is_finite());
                    Candidate {
                        value,
                        measurement_passed: analysis.success && value.is_some() && measured,
                        signed_margin: value
                            .and_then(|value| signed_margin(spec.min, spec.max, value))
                            .map(|margin| margin - guard_band.unwrap_or(0.0)),
                        source_instance_id,
                        is_monte_carlo,
                        member,
                    }
                };

            let analysis_level = analysis.measurements.iter().filter_map(move |measurement| {
                measurement
                    .name
                    .eq_ignore_ascii_case(&spec.measurement)
                    .then(|| {
                        make(
                            measurement.value,
                            measurement.passed && measurement.error.is_none(),
                            None,
                        )
                    })
            });

            // A family that measured its own members answers the limit over all
            // of them. This is what makes a Monte Carlo trial set or an
            // in-analysis sweep a spread a specification can judge rather than
            // one reduced number: without it the worst retained member is
            // invisible, and a yield gate divides by nothing.
            let member_level = analysis
                .family_metadata
                .iter()
                .flat_map(|metadata| metadata.member_measurements())
                .filter_map(move |member| {
                    let evidence = member.evidence_for(&spec.measurement)?;
                    Some(make(
                        evidence.value,
                        evidence.passed,
                        Some(member.member.clone()),
                    ))
                });

            analysis_level.chain(member_level)
        })
        .collect()
}

fn candidate_is_passing(candidate: &Candidate) -> bool {
    candidate.measurement_passed && candidate.signed_margin.is_none_or(|margin| margin >= 0.0)
}

pub(super) fn acceptance_is_blocked(
    specifications: &[PreparedSpecification],
    policy: &SpecificationPolicy,
    verdicts: &[SpecificationVerdict],
    analyses: &[AnalysisResult],
) -> bool {
    specifications
        .iter()
        .zip(verdicts)
        .any(|(specification, verdict)| {
            let definition = specification.definition();
            if definition.is_some_and(|definition| {
                definition.waiver.is_some() || definition.role != SpecificationRole::Blocking
            }) {
                return false;
            }

            if let MonteCarloSpecificationGate::YieldAtLeast { percent } = policy.monte_carlo
                && definition.is_some()
            {
                let monte_carlo = candidates_for(specification, analyses)
                    .into_iter()
                    .filter(|candidate| candidate.is_monte_carlo)
                    .collect::<Vec<_>>();
                if !monte_carlo.is_empty() {
                    let passing = monte_carlo
                        .iter()
                        .filter(|candidate| candidate_is_passing(candidate))
                        .count();
                    let yield_percent = 100.0 * passing as f64 / monte_carlo.len() as f64;
                    return yield_percent < percent;
                }
            }

            match verdict.status {
                SpecificationVerdictStatus::Pass => false,
                SpecificationVerdictStatus::MissingEvidence => {
                    policy.missing_measurement == MissingMeasurementPolicy::FailClosed
                }
                SpecificationVerdictStatus::BoundFailure
                | SpecificationVerdictStatus::MeasurementFailure => {
                    !(policy.nominal_failure == NominalFailurePolicy::RecordDisposition
                        && matches!(specification.entry().scope, SpecPointScope::Nominal))
                }
            }
        })
}

fn signed_margin(minimum: Option<f64>, maximum: Option<f64>, value: f64) -> Option<f64> {
    match (minimum, maximum) {
        (Some(minimum), Some(maximum)) => Some((value - minimum).min(maximum - value)),
        (Some(minimum), None) => Some(value - minimum),
        (None, Some(maximum)) => Some(maximum - value),
        (None, None) => None,
    }
}

fn compare_margin(left: Option<f64>, right: Option<f64>) -> std::cmp::Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.total_cmp(&right),
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (Some(_), None) => std::cmp::Ordering::Less,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ContentDigest, ObjectRevision, SimulationPlanId};
    use crate::state::{
        AnalysisResultProvenance, AnalysisType, SpecEntry, SpecPointScope, SpecificationDefinition,
    };

    fn result(sequence: u64, source_id: AnalysisInstanceId, value: f64) -> AnalysisResult {
        typed_result(sequence, source_id, AnalysisType::Ac, value)
    }

    fn typed_result(
        sequence: u64,
        source_id: AnalysisInstanceId,
        analysis_type: AnalysisType,
        value: f64,
    ) -> AnalysisResult {
        AnalysisResult::new(sequence, analysis_type, format!("{analysis_type:?}"))
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
        let review_specifications = vec![
            PreparedSpecification::from_definition(definition).expect("valid review requirement"),
        ];
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
        let definition =
            SpecificationDefinition::from_legacy(SimulationPlanId::new(), 0, &projection);
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
    fn monte_carlo_trials(
        source_id: AnalysisInstanceId,
        trials: &[(usize, f64)],
    ) -> AnalysisResult {
        use crate::state::{
            AnalysisResultFamilyMetadata, FamilyMeasurementEvidence, FamilyMemberId,
            FamilyMemberMeasurements,
        };

        let members = trials
            .iter()
            .map(|(index, value)| {
                FamilyMemberMeasurements::new(
                    FamilyMemberId::MonteCarloTrial {
                        index: *index,
                        seed: 4_000 + *index as u64,
                    },
                    vec![FamilyMeasurementEvidence {
                        name: "gain".to_owned(),
                        value: Some(*value),
                        passed: true,
                        error: None,
                    }],
                )
            })
            .collect();

        AnalysisResult::new(1, AnalysisType::MonteCarlo, "Monte Carlo")
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
        use crate::state::FamilyMemberId;

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

    /// An in-analysis sweep answers a limit over every point it solved.
    #[test]
    fn an_in_analysis_sweep_is_judged_over_its_points_not_its_last_one() {
        use crate::state::{
            AnalysisResultFamilyMetadata, FamilyMeasurementEvidence, FamilyMemberId,
            FamilyMemberMeasurements,
        };

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
                        name: "gain".to_owned(),
                        value: Some(value),
                        passed: true,
                        error: None,
                    }],
                )
            })
            .collect();
        let analyses = [
            AnalysisResult::new(1, AnalysisType::Parametric, "Parametric")
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
}
