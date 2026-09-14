//! Physical arrival clocks come from accepted transport memory, independently
//! of the source breakpoint manager's approximate matching and restart policy.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::engine::transient) struct PhaseArrival {
    pub time: Value,
    pub device_index: usize,
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
    abort: &dyn AbortSignal,
) -> Result<Option<PhaseArrival>, SimulationError> {
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
    let mut earliest: Option<PhaseArrival> = None;
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
        if let Some(arrival) = phase.next_discontinuity_after(time).map_err(refuse)? {
            if arrival <= time {
                return Err(refuse(
                    "arrival does not advance the accepted clock".to_owned(),
                ));
            }
            if arrival <= stop && earliest.is_none_or(|previous| arrival < previous.time) {
                earliest = Some(PhaseArrival {
                    time: arrival,
                    device_index: index,
                });
            }
        }
    }
    Ok(earliest)
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
