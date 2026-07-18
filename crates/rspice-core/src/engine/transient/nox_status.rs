//! Xyce 7.10 transient NOX status-test state.
//!
//! Xyce does not reduce nonlinear convergence to one update/residual boolean.
//! Its ordered tests also return configured nonlinear return codes for near
//! convergence, tiny updates, and sustained stagnation.

use crate::Value;

const XYCE_NOX_NORMAL_CONVERGENCE_CODE: i32 = 2;
// Xyce 7.10's TIMEINT metadata defaults NLNEARCONV to zero, which remaps the
// nominal +3 near-convergence return to -3. Thus Tests 3 and 7 reject the
// timestep unless a deck explicitly opts into near-convergence acceptance.
const XYCE_NOX_NEAR_CONVERGENCE_CODE: i32 = -3;
const XYCE_NOX_SMALL_UPDATE_CODE: i32 = 4;
const XYCE_NOX_TOO_MANY_STEPS_CODE: i32 = -1;
const XYCE_NOX_UPDATE_TOO_BIG_CODE: i32 = -2;
const XYCE_NOX_STALLED_CODE: i32 = -3;
const XYCE_NOX_DEVICE_CONVERGENCE_CODE: i32 = -1;
const XYCE_NOX_NORM_TOO_SMALL_CODE: i32 = 1;

const XYCE_NOX_REQUESTED_CONVERGENCE_RATE: Value = 1.0;
const XYCE_NOX_REQUESTED_RELATIVE_CONVERGENCE_RATE: Value = 0.9;
const XYCE_NOX_MAX_CONVERGENCE_RATE: Value = 0.5 * Value::MAX;
const XYCE_NOX_STAGNATION_TOLERANCE: Value = 1.0e-3;
const XYCE_NOX_MAX_BAD_STEPS: usize = 5;
const XYCE_NOX_SMALL_UPDATE_TOLERANCE: Value = 1.0e-6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum XyceNoxDecision {
    Continue,
    Accepted { test: u8, return_code: i32 },
    Failed { test: u8, return_code: i32 },
}

#[derive(Debug, Clone, Copy)]
pub(super) struct XyceNoxSample {
    pub(super) iteration: usize,
    pub(super) residual_inf_norm: Value,
    pub(super) residual_l2_norm: Value,
    pub(super) weighted_update_norm: Option<Value>,
    pub(super) device_converged: bool,
}

#[derive(Debug, Clone)]
pub(super) struct XyceTransientNoxStatus {
    max_iterations: usize,
    initial_residual_l2_norm: Option<Value>,
    previous_residual_l2_norm: Option<Value>,
    bad_step_count: usize,
    min_convergence_rate: Value,
    last_counted_iteration: Option<usize>,
}

impl XyceTransientNoxStatus {
    pub(super) fn new(max_iterations: usize) -> Self {
        Self {
            max_iterations: max_iterations.max(1),
            initial_residual_l2_norm: None,
            previous_residual_l2_norm: None,
            bad_step_count: 0,
            min_convergence_rate: 1.0,
            last_counted_iteration: None,
        }
    }

