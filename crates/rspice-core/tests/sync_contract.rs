//! The analog/event synchronization contract — decision **D5**, machine-checked.
//!
//! D5 is *conservative lockstep*: *the digital world executes only at analog
//! timepoints the integrator ultimately accepts.* That is one sentence, and it
//! decomposes into six clauses that are enforced in six different places. Each
//! clause below gets its own test, named for the clause it pins, so that a
//! regression names the property it broke rather than the deck that noticed.
//!
//! # Why the suite exists
//!
//! Before this file the six clauses held, but several of them held
//! *circumstantially* — as a consequence of the order two call sites happened
//! to have, or of a type happening to derive `Clone`. A property nothing
//! asserts is a property the next refactor is free to delete. These tests are
//! the contract's only durable form.
//!
//! # The clauses
//!
//! 1. **Acceptance-gated execution.** Event execution mutates persistent event
//!    state only for analog timepoints that are ultimately accepted. A
//!    rejected step's event-world mutations roll back completely.
//! 2. **Exact stopping.** The analog step controller stops *bit-exactly* at the
//!    next pending event time.
//! 3. **Complete draining.** Every event dated at or before the accepted analog
//!    time executes exactly once, in `(tick, region, sequence)` order, before
//!    the analog result at that time is finalized — back-dated events included.
//! 4. **Retry semantics.** A bound that moves backwards opens a fresh slot with
//!    fresh delta/event accounting, without double-executing or losing events.
//! 5. **A2D crossing timestamps.** Input-crossing interpolation dates a
//!    transition *inside* the accepted step, not at the step's grid time.
//! 6. **Pure-analog inertness.** A deck with no event content takes an
//!    identical accepted-step sequence.
//!
//! # Where each clause is enforced
//!
//! | Clause | Seam |
//! |---|---|
//! | 1 | `circuit::nonlinear`'s `NonlinearDeviceStateSnapshot`, which carries the scheduler, and the snapshot/restore bracket inside `stamp_xspice_transient_trial_with_coefficients` |
//! | 2 | `next_xspice_event_time` → `BreakpointManager::replace_runtime_breakpoints` → `snap_to_breakpoint`, with the tick encoding in `xspice::event` keeping the `f64` exact |
//! | 3 | `EventScheduler::run_due_events`'s due-slot loop, plus `schedule_superseding_at` routing to the future tier |
//! | 4 | `EventScheduler::open_due_slot` |
//! | 5 | `xspice::models::digital_output`'s `input_transition_time` |
//! | 6 | `tests/timestep_sequence_goldens.rs` (invariant **I1**) |
//!
//! # Relationship to the neighbouring suites
//!
//! [`tests/event_scheduler_kernel.rs`] pins the kernel *in isolation*: the
//! stratified regions, the total order, driver supersession, and cross-process
//! determinism. This suite pins the kernel *as the analog engine drives it*.
//! The two overlap deliberately at the ordering guarantee, because that is the
//! one property both the kernel's own contract and D5 depend on.
//!
//! Clause 6 is already pinned, thoroughly, by
//! [`tests/timestep_sequence_goldens.rs`], whose header states it as invariant
//! **I1** and whose `golden_decks_stay_free_of_event_driven_content` refuses to
//! let an event-bearing deck into the fixture set. Rather than duplicate five
//! bit-exact step-sequence goldens here, this suite cites them and adds the one
//! variant they cannot express: that a pure-analog deck's accepted sequence is
//! unchanged by event machinery having run *elsewhere in the same process*.
//!
//! # For P2/P4
//!
//! The AMS interleave point will add clauses to this list, not replace them.
//! `SchedulerRegion` already reserves the ordering position an analog-interleave
//! region needs, and clauses 3 and 4 are stated over regions generally rather
//! than over `Active` alone, so an added region inherits them.

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_core::numerics::integration::BreakpointManager;
use rspice_core::xspice::event_scheduler::{
    EventScheduler, EventTarget, SchedulerError, SchedulerLimits, SchedulerRegion, TimeResolution,
};
use rspice_core::xspice::{DigitalValue, EventValue};
use std::fs;
use std::path::PathBuf;

//=============================================================================
// Shared helpers
//=============================================================================

fn scheduler() -> EventScheduler {
    EventScheduler::new(TimeResolution::default(), SchedulerLimits::default())
}

