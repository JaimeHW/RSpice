//! Corner Analysis for Process, Voltage, Temperature (PVT) Simulation
//!
//! Provides commercial-grade corner analysis supporting:
//! - Standard process corners (TT, SS, SF, FS, FF)
//! - Voltage corners (Min, Nom, Max)
//! - Temperature corners (Cold, Room, Hot)
//! - Custom user-defined corners
//! - Corner sweep combinations
//!
//! # Example
//!
//! ```ignore
//! let config = CornerConfig::new()
//!     .with_process_corners(vec![ProcessCorner::TT, ProcessCorner::SS, ProcessCorner::FF])
//!     .with_voltages(vec![0.9, 1.0, 1.1])
//!     .with_temperatures(vec![-40.0, 25.0, 125.0]);
//!
//! let runner = CornerRunner::new(config);
//! let results = runner.run(|corner| simulate(corner));
//! ```

use crate::Value;
use std::collections::HashMap;
use std::fmt;

//=============================================================================
// Process Corner Definitions
//=============================================================================

/// Standard process corner types following industry convention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessCorner {
    /// Typical-Typical: nominal NMOS and PMOS
    TT,
    /// Slow-Slow: slow NMOS, slow PMOS (worst-case delay)
    SS,
    /// Fast-Fast: fast NMOS, fast PMOS (worst-case power)
    FF,
    /// Slow-Fast: slow NMOS, fast PMOS (skewed)
    SF,
    /// Fast-Slow: fast NMOS, slow PMOS (skewed)
    FS,
    /// Custom corner with name
    Custom(u8),
}

impl ProcessCorner {
    /// Get the standard 5 corners
    pub fn standard_five() -> Vec<Self> {
        vec![Self::TT, Self::SS, Self::FF, Self::SF, Self::FS]
    }

    /// Get typical corner only
    pub fn typical() -> Vec<Self> {
        vec![Self::TT]
    }

    /// Get worst-case speed corners (SS, TT, FF)
    pub fn speed_corners() -> Vec<Self> {
        vec![Self::SS, Self::TT, Self::FF]
    }

    /// Get NMOS speed factor (1.0 = nominal)
    pub fn nmos_factor(&self) -> Value {
        match self {
            Self::TT => 1.0,
            Self::SS | Self::SF => 0.9, // Slow NMOS
            Self::FF | Self::FS => 1.1, // Fast NMOS
            Self::Custom(_) => 1.0,
        }
    }

    /// Get PMOS speed factor (1.0 = nominal)
    pub fn pmos_factor(&self) -> Value {
        match self {
            Self::TT => 1.0,
            Self::SS | Self::FS => 0.9, // Slow PMOS
            Self::FF | Self::SF => 1.1, // Fast PMOS
            Self::Custom(_) => 1.0,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::TT => "TT",
            Self::SS => "SS",
            Self::FF => "FF",
            Self::SF => "SF",
            Self::FS => "FS",
            Self::Custom(_) => "Custom",
        }
    }
}

impl fmt::Display for ProcessCorner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

//=============================================================================
// Corner Point
//=============================================================================

/// A single PVT corner point
#[derive(Debug, Clone)]
pub struct CornerPoint {
    /// Process corner
    pub process: ProcessCorner,
    /// Supply voltage (V)
    pub voltage: Value,
    /// Temperature (°C or K)
    pub temperature: Value,
    /// Temperature unit (true = Kelvin, false = Celsius)
    pub temp_in_kelvin: bool,
    /// Custom parameter overrides
    pub params: HashMap<String, Value>,
}

impl CornerPoint {
    /// Create new corner point
    pub fn new(process: ProcessCorner, voltage: Value, temperature: Value) -> Self {
        Self {
            process,
            voltage,
            temperature,
            temp_in_kelvin: false,
            params: HashMap::new(),
        }
    }

    /// Get temperature in Kelvin
    pub fn temp_k(&self) -> Value {
        if self.temp_in_kelvin {
            self.temperature
        } else {
            self.temperature + 273.15
        }
    }

    /// Get temperature in Celsius
    pub fn temp_c(&self) -> Value {
        if self.temp_in_kelvin {
            self.temperature - 273.15
        } else {
            self.temperature
        }
    }

    /// Set a custom parameter
    pub fn with_param(mut self, name: &str, value: Value) -> Self {
        self.params.insert(name.to_string(), value);
        self
    }

