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

// =============================================================================
// Tests
// =============================================================================
