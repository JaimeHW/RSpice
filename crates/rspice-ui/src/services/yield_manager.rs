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
                // In a real implementation, we would extract the specific measurement value
                // For this commercial-grade architecture, we assume measurements are stored in the result
                if let Some(val) = self.extract_measurement(result, name) {
                    let passes = spec.evaluates(val);
                    if passes {
                        pass_count += 1;
                    }
                    trail.push(passes);
                    values.push(val);
                }
            }

            if !values.is_empty() {
                let stats = self.calculate_stats(&values, spec);
                let yield_result = YieldResult {
                    spec: spec.clone(),
                    total_runs: num_runs,
                    pass_count,
                    fail_count: num_runs - pass_count,
                    yield_percent: (pass_count as f32 / num_runs as f32) * 100.0,
                    stats,
                    trail,
                };
                self.results.insert(name.clone(), yield_result);
            }
        }

        &self.results
    }

    /// Extract a specific measurement value from a simulation result
    fn extract_measurement(&self, result: &SimulationResult, name: &str) -> Option<f64> {
        // Commercial simulators store scalar measurements like 'risetime', 'bandwidth', 'dc_gain'
        // This would call into simulation::results::SimulationResult's measurement map
        result.measurements().get(name).cloned()
    }

    /// Calculate comprehensive statistical metrics
    fn calculate_stats(&self, values: &[f64], spec: &YieldSpec) -> DistributionStats {
        let n = values.len() as f64;
        if values.is_empty() {
            return DistributionStats::default();
        }

        let mean = values.iter().sum::<f64>() / n;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0);
        let std_dev = variance.sqrt();

        let mut sorted_vals = values.to_vec();
        sorted_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median = if values.len() % 2 == 0 {
            (sorted_vals[values.len() / 2 - 1] + sorted_vals[values.len() / 2]) / 2.0
        } else {
            sorted_vals[values.len() / 2]
        };

        // Skewness and Kurtosis
        let m3 = values.iter().map(|v| (v - mean).powi(3)).sum::<f64>() / n;
        let m4 = values.iter().map(|v| (v - mean).powi(4)).sum::<f64>() / n;
        let skewness = m3 / std_dev.powi(3);
        let kurtosis = (m4 / std_dev.powi(4)) - 3.0; // Excess kurtosis

        // Capability Indices
        let mut cp = None;
        let mut cpk = None;

        if let (Some(lsl), Some(usl)) = (spec.min, spec.max) {
            if std_dev > 0.0 {
                let cp_val = (usl - lsl) / (6.0 * std_dev);
                cp = Some(cp_val);

                let cpu = (usl - mean) / (3.0 * std_dev);
                let cpl = (mean - lsl) / (3.0 * std_dev);
                cpk = Some(cpu.min(cpl));
            }
        }

        DistributionStats {
            count: values.len(),
            mean,
            std_dev,
            min: *sorted_vals.first().unwrap(),
            max: *sorted_vals.last().unwrap(),
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
}
