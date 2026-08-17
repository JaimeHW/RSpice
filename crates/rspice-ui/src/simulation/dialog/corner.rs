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

use crate::product::ProcessCorner;
use crate::simulation::run_set::RunSetState;

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

// =============================================================================
// Dialog State
// =============================================================================

/// The corner analysis's authored state.
///
/// The run space itself is one owner — [`RunSetState`] — rather than a fixed
/// process/min-nom-max/cold-room-hot triple. The engine has always accepted
/// arbitrary-length process, supply and temperature axes; the fixed buffers
/// were a UI restriction, and a run set that cannot say "four supplies" is a
/// run set that cannot express what the executor already runs.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(from = "CornerDialogStateRepr")]
pub struct CornerDialogState {
    /// The declared run space.
    pub run_set: RunSetState,
    /// Base analysis type index.
    pub base_analysis_idx: usize,
    /// Initialized flag.
    #[serde(skip)]
    pub initialized: bool,
}

/// Wire form of [`CornerDialogState`].
///
/// Projects authored before the run set stored the space as three fixed
/// buffers. Those keys are still read here and migrated exactly once, so an
/// existing plan opens on the space it was saved with instead of silently
/// reverting to the default PVT matrix. They are never written back.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CornerDialogStateRepr {
    #[serde(default)]
    run_set: Option<RunSetState>,
    #[serde(default)]
    base_analysis_idx: usize,
    #[serde(default)]
    process_tt: bool,
    #[serde(default)]
    process_ss: bool,
    #[serde(default)]
    process_ff: bool,
    #[serde(default)]
    process_sf: bool,
    #[serde(default)]
    process_fs: bool,
    #[serde(default)]
    voltage_min: String,
    #[serde(default)]
    voltage_nom: String,
    #[serde(default)]
    voltage_max: String,
    #[serde(default)]
    enable_voltage_sweep: bool,
    #[serde(default)]
    temp_cold: String,
    #[serde(default)]
    temp_room: String,
    #[serde(default)]
    temp_hot: String,
    #[serde(default)]
    enable_temp_sweep: bool,
    #[serde(default)]
    full_matrix: bool,
}

impl From<CornerDialogStateRepr> for CornerDialogState {
    fn from(mut repr: CornerDialogStateRepr) -> Self {
        let run_set = repr
            .run_set
            .take()
            .unwrap_or_else(|| migrate_legacy_space(&repr));
        Self {
            run_set,
            base_analysis_idx: repr.base_analysis_idx,
            initialized: true,
        }
    }
}

/// Rebuild the declared space from the pre-run-set buffers.
///
/// A value that no longer parses is carried across as written rather than
/// dropped: validation will refuse it by name, which is what a user needs to
/// see, and losing it would quietly shrink the space the plan was saved with.
fn migrate_legacy_space(repr: &CornerDialogStateRepr) -> RunSetState {
    use crate::simulation::run_set::{RunSetDimension, RunSetDimensionKind};

    let sections: Vec<&str> = [
        (repr.process_tt, "TT"),
        (repr.process_ss, "SS"),
        (repr.process_ff, "FF"),
        (repr.process_sf, "SF"),
        (repr.process_fs, "FS"),
    ]
    .into_iter()
    .filter_map(|(selected, name)| selected.then_some(name))
    .collect();
    if sections.is_empty() && repr.voltage_nom.is_empty() && repr.temp_room.is_empty() {
        // Nothing was authored: this is a fresh state, not a saved space.
        return RunSetState::default();
    }

    let supplies: Vec<&str> = if repr.enable_voltage_sweep {
        vec![
            repr.voltage_min.as_str(),
            repr.voltage_nom.as_str(),
            repr.voltage_max.as_str(),
        ]
    } else {
        vec![repr.voltage_nom.as_str()]
    };
    let temperatures: Vec<&str> = if repr.enable_temp_sweep {
        vec![
            repr.temp_cold.as_str(),
            repr.temp_room.as_str(),
            repr.temp_hot.as_str(),
        ]
    } else {
        vec![repr.temp_room.as_str()]
    };

    let mut supply = RunSetDimension::new(
        "dimension-supply",
        RunSetDimensionKind::Supply,
        &supplies,
        1,
    );
    supply.enabled = repr.enable_voltage_sweep;

    let mut state = RunSetState::default();
    state.composition.mode = if repr.full_matrix {
        crate::simulation::run_set::RunSetCompositionMode::Cartesian
    } else {
        crate::simulation::run_set::RunSetCompositionMode::Zipped
    };
    state.dimensions = vec![
        RunSetDimension::new(
            "dimension-process",
            RunSetDimensionKind::ProcessSection,
            &sections,
            1,
        ),
        supply,
        RunSetDimension::new(
            "dimension-temperature",
            RunSetDimensionKind::Temperature,
            &temperatures,
            1,
        ),
    ];
    state
}

impl CornerDialogState {
    /// Initialize from config.
    pub fn from_config(config: &CornerConfig) -> Self {
        Self {
            run_set: RunSetState::from_corner_config(config),
            base_analysis_idx: match config.base_analysis {
                CornerBaseAnalysis::Transient => 0,
                CornerBaseAnalysis::Ac => 1,
                CornerBaseAnalysis::Dc => 2,
                CornerBaseAnalysis::Op => 3,
            },
            initialized: true,
        }
    }

    /// The base analysis executed at every point.
    pub fn base_analysis(&self) -> CornerBaseAnalysis {
        match self.base_analysis_idx {
            0 => CornerBaseAnalysis::Transient,
            1 => CornerBaseAnalysis::Ac,
            2 => CornerBaseAnalysis::Dc,
            _ => CornerBaseAnalysis::Op,
        }
    }

    /// Convert to config against the plan's nominal point.
    ///
    /// The reference is passed in rather than stored: an axis the run set does
    /// not declare resolves to the plan's own value for that quantity, and the
    /// plan owns it.
    pub fn to_config(
        &self,
        reference: crate::simulation::run_set::ReferencePoint,
    ) -> Result<CornerConfig, String> {
        self.run_set
            .to_corner_config(self.base_analysis(), reference)
    }

    /// Ensure initialized.
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&CornerConfig::commercial_pvt());
        }
    }
}

// =============================================================================
// Tests
// =============================================================================
