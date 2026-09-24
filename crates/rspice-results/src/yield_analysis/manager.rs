//! Yield population accounting independent of a simulation result representation.
use super::{YieldResult, YieldSpec, calculate_distribution_stats};
use std::collections::BTreeMap;

/// Exact observations available for yield analysis of a completed result.
pub trait YieldInput {
    fn is_monte_carlo(&self) -> bool;
    fn monte_carlo_samples(&self, target: &str) -> Option<&[f64]>;
    fn measurement(&self, name: &str) -> Option<f64>;
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
    pub fn add_spec(&mut self, spec: YieldSpec) {
        self.specs.insert(spec.target.clone(), spec);
    }

    /// Run yield analysis over retained simulation results. A Monte Carlo
    /// result contributes every exact target sample; other result kinds
    /// contribute at most one scalar measurement each.
    pub fn analyze<T: YieldInput>(&mut self, mc_results: &[T]) -> &BTreeMap<String, YieldResult> {
        self.results.clear();

        for (name, spec) in &self.specs {
            let mut pass_count = 0;
            let mut trail = Vec::new();
            let mut values = Vec::new();

            for result in mc_results {
                if let Some(samples) = result.monte_carlo_samples(name) {
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
                } else if result.is_monte_carlo() {
                    // A Monte Carlo result is a collection of trials, never a
                    // single observation represented by its summary mean. If
                    // the target variable is absent, there is no exact sample
                    // evidence to evaluate for this specification.
                    continue;
                } else if let Some(value) = result.measurement(name) {
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
    pub fn analyze_monte_carlo<T: YieldInput>(
        &mut self,
        result: &T,
    ) -> Option<&BTreeMap<String, YieldResult>> {
        result
            .is_monte_carlo()
            .then(|| self.analyze(std::slice::from_ref(result)))
    }

    /// Retained per-specification evidence from the last analyzed population.
    pub fn results(&self) -> &BTreeMap<String, YieldResult> {
        &self.results
    }
}

#[cfg(test)]
#[test]
fn specifications_are_ordered_and_missing_scalar_measurements_fail() {
    struct Scalar;
    impl YieldInput for Scalar {
        fn is_monte_carlo(&self) -> bool {
            false
        }

        fn monte_carlo_samples(&self, _target: &str) -> Option<&[f64]> {
            None
        }

        fn measurement(&self, name: &str) -> Option<f64> {
            (name == "alpha").then_some(2.0)
        }
    }

    let mut manager = YieldAnalysisManager::new();
    manager.add_spec(YieldSpec::lower("zeta", 0.0, ""));
    manager.add_spec(YieldSpec::lower("alpha", 1.0, ""));
    let results = manager.analyze(&[Scalar]);
    assert_eq!(
        results.keys().map(String::as_str).collect::<Vec<_>>(),
        ["alpha", "zeta"]
    );
    assert_eq!(results["alpha"].pass_count, 1);
    assert_eq!(results["zeta"].fail_count, 1);
    assert_eq!(results["zeta"].trail, [false]);
}
