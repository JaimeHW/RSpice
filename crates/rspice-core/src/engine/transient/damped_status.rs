//! Xyce 7.10 transient DampedNewton status tests.
//!
//! Xyce has two different nonlinear contracts in transient analysis.  The
//! NOX path is represented by [`super::nox_status`]; the default path is the
//! older `DampedNewton` solver.  Keeping this state machine separate is
//! important because DampedNewton uses a frozen solution-weight vector,
//! evaluates the ordinary convergence tests in a different order, and keeps
//! its stagnation counter across timepoint solves.
//!
//! This module contains no circuit or timestep policy.  The transient driver
//! supplies the residual norms, the actual weighted Newton update, and the
//! device-status result after each candidate.  A caller should create one
//! status object for the whole transient run, call [`XyceTransientDampedStatus::begin_solve`]
//! before each attempted timepoint, and call [`XyceTransientDampedStatus::evaluate`]
//! after every Newton candidate.

use crate::Value;

const XYCE_DAMPED_NORMAL_CONVERGENCE_CODE: i32 = 2;
const XYCE_DAMPED_NEAR_CONVERGENCE_CODE: i32 = -3;
const XYCE_DAMPED_SMALL_UPDATE_CODE: i32 = 4;
const XYCE_DAMPED_TOO_MANY_STEPS_CODE: i32 = -1;
const XYCE_DAMPED_UPDATE_TOO_BIG_CODE: i32 = -2;
const XYCE_DAMPED_STALLED_CODE: i32 = -3;
const XYCE_DAMPED_LINEAR_SOLVER_FAILED_CODE: i32 = -9;
const XYCE_DAMPED_NAN_CODE: i32 = -6;
const XYCE_DAMPED_NORM_TOO_SMALL_CODE: i32 = 1;
const XYCE_DAMPED_DEVICE_CONVERGENCE_CODE: i32 = -1;

const XYCE_DAMPED_MIN_RESIDUAL_REDUCTION: Value = 0.9;
const XYCE_DAMPED_MAX_CONVERGENCE_RATE: Value = 0.5 * Value::MAX;
const XYCE_DAMPED_STAGNATION_TOLERANCE: Value = 1.0e-3;
const XYCE_DAMPED_MAX_BAD_STEPS: usize = 5;
const XYCE_DAMPED_SMALL_UPDATE_TOLERANCE: Value = 1.0e-6;

/// The return-code policy configured by Xyce's `TIMEINT` options.
///
/// Xyce 7.10 defaults `NLNEARCONV=0` (near convergence rejects the
/// timepoint) and `NLSMALLUPDATE=1` (a small update is accepted so the time
/// integrator can make the LTE decision).  The policy is explicit here so a
/// future parser can expose those options without changing the state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct XyceDampedReturnCodes {
    pub(super) near_convergence: i32,
    pub(super) small_update: i32,
}

impl Default for XyceDampedReturnCodes {
    fn default() -> Self {
        Self {
            near_convergence: XYCE_DAMPED_NEAR_CONVERGENCE_CODE,
            small_update: XYCE_DAMPED_SMALL_UPDATE_CODE,
        }
    }
}

/// Result of one DampedNewton status evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum XyceDampedDecision {
    /// Continue Newton iterations for the current attempted timepoint.
    Continue,
    /// The nonlinear solve succeeded.  The transient LTE policy still owns
    /// the final accept/reject decision for positive return codes.
    Accepted { test: u8, return_code: i32 },
    /// The nonlinear solve failed and the timestep must be retried or
    /// reported as failed by the transient driver.
    Failed { test: u8, return_code: i32 },
}

/// Persistent DampedNewton phase at an accepted transient boundary.
///
/// Xyce retains only the stagnation count and its best observed convergence
/// rate across nonlinear solves.  The residual references belong to the
/// completed attempt and are deliberately absent: both [`XyceTransientDampedStatus::begin_solve`]
/// entry points replace them before the next candidate is evaluated.
///
/// The transient driver's separate `first_solver_call` flag controls frozen
/// solution-weight construction rather than this status machine.  An
/// enclosing transient checkpoint must therefore persist that flag alongside
/// this payload.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct XyceDampedAcceptedBoundaryCheckpoint {
    pub(super) bad_step_count: usize,
    pub(super) min_convergence_rate: Value,
}

