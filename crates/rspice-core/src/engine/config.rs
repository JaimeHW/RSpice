//! Engine configuration types.

use crate::Value;
/// Simulation configuration
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    /// Convergence tolerance for Newton-Raphson
    pub tolerance: Value,
    /// Maximum Newton-Raphson iterations
    pub max_iterations: usize,
    /// Preferred minimum timestep for transient analysis.
    ///
    /// The transient engine may temporarily shrink below this during nonlinear
    /// recovery, but will bias accepted smooth regions back above it.
    pub min_timestep: Value,
    /// Maximum timestep for transient analysis
    pub max_timestep: Value,
    /// Temperature in Kelvin
    pub temperature: Value,
    /// Integration method for transient analysis
    pub integration_method: crate::analysis::IntegrationMethod,
    /// Model evaluation bypass configuration for latent device optimization
    pub bypass_config: BypassConfig,
    /// Convergence configuration for DC operating point
    pub convergence_config: ConvergenceConfig,
}

/// Configuration for DC convergence algorithms
///
/// Controls which convergence aids are used when Newton-Raphson fails to converge
/// directly on difficult circuits. Methods are tried in order of increasing
/// computational cost.
#[derive(Debug, Clone)]
pub struct ConvergenceConfig {
    /// Enable GMIN stepping (small conductances to ground)
    pub gmin_stepping: bool,
    /// Enable source stepping (ramp sources from 0 to 100%)
    pub source_stepping: bool,
    /// Enable pseudo-transient continuation
    pub pseudo_transient: bool,
    /// Enable arc-length continuation for non-monotonic curves
    pub arc_length: bool,
    /// Damping strategy for Newton iterations
    pub damping_strategy: DampingStrategy,
    /// Initial GMIN value (typically 1e-12)
    pub gmin_initial: Value,
    /// Target GMIN value (typically 1e-15)
    pub gmin_target: Value,
    /// Relative voltage tolerance for Newton voltage convergence checks.
    pub voltage_reltol: Value,
    /// Absolute voltage tolerance for Newton voltage convergence checks.
    ///
    /// A non-positive value keeps legacy behavior and falls back to
    /// `SimulationConfig::tolerance`.
    pub voltage_abstol: Value,
    /// Absolute current tolerance for equation residual convergence checks.
    ///
    /// This is used when normalizing `A*x-b` residuals in Newton acceptance.
    /// A non-positive value falls back to a conservative 1e-12 default.
    pub current_abstol: Value,
    /// Absolute charge tolerance for transient truncation on charge-state devices.
    ///
    /// This mirrors SPICE `CHGTOL` semantics for compact models whose timestep
    /// control is driven by integrated charge states instead of node voltages.
    /// A non-positive value falls back to a conservative 1e-14 default.
    pub charge_abstol: Value,
    /// Relative tolerance for equation residual convergence checks.
    ///
    /// This controls normalized `A*x-b` acceptance independently from
    /// voltage-step convergence (`voltage_reltol`).
    pub residual_reltol: Value,
    /// Verbose convergence logging
    pub verbose: bool,
}

/// Damping strategy for Newton-Raphson iterations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DampingStrategy {
    /// No damping (full Newton step)
    #[default]
    None,
    /// Backtracking line search (Armijo condition)
    LineSearch,
    /// Junction voltage limiting (SPICE-style)
    VoltageLimiting,
    /// Bank-Rose adaptive damping
    BankRose,
    /// Combined: voltage limiting + line search
    Combined,
}

impl Default for ConvergenceConfig {
    fn default() -> Self {
        Self {
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: false, // Only for difficult circuits
            damping_strategy: DampingStrategy::VoltageLimiting,
            gmin_initial: 1e-12,
            gmin_target: 1e-15,
            voltage_reltol: crate::constants::RELTOL,
            voltage_abstol: 0.0,
            current_abstol: crate::constants::ITOL,
            charge_abstol: crate::constants::CHGTOL,
            residual_reltol: crate::constants::RELTOL,
            verbose: false,
        }
    }
}

impl ConvergenceConfig {
    /// Create a minimal config (direct Newton only)
    pub fn fast() -> Self {
        Self {
            gmin_stepping: false,
            source_stepping: false,
            pseudo_transient: false,
            arc_length: false,
            damping_strategy: DampingStrategy::None,
            ..Default::default()
        }
    }

    /// Create a robust config (all methods enabled)
    pub fn robust() -> Self {
        Self {
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: true,
            damping_strategy: DampingStrategy::Combined,
            verbose: false,
            ..Default::default()
        }
    }

    /// Enable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Set damping strategy
    pub fn with_damping(mut self, strategy: DampingStrategy) -> Self {
        self.damping_strategy = strategy;
        self
    }

    /// Set Newton voltage convergence tolerances.
    pub fn with_voltage_tolerances(mut self, reltol: Value, abstol: Value) -> Self {
        self.voltage_reltol = reltol;
        self.voltage_abstol = abstol;
        self
    }

    /// Set absolute current tolerance used for residual convergence checks.
    pub fn with_current_tolerance(mut self, abstol: Value) -> Self {
        self.current_abstol = abstol;
        self
    }

    /// Set relative residual tolerance used for equation residual checks.
    pub fn with_residual_reltol(mut self, reltol: Value) -> Self {
        self.residual_reltol = reltol;
        self
    }
}

/// Configuration for model evaluation bypass (latent device optimization)
#[derive(Debug, Clone)]
pub struct BypassConfig {
    /// Enable bypass optimization (default: false for stability)
    pub enabled: bool,
    /// Relative voltage tolerance for bypass detection
    pub reltol: Value,
    /// Absolute voltage tolerance for bypass detection  
    pub abstol: Value,
}

impl Default for BypassConfig {
    fn default() -> Self {
        Self {
            enabled: false,                   // Conservative default - must be explicitly enabled
            reltol: crate::constants::RELTOL, // 0.1% relative change threshold
            abstol: crate::constants::VNTOL,  // 1uV absolute change threshold
        }
    }
}

impl BypassConfig {
    /// Create bypass config with optimization enabled
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Create bypass config with custom tolerances
    pub fn with_tolerances(reltol: Value, abstol: Value) -> Self {
        Self {
            enabled: true,
            reltol,
            abstol,
        }
    }
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            tolerance: crate::constants::DEFAULT_TOLERANCE,
            max_iterations: crate::constants::MAX_NR_ITERATIONS,
            min_timestep: crate::constants::MIN_TIMESTEP,
            max_timestep: crate::constants::MAX_TIMESTEP,
            temperature: crate::constants::TEMP_REFERENCE,
            integration_method: crate::analysis::IntegrationMethod::TrapGear,
            bypass_config: BypassConfig::default(),
            convergence_config: ConvergenceConfig::default(),
        }
    }
}
