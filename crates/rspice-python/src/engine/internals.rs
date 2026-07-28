//! Private implementation behind the engine's public methods.
//!
//! These are the `*_impl` helpers each `#[pymethods]` entry point delegates to
//! once its arguments are validated, plus the configuration resolution that
//! decides which engine a given netlist actually runs on. Rust permits a second
//! inherent `impl` block for a type in a sibling module, which is what keeps the
//! public surface in `mod.rs` readable as an API contract rather than as
//! implementation.

use super::*;

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
    pub(super) fn resolve_node(
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
                        crate::errors::key_error(format!("unknown {what} node '{name}'"))
                    })
            }
        }
    }

    /// Core transient runner shared by `run_tran` and `run()`.
    pub(super) fn tran_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        stop_time: f64,
        max_step: f64,
        start_time: f64,
    ) -> PyResult<PyTransientResult> {
        let engine = self.engine_for_netlist(&netlist.inner);
        let result = run_interruptible(py, &self.active_runs, |abort| {
            engine.run_tran_with_abort(&netlist.inner, stop_time, max_step, abort)
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
        Ok(PyAcResult::new(frequencies, results))
    }

    /// Core distortion runner shared by the direct and deck APIs.
    pub(super) fn distortion_impl(
        &self,
        py: Python<'_>,
        netlist: &PyNetlist,
        frequencies: Vec<f64>,
        f2_over_f1: Option<f64>,
    ) -> PyResult<PyDistortionResult> {
        validate_distortion_arguments(&frequencies, f2_over_f1)?;
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
        let input_pos = self.resolve_node(&engine, &netlist.inner, input_pos, "PZ input+")?;
        let input_neg = input_neg
            .map(|node| self.resolve_node(&engine, &netlist.inner, node, "PZ input-"))
            .transpose()?;
        let output_pos = self.resolve_node(&engine, &netlist.inner, output_pos, "PZ output+")?;
        let output_neg = output_neg
            .map(|node| self.resolve_node(&engine, &netlist.inner, node, "PZ output-"))
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
        Ok(PyPoleZeroResult::from_core(&result))
    }

    pub(super) fn sensitivity_linearized_impl(
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
                self.resolve_node(&engine, &netlist.inner, output, "DC sensitivity output")?;
            let negative = reference
                .map(|node| {
                    self.resolve_node(&engine, &netlist.inner, node, "DC sensitivity reference")
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
                self.resolve_node(&engine, &netlist.inner, output, "AC sensitivity output")?;
            let negative = reference
                .map(|node| {
                    self.resolve_node(&engine, &netlist.inner, node, "AC sensitivity reference")
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

    #[allow(clippy::needless_range_loop)]
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
            let num_ports = ports.len();
            let num_points = frequencies.len();
            let zero = rspice_core::Complex64::new(0.0, 0.0);
            let mut admittances = vec![vec![vec![zero; num_points]; num_ports]; num_ports];
            for excited_port in 0..num_ports {
                if abort.is_aborted() {
                    return Err(rspice_core::engine::SimulationError::Aborted);
                }
                let mut excited = netlist.inner.clone();
                s_param::set_excitations(&mut excited, &ports, excited_port).map_err(|error| {
                    rspice_core::engine::SimulationError::Circuit(error.to_string())
                })?;
                let points = engine.run_ac_with_abort(&excited, &frequencies, abort)?;
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
                let matrix = s_param::s_from_y(&y, &impedances).map_err(|error| {
                    rspice_core::engine::SimulationError::Circuit(error.to_string())
                })?;
                for row in 0..num_ports {
                    for column in 0..num_ports {
                        scattering[row][column][frequency_index] = matrix[row][column];
                    }
                }
            }

            let noise = if do_noise {
                let points = engine.run_port_noise_correlation_with_abort(
                    &netlist.inner,
                    &port_names,
                    &frequencies,
                    temperature,
                    abort,
                )?;
                if points.len() != num_points {
                    return Err(rspice_core::engine::SimulationError::Circuit(format!(
                        "SP noise solve returned {} points for {num_points} requested frequencies",
                        points.len()
                    )));
                }
                let mut current_correlation =
                    vec![vec![vec![zero; num_points]; num_ports]; num_ports];
                let mut two_port_points = Vec::with_capacity(num_points);
                for (frequency_index, point) in points.iter().enumerate() {
                    if point.frequency.to_bits() != frequencies[frequency_index].to_bits() {
                        return Err(rspice_core::engine::SimulationError::Circuit(format!(
                            "SP noise frequency mismatch at point {frequency_index}: expected {}, got {}",
                            frequencies[frequency_index], point.frequency
                        )));
                    }
                    if point.current_correlation.len() != num_ports
                        || point
                            .current_correlation
                            .iter()
                            .any(|row| row.len() != num_ports)
                    {
                        return Err(rspice_core::engine::SimulationError::Circuit(format!(
                            "SP noise returned a malformed Cy matrix at frequency point {frequency_index}"
                        )));
                    }
                    for row in 0..num_ports {
                        for column in 0..num_ports {
                            current_correlation[row][column][frequency_index] =
                                point.current_correlation[row][column];
                        }
                    }
                    if num_ports == 2 {
                        let y = (0..num_ports)
                            .map(|row| {
                                (0..num_ports)
                                    .map(|column| admittances[row][column][frequency_index])
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>();
                        two_port_points.push(s_param::derive_two_port_noise(
                            &y,
                            &point.current_correlation,
                            impedances[0],
                            temperature,
                        ));
                    }
                }

                let (
                    noise_resistance,
                    noise_factor,
                    minimum_noise_factor,
                    optimum_source_reflection,
                    parameter_validity,
                ) = if num_ports == 2 {
                    (
                        Some(
                            two_port_points
                                .iter()
                                .map(|point| point.noise_resistance)
                                .collect(),
                        ),
                        Some(
                            two_port_points
                                .iter()
                                .map(|point| point.noise_factor)
                                .collect(),
                        ),
                        Some(
                            two_port_points
                                .iter()
                                .map(|point| point.minimum_noise_factor)
                                .collect(),
                        ),
                        Some(
                            two_port_points
                                .iter()
                                .map(|point| point.optimum_source_reflection)
                                .collect(),
                        ),
                        Some(two_port_points.iter().map(|point| point.valid).collect()),
                    )
                } else {
                    (None, None, None, None, None)
                };
                Some(SParameterNoiseData {
                    temperature,
                    current_correlation,
                    noise_resistance,
                    noise_factor,
                    minimum_noise_factor,
                    optimum_source_reflection,
                    parameter_validity,
                })
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
