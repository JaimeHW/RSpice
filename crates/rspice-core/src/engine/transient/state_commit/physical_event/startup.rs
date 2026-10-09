//! DC/IC storage to outgoing transient state at exactly time zero.
//! Preparation reads the selected device histories, not the waveform's
//! nominal incoming side. All fallible work precedes publication.

use super::*;
use crate::engine::convergence::{
    AcceptedTransientOperatingPointContract, TransientOperatingPointLinearSystem,
};

/// An unconstrained operating point still belongs to the outgoing equations.
/// Construction checks source-side and conditioning identity; the event solve
/// independently audits charge, algebraic constraints and finite-current KCL.
pub(in crate::engine::transient) struct OperatingPointStartup {
    pub(super) stationary: bool,
}

#[cfg(test)]
#[path = "startup_contract_tests.rs"]
mod contract_tests;

fn constant_source(spec: &crate::netlist::SourceSpec) -> bool {
    use crate::netlist::SourceSpec as S;
    match spec {
        S::Dc(_) | S::Ac { .. } | S::DcAc { .. } => true,
        S::Distortion { inner, .. } => constant_source(inner),
        S::DcTransient { transient, .. }
        | S::AcTransient { transient, .. }
        | S::DcAcTransient { transient, .. } => constant_source(transient),
        // RF annotations can add a transient tone. Noise and even apparently
        // flat sampled waveforms need their own forcing proof.
        _ => false,
    }
}

impl Engine {
    fn physical_startup_operating_point(
        &self,
        circuit: &crate::CircuitData,
        contract: Option<AcceptedTransientOperatingPointContract>,
        options: &charge_event::EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Option<OperatingPointStartup>, SimulationError> {
        let Some(contract) = contract else {
            return Ok(None);
        };
        if contract.linear_system != TransientOperatingPointLinearSystem::IdealInductorShorts
            // A line owns incoming waves as well as electrical coordinates;
            // an OP certificate alone does not authenticate that history.
            || !circuit.tlines.is_empty()
            // An IC capacitor's voltage constraint is released into a
            // dynamic current equation. Its bias is not a static balance.
            || circuit.capacitors.ic_branch_indices.iter().any(Option::is_some)
            || !circuit.behavioral_sources.has_smooth_physical_equations()
            || contract.nodal_gmin != options.nodal_gmin
            || contract.junction_gmin.is_some_and(|gmin| {
                gmin != self.effective_device_junction_gmin(self.config.convergence_config.gmin_target)
            })
        {
            return Ok(None);
        }
        let voltage = &circuit.voltage_sources;
        let current = &circuit.current_sources;
        // Compare the authored forcing itself, not its residual after it is
        // summed with other contributions: a tiny source jump remains real.
        for i in 0..voltage.len().max(current.len()) {
            if i.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if (i < voltage.len()
                && voltage.transient_value_at_on_side(i, 0.0, SourceTimeSide::Published)
                    != voltage.transient_value_at_on_side(i, 0.0, SourceTimeSide::RightLimit))
                || (i < current.len()
                    && current.value_at_time_on_side(i, 0.0, SourceTimeSide::Published)
                        != current.value_at_time_on_side(i, 0.0, SourceTimeSide::RightLimit))
            {
                return Ok(None);
            }
        }
        let mut stationary = circuit
            .behavioral_sources
            .voltage_sources
            .iter()
            .all(|source| source.physical_time_is_stationary())
            && circuit
                .behavioral_sources
                .current_sources
                .iter()
                .all(|source| source.physical_time_is_stationary());
        for (index, spec) in voltage
            .source_specs
            .iter()
            .chain(&current.source_specs)
            .enumerate()
        {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            stationary &= spec.as_ref().is_none_or(constant_source);
        }
        Ok(Some(OperatingPointStartup { stationary }))
    }
}

pub(super) struct StartupSeed {
    pub charges: Vec<Value>,
    pub inputs: Vec<Option<Value>>,
}

fn aligned(count: usize, lengths: &[usize]) -> Result<(), SimulationError> {
    if lengths.iter().any(|length| *length != count) {
        Err(failure("unaligned physical startup history"))
    } else {
        Ok(())
    }
}

/// The same authored C/L/mutual products feed finite and distributional
/// startup owners. Rows use the circuit's one-based, ground-aware convention.
pub(super) fn linear_storage_terms<'a>(
    circuit: &'a crate::CircuitData,
    currents: &'a [Value],
) -> impl Iterator<Item = (usize, Value, Value)> + 'a {
    let c = &circuit.capacitors;
    let l = &circuit.inductors;
    let nodes = circuit.num_nodes();
    let caps = c.stamps.iter().enumerate().flat_map(move |(i, stamp)| {
        if let Some(ordinal) = c.ic_branch_indices[i] {
            [
                (nodes + ordinal, -c.capacitances[i], c.v_prev[i]),
                (0, 0.0, 0.0),
            ]
        } else {
            [
                (stamp.pp.row, c.capacitances[i], c.v_prev[i]),
                (stamp.nn.row, -c.capacitances[i], c.v_prev[i]),
            ]
        }
    });
    let windings = l
        .branch_indices
        .iter()
        .enumerate()
        .map(move |(i, &ordinal)| (nodes + ordinal, -l.inductances[i], l.i_prev[i]));
    let mutual = circuit.coupled_inductor_pairs.iter().flat_map(move |pair| {
        let a = nodes + pair.branch1_ordinal;
        let b = nodes + pair.branch2_ordinal;
        [
            (a, -pair.device.m, currents[b - 1]),
            (b, -pair.device.m, currents[a - 1]),
        ]
    });
    caps.chain(windings)
        .chain(mutual)
        .filter(|&(row, _, _)| row != 0)
}

