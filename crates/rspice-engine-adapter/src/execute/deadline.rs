//! The wall-clock budget one request runs under, and how a stop is labelled.
//!
//! The worker holds the authoritative external deadline; the budget here
//! exists so a pathological deck produces a bounded `engine.time_limit`
//! outcome instead of an opaque kill. A caller-requested stop and a spent
//! deadline are recorded as different causes, because an operator reading the
//! wire has to be able to tell "you asked me to stop" from "I ran out of
//! time".

// This module was split out of `execute.rs` and still works against
// the executor's own wire types, failures and imports, so it takes the
// parent's imports rather than restating them.
use super::*;

/// Default wall-clock ceiling for all engine work in one request. The worker
/// holds the authoritative external deadline; this internal one exists so a
/// pathological deck produces the bounded `engine.time_limit` outcome instead
/// of an opaque kill.
pub const DEFAULT_SOLVE_BUDGET: Duration = Duration::from_secs(240);

/// Launch variable that overrides [`DEFAULT_SOLVE_BUDGET`].
///
/// The value is a positive number of seconds. The launch contract clears the
/// environment before starting this executor, so the only way this is set is
/// the worker deliberately setting it, which is exactly the authority that
/// owns the external deadline it has to stay under.
pub const SOLVE_BUDGET_SECONDS_ENV: &str = "RSPICE_ENGINE_SOLVE_BUDGET_SECONDS";

/// Resolve the solve budget from the launch environment.
///
/// A malformed or non-positive value is refused rather than rounded into
/// something plausible: silently substituting the default for a budget the
/// operator meant to shorten would let a request outlive the deadline the
/// worker is holding it to.
pub fn solve_budget_from_env() -> Result<Duration, String> {
    // Read through `var_os` so a value that is not valid UTF-8 is refused as
    // a malformed budget rather than mistaken for an unset one.
    let Some(value) = std::env::var_os(SOLVE_BUDGET_SECONDS_ENV) else {
        return Ok(DEFAULT_SOLVE_BUDGET);
    };
    let value = value.to_str().ok_or_else(|| {
        format!("{SOLVE_BUDGET_SECONDS_ENV} is not valid UTF-8; expected a number of seconds")
    })?;
    parse_solve_budget(value)
}

fn parse_solve_budget(value: &str) -> Result<Duration, String> {
    let seconds: f64 = value.trim().parse().map_err(|_| {
        format!("{SOLVE_BUDGET_SECONDS_ENV} is {value:?}; expected a positive number of seconds")
    })?;
    if !seconds.is_finite() || seconds <= 0.0 {
        return Err(format!(
            "{SOLVE_BUDGET_SECONDS_ENV} is {value:?}; expected a positive number of seconds"
        ));
    }
    Duration::try_from_secs_f64(seconds).map_err(|_| {
        format!("{SOLVE_BUDGET_SECONDS_ENV} is {value:?}; the budget does not fit a duration")
    })
}

/// Why one request stopped early.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopCause {
    /// The caller asked this process to stop.
    Cancelled,
    /// The solve budget expired.
    Deadline,
}

const STOP_NONE: u8 = 0;

const STOP_CANCELLED: u8 = 1;

const STOP_DEADLINE: u8 = 2;

/// The one abort source every analysis in a request sees: the caller's
/// cancellation or this executor's own solve budget, whichever fires first.
///
/// Which of the two fired is remembered because the two are different
/// outcomes to the operator — a cancelled job is not a job that ran too long
/// — and by the time the engine has unwound to the response mapping the
/// distinction is otherwise gone: `SimulationError::Aborted` says only that
/// something asked it to stop.
pub struct RunAbort<'a> {
    start: Instant,
    budget: Duration,
    cancel: &'a dyn AbortSignal,
    cause: AtomicU8,
}

impl<'a> RunAbort<'a> {
    pub fn new(cancel: &'a dyn AbortSignal, budget: Duration) -> Self {
        Self {
            start: Instant::now(),
            budget,
            cancel,
            cause: AtomicU8::new(STOP_NONE),
        }
    }

    /// The first cause observed, or `None` while the request is still live.
    pub fn stop_cause(&self) -> Option<StopCause> {
        match self.cause.load(Ordering::SeqCst) {
            STOP_CANCELLED => Some(StopCause::Cancelled),
            STOP_DEADLINE => Some(StopCause::Deadline),
            _ => None,
        }
    }

    pub(super) fn record(&self, cause: u8) -> bool {
        let _ = self
            .cause
            .compare_exchange(STOP_NONE, cause, Ordering::SeqCst, Ordering::SeqCst);
        true
    }
}

