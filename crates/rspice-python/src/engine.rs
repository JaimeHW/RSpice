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
//! All simulation calls release the GIL. `run_tran` and `run_dc_sweep`
//! additionally poll Python signals so KeyboardInterrupt cancels them.

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use rspice_core::analysis::Distribution;
use rspice_core::analysis::ac::ac_sweep_frequencies;
use rspice_core::netlist::{AnalysisCommand, DcSweepSpec, FreqVariation};
use rspice_core::{Engine, SimulationConfigOverrides, resolve_simulation_config};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

use crate::abort::run_interruptible;
use crate::config::PySimulationConfig;
use crate::measure;
use crate::netlist::{PyNetlist, describe_analysis};
use crate::results::{
    NodeIdentifier, PyAcResult, PyAnalysisRecord, PyDcSweepResult, PyFourierResult,
    PyMonteCarloResult, PyNoiseResult, PyPoleZeroResult, PyRunReport, PySimulationResult,
    PyTransferFunctionResult, PyTransientResult, is_ground_name,
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

/// RSpice simulation engine
///
/// The Engine class is the main interface for running circuit simulations.
/// It can be configured with custom simulation parameters or use defaults.
///
/// Example:
///     >>> engine = Engine()
///     >>> result = engine.run_dc_op(netlist)
///     >>> print(f"V(out) = {result.voltage('out')} V")
#[pyclass(name = "Engine")]
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
    ) -> PyResult<PyTransientResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, |abort| {
            engine.run_tran_with_abort(&netlist.inner, stop_time, max_step, abort)
        })?;
        Ok(PyTransientResult::new(result))
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
            .allow_threads(|| engine.run_ac(&netlist.inner, &frequencies))
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
            .allow_threads(|| match input_source {
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
            .allow_threads(|| {
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
        let mut noise: Option<Vec<PyNoiseResult>> = None;
        let mut noise_core: Option<Vec<rspice_core::analysis::NoiseResult>> = None;
        let mut tf: Option<PyTransferFunctionResult> = None;
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
                    let results = results
                        .into_iter()
                        .map(|point| (point.sweep_value, point.result))
                        .collect();
                    let result = PyDcSweepResult::new(results);
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
                    let result = self.tran_impl(py, netlist, *stop, resolved)?;
                    tran = Some(Py::new(py, result)?);
                    let mut detail = format!(".tran {step} {stop}");
                    if tstart > 0.0 {
                        detail.push_str(&format!(
                            " (tstart={tstart} requested; full waveform returned)"
                        ));
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
                AnalysisCommand::Four {
                    fundamental,
                    outputs,
                    num_harmonics,
                } => {
                    pending_fourier.push((*fundamental, outputs.clone(), *num_harmonics));
                }
                other => {
                    let kind = match other {
                        AnalysisCommand::Disto { .. } => "disto",
                        AnalysisCommand::PoleZero { .. } => "pz",
                        AnalysisCommand::Sensitivity { .. } => "sens",
                        AnalysisCommand::MonteCarlo(_) => "mc",
                        AnalysisCommand::Step(_) => "step",
                        AnalysisCommand::Temp { .. } => "temp",
                        _ => "unknown",
                    };
                    records.push(PyAnalysisRecord::skipped(
                        kind,
                        format!("{other:?}"),
                        "not executed by Engine.run() yet; use the dedicated run_* method",
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
            noise,
            tf,
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
        if let Ok(tran) = result.downcast::<PyTransientResult>() {
            return Ok(measure::evaluate_tran_measurements(
                &netlist.inner,
                &tran.borrow().inner,
            ));
        }
        if let Ok(sweep) = result.downcast::<PyDcSweepResult>() {
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
        let result = py
            .allow_threads(|| engine.run_dc_op(&netlist.inner))
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PySimulationResult::new(result))
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
        let engine = self.engine_for_netlist(&netlist.inner);
        let results = run_interruptible(py, |abort| {
            engine.run_dc_sweep_with_abort(&netlist.inner, source_name, start, stop, step, abort)
        })?;
        Ok(PyDcSweepResult::new(results))
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
    ///               Defaults to stop_time / 50, matching SPICE convention.
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
    #[pyo3(signature = (netlist, stop_time, max_step=None))]
    pub fn run_tran(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: Option<f64>,
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
        let max_step = max_step.unwrap_or(stop_time / 50.0);
        self.tran_impl(py, netlist, stop_time, max_step)
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
    fn run_pz(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        input_node: NodeIdentifier,
        output_node: NodeIdentifier,
    ) -> PyResult<PyPoleZeroResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let input = self.resolve_node(&engine, &netlist.inner, &input_node, "PZ input")?;
        let output = self.resolve_node(&engine, &netlist.inner, &output_node, "PZ output")?;
        let result = py
            .allow_threads(|| engine.run_pz(&netlist.inner, input, output))
            .map_err(crate::errors::simulation_error_to_pyerr)?;

        Ok(PyPoleZeroResult::from_core(&result))
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
            .allow_threads(|| {
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
        py.allow_threads(|| {
            engine.run_sensitivity(&netlist.inner, output, param_name, param_value, delta)
        })
        .map_err(crate::errors::simulation_error_to_pyerr)
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
            .allow_threads(|| {
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
        for (index, value) in values.iter().enumerate() {
            if !value.is_finite() {
                return Err(PyValueError::new_err(format!(
                    "step value at index {index} must be finite, got {value}"
                )));
            }
        }

        let engine = self.engine_for_netlist(&netlist.inner);
        let results = py
            .allow_threads(|| engine.run_step(&netlist.inner, param_name, &values))
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
