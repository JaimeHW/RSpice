//! Yield Analysis Manager
//!
//! Provides commercial-grade statistical analysis for Monte Carlo simulations.
//! Extracts yield, distributions, and specifications pass/fail status.
//!
//! # Features
//!
//! - Statistical yield calculation (Pass/Fail %)
//! - Automatic distribution extraction (Mean, StdDev, Skewness, Kurtosis)
//! - Specification management (Min/Max targets)
//! - Capability indices (Cp, Cpk)
//! - Sensitivity analysis support (Importance)

use crate::product::{DatasetId, RunId};
use crate::simulation::results::SimulationResult;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// =============================================================================
// Yield Specifications
// =============================================================================

/// Type of specification limit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecLimitType {
    /// Lower specification limit (LSL)
    Lower,
    /// Upper specification limit (USL)
    Upper,
    /// Both LSL and USL (Range)
    Range,
}

/// A target specification for a measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldSpec {
    /// Name of the measurement/signal
    pub target: String,
    /// Type of limit
    pub limit_type: SpecLimitType,
    /// Lower limit value
    pub min: Option<f64>,
    /// Upper limit value
    pub max: Option<f64>,
    /// Target nominal value
    pub target_val: Option<f64>,
    /// Unit (e.g., "V", "ns")
    pub unit: String,
    /// Weight/Priority for optimization (0.0 - 1.0)
    pub weight: f32,
}

impl YieldSpec {
    /// Create a lower limit spec
    pub fn lower(target: impl Into<String>, min: f64, unit: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            limit_type: SpecLimitType::Lower,
            min: Some(min),
            max: None,
            target_val: None,
            unit: unit.into(),
            weight: 1.0,
        }
    }

    /// Create an upper limit spec
    pub fn upper(target: impl Into<String>, max: f64, unit: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            limit_type: SpecLimitType::Upper,
            min: None,
            max: Some(max),
            target_val: None,
            unit: unit.into(),
            weight: 1.0,
        }
    }

    /// Create a range spec
    pub fn range(target: impl Into<String>, min: f64, max: f64, unit: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            limit_type: SpecLimitType::Range,
            min: Some(min),
            max: Some(max),
            target_val: Some((min + max) / 2.0),
            unit: unit.into(),
            weight: 1.0,
        }
    }

    /// Check if a value passes the specification
    pub fn evaluates(&self, value: f64) -> bool {
        match self.limit_type {
            SpecLimitType::Lower => self.min.is_none_or(|m| value >= m),
            SpecLimitType::Upper => self.max.is_none_or(|m| value <= m),
            SpecLimitType::Range => {
                let lower = self.min.is_none_or(|m| value >= m);
                let upper = self.max.is_none_or(|m| value <= m);
                lower && upper
            }
        }
    }
}

// =============================================================================
// Statistical Results
// =============================================================================

/// Statistical distribution data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DistributionStats {
    pub count: usize,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    /// Process capability index
    pub cp: Option<f64>,
    /// Process capability index (centered)
    pub cpk: Option<f64>,
}

/// Yield result for a single specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldResult {
    pub spec: YieldSpec,
    pub total_runs: usize,
    pub pass_count: usize,
    pub fail_count: usize,
    pub yield_percent: f64,
    pub stats: DistributionStats,
    /// Pass/Fail list for each iteration
    pub trail: Vec<bool>,
    /// Exact finite sample values used for the distribution statistics. This
    /// keeps verification plots evidence-backed instead of reconstructing a
    /// synthetic distribution from summary moments.
    #[serde(default)]
    pub samples: Vec<f64>,
}

/// Immutable identity of the retained result dataset used for the current
/// yield evidence. Stable product IDs prevent a history reorder from silently
/// retargeting verification results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct YieldAnalysisProvenance {
    pub source_run_id: RunId,
    pub source_dataset_id: DatasetId,
    pub seed: u64,
    pub runs_requested: usize,
    pub runs_completed: usize,
    pub sampling_mode: MonteCarloSamplingMode,
}

