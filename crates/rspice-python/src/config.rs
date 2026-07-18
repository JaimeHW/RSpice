//! Simulation configuration Python bindings
//!
//! Provides Python access to simulation configuration options:
//! - `SimulationConfig` - Main simulation parameters
//! - `ConvergenceConfig` - Newton-Raphson convergence aids
//! - `BypassConfig` - Model evaluation bypass (latent device optimization)
//! - `ResourceLimits` - Ingestion, construction, analysis, and cache ceilings
//! - `DampingStrategy` - Newton iteration damping methods
//! - `IntegrationMethod` - Transient integration schemes
//!
//! All four config classes accept keyword arguments in their constructors so
//! a complete configuration can be built in one expression. This matters
//! because property getters return *copies* (Rust value semantics): mutating
//! `config.convergence.gmin_stepping` modifies a temporary and is lost.
//! Either build with keywords or assign whole sub-objects back.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rspice_core::ResourceLimits;
use rspice_core::analysis::IntegrationMethod;
use rspice_core::engine::{BypassConfig, ConvergenceConfig, DampingStrategy, SimulationConfig};

fn validate_positive(name: &str, value: f64) -> PyResult<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(PyValueError::new_err(format!(
            "{name} must be a positive finite number, got {value}"
        )))
    }
}

fn validate_nonnegative(name: &str, value: f64) -> PyResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(PyValueError::new_err(format!(
            "{name} must be a non-negative finite number, got {value}"
        )))
    }
}

fn validate_positive_usize(name: &str, value: usize) -> PyResult<usize> {
    if value > 0 {
        Ok(value)
    } else {
        Err(PyValueError::new_err(format!(
            "{name} must be at least 1, got {value}"
        )))
    }
}

fn validate_timestep_window(min_timestep: f64, max_timestep: f64) -> PyResult<()> {
    if min_timestep <= max_timestep {
        Ok(())
    } else {
        Err(PyValueError::new_err(format!(
            "min_timestep ({min_timestep}) must be <= max_timestep ({max_timestep})"
        )))
    }
}

fn validate_gmin_window(gmin_initial: f64, gmin_target: f64) -> PyResult<()> {
    if gmin_initial >= gmin_target {
        Ok(())
    } else {
        Err(PyValueError::new_err(format!(
            "gmin_initial ({gmin_initial}) must be >= gmin_target ({gmin_target})"
        )))
    }
}

