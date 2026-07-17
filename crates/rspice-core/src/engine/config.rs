//! Engine configuration types.

use crate::Value;
use crate::netlist::{NonlinearContinuationMode, TransientLteReference};
use thiserror::Error;

/// A violated invariant in [`SimulationConfig`].
///
/// The fields are intentionally structured so service and language bindings
/// can report invalid configuration without parsing display strings.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum SimulationConfigError {
    /// A floating-point field was not finite or did not satisfy its lower bound.
    #[error("{field} must be {requirement}, got {value}")]
    InvalidValue {
        /// Configuration field name.
        field: &'static str,
        /// Rejected value.
        value: Value,
        /// Stable human-readable constraint.
        requirement: &'static str,
    },
    /// An iteration or similar count was zero.
    #[error("{field} must be greater than zero, got {value}")]
    InvalidCount {
        /// Configuration field name.
        field: &'static str,
        /// Rejected value.
        value: usize,
    },
    /// The configured transient timestep bounds were reversed.
    #[error(
        "min_timestep ({min_timestep}) must be less than or equal to max_timestep ({max_timestep})"
    )]
    InvalidTimestepRange {
        /// Configured minimum timestep.
        min_timestep: Value,
        /// Configured maximum timestep.
        max_timestep: Value,
    },
    /// The explicitly requested first transient step exceeds the hard maximum.
    #[error(
        "transient_initial_timestep ({initial_timestep}) must be less than or equal to max_timestep ({max_timestep})"
    )]
    InitialTimestepExceedsMaximum {
        /// Configured initial timestep.
        initial_timestep: Value,
        /// Configured maximum timestep.
        max_timestep: Value,
    },
    /// The XSPICE digital delay selector was outside its documented domain.
    #[error("digital_delay_type must be an integer from 0 through 3, got {0}")]
    InvalidDigitalDelayType(i64),
    /// A prescribed transient grid contained a non-finite or negative point.
    #[error("locked_time_grid[{index}] must be a finite, non-negative time, got {value}")]
    InvalidLockedTimeGridPoint {
        /// Zero-based point index.
        index: usize,
        /// Rejected time value.
        value: Value,
    },
    /// A prescribed transient grid was not strictly increasing.
    #[error(
        "locked_time_grid must be strictly increasing; point {index} ({value}) is not greater than point {previous_index} ({previous})"
    )]
    NonIncreasingLockedTimeGrid {
        /// Zero-based index of the rejected point.
        index: usize,
        /// Rejected time value.
        value: Value,
        /// Zero-based index of the preceding point.
        previous_index: usize,
        /// Preceding time value.
        previous: Value,
    },
}

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

    /// Default transient LTE reference policy for this compatibility dialect.
    pub fn default_transient_lte_reference(self) -> TransientLteReference {
        match self {
            Self::Xyce => TransientLteReference::PointGlobal,
            Self::BestAvailable | Self::Ngspice => TransientLteReference::PredictorLocal,
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
    /// Optional explicit first transient timestep.
    ///
    /// When unset, the engine chooses its startup step from its normal
    /// compatibility policy. Xyce oracle runs set this from the `.TRAN`
    /// starting step because Xyce's time integrator uses that value directly
    /// for the first accepted transient point.
    pub transient_initial_timestep: Option<Value>,
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
    /// Explicit transient LTE relative tolerance. `None` uses Xyce's independent
    /// TIMEINT default in Xyce mode and voltage RELTOL in native/ngspice modes.
    pub transient_lte_reltol: Option<Value>,
    /// Explicit transient LTE absolute tolerance. `None` uses Xyce's independent
    /// TIMEINT default in Xyce mode and voltage ABSTOL in native/ngspice modes.
    pub transient_lte_abstol: Option<Value>,
    /// Reference magnitude policy for normalized transient LTE control.
    /// `None` selects the active [`SpiceDialect`]'s default policy.
    pub transient_lte_reference: Option<TransientLteReference>,
    /// Xyce `NEWBPSTEPPING` policy.
    ///
    /// When false, the first integration step after a breakpoint remains an
    /// order-one restart step but bypasses LTE rejection. The Xyce 7.10
    /// default is true, which tests LTE at noninitial breakpoint restarts.
    pub transient_new_bp_stepping: bool,
    /// Largest nonlinear-device terminal-voltage change allowed per accepted
    /// transient step (volts). Signal-activity step control complementing the
    /// polynomial charge LTE; see `constants::DEVICE_ACTIVITY_STEP_BOUND`.
    pub transient_node_activity_bound: Value,
    /// Model evaluation bypass configuration for latent device optimization
    pub bypass_config: BypassConfig,
    /// Convergence configuration for DC operating point
    pub convergence_config: ConvergenceConfig,
    /// Grid-locked transient stepping: every strictly increasing configured
    /// time is visited exactly, and adaptive LTE rejection cannot insert
    /// points. Source breakpoints and pending XSPICE events may still split an
    /// interval when the supplied grid omitted a required event time; those
    /// accepted event points are recorded as well. Accepted-reference modes
    /// restart integration history at source breakpoints and continue LTE
    /// evaluation for integration-order selection, but the estimate cannot
    /// reject or resize a prescribed step. Newton convergence is the
    /// acceptance authority; a step
    /// that cannot converge on its imposed interval fails rather than
    /// convergence-substepping, because history-coupled devices
    /// (TXL/CPL/LTRA convolutions) sample accepted points. Built for oracle
    /// comparison: replaying a reference grid isolates physics parity from
    /// adaptive step-selection parity.
    pub locked_time_grid: Option<std::sync::Arc<Vec<Value>>>,
}

impl SimulationConfig {
    /// Validate every engine-level configuration invariant.
    ///
    /// Frontends that accept configuration from files, users, or remote jobs
    /// should call this directly or construct the engine with
    /// [`crate::Engine::try_new`] before starting simulation work.
    pub fn validate(&self) -> Result<(), SimulationConfigError> {
        validate_positive("tolerance", self.tolerance)?;
        validate_count("max_iterations", self.max_iterations)?;
        validate_count("transient_max_iterations", self.transient_max_iterations)?;
        validate_positive("min_timestep", self.min_timestep)?;
        validate_positive("max_timestep", self.max_timestep)?;
        if self.min_timestep > self.max_timestep {
            return Err(SimulationConfigError::InvalidTimestepRange {
                min_timestep: self.min_timestep,
                max_timestep: self.max_timestep,
            });
        }
        if let Some(initial_timestep) = self.transient_initial_timestep {
            validate_positive("transient_initial_timestep", initial_timestep)?;
            if initial_timestep > self.max_timestep {
                return Err(SimulationConfigError::InitialTimestepExceedsMaximum {
                    initial_timestep,
                    max_timestep: self.max_timestep,
                });
            }
        }
        validate_positive("temperature", self.temperature)?;
        validate_finite("ramptime", self.ramptime)?;
        if let Some(delay_type) = self.digital_delay_type
            && !(0..=3).contains(&delay_type)
        {
            return Err(SimulationConfigError::InvalidDigitalDelayType(delay_type));
        }
        validate_positive("transient_trtol", self.transient_trtol)?;
        validate_optional_positive("transient_lte_reltol", self.transient_lte_reltol)?;
        validate_optional_positive("transient_lte_abstol", self.transient_lte_abstol)?;
        validate_positive(
            "transient_node_activity_bound",
            self.transient_node_activity_bound,
        )?;

        self.convergence_config.validate()?;
        self.bypass_config.validate()?;
        if let Some(grid) = self.locked_time_grid.as_deref() {
            validate_locked_time_grid(grid)?;
        }
        Ok(())
    }

    /// Select a broad compatibility dialect and use its device defaults.
    pub fn with_spice_dialect(mut self, dialect: SpiceDialect) -> Self {
        self.spice_dialect = dialect;
        self.apply_spice_dialect();
        self
    }

    /// Reset dialect-controlled device selections to follow the current dialect.
    pub fn apply_spice_dialect(&mut self) {
        self.jfet_level2_model = JfetLevel2Model::DialectDefault;
        self.transient_lte_reference = None;
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
    /// Explicit netlist-selected nonlinear continuation policy. `None` uses
    /// the frontend's normal convergence-aid sequence.
    pub nonlinear_continuation: Option<NonlinearContinuationMode>,
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
            nonlinear_continuation: None,
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
    /// Validate the numerical invariants of the convergence policy.
    pub fn validate(&self) -> Result<(), SimulationConfigError> {
        validate_positive("convergence_config.gmin_initial", self.gmin_initial)?;
        validate_positive("convergence_config.gmin_target", self.gmin_target)?;
        validate_positive(
            "convergence_config.junction_gmin_target",
            self.junction_gmin_target,
        )?;
        validate_positive("convergence_config.voltage_reltol", self.voltage_reltol)?;
        validate_finite("convergence_config.voltage_abstol", self.voltage_abstol)?;
        validate_finite("convergence_config.current_abstol", self.current_abstol)?;
        validate_finite("convergence_config.charge_abstol", self.charge_abstol)?;
        validate_positive("convergence_config.residual_reltol", self.residual_reltol)?;
        Ok(())
    }

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
    /// Validate bypass thresholds even when bypass is currently disabled.
    pub fn validate(&self) -> Result<(), SimulationConfigError> {
        validate_non_negative("bypass_config.reltol", self.reltol)?;
        validate_non_negative("bypass_config.abstol", self.abstol)
    }

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
            transient_initial_timestep: None,
            temperature: crate::constants::TEMP_REFERENCE,
            ramptime: 0.0,
            digital_delay_type: None,
            integration_method: crate::analysis::IntegrationMethod::TrapGear,
            spice_dialect: SpiceDialect::BestAvailable,
            jfet_level2_model: JfetLevel2Model::DialectDefault,
            b3soi_gmin_scaling: true,
            transient_trtol: crate::constants::TRTOL,
            transient_lte_reltol: None,
            transient_lte_abstol: None,
            transient_lte_reference: None,
            transient_new_bp_stepping: true,
            transient_node_activity_bound: crate::constants::DEVICE_ACTIVITY_STEP_BOUND,
            bypass_config: BypassConfig::default(),
            convergence_config: ConvergenceConfig::default(),
            locked_time_grid: None,
        }
    }
}

