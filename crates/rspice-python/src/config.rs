//! Simulation configuration Python bindings
//!
//! Provides Python access to simulation configuration options:
//! - `SimulationConfig` - Main simulation parameters
//! - `ConvergenceConfig` - Newton-Raphson convergence aids
//! - `BypassConfig` - Model evaluation bypass (latent device optimization)
//! - `DampingStrategy` - Newton iteration damping methods
//! - `IntegrationMethod` - Transient integration schemes
//!
//! All three config classes accept keyword arguments in their constructors so
//! a complete configuration can be built in one expression. This matters
//! because property getters return *copies* (Rust value semantics): mutating
//! `config.convergence.gmin_stepping` modifies a temporary and is lost.
//! Either build with keywords or assign whole sub-objects back.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rspice_core::analysis::IntegrationMethod;
use rspice_core::engine::{BypassConfig, ConvergenceConfig, DampingStrategy, SimulationConfig};

/// Damping strategy for Newton-Raphson iterations
///
/// Controls how Newton iteration steps are modified to improve convergence:
/// - `NONE` - Full Newton step (fastest but may diverge)
/// - `LINE_SEARCH` - Backtracking line search (Armijo condition)
/// - `VOLTAGE_LIMITING` - Junction voltage limiting (SPICE-style)
/// - `BANK_ROSE` - Bank-Rose adaptive damping
/// - `COMBINED` - Voltage limiting + line search (most robust)
#[pyclass(name = "DampingStrategy", eq, eq_int)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyDampingStrategy {
    #[pyo3(name = "NONE")]
    None = 0,
    #[pyo3(name = "LINE_SEARCH")]
    LineSearch = 1,
    #[pyo3(name = "VOLTAGE_LIMITING")]
    VoltageLimiting = 2,
    #[pyo3(name = "BANK_ROSE")]
    BankRose = 3,
    #[pyo3(name = "COMBINED")]
    Combined = 4,
}

impl From<PyDampingStrategy> for DampingStrategy {
    fn from(py_strategy: PyDampingStrategy) -> Self {
        match py_strategy {
            PyDampingStrategy::None => DampingStrategy::None,
            PyDampingStrategy::LineSearch => DampingStrategy::LineSearch,
            PyDampingStrategy::VoltageLimiting => DampingStrategy::VoltageLimiting,
            PyDampingStrategy::BankRose => DampingStrategy::BankRose,
            PyDampingStrategy::Combined => DampingStrategy::Combined,
        }
    }
}

impl From<DampingStrategy> for PyDampingStrategy {
    fn from(strategy: DampingStrategy) -> Self {
        match strategy {
            DampingStrategy::None => PyDampingStrategy::None,
            DampingStrategy::LineSearch => PyDampingStrategy::LineSearch,
            DampingStrategy::VoltageLimiting => PyDampingStrategy::VoltageLimiting,
            DampingStrategy::BankRose => PyDampingStrategy::BankRose,
            DampingStrategy::Combined => PyDampingStrategy::Combined,
        }
    }
}

#[pymethods]
impl PyDampingStrategy {
    fn __repr__(&self) -> String {
        match self {
            PyDampingStrategy::None => "DampingStrategy.NONE".to_string(),
            PyDampingStrategy::LineSearch => "DampingStrategy.LINE_SEARCH".to_string(),
            PyDampingStrategy::VoltageLimiting => "DampingStrategy.VOLTAGE_LIMITING".to_string(),
            PyDampingStrategy::BankRose => "DampingStrategy.BANK_ROSE".to_string(),
            PyDampingStrategy::Combined => "DampingStrategy.COMBINED".to_string(),
        }
    }
}

/// Transient integration method
///
/// - `BACKWARD_EULER` - 1st order, very stable, more numerical damping
/// - `TRAPEZOIDAL` - 2nd order, A-stable, can ring on discontinuities
/// - `GEAR2` - BDF2, good for stiff systems
/// - `TRAP_GEAR` - Hybrid: trapezoidal that auto-switches to Gear2 at
///   discontinuities (default)
#[pyclass(name = "IntegrationMethod", eq, eq_int)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyIntegrationMethod {
    #[pyo3(name = "BACKWARD_EULER")]
    BackwardEuler = 0,
    #[pyo3(name = "TRAPEZOIDAL")]
    Trapezoidal = 1,
    #[pyo3(name = "GEAR2")]
    Gear2 = 2,
    #[pyo3(name = "TRAP_GEAR")]
    TrapGear = 3,
}

