//! Simulation configuration Python bindings
//!
//! Provides Python access to simulation configuration options:
//! - `SimulationConfig` - Main simulation parameters
//! - `ConvergenceConfig` - Newton-Raphson convergence aids
//! - `BypassConfig` - Model evaluation bypass (latent device optimization)
//! - `DampingStrategy` - Newton iteration damping methods

use pyo3::prelude::*;
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

/// Configuration for model evaluation bypass (latent device optimization)
///
/// When enabled, device models are skipped if terminal voltages haven't
/// changed significantly since the last evaluation, reducing computation.
#[pyclass(name = "BypassConfig")]
#[derive(Clone)]
pub struct PyBypassConfig {
    pub(crate) inner: BypassConfig,
}

#[pymethods]
impl PyBypassConfig {
    /// Create a new bypass configuration (disabled by default)
    #[new]
    fn new() -> Self {
        Self {
            inner: BypassConfig::default(),
        }
    }

    /// Create bypass config with optimization enabled
    #[staticmethod]
    pub fn enabled() -> Self {
        Self {
            inner: BypassConfig::enabled(),
        }
    }

    /// Create bypass config with custom tolerances
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
#[pyclass(name = "ConvergenceConfig")]
#[derive(Clone)]
pub struct PyConvergenceConfig {
    pub(crate) inner: ConvergenceConfig,
}

#[pymethods]
impl PyConvergenceConfig {
    /// Create default convergence configuration
    #[new]
    pub fn new() -> Self {
        Self {
            inner: ConvergenceConfig::default(),
        }
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
            "ConvergenceConfig(gmin_stepping={}, source_stepping={}, pseudo_transient={}, arc_length={}, damping={:?}, voltage_reltol={:.0e}, residual_reltol={:.0e}, voltage_abstol={:.0e}, current_abstol={:.0e})",
            self.inner.gmin_stepping,
            self.inner.source_stepping,
            self.inner.pseudo_transient,
            self.inner.arc_length,
            self.inner.damping_strategy,
            self.inner.voltage_reltol,
            self.inner.residual_reltol,
            self.inner.voltage_abstol,
            self.inner.current_abstol
        )
    }
}

/// Main simulation configuration
///
/// Controls simulation parameters like tolerances, temperature, and
/// integration methods.
#[pyclass(name = "SimulationConfig")]
#[derive(Clone)]
pub struct PySimulationConfig {
    pub(crate) inner: SimulationConfig,
}

#[pymethods]
impl PySimulationConfig {
    /// Create default simulation configuration
    #[new]
    pub fn new() -> Self {
        Self {
            inner: SimulationConfig::default(),
        }
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

    /// Maximum Newton-Raphson iterations
    #[getter]
    fn get_max_iterations(&self) -> usize {
        self.inner.max_iterations
    }

    #[setter]
    pub fn set_max_iterations(&mut self, value: usize) {
        self.inner.max_iterations = value;
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

    /// Convergence configuration
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

    /// Bypass configuration
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
            "SimulationConfig(tolerance={:.0e}, max_iterations={}, temperature={:.1}K)",
            self.inner.tolerance, self.inner.max_iterations, self.inner.temperature
        )
    }
}

