//! The `.SP` runner: one scattering sweep, and optionally its port noise.
//!
//! Each frequency shares its AC bias and factorization across port drives.
//! Model completion accepts all scattering columns and optional port noise
//! together, including any required final-step re-evaluation.

use super::ac::{AcExcitation, PreparedAc};
use super::analog_tasks::FrequencyModelState;
use super::noise::{PreparedPortNoise, validate_port_noise_frequencies};
use crate::abort_signal::AbortSignal;
use crate::analysis::s_param::{
    MaterializedRfPort, NetworkError, PortError, PortNoiseAssembly, PortNoiseAssemblyError,
    SMatrix, SParameterPort, SParameterResult, assemble_port_noise_with_abort,
    s_column_from_port_voltages,
};
use crate::netlist::AnalysisCommand;
use crate::solver::ComplexMatrix;
use crate::{Complex64, Netlist, Value};

use super::{Engine, SimulationError};

/// One authored `.SP` card's complete typed result.
///
/// The scattering sweep and the port-noise sweep are separate published
/// documents with separate result families, so they are returned side by side
/// rather than folded into one.
#[derive(Debug, Clone)]
pub struct SParameterRun {
    /// Scattering parameters over the swept grid, in the exact shape the
    /// shared S-parameter document accepts.
    pub scattering: SParameterResult,
    /// The elaborated ports, in port order, with the qualified source names and
    /// reference-impedance realizations the sweep drove them through.
    pub ports: Vec<SParameterPort>,
    /// Port-noise evidence, present only when the card requested it.
    pub port_noise: Option<PortNoiseAssembly>,
}

impl Engine {
    /// Run one authored `.SP` card.
    ///
    /// The card supplies the frequency grid and whether port noise is
    /// requested; the deck's `portnum=` annotations supply the ports. Two-port
    /// noise parameters that are not physical are a typed failure, not a
    /// published placeholder: a noise figure that is present but meaningless
    /// will be believed.
    pub fn run_sp_with_abort(
        &self,
        netlist: &Netlist,
        card: &AnalysisCommand,
        abort: &dyn AbortSignal,
    ) -> Result<SParameterRun, SimulationError> {
        let AnalysisCommand::Sp {
            variation,
            points,
            start_freq,
            stop_freq,
            do_noise,
        } = card
        else {
            return Err(SimulationError::Netlist(
                "run_sp_with_abort was given a card that is not .SP".to_owned(),
            ));
        };
        let frequencies = card_frequency_grid(*variation, *points, *start_freq, *stop_freq, abort)?;
        self.run_sp_over_grid_with_abort(netlist, &frequencies, *do_noise, abort)
    }

    /// Run a scattering sweep over an explicit frequency grid.
    ///
    /// The card-driven entry point resolves the grid and calls this; a caller
    /// that already has a grid — a `.LIN`-style Touchstone export, a study
    /// sweeping something else — uses it directly.
    pub fn run_sp_over_grid_with_abort(
        &self,
        netlist: &Netlist,
        frequencies: &[Value],
        do_noise: bool,
        abort: &dyn AbortSignal,
    ) -> Result<SParameterRun, SimulationError> {
        self.run_sp_over_grid_with_default_ports_and_abort(
            netlist,
            frequencies,
            do_noise,
            &[],
            abort,
        )
    }