impl AbortSignal for RunAbort<'_> {
    fn is_aborted(&self) -> bool {
        // Cancellation is checked first: a caller that asked to stop deserves
        // that answer even if the budget expired in the same instant.
        if self.cancel.is_aborted() {
            return self.record(STOP_CANCELLED);
        }
        if self.start.elapsed() >= self.budget {
            return self.record(STOP_DEADLINE);
        }
        false
    }
}

/// A request stopped because its caller asked it to.
pub const CANCELLED_FAILURE_CODE: &str = "engine.cancelled";

/// A request stopped because it exhausted its solve budget.
pub const TIME_LIMIT_FAILURE_CODE: &str = "engine.time_limit";

/// Re-label a stop outcome once the whole request has unwound and the cause
/// that fired first is known.
pub(super) fn label_stop_cause(execution: Execution, cause: Option<StopCause>) -> Execution {
    let EngineResponse::Failed {
        failure_code,
        failure_detail,
    } = &execution.response
    else {
        return execution;
    };
    if failure_code != CANCELLED_FAILURE_CODE {
        return execution;
    }
    match cause {
        // Nothing this executor owns fired, so the engine stopped for an
        // abort source further in. Leave the honest cancellation label.
        None | Some(StopCause::Cancelled) => execution,
        Some(StopCause::Deadline) => Execution {
            response: EngineResponse::failed(TIME_LIMIT_FAILURE_CODE, failure_detail),
            artifacts: execution.artifacts,
        },
    }
}

/// A stop is either the caller's or the clock's, and the wire says which.
#[cfg(test)]
mod stop_cause_tests {
    use super::*;
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    const STEP_DECK: &str = "stop cause fixture\n\
         .param r=1k\n\
         V1 in 0 1\n\
         R1 in 0 {r}\n\
         .step param r list 1k 2k\n\
         .op\n\
         .end\n";

    pub(super) fn content() -> CircuitContent {
        CircuitContent::Deck {
            expanded_netlist: STEP_DECK.to_owned(),
        }
    }

    pub(super) fn failure_code(execution: &Execution) -> &str {
        match &execution.response {
            EngineResponse::Failed { failure_code, .. } => failure_code,
            EngineResponse::Succeeded { .. } => panic!("a stopped request reported success"),
        }
    }

    #[test]
    fn a_caller_requested_stop_is_reported_as_a_cancellation() {
        let execution = execute_with_abort(
            &serde_json::json!({"kind": "operating_point"}),
            &content(),
            "test",
            &ImmediateAbort,
            DEFAULT_SOLVE_BUDGET,
        );

        assert_eq!(failure_code(&execution), CANCELLED_FAILURE_CODE);
        assert!(
            execution.artifacts.is_empty(),
            "a cancelled request must declare no artifacts"
        );
    }

    #[test]
    fn an_exhausted_solve_budget_is_reported_as_a_time_limit() {
        // A zero budget is expired at its first poll, which makes the deadline
        // branch deterministic instead of a race against a real solve.
        let execution = execute_with_abort(
            &serde_json::json!({"kind": "operating_point"}),
            &content(),
            "test",
            &NoAbort,
            Duration::ZERO,
        );

        assert_eq!(failure_code(&execution), TIME_LIMIT_FAILURE_CODE);
        assert!(
            execution.artifacts.is_empty(),
            "a request that ran out of time must declare no artifacts"
        );
    }

    #[test]
    fn cancellation_wins_when_both_causes_are_live() {
        let execution = execute_with_abort(
            &serde_json::json!({"kind": "operating_point"}),
            &content(),
            "test",
            &ImmediateAbort,
            Duration::ZERO,
        );

        assert_eq!(
            failure_code(&execution),
            CANCELLED_FAILURE_CODE,
            "a caller who asked to stop must be told their request was cancelled, \
             not that it was too slow"
        );
    }

    #[test]
    fn a_completed_request_is_never_relabelled() {
        let execution = execute_with_abort(
            &serde_json::json!({"kind": "operating_point"}),
            &CircuitContent::Deck {
                expanded_netlist: "trivial\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n".to_owned(),
            },
            "test",
            &NoAbort,
            DEFAULT_SOLVE_BUDGET,
        );

        assert!(matches!(
            execution.response,
            EngineResponse::Succeeded { .. }
        ));
    }

    #[test]
    fn the_solve_budget_override_is_parsed_and_bounded() {
        assert_eq!(
            parse_solve_budget("30"),
            Ok(Duration::from_secs(30)),
            "a plain number of seconds is the documented form"
        );
        assert_eq!(parse_solve_budget(" 0.5 "), Ok(Duration::from_millis(500)));
        for refused in ["0", "-1", "abc", "", "inf", "NaN"] {
            assert!(
                parse_solve_budget(refused).is_err(),
                "{refused:?} must be refused rather than silently defaulted"
            );
        }
    }
}
