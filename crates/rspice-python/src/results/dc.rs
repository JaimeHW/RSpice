//! DC operating-point and sweep results.
//!
//! `SimulationResult` is the single-point operating solution and `DcSweepResult`
//! a swept sequence of them. `.STEP` and `.TEMP` runs produce the same shape and
//! reuse `DcSweepResult` rather than duplicating it. `DeviceOperatingPoint` is
//! the per-device small-signal projection taken at an operating point.

use super::*;

/// DC operating point simulation result
///
/// Contains node voltages and branch currents from a DC operating point
/// analysis. Access voltages by node index or name.
///
/// Example:
///     >>> result = engine.run_dc_op(netlist)
///     >>> v1 = result.voltage(1)
///     >>> v_out = result.voltage("out")
#[pyclass(name = "SimulationResult", module = "rspice")]
pub struct PySimulationResult {
    pub(crate) inner: SimulationResult,
    device_operating_points: Vec<PyDeviceOperatingPoint>,
}

impl PySimulationResult {
    pub fn new(inner: SimulationResult) -> Self {
        Self {
            inner,
            device_operating_points: Vec::new(),
        }
    }

    pub fn new_with_report(
        inner: SimulationResult,
        report: rspice_core::circuit::DeviceOpReport,
    ) -> Self {
        Self {
            inner,
            device_operating_points: report
                .entries
                .into_iter()
                .map(PyDeviceOperatingPoint::from_core)
                .collect(),
        }
    }

    fn new_with_device_operating_points(
        inner: SimulationResult,
        device_operating_points: Vec<PyDeviceOperatingPoint>,
    ) -> Self {
        Self {
            inner,
            device_operating_points,
        }
    }

    fn checked_voltage(&self, node: usize) -> AccessResult<f64> {
        checked_simulation_voltage(&self.inner, node)
    }

    fn checked_voltage_named(&self, name: &str) -> AccessResult<f64> {
        checked_simulation_voltage_named(&self.inner, name)
    }
}

#[pymethods]
impl PySimulationResult {
    /// Project this operating point onto a deck's authored output contract
    ///
    /// Returns the columns the deck's `.SAVE`, `.PROBE`, `.PRINT OP` and
    /// `.PLOT OP` cards select, each with its per-sample validity. Device
    /// observables resolve through the same `@device[param]` grammar the CLI
    /// uses. Whole-result access is unaffected.
    ///
    /// Args:
    ///     netlist: The parsed deck whose output cards to apply
    ///
    /// Returns:
    ///     list[ProjectedSignal]: Selected columns in authored order
    ///
    /// Raises:
    ///     RequestedSignalUnavailableError: If an authored symbol is absent
    ///
    /// Example:
    ///     >>> [s.name for s in op.saved_signals(netlist)]
    fn saved_signals(
        &self,
        netlist: &crate::netlist::PyNetlist,
    ) -> PyResult<Vec<crate::results::PyProjectedSignal>> {
        let inventory = rspice_core::execution::operating_point_projection_signals(&self.inner)
            .map_err(|error| crate::errors::value_error(error.to_string()))?;
        let observables = rspice_core::execution::operating_point_observable_series(&self.inner);
        crate::results::projection::project_real(
            &netlist.inner,
            rspice_core::execution::AnalysisResultKind::OperatingPoint,
            "DC OP",
            &[0.0],
            inventory,
            rspice_core::execution::observable_lookup(&observables),
            None,
        )
    }

    /// Get voltage at a node by index or name
    ///
    /// Args:
    ///     node: Node index (int) or node name (str)
    ///
    /// Returns:
    ///     float: Voltage at the specified node
    ///
    /// Raises:
    ///     IndexError: If the node index is out of range
    ///     KeyError: If the node name does not exist
    ///
    /// Example:
    ///     >>> v = result.voltage(1)      # By index
    ///     >>> v = result.voltage("out")  # By name
    fn voltage(&self, node: NodeIdentifier) -> PyResult<f64> {
        match node {
            NodeIdentifier::Index(idx) => self.checked_voltage(idx),
            NodeIdentifier::Name(name) => self.checked_voltage_named(&name),
        }
        .map_err(PyErr::from)
    }