    /// Run SP, using configured planes only if the elaborated circuit declares no RF ports.
    /// Authored ports take precedence, including scoped and hierarchical declarations.
    /// Port discovery, configured helper insertion and device construction share one
    /// elaboration and statistical sequence. The returned ports are authoritative.
    pub fn run_sp_over_grid_with_default_ports_and_abort(
        &self,
        netlist: &Netlist,
        frequencies: &[Value],
        do_noise: bool,
        default_ports: &[crate::analysis::s_param::Port],
        abort: &dyn AbortSignal,
    ) -> Result<SParameterRun, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if frequencies.is_empty() {
            return Err(SimulationError::Netlist(
                ".SP requires at least one sweep frequency".to_owned(),
            ));
        }
        super::ac::validate_ac_frequencies(frequencies)?;
        if do_noise {
            validate_port_noise_frequencies(frequencies)?;
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_analysis_points(frequencies.len())?;
        let run_scope = crate::abort_signal::ModelRunSignal::if_needed(abort);
        let abort: &dyn AbortSignal = run_scope.as_ref().map_or(abort, |scope| scope);
        Self::ensure_model_run_active(abort)?;
        let (circuit, rf_ports) =
            engine.build_circuit_with_rf_ports(netlist, default_ports, abort)?;
        if rf_ports.is_empty() {
            return Err(PortError::NoPortsDeclared.into());
        }
        let ports = rf_ports
            .iter()
            .map(|resolved| resolved.port.clone())
            .collect::<Vec<_>>();
        let count = ports.len();
        engine.ensure_result_shape(
            frequencies.len(),
            count
                .saturating_mul(count)
                .saturating_mul(2)
                .saturating_add(1),
        )?;
        // Both analyses use the same physical port circuit. Inserting Z0 only
        // for AC would bias nonlinear devices differently in the noise solve.
        // Each analysis still retains its own model phase and accepted state.
        let noise_circuit = do_noise.then(|| circuit.clone());
        let PreparedAc {
            circuit: ac_bias,
            mut matrix,
            linearization,
            excitation: _,
        } = engine.prepare_ac_circuit(netlist, circuit, abort)?;
        engine.ensure_result_shape(
            frequencies.len(),
            ac_bias.matrix_size().saturating_mul(2).saturating_add(1),
        )?;
        let excitations = ports
            .iter()
            .map(|port| AcExcitation::for_port(&ac_bias, &port.source_name))
            .collect::<Result<Vec<_>, _>>()?;
        let ground = netlist.ground_policy();
        let node_id = |name: &str| {
            let name = ground.canonical_node(name);
            if name == "0" {
                return Ok(0);
            }
            ac_bias.get_node_by_name(name).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "SP reference-plane node '{name}' is not in the solved circuit"
                ))
            })
        };
        let nodes = ports
            .iter()
            .map(|port| Ok((node_id(&port.node_pos)?, node_id(&port.node_neg)?)))
            .collect::<Result<Vec<_>, SimulationError>>()?;
        let impedances = ports.iter().map(|port| port.z0).collect::<Vec<_>>();
        // Hierarchical multiplicity can change a physical port resistor. Read
        // the realized AC termination, then express the measured waves at the
        // authored reference; changing the resistor would also change bias.
        let terminations = rf_ports
            .iter()
            .map(|port| termination_impedance(&ac_bias, port, abort))
            .collect::<Result<Vec<_>, _>>()?;
        let renormalize = terminations != impedances;
        let reference_impedance = impedances[0];
        let temperature = netlist.options.temp.map_or(
            engine.config().temperature,
            crate::constants::celsius_to_kelvin,
        );
        let (noise_bias, mut noise_matrix, noise_linearization) =
            if let Some(circuit) = noise_circuit {
                let names = ports
                    .iter()
                    .map(|port| port.source_name.clone())
                    .collect::<Vec<_>>();
                let mut prepared = engine.prepare_port_noise_circuit(
                    netlist,
                    Some(circuit),
                    &names,
                    frequencies.len(),
                    temperature,
                    abort,
                )?;
                prepared.use_sp_reference_planes(netlist, &rf_ports, abort)?;
                let PreparedPortNoise {
                    circuit,
                    matrix,
                    linearization,
                } = prepared;
                (Some(circuit), Some(matrix), Some(linearization))
            } else {
                (None, None, None)
            };
        let has_tasks = ac_bias.has_point_analog_tasks()
            || noise_bias
                .as_ref()
                .is_some_and(|circuit| circuit.has_point_analog_tasks());
        let solve_point =
            |ac_point: &mut crate::CircuitData,
             ac_workspace: &mut ComplexMatrix,
             noise_point: Option<(
                &mut crate::CircuitData,
                &mut super::noise::PortNoiseWorkspace,
            )>,
             frequency: Value,
             final_step: bool|
             -> Result<(SMatrix, Option<PortNoiseAssembly>), SimulationError> {
                linearization.prepare_frequency(
                    ac_point,
                    ac_workspace,
                    frequency,
                    final_step,
                    abort,
                )?;
                let mut matrix = SMatrix::new(frequency, count);
                let mut voltages = Vec::with_capacity(count);
                for (column, excitation) in excitations.iter().enumerate() {
                    if abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    let solution = linearization.solve(ac_workspace, excitation, abort)?;
                    let voltage = |node: usize| -> Result<Complex64, SimulationError> {
                        if node == 0 {
                            return Ok(Complex64::new(0.0, 0.0));
                        }
                        solution.get(node - 1).copied().ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "SP reference-plane node {node} is outside the solved system"
                            ))
                        })
                    };
                    voltages.clear();
                    for &(positive, negative) in &nodes {
                        voltages.push(voltage(positive)? - voltage(negative)?);
                    }
                    let values = s_column_from_port_voltages(&voltages, column, &terminations)
                        .map_err(map_wave_error)?;
                    for (row, value) in values.into_iter().enumerate() {
                        matrix.set(row + 1, column + 1, value);
                    }
                }
                if renormalize {
                    matrix
                        .renormalize_with_abort(&terminations, &impedances, abort)
                        .map_err(map_wave_error)?;
                }
                let noise = if let Some((solver, (circuit, workspace))) =
                    noise_linearization.as_ref().zip(noise_point)
                {
                    let point = solver.solve(circuit, workspace, frequency, final_step, abort)?;
                    // Derive and validate noise parameters before model control
                    // can be published for this point.
                    Some(
                        assemble_port_noise_with_abort(
                            &ports,
                            std::slice::from_ref(&matrix),
                            count,
                            vec![point],
                            temperature,
                            abort,
                        )
                        .map_err(map_port_noise_error)?,
                    )
                } else {
                    None
                };
                Ok((matrix, noise))
            };

        // All excitations at a frequency share one factorization. Independent
        // frequencies still use the configured pool when no model tasks can
        // terminate the run. Each worker retains its own numerical caches.
        #[cfg(feature = "parallel")]
        let parallel_points = {
            let workers = engine.parallel_worker_count(frequencies.len());
            if !has_tasks && frequencies.len() >= 10 && workers > 1 {
                use rayon::prelude::*;
                let chunk_len = frequencies.len().div_ceil(workers);
                let work = frequencies
                    .chunks(chunk_len)
                    .enumerate()
                    .map(|(chunk_index, chunk)| {
                        let noise = noise_bias
                            .as_ref()
                            .zip(noise_matrix.as_ref())
                            .zip(noise_linearization.as_ref())
                            .map(|((bias, matrix), solver)| {
                                (bias.clone(), solver.workspace(matrix))
                            });
                        (
                            ac_bias.clone(),
                            ComplexMatrix::from_real_structure(&matrix),
                            noise,
                            chunk_index * chunk_len,
                            chunk,
                        )
                    })
                    .collect::<Vec<_>>();
                let chunks = engine.install_parallel(|| {
                    work.into_par_iter()
                        .map(|(mut circuit, mut workspace, mut noise, start, chunk)| {
                            chunk
                                .iter()
                                .enumerate()
                                .map(|(offset, &frequency)| {
                                    solve_point(
                                        &mut circuit,
                                        &mut workspace,
                                        noise
                                            .as_mut()
                                            .map(|(circuit, workspace)| (circuit, workspace)),
                                        frequency,
                                        start + offset + 1 == frequencies.len(),
                                    )
                                })
                                .collect::<Result<Vec<_>, SimulationError>>()
                        })
                        .collect::<Result<Vec<_>, SimulationError>>()
                })??;
                Some(chunks.into_iter().flatten().collect::<Vec<_>>())
            } else {
                None
            }
        };
        #[cfg(not(feature = "parallel"))]
        let parallel_points: Option<Vec<(SMatrix, Option<PortNoiseAssembly>)>> = None;

        let points = if let Some(points) = parallel_points {
            points
        } else {
            let mut ac_workspace = ComplexMatrix::from_real_structure(&matrix);
            let mut noise_workspace = noise_linearization
                .as_ref()
                .zip(noise_matrix.as_ref())
                .map(|(linearization, matrix)| linearization.workspace(matrix));
            let mut ac_point = ac_bias.clone();
            let mut noise_point = noise_bias.clone();
            let mut points = Vec::with_capacity(frequencies.len());
            for (index, &frequency) in frequencies.iter().enumerate() {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let final_step = index + 1 == frequencies.len();
                let point = if has_tasks {
                    if index > 0 {
                        ac_point.clone_from(&ac_bias);
                        noise_point.clone_from(&noise_bias);
                    }
                    let mut states = vec![FrequencyModelState {
                        circuit: &mut ac_point,
                        matrix: &mut matrix,
                        bias: &linearization.bias,
                        analysis: 1,
                    }];
                    if let (Some(circuit), Some(matrix), Some(linearization)) = (
                        noise_point.as_mut(),
                        noise_matrix.as_mut(),
                        noise_linearization.as_ref(),
                    ) {
                        states.push(FrequencyModelState {
                            circuit,
                            matrix,
                            bias: &linearization.bias,
                            analysis: 3,
                        });
                    }
                    Self::solve_accepted_frequency_group(
                        &mut states,
                        frequency,
                        final_step,
                        abort,
                        |states, final_step| {
                            let (ac, noise) = states.split_at_mut(1);
                            solve_point(
                                ac[0].circuit,
                                &mut ac_workspace,
                                noise
                                    .first_mut()
                                    .map(|state| &mut *state.circuit)
                                    .zip(noise_workspace.as_mut()),
                                frequency,
                                final_step,
                            )
                        },
                    )?
                } else {
                    solve_point(
                        &mut ac_point,
                        &mut ac_workspace,
                        noise_point.as_mut().zip(noise_workspace.as_mut()),
                        frequency,
                        final_step,
                    )?
                };
                points.push(point);
                if abort
                    .model_control()
                    .is_some_and(|control| control.is_finished())
                {
                    break;
                }
            }
            points
        };
        let mut scattering = SParameterResult::new(
            reference_impedance,
            ports
                .iter()
                .map(|port| crate::analysis::s_param::Port {
                    number: port.number,
                    node_pos: port.node_pos.clone(),
                    node_neg: port.node_neg.clone(),
                    z0: port.z0,
                })
                .collect(),
        );
        let mut port_noise: Option<PortNoiseAssembly> = None;
        for (point, noise) in points {
            scattering.add(point);
            if let Some(mut point_noise) = noise {
                if let Some(all) = &mut port_noise {
                    all.points.append(&mut point_noise.points);
                    if let (Some(all), Some(point)) = (&mut all.two_port, &mut point_noise.two_port)
                    {
                        all.append(point);
                    }
                } else {
                    port_noise = Some(point_noise);
                }
            }
        }

        Ok(SParameterRun {
            scattering,
            ports,
            port_noise,
        })
    }
}

