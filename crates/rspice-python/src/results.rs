//! Simulation results Python bindings with NumPy integration
//!
//! Provides Python access to simulation results:
//! - `SimulationResult` - DC operating point results
//! - `TransientResult` - Time-domain waveforms (voltages and branch currents)
//! - `AcResult` - Frequency-domain complex phasors
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
use rspice_core::analysis::AcResult;
use rspice_core::analysis::{FourierAnalysis, FourierConfig};
use rspice_core::engine::TransientResult;
use rspice_core::solver::SimulationResult;

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
#[pyclass(name = "SimulationResult")]
pub struct PySimulationResult {
    pub(crate) inner: SimulationResult,
}

impl PySimulationResult {
    pub fn new(inner: SimulationResult) -> Self {
        Self { inner }
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

/// Transient simulation result with time-domain waveforms
///
/// Contains time points, node voltage waveforms, and branch current
/// waveforms. Arrays are returned as NumPy ndarrays.
///
/// Example:
///     >>> result = engine.run_tran(netlist, 1e-3, 1e-6)
///     >>> import matplotlib.pyplot as plt
///     >>> plt.plot(result.time, result.voltage_waveform("out"))
#[pyclass(name = "TransientResult")]
pub struct PyTransientResult {
    pub(crate) inner: TransientResult,
}

impl PyTransientResult {
    pub fn new(inner: TransientResult) -> Self {
        Self { inner }
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
#[pyclass(name = "AcResult")]
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
        let magnitudes: Vec<f64> = self
            .results
            .iter()
            .map(|r| r.voltage_magnitude(idx))
            .collect();
        Ok(magnitudes.to_pyarray(py))
    }

    /// Get voltage phase at a node across all frequencies (radians)
    fn voltage_phase<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let phases: Vec<f64> = self.results.iter().map(|r| r.voltage_phase(idx)).collect();
        Ok(phases.to_pyarray(py))
    }

    /// Get voltage phase at a node across all frequencies (degrees)
    fn voltage_phase_degrees<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let phases: Vec<f64> = self
            .results
            .iter()
            .map(|r| r.voltage_phase(idx).to_degrees())
            .collect();
        Ok(phases.to_pyarray(py))
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
        let zero = rspice_core::Complex64::new(0.0, 0.0);
        let values: Vec<rspice_core::Complex64> = self
            .results
            .iter()
            .map(|r| {
                if idx == 0 {
                    zero
                } else {
                    r.voltages.get(idx - 1).copied().unwrap_or(zero)
                }
            })
            .collect();
        Ok(values.to_pyarray(py))
    }

