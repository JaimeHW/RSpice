//! Simulation Options Dialog
//!
//! Commercial-grade simulation configuration with categories:
//! - Transient Analysis Options (timestep, method, etc.)
//! - AC Analysis Options (interpolation, points per decade)
//! - DC Analysis Options (convergence, source stepping)
//! - Convergence Options (tolerances, iteration limits)
//! - Advanced Options (pivoting, matrix solver, etc.)
//!
//! Based on industry-standard SPICE simulator option sets.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Integration Method
// =============================================================================

/// Integration method for transient analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum IntegrationMethod {
    /// Trapezoidal rule (default, A-stable)
    #[default]
    Trapezoidal,
    /// Backward Euler (L-stable, more damping)
    BackwardEuler,
    /// Gear's method (BDF) order 2
    Gear2,
    /// Gear's method order 3-6 (stiff systems)
    GearVariable,
}

impl IntegrationMethod {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Trapezoidal => "Trapezoidal",
            Self::BackwardEuler => "Backward Euler",
            Self::Gear2 => "Gear (BDF-2)",
            Self::GearVariable => "Gear Variable Order",
        }
    }

    /// Get all methods
    pub fn all() -> &'static [IntegrationMethod] {
        &[
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Gear2,
            IntegrationMethod::GearVariable,
        ]
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Trapezoidal => "Second-order accurate, A-stable. Good general choice.",
            Self::BackwardEuler => "First-order, L-stable. Better for stiff circuits.",
            Self::Gear2 => "BDF order 2. Good stability for moderately stiff systems.",
            Self::GearVariable => "Variable order BDF (2-6). Best for very stiff systems.",
        }
    }
}

// =============================================================================
// Matrix Solver Type
// =============================================================================

/// Matrix solver algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MatrixSolver {
    /// Sparse LU factorization (default)
    #[default]
    SparseLU,
    /// KLU sparse solver
    KLU,
    /// SuperLU solver
    SuperLU,
    /// Iterative GMRES
    GMRES,
    /// Direct dense solver (small circuits only)
    Dense,
}

impl MatrixSolver {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::SparseLU => "Sparse LU",
            Self::KLU => "KLU",
            Self::SuperLU => "SuperLU",
            Self::GMRES => "GMRES (Iterative)",
            Self::Dense => "Dense (Small Circuits)",
        }
    }

    pub fn all() -> &'static [MatrixSolver] {
        &[
            MatrixSolver::SparseLU,
            MatrixSolver::KLU,
            MatrixSolver::SuperLU,
            MatrixSolver::GMRES,
            MatrixSolver::Dense,
        ]
    }
}

// =============================================================================
// Pivot Strategy
// =============================================================================

/// Pivoting strategy for matrix operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PivotStrategy {
    /// No pivoting (fastest, least stable)
    None,
    /// Partial pivoting (default)
    #[default]
    Partial,
    /// Full pivoting (most stable)
    Full,
    /// Threshold pivoting
    Threshold,
}

impl PivotStrategy {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Partial => "Partial",
            Self::Full => "Full",
            Self::Threshold => "Threshold",
        }
    }

    pub fn all() -> &'static [PivotStrategy] {
        &[
            PivotStrategy::None,
            PivotStrategy::Partial,
            PivotStrategy::Full,
            PivotStrategy::Threshold,
        ]
    }
}

// =============================================================================
// Transient Options
// =============================================================================

/// Transient analysis options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientOptions {
    /// Maximum timestep (0 = auto)
    pub tstep_max: f64,

    /// Minimum timestep
    pub tstep_min: f64,

    /// Initial timestep
    pub tstep_initial: f64,

    /// Local truncation error tolerance
    pub lte_rel: f64,

    /// Absolute LTE tolerance
    pub lte_abs: f64,

    /// Integration method
    pub method: IntegrationMethod,

    /// Maximum order for variable-order methods
    pub max_order: u32,

    /// Charge conservation tolerance
    pub chgtol: f64,

    /// Timestep increase factor
    pub tstep_grow: f64,

    /// Timestep decrease factor
    pub tstep_shrink: f64,

    /// Use initial conditions (.IC)
    pub use_ic: bool,

    /// Skip initial DC operating point
    pub skip_dc: bool,

    /// Number of startup cycles to skip for PSS
    pub pss_skip_cycles: u32,
}

impl Default for TransientOptions {
    fn default() -> Self {
        Self {
            tstep_max: 0.0,       // Auto
            tstep_min: 1e-18,     // 1 attosecond
            tstep_initial: 1e-12, // 1 ps
            lte_rel: 1e-3,        // 0.1%
            lte_abs: 1e-12,       // 1 pA/pV
            method: IntegrationMethod::Trapezoidal,
            max_order: 2,
            chgtol: 1e-14,      // Charge tolerance
            tstep_grow: 2.0,    // Double timestep max
            tstep_shrink: 0.25, // Quarter timestep min
            use_ic: false,
            skip_dc: false,
            pss_skip_cycles: 5,
        }
    }
}

impl TransientOptions {
    /// Validate options and return errors
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.tstep_min <= 0.0 {
            errors.push("Minimum timestep must be positive".to_string());
        }
        if self.tstep_initial <= 0.0 {
            errors.push("Initial timestep must be positive".to_string());
        }
        if self.tstep_max > 0.0 && self.tstep_max < self.tstep_min {
            errors.push("Maximum timestep must be >= minimum".to_string());
        }
        if self.lte_rel <= 0.0 || self.lte_rel > 1.0 {
            errors.push("Relative LTE must be in (0, 1]".to_string());
        }
        if self.lte_abs <= 0.0 {
            errors.push("Absolute LTE must be positive".to_string());
        }
        if self.tstep_grow <= 1.0 {
            errors.push("Timestep grow factor must be > 1".to_string());
        }
        if self.tstep_shrink >= 1.0 || self.tstep_shrink <= 0.0 {
            errors.push("Timestep shrink factor must be in (0, 1)".to_string());
        }

        errors
    }
}

// =============================================================================
// AC Analysis Options
// =============================================================================

/// AC analysis frequency scale
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FrequencyScale {
    #[default]
    Decade,
    Octave,
    Linear,
}

impl FrequencyScale {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Decade => "Decade",
            Self::Octave => "Octave",
            Self::Linear => "Linear",
        }
    }

    pub fn all() -> &'static [FrequencyScale] {
        &[
            FrequencyScale::Decade,
            FrequencyScale::Octave,
            FrequencyScale::Linear,
        ]
    }
}

/// AC analysis options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcOptions {
    /// Frequency scale type
    pub scale: FrequencyScale,

    /// Points per decade/octave (for log scale)
    pub points_per_decade: u32,

    /// Total points (for linear scale)
    pub num_points: u32,

    /// Interpolation between points
    pub interpolate: bool,

    /// Phase unwrapping
    pub unwrap_phase: bool,

    /// Group delay calculation
    pub calc_group_delay: bool,
}

impl Default for AcOptions {
    fn default() -> Self {
        Self {
            scale: FrequencyScale::Decade,
            points_per_decade: 10,
            num_points: 100,
            interpolate: true,
            unwrap_phase: true,
            calc_group_delay: false,
        }
    }
}

impl AcOptions {
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.points_per_decade == 0 {
            errors.push("Points per decade must be > 0".to_string());
        }
        if self.num_points == 0 {
            errors.push("Number of points must be > 0".to_string());
        }

        errors
    }
}

// =============================================================================
// DC Analysis Options
// =============================================================================

/// DC convergence aid method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DcConvergenceAid {
    /// No convergence aid
    #[default]
    None,
    /// Source stepping (ramp sources from 0)
    SourceStepping,
    /// Gmin stepping (add parallel conductances)
    GminStepping,
    /// Pseudo-transient (add capacitors)
    PseudoTransient,
    /// Continuation method
    Continuation,
}

impl DcConvergenceAid {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::SourceStepping => "Source Stepping",
            Self::GminStepping => "Gmin Stepping",
            Self::PseudoTransient => "Pseudo-Transient",
            Self::Continuation => "Continuation",
        }
    }

    pub fn all() -> &'static [DcConvergenceAid] {
        &[
            DcConvergenceAid::None,
            DcConvergenceAid::SourceStepping,
            DcConvergenceAid::GminStepping,
            DcConvergenceAid::PseudoTransient,
            DcConvergenceAid::Continuation,
        ]
    }
}

/// DC analysis options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcOptions {
    /// Convergence aid method
    pub convergence_aid: DcConvergenceAid,

    /// Gmin value (minimum conductance for Gmin stepping)
    pub gmin: f64,

    /// Number of source stepping steps
    pub source_steps: u32,

    /// Operating point only (no sweeps)
    pub op_only: bool,

    /// Save operating point information
    pub save_op_info: bool,

    /// Use nodeset values
    pub use_nodeset: bool,

    /// Reltol relaxation for DC
    pub dc_tol_factor: f64,
}