    /// Get all node voltages as a NumPy array
    ///
    /// Returns:
    ///     numpy.ndarray: Array of all node voltages (index 0 = ground = 0V)
    #[getter]
    fn node_voltages<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.node_voltages.to_pyarray(py)
    }

    /// Get all node names
    ///
    /// Returns:
    ///     list[str]: List of node names indexed by node ID
    #[getter]
    pub fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    /// Get branch current by element name
    ///
    /// Args:
    ///     name: Element name (e.g., "V1", "L1")
    ///
    /// Returns:
    ///     float: Current through the element
    ///
    /// Raises:
    ///     KeyError: If no branch carries that name
    fn branch_current(&self, name: &str) -> PyResult<f64> {
        self.inner
            .branch_current_named(name)
            .ok_or_else(|| unknown_branch_name_error(name))
            .map_err(PyErr::from)
    }

    /// Per-device operating-point summaries captured by DC OP analysis.
    #[getter]
    fn device_operating_points(&self) -> Vec<PyDeviceOperatingPoint> {
        self.device_operating_points.clone()
    }

    /// Look up one device's operating-point summary (case-insensitive).
    fn device_operating_point(&self, name: &str) -> PyResult<PyDeviceOperatingPoint> {
        self.device_operating_points
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
            .cloned()
            .ok_or_else(|| crate::errors::key_error(format!("unknown device '{name}'")))
    }

    /// Get all branch currents as a NumPy array
    ///
    /// Returns:
    ///     numpy.ndarray: Array of all branch currents
    #[getter]
    fn branch_currents<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.branch_currents.to_pyarray(py)
    }

    /// Get canonical branch names aligned with `branch_currents`.
    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.inner.branch_names.clone()
    }

    /// Number of nodes in the circuit (excluding ground)
    #[getter]
    pub fn num_nodes(&self) -> usize {
        self.inner.node_voltages.len().saturating_sub(1)
    }

    fn __repr__(&self) -> String {
        format!(
            "SimulationResult(nodes={}, branches={})",
            self.num_nodes(),
            self.inner.branch_currents.len()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        state: SimulationResultState,
        device_operating_points: Vec<PyDeviceOperatingPoint>,
    ) -> Self {
        Self::new_with_device_operating_points(
            rebuild_simulation_result(state),
            device_operating_points,
        )
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (SimulationResultState, Vec<PyDeviceOperatingPoint>),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                simulation_result_state(&self.inner),
                self.device_operating_points.clone(),
            ),
        ))
    }
}

/// Spectre-style operating-point information for one device instance.
#[pyclass(name = "DeviceOperatingPoint", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyDeviceOperatingPoint {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub device_kind: String,
    #[pyo3(get)]
    pub region: Option<String>,
    params: Vec<(String, f64)>,
}

impl PyDeviceOperatingPoint {
    pub(crate) fn from_core(entry: rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: entry.name,
            device_kind: entry.device_kind.to_string(),
            region: entry.region.map(str::to_string),
            params: entry
                .params
                .into_iter()
                .map(|(name, value)| (name.to_string(), value))
                .collect(),
        }
    }
}

#[pymethods]
impl PyDeviceOperatingPoint {
    /// Named operating-point quantities in stable display order.
    #[getter]
    fn params<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let result = PyDict::new(py);
        for (name, value) in &self.params {
            result.set_item(name, value)?;
        }
        Ok(result)
    }

    #[getter]
    fn param_names(&self) -> Vec<String> {
        self.params.iter().map(|(name, _)| name.clone()).collect()
    }

    /// Read one operating-point quantity (case-insensitive).
    fn param(&self, name: &str) -> PyResult<f64> {
        self.params
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
            .ok_or_else(|| {
                crate::errors::key_error(format!(
                    "device '{}' has no operating-point parameter '{name}'",
                    self.name
                ))
            })
    }

    fn __getitem__(&self, name: &str) -> PyResult<f64> {
        self.param(name)
    }

    fn __repr__(&self) -> String {
        format!(
            "DeviceOperatingPoint(name='{}', kind='{}', region={:?}, parameters={})",
            self.name,
            self.device_kind,
            self.region,
            self.params.len()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        name: String,
        device_kind: String,
        region: Option<String>,
        params: Vec<(String, f64)>,
    ) -> Self {
        Self {
            name,
            device_kind,
            region,
            params,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (String, String, Option<String>, Vec<(String, f64)>),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.name.clone(),
                self.device_kind.clone(),
                self.region.clone(),
                self.params.clone(),
            ),
        ))
    }
}

