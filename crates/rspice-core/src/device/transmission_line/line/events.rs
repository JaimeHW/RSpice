//! Owned two-sided events in an ordinary scalar line's accepted history.
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransmissionLineTimeSide {
    Incoming,
    Outgoing,
}

/// Both physical port limits at an accepted event. Port arrays are ordered
/// V1, I1, V2, I2; slope arrays are d(V1+Z0*I1)/dt, d(V2+Z0*I2)/dt.
/// Slopes are finite one-sided derivatives, never jump size divided by dt.
#[derive(Debug, Clone, Copy)]
pub struct TransmissionLineHistoryEvent {
    pub time: Value,
    pub incoming: [Value; 4],
    pub outgoing: [Value; 4],
    pub incoming_wave_slopes: [Value; 2],
    pub outgoing_wave_slopes: [Value; 2],
}

#[derive(Debug, Clone, Copy)]
pub(super) struct HistoryEvent {
    pub(super) incoming: TlineStateSample,
    incoming_wave_slopes: [Value; 2],
    outgoing_wave_slopes: [Value; 2],
}

impl HistoryEvent {
    pub(super) fn checkpoint(&self) -> [Value; 9] {
        let s = self.incoming;
        [
            s.time,
            s.v1,
            s.i1,
            s.v2,
            s.i2,
            self.incoming_wave_slopes[0],
            self.incoming_wave_slopes[1],
            self.outgoing_wave_slopes[0],
            self.outgoing_wave_slopes[1],
        ]
    }
    pub(super) fn from_checkpoint(s: [Value; 9]) -> Self {
        Self {
            incoming: TransmissionLine::sample_from_checkpoint([s[0], s[1], s[2], s[3], s[4]]),
            incoming_wave_slopes: [s[5], s[6]],
            outgoing_wave_slopes: [s[7], s[8]],
        }
    }
}

// The first representable receiver clock on or after the exact launch+delay.
// TwoSum retains which side owns a rounded clock; no controller tolerance is
// allowed to consume an event on its incoming side.
fn arrival(time: Value, delay: Value) -> Value {
    let high = time + delay;
    let recovered = high - time;
    let low = (time - (high - recovered)) + (delay - recovered);
    if low > 0.0 { high.next_up() } else { high }
}

impl TransmissionLine {
    pub(super) fn history_event_at(&self, time: Value) -> Option<&HistoryEvent> {
        self.history_events
            .get(
                self.history_events
                    .partition_point(|event| event.incoming.time < time),
            )
            .filter(|event| event.incoming.time == time)
    }

    pub(super) fn validate_history_events(
        state: &TransmissionLineCheckpoint,
    ) -> Result<(), String> {
        let failure = || {
            format!(
                "transmission line '{}': invalid or unanchored sided history event",
                state.name
            )
        };
        let mut previous = None;
        for event in &state.events {
            if event.iter().any(|value| !value.is_finite())
                || previous.is_some_and(|time| event[0] <= time)
            {
                return Err(failure());
            }
            let index = state
                .state_history
                .partition_point(|sample| sample[0] < event[0]);
            let Some(outgoing) = state
                .state_history
                .get(index)
                .filter(|sample| sample[0] == event[0])
            else {
                return Err(failure());
            };
            if !(event[1] + state.impedance * event[2]).is_finite()
                || !(event[3] + state.impedance * event[4]).is_finite()
                || state.forward_history[index][2].to_bits() != event[7].to_bits()
                || state.backward_history[index][2].to_bits() != event[8].to_bits()
                || outgoing[0] > state.current_time
            {
                return Err(failure());
            }
            if let Some(initial) = state.initial_state
                && initial[0] == event[0]
                && initial[1..]
                    .iter()
                    .zip(&event[1..5])
                    .any(|(a, b)| a.to_bits() != b.to_bits())
            {
                return Err(failure());
            }
            previous = Some(event[0]);
        }
        Ok(())
    }

    pub(super) fn validate_event_arrivals(
        events: &[[Value; 9]],
        delay: Value,
    ) -> Result<(), String> {
        let mut previous = None;
        for event in events {
            let clock = arrival(event[0], delay);
            if previous.is_some_and(|old| clock <= old) {
                return Err(
                    "transmission-line event arrivals are too dense for the receiver clock".into(),
                );
            }
            previous = Some(clock);
        }
        Ok(())
    }

