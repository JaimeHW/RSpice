//! Simulation Options - Spectre-Compatible Configuration
//!
//! This module provides comprehensive simulation options matching commercial
//! SPICE simulators like Cadence Spectre. Options control accuracy, convergence,
//! algorithm selection, and performance optimization.
//!
//! # Option Categories
//!
//! - **Accuracy**: Tolerances (reltol, abstol, vntol, etc.)
//! - **Convergence**: Newton-Raphson settings, damping, aids
//! - **Algorithm**: Integration method, matrix solver
//! - **Limits**: Iteration limits, timestep bounds
//! - **Temperature**: Operating and nominal temperature

use std::fmt;

//=============================================================================
// SI Prefix Parsing (shared with other dialogs)
//=============================================================================

/// Parse a value string with SI prefix (e.g., "1u", "10n", "1e-9")
/// Uses SPICE conventions: Meg for mega (not M), m for milli
pub fn parse_si_value(s: &str) -> Result<f64, ParseError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    // Try direct float parsing first
    if let Ok(v) = s.parse::<f64>() {
        return Ok(v);
    }

    // Find where the numeric part ends by scanning forward
    // Numeric chars: digits, '.', 'e', 'E', and '-'/'+' only after e/E
    let bytes = s.as_bytes();
    let mut split_idx = 0;
    let mut in_exponent = false;

    for (i, &b) in bytes.iter().enumerate() {
        let c = b as char;
        if c.is_ascii_digit() || c == '.' {
            split_idx = i + 1;
        } else if c == 'e' || c == 'E' {
            // Check if this looks like scientific notation
            if i + 1 < bytes.len() {
                let next_c = bytes[i + 1] as char;
                if next_c.is_ascii_digit() || next_c == '-' || next_c == '+' {
                    split_idx = i + 1;
                    in_exponent = true;
                } else {
                    // This 'e' is part of a suffix like "Meg"
                    break;
                }
            } else {
                break;
            }
        } else if (c == '-' || c == '+') && in_exponent {
            split_idx = i + 1;
        } else {
            // Not a numeric character
            break;
        }
    }

    if split_idx == 0 {
        return Err(ParseError::NoNumericPart);
    }

    let (num_part, suffix) = s.split_at(split_idx);
    let base: f64 = num_part
        .parse()
        .map_err(|_| ParseError::InvalidNumber(num_part.to_string()))?;

    let multiplier = match suffix.trim().to_lowercase().as_str() {
        "" => 1.0,
        "t" | "tera" => 1e12,
        "g" | "gig" | "giga" => 1e9,
        "meg" | "mega" | "x" => 1e6, // SPICE uses Meg for mega, x sometimes used
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3, // SPICE: m = milli (NOT mega)
        "u" | "µ" | "micro" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        "a" | "atto" => 1e-18,
        other => return Err(ParseError::UnknownSuffix(other.to_string())),
    };

    Ok(base * multiplier)
}

/// Format a value with SI prefix
/// Uses SPICE conventions: Meg for mega (not M which means milli)
pub fn format_si_value(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }

    let abs = v.abs();
    let (scaled, suffix) = if abs >= 1e12 {
        (v / 1e12, "T")
    } else if abs >= 1e9 {
        (v / 1e9, "G")
    } else if abs >= 1e6 {
        (v / 1e6, "Meg") // SPICE convention: Meg, not M
    } else if abs >= 1e3 {
        (v / 1e3, "k")
    } else if abs >= 1.0 {
        (v, "")
    } else if abs >= 1e-3 {
        (v * 1e3, "m")
    } else if abs >= 1e-6 {
        (v * 1e6, "u")
    } else if abs >= 1e-9 {
        (v * 1e9, "n")
    } else if abs >= 1e-12 {
        (v * 1e12, "p")
    } else if abs >= 1e-15 {
        (v * 1e15, "f")
    } else {
        (v * 1e18, "a")
    };

    // Remove trailing zeros
    let formatted = format!("{:.6}", scaled);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    format!("{}{}", trimmed, suffix)
}

/// Parse error types
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Empty,
    NoNumericPart,
    InvalidNumber(String),
    UnknownSuffix(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "Empty string"),
            ParseError::NoNumericPart => write!(f, "No numeric part found"),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::UnknownSuffix(s) => write!(f, "Unknown SI suffix: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

//=============================================================================
// Integration Method
//=============================================================================

/// Integration method for transient analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntegrationMethod {
    /// Trapezoidal rule (A-stable, 2nd order)
    Trap,
    /// Backward Euler (L-stable, 1st order)
    Euler,
    /// Gear's method (BDF, stiff circuits)
    Gear,
    /// Second-order Gear (BDF-2)
    Gear2,
    /// Automatic trap/gear switching (Spectre default)
    #[default]
    TrapGear,
    /// Gear only, no trap phase
    Gear2Only,
}

impl IntegrationMethod {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            IntegrationMethod::Trap => "Trapezoidal",
            IntegrationMethod::Euler => "Backward Euler",
            IntegrationMethod::Gear => "Gear (BDF)",
            IntegrationMethod::Gear2 => "Gear-2",
            IntegrationMethod::TrapGear => "Trap/Gear (Auto)",
            IntegrationMethod::Gear2Only => "Gear-2 Only",
        }
    }

    /// Get SPICE option name
    pub fn spice_name(&self) -> &'static str {
        match self {
            IntegrationMethod::Trap => "TRAP",
            IntegrationMethod::Euler => "EULER",
            IntegrationMethod::Gear => "GEAR",
            IntegrationMethod::Gear2 => "GEAR2",
            IntegrationMethod::TrapGear => "TRAPGEAR",
            IntegrationMethod::Gear2Only => "GEAR2ONLY",
        }
    }

    /// Parse from SPICE option string
    pub fn from_spice(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRAP" | "TRAPEZOIDAL" => Some(IntegrationMethod::Trap),
            "EULER" | "BE" => Some(IntegrationMethod::Euler),
            "GEAR" | "BDF" => Some(IntegrationMethod::Gear),
            "GEAR2" => Some(IntegrationMethod::Gear2),
            "TRAPGEAR" | "AUTO" => Some(IntegrationMethod::TrapGear),
            "GEAR2ONLY" => Some(IntegrationMethod::Gear2Only),
            _ => None,
        }
    }

    /// All available methods
    pub fn all() -> &'static [IntegrationMethod] {
        &[
            IntegrationMethod::Trap,
            IntegrationMethod::Euler,
            IntegrationMethod::Gear,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
            IntegrationMethod::Gear2Only,
        ]
    }
}

