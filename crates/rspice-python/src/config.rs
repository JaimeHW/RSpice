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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damping_strategy_conversion_roundtrip() {
        let strategies = [
            (PyDampingStrategy::None, DampingStrategy::None),
            (PyDampingStrategy::LineSearch, DampingStrategy::LineSearch),
            (
                PyDampingStrategy::VoltageLimiting,
                DampingStrategy::VoltageLimiting,
            ),
            (PyDampingStrategy::BankRose, DampingStrategy::BankRose),
            (PyDampingStrategy::Combined, DampingStrategy::Combined),
        ];

        for (py_strat, rust_strat) in strategies {
            // Python to Rust
            let converted: DampingStrategy = py_strat.into();
            assert_eq!(converted, rust_strat);

            // Rust to Python
            let back: PyDampingStrategy = converted.into();
            assert_eq!(back, py_strat);
        }
    }

    #[test]
    fn test_bypass_config_default() {
        let config = PyBypassConfig::new();
        assert!(!config.inner.enabled);
        assert!(config.inner.reltol > 0.0);
        assert!(config.inner.abstol > 0.0);
    }

    #[test]
    fn test_bypass_config_enabled() {
        let config = PyBypassConfig::enabled();
        assert!(config.inner.enabled);
    }

    #[test]
    fn test_bypass_config_with_tolerances() {
        let config = PyBypassConfig::with_tolerances(1e-4, 1e-7);
        assert!(config.inner.enabled);
        assert!((config.inner.reltol - 1e-4).abs() < 1e-10);
        assert!((config.inner.abstol - 1e-7).abs() < 1e-12);
    }

    #[test]
    fn test_convergence_config_default() {
        let config = PyConvergenceConfig::new();
        assert!(config.inner.gmin_stepping);
        assert!(config.inner.source_stepping);
        assert!(config.inner.pseudo_transient);
        assert!(!config.inner.arc_length);
        assert_eq!(
            config.inner.damping_strategy,
            DampingStrategy::VoltageLimiting
        );
        assert!(config.inner.voltage_reltol > 0.0);
        assert_eq!(config.inner.voltage_abstol, 0.0);
        assert!(config.inner.current_abstol > 0.0);
        assert!(config.inner.residual_reltol > 0.0);
    }

    #[test]
    fn test_convergence_config_fast() {
        let config = PyConvergenceConfig::fast();
        assert!(!config.inner.gmin_stepping);
        assert!(!config.inner.source_stepping);
        assert!(!config.inner.pseudo_transient);
        assert!(!config.inner.arc_length);
        assert_eq!(config.inner.damping_strategy, DampingStrategy::None);
    }

    #[test]
    fn test_convergence_config_robust() {
        let config = PyConvergenceConfig::robust();
        assert!(config.inner.gmin_stepping);
        assert!(config.inner.source_stepping);
        assert!(config.inner.pseudo_transient);
        assert!(config.inner.arc_length);
        assert_eq!(config.inner.damping_strategy, DampingStrategy::Combined);
    }

    #[test]
    fn test_convergence_config_gmin_values() {
        let config = PyConvergenceConfig::new();
        assert!(config.inner.gmin_initial > config.inner.gmin_target);
        assert!(config.inner.gmin_initial > 1e-14);
        assert!(config.inner.gmin_target > 1e-17);
    }

    #[test]
    fn test_simulation_config_default() {
        let config = PySimulationConfig::new();
        let expected_tolerance = SimulationConfig::default().tolerance;
        assert!((config.inner.tolerance - expected_tolerance).abs() < 1e-18);
        assert_eq!(config.inner.max_iterations, 50);
        assert!((config.inner.temperature - 300.0).abs() < 0.1);
    }

    #[test]
    fn test_simulation_config_setters() {
        let mut config = PySimulationConfig::new();

        config.set_tolerance(1e-12);
        assert!((config.inner.tolerance - 1e-12).abs() < 1e-15);

        config.set_max_iterations(100);
        assert_eq!(config.inner.max_iterations, 100);

        config.set_temperature(350.0);
        assert!((config.inner.temperature - 350.0).abs() < 0.1);

        config.set_min_timestep(1e-18);
        assert!((config.inner.min_timestep - 1e-18).abs() < 1e-21);

        config.set_max_timestep(1e-6);
        assert!((config.inner.max_timestep - 1e-6).abs() < 1e-9);
    }

    #[test]
    fn test_simulation_config_convergence_integration() {
        let mut config = PySimulationConfig::new();
        let convergence = PyConvergenceConfig::robust();
        config.set_convergence(convergence);

        assert!(config.inner.convergence_config.arc_length);
        assert_eq!(
            config.inner.convergence_config.damping_strategy,
            DampingStrategy::Combined
        );
    }

    #[test]
    fn test_simulation_config_bypass_integration() {
        let mut config = PySimulationConfig::new();
        let bypass = PyBypassConfig::enabled();
        config.set_bypass(bypass);

        assert!(config.inner.bypass_config.enabled);
    }

    #[test]
    fn test_damping_strategy_repr() {
        assert_eq!(PyDampingStrategy::None.__repr__(), "DampingStrategy.NONE");
        assert_eq!(
            PyDampingStrategy::LineSearch.__repr__(),
            "DampingStrategy.LINE_SEARCH"
        );
        assert_eq!(
            PyDampingStrategy::VoltageLimiting.__repr__(),
            "DampingStrategy.VOLTAGE_LIMITING"
        );
        assert_eq!(
            PyDampingStrategy::BankRose.__repr__(),
            "DampingStrategy.BANK_ROSE"
        );
        assert_eq!(
            PyDampingStrategy::Combined.__repr__(),
            "DampingStrategy.COMBINED"
        );
    }

    #[test]
    fn test_bypass_config_repr() {
        let config = PyBypassConfig::enabled();
        let repr = config.__repr__();
        assert!(repr.contains("enabled=true"));
        assert!(repr.contains("reltol="));
        assert!(repr.contains("abstol="));
    }

    #[test]
    fn test_convergence_config_repr() {
        let config = PyConvergenceConfig::robust();
        let repr = config.__repr__();
        assert!(repr.contains("gmin_stepping=true"));
        assert!(repr.contains("arc_length=true"));
        assert!(repr.contains("residual_reltol="));
        assert!(repr.contains("current_abstol="));
    }

    #[test]
    fn test_simulation_config_repr() {
        let config = PySimulationConfig::new();
        let repr = config.__repr__();
        assert!(repr.contains("tolerance="));
        assert!(repr.contains("max_iterations=50"));
        assert!(repr.contains("temperature=300.0K"));
    }

    #[test]
    fn test_bypass_config_setters() {
        let mut config = PyBypassConfig::new();

        config.set_enabled(true);
        assert!(config.get_enabled());

        config.set_reltol(5e-4);
        assert!((config.get_reltol() - 5e-4).abs() < 1e-10);

        config.set_abstol(2e-7);
        assert!((config.get_abstol() - 2e-7).abs() < 1e-12);
    }

    #[test]
    fn test_convergence_config_setters() {
        let mut config = PyConvergenceConfig::new();

        config.set_gmin_stepping(false);
        assert!(!config.get_gmin_stepping());

        config.set_source_stepping(false);
        assert!(!config.get_source_stepping());

        config.set_pseudo_transient(false);
        assert!(!config.get_pseudo_transient());

        config.set_arc_length(true);
        assert!(config.get_arc_length());

        config.set_damping_strategy(PyDampingStrategy::BankRose);
        assert_eq!(config.get_damping_strategy(), PyDampingStrategy::BankRose);

        config.set_gmin_initial(1e-10);
        assert!((config.get_gmin_initial() - 1e-10).abs() < 1e-13);

        config.set_gmin_target(1e-16);
        assert!((config.get_gmin_target() - 1e-16).abs() < 1e-19);

        config.set_voltage_reltol(2e-3);
        assert!((config.get_voltage_reltol() - 2e-3).abs() < 1e-15);

        config.set_residual_reltol(4e-4);
        assert!((config.get_residual_reltol() - 4e-4).abs() < 1e-15);

        config.set_voltage_abstol(5e-7);
        assert!((config.get_voltage_abstol() - 5e-7).abs() < 1e-18);

        config.set_current_abstol(3e-12);
        assert!((config.get_current_abstol() - 3e-12).abs() < 1e-24);

        config.set_verbose(true);
        assert!(config.get_verbose());
    }

    #[test]
    fn test_damping_strategy_equality() {
        assert_eq!(PyDampingStrategy::None, PyDampingStrategy::None);
        assert_ne!(PyDampingStrategy::None, PyDampingStrategy::LineSearch);
        assert_ne!(
            PyDampingStrategy::LineSearch,
            PyDampingStrategy::VoltageLimiting
        );
        assert_ne!(
            PyDampingStrategy::VoltageLimiting,
            PyDampingStrategy::BankRose
        );
        assert_ne!(PyDampingStrategy::BankRose, PyDampingStrategy::Combined);
    }

    #[test]
    fn test_config_clone() {
        let config = PySimulationConfig::new();
        let cloned = config.clone();
        assert!((config.inner.tolerance - cloned.inner.tolerance).abs() < 1e-15);
        assert_eq!(config.inner.max_iterations, cloned.inner.max_iterations);
    }
}
