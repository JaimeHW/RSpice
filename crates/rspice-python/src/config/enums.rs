//! Newton damping and transient integration selectors.
//!
//! Both are plain enums mirroring core's, with conversions in each direction
//! and pickle support. They pickle by name rather than by ordinal so a stored
//! configuration survives a reordering of the underlying enum.

use super::*;

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

    /// Pickle as a reference to the class attribute, so an unpickled member
    /// is the same singleton the module exposes.
    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (Bound<'py, PyAny>, &'static str))> {
        enum_reduce(py, py.get_type::<Self>(), self.variant_name())
    }
}

impl PyDampingStrategy {
    fn variant_name(self) -> &'static str {
        match self {
            PyDampingStrategy::None => "NONE",
            PyDampingStrategy::LineSearch => "LINE_SEARCH",
            PyDampingStrategy::VoltageLimiting => "VOLTAGE_LIMITING",
            PyDampingStrategy::BankRose => "BANK_ROSE",
            PyDampingStrategy::Combined => "COMBINED",
        }
    }
}

/// A `__reduce__` payload for an enum member: the `getattr` builtin plus the
/// class and member name it should look up.
type EnumReduction<'py> = (Bound<'py, PyAny>, (Bound<'py, PyAny>, &'static str));

/// Shared `__reduce__` body for the module's simple enums.
///
/// `getattr(EnumClass, "MEMBER")` is picklable by reference and always
/// resolves to the canonical member, so round-tripping preserves identity
/// semantics rather than creating a detached copy.
fn enum_reduce<'py>(
    py: Python<'py>,
    class: Bound<'py, pyo3::types::PyType>,
    member: &'static str,
) -> PyResult<EnumReduction<'py>> {
    let getattr = py.import("builtins")?.getattr("getattr")?;
    Ok((getattr, (class.into_any(), member)))
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

    /// Pickle as a reference to the class attribute.
    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (Bound<'py, PyAny>, &'static str))> {
        enum_reduce(py, py.get_type::<Self>(), self.variant_name())
    }
}

impl PyIntegrationMethod {
    fn variant_name(self) -> &'static str {
        match self {
            PyIntegrationMethod::BackwardEuler => "BACKWARD_EULER",
            PyIntegrationMethod::Trapezoidal => "TRAPEZOIDAL",
            PyIntegrationMethod::Gear2 => "GEAR2",
            PyIntegrationMethod::TrapGear => "TRAP_GEAR",
        }
    }
}
