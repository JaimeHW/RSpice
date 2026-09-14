//! Exact one-sided source values for incoming and outgoing event equations.

use super::*;

mod pat;

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

fn slope(left: Value, right: Value, width: Value) -> Value {
    rspice_veriloga_runtime::arithmetic::sum_products_ratio(
        [(right, 1.0), (left, -1.0)].into_iter(),
        [(width, 1.0)].into_iter(),
    )
    .unwrap_or(Value::NAN)
}

fn segment_component<const DERIVATIVE: bool>(
    time: Value,
    left: (Value, Value),
    right: (Value, Value),
) -> Value {
    if DERIVATIVE {
        rspice_veriloga_runtime::arithmetic::sum_products_ratio(
            [(right.1, 1.0), (left.1, -1.0)].into_iter(),
            [(right.0, 1.0), (left.0, -1.0)].into_iter(),
        )
        .unwrap_or(Value::NAN)
    } else {
        interpolate_at(time, left, right)
    }
}

fn shifted_segment_value(
    time: Value,
    delay: Value,
    period: Value,
    cycle: Value,
    left: (Value, Value),
    right: (Value, Value),
) -> Value {
    // Event clocks select the interval. Evaluate the ramp in authored time:
    // adding a large delay to its endpoints must not change its duration.
    let weight = rspice_veriloga_runtime::arithmetic::sum_products_ratio(
        [(time, 1.0), (delay, -1.0), (left.0, -1.0), (period, -cycle)].into_iter(),
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

    pub(super) fn pulse_limit<const DERIVATIVE: bool>(
        levels: [Value; 2],
        timing: [Value; 5],
        count: Value,
        time: Value,
        side: SourceTimeSide,
    ) -> Value {
        let [low, high] = levels;
        let [delay, rise, fall, width, period] = timing;
        if before(time, delay, side) || period < 0.0 {
            return if DERIVATIVE { 0.0 } else { low };
        }
        let elapsed = time - delay;
        let repeating = period.is_finite() && period > 0.0;
        if repeating && count > 0.0 && !before(time, delay + count * period, side) {
            return if DERIVATIVE { 0.0 } else { low };
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
            if DERIVATIVE { 0.0 } else { low }
        } else if rise != 0.0 && before(phase, rise, side) {
            if DERIVATIVE {
                slope(low, high, rise)
            } else {
                interpolate(low, high, phase / rise)
            }
        } else if before(phase, plateau_end, side) {
            if DERIVATIVE { 0.0 } else { high }
        } else if fall != 0.0 {
            if DERIVATIVE {
                slope(high, low, fall)
            } else {
                interpolate(high, low, (phase - plateau_end) / fall)
            }
        } else {
            if DERIVATIVE { 0.0 } else { low }
        }
    }

    pub(super) fn pwl_limit<const DERIVATIVE: bool>(
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
        let upper = tail.partition_point(|point| {
            let at = clock(point.0);
            at < time || (DERIVATIVE && side == SourceTimeSide::RightLimit && at == time)
        });
        if !DERIVATIVE
            && let Some(&(at, value)) = tail.get(upper)
            && clock(at) == time
        {
            return if side == SourceTimeSide::LeftLimit {
                value
            } else {
                tail[tail.partition_point(|point| clock(point.0) <= time) - 1].1
            };
        }
        if upper == 0 {
            let Some((start, period)) = active else {
                return if DERIVATIVE { 0.0 } else { points[0].1 };
            };
            let value = Self::pwl_raw_limit(points, start, SourceTimeSide::RightLimit);
            if !DERIVATIVE && time <= base {
                return value;
            }
            if DERIVATIVE {
                return segment_component::<true>(start, (start, value), tail[0]);
            }
            return shifted_segment_value(time, delay, period, cycle, (start, value), tail[0]);
        }
        if upper == tail.len() {
            return if DERIVATIVE { 0.0 } else { tail[upper - 1].1 };
        }
        let (t0, v0) = tail[upper - 1];
        let (t1, v1) = tail[upper];
        if DERIVATIVE {
            // Absolute event clocks choose the side, but rounding a large
            // delay into those clocks must not change the authored slope.
            segment_component::<true>(time, (t0, v0), (t1, v1))
        } else {
            shifted_segment_value(
                time,
                delay,
                active.map_or(0.0, |(_, period)| period),
                cycle,
                (t0, v0),
                (t1, v1),
            )
        }
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

    pub(super) fn pwl_file_limit<const DERIVATIVE: bool>(
        waveform: &crate::device::pwl_file::PwlWaveform,
        time: Value,
        delay: Value,
        repeat_from: Option<Value>,
        side: SourceTimeSide,
    ) -> Value {
        if before(time, delay, side) {
            0.0
        } else if DERIVATIVE {
            waveform.derivative_limit_at_repeating(
                time,
                repeat_from,
                side == SourceTimeSide::RightLimit,
                delay,
            )
        } else {
            waveform.limit_at_repeating(
                time,
                repeat_from,
                side == SourceTimeSide::RightLimit,
                delay,
            )
        }
    }

    pub(super) fn exp_limit<const DERIVATIVE: bool>(
        levels: [Value; 2],
        timing: [Value; 4],
        time: Value,
        side: SourceTimeSide,
    ) -> Value {
        let [low, high] = levels;
        let [td1, tau1, td2, tau2] = timing;
        if before(time, td1, side) {
            return if DERIVATIVE { 0.0 } else { low };
        }
        if DERIVATIVE {
            use rspice_veriloga_runtime::arithmetic::ScaledValue as S;
            let regular = |delay: Value, tau: Value| {
                if before(time, delay, side) || tau == 0.0 {
                    return Ok(S::new(0.0));
                }
                let decay = S::new((-(time - delay) / tau).exp());
                S::sum_products_div(
                    [[S::new(high), decay], [S::new(-low), decay]].into_iter(),
                    S::new(tau),
                )
            };
            return regular(td1, tau1)
                .and_then(|rise| {
                    regular(td2, tau2).and_then(|fall| {
                        S::sum_products_div(
                            [[rise, S::new(1.0)], [fall.negated(), S::new(1.0)]].into_iter(),
                            S::new(1.0),
                        )
                    })
                })
                .map_or(Value::NAN, |value| value.binary64());
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
        let weight = response(td1, tau1) - response(td2, tau2);
        interpolate(low, high, weight)
    }
}

#[cfg(test)]
mod tests;
