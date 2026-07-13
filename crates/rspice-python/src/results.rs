//! Simulation results Python bindings with NumPy integration
//!
//! Provides Python access to simulation results:
//! - `SimulationResult` - DC operating point results
//! - `TransientResult` - Time-domain waveforms (voltages and branch currents)
//! - `AcResult` - Frequency-domain complex phasors
//! - `DistortionResult` - Harmonic and two-tone Volterra products
//! - `DcSweepResult` - Collection of DC solutions
//! - `FourierResult` - Harmonic decomposition / THD of a waveform
//! - `TransferFunctionResult` - Small-signal gain and impedances
//! - `Measurement` / `RunReport` - .MEAS verification outcomes
//!
//! Error discipline: every accessor raises `IndexError` for out-of-range
//! indices and `KeyError` for unknown node/branch names — silent zeros are
//! never fabricated.

use numpy::{PyArray1, ToPyArray};
use pyo3::exceptions::{PyIndexError, PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rspice_core::analysis::{AcResult, DistortionAnalysisResult, DistortionProduct};
use rspice_core::analysis::{FourierAnalysis, FourierConfig};
use rspice_core::engine::TransientResult;
use rspice_core::solver::SimulationResult;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ResultAccessError {
    InvalidNodeIndex {
        node: usize,
        available_nodes: usize,
    },
    InvalidTimeIndex {
        time_index: usize,
        available_points: usize,
    },
    InvalidSweepIndex {
        index: usize,
        available_points: usize,
    },
    InvalidFreqIndex {
        index: usize,
        available_points: usize,
    },
    UnknownNodeName {
        name: String,
    },
    UnknownBranchName {
        name: String,
    },
}

impl From<ResultAccessError> for PyErr {
    fn from(error: ResultAccessError) -> Self {
        match error {
            ResultAccessError::InvalidNodeIndex {
                node,
                available_nodes,
            } => PyIndexError::new_err(format!(
                "node index {node} is out of range for result with {available_nodes} nodes"
            )),
            ResultAccessError::InvalidTimeIndex {
                time_index,
                available_points,
            } => PyIndexError::new_err(format!(
                "time index {time_index} is out of range for result with {available_points} points"
            )),
            ResultAccessError::InvalidSweepIndex {
                index,
                available_points,
            } => PyIndexError::new_err(format!(
                "sweep index {index} is out of range for result with {available_points} points"
            )),
            ResultAccessError::InvalidFreqIndex {
                index,
                available_points,
            } => PyIndexError::new_err(format!(
                "frequency index {index} is out of range for result with {available_points} points"
            )),
            ResultAccessError::UnknownNodeName { name } => {
                PyKeyError::new_err(format!("unknown node '{name}'"))
            }
            ResultAccessError::UnknownBranchName { name } => {
                PyKeyError::new_err(format!("unknown branch '{name}'"))
            }
        }
    }
}

type AccessResult<T> = Result<T, ResultAccessError>;

fn invalid_node_index_error(node: usize, available_nodes: usize) -> ResultAccessError {
    ResultAccessError::InvalidNodeIndex {
        node,
        available_nodes,
    }
}

fn invalid_time_index_error(time_index: usize, available_points: usize) -> ResultAccessError {
    ResultAccessError::InvalidTimeIndex {
        time_index,
        available_points,
    }
}

fn invalid_sweep_index_error(index: usize, available_points: usize) -> ResultAccessError {
    ResultAccessError::InvalidSweepIndex {
        index,
        available_points,
    }
}

fn invalid_freq_index_error(index: usize, available_points: usize) -> ResultAccessError {
    ResultAccessError::InvalidFreqIndex {
        index,
        available_points,
    }
}

fn unknown_node_name_error(name: &str) -> ResultAccessError {
    ResultAccessError::UnknownNodeName {
        name: name.to_string(),
    }
}

fn unknown_branch_name_error(name: &str) -> ResultAccessError {
    ResultAccessError::UnknownBranchName {
        name: name.to_string(),
    }
}

pub(crate) fn is_ground_name(name: &str) -> bool {
    matches!(name, "0") || name.eq_ignore_ascii_case("gnd")
}

fn checked_simulation_voltage(result: &SimulationResult, node: usize) -> AccessResult<f64> {
    result
        .try_voltage(node)
        .ok_or_else(|| invalid_node_index_error(node, result.node_voltages.len().saturating_sub(1)))
}

fn checked_simulation_voltage_named(result: &SimulationResult, name: &str) -> AccessResult<f64> {
    result
        .try_voltage_named(name)
        .ok_or_else(|| unknown_node_name_error(name))
}

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
            .ok_or_else(|| PyKeyError::new_err(format!("unknown device '{name}'")))
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
                PyKeyError::new_err(format!(
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
}

/// Memory-decimated transient voltage waveforms with bounded interpolation error.
#[pyclass(name = "CompressedTransientResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyCompressedTransientResult {
    inner: rspice_core::analysis::TransientResultCompressed,
}

impl PyCompressedTransientResult {
    pub fn new(inner: rspice_core::analysis::TransientResultCompressed) -> Self {
        Self { inner }
    }

    fn node_index(&self, node: &NodeIdentifier) -> PyResult<Option<usize>> {
        match node {
            NodeIdentifier::Index(0) => Ok(None),
            NodeIdentifier::Index(index) if *index <= self.inner.num_nodes => Ok(Some(index - 1)),
            NodeIdentifier::Index(index) => {
                Err(invalid_node_index_error(*index, self.inner.num_nodes).into())
            }
            NodeIdentifier::Name(name) if is_ground_name(name) => Ok(None),
            NodeIdentifier::Name(name) => self
                .inner
                .node_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .map(Some)
                .ok_or_else(|| unknown_node_name_error(name).into()),
        }
    }
}

#[pymethods]
impl PyCompressedTransientResult {
    #[getter]
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.time.to_pyarray(py)
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    #[getter]
    fn num_nodes(&self) -> usize {
        self.inner.num_nodes
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.inner.time.len()
    }

    #[getter]
    fn input_points(&self) -> usize {
        self.inner.input_points
    }

    #[getter]
    fn compression_ratio(&self) -> f64 {
        self.inner.compression_ratio
    }

    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = match self.node_index(&node)? {
            Some(index) => self.inner.voltages.get(index).cloned().ok_or_else(|| {
                PyValueError::new_err("malformed compressed transient voltage matrix")
            })?,
            None => vec![0.0; self.inner.time.len()],
        };
        if values.len() != self.inner.time.len() {
            return Err(PyValueError::new_err(
                "malformed compressed transient waveform length",
            ));
        }
        Ok(values.to_pyarray(py))
    }

    fn voltage_at(&self, node: NodeIdentifier, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(PyValueError::new_err("time must be finite"));
        }
        match self.node_index(&node)? {
            Some(index) => self.inner.interpolate(index, time).ok_or_else(|| {
                PyValueError::new_err("compressed transient waveform cannot be interpolated")
            }),
            None => Ok(0.0),
        }
    }

    #[allow(clippy::type_complexity)]
    fn resample<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
        num_points: usize,
    ) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
        if num_points < 2 {
            return Err(PyValueError::new_err("num_points must be at least 2"));
        }
        match self.node_index(&node)? {
            Some(index) => self
                .inner
                .resample(index, num_points)
                .map(|(time, values)| (time.to_pyarray(py), values.to_pyarray(py)))
                .ok_or_else(|| PyValueError::new_err("compressed waveform cannot be resampled")),
            None => {
                let start = self.inner.time.first().copied().unwrap_or(0.0);
                let stop = self.inner.time.last().copied().unwrap_or(start);
                let step = (stop - start) / (num_points - 1) as f64;
                let time = (0..num_points)
                    .map(|index| start + index as f64 * step)
                    .collect::<Vec<_>>();
                Ok((time.to_pyarray(py), vec![0.0; num_points].to_pyarray(py)))
            }
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "CompressedTransientResult(nodes={}, stored_points={}, input_points={}, ratio={:.2}x)",
            self.inner.num_nodes,
            self.inner.time.len(),
            self.inner.input_points,
            self.inner.compression_ratio
        )
    }
}

/// Versioned transient integrator state for fingerprint-validated continuation.
#[pyclass(name = "TransientCheckpoint", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyTransientCheckpoint {
    pub(crate) inner: rspice_core::engine::TransientCheckpoint,
}

impl PyTransientCheckpoint {
    pub fn new(inner: rspice_core::engine::TransientCheckpoint) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyTransientCheckpoint {
    #[staticmethod]
    fn load(path: PathBuf) -> PyResult<Self> {
        rspice_core::engine::TransientCheckpoint::load(&path)
            .map(Self::new)
            .map_err(PyValueError::new_err)
    }

    fn save(&self, path: PathBuf) -> PyResult<()> {
        self.inner.save(&path).map_err(PyValueError::new_err)
    }

    #[getter]
    fn time(&self) -> f64 {
        self.inner.time
    }

    #[getter]
    fn solution<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.solution.to_pyarray(py)
    }

    #[getter]
    fn netlist_fingerprint(&self) -> u64 {
        self.inner.netlist_fingerprint
    }

    fn __repr__(&self) -> String {
        format!(
            "TransientCheckpoint(time={:.6e}, state_size={}, fingerprint={:#018x})",
            self.inner.time,
            self.inner.solution.len(),
            self.inner.netlist_fingerprint
        )
    }
}

/// Transient simulation result with time-domain waveforms
///
/// Contains time points, node voltage waveforms, and branch current
/// waveforms. Arrays are returned as NumPy ndarrays.
///
/// Example:
///     >>> result = engine.run_tran(netlist, 1e-3, 1e-6)
///     >>> import matplotlib.pyplot as plt
///     >>> plt.plot(result.time, result.voltage_waveform("out"))
#[pyclass(name = "TransientResult", module = "rspice")]
pub struct PyTransientResult {
    pub(crate) inner: TransientResult,
}

impl PyTransientResult {
    pub fn new(inner: TransientResult) -> Self {
        Self { inner }
    }

    fn device_op_values(&self, device: &str, parameter: &str) -> PyResult<&[f64]> {
        self.inner
            .device_op_traces
            .iter()
            .find(|trace| {
                trace.device_name.eq_ignore_ascii_case(device)
                    && trace.parameter.eq_ignore_ascii_case(parameter)
            })
            .map(|trace| trace.values.as_slice())
            .ok_or_else(|| {
                PyKeyError::new_err(format!(
                    "device operating-point trace '@{device}[{parameter}]' was not recorded; add it to .SAVE"
                ))
            })
    }

    /// Discard samples before a requested SPICE `TSTART` output boundary.
    ///
    /// The solver must still integrate from zero so dynamic state at TSTART
    /// is correct; only the returned/output data is clipped. All time-aligned
    /// vectors are validated before mutation so malformed core results cannot
    /// become silently misaligned Python arrays.
    pub(crate) fn new_with_start(mut inner: TransientResult, start_time: f64) -> PyResult<Self> {
        if start_time <= 0.0 {
            return Ok(Self { inner });
        }

        let original_len = inner.time.len();
        let start_index = inner.time.partition_point(|time| *time < start_time);
        if start_index >= original_len {
            return Err(crate::errors::SimulationError::new_err(format!(
                "transient result contains no sample at or after requested start_time {start_time}"
            )));
        }

        for (kind, series) in inner
            .voltages
            .iter()
            .map(|series| ("voltage", series))
            .chain(
                inner
                    .branch_currents
                    .iter()
                    .map(|series| ("branch-current", series)),
            )
            .chain(
                inner
                    .device_op_traces
                    .iter()
                    .map(|trace| ("device operating-point", &trace.values)),
            )
        {
            if series.len() != original_len {
                return Err(crate::errors::SimulationError::new_err(format!(
                    "malformed transient result: {kind} series has {} samples but time has {original_len}",
                    series.len()
                )));
            }
        }

        inner.time.drain(..start_index);
        for series in &mut inner.voltages {
            series.drain(..start_index);
        }
        for series in &mut inner.branch_currents {
            series.drain(..start_index);
        }
        for trace in &mut inner.device_op_traces {
            trace.values.drain(..start_index);
        }

        // Event traces store changes rather than one value per accepted analog
        // point. Preserve the state in force at TSTART, then subsequent events.
        for trace in &mut inner.digital_traces {
            let prior = trace
                .points
                .iter()
                .rev()
                .find(|point| point.time < start_time)
                .copied();
            trace.points.retain(|point| point.time >= start_time);
            if trace
                .points
                .first()
                .is_none_or(|point| point.time > start_time)
                && let Some(mut point) = prior
            {
                point.time = start_time;
                trace.points.insert(0, point);
            }
        }
        for trace in &mut inner.real_traces {
            let prior = trace
                .points
                .iter()
                .rev()
                .find(|point| point.time < start_time)
                .copied();
            trace.points.retain(|point| point.time >= start_time);
            if trace
                .points
                .first()
                .is_none_or(|point| point.time > start_time)
                && let Some(mut point) = prior
            {
                point.time = start_time;
                trace.points.insert(0, point);
            }
        }

        Ok(Self { inner })
    }