//=============================================================================
// Damping Strategy
//=============================================================================

/// Damping strategy for Newton-Raphson convergence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DampingStrategy {
    /// No damping (full Newton step)
    None,
    /// Backtracking line search (Armijo condition)
    LineSearch,
    /// Junction voltage limiting (SPICE-style)
    #[default]
    VoltageLimiting,
    /// Bank-Rose adaptive damping
    BankRose,
    /// Combined: voltage limiting + line search
    Combined,
}

impl DampingStrategy {
    pub fn display_name(&self) -> &'static str {
        match self {
            DampingStrategy::None => "None",
            DampingStrategy::LineSearch => "Line Search",
            DampingStrategy::VoltageLimiting => "Voltage Limiting",
            DampingStrategy::BankRose => "Bank-Rose",
            DampingStrategy::Combined => "Combined",
        }
    }

    pub fn all() -> &'static [DampingStrategy] {
        &[
            DampingStrategy::None,
            DampingStrategy::LineSearch,
            DampingStrategy::VoltageLimiting,
            DampingStrategy::BankRose,
            DampingStrategy::Combined,
        ]
    }
}

//=============================================================================
// Matrix Solver Type
//=============================================================================

/// Matrix solver algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatrixSolver {
    /// LU decomposition with partial pivoting
    #[default]
    Lu,
    /// Sparse LU (for large circuits)
    SparseLu,
    /// Iterative GMRES (for very large circuits)
    Gmres,
    /// Direct KLU solver (SuiteSparse)
    Klu,
}

impl MatrixSolver {
    pub fn display_name(&self) -> &'static str {
        match self {
            MatrixSolver::Lu => "LU Decomposition",
            MatrixSolver::SparseLu => "Sparse LU",
            MatrixSolver::Gmres => "GMRES (Iterative)",
            MatrixSolver::Klu => "KLU (SuiteSparse)",
        }
    }

    pub fn all() -> &'static [MatrixSolver] {
        &[
            MatrixSolver::Lu,
            MatrixSolver::SparseLu,
            MatrixSolver::Gmres,
            MatrixSolver::Klu,
        ]
    }
}

//=============================================================================
// Simulation Options (Spectre-Compatible)
//=============================================================================

/// Complete simulation options matching Cadence Spectre
///
/// These options control all aspects of simulation accuracy, convergence,
/// and performance. Default values match industry-standard SPICE defaults.
#[derive(Debug, Clone)]
pub struct SimulationOptions {
    //=========================================================================
    // Accuracy Options
    //=========================================================================
    /// Relative tolerance for convergence (default: 1e-3)
    pub reltol: f64,

    /// Relative residual tolerance for equation convergence (default: 1e-3)
    pub residual_reltol: f64,

    /// Absolute voltage tolerance (default: 1e-6 V)
    pub vntol: f64,

    /// Absolute current tolerance (default: 1e-12 A)
    pub abstol: f64,

    /// Absolute current tolerance for device models (default: 1e-12 A)
    pub iabstol: f64,

    /// Charge tolerance (default: 1e-14 C)
    pub chgtol: f64,

    /// Pivot relative tolerance (default: 1e-3)
    pub pivrel: f64,

    /// Pivot absolute tolerance (default: 1e-13)
    pub pivtol: f64,

    //=========================================================================
    // Convergence Options
    //=========================================================================
    /// Maximum Newton-Raphson iterations (default: 50)
    pub itl1: usize,

    /// Maximum DC sweep iterations per point (default: 100)
    pub itl2: usize,

    /// Maximum transient iterations per timepoint (default: 6)
    pub itl4: usize,

    /// GMIN stepping enabled (default: true)
    pub gmin_stepping: bool,

    /// Source stepping enabled (default: true)
    pub source_stepping: bool,

    /// Pseudo-transient continuation enabled (default: true)
    pub pseudo_transient: bool,

    /// Arc-length continuation enabled (default: false)
    pub arc_length: bool,

    /// Initial GMIN value (default: 1e-12)
    pub gmin: f64,

    /// Damping strategy (default: VoltageLimiting)
    pub damping: DampingStrategy,

    //=========================================================================
    // Algorithm Options
    //=========================================================================
    /// Integration method (default: TrapGear)
    pub method: IntegrationMethod,

    /// Matrix solver (default: Lu)
    pub solver: MatrixSolver,

    /// Enable model evaluation bypass (default: false)
    pub bypass_enabled: bool,

    /// Bypass relative tolerance (default: 1e-3)
    pub bypass_reltol: f64,

    /// Bypass absolute tolerance (default: 1e-6)
    pub bypass_abstol: f64,

    //=========================================================================
    // Limit Options
    //=========================================================================
    /// Minimum timestep for transient (default: 1e-15 s)
    pub min_timestep: f64,

    /// Maximum timestep for transient (default: 1e-3 s)
    pub max_timestep: f64,

    /// Timestep reduction factor on convergence failure (default: 8)
    pub timestep_factor: f64,

    //=========================================================================
    // Temperature Options
    //=========================================================================
    /// Operating temperature in Celsius (default: 27°C)
    pub temp: f64,

