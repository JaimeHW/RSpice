//! Immutable evaluation of the specifications sealed into a prepared run.

use std::borrow::Borrow;

use rspice_vocabulary::product::{AnalysisInstanceId, SpecificationId};

use crate::analysis_result::AnalysisResult;
use crate::analysis_type::AnalysisType;
use crate::family_measurements::FamilyMemberId;
use crate::specification::PreparedSpecification;
use crate::specification::{
    MissingMeasurementPolicy, MonteCarloSpecificationGate, NominalFailurePolicy, SpecPointScope,
    SpecificationPolicy, SpecificationRole,
};
use crate::waveform::RetainedWaveform;

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
    worst_member: Option<FamilyMemberId>,
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
    pub const fn worst_member(&self) -> Option<&FamilyMemberId> {
        self.worst_member.as_ref()
    }

    /// Judge one frozen requirement set against retained evidence.
    ///
    /// The judge, for every surface that has to state a verdict. A guard
    /// band, a point scope and a producing-analysis binding are requirement
    /// semantics: they belong to the requirement, not to the moment a run
    /// reaches its terminal state. A reader watching a run stream is
    /// therefore shown the answer this function gives over the evidence
    /// retained so far, and sealing the run re-asks the same question of the
    /// same function — so nothing about a row may change at the terminality
    /// boundary unless the evidence itself changed.
    ///
    /// It is exposed because the Results specification sheet used to keep a
    /// projection of its own for the streaming case, which knew about none of
    /// the three and additionally refused any measurement published by more
    /// than one prepared task. Verdicts visibly flipped the instant a run
    /// completed, and an ordinary corner sweep read as an ambiguous lineage
    /// until it did.
    #[must_use]
    pub fn evaluate<'a, W: AsRef<RetainedWaveform> + 'a>(
        specifications: &[PreparedSpecification],
        analyses: impl IntoIterator<Item = &'a AnalysisResult<W>> + Clone,
    ) -> Vec<Self> {
        evaluate_specifications(specifications, analyses)
    }
}

/// One retained value that answers a requirement, as the judge reads it.
pub(crate) struct Candidate {
    pub(crate) value: Option<f64>,
    pub(crate) measurement_passed: bool,
    pub(crate) signed_margin: Option<f64>,
    source_instance_id: AnalysisInstanceId,
    pub(crate) is_monte_carlo: bool,
    /// Which family member produced this candidate, for evidence that came
    /// from one. Analysis-level measurements carry `None`.
    pub(crate) member: Option<FamilyMemberId>,
    /// The position, in the evidence set the judge was handed, of the
    /// analysis this candidate came from.
    pub(crate) analysis: usize,
}

impl PreparedSpecification {
    /// Whether this analysis is evidence for the retained requirement.
    pub fn admits_analysis<W: AsRef<RetainedWaveform>>(
        &self,
        analysis: &AnalysisResult<W>,
    ) -> bool {
        analysis.provenance().is_some_and(|provenance| {
            self.entry().scope.admits(provenance.pvt_point())
                && self
                    .definition()
                    .and_then(|definition| definition.producing_analysis)
                    .is_none_or(|expected| expected == provenance.authored_source_instance_id())
        })
    }

    pub fn guard_band(&self) -> f64 {
        self.definition()
            .and_then(|definition| definition.guard_band)
            .unwrap_or(0.0)
    }

    /// Subtract the guard band from the margin, preserving the evaluator's
    /// operation order even when adding it to a bound would round it away.
    pub fn signed_margin(&self, value: f64) -> Option<f64> {
        signed_margin(self.entry().min, self.entry().max, value)
            .map(|margin| margin - self.guard_band())
    }

    /// Every trial of one Monte Carlo analysis that answers this requirement,
    /// in retained order, judged as the yield gate counts it.
    ///
    /// The same candidates and the same passing rule as
    /// [`acceptance_is_blocked`], so a yield stated from these cannot differ
    /// from the population the sign-off divides: a trial that did not converge
    /// is a failing trial with no value, a guard band narrows what passes, the
    /// value is in the requirement's unit, and a trial outside the
    /// requirement's scope or producing analysis is not one of its trials.
    #[must_use]
    pub fn judge_trials<'a, W: AsRef<RetainedWaveform> + 'a>(
        &self,
        analysis: &'a AnalysisResult<W>,
    ) -> Vec<TrialJudgment> {
        if analysis.analysis_type != AnalysisType::MonteCarlo {
            return Vec::new();
        }
        candidates_for(self, std::iter::once(analysis))
            .iter()
            .map(|candidate| TrialJudgment {
                value: candidate.value,
                passing: candidate_is_passing(candidate),
            })
            .collect()
    }
}