    fn checked_time_index(&self, time_index: usize) -> AccessResult<()> {
        if time_index < self.inner.time.len() {
            Ok(())
        } else {
            Err(invalid_time_index_error(time_index, self.inner.time.len()))
        }
    }

    fn checked_waveform(&self, node: usize) -> AccessResult<Vec<f64>> {
        if node == 0 {
            return Ok(vec![0.0; self.inner.num_points()]);
        }

        self.inner
            .try_voltage_waveform(node)
            .map(|waveform| waveform.to_vec())
            .ok_or_else(|| invalid_node_index_error(node, self.inner.num_nodes))
    }

    fn checked_waveform_named(&self, name: &str) -> AccessResult<Vec<f64>> {
        if is_ground_name(name) {
            return self.checked_waveform(0);
        }

        let node = self
            .inner
            .node_index_named(name)
            .ok_or_else(|| unknown_node_name_error(name))?;
        self.checked_waveform(node)
    }

    pub(crate) fn waveform_for(&self, node: &NodeIdentifier) -> PyResult<Vec<f64>> {
        match node {
            NodeIdentifier::Index(idx) => self.checked_waveform(*idx),
            NodeIdentifier::Name(name) => self.checked_waveform_named(name),
        }
        .map_err(PyErr::from)
    }
}

#[pymethods]
impl PyTransientResult {
    /// Get the time points array
    ///
    /// Returns:
    ///     numpy.ndarray: Array of time points in seconds
    #[getter]
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.time.to_pyarray(py)
    }

    /// Get the voltage waveform for a node
    ///
    /// Args:
    ///     node: Node index or name
    ///
    /// Returns:
    ///     numpy.ndarray: Voltage values at each time point
    ///
    /// Raises:
    ///     IndexError: If the node index is out of range
    ///     KeyError: If the node name does not exist
    ///
    /// Example:
    ///     >>> v_out = result.voltage_waveform(2)
    ///     >>> v_out = result.voltage_waveform("out")
    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self.waveform_for(&node)?.to_pyarray(py))
    }

    /// Get the current waveform through a branch element
    ///
    /// Branch currents exist for voltage sources and inductors (MNA branch
    /// equations).
    ///
    /// Args:
    ///     name: Element name (e.g. "V1", "L2")
    ///
    /// Returns:
    ///     numpy.ndarray: Current values at each time point
    ///
    /// Raises:
    ///     KeyError: If no branch carries that name
    ///
    /// Example:
    ///     >>> i_supply = result.branch_current_waveform("V1")
    fn branch_current_waveform<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        self.inner
            .try_branch_current_waveform_named(name)
            .map(|waveform| waveform.to_pyarray(py))
            .ok_or_else(|| unknown_branch_name_error(name))
            .map_err(PyErr::from)
    }

    /// Device operating-point traces requested with `.SAVE @device[param]`.
    #[getter]
    fn device_parameter_names(&self) -> Vec<String> {
        self.inner
            .device_op_traces
            .iter()
            .map(|trace| format!("@{}[{}]", trace.device_name, trace.parameter))
            .collect()
    }

    fn device_parameter_waveform<'py>(
        &self,
        py: Python<'py>,
        device: &str,
        parameter: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self.device_op_values(device, parameter)?;
        if values.len() != self.inner.time.len() {
            return Err(PyValueError::new_err(format!(
                "malformed device operating-point trace '@{device}[{parameter}]': {} samples for {} time points",
                values.len(),
                self.inner.time.len()
            )));
        }
        Ok(values.to_pyarray(py))
    }

    /// Get voltage at a specific node and time index
    ///
    /// Args:
    ///     node: Node index
    ///     time_index: Index into the time array
    ///
    /// Returns:
    ///     float: Voltage at the specified node and time
    ///
    /// Raises:
    ///     IndexError: If the node or time index is out of range
    pub fn voltage_at(&self, node: usize, time_index: usize) -> PyResult<f64> {
        self.checked_time_index(time_index).map_err(PyErr::from)?;
        if node == 0 {
            return Ok(0.0);
        }

        self.inner
            .try_voltage_at(node, time_index)
            .ok_or_else(|| invalid_node_index_error(node, self.inner.num_nodes))
            .map_err(PyErr::from)
    }

    /// Fourier-analyze a node waveform
    ///
    /// Decomposes the waveform into harmonics of `fundamental` and computes
    /// total harmonic distortion. Equivalent to the `.FOUR` SPICE analysis.
    ///
    /// Args:
    ///     node: Node index or name
    ///     fundamental: Fundamental frequency in Hz
    ///     num_harmonics: Number of harmonics to compute (default 9)
    ///
    /// Returns:
    ///     FourierResult: DC component, harmonics, and THD
    ///
    /// Raises:
    ///     ValueError: If fundamental is not a positive finite number
    ///     IndexError / KeyError: For invalid nodes
    ///
    /// Example:
    ///     >>> four = tran.fourier("out", fundamental=1e3)
    ///     >>> print(f"THD = {four.thd_percent:.2f}%")
    #[pyo3(signature = (node, fundamental, num_harmonics=9))]
    fn fourier(
        &self,
        node: NodeIdentifier,
        fundamental: f64,
        num_harmonics: usize,
    ) -> PyResult<PyFourierResult> {
        if !fundamental.is_finite() || fundamental <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "fundamental must be a positive finite frequency in Hz, got {fundamental}"
            )));
        }
        if num_harmonics == 0 {
            return Err(PyValueError::new_err("num_harmonics must be at least 1"));
        }
        let waveform = self.waveform_for(&node)?;
        let analysis =
            FourierAnalysis::new(FourierConfig::new(fundamental).with_harmonics(num_harmonics));
        let result = analysis.analyze(&self.inner.time, &waveform);
        Ok(PyFourierResult::from_core(&result))
    }

    /// Get the number of time points
    ///
    /// Returns:
    ///     int: Number of time points in the simulation
    #[getter]
    pub fn num_points(&self) -> usize {
        self.inner.num_points()
    }

    /// Get the number of nodes
    #[getter]
    fn num_nodes(&self) -> usize {
        self.inner.num_nodes
    }

    /// Get node names
    ///
    /// Returns:
    ///     list[str]: List of node names
    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    /// Get branch names aligned with branch current waveforms
    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.inner.branch_names.clone()
    }

    /// Get the simulation stop time
    #[getter]
    pub fn stop_time(&self) -> f64 {
        self.inner.time.last().copied().unwrap_or(0.0)
    }

    fn __repr__(&self) -> String {
        format!(
            "TransientResult(nodes={}, points={}, stop_time={:.3e}s)",
            self.inner.num_nodes,
            self.inner.num_points(),
            self.stop_time()
        )
    }
}

/// AC analysis result with complex frequency-domain data
///
/// Contains frequencies and complex voltage phasors for each node at each
/// frequency. Nodes are addressable by index or name; out-of-range nodes
/// raise rather than returning silent zeros.
///
/// Example:
///     >>> result = engine.run_ac(netlist, [10, 100, 1000])
///     >>> mag_db = result.voltage_db("out")
///     >>> phase_deg = result.voltage_phase_degrees("out")
#[pyclass(name = "AcResult", module = "rspice")]
pub struct PyAcResult {
    frequencies: Vec<f64>,
    pub(crate) results: Vec<AcResult>,
    node_names: Vec<String>,
    branch_names: Vec<String>,
}

impl PyAcResult {
    pub fn new(frequencies: Vec<f64>, results: Vec<AcResult>) -> Self {
        let node_names = results
            .first()
            .map(|r| r.node_names.clone())
            .unwrap_or_default();
        let branch_names = results
            .first()
            .map(|r| r.branch_names.clone())
            .unwrap_or_default();
        Self {
            frequencies,
            results,
            node_names,
            branch_names,
        }
    }

    /// Number of non-ground nodes with phasor data.
    fn node_count(&self) -> usize {
        self.results
            .first()
            .map(|r| r.voltages.len())
            .unwrap_or(self.node_names.len())
    }

    /// Resolve a node identifier to a node index (0 = ground).
    ///
    /// `node_names[i]` corresponds to node index `i + 1` (core's
    /// `node_names_sorted` excludes ground).
    fn resolve_node(&self, node: &NodeIdentifier) -> AccessResult<usize> {
        match node {
            NodeIdentifier::Index(idx) => {
                if *idx <= self.node_count() {
                    Ok(*idx)
                } else {
                    Err(invalid_node_index_error(*idx, self.node_count()))
                }
            }
            NodeIdentifier::Name(name) => {
                if is_ground_name(name) {
                    return Ok(0);
                }
                self.node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .map(|pos| pos + 1)
                    .ok_or_else(|| unknown_node_name_error(name))
            }
        }
    }

    fn resolve_branch(&self, name: &str) -> AccessResult<usize> {
        self.branch_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .ok_or_else(|| unknown_branch_name_error(name))
    }

    fn checked_freq_index(&self, index: usize) -> AccessResult<()> {
        if index < self.results.len() {
            Ok(())
        } else {
            Err(invalid_freq_index_error(index, self.results.len()))
        }
    }

    fn frequency_label(&self, freq_index: usize, result: &AcResult) -> String {
        let frequency = self
            .frequencies
            .get(freq_index)
            .copied()
            .unwrap_or(result.frequency);
        format!("{frequency:.6e} Hz")
    }

    fn node_label(&self, node: usize) -> String {
        if node == 0 {
            return "ground".to_string();
        }
        match self.node_names.get(node - 1) {
            Some(name) => format!("{node} ('{name}')"),
            None => node.to_string(),
        }
    }

    fn branch_label(&self, branch: usize) -> String {
        match self.branch_names.get(branch) {
            Some(name) => format!("{branch} ('{name}')"),
            None => branch.to_string(),
        }
    }

    fn checked_voltage_phasor(
        &self,
        freq_index: usize,
        node: usize,
    ) -> PyResult<rspice_core::Complex64> {
        let result = self
            .results
            .get(freq_index)
            .ok_or_else(|| PyErr::from(invalid_freq_index_error(freq_index, self.results.len())))?;
        self.voltage_phasor_from_row(freq_index, result, node)
            .map_err(PyValueError::new_err)
    }

    fn voltage_phasor_from_row(
        &self,
        freq_index: usize,
        result: &AcResult,
        node: usize,
    ) -> Result<rspice_core::Complex64, String> {
        if node == 0 {
            return Ok(rspice_core::Complex64::new(0.0, 0.0));
        }
        result.voltages.get(node - 1).copied().ok_or_else(|| {
            format!(
                "malformed AC result row {freq_index} ({}): missing voltage for node {}; \
                 row has {} voltage value(s), expected at least {}",
                self.frequency_label(freq_index, result),
                self.node_label(node),
                result.voltages.len(),
                node
            )
        })
    }

    fn checked_branch_current(
        &self,
        freq_index: usize,
        branch: usize,
    ) -> PyResult<rspice_core::Complex64> {
        let result = self
            .results
            .get(freq_index)
            .ok_or_else(|| PyErr::from(invalid_freq_index_error(freq_index, self.results.len())))?;
        self.branch_current_from_row(freq_index, result, branch)
            .map_err(PyValueError::new_err)
    }

    fn branch_current_from_row(
        &self,
        freq_index: usize,
        result: &AcResult,
        branch: usize,
    ) -> Result<rspice_core::Complex64, String> {
        result.currents.get(branch).copied().ok_or_else(|| {
            format!(
                "malformed AC result row {freq_index} ({}): missing current for branch {}; \
                 row has {} current value(s), expected at least {}",
                self.frequency_label(freq_index, result),
                self.branch_label(branch),
                result.currents.len(),
                branch + 1
            )
        })
    }
}

