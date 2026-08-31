//! The settle loop's cost on a gate-level deck may shrink, never grow.
//!
//! Two structures make the XSPICE settle loop cheap on digital decks, and a
//! change that undoes either leaves every existing test green. The run is
//! still correct — the dispatch's skip is provably a subset of a skip the
//! engine already performed, and a copy-on-write cell reproduces the image the
//! deep copy produced — so correctness tests cannot see the difference. Only
//! the cost moves.
//!
//! This test is what sees it. It runs one pinned deck and asserts exact
//! counts.
//!
//! # What each ratchet protects
//!
//! [`MAX_INSTANCE_EVALUATIONS`] protects the **event-driven dispatch** in
//! `circuit::xspice_dispatch`. The settle loop used to open every delta cycle
//! with a full pass over the instance list; it now visits only the instances
//! an executed event could have reached, in the same registration order. Lose
//! the narrowing — by widening the fan-out map, by marking instances dirty
//! that no event touched, or by dropping the eligibility rule that admits a
//! model to the skip — and this count jumps by roughly the factor the
//! narrowing was buying.
//!
//! [`MAX_INSTANCE_DEEP_COPIES`] protects `SharedXspiceInstance`. A rollback
//! capture is a reference-count bump; the deep copy of the instance and its
//! `CmContext` is deferred to the first write through a handle a snapshot
//! still shares. A `make_mut` added on a per-step path without a shared-borrow
//! predicate in front of it — a write that stores a value already there, a
//! flag set that was already set — costs a full copy for nothing, and this is
//! the count that says so.
//!
//! [`MAX_EVENT_WORLD_DEEP_COPIES`] protects `SharedXspiceEventValues` and
//! `SharedXspiceEventQueue` together. Same structure, same failure: the six
//! resolved-value maps and the scheduler are copied on the first write after a
//! capture, and the drain and the scheduling sweep each ask a cheap predicate
//! before taking a mutable view. The two cells are summed because they are
//! captured as a unit — moving a copy from one to the other has made nothing
//! cheaper.
//!
//! # Why counts and not wall-clock
//!
//! A timing threshold on shared CI hardware is either loose enough to miss the
//! regression it is aimed at or tight enough to fail on a noisy neighbour.
//! These counts are exactly reproducible: same deck, same numbers, every run
//! and every machine. `settle_costs_are_exactly_reproducible` asserts that
//! property directly rather than assuming it, because a ratchet on a number
//! that wobbles is a flaky test with a ceiling on it.
//!
//! # Raising a ceiling
//!
//! One case justifies it, and it requires saying so in the commit: the deck's
//! *work* genuinely grew, because a correctness fix made the settle loop do
//! something it was previously skipping wrongly. That is a real raise, and the
//! commit should name the defect.
//!
//! Everything else is the case this test exists to catch. In particular "the
//! number went up a little after my refactor" is not a reason to raise a
//! ceiling — it is the report that the refactor cost something, and the
//! question is what. Prefer investigating a rise to explaining it away.
//!
//! Changing the deck is a third thing again, and it invalidates all three
//! numbers at once. [`DECK`] is pinned for that reason; re-measure and
//! re-document every ceiling in the same commit that edits it.
//!
//! # Lowering a ceiling
//!
//! Not optional. Each ceiling carries a staleness floor
//! ([`STALE_CEILING_SLACK_PERCENT`]): a count that falls far enough below its
//! ceiling fails the test until the ceiling follows it down. Without that, a
//! ratchet silently stops ratcheting — the number improves, nobody updates the
//! constant, and the gap quietly becomes headroom to regrow into.
//!
//! That floor is also how the deletion case surfaces. A copy-on-write cell
//! swapped back for an owned deep copy takes no copy-on-write copies, so its
//! count goes to nearly zero — which is not a ceiling failure and would
//! otherwise pass. It trips the floor instead, and the floor's message says to
//! check whether the work got cheaper or simply moved somewhere nothing counts
//! it.

use crate::engine::{Engine, SimulationConfig};
use crate::netlist::Netlist;
use crate::xspice::settle_cost::{self, XspiceSettleCounts};

/// The pinned input. See the deck's own header for what it is and why.
///
/// Committed rather than written inline because the numbers below are a
/// property of these exact 72 gates and this exact stimulus, and a deck that
/// is easy to tweak in passing is a deck whose ceilings mean nothing.
const DECK: &str = include_str!("../../tests/testdata/xspice_nand_ripple_adder_8bit.cir");

/// Run bound, matching the deck's own `.tran` card.
const TSTOP: f64 = 20.0e-9;
/// Print/step bound, matching the deck's own `.tran` card.
const TSTEP: f64 = 100.0e-12;

/// Ceiling on instances the settle loop evaluates over the whole run.
///
/// Measured at 13,477 when this ratchet landed, against a deck of 74 XSPICE
/// instances. Forcing the settle loop's skip check to decline every skip — the
/// regression this ceiling exists for — takes the same deck to 382,431, a
/// factor of 28. That is what the roughly 3% of headroom here is chosen
/// against: wide enough that a benign shift of a few evaluations does not fail
/// a build, and nowhere near wide enough to absorb a lost narrowing, which
/// arrives as a multiple rather than a margin.
const MAX_INSTANCE_EVALUATIONS: u64 = 13_900;

