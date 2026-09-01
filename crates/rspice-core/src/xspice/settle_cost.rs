//! Exact counters for the two settle-loop cost structures.
//!
//! The XSPICE settle loop carries two performance structures whose regression
//! would be silent — the run would still be correct, just slower, and nothing
//! in the test suite would notice:
//!
//! 1. **Event-driven dispatch.** [`crate::circuit::xspice_dispatch`] narrows
//!    each delta cycle to the instances an executed event could have reached.
//!    On a gate-level deck that is a small minority of the instance list; a
//!    change that loses the narrowing turns every delta cycle back into a full
//!    pass over every instance.
//! 2. **Copy-on-write rollback snapshots.** `SharedXspiceInstance`,
//!    `SharedXspiceEventValues` and `SharedXspiceEventQueue` hold their
//!    payloads behind an [`Arc`], so a rollback capture is a reference-count
//!    bump and the deep copy is deferred to the first write through a shared
//!    handle. A new unconditional `make_mut` on a per-step path, or a write
//!    that stores what is already there, silently restores the old cost.
//!
//! Wall-clock is the obvious thing to measure and the wrong thing to assert:
//! a timing threshold on shared CI hardware is either loose enough to miss the
//! regression or tight enough to fail on a noisy neighbour. These counters are
//! the deterministic substitute. They count events, not nanoseconds, so a
//! pinned deck produces the same numbers on every run of every machine, and a
//! ceiling on them fails only when the structure it protects actually moved.
//!
//! `engine::xspice_settle_ratchet` is the test that reads them. Nothing else
//! does.
//!
//! # Why thread-local, and not atomics
//!
//! The settle loop is single-threaded by construction: every entry point takes
//! `&mut CircuitData`, so two threads cannot be inside it for the same circuit,
//! and the crate's rayon use is confined to frequency sweeps and the classic
//! MOSFET chunk loop — neither of which reaches XSPICE.
//!
//! Process-global atomics would still be wrong here, for a reason that has
//! nothing to do with the engine: `cargo test` runs a binary's tests
//! concurrently in one process, so a global counter would tally every other
//! test's XSPICE activity alongside the ratchet's own and the measurement
//! would depend on the scheduler. A thread-local counter is private to the
//! test that resets it, under both `--test-threads=1` and the default.
//!
//! # Cost
//!
//! An `#[inline]` increment of a plain `u64` in thread-local storage, on
//! paths that already do far more than that: the evaluation counter sits
//! beside a model call, and each copy counter sits beside an `Arc::make_mut`.
//! Detecting a copy costs one non-atomic pointer read and one comparison —
//! see [`make_mut_reporting_copy`] — so nothing here adds an atomic operation
//! to any path. This ships in release for the same reason the ratchet exists:
//! a counter compiled out of the build being measured measures nothing.

use std::cell::Cell;
use std::sync::Arc;

/// One thread's tally of the quantities the settle-cost ratchet pins.
///
/// Every field is monotonically increasing from the last [`reset`]. Read one
/// with [`counts`].
///
/// The *readers* are `#[cfg(test)]` — this type, [`reset`] and [`counts`] —
/// while the writers below are not. That split is deliberate and it is the
/// only one: the increments ship, so the build a ratchet measures is the build
/// that runs, and a `make_mut` added on a hot path is counted wherever it is
/// added rather than only under a test cfg. Nothing outside a test has any use
/// for the total, so exporting a getter into release builds would be dead
/// code, and this crate does not ship dead code.
#[cfg(test)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct XspiceSettleCounts {
    /// Instances the settle loop actually evaluated.
    ///
    /// Counted where the loop commits to running an instance — past the
    /// dirty-dispatch skip check, before the `make_mut` that unshares it. An
    /// instance the dispatch skipped is not counted, which is the whole point:
    /// this number is what the dispatch did *not* avoid.
    pub(crate) instance_evaluations: u64,
    /// Deep copies of an instance cell taken by `SharedXspiceInstance`.
    pub(crate) instance_deep_copies: u64,
    /// Deep copies of the six resolved event-value maps taken by
    /// `SharedXspiceEventValues`.
    pub(crate) event_values_deep_copies: u64,
    /// Deep copies of the event scheduler taken by `SharedXspiceEventQueue`.
    pub(crate) event_queue_deep_copies: u64,
    /// Deep copies of a mixed Verilog-AMS module's analog device, digital host
    /// or bridge table taken by the trial rollback cells in
    /// `xspice::verilog::mixed`.
    ///
    /// Counted here rather than in a counter of its own because the structure
    /// is the same structure and the failure is the same failure — a rollback
    /// capture that stopped being a reference-count bump. It has no ceiling in
    /// `engine::xspice_settle_ratchet`, whose deck holds no mixed module and
    /// would ratchet a constant zero; `mixed_trial_copy_ratchet` reads it off
    /// a scripted sequence of trials instead.
    pub(crate) mixed_trial_deep_copies: u64,
}