//=============================================================================
// Distortion Results
//=============================================================================

/// Third-order Volterra distortion sweep.
///
/// Every returned `AcResult` contains actual sinusoidal peak phasors at the
/// physical product frequency. They are not internal Volterra kernels or
/// pre-normalized distortion ratios.
#[pyclass(name = "DistortionResult", module = "rspice")]
#[derive(Debug)]
pub struct PyDistortionResult {
    f2_over_f1: Option<f64>,
    f1_frequencies: Vec<f64>,
    fundamental_f1: Vec<AcResult>,
    fundamental_f2: Option<Vec<AcResult>>,
    products: Vec<(DistortionProduct, Vec<AcResult>)>,
    node_names: Vec<String>,
    branch_names: Vec<String>,
}

impl PyDistortionResult {
    pub fn from_core(result: &DistortionAnalysisResult) -> PyResult<Self> {
        if result.points.is_empty() {
            return Err(PyValueError::new_err(
                "malformed distortion result: no F1 points",
            ));
        }
        let product_kinds: &[DistortionProduct] = if result.is_two_tone() {
            &[
                DistortionProduct::Sum,
                DistortionProduct::Difference,
                DistortionProduct::ThirdOrderDifference,
            ]
        } else {
            &[
                DistortionProduct::SecondHarmonic,
                DistortionProduct::ThirdHarmonic,
            ]
        };
        let f1_frequencies = result
            .points
            .iter()
            .map(|point| point.fundamental_f1.frequency)
            .collect();
        let fundamental_f1: Vec<_> = result
            .points
            .iter()
            .map(|point| point.fundamental_f1.clone())
            .collect();
        let fundamental_f2 = if result.is_two_tone() {
            Some(
                result
                    .points
                    .iter()
                    .enumerate()
                    .map(|(index, point)| {
                        point.fundamental_f2.clone().ok_or_else(|| {
                            PyValueError::new_err(format!(
                                "malformed distortion result: missing F2 response at F1 index {index}"
                            ))
                        })
                    })
                    .collect::<PyResult<Vec<_>>>()?,
            )
        } else {
            None
        };
        let products = product_kinds
            .iter()
            .map(|&kind| {
                let rows = result
                    .points
                    .iter()
                    .enumerate()
                    .map(|(index, point)| {
                        point
                            .product(kind)
                            .map(|value| value.response.clone())
                            .ok_or_else(|| {
                                PyValueError::new_err(format!(
                                    "malformed distortion result: missing '{}' response at F1 index {index}",
                                    kind.label()
                                ))
                            })
                    })
                    .collect::<PyResult<Vec<_>>>()?;
                Ok((kind, rows))
            })
            .collect::<PyResult<Vec<_>>>()?;
        let node_names = fundamental_f1
            .first()
            .map(|row| row.node_names.clone())
            .unwrap_or_default();
        let branch_names = fundamental_f1
            .first()
            .map(|row| row.branch_names.clone())
            .unwrap_or_default();
        Ok(Self {
            f2_over_f1: result.f2_over_f1,
            f1_frequencies,
            fundamental_f1,
            fundamental_f2,
            products,
            node_names,
            branch_names,
        })
    }

    fn parse_product(&self, name: &str) -> PyResult<DistortionProduct> {
        let normalized = name.trim().to_ascii_lowercase().replace([' ', '_'], "");
        let product = match normalized.as_str() {
            "2f1" | "hd2" | "secondharmonic" => DistortionProduct::SecondHarmonic,
            "3f1" | "hd3" | "thirdharmonic" => DistortionProduct::ThirdHarmonic,
            "f1+f2" | "sum" | "im2sum" => DistortionProduct::Sum,
            "f1-f2" | "difference" | "im2difference" => DistortionProduct::Difference,
            "2f1-f2" | "im3" | "thirdorderdifference" => DistortionProduct::ThirdOrderDifference,
            _ => {
                return Err(PyValueError::new_err(format!(
                    "unknown distortion product '{name}'; available products: {}",
                    self.available_product_labels().join(", ")
                )));
            }
        };
        if self.products.iter().any(|(kind, _)| *kind == product) {
            Ok(product)
        } else {
            Err(PyValueError::new_err(format!(
                "distortion product '{}' is not available in {} mode; available products: {}",
                product.label(),
                if self.f2_over_f1.is_some() {
                    "two-tone"
                } else {
                    "harmonic"
                },
                self.available_product_labels().join(", ")
            )))
        }
    }

    fn available_product_labels(&self) -> Vec<String> {
        self.products
            .iter()
            .map(|(product, _)| product.label().to_string())
            .collect()
    }

    fn product_rows(&self, product: DistortionProduct) -> PyResult<&[AcResult]> {
        self.products
            .iter()
            .find(|(kind, _)| *kind == product)
            .map(|(_, rows)| rows.as_slice())
            .ok_or_else(|| PyValueError::new_err("distortion result is missing a product series"))
    }

    fn validate_series_length(&self, label: &str, rows: &[AcResult]) -> PyResult<()> {
        if rows.len() == self.fundamental_f1.len() {
            Ok(())
        } else {
            Err(PyValueError::new_err(format!(
                "malformed distortion result: {label} has {} rows for {} F1 points",
                rows.len(),
                self.fundamental_f1.len()
            )))
        }
    }

    fn resolve_node(&self, node: &NodeIdentifier) -> AccessResult<usize> {
        match node {
            NodeIdentifier::Index(index) => {
                let count = self
                    .fundamental_f1
                    .first()
                    .map(|row| row.voltages.len())
                    .unwrap_or(self.node_names.len());
                if *index <= count {
                    Ok(*index)
                } else {
                    Err(invalid_node_index_error(*index, count))
                }
            }
            NodeIdentifier::Name(name) => {
                if is_ground_name(name) {
                    return Ok(0);
                }
                self.node_names
                    .iter()
                    .position(|candidate| candidate.eq_ignore_ascii_case(name))
                    .map(|index| index + 1)
                    .ok_or_else(|| unknown_node_name_error(name))
            }
        }
    }

    fn resolve_branch(&self, name: &str) -> AccessResult<usize> {
        self.branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))
            .ok_or_else(|| unknown_branch_name_error(name))
    }

    fn voltage_ratio_values(
        &self,
        product: DistortionProduct,
        node: &NodeIdentifier,
    ) -> PyResult<Vec<f64>> {
        let node = self.resolve_node(node).map_err(PyErr::from)?;
        let product_rows = self.product_rows(product)?;
        self.validate_series_length(product.label(), product_rows)?;
        product_rows
            .iter()
            .zip(self.fundamental_f1.iter())
            .enumerate()
            .map(|(index, (numerator, denominator))| {
                if node == 0 {
                    return Ok(0.0);
                }
                let numerator = numerator.voltages.get(node - 1).ok_or_else(|| {
                    PyValueError::new_err(format!(
                        "malformed distortion product '{}' at F1 index {index}: missing node {node}",
                        product.label()
                    ))
                })?;
                let denominator = denominator.voltages.get(node - 1).ok_or_else(|| {
                    PyValueError::new_err(format!(
                        "malformed F1 result at index {index}: missing node {node}"
                    ))
                })?;
                Ok(magnitude_ratio(numerator.norm(), denominator.norm()))
            })
            .collect()
    }

    fn branch_ratio_values(&self, product: DistortionProduct, branch: &str) -> PyResult<Vec<f64>> {
        let branch = self.resolve_branch(branch).map_err(PyErr::from)?;
        let product_rows = self.product_rows(product)?;
        self.validate_series_length(product.label(), product_rows)?;
        product_rows
            .iter()
            .zip(self.fundamental_f1.iter())
            .enumerate()
            .map(|(index, (numerator, denominator))| {
                let numerator = numerator.currents.get(branch).ok_or_else(|| {
                    PyValueError::new_err(format!(
                        "malformed distortion product '{}' at F1 index {index}: missing branch current {branch}",
                        product.label()
                    ))
                })?;
                let denominator = denominator.currents.get(branch).ok_or_else(|| {
                    PyValueError::new_err(format!(
                        "malformed F1 result at index {index}: missing branch current {branch}"
                    ))
                })?;
                Ok(magnitude_ratio(numerator.norm(), denominator.norm()))
            })
            .collect()
    }
}

fn magnitude_ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        if numerator == 0.0 { 0.0 } else { f64::INFINITY }
    } else {
        numerator / denominator
    }
}

#[pymethods]
impl PyDistortionResult {
    /// Swept F1 frequencies in Hz.
    #[getter]
    fn f1_frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.f1_frequencies.to_pyarray(py)
    }

    /// Fixed F2 frequency in Hz in two-tone mode, otherwise None.
    #[getter]
    fn f2_frequency(&self) -> Option<f64> {
        self.fundamental_f2
            .as_ref()
            .and_then(|rows| rows.first())
            .map(|row| row.frequency)
    }

    #[getter]
    fn f2_over_f1(&self) -> Option<f64> {
        self.f2_over_f1
    }

    #[getter]
    fn is_two_tone(&self) -> bool {
        self.f2_over_f1.is_some()
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.f1_frequencies.len()
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.node_names.clone()
    }

    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.branch_names.clone()
    }

    /// Canonical product names accepted by `product()` for this mode.
    #[getter]
    fn available_products(&self) -> Vec<String> {
        self.available_product_labels()
    }

    /// Actual first-order F1 response, aligned with `f1_frequencies`.
    #[getter]
    fn fundamental_f1(&self) -> PyAcResult {
        PyAcResult::new(self.f1_frequencies.clone(), self.fundamental_f1.clone())
    }

    /// Actual first-order F2 response at each F1 sweep point.
    ///
    /// F2 is fixed by SPICE's two-tone contract, so its frequency array
    /// repeats the same value. Returns None in harmonic mode.
    #[getter]
    fn fundamental_f2(&self) -> Option<PyAcResult> {
        self.fundamental_f2.as_ref().map(|rows| {
            PyAcResult::new(rows.iter().map(|row| row.frequency).collect(), rows.clone())
        })
    }

    /// Actual complex response for a spectral product.
    fn product(&self, name: &str) -> PyResult<PyAcResult> {
        let product = self.parse_product(name)?;
        let rows = self.product_rows(product)?;
        self.validate_series_length(product.label(), rows)?;
        Ok(PyAcResult::new(
            rows.iter().map(|row| row.frequency).collect(),
            rows.to_vec(),
        ))
    }

    /// |V(product)| / |V(F1)| across the F1 sweep.
    fn voltage_ratio<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let product = self.parse_product(name)?;
        Ok(self.voltage_ratio_values(product, &node)?.to_pyarray(py))
    }

    /// Product voltage relative to F1 in dBc (20*log10 of the ratio).
    fn voltage_db_relative<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let product = self.parse_product(name)?;
        let values = self
            .voltage_ratio_values(product, &node)?
            .into_iter()
            .map(|ratio| 20.0 * ratio.log10())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    /// |I(product)| / |I(F1)| for a named MNA branch across the sweep.
    fn branch_current_ratio<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        branch: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let product = self.parse_product(name)?;
        Ok(self.branch_ratio_values(product, branch)?.to_pyarray(py))
    }

    /// Product branch current relative to F1 in dBc.
    fn branch_current_db_relative<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        branch: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let product = self.parse_product(name)?;
        let values = self
            .branch_ratio_values(product, branch)?
            .into_iter()
            .map(|ratio| 20.0 * ratio.log10())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn __repr__(&self) -> String {
        format!(
            "DistortionResult(mode={}, points={}, products=[{}])",
            if self.is_two_tone() {
                "two-tone"
            } else {
                "harmonic"
            },
            self.num_points(),
            self.available_product_labels().join(", ")
        )
    }
}

#[pymethods]
impl PyAcResult {
    /// Get the frequency array
    ///
    /// Returns:
    ///     numpy.ndarray: Array of frequencies in Hz
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    /// Get the number of frequency points
    #[getter]
    pub fn num_frequencies(&self) -> usize {
        self.frequencies.len()
    }

