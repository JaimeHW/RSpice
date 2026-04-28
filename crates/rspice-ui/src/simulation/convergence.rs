//! Convergence Aids Dialog
//!
//! UI and configuration for simulation convergence options.
//! Matches Cadence Spectre's convergence controls.
//!
//! # Features
//!
//! - GMIN stepping configuration
//! - Source stepping options
//! - Tolerance settings (reltol, abstol, vntol)
//! - IC and nodeset management
//! - Algorithm selection

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Convergence Algorithm
// =============================================================================

/// Convergence algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ConvergenceAlgorithm {
    /// Standard Newton-Raphson
    #[default]
    Newton,
    /// Newton-Raphson with GMIN stepping
    NewtonGmin,
    /// Newton-Raphson with source stepping
    NewtonSource,
    /// Pseudo-transient continuation
    PseudoTransient,
    /// Damped Newton
    DampedNewton,
}

impl ConvergenceAlgorithm {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ConvergenceAlgorithm::Newton => "Standard Newton-Raphson",
            ConvergenceAlgorithm::NewtonGmin => "Newton with GMIN Stepping",
            ConvergenceAlgorithm::NewtonSource => "Newton with Source Stepping",
            ConvergenceAlgorithm::PseudoTransient => "Pseudo-Transient",
            ConvergenceAlgorithm::DampedNewton => "Damped Newton",
        }
    }

    /// Description of algorithm
    pub fn description(&self) -> &'static str {
        match self {
            ConvergenceAlgorithm::Newton => {
                "Standard Newton-Raphson iteration. Fastest when it works."
            }
            ConvergenceAlgorithm::NewtonGmin => {
                "Adds shunt conductances to ground, gradually reducing them."
            }
            ConvergenceAlgorithm::NewtonSource => {
                "Starts with reduced source strengths, ramping to full value."
            }
            ConvergenceAlgorithm::PseudoTransient => {
                "Adds artificial capacitors to find DC solution via transient."
            }
            ConvergenceAlgorithm::DampedNewton => "Limits update step size to prevent divergence.",
        }
    }

    /// All available algorithms
    pub const ALL: [ConvergenceAlgorithm; 5] = [
        ConvergenceAlgorithm::Newton,
        ConvergenceAlgorithm::NewtonGmin,
        ConvergenceAlgorithm::NewtonSource,
        ConvergenceAlgorithm::PseudoTransient,
        ConvergenceAlgorithm::DampedNewton,
    ];
}

// =============================================================================
// Tolerance Settings
// =============================================================================

/// Tolerance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToleranceSettings {
    /// Relative tolerance for voltages/currents
    pub reltol: f64,
    /// Absolute voltage tolerance (V)
    pub vntol: f64,
    /// Absolute current tolerance (A)
    pub abstol: f64,
    /// Charge tolerance (C)
    pub chgtol: f64,
    /// Pivot tolerance for matrix solving
    pub pivtol: f64,
    /// Pivot relative tolerance
    pub pivrel: f64,
    /// Truncation error tolerance (for transient)
    pub trtol: f64,
}

impl Default for ToleranceSettings {
    fn default() -> Self {
        // Standard SPICE defaults matching Spectre
        Self {
            reltol: 1e-3,
            vntol: 1e-6,
            abstol: 1e-12,
            chgtol: 1e-14,
            pivtol: 1e-13,
            pivrel: 1e-3,
            trtol: 7.0,
        }
    }
}

impl ToleranceSettings {
    /// Tighten tolerances for higher accuracy
    pub fn tight() -> Self {
        Self {
            reltol: 1e-5,
            vntol: 1e-9,
            abstol: 1e-15,
            chgtol: 1e-17,
            ..Default::default()
        }
    }

    /// Loosen tolerances for faster (less accurate) simulation
    pub fn loose() -> Self {
        Self {
            reltol: 1e-2,
            vntol: 1e-4,
            abstol: 1e-10,
            chgtol: 1e-12,
            ..Default::default()
        }
    }

    /// Apply a multiplier to all tolerances
    pub fn scale_by(&mut self, factor: f64) {
        self.reltol *= factor;
        self.vntol *= factor;
        self.abstol *= factor;
        self.chgtol *= factor;
    }
}

// =============================================================================
// GMIN Stepping Configuration
// =============================================================================

/// GMIN stepping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GminConfig {
    /// Initial GMIN value (S)
    pub gmin_initial: f64,
    /// Final GMIN value (S)
    pub gmin_final: f64,
    /// Reduction factor per step
    pub reduction_factor: f64,
    /// Maximum steps
    pub max_steps: usize,
    /// Whether GMIN stepping is enabled
    pub enabled: bool,
}

impl Default for GminConfig {
    fn default() -> Self {
        Self {
            gmin_initial: 1e-3,
            gmin_final: 1e-12,
            reduction_factor: 10.0,
            max_steps: 20,
            enabled: true,
        }
    }
}

impl GminConfig {
    /// Calculate number of steps needed
    pub fn steps_needed(&self) -> usize {
        if self.gmin_initial <= self.gmin_final {
            return 0;
        }
        let ratio = self.gmin_initial / self.gmin_final;
        let steps = (ratio.log10() / self.reduction_factor.log10()).ceil() as usize;
        steps.min(self.max_steps)
    }

    /// Get GMIN value at step
    pub fn gmin_at_step(&self, step: usize) -> f64 {
        let factor = self.reduction_factor.powi(step as i32);
        (self.gmin_initial / factor).max(self.gmin_final)
    }
}

// =============================================================================
// Source Stepping Configuration
// =============================================================================

/// Source stepping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSteppingConfig {
    /// Starting fraction (0-1)
    pub start_fraction: f64,
    /// Increment per step
    pub step_size: f64,
    /// Maximum steps
    pub max_steps: usize,
    /// Whether source stepping is enabled
    pub enabled: bool,
}