#[cfg(test)]
impl XspiceSettleCounts {
    /// Deep copies of the event world, which the two cells describe together.
    ///
    /// The drain resolves a node's drivers into its value and stamps the event
    /// time in one step, and the scheduler is what it drains from; a snapshot
    /// holding one without the other describes a state the drain never
    /// produces. They are ratcheted as a sum for the same reason they are
    /// captured as a unit — a change that moves a copy from one cell to the
    /// other has not made anything cheaper.
    pub(crate) fn event_world_deep_copies(&self) -> u64 {
        self.event_values_deep_copies + self.event_queue_deep_copies
    }
}

/// The live cells behind [`XspiceSettleCounts`].
///
/// `Cell<u64>` rather than the plain integers of the snapshot so the `note_*`
/// functions can take `&self` out of thread-local storage. No field owns a
/// destructor, so the thread-local needs no lazy-drop registration and
/// `LocalKey::with` cannot fail on it.
struct Counters {
    instance_evaluations: Cell<u64>,
    instance_deep_copies: Cell<u64>,
    event_values_deep_copies: Cell<u64>,
    event_queue_deep_copies: Cell<u64>,
    mixed_trial_deep_copies: Cell<u64>,
}

thread_local! {
    static COUNTERS: Counters = const {
        Counters {
            instance_evaluations: Cell::new(0),
            instance_deep_copies: Cell::new(0),
            event_values_deep_copies: Cell::new(0),
            event_queue_deep_copies: Cell::new(0),
            mixed_trial_deep_copies: Cell::new(0),
        }
    };
}

/// Add one to `cell`.
///
/// `wrapping_add` because a saturating or checked add would put a branch on
/// the hot path to defend against an overflow that needs 2^64 settle
/// evaluations to reach.
#[inline]
fn bump(cell: &Cell<u64>) {
    cell.set(cell.get().wrapping_add(1));
}

/// Record that the settle loop evaluated one instance.
#[inline]
pub(crate) fn note_instance_evaluation() {
    COUNTERS.with(|counters| bump(&counters.instance_evaluations));
}

/// Record that an instance cell was deep-copied out of a rollback snapshot.
#[inline]
pub(crate) fn note_instance_deep_copy() {
    COUNTERS.with(|counters| bump(&counters.instance_deep_copies));
}

/// Record that the resolved event-value maps were deep-copied.
#[inline]
pub(crate) fn note_event_values_deep_copy() {
    COUNTERS.with(|counters| bump(&counters.event_values_deep_copies));
}

/// Record that the event scheduler was deep-copied.
#[inline]
pub(crate) fn note_event_queue_deep_copy() {
    COUNTERS.with(|counters| bump(&counters.event_queue_deep_copies));
}

/// Record that a mixed Verilog-AMS trial cell was deep-copied.
#[cfg(feature = "veriloga")]
#[inline]
pub(crate) fn note_mixed_trial_deep_copy() {
    COUNTERS.with(|counters| bump(&counters.mixed_trial_deep_copies));
}

/// Zero this thread's counters.
///
/// A ratchet measures a delta, so it resets before the run it is measuring.
/// That is what makes the measurement independent of `--test-threads`: under
/// `-1` the tests share a thread and run in sequence, so a reset immediately
/// before the run still leaves the run alone in the window; under the default
/// each test owns its thread and its counters outright.
#[cfg(test)]
pub(crate) fn reset() {
    COUNTERS.with(|counters| {
        counters.instance_evaluations.set(0);
        counters.instance_deep_copies.set(0);
        counters.event_values_deep_copies.set(0);
        counters.event_queue_deep_copies.set(0);
        counters.mixed_trial_deep_copies.set(0);
    });
}