    /// Node names addressable in this result (excluding ground)
    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.node_names.clone()
    }

    /// Branch names with complex current phasors
    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.branch_names.clone()
    }

    /// Get voltage magnitude at a node across all frequencies
    ///
    /// Args:
    ///     node: Node index or name
    ///
    /// Returns:
    ///     numpy.ndarray: Magnitude values at each frequency
    ///
    /// Raises:
    ///     IndexError / KeyError: For invalid nodes
    fn voltage_magnitude<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let magnitudes: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_voltage_phasor(freq_index, idx)
                    .map(|v| v.norm())
            })
            .collect();
        Ok(magnitudes?.to_pyarray(py))
    }

    /// Get voltage phase at a node across all frequencies (radians)
    fn voltage_phase<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let phases: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_voltage_phasor(freq_index, idx)
                    .map(|v| v.arg())
            })
            .collect();
        Ok(phases?.to_pyarray(py))
    }

    /// Get voltage phase at a node across all frequencies (degrees)
    fn voltage_phase_degrees<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let phases: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_voltage_phasor(freq_index, idx)
                    .map(|v| v.arg().to_degrees())
            })
            .collect();
        Ok(phases?.to_pyarray(py))
    }

    /// Get complex voltage at a node across all frequencies
    ///
    /// Returns:
    ///     numpy.ndarray: complex128 phasor values at each frequency
    ///
    /// Example:
    ///     >>> h = ac.voltage_complex("out") / ac.voltage_complex("in")
    fn voltage_complex<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let values: PyResult<Vec<rspice_core::Complex64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| self.checked_voltage_phasor(freq_index, idx))
            .collect();
        Ok(values?.to_pyarray(py))
    }

    /// Get voltage magnitude in dB (20·log10 |V|) at a node across all frequencies
    fn voltage_db<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let db: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_voltage_phasor(freq_index, idx)
                    .map(|v| 20.0 * v.norm().log10())
            })
            .collect();
        Ok(db?.to_pyarray(py))
    }

    /// Get complex branch current through an element across all frequencies
    ///
    /// Branch currents exist for voltage sources and inductors.
    ///
    /// Args:
    ///     name: Element name (e.g. "V1", "L2")
    ///
    /// Raises:
    ///     KeyError: If no branch carries that name
    fn branch_current_complex<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        let idx = self.resolve_branch(name).map_err(PyErr::from)?;
        let values: PyResult<Vec<rspice_core::Complex64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| self.checked_branch_current(freq_index, idx))
            .collect();
        Ok(values?.to_pyarray(py))
    }

    /// Get branch current magnitude through an element across all frequencies
    fn branch_current_magnitude<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_branch(name).map_err(PyErr::from)?;
        let values: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_branch_current(freq_index, idx)
                    .map(|c| c.norm())
            })
            .collect();
        Ok(values?.to_pyarray(py))
    }

    /// Get voltage magnitude at a specific frequency index and node
    ///
    /// Raises:
    ///     IndexError: If the frequency index is out of range
    pub fn magnitude_at(&self, freq_index: usize, node: NodeIdentifier) -> PyResult<f64> {
        self.checked_freq_index(freq_index).map_err(PyErr::from)?;
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        Ok(self.checked_voltage_phasor(freq_index, idx)?.norm())
    }

    /// Get phase at a specific frequency index and node (radians)
    ///
    /// Raises:
    ///     IndexError: If the frequency index is out of range
    fn phase_at(&self, freq_index: usize, node: NodeIdentifier) -> PyResult<f64> {
        self.checked_freq_index(freq_index).map_err(PyErr::from)?;
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        Ok(self.checked_voltage_phasor(freq_index, idx)?.arg())
    }

    fn __repr__(&self) -> String {
        let freq_range = match (self.frequencies.first(), self.frequencies.last()) {
            (Some(first), Some(last)) => format!("{first:.1e}-{last:.1e} Hz"),
            _ => "no frequencies".to_string(),
        };
        format!(
            "AcResult(frequencies={}, range={}, nodes={})",
            self.frequencies.len(),
            freq_range,
            self.node_count()
        )
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
    /// Raises:
    ///     IndexError: If the sweep index is out of range
    pub fn result_at(&self, index: usize) -> PyResult<PySimulationResult> {
        self.point(index)
            .map(|(_, r)| PySimulationResult::new(r.clone()))
            .map_err(PyErr::from)
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
}

//=============================================================================
// Noise Analysis Results
//=============================================================================

/// Noise contribution from a single device
///
/// Provides information about one noise source's contribution to total output noise.
///
/// Example:
///     >>> for contrib in result.contributions:
///     ...     print(f"{contrib.device_name}: {contrib.percentage:.1f}%")
#[pyclass(name = "NoiseContribution", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyNoiseContribution {
    /// Device name generating this noise
    #[pyo3(get)]
    pub device_name: String,
    /// Noise type as string (Thermal, Shot, Flicker, Burst)
    #[pyo3(get)]
    pub noise_type: String,
    /// Output contribution in V²/Hz
    #[pyo3(get)]
    pub output_contribution: f64,
    /// Percentage of total noise
    #[pyo3(get)]
    pub percentage: f64,
}

#[pymethods]
impl PyNoiseContribution {
    fn __repr__(&self) -> String {
        format!(
            "NoiseContribution({}: {:.1}%, type={})",
            self.device_name, self.percentage, self.noise_type
        )
    }
}

/// Noise analysis result at a single frequency
///
/// Contains output noise spectral density and contribution breakdown.
///
/// Example:
///     >>> result = engine.run_noise(netlist, output_node="out", frequencies=[1e3, 10e3])
///     >>> for r in result:
///     ...     print(f"f={r.frequency:.0f}Hz: {r.output_noise_rms*1e9:.2f}nV/√Hz")
#[pyclass(name = "NoiseResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyNoiseResult {
    /// Frequency in Hz
    #[pyo3(get)]
    pub frequency: f64,
    /// Total output voltage noise spectral density (V²/Hz)
    #[pyo3(get)]
    pub output_noise_density: f64,
    /// Input-referred noise spectral density (V²/Hz)
    #[pyo3(get)]
    pub input_referred_density: f64,
    /// Individual noise contributions
    contributions: Vec<PyNoiseContribution>,
}

impl PyNoiseResult {
    /// Create from core NoiseResult
    pub fn from_core(result: &rspice_core::analysis::NoiseResult) -> Self {
        let contributions = result
            .contributions
            .iter()
            .map(|c| PyNoiseContribution {
                device_name: c.device_name.clone(),
                noise_type: format!("{:?}", c.noise_type),
                output_contribution: c.output_contribution,
                percentage: c.percentage,
            })
            .collect();

        Self {
            frequency: result.frequency,
            output_noise_density: result.output_noise_density,
            input_referred_density: result.input_referred_density,
            contributions,
        }
    }
}

#[pymethods]
impl PyNoiseResult {
    /// Get output noise in V/√Hz (RMS voltage noise density)
    #[getter]
    fn output_noise_rms(&self) -> f64 {
        self.output_noise_density.sqrt()
    }

    /// Get input-referred noise in V/√Hz
    #[getter]
    fn input_referred_rms(&self) -> f64 {
        self.input_referred_density.sqrt()
    }

    /// Get output noise in dBV/sqrt(Hz).
    ///
    /// This is `20*log10(sqrt(Svo) / 1 V/sqrt(Hz))`, equivalently
    /// `10*log10(Svo / 1 V^2/Hz)`. Zero density is represented by negative
    /// infinity; invalid negative density propagates as NaN.
    #[getter]
    fn output_noise_dbv(&self) -> f64 {
        10.0 * self.output_noise_density.log10()
    }

    /// Get all noise contributions
    #[getter]
    fn contributions(&self) -> Vec<PyNoiseContribution> {
        self.contributions.clone()
    }

    /// Get the dominant noise source
    fn dominant_source(&self) -> Option<PyNoiseContribution> {
        self.contributions
            .iter()
            .max_by(|a, b| {
                a.output_contribution
                    .partial_cmp(&b.output_contribution)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    fn __repr__(&self) -> String {
        format!(
            "NoiseResult(f={:.2e}Hz, Svo={:.2e}V²/Hz, Svo_rms={:.2e}V/√Hz)",
            self.frequency,
            self.output_noise_density,
            self.output_noise_rms()
        )
    }
}

//=============================================================================
// Sensitivity Analysis Results
//=============================================================================

/// Sensitivity of one output to one device/source parameter.
#[pyclass(name = "ElementSensitivity", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyElementSensitivity {
    #[pyo3(get)]
    pub element: String,
    #[pyo3(get)]
    pub element_type: String,
    #[pyo3(get)]
    pub parameter: String,
    #[pyo3(get)]
    pub nominal_value: f64,
    #[pyo3(get)]
    pub absolute: f64,
    #[pyo3(get)]
    pub normalized: f64,
}

impl PyElementSensitivity {
    fn from_core(value: &rspice_core::analysis::Sensitivity) -> Self {
        Self {
            element: value.element.clone(),
            element_type: format!("{:?}", value.element_type),
            parameter: value.parameter.clone(),
            nominal_value: value.nominal_value,
            absolute: value.absolute,
            normalized: value.normalized,
        }
    }
}

#[pymethods]
impl PyElementSensitivity {
    #[getter]
    fn percent_per_percent(&self) -> f64 {
        self.normalized * 100.0
    }

    fn __repr__(&self) -> String {
        format!(
            "ElementSensitivity(element='{}', parameter='{}', absolute={:.6e}, normalized={:.6e})",
            self.element, self.parameter, self.absolute, self.normalized
        )
    }
}

/// Complete adjoint DC sensitivity result for an output voltage.
#[pyclass(name = "SensitivityResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PySensitivityResult {
    #[pyo3(get)]
    pub output: String,
    #[pyo3(get)]
    pub output_value: f64,
    sensitivities: Vec<PyElementSensitivity>,
}

impl PySensitivityResult {
    pub fn from_core(result: &rspice_core::analysis::SensitivityResult) -> Self {
        Self {
            output: result.output.clone(),
            output_value: result.output_value,
            sensitivities: result
                .sensitivities
                .iter()
                .map(PyElementSensitivity::from_core)
                .collect(),
        }
    }
}

#[pymethods]
impl PySensitivityResult {
    #[getter]
    fn sensitivities(&self) -> Vec<PyElementSensitivity> {
        self.sensitivities.clone()
    }

    fn __len__(&self) -> usize {
        self.sensitivities.len()
    }

    /// Look up a sensitivity by element and optional parameter name.
    #[pyo3(signature = (element, parameter=None))]
    fn get(&self, element: &str, parameter: Option<&str>) -> PyResult<PyElementSensitivity> {
        self.sensitivities
            .iter()
            .find(|value| {
                value.element.eq_ignore_ascii_case(element)
                    && parameter
                        .is_none_or(|parameter| value.parameter.eq_ignore_ascii_case(parameter))
            })
            .cloned()
            .ok_or_else(|| {
                let suffix = parameter.map_or(String::new(), |name| format!("/{name}"));
                PyKeyError::new_err(format!("unknown sensitivity '{element}{suffix}'"))
            })
    }

    /// Most influential entries by absolute normalized sensitivity.
    #[pyo3(signature = (count=10))]
    fn top(&self, count: usize) -> Vec<PyElementSensitivity> {
        let mut values = self.sensitivities.clone();
        values.sort_by(|a, b| {
            b.normalized
                .abs()
                .total_cmp(&a.normalized.abs())
                .then_with(|| a.element.cmp(&b.element))
        });
        values.truncate(count);
        values
    }

    fn __repr__(&self) -> String {
        format!(
            "SensitivityResult(output='{}', output_value={:.6e}, entries={})",
            self.output,
            self.output_value,
            self.sensitivities.len()
        )
    }
}

//=============================================================================
// Monte Carlo Analysis Results
//=============================================================================

/// Statistics for a single output variable from Monte Carlo analysis
///
/// Example:
///     >>> stats = result.get_variable("V(out)")
///     >>> print(f"Mean: {stats.mean:.3f}, Std: {stats.std_dev:.3f}")
///     >>> print(f"3σ range: {stats.three_sigma_range}")
#[pyclass(name = "VariableStatistics", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyVariableStatistics {
    /// Variable name
    #[pyo3(get)]
    pub name: String,
    /// Computed mean
    #[pyo3(get)]
    pub mean: f64,
    /// Computed standard deviation
    #[pyo3(get)]
    pub std_dev: f64,
    /// Minimum value
    #[pyo3(get)]
    pub min: f64,
    /// Maximum value
    #[pyo3(get)]
    pub max: f64,
    /// All sampled values
    samples: Vec<f64>,
    /// Histogram bin counts
    histogram: Vec<usize>,
    /// Histogram bin edges
    bin_edges: Vec<f64>,
}

