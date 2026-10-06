//! Candidate clocks and physical-event interval fitting.
//!
//! The accepted clock belongs to the integration driver; these helpers only
//! return proposals. They never consume a source/transport event, advance a
//! device history, publish output, or commit a controller step. A rejected or
//! failed proposal therefore leaves every accepted-state owner unchanged.
//!
//! Integration width and endpoint identity are separate: subtracting and then
//! adding floating-point clocks is not an identity. Keep the original event
//! clock for device loads, observations and checkpoints while retaining the
//! interval used to form integration coefficients.

use super::{SimulationError, SpiceDialect, Value, breakpoints, xyce_hard_min_timestep};

/// Return the exact requested horizon when a step consumes the remaining
/// transient interval. Floating-point subtraction followed by addition is not
/// an identity for every pair of finite values, so endpoint-sensitive sources,
/// device loads, checkpoints, and recorded samples must share this canonical
/// time instead of independently recomputing `current_time + dt`.
#[inline]
pub(super) fn canonical_transient_step_time(
    current_time: Value,
    dt: Value,
    stop_time: Value,
) -> Value {
    if dt >= stop_time - current_time {
        stop_time
    } else {
        current_time + dt
    }
}

/// Preserve the exact absolute time requested by an accepted Verilog-A event.
/// Floating-point subtraction followed by addition is not an identity for
/// every pair, so the event target must remain authoritative while `dt`
/// separately supplies integration coefficients.
#[inline]
pub(super) fn canonical_transient_step_time_with_device_event(
    current_time: Value,
    dt: Value,
    stop_time: Value,
    exact_device_event_time: Option<Value>,
) -> Value {
    exact_device_event_time
        .unwrap_or_else(|| canonical_transient_step_time(current_time, dt, stop_time))
}

/// A disposable integration proposal, not an accepted solution. An exact
/// endpoint retains the event owner's clock independently of its interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct StepProposal {
    pub dt: Value,
    pub time: Value,
    pub exact_event_time: Option<Value>,
}

/// Immutable scheduling limits at one accepted point. The persistent maximum
/// describes future intervals; the controller maximum bounds this attempt.
#[derive(Clone, Copy)]
pub(super) struct PhysicalStepWindow {
    pub accepted_time: Value,
    pub stop_time: Value,
    pub hard_min_dt: Value,
    pub persistent_maximum: Value,
    pub controller_maximum: Value,
}

/// Reserve a representable interval before an owned physical deadline.
///
/// Approximate source/line breakpoints can fall a few ulps before an owned
/// arrival. Fit the still-unaccepted interval so its next gap respects the
/// minimum; never consume an arrival early or manufacture a subminimum step.
/// Xyce's floor grows with accepted time, so reserve the floor at the deadline.
///
/// The driver calls this only for an adaptive proposal whose earliest exact
/// event is this physical deadline. Other event policies retain ownership of
/// their own clocks. `None` means the original proposal needs no adjustment,
/// including preservation of any exact endpoint it already carries.
pub(super) fn fit_physical_event_approach(
    window: PhysicalStepWindow,
    proposal: StepProposal,
    physical_event_time: Option<Value>,
    dialect: SpiceDialect,
) -> Result<Option<StepProposal>, SimulationError> {
    let physical_event_min_dt = physical_event_time.map_or(window.hard_min_dt, |time| {
        if dialect == SpiceDialect::Xyce {
            window.hard_min_dt.max(xyce_hard_min_timestep(time))
        } else {
            window.hard_min_dt
        }
    });
    let Some(deadline) = physical_event_time.filter(|deadline| {
        *deadline > proposal.time && *deadline - proposal.time < physical_event_min_dt
    }) else {
        return Ok(None);
    };
    let bound = window.controller_maximum.min(window.persistent_maximum);
    let dt = breakpoints::fit_model_interval(
        window.accepted_time,
        deadline,
        proposal.dt,
        physical_event_min_dt,
        window.persistent_maximum,
        bound,
        false,
    )?;
    if dt > bound {
        return Err(SimulationError::Circuit(
            "physical event fitting exceeds the step bound".into(),
        ));
    }
    let exact_event_time = (window.accepted_time + dt >= deadline).then_some(deadline);
    let time = canonical_transient_step_time_with_device_event(
        window.accepted_time,
        dt,
        window.stop_time,
        exact_event_time,
    );
    Ok(Some(StepProposal {
        dt,
        time,
        exact_event_time,
    }))
}

