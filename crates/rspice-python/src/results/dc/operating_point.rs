//! One solved DC operating point and its per-device projection.
//!
//! `SimulationResult` is also the point type a DC sweep, a `.STEP` run and a
//! `.TEMP` run are made of, so it is deliberately free of any notion of a
//! sweep axis.

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
    /// `None` when the producing analysis captured no device operating-point
    /// report at all, which is a different fact from "the circuit has no
    /// devices with operating points". An empty list means the latter.
    device_operating_points: Option<Vec<PyDeviceOperatingPoint>>,
}

impl PySimulationResult {
    /// A solution whose producer captured no device operating-point report.
    pub fn new(inner: SimulationResult) -> Self {
        Self {
            inner,
            device_operating_points: None,
        }
    }

    pub fn new_with_report(
        inner: SimulationResult,
        report: rspice_core::circuit::DeviceOpReport,
    ) -> Self {
        Self {
            inner,
            device_operating_points: Some(
                report
                    .entries
                    .into_iter()
                    .map(PyDeviceOperatingPoint::from_core)
                    .collect(),
            ),
        }
    }

    pub(super) fn new_with_device_operating_points(
        inner: SimulationResult,
        device_operating_points: Option<Vec<PyDeviceOperatingPoint>>,
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
    ///
    /// `None` when the analysis that produced this solution captured no
    /// report; an empty list when it captured one and the circuit has no
    /// device with an operating point. The two are not the same fact.
    #[getter]
    fn device_operating_points(&self) -> Option<Vec<PyDeviceOperatingPoint>> {
        self.device_operating_points.clone()
    }

    /// Whether this solution carries a device operating-point report.
    #[getter]
    fn has_device_operating_points(&self) -> bool {
        self.device_operating_points.is_some()
    }

    /// Look up one device's operating-point summary (case-insensitive).
    ///
    /// Raises `KeyError` naming the absent report rather than the device when
    /// the producing analysis captured none, so "this run did not record them"
    /// never reads as "your circuit does not contain that device".
    fn device_operating_point(&self, name: &str) -> PyResult<PyDeviceOperatingPoint> {
        let entries = self.device_operating_points.as_ref().ok_or_else(|| {
            crate::errors::key_error(
                "this result carries no device operating-point report; it was produced by an \
                 analysis that does not capture one",
            )
        })?;
        entries
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
        device_operating_points: Option<Vec<PyDeviceOperatingPoint>>,
    ) -> Self {
        Self::new_with_device_operating_points(
            rebuild_simulation_result(state),
            device_operating_points,
        )
    }

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

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (SimulationResultState, Option<Vec<PyDeviceOperatingPoint>>),
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
