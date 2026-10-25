//! Monte Carlo yield, judged by the specification judge the sign-off uses.
//!
//! A yield is one requirement's trials: how many there were and how many held.
//! The acceptance gate already divides that population, so the yield shown
//! beside a limit is taken from the same judgment rather than a second
//! arithmetic that could disagree with it about a trial that did not converge,
//! a guard band, a unit or a scope.

use super::{
    MonteCarloSamplingMode, SpecLimitType, YieldAnalysisProvenance, YieldResult, YieldSpec,
    calculate_distribution_stats,
};
use crate::analysis_result::AnalysisResult;
use crate::family_metadata::AnalysisResultFamilyMetadata;
use crate::specification::{PreparedSpecification, SpecEntry, SpecificationDefinition};
use crate::waveform::RetainedWaveform;
use rspice_vocabulary::product::{DatasetId, RunId};

/// The inputs one yield population was judged from: the Monte Carlo analysis
/// (by dataset and analysis sequence) and the limits it was held against.
///
/// Held so that judging again is asked only when one of them changed, and
/// compared by value, because a limit can change by any route that edits the
/// plan.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldBasis {
    analysis: Option<(DatasetId, u64)>,
    specs: Vec<SpecEntry>,
    definitions: Vec<SpecificationDefinition>,
}

impl YieldBasis {
    #[must_use]
    pub fn new(
        analysis: Option<(DatasetId, u64)>,
        specs: &[SpecEntry],
        definitions: &[SpecificationDefinition],
    ) -> Self {
        Self {
            analysis,
            specs: specs.to_vec(),
            definitions: definitions.to_vec(),
        }
    }

    pub(super) fn is(
        &self,
        analysis: Option<(DatasetId, u64)>,
        specs: &[SpecEntry],
        definitions: &[SpecificationDefinition],
    ) -> bool {
        self.analysis == analysis && self.specs == specs && self.definitions == definitions
    }
}

/// The yield of every bounded requirement over one Monte Carlo analysis.
///
/// A requirement with neither bound has no yield to state, and one that no
/// trial answers (its measurement was not taken, or the trials are outside its
/// scope or producing analysis) has no population: neither gives a row.
#[must_use]
pub fn judge_monte_carlo<'a, W: AsRef<RetainedWaveform> + 'a>(
    analysis: &'a AnalysisResult<W>,
    specifications: &[PreparedSpecification],
) -> Vec<YieldResult> {
    specifications
        .iter()
        .filter_map(|specification| judge(analysis, specification))
        .collect()
}

fn judge<'a, W: AsRef<RetainedWaveform> + 'a>(
    analysis: &'a AnalysisResult<W>,
    specification: &PreparedSpecification,
) -> Option<YieldResult> {
    let entry = specification.entry();
    let limit_type = match (entry.min, entry.max) {
        (Some(_), Some(_)) => SpecLimitType::Range,
        (Some(_), None) => SpecLimitType::Lower,
        (None, Some(_)) => SpecLimitType::Upper,
        (None, None) => return None,
    };
    let trials = specification.judge_trials(analysis);
    if trials.is_empty() {
        return None;
    }
    let spec = YieldSpec {
        target: output_name(analysis, &entry.measurement),
        limit_type,
        min: entry.min,
        max: entry.max,
        target_val: None,
        unit: entry.unit.clone(),
        weight: 1.0,
    };
    let trail = trials.iter().map(|trial| trial.passing).collect::<Vec<_>>();
    let samples = trials
        .iter()
        .filter_map(|trial| trial.value)
        .collect::<Vec<_>>();
    let total_runs = trail.len();
    let pass_count = trail.iter().filter(|passing| **passing).count();
    Some(YieldResult {
        stats: calculate_distribution_stats(&samples, &spec),
        spec,
        total_runs,
        pass_count,
        fail_count: total_runs - pass_count,
        yield_percent: pass_count as f64 / total_runs as f64 * 100.0,
        trail,
        samples,
    })
}