impl Default for DcOptions {
    fn default() -> Self {
        Self {
            convergence_aid: DcConvergenceAid::None,
            gmin: 1e-12, // 1 pS
            source_steps: 10,
            op_only: false,
            save_op_info: true,
            use_nodeset: true,
            dc_tol_factor: 1.0, // No relaxation
        }
    }
}

impl DcOptions {
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.gmin <= 0.0 {
            errors.push("Gmin must be positive".to_string());
        }
        if self.source_steps == 0 {
            errors.push("Source steps must be > 0".to_string());
        }
        if self.dc_tol_factor <= 0.0 {
            errors.push("DC tolerance factor must be positive".to_string());
        }

        errors
    }
}

// =============================================================================
// Convergence Options
// =============================================================================

/// Newton-Raphson convergence options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConvergenceOptions {
    /// Relative tolerance
    pub reltol: f64,

    /// Absolute voltage tolerance
    pub vntol: f64,

    /// Absolute current tolerance
    pub abstol: f64,

    /// Maximum Newton-Raphson iterations
    pub itl1: u32,

    /// DC operating point iteration limit
    pub itl2: u32,

    /// Transient timepoint iteration limit
    pub itl4: u32,

    /// Enable damping (limiting)
    pub damping: bool,

    /// Voltage limiting (junction clamping)
    pub vn_limit: f64,

    /// Node limiting enabled
    pub node_limiting: bool,

    /// Convergence check mode (0=both, 1=voltage, 2=current)
    pub conv_mode: u32,
}

impl Default for ConvergenceOptions {
    fn default() -> Self {
        Self {
            reltol: 1e-3,  // 0.1% relative tolerance
            vntol: 1e-6,   // 1 µV absolute voltage
            abstol: 1e-12, // 1 pA absolute current
            itl1: 100,     // DC iterations
            itl2: 50,      // DC op iterations
            itl4: 10,      // Transient iterations per timepoint
            damping: true,
            vn_limit: 0.5, // 500 mV junction limiting
            node_limiting: true,
            conv_mode: 0, // Check both V and I
        }
    }
}

impl ConvergenceOptions {
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.reltol <= 0.0 || self.reltol > 1.0 {
            errors.push("Reltol must be in (0, 1]".to_string());
        }
        if self.vntol <= 0.0 {
            errors.push("Vntol must be positive".to_string());
        }
        if self.abstol <= 0.0 {
            errors.push("Abstol must be positive".to_string());
        }
        if self.itl1 == 0 {
            errors.push("ITL1 must be > 0".to_string());
        }
        if self.itl4 == 0 {
            errors.push("ITL4 must be > 0".to_string());
        }
        if self.vn_limit <= 0.0 {
            errors.push("VN limit must be positive".to_string());
        }
        if self.conv_mode > 2 {
            errors.push("Convergence mode must be 0, 1, or 2".to_string());
        }

        errors
    }

    /// Get tolerance-relaxed version (for difficult convergence)
    pub fn relaxed(&self) -> Self {
        Self {
            reltol: self.reltol * 10.0,
            vntol: self.vntol * 10.0,
            abstol: self.abstol * 10.0,
            itl1: self.itl1 * 2,
            itl2: self.itl2 * 2,
            itl4: self.itl4 * 2,
            ..self.clone()
        }
    }

    /// Get tight tolerance version
    pub fn tight(&self) -> Self {
        Self {
            reltol: self.reltol / 10.0,
            vntol: self.vntol / 10.0,
            abstol: self.abstol / 10.0,
            ..self.clone()
        }
    }
}

// =============================================================================
// Advanced Options
// =============================================================================

/// Advanced simulation options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedOptions {
    /// Matrix solver type
    pub matrix_solver: MatrixSolver,

    /// Pivoting strategy
    pub pivot_strategy: PivotStrategy,

    /// Pivot threshold
    pub pivot_threshold: f64,

    /// Enable scaling
    pub scaling: bool,

    /// Fill-in reduction ordering
    pub ordering: bool,

    /// Parallel evaluation threads (0 = auto)
    pub threads: u32,

    /// Temperature (Kelvin)
    pub temp: f64,

    /// Nominal temperature for models
    pub tnom: f64,

    /// Enable charge conservation checks
    pub check_charge: bool,

    /// Enable inductor flux conservation
    pub check_flux: bool,

    /// Print iteration statistics
    pub verbose: bool,

    /// Debug node voltages on non-convergence
    pub debug_nonconv: bool,
}

impl Default for AdvancedOptions {
    fn default() -> Self {
        Self {
            matrix_solver: MatrixSolver::SparseLU,
            pivot_strategy: PivotStrategy::Partial,
            pivot_threshold: 1e-3,
            scaling: true,
            ordering: true,
            threads: 0,   // Auto
            temp: 300.15, // 27°C
            tnom: 300.15,
            check_charge: false,
            check_flux: false,
            verbose: false,
            debug_nonconv: false,
        }
    }
}

impl AdvancedOptions {
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.pivot_threshold <= 0.0 || self.pivot_threshold >= 1.0 {
            errors.push("Pivot threshold must be in (0, 1)".to_string());
        }
        if self.temp <= 0.0 {
            errors.push("Temperature must be positive".to_string());
        }
        if self.tnom <= 0.0 {
            errors.push("Nominal temperature must be positive".to_string());
        }

        errors
    }
}

// =============================================================================
// Complete Simulation Options
// =============================================================================

/// Complete simulation options state
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SimulationOptions {
    /// Transient analysis options
    pub transient: TransientOptions,

    /// AC analysis options
    pub ac: AcOptions,

    /// DC analysis options
    pub dc: DcOptions,

    /// Convergence options
    pub convergence: ConvergenceOptions,

    /// Advanced options
    pub advanced: AdvancedOptions,

    /// Custom options (key-value overrides)
    pub custom: HashMap<String, String>,
}