    /// Nominal/reference temperature in Celsius (default: 27°C)
    pub tnom: f64,

    //=========================================================================
    // Output Options
    //=========================================================================
    /// Verbose convergence info (default: false)
    pub verbose: bool,

    /// Save internal node voltages (default: false)
    pub save_internals: bool,
}

impl Default for SimulationOptions {
    fn default() -> Self {
        Self {
            // Accuracy - SPICE defaults
            reltol: 1e-3,
            residual_reltol: 1e-3,
            vntol: 1e-6,
            abstol: 1e-12,
            iabstol: 1e-12,
            chgtol: 1e-14,
            pivrel: 1e-3,
            pivtol: 1e-13,

            // Convergence
            itl1: 50,
            itl2: 100,
            itl4: 6,
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: false,
            gmin: 1e-12,
            damping: DampingStrategy::VoltageLimiting,

            // Algorithm
            method: IntegrationMethod::TrapGear,
            solver: MatrixSolver::Lu,
            bypass_enabled: false,
            bypass_reltol: 1e-3,
            bypass_abstol: 1e-6,

            // Limits
            min_timestep: 1e-15,
            max_timestep: 1e-3,
            timestep_factor: 8.0,

            // Temperature
            temp: 27.0,
            tnom: 27.0,

            // Output
            verbose: false,
            save_internals: false,
        }
    }
}

impl SimulationOptions {
    /// Create options optimized for fast simulation (loose tolerances)
    pub fn fast() -> Self {
        Self {
            reltol: 1e-2,
            residual_reltol: 1e-2,
            abstol: 1e-9,
            iabstol: 1e-9,
            itl1: 30,
            itl4: 4,
            gmin_stepping: false,
            source_stepping: false,
            pseudo_transient: false,
            bypass_enabled: true,
            ..Default::default()
        }
    }

    /// Create options optimized for accuracy (tight tolerances)
    pub fn accurate() -> Self {
        Self {
            reltol: 1e-4,
            residual_reltol: 1e-4,
            vntol: 1e-7,
            abstol: 1e-14,
            iabstol: 1e-14,
            chgtol: 1e-16,
            itl1: 100,
            itl4: 10,
            min_timestep: 1e-18,
            damping: DampingStrategy::Combined,
            ..Default::default()
        }
    }

    /// Create options optimized for difficult/stiff circuits
    pub fn robust() -> Self {
        Self {
            reltol: 1e-3,
            residual_reltol: 1e-3,
            itl1: 200,
            itl4: 20,
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: true,
            gmin: 1e-10,
            method: IntegrationMethod::Gear2Only,
            damping: DampingStrategy::Combined,
            timestep_factor: 16.0,
            ..Default::default()
        }
    }

    /// Get temperature in Kelvin
    pub fn temp_kelvin(&self) -> f64 {
        self.temp + 273.15
    }

    /// Get nominal temperature in Kelvin
    pub fn tnom_kelvin(&self) -> f64 {
        self.tnom + 273.15
    }

    fn core_damping_strategy(&self) -> rspice_core::engine::DampingStrategy {
        match self.damping {
            DampingStrategy::None => rspice_core::engine::DampingStrategy::None,
            DampingStrategy::LineSearch => rspice_core::engine::DampingStrategy::LineSearch,
            DampingStrategy::VoltageLimiting => {
                rspice_core::engine::DampingStrategy::VoltageLimiting
            }
            DampingStrategy::BankRose => rspice_core::engine::DampingStrategy::BankRose,
            DampingStrategy::Combined => rspice_core::engine::DampingStrategy::Combined,
        }
    }

    fn core_integration_method(&self) -> rspice_core::analysis::IntegrationMethod {
        match self.method {
            IntegrationMethod::Trap => rspice_core::analysis::IntegrationMethod::Trapezoidal,
            IntegrationMethod::Euler => rspice_core::analysis::IntegrationMethod::BackwardEuler,
            IntegrationMethod::Gear => rspice_core::analysis::IntegrationMethod::Gear2,
            IntegrationMethod::Gear2 => rspice_core::analysis::IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear => rspice_core::analysis::IntegrationMethod::TrapGear,
            IntegrationMethod::Gear2Only => rspice_core::analysis::IntegrationMethod::Gear2,
        }
    }

    fn simulation_config_overrides(&self) -> rspice_core::SimulationConfigOverrides {
        rspice_core::SimulationConfigOverrides {
            temperature_kelvin: Some(self.temp_kelvin()),
            max_iterations: Some(self.itl1),
            min_timestep: Some(self.min_timestep),
            max_timestep: Some(self.max_timestep),
            integration_method: Some(self.core_integration_method()),
            convergence_preset: None,
            reltol: Some(self.reltol),
            abstol: Some(self.abstol),
            voltage_abstol: Some(self.vntol),
            current_abstol: Some(self.iabstol),
            charge_abstol: Some(self.chgtol),
            residual_reltol: Some(self.residual_reltol),
            gmin_initial: Some(self.gmin),
        }
    }

    /// Resolve to core simulation config using layered precedence:
    /// core defaults < netlist `.OPTIONS` < UI options.
    pub fn resolve_simulation_config(
        &self,
        netlist_options: Option<&rspice_core::netlist::SimulationOptions>,
    ) -> rspice_core::engine::SimulationConfig {
        use rspice_core::engine::{BypassConfig, SimulationConfig};

        let overrides = self.simulation_config_overrides();
        let mut sim_config = rspice_core::resolve_simulation_config(
            &SimulationConfig::default(),
            netlist_options,
            &overrides,
        );

        sim_config.bypass_config = BypassConfig {
            enabled: self.bypass_enabled,
            reltol: self.bypass_reltol,
            abstol: self.bypass_abstol,
        };

        sim_config.convergence_config.gmin_stepping = self.gmin_stepping;
        sim_config.convergence_config.source_stepping = self.source_stepping;
        sim_config.convergence_config.pseudo_transient = self.pseudo_transient;
        sim_config.convergence_config.arc_length = self.arc_length;
        sim_config.convergence_config.damping_strategy = self.core_damping_strategy();
        sim_config.convergence_config.verbose = self.verbose;
        sim_config.convergence_config.gmin_target = sim_config
            .convergence_config
            .gmin_target
            .min(sim_config.convergence_config.gmin_initial);

        sim_config
    }

