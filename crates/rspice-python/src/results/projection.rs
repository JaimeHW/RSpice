//! `.SAVE`/`.PROBE`/`.PRINT`/`.PLOT` projection for Python results.
//!
//! Whole-result access stays exactly as it was; this module adds the authored
//! view. The deck's output contract is resolved by the one core
//! [`SignalProjection`](rspice_core::execution::SignalProjection), so a
//! `@device[param]` observable, a differential probe, or a wildcard means here
//! what it means on the CLI. A symbol the analysis cannot supply raises the
//! typed `RequestedSignalUnavailable` error rather than yielding a shorter
//! list.

use numpy::{PyArray1, ToPyArray};
use pyo3::prelude::*;
use rspice_core::execution::{
    AnalysisResultKind, ProjectedSignal, ProjectionSource, ProjectionSourceSignal, SignalProjection,
};

/// One column an authored output directive selected.
#[pyclass(name = "ProjectedSignal", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyProjectedSignal {
    /// Authored spelling of the selected signal, e.g. `V(out)` or `@d1[id]`.
    #[pyo3(get)]
    pub name: String,
    /// Typed signal kind: `voltage`, `current`, `digital`, or `parameter`.
    #[pyo3(get)]
    pub kind: String,
    values: Vec<f64>,
    validity: Vec<bool>,
}

#[pymethods]
impl PyProjectedSignal {
    /// Sample values in analysis order.
    #[getter]
    fn values<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.values.to_pyarray(py)
    }

    /// Per-sample presence. A `False` entry is an absent sample, never a zero.
    #[getter]
    fn validity(&self) -> Vec<bool> {
        self.validity.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "ProjectedSignal(name='{}', kind='{}', samples={})",
            self.name,
            self.kind,
            self.values.len()
        )
    }
}

impl PyProjectedSignal {
    /// Convert one projected real column.
    ///
    /// A complex column has no real-valued Python view, so it is rejected
    /// rather than silently reported as its real part.
    fn from_real(signal: ProjectedSignal) -> PyResult<Self> {
        let name = signal.descriptor().display_name().to_string();
        let kind =
            rspice_core::execution::raw_variable_type(signal.descriptor().kind()).to_string();
        let validity = signal.validity().to_vec();
        let values = signal.real().map(<[f64]>::to_vec).ok_or_else(|| {
            crate::errors::value_error(format!(
                "projected signal '{name}' is complex and has no real-valued view"
            ))
        })?;
        Ok(Self {
            name,
            kind,
            values,
            validity,
        })
    }
}

/// Project one real-valued analysis result onto a deck's output contract.
pub(crate) fn project_real(
    netlist: &rspice_core::Netlist,
    kind: AnalysisResultKind,
    instance: &str,
    axis: &[f64],
    signals: Vec<ProjectionSourceSignal<'_>>,
    lookup: std::collections::HashMap<String, &[f64]>,
    ordered: Option<Vec<ProjectedSignal>>,
) -> PyResult<Vec<PyProjectedSignal>> {
    let projection = SignalProjection::from_netlist(netlist)
        .map_err(crate::errors::simulation_error_to_pyerr)?;
    let source = ProjectionSource::new(kind, instance)
        .with_axis(axis)
        .with_signals(signals)
        .with_lookup(lookup)
        .with_ordered_print_columns(ordered);
    let projected = projection
        .project(
            &netlist.params,
            &source,
            &rspice_core::abort_signal::NoAbort,
        )
        .map_err(crate::errors::simulation_error_to_pyerr)?;
    projected
        .into_signals()
        .into_iter()
        .map(PyProjectedSignal::from_real)
        .collect()
}
