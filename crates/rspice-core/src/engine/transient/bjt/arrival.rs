//! Physical arrival clocks come from accepted transport memory, independently
//! of the source breakpoint manager's approximate matching and restart policy.

use super::*;
use rspice_veriloga_runtime::transport_delay::DelayEventOrder;

/// Linear delay interpolation needs bounded curvature. BE, trapezoidal and
/// BDF2 integration require no derivative continuity above order two. A C2
/// physical input can therefore cross the ordinary adaptive grid; its stored
/// knot and interpolation-error control remain intact. Unknown provenance
/// never qualifies. Keep order-two events even during first-order restarts.
fn requires_tracking(order: DelayEventOrder) -> bool {
    !matches!(order, DelayEventOrder::AtLeast(n) if n >= 3)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::engine::transient) struct PhaseArrival {
    pub time: Value,
    pub device_index: usize,
    pub order: DelayEventOrder,
}

impl PhaseArrival {
    pub(in crate::engine::transient) fn ensure_reachable(
        self,
        circuit: &crate::CircuitData,
        accepted: Value,
        minimum: Value,
    ) -> Result<(), SimulationError> {
        let gap = self.time - accepted;
        if !minimum.is_finite() || minimum <= 0.0 || !gap.is_finite() || gap < minimum {
            return Err(SimulationError::Circuit(format!(
                "BJT '{}' phase arrival {:.17e} s cannot be reached from accepted time {accepted:.17e} s with minimum step {minimum:.17e} s; coalescing this physical delay event is not qualified",
                circuit.bjts.devices[self.device_index].name, self.time
            )));
        }
        Ok(())
    }

    /// Apply after every policy that can widen or re-time a proposal. A rounded
    /// endpoint can reach an arrival even when subtracting its clock yields a
    /// slightly larger interval; preserve that smaller valid integration width.
    pub(in crate::engine::transient) fn limit_step(
        self,
        circuit: &crate::CircuitData,
        accepted: Value,
        proposed_dt: Value,
        proposed_time: Value,
        minimum: Value,
    ) -> Result<Option<Value>, SimulationError> {
        self.ensure_reachable(circuit, accepted, minimum)?;
        if !proposed_dt.is_finite()
            || proposed_dt <= 0.0
            || !proposed_time.is_finite()
            || proposed_time <= accepted
        {
            return Err(SimulationError::Circuit(
                "GP phase arrival received an invalid transient proposal".into(),
            ));
        }
        if proposed_time < self.time {
            return Ok(None);
        }
        let dt = proposed_dt.min(self.time - accepted);
        if dt < minimum {
            return Err(SimulationError::Circuit(
                "GP phase arrival proposal is below the minimum integration step".into(),
            ));
        }
        Ok(Some(dt))
    }
}

/// Queries are read-only, so a rejected trial or checkpoint continuation owns
/// the same arrival without copying a second queue. Ordinary interpolation
/// knots do not create deadlines or impose a timestep-at-most-delay cap.
pub(in crate::engine::transient) fn next(
    circuit: &crate::CircuitData,
    history: &BjtTransientHistory,
    time: Value,
    stop: Value,
    method: IntegrationMethod,
    abort: &dyn AbortSignal,
) -> Result<Option<PhaseArrival>, SimulationError> {
    // Exhaustive on purpose: a higher-order integrator must qualify its own
    // tracking cutoff before it can use this dispatcher.
    match method {
        IntegrationMethod::BackwardEuler
        | IntegrationMethod::Trapezoidal
        | IntegrationMethod::Gear2
        | IntegrationMethod::TrapGear => {}
    }
    let mut earliest: Option<PhaseArrival> = None;
    visit_next_matching(
        circuit,
        history,
        time,
        stop,
        abort,
        |arrival| requires_tracking(arrival.order),
        |arrival| {
            match &mut earliest {
                Some(previous) if previous.time == arrival.time => {
                    previous.order = previous.order.merge(arrival.order);
                }
                previous if previous.is_none_or(|previous| arrival.time < previous.time) => {
                    *previous = Some(arrival);
                }
                _ => {}
            }
            Ok(())
        },
    )?;
    Ok(earliest)
}