impl SimulationOptions {
    /// Create with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate all options
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        errors.extend(self.transient.validate());
        errors.extend(self.ac.validate());
        errors.extend(self.dc.validate());
        errors.extend(self.convergence.validate());
        errors.extend(self.advanced.validate());
        errors
    }

    /// Check if options are valid
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Generate SPICE .OPTIONS statement
    pub fn to_spice_options(&self) -> String {
        let mut lines = Vec::new();

        // Convergence options
        lines.push(format!(
            ".OPTIONS RELTOL={:.2e} VNTOL={:.2e} ABSTOL={:.2e}",
            self.convergence.reltol, self.convergence.vntol, self.convergence.abstol
        ));
        lines.push(format!(
            ".OPTIONS ITL1={} ITL2={} ITL4={}",
            self.convergence.itl1, self.convergence.itl2, self.convergence.itl4
        ));

        // Transient options
        let method_str = match self.transient.method {
            IntegrationMethod::Trapezoidal => "TRAP".to_string(),
            IntegrationMethod::BackwardEuler => "BE".to_string(),
            IntegrationMethod::Gear2 => "GEAR".to_string(),
            IntegrationMethod::GearVariable => "GEAR".to_string(),
        };
        lines.push(format!(".OPTIONS METHOD={}", method_str));

        if self.transient.tstep_max > 0.0 {
            lines.push(format!(".OPTIONS MAXSTEP={:.2e}", self.transient.tstep_max));
        }

        // Temperature
        if (self.advanced.temp - 300.15).abs() > 0.01 {
            lines.push(format!(".OPTIONS TEMP={:.2}", self.advanced.temp - 273.15));
        }
        if (self.advanced.tnom - 300.15).abs() > 0.01 {
            lines.push(format!(".OPTIONS TNOM={:.2}", self.advanced.tnom - 273.15));
        }

        // DC options
        if self.dc.gmin != 1e-12 {
            lines.push(format!(".OPTIONS GMIN={:.2e}", self.dc.gmin));
        }

        // Custom options
        for (key, value) in &self.custom {
            lines.push(format!(".OPTIONS {}={}", key.to_uppercase(), value));
        }

        lines.join("\n")
    }

    /// Parse from SPICE .OPTIONS
    pub fn parse_spice_option(&mut self, key: &str, value: &str) -> Result<(), String> {
        let key_upper = key.to_uppercase();
        let value_parsed: Result<f64, _> = value.parse();

        match key_upper.as_str() {
            "RELTOL" => {
                self.convergence.reltol = value_parsed.map_err(|_| "Invalid reltol")?;
            }
            "VNTOL" => {
                self.convergence.vntol = value_parsed.map_err(|_| "Invalid vntol")?;
            }
            "ABSTOL" => {
                self.convergence.abstol = value_parsed.map_err(|_| "Invalid abstol")?;
            }
            "ITL1" => {
                self.convergence.itl1 = value.parse().map_err(|_| "Invalid itl1")?;
            }
            "ITL2" => {
                self.convergence.itl2 = value.parse().map_err(|_| "Invalid itl2")?;
            }
            "ITL4" => {
                self.convergence.itl4 = value.parse().map_err(|_| "Invalid itl4")?;
            }
            "GMIN" => {
                self.dc.gmin = value_parsed.map_err(|_| "Invalid gmin")?;
            }
            "TEMP" => {
                let temp_c: f64 = value_parsed.map_err(|_| "Invalid temp")?;
                self.advanced.temp = temp_c + 273.15;
            }
            "TNOM" => {
                let tnom_c: f64 = value_parsed.map_err(|_| "Invalid tnom")?;
                self.advanced.tnom = tnom_c + 273.15;
            }
            "METHOD" => {
                self.transient.method = match value.to_uppercase().as_str() {
                    "TRAP" | "TRAPEZOIDAL" => IntegrationMethod::Trapezoidal,
                    "BE" | "EULER" => IntegrationMethod::BackwardEuler,
                    "GEAR" => IntegrationMethod::Gear2,
                    _ => return Err(format!("Unknown method: {}", value)),
                };
            }
            "MAXSTEP" => {
                self.transient.tstep_max = value_parsed.map_err(|_| "Invalid maxstep")?;
            }
            _ => {
                // Store as custom option
                self.custom.insert(key_upper, value.to_string());
            }
        }

        Ok(())
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Get preset for fast simulation (relaxed tolerances)
    pub fn preset_fast() -> Self {
        Self {
            convergence: ConvergenceOptions {
                reltol: 1e-2,
                vntol: 1e-5,
                abstol: 1e-11,
                itl4: 5,
                ..Default::default()
            },
            transient: TransientOptions {
                lte_rel: 1e-2,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Get preset for accurate simulation (tight tolerances)
    pub fn preset_accurate() -> Self {
        Self {
            convergence: ConvergenceOptions {
                reltol: 1e-4,
                vntol: 1e-7,
                abstol: 1e-13,
                itl1: 200,
                itl4: 20,
                ..Default::default()
            },
            transient: TransientOptions {
                lte_rel: 1e-4,
                lte_abs: 1e-14,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Get preset for RF/high-frequency circuits
    pub fn preset_rf() -> Self {
        Self {
            transient: TransientOptions {
                method: IntegrationMethod::Trapezoidal,
                tstep_max: 1e-12, // 1 ps max step for RF
                lte_rel: 1e-4,
                ..Default::default()
            },
            convergence: ConvergenceOptions {
                reltol: 1e-4,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Get preset for power electronics (stiff systems)
    pub fn preset_power() -> Self {
        Self {
            transient: TransientOptions {
                method: IntegrationMethod::GearVariable,
                lte_rel: 1e-3,
                ..Default::default()
            },
            dc: DcOptions {
                convergence_aid: DcConvergenceAid::GminStepping,
                ..Default::default()
            },
            convergence: ConvergenceOptions {
                damping: true,
                node_limiting: true,
                itl1: 200,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Convert UI options to core engine SimulationConfig
    ///
    /// This bridges the UI-side SimulationOptions to the core's SimulationConfig
    /// for passing solver parameters to the simulation engine.
    pub fn to_simulation_config(&self) -> rspice_core::engine::SimulationConfig {
        use rspice_core::engine::{ConvergenceConfig, DampingStrategy, SimulationConfig};

        // Map integration method
        let integration_method = match self.transient.method {
            IntegrationMethod::Trapezoidal => rspice_core::analysis::IntegrationMethod::Trapezoidal,
            IntegrationMethod::BackwardEuler => {
                rspice_core::analysis::IntegrationMethod::BackwardEuler
            }
            IntegrationMethod::Gear2 => rspice_core::analysis::IntegrationMethod::Gear2,
            IntegrationMethod::GearVariable => rspice_core::analysis::IntegrationMethod::TrapGear,
        };

        // Map convergence aid to damping strategy
        let damping_strategy = if self.convergence.damping {
            if self.convergence.node_limiting {
                DampingStrategy::Combined
            } else {
                DampingStrategy::LineSearch
            }
        } else {
            DampingStrategy::None
        };

        // Map DC convergence aid settings
        let convergence_config = ConvergenceConfig {
            gmin_stepping: matches!(
                self.dc.convergence_aid,
                DcConvergenceAid::GminStepping | DcConvergenceAid::Continuation
            ),
            source_stepping: matches!(
                self.dc.convergence_aid,
                DcConvergenceAid::SourceStepping | DcConvergenceAid::Continuation
            ),
            pseudo_transient: matches!(
                self.dc.convergence_aid,
                DcConvergenceAid::PseudoTransient | DcConvergenceAid::Continuation
            ),
            arc_length: matches!(self.dc.convergence_aid, DcConvergenceAid::Continuation),
            damping_strategy,
            gmin_initial: self.dc.gmin,
            gmin_target: self.dc.gmin * 1e-3,
            verbose: self.advanced.verbose,
        };

        SimulationConfig {
            tolerance: self.convergence.reltol,
            max_iterations: self.convergence.itl1 as usize,
            min_timestep: self.transient.tstep_min,
            max_timestep: if self.transient.tstep_max > 0.0 {
                self.transient.tstep_max
            } else {
                1e-3 // Default if auto
            },
            temperature: self.advanced.temp,
            integration_method,
            bypass_config: Default::default(),
            convergence_config,
        }
    }
}
/// Option category for UI organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionCategory {
    Transient,
    Ac,
    Dc,
    Convergence,
    Advanced,
}

impl OptionCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Transient => "Transient",
            Self::Ac => "AC Analysis",
            Self::Dc => "DC Analysis",
            Self::Convergence => "Convergence",
            Self::Advanced => "Advanced",
        }
    }

    pub fn all() -> &'static [OptionCategory] {
        &[
            OptionCategory::Transient,
            OptionCategory::Ac,
            OptionCategory::Dc,
            OptionCategory::Convergence,
            OptionCategory::Advanced,
        ]
    }
}

// =============================================================================
// Simulation Options Dialog Component
// =============================================================================

/// Simulation options dialog properties
#[derive(Props, Clone, PartialEq)]
pub struct SimulationOptionsDialogProps {
    /// Current options (initial values)
    pub options: SimulationOptions,

    /// Whether dialog is open
    #[props(default = true)]
    pub is_open: bool,

    /// Callback when dialog is closed without saving
    #[props(default)]
    pub on_close: EventHandler<()>,

    /// Callback when options are saved
    #[props(default)]
    pub on_save: EventHandler<SimulationOptions>,
}

/// Simulation options dialog component with editable fields
#[component]
pub fn SimulationOptionsDialog(props: SimulationOptionsDialogProps) -> Element {
    let mut active_category = use_signal(|| OptionCategory::Transient);

    // Local mutable state for all options - initialized from props
    let mut local_options = use_signal(|| props.options.clone());

    // Update local options when props change (dialog reopened with new values)
    use_effect(move || {
        local_options.set(props.options.clone());
    });

    if !props.is_open {
        return rsx! {};
    }

    // Preset application handlers
    let apply_fast = move |_| {
        local_options.set(SimulationOptions::preset_fast());
    };
    let apply_accurate = move |_| {
        local_options.set(SimulationOptions::preset_accurate());
    };
    let apply_rf = move |_| {
        local_options.set(SimulationOptions::preset_rf());
    };
    let apply_power = move |_| {
        local_options.set(SimulationOptions::preset_power());
    };

    // Save and close handler
    let save_and_close = {
        let on_save = props.on_save.clone();
        let on_close = props.on_close.clone();
        move |_| {
            on_save.call(local_options.read().clone());
            on_close.call(());
        }
    };

    // Reset to defaults handler
    let reset_defaults = move |_| {
        local_options.set(SimulationOptions::default());
    };

    rsx! {
        // Backdrop overlay
        div {
            class: "dialog-backdrop",
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; \
                    background: rgba(0, 0, 0, 0.6); z-index: 999;",
            onclick: move |_| props.on_close.call(()),
        }

        // Dialog content
        div {
            class: "simulation-options-dialog",
            style: "position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); \
                    background: #1a1a2e; border: 1px solid #333; border-radius: 8px; \
                    padding: 20px; min-width: 650px; max-width: 800px; max-height: 85vh; overflow: auto; \
                    color: #fff; font-family: system-ui, sans-serif; box-shadow: 0 10px 40px rgba(0,0,0,0.5); \
                    z-index: 1000;",
            // Prevent clicks inside dialog from closing it
            onclick: move |evt| evt.stop_propagation(),

            // Header with close button
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; padding-bottom: 10px; border-bottom: 1px solid #333;",
                h2 { style: "margin: 0; font-size: 18px; color: #fff;", "Simulation Options" }
                button {
                    style: "background: transparent; border: none; color: #888; font-size: 20px; \
                            cursor: pointer; padding: 4px 8px; border-radius: 4px;",
                    onclick: move |_| props.on_close.call(()),
                    title: "Close (Esc)",
                    "✕"
                }
            }

            // Category tabs - interactive
            div {
                style: "display: flex; gap: 8px; margin-bottom: 20px; border-bottom: 1px solid #333;",
                for cat in OptionCategory::all() {
                    {
                        let cat_val = *cat;
                        let is_active = *active_category.read() == cat_val;
                        let bg = if is_active { "#2a2a3e" } else { "transparent" };
                        let color = if is_active { "#fff" } else { "#888" };
                        let border_bottom = if is_active { "2px solid #4CAF50" } else { "2px solid transparent" };
                        rsx! {
                            button {
                                style: "background: {bg}; color: {color}; border: none; padding: 10px 18px; \
                                        cursor: pointer; font-size: 13px; font-weight: 500; \
                                        border-bottom: {border_bottom}; transition: all 0.15s ease;",
                                onclick: move |_| {
                                    active_category.set(cat_val);
                                },
                                "{cat_val.display_name()}"
                            }
                        }
                    }
                }
            }

            // Content based on active category - now with editable fields
            div {
                style: "min-height: 320px; padding: 10px 0;",
                match *active_category.read() {
                    OptionCategory::Transient => rsx! {
                        TransientOptionsPanel { options: local_options }
                    },
                    OptionCategory::Ac => rsx! {
                        AcOptionsPanel { options: local_options }
                    },
                    OptionCategory::Dc => rsx! {
                        DcOptionsPanel { options: local_options }
                    },
                    OptionCategory::Convergence => rsx! {
                        ConvergenceOptionsPanel { options: local_options }
                    },
                    OptionCategory::Advanced => rsx! {
                        AdvancedOptionsPanel { options: local_options }
                    },
                }
            }

            // Validation errors
            {
                let errors = local_options.read().validate();
                if !errors.is_empty() {
                    rsx! {
                        div {
                            style: "margin-top: 15px; padding: 12px; background: #3a2222; border: 1px solid #552222; border-radius: 6px;",
                            div { style: "color: #f44336; font-weight: 600; margin-bottom: 8px; font-size: 13px;", "⚠ Validation Errors" }
                            for error in errors {
                                div { style: "color: #ff9999; font-size: 12px; margin-left: 8px;", "• {error}" }
                            }
                        }
                    }
                } else {
                    rsx! {}
                }
            }

            // Presets bar
            div {
                style: "margin-top: 20px; padding: 12px; background: #222233; border-radius: 6px; display: flex; align-items: center; gap: 10px;",
                span { style: "color: #888; font-size: 12px; margin-right: 8px;", "Quick Presets:" }
                button {
                    style: "background: #ff980022; color: #ff9800; border: 1px solid #ff9800; \
                            padding: 6px 14px; border-radius: 4px; cursor: pointer; font-size: 12px; font-weight: 500;",
                    onclick: apply_fast,
                    "⚡ Fast"
                }
                button {
                    style: "background: #4CAF5022; color: #4CAF50; border: 1px solid #4CAF50; \
                            padding: 6px 14px; border-radius: 4px; cursor: pointer; font-size: 12px; font-weight: 500;",
                    onclick: apply_accurate,
                    "✓ Accurate"
                }
                button {
                    style: "background: #2196F322; color: #2196F3; border: 1px solid #2196F3; \
                            padding: 6px 14px; border-radius: 4px; cursor: pointer; font-size: 12px; font-weight: 500;",
                    onclick: apply_rf,
                    "📡 RF"
                }
                button {
                    style: "background: #9c27b022; color: #9c27b0; border: 1px solid #9c27b0; \
                            padding: 6px 14px; border-radius: 4px; cursor: pointer; font-size: 12px; font-weight: 500;",
                    onclick: apply_power,
                    "⚡ Power"
                }
                button {
                    style: "background: transparent; color: #666; border: 1px solid #444; \
                            padding: 6px 14px; border-radius: 4px; cursor: pointer; font-size: 12px; margin-left: auto;",
                    onclick: reset_defaults,
                    "Reset Defaults"
                }
            }

            // Footer with action buttons
            div {
                style: "margin-top: 20px; padding-top: 15px; border-top: 1px solid #333; display: flex; justify-content: flex-end; gap: 10px;",
                button {
                    style: "background: transparent; color: #888; border: 1px solid #444; \
                            padding: 10px 20px; border-radius: 6px; cursor: pointer; font-size: 13px;",
                    onclick: move |_| props.on_close.call(()),
                    "Cancel"
                }
                button {
                    style: "background: #4CAF50; color: #fff; border: none; \
                            padding: 10px 24px; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 600;",
                    onclick: save_and_close,
                    "Apply Changes"
                }
            }
        }
    }
}

// =============================================================================
// Panel Components for Each Option Category
// =============================================================================

/// Transient options panel with editable fields
#[component]
fn TransientOptionsPanel(options: Signal<SimulationOptions>) -> Element {
    rsx! {
        div { class: "options-panel",
            h3 { style: "color: #4CAF50; margin-bottom: 16px; font-size: 15px; font-weight: 600;", "Transient Analysis" }

            // Integration Method
            OptionSelectRow {
                label: "Integration Method",
                help: "Numerical integration algorithm",
                value: options.read().transient.method.display_name().to_string(),
                options_list: IntegrationMethod::all().iter().map(|m| m.display_name().to_string()).collect(),
                onchange: move |val: String| {
                    let method = match val.as_str() {
                        "Trapezoidal" => IntegrationMethod::Trapezoidal,
                        "Backward Euler" => IntegrationMethod::BackwardEuler,
                        "Gear (BDF-2)" => IntegrationMethod::Gear2,
                        _ => IntegrationMethod::GearVariable,
                    };
                    options.write().transient.method = method;
                },
            }

            // Max Timestep
            OptionInputRow {
                label: "Max Timestep",
                help: "Maximum simulation step (0 = auto)",
                value: format_value(options.read().transient.tstep_max, "s"),
                suffix: "s",
                onchange: move |val: String| {
                    if let Some(v) = parse_value(&val) {
                        options.write().transient.tstep_max = v;
                    }
                },
            }

            // Min Timestep
            OptionInputRow {
                label: "Min Timestep",
                help: "Minimum allowed timestep",
                value: format_value(options.read().transient.tstep_min, "s"),
                suffix: "s",
                onchange: move |val: String| {
                    if let Some(v) = parse_value(&val) {
                        options.write().transient.tstep_min = v;
                    }
                },
            }

            // Initial Timestep
            OptionInputRow {
                label: "Initial Timestep",
                help: "Starting timestep size",
                value: format_value(options.read().transient.tstep_initial, "s"),
                suffix: "s",
                onchange: move |val: String| {
                    if let Some(v) = parse_value(&val) {
                        options.write().transient.tstep_initial = v;
                    }
                },
            }

            // LTE Relative
            OptionInputRow {
                label: "LTE (relative)",
                help: "Local truncation error tolerance",
                value: format!("{:.1e}", options.read().transient.lte_rel),
                suffix: "",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<f64>() {
                        options.write().transient.lte_rel = v;
                    }
                },
            }

            // Use IC checkbox
            OptionCheckboxRow {
                label: "Use Initial Conditions",
                help: "Apply .IC statements",
                checked: options.read().transient.use_ic,
                onchange: move |checked: bool| {
                    options.write().transient.use_ic = checked;
                },
            }

            // Skip DC checkbox
            OptionCheckboxRow {
                label: "Skip DC Operating Point",
                help: "Start from .IC without DC solve",
                checked: options.read().transient.skip_dc,
                onchange: move |checked: bool| {
                    options.write().transient.skip_dc = checked;
                },
            }
        }
    }
}

/// AC options panel with editable fields
#[component]
fn AcOptionsPanel(options: Signal<SimulationOptions>) -> Element {
    rsx! {
        div { class: "options-panel",
            h3 { style: "color: #2196F3; margin-bottom: 16px; font-size: 15px; font-weight: 600;", "AC Analysis" }

            // Frequency Scale
            OptionSelectRow {
                label: "Frequency Scale",
                help: "Sweep type (decade/linear)",
                value: options.read().ac.scale.display_name().to_string(),
                options_list: FrequencyScale::all().iter().map(|s| s.display_name().to_string()).collect(),
                onchange: move |val: String| {
                    let scale = match val.as_str() {
                        "Decade" => FrequencyScale::Decade,
                        "Octave" => FrequencyScale::Octave,
                        _ => FrequencyScale::Linear,
                    };
                    options.write().ac.scale = scale;
                },
            }

            // Points per decade
            OptionInputRow {
                label: "Points/Decade",
                help: "Frequency points per decade",
                value: format!("{}", options.read().ac.points_per_decade),
                suffix: "",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<u32>() {
                        options.write().ac.points_per_decade = v;
                    }
                },
            }

            // Unwrap Phase checkbox
            OptionCheckboxRow {
                label: "Unwrap Phase",
                help: "Continuous phase display",
                checked: options.read().ac.unwrap_phase,
                onchange: move |checked: bool| {
                    options.write().ac.unwrap_phase = checked;
                },
            }

            // Interpolate checkbox
            OptionCheckboxRow {
                label: "Interpolate",
                help: "Smooth frequency interpolation",
                checked: options.read().ac.interpolate,
                onchange: move |checked: bool| {
                    options.write().ac.interpolate = checked;
                },
            }

            // Group Delay checkbox
            OptionCheckboxRow {
                label: "Calculate Group Delay",
                help: "Compute group delay from phase",
                checked: options.read().ac.calc_group_delay,
                onchange: move |checked: bool| {
                    options.write().ac.calc_group_delay = checked;
                },
            }
        }
    }
}

/// DC options panel with editable fields
#[component]
fn DcOptionsPanel(options: Signal<SimulationOptions>) -> Element {
    rsx! {
        div { class: "options-panel",
            h3 { style: "color: #ff9800; margin-bottom: 16px; font-size: 15px; font-weight: 600;", "DC Analysis" }

            // Convergence Aid
            OptionSelectRow {
                label: "Convergence Aid",
                help: "Method to help DC convergence",
                value: options.read().dc.convergence_aid.display_name().to_string(),
                options_list: DcConvergenceAid::all().iter().map(|a| a.display_name().to_string()).collect(),
                onchange: move |val: String| {
                    let aid = match val.as_str() {
                        "None" => DcConvergenceAid::None,
                        "Source Stepping" => DcConvergenceAid::SourceStepping,
                        "Gmin Stepping" => DcConvergenceAid::GminStepping,
                        "Pseudo-Transient" => DcConvergenceAid::PseudoTransient,
                        _ => DcConvergenceAid::Continuation,
                    };
                    options.write().dc.convergence_aid = aid;
                },
            }

            // Gmin
            OptionInputRow {
                label: "Gmin",
                help: "Minimum conductance (Siemens)",
                value: format!("{:.2e}", options.read().dc.gmin),
                suffix: "S",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<f64>() {
                        options.write().dc.gmin = v;
                    }
                },
            }

            // Source Steps
            OptionInputRow {
                label: "Source Steps",
                help: "Number of source stepping iterations",
                value: format!("{}", options.read().dc.source_steps),
                suffix: "",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<u32>() {
                        options.write().dc.source_steps = v;
                    }
                },
            }

            // Save OP Info checkbox
            OptionCheckboxRow {
                label: "Save OP Info",
                help: "Store operating point data",
                checked: options.read().dc.save_op_info,
                onchange: move |checked: bool| {
                    options.write().dc.save_op_info = checked;
                },
            }

            // Use Nodeset checkbox
            OptionCheckboxRow {
                label: "Use Nodeset",
                help: "Apply .NODESET values",
                checked: options.read().dc.use_nodeset,
                onchange: move |checked: bool| {
                    options.write().dc.use_nodeset = checked;
                },
            }
        }
    }
}

/// Convergence options panel with editable fields
#[component]
fn ConvergenceOptionsPanel(options: Signal<SimulationOptions>) -> Element {
    rsx! {
        div { class: "options-panel",
            h3 { style: "color: #f44336; margin-bottom: 16px; font-size: 15px; font-weight: 600;", "Convergence" }

            // RELTOL
            OptionInputRow {
                label: "RELTOL",
                help: "Relative tolerance (0.001 = 0.1%)",
                value: format!("{:.1e}", options.read().convergence.reltol),
                suffix: "",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<f64>() {
                        options.write().convergence.reltol = v;
                    }
                },
            }

            // VNTOL
            OptionInputRow {
                label: "VNTOL",
                help: "Absolute voltage tolerance",
                value: format!("{:.1e}", options.read().convergence.vntol),
                suffix: "V",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<f64>() {
                        options.write().convergence.vntol = v;
                    }
                },
            }

            // ABSTOL
            OptionInputRow {
                label: "ABSTOL",
                help: "Absolute current tolerance",
                value: format!("{:.1e}", options.read().convergence.abstol),
                suffix: "A",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<f64>() {
                        options.write().convergence.abstol = v;
                    }
                },
            }

            // ITL1
            OptionInputRow {
                label: "ITL1 (DC iterations)",
                help: "Max Newton-Raphson iterations",
                value: format!("{}", options.read().convergence.itl1),
                suffix: "",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<u32>() {
                        options.write().convergence.itl1 = v;
                    }
                },
            }

            // ITL4
            OptionInputRow {
                label: "ITL4 (Transient)",
                help: "Iterations per timepoint",
                value: format!("{}", options.read().convergence.itl4),
                suffix: "",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<u32>() {
                        options.write().convergence.itl4 = v;
                    }
                },
            }

            // Damping checkbox
            OptionCheckboxRow {
                label: "Enable Damping",
                help: "Newton step size limiting",
                checked: options.read().convergence.damping,
                onchange: move |checked: bool| {
                    options.write().convergence.damping = checked;
                },
            }

            // Node Limiting checkbox
            OptionCheckboxRow {
                label: "Node Limiting",
                help: "Clamp junction voltages",
                checked: options.read().convergence.node_limiting,
                onchange: move |checked: bool| {
                    options.write().convergence.node_limiting = checked;
                },
            }
        }
    }
}