/// The frequency grid one authored card's sweep specification describes.
///
/// Cancellation during grid construction is a cancelled run, not a malformed
/// card, so the two failures stay distinguishable to the caller.
pub(crate) fn card_frequency_grid(
    variation: crate::netlist::FreqVariation,
    points: usize,
    start: Value,
    stop: Value,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    crate::analysis::ac::try_ac_sweep_frequencies_with_abort(variation, points, start, stop, abort)
        .map_err(|error| match error {
            crate::analysis::frequency_grid::FrequencyGridError::Aborted => {
                SimulationError::Aborted
            }
            other => SimulationError::Netlist(other.to_string()),
        })
}

/// Read the termination from the same small-signal storage used by AC stamping.
pub(super) fn termination_impedance(
    circuit: &crate::CircuitData,
    port: &MaterializedRfPort,
    abort: &dyn AbortSignal,
) -> Result<Value, SimulationError> {
    let validate = |resistance: Value| {
        if resistance.is_finite() && resistance > 0.0 {
            Ok(resistance)
        } else {
            Err(SimulationError::Circuit(format!(
                "SP port '{}' has invalid realized termination resistance {resistance}",
                port.port.source_name
            )))
        }
    };
    for (index, name) in circuit.resistors.names.iter().enumerate() {
        if index % 256 == 0 && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if name.eq_ignore_ascii_case(&port.termination) {
            let conductance = circuit.resistors.small_signal_conductance(index);
            // Preserve the exact reference when it produced this same stamp;
            // reciprocal rounding alone must not trigger a dense conversion.
            return validate(if conductance == port.port.z0.recip() {
                port.port.z0
            } else {
                conductance.recip()
            });
        }
    }
    for (index, name) in circuit.resistor_branches.names.iter().enumerate() {
        if index % 256 == 0 && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if name.eq_ignore_ascii_case(&port.termination) {
            return validate(circuit.resistor_branches.small_signal_resistances[index]);
        }
    }
    Err(SimulationError::Circuit(format!(
        "SP port '{}' is missing its realized termination '{}'",
        port.port.source_name, port.termination
    )))
}

