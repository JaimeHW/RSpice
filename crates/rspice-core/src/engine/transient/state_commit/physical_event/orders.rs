//! Solver-owned value-continuity certificates from physical event causes.
//! No interpolation knot, approximate clock match or equal sampled slope
//! creates a certificate. Higher derivative propagation remains unqualified.

use super::*;
use crate::engine::transient::source_events::{PhysicalSourceEvents, PhysicalSourceOwner};

pub(in crate::engine::transient) enum PhysicalEventOrders<'a> {
    /// The selected DC/IC history precedes the first outgoing transient
    /// point. Its waveform continuity is not a DC-to-transient certificate.
    Startup,
    /// An independently justified declaration, retained for explicit event
    /// owners and their acceptance-contract tests.
    Declared(&'a [Option<DelayEventOrder>]),
    /// Schedule prepared for this circuit and the actual accepted clock.
    /// The incoming solution must have been solved on the left of this event.
    /// Startup requires its separate DC-to-transient transition owner.
    FromCauses {
        sources: &'a PhysicalSourceEvents,
        accepted_time: Value,
    },
}

pub(super) struct ClassifiedOrders {
    pub orders: Vec<Option<DelayEventOrder>>,
    pub continuous: bool,
}

fn continuous(order: DelayEventOrder) -> bool {
    matches!(order, DelayEventOrder::AtLeast(n) if n > 0)
}

pub(super) fn classify(
    circuit: &crate::CircuitData,
    history: &BjtTransientHistory,
    step: &PhysicalEventStep<'_>,
    sampler: &PreparedEventCircuit<'_>,
    topology: &charge_event::ChargeEventTopology,
    abort: &dyn AbortSignal,
) -> Result<ClassifiedOrders, SimulationError> {
    if matches!(step.phase_events, PhysicalEventOrders::Startup) {
        return Ok(ClassifiedOrders {
            orders: sampler
                .models()
                .iter()
                .map(|model| {
                    (model.legacy_excess_phase_delay() != 0.0)
                        .then_some(DelayEventOrder::AtLeast(0))
                })
                .collect(),
            continuous: false,
        });
    }
    if let PhysicalEventOrders::Declared(orders) = &step.phase_events {
        if orders.len() != circuit.bjts.len() {
            return Err(failure("invalid physical event population"));
        }
        return Ok(ClassifiedOrders {
            orders: orders.to_vec(),
            continuous: false,
        });
    }
    let PhysicalEventOrders::FromCauses {
        sources,
        accepted_time,
    } = &step.phase_events
    else {
        unreachable!()
    };
    if step.time == 0.0 {
        return Err(failure(
            "startup events require the DC-to-transient transition owner",
        ));
    }
    if !accepted_time.is_finite()
        || *accepted_time < 0.0
        || *accepted_time >= step.time
        || (*accepted_time + step.dt != step.time && step.time - *accepted_time != step.dt)
    {
        return Err(failure("invalid accepted clock for physical event causes"));
    }
    if sources
        .next_after(*accepted_time, step.time)?
        .is_some_and(|t| t < step.time)
    {
        return Err(failure(
            "incoming interval crossed an unprocessed physical source event",
        ));
    }
    let mut has_cause = false;
    let mut has_unknown = false;
    let mut invariant = true;
    for event in sources.at(step.time)? {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        has_cause = true;
        has_unknown |= event.order == DelayEventOrder::Unknown;
        match event.owner {
            PhysicalSourceOwner::Voltage(index) => {
                if index >= circuit.voltage_sources.len() {
                    return Err(failure("physical voltage source owner is absent"));
                }
                invariant &= continuous(event.order);
            }
            PhysicalSourceOwner::Current(index) => {
                let table = &circuit.current_sources;
                if index >= table.len() {
                    return Err(failure("physical current source owner is absent"));
                }
                invariant &= continuous(event.order)
                    || topology
                        .current_jump_coupling(table.node_pos[index], table.node_neg[index])?
                        == charge_event::CurrentJumpCoupling::Cancels;
            }
        }
    }
    bjt::arrival::visit_next(
        circuit,
        history,
        *accepted_time,
        step.time,
        abort,
        |event| {
            if event.time != step.time {
                return Err(failure(
                    "incoming interval crossed an unprocessed physical delay event",
                ));
            }
            has_cause = true;
            has_unknown |= event.order == DelayEventOrder::Unknown;
            let (p, n) = sampler.models()[event.device_index]
                .legacy_forward_transport_nodes()
                .ok_or_else(|| failure("physical GP arrival has no current port"))?;
            invariant &= continuous(event.order)
                || topology.current_jump_coupling(p, n)?
                    == charge_event::CurrentJumpCoupling::Cancels;
            Ok(())
        },
    )?;
    if !has_cause {
        return Err(failure(
            "no physical source or delay cause at the requested clock",
        ));
    }
    // The certificate uses source declarations AND their represented
    // sides. A stale/mismatched schedule or a rounded discontinuity may
    // downgrade a declaration, never upgrade it from numerical equality.
    for index in 0..circuit.voltage_sources.len() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let table = &circuit.voltage_sources;
        invariant &= table.transient_value_at_on_side(index, step.time, SourceTimeSide::LeftLimit)
            == table.transient_value_at_on_side(index, step.time, SourceTimeSide::RightLimit);
    }
    for index in 0..circuit.current_sources.len() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let table = &circuit.current_sources;
        invariant &= table.value_at_time_on_side(index, step.time, SourceTimeSide::LeftLimit)
            == table.value_at_time_on_side(index, step.time, SourceTimeSide::RightLimit)
            || topology.current_jump_coupling(table.node_pos[index], table.node_neg[index])?
                == charge_event::CurrentJumpCoupling::Cancels;
    }
    // Every admitted nonlinear F/Q law must be C1 in a neighborhood of
    // the fixed coordinates. The two physical solves then establish local
    // rank and audit the same jump constraints at their incoming limit.
    for model in sampler.models() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        invariant &= model.legacy_event_locally_c1(step.incoming);
    }
    let order = if invariant {
        DelayEventOrder::AtLeast(1)
    } else if has_unknown {
        DelayEventOrder::Unknown
    } else {
        DelayEventOrder::AtLeast(0)
    };
    Ok(ClassifiedOrders {
        // Conservative circuit-wide dependency: every GP input retains
        // the physical event. Independent subcircuits can be refined later.
        orders: sampler
            .models()
            .iter()
            .map(|model| (model.legacy_excess_phase_delay() != 0.0).then_some(order))
            .collect(),
        continuous: invariant,
    })
}