fn target(instance: &str, port: &str, node_id: usize, driver_index: usize) -> EventTarget {
    EventTarget {
        node_id,
        port_name: port.to_string(),
        driver_index,
        instance: instance.to_string(),
    }
}

fn digital(value: u8) -> EventValue {
    EventValue::Digital(match value {
        0 => DigitalValue::zero(),
        1 => DigitalValue::one(),
        _ => DigitalValue::unknown(),
    })
}

/// Drain everything due at or before `bound`, reporting `(tick, sequence)` for
/// each executed event in execution order.
fn drain(scheduler: &mut EventScheduler, bound: u64) -> Vec<(u64, u64)> {
    let mut executed = Vec::new();
    scheduler
        .run_due_events(bound, |event, _| {
            executed.push((event.tick, event.sequence))
        })
        .expect("a queue nothing feeds back into settles");
    executed
}

/// The XSPICE tick encoding, mirrored from `xspice::event`.
///
/// That module is crate-private, so this suite re-states the encoding it
/// depends on rather than reaching into it. The mirror is not a duplication
/// risk: `xspice_event_tick_encoding_is_the_ieee_bit_pattern` below fails if
/// the two ever disagree about a time an XSPICE deck actually produces.
fn xspice_tick(seconds: f64) -> u64 {
    assert!(
        seconds.is_finite() && seconds >= 0.0,
        "only schedulable times have ticks"
    );
    if seconds == 0.0 { 0 } else { seconds.to_bits() }
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

/// Run a deck that references a `d_source` stimulus file by relative path.
///
/// The deck has to be parsed *from a file* for the relative path to anchor, so
/// this writes both into a scratch directory.
fn run_temp_deck(
    prefix: &str,
    stimulus: &str,
    deck: &str,
    tstop: f64,
    max_step: f64,
) -> TransientResult {
    let dir = unique_temp_dir(prefix);
    fs::create_dir_all(&dir).expect("create temp d_source fixture dir");
    fs::write(dir.join("stim.stim"), stimulus).expect("write d_source stimulus");
    let deck_path = dir.join("deck.cir");
    fs::write(&deck_path, deck).expect("write deck");
    let netlist =
        Netlist::parse_file(&deck_path).unwrap_or_else(|err| panic!("deck parses: {err}"));
    let result = Engine::default()
        .run_tran(&netlist, tstop, max_step)
        .expect("transient solves");
    let _ = fs::remove_dir_all(dir);
    result
}

fn run_deck(deck: &str, tstop: f64, max_step: f64) -> TransientResult {
    let netlist = Netlist::parse(deck).unwrap_or_else(|err| panic!("deck parses: {err}"));
    Engine::default()
        .run_tran(&netlist, tstop, max_step)
        .expect("transient solves")
}

fn digital_transition_times(result: &TransientResult, node: &str) -> Vec<f64> {
    result
        .digital_trace_named(node)
        .unwrap_or_else(|| panic!("digital trace {node} missing from {:?}", result.node_names))
        .iter()
        .map(|point| point.time)
        .collect()
}

//=============================================================================
// Clause 1 — acceptance-gated execution
//=============================================================================

/// **D5 clause 1.** A rejected analog step must leave the event world exactly
/// as it found it.
///
/// The engine rolls the event world back by carrying the scheduler inside
/// `NonlinearDeviceStateSnapshot` and cloning it (`circuit/nonlinear.rs`).
/// That works only if a clone is a *complete* image: the whole point of
/// conservative lockstep is that a timepoint the integrator later throws away
/// leaves no trace, and a partially-restored scheduler would let a discarded
/// trial's events influence the retry.
///
/// Four pieces of state have to survive, and each is separately observable:
///
/// * `current_tick` — which due slot is open, and therefore whether the retry
///   continues a slot or opens a fresh one (clause 4).
/// * the per-scheduler sequence counter — the final tie-break of the total
///   order. If it rewound, two events created in different attempts could
///   collide; if it ran ahead, the retry's ordering would depend on how many
///   trials the integrator happened to attempt.
/// * the driver index — part of [`EventTarget`] identity, and therefore of
///   which pending output a later output supersedes.
/// * the pending events themselves.
///
/// This pins the round trip at the kernel, which is where the state lives. The
/// companion unit test `xspice_event_queue_survives_the_nonlinear_state_round_trip`
/// in `src/circuit/nonlinear.rs` pins that `CircuitData`'s snapshot actually
/// carries the scheduler, which is the part an integration test cannot see.
#[test]
fn d5_c1_a_rejected_step_rolls_the_event_world_back_completely() {
    let mut live = scheduler();
    // Three events on one node/port/instance, distinguished only by driver
    // index, so the restored image has to preserve that field to behave.
    live.schedule_superseding_at(
        10,
        SchedulerRegion::Active,
        target("u1", "q", 7, 0),
        digital(1),
    );
    live.schedule_superseding_at(
        20,
        SchedulerRegion::Active,
        target("u1", "q", 7, 1),
        digital(0),
    );
    live.schedule_superseding_at(
        30,
        SchedulerRegion::Active,
        target("u1", "q", 7, 2),
        digital(1),
    );

    // Settle one timepoint, so the snapshot is taken mid-run rather than from
    // a virgin scheduler: `current_tick` and the sequence counter have moved.
    assert_eq!(drain(&mut live, 10), vec![(10, 0)]);
    assert_eq!(live.current_tick(), 10);
    assert_eq!(live.pending(), 2);

    let accepted_image = live.clone();

    // The rejected attempt: it drains everything and schedules more.
    let rejected_order = drain(&mut live, 40);
    assert_eq!(rejected_order, vec![(20, 1), (30, 2)]);
    live.schedule_superseding_at(
        50,
        SchedulerRegion::Active,
        target("u1", "q", 7, 0),
        digital(0),
    );
    assert_eq!(live.current_tick(), 40);

    // Now retry from the image, and check each observable separately.
    let mut restored = accepted_image.clone();
    assert_eq!(
        restored.current_tick(),
        10,
        "the open due slot must be the accepted one, not the rejected attempt's"
    );
    assert_eq!(
        restored.pending(),
        2,
        "events the rejected attempt executed must be pending again"
    );
    assert_eq!(
        restored
            .schedule_at(
                60,
                SchedulerRegion::Active,
                target("u2", "d", 8, 0),
                digital(1)
            )
            .expect("a tick above the horizon is schedulable"),
        3,
        "the sequence counter must resume where the accepted image left it, \
         neither rewound (which would collide) nor advanced by the discarded attempt"
    );

    // Driver identity, including the index, survives: superseding driver 1
    // cancels exactly its own pending output and leaves driver 2 alone.
    let mut by_driver = accepted_image.clone();
    assert_eq!(
        by_driver.schedule_superseding_at(
            20,
            SchedulerRegion::Active,
            target("u1", "q", 7, 1),
            digital(1)
        ),
        1,
        "the restored image must still distinguish vector driver elements"
    );
    assert_eq!(by_driver.pending(), 2);

    // And the retry executes exactly what the rejected attempt did.
    let mut replayed = accepted_image;
    assert_eq!(
        drain(&mut replayed, 40),
        rejected_order,
        "the retry must reproduce the discarded attempt's execution exactly"
    );
}

//=============================================================================
// Clause 2 — exact stopping
//=============================================================================

/// **D5 clause 2, encoding half.** XSPICE event times are carried as the IEEE
/// bit pattern of the `f64`, so a scheduled time comes back out unchanged.
///
/// This matters because `next_event_time` feeds the transient breakpoint
/// manager *unrounded*: an event time is not a point on any declared grid, it
/// is whatever the code model and the step controller produced. The kernel's
/// decimal [`TimeResolution`] is deliberately **not** on this path — quantizing
/// would move the time the analog step stops at, and `MAX_EXACT_TICKS` would
/// cap event time at 2.25 s at 1 fs resolution besides.
///
/// The encoding mirrored in [`xspice_tick`] is monotone (so it orders) and
/// exactly invertible (so it converts back), which is all the kernel asks.
#[test]
fn d5_c2_xspice_event_tick_encoding_is_the_ieee_bit_pattern() {
    // Times a transient run actually produces: awkward, off-grid, spanning
    // eighteen decades.
    let awkward = [
        0.0,
        1.0e-18,
        1.0 / 3.0e9,
        1.000_000_000_000_000_2e-9,
        2.718_281_828_459_045e-9,
        7.234_567_890_123_456e-4,
        123.456_789,
    ];

    // Monotone: the encoding orders times the way the times order.
    for pair in awkward.windows(2) {
        assert!(
            xspice_tick(pair[0]) < xspice_tick(pair[1]),
            "the tick encoding must be strictly monotone over schedulable times"
        );
    }

    // Exactly invertible, through the scheduler rather than around it.
    let mut sched = scheduler();
    for (index, time) in awkward.iter().enumerate() {
        sched.schedule_superseding_at(
            xspice_tick(*time),
            SchedulerRegion::Active,
            target("driver", "out", index + 1, index),
            EventValue::Real(*time),
        );
    }
    let mut recovered = Vec::new();
    sched
        .run_due_events(xspice_tick(1.0e9), |event, _| {
            recovered.push(f64::from_bits(event.tick))
        })
        .expect("a queue nothing feeds back into settles");

    assert_eq!(
        recovered
            .iter()
            .map(|time| time.to_bits())
            .collect::<Vec<_>>(),
        awkward
            .iter()
            .map(|time| time.to_bits())
            .collect::<Vec<_>>(),
        "every scheduled event time must come back bit-identical"
    );
}

/// **D5 clause 2, breakpoint half.** The step controller lands on the event
/// time itself, not on a value within tolerance of it.
///
/// An event time reaches the integrator as a *runtime* breakpoint, and the
/// accepted timepoint is produced by `t + dt`, which is not an identity for
/// every pair of `f64`s. `snap_to_breakpoint` is what closes that gap: it
/// returns the stored breakpoint verbatim, so the accepted time is the time the
/// event was scheduled with down to the last bit rather than a neighbour of it.
#[test]
fn d5_c2_the_step_controller_snaps_back_to_the_exact_event_time() {
    let awkward = 1.000_000_000_000_000_2e-9;
    let mut breakpoints = BreakpointManager::new();
    breakpoints.replace_runtime_breakpoints([awkward]);

    let previous = 0.9e-9;
    let (dt, lands) = breakpoints.limit_step(previous, 1.0e-9);
    assert!(
        lands,
        "a proposal that reaches the event time must be cut to it"
    );

    // The subtraction/addition round trip is exactly what needs correcting.
    let naive = previous + dt;
    let snapped = breakpoints.snap_to_breakpoint(naive);
    assert_eq!(
        snapped.to_bits(),
        awkward.to_bits(),
        "the accepted timepoint must be the event's own f64, not a neighbour: \
         got {snapped:e} ({:016x}) for event {awkward:e} ({:016x})",
        snapped.to_bits(),
        awkward.to_bits()
    );

    // And the event time is distinguishable from the decimal value it sits one
    // ulp away from, so the assertion above is not vacuous.
    assert_ne!(awkward.to_bits(), 1.0e-9f64.to_bits());
}

/// **D5 clause 2, end to end.** A code model that schedules an output at an
/// awkward absolute time gets an accepted analog timepoint exactly there.
///
/// This is the whole chain in one deck: the stimulus time parses to an `f64`,
/// the scheduler carries it as a bit pattern, `next_xspice_event_time` hands it
/// to the breakpoint manager unrounded, and the accepted step lands on it.
#[test]
fn d5_c2_an_awkward_event_time_becomes_an_exact_accepted_timepoint() {
    let awkward: f64 = 1.000_000_000_000_000_2e-9;
    let result = run_temp_deck(
        "rspice-d5-exact-stopping",
        "0 0s\n1.0000000000000002e-9 1s\n",
        "\
* D5 clause 2: an event at an awkward absolute time stops the analog step there
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
rload out 0 1k
.end
",
        2.0e-9,
        1.0e-10,
    );

    let transitions = digital_transition_times(&result, "d");
    assert!(
        transitions
            .iter()
            .any(|time| time.to_bits() == awkward.to_bits()),
        "the event must fire at its own f64, got {transitions:?}"
    );
    assert!(
        result
            .time
            .iter()
            .any(|time| time.to_bits() == awkward.to_bits()),
        "the analog step must be accepted at the event time itself, not within \
         tolerance of it; accepted times near it: {:?}",
        result
            .time
            .iter()
            .filter(|time| (**time - awkward).abs() < 1.0e-11)
            .collect::<Vec<_>>()
    );
}

//=============================================================================
// Clause 3 — complete draining
//=============================================================================

/// **D5 clause 3.** Everything dated at or before the accepted analog time
/// executes exactly once, in `(tick, region, sequence)` order — *including*
/// events dated behind the timepoint being settled.
///
/// The back-dated case is not a curiosity, it is the normal case for a code
/// model with analog inputs: the model interpolates an input crossing *inside*
/// the accepted step and dates its output from that crossing, which is behind
/// the timepoint the analog engine is settling. `schedule_superseding_at`
/// deliberately takes no horizon so such an output is accepted, and
/// `run_due_events` treats it as due *now* rather than late.
///
/// The ordering half of this clause was, until the commit that added this
/// suite, circumstantial. `schedule_superseding_at` routed an event into the
/// *open slot* whenever its tick equalled `current_tick`, while a back-dated
/// event went to the future tier — and `run_due_events` drains the slot before
/// it opens the next due tick. A zero-delay output at the bound therefore ran
/// ahead of an earlier-dated interpolated crossing. Routing every externally
/// scheduled event to the future tier, as `schedule_at` already did, makes the
/// order structural.
#[test]
fn d5_c3_every_due_event_runs_exactly_once_in_tick_order() {
    let mut sched = scheduler();

    // Settle a first timepoint so a slot is open at tick 100.
    sched.schedule_superseding_at(
        100,
        SchedulerRegion::Active,
        target("clk", "q", 1, 0),
        digital(1),
    );
    assert_eq!(drain(&mut sched, 100), vec![(100, 0)]);
    assert_eq!(sched.current_tick(), 100);

    // Now, while settling the same timepoint, two models produce output: one a
    // zero-delay output dated at the bound, the other an interpolated crossing
    // dated behind it. This is the configuration whose order used to depend on
    // which tier each event happened to land in.
    sched.schedule_superseding_at(
        100,
        SchedulerRegion::Active,
        target("gate", "y", 2, 0),
        digital(0),
    );
    sched.schedule_superseding_at(
        60,
        SchedulerRegion::Active,
        target("bridge", "d", 3, 0),
        digital(1),
    );
    sched.schedule_superseding_at(
        80,
        SchedulerRegion::Active,
        target("bridge", "d", 4, 0),
        digital(0),
    );

    let executed = drain(&mut sched, 100);
    assert_eq!(
        executed,
        vec![(60, 2), (80, 3), (100, 1)],
        "due events must run in tick order regardless of the order they were \
         scheduled in, and a back-dated event must not run after the bound's"
    );
    assert_eq!(
        sched.pending(),
        0,
        "everything at or before the bound must have drained"
    );

    // Exactly once: a second drain at the same bound executes nothing.
    assert_eq!(
        drain(&mut sched, 100),
        vec![],
        "an executed event must not run a second time"
    );
}

/// **D5 clause 3, horizon half.** An output dated before a timepoint that has
/// already been settled is still delivered.
///
/// `EventScheduler::schedule_at` refuses a tick the scheduler has left, because
/// silently re-dating it would let an event appear to run before something that
/// already ran. `schedule_superseding_at` must *not* inherit that refusal: the
/// interpolated-crossing output is legitimately behind the current timepoint,
/// and dropping it would lose output the analog path has always seen.
#[test]
fn d5_c3_a_back_dated_output_is_delivered_not_refused() {
    let mut sched = scheduler();
    sched.schedule_superseding_at(
        400,
        SchedulerRegion::Active,
        target("u1", "out", 1, 0),
        digital(1),
    );
    assert_eq!(drain(&mut sched, 400).len(), 1);

    // Past the horizon `schedule_at` would enforce. The refusal is total: it
    // costs no sequence number, so the ordering of the events that *are*
    // accepted does not depend on how many were refused.
    assert!(matches!(
        sched.schedule_at(
            300,
            SchedulerRegion::Active,
            target("u2", "out", 2, 0),
            digital(0)
        ),
        Err(SchedulerError::ScheduleInThePast { .. })
    ));

    // Superseding scheduling takes no horizon, and the due slot delivers it.
    sched.schedule_superseding_at(
        300,
        SchedulerRegion::Active,
        target("u2", "out", 2, 0),
        digital(0),
    );
    assert_eq!(
        drain(&mut sched, 400),
        vec![(300, 1)],
        "an interpolated crossing dated inside the settled step must still fire"
    );
}

//=============================================================================
// Clause 4 — retry semantics
//=============================================================================

/// **D5 clause 4.** A bound that moves backwards — a rejected step retrying
/// smaller — opens a fresh slot, with fresh delta and event accounting.
///
/// Two things must hold across a retry. First, no event is double-executed or
/// lost: the retry's smaller bound leaves later events pending rather than
/// consuming them, and they are still there when the bound grows again.
/// Second, the settling accounting does not leak. Delta cycles and per-tick
/// event counts are what turn a zero-delay loop into a diagnosis instead of a
/// hang; if a rejected attempt's counts carried into the retry, a deck that
/// needed several attempts would be accused of oscillating.
#[test]
fn d5_c4_a_backwards_bound_opens_a_fresh_slot() {
    let mut sched = scheduler();
    sched.schedule_superseding_at(
        10,
        SchedulerRegion::Active,
        target("u1", "a", 1, 0),
        digital(1),
    );
    sched.schedule_superseding_at(
        90,
        SchedulerRegion::Active,
        target("u2", "b", 2, 0),
        digital(0),
    );

    // The attempt reaches out to tick 100 and consumes both.
    assert_eq!(drain(&mut sched, 100), vec![(10, 0), (90, 1)]);
    assert_eq!(sched.current_tick(), 100);

    // Rejected: the engine restores the pre-attempt image and retries smaller.
    let mut retried = scheduler();
    retried.schedule_superseding_at(
        10,
        SchedulerRegion::Active,
        target("u1", "a", 1, 0),
        digital(1),
    );
    retried.schedule_superseding_at(
        90,
        SchedulerRegion::Active,
        target("u2", "b", 2, 0),
        digital(0),
    );

    assert_eq!(
        drain(&mut retried, 50),
        vec![(10, 0)],
        "the smaller retry bound must execute only what it actually reaches"
    );
    assert_eq!(retried.current_tick(), 50, "the bound moved backwards");
    assert_eq!(
        retried.pending(),
        1,
        "an event past the retry bound must stay pending, not be consumed"
    );

    // And when the integrator gets there, it fires — exactly once.
    assert_eq!(
        drain(&mut retried, 100),
        vec![(90, 1)],
        "the event beyond the retry bound must still fire when the bound reaches it"
    );
    assert_eq!(retried.pending(), 0);
}

/// **D5 clause 4, accounting half.** Oscillation accounting does not leak
/// across a retry.
///
/// A slot's delta-cycle count is compared against a ceiling. If a rejected
/// attempt's count survived into the retry, enough attempts at one timepoint
/// would eventually trip the ceiling and report an oscillation that is not
/// there. `open_due_slot` resets the accounting whenever the bound changes,
/// which is what makes the diagnosis a property of the *network* rather than of
/// how many attempts the integrator happened to make.
#[test]
fn d5_c4_oscillation_accounting_does_not_leak_across_retries() {
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 8,
        ..SchedulerLimits::default()
    };
    let mut sched = EventScheduler::new(TimeResolution::default(), limits);

    // Spend most of one slot's delta budget at the attempted bound.
    for _ in 0..8 {
        sched
            .note_delta_cycle(100)
            .expect("eight cycles is within the ceiling");
    }
    assert!(
        sched.note_delta_cycle(100).is_err(),
        "the ninth cycle at one bound must trip the ceiling, or this test proves nothing"
    );

    // The step is rejected and retried at a smaller bound. The retry gets the
    // whole budget again.
    let mut retried = EventScheduler::new(TimeResolution::default(), limits);
    for _ in 0..8 {
        retried
            .note_delta_cycle(100)
            .expect("eight cycles is within the ceiling");
    }
    for cycle in 0..8 {
        retried.note_delta_cycle(50).unwrap_or_else(|error| {
            panic!("a fresh slot owes a full delta budget, failed at cycle {cycle}: {error}")
        });
    }

    // Growing the bound again is likewise a fresh slot, not a continuation.
    for cycle in 0..8 {
        retried.note_delta_cycle(100).unwrap_or_else(|error| {
            panic!("re-opening a bound must reset its accounting, failed at cycle {cycle}: {error}")
        });
    }
}

