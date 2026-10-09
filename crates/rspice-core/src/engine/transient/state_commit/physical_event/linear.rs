//! Complete finite and distributional state for a constant linear circuit.
use super::*;

pub(super) fn prepare(
    circuit: &crate::CircuitData,
    history: &BjtTransientHistory,
    step: &PhysicalEventStep<'_>,
    sampler: &PreparedEventCircuit<'_>,
    options: &charge_event::EventOptions,
    abort: &dyn AbortSignal,
) -> Result<PreparedPhysicalEvent, SimulationError> {
    let descriptor = sampler
        .linear_descriptor()
        .ok_or_else(|| failure("missing linear descriptor"))?;
    if !circuit.bjts.is_empty() || !circuit.diodes.is_empty() || !circuit.tlines.is_empty() {
        return Err(failure(
            "nonlinear or delayed state in constant-linear transition",
        ));
    }
    let startup = matches!(step.phase_events, PhysicalEventOrders::Startup(_));
    if let PhysicalEventOrders::FromCauses {
        sources,
        accepted_time,
    } = &step.phase_events
        && (!accepted_time.is_finite()
            || *accepted_time < 0.0
            || *accepted_time >= step.time
            || (*accepted_time + step.dt != step.time && step.time - *accepted_time != step.dt)
            || sources
                .next_after(*accepted_time, step.time)?
                .is_some_and(|time| time < step.time))
    {
        return Err(failure(
            "invalid incoming interval for descriptor transition",
        ));
    }
    let incoming_q = if startup {
        let currents = startup::startup_winding_currents(circuit, abort)?;
        descriptor.startup_storage(
            startup::linear_storage_terms(circuit, &currents)
                .map(|(row, coefficient, value)| (row - 1, coefficient, value)),
            options,
            abort,
        )?
    } else {
        descriptor.storage(step.incoming, options, abort)?
    };
    let transition = descriptor
        .evaluate(circuit, step.time, &incoming_q, options, abort)
        .map_err(|error| match error {
            SimulationError::Circuit(message) => {
                failure(format!("at t={:e}: {message}", step.time))
            }
            error => error,
        })?;
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::ResultValues,
        descriptor
            .retained_values()
            .saturating_add(incoming_q.retained_words())
            .saturating_add(transition.value_count().saturating_mul(4))
            .saturating_add(circuit.matrix_size().saturating_mul(64)),
        options.limits.max_result_values,
    )?;
    let mut descriptor_impulses = Vec::with_capacity(transition.impulse_count());
    for order in 0..transition.impulse_count() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        descriptor_impulses.push(transition.impulse(order).unwrap().to_vec());
    }
    let state = charge_event::ChargeEventState {
        solution: transition.finite().to_vec(),
        source_impulses: descriptor
            .source_branches()
            .iter()
            .map(|&column| transition.impulse(0).map_or(0.0, |values| values[column]))
            .collect(),
        coordinate_rates: transition.rates().iter().copied().map(Some).collect(),
        iterations: 0,
    };
    let mut device_impulses =
        PhysicalDeviceImpulses::prepare(false, circuit.capacitors.len(), 0, 0)?;
    let capacitors =
        prepare_capacitors(circuit, step, &state, startup, &mut device_impulses, abort)?;
    let windings = prepare_windings(circuit, &state.solution, options, abort)?;
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    Ok(PreparedPhysicalEvent {
        state,
        source_branches: descriptor.source_branches().to_vec(),
        phase_current_couplings: Vec::new(),
        time: step.time,
        dt: step.dt,
        bjt: PreparedBjtHistory {
            values: Vec::new(),
            dt: step.dt,
            accepted_time: step.time,
        },
        capacitors,
        windings,
        diodes: Vec::new(),
        lines: Vec::new(),
        left_limits: Vec::new(),
        phase_anchors: phase_anchors(history),
        device_impulses,
        descriptor_impulses: Some(descriptor_impulses),
    })
}