fn validate_count(field: &'static str, value: usize) -> Result<(), SimulationConfigError> {
    if value == 0 {
        Err(SimulationConfigError::InvalidCount { field, value })
    } else {
        Ok(())
    }
}

fn validate_finite(field: &'static str, value: Value) -> Result<(), SimulationConfigError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SimulationConfigError::InvalidValue {
            field,
            value,
            requirement: "finite",
        })
    }
}

fn validate_positive(field: &'static str, value: Value) -> Result<(), SimulationConfigError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(SimulationConfigError::InvalidValue {
            field,
            value,
            requirement: "a positive finite number",
        })
    }
}

fn validate_non_negative(field: &'static str, value: Value) -> Result<(), SimulationConfigError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(SimulationConfigError::InvalidValue {
            field,
            value,
            requirement: "a non-negative finite number",
        })
    }
}

fn validate_optional_positive(
    field: &'static str,
    value: Option<Value>,
) -> Result<(), SimulationConfigError> {
    value.map_or(Ok(()), |value| validate_positive(field, value))
}

fn validate_locked_time_grid(grid: &[Value]) -> Result<(), SimulationConfigError> {
    let mut previous = None;
    for (index, value) in grid.iter().copied().enumerate() {
        if !value.is_finite() || value < 0.0 {
            return Err(SimulationConfigError::InvalidLockedTimeGridPoint { index, value });
        }
        if let Some((previous_index, previous)) = previous
            && value <= previous
        {
            return Err(SimulationConfigError::NonIncreasingLockedTimeGrid {
                index,
                value,
                previous_index,
                previous,
            });
        }
        previous = Some((index, value));
    }
    Ok(())
}