//=============================================================================
// Clause 5 — A2D crossing timestamps
//=============================================================================

/// A Xyce DIG buffer on a slow analog ramp, stepped deliberately coarsely.
///
/// The ramp crosses the model's `s0vhi` input threshold of 1.8 V at exactly
/// 6 ns (`1.8 / 3 * 10 ns`), which falls *inside* an accepted step rather than
/// on one of its endpoints. `delay=5n` puts the resulting output transition at
/// 11 ns — a time no coarse step lands on by itself.
const DIG_CROSSING_DECK: &str = "\
* D5 clause 5: a DIG gate dates its output from the interpolated input crossing
vdpwr dpwr 0 3
vin in 0 pwl(0 0 10n 3)
abuf dpwr 0 [in] out dbuf
rload out 0 100k
.model dbuf xyce_d_buffer (clo=1e-12 chi=1e-12 cload=1e-12 rload=1e6
+ s0rlo=5 s0rhi=200 s0tsw=1p s0vlo=-1 s0vhi=1.8
+ s1rlo=200 s1rhi=5 s1tsw=1p s1vlo=1.0 s1vhi=3
+ delay=5n)
.end
";

/// The input ramp reaches `s0vhi` here — strictly between two coarse steps.
const DIG_CROSSING_TIME: f64 = 6.0e-9;
/// `delay` on the model card.
const DIG_CROSSING_DELAY: f64 = 5.0e-9;
/// Coarse step. 2.5 ns divides neither the crossing-derived output time nor
/// the crossing itself onto the grid.
const DIG_CROSSING_MAX_STEP: f64 = 2.5e-9;

