//! Simulation engine Python bindings
//!
//! Provides Python access to the RSpice simulation engine:
//! - DC operating point and sweep analysis
//! - AC frequency analysis (explicit lists, dec/oct/lin sweeps, `.DATA` tables)
//! - Third-order Volterra distortion (`.DISTO`)
//! - Transient time-domain analysis (Ctrl-C interruptible), plus
//!   error-bounded waveform compression and fingerprinted checkpoint/resume
//! - Noise, pole-zero, transfer function, and Tian loop stability (`.STB`)
//! - N-port S-parameters, optionally with `.SP donoise` port-noise correlation
//! - Periodic/RF: PSS, single- and multi-tone HB, PAC, driven periodic noise,
//!   and autonomous oscillator phase noise
//! - Monte Carlo, DC and AC sensitivity, parametric step
//! - `run()`: execute the netlist's own analysis directives and evaluate
//!   .MEAS statements — the automated-verification entry point
//!
//! All simulation calls release the GIL. Long iterative and swept analyses
//! additionally poll Python signals so KeyboardInterrupt cancels them.

use numpy::{PyArray1, ToPyArray};
use pyo3::prelude::*;
use rspice_core::analysis::AcSensitivityOutput;
use rspice_core::analysis::PssConfig;
use rspice_core::analysis::harmonic_balance::{HbConfig, HbTone};
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::stb::{StbConfig, StbSweepType};
use rspice_core::netlist::{
    AnalysisCommand, DcSecondSweep, DcSweepMode, DcSweepSpec, FreqVariation,
    MonteCarloDistribution, PoleZeroAnalysisType, PoleZeroTransferType,
};
use rspice_core::{
    AbortSignal, Engine, SimulationConfig, SimulationConfigOverrides, resolve_simulation_config,
};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

use rspice_core::analysis::s_param;

use crate::abort::{ActiveRuns, run_interruptible};
use crate::config::{PyIntegrationMethod, PySimulationConfig};
use crate::measure;
use crate::netlist::{PyNetlist, describe_analysis};
use crate::results::{
    NodeIdentifier, PyAcResult, PyAcSensitivityResult, PyAnalysisRecord,
    PyCompressedTransientResult, PyDcSweepResult, PyDistortionResult, PyEnvelopeResult,
    PyFourierResult, PyHbResult, PyMeasurement, PyMonteCarloResult, PyNoiseResult,
    PyOscillatorNoiseResult, PyPacResult, PyPeriodicNoiseResult, PyPoleZeroResult, PyPssResult,
    PyRunCoordinate, PyRunReport, PySParameterResult, PySensitivityResult, PySimulationResult,
    PyStbResult, PyTransferFunctionResult, PyTransientCheckpoint, PyTransientResult,
    is_ground_name,
};

mod directives;
mod internals;
mod rf_config;
mod support;
mod types;

pub(crate) use types::{
    PyDcSweep, PyHbEnvelopeState, PyHealthReport, PyPssContinuationState, PyPssOperatingPoint,
};

use rf_config::*;
use support::*;

/// RSpice simulation engine
///
/// The Engine class is the main interface for running circuit simulations.
/// It can be configured with custom simulation parameters or use defaults.
///
/// Example:
///     >>> engine = Engine()
///     >>> result = engine.run_dc_op(netlist)
///     >>> print(f"V(out) = {result.voltage('out')} V")
#[pyclass(name = "Engine", module = "rspice", frozen)]
pub struct PyEngine {
    inner: Engine,
    active_runs: ActiveRuns,
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
    pub fn new(config: Option<PySimulationConfig>) -> PyResult<Self> {
        let config = config.map_or_else(SimulationConfig::default, |cfg| cfg.inner);
        let inner = Engine::try_new(config)
            .map_err(|error| crate::errors::value_error(error.to_string()))?;
        Ok(Self {
            inner,
            active_runs: ActiveRuns::default(),
        })
    }

    /// Cancel every simulation currently running through this Engine.
    ///
    /// This method is thread-safe and is intended for GUI cancel buttons,
    /// service-request cancellation, and application-managed deadlines. It
    /// returns the number of active calls that were signalled. Each cancelled
    /// call raises `CancelledError` in its calling thread.
    fn cancel(&self) -> usize {
        self.active_runs.cancel_all()
    }

    /// Whether at least one simulation is currently running on this Engine.
    #[getter]
    fn is_running(&self) -> bool {
        self.active_runs.count() != 0
    }

    /// Number of simulations currently running on this Engine.
    #[getter]
    fn active_run_count(&self) -> usize {
        self.active_runs.count()
    }

    /// Progress in [0.0, 1.0] for one active call, when available.
    ///
    /// Returns None when the Engine is idle, multiple calls are active, or
    /// the current analysis has not reported measurable progress.
    #[getter]
    fn progress(&self) -> Option<f64> {
        self.active_runs.progress()
    }

    /// Exercise the configured parser-to-solver path with a deterministic,
    /// bounded in-memory circuit.
    pub fn health_check(&self, py: Python<'_>) -> PyResult<PyHealthReport> {
        run_interruptible(py, &self.active_runs, |abort| {
            self.inner.health_check_with_abort(abort)
        })
        .map(Into::into)
    }

