//! Simulation engine Python bindings
//!
//! Provides Python access to the RSpice simulation engine:
//! - DC operating point and sweep analysis
//! - AC frequency analysis (explicit lists or dec/oct/lin sweeps)
//! - Transient time-domain analysis (Ctrl-C interruptible)
//! - Noise, pole-zero, Monte Carlo, sensitivity, parametric step
//! - Transfer function (.TF)
//! - `run()`: execute the netlist's own analysis directives and evaluate
//!   .MEAS statements — the automated-verification entry point
//!
//! All simulation calls release the GIL. Long iterative and swept analyses
//! additionally poll Python signals so KeyboardInterrupt cancels them.

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use rspice_core::analysis::Distribution;
use rspice_core::analysis::PssConfig;
use rspice_core::analysis::ac::ac_sweep_frequencies;
use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
use rspice_core::analysis::advanced::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::advanced::stb::{StbConfig, StbSweepType};
use rspice_core::netlist::{
    AnalysisCommand, DcSweepSpec, FreqVariation, PoleZeroAnalysisType, PoleZeroTransferType,
    StepCommand, StepSweep, StepTarget,
};
use rspice_core::{Engine, SimulationConfigOverrides, resolve_simulation_config};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

use crate::abort::run_interruptible;
use crate::config::PySimulationConfig;
use crate::measure;
use crate::netlist::{PyNetlist, describe_analysis};
use crate::results::{
    NodeIdentifier, PyAcResult, PyAnalysisRecord, PyCompressedTransientResult, PyDcSweepResult,
    PyFourierResult, PyHbResult, PyMonteCarloResult, PyNoiseResult, PyOscillatorNoiseResult,
    PyPacResult, PyPeriodicNoiseResult, PyPoleZeroResult, PyPssResult, PyRunReport,
    PySParameterResult, PySensitivityResult, PySimulationResult, PyStbResult,
    PyTransferFunctionResult, PyTransientCheckpoint, PyTransientResult, is_ground_name,
};

/// Validate that every frequency is finite and non-negative.
fn validate_frequencies(frequencies: &[f64]) -> PyResult<()> {
    if frequencies.is_empty() {
        return Err(PyValueError::new_err("frequencies must not be empty"));
    }
    for &f in frequencies {
        if !f.is_finite() || f < 0.0 {
            return Err(PyValueError::new_err(format!(
                "frequencies must be finite and non-negative, got {f}"
            )));
        }
    }
    Ok(())
}

fn parse_variation(variation: &str) -> PyResult<FreqVariation> {
    match variation.to_ascii_lowercase().as_str() {
        "dec" | "decade" => Ok(FreqVariation::Dec),
        "oct" | "octave" => Ok(FreqVariation::Oct),
        "lin" | "linear" => Ok(FreqVariation::Lin),
        other => Err(PyValueError::new_err(format!(
            "variation must be 'dec', 'oct', or 'lin', got '{other}'"
        ))),
    }
}

/// SPICE default transient max step: explicit value, else
/// min(tstep, window/50), floored at 1e-18 s.
fn resolve_tran_max_step(tstep: f64, tstop: f64, tstart: f64, explicit: Option<f64>) -> f64 {
    explicit
        .filter(|step| step.is_finite() && *step > 0.0)
        .unwrap_or_else(|| {
            let window = tstop - tstart;
            let window = if window.is_finite() && window > 0.0 {
                window
            } else {
                tstop.abs().max(tstep.abs())
            };
            (window / 50.0).min(if tstep > 0.0 { tstep } else { f64::INFINITY })
        })
        .max(1e-18)
}

/// Generate frequency points for an analysis directive's sweep spec.
fn sweep_frequencies(
    variation: FreqVariation,
    points: usize,
    start: f64,
    stop: f64,
) -> PyResult<Vec<f64>> {
    let frequencies = ac_sweep_frequencies(variation, points, start, stop);
    if frequencies.is_empty() {
        return Err(PyValueError::new_err(format!(
            "invalid frequency sweep: {variation:?} {points} points from {start} to {stop} Hz"
        )));
    }
    Ok(frequencies)
}

fn ac_data_frequencies(netlist: &rspice_core::Netlist, table_name: &str) -> PyResult<Vec<f64>> {
    let table = netlist
        .data_tables
        .iter()
        .find(|table| table.name.eq_ignore_ascii_case(table_name))
        .ok_or_else(|| PyValueError::new_err(format!("AC DATA table '{table_name}' not found")))?;
    let frequency_column = table
        .params
        .iter()
        .position(|param| param.eq_ignore_ascii_case("FREQ"))
        .ok_or_else(|| {
            PyValueError::new_err(format!(
                "AC DATA table '{}' must contain a FREQ column",
                table.name
            ))
        })?;
    if table.rows.is_empty() {
        return Err(PyValueError::new_err(format!(
            "AC DATA table '{}' has no rows",
            table.name
        )));
    }
    let mut frequencies = Vec::with_capacity(table.rows.len());
    for (row_index, row) in table.rows.iter().enumerate() {
        if row.len() != table.params.len() {
            return Err(PyValueError::new_err(format!(
                "AC DATA table '{}' row {} has {} values, expected {}",
                table.name,
                row_index + 1,
                row.len(),
                table.params.len()
            )));
        }
        let frequency = row[frequency_column];
        if !frequency.is_finite() || frequency < 0.0 {
            return Err(PyValueError::new_err(format!(
                "AC DATA table '{}' row {} has invalid frequency {frequency}",
                table.name,
                row_index + 1
            )));
        }
        frequencies.push(frequency);
    }
    Ok(frequencies)
}

#[derive(Debug, Clone)]
struct SParameterPortSpec {
    number: usize,
    source_name: String,
    reference_impedance: f64,
}

fn collect_sparameter_ports(netlist: &rspice_core::Netlist) -> PyResult<Vec<SParameterPortSpec>> {
    let mut ports = Vec::new();
    for element in &netlist.elements {
        let rspice_core::netlist::ElementKind::VoltageSource(spec) = &element.kind else {
            continue;
        };
        let Some(port) = spec.rf_port() else {
            continue;
        };
        if element.nodes.len() < 2 {
            return Err(PyValueError::new_err(format!(
                "S-parameter port source '{}' must have positive and negative nodes",
                element.name
            )));
        }
        if !port.z0.is_finite() || port.z0 <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "S-parameter port source '{}' has invalid z0 {}; expected a positive impedance",
                element.name, port.z0
            )));
        }
        ports.push(SParameterPortSpec {
            number: port.portnum,
            source_name: element.name.clone(),
            reference_impedance: port.z0,
        });
    }
    if ports.is_empty() {
        return Err(PyValueError::new_err(
            "S-parameter analysis requires voltage sources annotated with portnum=<n> [z0=<ohms>]",
        ));
    }
    ports.sort_by_key(|port| port.number);
    for (index, port) in ports.iter().enumerate() {
        let expected = index + 1;
        if port.number != expected {
            return Err(PyValueError::new_err(format!(
                "S-parameter port numbers must be dense and unique starting at 1; expected {expected}, found {} on '{}'",
                port.number, port.source_name
            )));
        }
    }
    Ok(ports)
}

fn replace_source_ac(spec: &mut rspice_core::netlist::SourceSpec, magnitude: f64) {
    let current = std::mem::replace(spec, rspice_core::netlist::SourceSpec::Dc(0.0));
    *spec = current.with_ac(magnitude, 0.0);
}

fn set_sparameter_excitations(
    netlist: &mut rspice_core::Netlist,
    ports: &[SParameterPortSpec],
    excited_port: usize,
) -> Result<(), rspice_core::engine::SimulationError> {
    for element in &mut netlist.elements {
        match &mut element.kind {
            rspice_core::netlist::ElementKind::VoltageSource(spec)
            | rspice_core::netlist::ElementKind::CurrentSource(spec) => {
                replace_source_ac(spec, 0.0);
            }
            _ => {}
        }
    }
    for (index, port) in ports.iter().enumerate() {
        let element = netlist
            .elements
            .iter_mut()
            .find(|element| element.name.eq_ignore_ascii_case(&port.source_name))
            .ok_or_else(|| {
                rspice_core::engine::SimulationError::Circuit(format!(
                    "S-parameter port source '{}' disappeared from the netlist",
                    port.source_name
                ))
            })?;
        let rspice_core::netlist::ElementKind::VoltageSource(spec) = &mut element.kind else {
            return Err(rspice_core::engine::SimulationError::Circuit(format!(
                "S-parameter port '{}' is not a voltage source",
                port.source_name
            )));
        };
        replace_source_ac(spec, if index == excited_port { 1.0 } else { 0.0 });
    }
    Ok(())
}

fn invert_complex_matrix(
    matrix: &[Vec<rspice_core::Complex64>],
) -> Option<Vec<Vec<rspice_core::Complex64>>> {
    let size = matrix.len();
    if size == 0 || matrix.iter().any(|row| row.len() != size) {
        return None;
    }
    let zero = rspice_core::Complex64::new(0.0, 0.0);
    let one = rspice_core::Complex64::new(1.0, 0.0);
    let mut augmented = vec![vec![zero; 2 * size]; size];
    for row in 0..size {
        augmented[row][..size].copy_from_slice(&matrix[row]);
        augmented[row][size + row] = one;
    }
    for column in 0..size {
        let pivot = (column..size).max_by(|&lhs, &rhs| {
            augmented[lhs][column]
                .norm()
                .total_cmp(&augmented[rhs][column].norm())
        })?;
        if augmented[pivot][column].norm() <= 1e-24 {
            return None;
        }
        augmented.swap(pivot, column);
        let pivot_value = augmented[column][column];
        for value in &mut augmented[column] {
            *value /= pivot_value;
        }
        let pivot_row = augmented[column].clone();
        for (row, values) in augmented.iter_mut().enumerate() {
            if row == column {
                continue;
            }
            let factor = values[column];
            for index in 0..2 * size {
                values[index] -= factor * pivot_row[index];
            }
        }
    }
    Some(
        augmented
            .into_iter()
            .map(|row| row[size..].to_vec())
            .collect(),
    )
}