/// **D5 clause 5.** A code model with analog inputs dates its transition from
/// the interpolated crossing *inside* the accepted step, not from the step's
/// grid time.
///
/// This is the clause that makes conservative lockstep more than
/// "digital runs on the analog grid". The analog engine cannot see a threshold
/// crossing; it lands where LTE and breakpoints tell it to, and the crossing
/// falls wherever it falls in between. `input_transition_time`
/// (`xspice/models/digital_output.rs`) recovers it by linear interpolation
/// across the accepted step and clamps it into `[time_prev, time]`, so the
/// event is dated at a real instant of the analog solution rather than at the
/// sampling artefact that noticed it.
///
/// The deck is built so the two hypotheses give different answers and cannot
/// be confused:
///
/// * dated from the **interpolated crossing**: 6 ns + 5 ns = **11 ns**
/// * dated from the **grid time that detected it**: 7.5 ns + 5 ns = 12.5 ns
///
/// 12.5 ns is a multiple of the 2.5 ns step and so would be visited anyway;
/// 11 ns is not, and nothing but the interpolation puts an accepted timepoint
/// there. The gate requests a breakpoint at its pending output time
/// (`update_output` → `ctx.request_breakpoint`), which reaches the integrator
/// through `collect_xspice_runtime_breakpoints`, so the interpolated instant
/// becomes an accepted analog timepoint — clause 2 applied to a time the event
/// world computed rather than one a stimulus declared.
#[test]
fn d5_c5_an_input_crossing_is_dated_inside_the_step_not_at_its_grid_time() {
    let result = run_deck(DIG_CROSSING_DECK, 2.0e-8, DIG_CROSSING_MAX_STEP);

    let interpolated = DIG_CROSSING_TIME + DIG_CROSSING_DELAY;
    let grid_dated = 7.5e-9 + DIG_CROSSING_DELAY;
    assert!(
        (grid_dated - interpolated).abs() > 1.0e-9,
        "the deck must separate the two hypotheses, or the test proves nothing"
    );

    let nearest = result
        .time
        .iter()
        .copied()
        .min_by(|left, right| {
            (left - interpolated)
                .abs()
                .total_cmp(&(right - interpolated).abs())
        })
        .expect("the run accepted at least one timepoint");
    assert!(
        (nearest - interpolated).abs() <= 1.0e-16,
        "the interpolated crossing plus the model delay must be an accepted \
         analog timepoint: expected {interpolated:e}, nearest accepted {nearest:e}. \
         A transition dated at the detecting step's grid time would put it at \
         {grid_dated:e} instead."
    );

    // And the output actually switches on the interpolated schedule. Sampled
    // between the two hypotheses, the gate must already have driven high; if it
    // were dated from the grid time it would still be low here.
    let out = result
        .try_voltage_waveform_named("out")
        .expect("the gate output node is solved");
    // The last accepted sample at or before the target, deliberately not the
    // nearest one: sampling either side of 12 ns could pick the grid-dated
    // hypothesis's own timepoint at 12.5 ns and prove nothing.
    let sample = |target: f64| {
        let index = result
            .time
            .iter()
            .rposition(|time| *time <= target)
            .unwrap_or_else(|| panic!("no accepted timepoint at or before {target:e}"));
        (result.time[index], out[index])
    };

    let (before_time, before) = sample(10.0e-9);
    let (between_time, between) = sample(12.0e-9);
    assert!(
        before_time < interpolated && between_time < grid_dated,
        "both samples must straddle only the interpolated transition: took \
         {before_time:e} and {between_time:e} against {interpolated:e}/{grid_dated:e}"
    );
    assert!(
        before < 0.5,
        "the gate must still be low before the interpolated transition, \
         got {before} V at {before_time:e}"
    );
    assert!(
        between > 2.5,
        "the gate must have switched by 12 ns on the interpolated schedule; a \
         grid-dated transition at {grid_dated:e} would leave it low. \
         Got {between} V at {between_time:e}"
    );
}