/// DC sweep analysis result
///
/// A sequence of (sweep_value, SimulationResult) pairs. Supports `len()`,
/// indexing (including negative indices), and iteration:
///
///     >>> for v_in, sol in engine.run_dc_sweep(netlist, "V1", 0, 5, 0.1):
///     ...     print(v_in, sol.voltage("out"))
#[pyclass(name = "DcSweepResult", module = "rspice", from_py_object)]
#[derive(Clone)]
pub struct PyDcSweepResult {
    pub(crate) results: Vec<(f64, SimulationResult)>,
    device_operating_points: Vec<Vec<PyDeviceOperatingPoint>>,
    primary_source: Option<String>,
    secondary_source: Option<String>,
    secondary_sweep_values: Option<Vec<f64>>,
    inner_points: usize,
}

impl PyDcSweepResult {
    pub fn new(results: Vec<(f64, SimulationResult)>) -> Self {
        let inner_points = results.len();
        let device_operating_points = vec![Vec::new(); results.len()];
        Self {
            results,
            device_operating_points,
            primary_source: None,
            secondary_source: None,
            secondary_sweep_values: None,
            inner_points,
        }
    }

    pub fn new_named(results: Vec<(f64, SimulationResult)>, primary_source: &str) -> Self {
        let mut result = Self::new(results);
        result.primary_source = Some(primary_source.to_string());
        result
    }

    pub fn new_named_with_reports(
        results: Vec<rspice_core::engine::DcSweepPointResult>,
        primary_source: &str,
    ) -> Self {
        let inner_points = results.len();
        let (points, device_operating_points) = results
            .into_iter()
            .map(|point| {
                (
                    (point.sweep_value, point.result),
                    point
                        .device_op_report
                        .entries
                        .into_iter()
                        .map(PyDeviceOperatingPoint::from_core)
                        .collect::<Vec<_>>(),
                )
            })
            .unzip();
        Self {
            results: points,
            device_operating_points,
            primary_source: Some(primary_source.to_string()),
            secondary_source: None,
            secondary_sweep_values: None,
            inner_points,
        }
    }

    pub fn new_nested(
        results: Vec<(f64, SimulationResult)>,
        primary_source: &str,
        secondary_source: &str,
        secondary_sweep_values: Vec<f64>,
    ) -> PyResult<Self> {
        if secondary_sweep_values.is_empty()
            || !results.len().is_multiple_of(secondary_sweep_values.len())
        {
            return Err(crate::errors::SimulationError::new_err(format!(
                "malformed nested DC sweep: {} result points cannot be divided across {} secondary sweep values",
                results.len(),
                secondary_sweep_values.len()
            )));
        }
        let inner_points = results.len() / secondary_sweep_values.len();
        Ok(Self {
            results,
            device_operating_points: vec![Vec::new(); inner_points * secondary_sweep_values.len()],
            primary_source: Some(primary_source.to_string()),
            secondary_source: Some(secondary_source.to_string()),
            secondary_sweep_values: Some(secondary_sweep_values),
            inner_points,
        })
    }

    pub fn new_nested_with_reports(
        results: Vec<rspice_core::engine::DcSweepPointResult>,
        primary_source: &str,
        secondary_source: &str,
        secondary_sweep_values: Vec<f64>,
    ) -> PyResult<Self> {
        if secondary_sweep_values.is_empty()
            || !results.len().is_multiple_of(secondary_sweep_values.len())
        {
            return Err(crate::errors::SimulationError::new_err(format!(
                "malformed nested DC sweep: {} result points cannot be divided across {} secondary sweep values",
                results.len(),
                secondary_sweep_values.len()
            )));
        }
        let inner_points = results.len() / secondary_sweep_values.len();
        let (points, device_operating_points) = results
            .into_iter()
            .map(|point| {
                (
                    (point.sweep_value, point.result),
                    point
                        .device_op_report
                        .entries
                        .into_iter()
                        .map(PyDeviceOperatingPoint::from_core)
                        .collect::<Vec<_>>(),
                )
            })
            .unzip();
        Ok(Self {
            results: points,
            device_operating_points,
            primary_source: Some(primary_source.to_string()),
            secondary_source: Some(secondary_source.to_string()),
            secondary_sweep_values: Some(secondary_sweep_values),
            inner_points,
        })
    }

