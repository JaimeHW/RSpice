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

pub mod integration;

use crate::Value;

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