/// Ceiling on deep copies of an instance cell over the whole run.
///
/// Measured at 13,398, a little under the evaluation count. That the two are
/// close is the design working, not failing: an instance the settle loop
/// writes to is usually one a rollback snapshot still shares, so that write
/// pays for a copy. What copy-on-write removed was never those — it was the
/// copy of the *other* seventy-odd instances a capture used to take whether or
/// not they had moved. Reverting it puts this count at captures × instances,
/// which is bounded below by the event-world copy count times the deck's 74
/// instances, i.e. several times what it reads now.
///
/// Every write site shares this counter, not just the settle loop's:
/// `mark_fanout_dirty`, timestep acceptance and the breakpoint drain all reach
/// an instance through the same cell. A `make_mut` added to any of them,
/// unguarded, raises this count without raising
/// [`MAX_INSTANCE_EVALUATIONS`] — the asymmetry that makes two ceilings worth
/// more than one.
///
/// One measured caveat, so the next reader does not over-trust this number.
/// `mark_fanout_dirty`'s "the bit is already set" check was removed
/// experimentally and this count did not move at all: by the time that sweep
/// reaches an instance whose bit is already set, the instance is already
/// uniquely owned, so the guard saves nothing *on this deck*. It is a real
/// guard on decks with a different capture rhythm, but do not read a green
/// build here as evidence that it is still in place.
const MAX_INSTANCE_DEEP_COPIES: u64 = 13_800;

/// Ceiling on deep copies of the event world — the six resolved-value maps
/// plus the scheduler — over the whole run.
///
/// Measured at 1,716: 853 copies of the value maps and 863 of the queue,
/// against 13,477 instance evaluations. Two orders of magnitude below the
/// per-instance counts is the shape to expect, because the drain and the
/// scheduling sweep each ask a shared-borrow predicate
/// (`has_event_at_or_before`, `has_pending_events`) before taking a mutable
/// view, so a quiet delta cycle touches neither cell.
///
/// Removing just one of those predicates — taking the queue's mutable view on
/// every evaluation instead of only when the instance queued something — takes
/// this count to 6,017 on the same deck, so this ceiling is load-bearing and
/// not a formality.
const MAX_EVENT_WORLD_DEEP_COPIES: u64 = 1_800;

/// How far under its ceiling a count may sit before the ceiling is considered
/// stale and must be lowered, as a percentage of the ceiling.
///
/// A percentage rather than the absolute slack `tests/public_surface.rs` uses,
/// because these three counts differ from each other by orders of magnitude
/// and one absolute would be either meaningless on the largest or unusable on
/// the smallest.
const STALE_CEILING_SLACK_PERCENT: u64 = 15;

/// Run the pinned deck once and report what it cost.
fn measure() -> XspiceSettleCounts {
    let netlist = Netlist::parse(DECK).expect("the pinned settle-cost deck parses");
    let engine = Engine::new(SimulationConfig::default());
    settle_cost::reset();
    let result = engine
        .run_tran(&netlist, TSTOP, TSTEP)
        .expect("the pinned settle-cost deck runs");
    let counts = settle_cost::counts();

    // The deck has to have actually settled events, or the counts below are a
    // ceiling on nothing. This also catches the one way a thread-local counter
    // could read zero on a working run: the settle loop having moved onto some
    // other thread, where the reset and the read no longer see what it did.
    assert!(
        !result.time.is_empty(),
        "the pinned deck must produce a transient waveform"
    );
    assert!(
        counts.instance_evaluations > 0,
        "the settle loop must have run on this thread; a zero tally means the \
         counters and the work are no longer on the same thread, not that the \
         work was free"
    );

    counts
}

/// Assert one count against its ceiling, in both directions.
fn assert_ratchet(what: &str, measured: u64, ceiling: u64, constant: &str) {
    assert!(
        measured <= ceiling,
        "{what} is {measured}, over the ceiling of {ceiling}.\n\n\
         This is a cost regression, not a correctness failure: the run is \
         still right, it is doing more work to get there. Find what stopped \
         being skipped or shared before raising {constant} — see this \
         module's header for the two cases that justify a raise and the one \
         that does not."
    );

    let floor = ceiling - ceiling * STALE_CEILING_SLACK_PERCENT / 100;
    assert!(
        measured >= floor,
        "{what} is {measured} but the ceiling is still {ceiling}, a gap of \
         {}.\n\n\
         Lower {constant} to {measured}. A ceiling left far above the real \
         count is not a ratchet — it is headroom for the cost to grow back \
         into.\n\n\
         If the drop is a collapse rather than an improvement, check what \
         removed the work before lowering anything: a copy-on-write cell \
         replaced by an owned copy takes no copy-on-write copies at all, and \
         a structure that is gone reads here exactly like one that got \
         cheaper.",
        ceiling - measured
    );
}

#[test]
fn settle_cost_stays_within_its_ceilings() {
    let counts = measure();

    assert_ratchet(
        "settle-loop instance evaluations",
        counts.instance_evaluations,
        MAX_INSTANCE_EVALUATIONS,
        "MAX_INSTANCE_EVALUATIONS",
    );
    assert_ratchet(
        "instance-cell deep copies",
        counts.instance_deep_copies,
        MAX_INSTANCE_DEEP_COPIES,
        "MAX_INSTANCE_DEEP_COPIES",
    );
    assert_ratchet(
        &format!(
            "event-world deep copies ({} values + {} queue)",
            counts.event_values_deep_copies, counts.event_queue_deep_copies
        ),
        counts.event_world_deep_copies(),
        MAX_EVENT_WORLD_DEEP_COPIES,
        "MAX_EVENT_WORLD_DEEP_COPIES",
    );
}

#[test]
fn settle_costs_are_exactly_reproducible() {
    let first = measure();
    let second = measure();

    assert_eq!(
        first, second,
        "the settle-cost counters must be exactly reproducible run to run, or \
         the ceilings above are thresholds on a number that wobbles rather \
         than a ratchet"
    );
}