/// Retain each device's cause at a selected physical endpoint, including a
/// smooth cause coincident with another event. Earlier C2 arrivals impose no
/// deadline; earlier rough/unknown causes still expose a skipped clock.
pub(in crate::engine::transient) fn visit_next(
    circuit: &crate::CircuitData,
    history: &BjtTransientHistory,
    time: Value,
    stop: Value,
    abort: &dyn AbortSignal,
    visit: impl FnMut(PhaseArrival) -> Result<(), SimulationError>,
) -> Result<(), SimulationError> {
    visit_next_matching(
        circuit,
        history,
        time,
        stop,
        abort,
        |arrival| arrival.time == stop || requires_tracking(arrival.order),
        visit,
    )
}

fn visit_next_matching(
    circuit: &crate::CircuitData,
    history: &BjtTransientHistory,
    time: Value,
    stop: Value,
    abort: &dyn AbortSignal,
    retain: impl Fn(PhaseArrival) -> bool,
    mut visit: impl FnMut(PhaseArrival) -> Result<(), SimulationError>,
) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    if !time.is_finite() || time < 0.0 || !stop.is_finite() || stop < time {
        return Err(SimulationError::Circuit(
            "GP phase arrival requires finite ordered transient times".into(),
        ));
    }
    if history.phase.len() != circuit.bjts.devices.len() {
        return Err(SimulationError::Circuit(
            "GP phase arrival history does not match the BJT population".into(),
        ));
    }
    for (index, (bjt, phase)) in circuit.bjts.devices.iter().zip(&history.phase).enumerate() {
        if index != 0 && index.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let refuse =
            |error| SimulationError::Circuit(format!("BJT '{}' phase arrival: {error}", bjt.name));
        let delay = bjt.legacy_excess_phase_delay();
        let Some(phase) = phase else {
            if delay != 0.0 {
                return Err(refuse("missing accepted transport history".to_owned()));
            }
            continue;
        };
        if !delay.is_finite() || delay <= 0.0 {
            return Err(refuse(
                "requires a finite positive nominal phase delay".to_owned(),
            ));
        }
        phase.validate_accepted_time(time).map_err(refuse)?;
        let current = phase
            .accepted_samples()
            .next_back()
            .expect("validated history")
            .1;
        let retained = phase
            .small_signal_delay(time, current, delay, None)
            .map_err(refuse)?;
        if retained.to_bits() != delay.to_bits() {
            return Err(refuse(
                "history belongs to a different nominal phase delay".to_owned(),
            ));
        }
        let mut cursor = time;
        while let Some(arrival) = phase.next_event_after(cursor).map_err(refuse)? {
            if arrival.time <= cursor {
                return Err(refuse(
                    "arrival does not advance the accepted clock".to_owned(),
                ));
            }
            if arrival.time > stop {
                break;
            }
            let arrival = PhaseArrival {
                time: arrival.time,
                device_index: index,
                order: arrival.order,
            };
            // The buffer merges every contributor to this represented clock
            // before filtering. A known smooth event cannot hide an unknown
            // or rough event in the same group or a later group.
            if retain(arrival) {
                visit(arrival)?;
                break;
            }
            cursor = arrival.time;
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
        }
    }
    Ok(())
}

/// Accepted transport samples must retain the time used in their equations.
pub(in crate::engine::transient) fn validate_clock(
    has_phase: bool,
    solved: Value,
    accepted: Value,
) -> Result<(), SimulationError> {
    if has_phase && solved.to_bits() != accepted.to_bits() {
        return Err(SimulationError::Circuit(
            "GP phase state cannot be relabelled after its transient solve".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