impl From<PyIntegrationMethod> for IntegrationMethod {
    fn from(method: PyIntegrationMethod) -> Self {
        match method {
            PyIntegrationMethod::BackwardEuler => IntegrationMethod::BackwardEuler,
            PyIntegrationMethod::Trapezoidal => IntegrationMethod::Trapezoidal,
            PyIntegrationMethod::Gear2 => IntegrationMethod::Gear2,
            PyIntegrationMethod::TrapGear => IntegrationMethod::TrapGear,
        }
    }
}

impl From<IntegrationMethod> for PyIntegrationMethod {
    fn from(method: IntegrationMethod) -> Self {
        match method {
            IntegrationMethod::BackwardEuler => PyIntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal => PyIntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2 => PyIntegrationMethod::Gear2,
            IntegrationMethod::TrapGear => PyIntegrationMethod::TrapGear,
        }
    }
}

#[pymethods]
impl PyIntegrationMethod {
    fn __repr__(&self) -> String {
        match self {
            PyIntegrationMethod::BackwardEuler => "IntegrationMethod.BACKWARD_EULER".to_string(),
            PyIntegrationMethod::Trapezoidal => "IntegrationMethod.TRAPEZOIDAL".to_string(),
            PyIntegrationMethod::Gear2 => "IntegrationMethod.GEAR2".to_string(),
            PyIntegrationMethod::TrapGear => "IntegrationMethod.TRAP_GEAR".to_string(),
        }
    }
}

/// Configuration for model evaluation bypass (latent device optimization)
///
/// When enabled, device models are skipped if terminal voltages haven't
/// changed significantly since the last evaluation, reducing computation.
///
/// Example:
///     >>> bypass = BypassConfig(enabled=True, reltol=1e-3)
///     >>> bypass.enabled
///     True
#[pyclass(name = "BypassConfig")]
#[derive(Clone)]
pub struct PyBypassConfig {
    pub(crate) inner: BypassConfig,
}

#[pymethods]
impl PyBypassConfig {
    /// Create a bypass configuration (disabled by default)
    ///
    /// Args:
    ///     enabled: Whether bypass optimization is active
    ///     reltol: Relative voltage tolerance for bypass detection
    ///     abstol: Absolute voltage tolerance for bypass detection
    #[new]
    #[pyo3(signature = (*, enabled=None, reltol=None, abstol=None))]
    fn new(enabled: Option<bool>, reltol: Option<f64>, abstol: Option<f64>) -> Self {
        let mut inner = BypassConfig::default();
        if let Some(v) = enabled {
            inner.enabled = v;
        }
        if let Some(v) = reltol {
            inner.reltol = v;
        }
        if let Some(v) = abstol {
            inner.abstol = v;
        }
        Self { inner }
    }

    /// Create bypass config with custom tolerances (enabled)
    #[staticmethod]
    fn with_tolerances(reltol: f64, abstol: f64) -> Self {
        Self {
            inner: BypassConfig::with_tolerances(reltol, abstol),
        }
    }

    /// Whether bypass optimization is enabled
    #[getter]
    fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }

    /// Relative voltage tolerance for bypass detection
    #[getter]
    fn get_reltol(&self) -> f64 {
        self.inner.reltol
    }

    #[setter]
    fn set_reltol(&mut self, value: f64) {
        self.inner.reltol = value;
    }

    /// Absolute voltage tolerance for bypass detection
    #[getter]
    fn get_abstol(&self) -> f64 {
        self.inner.abstol
    }

    #[setter]
    fn set_abstol(&mut self, value: f64) {
        self.inner.abstol = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "BypassConfig(enabled={}, reltol={:.0e}, abstol={:.0e})",
            self.inner.enabled, self.inner.reltol, self.inner.abstol
        )
    }
}

/// Configuration for DC convergence algorithms
///
/// Controls which convergence aids are used when Newton-Raphson fails
/// to converge directly on difficult circuits.
///
/// All fields can be set via keyword arguments:
///     >>> conv = ConvergenceConfig(gmin_stepping=True, verbose=True)
///
/// Note: reading a nested config (e.g. `sim_config.convergence`) returns a
/// copy; mutate it and assign it back, or construct with keywords.
#[pyclass(name = "ConvergenceConfig")]
#[derive(Clone)]
pub struct PyConvergenceConfig {
    pub(crate) inner: ConvergenceConfig,
}

