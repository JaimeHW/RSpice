//! Yield analysis over completed simulation results.
use crate::product::{DatasetId, RunId};
use crate::simulation::results::SimulationResult;
use std::collections::BTreeMap;

use rspice_results::yield_analysis::{
    MonteCarloSamplingMode, YieldAnalysisProvenance, YieldResult, YieldSpec,
    calculate_distribution_stats,
};

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

// =============================================================================
// Yield Analysis Manager
// =============================================================================

/// Manager for yield analysis workflows
pub struct YieldAnalysisManager {
    /// Active specifications
    specs: BTreeMap<String, YieldSpec>,
    /// Results cached by spec name
    results: BTreeMap<String, YieldResult>,
}

impl Default for YieldAnalysisManager {
    fn default() -> Self {
        Self::new()
    }
}

impl YieldAnalysisManager {
    pub fn new() -> Self {
        Self {
            specs: BTreeMap::new(),
            results: BTreeMap::new(),
        }
    }

    /// Add a yield specification
    #[cfg(test)]
    pub fn add_spec(&mut self, spec: YieldSpec) {
        self.specs.insert(spec.target.clone(), spec);
    }

    /// Run yield analysis over retained simulation results. A Monte Carlo
    /// result contributes every exact target sample; other result kinds
    /// contribute at most one scalar measurement each.
    pub fn analyze(&mut self, mc_results: &[SimulationResult]) -> &BTreeMap<String, YieldResult> {
        self.results.clear();

        for (name, spec) in &self.specs {
            let mut pass_count = 0;
            let mut trail = Vec::new();
            let mut values = Vec::new();

            for result in mc_results {
                if let Some(samples) = Self::monte_carlo_samples(result, name) {
                    trail.reserve(samples.len());
                    values.reserve(samples.len());
                    for value in samples.iter().copied() {
                        let passes = value.is_finite() && spec.evaluates(value);
                        if passes {
                            pass_count += 1;
                        }
                        trail.push(passes);
                        if value.is_finite() {
                            values.push(value);
                        }
                    }
                } else if matches!(result, SimulationResult::MonteCarlo { .. }) {
                    // A Monte Carlo result is a collection of trials, never a
                    // single observation represented by its summary mean. If
                    // the target variable is absent, there is no exact sample
                    // evidence to evaluate for this specification.
                    continue;
                } else if let Some(value) = self.extract_measurement(result, name) {
                    let passes = value.is_finite() && spec.evaluates(value);
                    if passes {
                        pass_count += 1;
                    }
                    trail.push(passes);
                    if value.is_finite() {
                        values.push(value);
                    }
                } else {
                    // Missing measurement is treated as a failed run for this spec.
                    trail.push(false);
                }
            }

            let num_runs = trail.len();
            let stats = calculate_distribution_stats(&values, spec);
            let yield_result = YieldResult {
                spec: spec.clone(),
                total_runs: num_runs,
                pass_count,
                fail_count: num_runs.saturating_sub(pass_count),
                yield_percent: if num_runs == 0 {
                    0.0
                } else {
                    (pass_count as f64 / num_runs as f64) * 100.0
                },
                stats,
                trail,
                samples: values,
            };
            self.results.insert(name.clone(), yield_result);
        }

        &self.results
    }

    /// Analyze one aggregate Monte Carlo dataset. Non-Monte-Carlo results do
    /// not clear or replace the previously retained Monte Carlo evidence.
    pub fn analyze_monte_carlo(
        &mut self,
        result: &SimulationResult,
    ) -> Option<&BTreeMap<String, YieldResult>> {
        matches!(result, SimulationResult::MonteCarlo { .. })
            .then(|| self.analyze(std::slice::from_ref(result)))
    }

    /// Return exact Monte Carlo samples for a target. `mean(name)` remains an
    /// accepted target spelling for compatibility, but it selects the sample
    /// population rather than collapsing it to the recorded summary mean.
    fn monte_carlo_samples<'a>(result: &'a SimulationResult, target: &str) -> Option<&'a [f64]> {
        let SimulationResult::MonteCarlo { variables, .. } = result else {
            return None;
        };
        let target = target.trim();
        let variable_name = parse_wrapped_target(target, "mean").unwrap_or(target);
        variables
            .iter()
            .find(|variable| variable.name == variable_name)
            .map(|variable| variable.samples.as_slice())
    }

    /// Extract a specific measurement value from a simulation result
    fn extract_measurement(&self, result: &SimulationResult, name: &str) -> Option<f64> {
        result.measurement(name)
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

    #[test]
    fn specifications_and_results_iterate_in_deterministic_target_order() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::lower("zeta", 0.0, ""));
        manager.add_spec(YieldSpec::lower("alpha", 0.0, ""));

        let keys = manager
            .analyze(&[])
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>();

        assert_eq!(keys, vec!["alpha", "zeta"]);
    }

    /// The denominator convention, stated where the number is produced.
    ///
    /// It counts every trial the engine completed, not every trial that
    /// produced a usable observation: a diverged trial is in the denominator
    /// *and* in the failures, because a specification a run could not
    /// evaluate is not one it met. That is what [`YieldResult::total_runs`]
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

        assert_eq!(manager.results["gain"].total_runs, initial.total_runs);
        assert_eq!(manager.results["gain"].trail, initial.trail);
        assert_eq!(manager.results["gain"].samples, initial.samples);
    }
}