    /// Convert to rspice_core SimulationConfig for engine use
    pub fn to_simulation_config(&self) -> rspice_core::engine::SimulationConfig {
        self.resolve_simulation_config(None)
    }

    /// Validate all options
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Tolerances must be positive
        if self.reltol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("reltol", self.reltol));
        }
        if self.residual_reltol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance(
                "residual_reltol",
                self.residual_reltol,
            ));
        }
        if self.vntol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("vntol", self.vntol));
        }
        if self.abstol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("abstol", self.abstol));
        }
        if self.iabstol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("iabstol", self.iabstol));
        }
        if self.chgtol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("chgtol", self.chgtol));
        }

        // Iteration limits must be positive
        if self.itl1 == 0 {
            errors.push(ValidationError::InvalidIteration("itl1", self.itl1));
        }
        if self.itl4 == 0 {
            errors.push(ValidationError::InvalidIteration("itl4", self.itl4));
        }

        // Timestep bounds
        if self.min_timestep <= 0.0 {
            errors.push(ValidationError::InvalidTimestep(
                "min_timestep",
                self.min_timestep,
            ));
        }
        if self.max_timestep <= 0.0 {
            errors.push(ValidationError::InvalidTimestep(
                "max_timestep",
                self.max_timestep,
            ));
        }
        if self.min_timestep >= self.max_timestep {
            errors.push(ValidationError::TimestepOrder(
                self.min_timestep,
                self.max_timestep,
            ));
        }

        // Temperature sanity (must be above absolute zero in Celsius)
        if self.temp < -273.15 {
            errors.push(ValidationError::InvalidTemperature("temp", self.temp));
        }
        if self.tnom < -273.15 {
            errors.push(ValidationError::InvalidTemperature("tnom", self.tnom));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Export as SPICE .options string
    pub fn to_spice_options(&self) -> String {
        let mut lines = vec![".OPTIONS".to_string()];

        // Only output non-default values
        let default = Self::default();

        if (self.reltol - default.reltol).abs() > 1e-15 {
            lines.push(format!("+ RELTOL={:.2e}", self.reltol));
        }
        if (self.residual_reltol - default.residual_reltol).abs() > 1e-15 {
            lines.push(format!("+ RESIDUAL_RELTOL={:.2e}", self.residual_reltol));
        }
        if (self.abstol - default.abstol).abs() > 1e-20 {
            lines.push(format!("+ ABSTOL={:.2e}", self.abstol));
        }
        if (self.vntol - default.vntol).abs() > 1e-12 {
            lines.push(format!("+ VNTOL={:.2e}", self.vntol));
        }
        if self.itl1 != default.itl1 {
            lines.push(format!("+ ITL1={}", self.itl1));
        }
        if self.itl4 != default.itl4 {
            lines.push(format!("+ ITL4={}", self.itl4));
        }
        if self.method != default.method {
            lines.push(format!("+ METHOD={}", self.method.spice_name()));
        }
        if (self.temp - default.temp).abs() > 0.01 {
            lines.push(format!("+ TEMP={:.2}", self.temp));
        }
        if (self.tnom - default.tnom).abs() > 0.01 {
            lines.push(format!("+ TNOM={:.2}", self.tnom));
        }
        if (self.gmin - default.gmin).abs() > 1e-20 {
            lines.push(format!("+ GMIN={:.2e}", self.gmin));
        }

        lines.join("\n")
    }

    /// Parse from SPICE .options lines
    pub fn from_spice_options(text: &str) -> Result<Self, ParseError> {
        let mut opts = Self::default();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('*') {
                continue;
            }

            // Remove .OPTIONS prefix and continuation chars
            let content = line
                .trim_start_matches(".OPTIONS")
                .trim_start_matches(".options")
                .trim_start_matches('+')
                .trim();

            // Parse key=value pairs
            for part in content.split_whitespace() {
                if let Some((key, val)) = part.split_once('=') {
                    match key.to_uppercase().as_str() {
                        "RELTOL" => opts.reltol = parse_si_value(val)?,
                        "RESIDUAL_RELTOL" | "RESRELTOL" => {
                            opts.residual_reltol = parse_si_value(val)?
                        }
                        "ABSTOL" => opts.abstol = parse_si_value(val)?,
                        "VNTOL" => opts.vntol = parse_si_value(val)?,
                        "IABSTOL" => opts.iabstol = parse_si_value(val)?,
                        "CHGTOL" => opts.chgtol = parse_si_value(val)?,
                        "PIVREL" => opts.pivrel = parse_si_value(val)?,
                        "PIVTOL" => opts.pivtol = parse_si_value(val)?,
                        "GMIN" => opts.gmin = parse_si_value(val)?,
                        "ITL1" => {
                            opts.itl1 = val
                                .parse()
                                .map_err(|_| ParseError::InvalidNumber(val.to_string()))?
                        }
                        "ITL2" => {
                            opts.itl2 = val
                                .parse()
                                .map_err(|_| ParseError::InvalidNumber(val.to_string()))?
                        }
                        "ITL4" => {
                            opts.itl4 = val
                                .parse()
                                .map_err(|_| ParseError::InvalidNumber(val.to_string()))?
                        }
                        "METHOD" => {
                            if let Some(m) = IntegrationMethod::from_spice(val) {
                                opts.method = m;
                            }
                        }
                        "TEMP" => opts.temp = parse_si_value(val)?,
                        "TNOM" => opts.tnom = parse_si_value(val)?,
                        _ => {} // Ignore unknown options
                    }
                }
            }
        }

        Ok(opts)
    }
}

