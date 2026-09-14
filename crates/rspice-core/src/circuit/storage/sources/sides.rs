//! Exact one-sided source values for incoming and outgoing event equations.

use super::*;

fn before(time: Value, boundary: Value, side: SourceTimeSide) -> bool {
    time < boundary || (time == boundary && side == SourceTimeSide::LeftLimit)
}

fn interpolate(left: Value, right: Value, weight: Value) -> Value {
    if weight == 0.0 {
        return left;
    }
    if weight == 1.0 {
        return right;
    }
    rspice_veriloga_runtime::arithmetic::sum_products(
        [(left, 1.0 - weight), (right, weight)].into_iter(),
    )
    .unwrap_or(Value::NAN)
}

fn interpolate_at(time: Value, left: (Value, Value), right: (Value, Value)) -> Value {
    let weight = rspice_veriloga_runtime::arithmetic::sum_products_ratio(
        [(time, 1.0), (left.0, -1.0)].into_iter(),
        [(right.0, 1.0), (left.0, -1.0)].into_iter(),
    )
    .unwrap_or(Value::NAN);
    interpolate(left.1, right.1, weight)
}

impl VoltageSources {
    /// Shared origin for the exact PULSE event clock. Skip wholly negative
    /// cycles without enumerating them; retain any cycle with an edge at t>=0.
    pub(crate) fn first_pulse_event_cycle(
        delay: Value,
        period: Value,
        maximum_offset: Value,
    ) -> Value {
        let earliest = -maximum_offset;
        if period.is_finite() && period > 0.0 && delay < earliest {
            earliest + (delay - earliest).rem_euclid(period)
        } else {
            delay
        }
    }

    fn pulse_phase(timing: [Value; 5], time: Value, side: SourceTimeSide) -> Option<Value> {
        let [delay, rise, fall, width, period] = timing;
        let offsets = [0.0, rise, rise + width, rise + width + fall];
        let repeating = period.is_finite() && period > 0.0;
        let origin = Self::first_pulse_event_cycle(
            delay,
            period,
            offsets.into_iter().fold(Value::NEG_INFINITY, Value::max),
        );
        let center = if repeating {
            ((time - origin) / period).round()
        } else {
            0.0
        };
        let mut selected: Option<(Value, Value)> = None;
        let mut preceding_start: Option<Value> = None;
        for delta in [-1.0, 0.0, 1.0] {
            let cycle = center + delta;
            if !cycle.is_finite() || cycle < 0.0 || (!repeating && cycle != 0.0) {
                continue;
            }
            let base = if repeating {
                period.mul_add(cycle, origin)
            } else {
                origin
            };
            if base <= time && preceding_start.is_none_or(|old| base > old) {
                preceding_start = Some(base);
            }
            for (index, offset) in offsets
                .into_iter()
                .chain(std::iter::once(period))
                .enumerate()
            {
                if !offset.is_finite()
                    || offset < 0.0
                    || (repeating && offset > period)
                    || (index == 4 && !repeating)
                {
                    continue;
                }
                let event = if index == 4 {
                    period.mul_add(cycle + 1.0, origin)
                } else {
                    base + offset
                };
                if event != time {
                    continue;
                }
                let candidate = (cycle, offset);
                if selected.is_none_or(|old| {
                    if side == SourceTimeSide::LeftLimit {
                        candidate < old
                    } else {
                        candidate > old
                    }
                }) {
                    selected = Some(candidate);
                }
            }
        }
        selected
            .map(|(_, phase)| phase)
            .or_else(|| preceding_start.map(|base| time - base))
    }

    pub(super) fn pulse_limit(
        levels: [Value; 2],
        timing: [Value; 5],
        count: Value,
        time: Value,
        side: SourceTimeSide,
    ) -> Value {
        let [low, high] = levels;
        let [delay, rise, fall, width, period] = timing;
        if before(time, delay, side) || period < 0.0 {
            return low;
        }
        let elapsed = time - delay;
        let repeating = period.is_finite() && period > 0.0;
        if repeating && count > 0.0 && !before(time, delay + count * period, side) {
            return low;
        }
        let mut phase = elapsed;
        if repeating && elapsed >= period {
            let cycle = (elapsed / period).floor();
            phase = elapsed - period * cycle;
            if phase == 0.0 && side == SourceTimeSide::LeftLimit {
                phase = period;
            }
        }
        if let Some(event_phase) = Self::pulse_phase(timing, time, side) {
            phase = event_phase;
        }
        let plateau_end = rise + width;
        let end = plateau_end + fall;
        if before(phase, 0.0, side) || !before(phase, end, side) {
            low
        } else if rise != 0.0 && before(phase, rise, side) {
            interpolate(low, high, phase / rise)
        } else if before(phase, plateau_end, side) {
            high
        } else if fall != 0.0 {
            interpolate(high, low, (phase - plateau_end) / fall)
        } else {
            low
        }
    }

