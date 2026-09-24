//! Adapter from completed simulation results to portable yield analysis.
use crate::product::{DatasetId, RunId};
use crate::simulation::results::SimulationResult;
use rspice_results::yield_analysis::{MonteCarloSamplingMode, YieldAnalysisProvenance, YieldInput};
#[cfg(test)]
use rspice_results::yield_analysis::{YieldAnalysisManager, YieldSpec};
#[cfg(test)]
use std::collections::BTreeMap;

/// Bind completed Monte Carlo evidence to its retained run and dataset.
#[must_use]
pub fn yield_provenance_from_monte_carlo_result(
    source_run_id: RunId,
    source_dataset_id: DatasetId,
    result: &SimulationResult,
) -> Option<YieldAnalysisProvenance> {
    match result {
        SimulationResult::MonteCarlo {
            seed,
            runs_requested,
            runs_completed,
            ..
        } => Some(YieldAnalysisProvenance {
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

impl YieldInput for SimulationResult {
    fn is_monte_carlo(&self) -> bool {
        matches!(self, Self::MonteCarlo { .. })
    }

    fn monte_carlo_samples(&self, target: &str) -> Option<&[f64]> {
        let Self::MonteCarlo { variables, .. } = self else {
            return None;
        };
        let target = target.trim();
        let variable_name = parse_wrapped_target(target, "mean").unwrap_or(target);
        variables
            .iter()
            .find(|variable| variable.name == variable_name)
            .map(|variable| variable.samples.as_slice())
    }

    fn measurement(&self, name: &str) -> Option<f64> {
        SimulationResult::measurement(self, name)
    }
}

fn parse_wrapped_target<'a>(target: &'a str, prefix: &str) -> Option<&'a str> {
    if target.len() <= prefix.len() + 2
        || !target[..prefix.len()].eq_ignore_ascii_case(prefix)
        || !target[prefix.len()..].starts_with('(')
        || !target.ends_with(')')
    {
        return None;
    }
    Some(&target[prefix.len() + 1..target.len() - 1])
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::results::MonteCarloVariableResult;

    fn monte_carlo_result(name: &str, samples: Vec<f64>) -> SimulationResult {
        let finite = samples
            .iter()
            .copied()
            .filter(|value| value.is_finite())
            .collect::<Vec<_>>();
        let mean = finite.iter().sum::<f64>() / finite.len() as f64;
        let min = finite.iter().copied().fold(f64::INFINITY, f64::min);
        let max = finite.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        SimulationResult::MonteCarlo {
            member_measurements: Vec::new(),
            seed: 42,
            runs_requested: samples.len(),
            runs_completed: samples.len(),
            num_failures: 0,
            all_converged: true,
            variables: vec![MonteCarloVariableResult {
                mean_confidence: None,
                name: name.to_owned(),
                samples,
                mean,
                std_dev: 0.0,
                min,
                max,
                histogram: Vec::new(),
                bin_edges: Vec::new(),
            }],
        }
    }

    #[test]
    fn monte_carlo_yield_evaluates_every_exact_sample() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::range("V(out)", 0.9, 1.1, "V"));
        let result = monte_carlo_result("V(out)", vec![0.8, 0.9, 1.0, 1.1, 1.2]);

        let yield_result = &manager.analyze(&[result])["V(out)"];

        assert_eq!(yield_result.total_runs, 5);
        assert_eq!(yield_result.pass_count, 3);
        assert_eq!(yield_result.fail_count, 2);
        assert_eq!(yield_result.yield_percent, 60.0);
        assert_eq!(yield_result.trail, vec![false, true, true, true, false]);
        assert_eq!(yield_result.samples, vec![0.8, 0.9, 1.0, 1.1, 1.2]);
        assert_eq!(yield_result.stats.count, 5);
        assert_eq!(yield_result.stats.mean, 1.0);
    }

    #[test]
    fn wrapped_mean_target_uses_samples_instead_of_summary_mean() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::upper("mean(V(out))", 1.0, "V"));
        let result = monte_carlo_result("V(out)", vec![0.8, 1.2]);

        let yield_result = &manager.analyze(&[result])["mean(V(out))"];

        assert_eq!(yield_result.total_runs, 2);
        assert_eq!(yield_result.pass_count, 1);
        assert_eq!(yield_result.fail_count, 1);
        assert_eq!(yield_result.yield_percent, 50.0);
    }

    #[test]
    fn non_finite_monte_carlo_observation_is_a_failed_trial_not_a_statistic() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::lower("gain", 0.0, ""));
        let result = monte_carlo_result("gain", vec![1.0, f64::NAN, 2.0]);

        let yield_result = &manager.analyze(&[result])["gain"];

        assert_eq!(yield_result.total_runs, 3);
        assert_eq!(yield_result.pass_count, 2);
        assert_eq!(yield_result.fail_count, 1);
        assert_eq!(yield_result.trail, vec![true, false, true]);
        assert_eq!(yield_result.samples, vec![1.0, 2.0]);
        assert_eq!(yield_result.stats.count, 2);
    }

    #[test]
    fn non_monte_carlo_results_preserve_one_scalar_per_result_behavior() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::lower("gain", 1.0, ""));
        let passing = SimulationResult::MeasurementsOnly {
            measurements: BTreeMap::from([("gain".to_owned(), 1.2)])
                .into_iter()
                .collect(),
        };
        let failing = SimulationResult::MeasurementsOnly {
            measurements: BTreeMap::from([("gain".to_owned(), 0.8)])
                .into_iter()
                .collect(),
        };

        let yield_result = &manager.analyze(&[passing, failing])["gain"];

        assert_eq!(yield_result.total_runs, 2);
        assert_eq!(yield_result.pass_count, 1);
        assert_eq!(yield_result.fail_count, 1);
        assert_eq!(yield_result.trail, vec![true, false]);
        assert_eq!(yield_result.samples, vec![1.2, 0.8]);
    }

    /// The denominator convention, stated where the number is produced.
    ///
    /// It counts every trial the engine completed, not every trial that
    /// produced a usable observation: a diverged trial is in the denominator
    /// *and* in the failures, because a specification a run could not
    /// evaluate is not one it met. That is what [`rspice_results::yield_analysis::YieldResult::total_runs`]
    /// documents, and this name used to claim the opposite of it.
    #[test]
    fn the_yield_denominator_counts_every_trial_the_engine_completed() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::lower("gain", 1.0, ""));
        // Nine completed trials, one of which diverged into a non-finite
        // observation. The engine reports it as a trial; it is not a sample.
        let result = monte_carlo_result(
            "gain",
            vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, f64::NAN],
        );

        let yield_result = &manager.analyze(&[result])["gain"];

        assert_eq!(
            yield_result.total_runs, 9,
            "every trial the engine reported is in the denominator"
        );
        assert_eq!(yield_result.samples.len(), 8);
        assert_eq!(yield_result.pass_count, 8);
        assert_eq!(
            yield_result.fail_count, 1,
            "a trial with no finite observation cannot be counted as a pass"
        );
    }

    #[test]
    fn aggregate_monte_carlo_entry_point_preserves_evidence_on_later_scalar_result() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::lower("gain", 1.0, ""));
        let monte_carlo = monte_carlo_result("gain", vec![0.8, 1.2]);
        let scalar = SimulationResult::MeasurementsOnly {
            measurements: BTreeMap::from([("gain".to_owned(), 9.0)])
                .into_iter()
                .collect(),
        };

        let initial = manager
            .analyze_monte_carlo(&monte_carlo)
            .expect("Monte Carlo result is analyzed")["gain"]
            .clone();
        assert!(manager.analyze_monte_carlo(&scalar).is_none());

        assert_eq!(manager.results()["gain"].total_runs, initial.total_runs);
        assert_eq!(manager.results()["gain"].trail, initial.trail);
        assert_eq!(manager.results()["gain"].samples, initial.samples);
    }
}