    /// Get corner name (e.g., "SS_0.9V_-40C")
    pub fn name(&self) -> String {
        format!(
            "{}_{:.2}V_{:.0}C",
            self.process,
            self.voltage,
            self.temp_c()
        )
    }
}

impl Default for CornerPoint {
    fn default() -> Self {
        Self::new(ProcessCorner::TT, 1.0, 25.0)
    }
}

//=============================================================================
// Corner Configuration
//=============================================================================

/// Configuration for corner analysis
#[derive(Debug, Clone)]
pub struct CornerConfig {
    /// Process corners to simulate
    pub process_corners: Vec<ProcessCorner>,
    /// Supply voltages to sweep
    pub voltages: Vec<Value>,
    /// Temperatures to sweep (in Celsius by default)
    pub temperatures: Vec<Value>,
    /// Use Kelvin for temperatures
    pub temp_in_kelvin: bool,
    /// Custom corners (overrides PVT sweep)
    pub custom_corners: Vec<CornerPoint>,
    /// Run all combinations or just matched corners
    pub full_matrix: bool,
}

impl CornerConfig {
    /// Create new configuration with TT corner
    pub fn new() -> Self {
        Self {
            process_corners: vec![ProcessCorner::TT],
            voltages: vec![1.0],
            temperatures: vec![25.0],
            temp_in_kelvin: false,
            custom_corners: Vec::new(),
            full_matrix: true,
        }
    }

    /// Use standard 5 process corners
    pub fn with_standard_corners(mut self) -> Self {
        self.process_corners = ProcessCorner::standard_five();
        self
    }

    /// Set process corners
    pub fn with_process_corners(mut self, corners: Vec<ProcessCorner>) -> Self {
        self.process_corners = corners;
        self
    }

    /// Set voltage sweep points
    pub fn with_voltages(mut self, voltages: Vec<Value>) -> Self {
        self.voltages = voltages;
        self
    }

    /// Set temperature sweep points
    pub fn with_temperatures(mut self, temps: Vec<Value>) -> Self {
        self.temperatures = temps;
        self
    }

    /// Use Kelvin for temperature
    pub fn with_kelvin(mut self) -> Self {
        self.temp_in_kelvin = true;
        self
    }

    /// Add custom corner
    pub fn add_corner(mut self, corner: CornerPoint) -> Self {
        self.custom_corners.push(corner);
        self
    }

    /// Set to diagonal sweep (matched corners, not full matrix)
    pub fn diagonal_only(mut self) -> Self {
        self.full_matrix = false;
        self
    }

    /// Generate all corner points based on configuration
    pub fn generate_corners(&self) -> Vec<CornerPoint> {
        // If custom corners defined, use those
        if !self.custom_corners.is_empty() {
            return self.custom_corners.clone();
        }

        let mut corners = Vec::new();

        if self.full_matrix {
            // Full matrix: all combinations
            for &process in &self.process_corners {
                for &voltage in &self.voltages {
                    for &temp in &self.temperatures {
                        corners.push(CornerPoint {
                            process,
                            voltage,
                            temperature: temp,
                            temp_in_kelvin: self.temp_in_kelvin,
                            params: HashMap::new(),
                        });
                    }
                }
            }
        } else {
            // Diagonal: matched indices
            let n = self
                .process_corners
                .len()
                .max(self.voltages.len())
                .max(self.temperatures.len());

            for i in 0..n {
                let process = self
                    .process_corners
                    .get(i % self.process_corners.len())
                    .copied()
                    .unwrap_or(ProcessCorner::TT);
                let voltage = self
                    .voltages
                    .get(i % self.voltages.len())
                    .copied()
                    .unwrap_or(1.0);
                let temp = self
                    .temperatures
                    .get(i % self.temperatures.len())
                    .copied()
                    .unwrap_or(25.0);

                corners.push(CornerPoint {
                    process,
                    voltage,
                    temperature: temp,
                    temp_in_kelvin: self.temp_in_kelvin,
                    params: HashMap::new(),
                });
            }
        }

        corners
    }

    /// Get total number of corners
    pub fn num_corners(&self) -> usize {
        if !self.custom_corners.is_empty() {
            self.custom_corners.len()
        } else if self.full_matrix {
            self.process_corners.len() * self.voltages.len() * self.temperatures.len()
        } else {
            self.process_corners
                .len()
                .max(self.voltages.len())
                .max(self.temperatures.len())
        }
    }
}