/// Advanced options panel with editable fields
#[component]
fn AdvancedOptionsPanel(options: Signal<SimulationOptions>) -> Element {
    rsx! {
        div { class: "options-panel",
            h3 { style: "color: #9c27b0; margin-bottom: 16px; font-size: 15px; font-weight: 600;", "Advanced" }

            // Matrix Solver
            OptionSelectRow {
                label: "Matrix Solver",
                help: "Linear solver algorithm",
                value: options.read().advanced.matrix_solver.display_name().to_string(),
                options_list: MatrixSolver::all().iter().map(|s| s.display_name().to_string()).collect(),
                onchange: move |val: String| {
                    let solver = match val.as_str() {
                        "Sparse LU" => MatrixSolver::SparseLU,
                        "KLU" => MatrixSolver::KLU,
                        "SuperLU" => MatrixSolver::SuperLU,
                        "GMRES (Iterative)" => MatrixSolver::GMRES,
                        _ => MatrixSolver::Dense,
                    };
                    options.write().advanced.matrix_solver = solver;
                },
            }

            // Pivot Strategy
            OptionSelectRow {
                label: "Pivot Strategy",
                help: "Matrix pivoting method",
                value: options.read().advanced.pivot_strategy.display_name().to_string(),
                options_list: PivotStrategy::all().iter().map(|p| p.display_name().to_string()).collect(),
                onchange: move |val: String| {
                    let pivot = match val.as_str() {
                        "None" => PivotStrategy::None,
                        "Partial" => PivotStrategy::Partial,
                        "Full" => PivotStrategy::Full,
                        _ => PivotStrategy::Threshold,
                    };
                    options.write().advanced.pivot_strategy = pivot;
                },
            }

            // Temperature
            OptionInputRow {
                label: "Temperature",
                help: "Simulation temperature",
                value: format!("{:.2}", options.read().advanced.temp - 273.15),
                suffix: "°C",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<f64>() {
                        options.write().advanced.temp = v + 273.15;
                    }
                },
            }

            // Nominal Temperature
            OptionInputRow {
                label: "Nominal Temp",
                help: "Reference temperature for models",
                value: format!("{:.2}", options.read().advanced.tnom - 273.15),
                suffix: "°C",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<f64>() {
                        options.write().advanced.tnom = v + 273.15;
                    }
                },
            }

            // Threads
            OptionInputRow {
                label: "Threads",
                help: "Parallel threads (0 = auto)",
                value: format!("{}", options.read().advanced.threads),
                suffix: "",
                onchange: move |val: String| {
                    if let Ok(v) = val.parse::<u32>() {
                        options.write().advanced.threads = v;
                    }
                },
            }

            // Verbose checkbox
            OptionCheckboxRow {
                label: "Verbose Output",
                help: "Print iteration statistics",
                checked: options.read().advanced.verbose,
                onchange: move |checked: bool| {
                    options.write().advanced.verbose = checked;
                },
            }
        }
    }
}

