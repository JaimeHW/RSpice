//! Simulation results Python bindings with NumPy integration
//!
//! Provides Python access to simulation results:
//! - `SimulationResult` - DC operating point results
//! - `TransientResult` - Time-domain waveforms
//! - `AcResult` - Frequency-domain complex phasors
//! - `DcSweepResult` - Collection of DC solutions

use numpy::{PyArray1, ToPyArray};
use pyo3::exceptions::{PyIndexError, PyKeyError};
use pyo3::prelude::*;
use rspice_core::analysis::AcResult;
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
    UnknownNodeName {
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
            ResultAccessError::UnknownNodeName { name } => {
                PyKeyError::new_err(format!("unknown node '{name}'"))
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

fn unknown_node_name_error(name: &str) -> ResultAccessError {
    ResultAccessError::UnknownNodeName {
        name: name.to_string(),
    }
}

fn is_ground_name(name: &str) -> bool {
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
    inner: SimulationResult,
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

    /// Get voltage at a node by index (internal method for tests)
    pub fn voltage_by_index(&self, node: usize) -> PyResult<f64> {
        self.checked_voltage(node).map_err(PyErr::from)
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
    ///     float or None: Current through the element, or None if not found
    fn branch_current(&self, name: &str) -> Option<f64> {
        self.inner.branch_current_named(name).or_else(|| {
            // Preserve legacy index-based fallback for synthetic test results
            // that have branch currents but no canonical branch metadata.
            if !self.inner.branch_currents.is_empty()
                && (name.to_uppercase().starts_with('V') || name.to_uppercase().starts_with('L'))
                && let Ok(idx) = name[1..].parse::<usize>()
            {
                return self
                    .inner
                    .branch_currents
                    .get(idx.saturating_sub(1))
                    .copied();
            }
            None
        })
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
/// Contains time points and voltage waveforms for all nodes.
/// Arrays are returned as NumPy ndarrays for efficient numerical processing.
///
/// Example:
///     >>> result = engine.run_tran(netlist, 1e-3, 1e-6)
///     >>> import matplotlib.pyplot as plt
///     >>> plt.plot(result.time, result.voltage_waveform(2))
#[pyclass(name = "TransientResult")]
pub struct PyTransientResult {
    inner: TransientResult,
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
        let waveform = match node {
            NodeIdentifier::Index(idx) => self.checked_waveform(idx)?,
            NodeIdentifier::Name(name) => self.checked_waveform_named(&name)?,
        };
        Ok(waveform.to_pyarray(py))
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
/// Contains frequencies and complex voltage phasors for each node
/// at each frequency. Arrays are returned as NumPy ndarrays.
///
/// Example:
///     >>> result = engine.run_ac(netlist, [10, 100, 1000])
///     >>> mag_db = 20 * np.log10(result.voltage_magnitude(2))
///     >>> phase_deg = np.degrees(result.voltage_phase(2))
#[pyclass(name = "AcResult")]
pub struct PyAcResult {
    frequencies: Vec<f64>,
    results: Vec<AcResult>,
}

impl PyAcResult {
    pub fn new(frequencies: Vec<f64>, results: Vec<AcResult>) -> Self {
        Self {
            frequencies,
            results,
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

    /// Get voltage magnitude at a node across all frequencies
    ///
    /// Args:
    ///     node: Node index
    ///
    /// Returns:
    ///     numpy.ndarray: Magnitude values at each frequency
    fn voltage_magnitude<'py>(&self, py: Python<'py>, node: usize) -> Bound<'py, PyArray1<f64>> {
        let magnitudes: Vec<f64> = self
            .results
            .iter()
            .map(|r| r.voltage_magnitude(node))
            .collect();
        magnitudes.to_pyarray(py)
    }

    /// Get voltage phase at a node across all frequencies
    ///
    /// Args:
    ///     node: Node index
    ///
    /// Returns:
    ///     numpy.ndarray: Phase values in radians at each frequency
    fn voltage_phase<'py>(&self, py: Python<'py>, node: usize) -> Bound<'py, PyArray1<f64>> {
        let phases: Vec<f64> = self.results.iter().map(|r| r.voltage_phase(node)).collect();
        phases.to_pyarray(py)
    }

    /// Get voltage phase at a node across all frequencies in degrees
    ///
    /// Args:
    ///     node: Node index
    ///
    /// Returns:
    ///     numpy.ndarray: Phase values in degrees at each frequency
    fn voltage_phase_degrees<'py>(
        &self,
        py: Python<'py>,
        node: usize,
    ) -> Bound<'py, PyArray1<f64>> {
        let phases: Vec<f64> = self
            .results
            .iter()
            .map(|r| r.voltage_phase(node).to_degrees())
            .collect();
        phases.to_pyarray(py)
    }

    /// Get complex voltage at a node across all frequencies
    ///
    /// Args:
    ///     node: Node index
    ///
    /// Returns:
    ///     numpy.ndarray: complex128 phasor values at each frequency
    ///
    /// Example:
    ///     >>> h = ac.voltage_complex(2) / ac.voltage_complex(1)  # transfer fn
    fn voltage_complex<'py>(
        &self,
        py: Python<'py>,
        node: usize,
    ) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        let zero = rspice_core::Complex64::new(0.0, 0.0);
        let values: Vec<rspice_core::Complex64> = self
            .results
            .iter()
            .map(|r| {
                // Node 0 is ground, so voltages are 0-indexed for node-1
                if node == 0 {
                    zero
                } else {
                    r.voltages.get(node - 1).copied().unwrap_or(zero)
                }
            })
            .collect();
        values.to_pyarray(py)
    }

    /// Get voltage magnitude in dB (20·log10 |V|) at a node across all frequencies
    ///
    /// Args:
    ///     node: Node index
    ///
    /// Returns:
    ///     numpy.ndarray: Magnitude in dB at each frequency
    fn voltage_db<'py>(&self, py: Python<'py>, node: usize) -> Bound<'py, PyArray1<f64>> {
        let db: Vec<f64> = self.results.iter().map(|r| r.voltage_db(node)).collect();
        db.to_pyarray(py)
    }

    /// Get voltage magnitude at a specific frequency and node
    ///
    /// Args:
    ///     freq_index: Index into the frequency array
    ///     node: Node index
    ///
    /// Returns:
    ///     float: Voltage magnitude at that frequency/node
    pub fn magnitude_at(&self, freq_index: usize, node: usize) -> f64 {
        self.results
            .get(freq_index)
            .map(|r| r.voltage_magnitude(node))
            .unwrap_or(0.0)
    }

    /// Get phase at a specific frequency index and node
    fn phase_at(&self, freq_index: usize, node: usize) -> f64 {
        self.results
            .get(freq_index)
            .map(|r| r.voltage_phase(node))
            .unwrap_or(0.0)
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
            "AcResult(frequencies={}, range={})",
            self.frequencies.len(),
            freq_range
        )
    }
}