fn s_from_y(
    admittance: &[Vec<rspice_core::Complex64>],
    impedances: &[f64],
) -> Result<Vec<Vec<rspice_core::Complex64>>, rspice_core::engine::SimulationError> {
    let size = admittance.len();
    if size == 0 || impedances.len() != size || admittance.iter().any(|row| row.len() != size) {
        return Err(rspice_core::engine::SimulationError::Circuit(
            "malformed S-parameter admittance matrix".to_string(),
        ));
    }
    let zero = rspice_core::Complex64::new(0.0, 0.0);
    let one = rspice_core::Complex64::new(1.0, 0.0);
    let mut plus = vec![vec![zero; size]; size];
    let mut minus = vec![vec![zero; size]; size];
    for row in 0..size {
        for column in 0..size {
            let identity = if row == column { one } else { zero };
            let normalized = impedances[row] * admittance[row][column];
            plus[row][column] = identity + normalized;
            minus[row][column] = identity - normalized;
        }
    }
    let inverse = invert_complex_matrix(&plus).ok_or_else(|| {
        rspice_core::engine::SimulationError::Circuit(
            "S-parameter normalization matrix is singular".to_string(),
        )
    })?;
    let mut scattering = vec![vec![zero; size]; size];
    for row in 0..size {
        for column in 0..size {
            for inner in 0..size {
                scattering[row][column] += minus[row][inner] * inverse[inner][column];
            }
            scattering[row][column] *= (impedances[column] / impedances[row]).sqrt();
        }
    }
    Ok(scattering)
}

/// RSpice simulation engine
///
/// The Engine class is the main interface for running circuit simulations.
/// It can be configured with custom simulation parameters or use defaults.
///
/// Example:
///     >>> engine = Engine()
///     >>> result = engine.run_dc_op(netlist)
///     >>> print(f"V(out) = {result.voltage('out')} V")
#[pyclass(name = "Engine", module = "rspice")]
pub struct PyEngine {
    inner: Engine,
}

impl PyEngine {
    fn engine_for_netlist(&self, netlist: &rspice_core::Netlist) -> Engine {
        let resolved = resolve_simulation_config(
            self.inner.config(),
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        );
        Engine::new(resolved)
    }

    /// Resolve a node identifier (index or name) to a node index, building
    /// the circuit to obtain the node map when a name is given.
    fn resolve_node(
        &self,
        engine: &Engine,
        netlist: &rspice_core::Netlist,
        node: &NodeIdentifier,
        what: &str,
    ) -> PyResult<usize> {
        match node {
            NodeIdentifier::Index(idx) => Ok(*idx),
            NodeIdentifier::Name(name) => {
                let name = name.trim();
                if is_ground_name(name) {
                    return Ok(0);
                }
                if let Ok(idx) = name.parse::<usize>() {
                    return Ok(idx);
                }
                let circuit = engine
                    .build_circuit(netlist)
                    .map_err(crate::errors::simulation_error_to_pyerr)?;
                circuit
                    .node_names_sorted()
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .map(|pos| pos + 1)
                    .ok_or_else(|| {
                        pyo3::exceptions::PyKeyError::new_err(format!(
                            "unknown {what} node '{name}'"
                        ))
                    })
            }
        }
    }

    /// Core transient runner shared by `run_tran` and `run()`.
    fn tran_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: f64,
        start_time: f64,
    ) -> PyResult<PyTransientResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, |abort| {
            engine.run_tran_with_abort(&netlist.inner, stop_time, max_step, abort)
        })?;
        PyTransientResult::new_with_start(result, start_time)
    }

    /// Core AC runner shared by `run_ac`, `run_ac_sweep`, and `run()`.
    fn ac_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
    ) -> PyResult<PyAcResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let results = py
            .detach(|| engine.run_ac(&netlist.inner, &frequencies))
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PyAcResult::new(frequencies, results))
    }

    /// Core noise runner shared by `run_noise` and `run()`.
    #[allow(clippy::too_many_arguments)]
    fn noise_core_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: usize,
        output_neg: Option<usize>,
        input_source: Option<&str>,
        frequencies: &[f64],
        temperature: Option<f64>,
    ) -> PyResult<Vec<rspice_core::analysis::NoiseResult>> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let temp = temperature.unwrap_or(engine.config().temperature);
        if !temp.is_finite() || temp <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "temperature must be a positive number of Kelvin, got {temp}"
            )));
        }

        let results = py
            .detach(|| match input_source {
                Some(source) => engine.run_noise_with_input_source(
                    &netlist.inner,
                    output_node,
                    output_neg,
                    source,
                    frequencies,
                    temp,
                ),
                None => match output_neg {
                    Some(_) => engine.run_noise_ports(
                        &netlist.inner,
                        output_node,
                        output_neg,
                        frequencies,
                        temp,
                    ),
                    None => engine.run_noise(&netlist.inner, output_node, frequencies, temp),
                },
            })
            .map_err(crate::errors::simulation_error_to_pyerr)?;

        Ok(results)
    }

    /// Python noise runner shared by `run_noise` and direct API calls.
    #[allow(clippy::too_many_arguments)]
    fn noise_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: usize,
        output_neg: Option<usize>,
        input_source: Option<&str>,
        frequencies: &[f64],
        temperature: Option<f64>,
    ) -> PyResult<Vec<PyNoiseResult>> {
        let results = self.noise_core_impl(
            py,
            netlist,
            output_node,
            output_neg,
            input_source,
            frequencies,
            temperature,
        )?;
        Ok(results.iter().map(PyNoiseResult::from_core).collect())
    }

    /// Core transfer-function runner.
    fn tf_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: &str,
        reference_node: Option<&str>,
        output_is_current: bool,
        input_source: &str,
    ) -> PyResult<PyTransferFunctionResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = py
            .detach(|| {
                engine.run_transfer_function(
                    &netlist.inner,
                    output_node,
                    reference_node,
                    output_is_current,
                    input_source,
                )
            })
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PyTransferFunctionResult::from_core(&result))
    }

    #[allow(clippy::too_many_arguments)]
    fn stb_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        probe: &str,
        variation: FreqVariation,
        points: usize,
        start_freq: f64,
        stop_freq: f64,
    ) -> PyResult<PyStbResult> {
        let sweep_type = match variation {
            FreqVariation::Lin => StbSweepType::Linear,
            FreqVariation::Dec => StbSweepType::Decade,
            FreqVariation::Oct => StbSweepType::Octave,
        };
        let config = StbConfig::new()
            .with_sweep(start_freq, stop_freq, points)
            .with_sweep_type(sweep_type)
            .with_probe(probe)
            .with_nyquist(true);
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = py
            .detach(|| engine.run_stb(&netlist.inner, config))
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PyStbResult::from_core(&result))
    }

    #[allow(clippy::too_many_arguments)]
    fn pz_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        input_pos: &NodeIdentifier,
        input_neg: Option<&NodeIdentifier>,
        output_pos: &NodeIdentifier,
        output_neg: Option<&NodeIdentifier>,
        input_is_current: bool,
        compute_poles: bool,
        compute_zeros: bool,
    ) -> PyResult<PyPoleZeroResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let input_pos = self.resolve_node(&engine, &netlist.inner, input_pos, "PZ input+")?;
        let input_neg = input_neg
            .map(|node| self.resolve_node(&engine, &netlist.inner, node, "PZ input-"))
            .transpose()?;
        let output_pos = self.resolve_node(&engine, &netlist.inner, output_pos, "PZ output+")?;
        let output_neg = output_neg
            .map(|node| self.resolve_node(&engine, &netlist.inner, node, "PZ output-"))
            .transpose()?;
        let result = py
            .detach(|| {
                engine.run_pz_ports(
                    &netlist.inner,
                    input_pos,
                    input_neg,
                    output_pos,
                    output_neg,
                    input_is_current,
                    compute_poles,
                    compute_zeros,
                )
            })
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PyPoleZeroResult::from_core(&result))
    }

    fn sensitivity_linearized_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output: &NodeIdentifier,
        reference: Option<&NodeIdentifier>,
    ) -> PyResult<PySensitivityResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let output = self.resolve_node(&engine, &netlist.inner, output, "sensitivity output")?;
        let reference = reference
            .map(|node| self.resolve_node(&engine, &netlist.inner, node, "sensitivity reference"))
            .transpose()?;
        let result = py
            .detach(|| engine.run_sensitivity_linearized(&netlist.inner, output, reference))
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PySensitivityResult::from_core(&result))
    }

    #[allow(clippy::needless_range_loop)]
    fn sparameter_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
    ) -> PyResult<PySParameterResult> {
        validate_frequencies(&frequencies)?;
        if frequencies.contains(&0.0) {
            return Err(PyValueError::new_err(
                "S-parameter frequencies must be strictly positive",
            ));
        }
        let ports = collect_sparameter_ports(&netlist.inner)?;
        let port_names = ports
            .iter()
            .map(|port| port.source_name.clone())
            .collect::<Vec<_>>();
        let impedances = ports
            .iter()
            .map(|port| port.reference_impedance)
            .collect::<Vec<_>>();
        let engine = self.engine_for_netlist(&netlist.inner);
        let parameters = py
            .detach(|| {
                let num_ports = ports.len();
                let num_points = frequencies.len();
                let zero = rspice_core::Complex64::new(0.0, 0.0);
                let mut admittances = vec![vec![vec![zero; num_points]; num_ports]; num_ports];
                for excited_port in 0..num_ports {
                    let mut excited = netlist.inner.clone();
                    set_sparameter_excitations(&mut excited, &ports, excited_port)?;
                    let points = engine.run_ac(&excited, &frequencies)?;
                    if points.len() != num_points {
                        return Err(rspice_core::engine::SimulationError::Circuit(format!(
                            "S-parameter AC solve returned {} points for {num_points} requested frequencies",
                            points.len()
                        )));
                    }
                    for (frequency_index, point) in points.iter().enumerate() {
                        for (output_port, port) in ports.iter().enumerate() {
                            let branch_index = point
                                .branch_names
                                .iter()
                                .position(|name| name.eq_ignore_ascii_case(&port.source_name))
                                .ok_or_else(|| {
                                    rspice_core::engine::SimulationError::Circuit(format!(
                                        "S-parameter source '{}' has no branch current at frequency point {frequency_index}",
                                        port.source_name
                                    ))
                                })?;
                            let current = point.currents.get(branch_index).copied().ok_or_else(|| {
                                rspice_core::engine::SimulationError::Circuit(format!(
                                    "S-parameter source '{}' branch-current vector is malformed at frequency point {frequency_index}",
                                    port.source_name
                                ))
                            })?;
                            admittances[output_port][excited_port][frequency_index] = -current;
                        }
                    }
                }

                let mut scattering = vec![vec![vec![zero; num_points]; num_ports]; num_ports];
                for frequency_index in 0..num_points {
                    let y = (0..num_ports)
                        .map(|row| {
                            (0..num_ports)
                                .map(|column| admittances[row][column][frequency_index])
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    let matrix = s_from_y(&y, &impedances)?;
                    for row in 0..num_ports {
                        for column in 0..num_ports {
                            scattering[row][column][frequency_index] = matrix[row][column];
                        }
                    }
                }
                Ok(scattering)
            })
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PySParameterResult::new(
            frequencies,
            port_names,
            impedances,
            parameters,
        ))
    }
}