    /// Run every analysis directive in the netlist and evaluate .MEAS
    ///
    /// Executes every directive the netlist carries, in deck order: `.op`,
    /// `.dc`, `.tran`, `.ac` and `.ac data`, `.disto`, `.hb`, `.pss`, `.pac`,
    /// `.pnoise`, `.envelope`, `.sp` (with `donoise`), `.noise` and
    /// `.noise data`, `.tf`, `.stb`, `.pz`, `.mc`, `.step`, `.temp`, DC and AC
    /// `.sens`, and `.four`. `.four` is evaluated after the loop so it may
    /// precede its `.tran` in the deck. `.pac`, `.pnoise`, and `.envelope`
    /// linearize around the `.pss`/`.hb` instance the deck plan bound them to.
    ///
    /// `.MEAS` statements are then evaluated against the TRAN, DC, AC, and
    /// NOISE results. Every directive contributes at least one entry to
    /// `records`; anything that could not be executed is recorded with
    /// `skipped=True` and the reason instead of being dropped silently.
    ///
    /// Measurements whose analysis did not run are reported as failed with
    /// an explanatory error, so `assert_passed()` cannot green-wash a CI
    /// pipeline.
    ///
    /// Args:
    ///     netlist: Parsed netlist with analysis directives
    ///     continue_on_error: When True (the default), a directive that fails
    ///         is recorded with `skipped=True` and its error text, and the
    ///         remaining directives still run — so one unconverged sweep
    ///         still yields the results and .MEAS outcomes of everything
    ///         else, and `assert_passed()` still fails. Set False to abort
    ///         the whole run and raise the first failure instead.
    ///
    /// Returns:
    ///     RunReport: results, per-directive records, and measurements
    ///
    /// Example:
    ///     >>> report = engine.run(netlist)
    ///     >>> report.assert_passed()
    ///     >>> print(report.measurement("tpd").value)
    #[pyo3(signature = (netlist, *, continue_on_error=true))]
    pub fn run(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        continue_on_error: bool,
    ) -> PyResult<PyRunReport> {
        directives::run(self, py, netlist, continue_on_error)
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
        Err(crate::errors::type_error(
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
    pub fn run_dc_op(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
    ) -> PyResult<Py<PySimulationResult>> {
        directives::run_one_card(self, py, netlist, AnalysisCommand::Op)?
            .op
            .into_single(".op")
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
    ) -> PyResult<Py<PyDcSweepResult>> {
        require_linear_bounds(Some(start), Some(stop), Some(step))?;
        directives::run_one_card(
            self,
            py,
            netlist,
            AnalysisCommand::Dc {
                source: source_name.to_string(),
                start,
                stop,
                step,
                mode: DcSweepMode::Linear,
                sweep2: None,
            },
        )?
        .dc
        .into_single(".dc")
    }

    /// Run a DC sweep described by one or two `DcSweep` axes
    ///
    /// The general form of `.DC`: linear, explicit-list, and logarithmic
    /// decade/octave sweeps, optionally nested inside a second swept source.
    /// `run_dc_sweep` remains the shorthand for the linear single-source case.
    ///
    /// A nested result is flattened in the same order `.DC` produces: the
    /// inner axis varies fastest. Use `shape`, `is_nested`, and
    /// `secondary_value_at` on the result to address the grid.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     sweep: Inner (fastest-varying) sweep axis
    ///     sweep2: Optional outer sweep axis
    ///
    /// Returns:
    ///     DcSweepResult: One solution per grid point
    ///
    /// Example:
    ///     >>> vgs = rspice.DcSweep("VG", start=0, stop=1.8, step=0.05)
    ///     >>> vds = rspice.DcSweep("VD", values=[0.5, 1.0, 1.8])
    ///     >>> curves = engine.run_dc_sweep_spec(netlist, vgs, sweep2=vds)
    ///     >>> curves.shape
    ///     (3, 37)
    #[pyo3(signature = (netlist, sweep, *, sweep2=None))]
    fn run_dc_sweep_spec(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        sweep: &PyDcSweep,
        sweep2: Option<&PyDcSweep>,
    ) -> PyResult<Py<PyDcSweepResult>> {
        if let Some(outer) = sweep2
            && outer.source.eq_ignore_ascii_case(&sweep.source)
        {
            return Err(crate::errors::value_error(format!(
                "a nested sweep must vary two different sources, but both axes sweep '{}'",
                sweep.source
            )));
        }

        directives::run_one_card(
            self,
            py,
            netlist,
            AnalysisCommand::Dc {
                source: sweep.source.clone(),
                start: sweep.spec.start,
                stop: sweep.spec.stop,
                step: sweep.spec.step,
                mode: sweep.spec.mode.clone(),
                sweep2: sweep2.map(|outer| DcSecondSweep {
                    source: outer.source.clone(),
                    start: outer.spec.start,
                    stop: outer.spec.stop,
                    step: outer.spec.step,
                    mode: outer.spec.mode.clone(),
                }),
            },
        )?
        .dc
        .into_single(".dc")
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
    ) -> PyResult<Py<PyAcResult>> {
        directives::run_one_card(
            self,
            py,
            netlist,
            AnalysisCommand::AcData {
                table_name: table_name.to_string(),
            },
        )?
        .ac
        .into_single(".ac data")
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
    ) -> PyResult<Py<PyAcResult>> {
        directives::run_one_card(
            self,
            py,
            netlist,
            AnalysisCommand::Ac {
                variation: parse_variation(variation)?,
                points,
                start_freq,
                stop_freq,
            },
        )?
        .ac
        .into_single(".ac")
    }

    /// Run third-order Volterra distortion analysis at explicit F1 frequencies.
    ///
    /// Sources opt into the analysis with `DISTOF1 magnitude [phase]` and,
    /// in two-tone mode, `DISTOF2 magnitude [phase]`. With `f2_over_f1`, F2
    /// is fixed at `f2_over_f1 * frequencies[0]` while F1 is swept.
    #[pyo3(signature = (netlist, frequencies, f2_over_f1=None))]
    fn run_distortion(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
        f2_over_f1: Option<f64>,
    ) -> PyResult<PyDistortionResult> {
        self.distortion_impl(py, netlist, frequencies, f2_over_f1)
    }

    /// Run harmonic or two-tone `.DISTO` on a DEC/OCT/LIN F1 sweep.
    #[pyo3(signature = (netlist, variation, points, start_freq, stop_freq, f2_over_f1=None))]
    #[allow(clippy::too_many_arguments)]
    fn run_distortion_sweep(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        variation: &str,
        points: usize,
        start_freq: f64,
        stop_freq: f64,
        f2_over_f1: Option<f64>,
    ) -> PyResult<Py<PyDistortionResult>> {
        directives::run_one_card(
            self,
            py,
            netlist,
            AnalysisCommand::Disto {
                variation: parse_variation(variation)?,
                points,
                start_freq,
                stop_freq,
                f2_over_f1,
            },
        )?
        .distortion
        .into_single(".disto")
    }

    /// Run N-port S-parameter analysis using annotated voltage-source ports.
    ///
    /// Port sources use ngspice-compatible `portnum=<n>` and optional
    /// `z0=<ohms>` annotations. Port numbering must be dense starting at 1.
    #[pyo3(signature = (netlist, frequencies, do_noise=false))]
    fn run_s_parameters(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
        do_noise: bool,
    ) -> PyResult<PySParameterResult> {
        self.sparameter_impl(py, netlist, frequencies, do_noise)
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
        directives::run_one_card(
            self,
            py,
            netlist,
            AnalysisCommand::Stb {
                variation: parse_variation(variation)?,
                points,
                start_freq,
                stop_freq,
                probe: probe.to_string(),
            },
        )?
        .stb
        .into_single(".stb")
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
    ) -> PyResult<Py<PyTransientResult>> {
        if !stop_time.is_finite() || stop_time <= 0.0 {
            return Err(crate::errors::value_error(format!(
                "stop_time must be a positive finite number of seconds, got {stop_time}"
            )));
        }
        if let Some(step) = max_step
            && (!step.is_finite() || step <= 0.0)
        {
            return Err(crate::errors::value_error(format!(
                "max_step must be a positive finite number of seconds, got {step}"
            )));
        }
        if !start_time.is_finite() || start_time < 0.0 || start_time >= stop_time {
            return Err(crate::errors::value_error(format!(
                "start_time must be finite and satisfy 0 <= start_time < stop_time, got {start_time}"
            )));
        }
        // A `.TRAN` card's TSTEP is a print cadence that only bounds TMAX; the
        // default ceiling this method documents is a window fraction, so it is
        // authored as the print step and core's own resolution reproduces it.
        directives::run_one_uncarded_transient(
            self,
            py,
            netlist,
            AnalysisCommand::Tran {
                step: (stop_time - start_time) / 50.0,
                stop: stop_time,
                start: Some(start_time),
                max_step,
                uic: false,
            },
        )?
        .tran
        .into_single(".tran")
    }

    /// Run transient analysis with error-bounded voltage waveform compression.
    ///
    /// The solver runs unchanged; the finished waveform is then decimated by
    /// recursively splitting each time span at its worst-fitting sample until
    /// linear interpolation between the retained samples reproduces every
    /// dropped one within `abs_tol + rel_tol * |v|`. The result therefore
    /// carries a bounded reconstruction error rather than a fixed stride.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     stop_time: Simulation end time in seconds (positive, finite)
    ///     max_step: Maximum solver timestep in seconds. Defaults to
    ///               stop_time / 50.
    ///     abs_tol: Absolute interpolation error budget in each signal's
    ///              native unit
    ///     rel_tol: Relative interpolation error budget
    ///     max_interval: Upper bound in seconds on the gap between two
    ///                   retained samples. 0.0 (the default) imposes no
    ///                   bound, leaving decimation purely error-driven. A
    ///                   positive value forces a retained sample roughly
    ///                   every `max_interval` seconds, so it *lowers* the
    ///                   compression ratio in exchange for a guaranteed
    ///                   sampling cadence on long, slow-moving runs.
    ///
    /// Returns:
    ///     CompressedTransientResult: Decimated analog waveforms with an
    ///                                applied-policy and worst-error report
    ///
    /// Raises:
    ///     ValueError: If stop_time or max_step is not positive and finite,
    ///                 or a tolerance/interval is negative or non-finite
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
        let max_step = solver_window(stop_time, max_step)?;
        for (name, value) in [
            ("abs_tol", abs_tol),
            ("rel_tol", rel_tol),
            ("max_interval", max_interval),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(crate::errors::value_error(format!(
                    "{name} must be finite and non-negative, got {value}"
                )));
            }
        }
        let compression = rspice_core::engine::CompressionConfig {
            abs_tol,
            rel_tol,
            enabled: true,
            min_interval: max_interval,
        };
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| {
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
    ///
    /// Returns `(TransientResult, TransientCheckpoint)`. `max_step` defaults
    /// to stop_time / 50. Persist the checkpoint with `checkpoint.save(path)`
    /// and continue it later with `resume_tran`; the fingerprint makes
    /// resuming against a different netlist fail rather than produce
    /// silently wrong state.
    #[pyo3(signature = (netlist, stop_time, max_step=None))]
    fn run_tran_checkpointed(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: Option<f64>,
    ) -> PyResult<(PyTransientResult, PyTransientCheckpoint)> {
        let max_step = solver_window(stop_time, max_step)?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, checkpoint) = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_tran_checkpointed_with_abort(&netlist.inner, stop_time, max_step, abort)
        })?;
        Ok((
            PyTransientResult::new(result),
            PyTransientCheckpoint::new(checkpoint),
        ))
    }

