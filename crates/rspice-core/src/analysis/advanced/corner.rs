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
                .filter(|(_, value)| value.is_finite())
                .collect();

            if values.is_empty() {
                continue;
            }

            let min_entry = values.iter().min_by(|a, b| a.1.total_cmp(&b.1)).unwrap();
            let max_entry = values.iter().max_by(|a, b| a.1.total_cmp(&b.1)).unwrap();

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
                .filter_map(|result| {
                    result
                        .outputs
                        .get(name)
                        .copied()
                        .filter(|value| value.is_finite())
                        .map(|value| (result, value))
                })
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(result, _)| result)
        } else {
            self.corners
                .iter()
                .filter_map(|result| {
                    result
                        .outputs
                        .get(name)
                        .copied()
                        .filter(|value| value.is_finite())
                        .map(|value| (result, value))
                })
                .max_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(result, _)| result)
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

