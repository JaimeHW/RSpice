//! Continuous-event current references from retained integration history.
//! These are predictors, not replacements for the physical equations: the
//! rate solver admits them only within each original equation's backward-error
//! budget, including the KCL row replaced by a floating-component sum.

use super::*;
use rspice_veriloga_runtime::transport_delay::DelayTimeSide;

pub(super) fn currents(
    circuit: &crate::CircuitData,
    history: &BjtTransientHistory,
    step: &PhysicalEventStep<'_>,
    sampler: &PreparedEventCircuit<'_>,
    phases: &[Option<EventPhase<'_>>],
    abort: &dyn AbortSignal,
) -> Result<Option<Vec<Value>>, SimulationError> {
    let Some(coeff) = step.integration_coefficients else {
        return Ok(None);
    };
    if !step.dt.is_finite() || step.dt <= 0.0 {
        return Err(failure("invalid incoming companion interval"));
    }
    // Line histories need their own incoming-wave integration certificate.
    if !circuit.tlines.is_empty()
        || !circuit
            .behavioral_sources
            .has_smooth_physical_time_equations()
    {
        return Ok(None);
    }
    // Require each forcing value to be unchanged, before summing into KCL.
    let voltage = &circuit.voltage_sources;
    let current = &circuit.current_sources;
    for i in 0..voltage.len().max(current.len()) {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if (i < voltage.len()
            && voltage.transient_value_at_on_side(i, step.time, SourceTimeSide::LeftLimit)
                != voltage.transient_value_at_on_side(i, step.time, SourceTimeSide::RightLimit))
            || (i < current.len()
                && current.value_at_time_on_side(i, step.time, SourceTimeSide::LeftLimit)
                    != current.value_at_time_on_side(i, step.time, SourceTimeSide::RightLimit))
        {
            return Ok(None);
        }
    }
    for (model, phase) in sampler.models().iter().zip(phases) {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let Some(phase) = phase {
            let value = |side| {
                phase
                    .history
                    .difference_at_discontinuity_on_side(
                        step.time,
                        phase.endpoint,
                        0.0,
                        model.legacy_excess_phase_delay(),
                        None,
                        side,
                    )
                    .map(|value| value.output)
                    .map_err(failure)
            };
            if value(DelayTimeSide::Incoming)? != value(DelayTimeSide::Outgoing)? {
                return Ok(None);
            }
        }
    }
    // Use the same per-branch charge reconstruction as accepted history.
    // In a weighted static-history method this reference can differ from the
    // assembled endpoint equation; the rate solver checks rather than trusts it.
    let bjt = Engine::prepare_bjt_history(
        circuit,
        history,
        step.incoming,
        coeff,
        step.dt,
        step.time,
        None,
        bjt::BjtPhaseContext {
            incoming_arrival: true,
            input_left_limits: None,
        },
    )?;
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let mut currents = vec![0.0; circuit.matrix_size()];
    let mut add = |node: usize, value: Value| -> Result<(), SimulationError> {
        if !value.is_finite() {
            return Err(failure("nonfinite incoming storage current"));
        }
        if node != 0 {
            currents[node - 1] = sum([(currents[node - 1], 1.0), (value, 1.0)].into_iter())?;
        }
        Ok(())
    };
    let c = &circuit.capacitors;
    for (index, stamp) in c.stamps.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let voltage = Engine::differential_voltage(step.incoming, stamp.pp.row, stamp.nn.row);
        let current = coeff.capacitor_geq(c.capacitances[index], step.dt) * voltage
            - coeff.capacitor_ieq(
                c.capacitances[index],
                step.dt,
                c.v_prev[index],
                c.v_prev_prev[index],
                c.i_prev[index],
            );
        add(stamp.pp.row, current)?;
        add(stamp.nn.row, -current)?;
    }
    for (model, accepted) in sampler.models().iter().zip(bjt.values) {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        for (branch, port) in model.charge_storage_nodes().iter().enumerate() {
            if let Some((p, n)) = port {
                let current = model.charge_branch_polarity(branch) * accepted.currents[branch];
                add(*p, current)?;
                add(*n, -current)?;
            }
        }
    }
    let l = &circuit.inductors;
    let nodes = circuit.num_nodes();
    let mut fluxes = vec![[0.0; 3]; circuit.matrix_size() - nodes];
    let mut values = vec![[0.0; 3]; fluxes.len()];
    for (index, &ordinal) in l.branch_indices.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let value = [
            step.incoming[nodes + ordinal - 1],
            l.i_prev[index],
            l.i_prev_prev[index],
        ];
        values[ordinal - 1] = value;
        fluxes[ordinal - 1] = value.map(|value| -l.inductances[index] * value);
    }
    for pair in &circuit.coupled_inductor_pairs {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let a = pair.branch1_ordinal - 1;
        let b = pair.branch2_ordinal - 1;
        for level in 0..3 {
            fluxes[a][level] =
                sum([(fluxes[a][level], 1.0), (values[b][level], -pair.device.m)].into_iter())?;
            fluxes[b][level] =
                sum([(fluxes[b][level], 1.0), (values[a][level], -pair.device.m)].into_iter())?;
        }
    }
    for (index, &ordinal) in l.branch_indices.iter().enumerate() {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let [q, q_prev, q_prev_prev] = fluxes[ordinal - 1];
        add(
            nodes + ordinal,
            Engine::jfet_companion_ccap(
                coeff,
                step.dt,
                q,
                BranchChargeHistory {
                    q_prev,
                    q_prev_prev,
                    cq_prev: -l.v_prev[index],
                },
            ),
        )?;
    }
    Ok(Some(currents))
}