impl Default for XyceDampedAcceptedBoundaryCheckpoint {
    fn default() -> Self {
        Self {
            bad_step_count: 0,
            min_convergence_rate: 1.0,
        }
    }
}

impl XyceDampedAcceptedBoundaryCheckpoint {
    /// Validate the complete persistent DampedNewton payload before restore.
    pub(super) fn validate(&self) -> Result<(), String> {
        if self.bad_step_count >= XYCE_DAMPED_MAX_BAD_STEPS {
            return Err(format!(
                "DampedNewton checkpoint bad-step count {} must be less than {}",
                self.bad_step_count, XYCE_DAMPED_MAX_BAD_STEPS
            ));
        }

        if !self.min_convergence_rate.is_finite() {
            return Err(
                "DampedNewton checkpoint minimum convergence rate must be finite".to_string(),
            );
        }

        let minimum_rate = 1.0 - XYCE_DAMPED_STAGNATION_TOLERANCE;
        let maximum_rate = 1.0 + XYCE_DAMPED_STAGNATION_TOLERANCE;
        if !(minimum_rate..=maximum_rate).contains(&self.min_convergence_rate) {
            return Err(format!(
                "DampedNewton checkpoint minimum convergence rate {} is outside the stagnation interval [{minimum_rate}, {maximum_rate}]",
                self.min_convergence_rate
            ));
        }

        Ok(())
    }
}

/// Data observed after one DampedNewton candidate.
///
/// `newton_step` is the one-based Xyce `nlStep_` value.  The update norm must
/// be the actual weighted update (`wtNormDX / stepLength`), including any
/// line-search or damping factor.  Residual norms are the norms of the fresh
/// RHS evaluated at the candidate: `residual_inf_norm` is the max norm and
/// `residual_l2_norm` is the 2-norm.
#[derive(Debug, Clone, Copy)]
pub(super) struct XyceDampedSample {
    pub(super) newton_step: usize,
    pub(super) residual_inf_norm: Value,
    pub(super) residual_l2_norm: Value,
    pub(super) weighted_update_norm: Value,
    /// Pass `true` when `ENFORCEDEVICECONV` is disabled for this solve.
    pub(super) device_converged: bool,
    /// Xyce always requires inner-device convergence.  RSpice's one-level
    /// transient path can pass `true` because it has no separate inner solve.
    pub(super) inner_device_converged: bool,
    /// Whether the linear solve produced a usable candidate.
    pub(super) linear_solve_ok: bool,
    /// Distinguish a NaN/Inf failure (-6) from an ordinary linear-solver
    /// failure (-9), matching Xyce's return-code contract.
    pub(super) linear_solve_nan: bool,
}

/// Build Xyce DampedNewton's frozen transient solution weights.
///
/// Xyce calls `updateWeights_()` once at the start of each nonlinear solve,
/// then reuses the vector for every Newton correction at that timepoint.  On
/// the first solver call, an all-zero predictor uses a uniform
/// `RELTOL + ABSTOL` weight; all later calls use
/// `RELTOL * max(abs(predictor), abs(accepted)) + ABSTOL` per unknown.
///
/// The caller owns masking semantics (`USEMASKING`), which can replace a
/// selected weight with `MachineBig` after this function returns.
pub(super) fn xyce_damped_transient_weights(
    predictor: &[Value],
    accepted: &[Value],
    reltol: Value,
    abstol: Value,
    first_solver_call: bool,
) -> Option<Vec<Value>> {
    if predictor.len() != accepted.len()
        || !reltol.is_finite()
        || !abstol.is_finite()
        || reltol < 0.0
        || abstol < 0.0
        || reltol + abstol <= 0.0
        || predictor
            .iter()
            .chain(accepted)
            .any(|value| !value.is_finite())
    {
        return None;
    }

    let predictor_inf_norm = predictor
        .iter()
        .fold(0.0_f64, |norm, &value| norm.max(value.abs()));
    if first_solver_call && predictor_inf_norm <= Value::MIN_POSITIVE {
        return Some(vec![reltol + abstol; predictor.len()]);
    }

    Some(
        predictor
            .iter()
            .zip(accepted)
            .map(|(&next, &current)| reltol * next.abs().max(current.abs()) + abstol)
            .collect(),
    )
}