#[pymethods]
impl PyEngine {
    /// Create a new simulation engine
    ///
    /// Args:
    ///     config: Optional simulation configuration. If not provided,
    ///             default settings are used.
    ///
    /// Example:
    ///     >>> engine = Engine()  # Default config
    ///     >>> engine = Engine(SimulationConfig(tolerance=1e-12))
    #[new]
    #[pyo3(signature = (config=None))]
    pub fn new(config: Option<PySimulationConfig>) -> Self {
        let inner = match config {
            Some(cfg) => Engine::new(cfg.inner),
            None => Engine::default(),
        };
        Self { inner }
    }

    /// Run every analysis directive in the netlist and evaluate .MEAS
    ///
    /// Executes the netlist's own `.op`, `.dc`, `.ac`, `.tran`, `.noise`,
    /// `.tf`, and `.four` directives in order, then evaluates `.MEAS`
    /// statements against the corresponding results. Directives the engine
    /// cannot execute are reported in `records` with `skipped=True` and a
    /// reason — nothing is dropped silently.
    ///
    /// Measurements whose analysis did not run are reported as failed with
    /// an explanatory error, so `assert_passed()` cannot green-wash a CI
    /// pipeline.
    ///
    /// Args:
    ///     netlist: Parsed netlist with analysis directives
    ///
    /// Returns:
    ///     RunReport: results, per-directive records, and measurements
    ///
    /// Example:
    ///     >>> report = engine.run(netlist)
    ///     >>> report.assert_passed()
    ///     >>> print(report.measurement("tpd").value)
    pub fn run(&self, py: Python<'_>, netlist: &PyNetlist) -> PyResult<PyRunReport> {
        let net = &netlist.inner;
        let mut records: Vec<PyAnalysisRecord> = Vec::new();
        let mut op: Option<Py<PySimulationResult>> = None;
        let mut dc: Option<Py<PyDcSweepResult>> = None;
        let mut tran: Option<Py<PyTransientResult>> = None;
        let mut ac: Option<Py<PyAcResult>> = None;
        let mut s_parameters: Option<PySParameterResult> = None;
        let mut noise: Option<Vec<PyNoiseResult>> = None;
        let mut noise_core: Option<Vec<rspice_core::analysis::NoiseResult>> = None;
        let mut tf: Option<PyTransferFunctionResult> = None;
        let mut stb: Option<PyStbResult> = None;
        let mut pz: Option<PyPoleZeroResult> = None;
        let mut monte_carlo: Option<PyMonteCarloResult> = None;
        let mut step_result: Option<PyDcSweepResult> = None;
        let mut temperature: Option<PyDcSweepResult> = None;
        let mut sensitivity: Option<PySensitivityResult> = None;
        let mut fourier: Vec<PyFourierResult> = Vec::new();
        let mut pending_fourier: Vec<(f64, Vec<String>, usize)> = Vec::new();

        for analysis in &net.analyses {
            match analysis {
                AnalysisCommand::Op => {
                    let result = self.run_dc_op(py, netlist)?;
                    op = Some(Py::new(py, result)?);
                    records.push(PyAnalysisRecord::executed("op", ".op".to_string()));
                }
                AnalysisCommand::Dc {
                    source,
                    start,
                    stop,
                    step,
                    mode,
                    sweep2,
                } => {
                    let engine = self.engine_for_netlist(&netlist.inner);
                    let primary = DcSweepSpec {
                        start: *start,
                        stop: *stop,
                        step: *step,
                        mode: mode.clone(),
                    };
                    let results = run_interruptible(py, |abort| {
                        engine.run_dc_sweep2_spec_with_report_and_abort(
                            &netlist.inner,
                            source,
                            &primary,
                            sweep2.as_ref(),
                            abort,
                        )
                    })?;
                    let result = match sweep2 {
                        Some(outer) => PyDcSweepResult::new_nested_with_reports(
                            results,
                            source,
                            &outer.source,
                            outer.spec().points(),
                        )?,
                        None => PyDcSweepResult::new_named_with_reports(results, source),
                    };
                    dc = Some(Py::new(py, result)?);
                    let description = describe_analysis(analysis);
                    records.push(PyAnalysisRecord::executed("dc", description));
                }
                AnalysisCommand::Tran {
                    step,
                    stop,
                    start,
                    max_step,
                    uic: _,
                } => {
                    let tstart = start.unwrap_or(0.0);
                    let resolved = resolve_tran_max_step(*step, *stop, tstart, *max_step);
                    let result = self.tran_impl(py, netlist, *stop, resolved, tstart)?;
                    tran = Some(Py::new(py, result)?);
                    let mut detail = format!(".tran {step} {stop}");
                    if tstart > 0.0 {
                        detail.push_str(&format!(" (tstart={tstart})"));
                    }
                    records.push(PyAnalysisRecord::executed("tran", detail));
                }
                AnalysisCommand::Ac {
                    variation,
                    points,
                    start_freq,
                    stop_freq,
                } => {
                    let frequencies =
                        sweep_frequencies(*variation, *points, *start_freq, *stop_freq)?;
                    let result = self.ac_impl(py, netlist, frequencies)?;
                    ac = Some(Py::new(py, result)?);
                    records.push(PyAnalysisRecord::executed(
                        "ac",
                        format!(
                            ".ac {} {points} {start_freq} {stop_freq}",
                            format!("{variation:?}").to_lowercase()
                        ),
                    ));
                }
                AnalysisCommand::AcData { table_name } => {
                    let frequencies = ac_data_frequencies(net, table_name)?;
                    let result = self.ac_impl(py, netlist, frequencies)?;
                    ac = Some(Py::new(py, result)?);
                    records.push(PyAnalysisRecord::executed(
                        "ac_data",
                        describe_analysis(analysis),
                    ));
                }
                AnalysisCommand::Sp {
                    variation,
                    points,
                    start_freq,
                    stop_freq,
                    do_noise,
                } => {
                    let frequencies =
                        sweep_frequencies(*variation, *points, *start_freq, *stop_freq)?;
                    s_parameters = Some(self.sparameter_impl(py, netlist, frequencies)?);
                    records.push(PyAnalysisRecord::executed(
                        "sp",
                        describe_analysis(analysis),
                    ));
                    if *do_noise {
                        records.push(PyAnalysisRecord::skipped(
                            "sp_noise",
                            describe_analysis(analysis),
                            ".SP donoise requires the complex port-current noise-correlation matrix (Cy) and, for two ports, Rn/NF/NFmin/Sopt; scalar .NOISE results cannot substitute for them",
                        ));
                    }
                }
                AnalysisCommand::Noise {
                    output_node,
                    reference_node,
                    input_source,
                    variation,
                    points,
                    start_freq,
                    stop_freq,
                } => {
                    let engine = self.engine_for_netlist(net);
                    let output = self.resolve_node(
                        &engine,
                        net,
                        &NodeIdentifier::Name(output_node.clone()),
                        "noise output",
                    )?;
                    let output_neg = match reference_node {
                        Some(reference) => Some(self.resolve_node(
                            &engine,
                            net,
                            &NodeIdentifier::Name(reference.clone()),
                            "noise reference",
                        )?),
                        None => None,
                    };
                    let frequencies =
                        sweep_frequencies(*variation, *points, *start_freq, *stop_freq)?;
                    let source = if input_source.is_empty() {
                        None
                    } else {
                        Some(input_source.as_str())
                    };
                    let results = self.noise_core_impl(
                        py,
                        netlist,
                        output,
                        output_neg,
                        source,
                        &frequencies,
                        None,
                    )?;
                    noise = Some(results.iter().map(PyNoiseResult::from_core).collect());
                    noise_core = Some(results);
                    records.push(PyAnalysisRecord::executed(
                        "noise",
                        format!(".noise V({output_node}) {input_source}"),
                    ));
                }
                AnalysisCommand::Tf {
                    output_node,
                    reference_node,
                    output_is_current,
                    input_source,
                } => {
                    let result = self.tf_impl(
                        py,
                        netlist,
                        output_node,
                        reference_node.as_deref(),
                        *output_is_current,
                        input_source,
                    )?;
                    tf = Some(result);
                    records.push(PyAnalysisRecord::executed(
                        "tf",
                        format!(".tf {output_node} {input_source}"),
                    ));
                }
                AnalysisCommand::Stb {
                    variation,
                    points,
                    start_freq,
                    stop_freq,
                    probe,
                } => {
                    let result = self.stb_impl(
                        py,
                        netlist,
                        probe,
                        *variation,
                        *points,
                        *start_freq,
                        *stop_freq,
                    )?;
                    stb = Some(result);
                    records.push(PyAnalysisRecord::executed(
                        "stb",
                        describe_analysis(analysis),
                    ));
                }
                AnalysisCommand::PoleZero {
                    input_pos,
                    input_neg,
                    output_pos,
                    output_neg,
                    transfer_type,
                    analysis_type,
                } => {
                    let (compute_poles, compute_zeros) = match analysis_type {
                        PoleZeroAnalysisType::PoleZero => (true, true),
                        PoleZeroAnalysisType::PolesOnly => (true, false),
                        PoleZeroAnalysisType::ZerosOnly => (false, true),
                    };
                    let result = self.pz_impl(
                        py,
                        netlist,
                        &NodeIdentifier::Name(input_pos.clone()),
                        Some(&NodeIdentifier::Name(input_neg.clone())),
                        &NodeIdentifier::Name(output_pos.clone()),
                        Some(&NodeIdentifier::Name(output_neg.clone())),
                        matches!(transfer_type, PoleZeroTransferType::Current),
                        compute_poles,
                        compute_zeros,
                    )?;
                    pz = Some(result);
                    records.push(PyAnalysisRecord::executed(
                        "pz",
                        describe_analysis(analysis),
                    ));
                }
                AnalysisCommand::MonteCarlo(command) => {
                    let distribution = match command.distribution {
                        rspice_core::netlist::MonteCarloDistribution::Gaussian => "gaussian",
                        rspice_core::netlist::MonteCarloDistribution::Uniform => "uniform",
                        rspice_core::netlist::MonteCarloDistribution::WorstCase => "worst_case",
                    };
                    let params = (!command.params.is_empty()).then(|| command.params.clone());
                    let result = self.run_monte_carlo(
                        py,
                        netlist,
                        command.runs,
                        command.seed,
                        distribution,
                        command.relative_spread,
                        params,
                    )?;
                    monte_carlo = Some(result);
                    records.push(PyAnalysisRecord::executed(
                        "mc",
                        describe_analysis(analysis),
                    ));
                }
                AnalysisCommand::Step(command) => {
                    let values = command.sweep.values();
                    let engine = self.engine_for_netlist(net);
                    let results = py
                        .detach(|| engine.run_step_command(net, command, &values))
                        .map_err(crate::errors::simulation_error_to_pyerr)?;
                    step_result = Some(PyDcSweepResult::new_named(results, &command.name));
                    records.push(PyAnalysisRecord::executed(
                        "step",
                        describe_analysis(analysis),
                    ));
                }
                AnalysisCommand::Temp { temperatures } => {
                    let command = StepCommand {
                        target: StepTarget::Temp,
                        name: "TEMP".to_string(),
                        param_name: None,
                        sweep: StepSweep::List(temperatures.clone()),
                    };
                    let engine = self.engine_for_netlist(net);
                    let results = py
                        .detach(|| engine.run_step_command(net, &command, temperatures))
                        .map_err(crate::errors::simulation_error_to_pyerr)?;
                    temperature = Some(PyDcSweepResult::new_named(results, "TEMP"));
                    records.push(PyAnalysisRecord::executed(
                        "temp",
                        describe_analysis(analysis),
                    ));
                }
                AnalysisCommand::Sensitivity {
                    output_node,
                    reference_node,
                    ac_sweep,
                } => {
                    if ac_sweep.is_some() {
                        records.push(PyAnalysisRecord::skipped(
                            "sens_ac",
                            describe_analysis(analysis),
                            ".SENS AC requires sensitivities for every eligible device and model parameter; run_sensitivity_ac() intentionally covers one explicitly bound parameter and is not a standards-equivalent substitute",
                        ));
                    } else {
                        let output = NodeIdentifier::Name(output_node.clone());
                        let reference = reference_node
                            .as_ref()
                            .map(|name| NodeIdentifier::Name(name.clone()));
                        sensitivity = Some(self.sensitivity_linearized_impl(
                            py,
                            netlist,
                            &output,
                            reference.as_ref(),
                        )?);
                        records.push(PyAnalysisRecord::executed(
                            "sens",
                            describe_analysis(analysis),
                        ));
                    }
                }
                AnalysisCommand::Four {
                    fundamental,
                    outputs,
                    num_harmonics,
                } => {
                    pending_fourier.push((*fundamental, outputs.clone(), *num_harmonics));
                }
                other => {
                    let (kind, reason) = match other {
                        AnalysisCommand::Disto { .. } => (
                            "disto",
                            "the core does not yet provide a nonlinear .DISTO solver; use harmonic-balance analysis for distortion products",
                        ),
                        _ => ("unknown", "this analysis is not executed by Engine.run()"),
                    };
                    records.push(PyAnalysisRecord::skipped(
                        kind,
                        format!("{other:?}"),
                        reason,
                    ));
                }
            }
        }

        // .FOUR needs a transient result; evaluate after the loop so a
        // .four directive may precede its .tran in the deck.
        for (fundamental, outputs, num_harmonics) in pending_fourier {
            match &tran {
                Some(tran_obj) => {
                    let tran_ref = tran_obj.borrow(py);
                    for output in &outputs {
                        let node_name = strip_probe_wrapper(output);
                        match tran_ref.waveform_for(&NodeIdentifier::Name(node_name.to_string())) {
                            Ok(waveform) => {
                                let analysis = rspice_core::analysis::FourierAnalysis::new(
                                    rspice_core::analysis::FourierConfig::new(fundamental)
                                        .with_harmonics(num_harmonics),
                                );
                                let result = analysis.analyze(&tran_ref.inner.time, &waveform);
                                fourier.push(PyFourierResult::from_core(&result));
                                records.push(PyAnalysisRecord::executed(
                                    "four",
                                    format!(".four {fundamental} {output}"),
                                ));
                            }
                            Err(err) => {
                                records.push(PyAnalysisRecord::skipped(
                                    "four",
                                    format!(".four {fundamental} {output}"),
                                    &format!("output not found: {err}"),
                                ));
                            }
                        }
                    }
                }
                None => {
                    records.push(PyAnalysisRecord::skipped(
                        "four",
                        format!(".four {fundamental} {}", outputs.join(" ")),
                        "requires a .tran analysis in the netlist",
                    ));
                }
            }
        }

        // Evaluate measurements; report unevaluated ones as failures so CI
        // cannot silently skip checks.
        let mut measurements = Vec::new();
        match &tran {
            Some(tran_obj) => {
                let tran_ref = tran_obj.borrow(py);
                measurements.extend(measure::evaluate_tran_measurements(net, &tran_ref.inner));
            }
            None => measurements.extend(measure::unevaluated_measurements(
                net,
                "TRAN",
                "requires a .tran analysis in the netlist",
            )),
        }
        match &dc {
            Some(dc_obj) => {
                let dc_ref = dc_obj.borrow(py);
                measurements.extend(measure::evaluate_dc_measurements(net, &dc_ref.results));
            }
            None => measurements.extend(measure::unevaluated_measurements(
                net,
                "DC",
                "requires a .dc analysis in the netlist",
            )),
        }
        match &ac {
            Some(ac_obj) => {
                let ac_ref = ac_obj.borrow(py);
                measurements.extend(measure::evaluate_ac_measurements(net, &ac_ref.results));
            }
            None => measurements.extend(measure::unevaluated_measurements(
                net,
                "AC",
                "requires a .ac analysis in the netlist",
            )),
        }
        match &noise_core {
            Some(noise_results) => {
                measurements.extend(measure::evaluate_noise_measurements(net, noise_results));
            }
            None => measurements.extend(measure::unevaluated_measurements(
                net,
                "NOISE",
                "requires a .noise analysis in the netlist",
            )),
        }

        Ok(PyRunReport {
            op,
            dc,
            tran,
            ac,
            s_parameters,
            noise,
            tf,
            stb,
            pz,
            monte_carlo,
            step: step_result,
            temperature,
            sensitivity,
            fourier,
            records,
            measurements,
        })
    }