#[pymethods]
impl PyConvergenceConfig {
    /// Create a convergence configuration
    ///
    /// All arguments are optional keywords; unspecified fields keep their
    /// defaults.
    #[new]
    #[pyo3(signature = (*, gmin_stepping=None, source_stepping=None, pseudo_transient=None,
                        arc_length=None, damping_strategy=None, gmin_initial=None,
                        gmin_target=None, voltage_reltol=None, residual_reltol=None,
                        voltage_abstol=None, current_abstol=None, charge_abstol=None,
                        verbose=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        gmin_stepping: Option<bool>,
        source_stepping: Option<bool>,
        pseudo_transient: Option<bool>,
        arc_length: Option<bool>,
        damping_strategy: Option<PyDampingStrategy>,
        gmin_initial: Option<f64>,
        gmin_target: Option<f64>,
        voltage_reltol: Option<f64>,
        residual_reltol: Option<f64>,
        voltage_abstol: Option<f64>,
        current_abstol: Option<f64>,
        charge_abstol: Option<f64>,
        verbose: Option<bool>,
    ) -> Self {
        let mut inner = ConvergenceConfig::default();
        if let Some(v) = gmin_stepping {
            inner.gmin_stepping = v;
        }
        if let Some(v) = source_stepping {
            inner.source_stepping = v;
        }
        if let Some(v) = pseudo_transient {
            inner.pseudo_transient = v;
        }
        if let Some(v) = arc_length {
            inner.arc_length = v;
        }
        if let Some(v) = damping_strategy {
            inner.damping_strategy = v.into();
        }
        if let Some(v) = gmin_initial {
            inner.gmin_initial = v;
        }
        if let Some(v) = gmin_target {
            inner.gmin_target = v;
        }
        if let Some(v) = voltage_reltol {
            inner.voltage_reltol = v;
        }
        if let Some(v) = residual_reltol {
            inner.residual_reltol = v;
        }
        if let Some(v) = voltage_abstol {
            inner.voltage_abstol = v;
        }
        if let Some(v) = current_abstol {
            inner.current_abstol = v;
        }
        if let Some(v) = charge_abstol {
            inner.charge_abstol = v;
        }
        if let Some(v) = verbose {
            inner.verbose = v;
        }
        Self { inner }
    }

    /// Create minimal config (direct Newton only - fastest but may fail)
    #[staticmethod]
    pub fn fast() -> Self {
        Self {
            inner: ConvergenceConfig::fast(),
        }
    }

    /// Create robust config (all methods enabled - most reliable)
    #[staticmethod]
    pub fn robust() -> Self {
        Self {
            inner: ConvergenceConfig::robust(),
        }
    }

    /// Enable GMIN stepping (small conductances to ground)
    #[getter]
    fn get_gmin_stepping(&self) -> bool {
        self.inner.gmin_stepping
    }

    #[setter]
    fn set_gmin_stepping(&mut self, value: bool) {
        self.inner.gmin_stepping = value;
    }

    /// Enable source stepping (ramp sources from 0 to 100%)
    #[getter]
    fn get_source_stepping(&self) -> bool {
        self.inner.source_stepping
    }

    #[setter]
    fn set_source_stepping(&mut self, value: bool) {
        self.inner.source_stepping = value;
    }

    /// Enable pseudo-transient continuation
    #[getter]
    fn get_pseudo_transient(&self) -> bool {
        self.inner.pseudo_transient
    }

    #[setter]
    fn set_pseudo_transient(&mut self, value: bool) {
        self.inner.pseudo_transient = value;
    }

    /// Enable arc-length continuation for non-monotonic curves
    #[getter]
    fn get_arc_length(&self) -> bool {
        self.inner.arc_length
    }

    #[setter]
    fn set_arc_length(&mut self, value: bool) {
        self.inner.arc_length = value;
    }

    /// Damping strategy for Newton iterations
    #[getter]
    fn get_damping_strategy(&self) -> PyDampingStrategy {
        self.inner.damping_strategy.into()
    }

    #[setter]
    pub fn set_damping_strategy(&mut self, value: PyDampingStrategy) {
        self.inner.damping_strategy = value.into();
    }

    /// Initial GMIN value (typically 1e-12)
    #[getter]
    fn get_gmin_initial(&self) -> f64 {
        self.inner.gmin_initial
    }

    #[setter]
    fn set_gmin_initial(&mut self, value: f64) {
        self.inner.gmin_initial = value;
    }

    /// Target GMIN value (typically 1e-15)
    #[getter]
    fn get_gmin_target(&self) -> f64 {
        self.inner.gmin_target
    }

    #[setter]
    fn set_gmin_target(&mut self, value: f64) {
        self.inner.gmin_target = value;
    }

    /// Relative voltage tolerance for Newton convergence checks.
    #[getter]
    fn get_voltage_reltol(&self) -> f64 {
        self.inner.voltage_reltol
    }

    #[setter]
    fn set_voltage_reltol(&mut self, value: f64) {
        self.inner.voltage_reltol = value;
    }

    /// Relative equation residual tolerance for Newton convergence checks.
    #[getter]
    fn get_residual_reltol(&self) -> f64 {
        self.inner.residual_reltol
    }

    #[setter]
    fn set_residual_reltol(&mut self, value: f64) {
        self.inner.residual_reltol = value;
    }

    /// Absolute voltage tolerance for Newton convergence checks.
    #[getter]
    fn get_voltage_abstol(&self) -> f64 {
        self.inner.voltage_abstol
    }

    #[setter]
    fn set_voltage_abstol(&mut self, value: f64) {
        self.inner.voltage_abstol = value;
    }

    /// Absolute current tolerance for equation residual convergence checks.
    #[getter]
    fn get_current_abstol(&self) -> f64 {
        self.inner.current_abstol
    }

    #[setter]
    fn set_current_abstol(&mut self, value: f64) {
        self.inner.current_abstol = value;
    }

    /// Absolute charge tolerance for transient devices (CHGTOL).
    #[getter]
    fn get_charge_abstol(&self) -> f64 {
        self.inner.charge_abstol
    }

    #[setter]
    fn set_charge_abstol(&mut self, value: f64) {
        self.inner.charge_abstol = value;
    }

    /// Enable verbose convergence logging
    #[getter]
    fn get_verbose(&self) -> bool {
        self.inner.verbose
    }

    #[setter]
    fn set_verbose(&mut self, value: bool) {
        self.inner.verbose = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "ConvergenceConfig(gmin_stepping={}, source_stepping={}, pseudo_transient={}, arc_length={}, damping={:?}, voltage_reltol={:.0e}, residual_reltol={:.0e}, voltage_abstol={:.0e}, current_abstol={:.0e}, charge_abstol={:.0e})",
            self.inner.gmin_stepping,
            self.inner.source_stepping,
            self.inner.pseudo_transient,
            self.inner.arc_length,
            self.inner.damping_strategy,
            self.inner.voltage_reltol,
            self.inner.residual_reltol,
            self.inner.voltage_abstol,
            self.inner.current_abstol,
            self.inner.charge_abstol
        )
    }
}