    /// Get voltage magnitude in dB (20·log10 |V|) at a node across all frequencies
    fn voltage_db<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let db: Vec<f64> = self.results.iter().map(|r| r.voltage_db(idx)).collect();
        Ok(db.to_pyarray(py))
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
        let zero = rspice_core::Complex64::new(0.0, 0.0);
        let values: Vec<rspice_core::Complex64> = self
            .results
            .iter()
            .map(|r| r.currents.get(idx).copied().unwrap_or(zero))
            .collect();
        Ok(values.to_pyarray(py))
    }

    /// Get branch current magnitude through an element across all frequencies
    fn branch_current_magnitude<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_branch(name).map_err(PyErr::from)?;
        let values: Vec<f64> = self
            .results
            .iter()
            .map(|r| r.currents.get(idx).map(|c| c.norm()).unwrap_or(0.0))
            .collect();
        Ok(values.to_pyarray(py))
    }

    /// Get voltage magnitude at a specific frequency index and node
    ///
    /// Raises:
    ///     IndexError: If the frequency index is out of range
    pub fn magnitude_at(&self, freq_index: usize, node: NodeIdentifier) -> PyResult<f64> {
        self.checked_freq_index(freq_index).map_err(PyErr::from)?;
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        Ok(self.results[freq_index].voltage_magnitude(idx))
    }

    /// Get phase at a specific frequency index and node (radians)
    ///
    /// Raises:
    ///     IndexError: If the frequency index is out of range
    fn phase_at(&self, freq_index: usize, node: NodeIdentifier) -> PyResult<f64> {
        self.checked_freq_index(freq_index).map_err(PyErr::from)?;
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        Ok(self.results[freq_index].voltage_phase(idx))
    }

    fn __repr__(&self) -> String {
        let freq_range = if !self.frequencies.is_empty() {
            format!(
                "{:.1e}-{:.1e} Hz",
                self.frequencies.first().unwrap(),
                self.frequencies.last().unwrap()
            )
        } else {
            "no frequencies".to_string()
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
#[pyclass(name = "DcSweepResult")]
pub struct PyDcSweepResult {
    pub(crate) results: Vec<(f64, SimulationResult)>,
}

impl PyDcSweepResult {
    pub fn new(results: Vec<(f64, SimulationResult)>) -> Self {
        Self { results }
    }

    fn point(&self, index: usize) -> AccessResult<&(f64, SimulationResult)> {
        self.results
            .get(index)
            .ok_or_else(|| invalid_sweep_index_error(index, self.results.len()))
    }
}

#[pymethods]
impl PyDcSweepResult {
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
            .map(|(value, result)| (*value, PySimulationResult::new(result.clone())))
            .collect()
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
        Ok((*value, PySimulationResult::new(result.clone())))
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
        let sweep_range = if !self.results.is_empty() {
            format!(
                "{:.2}-{:.2}",
                self.results.first().unwrap().0,
                self.results.last().unwrap().0
            )
        } else {
            "empty".to_string()
        };
        format!(
            "DcSweepResult(points={}, range={})",
            self.results.len(),
            sweep_range
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
#[pyclass(name = "NoiseContribution")]
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
#[pyclass(name = "NoiseResult")]
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

    /// Get output noise in dBV/Hz
    #[getter]
    fn output_noise_dbv(&self) -> f64 {
        if self.output_noise_density > 0.0 {
            10.0 * self.output_noise_density.log10()
        } else {
            -200.0
        }
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
// Monte Carlo Analysis Results
//=============================================================================

/// Statistics for a single output variable from Monte Carlo analysis
///
/// Example:
///     >>> stats = result.get_variable("V(out)")
///     >>> print(f"Mean: {stats.mean:.3f}, Std: {stats.std_dev:.3f}")
///     >>> print(f"3σ range: {stats.three_sigma_range}")
#[pyclass(name = "VariableStatistics")]
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

    /// Get a specific percentile value (0-100, linear interpolation)
    fn percentile(&self, pct: f64) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let mut sorted = self.samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let rank = (pct.clamp(0.0, 100.0) / 100.0) * (sorted.len() - 1) as f64;
        let lo = rank.floor() as usize;
        let hi = rank.ceil() as usize;
        if lo == hi {
            sorted[lo]
        } else {
            let frac = rank - lo as f64;
            sorted[lo] * (1.0 - frac) + sorted[hi] * frac
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

    /// Get coefficient of variation (std_dev / mean) as percentage
    #[getter]
    fn cv_percent(&self) -> f64 {
        if self.mean.abs() > 1e-15 {
            (self.std_dev / self.mean.abs()) * 100.0
        } else {
            0.0
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
#[pyclass(name = "MonteCarloResult")]
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
#[pyclass(name = "ComplexValue")]
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
        self.imag.abs() < 1e-10
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

    /// Get time constant τ = -1/Re(p) for real pole
    #[getter]
    fn time_constant(&self) -> Option<f64> {
        if self.is_real() && self.real.abs() > 1e-15 {
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
#[pyclass(name = "PoleZeroResult")]
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

    /// Check if system is stable (no pole with Re > 1e-10)
    #[getter]
    fn is_stable(&self) -> bool {
        self.poles.iter().all(|p| p.real < 1e-10)
    }

    /// Get the dominant pole (closest to imaginary axis with Re < 0)
    fn dominant_pole(&self) -> Option<PyComplexValue> {
        self.poles
            .iter()
            .filter(|p| p.real < 0.0 && p.real.is_finite())
            .min_by(|a, b| {
                a.real
                    .abs()
                    .partial_cmp(&b.real.abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }

    /// Get bandwidth in Hz (frequency of dominant pole)
    #[getter]
    fn bandwidth_hz(&self) -> Option<f64> {
        self.dominant_pole()
            .map(|p| p.real.abs() / (2.0 * std::f64::consts::PI))
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
#[pyclass(name = "Harmonic")]
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
#[pyclass(name = "FourierResult")]
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
// Transfer Function Results
//=============================================================================

/// Small-signal transfer function result (.TF)
///
/// Example:
///     >>> tf = engine.run_transfer_function(netlist, "out", "V1")
///     >>> print(f"gain={tf.gain:.3f}, Zin={tf.input_impedance:.1f}Ω")
#[pyclass(name = "TransferFunctionResult")]
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
#[pyclass(name = "Measurement")]
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
        self.value.ok_or_else(|| {
            PyValueError::new_err(format!(
                "measurement '{}' has no value: {}",
                self.name,
                self.error.as_deref().unwrap_or("evaluation failed")
            ))
        })
    }

    fn __repr__(&self) -> String {
        match self.value {
            Some(v) => format!("Measurement({}={:.6e} [{}])", self.name, v, self.analysis),
            None => format!(
                "Measurement({} FAILED [{}]: {})",
                self.name,
                self.analysis,
                self.error.as_deref().unwrap_or("evaluation failed")
            ),
        }
    }
}

/// Record of one analysis directive handled by `Engine.run`
#[pyclass(name = "AnalysisRecord")]
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
/// any measurement failed (or none were evaluated), with a message listing
/// each failure.
///
/// Example:
///     >>> report = engine.run(netlist)
///     >>> report.assert_passed()
///     >>> tpd = report.measurement("tpd").value
#[pyclass(name = "RunReport")]
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
    /// Noise results (last .noise)
    #[pyo3(get)]
    pub noise: Option<Vec<PyNoiseResult>>,
    /// Transfer function result (last .tf)
    #[pyo3(get)]
    pub tf: Option<PyTransferFunctionResult>,
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

    /// True when every measurement produced a value (vacuously true with none)
    #[getter]
    fn all_passed(&self) -> bool {
        self.measurements.iter().all(|m| m.value.is_some())
    }

    /// Measurements that failed to evaluate
    #[getter]
    fn failures(&self) -> Vec<PyMeasurement> {
        self.measurements
            .iter()
            .filter(|m| m.value.is_none())
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

    /// Raise MeasurementError unless at least one measurement was evaluated
    /// and all of them passed.
    ///
    /// This is the CI primitive: a netlist whose .MEAS statements were
    /// silently skipped fails loudly instead of green-washing a pipeline.
    fn assert_passed(&self) -> PyResult<()> {
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
                f.error.as_deref().unwrap_or("evaluation failed")
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