/// Fit a rounding-sized final remainder before solving an exact-history step.
/// Such histories cannot use the ordinary post-solve breakpoint snap: that
/// would label state evaluated at another clock as the requested endpoint.
/// Land exactly when the current bound permits it; otherwise divide the last
/// two intervals so both obey the bound without a near-zero final solve.
/// Real event endpoints and proposals at the clock's own resolution retain
/// their original owner/policy.
pub(super) fn fit_roundoff_stop_approach(
    window: PhysicalStepWindow,
    proposal: StepProposal,
) -> Option<StepProposal> {
    let roundoff = 64.0 * Value::EPSILON * window.accepted_time.abs().max(window.stop_time.abs());
    let tail = window.stop_time - proposal.time;
    if proposal.exact_event_time.is_some()
        || tail <= 0.0
        || tail > roundoff
        || proposal.dt <= roundoff
    {
        return None;
    }
    let remaining = window.stop_time - window.accepted_time;
    let bound = window.controller_maximum.min(window.persistent_maximum);
    let dt = if remaining <= bound {
        remaining
    } else {
        remaining / 2.0
    };
    if !dt.is_finite() || dt < window.hard_min_dt || dt > bound {
        return None;
    }
    Some(StepProposal {
        dt,
        time: canonical_transient_step_time(window.accepted_time, dt, window.stop_time),
        exact_event_time: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn window(minimum: Value) -> PhysicalStepWindow {
        PhysicalStepWindow {
            accepted_time: 0.0,
            stop_time: 2.0,
            hard_min_dt: minimum,
            persistent_maximum: 1.0,
            controller_maximum: 1.0,
        }
    }

    #[test]
    fn rounded_final_remainder_is_split_without_exceeding_the_step_bound() {
        let window = PhysicalStepWindow {
            accepted_time: 2e-9 - 4e-12 - 6e-24,
            stop_time: 2e-9,
            hard_min_dt: xyce_hard_min_timestep(2e-9),
            persistent_maximum: 4e-12,
            controller_maximum: 4e-12,
        };
        let proposal = StepProposal {
            dt: 4e-12,
            time: window.accepted_time + 4e-12,
            exact_event_time: None,
        };
        let fitted = fit_roundoff_stop_approach(window, proposal).unwrap();
        assert!(fitted.dt <= window.controller_maximum);
        assert!(fitted.dt > 1.9e-12);
        assert!(window.stop_time - fitted.time > 1.9e-12);
        assert_eq!(fitted.exact_event_time, None);
        assert_eq!(
            canonical_transient_step_time(
                fitted.time,
                window.stop_time - fitted.time,
                window.stop_time
            ),
            window.stop_time
        );
    }

    #[test]
    fn rounded_final_remainder_lands_directly_when_the_bound_allows_it() {
        let window = PhysicalStepWindow {
            accepted_time: 1.0,
            stop_time: 2.0,
            ..window(0.1)
        };
        let proposal = StepProposal {
            dt: 1.0 - Value::EPSILON,
            time: 2.0 - Value::EPSILON,
            exact_event_time: None,
        };
        let fitted = fit_roundoff_stop_approach(window, proposal).unwrap();
        assert_eq!(fitted.dt, 1.0);
        assert_eq!(fitted.time, 2.0);
        assert!(
            fit_roundoff_stop_approach(
                window,
                StepProposal {
                    exact_event_time: Some(proposal.time),
                    ..proposal
                }
            )
            .is_none()
        );
        assert!(
            fit_roundoff_stop_approach(
                window,
                StepProposal {
                    dt: 0.5,
                    time: 1.5,
                    exact_event_time: None
                }
            )
            .is_none()
        );
    }

    #[test]
    fn sufficient_gap_preserves_the_existing_exact_proposal() {
        let proposal = StepProposal {
            dt: 0.5,
            time: 0.5,
            exact_event_time: Some(0.5),
        };
        for dialect in [
            SpiceDialect::BestAvailable,
            SpiceDialect::Ngspice,
            SpiceDialect::Xyce,
        ] {
            assert!(
                fit_physical_event_approach(window(0.1), proposal, Some(1.0), dialect)
                    .unwrap()
                    .is_none()
            );
            assert!(
                fit_physical_event_approach(window(0.1), proposal, None, dialect)
                    .unwrap()
                    .is_none()
            );
        }
    }

    #[test]
    fn fitting_leaves_a_reachable_physical_deadline_without_consuming_it() {
        let proposal = StepProposal {
            dt: 0.95,
            time: 0.95,
            exact_event_time: None,
        };
        let fitted =
            fit_physical_event_approach(window(0.1), proposal, Some(1.0), SpiceDialect::Ngspice)
                .unwrap()
                .unwrap();
        assert!(fitted.dt <= proposal.dt);
        assert!((fitted.dt - 0.9).abs() < 4.0 * Value::EPSILON);
        assert!(1.0 - fitted.time >= 0.1);
        assert_eq!(fitted.exact_event_time, None);
        // The same accepted state and proposal produce the same answer after rejection.
        assert_eq!(
            Some(fitted),
            fit_physical_event_approach(window(0.1), proposal, Some(1.0), SpiceDialect::Ngspice)
                .unwrap()
        );
    }

    #[test]
    fn xyce_reserves_the_floor_at_the_future_clock() {
        let future_floor = xyce_hard_min_timestep(1.0);
        let dt = 1.0 - future_floor * 0.5;
        let proposal = StepProposal {
            dt,
            time: dt,
            exact_event_time: None,
        };
        let fitted =
            fit_physical_event_approach(window(1e-30), proposal, Some(1.0), SpiceDialect::Xyce)
                .unwrap()
                .unwrap();
        assert!(fitted.time < 1.0);
        assert!(1.0 - fitted.time >= future_floor);
        assert_eq!(fitted.exact_event_time, None);
    }

    #[test]
    fn indivisible_gap_lands_on_the_original_event_or_refuses_the_bound() {
        let proposal = StepProposal {
            dt: 0.9,
            time: 0.9,
            exact_event_time: None,
        };
        let fitted =
            fit_physical_event_approach(window(0.6), proposal, Some(1.0), SpiceDialect::Ngspice)
                .unwrap()
                .unwrap();
        assert_eq!(
            fitted,
            StepProposal {
                dt: 1.0,
                time: 1.0,
                exact_event_time: Some(1.0)
            }
        );
        let bounded = PhysicalStepWindow {
            controller_maximum: 0.9,
            ..window(0.6)
        };
        assert!(
            fit_physical_event_approach(bounded, proposal, Some(1.0), SpiceDialect::Ngspice)
                .is_err()
        );
    }
}
