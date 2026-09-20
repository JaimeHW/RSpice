//! The per-case wall-clock budget every suite runs its decks under.
//!
//! A conformance corpus contains decks that do not converge, and a suite that
//! waits for one of them forever reports nothing at all. Each suite therefore
//! gives every case a budget and stops it cooperatively when the budget is
//! spent, which the runner then records as a timed-out case rather than as a
//! numerical mismatch.
//!
//! This lived as four byte-identical copies, one per suite. The budget is the
//! same idea in all four — the suites differ in what they compare, not in how
//! they bound a run — so it lives here once.

use std::time::{Duration, Instant};

use rspice_core::abort_signal::AbortSignal;

/// Aborts a case once its wall-clock budget is spent.
#[derive(Debug, Clone, Copy)]
pub struct DeadlineAbort {
    start: Instant,
    budget: Duration,
}

impl DeadlineAbort {
    /// Budget `budget_ms` milliseconds of wall clock from `start`.
    ///
    /// A budget past the range of a `Duration` is clamped rather than
    /// rejected: a case manifest asking for an effectively unbounded run is
    /// asking for exactly that.
    pub fn new(start: Instant, budget_ms: u128) -> Self {
        Self {
            start,
            budget: Duration::from_millis(budget_ms.min(u128::from(u64::MAX)) as u64),
        }
    }
}

impl AbortSignal for DeadlineAbort {
    #[inline]
    fn is_aborted(&self) -> bool {
        self.start.elapsed() >= self.budget
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_spent_budget_aborts_and_a_live_one_does_not() {
        let start = Instant::now();
        assert!(
            DeadlineAbort::new(start, 0).is_aborted(),
            "a zero budget is already spent at its first poll"
        );
        assert!(
            !DeadlineAbort::new(start, 600_000).is_aborted(),
            "a ten-minute budget is not spent by the time this line runs"
        );
    }

    #[test]
    fn an_out_of_range_budget_clamps_instead_of_overflowing() {
        let abort = DeadlineAbort::new(Instant::now(), u128::MAX);
        assert!(!abort.is_aborted());
    }
}
