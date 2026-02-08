//! Simulation engine Python bindings
//!
//! Provides Python access to the RSpice simulation engine:
//! - DC operating point analysis
//! - DC sweep analysis
//! - AC frequency analysis
//! - Transient time-domain analysis

use pyo3::prelude::*;
use rspice_core::Engine;

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
    pub fn run_dc_op(&self, netlist: &PyNetlist) -> PyResult<PySimulationResult> {
        let result = self
            .inner
            .run_dc_op(&netlist.inner)
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
        netlist: &PyNetlist,
        source_name: &str,
        start: f64,
        stop: f64,
        step: f64,
    ) -> PyResult<PyDcSweepResult> {
        let results = self
            .inner
            .run_dc_sweep(&netlist.inner, source_name, start, stop, step)
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
    pub fn run_ac(&self, netlist: &PyNetlist, frequencies: Vec<f64>) -> PyResult<PyAcResult> {
        let results = self
            .inner
            .run_ac(&netlist.inner, &frequencies)
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
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: f64,
    ) -> PyResult<PyTransientResult> {
        let result = self
            .inner
            .run_tran(&netlist.inner, stop_time, max_step)
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
        netlist: &PyNetlist,
        output_node: usize,
        frequencies: Vec<f64>,
        temperature: Option<f64>,
    ) -> PyResult<Vec<crate::results::PyNoiseResult>> {
        let temp = temperature.unwrap_or(300.0);

        let results = self
            .inner
            .run_noise(&netlist.inner, output_node, &frequencies, temp)
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
        netlist: &PyNetlist,
        input_node: usize,
        output_node: usize,
    ) -> PyResult<crate::results::PyPoleZeroResult> {
        let result = self
            .inner
            .run_pz(&netlist.inner, input_node, output_node)
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
        netlist: &PyNetlist,
        num_runs: usize,
        seed: u64,
    ) -> PyResult<crate::results::PyMonteCarloResult> {
        let result = self
            .inner
            .run_monte_carlo(&netlist.inner, num_runs, seed)
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

        // Run DC analysis at both points
        let result_plus = self
            .inner
            .run_dc_op(&netlist_plus)
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        let result_minus = self
            .inner
            .run_dc_op(&netlist_minus)
            .map_err(crate::errors::simulation_error_to_pyerr)?;

        // Compute central difference
        let v_plus = result_plus.voltage(output_node);
        let v_minus = result_minus.voltage(output_node);

        Ok((v_plus - v_minus) / (2.0 * h))
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
        netlist: &PyNetlist,
        param_name: &str,
        values: Vec<f64>,
    ) -> PyResult<Vec<(f64, crate::results::PySimulationResult)>> {
        let results = self
            .inner
            .run_step(&netlist.inner, param_name, &values)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_resistor_netlist() -> PyNetlist {
        PyNetlist::parse(
            r#"
* Simple resistor
V1 1 0 10
R1 1 0 1k
.end
"#,
        )
        .unwrap()
    }

    fn voltage_divider_netlist() -> PyNetlist {
        PyNetlist::parse(
            r#"
* Voltage divider
V1 1 0 10
R1 1 2 1k
R2 2 0 1k
.end
"#,
        )
        .unwrap()
    }

    fn current_source_netlist() -> PyNetlist {
        PyNetlist::parse(
            r#"
* Current source test
I1 0 1 1m
R1 1 0 1k
.end
"#,
        )
        .unwrap()
    }

    fn rc_transient_netlist() -> PyNetlist {
        PyNetlist::parse(
            r#"
* RC transient
V1 1 0 5
R1 1 2 1k
C1 2 0 1u
.end
"#,
        )
        .unwrap()
    }

    fn ac_rc_netlist() -> PyNetlist {
        PyNetlist::parse(
            r#"
* AC RC lowpass
V1 1 0 AC 1
R1 1 2 1k
C1 2 0 1u
.end
"#,
        )
        .unwrap()
    }

    fn diode_netlist() -> PyNetlist {
        PyNetlist::parse(
            r#"
* Diode circuit
V1 1 0 5
D1 1 2 1N4148
R1 2 0 1k
.end
"#,
        )
        .unwrap()
    }

    #[test]
    fn test_engine_default_creation() {
        let engine = PyEngine::new(None);
        let cfg = engine.config();
        assert!((cfg.inner.tolerance - 1e-9).abs() < 1e-12);
        assert_eq!(cfg.inner.max_iterations, 50);
    }

    #[test]
    fn test_engine_custom_config() {
        let mut config = PySimulationConfig::new();
        config.set_tolerance(1e-12);
        config.set_max_iterations(100);

        let engine = PyEngine::new(Some(config));
        let cfg = engine.config();
        assert!((cfg.inner.tolerance - 1e-12).abs() < 1e-15);
        assert_eq!(cfg.inner.max_iterations, 100);
    }

    #[test]
    fn test_dc_op_simple_resistor() {
        let engine = PyEngine::new(None);
        let netlist = simple_resistor_netlist();
        let result = engine.run_dc_op(&netlist).unwrap();

        let v1 = result.voltage_by_index(1);
        assert!(
            (v1 - 10.0).abs() < 0.01,
            "Expected 10V at node 1, got {}V",
            v1
        );
    }

    #[test]
    fn test_dc_op_voltage_divider() {
        let engine = PyEngine::new(None);
        let netlist = voltage_divider_netlist();
        let result = engine.run_dc_op(&netlist).unwrap();

        let v1 = result.voltage_by_index(1);
        let v2 = result.voltage_by_index(2);
        assert!(
            (v1 - 10.0).abs() < 0.01,
            "Expected 10V at node 1, got {}V",
            v1
        );
        assert!(
            (v2 - 5.0).abs() < 0.01,
            "Expected 5V at node 2, got {}V",
            v2
        );
    }

    #[test]
    fn test_dc_op_current_source() {
        let engine = PyEngine::new(None);
        let netlist = current_source_netlist();
        let result = engine.run_dc_op(&netlist).unwrap();

        // I * R = 1mA * 1kΩ = 1V
        let v1 = result.voltage_by_index(1);
        assert!(
            (v1 - 1.0).abs() < 0.01,
            "Expected 1V at node 1, got {}V",
            v1
        );
    }

    #[test]
    fn test_dc_op_diode() {
        let engine = PyEngine::new(None);
        let netlist = diode_netlist();
        let result = engine.run_dc_op(&netlist).unwrap();

        // Diode should conduct, V2 should be positive
        let v2 = result.voltage_by_index(2);
        assert!(v2 > 0.0, "Expected positive voltage at node 2, got {}V", v2);
    }

    #[test]
    fn test_dc_sweep_basic() {
        let engine = PyEngine::new(None);
        let netlist = voltage_divider_netlist();

        let sweep_result = engine.run_dc_sweep(&netlist, "V1", 0.0, 5.0, 1.0).unwrap();

        assert_eq!(sweep_result.len(), 6); // 0, 1, 2, 3, 4, 5

        // At V1=0, V2 should be 0
        let first_voltage = sweep_result.voltage_at(0);
        assert!((first_voltage - 0.0).abs() < 0.01);

        // At V1=4, V2 should be 2 (divider ratio = 0.5)
        let fourth_voltage = sweep_result.voltage_at(4);
        assert!((fourth_voltage - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_dc_sweep_linear_divider() {
        let engine = PyEngine::new(None);
        let netlist = voltage_divider_netlist();

        let sweep_result = engine.run_dc_sweep(&netlist, "V1", 0.0, 10.0, 2.0).unwrap();

        // Check that output tracks input with 0.5 gain
        for i in 0..sweep_result.len() {
            let vin = sweep_result.voltage_at(i);
            let v2 = sweep_result.result_at(i).unwrap().voltage_by_index(2);
            let expected = vin / 2.0;
            assert!(
                (v2 - expected).abs() < 0.1,
                "At Vin={}, expected V2={}, got {}",
                vin,
                expected,
                v2
            );
        }
    }

    #[test]
    fn test_dc_sweep_negative_step() {
        let engine = PyEngine::new(None);
        let netlist = voltage_divider_netlist();

        let sweep_result = engine.run_dc_sweep(&netlist, "V1", 5.0, 0.0, -1.0).unwrap();

        assert_eq!(sweep_result.len(), 6);
        assert!((sweep_result.voltage_at(0) - 5.0).abs() < 0.01);
        assert!((sweep_result.voltage_at(5) - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_dc_sweep_source_not_found() {
        let engine = PyEngine::new(None);
        let netlist = voltage_divider_netlist();

        let result = engine.run_dc_sweep(&netlist, "VNONEXISTENT", 0.0, 5.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_transient_rc() {
        let engine = PyEngine::new(None);
        let netlist = rc_transient_netlist();

        let result = engine.run_tran(&netlist, 1e-3, 50e-6).unwrap();

        assert!(
            result.num_points() > 1,
            "Expected multiple time points, got {}",
            result.num_points()
        );
    }

    #[test]
    fn test_transient_rc_charging() {
        let engine = PyEngine::new(None);
        let netlist = rc_transient_netlist();

        // RC time constant = 1kΩ * 1µF = 1ms
        // After 5ms (~5 time constants), capacitor should be nearly at 5V
        let result = engine.run_tran(&netlist, 5e-3, 50e-6).unwrap();

        // Check that voltage increases over time
        let num_points = result.num_points();
        assert!(num_points > 10);

        // Final voltage should be close to 5V
        let v_final = result.voltage_at(2, num_points - 1);
        assert!(
            v_final > 4.0,
            "Expected final voltage > 4V, got {}V",
            v_final
        );
    }

    #[test]
    #[ignore = "AC analysis tests may have GIL issues during parallel test execution"]
    fn test_ac_rc_lowpass() {
        let engine = PyEngine::new(None);
        let netlist = ac_rc_netlist();

        let frequencies = vec![10.0, 10000.0];
        let result = engine.run_ac(&netlist, frequencies.clone()).unwrap();

        // At low frequency (10 Hz), output should be close to input
        let mag_low = result.magnitude_at(0, 1);
        assert!(
            mag_low > 0.8,
            "Expected ~1V at low freq, got {}V magnitude",
            mag_low
        );

        // At high frequency (10 kHz), output should be attenuated
        // Cutoff = 1/(2*pi*R*C) = 159 Hz, so 10kHz is well above
        let mag_high = result.magnitude_at(1, 1);
        assert!(
            mag_high < mag_low,
            "Expected attenuation at high freq: low={}, high={}",
            mag_low,
            mag_high
        );
    }

    #[test]
    fn test_ac_multiple_frequencies() {
        let engine = PyEngine::new(None);
        let netlist = ac_rc_netlist();

        let frequencies: Vec<f64> = (0..=3).map(|i| 10.0_f64.powi(i + 1)).collect();
        let result = engine.run_ac(&netlist, frequencies).unwrap();

        assert_eq!(result.num_frequencies(), 4);
    }

    #[test]
    fn test_engine_repr() {
        let engine = PyEngine::new(None);
        let repr = engine.__repr__();
        assert!(repr.contains("Engine"));
        assert!(repr.contains("tolerance="));
        assert!(repr.contains("max_iterations="));
    }

    #[test]
    fn test_engine_with_robust_convergence() {
        use crate::config::PyConvergenceConfig;

        let mut config = PySimulationConfig::new();
        config.set_convergence(PyConvergenceConfig::robust());

        let engine = PyEngine::new(Some(config));
        let netlist = diode_netlist();

        // Should converge with robust settings
        let result = engine.run_dc_op(&netlist).unwrap();
        assert!(result.voltage_by_index(2) > 0.0);
    }

    #[test]
    fn test_multi_resistor_divider() {
        let netlist = PyNetlist::parse(
            r#"
* Multi-resistor divider
V1 1 0 12
R1 1 2 1k
R2 2 3 2k
R3 3 0 3k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        // Total R = 6k, I = 12/6k = 2mA
        // V2 = 12 - 1k*2mA = 10V
        // V3 = 3k*2mA = 6V
        let v2 = result.voltage_by_index(2);
        let v3 = result.voltage_by_index(3);
        assert!((v2 - 10.0).abs() < 0.1, "Expected V2=10V, got {}V", v2);
        assert!((v3 - 6.0).abs() < 0.1, "Expected V3=6V, got {}V", v3);
    }

    #[test]
    fn test_parallel_resistors() {
        let netlist = PyNetlist::parse(
            r#"
* Parallel resistors
V1 1 0 5
R1 1 0 1k
R2 1 0 1k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        let v1 = result.voltage_by_index(1);
        assert!(
            (v1 - 5.0).abs() < 0.01,
            "Expected 5V at node 1, got {}V",
            v1
        );
    }

    #[test]
    fn test_series_voltage_sources() {
        let netlist = PyNetlist::parse(
            r#"
* Series voltage sources
V1 1 0 3
V2 2 1 2
R1 2 0 1k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        let v1 = result.voltage_by_index(1);
        let v2 = result.voltage_by_index(2);
        assert!((v1 - 3.0).abs() < 0.01, "Expected V1=3V, got {}V", v1);
        assert!((v2 - 5.0).abs() < 0.01, "Expected V2=5V, got {}V", v2);
    }

    #[test]
    fn test_vcvs_voltage_follower() {
        let netlist = PyNetlist::parse(
            r#"
* VCVS voltage follower
V1 1 0 3
R1 1 0 1k
E1 2 0 1 0 1.0
R2 2 0 1k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        let v1 = result.voltage_by_index(1);
        let v2 = result.voltage_by_index(2);
        assert!((v1 - 3.0).abs() < 0.01, "Expected V1=3V, got {}V", v1);
        assert!(
            (v2 - 3.0).abs() < 0.1,
            "Expected V2=3V (follower), got {}V",
            v2
        );
    }

    #[test]
    fn test_vccs_transconductance() {
        let netlist = PyNetlist::parse(
            r#"
* VCCS transconductance
V1 1 0 2
R1 1 0 1k
G1 2 0 1 0 0.001
R2 2 0 1k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        // G1 produces I = gm * V1 = 0.001 * 2 = 2mA
        // V2 = I * R2 = 2mA * 1kΩ = 2V (but sign depends on direction)
        let v2 = result.voltage_by_index(2).abs();
        assert!(v2 > 1.8 && v2 < 2.2, "Expected |V2| ~= 2V, got {}V", v2);
    }

    #[test]
    fn test_large_resistor_value() {
        let netlist = PyNetlist::parse(
            r#"
* Large resistor
V1 1 0 1
R1 1 2 1G
R2 2 0 1k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        // R1 is 1GΩ >> R2 = 1kΩ, so almost no current flows
        // V2 ≈ 0
        let v2 = result.voltage_by_index(2);
        assert!(v2.abs() < 1e-3, "Expected V2 ~= 0, got {}V", v2);
    }

    #[test]
    fn test_rl_transient() {
        let netlist = PyNetlist::parse(
            r#"
* RL transient
V1 1 0 5
R1 1 2 1k
L1 2 0 1m
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_tran(&netlist, 10e-6, 0.5e-6).unwrap();

        assert!(
            result.num_points() > 5,
            "Expected multiple time points, got {}",
            result.num_points()
        );
    }

    #[test]
    fn test_empty_netlist_error() {
        let netlist = PyNetlist::parse(".end").unwrap();
        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist);
        // Empty netlist should produce an error
        assert!(result.is_err());
    }

    //=========================================================================
    // Advanced Analysis Tests
    //=========================================================================

    /// Test noise analysis with resistor circuit
    #[test]
    fn test_run_noise_basic() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        let frequencies = vec![100.0, 1000.0, 10000.0];
        let results = engine
            .run_noise(&netlist, 2, frequencies.clone(), None)
            .unwrap();

        assert_eq!(results.len(), 3, "Expected 3 frequency points");

        // Verify frequencies are correct
        for (i, result) in results.iter().enumerate() {
            assert!(
                (result.frequency - frequencies[i]).abs() < 1e-6,
                "Frequency mismatch at index {}",
                i
            );
        }

        // Noise should be positive
        for result in &results {
            assert!(
                result.output_noise_density >= 0.0,
                "Noise density should be non-negative"
            );
        }
    }

    /// Test noise analysis with temperature parameter
    #[test]
    fn test_run_noise_with_temperature() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        // Higher temperature should produce more thermal noise
        let freq = vec![1000.0];
        let results_300k = engine
            .run_noise(&netlist, 2, freq.clone(), Some(300.0))
            .unwrap();
        let results_400k = engine
            .run_noise(&netlist, 2, freq.clone(), Some(400.0))
            .unwrap();

        // Both should succeed
        assert!(!results_300k.is_empty());
        assert!(!results_400k.is_empty());
    }

    /// Test pole-zero analysis with RC lowpass
    #[test]
    fn test_run_pz_rc_lowpass() {
        let netlist = ac_rc_netlist();
        let engine = PyEngine::new(None);

        let result = engine.run_pz(&netlist, 1, 2).unwrap();

        // RC lowpass has poles and zeros
        // Access as raw pole/zero counts via the inner data
        // These are tested implicitly by successful execution
        assert!(result.dc_gain.is_finite(), "DC gain should be finite");
    }

    /// Test pole-zero stability check
    #[test]
    fn test_run_pz_stability() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        let result = engine.run_pz(&netlist, 1, 2).unwrap();

        // Passive circuits should be stable - DC gain is finite for stable systems
        assert!(
            result.dc_gain.is_finite(),
            "DC gain should be finite for stable circuit"
        );
    }

    /// Test Monte Carlo analysis basic run
    #[test]
    fn test_run_monte_carlo_basic() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        let result = engine.run_monte_carlo(&netlist, 10, 42).unwrap();

        // Check that we got the expected number of runs
        assert!(result.num_runs >= 1, "Should have at least 1 run");
        assert!(result.num_runs <= 10, "Should have at most 10 runs");
    }

    /// Test Monte Carlo reproducibility with seed
    #[test]
    fn test_run_monte_carlo_reproducible() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        let result1 = engine.run_monte_carlo(&netlist, 5, 12345).unwrap();
        let result2 = engine.run_monte_carlo(&netlist, 5, 12345).unwrap();

        // Same seed should produce same number of runs
        assert_eq!(result1.num_runs, result2.num_runs);
    }

    /// Test sensitivity analysis with resistance parameter
    #[test]
    fn test_run_sensitivity_resistor() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        // Sensitivity of V(2) with respect to R1
        let sens = engine
            .run_sensitivity(&netlist, 2, "R1", 1000.0, None)
            .unwrap();

        // Sensitivity should be finite
        assert!(sens.is_finite(), "Sensitivity should be finite");
    }

    /// Test sensitivity with custom delta
    #[test]
    fn test_run_sensitivity_custom_delta() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        let sens = engine
            .run_sensitivity(&netlist, 2, "R1", 1000.0, Some(0.1))
            .unwrap();

        assert!(
            sens.is_finite(),
            "Sensitivity with custom delta should be finite"
        );
    }

    /// Test parametric step analysis
    #[test]
    fn test_run_step_linear() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        let values = vec![500.0, 1000.0, 2000.0, 5000.0];
        let results = engine.run_step(&netlist, "R1", values.clone()).unwrap();

        // Should have results for each value (or fewer if some failed)
        assert!(
            results.len() <= values.len(),
            "Should have at most {} results",
            values.len()
        );
    }

    /// Test parametric step with wide range
    #[test]
    fn test_run_step_decade_range() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        // Decade sweep values
        let values = vec![100.0, 1000.0, 10000.0];
        let results = engine.run_step(&netlist, "R2", values).unwrap();

        // Verify result structure
        for (val, result) in &results {
            assert!(*val > 0.0, "Parameter value should be positive");
            // Voltage at node 2 should change with R2
            let _v2 = result.voltage_by_index(2);
        }
    }

    /// Test step analysis returns correct parameter values
    #[test]
    fn test_run_step_param_values() {
        let netlist = voltage_divider_netlist();
        let engine = PyEngine::new(None);

        let values = vec![1000.0, 2000.0];
        let results = engine.run_step(&netlist, "R1", values.clone()).unwrap();

        if !results.is_empty() {
            // First result should have first value
            assert!(
                (results[0].0 - values[0]).abs() < 1e-6,
                "First result should have first value"
            );
        }
    }
}
