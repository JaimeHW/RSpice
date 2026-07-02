//! Engine configuration types.

use crate::Value;

/// Broad SPICE compatibility policy for internal device-model selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpiceDialect {
    /// Prefer RSpice's most accurate available evaluator per device.
    #[default]
    BestAvailable,
    /// Prefer ngspice-compatible evaluators where RSpice carries variants.
    Ngspice,
    /// Prefer Xyce-compatible evaluators where RSpice carries variants.
    Xyce,
}

impl SpiceDialect {
    /// Default evaluator for `NJF`/`PJF LEVEL=2` under this dialect.
    pub fn default_jfet_level2_model(self) -> JfetLevel2Model {
        match self {
            Self::BestAvailable | Self::Ngspice => JfetLevel2Model::ParkerSkellern,
            Self::Xyce => JfetLevel2Model::XyceModifiedShockley,
        }
    }
}

/// Internal evaluator selection used for `NJF`/`PJF LEVEL=2` model cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JfetLevel2Model {
    /// Defer the concrete evaluator to [`SpiceDialect`].
    #[default]
    DialectDefault,
    /// ngspice JFET2 Parker-Skellern short-channel model.
    ParkerSkellern,
    /// Xyce modified-Shockley level-2 JFET model.
    XyceModifiedShockley,
}

/// Simulation configuration
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    /// Convergence tolerance for Newton-Raphson
    pub tolerance: Value,
    /// Maximum Newton-Raphson iterations
    pub max_iterations: usize,
    /// Maximum Newton-Raphson iterations per transient timestep.
    ///
    /// This is the transient-specific SPICE `ITL4` limit. It is intentionally
    /// separate from the DC operating-point iteration budget.
    pub transient_max_iterations: usize,
    /// Preferred minimum timestep for transient analysis.
    ///
    /// The transient engine may temporarily shrink below this during nonlinear
    /// recovery, but will bias accepted smooth regions back above it.
    pub min_timestep: Value,
    /// Maximum timestep for transient analysis
    pub max_timestep: Value,
    /// Temperature in Kelvin
    pub temperature: Value,
    /// Transient source/code-model ramping time in seconds. A value <= 0 disables it.
    pub ramptime: Value,
    /// Ngspice XSPICE digital delay policy:
    /// 0 = default transport, 1 = default inertial,
    /// 2 = force transport, 3 = force inertial.
    pub digital_delay_type: Option<i64>,
    /// Integration method for transient analysis
    pub integration_method: crate::analysis::IntegrationMethod,
    /// Broad SPICE compatibility policy used by config resolution.
    pub spice_dialect: SpiceDialect,
    /// Internal evaluator used for `NJF`/`PJF LEVEL=2`.
    pub jfet_level2_model: JfetLevel2Model,
    /// Xyce BSIMSOI3 terminal-GMIN policy. When true, B3SOI devices receive
    /// `GMIN * 1e-6`; when false, they receive the full device GMIN.
    pub b3soi_gmin_scaling: bool,
    /// Transient truncation tolerance factor for charge-state timestep control.
    pub transient_trtol: Value,
    /// Largest nonlinear-device terminal-voltage change allowed per accepted
    /// transient step (volts). Signal-activity step control complementing the
    /// polynomial charge LTE; see `constants::DEVICE_ACTIVITY_STEP_BOUND`.
    pub transient_node_activity_bound: Value,
    /// Model evaluation bypass configuration for latent device optimization
    pub bypass_config: BypassConfig,
    /// Convergence configuration for DC operating point
    pub convergence_config: ConvergenceConfig,
    /// Grid-locked transient stepping: when set, accepted timepoints are
    /// exactly these strictly-increasing times — the dt sequence is the
    /// successive grid deltas, with no intermediate adaptive points, no
    /// breakpoint-restart re-seeding, and LTE control disabled (the grid is
    /// given). Newton-only acceptance per step; a step that cannot converge
    /// on its imposed dt fails the run rather than sub-stepping, because
    /// history-coupled devices (TXL/CPL/LTRA convolutions) sample accepted
    /// points and internal sub-steps would change the trajectory being
    /// validated. Built for oracle comparison: replaying a reference's
    /// recorded grid isolates physics parity from step-control parity.
    pub locked_time_grid: Option<std::sync::Arc<Vec<Value>>>,
}

impl SimulationConfig {
    /// Select a broad compatibility dialect and use its device defaults.
    pub fn with_spice_dialect(mut self, dialect: SpiceDialect) -> Self {
        self.spice_dialect = dialect;
        self.apply_spice_dialect();
        self
    }

    /// Reset dialect-controlled device selections to follow the current dialect.
    pub fn apply_spice_dialect(&mut self) {
        self.jfet_level2_model = JfetLevel2Model::DialectDefault;
    }

    /// Resolve the concrete evaluator for `NJF`/`PJF LEVEL=2`.
    pub fn resolved_jfet_level2_model(&self) -> JfetLevel2Model {
        match self.jfet_level2_model {
            JfetLevel2Model::DialectDefault => self.spice_dialect.default_jfet_level2_model(),
            model => model,
        }
    }
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
    /// Initial continuation GMIN value for stepping.
    pub gmin_initial: Value,
    /// Final global nodal diagonal floor used only for numerical conditioning.
    pub gmin_target: Value,
    /// Final SPICE device-junction GMIN floor seen by compact models.
    pub junction_gmin_target: Value,
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
            junction_gmin_target: crate::constants::GMIN,
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
            transient_max_iterations: crate::constants::MAX_TRANSIENT_NR_ITERATIONS,
            min_timestep: crate::constants::MIN_TIMESTEP,
            max_timestep: crate::constants::MAX_TIMESTEP,
            temperature: crate::constants::TEMP_REFERENCE,
            ramptime: 0.0,
            digital_delay_type: None,
            integration_method: crate::analysis::IntegrationMethod::TrapGear,
            spice_dialect: SpiceDialect::BestAvailable,
            jfet_level2_model: JfetLevel2Model::DialectDefault,
            b3soi_gmin_scaling: true,
            transient_trtol: crate::constants::TRTOL,
            transient_node_activity_bound: crate::constants::DEVICE_ACTIVITY_STEP_BOUND,
            bypass_config: BypassConfig::default(),
            convergence_config: ConvergenceConfig::default(),
            locked_time_grid: None,
        }
    }
}