/// Xyce's transient DampedNewton status state.
///
/// The residual reference (`initial_residual_l2_norm` and
/// `previous_residual_l2_norm`) is reset for each attempted timepoint.  The
/// stagnation counter and its best rate deliberately persist across solves,
/// matching the corresponding fields in Xyce's `DampedNewton` object.
#[derive(Debug, Clone)]
pub(super) struct XyceTransientDampedStatus {
    max_iterations: usize,
    return_codes: XyceDampedReturnCodes,
    initial_residual_l2_norm: Option<Value>,
    previous_residual_l2_norm: Option<Value>,
    bad_step_count: usize,
    min_convergence_rate: Value,
}

impl XyceTransientDampedStatus {
    /// Create a status object with Xyce 7.10's default return-code policy.
    pub(super) fn new(max_iterations: usize) -> Self {
        Self::with_return_codes(max_iterations, XyceDampedReturnCodes::default())
    }

    /// Create a status object with explicit `NLNEARCONV`/`NLSMALLUPDATE`
    /// return codes.
    pub(super) fn with_return_codes(
        max_iterations: usize,
        return_codes: XyceDampedReturnCodes,
    ) -> Self {
        Self {
            max_iterations: max_iterations.max(1),
            return_codes,
            initial_residual_l2_norm: None,
            previous_residual_l2_norm: None,
            bad_step_count: 0,
            min_convergence_rate: 1.0,
        }
    }

    /// Start a new attempted timepoint.
    ///
    /// Xyce changes `MAXSTEP` with the active transient settings, so the
    /// budget is supplied for every attempt.  Stagnation state is intentionally
    /// retained across this reset; `DampedNewton::resetCountersAndTimers_()`
    /// does not clear those fields.
    pub(super) fn begin_solve(&mut self, max_iterations: usize) {
        self.max_iterations = max_iterations.max(1);
        self.initial_residual_l2_norm = None;
        self.previous_residual_l2_norm = None;
    }

    /// Start a solve and seed the residual reference with the predictor RHS.
    ///
    /// Xyce evaluates the predictor once before entering its Newton loop.  It
    /// does not run the convergence tests at that point, but it does use that
    /// RHS norm as both `normRHS_init_` and `normRHS_old_` for the first
    /// corrected candidate.
    pub(super) fn begin_solve_with_initial_residual(
        &mut self,
        max_iterations: usize,
        initial_residual_l2_norm: Value,
    ) {
        self.begin_solve(max_iterations);
        if initial_residual_l2_norm.is_finite() && initial_residual_l2_norm >= 0.0 {
            self.initial_residual_l2_norm = Some(initial_residual_l2_norm);
            self.previous_residual_l2_norm = Some(initial_residual_l2_norm);
        }
    }

    /// Capture the persistent status at an accepted transient boundary.
    ///
    /// The caller establishes that the attempted timepoint was accepted.  Any
    /// residual references still present describe that completed attempt and
    /// are intentionally ignored because the next `begin_solve*` call replaces
    /// them unconditionally.  Return-code policy and the iteration budget are
    /// live configuration, not mutable continuation phase, and remain owned by
    /// the status object reconstructed for the resumed analysis.
    pub(super) fn capture_accepted_boundary_checkpoint(
        &self,
    ) -> Result<XyceDampedAcceptedBoundaryCheckpoint, String> {
        let checkpoint = XyceDampedAcceptedBoundaryCheckpoint {
            bad_step_count: self.bad_step_count,
            min_convergence_rate: self.min_convergence_rate,
        };
        checkpoint.validate()?;
        Ok(checkpoint)
    }

    /// Restore the persistent phase before beginning the next nonlinear solve.
    ///
    /// Validation completes before any live state is modified.  Attempt-local
    /// residual references are cleared defensively so a restored object cannot
    /// accidentally reuse norms from work performed before the checkpoint.
    pub(super) fn restore_accepted_boundary_checkpoint(
        &mut self,
        checkpoint: &XyceDampedAcceptedBoundaryCheckpoint,
    ) -> Result<(), String> {
        checkpoint.validate()?;

        self.bad_step_count = checkpoint.bad_step_count;
        self.min_convergence_rate = checkpoint.min_convergence_rate;
        self.initial_residual_l2_norm = None;
        self.previous_residual_l2_norm = None;
        Ok(())
    }