    /// Continue a transient from a checkpoint to a later absolute stop time.
    ///
    /// `stop_time` is absolute, not a duration, and must exceed
    /// `checkpoint.time`. `max_step` defaults to the remaining window / 50.
    /// Returns the next `(TransientResult, TransientCheckpoint)` pair, so a
    /// long run can be advanced in segments.
    ///
    /// Raises:
    ///     ValueError: If stop_time is not finite and greater than the
    ///                 checkpoint time, or max_step is not positive
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
            return Err(crate::errors::value_error(format!(
                "stop_time must be finite and greater than checkpoint time {}, got {stop_time}",
                checkpoint.inner.time
            )));
        }
        if let Some(step) = max_step
            && (!step.is_finite() || step <= 0.0)
        {
            return Err(crate::errors::value_error(format!(
                "max_step must be a positive finite number of seconds, got {step}"
            )));
        }
        let max_step = max_step.unwrap_or((stop_time - checkpoint.inner.time) / 50.0);
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, next_checkpoint) = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_tran_resume_with_abort(
                &netlist.inner,
                &checkpoint.inner,
                stop_time,
                max_step,
                abort,
            )
        })?;
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
        let output =
            self.resolve_node(py, &engine, &netlist.inner, &output_node, "noise output")?;
        let output_neg = match &reference_node {
            Some(node) => {
                Some(self.resolve_node(py, &engine, &netlist.inner, node, "noise reference")?)
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
    /// between input and output nodes. Pole and zero locations are
    /// input-independent; only the gain scaling depends on `input_type`.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     input_node: Input node index or name (non-ground)
    ///     output_node: Output node index or name
    ///     input_negative: Optional negative input node (differential drive)
    ///     output_negative: Optional negative output node (differential probe)
    ///     input_type: "current" (default) drives a unit current, making
    ///                 `dc_gain` a transimpedance in V/A; "voltage" drives a
    ///                 unit voltage, making `dc_gain` a voltage ratio
    ///     analysis: "pz" for both (default), "poles", or "zeros"
    ///
    /// Returns:
    ///     PoleZeroResult: Poles, zeros, and gain information
    ///
    /// Raises:
    ///     ValueError: For an unknown input_type or analysis
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
        let card = pole_zero_card(
            &input_node,
            input_negative.as_ref(),
            &output_node,
            output_negative.as_ref(),
            input_type,
            analysis,
        )?;
        directives::run_one_card(self, py, netlist, card)?
            .pz
            .into_single(".pz")
    }

    /// Run shooting periodic steady-state analysis.
    #[pyo3(signature = (netlist, fundamental_frequency=None, *, harmonics=9, tstab=0.0, tstab_periods=None, max_iterations=100, tolerance=1e-6, abstol=1e-12, damping=1.0, max_period_change=0.1, points_per_period=256, integration_method=None, autonomous=false, period_guess=None, verbose=false))]
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
        abstol: f64,
        damping: f64,
        max_period_change: f64,
        points_per_period: usize,
        integration_method: Option<PyIntegrationMethod>,
        autonomous: bool,
        period_guess: Option<f64>,
        verbose: bool,
    ) -> PyResult<PyPssResult> {
        let card = pss_card(
            fundamental_frequency,
            harmonics,
            tstab,
            tstab_periods,
            max_iterations,
            tolerance,
            abstol,
            damping,
            max_period_change,
            points_per_period,
            integration_method,
            autonomous,
            period_guess,
            verbose,
        )?;
        directives::run_one_card(self, py, netlist, AnalysisCommand::Pss(Box::new(card)))?
            .pss
            .into_single(".pss")
    }

    /// Solve a periodic operating point once for reuse by PAC and PNoise
    ///
    /// Both small-signal periodic analyses linearize around a PSS solution.
    /// Solving it once here and passing the result as `pss=` replaces one
    /// full shooting solve per call, which dominates an RF sweep.
    ///
    /// Takes the same arguments as `run_pss`.
    ///
    /// Returns:
    ///     PssOperatingPoint: Reusable converged periodic state
    ///
    /// Example:
    ///     >>> op = engine.run_pss_operating_point(netlist, 1e9)
    ///     >>> pac = engine.run_pac(netlist, 1e9, 1e3, 1e8, 20, "VRF", "out", pss=op)
    ///     >>> pn = engine.run_pnoise(netlist, 1e9, [1e3, 1e4], "out", pss=op)
    #[pyo3(signature = (netlist, fundamental_frequency=None, *, harmonics=9, tstab=0.0, tstab_periods=None, max_iterations=100, tolerance=1e-6, abstol=1e-12, damping=1.0, max_period_change=0.1, points_per_period=256, integration_method=None, autonomous=false, period_guess=None, verbose=false))]
    #[allow(clippy::too_many_arguments)]
    fn run_pss_operating_point(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        fundamental_frequency: Option<f64>,
        harmonics: usize,
        tstab: f64,
        tstab_periods: Option<usize>,
        max_iterations: usize,
        tolerance: f64,
        abstol: f64,
        damping: f64,
        max_period_change: f64,
        points_per_period: usize,
        integration_method: Option<PyIntegrationMethod>,
        autonomous: bool,
        period_guess: Option<f64>,
        verbose: bool,
    ) -> PyResult<PyPssOperatingPoint> {
        let card = pss_card(
            fundamental_frequency,
            harmonics,
            tstab,
            tstab_periods,
            max_iterations,
            tolerance,
            abstol,
            damping,
            max_period_change,
            points_per_period,
            integration_method,
            autonomous,
            period_guess,
            verbose,
        )?;
        let config = PssConfig::from(&card);
        let engine = self.engine_for_netlist(&netlist.inner);
        let inner = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_pss_operating_point_with_abort(&netlist.inner, config, abort)
        })?;
        Ok(PyPssOperatingPoint { inner })
    }

    /// Run PSS and also return a state a transient can continue from
    ///
    /// Same arguments as `run_pss`, plus `frozen_sources`: independent
    /// sources held at their exact time-zero values during the solve, which
    /// is how a modulated carrier is separated from its envelope.
    ///
    /// Returns:
    ///     tuple[PssResult, PssContinuationState]
    ///
    /// Example:
    ///     >>> pss, state = engine.run_pss_continuation(netlist, 1e9)
    ///     >>> tran, checkpoint = engine.run_tran_from_pss(netlist, state, 1e-6)
    #[pyo3(signature = (netlist, fundamental_frequency=None, *, frozen_sources=None, harmonics=9, tstab=0.0, tstab_periods=None, max_iterations=100, tolerance=1e-6, abstol=1e-12, damping=1.0, max_period_change=0.1, points_per_period=256, integration_method=None, autonomous=false, period_guess=None, verbose=false))]
    #[allow(clippy::too_many_arguments)]
    fn run_pss_continuation(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        fundamental_frequency: Option<f64>,
        frozen_sources: Option<Vec<String>>,
        harmonics: usize,
        tstab: f64,
        tstab_periods: Option<usize>,
        max_iterations: usize,
        tolerance: f64,
        abstol: f64,
        damping: f64,
        max_period_change: f64,
        points_per_period: usize,
        integration_method: Option<PyIntegrationMethod>,
        autonomous: bool,
        period_guess: Option<f64>,
        verbose: bool,
    ) -> PyResult<(PyPssResult, PyPssContinuationState)> {
        let card = pss_card(
            fundamental_frequency,
            harmonics,
            tstab,
            tstab_periods,
            max_iterations,
            tolerance,
            abstol,
            damping,
            max_period_change,
            points_per_period,
            integration_method,
            autonomous,
            period_guess,
            verbose,
        )?;
        let config = PssConfig::from(&card);
        let frozen = frozen_sources.unwrap_or_default();
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, state) = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_pss_with_frozen_source_continuation_state_abort(
                &netlist.inner,
                config,
                &frozen,
                abort,
            )
        })?;
        Ok((
            PyPssResult::from_core(&result, harmonics),
            PyPssContinuationState { inner: state },
        ))
    }

    /// Continue a transient from a converged PSS orbit
    ///
    /// Starts at the phase-equivalent state rather than from a cold start, so
    /// the settling interval does not have to be integrated again.
    ///
    /// Args:
    ///     netlist: The same netlist the PSS state came from
    ///     state: State from `run_pss_continuation`
    ///     duration: Length of the continued run in seconds
    ///     max_step: Maximum timestep; defaults to duration / 50
    ///
    /// Returns:
    ///     tuple[TransientResult, TransientCheckpoint]
    ///
    /// Raises:
    ///     ValueError: If duration or max_step is not positive and finite
    ///     SimulationError: If the state does not match this netlist
    #[pyo3(signature = (netlist, state, duration, max_step=None))]
    fn run_tran_from_pss(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        state: &PyPssContinuationState,
        duration: f64,
        max_step: Option<f64>,
    ) -> PyResult<(PyTransientResult, PyTransientCheckpoint)> {
        let (duration, max_step) = continuation_window(duration, max_step)?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, checkpoint) = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_tran_from_pss_state_with_abort(
                &netlist.inner,
                &state.inner,
                duration,
                max_step,
                abort,
            )
        })?;
        Ok((
            PyTransientResult::new(result),
            PyTransientCheckpoint::new(checkpoint),
        ))
    }

    /// Run single-tone harmonic-balance analysis.
    #[pyo3(signature = (netlist, fundamental_frequency, *, harmonics=9, tolerance=1e-6, max_iterations=100, damping=1.0, oversample=2, use_krylov=false, source_stepping=false, abstol=1e-12, min_damping=0.1, collocation_points=None, max_mixing_order=5, gmres_restart=30, use_exact_jacobian=true, source_name=None, verbose=false))]
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
        abstol: f64,
        min_damping: f64,
        collocation_points: Option<usize>,
        max_mixing_order: usize,
        gmres_restart: usize,
        use_exact_jacobian: bool,
        source_name: Option<&str>,
        verbose: bool,
    ) -> PyResult<PyHbResult> {
        if !fundamental_frequency.is_finite() || fundamental_frequency <= 0.0 {
            return Err(crate::errors::value_error(format!(
                "fundamental_frequency must be positive and finite, got {fundamental_frequency}"
            )));
        }
        if harmonics == 0 {
            return Err(crate::errors::value_error("harmonics must be at least 1"));
        }
        let source_names = source_name.map(|name| vec![name.to_string()]);
        let config = hb_config(
            &[fundamental_frequency],
            &[harmonics],
            source_names.as_deref(),
            HbNumerics {
                tolerance,
                abstol,
                max_iterations,
                damping,
                min_damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                use_exact_jacobian,
                verbose,
            },
        )?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_hb_with_abort(&netlist.inner, config, abort)
        })?;
        Ok(PyHbResult::from_core(&result))
    }

    /// Run one- or multi-tone harmonic balance on a shared spectral basis.
    ///
    /// `harmonics` may contain one order broadcast to every tone or one order
    /// per frequency. `source_names`, when provided, maps each tone to one
    /// independent source; an empty name broadcasts that tone.
    #[pyo3(signature = (netlist, frequencies, *, harmonics=None, source_names=None, tolerance=1e-6, abstol=1e-12, max_iterations=100, damping=1.0, min_damping=0.1, oversample=2, collocation_points=None, max_mixing_order=5, use_krylov=false, gmres_restart=30, source_stepping=false, use_exact_jacobian=true, verbose=false))]
    #[allow(clippy::too_many_arguments)]
    fn run_hb_multitone(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
        harmonics: Option<Vec<usize>>,
        source_names: Option<Vec<String>>,
        tolerance: f64,
        abstol: f64,
        max_iterations: usize,
        damping: f64,
        min_damping: f64,
        oversample: usize,
        collocation_points: Option<usize>,
        max_mixing_order: usize,
        use_krylov: bool,
        gmres_restart: usize,
        source_stepping: bool,
        use_exact_jacobian: bool,
        verbose: bool,
    ) -> PyResult<PyHbResult> {
        let orders = resolve_hb_harmonic_orders(
            frequencies.len(),
            harmonics.as_deref(),
            "run_hb_multitone",
        )?;
        let config = hb_config(
            &frequencies,
            &orders,
            source_names.as_deref(),
            HbNumerics {
                tolerance,
                abstol,
                max_iterations,
                damping,
                min_damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                use_exact_jacobian,
                verbose,
            },
        )?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_hb_with_abort(&netlist.inner, config, abort)
        })?;
        Ok(PyHbResult::from_core(&result))
    }

    /// Run harmonic balance and return an envelope continuation state
    ///
    /// Solves the carrier with the named sources frozen at their time-zero
    /// values, then hands back a state a transient can continue from. That
    /// pairing is how an envelope-following run separates a slowly modulated
    /// envelope from the fast carrier the HB solve already resolved.
    ///
    /// Takes the same arguments as `run_hb`, plus:
    ///     frozen_sources: Independent sources held at their time-zero values
    ///
    /// Returns:
    ///     tuple[HbResult, HbEnvelopeState]
    ///
    /// Example:
    ///     >>> hb, state = engine.run_hb_envelope(netlist, 1e9,
    ///     ...                                    frozen_sources=["VMOD"])
    ///     >>> tran, checkpoint = engine.run_tran_from_hb_envelope(
    ///     ...     netlist, state, duration=1e-6)
    #[pyo3(signature = (netlist, fundamental_frequency, *, frozen_sources=None, harmonics=9, tolerance=1e-6, max_iterations=100, damping=1.0, oversample=2, use_krylov=false, source_stepping=false, abstol=1e-12, min_damping=0.1, collocation_points=None, max_mixing_order=5, gmres_restart=30, use_exact_jacobian=true, source_name=None, verbose=false))]
    #[allow(clippy::too_many_arguments)]
    fn run_hb_envelope(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        fundamental_frequency: f64,
        frozen_sources: Option<Vec<String>>,
        harmonics: usize,
        tolerance: f64,
        max_iterations: usize,
        damping: f64,
        oversample: usize,
        use_krylov: bool,
        source_stepping: bool,
        abstol: f64,
        min_damping: f64,
        collocation_points: Option<usize>,
        max_mixing_order: usize,
        gmres_restart: usize,
        use_exact_jacobian: bool,
        source_name: Option<&str>,
        verbose: bool,
    ) -> PyResult<(PyHbResult, PyHbEnvelopeState)> {
        if !fundamental_frequency.is_finite() || fundamental_frequency <= 0.0 {
            return Err(crate::errors::value_error(format!(
                "fundamental_frequency must be positive and finite, got {fundamental_frequency}"
            )));
        }
        if harmonics == 0 {
            return Err(crate::errors::value_error("harmonics must be at least 1"));
        }
        let source_names = source_name.map(|name| vec![name.to_string()]);
        let config = hb_config(
            &[fundamental_frequency],
            &[harmonics],
            source_names.as_deref(),
            HbNumerics {
                tolerance,
                abstol,
                max_iterations,
                damping,
                min_damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                use_exact_jacobian,
                verbose,
            },
        )?;

        let frozen = frozen_sources.unwrap_or_default();
        // The continuation state is only valid against the configuration and
        // frozen-source list that produced it, so both are retained on the
        // returned object rather than left for the caller to re-supply.
        let retained_config = config.clone();
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, state) = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_hb_envelope_continuation_state_with_abort(
                &netlist.inner,
                config,
                &frozen,
                abort,
            )
        })?;
        Ok((
            PyHbResult::from_core(&result),
            PyHbEnvelopeState {
                inner: state,
                config: retained_config,
                frozen_sources: frozen,
            },
        ))
    }

    /// Continue a transient from a harmonic-balance envelope state
    ///
    /// Args:
    ///     netlist: The same netlist the envelope state came from
    ///     state: State from `run_hb_envelope`
    ///     duration: Length of the continued run in seconds
    ///     max_step: Maximum timestep; defaults to duration / 50
    ///
    /// Returns:
    ///     tuple[TransientResult, TransientCheckpoint]
    ///
    /// Raises:
    ///     ValueError: If duration or max_step is not positive and finite
    ///     SimulationError: If the state does not match this netlist
    #[pyo3(signature = (netlist, state, duration, max_step=None))]
    fn run_tran_from_hb_envelope(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        state: &PyHbEnvelopeState,
        duration: f64,
        max_step: Option<f64>,
    ) -> PyResult<(PyTransientResult, PyTransientCheckpoint)> {
        let (duration, max_step) = continuation_window(duration, max_step)?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, checkpoint) = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_tran_from_hb_envelope_state_with_abort(
                &netlist.inner,
                &state.config,
                &state.frozen_sources,
                &state.inner,
                duration,
                max_step,
                abort,
            )
        })?;
        Ok((
            PyTransientResult::new(result),
            PyTransientCheckpoint::new(checkpoint),
        ))
    }

    /// Run periodic small-signal AC analysis around an HB operating point.
    #[pyo3(signature = (netlist, fundamental_frequency, start_frequency, stop_frequency, points, input_source, output_node, *, variation="dec", sideband_min=None, sideband_max=5, reference_node=None, reltol=1e-3, abstol=1e-12, pss=None))]
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
        reltol: f64,
        abstol: f64,
        pss: Option<&PyPssOperatingPoint>,
    ) -> PyResult<PyPacResult> {
        let config = pac_config(PacRequest {
            fundamental_frequency,
            start_frequency,
            stop_frequency,
            points,
            input_source,
            output_node,
            variation,
            sideband_min,
            sideband_max,
            reference_node,
            reltol,
            abstol,
        })?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| match pss {
            // Reusing a converged orbit replaces the PSS solve this analysis
            // would otherwise repeat for every call.
            Some(operating_point) => engine.run_pac_from_pss_with_abort(
                &netlist.inner,
                config,
                &operating_point.inner,
                abort,
            ),
            None => engine.run_pac_with_abort(&netlist.inner, config, abort),
        })?;
        Ok(PyPacResult::from_core(&result))
    }

    /// Run driven periodic-noise analysis with sideband folding.
    #[pyo3(signature = (netlist, fundamental_frequency, offsets, output_node, *, reference_node=None, input_source=None, max_sideband=6, pss=None))]
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
        pss: Option<&PyPssOperatingPoint>,
    ) -> PyResult<PyPeriodicNoiseResult> {
        if !fundamental_frequency.is_finite() || fundamental_frequency <= 0.0 {
            return Err(crate::errors::value_error(format!(
                "fundamental_frequency must be positive and finite, got {fundamental_frequency}"
            )));
        }
        validate_frequencies(&offsets)?;
        if offsets.contains(&0.0) {
            return Err(crate::errors::value_error(
                "periodic-noise offsets must be strictly positive",
            ));
        }
        if output_node.trim().is_empty() {
            return Err(crate::errors::value_error("output_node must not be empty"));
        }
        if max_sideband < 1 {
            return Err(crate::errors::value_error(
                "max_sideband must be at least 1",
            ));
        }
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| match pss {
            Some(operating_point) => engine.run_pnoise_from_pss_with_abort(
                &netlist.inner,
                &offsets,
                output_node,
                reference_node,
                input_source,
                max_sideband,
                &operating_point.inner,
                abort,
            ),
            None => engine.run_pnoise_with_abort(
                &netlist.inner,
                fundamental_frequency,
                &offsets,
                output_node,
                reference_node,
                input_source,
                max_sideband,
                abort,
            ),
        })?;
        Ok(PyPeriodicNoiseResult::from_core(&result))
    }

    /// Run autonomous-oscillator phase noise using PSS and PPV projection.
    ///
    /// Projects the complete device-noise model along the periodic orbit,
    /// including thermal, shot, flicker, tabulated, Verilog-A, and correlated
    /// BSIM4 sources. Offset frequencies must be finite and strictly positive.
    #[pyo3(signature = (netlist, offsets, *, period_guess, harmonics=9, tstab=0.0, tstab_periods=20, max_iterations=100, tolerance=1e-6, abstol=1e-12, damping=1.0, max_period_change=0.1, points_per_period=256, integration_method=None, verbose=false))]
    #[allow(clippy::too_many_arguments)]
    fn run_oscillator_noise(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        offsets: Vec<f64>,
        period_guess: f64,
        harmonics: usize,
        tstab: f64,
        tstab_periods: usize,
        max_iterations: usize,
        tolerance: f64,
        abstol: f64,
        damping: f64,
        max_period_change: f64,
        points_per_period: usize,
        integration_method: Option<PyIntegrationMethod>,
        verbose: bool,
    ) -> PyResult<PyOscillatorNoiseResult> {
        validate_frequencies(&offsets)?;
        if offsets.contains(&0.0) {
            return Err(crate::errors::value_error(
                "oscillator-noise offsets must be strictly positive",
            ));
        }
        // An oscillator's carrier is an autonomous shooting solve, so its
        // configuration is built by the same card constructor every other PSS
        // entry point uses rather than assembled a second time here.
        let card = pss_card(
            None,
            harmonics,
            tstab,
            Some(tstab_periods),
            max_iterations,
            tolerance,
            abstol,
            damping,
            max_period_change,
            points_per_period,
            integration_method,
            true,
            Some(period_guess),
            verbose,
        )?;
        let config = PssConfig::from(&card);
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| {
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
        let card = monte_carlo_card(num_runs, seed, distribution, spread, params)?;
        directives::run_one_card(self, py, netlist, card)?
            .monte_carlo
            .into_single(".mc")
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
            return Err(crate::errors::value_error(format!(
                "param_value must be finite, got {param_value}"
            )));
        }
        if let Some(d) = delta
            && (!d.is_finite() || d <= 0.0)
        {
            return Err(crate::errors::value_error(format!(
                "delta must be a positive finite number, got {d}"
            )));
        }
        let engine = self.engine_for_netlist(&netlist.inner);
        let output = self.resolve_node(py, &engine, &netlist.inner, &output_node, "output")?;
        run_interruptible(py, &self.active_runs, |abort| {
            engine.run_sensitivity_with_abort(
                &netlist.inner,
                output,
                param_name,
                param_value,
                delta,
                abort,
            )
        })
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

    /// Run complete netlist-wide DC sensitivity.
    ///
    /// Covers every eligible real instance/model/source parameter across
    /// flattened hierarchy. Outputs may be differential voltages or branch
    /// currents, and SPICE wildcard filters select devices or parameters.
    #[pyo3(signature = (netlist, output, reference=None, filters=None, output_is_current=false))]
    fn run_sensitivity_dc_complete(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output: NodeIdentifier,
        reference: Option<NodeIdentifier>,
        filters: Option<Vec<String>>,
        output_is_current: bool,
    ) -> PyResult<PySensitivityResult> {
        directives::run_one_card(
            self,
            py,
            netlist,
            AnalysisCommand::Sensitivity {
                output_node: node_identifier_name(&output),
                reference_node: reference.as_ref().map(node_identifier_name),
                output_is_current,
                filters: filters.unwrap_or_default(),
                ac_sweep: None,
            },
        )?
        .sensitivity
        .into_single(".sens")
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
            return Err(crate::errors::value_error(format!(
                "param_value must be finite, got {param_value}"
            )));
        }
        if let Some(d) = delta
            && (!d.is_finite() || d <= 0.0)
        {
            return Err(crate::errors::value_error(format!(
                "delta must be a positive finite number, got {d}"
            )));
        }
        validate_frequencies(&frequencies)?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let output = self.resolve_node(py, &engine, &netlist.inner, &output_node, "output")?;
        let values = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_sensitivity_ac_with_abort(
                &netlist.inner,
                output,
                param_name,
                param_value,
                &frequencies,
                delta,
                abort,
            )
        })?;
        Ok(values.to_pyarray(py))
    }

    /// Run complete netlist-wide complex AC sensitivity analysis.
    ///
    /// Every eligible explicit real-valued device, instance, model, source,
    /// and real-vector parameter in the flattened hierarchy is varied. The
    /// result includes complex absolute and normalized derivatives plus
    /// magnitude, phase, and dB derivatives.
    #[pyo3(signature = (
        netlist,
        output,
        frequencies,
        reference=None,
        filters=None,
        output_is_current=false
    ))]
    #[allow(clippy::too_many_arguments)]
    fn run_sensitivity_ac_complete(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output: NodeIdentifier,
        frequencies: Vec<f64>,
        reference: Option<NodeIdentifier>,
        filters: Option<Vec<String>>,
        output_is_current: bool,
    ) -> PyResult<PyAcSensitivityResult> {
        self.sensitivity_ac_complete_impl(
            py,
            netlist,
            &output,
            reference.as_ref(),
            output_is_current,
            &frequencies,
            filters.as_deref().unwrap_or(&[]),
        )
    }

    /// Run complete netlist-wide complex AC sensitivity over a DEC/OCT/LIN
    /// sweep, exactly as a `.SENS ... AC` card states it.
    #[pyo3(signature = (
        netlist,
        output,
        variation,
        points,
        start_freq,
        stop_freq,
        reference=None,
        filters=None,
        output_is_current=false
    ))]
    #[allow(clippy::too_many_arguments)]
    fn run_sensitivity_ac_sweep(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output: NodeIdentifier,
        variation: &str,
        points: usize,
        start_freq: f64,
        stop_freq: f64,
        reference: Option<NodeIdentifier>,
        filters: Option<Vec<String>>,
        output_is_current: bool,
    ) -> PyResult<PyAcSensitivityResult> {
        directives::run_one_card(
            self,
            py,
            netlist,
            AnalysisCommand::Sensitivity {
                output_node: node_identifier_name(&output),
                reference_node: reference.as_ref().map(node_identifier_name),
                output_is_current,
                filters: filters.unwrap_or_default(),
                ac_sweep: Some(rspice_core::netlist::SensitivityAcSweep {
                    variation: parse_variation(variation)?,
                    points,
                    start_freq,
                    stop_freq,
                }),
            },
        )?
        .sensitivity_ac
        .into_single(".sens ac")
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
            return Err(crate::errors::value_error("values must not be empty"));
        }
        for (index, value) in values.iter().enumerate() {
            if !value.is_finite() {
                return Err(crate::errors::value_error(format!(
                    "step value at index {index} must be finite, got {value}"
                )));
            }
        }

        let engine = self.engine_for_netlist(&netlist.inner);
        let results = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_step_with_abort(&netlist.inner, param_name, &values, abort)
        })?;

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
        directives::run_one_card(
            self,
            py,
            netlist,
            AnalysisCommand::Tf {
                output_node: output_node.to_string(),
                reference_node: reference_node.map(str::to_string),
                output_is_current,
                input_source: input_source.to_string(),
            },
        )?
        .tf
        .into_single(".tf")
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