// =============================================================================
// Reusable Option Row Components
// =============================================================================

/// A labeled input row for options
#[component]
fn OptionInputRow(
    label: &'static str,
    help: &'static str,
    value: String,
    suffix: &'static str,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            style: "display: grid; grid-template-columns: 160px 1fr; gap: 16px; align-items: center; margin-bottom: 12px; padding: 8px 0; border-bottom: 1px solid #2a2a3e;",
            div {
                span { style: "display: block; font-size: 13px; color: #ccc; font-weight: 500;", "{label}" }
                span { style: "display: block; font-size: 11px; color: #666; margin-top: 2px;", "{help}" }
            }
            div {
                style: "display: flex; align-items: center; gap: 6px;",
                input {
                    r#type: "text",
                    style: "flex: 1; padding: 8px 10px; background: #1a1a2e; border: 1px solid #444; \
                            border-radius: 4px; color: #fff; font-size: 13px; font-family: 'JetBrains Mono', monospace;",
                    value: "{value}",
                    oninput: move |e| onchange.call(e.value()),
                }
                if !suffix.is_empty() {
                    span { style: "color: #888; font-size: 12px; min-width: 24px;", "{suffix}" }
                }
            }
        }
    }
}

/// A labeled select row for options
#[component]
fn OptionSelectRow(
    label: &'static str,
    help: &'static str,
    value: String,
    options_list: Vec<String>,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            style: "display: grid; grid-template-columns: 160px 1fr; gap: 16px; align-items: center; margin-bottom: 12px; padding: 8px 0; border-bottom: 1px solid #2a2a3e;",
            div {
                span { style: "display: block; font-size: 13px; color: #ccc; font-weight: 500;", "{label}" }
                span { style: "display: block; font-size: 11px; color: #666; margin-top: 2px;", "{help}" }
            }
            select {
                style: "padding: 8px 10px; background: #1a1a2e; border: 1px solid #444; \
                        border-radius: 4px; color: #fff; font-size: 13px; cursor: pointer;",
                value: "{value}",
                onchange: move |e| onchange.call(e.value()),
                for opt in options_list.iter() {
                    option {
                        value: "{opt}",
                        selected: *opt == value,
                        "{opt}"
                    }
                }
            }
        }
    }
}

