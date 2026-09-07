//! Numerical methods shared by every analysis.
//!
//! What the analyses have in common below the level of any one of them: how a
//! derivative is discretized, how large a step may be, and where a step is not
//! allowed to land. These sit beneath the circuit store and the device models,
//! because those stamp into structures this module defines.
//!
//! `crate::solver` — sparse LU, the Newton loop, damping and continuation —
//! is the other half of this and sits one layer up, because it solves against
//! an assembled matrix rather than defining one.

pub(crate) mod eigenspectrum;
pub mod integration;
pub mod rustfft_qualification;

use crate::Value;

/// Authenticate an authored frequency ratio without accepting a fraction of
/// a cycle as floating-point tolerance. Zero is a separate constant case.
pub(crate) fn is_integral_cycle_count(cycles: Value) -> bool {
    cycles.is_finite()
        && cycles > 0.0
        && (cycles - cycles.round()).abs() <= (32.0 * Value::EPSILON * cycles.max(1.0)).min(1e-10)
        && cycles.round() >= 1.0
}

/// Smallest PWL feature interval, discarding interior knots in constant runs.
/// Keep both sides of ideal jumps and the complete width of a flat pulse;
/// redundant flat knots must not force an arbitrarily fine integration grid.
pub(crate) fn minimum_pwl_interval(
    points: impl IntoIterator<Item = (Value, Value)>,
) -> Option<Value> {
    let mut points = points.into_iter().peekable();
    let mut previous = points.next()?;
    let mut minimum: Option<Value> = None;
    let mut changes = false;
    while let Some(point) = points.next() {
        if point.1 == previous.1 && points.peek().is_some_and(|next| next.1 == point.1) {
            continue;
        }
        changes |= point.1 != previous.1;
        let interval = (point.0 - previous.0).abs();
        if interval > 0.0 && interval.is_finite() {
            minimum = Some(minimum.map_or(interval, |value| value.min(interval)));
        }
        previous = point;
    }
    if changes { minimum } else { None }
}

/// Map a PWL clock into its repeated tail. The authored endpoint is retained
/// at exact repeat boundaries; the next representable instant belongs to the
/// next cycle. A tolerance in seconds would flatten small waveforms and hold
/// discontinuous endpoints past their seam.
pub(crate) fn pwl_repeated_time(
    time: Value,
    first: Value,
    last: Value,
    repeat_from: Option<Value>,
) -> Value {
    let Some(start) = repeat_from.filter(|start| start.is_finite()) else {
        return time;
    };
    let start = start.max(first);
    let period = last - start;
    if !time.is_finite() || time <= last || !period.is_finite() || period <= 0.0 {
        return time;
    }
    let elapsed = time - last;
    let remainder = elapsed.rem_euclid(period);
    // Multiplication can round an authored boundary differently from modulo.
    // Authenticate its represented clock instead of widening the seam into
    // an interval, which would include an actual point after the boundary.
    let cycle = (elapsed / period).round();
    if remainder == 0.0 || (cycle.is_finite() && time == last + cycle * period) {
        last
    } else {
        start + remainder
    }
}

/// Neumaier compensated accumulation. Callers scale their operands when an
/// unscaled sum could overflow; compensation recovers low-order terms lost
/// when finite contributions of opposite sign nearly cancel.
#[inline]
pub(crate) fn compensated_add(sum: &mut Value, correction: &mut Value, value: Value) {
    let next = *sum + value;
    *correction += if sum.abs() >= value.abs() {
        (*sum - next) + value
    } else {
        (value - next) + *sum
    };
    *sum = next;
}

/// The smallest timestep Xyce will take at `current_time`, and the scale its
/// breakpoint comparisons are measured against.
///
/// Xyce derives this from the floating-point resolution of the clock itself:
/// once a step is small enough that adding it to the current time changes only
/// the last few bits, advancing it means nothing. Twice this value is the
/// tolerance for deciding whether a transient has landed *on* a waveform
/// breakpoint, which is why a source waveform needs it as much as the step
/// controller does.
///
/// Multiplying before taking the magnitude would overflow near `Value::MAX`, so
/// the magnitude comes first; a non-finite clock has no resolution to speak of
/// and yields zero.
#[must_use]
pub fn xyce_hard_min_timestep(current_time: Value) -> Value {
    if current_time.is_finite() {
        current_time.abs() * (10.0 * Value::EPSILON)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pwl_feature_width_ignores_redundant_holds_and_retains_ideal_pulses() {
        assert_eq!(
            minimum_pwl_interval([
                (0.0, 0.0),
                (1e-300, 0.0),
                (0.25, 0.0),
                (0.5, 1.0),
                (1.0, 0.0)
            ]),
            Some(0.25)
        );
        assert_eq!(
            minimum_pwl_interval([
                (0.0, 0.0),
                (0.5, 0.0),
                (0.5, 1.0),
                (0.6, 1.0),
                (0.6, 0.0),
                (1.0, 0.0)
            ]),
            Some(0.6 - 0.5)
        );
        assert_eq!(
            minimum_pwl_interval([(0.0, 1.0), (1e-300, 1.0), (1.0, 1.0)]),
            None
        );
    }

    #[test]
    fn xyce_hard_minimum_tracks_current_time_machine_precision() {
        let transition_time = 5.380_978_556_560e-4;
        assert_eq!(xyce_hard_min_timestep(0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(
            xyce_hard_min_timestep(transition_time).to_bits(),
            (transition_time * 10.0 * Value::EPSILON).to_bits()
        );
        assert_eq!(
            xyce_hard_min_timestep(-transition_time).to_bits(),
            (transition_time * 10.0 * Value::EPSILON).to_bits()
        );
        assert_eq!(xyce_hard_min_timestep(Value::NAN), 0.0);
        assert_eq!(xyce_hard_min_timestep(Value::INFINITY), 0.0);
        assert_eq!(xyce_hard_min_timestep(Value::NEG_INFINITY), 0.0);
    }

    #[test]
    fn xyce_hard_min_timestep_avoids_intermediate_overflow() {
        let minimum = xyce_hard_min_timestep(Value::MAX);

        assert!(minimum.is_finite());
        assert!(minimum > 0.0);
        assert_eq!(minimum, Value::MAX * (10.0 * Value::EPSILON));
    }
}