    pub(super) fn pwl_limit(
        points: &[(Value, Value)],
        time: Value,
        delay: Value,
        repeat_from: Option<Value>,
        side: SourceTimeSide,
    ) -> Value {
        if points.is_empty() || before(time, delay, side) {
            return 0.0;
        }
        let first = points[0].0;
        let last = points[points.len() - 1].0;
        let repeat = crate::numerics::pwl_repeat_geometry(first, last, repeat_from);
        let end_clock = last + delay;
        let mut cycle = 0.0;
        let mut base = end_clock;
        if let Some((_, period)) = repeat {
            let center = ((time - end_clock) / period).round() + 1.0;
            for delta in [-1.0, 0.0, 1.0] {
                let candidate = center + delta;
                let at = end_clock + period * (candidate - 1.0);
                if candidate.is_finite()
                    && candidate >= 1.0
                    && candidate > cycle
                    && (at < time || (at == time && side == SourceTimeSide::RightLimit))
                {
                    cycle = candidate;
                    base = at;
                }
            }
        }
        let active = if cycle > 0.0 { repeat } else { None };
        let first_index = active.map_or(0, |(start, _)| {
            points.partition_point(|point| point.0 < start)
        });
        let clock = |point: Value| {
            active.map_or(point + delay, |(start, period)| {
                crate::numerics::pwl_event_clock(
                    point + delay,
                    start + delay,
                    end_clock,
                    period,
                    cycle,
                )
            })
        };
        let tail = &points[first_index..];
        let upper = tail.partition_point(|point| clock(point.0) < time);
        if let Some(&(at, value)) = tail.get(upper)
            && clock(at) == time
        {
            return if side == SourceTimeSide::LeftLimit {
                value
            } else {
                tail[tail.partition_point(|point| clock(point.0) <= time) - 1].1
            };
        }
        if upper == 0 {
            let Some((start, _)) = active else {
                return points[0].1;
            };
            let value = Self::pwl_raw_limit(points, start, SourceTimeSide::RightLimit);
            if time <= base {
                return value;
            }
            return interpolate_at(time, (base, value), (clock(tail[0].0), tail[0].1));
        }
        if upper == tail.len() {
            return tail[upper - 1].1;
        }
        let (t0, v0) = tail[upper - 1];
        let (t1, v1) = tail[upper];
        interpolate_at(time, (clock(t0), v0), (clock(t1), v1))
    }

    fn pwl_raw_limit(points: &[(Value, Value)], time: Value, side: SourceTimeSide) -> Value {
        let upper = points.partition_point(|point| point.0 < time);
        if let Some(&(at, value)) = points.get(upper)
            && at == time
        {
            return if side == SourceTimeSide::LeftLimit {
                value
            } else {
                points[points.partition_point(|point| point.0 <= time) - 1].1
            };
        }
        if upper == 0 {
            return points[0].1;
        }
        if upper == points.len() {
            return points[upper - 1].1;
        }
        interpolate_at(time, points[upper - 1], points[upper])
    }

    pub(super) fn pwl_file_limit(
        waveform: &crate::device::pwl_file::PwlWaveform,
        time: Value,
        delay: Value,
        repeat_from: Option<Value>,
        side: SourceTimeSide,
    ) -> Value {
        if before(time, delay, side) {
            0.0
        } else {
            waveform.limit_at_repeating(
                time,
                repeat_from,
                side == SourceTimeSide::RightLimit,
                delay,
            )
        }
    }

    pub(super) fn exp_limit(
        levels: [Value; 2],
        timing: [Value; 4],
        time: Value,
        side: SourceTimeSide,
    ) -> Value {
        let [low, high] = levels;
        let [td1, tau1, td2, tau2] = timing;
        if before(time, td1, side) {
            return low;
        }
        let response = |delay: Value, tau: Value| {
            if before(time, delay, side) {
                0.0
            } else if tau == 0.0 {
                1.0
            } else {
                -(-(time - delay) / tau).exp_m1()
            }
        };
        interpolate(low, high, response(td1, tau1) - response(td2, tau2))
    }
}

#[cfg(test)]
mod tests;