impl PyVariableStatistics {
    pub fn from_core(stats: &rspice_core::analysis::VariableStatistics) -> Self {
        Self {
            name: stats.name.clone(),
            mean: stats.mean,
            std_dev: stats.std_dev,
            min: stats.min,
            max: stats.max,
            samples: stats.samples.clone(),
            histogram: stats.histogram.clone(),
            bin_edges: stats.bin_edges.clone(),
        }
    }
}

#[pymethods]
impl PyVariableStatistics {
    /// Get all sampled values as NumPy array
    #[getter]
    fn samples<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.samples.to_pyarray(py)
    }

    /// Get histogram bin counts
    #[getter]
    fn histogram(&self) -> Vec<usize> {
        self.histogram.clone()
    }

    /// Get histogram bin edges
    #[getter]
    fn bin_edges<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.bin_edges.to_pyarray(py)
    }

    /// Get a specific percentile value (0-100, linear interpolation).
    ///
    /// Raises ValueError for a non-finite/out-of-range percentile or when no
    /// successful samples are available.
    fn percentile(&self, pct: f64) -> PyResult<f64> {
        if !pct.is_finite() || !(0.0..=100.0).contains(&pct) {
            return Err(PyValueError::new_err(format!(
                "percentile must be a finite number from 0 to 100, got {pct}"
            )));
        }
        if self.samples.is_empty() {
            return Err(PyValueError::new_err(
                "cannot compute a percentile without successful samples",
            ));
        }
        let mut sorted = self.samples.clone();
        if sorted.iter().any(|sample| !sample.is_finite()) {
            return Err(PyValueError::new_err(
                "cannot compute a percentile from non-finite samples",
            ));
        }
        sorted.sort_by(f64::total_cmp);
        let rank = (pct / 100.0) * (sorted.len() - 1) as f64;
        let lo = rank.floor() as usize;
        let hi = rank.ceil() as usize;
        if lo == hi {
            Ok(sorted[lo])
        } else {
            let frac = rank - lo as f64;
            Ok(sorted[lo] * (1.0 - frac) + sorted[hi] * frac)
        }
    }

    /// Get 3-sigma range (mean ± 3*std_dev) as tuple
    #[getter]
    fn three_sigma_range(&self) -> (f64, f64) {
        (
            self.mean - 3.0 * self.std_dev,
            self.mean + 3.0 * self.std_dev,
        )
    }

    /// Get coefficient of variation (std_dev / |mean|) as percentage.
    ///
    /// Returns None when the mean is exactly zero because the coefficient is
    /// undefined rather than zero.
    #[getter]
    fn cv_percent(&self) -> Option<f64> {
        if self.mean != 0.0 {
            Some((self.std_dev / self.mean.abs()) * 100.0)
        } else {
            None
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "VariableStatistics({}: mean={:.4e}, std={:.4e}, range=[{:.4e}, {:.4e}])",
            self.name, self.mean, self.std_dev, self.min, self.max
        )
    }
}

/// Monte Carlo analysis results
///
/// Contains statistical results for all output variables from a Monte Carlo run.
///
/// Example:
///     >>> result = engine.run_monte_carlo(netlist, num_runs=1000, seed=42)
///     >>> v_out = result.get_variable("V(OUT)")
///     >>> print(f"V(out): {v_out.mean:.3f} ± {v_out.std_dev:.3f}V")
#[pyclass(name = "MonteCarloResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyMonteCarloResult {
    /// Number of runs completed
    #[pyo3(get)]
    pub num_runs: usize,
    /// Whether all runs converged
    #[pyo3(get)]
    pub all_converged: bool,
    /// Number of failed runs
    #[pyo3(get)]
    pub num_failures: usize,
    /// Statistics for all variables (internal storage)
    variables: std::collections::HashMap<String, PyVariableStatistics>,
}

impl PyMonteCarloResult {
    pub fn from_core(result: &rspice_core::analysis::MonteCarloResult) -> Self {
        let variables = result
            .variables
            .iter()
            .map(|(name, stats)| (name.clone(), PyVariableStatistics::from_core(stats)))
            .collect();

        Self {
            num_runs: result.num_runs,
            all_converged: result.all_converged,
            num_failures: result.num_failures,
            variables,
        }
    }
}

#[pymethods]
impl PyMonteCarloResult {
    /// Get statistics for a specific variable by name (case-insensitive)
    fn get_variable(&self, name: &str) -> Option<PyVariableStatistics> {
        self.variables.get(name).cloned().or_else(|| {
            self.variables
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(name))
                .map(|(_, v)| v.clone())
        })
    }

    /// Get all variable names
    #[getter]
    fn variable_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.variables.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get mean for a variable
    fn mean(&self, name: &str) -> Option<f64> {
        self.get_variable(name).map(|v| v.mean)
    }

    /// Get standard deviation for a variable
    fn std_dev(&self, name: &str) -> Option<f64> {
        self.get_variable(name).map(|v| v.std_dev)
    }

    /// Get min/max range as tuple
    fn range(&self, name: &str) -> Option<(f64, f64)> {
        self.get_variable(name).map(|v| (v.min, v.max))
    }

    /// Get success rate as percentage
    #[getter]
    fn success_rate(&self) -> f64 {
        if self.num_runs > 0 {
            ((self.num_runs - self.num_failures) as f64 / self.num_runs as f64) * 100.0
        } else {
            0.0
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "MonteCarloResult(runs={}, failures={}, variables={})",
            self.num_runs,
            self.num_failures,
            self.variables.len()
        )
    }
}

//=============================================================================
// Pole-Zero Analysis Results
//=============================================================================

/// Complex number for poles and zeros
///
/// Represents a pole or zero in the s-domain (Laplace domain). Convertible
/// to a built-in complex with `complex(value)`.
///
/// Example:
///     >>> for pole in result.poles:
///     ...     print(complex(pole))
///     ...     if pole.is_real:
///     ...         print(f"  Time constant: {pole.time_constant:.3e}s")
#[pyclass(name = "ComplexValue", module = "rspice", from_py_object)]
#[derive(Debug, Clone, Copy)]
pub struct PyComplexValue {
    /// Real part
    #[pyo3(get)]
    pub real: f64,
    /// Imaginary part
    #[pyo3(get)]
    pub imag: f64,
}

impl PyComplexValue {
    pub fn from_core(c: &rspice_core::analysis::pole_zero::Complex) -> Self {
        Self {
            real: c.re,
            imag: c.im,
        }
    }
}

#[pymethods]
impl PyComplexValue {
    /// Convert to a built-in Python complex number
    fn __complex__<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyComplex> {
        pyo3::types::PyComplex::from_doubles(py, self.real, self.imag)
    }

    /// Get magnitude |z|
    #[getter]
    fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Get phase in radians
    #[getter]
    fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    /// Get phase in degrees
    #[getter]
    fn phase_degrees(&self) -> f64 {
        self.phase().to_degrees()
    }

    /// Check if this is a real value (imaginary part near zero)
    #[getter]
    fn is_real(&self) -> bool {
        let scale = self.real.abs().max(self.imag.abs()).max(1.0);
        self.imag.abs() <= 64.0 * f64::EPSILON * scale
    }

    /// Get frequency in Hz (|Im| / 2π)
    #[getter]
    fn frequency_hz(&self) -> f64 {
        self.imag.abs() / (2.0 * std::f64::consts::PI)
    }

    /// Get damping factor ζ = -Re(p)/|p| for complex pole
    #[getter]
    fn damping_factor(&self) -> f64 {
        let mag = self.magnitude();
        if mag > 1e-15 { -self.real / mag } else { 0.0 }
    }

    /// Get decay time constant τ = -1/Re(p) for a stable real pole.
    #[getter]
    fn time_constant(&self) -> Option<f64> {
        if self.is_real() && self.real < 0.0 {
            Some(-1.0 / self.real)
        } else {
            None
        }
    }

    fn __repr__(&self) -> String {
        if self.imag >= 0.0 {
            format!("{:.6e}+{:.6e}j", self.real, self.imag)
        } else {
            format!("{:.6e}{:.6e}j", self.real, self.imag)
        }
    }
}

/// Pole-zero analysis result
///
/// Contains poles and zeros of a circuit's transfer function.
///
/// Note: `run_pz` injects a unit *current* at the input node, so `dc_gain`
/// is a transimpedance (V/A), not a voltage ratio. Pole/zero locations are
/// input-independent.
///
/// Example:
///     >>> result = engine.run_pz(netlist, input_node="in", output_node="out")
///     >>> print(f"Stable: {result.is_stable}")
///     >>> for pole in result.poles:
///     ...     print(f"Pole: {pole}")
#[pyclass(name = "PoleZeroResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPoleZeroResult {
    /// System poles (natural frequencies)
    poles: Vec<PyComplexValue>,
    /// System zeros
    zeros: Vec<PyComplexValue>,
    /// DC transimpedance H(0) in V/A (unit current input)
    #[pyo3(get)]
    pub dc_gain: f64,
    /// High-frequency gain H(∞) if finite
    #[pyo3(get)]
    pub hf_gain: Option<f64>,
    /// Input specification
    #[pyo3(get)]
    pub input: String,
    /// Output specification
    #[pyo3(get)]
    pub output: String,
}

impl PyPoleZeroResult {
    pub fn from_core(result: &rspice_core::analysis::PoleZeroResult) -> Self {
        Self {
            poles: result.poles.iter().map(PyComplexValue::from_core).collect(),
            zeros: result.zeros.iter().map(PyComplexValue::from_core).collect(),
            dc_gain: result.dc_gain,
            hf_gain: result.hf_gain,
            input: result.input.clone(),
            output: result.output.clone(),
        }
    }
}

#[pymethods]
impl PyPoleZeroResult {
    /// Get all poles
    #[getter]
    fn poles(&self) -> Vec<PyComplexValue> {
        self.poles.clone()
    }

    /// Get all zeros
    #[getter]
    fn zeros(&self) -> Vec<PyComplexValue> {
        self.zeros.clone()
    }

    /// Get all poles as a complex128 NumPy array
    #[getter]
    fn poles_array<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        let values: Vec<rspice_core::Complex64> = self
            .poles
            .iter()
            .map(|p| rspice_core::Complex64::new(p.real, p.imag))
            .collect();
        values.to_pyarray(py)
    }

    /// Get all zeros as a complex128 NumPy array
    #[getter]
    fn zeros_array<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        let values: Vec<rspice_core::Complex64> = self
            .zeros
            .iter()
            .map(|z| rspice_core::Complex64::new(z.real, z.imag))
            .collect();
        values.to_pyarray(py)
    }

    /// Get real poles only (as list)
    fn real_poles(&self) -> Vec<PyComplexValue> {
        self.poles.iter().filter(|p| p.is_real()).copied().collect()
    }

    /// Get complex poles only (as list)
    fn complex_poles(&self) -> Vec<PyComplexValue> {
        self.poles
            .iter()
            .filter(|p| !p.is_real())
            .copied()
            .collect()
    }

    /// Check asymptotic stability: every pole is finite and strictly in the
    /// open left half-plane. Marginal poles are not reported as stable.
    #[getter]
    fn is_stable(&self) -> bool {
        !self.poles.is_empty()
            && self
                .poles
                .iter()
                .all(|pole| pole.real.is_finite() && pole.imag.is_finite() && pole.real < 0.0)
    }

    /// Get the dominant pole (closest to imaginary axis with Re < 0)
    fn dominant_pole(&self) -> Option<PyComplexValue> {
        self.poles
            .iter()
            .filter(|p| p.real < 0.0 && p.real.is_finite())
            .min_by(|a, b| a.real.abs().total_cmp(&b.real.abs()))
            .copied()
    }

    /// Decay frequency `|Re(p_dominant)| / 2π` in Hz.
    ///
    /// This is a pole metric, not a general 3 dB bandwidth.
    #[getter]
    fn dominant_pole_decay_hz(&self) -> Option<f64> {
        self.dominant_pole()
            .map(|p| p.real.abs() / (2.0 * std::f64::consts::PI))
    }

    /// Exact 3 dB bandwidth for the special one-real-pole/no-zero case.
    ///
    /// Returns None for higher-order or zero-containing transfer functions;
    /// use an AC sweep to compute their actual bandwidth.
    #[getter]
    fn bandwidth_hz(&self) -> Option<f64> {
        if self.poles.len() == 1 && self.zeros.is_empty() && self.poles[0].is_real() {
            self.dominant_pole_decay_hz()
        } else {
            None
        }
    }

    /// Get number of poles
    #[getter]
    fn num_poles(&self) -> usize {
        self.poles.len()
    }

    /// Get number of zeros
    #[getter]
    fn num_zeros(&self) -> usize {
        self.zeros.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "PoleZeroResult(poles={}, zeros={}, dc_gain={:.3e}, stable={})",
            self.poles.len(),
            self.zeros.len(),
            self.dc_gain,
            self.is_stable()
        )
    }
}

