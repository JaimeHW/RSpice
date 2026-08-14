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
}

struct Candidate {
    value: Option<f64>,
    measurement_passed: bool,
    signed_margin: Option<f64>,
    source_instance_id: AnalysisInstanceId,
    is_monte_carlo: bool,
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
            analysis.measurements.iter().filter_map(move |measurement| {
                if !measurement.name.eq_ignore_ascii_case(&spec.measurement) {
                    return None;
                }
                let value = measurement.value.filter(|value| value.is_finite());
                Some(Candidate {
                    value,
                    measurement_passed: analysis.success
                        && value.is_some()
                        && measurement.passed
                        && measurement.error.is_none(),
                    signed_margin: value
                        .and_then(|value| signed_margin(spec.min, spec.max, value))
                        .map(|margin| margin - guard_band.unwrap_or(0.0)),
                    source_instance_id,
                    is_monte_carlo: analysis.analysis_type == AnalysisType::MonteCarlo,
                })
            })
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
}