/// A labeled checkbox row for options
#[component]
fn OptionCheckboxRow(
    label: &'static str,
    help: &'static str,
    checked: bool,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        div {
            style: "display: grid; grid-template-columns: 160px 1fr; gap: 16px; align-items: center; margin-bottom: 12px; padding: 8px 0; border-bottom: 1px solid #2a2a3e;",
            div {
                span { style: "display: block; font-size: 13px; color: #ccc; font-weight: 500;", "{label}" }
                span { style: "display: block; font-size: 11px; color: #666; margin-top: 2px;", "{help}" }
            }
            label {
                style: "display: flex; align-items: center; gap: 8px; cursor: pointer;",
                input {
                    r#type: "checkbox",
                    style: "width: 18px; height: 18px; accent-color: #4CAF50; cursor: pointer;",
                    checked: checked,
                    onchange: move |e| onchange.call(e.checked()),
                }
                span { style: "color: #aaa; font-size: 12px;", if checked { "Enabled" } else { "Disabled" } }
            }
        }
    }
}

// =============================================================================
// Value Formatting Helpers
// =============================================================================

/// Format a value with engineering notation
fn format_value(val: f64, _unit: &str) -> String {
    if val == 0.0 {
        return "0".to_string();
    }
    format_sci(val)
}

/// Parse a value that may have engineering suffix
fn parse_value(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Try direct parse first
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }

    // Check for engineering suffixes
    let len = s.len();
    if len < 2 {
        return None;
    }

    let (num_str, suffix) = s.split_at(len - 1);
    let multiplier = match suffix {
        "G" => 1e9,
        "M" => 1e6,
        "k" | "K" => 1e3,
        "m" => 1e-3,
        "u" | "µ" => 1e-6,
        "n" => 1e-9,
        "p" => 1e-12,
        "f" => 1e-15,
        "a" => 1e-18,
        _ => return None,
    };

    num_str.parse::<f64>().ok().map(|v| v * multiplier)
}