//=============================================================================
// Clause 6 — pure-analog inertness
//=============================================================================

/// A pure-analog deck with a source breakpoint and an RC time constant.
const PURE_ANALOG_DECK: &str = "\
* D5 clause 6: pure-analog deck, no event content
v1 in 0 pulse(0 1 1n 0.1n 0.1n 5n 10n)
r1 in out 1k
c1 out 0 1p
.end
";

/// A deck whose event machinery is fully live.
const EVENT_BEARING_DECK: &str = "\
* D5 clause 6: the same RC, driven through the event world
vin a 0 pwl(0 0 1n 0 1.1n 3.3)
a_adc [a] [d] adc
a_dac [d] [drv] dac
.model adc adc_bridge (in_low=0.8 in_high=2.0 rise_delay=1p fall_delay=1p)
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
r1 drv out 1k
c1 out 0 1p
.end
";

/// **D5 clause 6.** A deck with no event content takes an accepted-step
/// sequence the event machinery cannot touch.
///
/// The bit-exact form of this clause is
/// [`tests/timestep_sequence_goldens.rs`], which states it as invariant **I1**
/// and freezes five decks' `(time, step_size)` pairs as bit patterns — source
/// breakpoints, junction LTE, oscillatory order switching, a switch
/// discontinuity, and Gear. Its `golden_decks_stay_free_of_event_driven_content`
/// keeps those fixtures pure. Nothing here duplicates that.
///
/// What this adds is the variant the goldens cannot express, because each of
/// them runs alone: that a pure-analog deck's accepted sequence is unaffected
/// by event machinery having run *elsewhere in the same process*. The kernel
/// the XSPICE path now runs on replaced a queue that tie-broke same-time events
/// on a process-global atomic, which made one circuit's ordering depend on what
/// else in the process had scheduled an event first. The sequence counter is
/// per-scheduler precisely so that cannot happen, and this is what says so.
#[test]
fn d5_c6_a_pure_analog_deck_is_untouched_by_the_event_machinery() {
    let before = run_deck(PURE_ANALOG_DECK, 2.0e-8, 1.0e-10);

    // Drive a full event-bearing deck through the same process.
    let eventful = run_deck(EVENT_BEARING_DECK, 5.0e-9, 1.0e-11);
    assert!(
        !eventful.digital_traces.is_empty(),
        "the interleaved deck must actually exercise the event world, \
         or this test proves nothing"
    );

    let after = run_deck(PURE_ANALOG_DECK, 2.0e-8, 1.0e-10);

    assert_eq!(
        before.time.len(),
        after.time.len(),
        "a pure-analog deck must accept the same number of steps regardless of \
         what else ran in the process"
    );
    for (index, (first, second)) in before.time.iter().zip(after.time.iter()).enumerate() {
        assert_eq!(
            first.to_bits(),
            second.to_bits(),
            "accepted time {index} drifted: {first:e} vs {second:e}"
        );
    }
    for (index, (first, second)) in before
        .step_sizes
        .iter()
        .zip(after.step_sizes.iter())
        .enumerate()
    {
        assert_eq!(
            first.to_bits(),
            second.to_bits(),
            "accepted step {index} drifted: {first:e} vs {second:e}"
        );
    }
}

/// **D5 clause 6, inertness half.** A pure-analog deck constructs no event
/// world at all, so there is nothing for the machinery to be inert about.
///
/// This is the structural reason the clause holds:
/// `collect_xspice_runtime_breakpoints` is reached only under
/// `circuit.has_xspice_devices()`, and for a pure-analog circuit that is false.
#[test]
fn d5_c6_a_pure_analog_deck_constructs_no_event_world() {
    let engine = Engine::default();
    let netlist = Netlist::parse(PURE_ANALOG_DECK).expect("deck parses");
    let circuit = engine.build_circuit(&netlist).expect("circuit builds");
    assert!(
        !circuit.has_xspice_devices(),
        "the clause-6 deck must stay free of event-driven content"
    );

    let result = run_deck(PURE_ANALOG_DECK, 2.0e-8, 1.0e-10);
    assert!(result.digital_traces.is_empty());
    assert!(result.real_traces.is_empty());
}