/// Main simulation configuration
///
/// Controls simulation parameters like tolerances, temperature, and
/// integration methods. All fields can be set via keyword arguments:
///
///     >>> config = SimulationConfig(
///     ...     tolerance=1e-12,
///     ...     temperature=350.0,
///     ...     integration_method=IntegrationMethod.GEAR2,
///     ...     convergence=ConvergenceConfig.robust(),
///     ... )
///
/// Note: property getters for `convergence` and `bypass` return copies.
/// `config.convergence.verbose = True` mutates a temporary and is silently
/// lost — assign a whole ConvergenceConfig instead.
#[pyclass(name = "SimulationConfig")]
#[derive(Clone)]
pub struct PySimulationConfig {
    pub(crate) inner: SimulationConfig,
}

#[pymethods]
impl PySimulationConfig {
    /// Create a simulation configuration
    ///
    /// All arguments are optional keywords; unspecified fields keep their
    /// defaults.
    ///
    /// Args:
    ///     tolerance: Newton-Raphson convergence tolerance
    ///     max_iterations: Maximum DC Newton-Raphson iterations
    ///     transient_max_iterations: Newton budget per transient step (ITL4)
    ///     min_timestep: Preferred minimum transient timestep (seconds)
    ///     max_timestep: Maximum transient timestep (seconds)
    ///     temperature: Simulation temperature in Kelvin
    ///     integration_method: Transient integration scheme
    ///     transient_trtol: Truncation-error tolerance factor (TRTOL)
    ///     convergence: DC convergence aid configuration
    ///     bypass: Latent-device bypass configuration
    #[new]
    #[pyo3(signature = (*, tolerance=None, max_iterations=None, transient_max_iterations=None,
                        min_timestep=None, max_timestep=None, temperature=None,
                        integration_method=None, transient_trtol=None,
                        convergence=None, bypass=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tolerance: Option<f64>,
        max_iterations: Option<usize>,
        transient_max_iterations: Option<usize>,
        min_timestep: Option<f64>,
        max_timestep: Option<f64>,
        temperature: Option<f64>,
        integration_method: Option<PyIntegrationMethod>,
        transient_trtol: Option<f64>,
        convergence: Option<PyConvergenceConfig>,
        bypass: Option<PyBypassConfig>,
    ) -> PyResult<Self> {
        let mut inner = SimulationConfig::default();
        if let Some(v) = tolerance {
            inner.tolerance = v;
        }
        if let Some(v) = max_iterations {
            inner.max_iterations = v;
        }
        if let Some(v) = transient_max_iterations {
            inner.transient_max_iterations = v;
        }
        if let Some(v) = min_timestep {
            inner.min_timestep = v;
        }
        if let Some(v) = max_timestep {
            inner.max_timestep = v;
        }
        if let Some(v) = temperature {
            if !v.is_finite() || v <= 0.0 {
                return Err(PyValueError::new_err(format!(
                    "temperature must be a positive number of Kelvin, got {v}"
                )));
            }
            inner.temperature = v;
        }
        if let Some(v) = integration_method {
            inner.integration_method = v.into();
        }
        if let Some(v) = transient_trtol {
            inner.transient_trtol = v;
        }
        if let Some(v) = convergence {
            inner.convergence_config = v.inner;
        }
        if let Some(v) = bypass {
            inner.bypass_config = v.inner;
        }
        Ok(Self { inner })
    }

    /// Convergence tolerance for Newton-Raphson
    #[getter]
    fn get_tolerance(&self) -> f64 {
        self.inner.tolerance
    }

    #[setter]
    pub fn set_tolerance(&mut self, value: f64) {
        self.inner.tolerance = value;
    }

    /// Maximum Newton-Raphson iterations (DC)
    #[getter]
    fn get_max_iterations(&self) -> usize {
        self.inner.max_iterations
    }

    #[setter]
    pub fn set_max_iterations(&mut self, value: usize) {
        self.inner.max_iterations = value;
    }

    /// Maximum Newton-Raphson iterations per transient timestep (ITL4)
    #[getter]
    fn get_transient_max_iterations(&self) -> usize {
        self.inner.transient_max_iterations
    }

    #[setter]
    fn set_transient_max_iterations(&mut self, value: usize) {
        self.inner.transient_max_iterations = value;
    }

    /// Minimum timestep for transient analysis
    #[getter]
    fn get_min_timestep(&self) -> f64 {
        self.inner.min_timestep
    }

    #[setter]
    fn set_min_timestep(&mut self, value: f64) {
        self.inner.min_timestep = value;
    }

    /// Maximum timestep for transient analysis
    #[getter]
    fn get_max_timestep(&self) -> f64 {
        self.inner.max_timestep
    }

    #[setter]
    fn set_max_timestep(&mut self, value: f64) {
        self.inner.max_timestep = value;
    }

    /// Temperature in Kelvin
    #[getter]
    fn get_temperature(&self) -> f64 {
        self.inner.temperature
    }

    #[setter]
    pub fn set_temperature(&mut self, value: f64) {
        self.inner.temperature = value;
    }

    /// Transient integration method
    #[getter]
    fn get_integration_method(&self) -> PyIntegrationMethod {
        self.inner.integration_method.into()
    }

    #[setter]
    fn set_integration_method(&mut self, value: PyIntegrationMethod) {
        self.inner.integration_method = value.into();
    }

    /// Transient truncation tolerance factor (TRTOL)
    #[getter]
    fn get_transient_trtol(&self) -> f64 {
        self.inner.transient_trtol
    }

    #[setter]
    fn set_transient_trtol(&mut self, value: f64) {
        self.inner.transient_trtol = value;
    }

    /// Convergence configuration (returns a copy; assign back to modify)
    #[getter]
    fn get_convergence(&self) -> PyConvergenceConfig {
        PyConvergenceConfig {
            inner: self.inner.convergence_config.clone(),
        }
    }

    #[setter]
    pub fn set_convergence(&mut self, value: PyConvergenceConfig) {
        self.inner.convergence_config = value.inner;
    }

    /// Bypass configuration (returns a copy; assign back to modify)
    #[getter]
    fn get_bypass(&self) -> PyBypassConfig {
        PyBypassConfig {
            inner: self.inner.bypass_config.clone(),
        }
    }

    #[setter]
    pub fn set_bypass(&mut self, value: PyBypassConfig) {
        self.inner.bypass_config = value.inner;
    }

    fn __repr__(&self) -> String {
        format!(
            "SimulationConfig(tolerance={:.0e}, max_iterations={}, temperature={:.1}K, integration={:?})",
            self.inner.tolerance,
            self.inner.max_iterations,
            self.inner.temperature,
            self.inner.integration_method
        )
    }
}
