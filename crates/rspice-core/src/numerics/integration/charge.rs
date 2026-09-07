//! Discretization of a physical branch charge, independent of its device or
//! analysis. A driver owns accepted history; trial evaluation only borrows it.

use super::CompanionCoefficients;
use crate::Value;

/// Accepted history for one oriented physical charge Q, in coulombs. The
/// current is dQ/dt in amperes with the same terminal orientation. These are
/// accepted time points, never intermediate Newton iterates. Copying this
/// small value into a trial leaves commit, rejection and checkpoint ownership
/// with the driver.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct BranchChargeHistory {
    pub q_prev: Value,
    pub q_prev_prev: Value,
    pub cq_prev: Value,
}

/// Scale a finite charge or charge derivative without overflowing an
/// intermediate product when the final quotient is representable.
#[inline]
fn scaled_rate(coefficient: Value, value: Value, dt: Value) -> Value {
    let rate = coefficient * value / dt;
    if rate.is_finite() {
        rate
    } else {
        coefficient * (value / dt)
    }
}

/// Discretize dQ/dt using accepted history. `dt` must be finite and positive,
/// and the coefficients must describe the selected integration method.
/// Charge differences preserve invariance to the arbitrary additive origin
/// of Q and avoid subtracting separately scaled large absolute charges.
#[inline]
pub(crate) fn integrated_charge_current(
    coeff: &CompanionCoefficients,
    dt: Value,
    charge: Value,
    history: BranchChargeHistory,
) -> Value {
    let difference_rate = |coefficient, left: Value, right: Value| {
        let difference = left - right;
        if difference.is_finite() {
            scaled_rate(coefficient, difference, dt)
        } else {
            coefficient * (left / dt - right / dt)
        }
    };
    let mut current = difference_rate(coeff.coeff_g, charge, history.q_prev);
    if coeff.needs_two_history {
        current -= difference_rate(coeff.coeff_v_n_minus_1, history.q_prev_prev, history.q_prev);
    }
    if coeff.coeff_i_n != 0.0 {
        current -= coeff.coeff_i_n * history.cq_prev;
    }
    current
}

/// Linearize the discretized charge current at `(voltage, charge, dQ/dV)`.
/// Returns `(conductance, source, charge, current)` with branch convention
/// `I(V) ~= conductance * V - source`. This supplies the Q part of
/// `F(x,t) + dQ(x,t)/dt = 0`; the device supplies F and its Jacobian separately.
///
/// A zero or negative dQ/dV is valid here: it does not erase accepted charge
/// or its displacement current. This kernel makes no passivity assumption
/// about an individual branch of a multi-terminal device. Nonfinite device
/// values remain nonfinite so the enclosing solve can diagnose them.
#[inline]
pub(crate) fn nonlinear_charge_companion_terms(
    coeff: &CompanionCoefficients,
    dt: Value,
    charge_slope: Value,
    voltage: Value,
    charge: Value,
    history: BranchChargeHistory,
) -> (Value, Value, Value, Value) {
    let conductance = scaled_rate(coeff.coeff_g, charge_slope, dt);
    let current = integrated_charge_current(coeff, dt, charge, history);
    let source = conductance * voltage - current;
    (conductance, source, charge, current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_charge_slope_does_not_erase_displacement_current() {
        // Q(V)=V^3 at V=0 has dQ/dV=0, but the previous Q=1 must discharge.
        let history = BranchChargeHistory {
            q_prev: 1.0,
            q_prev_prev: 1.0,
            cq_prev: 0.0,
        };
        let terms = nonlinear_charge_companion_terms(
            &CompanionCoefficients::backward_euler(),
            0.25,
            0.0,
            0.0,
            0.0,
            history,
        );
        assert_eq!(terms, (0.0, 4.0, 0.0, -4.0));
    }

    #[test]
    fn charge_origin_and_terminal_orientation_do_not_change_the_physics() {
        for coeff in [
            CompanionCoefficients::backward_euler(),
            CompanionCoefficients::trapezoidal(),
            CompanionCoefficients::gear2_variable_step(0.01, 0.007),
        ] {
            for origin in [0.0, 1e16, -1e16] {
                for sign in [-1.0, 1.0] {
                    let history = BranchChargeHistory {
                        q_prev: origin + sign * 4.0,
                        q_prev_prev: origin + sign * 2.0,
                        cq_prev: sign * 3.0,
                    };
                    let current =
                        integrated_charge_current(&coeff, 0.01, origin + sign * 6.0, history);
                    let reference = integrated_charge_current(
                        &coeff,
                        0.01,
                        6.0,
                        BranchChargeHistory {
                            q_prev: 4.0,
                            q_prev_prev: 2.0,
                            cq_prev: 3.0,
                        },
                    );
                    assert_eq!(current, sign * reference);
                }
            }
        }
    }

    #[test]
    fn signed_charge_jacobian_matches_a_numerical_derivative() {
        let coeff = CompanionCoefficients::gear2_variable_step(0.1, 0.07);
        let history = BranchChargeHistory {
            q_prev: 0.3,
            q_prev_prev: 0.2,
            cq_prev: 0.1,
        };
        for voltage in [-0.7_f64, 0.0, 0.7] {
            let charge = |voltage: Value| -voltage + voltage.powi(3);
            let (geq, source, _, current) = nonlinear_charge_companion_terms(
                &coeff,
                0.1,
                -1.0 + 3.0 * voltage * voltage,
                voltage,
                charge(voltage),
                history,
            );
            let delta = 1e-6;
            let finite_difference =
                (integrated_charge_current(&coeff, 0.1, charge(voltage + delta), history)
                    - integrated_charge_current(&coeff, 0.1, charge(voltage - delta), history))
                    / (2.0 * delta);
            assert!((geq - finite_difference).abs() < 1e-8);
            assert!((geq * voltage - source - current).abs() < 1e-14);
        }
    }

    #[test]
    fn finite_rates_survive_overflowing_intermediate_charge_arithmetic() {
        let history = BranchChargeHistory {
            q_prev: -Value::MAX,
            q_prev_prev: -Value::MAX,
            cq_prev: 0.0,
        };
        assert_eq!(
            integrated_charge_current(
                &CompanionCoefficients::backward_euler(),
                Value::MAX,
                Value::MAX,
                history
            ),
            2.0
        );
        let terms = nonlinear_charge_companion_terms(
            &CompanionCoefficients::trapezoidal(),
            Value::MAX,
            Value::MAX,
            1.0,
            0.0,
            BranchChargeHistory {
                q_prev: 0.0,
                q_prev_prev: 0.0,
                cq_prev: 0.0,
            },
        );
        assert_eq!(terms.0, 2.0);
    }
}