/// This thread's tally since the last [`reset`].
#[cfg(test)]
pub(crate) fn counts() -> XspiceSettleCounts {
    COUNTERS.with(|counters| XspiceSettleCounts {
        instance_evaluations: counters.instance_evaluations.get(),
        instance_deep_copies: counters.instance_deep_copies.get(),
        event_values_deep_copies: counters.event_values_deep_copies.get(),
        event_queue_deep_copies: counters.event_queue_deep_copies.get(),
        mixed_trial_deep_copies: counters.mixed_trial_deep_copies.get(),
    })
}

/// [`Arc::make_mut`], reporting whether it had to copy.
///
/// The report is by allocation identity, not by reference count. Asking
/// `Arc::strong_count` first would answer the same question, but it costs two
/// atomic loads on a path whose whole purpose is to be cheap when the answer
/// is "no copy", and it races: a snapshot dropped between the count and the
/// `make_mut` would be reported as a copy that did not happen.
///
/// Comparing the pointer before against the pointer after has neither problem.
/// It is a non-atomic read and a comparison, and it observes the outcome
/// rather than predicting it. It cannot report a false negative either: when
/// `make_mut` clones, the allocation it cloned *from* is still alive — that is
/// why it cloned — so the allocator cannot have handed the clone the same
/// address.
#[inline]
pub(crate) fn make_mut_reporting_copy<T: Clone>(arc: &mut Arc<T>) -> (&mut T, bool) {
    let before: *const T = Arc::as_ptr(arc);
    let value = Arc::make_mut(arc);
    let copied = !std::ptr::eq(before, std::ptr::from_ref(&*value));
    (value, copied)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_mut_reports_a_copy_only_when_the_cell_is_shared() {
        let mut owned = Arc::new(vec![1_u32, 2, 3]);
        let (_, copied) = make_mut_reporting_copy(&mut owned);
        assert!(
            !copied,
            "a uniquely owned Arc is mutated in place, so no copy is reported"
        );

        let snapshot = Arc::clone(&owned);
        let (value, copied) = make_mut_reporting_copy(&mut owned);
        value.push(4);
        assert!(
            copied,
            "an Arc a snapshot still shares must be copied before it is written"
        );
        assert_eq!(
            snapshot.len(),
            3,
            "the snapshot must not observe the write that copied"
        );

        let (_, copied) = make_mut_reporting_copy(&mut owned);
        assert!(
            !copied,
            "the copy left the handle uniquely owning, so the next write is free"
        );
    }

    #[test]
    fn counters_are_per_thread_and_reset_to_zero() {
        reset();
        note_instance_evaluation();
        note_instance_deep_copy();
        note_event_values_deep_copy();
        note_event_queue_deep_copy();
        note_event_queue_deep_copy();

        let measured = counts();
        assert_eq!(measured.instance_evaluations, 1);
        assert_eq!(measured.instance_deep_copies, 1);
        assert_eq!(measured.event_values_deep_copies, 1);
        assert_eq!(measured.event_queue_deep_copies, 2);
        assert_eq!(
            measured.event_world_deep_copies(),
            3,
            "the event world is the two cells summed"
        );

        // A second thread starts from zero and cannot see this one's tally.
        // This is the property that lets a ratchet measure its own run while
        // the rest of the binary's tests run beside it.
        let other = std::thread::spawn(|| {
            let before = counts();
            note_instance_evaluation();
            (before, counts())
        })
        .join()
        .expect("counter probe thread joins");
        assert_eq!(
            other.0,
            XspiceSettleCounts::default(),
            "a fresh thread starts with zeroed counters"
        );
        assert_eq!(other.1.instance_evaluations, 1);
        assert_eq!(
            counts().instance_evaluations,
            1,
            "the other thread's increment must not reach this thread"
        );

        reset();
        assert_eq!(counts(), XspiceSettleCounts::default());
    }
}