/// The output as the trials spell it, which is the name every other reader
/// of the run looks it up by; the requirement may spell it in another case.
fn output_name<W>(analysis: &AnalysisResult<W>, measurement: &str) -> String {
    analysis
        .family_metadata
        .iter()
        .flat_map(AnalysisResultFamilyMetadata::member_measurements)
        .find_map(|member| member.evidence_for(measurement))
        .map_or_else(|| measurement.to_owned(), |evidence| evidence.name.clone())
}

/// The Monte Carlo campaign one analysis retained, bound to its run.
#[must_use]
pub fn monte_carlo_provenance<W>(
    source_run_id: RunId,
    source_dataset_id: DatasetId,
    analysis: &AnalysisResult<W>,
) -> Option<YieldAnalysisProvenance> {
    match &analysis.family_metadata {
        Some(AnalysisResultFamilyMetadata::MonteCarlo {
            seed,
            runs_requested,
            runs_completed,
            ..
        }) => Some(YieldAnalysisProvenance {
            source_run_id,
            source_dataset_id,
            seed: *seed,
            runs_requested: *runs_requested,
            runs_completed: *runs_completed,
            sampling_mode: MonteCarloSamplingMode::PseudoRandom,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis_type::AnalysisType;
    use crate::family_measurements::{
        FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
    };
    use crate::provenance::AnalysisResultProvenance;
    use crate::specification::{SpecEntry, SpecPointScope, SpecificationDefinition};
    use crate::specification_verdict::SpecificationVerdict;
    use rspice_vocabulary::product::{
        AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId,
    };

    /// A Monte Carlo analysis whose trials measured `gain`; `None` is a trial
    /// that did not converge.
    fn trials(values: &[Option<f64>]) -> AnalysisResult {
        let members = values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                FamilyMemberMeasurements::new(
                    FamilyMemberId::MonteCarloTrial {
                        index,
                        seed: 700 + index as u64,
                    },
                    vec![FamilyMeasurementEvidence {
                        unit: None,
                        name: "Gain".to_owned(),
                        value: *value,
                        passed: value.is_some(),
                        error: value.is_none().then(|| "did not converge".to_owned()),
                    }],
                )
            })
            .collect();
        let completed = values.iter().filter(|value| value.is_some()).count();
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "Monte Carlo", 0.0)
            .with_family_metadata(AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 700,
                runs_requested: values.len(),
                runs_completed: completed,
                failures: values.len() - completed,
                all_converged: completed == values.len(),
                variables: Vec::new(),
                member_measurements: members,
            })
            .with_provenance(
                AnalysisResultProvenance::new(
                    AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0x37; 32]),
                    Vec::new(),
                )
                .expect("valid provenance"),
            )
    }

    fn limit(min: Option<f64>, max: Option<f64>) -> SpecEntry {
        SpecEntry {
            measurement: "gain".to_owned(),
            expression: String::new(),
            min,
            max,
            unit: "dB".to_owned(),
            scope: SpecPointScope::AllPoints,
        }
    }

    fn governed(entry: &SpecEntry, guard_band: Option<f64>) -> PreparedSpecification {
        let mut definition =
            SpecificationDefinition::from_legacy(SimulationPlanId::new(), 0, entry);
        definition.guard_band = guard_band;
        PreparedSpecification::from_definition(definition).expect("valid requirement")
    }

    #[test]
    fn a_bounded_limit_yields_over_every_trial_under_the_trials_own_name() {
        let analysis = trials(&[Some(12.0), Some(8.5), Some(11.0), Some(10.5)]);
        let results = judge_monte_carlo(
            &analysis,
            &[PreparedSpecification::new(limit(Some(10.0), None)).unwrap()],
        );

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.spec.target, "Gain", "named as the trials name it");
        assert_eq!(result.spec.limit_type, SpecLimitType::Lower);
        assert_eq!(result.spec.unit, "dB");
        assert_eq!((result.total_runs, result.pass_count), (4, 3));
        assert_eq!(result.fail_count, 1);
        assert_eq!(result.yield_percent, 75.0);
        assert_eq!(result.trail, [true, false, true, true]);
        assert_eq!(result.samples, [12.0, 8.5, 11.0, 10.5]);
        assert_eq!(result.stats.count, 4);
        assert_eq!(result.stats.mean, 10.5);
    }

    #[test]
    fn a_trial_that_did_not_converge_is_a_failing_trial_with_no_sample() {
        let analysis = trials(&[Some(12.0), None, Some(11.0)]);
        let results = judge_monte_carlo(
            &analysis,
            &[PreparedSpecification::new(limit(Some(10.0), Some(20.0))).unwrap()],
        );

        let result = &results[0];
        assert_eq!(result.spec.limit_type, SpecLimitType::Range);
        assert_eq!((result.total_runs, result.pass_count), (3, 2));
        assert_eq!(result.trail, [true, false, true]);
        assert_eq!(result.samples, [12.0, 11.0], "no value is invented");
    }

    #[test]
    fn a_guard_band_narrows_what_passes() {
        let analysis = trials(&[Some(12.0), Some(10.5), Some(11.0)]);
        let results =
            judge_monte_carlo(&analysis, &[governed(&limit(Some(10.0), None), Some(0.6))]);

        assert_eq!(
            results[0].trail,
            [true, false, true],
            "10.5 dB holds the 10 dB floor but not its 0.6 dB guard band"
        );
    }

    /// The yield is the population the sign-off divides, trial for trial.
    #[test]
    fn yield_counts_are_the_counts_the_verdict_and_gate_use() {
        let analysis = trials(&[Some(12.0), None, Some(10.5), Some(9.0), Some(11.0)]);
        let specifications = [governed(&limit(Some(10.0), Some(11.5)), Some(0.25))];

        let results = judge_monte_carlo(&analysis, &specifications);
        let verdicts = SpecificationVerdict::evaluate(&specifications, [&analysis]);

        assert_eq!(
            results[0].total_runs as u64,
            verdicts[0].evidence_count(),
            "the same trials"
        );
        assert_eq!(
            results[0].pass_count as u64,
            verdicts[0].passing_evidence_count(),
            "the same trials held"
        );
    }

    #[test]
    fn a_limit_with_no_bound_or_no_trials_gives_no_yield_row() {
        let analysis = trials(&[Some(12.0), Some(11.0)]);
        let unmeasured = SpecEntry {
            measurement: "bw".to_owned(),
            ..limit(Some(1.0), None)
        };
        let out_of_scope = SpecEntry {
            scope: SpecPointScope::SelectedCorners {
                corners: vec!["SS".to_owned()],
            },
            ..limit(Some(1.0), None)
        };

        let results = judge_monte_carlo(
            &analysis,
            &[
                PreparedSpecification::new(limit(None, None)).unwrap(),
                PreparedSpecification::new(unmeasured).unwrap(),
                PreparedSpecification::new(out_of_scope).unwrap(),
            ],
        );

        assert!(results.is_empty(), "{results:?}");
    }

    #[test]
    fn yield_provenance_comes_from_the_monte_carlo_metadata() {
        let analysis = trials(&[Some(12.0), None, Some(11.0)]);
        let run_id = RunId::new();
        let dataset_id = DatasetId::new();

        let provenance = monte_carlo_provenance(run_id, dataset_id, &analysis).unwrap();

        assert_eq!(provenance.source_run_id, run_id);
        assert_eq!(provenance.source_dataset_id, dataset_id);
        assert_eq!(provenance.seed, 700);
        assert_eq!(
            (provenance.runs_requested, provenance.runs_completed),
            (3, 2)
        );
        let transient: AnalysisResult =
            AnalysisResult::new(2, AnalysisType::Transient, "Transient", 0.0);
        assert!(monte_carlo_provenance(run_id, dataset_id, &transient).is_none());
    }
}