    /// Evaluate one candidate using Xyce's ordered DampedNewton tests.
    pub(super) fn evaluate(
        &mut self,
        sample: XyceDampedSample,
        delta_x_tolerance: Value,
        rhs_tolerance: Value,
    ) -> XyceDampedDecision {
        if !sample.linear_solve_ok {
            return XyceDampedDecision::Failed {
                test: 0,
                return_code: if sample.linear_solve_nan {
                    XYCE_DAMPED_NAN_CODE
                } else {
                    XYCE_DAMPED_LINEAR_SOLVER_FAILED_CODE
                },
            };
        }

        if !sample.device_converged {
            return if sample.newton_step < self.max_iterations {
                XyceDampedDecision::Continue
            } else {
                XyceDampedDecision::Failed {
                    test: 8,
                    return_code: XYCE_DAMPED_DEVICE_CONVERGENCE_CODE,
                }
            };
        }

        if !sample.inner_device_converged {
            return XyceDampedDecision::Continue;
        }

        if !sample.residual_inf_norm.is_finite()
            || !sample.residual_l2_norm.is_finite()
            || !sample.weighted_update_norm.is_finite()
            || sample.residual_inf_norm < 0.0
            || sample.residual_l2_norm < 0.0
            || sample.weighted_update_norm < 0.0
            || !delta_x_tolerance.is_finite()
            || !rhs_tolerance.is_finite()
            || delta_x_tolerance < 0.0
            || rhs_tolerance < 0.0
        {
            return XyceDampedDecision::Failed {
                test: 0,
                return_code: XYCE_DAMPED_NAN_CODE,
            };
        }

        let previous_norm = self
            .previous_residual_l2_norm
            .unwrap_or(sample.residual_l2_norm);
        let initial_norm = *self
            .initial_residual_l2_norm
            .get_or_insert(sample.residual_l2_norm);

        // Xyce checks this before forming either convergence rate.  Preserve
        // the current value as the previous norm for the next candidate.
        if sample.residual_l2_norm < Value::EPSILON {
            self.previous_residual_l2_norm = Some(sample.residual_l2_norm);
            return XyceDampedDecision::Accepted {
                test: 1,
                return_code: XYCE_DAMPED_NORM_TOO_SMALL_CODE,
            };
        }

        let relative_rate = ratio_or_infinity(sample.residual_l2_norm, initial_norm);
        let convergence_rate = ratio_or_infinity(sample.residual_l2_norm, previous_norm);
        self.previous_residual_l2_norm = Some(sample.residual_l2_norm);

        // Normal convergence is intentionally inclusive in Xyce.
        if sample.residual_inf_norm <= rhs_tolerance
            && sample.weighted_update_norm <= delta_x_tolerance
        {
            return XyceDampedDecision::Accepted {
                test: 2,
                return_code: XYCE_DAMPED_NORMAL_CONVERGENCE_CODE,
            };
        }

        // The transient near-convergence test runs before small-update and
        // max-step tests.  The default return code is -3 because Xyce's
        // TIMEINT metadata sets NLNEARCONV=0.
        if sample.newton_step >= self.max_iterations
            && relative_rate <= XYCE_DAMPED_MIN_RESIDUAL_REDUCTION
            && convergence_rate <= 1.0
        {
            return if self.return_codes.near_convergence > 0 {
                XyceDampedDecision::Accepted {
                    test: 3,
                    return_code: self.return_codes.near_convergence,
                }
            } else {
                XyceDampedDecision::Failed {
                    test: 3,
                    return_code: self.return_codes.near_convergence,
                }
            };
        }

        // Small updates are positive by default; the transient integrator
        // decides whether the resulting LTE permits acceptance.
        if sample.weighted_update_norm <= XYCE_DAMPED_SMALL_UPDATE_TOLERANCE {
            return if self.return_codes.small_update > 0 {
                XyceDampedDecision::Accepted {
                    test: 4,
                    return_code: self.return_codes.small_update,
                }
            } else {
                XyceDampedDecision::Failed {
                    test: 4,
                    return_code: self.return_codes.small_update,
                }
            };
        }

        if sample.newton_step >= self.max_iterations {
            return XyceDampedDecision::Failed {
                test: 5,
                return_code: XYCE_DAMPED_TOO_MANY_STEPS_CODE,
            };
        }

        if convergence_rate > XYCE_DAMPED_MAX_CONVERGENCE_RATE {
            return XyceDampedDecision::Failed {
                test: 6,
                return_code: XYCE_DAMPED_UPDATE_TOO_BIG_CODE,
            };
        }

        // Count each stagnating Newton step.  A non-stagnating step resets the
        // count, while the best rate is retained until the five-step test
        // fires, matching DampedNewton's `count`/`tmpConvRate` fields.
        if (convergence_rate - 1.0).abs() <= XYCE_DAMPED_STAGNATION_TOLERANCE {
            if self.bad_step_count == 0 || convergence_rate < self.min_convergence_rate {
                self.min_convergence_rate = convergence_rate;
            }
            self.bad_step_count += 1;
        } else {
            self.bad_step_count = 0;
        }

        if self.bad_step_count >= XYCE_DAMPED_MAX_BAD_STEPS {
            self.bad_step_count = 0;
            return if relative_rate < XYCE_DAMPED_MIN_RESIDUAL_REDUCTION
                && self.min_convergence_rate <= 1.0
            {
                XyceDampedDecision::Failed {
                    test: 7,
                    return_code: self.return_codes.near_convergence,
                }
            } else {
                XyceDampedDecision::Failed {
                    test: 7,
                    return_code: XYCE_DAMPED_STALLED_CODE,
                }
            };
        }

        XyceDampedDecision::Continue
    }
}

