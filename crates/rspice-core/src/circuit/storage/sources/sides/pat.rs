//! PAT event-side evaluation using the scheduler's original physical clock.

use super::*;

impl VoltageSources {
    pub(in crate::circuit::storage::sources) fn pat_limit<const DERIVATIVE: bool>(
        levels: [Value; 2],
        timing: [Value; 4],
        data: &str,
        repeat_count: i32,
        time: Value,
        side: SourceTimeSide,
    ) -> Value {
        let [high, low] = levels;
        let [delay, rise, fall, sample] = timing;
        let Some((first, last, count)) = Self::pat_data_shape(data) else {
            return Value::NAN;
        };
        if ![high, low, delay, rise, fall, sample, time]
            .into_iter()
            .all(Value::is_finite)
            || rise <= 0.0
            || fall <= 0.0
            || sample <= 0.0
        {
            return Value::NAN;
        }
        let duration = count as Value * sample;
        if !duration.is_finite() || duration <= 0.0 {
            return Value::NAN;
        }
        let first_value = if first == b'1' { high } else { low };
        let last_value = if last == b'1' { high } else { low };
        let first_plateau = 0.5 * if first == b'1' { rise } else { fall };
        let last_plateau = duration - 0.5 * if last == b'1' { fall } else { rise };
        let initial_hold = before(time, delay + first_plateau, side);
        let final_hold = repeat_count >= 0
            && !before(
                time,
                (delay + Value::from(repeat_count) * duration) + last_plateau,
                side,
            );

        // Bounded neighbouring-cycle lookup avoids advancing the query time
        // or enumerating every earlier pattern. Keep the addition order used
        // by the source-event scheduler.
        let center = ((time - delay) / duration).round();
        let mut cycle = 0.0;
        for delta in [-1.0, 0.0, 1.0] {
            let candidate = center + delta;
            let start = delay + candidate * duration;
            if candidate.is_finite() && candidate > cycle && !before(time, start, side) {
                cycle = candidate;
            }
        }
        let base = delay + cycle * duration;
        let clock = |at: Value| {
            if at == duration {
                // The scheduler also emits the next cycle's first point.
                // Give this shared seam one clock: (base + duration) can
                // round differently and otherwise create a spurious hold.
                delay + (cycle + 1.0) * duration
            } else {
                base + at
            }
        };
        let mut previous: Option<(Value, Value)> = None;
        let mut selected = None;
        let mut valid = true;
        Self::visit_pat_points(high, low, rise, fall, sample, data, |at, value| {
            valid &= at.is_finite()
                && value.is_finite()
                && at >= 0.0
                && at <= duration
                && previous.is_none_or(|point| point.0 <= at);
            if selected.is_none() && before(time, clock(at), side) {
                selected = Some(if DERIVATIVE {
                    previous.map_or(0.0, |left| segment_component::<true>(at, left, (at, value)))
                } else if time == clock(at) {
                    value
                } else {
                    previous.map_or(value, |left| {
                        if time == clock(left.0) {
                            left.1
                        } else {
                            shifted_segment_value(time, delay, duration, cycle, left, (at, value))
                        }
                    })
                });
            }
            previous = Some((at, value));
        });
        if !valid {
            return Value::NAN;
        }
        if initial_hold {
            return if DERIVATIVE { 0.0 } else { first_value };
        }
        if final_hold {
            return if DERIVATIVE { 0.0 } else { last_value };
        }
        selected.unwrap_or_else(|| if DERIVATIVE { 0.0 } else { previous.unwrap().1 })
    }
}
