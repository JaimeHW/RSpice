//! Exact source-root clocks and conservative continuity declarations.
//! This schedule has no controller tolerance or consumed cursor. Rejection
//! and checkpoint continuation query the same immutable source ownership.

use super::*;
use crate::circuit::{SourceTimeBasis, SourceTimeSide, VoltageSources};
use crate::netlist::SourceSpec;
use breakpoints::{SourceBreakpointContext, SourceBreakpointGeometry, SourceClockSink};
use rspice_veriloga_runtime::transport_delay::DelayEventOrder;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum PhysicalSourceOwner {
    Voltage(usize),
    Current(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct PhysicalSourceEvent {
    pub time: Value,
    pub owner: PhysicalSourceOwner,
    pub order: DelayEventOrder,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct PhysicalSourceEvents {
    events: Vec<PhysicalSourceEvent>,
}

fn failure(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("physical source events: {}", message.into()))
}

impl PhysicalSourceEvents {
    pub(super) fn at(&self, time: Value) -> Result<&[PhysicalSourceEvent], SimulationError> {
        if !time.is_finite() || time < 0.0 {
            return Err(failure("invalid event query time"));
        }
        let first = self.events.partition_point(|event| event.time < time);
        let end = self.events.partition_point(|event| event.time <= time);
        Ok(&self.events[first..end])
    }

    pub(super) fn next_after(
        &self,
        time: Value,
        stop: Value,
    ) -> Result<Option<Value>, SimulationError> {
        if !time.is_finite() || time < 0.0 || !stop.is_finite() || stop < time {
            return Err(failure("invalid event query interval"));
        }
        Ok(self
            .events
            .get(self.events.partition_point(|event| event.time <= time))
            .map(|event| event.time)
            .filter(|&next| next <= stop))
    }
}

/// Positive finite times have the same bit and numeric ordering. Canonicalize
/// zero, deduplicate exact clocks only, and refuse before exceeding capacity.
struct ExactClocks {
    times: BTreeSet<u64>,
    maximum: usize,
    overflow: bool,
    minimum: Value,
}

impl SourceClockSink for ExactClocks {
    fn insert_clock(&mut self, time: Value) {
        if time < self.minimum {
            return;
        }
        let bits = if time == 0.0 { 0 } else { time.to_bits() };
        if self.times.contains(&bits) {
            return;
        }
        if self.times.len() >= self.maximum {
            self.overflow = true;
        } else {
            self.times.insert(bits);
        }
    }
    fn clock_count(&self) -> usize {
        self.times.len().saturating_add(usize::from(self.overflow))
    }
    fn minimum_clock(&self) -> Value {
        self.minimum
    }
}

fn activation(spec: &SourceSpec, basis: SourceTimeBasis, dialect: SpiceDialect) -> Value {
    match spec {
        SourceSpec::Distortion { inner, .. }
        | SourceSpec::RfPort { inner, .. }
        | SourceSpec::DcTransient {
            transient: inner, ..
        }
        | SourceSpec::AcTransient {
            transient: inner, ..
        }
        | SourceSpec::DcAcTransient {
            transient: inner, ..
        } => activation(inner, basis, dialect),
        SourceSpec::Pwl { delay, .. }
        | SourceSpec::PwlFile { delay, .. }
        | SourceSpec::Pulse { delay, .. } => delay.max(0.0),
        SourceSpec::Exp {
            td1,
            tau1,
            td2,
            tau2,
            ..
        } => VoltageSources::resolve_exp_timing_with_defaults(
            *td1,
            *tau1,
            *td2,
            *tau2,
            basis.tstep.max(1e-18),
            dialect,
        )
        .0
        .max(0.0),
        _ => 0.0,
    }
}

/// A structural C0 certificate, not a sampled value/slope comparison. A zero
/// bound is valid for the other finite native waveforms; no higher order or
/// absence of an event is inferred. Refine these conservative bounds only
/// with a corresponding analytic argument, especially for repeat seams.
fn continuous(
    spec: &SourceSpec,
    basis: SourceTimeBasis,
    dialect: SpiceDialect,
    abort: &dyn AbortSignal,
) -> Result<bool, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    Ok(match spec {
        SourceSpec::Distortion { inner, .. }
        | SourceSpec::RfPort { inner, .. }
        | SourceSpec::DcTransient {
            transient: inner, ..
        }
        | SourceSpec::AcTransient {
            transient: inner, ..
        }
        | SourceSpec::DcAcTransient {
            transient: inner, ..
        } => return continuous(inner, basis, dialect, abort),
        SourceSpec::Dc(_)
        | SourceSpec::Ac { .. }
        | SourceSpec::DcAc { .. }
        | SourceSpec::Sin { .. } => true,
        SourceSpec::Pulse {
            delay,
            rise,
            fall,
            width,
            period,
            pulse_count,
            width_defaults_to_zero,
            ..
        } => {
            let (_, rise, fall, width, period) = VoltageSources::resolve_pulse_timing_with_defaults(
                *delay,
                *rise,
                *fall,
                *width,
                *period,
                *width_defaults_to_zero,
                basis.tstep.max(1e-18),
                basis.tstop.max(1e-18),
                dialect,
            );
            rise > 0.0
                && fall > 0.0
                && width >= 0.0
                && [rise, fall, width].into_iter().all(Value::is_finite)
                && (!period.is_finite() || period == 0.0 || period >= rise + width + fall)
                && (*pulse_count <= 0.0 || pulse_count.fract() == 0.0)
        }
        SourceSpec::Pwl {
            points,
            delay,
            repeat_from,
        } => {
            if !delay.is_finite()
                || repeat_from.is_some()
                || !points
                    .first()
                    .is_some_and(|&(time, value)| time >= 0.0 && value == 0.0)
            {
                return Ok(false);
            }
            let mut previous = None;
            for (index, &(time, value)) in points.iter().enumerate() {
                if index.is_multiple_of(1024) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if !time.is_finite()
                    || !value.is_finite()
                    || previous.is_some_and(|old| old >= time)
                {
                    return Ok(false);
                }
                previous = Some(time);
            }
            true
        }
        SourceSpec::Exp {
            td1,
            tau1,
            td2,
            tau2,
            ..
        } => {
            let (first, rise, second, fall) = VoltageSources::resolve_exp_timing_with_defaults(
                *td1,
                *tau1,
                *td2,
                *tau2,
                basis.tstep.max(1e-18),
                dialect,
            );
            first <= second && rise.is_finite() && rise > 0.0 && fall.is_finite() && fall > 0.0
        }
        // File transformations, pattern/repeat seams and switched carriers
        // retain order zero until their own structural continuity is certified.
        SourceSpec::PwlFile { .. }
        | SourceSpec::Pat { .. }
        | SourceSpec::Sffm { .. }
        | SourceSpec::Am { .. }
        | SourceSpec::TrNoise { .. }
        | SourceSpec::TrRandom { .. } => false,
    })
}

impl Engine {
    pub(super) fn collect_physical_source_events(
        circuit: &crate::CircuitData,
        stop: Value,
        limits: &crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<PhysicalSourceEvents, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !stop.is_finite() || stop < 0.0 {
            return Err(failure("invalid stop time"));
        }
        if circuit.voltage_sources.is_empty() && circuit.current_sources.is_empty() {
            return Ok(PhysicalSourceEvents::default());
        }
        let (basis, dialect) = circuit.independent_source_event_basis().map_err(failure)?;
        let mut events = Vec::new();
        // Preserve storage indices even when DC-only entries have no SourceSpec.
        let voltage = circuit
            .voltage_sources
            .source_specs
            .iter()
            .enumerate()
            .filter_map(|(index, spec)| spec.as_ref().map(|_| index))
            .zip(circuit.voltage_sources.transient_specs_named_with_pwl())
            .map(|(index, entry)| (PhysicalSourceOwner::Voltage(index), entry));
        let current = circuit
            .current_sources
            .source_specs
            .iter()
            .enumerate()
            .filter_map(|(index, spec)| spec.as_ref().map(|_| index))
            .zip(circuit.current_sources.transient_specs_named_with_pwl())
            .map(|(index, entry)| (PhysicalSourceOwner::Current(index), entry));
        let expected = circuit
            .voltage_sources
            .source_specs
            .iter()
            .chain(&circuit.current_sources.source_specs)
            .filter(|spec| spec.is_some())
            .count();
        let mut seen = 0;
        // Bound logical event records and temporary keys before allocation.
        // This is not a measurement of allocator/B-tree backend heap overhead.
        let words =
            std::mem::size_of::<PhysicalSourceEvent>().div_ceil(std::mem::size_of::<Value>()) + 1;
        let maximum = limits
            .max_analysis_points
            .min(limits.max_result_values / words);
        for (owner, (name, spec, waveform)) in voltage.chain(current) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            seen += 1;
            let remaining = maximum.saturating_sub(events.len());
            let mut clocks = ExactClocks {
                times: BTreeSet::new(),
                maximum: remaining,
                overflow: false,
                minimum: activation(spec, basis, dialect),
            };
            Self::add_source_spec_breakpoints_with_pwl(
                &mut clocks,
                spec,
                waveform,
                BreakpointWindow {
                    tstop: stop,
                    tstep_hint: basis.tstep,
                    dialect,
                },
                SourceBreakpointContext {
                    basis: Some(basis),
                    geometry: SourceBreakpointGeometry::ExactPhysicalEvents,
                },
                abort,
                remaining,
            )?;
            let base_order = u32::from(continuous(spec, basis, dialect, abort)?);
            for (ordinal, bits) in clocks.times.into_iter().enumerate() {
                if ordinal.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let time = Value::from_bits(bits);
                let value = |side| match owner {
                    PhysicalSourceOwner::Voltage(index) => circuit
                        .voltage_sources
                        .transient_value_at_on_side(index, time, side),
                    PhysicalSourceOwner::Current(index) => circuit
                        .current_sources
                        .value_at_time_on_side(index, time, side),
                };
                let left = value(SourceTimeSide::LeftLimit);
                let right = value(SourceTimeSide::RightLimit);
                if !left.is_finite() || !right.is_finite() {
                    return Err(failure(format!(
                        "source '{name}' has nonfinite limits at {time:.17e}"
                    )));
                }
                // A collapsed represented ramp can have unequal limits even
                // when its authored curve is continuous. Never publish an
                // inconsistent positive bound for that represented event.
                let order = DelayEventOrder::AtLeast(if left == right { base_order } else { 0 });
                events.push(PhysicalSourceEvent { time, owner, order });
            }
        }
        if seen != expected {
            return Err(failure("unaligned source waveform storage"));
        }
        events.sort_by(|a, b| a.time.total_cmp(&b.time).then(a.owner.cmp(&b.owner)));
        Ok(PhysicalSourceEvents { events })
    }
}

#[cfg(test)]
mod tests;