impl Default for SourceSteppingConfig {
    fn default() -> Self {
        Self {
            start_fraction: 0.0,
            step_size: 0.1,
            max_steps: 20,
            enabled: true,
        }
    }
}

impl SourceSteppingConfig {
    /// Get source fraction at step
    pub fn fraction_at_step(&self, step: usize) -> f64 {
        let frac = self.start_fraction + (step as f64 * self.step_size);
        frac.min(1.0)
    }

    /// Calculate steps to reach full strength
    pub fn steps_to_full(&self) -> usize {
        let remaining = 1.0 - self.start_fraction;
        (remaining / self.step_size).ceil() as usize
    }
}

// =============================================================================
// Initial Condition
// =============================================================================

/// Type of initial condition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IcType {
    /// Node voltage IC
    Ic,
    /// Node set (hint, not forced)
    Nodeset,
}

/// A single initial condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialCondition {
    /// Node name
    pub node: String,
    /// Value
    pub value: f64,
    /// IC type
    pub ic_type: IcType,
    /// Whether this IC is enabled
    pub enabled: bool,
}

impl InitialCondition {
    /// Create a new IC
    pub fn new(node: impl Into<String>, value: f64, ic_type: IcType) -> Self {
        Self {
            node: node.into(),
            value,
            ic_type,
            enabled: true,
        }
    }

    /// Create a nodeset
    pub fn nodeset(node: impl Into<String>, value: f64) -> Self {
        Self::new(node, value, IcType::Nodeset)
    }

    /// Create an IC
    pub fn ic(node: impl Into<String>, value: f64) -> Self {
        Self::new(node, value, IcType::Ic)
    }
}

// =============================================================================
// Convergence Options
// =============================================================================

/// Complete convergence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceOptions {
    /// Primary algorithm
    pub algorithm: ConvergenceAlgorithm,
    /// Fallback algorithm chain
    pub fallback_algorithms: Vec<ConvergenceAlgorithm>,
    /// Tolerance settings
    pub tolerances: ToleranceSettings,
    /// GMIN stepping config
    pub gmin: GminConfig,
    /// Source stepping config
    pub source_stepping: SourceSteppingConfig,
    /// Maximum Newton iterations per step
    pub max_iterations: usize,
    /// Initial conditions
    pub initial_conditions: Vec<InitialCondition>,
    /// Operating temperature (C)
    pub temperature: f64,
    /// Nominal temperature (C)
    pub tnom: f64,
    /// Whether to save internal states
    pub save_all_currents: bool,
    /// Custom options
    pub custom_options: HashMap<String, String>,
}

impl Default for ConvergenceOptions {
    fn default() -> Self {
        Self {
            algorithm: ConvergenceAlgorithm::Newton,
            fallback_algorithms: vec![
                ConvergenceAlgorithm::NewtonGmin,
                ConvergenceAlgorithm::NewtonSource,
                ConvergenceAlgorithm::PseudoTransient,
            ],
            tolerances: ToleranceSettings::default(),
            gmin: GminConfig::default(),
            source_stepping: SourceSteppingConfig::default(),
            max_iterations: 200,
            initial_conditions: Vec::new(),
            temperature: 27.0,
            tnom: 27.0,
            save_all_currents: false,
            custom_options: HashMap::new(),
        }
    }
}

impl ConvergenceOptions {
    /// Create with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create for high-precision simulation
    pub fn high_precision() -> Self {
        Self {
            tolerances: ToleranceSettings::tight(),
            max_iterations: 500,
            ..Default::default()
        }
    }

    /// Create for fast simulation
    pub fn fast() -> Self {
        Self {
            tolerances: ToleranceSettings::loose(),
            max_iterations: 100,
            ..Default::default()
        }
    }

    /// Add an initial condition
    pub fn add_ic(&mut self, ic: InitialCondition) {
        self.initial_conditions.push(ic);
    }

    /// Add a nodeset
    pub fn add_nodeset(&mut self, node: impl Into<String>, value: f64) {
        self.initial_conditions
            .push(InitialCondition::nodeset(node, value));
    }

    /// Set a custom option
    pub fn set_option(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.custom_options.insert(key.into(), value.into());
    }

    /// Get active initial conditions
    pub fn active_ics(&self) -> Vec<&InitialCondition> {
        self.initial_conditions
            .iter()
            .filter(|ic| ic.enabled)
            .collect()
    }

    /// Generate SPICE options string
    pub fn to_spice_options(&self) -> String {
        let mut lines = Vec::new();

        lines.push(format!(".OPTIONS RELTOL={:.2e}", self.tolerances.reltol));
        lines.push(format!(".OPTIONS ABSTOL={:.2e}", self.tolerances.abstol));
        lines.push(format!(".OPTIONS VNTOL={:.2e}", self.tolerances.vntol));
        lines.push(format!(".OPTIONS CHGTOL={:.2e}", self.tolerances.chgtol));
        lines.push(format!(".OPTIONS ITL1={}", self.max_iterations));
        lines.push(format!(".OPTIONS TEMP={}", self.temperature));
        lines.push(format!(".OPTIONS TNOM={}", self.tnom));

        if self.gmin.enabled {
            lines.push(format!(".OPTIONS GMIN={:.2e}", self.gmin.gmin_final));
        }

        // Initial conditions
        for ic in self.active_ics() {
            match ic.ic_type {
                IcType::Ic => lines.push(format!(".IC V({})={}", ic.node, ic.value)),
                IcType::Nodeset => lines.push(format!(".NODESET V({})={}", ic.node, ic.value)),
            }
        }

        lines.join("\n")
    }
}

// =============================================================================
// Tests
// =============================================================================
