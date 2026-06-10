//! Simulation engine Python bindings
//!
//! Provides Python access to the RSpice simulation engine:
//! - DC operating point analysis
//! - DC sweep analysis
//! - AC frequency analysis
//! - Transient time-domain analysis

use pyo3::prelude::*;
use rspice_core::{Engine, SimulationConfigOverrides, resolve_simulation_config};

use crate::config::PySimulationConfig;
use crate::netlist::PyNetlist;
use crate::results::{PyAcResult, PyDcSweepResult, PySimulationResult, PyTransientResult};

/// RSpice simulation engine
///
/// The Engine class is the main interface for running circuit simulations.
/// It can be configured with custom simulation parameters or use defaults.
///
/// Example:
///     >>> engine = Engine()
///     >>> result = engine.run_dc_op(netlist)
///     >>> print(f"V(1) = {result.voltage(1)} V")
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
    ///     >>> config = SimulationConfig()
    ///     >>> config.tolerance = 1e-12
    ///     >>> engine = Engine(config)  # Custom config
    #[new]
    #[pyo3(signature = (config=None))]
    pub fn new(config: Option<PySimulationConfig>) -> Self {
        let inner = match config {
            Some(cfg) => Engine::new(cfg.inner),
            None => Engine::default(),
        };
        Self { inner }
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
    /// DC operating point at each step.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     source_name: Name of voltage source to sweep (e.g., "V1")
    ///     start: Starting voltage value
    ///     stop: Ending voltage value
    ///     step: Voltage step size
    ///
    /// Returns:
    ///     DcSweepResult: Collection of DC solutions at each sweep point
    ///
    /// Raises:
    ///     SimulationError: If the sweep fails
    ///
    /// Example:
    ///     >>> result = engine.run_dc_sweep(netlist, "V1", 0, 5, 0.1)
    ///     >>> for v_in, sol in result.points():
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
        let engine = self.engine_for_netlist(&netlist.inner);
        let results = py
            .allow_threads(|| engine.run_dc_sweep(&netlist.inner, source_name, start, stop, step))
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PyDcSweepResult::new(results))
    }

    /// Run AC small-signal analysis
    ///
    /// Linearizes the circuit around its DC operating point and computes
    /// the frequency response at the specified frequencies.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     frequencies: List of frequencies (Hz) to analyze
    ///
    /// Returns:
    ///     AcResult: Complex voltage/current phasors at each frequency
    ///
    /// Raises:
    ///     SimulationError: If the analysis fails
    ///
    /// Example:
    ///     >>> freqs = [10, 100, 1000, 10000]
    ///     >>> result = engine.run_ac(netlist, freqs)
    ///     >>> for f, mag in zip(result.frequencies, result.voltage_magnitude(2)):
    ///     ...     print(f"{f:.0f} Hz: {20*np.log10(mag):.1f} dB")
    pub fn run_ac(
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

    /// Run transient time-domain analysis
    ///
    /// Simulates the circuit from t=0 to stop_time using numerical
    /// integration methods (Trapezoidal/Gear).
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     stop_time: Simulation end time in seconds
    ///     max_step: Maximum timestep in seconds (controls accuracy)
    ///
    /// Returns:
    ///     TransientResult: Time-domain waveforms for all nodes
    ///
    /// Raises:
    ///     SimulationError: If the simulation fails
    ///     ConvergenceError: If a timestep fails to converge
    ///
    /// Example:
    ///     >>> result = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
    ///     >>> import matplotlib.pyplot as plt
    ///     >>> plt.plot(result.time, result.voltage_waveform(2))
    ///     >>> plt.show()
    #[pyo3(signature = (netlist, stop_time, max_step))]
    pub fn run_tran(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: f64,
    ) -> PyResult<PyTransientResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = py
            .allow_threads(|| engine.run_tran(&netlist.inner, stop_time, max_step))
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        Ok(PyTransientResult::new(result))
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
    ///     output_node: Node index to compute noise at
    ///     frequencies: List of frequencies (Hz) to compute noise at
    ///     temperature: Optional temperature in Kelvin (default: 300K)
    ///
    /// Returns:
    ///     list[NoiseResult]: Noise analysis results at each frequency
    ///
    /// Example:
    ///     >>> freqs = np.logspace(0, 6, 100)  # 1 Hz to 1 MHz
    ///     >>> results = engine.run_noise(netlist, 2, freqs.tolist())
    ///     >>> for r in results:
    ///     ...     print(f"{r.frequency:.0f}Hz: {r.output_noise_rms*1e9:.2f}nV/√Hz")
    #[pyo3(signature = (netlist, output_node, frequencies, temperature=None))]
    fn run_noise(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: usize,
        frequencies: Vec<f64>,
        temperature: Option<f64>,
    ) -> PyResult<Vec<crate::results::PyNoiseResult>> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let temp = temperature.unwrap_or(engine.config().temperature);

        let results = py
            .allow_threads(|| engine.run_noise(&netlist.inner, output_node, &frequencies, temp))
            .map_err(crate::errors::simulation_error_to_pyerr)?;

        Ok(results
            .iter()
            .map(crate::results::PyNoiseResult::from_core)
            .collect())
    }

    /// Run pole-zero analysis
    ///
    /// Finds the poles and zeros of the circuit's transfer function
    /// between input and output nodes.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     input_node: Input node index
    ///     output_node: Output node index
    ///
    /// Returns:
    ///     PoleZeroResult: Poles, zeros, and gain information
    ///
    /// Example:
    ///     >>> result = engine.run_pz(netlist, 1, 2)
    ///     >>> print(f"Number of poles: {result.num_poles}")
    ///     >>> print(f"Stable: {result.is_stable}")
    ///     >>> for p in result.poles:
    ///     ...     print(f"Pole at {p.real:.2e} + {p.imag:.2e}j")
    fn run_pz(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        input_node: usize,
        output_node: usize,
    ) -> PyResult<crate::results::PyPoleZeroResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = py
            .allow_threads(|| engine.run_pz(&netlist.inner, input_node, output_node))
            .map_err(crate::errors::simulation_error_to_pyerr)?;

        Ok(crate::results::PyPoleZeroResult::from_core(&result))
    }

    /// Run Monte Carlo analysis
    ///
    /// Runs multiple simulations with random parameter variations to
    /// compute statistical distributions of output values.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     num_runs: Number of Monte Carlo iterations
    ///     seed: Random seed for reproducibility
    ///
    /// Returns:
    ///     MonteCarloResult: Statistical results for all output variables
    ///
    /// Example:
    ///     >>> result = engine.run_monte_carlo(netlist, 1000, 42)
    ///     >>> v_out = result.get_variable("V(2)")
    ///     >>> print(f"V(2): {v_out.mean:.3f} ± {v_out.std_dev:.3f}V")
    fn run_monte_carlo(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        num_runs: usize,
        seed: u64,
    ) -> PyResult<crate::results::PyMonteCarloResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = py
            .allow_threads(|| engine.run_monte_carlo(&netlist.inner, num_runs, seed))
            .map_err(crate::errors::simulation_error_to_pyerr)?;

        Ok(crate::results::PyMonteCarloResult::from_core(&result))
    }

    /// Run sensitivity analysis
    ///
    /// Computes the sensitivity of an output node voltage with respect
    /// to a circuit parameter using finite differences.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     output_node: Node index to measure
    ///     param_name: Name of parameter to vary
    ///     param_value: Current value of parameter
    ///     delta: Optional perturbation size (default: 1% of value)
    ///
    /// Returns:
    ///     float: dV/dParam sensitivity value
    ///
    /// Example:
    ///     >>> sens = engine.run_sensitivity(netlist, 2, "R1", 1000.0)
    ///     >>> print(f"Sensitivity: {sens:.6f} V/Ω")
    #[pyo3(signature = (netlist, output_node, param_name, param_value, delta=None))]
    fn run_sensitivity(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: usize,
        param_name: &str,
        param_value: f64,
        delta: Option<f64>,
    ) -> PyResult<f64> {
        let h = delta.unwrap_or(param_value.abs() * 0.01).max(1e-12);

        // Create perturbed netlists
        let mut netlist_plus = netlist.inner.clone();
        let mut netlist_minus = netlist.inner.clone();

        // Set parameter values
        netlist_plus.params.set(param_name, param_value + h);
        netlist_minus.params.set(param_name, param_value - h);

        let plus_engine = self.engine_for_netlist(&netlist_plus);
        let minus_engine = self.engine_for_netlist(&netlist_minus);

        // Central difference from two DC solves
        py.allow_threads(
            || -> Result<f64, rspice_core::engine::SimulationError> {
                let result_plus = plus_engine.run_dc_op(&netlist_plus)?;
                let result_minus = minus_engine.run_dc_op(&netlist_minus)?;
                let v_plus = result_plus.voltage(output_node);
                let v_minus = result_minus.voltage(output_node);
                Ok((v_plus - v_minus) / (2.0 * h))
            },
        )
        .map_err(crate::errors::simulation_error_to_pyerr)
    }

    /// Run parametric step analysis
    ///
    /// Executes multiple DC operating point analyses with different
    /// parameter values, useful for design sweeps and optimization.
    ///
    /// Args:
    ///     netlist: Parsed netlist to simulate
    ///     param_name: Name of parameter to sweep
    ///     values: List of parameter values to simulate
    ///
    /// Returns:
    ///     list[tuple[float, SimulationResult]]: Results indexed by parameter value
    ///
    /// Example:
    ///     >>> values = [1000.0, 2000.0, 5000.0, 10000.0]
    ///     >>> results = engine.run_step(netlist, "R1", values)
    ///     >>> for val, result in results:
    ///     ...     print(f"R1={val:.0f}Ω: V(2)={result.voltage(2):.3f}V")
    fn run_step(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        param_name: &str,
        values: Vec<f64>,
    ) -> PyResult<Vec<(f64, crate::results::PySimulationResult)>> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let results = py
            .allow_threads(|| engine.run_step(&netlist.inner, param_name, &values))
            .map_err(crate::errors::simulation_error_to_pyerr)?;

        Ok(results
            .into_iter()
            .map(|(val, sim_result)| (val, crate::results::PySimulationResult::new(sim_result)))
            .collect())
    }

    /// Get a copy of the current simulation configuration
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
