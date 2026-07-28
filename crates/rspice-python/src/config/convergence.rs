//! Convergence aids and model-evaluation bypass.
//!
//! `ConvergenceConfig` holds the continuation strategies a hard DC operating
//! point needs: gmin stepping, source stepping, and the damping schedule.
//! `BypassConfig` governs latent-device bypass, which skips re-evaluating a
//! model whose terminal voltages have not moved.

use super::*;

impl PartialEq for PyBypassConfig {
    fn eq(&self, other: &Self) -> bool {
        self.inner.enabled == other.inner.enabled
            && same_float(self.inner.reltol, other.inner.reltol)
            && same_float(self.inner.abstol, other.inner.abstol)
    }
}

impl PartialEq for PyConvergenceConfig {
    fn eq(&self, other: &Self) -> bool {
        let (left, right) = (&self.inner, &other.inner);
        left.gmin_stepping == right.gmin_stepping
            && left.source_stepping == right.source_stepping
            && left.pseudo_transient == right.pseudo_transient
            && left.arc_length == right.arc_length
            && left.damping_strategy == right.damping_strategy
            && left.verbose == right.verbose
            && same_float(left.gmin_initial, right.gmin_initial)
            && same_float(left.gmin_target, right.gmin_target)
            && same_float(left.voltage_reltol, right.voltage_reltol)
            && same_float(left.residual_reltol, right.residual_reltol)
            && same_float(left.voltage_abstol, right.voltage_abstol)
            && same_float(left.current_abstol, right.current_abstol)
            && same_float(left.charge_abstol, right.charge_abstol)
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
#[pyclass(name = "BypassConfig", module = "rspice", from_py_object, eq)]
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

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(enabled: bool, reltol: f64, abstol: f64) -> PyResult<Self> {
        Self::new(Some(enabled), Some(reltol), Some(abstol))
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyAny>, (bool, f64, f64))> {
        Ok((
            py.get_type::<Self>().getattr("_unpickle")?,
            (self.inner.enabled, self.inner.reltol, self.inner.abstol),
        ))
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
#[pyclass(name = "ConvergenceConfig", module = "rspice", from_py_object, eq)]
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

    /// Rebuild from pickled state. Not part of the public API.
    ///
    /// State is grouped into flags, the damping strategy, and the tolerance
    /// vector so it stays inside PyO3's tuple conversion limit and reads as
    /// three coherent groups rather than a thirteen-slot positional list.
    #[staticmethod]
    fn _unpickle(
        flags: (bool, bool, bool, bool, bool),
        damping_strategy: PyDampingStrategy,
        tolerances: [f64; 7],
    ) -> PyResult<Self> {
        let (gmin_stepping, source_stepping, pseudo_transient, arc_length, verbose) = flags;
        Self::new(
            Some(gmin_stepping),
            Some(source_stepping),
            Some(pseudo_transient),
            Some(arc_length),
            Some(damping_strategy),
            Some(tolerances[0]),
            Some(tolerances[1]),
            Some(tolerances[2]),
            Some(tolerances[3]),
            Some(tolerances[4]),
            Some(tolerances[5]),
            Some(tolerances[6]),
            Some(verbose),
        )
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        ((bool, bool, bool, bool, bool), PyDampingStrategy, [f64; 7]),
    )> {
        Ok((
            py.get_type::<Self>().getattr("_unpickle")?,
            (
                (
                    self.inner.gmin_stepping,
                    self.inner.source_stepping,
                    self.inner.pseudo_transient,
                    self.inner.arc_length,
                    self.inner.verbose,
                ),
                self.inner.damping_strategy.into(),
                [
                    self.inner.gmin_initial,
                    self.inner.gmin_target,
                    self.inner.voltage_reltol,
                    self.inner.residual_reltol,
                    self.inner.voltage_abstol,
                    self.inner.current_abstol,
                    self.inner.charge_abstol,
                ],
            ),
        ))
    }
}
