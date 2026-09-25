//! Corner Analysis Configuration
//!
//! Configuration for PVT (Process, Voltage, Temperature) corner analysis.
//! Corner analysis runs simulations across multiple operating conditions
//! to verify design robustness.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Standard process corners (TT, SS, FF, SF, FS)
//! - Voltage corner sweep
//! - Temperature corner sweep
//! - Full matrix or diagonal sweep modes
//! - Summary statistics and worst-case identification
//!
//! # Example Usage
//!
//! Run transient analysis at SS corner with reduced voltage and hot temperature.

use rspice_app_types::product::ProcessCorner;

// =============================================================================
// Base Analysis Type
// =============================================================================

/// Base analysis to run at each corner
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CornerBaseAnalysis {
    /// Transient analysis
    #[default]
    Transient,
    /// AC analysis
    Ac,
    /// DC analysis
    Dc,
    /// Operating point
    Op,
}

// =============================================================================
// Corner Configuration
// =============================================================================

/// One explicitly declared point of a filtered space.
///
/// A filtered run set is not an axis product, so it cannot be carried as one.
/// Every point states all three quantities: an axis the run set left undeclared
/// has already been resolved against the plan's reference by the time a point
/// reaches here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerPointSpec {
    pub process: ProcessCorner,
    pub voltage: f64,
    pub temperature_celsius: f64,
}

/// Corner analysis configuration
#[derive(Debug, Clone)]
pub struct CornerConfig {
    /// Process corners to simulate
    pub process_corners: Vec<ProcessCorner>,
    /// Voltage values to sweep (V)
    pub voltages: Vec<f64>,
    /// Exact independent voltage-source instances that form the swept supply
    /// domain. Empty is valid only while no supply axis is enabled.
    pub supply_source_names: Vec<String>,
    /// Temperature values to sweep (°C)
    ///
    /// Always Celsius. The run configuration, the engine's `TEMP` option and
    /// every result label agree on that one unit, so there is nothing here to
    /// select between.
    pub temperatures: Vec<f64>,
    /// Full matrix (all combinations) or diagonal sweep
    pub full_matrix: bool,
    /// The exact points to run, when the space is not an axis composition.
    ///
    /// Empty means the axes above compose as `full_matrix` says. Non-empty
    /// replaces that composition entirely: it is the list the run set resolved,
    /// and re-deriving it from the axes would re-admit the points a filtered
    /// space removed. The axes stay populated with the distinct values the
    /// points use, because the process axis still selects which model sections
    /// are materialized.
    pub points: Vec<CornerPointSpec>,
    /// Base analysis type
    pub base_analysis: CornerBaseAnalysis,
}

impl Default for CornerConfig {
    fn default() -> Self {
        Self {
            process_corners: vec![ProcessCorner::TT],
            voltages: vec![1.0],
            supply_source_names: Vec::new(),
            temperatures: vec![25.0],
            full_matrix: true,
            points: Vec::new(),
            base_analysis: CornerBaseAnalysis::Transient,
        }
    }
}

impl CornerConfig {
    /// Create typical commercial PVT setup
    pub fn commercial_pvt() -> Self {
        Self {
            process_corners: ProcessCorner::speed_corners(),
            voltages: vec![0.9, 1.0, 1.1],          // ±10%
            temperatures: vec![-40.0, 25.0, 125.0], // Mil-spec range
            full_matrix: true,
            ..Default::default()
        }
    }

    /// Total number of corners
    pub fn num_corners(&self) -> usize {
        if !self.points.is_empty() {
            return self.points.len();
        }
        if self.full_matrix {
            self.process_corners.len() * self.voltages.len() * self.temperatures.len()
        } else {
            self.process_corners
                .len()
                .max(self.voltages.len())
                .max(self.temperatures.len())
        }
    }

    /// Generate corner names
    pub fn corner_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        if !self.points.is_empty() {
            return self
                .points
                .iter()
                .map(|point| {
                    format!(
                        "{}_{:.2}V_{:.0}C",
                        point.process.short_name(),
                        point.voltage,
                        point.temperature_celsius
                    )
                })
                .collect();
        }

        if self.full_matrix {
            for p in &self.process_corners {
                for v in &self.voltages {
                    for t in &self.temperatures {
                        names.push(format!("{}_{:.2}V_{:.0}C", p.short_name(), v, t));
                    }
                }
            }
        } else {
            let n = self.num_corners();
            for i in 0..n {
                let p = self
                    .process_corners
                    .get(i % self.process_corners.len())
                    .copied()
                    .unwrap_or(ProcessCorner::TT);
                let v = self
                    .voltages
                    .get(i % self.voltages.len())
                    .copied()
                    .unwrap_or(1.0);
                let t = self
                    .temperatures
                    .get(i % self.temperatures.len())
                    .copied()
                    .unwrap_or(25.0);
                names.push(format!("{}_{:.2}V_{:.0}C", p.short_name(), v, t));
            }
        }

        names
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.process_corners.is_empty() {
            return Err("At least one process corner required".to_string());
        }

        if self.voltages.is_empty() {
            return Err("At least one voltage value required".to_string());
        }

        for v in &self.voltages {
            if *v <= 0.0 {
                return Err("Voltage values must be positive".to_string());
            }
        }

        let mut distinct_supply_values: Vec<u64> = if self.points.is_empty() {
            self.voltages.iter().map(|value| value.to_bits()).collect()
        } else {
            self.points
                .iter()
                .map(|point| point.voltage.to_bits())
                .collect()
        };
        distinct_supply_values.sort_unstable();
        distinct_supply_values.dedup();
        if distinct_supply_values.len() > 1 && self.supply_source_names.is_empty() {
            return Err(
                "Voltage sweeping requires an explicit netlist supply-source binding".to_owned(),
            );
        }
        let mut seen_supply_sources = std::collections::BTreeSet::new();
        for source in &self.supply_source_names {
            if source.trim().is_empty()
                || source != source.trim()
                || source.chars().any(char::is_control)
                || !seen_supply_sources.insert(source.to_ascii_lowercase())
            {
                return Err(format!(
                    "Supply source binding {source:?} is empty, malformed, or duplicated"
                ));
            }
        }

        if self.temperatures.is_empty() {
            return Err("At least one temperature value required".to_string());
        }

        for t in &self.temperatures {
            if *t < -273.15 {
                return Err("Temperature cannot be below absolute zero".to_string());
            }
        }

        for point in &self.points {
            if !point.voltage.is_finite() || point.voltage <= 0.0 {
                return Err("Explicit corner points require positive voltages".to_string());
            }
            if !point.temperature_celsius.is_finite() || point.temperature_celsius < -273.15 {
                return Err(
                    "Explicit corner point temperature cannot be below absolute zero".to_string(),
                );
            }
            // The process axis is what selects the model section a point is
            // materialized against, so a point naming a process the axis omits
            // would run against whichever section happened to be loaded.
            if !self.process_corners.contains(&point.process) {
                return Err(format!(
                    "Explicit corner point uses process {} which the process axis does not declare",
                    point.process.short_name()
                ));
            }
        }

        Ok(())
    }
}