pub(super) fn startup_winding_currents(
    circuit: &crate::CircuitData,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let c = &circuit.capacitors;
    let l = &circuit.inductors;
    aligned(
        c.len(),
        &[
            c.v_prev.len(),
            c.v_prev_prev.len(),
            c.v_prev_prev_prev.len(),
            c.i_prev.len(),
        ],
    )?;
    aligned(
        l.len(),
        &[
            l.i_prev.len(),
            l.i_prev_prev.len(),
            l.i_prev_prev_prev.len(),
            l.v_prev.len(),
        ],
    )?;
    let mut currents = Vec::new();
    currents
        .try_reserve_exact(circuit.matrix_size())
        .map_err(|source| SimulationError::Allocation {
            object: "initial winding current coordinates",
            source,
        })?;
    currents.resize(circuit.matrix_size(), 0.0);
    for (index, &ordinal) in l.branch_indices.iter().enumerate() {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        currents[circuit.num_nodes() + ordinal - 1] = l.i_prev[index];
    }
    Ok(currents)
}

pub(super) fn seed(
    circuit: &crate::CircuitData,
    history: &BjtTransientHistory,
    diode_history: &DiodeTransientHistory,
    sampler: &PreparedEventCircuit<'_>,
    abort: &dyn AbortSignal,
) -> Result<StartupSeed, SimulationError> {
    aligned(
        circuit.bjts.len(),
        &[
            history.phase.len(),
            history.phase_outgoing_slopes.len(),
            history.vbe_prev.len(),
            history.vbe_prev_prev.len(),
            history.ibe_prev.len(),
            history.vbc_prev.len(),
            history.vbc_prev_prev.len(),
            history.ibc_prev.len(),
            history.vcs_prev.len(),
            history.vcs_prev_prev.len(),
            history.ics_prev.len(),
            history.charge_q_prev.len(),
            history.charge_q_prev_prev.len(),
            history.charge_q_prev_prev_prev.len(),
            history.charge_cq_prev.len(),
            history.accepted_external_bc_current.len(),
            history.accepted_terminal_currents.len(),
            history.dynamic_internal_prev.len(),
            history.dynamic_internal_prev_prev.len(),
            history.dynamic_linear_prev.len(),
            history.dynamic_linear_prev_prev.len(),
        ],
    )?;
    let currents = startup_winding_currents(circuit, abort)?;
    let mut charges = vec![0.0; circuit.matrix_size()];
    let mut add = |row: usize, value: Value| -> Result<(), SimulationError> {
        if !value.is_finite() {
            return Err(failure("nonfinite initial charge or flux"));
        }
        if row != 0 {
            charges[row - 1] = sum([(charges[row - 1], 1.0), (value, 1.0)].into_iter())?;
        }
        Ok(())
    };
    for (row, coefficient, coordinate) in linear_storage_terms(circuit, &currents) {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        add(row, sum([(coefficient, coordinate)].into_iter())?)?;
    }
    diodes::validate_history(circuit, diode_history)?;
    for (diode, &charge) in circuit.diodes.devices.iter().zip(&diode_history.qd_prev) {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        add(diode.node_anode, charge)?;
        add(diode.node_cathode, -charge)?;
    }
    let mut inputs = Vec::with_capacity(sampler.models().len());
    for (index, model) in sampler.models().iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if history.dynamic_internal_prev[index]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err(failure("nonfinite initial BJT coordinates"));
        }
        // These are the same per-branch incidences used to prepare the jump
        // topology, including an externalized XCJC base/collector charge.
        for (branch, (port, &charge)) in model
            .charge_storage_nodes()
            .iter()
            .zip(&history.charge_q_prev[index])
            .enumerate()
        {
            if !charge.is_finite() {
                return Err(failure("nonfinite initial BJT charge"));
            }
            if let Some((positive, negative)) = port {
                let charge = model.charge_branch_polarity(branch) * charge;
                add(*positive, charge)?;
                add(*negative, -charge)?;
            }
        }
        let delay = model.legacy_excess_phase_delay();
        if delay == 0.0 {
            if history.phase[index].is_some() {
                return Err(failure("startup history belongs to another phase model"));
            }
            inputs.push(None);
            continue;
        }
        let phase = history.phase[index]
            .as_ref()
            .ok_or_else(|| failure("missing initial phase history"))?;
        let current = model
            .legacy_forward_transport_branch(&history.dynamic_internal_prev[index])
            .ok_or_else(|| failure("missing initial forward-current equation"))?
            .current;
        if !current.is_finite()
            || phase.accepted_sample_count() != 1
            || phase.accepted_samples().next() != Some((0.0, current))
            || phase.accepted_left_limits().next().is_some()
            || phase.accepted_event_orders().next().is_some()
            || phase
                .small_signal_delay(0.0, current, delay, None)
                .map_err(failure)?
                .to_bits()
                != delay.to_bits()
        {
            return Err(failure(
                "startup requires the selected unsided time-zero phase seed",
            ));
        }
        inputs.push(Some(current));
    }
    Ok(StartupSeed { charges, inputs })
}