/// Sampling algorithm used by the current Monte Carlo engine. The engine uses
/// a deterministic pseudo-random number generator; stratified and Latin
/// hypercube modes are not represented until they are actually implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonteCarloSamplingMode {
    PseudoRandom,
}

impl MonteCarloSamplingMode {
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::PseudoRandom => "Pseudo-random",
        }
    }
}

impl YieldAnalysisProvenance {
    #[must_use]
    pub fn from_monte_carlo_result(
        source_run_id: RunId,
        source_dataset_id: DatasetId,
        result: &SimulationResult,
    ) -> Option<Self> {
        match result {
            SimulationResult::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                ..
            } => Some(Self {
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

    /// Clear all specs
    pub fn clear_specs(&mut self) {
        self.specs.clear();
        self.results.clear();
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
            let stats = self.calculate_stats(&values, spec);
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

    /// Calculate comprehensive statistical metrics
    fn calculate_stats(&self, values: &[f64], spec: &YieldSpec) -> DistributionStats {
        if values.is_empty() {
            return DistributionStats::default();
        }

        // Defensive filtering so direct callers cannot poison statistics with NaN/Inf.
        let mut finite_values = values
            .iter()
            .copied()
            .filter(|value| value.is_finite())
            .collect::<Vec<_>>();
        if finite_values.is_empty() {
            return DistributionStats::default();
        }

        if finite_values.len() == 1 {
            let value = finite_values[0];
            return DistributionStats {
                count: 1,
                mean: value,
                std_dev: 0.0,
                min: value,
                max: value,
                median: value,
                skewness: 0.0,
                kurtosis: 0.0,
                cp: None,
                cpk: None,
            };
        }

        let n = finite_values.len() as f64;
        let mean = finite_values.iter().sum::<f64>() / n;
        let variance_num = finite_values
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>();
        let variance = variance_num / (n - 1.0);
        let std_dev = if variance.is_finite() && variance > 0.0 {
            variance.sqrt()
        } else {
            0.0
        };

        finite_values.sort_by(f64::total_cmp);
        let lower_mid = finite_values[(finite_values.len() - 1) / 2];
        let upper_mid = finite_values[finite_values.len() / 2];
        let median = (lower_mid + upper_mid) * 0.5;
        let min = finite_values[0];
        let max = finite_values[finite_values.len() - 1];

        // Skewness and kurtosis remain centered moments around the mean.
        let (skewness, kurtosis) = if std_dev > 0.0 {
            let m3 = finite_values
                .iter()
                .map(|value| (value - mean).powi(3))
                .sum::<f64>()
                / n;
            let m4 = finite_values
                .iter()
                .map(|value| (value - mean).powi(4))
                .sum::<f64>()
                / n;
            let skewness = m3 / std_dev.powi(3);
            let kurtosis = (m4 / std_dev.powi(4)) - 3.0;
            if skewness.is_finite() && kurtosis.is_finite() {
                (skewness, kurtosis)
            } else {
                (0.0, 0.0)
            }
        } else {
            (0.0, 0.0)
        };

        // Capability Indices
        let mut cp = None;
        let mut cpk = None;

        if let (Some(lsl), Some(usl)) = (spec.min, spec.max)
            && lsl.is_finite()
            && usl.is_finite()
            && usl > lsl
            && std_dev > 0.0
        {
            let cp_val = (usl - lsl) / (6.0 * std_dev);
            if cp_val.is_finite() {
                cp = Some(cp_val);
            }

            let cpu = (usl - mean) / (3.0 * std_dev);
            let cpl = (mean - lsl) / (3.0 * std_dev);
            let cpk_val = cpu.min(cpl);
            if cpk_val.is_finite() {
                cpk = Some(cpk_val);
            }
        }

        DistributionStats {
            count: finite_values.len(),
            mean,
            std_dev,
            min,
            max,
            median,
            skewness,
            kurtosis,
            cp,
            cpk,
        }
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
            seed: 42,
            runs_requested: samples.len(),
            runs_completed: samples.len(),
            num_failures: 0,
            all_converged: true,
            variables: vec![MonteCarloVariableResult {
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