    /// Evaluate the netlist's .MEAS statements against an existing result
    ///
    /// Accepts a TransientResult (evaluates TRAN measurements) or a
    /// DcSweepResult (evaluates DC measurements).
    ///
    /// Args:
    ///     netlist: Netlist containing .MEAS statements
    ///     result: TransientResult or DcSweepResult to measure
    ///
    /// Returns:
    ///     list[Measurement]: One entry per applicable .MEAS statement
    ///
    /// Example:
    ///     >>> tran = engine.run_tran(netlist, stop_time=1e-3)
    ///     >>> for m in engine.measure(netlist, tran):
    ///     ...     print(m)
    pub fn measure(
        &self,
        netlist: &PyNetlist,
        result: &Bound<'_, PyAny>,
    ) -> PyResult<Vec<crate::results::PyMeasurement>> {
        if let Ok(tran) = result.cast::<PyTransientResult>() {
            return Ok(measure::evaluate_tran_measurements(
                &netlist.inner,
                &tran.borrow().inner,
            ));
        }
        if let Ok(sweep) = result.cast::<PyDcSweepResult>() {
            return Ok(measure::evaluate_dc_measurements(
                &netlist.inner,
                &sweep.borrow().results,
            ));
        }
        Err(PyTypeError::new_err(
            "measure() expects a TransientResult or DcSweepResult",
        ))
    }

