//! AC Small-Signal Analysis

use crate::Complex64;
use crate::Value;
use crate::analysis::AnalysisConfig;

/// AC Analysis engine
#[derive(Debug)]
pub struct AcAnalysis {
    config: AnalysisConfig,
    /// Frequency points to analyze
    frequencies: Vec<Value>,
}

impl AcAnalysis {
    pub fn new(config: AnalysisConfig) -> Self {
        Self {
            config,
            frequencies: Vec::new(),
        }
    }

    /// Get config
    pub fn config(&self) -> &AnalysisConfig {
        &self.config
    }

    /// Set up linear frequency sweep
    pub fn linear_sweep(&mut self, start: Value, stop: Value, points: usize) {
        self.frequencies = (0..points)
            .map(|i| start + (stop - start) * (i as f64) / ((points - 1) as f64))
            .collect();
    }

    /// Set up decade frequency sweep
    pub fn decade_sweep(&mut self, start: Value, stop: Value, points_per_decade: usize) {
        let start_log = start.log10();
        let stop_log = stop.log10();
        let num_decades = stop_log - start_log;
        let total_points = (num_decades * points_per_decade as f64).ceil() as usize;

        self.frequencies = (0..=total_points)
            .map(|i| {
                let log_f = start_log + (stop_log - start_log) * (i as f64) / (total_points as f64);
                10.0_f64.powf(log_f)
            })
            .collect();
    }

    /// Set up octave frequency sweep
    pub fn octave_sweep(&mut self, start: Value, stop: Value, points_per_octave: usize) {
        let start_log2 = start.log2();
        let stop_log2 = stop.log2();
        let num_octaves = stop_log2 - start_log2;
        let total_points = (num_octaves * points_per_octave as f64).ceil() as usize;

        self.frequencies = (0..=total_points)
            .map(|i| {
                let log_f =
                    start_log2 + (stop_log2 - start_log2) * (i as f64) / (total_points as f64);
                2.0_f64.powf(log_f)
            })
            .collect();
    }

    /// Get frequency points
    pub fn frequencies(&self) -> &[Value] {
        &self.frequencies
    }
}

impl Default for AcAnalysis {
    fn default() -> Self {
        Self::new(AnalysisConfig::default())
    }
}

/// AC analysis result at a single frequency
#[derive(Debug, Clone)]
pub struct AcResult {
    /// Frequency
    pub frequency: Value,
    /// Stable node names aligned with `voltages`
    pub node_names: Vec<String>,
    /// Stable branch names aligned with `currents`
    pub branch_names: Vec<String>,
    /// Complex node voltages
    pub voltages: Vec<Complex64>,
    /// Complex branch currents  
    pub currents: Vec<Complex64>,
}

impl AcResult {
    /// Get voltage magnitude at a node (1-indexed, consistent with SPICE)
    pub fn voltage_magnitude(&self, node: usize) -> Value {
        if node == 0 {
            return 0.0; // Ground is always 0V
        }
        self.voltages.get(node - 1).map(|v| v.norm()).unwrap_or(0.0)
    }

    /// Get voltage phase at a node (in radians, 1-indexed)
    pub fn voltage_phase(&self, node: usize) -> Value {
        if node == 0 {
            return 0.0; // Ground phase is 0
        }
        self.voltages.get(node - 1).map(|v| v.arg()).unwrap_or(0.0)
    }

    /// Get voltage in dB at a node
    pub fn voltage_db(&self, node: usize) -> Value {
        20.0 * self.voltage_magnitude(node).log10()
    }
}
