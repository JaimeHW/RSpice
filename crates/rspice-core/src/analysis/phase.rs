//! Bounded phase arithmetic shared by frequency-domain measurements.

use crate::Value;

/// Shortest signed change, preserving the sign of a half-turn tie. Reducing
/// each operand first also avoids overflow for finite but very large phases.
/// Unwrapping assumes adjacent samples differ by at most half a turn.
pub(super) fn difference(from: Value, to: Value, period: Value) -> Option<Value> {
    if !from.is_finite() || !to.is_finite() || !period.is_finite() || period <= 0.0 {
        return None;
    }
    let delta = (to % period) - (from % period);
    let mut reduced = delta % period;
    if reduced > period / 2.0 {
        reduced -= period;
    } else if reduced < -period / 2.0 {
        reduced += period;
    }
    Some(reduced)
}

pub(super) fn group_delay(f0: Value, p0: Value, f1: Value, p1: Value) -> Option<Value> {
    if !f0.is_finite() || !f1.is_finite() || f0 < 0.0 || f1 <= f0 {
        return None;
    }
    let delta = difference(p0, p1, std::f64::consts::TAU)?;
    let delay = -(delta / std::f64::consts::TAU) / (f1 - f0);
    delay.is_finite().then_some(delay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_difference_crosses_the_branch_cut_without_unbounded_iteration() {
        assert_eq!(difference(-170.0, 170.0, 360.0), Some(-20.0));
        assert_eq!(difference(-10.0, 10.0, 360.0), Some(20.0));
        for (from, to) in [(0.0, 1e20), (-Value::MAX, Value::MAX)] {
            let delta = difference(from, to, std::f64::consts::TAU).unwrap();
            assert!(delta.abs() <= std::f64::consts::PI);
        }
        assert_eq!(difference(0.0, Value::NAN, 360.0), None);
        assert_eq!(group_delay(1.0, 0.0, 1.0, 1.0), None);
    }
}