/// DC sweep analysis result
///
/// Contains a collection of DC operating point solutions at different
/// sweep values.
///
/// Example:
///     >>> result = engine.run_dc_sweep(netlist, "V1", 0, 5, 0.1)
///     >>> for i in range(len(result)):
///     ...     v_in = result.sweep_values[i]
///     ...     v_out = result.result_at(i).voltage(2)
///     ...     print(f"Vin={v_in:.1f}V -> Vout={v_out:.3f}V")
#[pyclass(name = "DcSweepResult")]
pub struct PyDcSweepResult {
    results: Vec<(f64, SimulationResult)>,
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

    /// Get the number of sweep points
    #[getter]
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Get all sweep points as (value, result) pairs
    ///
    /// Returns:
    ///     list[tuple[float, SimulationResult]]: One entry per sweep point
    ///
    /// Example:
    ///     >>> for v_in, sol in result.points():
    ///     ...     print(f"V1={v_in:.1f}V -> V(out)={sol.voltage('out'):.3f}V")
    pub fn points(&self) -> Vec<(f64, PySimulationResult)> {
        self.results
            .iter()
            .map(|(value, result)| (*value, PySimulationResult::new(result.clone())))
            .collect()
    }

    /// Index into the sweep: `result[i]` -> (value, SimulationResult)
    ///
    /// Supports negative indices and the Python iteration protocol, so
    /// `for value, solution in result:` just works.
    fn __getitem__(&self, index: isize) -> PyResult<(f64, PySimulationResult)> {
        let len = self.results.len() as isize;
        let idx = if index < 0 { index + len } else { index };
        if idx < 0 || idx >= len {
            return Err(
                invalid_sweep_index_error(index.unsigned_abs(), self.results.len()).into(),
            );
        }
        let (value, result) = &self.results[idx as usize];
        Ok((*value, PySimulationResult::new(result.clone())))
    }