    /// Commit a solved ideal event without integrating its jump. Existing
    /// accepted history is unchanged on invalid input. Distributed/convolution
    /// lines require their own event equations and are not admitted here.
    pub fn accept_history_event(
        &mut self,
        event: TransmissionLineHistoryEvent,
    ) -> Result<(), String> {
        if !self.td.is_finite()
            || self.td <= 0.0
            || !self.z0.is_finite()
            || self.z0 <= 0.0
            || self.txl.is_some()
            || self.has_distributed_rlgc()
            || self.is_memoryless_two_port()
            || self.attenuation != 1.0
            || self.loss_time_constant != 0.0
        {
            return Err(
                "sided transmission-line events require an ordinary lossless scalar delay".into(),
            );
        }
        let incoming = Self::sample_from_checkpoint([
            event.time,
            event.incoming[0],
            event.incoming[1],
            event.incoming[2],
            event.incoming[3],
        ]);
        let retained = HistoryEvent {
            incoming,
            incoming_wave_slopes: event.incoming_wave_slopes,
            outgoing_wave_slopes: event.outgoing_wave_slopes,
        };
        if !event.time.is_finite()
            || event.time < 0.0
            || event.time < self.current_time
            || retained
                .checkpoint()
                .iter()
                .chain(&event.outgoing)
                .any(|value| !value.is_finite())
            || self
                .history_events
                .back()
                .is_some_and(|last| last.incoming.time >= event.time)
            || [event.incoming, event.outgoing].iter().any(|port| {
                !(port[0] + self.z0 * port[1]).is_finite()
                    || !(port[2] + self.z0 * port[3]).is_finite()
            })
        {
            return Err("invalid transmission-line event values, slopes or accepted clock".into());
        }
        if let Some(last) = self.history_events.back()
            && arrival(last.incoming.time, self.td) >= arrival(event.time, self.td)
        {
            return Err(
                "transmission-line event arrivals are too dense for the receiver clock".into(),
            );
        }
        let replacing = self
            .state_history
            .back()
            .is_some_and(|last| last.time == event.time);
        if replacing {
            let previous = Self::checkpoint_sample(*self.state_history.back().unwrap());
            if previous[1..]
                .iter()
                .zip(event.incoming)
                .any(|(a, b)| a.to_bits() != b.to_bits())
            {
                return Err(
                    "transmission-line event incoming state differs from its accepted anchor"
                        .into(),
                );
            }
        }
        let was_empty = self.state_history.is_empty();
        if replacing {
            let outgoing = self.state_history.back_mut().unwrap();
            *outgoing = Self::sample_from_checkpoint([
                event.time,
                event.outgoing[0],
                event.outgoing[1],
                event.outgoing[2],
                event.outgoing[3],
            ]);
            self.launched_forward = event.outgoing[0] + self.z0 * event.outgoing[1];
            self.launched_backward = event.outgoing[2] + self.z0 * event.outgoing[3];
        } else {
            self.update_history(
                event.time,
                event.outgoing[0],
                event.outgoing[1],
                event.outgoing[2],
                event.outgoing[3],
            );
        }
        if was_empty {
            self.initial_state = Some(incoming);
        }
        self.history_forward.replace_endpoint(
            event.time,
            self.launched_forward,
            event.outgoing_wave_slopes[0],
        );
        self.history_backward.replace_endpoint(
            event.time,
            self.launched_backward,
            event.outgoing_wave_slopes[1],
        );
        self.history_events.push_back(retained);
        self.distributed_rlc_cache.set(None);
        Ok(())
    }

    /// Next exactly owned delayed event, including a slope-only corner.
    pub fn next_history_event_arrival_after(&self, time: Value) -> Result<Option<Value>, String> {
        if !time.is_finite() {
            return Err("invalid transmission-line event query clock".into());
        }
        let index = self
            .history_events
            .partition_point(|event| arrival(event.incoming.time, self.td) <= time);
        Ok(self
            .history_events
            .get(index)
            .map(|event| arrival(event.incoming.time, self.td))
            .filter(|clock| clock.is_finite()))
    }

    /// Native delayed launched wave on one side of its physical arrival.
    pub fn lossless_wave_on_side(
        &self,
        time: Value,
        forward: bool,
        side: TransmissionLineTimeSide,
    ) -> Value {
        self.history_event_limit(time, forward, side)
            .unwrap_or_else(|| self.lossless_wave_at_on_side(time - self.td, forward, side))
    }

    pub(super) fn history_event_limit(
        &self,
        time: Value,
        forward: bool,
        side: TransmissionLineTimeSide,
    ) -> Option<Value> {
        if self.history_events.is_empty() {
            return None;
        }
        let high = time - self.td;
        // The incoming equation at a rounded arrival belongs to the event,
        // even when the exact subtraction is slightly past its launch clock.
        let index = self
            .history_events
            .partition_point(|event| arrival(event.incoming.time, self.td) < time);
        if let Some(event) = self.history_events.get(index)
            && arrival(event.incoming.time, self.td) == time
        {
            return Some(self.lossless_wave_at_on_side(event.incoming.time, forward, side));
        }
        let recovered = time - high;
        let low = (time - (high + recovered)) + (recovered - self.td);
        if let Some(event) = self.history_event_at(high) {
            let selected = if low < 0.0 {
                TransmissionLineTimeSide::Incoming
            } else {
                side
            };
            let value = self.lossless_wave_at_on_side(high, forward, selected);
            let rates = if selected == TransmissionLineTimeSide::Incoming {
                event.incoming_wave_slopes
            } else {
                event.outgoing_wave_slopes
            };
            return Some(rates[usize::from(!forward)].mul_add(low, value));
        }
        None
    }

    pub(super) fn incoming_buffer_sample(
        &self,
        time: Value,
        forward: bool,
    ) -> Option<(Value, Value)> {
        self.history_event_at(time).map(|event| {
            let state = event.incoming;
            let wave = if forward {
                state.v1 + self.z0 * state.i1
            } else {
                state.v2 + self.z0 * state.i2
            };
            (wave, event.incoming_wave_slopes[usize::from(!forward)])
        })
    }
}

#[cfg(test)]
mod tests;