    /// Run DC operating point analysis
    ///
    /// Solves for the DC steady-state of the circuit with all capacitors
    /// open and all inductors shorted.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///
    /// Returns:
    ///     SimulationResult: DC operating point solution
    ///
    /// Raises:
    ///     SimulationError: If the circuit cannot be solved
    ///     ConvergenceError: If Newton-Raphson fails to converge
    ///
    /// Example:
    ///     >>> result = engine.run_dc_op(netlist)
    ///     >>> print(f"V(out) = {result.voltage('out'):.3f} V")
    pub fn run_dc_op(&self, py: Python<'_>, netlist: &PyNetlist) -> PyResult<PySimulationResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, device_op_report) = py
            .detach(|| engine.run_dc_op_with_report(&netlist.inner))
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PySimulationResult::new_with_report(
            result,
            device_op_report,
        ))
    }

    /// Run DC sweep analysis
    ///
    /// Sweeps a voltage source through a range of values, solving the
    /// DC operating point at each step. Interruptible with Ctrl-C.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     source_name: Name of voltage source to sweep (e.g., "V1")
    ///     start: Starting voltage value
    ///     stop: Ending voltage value
    ///     step: Voltage step size (non-zero, sign-consistent with range)
    ///
    /// Returns:
    ///     DcSweepResult: Collection of DC solutions at each sweep point
    ///
    /// Raises:
    ///     ValueError: If start/stop/step are not finite or step is zero
    ///     SimulationError: If the sweep fails
    ///
    /// Example:
    ///     >>> result = engine.run_dc_sweep(netlist, "V1", 0, 5, 0.1)
    ///     >>> for v_in, sol in result:
    ///     ...     print(f"V1={v_in:.1f}V -> V(out)={sol.voltage('out'):.3f}V")
    pub fn run_dc_sweep(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        source_name: &str,
        start: f64,
        stop: f64,
        step: f64,
    ) -> PyResult<PyDcSweepResult> {
        if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
            return Err(PyValueError::new_err(format!(
                "sweep bounds must be finite, got start={start}, stop={stop}, step={step}"
            )));
        }
        if step == 0.0 {
            return Err(PyValueError::new_err("sweep step must be non-zero"));
        }
        if (stop > start && step < 0.0) || (stop < start && step > 0.0) {
            return Err(PyValueError::new_err(format!(
                "sweep step sign must move from start toward stop, got start={start}, stop={stop}, step={step}"
            )));
        }
        let engine = self.engine_for_netlist(&netlist.inner);
        let results = run_interruptible(py, |abort| {
            engine.run_dc_sweep_with_report_and_abort(
                &netlist.inner,
                source_name,
                start,
                stop,
                step,
                abort,
            )
        })?;
        Ok(PyDcSweepResult::new_named_with_reports(
            results,
            source_name,
        ))
    }

    /// Run AC small-signal analysis at explicit frequencies
    ///
    /// Linearizes the circuit around its DC operating point and computes
    /// the frequency response at the specified frequencies.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     frequencies: Frequencies in Hz (finite, non-negative)
    ///
    /// Returns:
    ///     AcResult: Complex voltage/current phasors at each frequency
    ///
    /// Raises:
    ///     ValueError: If the frequency list is empty or contains
    ///                 non-finite/negative values
    ///     SimulationError: If the analysis fails
    ///
    /// Example:
    ///     >>> result = engine.run_ac(netlist, np.logspace(0, 6, 121))
    ///     >>> gain_db = result.voltage_db("out")
    pub fn run_ac(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
    ) -> PyResult<PyAcResult> {
        validate_frequencies(&frequencies)?;
        self.ac_impl(py, netlist, frequencies)
    }

    /// Run AC analysis at frequencies from a named netlist .DATA table.
    fn run_ac_data(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        table_name: &str,
    ) -> PyResult<PyAcResult> {
        let frequencies = ac_data_frequencies(&netlist.inner, table_name)?;
        self.ac_impl(py, netlist, frequencies)
    }

    /// Run AC analysis with a dec/oct/lin sweep specification
    ///
    /// Equivalent to the `.AC DEC|OCT|LIN` netlist directive.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     variation: "dec", "oct", or "lin"
    ///     points: Points per decade/octave, or total points for "lin"
    ///     start_freq: Sweep start frequency in Hz
    ///     stop_freq: Sweep stop frequency in Hz
    ///
    /// Returns:
    ///     AcResult: Complex voltage/current phasors at each frequency
    ///
    /// Example:
    ///     >>> ac = engine.run_ac_sweep(netlist, "dec", 20, 1.0, 1e6)
    pub fn run_ac_sweep(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        variation: &str,
        points: usize,
        start_freq: f64,
        stop_freq: f64,
    ) -> PyResult<PyAcResult> {
        let variation = parse_variation(variation)?;
        let frequencies = sweep_frequencies(variation, points, start_freq, stop_freq)?;
        self.ac_impl(py, netlist, frequencies)
    }

    /// Run N-port S-parameter analysis using annotated voltage-source ports.
    ///
    /// Port sources use ngspice-compatible `portnum=<n>` and optional
    /// `z0=<ohms>` annotations. Port numbering must be dense starting at 1.
    fn run_s_parameters(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
    ) -> PyResult<PySParameterResult> {
        self.sparameter_impl(py, netlist, frequencies)
    }

    /// Run Tian double-injection loop-stability analysis.
    ///
    /// `probe` must name a 0 V voltage source inserted in series with the
    /// feedback loop, with neither terminal grounded.
    #[pyo3(signature = (netlist, probe, variation="dec", points=50, start_freq=1.0, stop_freq=100e6))]
    #[allow(clippy::too_many_arguments)]
    fn run_stb(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        probe: &str,
        variation: &str,
        points: usize,
        start_freq: f64,
        stop_freq: f64,
    ) -> PyResult<PyStbResult> {
        let variation = parse_variation(variation)?;
        // Reuse the shared sweep generator for strict argument validation and
        // exact DEC/OCT/LIN point semantics before starting the analysis.
        sweep_frequencies(variation, points, start_freq, stop_freq)?;
        self.stb_impl(py, netlist, probe, variation, points, start_freq, stop_freq)
    }

    /// Run transient time-domain analysis
    ///
    /// Simulates the circuit from t=0 to stop_time using numerical
    /// integration. Interruptible with Ctrl-C: a KeyboardInterrupt aborts
    /// the simulation promptly instead of after completion.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     stop_time: Simulation end time in seconds (positive, finite)
    ///     max_step: Maximum timestep in seconds (positive, finite).
    ///               Defaults to the output window / 50.
    ///     start_time: Optional output start time. The solver integrates from
    ///                 zero but returned data before this time is discarded.
    ///
    /// Returns:
    ///     TransientResult: Time-domain waveforms for all nodes and branches
    ///
    /// Raises:
    ///     ValueError: If stop_time or max_step is not a positive finite
    ///                 number
    ///     SimulationError: If the simulation fails
    ///     ConvergenceError: If a timestep fails to converge
    ///
    /// Example:
    ///     >>> result = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
    ///     >>> plt.plot(result.time, result.voltage_waveform("out"))
    #[pyo3(signature = (netlist, stop_time, max_step=None, *, start_time=0.0))]
    pub fn run_tran(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: Option<f64>,
        start_time: f64,
    ) -> PyResult<PyTransientResult> {
        if !stop_time.is_finite() || stop_time <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "stop_time must be a positive finite number of seconds, got {stop_time}"
            )));
        }
        if let Some(step) = max_step
            && (!step.is_finite() || step <= 0.0)
        {
            return Err(PyValueError::new_err(format!(
                "max_step must be a positive finite number of seconds, got {step}"
            )));
        }
        if !start_time.is_finite() || start_time < 0.0 || start_time >= stop_time {
            return Err(PyValueError::new_err(format!(
                "start_time must be finite and satisfy 0 <= start_time < stop_time, got {start_time}"
            )));
        }
        let max_step = max_step.unwrap_or((stop_time - start_time) / 50.0);
        self.tran_impl(py, netlist, stop_time, max_step, start_time)
    }

    /// Run transient analysis with error-bounded voltage waveform compression.
    #[pyo3(signature = (netlist, stop_time, max_step=None, *, abs_tol=1e-6, rel_tol=1e-3, max_interval=0.0))]
    #[allow(clippy::too_many_arguments)]
    fn run_tran_compressed(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: Option<f64>,
        abs_tol: f64,
        rel_tol: f64,
        max_interval: f64,
    ) -> PyResult<PyCompressedTransientResult> {
        if !stop_time.is_finite() || stop_time <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "stop_time must be a positive finite number of seconds, got {stop_time}"
            )));
        }
        if let Some(step) = max_step
            && (!step.is_finite() || step <= 0.0)
        {
            return Err(PyValueError::new_err(format!(
                "max_step must be a positive finite number of seconds, got {step}"
            )));
        }
        if !abs_tol.is_finite() || abs_tol < 0.0 {
            return Err(PyValueError::new_err(format!(
                "abs_tol must be finite and non-negative, got {abs_tol}"
            )));
        }
        if !rel_tol.is_finite() || rel_tol < 0.0 {
            return Err(PyValueError::new_err(format!(
                "rel_tol must be finite and non-negative, got {rel_tol}"
            )));
        }
        if !max_interval.is_finite() || max_interval < 0.0 {
            return Err(PyValueError::new_err(format!(
                "max_interval must be finite and non-negative, got {max_interval}"
            )));
        }
        let max_step = max_step.unwrap_or(stop_time / 50.0);
        let compression = rspice_core::analysis::CompressionConfig {
            abs_tol,
            rel_tol,
            enabled: true,
            min_interval: max_interval,
        };
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, |abort| {
            engine.run_tran_compressed_with_abort(
                &netlist.inner,
                stop_time,
                max_step,
                compression,
                abort,
            )
        })?;
        Ok(PyCompressedTransientResult::new(result))
    }

    /// Run a transient and return a resumable, netlist-fingerprinted checkpoint.
    #[pyo3(signature = (netlist, stop_time, max_step=None))]
    fn run_tran_checkpointed(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: Option<f64>,
    ) -> PyResult<(PyTransientResult, PyTransientCheckpoint)> {
        if !stop_time.is_finite() || stop_time <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "stop_time must be a positive finite number of seconds, got {stop_time}"
            )));
        }
        if let Some(step) = max_step
            && (!step.is_finite() || step <= 0.0)
        {
            return Err(PyValueError::new_err(format!(
                "max_step must be a positive finite number of seconds, got {step}"
            )));
        }
        let max_step = max_step.unwrap_or(stop_time / 50.0);
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, checkpoint) = py
            .detach(|| engine.run_tran_checkpointed(&netlist.inner, stop_time, max_step))
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok((
            PyTransientResult::new(result),
            PyTransientCheckpoint::new(checkpoint),
        ))
    }

    /// Continue a transient from a checkpoint to a later absolute stop time.
    #[pyo3(signature = (netlist, checkpoint, stop_time, max_step=None))]
    fn resume_tran(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        checkpoint: &PyTransientCheckpoint,
        stop_time: f64,
        max_step: Option<f64>,
    ) -> PyResult<(PyTransientResult, PyTransientCheckpoint)> {
        if !stop_time.is_finite() || stop_time <= checkpoint.inner.time {
            return Err(PyValueError::new_err(format!(
                "stop_time must be finite and greater than checkpoint time {}, got {stop_time}",
                checkpoint.inner.time
            )));
        }
        if let Some(step) = max_step
            && (!step.is_finite() || step <= 0.0)
        {
            return Err(PyValueError::new_err(format!(
                "max_step must be a positive finite number of seconds, got {step}"
            )));
        }
        let max_step = max_step.unwrap_or((stop_time - checkpoint.inner.time) / 50.0);
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, next_checkpoint) = py
            .detach(|| {
                engine.run_tran_resume(&netlist.inner, &checkpoint.inner, stop_time, max_step)
            })
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok((
            PyTransientResult::new(result),
            PyTransientCheckpoint::new(next_checkpoint),
        ))
    }

    //=========================================================================
    // Advanced Analysis Methods
    //=========================================================================

    /// Run noise analysis
    ///
    /// Computes the noise spectral density at the output node due to all
    /// noise sources in the circuit (thermal, shot, flicker).
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     output_node: Output node index or name
    ///     frequencies: Frequencies in Hz to compute noise at
    ///     temperature: Optional temperature in Kelvin (default: engine
    ///                  configuration)
    ///     input_source: Optional input source name for input-referred noise
    ///     reference_node: Optional negative output node (differential)
    ///
    /// Returns:
    ///     list[NoiseResult]: Noise analysis results at each frequency
    ///
    /// Example:
    ///     >>> results = engine.run_noise(netlist, "out", np.logspace(0, 6, 61))
    ///     >>> for r in results:
    ///     ...     print(f"{r.frequency:.0f}Hz: {r.output_noise_rms*1e9:.2f}nV/√Hz")
    #[pyo3(signature = (netlist, output_node, frequencies, temperature=None, input_source=None, reference_node=None))]
    #[allow(clippy::too_many_arguments)]
    fn run_noise(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: NodeIdentifier,
        frequencies: Vec<f64>,
        temperature: Option<f64>,
        input_source: Option<&str>,
        reference_node: Option<NodeIdentifier>,
    ) -> PyResult<Vec<PyNoiseResult>> {
        validate_frequencies(&frequencies)?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let output = self.resolve_node(&engine, &netlist.inner, &output_node, "noise output")?;
        let output_neg = match &reference_node {
            Some(node) => {
                Some(self.resolve_node(&engine, &netlist.inner, node, "noise reference")?)
            }
            None => None,
        };
        self.noise_impl(
            py,
            netlist,
            output,
            output_neg,
            input_source,
            &frequencies,
            temperature,
        )
    }

    /// Run pole-zero analysis
    ///
    /// Finds the poles and zeros of the circuit's transfer function
    /// between input and output nodes. The input is driven with a unit
    /// current, so `dc_gain` is a transimpedance; pole/zero locations are
    /// input-independent.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     input_node: Input node index or name (non-ground)
    ///     output_node: Output node index or name
    ///
    /// Returns:
    ///     PoleZeroResult: Poles, zeros, and gain information
    ///
    /// Example:
    ///     >>> result = engine.run_pz(netlist, "in", "out")
    ///     >>> print(result.is_stable, result.bandwidth_hz)
    #[pyo3(signature = (netlist, input_node, output_node, *, input_negative=None, output_negative=None, input_type="current", analysis="pz"))]
    #[allow(clippy::too_many_arguments)]
    fn run_pz(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        input_node: NodeIdentifier,
        output_node: NodeIdentifier,
        input_negative: Option<NodeIdentifier>,
        output_negative: Option<NodeIdentifier>,
        input_type: &str,
        analysis: &str,
    ) -> PyResult<PyPoleZeroResult> {
        let input_is_current = match input_type.to_ascii_lowercase().as_str() {
            "current" | "cur" | "i" => true,
            "voltage" | "vol" | "v" => false,
            other => {
                return Err(PyValueError::new_err(format!(
                    "input_type must be 'current' or 'voltage', got '{other}'"
                )));
            }
        };
        let (compute_poles, compute_zeros) = match analysis.to_ascii_lowercase().as_str() {
            "pz" | "pole_zero" | "poles_zeros" => (true, true),
            "pol" | "poles" => (true, false),
            "zer" | "zeros" => (false, true),
            other => {
                return Err(PyValueError::new_err(format!(
                    "analysis must be 'pz', 'poles', or 'zeros', got '{other}'"
                )));
            }
        };
        self.pz_impl(
            py,
            netlist,
            &input_node,
            input_negative.as_ref(),
            &output_node,
            output_negative.as_ref(),
            input_is_current,
            compute_poles,
            compute_zeros,
        )
    }

    /// Run shooting periodic steady-state analysis.
    #[pyo3(signature = (netlist, fundamental_frequency=None, *, harmonics=9, tstab=0.0, tstab_periods=None, max_iterations=100, tolerance=1e-6, points_per_period=256, autonomous=false, period_guess=None))]
    #[allow(clippy::too_many_arguments)]
    fn run_pss(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        fundamental_frequency: Option<f64>,
        harmonics: usize,
        tstab: f64,
        tstab_periods: Option<usize>,
        max_iterations: usize,
        tolerance: f64,
        points_per_period: usize,
        autonomous: bool,
        period_guess: Option<f64>,
    ) -> PyResult<PyPssResult> {
        if harmonics == 0 {
            return Err(PyValueError::new_err("harmonics must be at least 1"));
        }
        if let Some(frequency) = fundamental_frequency
            && (!frequency.is_finite() || frequency <= 0.0)
        {
            return Err(PyValueError::new_err(format!(
                "fundamental_frequency must be positive and finite, got {frequency}"
            )));
        }
        if !autonomous && fundamental_frequency.is_none() {
            return Err(PyValueError::new_err(
                "fundamental_frequency is required for driven PSS",
            ));
        }
        if let Some(period) = period_guess
            && (!period.is_finite() || period <= 0.0)
        {
            return Err(PyValueError::new_err(format!(
                "period_guess must be positive and finite, got {period}"
            )));
        }

        let mut config = if autonomous {
            PssConfig::autonomous()
        } else if let Some(frequency) = fundamental_frequency {
            PssConfig::new(frequency)
        } else {
            return Err(PyValueError::new_err(
                "fundamental_frequency is required for driven PSS",
            ));
        };
        if autonomous {
            if let Some(period) = period_guess {
                config.period_guess = period;
                config.fundamental_freq = 1.0 / period;
            } else if let Some(frequency) = fundamental_frequency {
                config.period_guess = 1.0 / frequency;
                config.fundamental_freq = frequency;
            }
        }
        config.num_harmonics = harmonics;
        config.tstab = tstab;
        if let Some(periods) = tstab_periods {
            config.tstab_periods = periods;
        }
        config.max_iterations = max_iterations;
        config.tolerance = tolerance;
        config.points_per_period = points_per_period;
        config.validate().map_err(|message| {
            PyValueError::new_err(format!("invalid PSS configuration: {message}"))
        })?;

        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, |abort| {
            engine.run_pss_with_abort(&netlist.inner, config, abort)
        })?;
        Ok(PyPssResult::from_core(&result))
    }

    /// Run single-tone harmonic-balance analysis.
    #[pyo3(signature = (netlist, fundamental_frequency, *, harmonics=9, tolerance=1e-6, max_iterations=100, damping=1.0, oversample=2, use_krylov=false, source_stepping=false))]
    #[allow(clippy::too_many_arguments)]
    fn run_hb(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        fundamental_frequency: f64,
        harmonics: usize,
        tolerance: f64,
        max_iterations: usize,
        damping: f64,
        oversample: usize,
        use_krylov: bool,
        source_stepping: bool,
    ) -> PyResult<PyHbResult> {
        if !fundamental_frequency.is_finite() || fundamental_frequency <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "fundamental_frequency must be positive and finite, got {fundamental_frequency}"
            )));
        }
        if harmonics == 0 {
            return Err(PyValueError::new_err("harmonics must be at least 1"));
        }
        if !tolerance.is_finite() || tolerance <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "tolerance must be positive and finite, got {tolerance}"
            )));
        }
        if max_iterations == 0 {
            return Err(PyValueError::new_err("max_iterations must be at least 1"));
        }
        if !damping.is_finite() || !(0.1..=1.0).contains(&damping) {
            return Err(PyValueError::new_err(format!(
                "damping must be finite and in [0.1, 1.0], got {damping}"
            )));
        }
        if oversample == 0 {
            return Err(PyValueError::new_err("oversample must be at least 1"));
        }

        let mut config = HbConfig::new(fundamental_frequency);
        config.num_harmonics = harmonics;
        config.tolerance = tolerance;
        config.max_iterations = max_iterations;
        config.damping = damping;
        config.oversample_factor = oversample;
        config.use_krylov = use_krylov;
        config.source_stepping = source_stepping;
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, |abort| {
            engine.run_hb_with_abort(&netlist.inner, config, abort)
        })?;
        Ok(PyHbResult::from_core(&result))
    }

    /// Run periodic small-signal AC analysis around an HB operating point.
    #[pyo3(signature = (netlist, fundamental_frequency, start_frequency, stop_frequency, points, input_source, output_node, *, variation="dec", sideband_min=None, sideband_max=5, reference_node=None))]
    #[allow(clippy::too_many_arguments)]
    fn run_pac(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        fundamental_frequency: f64,
        start_frequency: f64,
        stop_frequency: f64,
        points: usize,
        input_source: &str,
        output_node: &str,
        variation: &str,
        sideband_min: Option<i32>,
        sideband_max: i32,
        reference_node: Option<&str>,
    ) -> PyResult<PyPacResult> {
        if !fundamental_frequency.is_finite() || fundamental_frequency <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "fundamental_frequency must be positive and finite, got {fundamental_frequency}"
            )));
        }
        if input_source.trim().is_empty() {
            return Err(PyValueError::new_err("input_source must not be empty"));
        }
        if output_node.trim().is_empty() {
            return Err(PyValueError::new_err("output_node must not be empty"));
        }
        let sweep_type = match variation.to_ascii_lowercase().as_str() {
            "dec" | "decade" => PacSweepType::Decade,
            "oct" | "octave" => PacSweepType::Octave,
            "lin" | "linear" => PacSweepType::Linear,
            other => {
                return Err(PyValueError::new_err(format!(
                    "variation must be 'dec', 'oct', or 'lin', got '{other}'"
                )));
            }
        };
        let sideband_min = sideband_min.unwrap_or(-5);
        let mut config = PacConfig::new()
            .with_fundamental(fundamental_frequency)
            .with_sweep(start_frequency, stop_frequency, points)
            .with_sweep_type(sweep_type)
            .with_sidebands(sideband_min, sideband_max)
            .with_input_source(input_source)
            .with_output_node(output_node);
        if let Some(reference) = reference_node {
            if reference.trim().is_empty() {
                return Err(PyValueError::new_err("reference_node must not be empty"));
            }
            config = config.with_output_ref(reference);
        }
        config.validate().map_err(|message| {
            PyValueError::new_err(format!("invalid PAC configuration: {message}"))
        })?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, |abort| {
            engine.run_pac_with_abort(&netlist.inner, config, abort)
        })?;
        Ok(PyPacResult::from_core(&result))
    }

    /// Run driven periodic-noise analysis with sideband folding.
    #[pyo3(signature = (netlist, fundamental_frequency, offsets, output_node, *, reference_node=None, input_source=None, max_sideband=6))]
    #[allow(clippy::too_many_arguments)]
    fn run_pnoise(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        fundamental_frequency: f64,
        offsets: Vec<f64>,
        output_node: &str,
        reference_node: Option<&str>,
        input_source: Option<&str>,
        max_sideband: i32,
    ) -> PyResult<PyPeriodicNoiseResult> {
        if !fundamental_frequency.is_finite() || fundamental_frequency <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "fundamental_frequency must be positive and finite, got {fundamental_frequency}"
            )));
        }
        validate_frequencies(&offsets)?;
        if offsets.contains(&0.0) {
            return Err(PyValueError::new_err(
                "periodic-noise offsets must be strictly positive",
            ));
        }
        if output_node.trim().is_empty() {
            return Err(PyValueError::new_err("output_node must not be empty"));
        }
        if max_sideband < 1 {
            return Err(PyValueError::new_err("max_sideband must be at least 1"));
        }
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, |abort| {
            engine.run_pnoise_with_abort(
                &netlist.inner,
                fundamental_frequency,
                &offsets,
                output_node,
                reference_node,
                input_source,
                max_sideband,
                abort,
            )
        })?;
        Ok(PyPeriodicNoiseResult::from_core(&result))
    }

    /// Run autonomous-oscillator phase noise using PSS and PPV projection.
    #[pyo3(signature = (netlist, offsets, *, period_guess, tstab_periods=20, max_iterations=100, tolerance=1e-6, points_per_period=256))]
    #[allow(clippy::too_many_arguments)]
    fn run_oscillator_noise(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        offsets: Vec<f64>,
        period_guess: f64,
        tstab_periods: usize,
        max_iterations: usize,
        tolerance: f64,
        points_per_period: usize,
    ) -> PyResult<PyOscillatorNoiseResult> {
        validate_frequencies(&offsets)?;
        if offsets.contains(&0.0) {
            return Err(PyValueError::new_err(
                "oscillator-noise offsets must be strictly positive",
            ));
        }
        if !period_guess.is_finite() || period_guess <= 0.0 {
            return Err(PyValueError::new_err(format!(
                "period_guess must be positive and finite, got {period_guess}"
            )));
        }
        let mut config = PssConfig::autonomous();
        config.period_guess = period_guess;
        config.fundamental_freq = 1.0 / period_guess;
        config.tstab_periods = tstab_periods;
        config.max_iterations = max_iterations;
        config.tolerance = tolerance;
        config.points_per_period = points_per_period;
        config.validate().map_err(|message| {
            PyValueError::new_err(format!("invalid oscillator PSS configuration: {message}"))
        })?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, |abort| {
            engine.run_pnoise_oscillator_with_abort(&netlist.inner, config, &offsets, abort)
        })?;
        Ok(PyOscillatorNoiseResult::from_core(&result))
    }

    /// Run Monte Carlo analysis
    ///
    /// Runs repeated simulations with random variations applied to every
    /// `.param` value referenced by the netlist (or to `params` when given),
    /// and reports statistics for all node voltages.
    ///
    /// Args:
    ///     netlist: Parsed netlist (parameters via .param and {expr} bindings)
    ///     num_runs: Number of Monte Carlo iterations (>= 1)
    ///     seed: Random seed for reproducibility; random when omitted
    ///     distribution: "gaussian", "uniform", or "worst_case"
    ///     spread: Relative sigma (gaussian) or tolerance half-width
    ///             (uniform/worst_case); default 0.01 = 1%
    ///     params: Restrict variation to these parameter names
    ///
    /// Returns:
    ///     MonteCarloResult: Statistical results for all output variables
    ///
    /// Raises:
    ///     ValueError: For invalid num_runs, distribution, or spread
    ///     SimulationError: If parameters are unbound or runs fail to build
    ///
    /// Example:
    ///     >>> mc = engine.run_monte_carlo(netlist, 1000, seed=42,
    ///     ...                             distribution="uniform", spread=0.05)
    ///     >>> stats = mc.get_variable("V(OUT)")
    #[pyo3(signature = (netlist, num_runs, seed=None, distribution="gaussian", spread=0.01, params=None))]
    #[allow(clippy::too_many_arguments)]
    fn run_monte_carlo(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        num_runs: usize,
        seed: Option<u64>,
        distribution: &str,
        spread: f64,
        params: Option<Vec<String>>,
    ) -> PyResult<PyMonteCarloResult> {
        if num_runs == 0 {
            return Err(PyValueError::new_err("num_runs must be at least 1"));
        }
        if !spread.is_finite() || spread < 0.0 {
            return Err(PyValueError::new_err(format!(
                "spread must be finite and non-negative, got {spread}"
            )));
        }
        let dist = match distribution.to_ascii_lowercase().as_str() {
            "gaussian" | "normal" => Distribution::Gaussian { sigma: spread },
            "uniform" => Distribution::Uniform { tolerance: spread },
            "worst_case" | "worstcase" | "worst-case" => {
                Distribution::WorstCase { tolerance: spread }
            }
            other => {
                return Err(PyValueError::new_err(format!(
                    "distribution must be 'gaussian', 'uniform', or 'worst_case', got '{other}'"
                )));
            }
        };
        let seed = seed.unwrap_or_else(|| RandomState::new().build_hasher().finish());

        let engine = self.engine_for_netlist(&netlist.inner);
        let result = py
            .detach(|| {
                engine.run_monte_carlo_with_options(
                    &netlist.inner,
                    num_runs,
                    seed,
                    dist,
                    params.as_deref(),
                )
            })
            .map_err(crate::errors::simulation_error_to_pyerr)?;

        Ok(PyMonteCarloResult::from_core(&result))
    }

    /// Run DC sensitivity analysis
    ///
    /// Computes dV(output)/d(param) by central finite differences. The
    /// parameter must be a `.param` name referenced by the netlist via
    /// `{...}` expressions — element names are not parameters.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     output_node: Output node index or name
    ///     param_name: Name of the .param to vary (e.g. "rval")
    ///     param_value: Nominal parameter value
    ///     delta: Optional perturbation size (default: 1% of value)
    ///
    /// Returns:
    ///     float: dV/dParam sensitivity value
    ///
    /// Raises:
    ///     ValueError: For non-finite param_value or non-positive delta
    ///     SimulationError: If the parameter is not bound to any netlist
    ///                      expression, or a perturbed solve fails
    ///
    /// Example:
    ///     >>> # netlist: .param rval=1k / R1 in out {rval} / ...
    ///     >>> sens = engine.run_sensitivity(netlist, "out", "rval", 1000.0)
    #[pyo3(signature = (netlist, output_node, param_name, param_value, delta=None))]
    fn run_sensitivity(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: NodeIdentifier,
        param_name: &str,
        param_value: f64,
        delta: Option<f64>,
    ) -> PyResult<f64> {
        if !param_value.is_finite() {
            return Err(PyValueError::new_err(format!(
                "param_value must be finite, got {param_value}"
            )));
        }
        if let Some(d) = delta
            && (!d.is_finite() || d <= 0.0)
        {
            return Err(PyValueError::new_err(format!(
                "delta must be a positive finite number, got {d}"
            )));
        }
        let engine = self.engine_for_netlist(&netlist.inner);
        let output = self.resolve_node(&engine, &netlist.inner, &output_node, "output")?;
        py.detach(|| engine.run_sensitivity(&netlist.inner, output, param_name, param_value, delta))
            .map_err(crate::errors::simulation_error_to_pyerr)
    }

    /// Run single-solve adjoint DC sensitivity for all eligible linear
    /// elements and independent sources.
    #[pyo3(signature = (netlist, output_node, reference_node=None))]
    fn run_sensitivity_linearized(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: NodeIdentifier,
        reference_node: Option<NodeIdentifier>,
    ) -> PyResult<PySensitivityResult> {
        self.sensitivity_linearized_impl(py, netlist, &output_node, reference_node.as_ref())
    }

    /// Run AC sensitivity analysis
    ///
    /// Computes d|V(output)|/d(param) at each frequency by central finite
    /// differences. Same parameter-binding rules as `run_sensitivity`.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     output_node: Output node index or name
    ///     param_name: Name of the .param to vary
    ///     param_value: Nominal parameter value
    ///     frequencies: Frequencies in Hz
    ///     delta: Optional perturbation size (default: 1% of value)
    ///
    /// Returns:
    ///     numpy.ndarray: Sensitivity at each frequency
    ///
    /// Raises:
    ///     ValueError: For non-finite param_value or non-positive delta
    #[pyo3(signature = (netlist, output_node, param_name, param_value, frequencies, delta=None))]
    #[allow(clippy::too_many_arguments)]
    fn run_sensitivity_ac<'py>(
        &self,
        py: Python<'py>,
        netlist: &PyNetlist,
        output_node: NodeIdentifier,
        param_name: &str,
        param_value: f64,
        frequencies: Vec<f64>,
        delta: Option<f64>,
    ) -> PyResult<Bound<'py, numpy::PyArray1<f64>>> {
        use numpy::ToPyArray;
        if !param_value.is_finite() {
            return Err(PyValueError::new_err(format!(
                "param_value must be finite, got {param_value}"
            )));
        }
        if let Some(d) = delta
            && (!d.is_finite() || d <= 0.0)
        {
            return Err(PyValueError::new_err(format!(
                "delta must be a positive finite number, got {d}"
            )));
        }
        validate_frequencies(&frequencies)?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let output = self.resolve_node(&engine, &netlist.inner, &output_node, "output")?;
        let values = py
            .detach(|| {
                engine.run_sensitivity_ac(
                    &netlist.inner,
                    output,
                    param_name,
                    param_value,
                    &frequencies,
                    delta,
                )
            })
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(values.to_pyarray(py))
    }

    /// Run parametric step analysis
    ///
    /// Executes a DC operating point at each parameter value. The parameter
    /// must be a `.param` name referenced by the netlist via `{...}`
    /// expressions — stepping an element name does not vary anything and
    /// raises SimulationError.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     param_name: Name of the .param to sweep (e.g. "rval")
    ///     values: Parameter values to simulate
    ///
    /// Returns:
    ///     list[tuple[float, SimulationResult]]: Results per parameter value
    ///
    /// Example:
    ///     >>> # netlist: .param rval=1k / R1 in out {rval} / ...
    ///     >>> for val, sol in engine.run_step(netlist, "rval", [1e3, 2e3, 5e3]):
    ///     ...     print(f"rval={val:.0f}: V(out)={sol.voltage('out'):.3f}V")
    fn run_step(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        param_name: &str,
        values: Vec<f64>,
    ) -> PyResult<Vec<(f64, PySimulationResult)>> {
        if values.is_empty() {
            return Err(PyValueError::new_err("values must not be empty"));
        }
        for (index, value) in values.iter().enumerate() {
            if !value.is_finite() {
                return Err(PyValueError::new_err(format!(
                    "step value at index {index} must be finite, got {value}"
                )));
            }
        }

        let engine = self.engine_for_netlist(&netlist.inner);
        let results = py
            .detach(|| engine.run_step(&netlist.inner, param_name, &values))
            .map_err(crate::errors::simulation_error_to_pyerr)?;

        Ok(results
            .into_iter()
            .map(|(val, sim_result)| (val, PySimulationResult::new(sim_result)))
            .collect())
    }

    /// Run small-signal transfer function analysis (.TF)
    ///
    /// Computes DC gain, input impedance, and output impedance from an
    /// input source to an output node.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     output_node: Output node name (e.g. "out")
    ///     input_source: Input source name (e.g. "V1")
    ///     output_is_current: Measure current instead of voltage
    ///     reference_node: Optional reference node for differential output
    ///
    /// Returns:
    ///     TransferFunctionResult: gain, input_impedance, output_impedance
    ///
    /// Example:
    ///     >>> tf = engine.run_transfer_function(netlist, "out", "V1")
    ///     >>> print(f"Av={tf.gain:.3f}, Zin={tf.input_impedance:.0f}Ω")
    #[pyo3(signature = (netlist, output_node, input_source, output_is_current=false, reference_node=None))]
    fn run_transfer_function(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: &str,
        input_source: &str,
        output_is_current: bool,
        reference_node: Option<&str>,
    ) -> PyResult<PyTransferFunctionResult> {
        self.tf_impl(
            py,
            netlist,
            output_node,
            reference_node,
            output_is_current,
            input_source,
        )
    }

    /// Get a copy of the current simulation configuration
    ///
    /// Note: this is a copy — mutating it does not reconfigure the engine.
    /// Build a new Engine with the modified config instead.
    #[getter]
    pub fn config(&self) -> PySimulationConfig {
        PySimulationConfig {
            inner: self.inner.config().clone(),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Engine(tolerance={:.0e}, max_iterations={})",
            self.inner.config().tolerance,
            self.inner.config().max_iterations
        )
    }
}

/// Strip `V(...)` / `I(...)` probe wrappers from a .four output spec.
fn strip_probe_wrapper(spec: &str) -> &str {
    let trimmed = spec.trim();
    let lower = trimmed.to_ascii_lowercase();
    if (lower.starts_with("v(") || lower.starts_with("i(")) && trimmed.ends_with(')') {
        &trimmed[2..trimmed.len() - 1]
    } else {
        trimmed
    }
}