    fn point(&self, index: usize) -> AccessResult<&(f64, SimulationResult)> {
        self.results
            .get(index)
            .ok_or_else(|| invalid_sweep_index_error(index, self.results.len()))
    }
}

#[pymethods]
impl PyDcSweepResult {
    /// Name of the primary (inner) sweep source, when known.
    #[getter]
    fn primary_source(&self) -> Option<String> {
        self.primary_source.clone()
    }

    /// Name of the secondary (outer) sweep source for a nested `.DC` sweep.
    #[getter]
    fn secondary_source(&self) -> Option<String> {
        self.secondary_source.clone()
    }

    /// Unique secondary sweep values in outer-loop order.
    #[getter]
    fn secondary_sweep_values<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.secondary_sweep_values
            .as_ref()
            .map(|values| values.to_pyarray(py))
    }

    /// `(outer_points, inner_points)` shape of the flattened sweep result.
    #[getter]
    fn shape(&self) -> (usize, usize) {
        (
            self.secondary_sweep_values.as_ref().map_or(1, Vec::len),
            self.inner_points,
        )
    }

    /// Whether this result came from a two-source nested `.DC` sweep.
    #[getter]
    fn is_nested(&self) -> bool {
        self.secondary_sweep_values.is_some()
    }

    /// Secondary sweep coordinate for a flattened result index.
    fn secondary_value_at(&self, index: usize) -> PyResult<Option<f64>> {
        self.point(index).map_err(PyErr::from)?;
        Ok(self.secondary_sweep_values.as_ref().map(|values| {
            let outer_index = index / self.inner_points;
            values[outer_index]
        }))
    }

    /// Get the sweep values array
    ///
    /// Returns:
    ///     numpy.ndarray: Array of swept source values
    #[getter]
    fn sweep_values<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        let values: Vec<f64> = self.results.iter().map(|(v, _)| *v).collect();
        values.to_pyarray(py)
    }

    /// Get the number of sweep points
    fn __len__(&self) -> usize {
        self.results.len()
    }

    /// Get all sweep points as (value, result) pairs
    ///
    /// Returns:
    ///     list[tuple[float, SimulationResult]]: One entry per sweep point
    pub fn points(&self) -> Vec<(f64, PySimulationResult)> {
        self.results
            .iter()
            .enumerate()
            .map(|(index, (value, result))| {
                (
                    *value,
                    PySimulationResult::new_with_device_operating_points(
                        result.clone(),
                        self.device_operating_points
                            .get(index)
                            .cloned()
                            .unwrap_or_default(),
                    ),
                )
            })
            .collect()
    }

    /// Device operating-point data at one sweep point.
    fn device_operating_points_at(&self, index: usize) -> PyResult<Vec<PyDeviceOperatingPoint>> {
        self.point(index).map_err(PyErr::from)?;
        Ok(self
            .device_operating_points
            .get(index)
            .cloned()
            .unwrap_or_default())
    }

    /// Iterate over (value, SimulationResult) pairs
    fn __iter__(slf: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let points = slf.points();
        let list = points.into_pyobject(py)?;
        Ok(list.call_method0("__iter__")?.unbind())
    }

    /// Index into the sweep: `result[i]` -> (value, SimulationResult)
    ///
    /// Supports negative indices.
    fn __getitem__(&self, index: isize) -> PyResult<(f64, PySimulationResult)> {
        let len = self.results.len() as isize;
        let idx = if index < 0 { index + len } else { index };
        if idx < 0 || idx >= len {
            return Err(invalid_sweep_index_error(index.unsigned_abs(), self.results.len()).into());
        }
        let (value, result) = &self.results[idx as usize];
        Ok((
            *value,
            PySimulationResult::new_with_device_operating_points(
                result.clone(),
                self.device_operating_points
                    .get(idx as usize)
                    .cloned()
                    .unwrap_or_default(),
            ),
        ))
    }

    /// Get the result at a specific sweep index
    ///
    /// Carries the same per-device operating-point data as `sweep[index]`
    /// and `points()`.
    ///
    /// Raises:
    ///     IndexError: If the sweep index is out of range
    pub fn result_at(&self, index: usize) -> PyResult<PySimulationResult> {
        let (_, result) = self.point(index).map_err(PyErr::from)?;
        Ok(PySimulationResult::new_with_device_operating_points(
            result.clone(),
            self.device_operating_points
                .get(index)
                .cloned()
                .unwrap_or_default(),
        ))
    }

    /// Get the sweep value at a specific index
    ///
    /// Raises:
    ///     IndexError: If the sweep index is out of range
    pub fn sweep_value_at(&self, index: usize) -> PyResult<f64> {
        self.point(index)
            .map(|(value, _)| *value)
            .map_err(PyErr::from)
    }

    /// Get voltage at a node for a specific sweep point
    ///
    /// Raises:
    ///     IndexError: If the sweep point or node index is out of range
    ///     KeyError: If the node name does not exist
    fn voltage(&self, index: usize, node: NodeIdentifier) -> PyResult<f64> {
        let (_, result) = self.point(index).map_err(PyErr::from)?;
        match node {
            NodeIdentifier::Index(idx) => checked_simulation_voltage(result, idx),
            NodeIdentifier::Name(name) => checked_simulation_voltage_named(result, &name),
        }
        .map_err(PyErr::from)
    }

    /// Get voltage at a node across all sweep points as a NumPy array
    ///
    /// Args:
    ///     node: Node index or name
    ///
    /// Raises:
    ///     IndexError / KeyError: For invalid nodes
    fn voltage_array<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let mut voltages = Vec::with_capacity(self.results.len());
        for (_, result) in &self.results {
            let v = match &node {
                NodeIdentifier::Index(idx) => checked_simulation_voltage(result, *idx),
                NodeIdentifier::Name(name) => checked_simulation_voltage_named(result, name),
            }
            .map_err(PyErr::from)?;
            voltages.push(v);
        }
        Ok(voltages.to_pyarray(py))
    }

    /// Column headers used by `to_csv` and the raw exporters, in order.
    #[getter]
    fn export_columns(&self) -> Vec<String> {
        self.raw_plot("")
            .variables
            .into_iter()
            .map(|v| v.name)
            .collect()
    }

    /// Render the sweep as RFC 4180 CSV.
    ///
    /// The first column is the swept source; the rest are `V(node)` and
    /// `I(element)`. A nested sweep is flattened in the same order as
    /// `points()`, with the secondary coordinate in its own column.
    fn to_csv(&self) -> PyResult<String> {
        let plot = self.raw_plot("");
        crate::export::csv(
            &plot
                .variables
                .iter()
                .map(|v| v.name.clone())
                .collect::<Vec<_>>(),
            &transpose_real(&plot.series),
        )
        .map_err(crate::errors::value_error)
    }

    /// Write `to_csv()` to a file.
    fn write_csv(&self, path: PathBuf) -> PyResult<()> {
        write_export_file(&path, self.to_csv()?.as_bytes())
    }

    /// Serialize as an ngspice-compatible SPICE raw file.
    ///
    /// Args:
    ///     format: "ascii" (default) or "binary"
    ///     title: Header title; defaults to "RSpice DC sweep"
    #[pyo3(signature = (*, format="ascii", title=None, timestamp=None))]
    fn to_raw<'py>(
        &self,
        py: Python<'py>,
        format: &str,
        title: Option<&str>,
        timestamp: Option<&str>,
    ) -> PyResult<Bound<'py, pyo3::types::PyBytes>> {
        let mut plot = self.raw_plot(title.unwrap_or("RSpice DC sweep"));
        plot.timestamp = timestamp.map(str::to_string);
        let bytes = raw_export_bytes(
            &plot,
            crate::export::RawFormat::parse(format).map_err(crate::errors::value_error)?,
        )?;
        Ok(pyo3::types::PyBytes::new(py, &bytes))
    }

    /// Write a SPICE raw file. Same options as `to_raw`.
    #[pyo3(signature = (path, *, format="ascii", title=None, timestamp=None))]
    fn write_raw(
        &self,
        path: PathBuf,
        format: &str,
        title: Option<&str>,
        timestamp: Option<&str>,
    ) -> PyResult<()> {
        let mut plot = self.raw_plot(title.unwrap_or("RSpice DC sweep"));
        plot.timestamp = timestamp.map(str::to_string);
        let bytes = raw_export_bytes(
            &plot,
            crate::export::RawFormat::parse(format).map_err(crate::errors::value_error)?,
        )?;
        write_export_file(&path, &bytes)
    }

    fn __repr__(&self) -> String {
        let sweep_range = match (self.results.first(), self.results.last()) {
            (Some(first), Some(last)) => format!("{:.2}-{:.2}", first.0, last.0),
            _ => "empty".to_string(),
        };
        format!(
            "DcSweepResult(points={}, range={}, shape={:?})",
            self.results.len(),
            sweep_range,
            self.shape()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(
        points: Vec<(f64, SimulationResultState)>,
        device_operating_points: Vec<Vec<PyDeviceOperatingPoint>>,
        primary_source: Option<String>,
        secondary_source: Option<String>,
        secondary_sweep_values: Option<Vec<f64>>,
        inner_points: usize,
    ) -> Self {
        Self {
            results: points
                .into_iter()
                .map(|(value, state)| (value, rebuild_simulation_result(state)))
                .collect(),
            device_operating_points,
            primary_source,
            secondary_source,
            secondary_sweep_values,
            inner_points,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            Vec<(f64, SimulationResultState)>,
            Vec<Vec<PyDeviceOperatingPoint>>,
            Option<String>,
            Option<String>,
            Option<Vec<f64>>,
            usize,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.results
                    .iter()
                    .map(|(value, result)| (*value, simulation_result_state(result)))
                    .collect(),
                self.device_operating_points.clone(),
                self.primary_source.clone(),
                self.secondary_source.clone(),
                self.secondary_sweep_values.clone(),
                self.inner_points,
            ),
        ))
    }
}

