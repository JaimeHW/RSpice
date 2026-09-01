//! When, inside an accepted analog step, a voltage crossed a threshold.
//!
//! An analog solver reports a voltage at the timepoints its step controller
//! chose, and nothing makes a logic threshold one of them. The crossing
//! happened somewhere strictly inside the last accepted step, and dating the
//! consequence at the step's end instead is an error of up to a whole
//! timestep — which on a `.tran` whose step is chosen by truncation error is
//! not a small or a fixed quantity.
//!
//! Linear interpolation between the two accepted samples is what the Xyce DIG
//! devices do, and it is what every consumer here does, so it is stated once:
//! [`threshold_crossing_time`] is the arithmetic, taking plain values, and the
//! callers that have a [`CmContext`](crate::xspice::CmContext) or a mixed-signal
//! trial supply them from whatever they call their own time fields.
//!
//! # Why it takes values rather than a context
//!
//! The two original statements of this were both methods reading `ctx.time`,
//! `ctx.time_prev` and `ctx.timestep` off a code-model context — which is the
//! one thing the mixed Verilog-AMS host does not have. Passing the three
//! numbers is what lets the interleave date an A/D crossing by the same rule
//! the code models date theirs, rather than by a third transcription that
//! could drift from them.

use crate::Value;

/// The time at which a voltage moving from `previous_voltage` to `voltage`
/// crossed `threshold`, interpolated inside the accepted step.
///
/// `time` is the end of the accepted step, `time_prev` its start, and
/// `timestep` its length. The answer is clamped to the closed step interval,
/// so it can never date a consequence outside the step that produced it.
///
/// # When it declines to interpolate
///
/// Returning `time` — the end of the step, which is what an uninterpolated
/// sample would have said — is the answer whenever the interpolation has no
/// meaning to compute:
///
/// * a non-finite or non-positive timestep, which is the operating point and
///   the first transient point, where there is no interval to interpolate in;
/// * a non-finite voltage or threshold, where the arithmetic would propagate a
///   NaN into an event time;
/// * a voltage that did not move, where the line through the two samples is
///   horizontal and crosses the threshold either nowhere or everywhere.
///
/// Declining is not a failure and is not reported as one: an event dated at
/// the end of the step is exactly the behaviour of a sampler that does not
/// interpolate, so the caller gets the conservative answer rather than an
/// error it would have to invent a recovery for.
pub(crate) fn threshold_crossing_time(
    time: Value,
    time_prev: Value,
    timestep: Value,
    previous_voltage: Value,
    voltage: Value,
    threshold: Value,
) -> Value {
    let denominator = voltage - previous_voltage;
    if !timestep.is_finite()
        || timestep <= 0.0
        || !previous_voltage.is_finite()
        || !voltage.is_finite()
        || !threshold.is_finite()
        || denominator.abs() <= Value::EPSILON
    {
        return time;
    }

    // How far back from the end of the step the straight line between the two
    // samples was at the threshold.
    let delta = timestep * (voltage - threshold) / denominator;
    let crossing = time - delta;
    if crossing.is_finite() {
        crossing.clamp(time_prev.min(time), time_prev.max(time))
    } else {
        time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_crossing_is_dated_where_the_line_meets_the_threshold() {
        // 0 V to 1 V across 1 ns, threshold 0.25 V: a quarter of the way in.
        let crossing = threshold_crossing_time(1.0e-9, 0.0, 1.0e-9, 0.0, 1.0, 0.25);
        assert!(
            (crossing - 0.25e-9).abs() < 1.0e-24,
            "expected 0.25 ns, got {crossing:e}"
        );

        // Falling is the same line read the other way.
        let crossing = threshold_crossing_time(2.0e-9, 1.0e-9, 1.0e-9, 1.0, 0.0, 0.75);
        assert!(
            (crossing - 1.25e-9).abs() < 1.0e-24,
            "expected 1.25 ns, got {crossing:e}"
        );
    }

    #[test]
    fn a_crossing_is_clamped_into_the_step_that_produced_it() {
        // A threshold already passed at the start of the step extrapolates
        // backwards; the answer is the start of the step, never before it.
        let crossing = threshold_crossing_time(2.0e-9, 1.0e-9, 1.0e-9, 2.0, 3.0, 0.5);
        assert!(
            (crossing - 1.0e-9).abs() < 1.0e-24,
            "expected the step start, got {crossing:e}"
        );

        // A threshold not yet reached extrapolates forwards; the answer is the
        // end of the step.
        let crossing = threshold_crossing_time(2.0e-9, 1.0e-9, 1.0e-9, 0.0, 0.1, 0.5);
        assert!(
            (crossing - 2.0e-9).abs() < 1.0e-24,
            "expected the step end, got {crossing:e}"
        );
    }

    #[test]
    fn a_step_with_nothing_to_interpolate_reports_its_own_end() {
        for (timestep, previous, voltage, threshold) in [
            (0.0, 0.0, 1.0, 0.5),
            (-1.0e-9, 0.0, 1.0, 0.5),
            (f64::NAN, 0.0, 1.0, 0.5),
            (1.0e-9, 0.5, 0.5, 0.5),
            (1.0e-9, f64::NAN, 1.0, 0.5),
            (1.0e-9, 0.0, f64::INFINITY, 0.5),
            (1.0e-9, 0.0, 1.0, f64::NAN),
        ] {
            assert_eq!(
                threshold_crossing_time(2.0e-9, 1.0e-9, timestep, previous, voltage, threshold),
                2.0e-9,
                "timestep {timestep:e} previous {previous} voltage {voltage} threshold {threshold}"
            );
        }
    }
}
