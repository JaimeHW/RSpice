//! Native diode charge and finite current prepared before joint acceptance.
use super::*;

pub(in crate::engine::transient::state_commit) struct AcceptedDiode {
    voltage: Value,
    charge: Value,
    current: Value,
}

pub(super) fn validate_history(
    circuit: &crate::CircuitData,
    history: &DiodeTransientHistory,
) -> Result<(), SimulationError> {
    let count = circuit.diodes.len();
    if [
        history.vd_prev.len(),
        history.vd_prev_prev.len(),
        history.qd_prev.len(),
        history.qd_prev_prev.len(),
        history.qd_prev_prev_prev.len(),
        history.cqd_prev.len(),
    ]
    .into_iter()
    .any(|length| length != count)
        || ![history.accepted_dt_prev, history.accepted_dt_prev_prev]
            .into_iter()
            .all(|dt| dt.is_finite() && dt >= 0.0)
    {
        return Err(failure("unaligned physical diode history"));
    }
    Ok(())
}

pub(super) fn prepare(
    circuit: &crate::CircuitData,
    step: &PhysicalEventStep<'_>,
    state: &charge_event::ChargeEventState,
    startup: bool,
    impulses: &mut PhysicalDeviceImpulses,
    options: &charge_event::EventOptions,
    abort: &dyn AbortSignal,
) -> Result<Vec<AcceptedDiode>, SimulationError> {
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::ResultValues,
        circuit
            .matrix_size()
            .saturating_mul(64)
            .saturating_add(circuit.diodes.len().saturating_mul(8)),
        options.limits.max_result_values,
    )?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(circuit.diodes.len())
        .map_err(|source| SimulationError::Allocation {
            object: "physical diode history",
            source,
        })?;
    for (index, diode) in circuit.diodes.devices.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let p = diode.node_anode;
        let n = diode.node_cathode;
        let voltage = Engine::differential_voltage(&state.solution, p, n);
        let (charge, capacitance) = diode.junction_charge_and_capacitance(voltage);
        if ![voltage, charge, capacitance]
            .into_iter()
            .all(Value::is_finite)
        {
            return Err(failure(format!(
                "diode '{}' has nonfinite outgoing charge",
                diode.name
            )));
        }
        let current = if capacitance == 0.0 {
            0.0
        } else {
            sum([
                (rate(&state.coordinate_rates, p)?, capacitance),
                (rate(&state.coordinate_rates, n)?, -capacitance),
            ]
            .into_iter())?
        };
        if let PhysicalDeviceImpulses::Jumps { diodes, .. } = impulses {
            let incoming = if startup {
                step.diode_history.qd_prev[index]
            } else {
                diode
                    .junction_charge_and_capacitance(diode.terminal_voltage(step.incoming))
                    .0
            };
            diodes.push(sum([(charge, 1.0), (incoming, -1.0)].into_iter())?);
        }
        values.push(AcceptedDiode {
            voltage,
            charge,
            current,
        });
    }
    Ok(values)
}

pub(in crate::engine::transient::state_commit) fn commit(
    history: &mut DiodeTransientHistory,
    values: &[AcceptedDiode],
) {
    for (index, value) in values.iter().enumerate() {
        history.accept_physical_branch(index, value.voltage, value.charge, value.current);
    }
}
