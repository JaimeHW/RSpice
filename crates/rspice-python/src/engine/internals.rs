//! Private implementation behind the engine's public methods.
//!
//! These are the `*_impl` helpers each `#[pymethods]` entry point delegates to
//! once its arguments are validated, plus the configuration resolution that
//! decides which engine a given netlist actually runs on. Rust permits a second
//! inherent `impl` block for a type in a sibling module, which is what keeps the
//! public surface in `mod.rs` readable as an API contract rather than as
//! implementation.

use super::*;
use rspice_core::analysis::Distribution;

impl PyEngine {
    pub(super) fn engine_for_netlist(&self, netlist: &rspice_core::Netlist) -> Engine {
        let resolved = resolve_simulation_config(
            self.inner.config(),
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        );
        Engine::new(resolved)
    }

    /// Resolve a node identifier (index or name) to a node index, building
    /// the circuit to obtain the node map when a name is given.
    ///
    /// Elaborating a large hierarchical deck is real work that happens before
    /// any solver starts, so it runs on the interruptible worker like every
    /// other long call: `KeyboardInterrupt` reaches a name lookup, and the GIL
    /// is not held across the build.
    pub(super) fn resolve_node(
        &self,
        py: Python<'_>,
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
                let circuit = run_interruptible(py, &self.active_runs, |abort| {
                    engine.build_circuit_with_abort(netlist, abort)
                })?;
                circuit
                    .node_names_sorted()
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .map(|pos| pos + 1)
                    .ok_or_else(|| {
                        crate::errors::key_error(format!("unknown {what} node '{name}'"))
                    })
            }
        }
    }

    /// Core operating-point runner shared by the `.op` card and implicit OP.
    pub(super) fn dc_op_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
    ) -> PyResult<PySimulationResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let (result, device_op_report) = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_dc_op_with_report_and_abort(&netlist.inner, abort)
        })?;
        Ok(PySimulationResult::new_with_report(
            result,
            device_op_report,
        ))
    }

    /// Core Monte Carlo runner shared by `run_monte_carlo` and the `.mc` card.
    ///
    /// An unseeded request draws one seed here, so both surfaces are
    /// reproducible in exactly the same way: a `.MC` card without `SEED=` and
    /// a `run_monte_carlo(seed=None)` call are the same request.
    pub(super) fn monte_carlo_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        command: &rspice_core::netlist::MonteCarloCommand,
    ) -> PyResult<PyMonteCarloResult> {
        let spread = command.relative_spread;
        let distribution = match command.distribution {
            rspice_core::netlist::MonteCarloDistribution::Gaussian => {
                Distribution::Gaussian { sigma: spread }
            }
            rspice_core::netlist::MonteCarloDistribution::Uniform => {
                Distribution::Uniform { tolerance: spread }
            }
            rspice_core::netlist::MonteCarloDistribution::WorstCase => {
                Distribution::WorstCase { tolerance: spread }
            }
        };
        let seed = command
            .seed
            .unwrap_or_else(|| RandomState::new().build_hasher().finish());
        let params = (!command.params.is_empty()).then(|| command.params.clone());
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_monte_carlo_with_options_and_abort(
                &netlist.inner,
                command.runs,
                seed,
                distribution,
                params.as_deref(),
                abort,
            )
        })?;
        Ok(PyMonteCarloResult::from_core(&result))
    }

    /// Core transient runner shared by `run_tran` and `run()`.
    pub(super) fn tran_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: f64,
        start_time: f64,
        startup_mode: Option<rspice_core::engine::TransientStartupMode>,
    ) -> PyResult<PyTransientResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| {
            if let Some(startup_mode) = startup_mode {
                engine.run_tran_with_startup_mode_and_abort(
                    &netlist.inner,
                    stop_time,
                    max_step,
                    startup_mode,
                    abort,
                )
            } else {
                engine.run_tran_with_abort(&netlist.inner, stop_time, max_step, abort)
            }
        })?;
        PyTransientResult::new_with_start(result, start_time)
    }

    /// Core AC runner shared by `run_ac`, `run_ac_sweep`, and `run()`.
    pub(super) fn ac_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
    ) -> PyResult<PyAcResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let results = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_ac_with_abort(&netlist.inner, &frequencies, abort)
        })?;
        PyAcResult::new(frequencies, results)
    }

    /// Core distortion runner shared by the direct and deck APIs.
    ///
    /// The F1 grid and the fixed-F2 ratio are validated by core's `.DISTO`
    /// rules inside the runner, not again here: what a `.DISTO` argument means
    /// is decided in exactly one place, so the direct API and a deck's own
    /// card cannot accept different inputs.
    pub(super) fn distortion_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
        f2_over_f1: Option<f64>,
    ) -> PyResult<PyDistortionResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_distortion_with_abort(&netlist.inner, &frequencies, f2_over_f1, abort)
        })?;
        PyDistortionResult::from_core(&result)
    }

    /// Core noise runner shared by `run_noise` and `run()`.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn noise_core_impl(
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
            return Err(crate::errors::value_error(format!(
                "temperature must be a positive number of Kelvin, got {temp}"
            )));
        }

        let results = run_interruptible(py, &self.active_runs, |abort| match input_source {
            Some(source) => engine.run_noise_with_input_source_and_abort(
                &netlist.inner,
                output_node,
                output_neg,
                source,
                frequencies,
                temp,
                abort,
            ),
            None => match output_neg {
                Some(_) => engine.run_noise_ports_with_abort(
                    &netlist.inner,
                    output_node,
                    output_neg,
                    frequencies,
                    temp,
                    abort,
                ),
                None => engine.run_noise_with_abort(
                    &netlist.inner,
                    output_node,
                    frequencies,
                    temp,
                    abort,
                ),
            },
        })?;

        Ok(results)
    }

    /// Python noise runner shared by `run_noise` and direct API calls.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn noise_impl(
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
    pub(super) fn tf_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output_node: &str,
        reference_node: Option<&str>,
        output_is_current: bool,
        input_source: &str,
    ) -> PyResult<PyTransferFunctionResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_transfer_function_with_abort(
                &netlist.inner,
                output_node,
                reference_node,
                output_is_current,
                input_source,
                abort,
            )
        })?;
        Ok(PyTransferFunctionResult::from_core(&result))
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn stb_impl(
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
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_stb_with_abort(&netlist.inner, config, abort)
        })?;
        Ok(PyStbResult::from_core(&result))
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn pz_impl(
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
        let input_pos = self.resolve_node(py, &engine, &netlist.inner, input_pos, "PZ input+")?;
        let input_neg = input_neg
            .map(|node| self.resolve_node(py, &engine, &netlist.inner, node, "PZ input-"))
            .transpose()?;
        let output_pos =
            self.resolve_node(py, &engine, &netlist.inner, output_pos, "PZ output+")?;
        let output_neg = output_neg
            .map(|node| self.resolve_node(py, &engine, &netlist.inner, node, "PZ output-"))
            .transpose()?;
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_pz_ports_with_abort(
                &netlist.inner,
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                input_is_current,
                compute_poles,
                compute_zeros,
                abort,
            )
        })?;
        PyPoleZeroResult::from_core(&result)
    }

    pub(super) fn sensitivity_linearized_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output: &NodeIdentifier,
        reference: Option<&NodeIdentifier>,
    ) -> PyResult<PySensitivityResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let output =
            self.resolve_node(py, &engine, &netlist.inner, output, "sensitivity output")?;
        let reference = reference
            .map(|node| {
                self.resolve_node(py, &engine, &netlist.inner, node, "sensitivity reference")
            })
            .transpose()?;
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_sensitivity_linearized_with_abort(&netlist.inner, output, reference, abort)
        })?;
        Ok(PySensitivityResult::from_core(&result))
    }

    pub(super) fn sensitivity_dc_complete_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output: &NodeIdentifier,
        reference: Option<&NodeIdentifier>,
        output_is_current: bool,
        filters: &[String],
    ) -> PyResult<PySensitivityResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let output = if output_is_current {
            if reference.is_some() {
                return Err(crate::errors::value_error(
                    "a branch-current sensitivity output cannot have a reference node",
                ));
            }
            let NodeIdentifier::Name(element) = output else {
                return Err(crate::errors::type_error(
                    "a branch-current sensitivity output must be an element name",
                ));
            };
            AcSensitivityOutput::BranchCurrent(element.clone())
        } else {
            let positive =
                self.resolve_node(py, &engine, &netlist.inner, output, "DC sensitivity output")?;
            let negative = reference
                .map(|node| {
                    self.resolve_node(
                        py,
                        &engine,
                        &netlist.inner,
                        node,
                        "DC sensitivity reference",
                    )
                })
                .transpose()?;
            AcSensitivityOutput::Voltage { positive, negative }
        };
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_sensitivity_dc_complete_with_abort(&netlist.inner, output, filters, abort)
        })?;
        Ok(PySensitivityResult::from_core(&result))
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn sensitivity_ac_complete_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        output: &NodeIdentifier,
        reference: Option<&NodeIdentifier>,
        output_is_current: bool,
        frequencies: &[f64],
        filters: &[String],
    ) -> PyResult<PyAcSensitivityResult> {
        validate_frequencies(frequencies)?;
        let engine = self.engine_for_netlist(&netlist.inner);
        let output = if output_is_current {
            if reference.is_some() {
                return Err(crate::errors::value_error(
                    "a branch-current sensitivity output cannot have a reference node",
                ));
            }
            let NodeIdentifier::Name(element) = output else {
                return Err(crate::errors::type_error(
                    "a branch-current sensitivity output must be an element name",
                ));
            };
            AcSensitivityOutput::BranchCurrent(element.clone())
        } else {
            let positive =
                self.resolve_node(py, &engine, &netlist.inner, output, "AC sensitivity output")?;
            let negative = reference
                .map(|node| {
                    self.resolve_node(
                        py,
                        &engine,
                        &netlist.inner,
                        node,
                        "AC sensitivity reference",
                    )
                })
                .transpose()?;
            AcSensitivityOutput::Voltage { positive, negative }
        };
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_sensitivity_ac_complete_with_abort(
                &netlist.inner,
                output,
                frequencies,
                filters,
                abort,
            )
        })?;
        Ok(PyAcSensitivityResult::from_core(&result))
    }

    pub(super) fn sparameter_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
        do_noise: bool,
    ) -> PyResult<PySParameterResult> {
        validate_frequencies(&frequencies)?;
        if frequencies.contains(&0.0) {
            return Err(crate::errors::value_error(
                "S-parameter frequencies must be strictly positive",
            ));
        }
        let ports = s_param::collect_ports(&netlist.inner)
            .map_err(|error| crate::errors::value_error(error.to_string()))?;
        let port_names = ports
            .iter()
            .map(|port| port.source_name.clone())
            .collect::<Vec<_>>();
        let impedances = ports.iter().map(|port| port.z0).collect::<Vec<_>>();
        let engine = self.engine_for_netlist(&netlist.inner);
        let temperature = engine.config().temperature;
        let (parameters, noise) = run_interruptible(py, &self.active_runs, |abort| {
            let scattering = s_param::extract_s_matrix_with_abort(
                &netlist.inner,
                &ports,
                &frequencies,
                |driven| {
                    engine
                        .run_ac_with_abort(driven, &frequencies, abort)
                        .map_err(|error| error.to_string())
                },
                abort,
            )
            // The extraction reports its own cancellation, but an interrupt
            // caught inside the caller's AC solve still surfaces as a failed
            // solve, so the abort signal — not the error text — decides which
            // it was. A cancelled run must never reach the caller dressed up
            // as a defect in their circuit.
            .map_err(|error| {
                if matches!(error, s_param::ExtractError::Aborted) || abort.is_aborted() {
                    rspice_core::engine::SimulationError::Aborted
                } else {
                    rspice_core::engine::SimulationError::Circuit(error.to_string())
                }
            })?;

            let noise = if do_noise {
                let points = engine.run_port_noise_correlation_with_abort(
                    &netlist.inner,
                    &port_names,
                    &frequencies,
                    temperature,
                    abort,
                )?;
                // Folding the sweep, checking its alignment, and deriving the
                // two-port parameters is one core operation with one validity
                // policy: an undefined parameter set fails the analysis rather
                // than being published behind a mask.
                Some(
                    s_param::assemble_port_noise_with_abort(
                        &points,
                        &frequencies,
                        &scattering,
                        &impedances,
                        temperature,
                        abort,
                    )
                    .map_err(port_noise_simulation_error)?,
                )
            } else {
                None
            };
            Ok((scattering, noise))
        })?;
        Ok(PySParameterResult::new(
            frequencies,
            port_names,
            impedances,
            parameters,
            noise,
        ))
    }
}

/// Map a core port-noise assembly failure onto the engine's error type.
///
/// Cancellation stays cancellation: a `KeyboardInterrupt` that lands inside
/// the assembly must not reach the caller dressed up as a defect in their
/// circuit.
fn port_noise_simulation_error(
    error: rspice_core::analysis::s_param::PortNoiseAssemblyError,
) -> rspice_core::engine::SimulationError {
    match error {
        rspice_core::analysis::s_param::PortNoiseAssemblyError::Aborted => {
            rspice_core::engine::SimulationError::Aborted
        }
        other => rspice_core::engine::SimulationError::Circuit(other.to_string()),
    }
}