//=============================================================================
// Fourier Analysis Results
//=============================================================================

/// A single harmonic component from Fourier analysis
#[pyclass(name = "Harmonic", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyHarmonic {
    /// Harmonic number (1 = fundamental)
    #[pyo3(get)]
    pub n: usize,
    /// Frequency in Hz
    #[pyo3(get)]
    pub frequency: f64,
    /// Magnitude
    #[pyo3(get)]
    pub magnitude: f64,
    /// Phase in radians
    #[pyo3(get)]
    pub phase: f64,
}

#[pymethods]
impl PyHarmonic {
    /// Phase in degrees
    #[getter]
    fn phase_degrees(&self) -> f64 {
        self.phase.to_degrees()
    }

    fn __repr__(&self) -> String {
        format!(
            "Harmonic(n={}, f={:.4e}Hz, mag={:.6e}, phase={:.2}°)",
            self.n,
            self.frequency,
            self.magnitude,
            self.phase.to_degrees()
        )
    }
}

/// Fourier analysis result (harmonic decomposition + THD)
///
/// Example:
///     >>> four = tran.fourier("out", fundamental=1e3)
///     >>> print(f"THD = {four.thd_percent:.3f}%")
///     >>> for h in four.harmonics:
///     ...     print(h)
#[pyclass(name = "FourierResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyFourierResult {
    /// DC component of the waveform
    #[pyo3(get)]
    pub dc_component: f64,
    /// Total harmonic distortion as a ratio (0-1)
    #[pyo3(get)]
    pub thd: f64,
    harmonics: Vec<PyHarmonic>,
}

impl PyFourierResult {
    pub fn from_core(result: &rspice_core::analysis::FourierResult) -> Self {
        // Core's harmonic 0 is the DC term; expose it via `dc_component`
        // and keep the list at n >= 1 so harmonics[0] is the fundamental.
        // Core reports phase in degrees; this API uses radians plus
        // *_degrees helpers everywhere.
        let harmonics = result
            .harmonics
            .iter()
            .filter(|h| h.harmonic_number >= 1)
            .map(|h| PyHarmonic {
                n: h.harmonic_number,
                frequency: h.frequency,
                magnitude: h.magnitude,
                phase: h.phase.to_radians(),
            })
            .collect();
        Self {
            dc_component: result.dc_component,
            // Core reports THD in percent already.
            thd: result.thd / 100.0,
            harmonics,
        }
    }
}

#[pymethods]
impl PyFourierResult {
    /// Total harmonic distortion in percent
    #[getter]
    fn thd_percent(&self) -> f64 {
        self.thd * 100.0
    }

    /// All harmonic components (index 0 = fundamental)
    #[getter]
    fn harmonics(&self) -> Vec<PyHarmonic> {
        self.harmonics.clone()
    }

    /// Harmonic magnitudes as a NumPy array (index 0 = fundamental)
    #[getter]
    fn magnitudes<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        let mags: Vec<f64> = self.harmonics.iter().map(|h| h.magnitude).collect();
        mags.to_pyarray(py)
    }

    /// Magnitude of the fundamental
    #[getter]
    fn fundamental_magnitude(&self) -> Option<f64> {
        self.harmonics.first().map(|h| h.magnitude)
    }

    fn __repr__(&self) -> String {
        format!(
            "FourierResult(harmonics={}, dc={:.4e}, thd={:.4}%)",
            self.harmonics.len(),
            self.dc_component,
            self.thd * 100.0
        )
    }
}

//=============================================================================
// S-Parameter Results
//=============================================================================

/// N-port scattering-parameter sweep with per-port reference impedances.
#[pyclass(name = "SParameterResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PySParameterResult {
    frequencies: Vec<f64>,
    port_names: Vec<String>,
    reference_impedances: Vec<f64>,
    parameters: Vec<Vec<Vec<rspice_core::Complex64>>>,
}

impl PySParameterResult {
    pub fn new(
        frequencies: Vec<f64>,
        port_names: Vec<String>,
        reference_impedances: Vec<f64>,
        parameters: Vec<Vec<Vec<rspice_core::Complex64>>>,
    ) -> Self {
        Self {
            frequencies,
            port_names,
            reference_impedances,
            parameters,
        }
    }

    fn parameter(
        &self,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<&[rspice_core::Complex64]> {
        let num_ports = self.port_names.len();
        if !(1..=num_ports).contains(&output_port) {
            return Err(PyIndexError::new_err(format!(
                "output_port must be in 1..={num_ports}, got {output_port}"
            )));
        }
        if !(1..=num_ports).contains(&input_port) {
            return Err(PyIndexError::new_err(format!(
                "input_port must be in 1..={num_ports}, got {input_port}"
            )));
        }
        self.parameters
            .get(output_port - 1)
            .and_then(|row| row.get(input_port - 1))
            .map(Vec::as_slice)
            .ok_or_else(|| PyValueError::new_err("malformed S-parameter result matrix"))
    }
}

#[pymethods]
impl PySParameterResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    #[getter]
    fn port_names(&self) -> Vec<String> {
        self.port_names.clone()
    }

    #[getter]
    fn reference_impedances<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.reference_impedances.to_pyarray(py)
    }

    #[getter]
    fn num_ports(&self) -> usize {
        self.port_names.len()
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.frequencies.len()
    }

    fn s<'py>(
        &self,
        py: Python<'py>,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        Ok(self.parameter(output_port, input_port)?.to_pyarray(py))
    }

    fn magnitude<'py>(
        &self,
        py: Python<'py>,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self
            .parameter(output_port, input_port)?
            .iter()
            .map(|value| value.norm())
            .collect::<Vec<_>>()
            .to_pyarray(py))
    }

    fn magnitude_db<'py>(
        &self,
        py: Python<'py>,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self
            .parameter(output_port, input_port)?
            .iter()
            .map(|value| 20.0 * value.norm().log10())
            .collect::<Vec<_>>()
            .to_pyarray(py))
    }

    fn phase_degrees<'py>(
        &self,
        py: Python<'py>,
        output_port: usize,
        input_port: usize,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self
            .parameter(output_port, input_port)?
            .iter()
            .map(|value| value.arg().to_degrees())
            .collect::<Vec<_>>()
            .to_pyarray(py))
    }

    fn __repr__(&self) -> String {
        format!(
            "SParameterResult(ports={}, points={})",
            self.port_names.len(),
            self.frequencies.len()
        )
    }
}

//=============================================================================
// Periodic Steady-State / RF Results
//=============================================================================

/// Periodic steady-state waveform and convergence diagnostics.
#[pyclass(name = "PssResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPssResult {
    inner: rspice_core::analysis::PssResult,
    #[pyo3(get)]
    pub iterations: usize,
    #[pyo3(get)]
    pub residual_norm: f64,
    #[pyo3(get)]
    pub period: f64,
    #[pyo3(get)]
    pub is_stable: bool,
}

impl PyPssResult {
    pub fn from_core(result: &rspice_core::engine::PssAnalysisResult) -> Self {
        Self {
            inner: result.result.clone(),
            iterations: result.iterations,
            residual_norm: result.final_residual,
            period: result.period,
            is_stable: result.is_stable,
        }
    }

    fn waveform_index(&self, node: &NodeIdentifier) -> PyResult<Option<usize>> {
        match node {
            NodeIdentifier::Index(0) => Ok(None),
            NodeIdentifier::Index(index) => {
                let waveform_index = index - 1;
                if waveform_index < self.inner.waveforms.len() {
                    Ok(Some(waveform_index))
                } else {
                    Err(invalid_node_index_error(*index, self.inner.waveforms.len()).into())
                }
            }
            NodeIdentifier::Name(name) if is_ground_name(name) => Ok(None),
            NodeIdentifier::Name(name) => self
                .inner
                .node_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .map(Some)
                .ok_or_else(|| unknown_node_name_error(name).into()),
        }
    }
}

#[pymethods]
impl PyPssResult {
    #[getter]
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.time.to_pyarray(py)
    }

    #[getter]
    fn frequency(&self) -> f64 {
        self.inner.frequency
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    #[getter]
    fn period_detected(&self) -> bool {
        self.inner.period_detected
    }

    #[getter]
    fn floquet_multipliers<'py>(
        &self,
        py: Python<'py>,
    ) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.inner.floquet_multipliers.to_pyarray(py)
    }

    #[getter]
    fn num_nodes(&self) -> usize {
        self.inner.waveforms.len()
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.inner.time.len()
    }

    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = match self.waveform_index(&node)? {
            Some(index) => self.inner.waveforms[index].values.clone(),
            None => vec![0.0; self.inner.time.len()],
        };
        Ok(values.to_pyarray(py))
    }

    fn voltage_at(&self, node: NodeIdentifier, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(PyValueError::new_err("time must be finite"));
        }
        match self.waveform_index(&node)? {
            Some(index) => Ok(self.inner.waveforms[index].interpolate(
                &self.inner.time,
                time,
                self.inner.period,
            )),
            None => Ok(0.0),
        }
    }

    fn dc(&self, node: NodeIdentifier) -> PyResult<f64> {
        match self.waveform_index(&node)? {
            Some(index) => Ok(self.inner.waveforms[index].dc(&self.inner.time, self.inner.period)),
            None => Ok(0.0),
        }
    }

    fn peak_to_peak(&self, node: NodeIdentifier) -> PyResult<f64> {
        match self.waveform_index(&node)? {
            Some(index) => Ok(self.inner.waveforms[index].peak_to_peak()),
            None => Ok(0.0),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "PssResult(frequency={:.6e}Hz, nodes={}, points={}, iterations={}, residual={:.3e})",
            self.inner.frequency,
            self.inner.waveforms.len(),
            self.inner.time.len(),
            self.iterations,
            self.residual_norm
        )
    }
}

/// Harmonic-balance spectra and convergence diagnostics.
#[pyclass(name = "HbResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyHbResult {
    inner: rspice_core::analysis::HbResult,
}

impl PyHbResult {
    pub fn from_core(result: &rspice_core::engine::HbAnalysisResult) -> Self {
        Self {
            inner: result.result.clone(),
        }
    }

    fn spectral_voltage(
        &self,
        node: &str,
    ) -> PyResult<&rspice_core::analysis::advanced::harmonic_balance::SpectralVoltage> {
        self.inner
            .spectral_voltages
            .iter()
            .find(|value| value.node_name.eq_ignore_ascii_case(node))
            .ok_or_else(|| PyKeyError::new_err(format!("unknown node '{node}'")))
    }
}

#[pymethods]
impl PyHbResult {
    #[getter]
    fn converged(&self) -> bool {
        self.inner.converged
    }

    #[getter]
    fn iterations(&self) -> usize {
        self.inner.iterations
    }

    #[getter]
    fn residual_norm(&self) -> f64 {
        self.inner.residual_norm
    }

    #[getter]
    fn fundamental_frequency(&self) -> f64 {
        self.inner.fundamental_freq
    }

    #[getter]
    fn num_harmonics(&self) -> usize {
        self.inner.num_harmonics
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    #[getter]
    fn harmonic_frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.harmonic_frequencies.to_pyarray(py)
    }

    #[getter]
    fn solve_time_seconds(&self) -> f64 {
        self.inner.solve_time_seconds
    }

