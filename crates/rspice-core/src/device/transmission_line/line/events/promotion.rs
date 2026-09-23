//! Convert explicitly declared sampled edges to owned native delay events.
use super::*;

impl TransmissionLine {
    /// Each row is [event clock, incoming sample clock, outgoing sample clock].
    /// The owner supplies provenance; closeness/equality of samples is never a
    /// declaration. Smooth interpolation on each side supplies finite rates.
    /// The complete replacement is validated before any history is changed.
    pub(crate) fn promote_sampled_history_events(
        &mut self,
        edges: &[[Value; 3]],
    ) -> Result<(), String> {
        self.promote_sampled_history_events_with_endpoint_rates(edges, None)
    }

    /// An event at the newest sample has no outgoing interpolation interval.
    /// Its owner must independently solve the finite outgoing wave rates.
    pub(crate) fn promote_sampled_history_events_with_endpoint_rates(
        &mut self,
        edges: &[[Value; 3]],
        endpoint_rates: Option<[Value; 2]>,
    ) -> Result<(), String> {
        if let Some(rates) = endpoint_rates {
            if rates.iter().any(|rate| !rate.is_finite())
                || !edges.last().is_some_and(|edge| {
                    edge[0] == edge[2]
                        && self
                            .state_history
                            .back()
                            .is_some_and(|sample| sample.time == edge[0])
                })
            {
                return Err(format!(
                    "transmission line '{}': invalid sampled endpoint rate owner",
                    self.name
                ));
            }
        }
        if edges.is_empty() {
            return Ok(());
        }
        let fail = || {
            format!(
                "transmission line '{}': sampled event has missing or overlapping one-sided history",
                self.name
            )
        };
        if !self.supports_sided_history_events() {
            return Err(fail());
        }
        let mut checkpoint = self.checkpoint_state()?;
        let mut events = Vec::with_capacity(edges.len());
        let mut spans = Vec::with_capacity(edges.len());
        for (ordinal, &[time, incoming, outgoing]) in edges.iter().enumerate() {
            if ![time, incoming, outgoing]
                .iter()
                .all(|time| time.is_finite())
                || incoming >= outgoing
                || time < incoming
                || time > outgoing
                || ordinal > 0 && edges[ordinal - 1][2] >= incoming
            {
                return Err(fail());
            }
            let owned = self
                .history_events
                .partition_point(|event| event.incoming.time < incoming);
            if self
                .history_events
                .get(owned)
                .is_some_and(|event| event.incoming.time <= outgoing)
            {
                return Err(fail());
            }
            let at = |clock| {
                let index = self
                    .state_history
                    .partition_point(|sample| sample.time < clock);
                self.state_history
                    .get(index)
                    .filter(|sample| sample.time == clock)
                    .map(|_| index)
                    .ok_or_else(|| format!(
                        "transmission line '{}': declared event at {time:.17e} s is missing its sampled anchor at {clock:.17e} s",
                        self.name
                    ))
            };
            let left = at(incoming)?;
            let right = at(outgoing)?;
            // The outgoing derivative needs a same-side smooth interval. An
            // endpoint-only event must be supplied by a physical rate solver.
            let after = self.state_history.get(right + 1).map(|sample| {
                self.history_event_at(sample.time)
                    .map_or(sample, |event| &event.incoming)
            });
            if after.is_none() && endpoint_rates.is_none() {
                return Err(format!(
                    "transmission line '{}': sampled event at {time:.17e} s requires an independently resolved outgoing rate at the history endpoint",
                    self.name
                ));
            }
            if left >= right
                || edges
                    .get(ordinal + 1)
                    .is_some_and(|e| after.is_none_or(|after| after.time > e[1]))
            {
                return Err(fail());
            }
            let mut limits = [[0.0; 2]; 2];
            let mut slopes = [[0.0; 2]; 2];
            for port in 0..2 {
                let wave = |sample: &TlineStateSample| {
                    if port == 0 {
                        sample.v1 + self.z0 * sample.i1
                    } else {
                        sample.v2 + self.z0 * sample.i2
                    }
                };
                let incoming_sample = &self.state_history[left];
                let outgoing_sample = &self.state_history[right];
                let incoming_slope = if let Some(previous) =
                    left.checked_sub(1).and_then(|i| self.state_history.get(i))
                {
                    let previous2 = left
                        .checked_sub(2)
                        .and_then(|i| self.state_history.get(i))
                        .filter(|_| {
                            !self.owns_history_event(previous.time)
                                && !ordinal
                                    .checked_sub(1)
                                    .and_then(|i| edges.get(i))
                                    .is_some_and(|e| e[2] == previous.time)
                        });
                    Self::delayed_interpolate_with_slope(
                        self.lossless_interpolation_mode,
                        previous2,
                        previous,
                        incoming_sample,
                        incoming,
                        wave,
                    )
                    .1
                } else {
                    0.0
                };
                let outgoing_slope = if let Some(after) = after {
                    Self::delayed_interpolate_with_slope(
                        self.lossless_interpolation_mode,
                        None,
                        outgoing_sample,
                        after,
                        outgoing,
                        wave,
                    )
                    .1
                } else {
                    endpoint_rates.expect("validated endpoint rate owner")[port]
                };
                slopes[0][port] = incoming_slope;
                slopes[1][port] = outgoing_slope;
                limits[0][port] = incoming_slope.mul_add(time - incoming, wave(incoming_sample));
                limits[1][port] = outgoing_slope.mul_add(time - outgoing, wave(outgoing_sample));
            }
            events.push((time, limits, slopes));
            spans.push((left, right));
        }
        let mut state = Vec::with_capacity(checkpoint.state_history.len());
        let mut forward = Vec::with_capacity(checkpoint.forward_history.len());
        let mut backward = Vec::with_capacity(checkpoint.backward_history.len());
        let mut cursor = 0;
        for ((left, right), (time, limits, slopes)) in spans.into_iter().zip(events) {
            state.extend_from_slice(&checkpoint.state_history[cursor..left]);
            forward.extend_from_slice(&checkpoint.forward_history[cursor..left]);
            backward.extend_from_slice(&checkpoint.backward_history[cursor..left]);
            // Waves are the independent history coordinates. The canonical
            // V=wave, I=0 embedding retains both native port forcings exactly.
            state.push([time, limits[1][0], 0.0, limits[1][1], 0.0]);
            forward.push([time, limits[1][0], slopes[1][0]]);
            backward.push([time, limits[1][1], slopes[1][1]]);
            checkpoint.events.push([
                time,
                limits[0][0],
                0.0,
                limits[0][1],
                0.0,
                slopes[0][0],
                slopes[0][1],
                slopes[1][0],
                slopes[1][1],
            ]);
            if checkpoint.initial_state.is_some_and(|initial| {
                initial[0] >= checkpoint.state_history[left][0]
                    && initial[0] <= checkpoint.state_history[right][0]
            }) {
                checkpoint.initial_state = Some([time, limits[0][0], 0.0, limits[0][1], 0.0]);
            }
            cursor = right + 1;
        }
        state.extend_from_slice(&checkpoint.state_history[cursor..]);
        forward.extend_from_slice(&checkpoint.forward_history[cursor..]);
        backward.extend_from_slice(&checkpoint.backward_history[cursor..]);
        checkpoint.state_history = state;
        checkpoint.forward_history = forward;
        checkpoint.backward_history = backward;
        checkpoint
            .events
            .sort_by(|left, right| left[0].total_cmp(&right[0]));
        self.restore_checkpoint_state(&checkpoint)
    }
}
