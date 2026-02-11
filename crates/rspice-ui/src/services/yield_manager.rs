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

use crate::simulation::results::SimulationResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
            SpecLimitType::Lower => self.min.map_or(true, |m| value >= m),
            SpecLimitType::Upper => self.max.map_or(true, |m| value <= m),
            SpecLimitType::Range => {
                let lower = self.min.map_or(true, |m| value >= m);
                let upper = self.max.map_or(true, |m| value <= m);
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
    pub yield_percent: f32,
    pub stats: DistributionStats,
    /// Pass/Fail list for each iteration
    pub trail: Vec<bool>,
}

// =============================================================================
// Yield Analysis Manager
// =============================================================================

/// Manager for yield analysis workflows
pub struct YieldAnalysisManager {
    /// Active specifications
    specs: HashMap<String, YieldSpec>,
    /// Results cached by spec name
    results: HashMap<String, YieldResult>,
}

impl Default for YieldAnalysisManager {
    fn default() -> Self {
        Self::new()
    }
}

impl YieldAnalysisManager {
    pub fn new() -> Self {
        Self {
            specs: HashMap::new(),
            results: HashMap::new(),
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

    /// Run yield analysis on a set of Monte Carlo results
    /// `mc_results` is a list of results, one per iteration
    pub fn analyze(&mut self, mc_results: &[SimulationResult]) -> &HashMap<String, YieldResult> {
        self.results.clear();
        let num_runs = mc_results.len();

        for (name, spec) in &self.specs {
            let mut pass_count = 0;
            let mut trail = Vec::with_capacity(num_runs);
            let mut values = Vec::with_capacity(num_runs);

            for result in mc_results {
                if let Some(val) = self.extract_measurement(result, name) {
                    if val.is_finite() {
                        let passes = spec.evaluates(val);
                        if passes {
                            pass_count += 1;
                        }
                        trail.push(passes);
                        values.push(val);
                    } else {
                        trail.push(false);
                    }
                } else {
                    // Missing measurement is treated as a failed run for this spec.
                    trail.push(false);
                }
            }

            let stats = self.calculate_stats(&values, spec);
            let yield_result = YieldResult {
                spec: spec.clone(),
                total_runs: num_runs,
                pass_count,
                fail_count: num_runs.saturating_sub(pass_count),
                yield_percent: if num_runs == 0 {
                    0.0
                } else {
                    (pass_count as f32 / num_runs as f32) * 100.0
                },
                stats,
                trail,
            };
            self.results.insert(name.clone(), yield_result);
        }

        &self.results
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

        if let (Some(lsl), Some(usl)) = (spec.min, spec.max) {
            if lsl.is_finite() && usl.is_finite() && usl > lsl && std_dev > 0.0 {
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::results::SimulationResult;

    #[test]
    fn test_yield_spec_evaluation() {
        let spec = YieldSpec::range("v_out", 1.1, 1.3, "V");
        assert!(spec.evaluates(1.2));
        assert!(spec.evaluates(1.1));
        assert!(spec.evaluates(1.3));
        assert!(!spec.evaluates(1.09));
        assert!(!spec.evaluates(1.31));

        let lsl = YieldSpec::lower("i_dd", 0.001, "A");
        assert!(lsl.evaluates(0.002));
        assert!(!lsl.evaluates(0.0005));
    }

    #[test]
    fn test_statistical_calculations() {
        let manager = YieldAnalysisManager::new();
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let spec = YieldSpec::range("test", 1.0, 10.0, "U");

        let stats = manager.calculate_stats(&values, &spec);

        // Mean = 40 / 8 = 5.0
        assert!((stats.mean - 5.0).abs() < 1e-10);
        // Variance = (9+1+1+1+0+0+4+16)/7 = 32/7 = 4.5714...
        // StdDev = sqrt(4.5714) = 2.138
        assert!((stats.std_dev - 2.1380899).abs() < 1e-6);
        assert_eq!(stats.min, 2.0);
        assert_eq!(stats.max, 9.0);
        assert_eq!(stats.median, 4.5);
    }

    #[test]
    fn test_yield_analysis_workflow() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::range("gain", 45.0, 55.0, "dB"));

        let mut results = Vec::new();
        for val in vec![44.0, 46.0, 48.0, 50.0, 52.0, 54.0, 56.0] {
            let mut measurements = HashMap::new();
            measurements.insert("gain".to_string(), val);

            let res = SimulationResult::Empty { measurements };
            results.push(res);
        }

        let report = manager.analyze(&results);
        let gain_res = report.get("gain").unwrap();

        assert_eq!(gain_res.total_runs, 7);
        assert_eq!(gain_res.pass_count, 5); // 46, 48, 50, 52, 54
        assert_eq!(gain_res.fail_count, 2); // 44, 56
        assert!((gain_res.yield_percent - 71.428).abs() < 0.01);
    }

    #[test]
    fn test_yield_analysis_missing_measurements_count_as_failures() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::lower("gain", 1.0, "V"));

        let results = vec![
            SimulationResult::Empty {
                measurements: HashMap::from([("gain".to_string(), 1.2)]),
            },
            SimulationResult::Empty {
                measurements: HashMap::new(),
            },
            SimulationResult::Empty {
                measurements: HashMap::from([("gain".to_string(), f64::NAN)]),
            },
        ];

        let report = manager.analyze(&results);
        let gain = report.get("gain").expect("gain yield result should exist");
        assert_eq!(gain.total_runs, 3);
        assert_eq!(gain.pass_count, 1);
        assert_eq!(gain.fail_count, 2);
        assert_eq!(gain.trail, vec![true, false, false]);
    }

    #[test]
    fn test_calculate_stats_single_value_is_stable() {
        let manager = YieldAnalysisManager::new();
        let stats = manager.calculate_stats(&[3.3], &YieldSpec::upper("vout", 5.0, "V"));
        assert_eq!(stats.count, 1);
        assert_eq!(stats.mean, 3.3);
        assert_eq!(stats.std_dev, 0.0);
        assert_eq!(stats.skewness, 0.0);
        assert_eq!(stats.kurtosis, 0.0);
    }

    #[test]
    fn test_extract_measurement_from_monte_carlo_summary() {
        let mut manager = YieldAnalysisManager::new();
        manager.add_spec(YieldSpec::range("vout", 0.9, 1.1, "V"));

        let mc = SimulationResult::MonteCarlo {
            runs_requested: 20,
            runs_completed: 20,
            num_failures: 0,
            all_converged: true,
            variables: vec![crate::simulation::results::MonteCarloVariableResult {
                name: "vout".to_string(),
                mean: 1.0,
                std_dev: 0.01,
                min: 0.97,
                max: 1.03,
                histogram: vec![2, 6, 10, 2],
                bin_edges: vec![0.95, 0.98, 1.0, 1.02, 1.05],
            }],
        };

        let report = manager.analyze(&[mc]);
        let gain = report.get("vout").expect("vout result should exist");
        assert_eq!(gain.total_runs, 1);
        assert_eq!(gain.pass_count, 1);
        assert_eq!(gain.fail_count, 0);
        assert_eq!(gain.yield_percent, 100.0);
    }

    #[test]
    fn test_calculate_stats_ignores_non_finite_samples() {
        let manager = YieldAnalysisManager::new();
        let spec = YieldSpec::range("gain", 0.0, 10.0, "dB");
        let stats = manager.calculate_stats(&[1.0, f64::NAN, f64::INFINITY, 3.0], &spec);
        assert_eq!(stats.count, 2);
        assert!((stats.mean - 2.0).abs() < 1e-12);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 3.0);
        assert_eq!(stats.median, 2.0);
    }

    #[test]
    fn test_calculate_stats_returns_default_when_all_samples_non_finite() {
        let manager = YieldAnalysisManager::new();
        let spec = YieldSpec::range("gain", 0.0, 10.0, "dB");
        let stats = manager.calculate_stats(&[f64::NAN, f64::INFINITY], &spec);
        assert_eq!(stats.count, 0);
        assert_eq!(stats.mean, 0.0);
        assert!(stats.cp.is_none());
        assert!(stats.cpk.is_none());
    }

    #[test]
    fn test_calculate_stats_invalid_spec_limits_disable_capability_indices() {
        let manager = YieldAnalysisManager::new();
        let invalid_spec = YieldSpec {
            target: "gain".to_string(),
            limit_type: SpecLimitType::Range,
            min: Some(5.0),
            max: Some(5.0),
            target_val: Some(5.0),
            unit: "dB".to_string(),
            weight: 1.0,
        };
        let stats = manager.calculate_stats(&[4.0, 5.0, 6.0], &invalid_spec);
        assert!(stats.cp.is_none());
        assert!(stats.cpk.is_none());
    }
}