    #[getter]
    fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    fn coefficients<'py>(
        &self,
        py: Python<'py>,
        node: &str,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        Ok(self.spectral_voltage(node)?.coefficients.to_pyarray(py))
    }

    fn magnitude<'py>(&self, py: Python<'py>, node: &str) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self
            .spectral_voltage(node)?
            .coefficients
            .iter()
            .map(|value| value.norm())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn phase_degrees<'py>(
        &self,
        py: Python<'py>,
        node: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self
            .spectral_voltage(node)?
            .coefficients
            .iter()
            .map(|value| value.arg().to_degrees())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn dc(&self, node: &str) -> PyResult<f64> {
        Ok(self.spectral_voltage(node)?.dc())
    }

    fn rms(&self, node: &str) -> PyResult<f64> {
        Ok(self.spectral_voltage(node)?.rms())
    }

    fn thd_percent(&self, node: &str) -> PyResult<f64> {
        Ok(self.spectral_voltage(node)?.thd_percent())
    }

    fn __repr__(&self) -> String {
        format!(
            "HbResult(fundamental={:.6e}Hz, harmonics={}, nodes={}, converged={})",
            self.inner.fundamental_freq,
            self.inner.num_harmonics,
            self.inner.node_names.len(),
            self.inner.converged
        )
    }
}

/// Periodic small-signal AC sideband conversion result.
#[pyclass(name = "PacResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPacResult {
    inner: rspice_core::analysis::advanced::PacResult,
    #[pyo3(get)]
    pub converged: bool,
}

impl PyPacResult {
    pub fn from_core(result: &rspice_core::engine::PacAnalysisResult) -> Self {
        Self {
            inner: result.result.clone(),
            converged: result.converged,
        }
    }

    fn validate_sideband(&self, sideband: i32) -> PyResult<()> {
        if sideband < self.inner.sideband_min || sideband > self.inner.sideband_max {
            return Err(PyIndexError::new_err(format!(
                "sideband {sideband} is outside [{}, {}]",
                self.inner.sideband_min, self.inner.sideband_max
            )));
        }
        Ok(())
    }
}

#[pymethods]
impl PyPacResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.frequencies.to_pyarray(py)
    }

    #[getter]
    fn fundamental_frequency(&self) -> f64 {
        self.inner.fundamental_frequency
    }

    #[getter]
    fn sideband_min(&self) -> i32 {
        self.inner.sideband_min
    }

    #[getter]
    fn sideband_max(&self) -> i32 {
        self.inner.sideband_max
    }

    #[getter]
    fn sidebands(&self) -> Vec<i32> {
        self.inner.sideband_indices()
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    #[getter]
    fn input_source(&self) -> Option<String> {
        self.inner.input_source.clone()
    }

    #[getter]
    fn output_node(&self) -> Option<String> {
        self.inner.output_node.clone()
    }

    fn voltage<'py>(
        &self,
        py: Python<'py>,
        node: &str,
        sideband: i32,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        self.validate_sideband(sideband)?;
        let node_index = self
            .inner
            .node_index(node)
            .ok_or_else(|| PyKeyError::new_err(format!("unknown node '{node}'")))?;
        let values = (0..self.inner.frequencies.len())
            .map(|frequency_index| self.inner.voltage(node_index, frequency_index, sideband))
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn conversion_gain<'py>(
        &self,
        py: Python<'py>,
        input_sideband: i32,
        output_sideband: i32,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        self.validate_sideband(input_sideband)?;
        self.validate_sideband(output_sideband)?;
        let values = (0..self.inner.frequencies.len())
            .map(|frequency_index| {
                self.inner
                    .conversion_gain(input_sideband, output_sideband, frequency_index)
            })
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn conversion_gain_db<'py>(
        &self,
        py: Python<'py>,
        input_sideband: i32,
        output_sideband: i32,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        self.validate_sideband(input_sideband)?;
        self.validate_sideband(output_sideband)?;
        let values = (0..self.inner.frequencies.len())
            .map(|frequency_index| {
                self.inner
                    .conversion_gain_db(input_sideband, output_sideband, frequency_index)
            })
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn __repr__(&self) -> String {
        format!(
            "PacResult(fundamental={:.6e}Hz, points={}, sidebands={}..={}, converged={})",
            self.inner.fundamental_frequency,
            self.inner.frequencies.len(),
            self.inner.sideband_min,
            self.inner.sideband_max,
            self.converged
        )
    }
}

/// One source's folded periodic-noise power spectral density.
#[pyclass(name = "PeriodicNoiseContribution", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPeriodicNoiseContribution {
    #[pyo3(get)]
    pub name: String,
    values: Vec<f64>,
}

#[pymethods]
impl PyPeriodicNoiseContribution {
    #[getter]
    fn power_spectral_density<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.values.to_pyarray(py)
    }
}

/// Periodic-noise result, in power spectral density units (V^2/Hz).
#[pyclass(name = "PeriodicNoiseResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPeriodicNoiseResult {
    frequencies: Vec<f64>,
    output_noise: Vec<f64>,
    input_noise: Option<Vec<f64>>,
    contributors: Vec<PyPeriodicNoiseContribution>,
    #[pyo3(get)]
    pub fundamental_frequency: f64,
    #[pyo3(get)]
    pub converged: bool,
}

/// Autonomous-oscillator single-sideband phase noise from PPV projection.
#[pyclass(name = "OscillatorNoiseResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyOscillatorNoiseResult {
    frequencies: Vec<f64>,
    phase_noise_dbc: Vec<f64>,
    #[pyo3(get)]
    pub diffusion_constant: f64,
    #[pyo3(get)]
    pub period: f64,
    #[pyo3(get)]
    pub corner_frequency: f64,
}

impl PyOscillatorNoiseResult {
    pub fn from_core(result: &rspice_core::engine::OscPnoiseResult) -> Self {
        Self {
            frequencies: result.frequencies.clone(),
            phase_noise_dbc: result.phase_noise_dbc.clone(),
            diffusion_constant: result.diffusion_constant,
            period: result.period,
            corner_frequency: result.corner_hz,
        }
    }
}

#[pymethods]
impl PyOscillatorNoiseResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    #[getter]
    fn phase_noise_dbc<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.phase_noise_dbc.to_pyarray(py)
    }

    #[getter]
    fn carrier_frequency(&self) -> f64 {
        1.0 / self.period
    }

    fn __repr__(&self) -> String {
        format!(
            "OscillatorNoiseResult(carrier={:.6e}Hz, points={}, corner={:.6e}Hz)",
            1.0 / self.period,
            self.frequencies.len(),
            self.corner_frequency
        )
    }
}

impl PyPeriodicNoiseResult {
    pub fn from_core(result: &rspice_core::engine::PnoiseAnalysisResult) -> Self {
        Self {
            frequencies: result.frequencies.clone(),
            output_noise: result.output_noise.clone(),
            input_noise: result.input_noise.clone(),
            contributors: result
                .contributors
                .iter()
                .map(|(name, values)| PyPeriodicNoiseContribution {
                    name: name.clone(),
                    values: values.clone(),
                })
                .collect(),
            fundamental_frequency: result.fundamental_freq,
            converged: result.converged,
        }
    }
}

#[pymethods]
impl PyPeriodicNoiseResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    #[getter]
    fn output_noise<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.output_noise.to_pyarray(py)
    }

    #[getter]
    fn output_noise_density<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.output_noise
            .iter()
            .map(|value| value.sqrt())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn input_noise<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.input_noise
            .as_ref()
            .map(|values| values.to_pyarray(py))
    }

    #[getter]
    fn contributors(&self) -> Vec<PyPeriodicNoiseContribution> {
        self.contributors.clone()
    }

    fn contribution(&self, name: &str) -> PyResult<PyPeriodicNoiseContribution> {
        self.contributors
            .iter()
            .find(|value| value.name.eq_ignore_ascii_case(name))
            .cloned()
            .ok_or_else(|| PyKeyError::new_err(format!("unknown noise contributor '{name}'")))
    }

    fn __repr__(&self) -> String {
        format!(
            "PeriodicNoiseResult(fundamental={:.6e}Hz, points={}, contributors={}, converged={})",
            self.fundamental_frequency,
            self.frequencies.len(),
            self.contributors.len(),
            self.converged
        )
    }
}

//=============================================================================
// Loop Stability Results
//=============================================================================

/// Loop-gain sweep and stability margins from Tian double-injection STB.
#[pyclass(name = "StbResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyStbResult {
    frequencies: Vec<f64>,
    loop_gains: Vec<rspice_core::Complex64>,
    #[pyo3(get)]
    pub probe_name: String,
    #[pyo3(get)]
    pub gain_margin_db: f64,
    #[pyo3(get)]
    pub gain_margin_frequency: f64,
    #[pyo3(get)]
    pub phase_margin_degrees: f64,
    #[pyo3(get)]
    pub phase_margin_frequency: f64,
    #[pyo3(get)]
    pub dc_gain_db: f64,
    #[pyo3(get)]
    pub unity_gain_bandwidth: f64,
    #[pyo3(get)]
    pub conditionally_stable: bool,
    #[pyo3(get)]
    pub num_crossovers: usize,
    #[pyo3(get)]
    pub success: bool,
    #[pyo3(get)]
    pub warnings: Vec<String>,
    assessment: String,
}

impl PyStbResult {
    pub fn from_core(result: &rspice_core::engine::StbAnalysisResult) -> Self {
        let margins = &result.result.margins;
        Self {
            frequencies: result.frequencies.clone(),
            loop_gains: result.loop_gains.clone(),
            probe_name: result.probe_name.clone(),
            gain_margin_db: margins.gain_margin_db,
            gain_margin_frequency: margins.gain_margin_freq,
            phase_margin_degrees: margins.phase_margin_deg,
            phase_margin_frequency: margins.phase_margin_freq,
            dc_gain_db: margins.dc_gain_db,
            unity_gain_bandwidth: margins.unity_gain_bandwidth,
            conditionally_stable: margins.conditionally_stable,
            num_crossovers: margins.num_crossovers,
            success: result.result.success,
            warnings: result.result.warnings.clone(),
            assessment: result.result.assessment(),
        }
    }
}

#[pymethods]
impl PyStbResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    #[getter]
    fn loop_gain<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.loop_gains.to_pyarray(py)
    }

    #[getter]
    fn magnitude<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.loop_gains
            .iter()
            .map(|value| value.norm())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn magnitude_db<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.loop_gains
            .iter()
            .map(|value| 20.0 * value.norm().log10())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn phase_degrees<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.loop_gains
            .iter()
            .map(|value| value.arg().to_degrees())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn is_stable(&self) -> bool {
        self.gain_margin_db > 0.0 && self.phase_margin_degrees > 0.0
    }

    #[getter]
    fn assessment(&self) -> String {
        self.assessment.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "StbResult(probe='{}', points={}, gain_margin={:.2}dB, phase_margin={:.2}deg, assessment='{}')",
            self.probe_name,
            self.frequencies.len(),
            self.gain_margin_db,
            self.phase_margin_degrees,
            self.assessment
        )
    }
}

//=============================================================================
// Transfer Function Results
//=============================================================================

/// Small-signal transfer function result (.TF)
///
/// Example:
///     >>> tf = engine.run_transfer_function(netlist, "out", "V1")
///     >>> print(f"gain={tf.gain:.3f}, Zin={tf.input_impedance:.1f}Ω")
#[pyclass(name = "TransferFunctionResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyTransferFunctionResult {
    /// Output specification
    #[pyo3(get)]
    pub output: String,
    /// Input source name
    #[pyo3(get)]
    pub input: String,
    /// DC small-signal gain (output / input)
    #[pyo3(get)]
    pub gain: f64,
    /// Input impedance in Ohms
    #[pyo3(get)]
    pub input_impedance: f64,
    /// Output impedance (Thevenin) in Ohms
    #[pyo3(get)]
    pub output_impedance: f64,
}

impl PyTransferFunctionResult {
    pub fn from_core(result: &rspice_core::analysis::TransferFunctionResult) -> Self {
        Self {
            output: result.output.clone(),
            input: result.input.clone(),
            gain: result.gain,
            input_impedance: result.input_impedance,
            output_impedance: result.output_impedance,
        }
    }
}

#[pymethods]
impl PyTransferFunctionResult {
    /// Gain in dB (20·log10 |gain|)
    #[getter]
    fn gain_db(&self) -> f64 {
        20.0 * self.gain.abs().log10()
    }

    fn __repr__(&self) -> String {
        format!(
            "TransferFunctionResult({}/{}: gain={:.4e}, Zin={:.4e}, Zout={:.4e})",
            self.output, self.input, self.gain, self.input_impedance, self.output_impedance
        )
    }
}

