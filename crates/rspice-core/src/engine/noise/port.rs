//! Port-noise preparation and frequency evaluation, shared by standalone Cy and SP.

use super::*;
use crate::analysis::s_param::{
    MaterializedRfPort, NetworkError, invert_complex_matrix_with_abort,
};
use crate::solver::{ComplexMatrix, StaticMatrix};

pub(in crate::engine) struct PreparedPortNoise {
    pub circuit: CircuitData,
    pub matrix: StaticMatrix,
    pub linearization: PortNoiseLinearization,
}

pub(in crate::engine) struct PortNoiseLinearization {
    pub bias: Vec<Value>,
    temperature: Value,
    dialect: crate::config::SpiceDialect,
    noise_sources: Vec<NoiseSource>,
    noise_temperatures: Vec<Option<Value>>,
    correlated_noise_sources: Vec<CorrelatedNoisePair>,
    runtime_devices: HashSet<String>,
    generated_devices: HashSet<String>,
    port_rhs: Vec<Complex64>,
    num_ports: usize,
    norton_nodes: Vec<(usize, usize)>,
    termination_devices: HashSet<String>,
}

pub(in crate::engine) struct PortNoiseWorkspace {
    ac_matrix: ComplexMatrix,
    port_adjoint: Vec<Complex64>,
    adjoint_column: Vec<Complex64>,
}

impl PreparedPortNoise {
    /// Refer SP noise to the DUT while retaining the physical port's DC bias.
    /// Standalone source-current noise deliberately keeps its original probes.
    pub(in crate::engine) fn use_sp_reference_planes(
        &mut self,
        netlist: &Netlist,
        ports: &[MaterializedRfPort],
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let ground = netlist.ground_policy();
        let node_id = |name: &str| {
            let name = ground.canonical_node(name);
            if name == "0" {
                return Ok(0);
            }
            self.circuit.get_node_by_name(name).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "SP noise reference-plane node '{name}' was not found"
                ))
            })
        };
        for resolved in ports {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let port = &resolved.port;
            self.linearization
                .norton_nodes
                .push((node_id(&port.node_pos)?, node_id(&port.node_neg)?));
            self.linearization
                .termination_devices
                .insert(resolved.termination.clone());
        }
        Ok(())
    }
}

