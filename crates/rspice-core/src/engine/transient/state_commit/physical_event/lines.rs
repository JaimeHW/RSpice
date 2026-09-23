use super::*;
use crate::device::{TransmissionLine, TransmissionLineHistoryEvent, TransmissionLineTimeSide};

pub(in crate::engine::transient::state_commit) struct PreparedLineEvent {
    revision: u64,
    pub sample: TransmissionLineHistoryEvent,
}

impl PreparedLineEvent {
    pub(super) fn validate(&self, line: &TransmissionLine) -> Result<(), SimulationError> {
        if line.history_revision() != self.revision {
            return Err(failure(
                "transmission-line history changed after event preparation",
            ));
        }
        line.validate_history_event(&self.sample).map_err(failure)
    }
}

fn port_state(
    line: &TransmissionLine,
    time: Value,
    state: &charge_event::ChargeEventState,
    side: TransmissionLineTimeSide,
) -> Result<([Value; 4], [Value; 2]), SimulationError> {
    let v1 = Engine::differential_voltage(&state.solution, line.node1_pos, line.node1_neg);
    let v2 = Engine::differential_voltage(&state.solution, line.node2_pos, line.node2_neg);
    let (i1, i2) = line
        .transient_port_response_on_side(time, side)
        .port_currents(v1, v2);
    let mut slopes = [0.0; 2];
    for (index, (p, n, forward)) in [
        (line.node1_pos, line.node1_neg, false),
        (line.node2_pos, line.node2_neg, true),
    ]
    .into_iter()
    .enumerate()
    {
        // I=(V-W_delayed)/Z0, so d(V+Z0*I)/dt=2*dV/dt-dW_delayed/dt.
        let forcing_rate = line
            .lossless_wave_slope_on_side(time, forward, side)
            .map_err(failure)?;
        slopes[index] = sum([
            (rate(&state.coordinate_rates, p)?, 2.0),
            (rate(&state.coordinate_rates, n)?, -2.0),
            (forcing_rate, -1.0),
        ]
        .into_iter())?;
    }
    Ok(([v1, i1, v2, i2], slopes))
}

pub(super) fn prepare(
    circuit: &crate::CircuitData,
    step: &PhysicalEventStep<'_>,
    incoming: Option<&charge_event::ChargeEventState>,
    outgoing: &charge_event::ChargeEventState,
    options: &charge_event::EventOptions,
    abort: &dyn AbortSignal,
) -> Result<Vec<PreparedLineEvent>, SimulationError> {
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::ResultValues,
        circuit
            .matrix_size()
            .saturating_mul(64)
            .saturating_add(circuit.tlines.len().saturating_mul(24)),
        options.limits.max_result_values,
    )?;
    let mut prepared = Vec::with_capacity(circuit.tlines.len());
    for line in &circuit.tlines {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let (incoming, incoming_wave_slopes) =
            if matches!(step.phase_events, PhysicalEventOrders::Startup) {
                // Startup's incoming wave is the selected DC/IC history. It need
                // not solve the transient source's independently authored left limit.
                let anchor = line
                    .accepted_port_history()
                    .filter(|anchor| anchor[0] == 0.0)
                    .ok_or_else(|| failure("line startup requires its time-zero history anchor"))?;
                ([anchor[1], anchor[2], anchor[3], anchor[4]], [0.0; 2])
            } else {
                port_state(
                    line,
                    step.time,
                    incoming.ok_or_else(|| failure("missing incoming line rate solve"))?,
                    TransmissionLineTimeSide::Incoming,
                )?
            };
        let (outgoing, outgoing_wave_slopes) = port_state(
            line,
            step.time,
            outgoing,
            TransmissionLineTimeSide::Outgoing,
        )?;
        let sample = TransmissionLineHistoryEvent {
            time: step.time,
            incoming,
            outgoing,
            incoming_wave_slopes,
            outgoing_wave_slopes,
        };
        line.validate_history_event(&sample).map_err(failure)?;
        prepared.push(PreparedLineEvent {
            revision: line.history_revision(),
            sample,
        });
    }
    Ok(prepared)
}