/// One Monte Carlo trial, as one requirement judged it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrialJudgment {
    /// The trial's finite value in the requirement's unit; `None` when the
    /// trial produced none.
    pub value: Option<f64>,
    pub passing: bool,
}

fn evaluate_specifications<'a, W: AsRef<RetainedWaveform> + 'a>(
    specifications: &[PreparedSpecification],
    analyses: impl IntoIterator<Item = &'a AnalysisResult<W>> + Clone,
) -> Vec<SpecificationVerdict> {
    specifications
        .iter()
        .map(|specification| evaluate_specification(specification, analyses.clone()))
        .collect()
}

fn evaluate_specification<'a, W: AsRef<RetainedWaveform> + 'a>(
    specification: &PreparedSpecification,
    analyses: impl IntoIterator<Item = &'a AnalysisResult<W>>,
) -> SpecificationVerdict {
    let mut candidates = candidates_for(specification, analyses);
    verdict_over(specification, &mut candidates)
}

/// The verdict one requirement's candidates answer to.
///
/// Sorts the candidates worst first, as [`status_of`] does, so a caller that
/// placed them itself reads the worst one where the verdict found it.
pub(crate) fn verdict_over<C: Borrow<Candidate>>(
    specification: &PreparedSpecification,
    candidates: &mut [C],
) -> SpecificationVerdict {
    let spec = specification.entry();
    let definition = specification.definition();
    let specification_id = definition.map(|definition| definition.id);
    let requirement_key = definition.map(|definition| definition.requirement_key.clone());
    let evidence_count = u64::try_from(candidates.len()).unwrap_or(u64::MAX);
    let passing_evidence_count = definition.map_or(0, |_| {
        u64::try_from(
            candidates
                .iter()
                .filter(|candidate| candidate_is_passing((*candidate).borrow()))
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

    let status = status_of(candidates);
    let worst: &Candidate = candidates[0].borrow();
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

/// Every retained value that answers one requirement, in retained order: each
/// admitted analysis's own measurement, then its family members' values.
pub(crate) fn candidates_for<'a, W: AsRef<RetainedWaveform> + 'a>(
    specification: &PreparedSpecification,
    analyses: impl IntoIterator<Item = &'a AnalysisResult<W>>,
) -> Vec<Candidate> {
    let spec = specification.entry();
    analyses
        .into_iter()
        .enumerate()
        .filter_map(|(position, analysis)| {
            let provenance = analysis.provenance()?;
            let source_instance_id = provenance.authored_source_instance_id();
            specification.admits_analysis(analysis).then_some((
                position,
                analysis,
                source_instance_id,
            ))
        })
        .flat_map(|(position, analysis, source_instance_id)| {
            let is_monte_carlo = analysis.analysis_type == AnalysisType::MonteCarlo;
            let make = move |value: Option<f64>,
                             unit: Option<&rspice_core::analysis::MeasurementUnit>,
                             measured: bool,
                             member: Option<FamilyMemberId>| {
                let value = if !spec.unit.trim().is_empty() {
                    value.and_then(|value| {
                        unit.map_or(Ok(value), |unit| unit.convert_value(value, &spec.unit))
                            .ok()
                    })
                } else {
                    value
                };
                let value = value.filter(|value| value.is_finite());
                Candidate {
                    value,
                    measurement_passed: analysis.success && value.is_some() && measured,
                    signed_margin: value.and_then(|value| specification.signed_margin(value)),
                    source_instance_id,
                    is_monte_carlo,
                    member,
                    analysis: position,
                }
            };

            let analysis_level =
                analysis
                    .scalar_evidence(&spec.measurement)
                    .into_iter()
                    .map(move |evidence| {
                        make(
                            evidence.value,
                            evidence.unit.as_ref(),
                            evidence.passed,
                            None,
                        )
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
                        evidence.unit.as_ref(),
                        evidence.passed,
                        Some(member.member.clone()),
                    ))
                });

            analysis_level.chain(member_level)
        })
        .collect()
}

/// Whether one candidate holds: it was measured, and inside the guarded
/// limit when it has one.
pub(crate) fn candidate_is_passing(candidate: &Candidate) -> bool {
    candidate.measurement_passed && candidate.signed_margin.is_none_or(|margin| margin >= 0.0)
}

/// The terminal status one evidence set answers to, worst candidate first.
///
/// Sorts in place, so the caller that also needs the worst candidate reads it
/// at index zero rather than searching again. Extracted because the acceptance
/// gate has to ask the same question of a *subset* of the same evidence — the
/// non-Monte-Carlo candidates, once the yield gate has answered for the trials
/// — and a second copy of this ordering is how the gate would come to disagree
/// with the verdict it was handed.
pub(crate) fn status_of<C: Borrow<Candidate>>(candidates: &mut [C]) -> SpecificationVerdictStatus {
    candidates.sort_by(|left, right| {
        let (left, right) = (left.borrow(), right.borrow());
        left.measurement_passed
            .cmp(&right.measurement_passed)
            .then_with(|| compare_margin(left.signed_margin, right.signed_margin))
    });
    match candidates
        .first()
        .map(|worst| -> &Candidate { worst.borrow() })
    {
        None => SpecificationVerdictStatus::MissingEvidence,
        Some(worst) if !worst.measurement_passed => SpecificationVerdictStatus::MeasurementFailure,
        Some(worst) if worst.signed_margin.is_none_or(|margin| margin >= 0.0) => {
            SpecificationVerdictStatus::Pass
        }
        Some(_) => SpecificationVerdictStatus::BoundFailure,
    }
}

/// The Monte Carlo trials among one requirement's candidates, as the yield
/// gate counts them: every trial, and the ones that hold.
///
/// One count for the gate and for every surface that states a yield, so a
/// yield shown beside a limit is the population the sign-off divides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TrialCount {
    pub(crate) passing: usize,
    pub(crate) total: usize,
}

pub(crate) fn trial_count<C: Borrow<Candidate>>(candidates: &[C]) -> TrialCount {
    let trials = || {
        candidates
            .iter()
            .map(|candidate| -> &Candidate { candidate.borrow() })
            .filter(|candidate| candidate.is_monte_carlo)
    };
    TrialCount {
        passing: trials()
            .filter(|candidate| candidate_is_passing(candidate))
            .count(),
        total: trials().count(),
    }
}

pub fn acceptance_is_blocked<'a, W: AsRef<RetainedWaveform> + 'a>(
    specifications: &[PreparedSpecification],
    policy: &SpecificationPolicy,
    verdicts: &[SpecificationVerdict],
    analyses: impl IntoIterator<Item = &'a AnalysisResult<W>> + Clone,
) -> bool {
    specifications
        .iter()
        .zip(verdicts)
        .any(|(specification, verdict)| {
            blocks_acceptance(specification, policy, verdict, analyses.clone())
        })
}

/// Whether one requirement's verdict keeps the run from being accepted.
pub(crate) fn blocks_acceptance<'a, W: AsRef<RetainedWaveform> + 'a>(
    specification: &PreparedSpecification,
    policy: &SpecificationPolicy,
    verdict: &SpecificationVerdict,
    analyses: impl IntoIterator<Item = &'a AnalysisResult<W>>,
) -> bool {
    let definition = specification.definition();
    if definition.is_some_and(|definition| {
        definition.waiver.is_some() || definition.role != SpecificationRole::Blocking
    }) {
        return false;
    }

    let status_blocks = |status| match status {
        SpecificationVerdictStatus::Pass => false,
        SpecificationVerdictStatus::MissingEvidence => {
            policy.missing_measurement == MissingMeasurementPolicy::FailClosed
        }
        SpecificationVerdictStatus::BoundFailure
        | SpecificationVerdictStatus::MeasurementFailure => {
            !(policy.nominal_failure == NominalFailurePolicy::RecordDisposition
                && matches!(specification.entry().scope, SpecPointScope::Nominal))
        }
    };

    if matches!(
        policy.monte_carlo,
        MonteCarloSpecificationGate::YieldAtLeast { .. }
    ) && definition.is_some()
    {
        let mut candidates = candidates_for(specification, analyses);
        let count = trial_count(&candidates);
        if count.total > 0 {
            // Through the gate's own predicate, so the row the registry
            // draws and the sign-off this decides cannot disagree about
            // the same population.
            let yield_blocks = !policy.monte_carlo.clears(
                u64::try_from(count.passing).unwrap_or(u64::MAX),
                u64::try_from(count.total).unwrap_or(u64::MAX),
            );
            // The yield gate answers for the trials, and only for
            // them. A specification is routinely governed by corner
            // and parametric evidence as well, and a passing yield
            // says nothing about that — so this is the OR of the two,
            // never the first one that had an answer. Returning here
            // read a failed corner as "not blocked".
            candidates.retain(|candidate| !candidate.is_monte_carlo);
            let other_blocks = !candidates.is_empty() && status_blocks(status_of(&mut candidates));
            return yield_blocks || other_blocks;
        }
    }

    status_blocks(verdict.status)
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
mod tests;