impl Engine {
    /// Compute the short-circuit port-current noise correlation matrix.
    ///
    /// Every entry is a complex cross-power spectral density in A²/Hz using
    /// `E[I_i * conj(I_j)]`. Each named port must be an independent voltage
    /// source: its branch enforces zero small-signal voltage at its terminals.
    /// A source directly across the DUT observes its Norton current noise.
    /// A source behind a series resistor observes loaded current noise,
    /// including that resistor's noise. Use the SP runner to refer such a
    /// physical port to the DUT reference plane.
    pub fn run_port_noise_correlation(
        &self,
        netlist: &Netlist,
        port_sources: &[String],
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<PortNoiseCorrelationResult>, SimulationError> {
        self.run_port_noise_correlation_with_abort(
            netlist,
            port_sources,
            frequencies,
            temperature,
            &NoAbort,
        )
    }

    /// Compute `Cy` with cooperative cancellation.
    pub fn run_port_noise_correlation_with_abort(
        &self,
        netlist: &Netlist,
        port_sources: &[String],
        frequencies: &[Value],
        temperature: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<PortNoiseCorrelationResult>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        validate_port_noise_frequencies(frequencies)?;
        let engine = self.resolved_for_netlist(netlist);
        let run_scope = crate::abort_signal::ModelRunSignal::if_needed(abort);
        let abort: &dyn AbortSignal = run_scope.as_ref().map_or(abort, |scope| scope);
        let PreparedPortNoise {
            mut circuit,
            mut matrix,
            linearization,
        } = engine.prepare_port_noise_analysis(
            netlist,
            port_sources,
            frequencies.len(),
            temperature,
            abort,
        )?;
        let mut workspace = linearization.workspace(&matrix);
        let has_tasks = circuit.has_point_analog_tasks();
        let mut results = Vec::with_capacity(frequencies.len());
        for (index, &frequency) in frequencies.iter().enumerate() {
            let final_step = index + 1 == frequencies.len();
            let result = if has_tasks {
                let mut point_circuit = circuit.clone();
                Self::solve_accepted_frequency_point(
                    &mut point_circuit,
                    &mut matrix,
                    &linearization.bias,
                    super::super::analog_tasks::FrequencyModelPoint {
                        analysis: 3,
                        frequency,
                        final_step,
                    },
                    abort,
                    |point, final_step| {
                        linearization.solve(point, &mut workspace, frequency, final_step, abort)
                    },
                )?
            } else {
                linearization.solve(&mut circuit, &mut workspace, frequency, final_step, abort)?
            };
            results.push(result);
            if abort
                .model_control()
                .is_some_and(|control| control.is_finished())
            {
                break;
            }
        }
        Ok(results)
    }

    /// Prepare the noise-specific bias once, retaining its own model analysis identity.
    pub(in crate::engine) fn prepare_port_noise_analysis(
        &self,
        netlist: &Netlist,
        port_sources: &[String],
        frequency_count: usize,
        temperature: Value,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedPortNoise, SimulationError> {
        self.prepare_port_noise_circuit(
            netlist,
            None,
            port_sources,
            frequency_count,
            temperature,
            abort,
        )
    }

    /// Reuse an uninitialized elaborated circuit while retaining noise's own bias lifecycle.
    pub(in crate::engine) fn prepare_port_noise_circuit(
        &self,
        netlist: &Netlist,
        circuit: Option<CircuitData>,
        port_sources: &[String],
        frequency_count: usize,
        temperature: Value,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedPortNoise, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if port_sources.is_empty() {
            return Err(SimulationError::Circuit(
                "Port-noise analysis requires at least one voltage-source port".into(),
            ));
        }
        if !temperature.is_finite() || temperature <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Port-noise temperature must be finite and strictly positive, got {temperature}"
            )));
        }
        self.ensure_analysis_points(frequency_count)?;
        let mut unique_names = HashSet::with_capacity(port_sources.len());
        for source in port_sources {
            if source.trim().is_empty() {
                return Err(SimulationError::Circuit(
                    "Port-noise voltage-source names must not be empty".into(),
                ));
            }
            if !unique_names.insert(source.to_ascii_lowercase()) {
                return Err(SimulationError::Circuit(format!(
                    "Port-noise voltage source '{source}' is listed more than once"
                )));
            }
        }
        Self::ensure_model_run_active(abort)?;
        let mut circuit = match circuit {
            Some(circuit) => circuit,
            None => self.build_circuit_with_abort(netlist, abort)?,
        };
        Self::warn_xspice_mif_analysis_boundary(
            &circuit,
            "SP noise",
            "intrinsic XSPICE device-noise sources are not collected because ngspice MIF code models expose DEVnoise = NULL",
        );
        Self::ensure_no_mixed_signal_analysis(&circuit, "SP noise analysis")?;
        Self::ensure_supported_dynamic_charges(&circuit, "SP noise")?;
        if !circuit.ekv3s.is_empty() {
            return Err(SimulationError::unsupported_capability(
                "analysis.noise.device.ekv3_vanoise",
                "SP noise does not support the restricted EKV3 LEVEL=301 VANOISE oracle slice",
            ));
        }

        circuit
            .begin_veriloga_equilibrium_analysis(3)
            .map_err(SimulationError::Circuit)?;
        Self::deliver_initial_analog_tasks(&mut circuit, abort)?;
        circuit
            .prepare_veriloga_equilibrium_analysis_point(3, true, false)
            .map_err(SimulationError::Circuit)?;
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let dc_solution =
            self.solve_dc_operating_point_with_abort(netlist, &mut circuit, &mut matrix, abort)?;
        if circuit.has_nonlinear_devices() {
            self.try_observe_dc_operating_point(&mut circuit, &mut matrix, &dc_solution)?;
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let Some(message) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE evaluation failed: {message}"
            )));
        }
        self.accept_frequency_operating_point(
            netlist,
            &mut circuit,
            &mut matrix,
            &dc_solution,
            3,
            abort,
        )?;
        circuit
            .finish_veriloga_equilibrium_operating_point(3)
            .map_err(SimulationError::Circuit)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }
        circuit
            .prepare_behavioral_small_signal(&dc_solution)
            .map_err(SimulationError::Circuit)?;
        let CollectedNoiseSources {
            elementary: mut noise_sources,
            elementary_absolute_temperatures,
            correlated: mut correlated_noise_sources,
        } = Self::try_collect_noise_sources(&circuit, &dc_solution, self.config.spice_dialect)?;
        Self::configure_noise_physical_constants(
            &mut noise_sources,
            &mut correlated_noise_sources,
            self.config.spice_dialect,
        );
        let runtime_veriloga_device_names = Self::runtime_veriloga_device_names(&circuit);
        let generated_veriloga_device_names =
            Self::generated_grouped_veriloga_device_names(&circuit);
        let mut branch_matrix_indices = Vec::with_capacity(port_sources.len());
        for source_name in port_sources {
            let source_index = circuit
                .voltage_sources
                .names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(source_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Port-noise voltage source '{source_name}' was not found"
                    ))
                })?;
            let branch_ordinal = circuit.voltage_sources.branch_indices[source_index];
            let matrix_index = circuit.get_branch_matrix_index(branch_ordinal);
            if matrix_index == 0 || matrix_index > circuit.matrix_size() {
                return Err(SimulationError::Circuit(format!(
                    "Port-noise voltage source '{source_name}' has an invalid branch index"
                )));
            }
            branch_matrix_indices.push(matrix_index - 1);
        }

        let size = circuit.matrix_size();
        let num_ports = branch_matrix_indices.len();
        self.ensure_result_shape(
            frequency_count,
            num_ports
                .saturating_mul(num_ports)
                .saturating_mul(2)
                .saturating_add(1),
        )?;
        let zero = Complex64::new(0.0, 0.0);
        let mut port_rhs = vec![zero; size.saturating_mul(num_ports)];
        for (port, &branch_index) in branch_matrix_indices.iter().enumerate() {
            port_rhs[port * size + branch_index] = Complex64::new(-1.0, 0.0);
        }

        Ok(PreparedPortNoise {
            circuit,
            matrix,
            linearization: PortNoiseLinearization {
                bias: dc_solution,
                temperature,
                dialect: self.config.spice_dialect,
                noise_sources,
                noise_temperatures: elementary_absolute_temperatures,
                correlated_noise_sources,
                runtime_devices: runtime_veriloga_device_names,
                generated_devices: generated_veriloga_device_names,
                port_rhs,
                num_ports,
                norton_nodes: Vec::new(),
                termination_devices: HashSet::new(),
            },
        })
    }
}