impl PyDcSweepResult {
    /// Column layout shared by the CSV and raw exporters.
    ///
    /// The sweep axis is a voltage rather than time or frequency, so it is
    /// declared as a voltage column, matching how ngspice writes DC plots.
    fn raw_plot(&self, title: &str) -> crate::export::RawPlot {
        use crate::export::{RawVariable, RawVariableKind};
        let complex = |values: Vec<f64>| -> Vec<rspice_core::Complex64> {
            values
                .into_iter()
                .map(|value| rspice_core::Complex64::new(value, 0.0))
                .collect()
        };

        let axis_name = self
            .primary_source
            .clone()
            .unwrap_or_else(|| "sweep".to_string());
        let mut variables = vec![RawVariable {
            name: format!("v-sweep({axis_name})"),
            kind: RawVariableKind::Voltage,
        }];
        let mut series = vec![complex(
            self.results.iter().map(|(value, _)| *value).collect(),
        )];

        if let (Some(secondary), Some(values)) =
            (&self.secondary_source, &self.secondary_sweep_values)
        {
            variables.push(RawVariable {
                name: format!("v-sweep2({secondary})"),
                kind: RawVariableKind::Voltage,
            });
            series.push(complex(
                (0..self.results.len())
                    .map(|index| {
                        values
                            .get(index / self.inner_points.max(1))
                            .copied()
                            .unwrap_or(f64::NAN)
                    })
                    .collect(),
            ));
        }

        // Every point of a sweep shares one circuit, so the first result's
        // name vectors define the columns.
        let (node_names, branch_names) = self.results.first().map_or_else(
            || (Vec::new(), Vec::new()),
            |(_, result)| (result.node_names.clone(), result.branch_names.clone()),
        );
        // node_names[0] is ground; it is a constant zero column and is
        // deliberately omitted, matching the transient and AC exporters.
        for (index, name) in node_names.iter().enumerate().skip(1) {
            variables.push(RawVariable {
                name: format!("V({name})"),
                kind: RawVariableKind::Voltage,
            });
            series.push(complex(
                self.results
                    .iter()
                    .map(|(_, result)| result.node_voltages.get(index).copied().unwrap_or(f64::NAN))
                    .collect(),
            ));
        }
        for (index, name) in branch_names.iter().enumerate() {
            variables.push(RawVariable {
                name: format!("I({name})"),
                kind: RawVariableKind::Current,
            });
            series.push(complex(
                self.results
                    .iter()
                    .map(|(_, result)| {
                        result
                            .branch_currents
                            .get(index)
                            .copied()
                            .unwrap_or(f64::NAN)
                    })
                    .collect(),
            ));
        }

        crate::export::RawPlot {
            title: title.to_string(),
            plot_name: "DC transfer characteristic".to_string(),
            variables,
            series,
            complex: false,
            timestamp: None,
        }
    }
}