/// Validation error types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidTolerance(&'static str, f64),
    InvalidIteration(&'static str, usize),
    InvalidTimestep(&'static str, f64),
    TimestepOrder(f64, f64),
    InvalidTemperature(&'static str, f64),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidTolerance(name, val) => {
                write!(f, "{} must be positive, got {}", name, val)
            }
            ValidationError::InvalidIteration(name, val) => {
                write!(f, "{} must be > 0, got {}", name, val)
            }
            ValidationError::InvalidTimestep(name, val) => {
                write!(f, "{} must be positive, got {}", name, val)
            }
            ValidationError::TimestepOrder(min, max) => {
                write!(f, "min_timestep ({}) must be < max_timestep ({})", min, max)
            }
            ValidationError::InvalidTemperature(name, val) => {
                write!(f, "{} must be > -273.15°C, got {}", name, val)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

//=============================================================================
// UI State for Options Dialog
//=============================================================================

/// UI state for options dialog (string buffers for text editing)
#[derive(Debug, Clone)]
pub struct OptionsDialogState {
    /// Active tab (0=Accuracy, 1=Convergence, 2=Algorithm, 3=Limits, 4=Advanced)
    pub active_tab: usize,

    /// Accuracy tab fields
    pub reltol: String,
    pub residual_reltol: String,
    pub abstol: String,
    pub vntol: String,
    pub iabstol: String,
    pub chgtol: String,

    /// Convergence tab fields
    pub itl1: String,
    pub itl4: String,
    pub gmin: String,
    pub gmin_stepping: bool,
    pub source_stepping: bool,
    pub pseudo_transient: bool,
    pub damping: usize,

    /// Algorithm tab fields
    pub method: usize,
    pub solver: usize,
    pub bypass_enabled: bool,
    pub bypass_reltol: String,
    pub bypass_abstol: String,

    /// Limits tab fields
    pub min_timestep: String,
    pub max_timestep: String,
    pub timestep_factor: String,

    /// Temperature tab fields
    pub temp: String,
    pub tnom: String,
    pub verbose: bool,
    pub save_internals: bool,
}

impl Default for OptionsDialogState {
    fn default() -> Self {
        Self::from_options(&SimulationOptions::default())
    }
}

impl OptionsDialogState {
    /// Create dialog state from options
    pub fn from_options(opts: &SimulationOptions) -> Self {
        Self {
            active_tab: 0,

            reltol: format_si_value(opts.reltol),
            residual_reltol: format_si_value(opts.residual_reltol),
            abstol: format_si_value(opts.abstol),
            vntol: format_si_value(opts.vntol),
            iabstol: format_si_value(opts.iabstol),
            chgtol: format_si_value(opts.chgtol),

            itl1: opts.itl1.to_string(),
            itl4: opts.itl4.to_string(),
            gmin: format_si_value(opts.gmin),
            gmin_stepping: opts.gmin_stepping,
            source_stepping: opts.source_stepping,
            pseudo_transient: opts.pseudo_transient,
            damping: DampingStrategy::all()
                .iter()
                .position(|d| *d == opts.damping)
                .unwrap_or(0),

            method: IntegrationMethod::all()
                .iter()
                .position(|m| *m == opts.method)
                .unwrap_or(0),
            solver: MatrixSolver::all()
                .iter()
                .position(|s| *s == opts.solver)
                .unwrap_or(0),
            bypass_enabled: opts.bypass_enabled,
            bypass_reltol: format_si_value(opts.bypass_reltol),
            bypass_abstol: format_si_value(opts.bypass_abstol),

            min_timestep: format_si_value(opts.min_timestep),
            max_timestep: format_si_value(opts.max_timestep),
            timestep_factor: opts.timestep_factor.to_string(),

            temp: format!("{:.1}", opts.temp),
            tnom: format!("{:.1}", opts.tnom),
            verbose: opts.verbose,
            save_internals: opts.save_internals,
        }
    }

    /// Convert dialog state to options (with validation)
    pub fn to_options(&self) -> Result<SimulationOptions, Vec<String>> {
        let mut errors = Vec::new();

        let reltol = parse_si_value(&self.reltol).unwrap_or_else(|e| {
            errors.push(format!("reltol: {}", e));
            1e-3
        });
        let residual_reltol = parse_si_value(&self.residual_reltol).unwrap_or_else(|e| {
            errors.push(format!("residual_reltol: {}", e));
            1e-3
        });
        let abstol = parse_si_value(&self.abstol).unwrap_or_else(|e| {
            errors.push(format!("abstol: {}", e));
            1e-12
        });
        let vntol = parse_si_value(&self.vntol).unwrap_or_else(|e| {
            errors.push(format!("vntol: {}", e));
            1e-6
        });
        let iabstol = parse_si_value(&self.iabstol).unwrap_or_else(|e| {
            errors.push(format!("iabstol: {}", e));
            1e-12
        });
        let chgtol = parse_si_value(&self.chgtol).unwrap_or_else(|e| {
            errors.push(format!("chgtol: {}", e));
            1e-14
        });
        let gmin = parse_si_value(&self.gmin).unwrap_or_else(|e| {
            errors.push(format!("gmin: {}", e));
            1e-12
        });
        let min_timestep = parse_si_value(&self.min_timestep).unwrap_or_else(|e| {
            errors.push(format!("min_timestep: {}", e));
            1e-15
        });
        let max_timestep = parse_si_value(&self.max_timestep).unwrap_or_else(|e| {
            errors.push(format!("max_timestep: {}", e));
            1e-3
        });
        let bypass_reltol = parse_si_value(&self.bypass_reltol).unwrap_or_else(|e| {
            errors.push(format!("bypass_reltol: {}", e));
            1e-3
        });
        let bypass_abstol = parse_si_value(&self.bypass_abstol).unwrap_or_else(|e| {
            errors.push(format!("bypass_abstol: {}", e));
            1e-6
        });

        let itl1 = self.itl1.parse().unwrap_or_else(|_| {
            errors.push("itl1: invalid integer".to_string());
            50
        });
        let itl4 = self.itl4.parse().unwrap_or_else(|_| {
            errors.push("itl4: invalid integer".to_string());
            6
        });
        let timestep_factor = self.timestep_factor.parse().unwrap_or_else(|_| {
            errors.push("timestep_factor: invalid number".to_string());
            8.0
        });
        let temp = self.temp.parse().unwrap_or_else(|_| {
            errors.push("temp: invalid number".to_string());
            27.0
        });
        let tnom = self.tnom.parse().unwrap_or_else(|_| {
            errors.push("tnom: invalid number".to_string());
            27.0
        });

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(SimulationOptions {
            reltol,
            residual_reltol,
            abstol,
            vntol,
            iabstol,
            chgtol,
            pivrel: 1e-3,
            pivtol: 1e-13,
            itl1,
            itl2: 100,
            itl4,
            gmin_stepping: self.gmin_stepping,
            source_stepping: self.source_stepping,
            pseudo_transient: self.pseudo_transient,
            arc_length: false,
            gmin,
            damping: DampingStrategy::all()[self.damping.min(DampingStrategy::all().len() - 1)],
            method: IntegrationMethod::all()[self.method.min(IntegrationMethod::all().len() - 1)],
            solver: MatrixSolver::all()[self.solver.min(MatrixSolver::all().len() - 1)],
            bypass_enabled: self.bypass_enabled,
            bypass_reltol,
            bypass_abstol,
            min_timestep,
            max_timestep,
            timestep_factor,
            temp,
            tnom,
            verbose: self.verbose,
            save_internals: self.save_internals,
        })
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    //=========================================================================
    // SI Prefix Parsing Tests
    //=========================================================================

    #[test]
    fn test_parse_si_basic_numbers() {
        assert!((parse_si_value("1").unwrap() - 1.0).abs() < 1e-15);
        assert!((parse_si_value("1.5").unwrap() - 1.5).abs() < 1e-15);
        assert!((parse_si_value("-2.5").unwrap() - -2.5).abs() < 1e-15);
        assert!((parse_si_value("1e-9").unwrap() - 1e-9).abs() < 1e-20);
        assert!((parse_si_value("1.5e6").unwrap() - 1.5e6).abs() < 1.0);
    }

    #[test]
    fn test_parse_si_prefixes() {
        assert!((parse_si_value("1T").unwrap() - 1e12).abs() < 1e6);
        assert!((parse_si_value("1G").unwrap() - 1e9).abs() < 1e3);
        assert!((parse_si_value("1Meg").unwrap() - 1e6).abs() < 1.0);
        assert!((parse_si_value("1k").unwrap() - 1e3).abs() < 1e-3);
        assert!((parse_si_value("1m").unwrap() - 1e-3).abs() < 1e-9);
        assert!((parse_si_value("1u").unwrap() - 1e-6).abs() < 1e-12);
        assert!((parse_si_value("1n").unwrap() - 1e-9).abs() < 1e-15);
        assert!((parse_si_value("1p").unwrap() - 1e-12).abs() < 1e-18);
        assert!((parse_si_value("1f").unwrap() - 1e-15).abs() < 1e-21);
        assert!((parse_si_value("1a").unwrap() - 1e-18).abs() < 1e-24);
    }

    #[test]
    fn test_parse_si_with_spaces() {
        assert!((parse_si_value("  1.5u  ").unwrap() - 1.5e-6).abs() < 1e-12);
    }

    #[test]
    fn test_parse_si_errors() {
        assert!(matches!(parse_si_value(""), Err(ParseError::Empty)));
        assert!(matches!(
            parse_si_value("abc"),
            Err(ParseError::NoNumericPart)
        ));
        assert!(matches!(
            parse_si_value("1xyz"),
            Err(ParseError::UnknownSuffix(_))
        ));
    }

    #[test]
    fn test_format_si_roundtrip() {
        let values = [1e-15, 1e-12, 1e-9, 1e-6, 1e-3, 1.0, 1e3, 1e6, 1e9, 1e12];
        for v in values {
            let formatted = format_si_value(v);
            let parsed = parse_si_value(&formatted).unwrap();
            let diff = (parsed - v).abs() / v.abs().max(1e-20);
            assert!(
                diff < 1e-6,
                "Roundtrip failed for {}: formatted as '{}', parsed as {}",
                v,
                formatted,
                parsed
            );
        }
    }

    //=========================================================================
    // Integration Method Tests
    //=========================================================================

    #[test]
    fn test_integration_method_roundtrip() {
        for method in IntegrationMethod::all() {
            let spice = method.spice_name();
            let parsed = IntegrationMethod::from_spice(spice);
            assert_eq!(Some(*method), parsed, "Roundtrip failed for {:?}", method);
        }
    }

    #[test]
    fn test_integration_method_aliases() {
        assert_eq!(
            IntegrationMethod::from_spice("TRAPEZOIDAL"),
            Some(IntegrationMethod::Trap)
        );
        assert_eq!(
            IntegrationMethod::from_spice("BE"),
            Some(IntegrationMethod::Euler)
        );
        assert_eq!(
            IntegrationMethod::from_spice("BDF"),
            Some(IntegrationMethod::Gear)
        );
        assert_eq!(
            IntegrationMethod::from_spice("AUTO"),
            Some(IntegrationMethod::TrapGear)
        );
    }

    #[test]
    fn test_integration_method_case_insensitive() {
        assert_eq!(
            IntegrationMethod::from_spice("trap"),
            Some(IntegrationMethod::Trap)
        );
        assert_eq!(
            IntegrationMethod::from_spice("TRAP"),
            Some(IntegrationMethod::Trap)
        );
        assert_eq!(
            IntegrationMethod::from_spice("Trap"),
            Some(IntegrationMethod::Trap)
        );
    }

    //=========================================================================
    // Simulation Options Default Tests
    //=========================================================================

    #[test]
    fn test_options_default_matches_spice() {
        let opts = SimulationOptions::default();

        // SPICE3 default tolerances
        assert!((opts.reltol - 1e-3).abs() < 1e-10);
        assert!((opts.residual_reltol - 1e-3).abs() < 1e-10);
        assert!((opts.abstol - 1e-12).abs() < 1e-20);
        assert!((opts.vntol - 1e-6).abs() < 1e-12);

        // Default iterations
        assert_eq!(opts.itl1, 50);
        assert_eq!(opts.itl4, 6);

        // Temperature
        assert!((opts.temp - 27.0).abs() < 0.01);
    }

    #[test]
    fn test_options_fast_preset() {
        let opts = SimulationOptions::fast();

        // Fast = looser tolerances
        assert!(opts.reltol > SimulationOptions::default().reltol);
        assert!(!opts.gmin_stepping);
        assert!(opts.bypass_enabled);
    }

    #[test]
    fn test_options_accurate_preset() {
        let opts = SimulationOptions::accurate();

        // Accurate = tighter tolerances
        assert!(opts.reltol < SimulationOptions::default().reltol);
        assert!(opts.itl1 > SimulationOptions::default().itl1);
    }

    #[test]
    fn test_options_robust_preset() {
        let opts = SimulationOptions::robust();

        // Robust = all convergence aids enabled
        assert!(opts.gmin_stepping);
        assert!(opts.source_stepping);
        assert!(opts.pseudo_transient);
        assert!(opts.arc_length);
        assert_eq!(opts.damping, DampingStrategy::Combined);
    }

    #[test]
    fn test_to_simulation_config_maps_convergence_tolerances() {
        let mut opts = SimulationOptions::default();
        opts.reltol = 2e-3;
        opts.residual_reltol = 6e-4;
        opts.vntol = 3e-6;
        opts.iabstol = 4e-12;
        opts.chgtol = 7e-15;
        opts.gmin = 5e-10;

        let sim = opts.to_simulation_config();
        assert!((sim.tolerance - 2e-3).abs() < 1e-15);
        assert!((sim.convergence_config.voltage_reltol - 2e-3).abs() < 1e-15);
        assert!((sim.convergence_config.residual_reltol - 6e-4).abs() < 1e-15);
        assert!((sim.convergence_config.voltage_abstol - 3e-6).abs() < 1e-15);
        assert!((sim.convergence_config.current_abstol - 4e-12).abs() < 1e-24);
        assert!((sim.convergence_config.charge_abstol - 7e-15).abs() < 1e-27);
        assert!((sim.convergence_config.gmin_initial - 5e-10).abs() < 1e-21);
    }

    #[test]
    fn test_to_simulation_config_clamps_gmin_target_to_initial() {
        let mut opts = SimulationOptions::default();
        opts.gmin = 1e-16;

        let sim = opts.to_simulation_config();
        assert!((sim.convergence_config.gmin_initial - 1e-16).abs() < 1e-28);
        assert!((sim.convergence_config.gmin_target - 1e-16).abs() < 1e-28);
    }

    #[test]
    fn test_resolve_simulation_config_ui_chgtol_overrides_netlist() {
        let mut opts = SimulationOptions::default();
        opts.chgtol = 2e-14;

        let netlist_options = rspice_core::netlist::SimulationOptions {
            chgtol: Some(9e-15),
            ..Default::default()
        };

        let sim = opts.resolve_simulation_config(Some(&netlist_options));
        assert!((sim.convergence_config.charge_abstol - 2e-14).abs() < 1e-27);
    }

    //=========================================================================
    // Temperature Conversion Tests
    //=========================================================================

    #[test]
    fn test_temp_kelvin_conversion() {
        let opts = SimulationOptions::default();
        assert!((opts.temp_kelvin() - 300.15).abs() < 0.01);
    }

    #[test]
    fn test_temp_kelvin_at_zero_celsius() {
        let mut opts = SimulationOptions::default();
        opts.temp = 0.0;
        assert!((opts.temp_kelvin() - 273.15).abs() < 0.01);
    }

    //=========================================================================
    // Validation Tests
    //=========================================================================

    #[test]
    fn test_validate_default_passes() {
        let opts = SimulationOptions::default();
        assert!(opts.validate().is_ok());
    }

    #[test]
    fn test_validate_negative_tolerance_fails() {
        let mut opts = SimulationOptions::default();
        opts.residual_reltol = -1e-3;
        let result = opts.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::InvalidTolerance("residual_reltol", _)))
        );
    }

    #[test]
    fn test_validate_zero_tolerance_fails() {
        let mut opts = SimulationOptions::default();
        opts.abstol = 0.0;
        let result = opts.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_zero_iterations_fails() {
        let mut opts = SimulationOptions::default();
        opts.itl1 = 0;
        let result = opts.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::InvalidIteration("itl1", _)))
        );
    }

    #[test]
    fn test_validate_timestep_order_fails() {
        let mut opts = SimulationOptions::default();
        opts.min_timestep = 1e-3;
        opts.max_timestep = 1e-6;
        let result = opts.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::TimestepOrder(_, _)))
        );
    }

    #[test]
    fn test_validate_absolute_zero_fails() {
        let mut opts = SimulationOptions::default();
        opts.temp = -300.0; // Below absolute zero
        let result = opts.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_collects_multiple_errors() {
        let mut opts = SimulationOptions::default();
        opts.reltol = -1.0;
        opts.abstol = 0.0;
        opts.itl1 = 0;
        let result = opts.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 3);
    }

    //=========================================================================
    // SPICE Export/Import Tests
    //=========================================================================

    #[test]
    fn test_spice_export_default_minimal() {
        let opts = SimulationOptions::default();
        let spice = opts.to_spice_options();
        // Default options should produce minimal output
        assert!(spice.starts_with(".OPTIONS"));
        // Should not contain RELTOL since it's default
        assert!(!spice.contains("RELTOL"));
    }

    #[test]
    fn test_spice_export_non_default() {
        let mut opts = SimulationOptions::default();
        opts.reltol = 1e-4;
        opts.residual_reltol = 2e-4;
        opts.temp = 85.0;
        opts.itl1 = 100;
        let spice = opts.to_spice_options();

        assert!(spice.contains("RELTOL="));
        assert!(spice.contains("RESIDUAL_RELTOL="));
        assert!(spice.contains("TEMP=85"));
        assert!(spice.contains("ITL1=100"));
    }

    #[test]
    fn test_spice_import_basic() {
        let text = ".OPTIONS RELTOL=1e-4 RESIDUAL_RELTOL=2e-4 ABSTOL=1e-14 ITL1=100 TEMP=85";
        let opts = SimulationOptions::from_spice_options(text).unwrap();

        assert!((opts.reltol - 1e-4).abs() < 1e-10);
        assert!((opts.residual_reltol - 2e-4).abs() < 1e-10);
        assert!((opts.abstol - 1e-14).abs() < 1e-20);
        assert_eq!(opts.itl1, 100);
        assert!((opts.temp - 85.0).abs() < 0.1);
    }

    #[test]
    fn test_spice_import_multiline() {
        let text = r#"
.OPTIONS
+ RELTOL=1e-4
+ ABSTOL=1e-14
+ METHOD=GEAR
"#;
        let opts = SimulationOptions::from_spice_options(text).unwrap();
        assert!((opts.reltol - 1e-4).abs() < 1e-10);
        assert_eq!(opts.method, IntegrationMethod::Gear);
    }

    #[test]
    fn test_spice_import_with_si_prefixes() {
        let text = ".OPTIONS ABSTOL=1p VNTOL=1u GMIN=1n";
        let opts = SimulationOptions::from_spice_options(text).unwrap();

        assert!((opts.abstol - 1e-12).abs() < 1e-18);
        assert!((opts.vntol - 1e-6).abs() < 1e-12);
        assert!((opts.gmin - 1e-9).abs() < 1e-15);
    }

    #[test]
    fn test_spice_roundtrip() {
        let mut original = SimulationOptions::default();
        original.reltol = 1e-4;
        original.residual_reltol = 3e-4;
        original.abstol = 1e-14;
        original.itl1 = 100;
        original.temp = 85.0;
        original.method = IntegrationMethod::Gear;

        let spice = original.to_spice_options();
        let parsed = SimulationOptions::from_spice_options(&spice).unwrap();

        assert!((parsed.reltol - original.reltol).abs() < 1e-10);
        assert!((parsed.residual_reltol - original.residual_reltol).abs() < 1e-10);
        assert!(parsed.itl1 == original.itl1);
        assert!((parsed.temp - original.temp).abs() < 0.01);
        assert_eq!(parsed.method, original.method);
    }

    //=========================================================================
    // Dialog State Tests
    //=========================================================================

    #[test]
    fn test_dialog_state_from_options() {
        let opts = SimulationOptions::default();
        let state = OptionsDialogState::from_options(&opts);

        // Check that strings are non-empty
        assert!(!state.reltol.is_empty());
        assert!(!state.residual_reltol.is_empty());
        assert!(!state.abstol.is_empty());
        assert!(!state.itl1.is_empty());
    }

    #[test]
    fn test_dialog_state_to_options() {
        let state = OptionsDialogState::default();
        let result = state.to_options();
        assert!(result.is_ok());

        let opts = result.unwrap();
        assert!(opts.validate().is_ok());
    }

    #[test]
    fn test_dialog_state_roundtrip() {
        let original = SimulationOptions::accurate();
        let state = OptionsDialogState::from_options(&original);
        let result = state.to_options().unwrap();

        // Key values should roundtrip
        assert!((result.reltol - original.reltol).abs() / original.reltol < 0.01);
        assert!(
            (result.residual_reltol - original.residual_reltol).abs() / original.residual_reltol
                < 0.01
        );
        assert_eq!(result.itl1, original.itl1);
    }

    #[test]
    fn test_dialog_state_invalid_input() {
        let mut state = OptionsDialogState::default();
        state.reltol = "invalid".to_string();

        let result = state.to_options();
        assert!(result.is_err());
    }

    //=========================================================================
    // Edge Case Tests
    //=========================================================================

    #[test]
    fn test_extreme_tolerance_values() {
        let mut opts = SimulationOptions::default();
        opts.reltol = 1e-15; // Very tight
        opts.abstol = 1e-30; // Extremely tight
        assert!(opts.validate().is_ok());
    }

    #[test]
    fn test_extreme_iteration_limits() {
        let mut opts = SimulationOptions::default();
        opts.itl1 = 10000; // Very high
        assert!(opts.validate().is_ok());
    }

    #[test]
    fn test_cryogenic_temperature() {
        let mut opts = SimulationOptions::default();
        opts.temp = -200.0; // Cryogenic but valid
        assert!(opts.validate().is_ok());
        assert!((opts.temp_kelvin() - 73.15).abs() < 0.01);
    }

    #[test]
    fn test_high_temperature() {
        let mut opts = SimulationOptions::default();
        opts.temp = 175.0; // Automotive high temp
        assert!(opts.validate().is_ok());
    }
}