pub(in crate::engine) fn validate_port_noise_frequencies(
    frequencies: &[Value],
) -> Result<(), SimulationError> {
    if frequencies.is_empty() {
        return Err(SimulationError::Circuit(
            "Port-noise analysis requires at least one frequency".into(),
        ));
    }
    if let Some(frequency) = frequencies
        .iter()
        .find(|frequency| !frequency.is_finite() || **frequency <= 0.0)
    {
        return Err(SimulationError::Circuit(format!(
            "Port-noise frequencies must be finite and strictly positive, got {frequency}"
        )));
    }
    Ok(())
}

impl PortNoiseLinearization {
    pub(in crate::engine) fn workspace(&self, matrix: &StaticMatrix) -> PortNoiseWorkspace {
        PortNoiseWorkspace {
            ac_matrix: ComplexMatrix::from_real_structure(matrix),
            port_adjoint: Vec::with_capacity(self.port_rhs.len()),
            adjoint_column: vec![Complex64::new(0.0, 0.0); self.num_ports],
        }
    }

    fn refer_adjoint_to_dut(
        &self,
        workspace: &mut PortNoiseWorkspace,
        frequency: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if self.norton_nodes.is_empty() {
            return Ok(());
        }
        // L observes currents through the loaded port generators. Injecting a
        // unit current at each DUT plane gives H = L B. Any DUT Norton source
        // therefore produces measured current H i_n; H^-1 L is the unloaded
        // Norton observation. Obtain H from the *noise* operator, since a
        // model can have different AC/noise conductance at the same bias.
        let size = self.bias.len();
        let mut transfer = Vec::with_capacity(self.num_ports);
        for adjoint in workspace.port_adjoint.chunks_exact(size) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            transfer.push(
                self.norton_nodes
                    .iter()
                    .map(|&(positive, negative)| {
                        Engine::noise_transfer_from_adjoint(adjoint, positive, negative)
                    })
                    .collect(),
            );
        }
        let inverse = invert_complex_matrix_with_abort(&transfer, abort)
            .map_err(|error| match error {
                NetworkError::Aborted => SimulationError::Aborted,
                other => SimulationError::Circuit(format!("SP noise reference-plane conversion failed at {frequency} Hz: {other}")),
            })?
            .ok_or_else(|| SimulationError::Circuit(format!(
                "SP noise reference planes have no finite Norton representation at {frequency} Hz"
            )))?;
        // Transform in place with only one saved column, avoiding a second
        // ports-by-unknowns allocation for every frequency and worker.
        for unknown in 0..size {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            for (port, value) in workspace.adjoint_column.iter_mut().enumerate() {
                *value = workspace.port_adjoint[port * size + unknown];
            }
            for (port, weights) in inverse.iter().enumerate() {
                let mut sum = Complex64::new(0.0, 0.0);
                let mut correction = Complex64::new(0.0, 0.0);
                for (&weight, &value) in weights.iter().zip(&workspace.adjoint_column) {
                    let term = weight * value;
                    crate::numerics::compensated_add(&mut sum.re, &mut correction.re, term.re);
                    crate::numerics::compensated_add(&mut sum.im, &mut correction.im, term.im);
                }
                let value = sum + correction;
                if !value.re.is_finite() || !value.im.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        "SP noise reference-plane conversion overflowed at {frequency} Hz"
                    )));
                }
                workspace.port_adjoint[port * size + unknown] = value;
            }
        }
        Ok(())
    }

    pub(in crate::engine) fn solve(
        &self,
        circuit: &mut CircuitData,
        workspace: &mut PortNoiseWorkspace,
        frequency: Value,
        final_step: bool,
        abort: &dyn AbortSignal,
    ) -> Result<PortNoiseCorrelationResult, SimulationError> {
        let zero = Complex64::new(0.0, 0.0);
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let omega = 2.0 * PI * frequency;
        circuit
            .prepare_veriloga_frequency_analysis_point(3, final_step)
            .map_err(SimulationError::Circuit)?;
        circuit
            .prepare_behavioral_small_signal_at_frequency(&self.bias, frequency)
            .map_err(SimulationError::Circuit)?;
        Engine::try_fill_small_signal_matrix_with_vbic_delay_mode(
            circuit,
            &mut workspace.ac_matrix,
            &self.bias,
            omega,
            super::super::ac::SmallSignalAnalysisKind::Noise,
            true,
            true,
        )?;
        // As in ordinary noise, a final_step event may feed a generated
        // or runtime Verilog-A noise expression. Refresh only the final
        // public point against the accepted initial state. These
        // numerical probes never accept model state or publish tasks.
        let final_noise_sources = if final_step && Engine::has_veriloga_noise_devices(circuit) {
            Some(Engine::try_refresh_veriloga_noise_sources(
                circuit,
                &self.bias,
                &self.noise_sources,
                &self.noise_temperatures,
                self.dialect,
            )?)
        } else {
            None
        };
        let point_noise_sources = final_noise_sources
            .as_ref()
            .map_or(self.noise_sources.as_slice(), |sources| {
                sources.0.as_slice()
            });
        let point_noise_temperatures = final_noise_sources
            .as_ref()
            .map_or(self.noise_temperatures.as_slice(), |sources| {
                sources.1.as_slice()
            });
        let point_correlated_noise_sources = self.correlated_noise_sources.as_slice();
        #[cfg(feature = "veriloga")]
        let point_veriloga_processes = Engine::try_collect_veriloga_noise_processes_at_frequency(
            circuit, &self.bias, frequency,
        )?;
        #[cfg(feature = "veriloga-builtins-base")]
        let point_generated_processes =
            Engine::try_collect_generated_veriloga_noise_processes_at_frequency(
                circuit, &self.bias, frequency,
            )?;
        let mut covariance = vec![vec![zero; self.num_ports]; self.num_ports];
        let mut compensation = vec![vec![zero; self.num_ports]; self.num_ports];

        // One adjoint solve per observed port replaces one forward solve
        // per device-noise source. Port count is normally tiny while a
        // transistor-level circuit can contain thousands of sources.
        match workspace.ac_matrix.solve_many_transpose_into(
            &self.port_rhs,
            self.num_ports,
            &mut workspace.port_adjoint,
        ) {
            Ok(()) => {}
            Err(crate::solver::SolverError::InaccurateSolution(_)) if self.bias.len() <= 64 => {
                log::debug!(
                    "sparse port-noise transpose solve failed strict backward-error certification; retrying the small complex systems with extended precision"
                );
                workspace.port_adjoint.clear();
                for port in 0..self.num_ports {
                    let start = port * self.bias.len();
                    let extended = workspace
                        .ac_matrix
                        .solve_dense_extended_transpose(
                            &self.port_rhs[start..start + self.bias.len()],
                        )
                        .map_err(SimulationError::Solver)?;
                    workspace.port_adjoint.extend_from_slice(&extended);
                }
            }
            Err(error) => return Err(SimulationError::Solver(error)),
        }

        self.refer_adjoint_to_dut(workspace, frequency, abort)?;
        let solve_transfer =
            |node_pos: usize, node_neg: usize| -> Result<Vec<Complex64>, SimulationError> {
                (0..self.num_ports)
                    .map(|port| {
                        let adjoint = &workspace.port_adjoint
                            [port * self.bias.len()..(port + 1) * self.bias.len()];
                        Ok(Engine::noise_transfer_from_adjoint(
                            adjoint, node_pos, node_neg,
                        ))
                    })
                    .collect()
            };

        debug_assert_eq!(point_noise_sources.len(), point_noise_temperatures.len());
        for (source, &absolute_temperature) in
            point_noise_sources.iter().zip(point_noise_temperatures)
        {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let device_name = source.identity.device.to_ascii_lowercase();
            if self.runtime_devices.contains(&device_name)
                || self.generated_devices.contains(&device_name)
                || self.termination_devices.contains(&device_name)
            {
                continue;
            }
            let source_temperature =
                Engine::elementary_noise_temperature(self.temperature, absolute_temperature);
            let density = Engine::evaluated_noise_density(source, frequency, source_temperature)?;
            if density == 0.0 {
                continue;
            }
            let scale = density.sqrt();
            let amplitude = solve_transfer(source.node_pos, source.node_neg)?
                .into_iter()
                .map(|gain| gain * scale)
                .collect::<Vec<_>>();
            Engine::add_port_noise_outer_product(&mut covariance, &mut compensation, &amplitude)?;
        }

        #[cfg(feature = "veriloga")]
        for (instance, process) in &point_veriloga_processes {
            let density = Engine::evaluated_veriloga_process_density(instance, process, frequency)?;
            if density == 0.0 {
                continue;
            }
            let mut amplitude_sums = (0..self.num_ports)
                .map(|_| ComplexBinAccumulator::default())
                .collect::<Vec<_>>();
            for injection in &process.injections {
                for (slot, transfer) in solve_transfer(injection.node_pos, injection.node_neg)?
                    .into_iter()
                    .enumerate()
                {
                    Engine::add_complex_bin(
                        &mut amplitude_sums[slot],
                        transfer * injection.gain,
                        &process.name,
                        frequency,
                    )?;
                }
            }
            let scale = density.sqrt();
            let amplitude = amplitude_sums
                .into_iter()
                .map(|sum| {
                    Engine::finish_complex_bins(sum, &process.name, frequency)
                        .map(|value| value * scale)
                })
                .collect::<Result<Vec<_>, _>>()?;
            Engine::add_port_noise_outer_product(&mut covariance, &mut compensation, &amplitude)?;
        }

        #[cfg(feature = "veriloga-builtins-base")]
        for (instance, process) in &point_generated_processes {
            let density =
                Engine::evaluated_generated_process_density(instance, process, frequency)?;
            if density == 0.0 {
                continue;
            }
            let scale = density.sqrt();
            let amplitude = (0..self.num_ports)
                .map(|port| {
                    Engine::generated_process_transfer_from_adjoint(
                        &workspace.port_adjoint
                            [port * self.bias.len()..(port + 1) * self.bias.len()],
                        process,
                        frequency,
                    )
                    .map(|value| value * scale)
                })
                .collect::<Result<Vec<_>, _>>()?;
            Engine::add_port_noise_outer_product(&mut covariance, &mut compensation, &amplitude)?;
        }

        for source in point_correlated_noise_sources {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let Some(densities) =
                Engine::evaluated_correlated_noise_densities(source, frequency, self.temperature)?
            else {
                continue;
            };
            let first = solve_transfer(source.first.node_pos, source.first.node_neg)?;
            let second = solve_transfer(source.second.node_pos, source.second.node_neg)?;
            let first_scale = densities.first_psd.sqrt();
            let second_scale =
                Complex64::from_polar(densities.second_psd.sqrt(), densities.phase_rad);
            let amplitude = first
                .into_iter()
                .zip(second)
                .map(|(first_gain, second_gain)| {
                    first_gain * first_scale + second_gain * second_scale
                })
                .collect::<Vec<_>>();
            Engine::add_port_noise_outer_product(&mut covariance, &mut compensation, &amplitude)?;
        }

        // Make the mathematical Hermitian invariant exact in the public
        // result and remove only impossible signed zero on its diagonal.
        // Hermitian symmetrization writes `[row][column]` and its
        // transpose together, which are in different rows.
        #[allow(clippy::needless_range_loop)]
        for row in 0..self.num_ports {
            covariance[row][row] = Complex64::new(covariance[row][row].re.max(0.0), 0.0);
            for column in (row + 1)..self.num_ports {
                let value = (covariance[row][column] + covariance[column][row].conj()) * 0.5;
                covariance[row][column] = value;
                covariance[column][row] = value.conj();
            }
        }

        Ok(PortNoiseCorrelationResult {
            frequency,
            current_correlation: covariance,
        })
    }
}