/// Finite outgoing rates and integrated source currents remain separate
/// observables. Neither represents an elapsed integration interval.
pub(in crate::engine::transient) struct PhysicalStartupReport {
    pub coordinate_rates: Vec<Option<Value>>,
    pub impulses: Vec<(usize, Value)>,
}

/// Exclusive model and history targets promoted together at startup.
pub(in crate::engine::transient) struct PhysicalStartupTargets<'a> {
    pub circuit: &'a mut crate::CircuitData,
    pub solution: &'a mut Vec<Value>,
    pub history: &'a mut BjtTransientHistory,
    pub diode_history: &'a mut DiodeTransientHistory,
    pub operating_point: Option<AcceptedTransientOperatingPointContract>,
}

impl Engine {
    /// Consume the already selected DC/IC histories and publish one physical
    /// outgoing point. Exclusive targets cannot change between preparation
    /// and commit; cancellation or any failure leaves all targets untouched.
    #[cfg(test)]
    pub(in crate::engine::transient) fn transition_physical_startup(
        &self,
        circuit: &mut crate::CircuitData,
        solution: &mut Vec<Value>,
        history: &mut BjtTransientHistory,
        options: &charge_event::EventOptions,
        flux_tolerance: Value,
        abort: &dyn AbortSignal,
    ) -> Result<PhysicalStartupReport, SimulationError> {
        self.transition_physical_startup_with_observation(
            PhysicalStartupTargets {
                circuit,
                solution,
                history,
                diode_history: &mut DiodeTransientHistory::default(),
                operating_point: None,
            },
            options,
            flux_tolerance,
            abort,
            |_| Ok(()),
        )
        .map(|(report, ())| report)
    }

