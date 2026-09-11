//! When, inside an analog interval, a voltage crossed a threshold.
//!
//! An analog solver reports a voltage at the timepoints its step controller
//! chose, and nothing makes a logic threshold one of them. The crossing
//! may lie inside the attempted interval, and dating the
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
/// crossed `threshold`, interpolated between the accepted and candidate samples.
///
/// `time` is the candidate endpoint, `time_prev` the accepted start, and
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
    if !timestep.is_finite()
        || timestep <= 0.0
        || !previous_voltage.is_finite()
        || !voltage.is_finite()
        || !threshold.is_finite()
        || voltage == previous_voltage
    {
        return time;
    }
    // Resolve extrapolation before forming differences, so even extreme finite
    // voltages cannot overflow into an incorrect endpoint fallback.
    let rising = voltage > previous_voltage;
    if (rising && threshold <= previous_voltage) || (!rising && threshold >= previous_voltage) {
        return time_prev;
    }
    if (rising && threshold >= voltage) || (!rising && threshold <= voltage) {
        return time;
    }
    let mut before = (threshold - previous_voltage).abs();
    let mut after = (voltage - threshold).abs();
    if !before.is_finite() || !after.is_finite() {
        let scale = previous_voltage
            .abs()
            .max(voltage.abs())
            .max(threshold.abs());
        before = (threshold / scale - previous_voltage / scale).abs();
        after = (voltage / scale - threshold / scale).abs();
    }
    let scale = before.max(after);
    let before = before / scale;
    let after = after / scale;
    let crossing = time - timestep * (after / (before + after));
    crossing.clamp(time_prev.min(time), time_prev.max(time))
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
    fn small_and_extreme_finite_voltages_retain_their_crossing_fraction() {
        for (previous, voltage, threshold) in [
            (0.0, 1e-300, 2.5e-301),
            (-1e308, 1e308, -5e307),
            (1e308, -1e308, 5e307),
        ] {
            let crossing = threshold_crossing_time(1e-9, 0.0, 1e-9, previous, voltage, threshold);
            assert!(
                (crossing - 0.25e-9).abs() < 1e-24,
                "{previous:e} -> {voltage:e}, threshold={threshold:e}: {crossing:e}"
            );
        }
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