fn map_wave_error(error: NetworkError) -> SimulationError {
    match error {
        NetworkError::Aborted => SimulationError::Aborted,
        error => SimulationError::Circuit(format!(".SP wave conversion failed: {error}")),
    }
}

/// Preserve cancellation caught inside port-noise assembly.
fn map_port_noise_error(error: PortNoiseAssemblyError) -> SimulationError {
    match error {
        PortNoiseAssemblyError::Aborted => SimulationError::Aborted,
        other => SimulationError::Circuit(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::abort_signal::{ImmediateAbort, NoAbort};
    use crate::engine::{Engine, SimulationConfig, SimulationError};
    use crate::netlist::{AnalysisCommand, Netlist};

    const TWO_PORT: &str = "Two-port attenuator\n\
         V1 p1 0 AC 1 portnum=1 z0=50\n\
         V2 p2 0 AC 0 portnum=2 z0=50\n\
         R1 p1 mid 25\n\
         R2 mid 0 50\n\
         R3 mid p2 25\n\
         .sp lin 3 1meg 3meg\n\
         .end\n";

    fn sp_card(netlist: &Netlist) -> AnalysisCommand {
        netlist
            .analyses
            .iter()
            .find(|command| matches!(command, AnalysisCommand::Sp { .. }))
            .expect("the deck authors a .SP card")
            .clone()
    }

    #[test]
    fn the_sp_runner_produces_a_full_scattering_sweep() {
        let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let run = engine
            .run_sp_with_abort(&netlist, &sp_card(&netlist), &NoAbort)
            .expect(".SP runs");
        assert_eq!(run.scattering.num_ports, 2);
        assert_eq!(run.scattering.data.len(), 3);
        assert_eq!(run.ports.len(), 2);
        assert_eq!(run.scattering.frequencies(), vec![1.0e6, 2.0e6, 3.0e6]);
        for matrix in &run.scattering.data {
            for row in 1..=2 {
                for column in 1..=2 {
                    let value = matrix.get(row, column);
                    assert!(
                        value.re.is_finite() && value.im.is_finite(),
                        "S({row},{column}) at {} Hz is not finite",
                        matrix.frequency
                    );
                }
            }
        }
        assert!(run.port_noise.is_none(), "the card did not request noise");
    }

    #[test]
    fn a_resistive_pad_is_reciprocal_and_matched() {
        let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let run = engine
            .run_sp_with_abort(&netlist, &sp_card(&netlist), &NoAbort)
            .expect(".SP runs");
        // A symmetric resistive pad is reciprocal (S21 == S12) and symmetric
        // (S11 == S22) at every frequency; both are properties of the network,
        // not of the extraction, so they check the runner end to end.
        for matrix in &run.scattering.data {
            assert!(
                (matrix.s21() - matrix.s12()).norm() < 1.0e-9,
                "a resistive pad must be reciprocal at {} Hz",
                matrix.frequency
            );
            assert!(
                (matrix.s11() - matrix.s22()).norm() < 1.0e-9,
                "a symmetric pad must have equal reflections at {} Hz",
                matrix.frequency
            );
        }
    }

    #[test]
    fn the_sp_runner_carries_port_noise_when_the_card_asks_for_it() {
        let netlist =
            Netlist::parse(&TWO_PORT.replace(".sp lin 3 1meg 3meg", ".sp lin 3 1meg 3meg 1"))
                .expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let run = engine
            .run_sp_with_abort(&netlist, &sp_card(&netlist), &NoAbort)
            .expect(".SP DONOISE runs");
        let sweep_points = run.scattering.data.len();
        let noise = run.port_noise.expect("the card requested port noise");
        assert_eq!(noise.points.len(), sweep_points);
        let two_port = noise
            .two_port
            .expect("a two-port network has noise figures");
        assert_eq!(two_port.len(), sweep_points);
        for parameters in two_port {
            assert!(parameters.valid, "an assembled figure is always physical");
            assert!(parameters.noise_factor >= 1.0 - 1.0e-9);
        }
    }

    #[test]
    fn a_deck_with_no_declared_ports_fails_before_any_solve() {
        let netlist = Netlist::parse(
            "No ports\n\
             V1 in 0 AC 1\n\
             R1 in 0 50\n\
             .sp lin 2 1meg 2meg\n\
             .end\n",
        )
        .expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let error = engine
            .run_sp_with_abort(&netlist, &sp_card(&netlist), &NoAbort)
            .expect_err("a deck with no portnum annotations must fail closed");
        assert!(
            error.to_string().contains("portnum"),
            "the failure names what the deck is missing: {error}"
        );
    }

    #[test]
    fn the_sp_runner_honours_its_abort_source() {
        let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let error = engine
            .run_sp_with_abort(&netlist, &sp_card(&netlist), &ImmediateAbort)
            .expect_err("an aborted .SP must not produce a sweep");
        assert!(matches!(error, SimulationError::Aborted), "{error}");
    }

    #[test]
    fn a_card_that_is_not_sp_is_refused() {
        let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        engine
            .run_sp_with_abort(&netlist, &AnalysisCommand::Op, &NoAbort)
            .expect_err("only a .SP card selects the .SP runner");
    }
}