    /// Prepare observations while state is private, then cross the physical
    /// commit barrier. A refused observation leaves every model target intact.
    pub(in crate::engine::transient) fn transition_physical_startup_with_observation<T>(
        &self,
        targets: PhysicalStartupTargets<'_>,
        options: &charge_event::EventOptions,
        flux_tolerance: Value,
        abort: &dyn AbortSignal,
        prepare_observation: impl FnOnce(&PreparedPhysicalEvent) -> Result<T, SimulationError>,
    ) -> Result<(PhysicalStartupReport, T), SimulationError> {
        let PhysicalStartupTargets {
            circuit,
            solution,
            history,
            diode_history,
            operating_point,
        } = targets;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::ResultValues,
            circuit
                .matrix_size()
                .saturating_mul(64)
                .saturating_add(circuit.bjts.len().saturating_mul(512)),
            options.limits.max_result_values,
        )?;
        let point = self.prepare_physical_event(
            circuit,
            history,
            PhysicalEventStep {
                diode_history,
                integration_coefficients: None,
                incoming: solution,
                time: 0.0,
                dt: 0.0,
                phase_events: PhysicalEventOrders::Startup(self.physical_startup_operating_point(
                    circuit,
                    operating_point,
                    options,
                    abort,
                )?),
            },
            options,
            flux_tolerance,
            abort,
        )?;
        let impulses: Vec<_> = point.impulses().collect();
        let observation = prepare_observation(&point)?;
        self.ensure_transport_history_copy(history, 0)?;
        let incoming_transport_bytes = history.transport_allocated_bytes();
        let mut outgoing = history.try_clone()?;
        // The incoming anchor was a private prehistory seed. Build the first
        // accepted sided knot in fresh buffers; appending another t=0 sample
        // to the seed buffer would violate the transport ownership contract.
        let mut outgoing_transport_bytes = outgoing.transport_allocated_bytes();
        let fresh_bytes = DelayBuffer::new(0).allocation_after_sample(None);
        for phase in outgoing.phase.iter_mut().flatten() {
            self.ensure_transport_history_bytes(
                incoming_transport_bytes
                    .saturating_add(outgoing_transport_bytes)
                    .saturating_add(fresh_bytes),
            )?;
            outgoing_transport_bytes = outgoing_transport_bytes
                .saturating_sub(phase.allocated_bytes())
                .saturating_add(fresh_bytes);
            *phase = DelayBuffer::try_new(4).map_err(|source| SimulationError::Allocation {
                object: "GP startup phase history",
                source,
            })?;
        }
        point.bjt.reserve_phase_storage(
            &mut outgoing,
            incoming_transport_bytes,
            &self.config.resource_limits,
        )?;
        Self::commit_bjt_history(&mut outgoing, point.bjt);
        outgoing
            .charge_q_prev_prev
            .clone_from(&outgoing.charge_q_prev);
        outgoing
            .charge_q_prev_prev_prev
            .clone_from(&outgoing.charge_q_prev);
        outgoing
            .dynamic_internal_prev_prev
            .clone_from(&outgoing.dynamic_internal_prev);
        outgoing
            .dynamic_linear_prev_prev
            .clone_from(&outgoing.dynamic_linear_prev);
        outgoing.vbe_prev_prev.clone_from(&outgoing.vbe_prev);
        outgoing.vbc_prev_prev.clone_from(&outgoing.vbc_prev);
        outgoing.vcs_prev_prev.clone_from(&outgoing.vcs_prev);
        outgoing.accepted_dt_prev = 0.0;
        outgoing.accepted_dt_prev_prev = 0.0;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        // No fallible work remains. Preserve finite dQ/dt and terminal
        // currents; an ordinary integration-history reset would erase them.
        for (line, prepared) in circuit.tlines.iter_mut().zip(point.lines) {
            line.commit_history_event(prepared.sample);
        }
        for (index, value) in point.capacitors.into_iter().enumerate() {
            circuit.capacitors.v_prev[index] = value.voltage;
            circuit.capacitors.v_prev_prev[index] = value.voltage;
            circuit.capacitors.v_prev_prev_prev[index] = value.voltage;
            circuit.capacitors.i_prev[index] = value.current;
        }
        for (index, winding) in point.windings.into_iter().enumerate() {
            circuit.inductors.i_prev[index] = winding.current;
            circuit.inductors.i_prev_prev[index] = winding.current;
            circuit.inductors.i_prev_prev_prev[index] = winding.current;
            circuit.inductors.v_prev[index] = winding.voltage;
        }
        diodes::commit(diode_history, &point.diodes);
        diode_history.restart_preserving_current(0.0);
        circuit.reset_coupled_inductor_pair_state(&point.state.solution);
        *history = outgoing;
        *solution = point.state.solution;
        Ok((
            PhysicalStartupReport {
                coordinate_rates: point.state.coordinate_rates,
                impulses,
            },
            observation,
        ))
    }
}