    /// Get the result at a specific sweep index (as Py wrapper)
    ///
    /// Args:
    ///     index: Sweep point index
    ///
    /// Returns:
    ///     SimulationResult: DC solution at that sweep point
    pub fn result_at(&self, index: usize) -> Option<PySimulationResult> {
        self.results
            .get(index)
            .map(|(_, r)| PySimulationResult::new(r.clone()))
    }

    /// Get the sweep value at a specific index (internal for tests)
    pub fn voltage_at(&self, index: usize) -> PyResult<f64> {
        self.point(index)
            .map(|(value, _)| *value)
            .map_err(PyErr::from)
    }

    /// Get voltage at a node for a specific sweep point
    ///
    /// Raises:
    ///     IndexError: If the sweep point or node index is out of range
    fn voltage(&self, index: usize, node: usize) -> PyResult<f64> {
        let (_, result) = self.point(index).map_err(PyErr::from)?;
        checked_simulation_voltage(result, node).map_err(PyErr::from)
    }

    /// Get voltage at a node across all sweep points as a NumPy array
    ///
    /// Raises:
    ///     IndexError: If the node index is out of range
    fn voltage_array<'py>(
        &self,
        py: Python<'py>,
        node: usize,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let mut voltages = Vec::with_capacity(self.results.len());
        for (_, result) in &self.results {
            voltages.push(checked_simulation_voltage(result, node).map_err(PyErr::from)?);
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
///     >>> result = engine.run_noise(netlist, output_node=2, frequencies=[1e3, 10e3])
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
///     >>> config = MonteCarloConfig(num_runs=1000, seed=42)
///     >>> result = engine.run_monte_carlo(netlist, config)
///     >>> print(f"Runs: {result.num_runs}, Failures: {result.num_failures}")
///     >>> v_out = result.get_variable("V(out)")
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
    /// Get statistics for a specific variable by name
    fn get_variable(&self, name: &str) -> Option<PyVariableStatistics> {
        self.variables.get(name).cloned()
    }

    /// Get all variable names
    #[getter]
    fn variable_names(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }

    /// Get mean for a variable
    fn mean(&self, name: &str) -> Option<f64> {
        self.variables.get(name).map(|v| v.mean)
    }

    /// Get standard deviation for a variable
    fn std_dev(&self, name: &str) -> Option<f64> {
        self.variables.get(name).map(|v| v.std_dev)
    }

    /// Get min/max range as tuple
    fn range(&self, name: &str) -> Option<(f64, f64)> {
        self.variables.get(name).map(|v| (v.min, v.max))
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
/// Represents a pole or zero in the s-domain (Laplace domain).
///
/// Example:
///     >>> for pole in result.poles:
///     ...     print(f"Pole: {pole.real} + {pole.imag}j")
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

    /// Get frequency in Hz (for imaginary pole/zero)
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
/// Example:
///     >>> result = engine.run_pz(netlist, input_node=1, output_node=2)
///     >>> print(f"Stable: {result.is_stable}")
///     >>> print(f"DC Gain: {result.dc_gain:.3f}")
///     >>> for pole in result.poles:
///     ...     print(f"Pole: {pole}")
#[pyclass(name = "PoleZeroResult")]
#[derive(Debug, Clone)]
pub struct PyPoleZeroResult {
    /// System poles (natural frequencies)
    poles: Vec<PyComplexValue>,
    /// System zeros
    zeros: Vec<PyComplexValue>,
    /// DC gain H(0)
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

    /// Check if system is stable (all poles have Re < 0)
    #[getter]
    fn is_stable(&self) -> bool {
        self.poles.iter().all(|p| p.real < 1e-10)
    }

    /// Get the dominant pole (closest to imaginary axis with Re < 0)
    fn dominant_pole(&self) -> Option<PyComplexValue> {
        self.poles
            .iter()
            .filter(|p| p.real < 0.0)
            .min_by(|a, b| a.real.abs().partial_cmp(&b.real.abs()).unwrap())
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

/// Helper enum for node identification (by index or name)
#[derive(FromPyObject)]
pub enum NodeIdentifier {
    #[pyo3(transparent)]
    Index(usize),
    #[pyo3(transparent)]
    Name(String),
}