#[inline]
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
        newton_step: usize,
        residual_inf_norm: Value,
        residual_l2_norm: Value,
        weighted_update_norm: Value,
    ) -> XyceDampedSample {
        XyceDampedSample {
            newton_step,
            residual_inf_norm,
            residual_l2_norm,
            weighted_update_norm,
            device_converged: true,
            inner_device_converged: true,
            linear_solve_ok: true,
            linear_solve_nan: false,
        }
    }

    #[test]
    fn first_zero_predictor_uses_uniform_weights() {
        let weights = xyce_damped_transient_weights(
            &[0.0, -Value::MIN_POSITIVE],
            &[2.0, -4.0],
            1.0e-2,
            1.0e-6,
            true,
        )
        .expect("weights");
        assert_eq!(weights, vec![1.0001e-2, 1.0001e-2]);
    }

    #[test]
    fn later_calls_use_predictor_and_accepted_scales() {
        let weights = xyce_damped_transient_weights(&[2.0, -0.25], &[1.0, -1.0], 0.5, 0.25, false)
            .expect("weights");
        assert_eq!(weights, vec![1.25, 0.75]);
    }

    #[test]
    fn weight_builder_rejects_invalid_inputs() {
        assert!(xyce_damped_transient_weights(&[1.0], &[], 1.0e-2, 1.0e-6, false).is_none());
        assert!(
            xyce_damped_transient_weights(&[Value::NAN], &[0.0], 1.0e-2, 1.0e-6, false).is_none()
        );
        assert!(xyce_damped_transient_weights(&[0.0], &[0.0], 0.0, 0.0, false).is_none());
    }

    #[test]
    fn normal_convergence_is_inclusive() {
        let mut status = XyceTransientDampedStatus::new(20);
        assert_eq!(
            status.evaluate(sample(1, 1.0e-2, 1.0, 0.33), 0.33, 1.0e-2),
            XyceDampedDecision::Accepted {
                test: 2,
                return_code: 2,
            }
        );
    }

    #[test]
    fn norm_too_small_accepts_before_rate_tests() {
        let mut status = XyceTransientDampedStatus::new(20);
        assert_eq!(
            status.evaluate(sample(1, 1.0, Value::EPSILON * 0.5, 1.0), 0.33, 1.0e-2),
            XyceDampedDecision::Accepted {
                test: 1,
                return_code: 1,
            }
        );
    }

    #[test]
    fn small_update_returns_time_integrator_status() {
        let mut status = XyceTransientDampedStatus::new(20);
        assert_eq!(
            status.evaluate(sample(1, 1.0, 1.0, 1.0e-6), 0.33, 1.0e-2),
            XyceDampedDecision::Accepted {
                test: 4,
                return_code: 4,
            }
        );
    }

    #[test]
    fn max_step_near_convergence_rejects_by_default() {
        let mut status = XyceTransientDampedStatus::new(2);
        assert_eq!(
            status.evaluate(sample(1, 1.0, 1.0, 1.0), 0.33, 1.0e-2),
            XyceDampedDecision::Continue
        );
        assert_eq!(
            status.evaluate(sample(2, 1.0, 0.8, 1.0), 0.33, 1.0e-2),
            XyceDampedDecision::Failed {
                test: 3,
                return_code: -3,
            }
        );
    }

    #[test]
    fn configured_positive_near_convergence_is_accepted() {
        let mut status = XyceTransientDampedStatus::with_return_codes(
            2,
            XyceDampedReturnCodes {
                near_convergence: 3,
                small_update: 4,
            },
        );
        let _ = status.evaluate(sample(1, 1.0, 1.0, 1.0), 0.33, 1.0e-2);
        assert_eq!(
            status.evaluate(sample(2, 1.0, 0.8, 1.0), 0.33, 1.0e-2),
            XyceDampedDecision::Accepted {
                test: 3,
                return_code: 3,
            }
        );
    }

    #[test]
    fn max_step_without_progress_reports_too_many_steps() {
        let mut status = XyceTransientDampedStatus::new(2);
        let _ = status.evaluate(sample(1, 1.0, 1.0, 1.0), 0.33, 1.0e-2);
        assert_eq!(
            status.evaluate(sample(2, 1.0, 1.0, 1.0), 0.33, 1.0e-2),
            XyceDampedDecision::Failed {
                test: 5,
                return_code: -1,
            }
        );
    }

    #[test]
    fn persistent_stagnation_reports_near_or_stall() {
        let mut status = XyceTransientDampedStatus::new(20);
        let _ = status.evaluate(sample(1, 1.0, 100.0, 1.0), 0.33, 1.0e-2);
        let _ = status.evaluate(sample(2, 1.0, 80.0, 1.0), 0.33, 1.0e-2);
        let mut norm = 80.0;
        for newton_step in 3..=6 {
            norm *= 1.0005;
            assert_eq!(
                status.evaluate(sample(newton_step, 1.0, norm, 1.0), 0.33, 1.0e-2),
                XyceDampedDecision::Continue
            );
        }
        norm *= 1.0005;
        assert_eq!(
            status.evaluate(sample(7, 1.0, norm, 1.0), 0.33, 1.0e-2),
            XyceDampedDecision::Failed {
                test: 7,
                return_code: -3,
            }
        );
    }

    #[test]
    fn device_failure_at_budget_is_rejected() {
        let mut status = XyceTransientDampedStatus::new(2);
        let mut candidate = sample(2, 1.0, 1.0, 1.0);
        candidate.device_converged = false;
        assert_eq!(
            status.evaluate(candidate, 0.33, 1.0e-2),
            XyceDampedDecision::Failed {
                test: 8,
                return_code: -1,
            }
        );
    }

    #[test]
    fn device_failure_precedes_inner_device_status() {
        let mut status = XyceTransientDampedStatus::new(2);
        let mut candidate = sample(2, 1.0, 1.0, 1.0);
        candidate.device_converged = false;
        candidate.inner_device_converged = false;
        assert_eq!(
            status.evaluate(candidate, 0.33, 1.0e-2),
            XyceDampedDecision::Failed {
                test: 8,
                return_code: -1,
            }
        );
    }

    #[test]
    fn begin_solve_resets_residual_reference_not_stagnation_state() {
        let mut status = XyceTransientDampedStatus::new(20);
        let _ = status.evaluate(sample(1, 1.0, 100.0, 1.0), 0.33, 1.0e-2);
        let _ = status.evaluate(sample(2, 1.0, 80.0, 1.0), 0.33, 1.0e-2);
        status.begin_solve(20);
        // A new solve's first candidate is its own residual reference, so a
        // finite candidate does not inherit the previous solve's 80/100 rate.
        assert_eq!(
            status.evaluate(sample(1, 1.0e-2, 2.0, 0.33), 0.33, 1.0e-2),
            XyceDampedDecision::Accepted {
                test: 2,
                return_code: 2,
            }
        );
    }

    #[test]
    fn predictor_residual_is_used_for_first_candidate_rate() {
        let mut status = XyceTransientDampedStatus::new(2);
        status.begin_solve_with_initial_residual(2, 100.0);
        assert_eq!(
            status.evaluate(sample(1, 1.0, 80.0, 1.0), 0.33, 1.0e-2),
            XyceDampedDecision::Continue
        );
        assert_eq!(
            status.evaluate(sample(2, 1.0, 80.0, 1.0), 0.33, 1.0e-2),
            XyceDampedDecision::Failed {
                test: 3,
                return_code: -3,
            }
        );
    }

    #[test]
    fn accepted_boundary_checkpoint_round_trip_preserves_only_persistent_phase() {
        let mut source = XyceTransientDampedStatus::new(20);
        source.initial_residual_l2_norm = Some(100.0);
        source.previous_residual_l2_norm = Some(80.0);
        source.bad_step_count = 3;
        source.min_convergence_rate = 0.9995;

        let checkpoint = source
            .capture_accepted_boundary_checkpoint()
            .expect("valid accepted-boundary checkpoint");
        assert_eq!(
            checkpoint,
            XyceDampedAcceptedBoundaryCheckpoint {
                bad_step_count: 3,
                min_convergence_rate: 0.9995,
            }
        );

        let configured_codes = XyceDampedReturnCodes {
            near_convergence: 3,
            small_update: -4,
        };
        let mut restored = XyceTransientDampedStatus::with_return_codes(7, configured_codes);
        restored.initial_residual_l2_norm = Some(12.0);
        restored.previous_residual_l2_norm = Some(11.0);
        restored
            .restore_accepted_boundary_checkpoint(&checkpoint)
            .expect("restore accepted-boundary checkpoint");

        assert_eq!(restored.bad_step_count, 3);
        assert_eq!(
            restored.min_convergence_rate.to_bits(),
            0.9995_f64.to_bits()
        );
        assert_eq!(restored.initial_residual_l2_norm, None);
        assert_eq!(restored.previous_residual_l2_norm, None);
        assert_eq!(restored.max_iterations, 7);
        assert_eq!(restored.return_codes, configured_codes);
    }

    #[test]
    fn accepted_boundary_checkpoint_validation_is_strict() {
        for checkpoint in [
            XyceDampedAcceptedBoundaryCheckpoint {
                bad_step_count: XYCE_DAMPED_MAX_BAD_STEPS,
                min_convergence_rate: 1.0,
            },
            XyceDampedAcceptedBoundaryCheckpoint {
                bad_step_count: 0,
                min_convergence_rate: Value::NAN,
            },
            XyceDampedAcceptedBoundaryCheckpoint {
                bad_step_count: 0,
                min_convergence_rate: 1.0 - XYCE_DAMPED_STAGNATION_TOLERANCE - Value::EPSILON,
            },
            XyceDampedAcceptedBoundaryCheckpoint {
                bad_step_count: 0,
                min_convergence_rate: 1.0 + XYCE_DAMPED_STAGNATION_TOLERANCE + Value::EPSILON,
            },
        ] {
            assert!(checkpoint.validate().is_err(), "accepted {checkpoint:?}");
        }

        for rate in [
            1.0 - XYCE_DAMPED_STAGNATION_TOLERANCE,
            1.0,
            1.0 + XYCE_DAMPED_STAGNATION_TOLERANCE,
        ] {
            XyceDampedAcceptedBoundaryCheckpoint {
                bad_step_count: XYCE_DAMPED_MAX_BAD_STEPS - 1,
                min_convergence_rate: rate,
            }
            .validate()
            .expect("closed stagnation interval is valid");
        }
    }

    #[test]
    fn rejected_checkpoint_does_not_mutate_live_status() {
        let mut status = XyceTransientDampedStatus::new(13);
        status.initial_residual_l2_norm = Some(9.0);
        status.previous_residual_l2_norm = Some(8.0);
        status.bad_step_count = 2;
        status.min_convergence_rate = 0.99975;

        let error = status
            .restore_accepted_boundary_checkpoint(&XyceDampedAcceptedBoundaryCheckpoint {
                bad_step_count: XYCE_DAMPED_MAX_BAD_STEPS,
                min_convergence_rate: 1.0,
            })
            .expect_err("invalid checkpoint must fail closed");
        assert!(error.contains("bad-step count"), "{error}");
        assert_eq!(status.initial_residual_l2_norm, Some(9.0));
        assert_eq!(status.previous_residual_l2_norm, Some(8.0));
        assert_eq!(status.bad_step_count, 2);
        assert_eq!(status.min_convergence_rate, 0.99975);
    }

    #[test]
    fn capture_rejects_corrupt_live_persistent_phase() {
        let mut status = XyceTransientDampedStatus::new(20);
        status.min_convergence_rate = Value::INFINITY;
        let error = status
            .capture_accepted_boundary_checkpoint()
            .expect_err("corrupt live phase must not be serialized");
        assert!(error.contains("must be finite"), "{error}");
    }
}