    pub(super) fn evaluate(
        &mut self,
        sample: XyceNoxSample,
        delta_x_tolerance: Value,
        rhs_tolerance: Value,
    ) -> XyceNoxDecision {
        if !sample.residual_inf_norm.is_finite()
            || !sample.residual_l2_norm.is_finite()
            || sample
                .weighted_update_norm
                .is_some_and(|norm| !norm.is_finite())
        {
            return XyceNoxDecision::Failed {
                test: 0,
                return_code: -6,
            };
        }
        if !sample.device_converged {
            return if sample.iteration < self.max_iterations {
                XyceNoxDecision::Continue
            } else {
                XyceNoxDecision::Failed {
                    test: 8,
                    return_code: XYCE_NOX_DEVICE_CONVERGENCE_CODE,
                }
            };
        }

        let previous_norm = self
            .previous_residual_l2_norm
            .unwrap_or(sample.residual_l2_norm);
        let initial_norm = *self
            .initial_residual_l2_norm
            .get_or_insert(sample.residual_l2_norm);
        if sample.residual_l2_norm < Value::EPSILON {
            self.previous_residual_l2_norm = Some(sample.residual_l2_norm);
            return XyceNoxDecision::Accepted {
                test: 1,
                return_code: XYCE_NOX_NORM_TOO_SMALL_CODE,
            };
        }

        let weighted_update = sample.weighted_update_norm.unwrap_or(1.0);
        if sample.iteration > 0
            && weighted_update < delta_x_tolerance
            && sample.residual_inf_norm < rhs_tolerance
        {
            self.previous_residual_l2_norm = Some(sample.residual_l2_norm);
            return XyceNoxDecision::Accepted {
                test: 2,
                return_code: XYCE_NOX_NORMAL_CONVERGENCE_CODE,
            };
        }

        let current_rate = if sample.iteration == 0 {
            1.0
        } else {
            ratio_or_infinity(sample.residual_l2_norm, previous_norm)
        };
        let relative_rate = if sample.iteration == 0 {
            1.0
        } else {
            ratio_or_infinity(sample.residual_l2_norm, initial_norm)
        };
        self.previous_residual_l2_norm = Some(sample.residual_l2_norm);

        self.evaluate_after_normal_test(
            sample.iteration,
            weighted_update,
            current_rate,
            relative_rate,
        )
    }

    fn evaluate_after_normal_test(
        &mut self,
        iteration: usize,
        weighted_update: Value,
        current_rate: Value,
        relative_rate: Value,
    ) -> XyceNoxDecision {
        // Xyce Test 3: at MAXSTEP, a non-increasing last residual and at
        // least 10% total reduction return the configured near-convergence
        // code. Xyce 7.10 defaults that code to failure (-3).
        if iteration >= self.max_iterations {
            return if current_rate <= XYCE_NOX_REQUESTED_CONVERGENCE_RATE
                && relative_rate <= XYCE_NOX_REQUESTED_RELATIVE_CONVERGENCE_RATE
            {
                XyceNoxDecision::Failed {
                    test: 3,
                    return_code: XYCE_NOX_NEAR_CONVERGENCE_CODE,
                }
            } else {
                XyceNoxDecision::Failed {
                    test: 3,
                    return_code: XYCE_NOX_TOO_MANY_STEPS_CODE,
                }
            };
        }

        // Xyce Test 4: a tiny weighted update returns +4 so the time
        // integrator, rather than the nonlinear solver, decides the step.
        if iteration > 0 && weighted_update < XYCE_NOX_SMALL_UPDATE_TOLERANCE {
            return XyceNoxDecision::Accepted {
                test: 4,
                return_code: XYCE_NOX_SMALL_UPDATE_CODE,
            };
        }

        // Xyce Test 6.
        if current_rate > XYCE_NOX_MAX_CONVERGENCE_RATE {
            return XyceNoxDecision::Failed {
                test: 6,
                return_code: XYCE_NOX_UPDATE_TOO_BIG_CODE,
            };
        }

        // Xyce Test 7: count each nonlinear iteration once. The minimum rate
        // intentionally persists across resets, matching Xyce 7.10.
        if iteration == 0 {
            self.bad_step_count = 0;
            self.last_counted_iteration = Some(0);
        } else if self.last_counted_iteration != Some(iteration) {
            self.last_counted_iteration = Some(iteration);
            if (current_rate - 1.0).abs() <= XYCE_NOX_STAGNATION_TOLERANCE {
                if self.bad_step_count == 0 || current_rate < self.min_convergence_rate {
                    self.min_convergence_rate = current_rate;
                }
                self.bad_step_count += 1;
            } else {
                self.bad_step_count = 0;
            }
        }

        if self.bad_step_count >= XYCE_NOX_MAX_BAD_STEPS {
            return if relative_rate <= XYCE_NOX_REQUESTED_RELATIVE_CONVERGENCE_RATE
                && self.min_convergence_rate <= 1.0
            {
                XyceNoxDecision::Failed {
                    test: 7,
                    return_code: XYCE_NOX_NEAR_CONVERGENCE_CODE,
                }
            } else {
                XyceNoxDecision::Failed {
                    test: 7,
                    return_code: XYCE_NOX_STALLED_CODE,
                }
            };
        }

        XyceNoxDecision::Continue
    }
}