impl Default for CornerConfig {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Corner Result
//=============================================================================

/// Result from a single corner simulation
#[derive(Debug, Clone)]
pub struct CornerSimResult {
    /// Corner point that was simulated
    pub corner: CornerPoint,
    /// Output values from simulation
    pub outputs: HashMap<String, Value>,
    /// Whether simulation converged
    pub converged: bool,
}

impl CornerSimResult {
    /// Create new result
    pub fn new(corner: CornerPoint) -> Self {
        Self {
            corner,
            outputs: HashMap::new(),
            converged: true,
        }
    }

    /// Get output value
    pub fn get(&self, name: &str) -> Option<Value> {
        self.outputs.get(name).copied()
    }
}

/// Results from complete corner analysis
#[derive(Debug, Clone)]
pub struct CornerResult {
    /// Results for each corner
    pub corners: Vec<CornerSimResult>,
    /// Summary statistics for each output
    pub summary: HashMap<String, CornerSummary>,
}

/// Summary statistics across corners
#[derive(Debug, Clone)]
pub struct CornerSummary {
    /// Output variable name
    pub name: String,
    /// Minimum value across all corners
    pub min: Value,
    /// Maximum value across all corners
    pub max: Value,
    /// Mean value across all corners
    pub mean: Value,
    /// Corner with minimum value
    pub min_corner: String,
    /// Corner with maximum value
    pub max_corner: String,
    /// Spread (max - min)
    pub spread: Value,
    /// Spread as percentage of mean
    pub spread_pct: Value,
}

impl CornerResult {
    /// Create new result container
    pub fn new() -> Self {
        Self {
            corners: Vec::new(),
            summary: HashMap::new(),
        }
    }

    /// Add a corner result
    pub fn add(&mut self, result: CornerSimResult) {
        self.corners.push(result);
    }

    /// Compute summary statistics
    pub fn compute_summary(&mut self) {
        // Collect all output names
        let mut output_names: Vec<String> = Vec::new();
        for result in &self.corners {
            for name in result.outputs.keys() {
                if !output_names.contains(name) {
                    output_names.push(name.clone());
                }
            }
        }

        // Compute stats for each output
        for name in output_names {
            let values: Vec<(String, Value)> = self
                .corners
                .iter()
                .filter_map(|r| r.outputs.get(&name).map(|&v| (r.corner.name(), v)))
                .collect();

            if values.is_empty() {
                continue;
            }

            let min_entry = values
                .iter()
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();
            let max_entry = values
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            let sum: Value = values.iter().map(|(_, v)| v).sum();
            let mean = sum / values.len() as Value;
            let spread = max_entry.1 - min_entry.1;
            let spread_pct = if mean.abs() > 1e-15 {
                spread / mean.abs() * 100.0
            } else {
                0.0
            };

            self.summary.insert(
                name.clone(),
                CornerSummary {
                    name: name.clone(),
                    min: min_entry.1,
                    max: max_entry.1,
                    mean,
                    min_corner: min_entry.0.clone(),
                    max_corner: max_entry.0.clone(),
                    spread,
                    spread_pct,
                },
            );
        }
    }

    /// Get summary for an output
    pub fn get_summary(&self, name: &str) -> Option<&CornerSummary> {
        self.summary.get(name)
    }

    /// Get worst-case (min or max based on direction) corner for an output
    pub fn worst_case(&self, name: &str, minimize: bool) -> Option<&CornerSimResult> {
        if minimize {
            self.corners
                .iter()
                .filter(|r| r.outputs.contains_key(name))
                .min_by(|a, b| {
                    a.outputs
                        .get(name)
                        .unwrap()
                        .partial_cmp(b.outputs.get(name).unwrap())
                        .unwrap()
                })
        } else {
            self.corners
                .iter()
                .filter(|r| r.outputs.contains_key(name))
                .max_by(|a, b| {
                    a.outputs
                        .get(name)
                        .unwrap()
                        .partial_cmp(b.outputs.get(name).unwrap())
                        .unwrap()
                })
        }
    }
}

impl Default for CornerResult {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Corner Runner
//=============================================================================

/// Runner for corner analysis
pub struct CornerRunner {
    config: CornerConfig,
}

impl CornerRunner {
    /// Create new runner with configuration
    pub fn new(config: CornerConfig) -> Self {
        Self { config }
    }