fn format_sci(val: f64) -> String {
    if val.abs() >= 1e9 {
        format!("{:.2}G", val / 1e9)
    } else if val.abs() >= 1e6 {
        format!("{:.2}M", val / 1e6)
    } else if val.abs() >= 1e3 {
        format!("{:.2}k", val / 1e3)
    } else if val.abs() >= 1.0 {
        format!("{:.3}", val)
    } else if val.abs() >= 1e-3 {
        format!("{:.2}m", val * 1e3)
    } else if val.abs() >= 1e-6 {
        format!("{:.2}µ", val * 1e6)
    } else if val.abs() >= 1e-9 {
        format!("{:.2}n", val * 1e9)
    } else if val.abs() >= 1e-12 {
        format!("{:.2}p", val * 1e12)
    } else if val.abs() >= 1e-15 {
        format!("{:.2}f", val * 1e15)
    } else if val.abs() >= 1e-18 {
        format!("{:.2}a", val * 1e18)
    } else {
        format!("{:.2e}", val)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Integration Method Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_integration_method_default() {
        let method = IntegrationMethod::default();
        assert_eq!(method, IntegrationMethod::Trapezoidal);
    }

    #[test]
    fn test_integration_method_display_names() {
        assert_eq!(IntegrationMethod::Trapezoidal.display_name(), "Trapezoidal");
        assert_eq!(
            IntegrationMethod::BackwardEuler.display_name(),
            "Backward Euler"
        );
        assert_eq!(IntegrationMethod::Gear2.display_name(), "Gear (BDF-2)");
        assert_eq!(
            IntegrationMethod::GearVariable.display_name(),
            "Gear Variable Order"
        );
    }

    #[test]
    fn test_integration_method_all() {
        let methods = IntegrationMethod::all();
        assert_eq!(methods.len(), 4);
    }

    #[test]
    fn test_integration_method_descriptions() {
        for method in IntegrationMethod::all() {
            assert!(!method.description().is_empty());
        }
    }

    // -------------------------------------------------------------------------
    // Matrix Solver Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_matrix_solver_default() {
        let solver = MatrixSolver::default();
        assert_eq!(solver, MatrixSolver::SparseLU);
    }

    #[test]
    fn test_matrix_solver_all() {
        let solvers = MatrixSolver::all();
        assert_eq!(solvers.len(), 5);
    }

    #[test]
    fn test_matrix_solver_display_names() {
        assert_eq!(MatrixSolver::SparseLU.display_name(), "Sparse LU");
        assert_eq!(MatrixSolver::GMRES.display_name(), "GMRES (Iterative)");
    }

    // -------------------------------------------------------------------------
    // Pivot Strategy Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_pivot_strategy_default() {
        let strategy = PivotStrategy::default();
        assert_eq!(strategy, PivotStrategy::Partial);
    }

    #[test]
    fn test_pivot_strategy_all() {
        assert_eq!(PivotStrategy::all().len(), 4);
    }

    // -------------------------------------------------------------------------
    // Transient Options Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_transient_options_default() {
        let opts = TransientOptions::default();
        assert!(opts.tstep_min > 0.0);
        assert!(opts.lte_rel > 0.0 && opts.lte_rel <= 1.0);
        assert_eq!(opts.method, IntegrationMethod::Trapezoidal);
    }

    #[test]
    fn test_transient_options_validate_valid() {
        let opts = TransientOptions::default();
        let errors = opts.validate();
        assert!(errors.is_empty(), "Default should be valid: {:?}", errors);
    }

    #[test]
    fn test_transient_options_validate_invalid_tstep_min() {
        let mut opts = TransientOptions::default();
        opts.tstep_min = 0.0;
        let errors = opts.validate();
        assert!(!errors.is_empty());
        assert!(errors[0].contains("Minimum timestep"));
    }

    #[test]
    fn test_transient_options_validate_invalid_lte() {
        let mut opts = TransientOptions::default();
        opts.lte_rel = 2.0; // Invalid: > 1.0
        let errors = opts.validate();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_transient_options_validate_tstep_max_invalid() {
        let mut opts = TransientOptions::default();
        opts.tstep_max = 1e-20; // Less than min
        let errors = opts.validate();
        assert!(!errors.is_empty());
    }

    // -------------------------------------------------------------------------
    // AC Options Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_ac_options_default() {
        let opts = AcOptions::default();
        assert_eq!(opts.scale, FrequencyScale::Decade);
        assert!(opts.points_per_decade > 0);
    }

    #[test]
    fn test_ac_options_validate_valid() {
        let opts = AcOptions::default();
        assert!(opts.validate().is_empty());
    }

    #[test]
    fn test_ac_options_validate_invalid() {
        let mut opts = AcOptions::default();
        opts.points_per_decade = 0;
        assert!(!opts.validate().is_empty());
    }

    #[test]
    fn test_frequency_scale_all() {
        assert_eq!(FrequencyScale::all().len(), 3);
    }

    // -------------------------------------------------------------------------
    // DC Options Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_dc_options_default() {
        let opts = DcOptions::default();
        assert_eq!(opts.convergence_aid, DcConvergenceAid::None);
        assert!(opts.gmin > 0.0);
    }

    #[test]
    fn test_dc_options_validate_valid() {
        let opts = DcOptions::default();
        assert!(opts.validate().is_empty());
    }

    #[test]
    fn test_dc_options_validate_invalid_gmin() {
        let mut opts = DcOptions::default();
        opts.gmin = 0.0;
        assert!(!opts.validate().is_empty());
    }

    #[test]
    fn test_dc_convergence_aid_all() {
        assert_eq!(DcConvergenceAid::all().len(), 5);
    }

    // -------------------------------------------------------------------------
    // Convergence Options Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_convergence_options_default() {
        let opts = ConvergenceOptions::default();
        assert!(opts.reltol > 0.0);
        assert!(opts.vntol > 0.0);
        assert!(opts.abstol > 0.0);
    }

    #[test]
    fn test_convergence_options_validate_valid() {
        let opts = ConvergenceOptions::default();
        assert!(opts.validate().is_empty());
    }

    #[test]
    fn test_convergence_options_validate_invalid_reltol() {
        let mut opts = ConvergenceOptions::default();
        opts.reltol = 0.0;
        assert!(!opts.validate().is_empty());
    }

    #[test]
    fn test_convergence_options_relaxed() {
        let opts = ConvergenceOptions::default();
        let relaxed = opts.relaxed();
        assert!(relaxed.reltol > opts.reltol);
        assert!(relaxed.itl1 > opts.itl1);
    }

    #[test]
    fn test_convergence_options_tight() {
        let opts = ConvergenceOptions::default();
        let tight = opts.tight();
        assert!(tight.reltol < opts.reltol);
    }

    // -------------------------------------------------------------------------
    // Advanced Options Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_advanced_options_default() {
        let opts = AdvancedOptions::default();
        assert_eq!(opts.matrix_solver, MatrixSolver::SparseLU);
        assert!(opts.temp > 0.0);
    }

    #[test]
    fn test_advanced_options_validate_valid() {
        let opts = AdvancedOptions::default();
        assert!(opts.validate().is_empty());
    }

    #[test]
    fn test_advanced_options_validate_invalid_temp() {
        let mut opts = AdvancedOptions::default();
        opts.temp = 0.0;
        assert!(!opts.validate().is_empty());
    }

    // -------------------------------------------------------------------------
    // Simulation Options Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_simulation_options_new() {
        let opts = SimulationOptions::new();
        assert!(opts.is_valid());
    }

    #[test]
    fn test_simulation_options_validate() {
        let opts = SimulationOptions::default();
        let errors = opts.validate();
        assert!(errors.is_empty(), "Default should be valid: {:?}", errors);
    }

    #[test]
    fn test_simulation_options_to_spice() {
        let opts = SimulationOptions::default();
        let spice = opts.to_spice_options();
        assert!(spice.contains("RELTOL"));
        assert!(spice.contains("VNTOL"));
        assert!(spice.contains("METHOD"));
    }

    #[test]
    fn test_simulation_options_parse_reltol() {
        let mut opts = SimulationOptions::default();
        opts.parse_spice_option("RELTOL", "1e-4").unwrap();
        assert!((opts.convergence.reltol - 1e-4).abs() < 1e-10);
    }

    #[test]
    fn test_simulation_options_parse_temp() {
        let mut opts = SimulationOptions::default();
        opts.parse_spice_option("TEMP", "85").unwrap();
        assert!((opts.advanced.temp - 358.15).abs() < 0.1);
    }

    #[test]
    fn test_simulation_options_parse_method() {
        let mut opts = SimulationOptions::default();
        opts.parse_spice_option("METHOD", "GEAR").unwrap();
        assert_eq!(opts.transient.method, IntegrationMethod::Gear2);
    }

    #[test]
    fn test_simulation_options_parse_custom() {
        let mut opts = SimulationOptions::default();
        opts.parse_spice_option("UNKNOWN_OPT", "value123").unwrap();
        assert_eq!(
            opts.custom.get("UNKNOWN_OPT"),
            Some(&"value123".to_string())
        );
    }

    #[test]
    fn test_simulation_options_reset() {
        let mut opts = SimulationOptions::default();
        opts.convergence.reltol = 0.5;
        opts.reset();
        assert!((opts.convergence.reltol - 1e-3).abs() < 1e-10);
    }

    #[test]
    fn test_simulation_options_preset_fast() {
        let opts = SimulationOptions::preset_fast();
        assert!(opts.is_valid());
        assert!(opts.convergence.reltol > ConvergenceOptions::default().reltol);
    }

    #[test]
    fn test_simulation_options_preset_accurate() {
        let opts = SimulationOptions::preset_accurate();
        assert!(opts.is_valid());
        assert!(opts.convergence.reltol < ConvergenceOptions::default().reltol);
    }

    #[test]
    fn test_simulation_options_preset_rf() {
        let opts = SimulationOptions::preset_rf();
        assert!(opts.is_valid());
        assert!(opts.transient.tstep_max > 0.0);
    }

    #[test]
    fn test_simulation_options_preset_power() {
        let opts = SimulationOptions::preset_power();
        assert!(opts.is_valid());
        assert_eq!(opts.transient.method, IntegrationMethod::GearVariable);
        assert_eq!(opts.dc.convergence_aid, DcConvergenceAid::GminStepping);
    }

    // -------------------------------------------------------------------------
    // Option Category Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_option_category_all() {
        assert_eq!(OptionCategory::all().len(), 5);
    }

    #[test]
    fn test_option_category_display_names() {
        assert_eq!(OptionCategory::Transient.display_name(), "Transient");
        assert_eq!(OptionCategory::Convergence.display_name(), "Convergence");
    }

    // -------------------------------------------------------------------------
    // Serialization Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_simulation_options_serialize() {
        let opts = SimulationOptions::default();
        let json = serde_json::to_string(&opts).unwrap();
        assert!(json.contains("reltol"));
    }

    #[test]
    fn test_simulation_options_deserialize() {
        let opts = SimulationOptions::default();
        let json = serde_json::to_string(&opts).unwrap();
        let parsed: SimulationOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(opts, parsed);
    }

    #[test]
    fn test_transient_options_roundtrip() {
        let opts = TransientOptions::default();
        let json = serde_json::to_string(&opts).unwrap();
        let parsed: TransientOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(opts, parsed);
    }

    #[test]
    fn test_convergence_options_roundtrip() {
        let opts = ConvergenceOptions::default();
        let json = serde_json::to_string(&opts).unwrap();
        let parsed: ConvergenceOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(opts, parsed);
    }

    // -------------------------------------------------------------------------
    // to_simulation_config Tests (Backend Integration)
    // -------------------------------------------------------------------------

    #[test]
    fn test_to_simulation_config_default() {
        let opts = SimulationOptions::default();
        let config = opts.to_simulation_config();
        assert!((config.tolerance - 1e-3).abs() < 1e-10);
        assert_eq!(config.max_iterations, 100);
        assert!((config.temperature - 300.15).abs() < 0.01);
    }

    #[test]
    fn test_to_simulation_config_fast_preset() {
        let opts = SimulationOptions::preset_fast();
        let config = opts.to_simulation_config();
        // Fast has relaxed tolerances
        assert!(config.tolerance > 1e-3);
    }

    #[test]
    fn test_to_simulation_config_accurate_preset() {
        let opts = SimulationOptions::preset_accurate();
        let config = opts.to_simulation_config();
        // Accurate has tighter tolerances
        assert!(config.tolerance < 1e-3);
        assert!(config.max_iterations > 100);
    }

    #[test]
    fn test_to_simulation_config_integration_method_trapezoidal() {
        let mut opts = SimulationOptions::default();
        opts.transient.method = IntegrationMethod::Trapezoidal;
        let config = opts.to_simulation_config();
        assert_eq!(
            config.integration_method,
            rspice_core::analysis::IntegrationMethod::Trapezoidal
        );
    }

    #[test]
    fn test_to_simulation_config_integration_method_backward_euler() {
        let mut opts = SimulationOptions::default();
        opts.transient.method = IntegrationMethod::BackwardEuler;
        let config = opts.to_simulation_config();
        assert_eq!(
            config.integration_method,
            rspice_core::analysis::IntegrationMethod::BackwardEuler
        );
    }

    #[test]
    fn test_to_simulation_config_integration_method_gear2() {
        let mut opts = SimulationOptions::default();
        opts.transient.method = IntegrationMethod::Gear2;
        let config = opts.to_simulation_config();
        assert_eq!(
            config.integration_method,
            rspice_core::analysis::IntegrationMethod::Gear2
        );
    }

    #[test]
    fn test_to_simulation_config_integration_method_gear_variable() {
        let mut opts = SimulationOptions::default();
        opts.transient.method = IntegrationMethod::GearVariable;
        let config = opts.to_simulation_config();
        assert_eq!(
            config.integration_method,
            rspice_core::analysis::IntegrationMethod::TrapGear
        );
    }

    #[test]
    fn test_to_simulation_config_damping_enabled() {
        let mut opts = SimulationOptions::default();
        opts.convergence.damping = true;
        opts.convergence.node_limiting = true;
        let config = opts.to_simulation_config();
        assert_eq!(
            config.convergence_config.damping_strategy,
            rspice_core::engine::DampingStrategy::Combined
        );
    }

    #[test]
    fn test_to_simulation_config_damping_no_limiting() {
        let mut opts = SimulationOptions::default();
        opts.convergence.damping = true;
        opts.convergence.node_limiting = false;
        let config = opts.to_simulation_config();
        assert_eq!(
            config.convergence_config.damping_strategy,
            rspice_core::engine::DampingStrategy::LineSearch
        );
    }

    #[test]
    fn test_to_simulation_config_damping_disabled() {
        let mut opts = SimulationOptions::default();
        opts.convergence.damping = false;
        let config = opts.to_simulation_config();
        assert_eq!(
            config.convergence_config.damping_strategy,
            rspice_core::engine::DampingStrategy::None
        );
    }

    #[test]
    fn test_to_simulation_config_gmin_stepping() {
        let mut opts = SimulationOptions::default();
        opts.dc.convergence_aid = DcConvergenceAid::GminStepping;
        let config = opts.to_simulation_config();
        assert!(config.convergence_config.gmin_stepping);
        assert!(!config.convergence_config.source_stepping);
    }

    #[test]
    fn test_to_simulation_config_source_stepping() {
        let mut opts = SimulationOptions::default();
        opts.dc.convergence_aid = DcConvergenceAid::SourceStepping;
        let config = opts.to_simulation_config();
        assert!(!config.convergence_config.gmin_stepping);
        assert!(config.convergence_config.source_stepping);
    }

    #[test]
    fn test_to_simulation_config_continuation() {
        let mut opts = SimulationOptions::default();
        opts.dc.convergence_aid = DcConvergenceAid::Continuation;
        let config = opts.to_simulation_config();
        // Continuation enables all methods
        assert!(config.convergence_config.gmin_stepping);
        assert!(config.convergence_config.source_stepping);
        assert!(config.convergence_config.pseudo_transient);
        assert!(config.convergence_config.arc_length);
    }

    #[test]
    fn test_to_simulation_config_timestep_auto() {
        let mut opts = SimulationOptions::default();
        opts.transient.tstep_max = 0.0; // Auto
        let config = opts.to_simulation_config();
        assert!((config.max_timestep - 1e-3).abs() < 1e-10);
    }

    #[test]
    fn test_to_simulation_config_timestep_custom() {
        let mut opts = SimulationOptions::default();
        opts.transient.tstep_max = 1e-6;
        let config = opts.to_simulation_config();
        assert!((config.max_timestep - 1e-6).abs() < 1e-15);
    }

    #[test]
    fn test_to_simulation_config_temperature() {
        let mut opts = SimulationOptions::default();
        opts.advanced.temp = 400.0; // 126.85°C
        let config = opts.to_simulation_config();
        assert!((config.temperature - 400.0).abs() < 0.01);
    }

    #[test]
    fn test_to_simulation_config_verbose() {
        let mut opts = SimulationOptions::default();
        opts.advanced.verbose = true;
        let config = opts.to_simulation_config();
        assert!(config.convergence_config.verbose);
    }

    // -------------------------------------------------------------------------
    // parse_value Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_value_direct_number() {
        assert_eq!(parse_value("123.45"), Some(123.45));
        assert_eq!(parse_value("1e-3"), Some(1e-3));
        assert_eq!(parse_value("0"), Some(0.0));
    }

    #[test]
    fn test_parse_value_giga() {
        assert_eq!(parse_value("1G"), Some(1e9));
        assert_eq!(parse_value("2.5G"), Some(2.5e9));
    }

    #[test]
    fn test_parse_value_mega() {
        assert_eq!(parse_value("1M"), Some(1e6));
        assert_eq!(parse_value("2.5M"), Some(2.5e6));
    }

    #[test]
    fn test_parse_value_kilo() {
        assert_eq!(parse_value("1k"), Some(1e3));
        assert_eq!(parse_value("1K"), Some(1e3));
        assert_eq!(parse_value("4.7k"), Some(4.7e3));
    }

    #[test]
    fn test_parse_value_milli() {
        assert_eq!(parse_value("1m"), Some(1e-3));
        assert_eq!(parse_value("100m"), Some(100e-3));
    }

    #[test]
    fn test_parse_value_micro() {
        let val = parse_value("1u").unwrap();
        assert!((val - 1e-6).abs() < 1e-15);
        let val = parse_value("10u").unwrap();
        assert!((val - 1e-5).abs() < 1e-14);
    }

    #[test]
    fn test_parse_value_nano() {
        let val = parse_value("1n").unwrap();
        assert!((val - 1e-9).abs() < 1e-18);
        let val = parse_value("100n").unwrap();
        assert!((val - 1e-7).abs() < 1e-16);
    }

    #[test]
    fn test_parse_value_pico() {
        assert_eq!(parse_value("1p"), Some(1e-12));
        assert_eq!(parse_value("10p"), Some(10e-12));
    }

    #[test]
    fn test_parse_value_femto() {
        assert_eq!(parse_value("1f"), Some(1e-15));
    }

    #[test]
    fn test_parse_value_atto() {
        assert_eq!(parse_value("1a"), Some(1e-18));
    }

    #[test]
    fn test_parse_value_whitespace() {
        assert_eq!(parse_value("  123  "), Some(123.0));
        assert_eq!(parse_value("  1k  "), Some(1e3));
    }

    #[test]
    fn test_parse_value_empty() {
        assert_eq!(parse_value(""), None);
        assert_eq!(parse_value("   "), None);
    }

    #[test]
    fn test_parse_value_invalid() {
        assert_eq!(parse_value("abc"), None);
        assert_eq!(parse_value("x"), None);
    }

    // -------------------------------------------------------------------------
    // format_sci Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_sci_giga() {
        assert_eq!(format_sci(1e9), "1.00G");
        assert_eq!(format_sci(2.5e9), "2.50G");
    }

    #[test]
    fn test_format_sci_mega() {
        assert_eq!(format_sci(1e6), "1.00M");
        assert_eq!(format_sci(10e6), "10.00M");
    }

    #[test]
    fn test_format_sci_kilo() {
        assert_eq!(format_sci(1e3), "1.00k");
        assert_eq!(format_sci(4.7e3), "4.70k");
    }

    #[test]
    fn test_format_sci_unity() {
        assert_eq!(format_sci(1.0), "1.000");
        assert_eq!(format_sci(100.0), "100.000");
    }

    #[test]
    fn test_format_sci_milli() {
        assert_eq!(format_sci(1e-3), "1.00m");
        assert_eq!(format_sci(100e-3), "100.00m");
    }

    #[test]
    fn test_format_sci_micro() {
        assert_eq!(format_sci(1e-6), "1.00µ");
        assert_eq!(format_sci(10e-6), "10.00µ");
    }

    #[test]
    fn test_format_sci_nano() {
        assert_eq!(format_sci(1e-9), "1.00n");
        assert_eq!(format_sci(100e-9), "100.00n");
    }

    #[test]
    fn test_format_sci_pico() {
        assert_eq!(format_sci(1e-12), "1.00p");
    }

    #[test]
    fn test_format_sci_femto() {
        assert_eq!(format_sci(1e-15), "1.00f");
    }

    #[test]
    fn test_format_sci_atto() {
        assert_eq!(format_sci(1e-18), "1.00a");
    }

    #[test]
    fn test_format_sci_very_small() {
        let formatted = format_sci(1e-21);
        assert!(formatted.contains("e"));
    }
}