fn ratio_or_infinity(numerator: Value, denominator: Value) -> Value {
    if denominator > 0.0 && denominator.is_finite() {
        numerator / denominator
    } else {
        Value::INFINITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(
        iteration: usize,
        residual_inf_norm: Value,
        residual_l2_norm: Value,
        weighted_update_norm: Option<Value>,
    ) -> XyceNoxSample {
        XyceNoxSample {
            iteration,
            residual_inf_norm,
            residual_l2_norm,
            weighted_update_norm,
            device_converged: true,
        }
    }

    #[test]
    fn normal_convergence_requires_update_and_raw_residual() {
        let mut status = XyceTransientNoxStatus::new(20);
        assert_eq!(
            status.evaluate(sample(0, 10.0, 10.0, None), 0.33, 1.0e-2),
            XyceNoxDecision::Continue
        );
        assert_eq!(
            status.evaluate(sample(1, 5.0e-3, 1.0, Some(0.2)), 0.33, 1.0e-2),
            XyceNoxDecision::Accepted {
                test: 2,
                return_code: 2
            }
        );
    }

    #[test]
    fn max_iteration_near_convergence_rejects_by_default() {
        let mut status = XyceTransientNoxStatus::new(20);
        assert_eq!(
            status.evaluate(sample(0, 100.0, 100.0, None), 0.33, 1.0e-2),
            XyceNoxDecision::Continue
        );
        assert_eq!(
            status.evaluate(sample(20, 90.0, 90.0, Some(1.0)), 0.33, 1.0e-2),
            XyceNoxDecision::Failed {
                test: 3,
                return_code: -3
            }
        );
    }

    #[test]
    fn max_iteration_without_required_progress_fails() {
        let mut status = XyceTransientNoxStatus::new(20);
        let _ = status.evaluate(sample(0, 100.0, 100.0, None), 0.33, 1.0e-2);
        assert_eq!(
            status.evaluate(sample(20, 95.0, 95.0, Some(1.0)), 0.33, 1.0e-2),
            XyceNoxDecision::Failed {
                test: 3,
                return_code: -1
            }
        );
    }

    #[test]
    fn small_update_returns_time_integrator_status() {
        let mut status = XyceTransientNoxStatus::new(20);
        let _ = status.evaluate(sample(0, 10.0, 10.0, None), 0.33, 1.0e-2);
        assert_eq!(
            status.evaluate(sample(1, 1.0, 1.0, Some(1.0e-7)), 0.33, 1.0e-2),
            XyceNoxDecision::Accepted {
                test: 4,
                return_code: 4
            }
        );
    }

    #[test]
    fn sustained_stagnation_after_overall_progress_rejects_by_default() {
        let mut status = XyceTransientNoxStatus::new(20);
        let _ = status.evaluate(sample(0, 100.0, 100.0, None), 0.33, 1.0e-2);
        let _ = status.evaluate(sample(1, 80.0, 80.0, Some(1.0)), 0.33, 1.0e-2);
        let mut norm = 80.0;
        for iteration in 2..6 {
            norm *= 0.9995;
            assert_eq!(
                status.evaluate(sample(iteration, norm, norm, Some(1.0)), 0.33, 1.0e-2),
                XyceNoxDecision::Continue
            );
        }
        norm *= 0.9995;
        assert_eq!(
            status.evaluate(sample(6, norm, norm, Some(1.0)), 0.33, 1.0e-2),
            XyceNoxDecision::Failed {
                test: 7,
                return_code: -3
            }
        );
    }
}