    /// Run corner analysis
    ///
    /// # Arguments
    /// * `run_simulation` - Closure that runs simulation for a corner point
    ///   Returns `Result<HashMap<String, Value>, E>` with output values
    ///
    /// # Returns
    /// `CornerResult` with all corner results and summary statistics
    pub fn run<F, E>(&self, mut run_simulation: F) -> CornerResult
    where
        F: FnMut(&CornerPoint) -> Result<HashMap<String, Value>, E>,
    {
        let corners = self.config.generate_corners();
        let mut result = CornerResult::new();

        for corner in corners {
            let mut sim_result = CornerSimResult::new(corner.clone());

            match run_simulation(&corner) {
                Ok(outputs) => {
                    sim_result.outputs = outputs;
                    sim_result.converged = true;
                }
                Err(_) => {
                    sim_result.converged = false;
                }
            }

            result.add(sim_result);
        }

        result.compute_summary();
        result
    }

    /// Get number of corners to simulate
    pub fn num_corners(&self) -> usize {
        self.config.num_corners()
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Process Corner Tests
    // =========================================================================

    #[test]
    fn test_process_corner_names() {
        assert_eq!(ProcessCorner::TT.name(), "TT");
        assert_eq!(ProcessCorner::SS.name(), "SS");
        assert_eq!(ProcessCorner::FF.name(), "FF");
        assert_eq!(ProcessCorner::SF.name(), "SF");
        assert_eq!(ProcessCorner::FS.name(), "FS");
    }

    #[test]
    fn test_process_corner_factors() {
        // TT: nominal
        assert_eq!(ProcessCorner::TT.nmos_factor(), 1.0);
        assert_eq!(ProcessCorner::TT.pmos_factor(), 1.0);

        // SS: both slow
        assert!(ProcessCorner::SS.nmos_factor() < 1.0);
        assert!(ProcessCorner::SS.pmos_factor() < 1.0);

        // FF: both fast
        assert!(ProcessCorner::FF.nmos_factor() > 1.0);
        assert!(ProcessCorner::FF.pmos_factor() > 1.0);

        // SF: slow N, fast P
        assert!(ProcessCorner::SF.nmos_factor() < 1.0);
        assert!(ProcessCorner::SF.pmos_factor() > 1.0);

        // FS: fast N, slow P
        assert!(ProcessCorner::FS.nmos_factor() > 1.0);
        assert!(ProcessCorner::FS.pmos_factor() < 1.0);
    }

    #[test]
    fn test_standard_five() {
        let corners = ProcessCorner::standard_five();
        assert_eq!(corners.len(), 5);
        assert!(corners.contains(&ProcessCorner::TT));
        assert!(corners.contains(&ProcessCorner::SS));
        assert!(corners.contains(&ProcessCorner::FF));
        assert!(corners.contains(&ProcessCorner::SF));
        assert!(corners.contains(&ProcessCorner::FS));
    }

    // =========================================================================
    // Corner Point Tests
    // =========================================================================

    #[test]
    fn test_corner_point_creation() {
        let corner = CornerPoint::new(ProcessCorner::SS, 0.9, -40.0);

        assert_eq!(corner.process, ProcessCorner::SS);
        assert_eq!(corner.voltage, 0.9);
        assert_eq!(corner.temperature, -40.0);
        assert!(!corner.temp_in_kelvin);
    }

    #[test]
    fn test_corner_point_temperature_conversion() {
        let corner_c = CornerPoint::new(ProcessCorner::TT, 1.0, 25.0);
        assert!((corner_c.temp_c() - 25.0).abs() < 0.01);
        assert!((corner_c.temp_k() - 298.15).abs() < 0.01);

        let mut corner_k = CornerPoint::new(ProcessCorner::TT, 1.0, 300.0);
        corner_k.temp_in_kelvin = true;
        assert!((corner_k.temp_k() - 300.0).abs() < 0.01);
        assert!((corner_k.temp_c() - 26.85).abs() < 0.01);
    }

    #[test]
    fn test_corner_point_name() {
        let corner = CornerPoint::new(ProcessCorner::SS, 0.9, -40.0);
        let name = corner.name();

        assert!(name.contains("SS"));
        assert!(name.contains("0.90V"));
        assert!(name.contains("-40C"));
    }

    #[test]
    fn test_corner_point_custom_params() {
        let corner = CornerPoint::new(ProcessCorner::TT, 1.0, 25.0)
            .with_param("VTH0", 0.45)
            .with_param("TOX", 2e-9);

        assert_eq!(corner.params.get("VTH0"), Some(&0.45));
        assert_eq!(corner.params.get("TOX"), Some(&2e-9));
    }

    // =========================================================================
    // Corner Configuration Tests
    // =========================================================================

    #[test]
    fn test_config_default() {
        let config = CornerConfig::default();

        assert_eq!(config.process_corners.len(), 1);
        assert_eq!(config.process_corners[0], ProcessCorner::TT);
        assert_eq!(config.voltages.len(), 1);
        assert_eq!(config.temperatures.len(), 1);
    }

    #[test]
    fn test_config_with_standard() {
        let config = CornerConfig::new().with_standard_corners();

        assert_eq!(config.process_corners.len(), 5);
    }

    #[test]
    fn test_config_full_matrix() {
        let config = CornerConfig::new()
            .with_process_corners(vec![
                ProcessCorner::SS,
                ProcessCorner::TT,
                ProcessCorner::FF,
            ])
            .with_voltages(vec![0.9, 1.0, 1.1])
            .with_temperatures(vec![-40.0, 25.0, 125.0]);

        // 3 x 3 x 3 = 27 corners
        assert_eq!(config.num_corners(), 27);

        let corners = config.generate_corners();
        assert_eq!(corners.len(), 27);
    }

    #[test]
    fn test_config_diagonal() {
        let config = CornerConfig::new()
            .with_process_corners(vec![
                ProcessCorner::SS,
                ProcessCorner::TT,
                ProcessCorner::FF,
            ])
            .with_voltages(vec![0.9, 1.0, 1.1])
            .with_temperatures(vec![-40.0, 25.0, 125.0])
            .diagonal_only();

        // Diagonal: max(3, 3, 3) = 3 corners
        assert_eq!(config.num_corners(), 3);

        let corners = config.generate_corners();
        assert_eq!(corners.len(), 3);

        // Check first corner is SS, 0.9V, -40C
        assert_eq!(corners[0].process, ProcessCorner::SS);
        assert_eq!(corners[0].voltage, 0.9);
        assert_eq!(corners[0].temperature, -40.0);
    }

    #[test]
    fn test_config_custom_corners() {
        let config = CornerConfig::new()
            .add_corner(CornerPoint::new(ProcessCorner::SS, 0.85, -55.0))
            .add_corner(CornerPoint::new(ProcessCorner::FF, 1.1, 150.0));

        assert_eq!(config.num_corners(), 2);

        let corners = config.generate_corners();
        assert_eq!(corners.len(), 2);
        assert_eq!(corners[0].voltage, 0.85);
        assert_eq!(corners[1].temperature, 150.0);
    }

    // =========================================================================
    // Corner Runner Tests
    // =========================================================================

    #[test]
    fn test_runner_basic() {
        let config = CornerConfig::new()
            .with_process_corners(vec![
                ProcessCorner::SS,
                ProcessCorner::TT,
                ProcessCorner::FF,
            ])
            .with_voltages(vec![1.0])
            .with_temperatures(vec![25.0]);

        let runner = CornerRunner::new(config);

        let result = runner.run(|corner| {
            let mut outputs = HashMap::new();
            // Simulate: delay = base / (process_factor * voltage)
            let base_delay = 1e-9; // 1ns
            let delay = base_delay / (corner.process.nmos_factor() * corner.voltage);
            outputs.insert("delay".to_string(), delay);
            Ok::<_, ()>(outputs)
        });

        assert_eq!(result.corners.len(), 3);

        // SS should have max delay
        let summary = result.get_summary("delay").unwrap();
        assert!(summary.max_corner.contains("SS"));

        // FF should have min delay
        assert!(summary.min_corner.contains("FF"));
    }

    #[test]
    fn test_runner_with_failures() {
        let config =
            CornerConfig::new().with_process_corners(vec![ProcessCorner::SS, ProcessCorner::TT]);

        let runner = CornerRunner::new(config);

        let result = runner.run(|corner| {
            if corner.process == ProcessCorner::SS {
                Err("simulation failed")
            } else {
                let mut outputs = HashMap::new();
                outputs.insert("vout".to_string(), 0.5);
                Ok(outputs)
            }
        });

        // SS failed, TT succeeded
        assert!(!result.corners[0].converged);
        assert!(result.corners[1].converged);
    }

    #[test]
    fn test_summary_statistics() {
        let config = CornerConfig::new().with_process_corners(vec![
            ProcessCorner::SS,
            ProcessCorner::TT,
            ProcessCorner::FF,
        ]);

        let runner = CornerRunner::new(config);

        let result = runner.run(|corner| {
            let mut outputs = HashMap::new();
            let gain = 100.0 * corner.process.nmos_factor();
            outputs.insert("gain".to_string(), gain);
            Ok::<_, ()>(outputs)
        });

        let summary = result.get_summary("gain").unwrap();

        // SS: 90, TT: 100, FF: 110
        assert!((summary.min - 90.0).abs() < 0.1);
        assert!((summary.max - 110.0).abs() < 0.1);
        assert!((summary.mean - 100.0).abs() < 0.1);
        assert!((summary.spread - 20.0).abs() < 0.1);
        assert!((summary.spread_pct - 20.0).abs() < 0.1);
    }

    #[test]
    fn test_worst_case() {
        let config = CornerConfig::new().with_process_corners(vec![
            ProcessCorner::SS,
            ProcessCorner::TT,
            ProcessCorner::FF,
        ]);

        let runner = CornerRunner::new(config);

        let result = runner.run(|corner| {
            let mut outputs = HashMap::new();
            let delay = 10.0 / corner.process.nmos_factor();
            outputs.insert("delay".to_string(), delay);
            Ok::<_, ()>(outputs)
        });

        // Worst case for delay (maximize)
        let worst_max = result.worst_case("delay", false).unwrap();
        assert_eq!(worst_max.corner.process, ProcessCorner::SS);

        // Best case for delay (minimize)
        let worst_min = result.worst_case("delay", true).unwrap();
        assert_eq!(worst_min.corner.process, ProcessCorner::FF);
    }

    // =========================================================================
    // PVT Sweep Tests
    // =========================================================================

    #[test]
    fn test_pvt_sweep() {
        let config = CornerConfig::new()
            .with_process_corners(vec![ProcessCorner::SS, ProcessCorner::FF])
            .with_voltages(vec![0.9, 1.1])
            .with_temperatures(vec![-40.0, 125.0]);

        // 2 x 2 x 2 = 8 corners
        assert_eq!(config.num_corners(), 8);

        let runner = CornerRunner::new(config);

        let result = runner.run(|corner| {
            let mut outputs = HashMap::new();
            // F = V * T * P
            let freq = corner.voltage * (corner.temp_c() + 100.0) * corner.process.nmos_factor();
            outputs.insert("freq".to_string(), freq);
            Ok::<_, ()>(outputs)
        });

        assert_eq!(result.corners.len(), 8);

        // All should converge
        assert!(result.corners.iter().all(|r| r.converged));
    }

    #[test]
    fn test_temperature_extremes() {
        let config = CornerConfig::new().with_temperatures(vec![-40.0, 25.0, 125.0]);

        let corners = config.generate_corners();

        // Verify temperature conversions
        assert!((corners[0].temp_c() - (-40.0)).abs() < 0.01);
        assert!((corners[0].temp_k() - 233.15).abs() < 0.01);

        assert!((corners[2].temp_c() - 125.0).abs() < 0.01);
        assert!((corners[2].temp_k() - 398.15).abs() < 0.01);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_single_corner() {
        let config = CornerConfig::new();
        let runner = CornerRunner::new(config);

        let result = runner.run(|_| {
            let mut outputs = HashMap::new();
            outputs.insert("vout".to_string(), 1.234);
            Ok::<_, ()>(outputs)
        });

        assert_eq!(result.corners.len(), 1);
        assert_eq!(result.corners[0].get("vout"), Some(1.234));
    }

    #[test]
    fn test_empty_outputs() {
        let config = CornerConfig::new();
        let runner = CornerRunner::new(config);

        let result = runner.run(|_| Ok::<_, ()>(HashMap::new()));

        assert!(result.summary.is_empty());
    }

    #[test]
    fn test_process_corner_display() {
        let corner = ProcessCorner::TT;
        assert_eq!(format!("{}", corner), "TT");
    }
}