//=============================================================================
// Measurement / Run Report
//=============================================================================

/// Result of a single .MEAS statement
///
/// `passed` is true when the measurement evaluated to a value and, if the
/// statement declared `GOAL=` (optionally `TOL=`), the value landed within
/// tolerance. A failed measurement carries an `error` message.
///
/// Example:
///     >>> m = report.measurement("trise")
///     >>> assert m.passed and m.value < 1e-6
#[pyclass(name = "Measurement", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyMeasurement {
    /// Measurement name (from the .MEAS statement)
    #[pyo3(get)]
    pub name: String,
    /// Analysis the measurement applies to ("TRAN", "AC", "DC")
    #[pyo3(get)]
    pub analysis: String,
    /// Measured value, or None if evaluation failed
    #[pyo3(get)]
    pub value: Option<f64>,
    /// Failure description when evaluation failed
    #[pyo3(get)]
    pub error: Option<String>,
    /// Declared GOAL, when the statement carried one
    #[pyo3(get)]
    pub expected: Option<f64>,
    /// Effective tolerance applied to the GOAL check
    #[pyo3(get)]
    pub tolerance: Option<f64>,
    pub(crate) ok: bool,
}

impl PyMeasurement {
    pub fn from_core(result: &rspice_core::MeasureResult, analysis: &str) -> Self {
        Self {
            name: result.name.clone(),
            analysis: analysis.to_string(),
            value: result.value,
            error: result.error.clone(),
            expected: result.expected,
            tolerance: result.tolerance,
            ok: result.passed,
        }
    }

    pub fn unevaluated(name: &str, analysis: &str, reason: &str) -> Self {
        Self {
            name: name.to_string(),
            analysis: analysis.to_string(),
            value: None,
            error: Some(reason.to_string()),
            expected: None,
            tolerance: None,
            ok: false,
        }
    }

    fn failure_message(&self) -> String {
        if let Some(error) = &self.error {
            return error.clone();
        }
        match (self.value, self.expected, self.tolerance) {
            (Some(value), Some(expected), Some(tolerance)) => {
                format!("value {value:.6e} is outside goal {expected:.6e} +/- {tolerance:.6e}")
            }
            (Some(value), Some(expected), None) => {
                format!("value {value:.6e} did not meet goal {expected:.6e}")
            }
            (Some(value), None, _) => format!("measurement failed with value {value:.6e}"),
            (None, _, _) => "evaluation failed".to_string(),
        }
    }
}

#[pymethods]
impl PyMeasurement {
    /// True when the measurement produced a value within any declared GOAL
    #[getter]
    fn passed(&self) -> bool {
        self.ok
    }

    /// Convert to float; raises ValueError when the measurement failed
    fn __float__(&self) -> PyResult<f64> {
        match (self.ok, self.value) {
            (true, Some(value)) => Ok(value),
            _ => Err(PyValueError::new_err(format!(
                "measurement '{}' failed: {}",
                self.name,
                self.failure_message()
            ))),
        }
    }

    fn __repr__(&self) -> String {
        if self.ok {
            match self.value {
                Some(v) => format!("Measurement({}={:.6e} [{}])", self.name, v, self.analysis),
                None => format!(
                    "Measurement({} FAILED [{}]: {})",
                    self.name,
                    self.analysis,
                    self.failure_message()
                ),
            }
        } else {
            format!(
                "Measurement({} FAILED [{}]: {})",
                self.name,
                self.analysis,
                self.failure_message()
            )
        }
    }
}

/// Record of one analysis directive handled by `Engine.run`
#[pyclass(name = "AnalysisRecord", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyAnalysisRecord {
    /// Analysis kind: "op", "dc", "ac", "tran", "noise", "tf", "four", ...
    #[pyo3(get)]
    pub kind: String,
    /// Human-readable summary of the directive
    #[pyo3(get)]
    pub detail: String,
    /// True when the directive was not executed
    #[pyo3(get)]
    pub skipped: bool,
    /// Why the directive was skipped (when skipped)
    #[pyo3(get)]
    pub reason: Option<String>,
}

impl PyAnalysisRecord {
    pub fn executed(kind: &str, detail: String) -> Self {
        Self {
            kind: kind.to_string(),
            detail,
            skipped: false,
            reason: None,
        }
    }

    pub fn skipped(kind: &str, detail: String, reason: &str) -> Self {
        Self {
            kind: kind.to_string(),
            detail,
            skipped: true,
            reason: Some(reason.to_string()),
        }
    }
}

#[pymethods]
impl PyAnalysisRecord {
    fn __repr__(&self) -> String {
        if self.skipped {
            format!(
                "AnalysisRecord({} SKIPPED: {})",
                self.detail,
                self.reason.as_deref().unwrap_or("")
            )
        } else {
            format!("AnalysisRecord({})", self.detail)
        }
    }
}

/// Aggregated outcome of `Engine.run`: every analysis the netlist requested
/// plus all .MEAS verification results.
///
/// Designed for CI: `report.assert_passed()` raises `MeasurementError` if
/// any directive was skipped, any measurement failed (or none were evaluated),
/// with a message listing each failure.
///
/// Example:
///     >>> report = engine.run(netlist)
///     >>> report.assert_passed()
///     >>> tpd = report.measurement("tpd").value
#[pyclass(name = "RunReport", module = "rspice")]
pub struct PyRunReport {
    /// DC operating point result (last .op)
    #[pyo3(get)]
    pub op: Option<Py<PySimulationResult>>,
    /// DC sweep result (last .dc)
    #[pyo3(get)]
    pub dc: Option<Py<PyDcSweepResult>>,
    /// Transient result (last .tran)
    #[pyo3(get)]
    pub tran: Option<Py<PyTransientResult>>,
    /// AC result (last .ac)
    #[pyo3(get)]
    pub ac: Option<Py<PyAcResult>>,
    /// Third-order Volterra distortion result (last .disto)
    #[pyo3(get)]
    pub distortion: Option<Py<PyDistortionResult>>,
    /// N-port scattering parameters (last .sp)
    #[pyo3(get)]
    pub s_parameters: Option<PySParameterResult>,
    /// Noise results (last .noise)
    #[pyo3(get)]
    pub noise: Option<Vec<PyNoiseResult>>,
    /// Transfer function result (last .tf)
    #[pyo3(get)]
    pub tf: Option<PyTransferFunctionResult>,
    /// Loop-stability result (last .stb)
    #[pyo3(get)]
    pub stb: Option<PyStbResult>,
    /// Pole-zero result (last .pz)
    #[pyo3(get)]
    pub pz: Option<PyPoleZeroResult>,
    /// Monte Carlo result (last .mc)
    #[pyo3(get)]
    pub monte_carlo: Option<PyMonteCarloResult>,
    /// Parametric DC operating-point sweep (last .step)
    #[pyo3(get)]
    pub step: Option<PyDcSweepResult>,
    /// Temperature DC operating-point sweep (last .temp)
    #[pyo3(get)]
    pub temperature: Option<PyDcSweepResult>,
    /// Adjoint DC sensitivity result (last DC .sens)
    #[pyo3(get)]
    pub sensitivity: Option<PySensitivityResult>,
    /// Fourier results (one per .four output)
    #[pyo3(get)]
    pub fourier: Vec<PyFourierResult>,
    /// One record per analysis directive in the netlist
    #[pyo3(get)]
    pub records: Vec<PyAnalysisRecord>,
    /// All measurement outcomes
    #[pyo3(get)]
    pub measurements: Vec<PyMeasurement>,
}

#[pymethods]
impl PyRunReport {
    /// Look up a measurement by name (case-insensitive)
    fn measurement(&self, name: &str) -> Option<PyMeasurement> {
        self.measurements
            .iter()
            .find(|m| m.name.eq_ignore_ascii_case(name))
            .cloned()
    }

    /// Number of measurements evaluated
    #[getter]
    fn num_measurements(&self) -> usize {
        self.measurements.len()
    }

    /// True when no analysis directive was skipped, at least one measurement
    /// was evaluated, and every measurement passed.
    #[getter]
    fn all_passed(&self) -> bool {
        !self.measurements.is_empty()
            && self.records.iter().all(|r| !r.skipped)
            && self.measurements.iter().all(|m| m.ok)
    }

    /// Measurements that failed to evaluate or failed their goal/tolerance check
    #[getter]
    fn failures(&self) -> Vec<PyMeasurement> {
        self.measurements
            .iter()
            .filter(|m| !m.ok)
            .cloned()
            .collect()
    }

    /// Kinds of analyses that actually executed (e.g. ["op", "tran"])
    #[getter]
    fn analyses_run(&self) -> Vec<String> {
        self.records
            .iter()
            .filter(|r| !r.skipped)
            .map(|r| r.kind.clone())
            .collect()
    }

    /// Records for directives that were skipped
    #[getter]
    fn skipped(&self) -> Vec<PyAnalysisRecord> {
        self.records.iter().filter(|r| r.skipped).cloned().collect()
    }

    /// Raise MeasurementError unless every requested analysis ran, at least
    /// one measurement was evaluated, and all of them passed.
    ///
    /// This is the CI primitive: a netlist whose .MEAS statements were
    /// silently skipped fails loudly instead of green-washing a pipeline.
    fn assert_passed(&self) -> PyResult<()> {
        let skipped = self.skipped();
        if !skipped.is_empty() {
            let mut message = format!(
                "{} of {} analysis directives were skipped:",
                skipped.len(),
                self.records.len()
            );
            for record in &skipped {
                message.push_str(&format!(
                    "\n  {}: {}",
                    record.detail,
                    record.reason.as_deref().unwrap_or("skipped")
                ));
            }
            return Err(crate::errors::MeasurementError::new_err(message));
        }
        if self.measurements.is_empty() {
            return Err(crate::errors::MeasurementError::new_err(
                "no measurements were evaluated: the netlist has no .MEAS statements \
                 covered by the analyses that ran",
            ));
        }
        let failures = self.failures();
        if failures.is_empty() {
            return Ok(());
        }
        let mut message = format!(
            "{} of {} measurements failed:",
            failures.len(),
            self.measurements.len()
        );
        for f in &failures {
            message.push_str(&format!(
                "\n  {} [{}]: {}",
                f.name,
                f.analysis,
                f.failure_message()
            ));
        }
        Err(crate::errors::MeasurementError::new_err(message))
    }

    fn __repr__(&self) -> String {
        let executed = self.records.iter().filter(|r| !r.skipped).count();
        let skipped = self.records.len() - executed;
        format!(
            "RunReport(analyses={}, skipped={}, measurements={}, all_passed={})",
            executed,
            skipped,
            self.measurements.len(),
            self.all_passed()
        )
    }
}

/// Helper enum for node identification (by index or name)
#[derive(FromPyObject, Debug, Clone)]
pub enum NodeIdentifier {
    #[pyo3(transparent)]
    Index(usize),
    #[pyo3(transparent)]
    Name(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::Complex64;

    fn ac_row(frequency: f64, voltages: Vec<Complex64>, currents: Vec<Complex64>) -> AcResult {
        AcResult {
            frequency,
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            voltages,
            currents,
        }
    }

    #[test]
    fn ac_voltage_access_rejects_short_later_rows() {
        let ac = PyAcResult::new(
            vec![1.0, 2.0],
            vec![
                ac_row(1.0, vec![Complex64::new(1.0, 0.0)], Vec::new()),
                ac_row(2.0, Vec::new(), Vec::new()),
            ],
        );

        let message = ac
            .voltage_phasor_from_row(1, &ac.results[1], 1)
            .unwrap_err();
        assert!(message.contains("malformed AC result row 1"), "{message}");
        assert!(message.contains("missing voltage"), "{message}");
    }

    #[test]
    fn ac_branch_access_rejects_short_later_rows() {
        let ac = PyAcResult::new(
            vec![1.0, 2.0],
            vec![
                ac_row(
                    1.0,
                    vec![Complex64::new(1.0, 0.0)],
                    vec![Complex64::new(0.0, 1.0)],
                ),
                ac_row(2.0, vec![Complex64::new(1.0, 0.0)], Vec::new()),
            ],
        );

        let message = ac
            .branch_current_from_row(1, &ac.results[1], 0)
            .unwrap_err();
        assert!(message.contains("malformed AC result row 1"), "{message}");
        assert!(message.contains("missing current"), "{message}");
    }
}
