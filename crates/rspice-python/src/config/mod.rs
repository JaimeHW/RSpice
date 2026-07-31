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
//! `SimulationConfig`, `ConvergenceConfig`, `BypassConfig`, and
//! `ResourceLimits` all accept keyword arguments in their constructors so a
//! complete configuration can be built in one expression. This matters
//! because property getters return *copies* (Rust value semantics): mutating
//! `config.convergence.gmin_stepping` modifies a temporary and is lost.
//! Either build with keywords or assign whole sub-objects back.

use pyo3::prelude::*;
use rspice_core::ResourceLimits;
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::engine::{BypassConfig, ConvergenceConfig, DampingStrategy, SimulationConfig};

impl PartialEq for PySimulationConfig {
    fn eq(&self, other: &Self) -> bool {
        let (left, right) = (&self.inner, &other.inner);
        left.max_iterations == right.max_iterations
            && left.transient_max_iterations == right.transient_max_iterations
            && left.integration_method == right.integration_method
            && same_float(left.tolerance, right.tolerance)
            && same_float(left.min_timestep, right.min_timestep)
            && same_float(left.max_timestep, right.max_timestep)
            && same_float(left.temperature, right.temperature)
            && same_float(left.transient_trtol, right.transient_trtol)
            && PyConvergenceConfig {
                inner: left.convergence_config.clone(),
            } == PyConvergenceConfig {
                inner: right.convergence_config.clone(),
            }
            && PyBypassConfig {
                inner: left.bypass_config.clone(),
            } == PyBypassConfig {
                inner: right.bypass_config.clone(),
            }
            && PyResourceLimits::from_core(left.resource_limits)
                == PyResourceLimits::from_core(right.resource_limits)
    }
}

mod convergence;
mod enums;
mod limits;
mod validate;

pub(crate) use convergence::{PyBypassConfig, PyConvergenceConfig};
pub(crate) use enums::{PyDampingStrategy, PyIntegrationMethod};
pub(crate) use limits::PyResourceLimits;

use validate::*;

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
///
/// The default transient timestep ceiling is unbounded, and `max_timestep`
/// reads back as `inf`. Pass a finite value when the application requires a
/// product-level ceiling, and assign `float('inf')` to lift it again.
#[pyclass(name = "SimulationConfig", module = "rspice", from_py_object, eq)]
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
    ///     max_timestep: Maximum transient timestep (seconds), or
    ///                   float('inf') for no cap. Omitted keeps the
    ///                   unbounded default.
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
            inner.max_timestep = validate_positive_or_unbounded("max_timestep", v)?;
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

    /// Maximum timestep for transient analysis; `inf` means no cap
    #[getter]
    fn get_max_timestep(&self) -> f64 {
        self.inner.max_timestep
    }

    #[setter]
    fn set_max_timestep(&mut self, value: f64) -> PyResult<()> {
        let value = validate_positive_or_unbounded("max_timestep", value)?;
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

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(
        tolerance: f64,
        max_iterations: usize,
        transient_max_iterations: usize,
        min_timestep: f64,
        max_timestep: f64,
        temperature: f64,
        integration_method: PyIntegrationMethod,
        transient_trtol: f64,
        convergence: PyConvergenceConfig,
        bypass: PyBypassConfig,
        resource_limits: PyResourceLimits,
    ) -> PyResult<Self> {
        Self::new(
            Some(tolerance),
            Some(max_iterations),
            Some(transient_max_iterations),
            Some(min_timestep),
            Some(max_timestep),
            Some(temperature),
            Some(integration_method),
            Some(transient_trtol),
            Some(convergence),
            Some(bypass),
            Some(resource_limits),
        )
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            f64,
            usize,
            usize,
            f64,
            f64,
            f64,
            PyIntegrationMethod,
            f64,
            PyConvergenceConfig,
            PyBypassConfig,
            PyResourceLimits,
        ),
    )> {
        Ok((
            py.get_type::<Self>().getattr("_unpickle")?,
            (
                self.inner.tolerance,
                self.inner.max_iterations,
                self.inner.transient_max_iterations,
                self.inner.min_timestep,
                self.inner.max_timestep,
                self.inner.temperature,
                self.inner.integration_method.into(),
                self.inner.transient_trtol,
                self.get_convergence(),
                self.get_bypass(),
                self.get_resource_limits(),
            ),
        ))
    }
}