/// Damping strategy for Newton-Raphson iterations
///
/// Controls how Newton iteration steps are modified to improve convergence:
/// - `NONE` - Full Newton step (fastest but may diverge)
/// - `LINE_SEARCH` - Backtracking line search (Armijo condition)
/// - `VOLTAGE_LIMITING` - Junction voltage limiting (SPICE-style)
/// - `BANK_ROSE` - Bank-Rose adaptive damping
/// - `COMBINED` - Voltage limiting + line search (most robust)
#[pyclass(
    name = "DampingStrategy",
    module = "rspice",
    eq,
    eq_int,
    from_py_object
)]
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
#[pyclass(
    name = "IntegrationMethod",
    module = "rspice",
    eq,
    eq_int,
    from_py_object
)]
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
#[pyclass(name = "BypassConfig", module = "rspice", from_py_object)]
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
    fn new(enabled: Option<bool>, reltol: Option<f64>, abstol: Option<f64>) -> PyResult<Self> {
        let mut inner = BypassConfig::default();
        if let Some(v) = enabled {
            inner.enabled = v;
        }
        if let Some(v) = reltol {
            inner.reltol = validate_nonnegative("reltol", v)?;
        }
        if let Some(v) = abstol {
            inner.abstol = validate_nonnegative("abstol", v)?;
        }
        Ok(Self { inner })
    }

    /// Create bypass config with custom tolerances (enabled)
    #[staticmethod]
    fn with_tolerances(reltol: f64, abstol: f64) -> PyResult<Self> {
        let reltol = validate_nonnegative("reltol", reltol)?;
        let abstol = validate_nonnegative("abstol", abstol)?;
        Ok(Self {
            inner: BypassConfig::with_tolerances(reltol, abstol),
        })
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
    fn set_reltol(&mut self, value: f64) -> PyResult<()> {
        self.inner.reltol = validate_nonnegative("reltol", value)?;
        Ok(())
    }

    /// Absolute voltage tolerance for bypass detection
    #[getter]
    fn get_abstol(&self) -> f64 {
        self.inner.abstol
    }

    #[setter]
    fn set_abstol(&mut self, value: f64) -> PyResult<()> {
        self.inner.abstol = validate_nonnegative("abstol", value)?;
        Ok(())
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
#[pyclass(name = "ConvergenceConfig", module = "rspice", from_py_object)]
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
    ) -> PyResult<Self> {
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
            inner.gmin_initial = validate_nonnegative("gmin_initial", v)?;
        }
        if let Some(v) = gmin_target {
            inner.gmin_target = validate_nonnegative("gmin_target", v)?;
        }
        if let Some(v) = voltage_reltol {
            inner.voltage_reltol = validate_positive("voltage_reltol", v)?;
        }
        if let Some(v) = residual_reltol {
            inner.residual_reltol = validate_positive("residual_reltol", v)?;
        }
        if let Some(v) = voltage_abstol {
            inner.voltage_abstol = validate_nonnegative("voltage_abstol", v)?;
        }
        if let Some(v) = current_abstol {
            inner.current_abstol = validate_nonnegative("current_abstol", v)?;
        }
        if let Some(v) = charge_abstol {
            inner.charge_abstol = validate_nonnegative("charge_abstol", v)?;
        }
        if let Some(v) = verbose {
            inner.verbose = v;
        }
        validate_gmin_window(inner.gmin_initial, inner.gmin_target)?;
        Ok(Self { inner })
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
    fn set_gmin_initial(&mut self, value: f64) -> PyResult<()> {
        let value = validate_nonnegative("gmin_initial", value)?;
        validate_gmin_window(value, self.inner.gmin_target)?;
        self.inner.gmin_initial = value;
        Ok(())
    }

    /// Target GMIN value (typically 1e-15)
    #[getter]
    fn get_gmin_target(&self) -> f64 {
        self.inner.gmin_target
    }

    #[setter]
    fn set_gmin_target(&mut self, value: f64) -> PyResult<()> {
        let value = validate_nonnegative("gmin_target", value)?;
        validate_gmin_window(self.inner.gmin_initial, value)?;
        self.inner.gmin_target = value;
        Ok(())
    }

    /// Relative voltage tolerance for Newton convergence checks.
    #[getter]
    fn get_voltage_reltol(&self) -> f64 {
        self.inner.voltage_reltol
    }

    #[setter]
    fn set_voltage_reltol(&mut self, value: f64) -> PyResult<()> {
        self.inner.voltage_reltol = validate_positive("voltage_reltol", value)?;
        Ok(())
    }

    /// Relative equation residual tolerance for Newton convergence checks.
    #[getter]
    fn get_residual_reltol(&self) -> f64 {
        self.inner.residual_reltol
    }

    #[setter]
    fn set_residual_reltol(&mut self, value: f64) -> PyResult<()> {
        self.inner.residual_reltol = validate_positive("residual_reltol", value)?;
        Ok(())
    }

    /// Absolute voltage tolerance for Newton convergence checks.
    #[getter]
    fn get_voltage_abstol(&self) -> f64 {
        self.inner.voltage_abstol
    }

    #[setter]
    fn set_voltage_abstol(&mut self, value: f64) -> PyResult<()> {
        self.inner.voltage_abstol = validate_nonnegative("voltage_abstol", value)?;
        Ok(())
    }

    /// Absolute current tolerance for equation residual convergence checks.
    #[getter]
    fn get_current_abstol(&self) -> f64 {
        self.inner.current_abstol
    }

    #[setter]
    fn set_current_abstol(&mut self, value: f64) -> PyResult<()> {
        self.inner.current_abstol = validate_nonnegative("current_abstol", value)?;
        Ok(())
    }

    /// Absolute charge tolerance for transient devices (CHGTOL).
    #[getter]
    fn get_charge_abstol(&self) -> f64 {
        self.inner.charge_abstol
    }

    #[setter]
    fn set_charge_abstol(&mut self, value: f64) -> PyResult<()> {
        self.inner.charge_abstol = validate_nonnegative("charge_abstol", value)?;
        Ok(())
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

/// Resource ceilings for untrusted, interactive, and batch workloads.
///
/// The same object can be supplied to `Netlist.parse*` and
/// `SimulationConfig`, ensuring parsing and execution use one policy.
#[pyclass(name = "ResourceLimits", module = "rspice", from_py_object)]
#[derive(Clone)]
pub struct PyResourceLimits {
    #[pyo3(get, set)]
    pub max_netlist_bytes: usize,
    #[pyo3(get, set)]
    pub max_netlist_lines: usize,
    #[pyo3(get, set)]
    pub max_expanded_source_bytes: usize,
    #[pyo3(get, set)]
    pub max_dependency_source_bytes: usize,
    #[pyo3(get, set)]
    pub max_external_data_bytes: usize,
    #[pyo3(get, set)]
    pub max_external_data_values: usize,
    #[pyo3(get, set)]
    pub max_shared_cache_bytes: usize,
    #[pyo3(get, set)]
    pub max_include_depth: usize,
    #[pyo3(get, set)]
    pub max_hierarchy_depth: usize,
    #[pyo3(get, set)]
    pub max_flattened_elements: usize,
    #[pyo3(get, set)]
    pub max_circuit_nodes: usize,
    #[pyo3(get, set)]
    pub max_matrix_unknowns: usize,
    #[pyo3(get, set)]
    pub max_analysis_points: usize,
    #[pyo3(get, set)]
    pub max_result_values: usize,
    #[pyo3(get, set)]
    pub max_batch_runs: usize,
}

impl PyResourceLimits {
    pub(crate) fn from_core(limits: ResourceLimits) -> Self {
        Self {
            max_netlist_bytes: limits.max_netlist_bytes,
            max_netlist_lines: limits.max_netlist_lines,
            max_expanded_source_bytes: limits.max_expanded_source_bytes,
            max_dependency_source_bytes: limits.max_dependency_source_bytes,
            max_external_data_bytes: limits.max_external_data_bytes,
            max_external_data_values: limits.max_external_data_values,
            max_shared_cache_bytes: limits.max_shared_cache_bytes,
            max_include_depth: limits.max_include_depth,
            max_hierarchy_depth: limits.max_hierarchy_depth,
            max_flattened_elements: limits.max_flattened_elements,
            max_circuit_nodes: limits.max_circuit_nodes,
            max_matrix_unknowns: limits.max_matrix_unknowns,
            max_analysis_points: limits.max_analysis_points,
            max_result_values: limits.max_result_values,
            max_batch_runs: limits.max_batch_runs,
        }
    }

    pub(crate) fn to_core(&self) -> ResourceLimits {
        let mut limits = ResourceLimits::default();
        limits.max_netlist_bytes = self.max_netlist_bytes;
        limits.max_netlist_lines = self.max_netlist_lines;
        limits.max_expanded_source_bytes = self.max_expanded_source_bytes;
        limits.max_dependency_source_bytes = self.max_dependency_source_bytes;
        limits.max_external_data_bytes = self.max_external_data_bytes;
        limits.max_external_data_values = self.max_external_data_values;
        limits.max_shared_cache_bytes = self.max_shared_cache_bytes;
        limits.max_include_depth = self.max_include_depth;
        limits.max_hierarchy_depth = self.max_hierarchy_depth;
        limits.max_flattened_elements = self.max_flattened_elements;
        limits.max_circuit_nodes = self.max_circuit_nodes;
        limits.max_matrix_unknowns = self.max_matrix_unknowns;
        limits.max_analysis_points = self.max_analysis_points;
        limits.max_result_values = self.max_result_values;
        limits.max_batch_runs = self.max_batch_runs;
        limits
    }
}

#[pymethods]
impl PyResourceLimits {
    /// Construct a policy from production-safe defaults, overriding any
    /// supplied keyword fields. A zero ceiling intentionally rejects the
    /// first use of that resource.
    #[new]
    #[pyo3(signature = (*, max_netlist_bytes=None, max_netlist_lines=None,
                        max_expanded_source_bytes=None, max_dependency_source_bytes=None,
                        max_external_data_bytes=None, max_external_data_values=None,
                        max_shared_cache_bytes=None, max_include_depth=None,
                        max_hierarchy_depth=None, max_flattened_elements=None,
                        max_circuit_nodes=None, max_matrix_unknowns=None,
                        max_analysis_points=None, max_result_values=None,
                        max_batch_runs=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        max_netlist_bytes: Option<usize>,
        max_netlist_lines: Option<usize>,
        max_expanded_source_bytes: Option<usize>,
        max_dependency_source_bytes: Option<usize>,
        max_external_data_bytes: Option<usize>,
        max_external_data_values: Option<usize>,
        max_shared_cache_bytes: Option<usize>,
        max_include_depth: Option<usize>,
        max_hierarchy_depth: Option<usize>,
        max_flattened_elements: Option<usize>,
        max_circuit_nodes: Option<usize>,
        max_matrix_unknowns: Option<usize>,
        max_analysis_points: Option<usize>,
        max_result_values: Option<usize>,
        max_batch_runs: Option<usize>,
    ) -> Self {
        let mut limits = Self::from_core(ResourceLimits::default());
        if let Some(value) = max_netlist_bytes {
            limits.max_netlist_bytes = value;
        }
        if let Some(value) = max_netlist_lines {
            limits.max_netlist_lines = value;
        }
        if let Some(value) = max_expanded_source_bytes {
            limits.max_expanded_source_bytes = value;
        }
        if let Some(value) = max_dependency_source_bytes {
            limits.max_dependency_source_bytes = value;
        }
        if let Some(value) = max_external_data_bytes {
            limits.max_external_data_bytes = value;
        }
        if let Some(value) = max_external_data_values {
            limits.max_external_data_values = value;
        }
        if let Some(value) = max_shared_cache_bytes {
            limits.max_shared_cache_bytes = value;
        }
        if let Some(value) = max_include_depth {
            limits.max_include_depth = value;
        }
        if let Some(value) = max_hierarchy_depth {
            limits.max_hierarchy_depth = value;
        }
        if let Some(value) = max_flattened_elements {
            limits.max_flattened_elements = value;
        }
        if let Some(value) = max_circuit_nodes {
            limits.max_circuit_nodes = value;
        }
        if let Some(value) = max_matrix_unknowns {
            limits.max_matrix_unknowns = value;
        }
        if let Some(value) = max_analysis_points {
            limits.max_analysis_points = value;
        }
        if let Some(value) = max_result_values {
            limits.max_result_values = value;
        }
        if let Some(value) = max_batch_runs {
            limits.max_batch_runs = value;
        }
        limits
    }

    /// Construct a policy with every ceiling disabled.
    #[staticmethod]
    pub fn unlimited() -> Self {
        Self::from_core(ResourceLimits::unlimited())
    }

    fn __repr__(&self) -> String {
        format!(
            "ResourceLimits(max_netlist_bytes={}, max_analysis_points={}, max_result_values={}, max_batch_runs={})",
            self.max_netlist_bytes,
            self.max_analysis_points,
            self.max_result_values,
            self.max_batch_runs
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
/// Note: property getters for `convergence`, `bypass`, and `resource_limits`
/// return copies.
/// `config.convergence.verbose = True` mutates a temporary and is silently
/// lost — assign a whole ConvergenceConfig instead.
#[pyclass(name = "SimulationConfig", module = "rspice", from_py_object)]
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
    ///     resource_limits: Parsing, construction, analysis, and cache ceilings
    #[new]
    #[pyo3(signature = (*, tolerance=None, max_iterations=None, transient_max_iterations=None,
                        min_timestep=None, max_timestep=None, temperature=None,
                        integration_method=None, transient_trtol=None,
                        convergence=None, bypass=None, resource_limits=None))]
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
        resource_limits: Option<PyResourceLimits>,
    ) -> PyResult<Self> {
        let mut inner = SimulationConfig::default();
        if let Some(v) = tolerance {
            inner.tolerance = validate_positive("tolerance", v)?;
        }
        if let Some(v) = max_iterations {
            inner.max_iterations = validate_positive_usize("max_iterations", v)?;
        }
        if let Some(v) = transient_max_iterations {
            inner.transient_max_iterations =
                validate_positive_usize("transient_max_iterations", v)?;
        }
        if let Some(v) = min_timestep {
            inner.min_timestep = validate_positive("min_timestep", v)?;
        }
        if let Some(v) = max_timestep {
            inner.max_timestep = validate_positive("max_timestep", v)?;
        }
        if let Some(v) = temperature {
            inner.temperature = validate_positive("temperature", v)?;
        }
        if let Some(v) = integration_method {
            inner.integration_method = v.into();
        }
        if let Some(v) = transient_trtol {
            inner.transient_trtol = validate_positive("transient_trtol", v)?;
        }
        if let Some(v) = convergence {
            inner.convergence_config = v.inner;
        }
        if let Some(v) = bypass {
            inner.bypass_config = v.inner;
        }
        if let Some(v) = resource_limits {
            inner.resource_limits = v.to_core();
        }
        validate_timestep_window(inner.min_timestep, inner.max_timestep)?;
        Ok(Self { inner })
    }

    /// Convergence tolerance for Newton-Raphson
    #[getter]
    fn get_tolerance(&self) -> f64 {
        self.inner.tolerance
    }

    #[setter]
    pub fn set_tolerance(&mut self, value: f64) -> PyResult<()> {
        self.inner.tolerance = validate_positive("tolerance", value)?;
        Ok(())
    }

    /// Maximum Newton-Raphson iterations (DC)
    #[getter]
    fn get_max_iterations(&self) -> usize {
        self.inner.max_iterations
    }

    #[setter]
    pub fn set_max_iterations(&mut self, value: usize) -> PyResult<()> {
        self.inner.max_iterations = validate_positive_usize("max_iterations", value)?;
        Ok(())
    }

    /// Maximum Newton-Raphson iterations per transient timestep (ITL4)
    #[getter]
    fn get_transient_max_iterations(&self) -> usize {
        self.inner.transient_max_iterations
    }

    #[setter]
    fn set_transient_max_iterations(&mut self, value: usize) -> PyResult<()> {
        self.inner.transient_max_iterations =
            validate_positive_usize("transient_max_iterations", value)?;
        Ok(())
    }

    /// Minimum timestep for transient analysis
    #[getter]
    fn get_min_timestep(&self) -> f64 {
        self.inner.min_timestep
    }

    #[setter]
    fn set_min_timestep(&mut self, value: f64) -> PyResult<()> {
        let value = validate_positive("min_timestep", value)?;
        validate_timestep_window(value, self.inner.max_timestep)?;
        self.inner.min_timestep = value;
        Ok(())
    }

    /// Maximum timestep for transient analysis
    #[getter]
    fn get_max_timestep(&self) -> f64 {
        self.inner.max_timestep
    }

    #[setter]
    fn set_max_timestep(&mut self, value: f64) -> PyResult<()> {
        let value = validate_positive("max_timestep", value)?;
        validate_timestep_window(self.inner.min_timestep, value)?;
        self.inner.max_timestep = value;
        Ok(())
    }

    /// Temperature in Kelvin
    #[getter]
    fn get_temperature(&self) -> f64 {
        self.inner.temperature
    }

    #[setter]
    pub fn set_temperature(&mut self, value: f64) -> PyResult<()> {
        self.inner.temperature = validate_positive("temperature", value)?;
        Ok(())
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
    fn set_transient_trtol(&mut self, value: f64) -> PyResult<()> {
        self.inner.transient_trtol = validate_positive("transient_trtol", value)?;
        Ok(())
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

    /// Resource policy (returns a copy; assign back to modify)
    #[getter]
    fn get_resource_limits(&self) -> PyResourceLimits {
        PyResourceLimits::from_core(self.inner.resource_limits)
    }

    #[setter]
    pub fn set_resource_limits(&mut self, value: PyResourceLimits) {
        self.inner.resource_limits = value.to_core();
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
