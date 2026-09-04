//! Conformance tests for the discrete-event scheduler kernel.
//!
//! The XSPICE path runs on the kernel, but it exercises the kernel through one
//! shape: everything in the active region, driven by an outer settle loop.
//! These tests are what holds the rest — the stratified regions, the total
//! order, driver supersession, and the two properties everything above the
//! kernel rests on: the same schedule produces the same ordering on every run
//! and in every process, and that ordering is the one IEEE 1364-2005
//! specifies.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::env;
use std::process::Command;

use rspice_core::xspice::event_scheduler::{
    EventScheduler, EventTarget, OscillationCause, SchedulerContext, SchedulerError,
    SchedulerLimits, SchedulerRegion, TimeResolution, TimeSlotReport,
};
use rspice_core::xspice::{DigitalValue, EventValue};

/// Environment variable that turns the child probe below into an order dump.
const CHILD_ENV: &str = "RSPICE_SCHEDULER_ORDER_CHILD";

/// Marker the child prints its ordering behind.
const ORDER_MARKER: &str = "SCHEDULER_ORDER:";

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

fn scheduler() -> EventScheduler {
    EventScheduler::new(TimeResolution::default(), SchedulerLimits::default())
}

fn region_tag(region: SchedulerRegion) -> &'static str {
    match region {
        SchedulerRegion::Active => "A",
        SchedulerRegion::Inactive => "I",
        SchedulerRegion::NonBlockingAssign => "N",
        SchedulerRegion::Monitor => "M",
    }
}

/// Deterministic 64-bit PRNG. A fixed algorithm with a fixed seed keeps the
/// generated schedules identical across runs, processes and platforms, which is
/// the whole point of the determinism tests below.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

//=============================================================================
// Tick <-> seconds conversion
//=============================================================================

#[test]
fn tick_seconds_round_trip_is_lossless_across_the_exact_range() {
    for exponent in [-21i8, -18, -15, -12, -9, -6, -3, 0] {
        let resolution = TimeResolution::new(exponent).expect("supported resolution");
        let mut rng = Rng::new(0x2545_F491_4F6C_DD1D);

        let mut candidates = vec![
            0,
            1,
            2,
            999,
            1_000_000,
            TimeResolution::MAX_EXACT_TICKS,
            TimeResolution::MAX_EXACT_TICKS - 1,
        ];
        for bit in 0..51 {
            let base = 1u64 << bit;
            candidates.push(base);
            candidates.push(base - 1);
            candidates.push(base + 1);
        }
        for _ in 0..20_000 {
            candidates.push(rng.below(TimeResolution::MAX_EXACT_TICKS) + 1);
        }

        for ticks in candidates {
            let seconds = resolution
                .ticks_to_seconds(ticks)
                .expect("tick inside the exact range converts");
            let back = resolution
                .seconds_to_ticks(seconds)
                .expect("seconds from an exact tick convert back");
            assert_eq!(
                back, ticks,
                "round trip lost a tick at 1e{exponent} s: {ticks} -> {seconds:e} -> {back}"
            );
        }
    }
}

#[test]
fn ticks_past_the_exact_range_are_refused_rather_than_rounded() {
    let resolution = TimeResolution::default();
    let over = TimeResolution::MAX_EXACT_TICKS + 1;

    let error = resolution
        .ticks_to_seconds(over)
        .expect_err("a tick with no exact seconds image must be refused");
    assert_eq!(
        error,
        SchedulerError::TickNotExactlyRepresentable { ticks: over }
    );
    assert!(
        error.to_string().contains("no exact seconds image"),
        "unhelpful diagnostic: {error}"
    );

    let seconds = resolution.seconds_per_tick() * (over as f64);
    assert!(
        resolution.seconds_to_ticks(seconds).is_err(),
        "seconds past the exact range must be refused too"
    );
}

#[test]
fn unrepresentable_seconds_are_refused() {
    let resolution = TimeResolution::default();
    for seconds in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0e-12] {
        let error = resolution
            .seconds_to_ticks(seconds)
            .expect_err("non-finite or negative seconds have no tick");
        assert!(matches!(
            error,
            SchedulerError::SecondsNotRepresentable { .. }
        ));
    }
}

#[test]
fn resolution_range_is_enforced_and_finest_selects_the_finer_precision() {
    assert!(TimeResolution::new(1).is_err(), "coarser than 1 s");
    assert!(TimeResolution::new(-22).is_err(), "finer than 1e-21 s");
    assert_eq!(
        TimeResolution::new(1),
        Err(SchedulerError::UnsupportedResolution { exponent: 1 })
    );

    let picoseconds = TimeResolution::new(-12).expect("1 ps");
    let femtoseconds = TimeResolution::new(-15).expect("1 fs");
    assert_eq!(picoseconds.finest(femtoseconds), femtoseconds);
    assert_eq!(femtoseconds.finest(picoseconds), femtoseconds);
    assert_eq!(TimeResolution::default(), femtoseconds);

    // The elaboration fold: whichever order the modules are visited in, the
    // kernel resolution is the finest precision declared.
    let declared = [
        TimeResolution::new(-9).expect("1 ns"),
        femtoseconds,
        picoseconds,
    ];
    let folded = declared
        .iter()
        .copied()
        .fold(TimeResolution::new(0).expect("1 s"), TimeResolution::finest);
    assert_eq!(folded, femtoseconds);
    assert_eq!(femtoseconds.seconds_per_tick(), 1.0e-15);
}

#[test]
fn tick_conversion_feeds_analog_breakpoints_exactly() {
    // The value W1.4 will hand the breakpoint manager: a whole number of
    // picoseconds must land on the seconds value an analog step can stop at.
    let resolution = TimeResolution::new(-12).expect("1 ps");
    assert_eq!(resolution.ticks_to_seconds(1), Ok(1.0e-12));
    assert_eq!(resolution.ticks_to_seconds(1_000), Ok(1.0e-9));
    assert_eq!(resolution.ticks_to_seconds(0), Ok(0.0));
    assert_eq!(resolution.seconds_to_ticks(1.0e-9), Ok(1_000));
}

//=============================================================================
// Stratified region ordering
//=============================================================================

#[test]
fn regions_run_active_then_inactive_then_nba_then_monitor() {
    let mut scheduler = scheduler();
    // Scheduled deliberately out of region order so the ordering under test is
    // the region rank, not insertion order.
    for region in [
        SchedulerRegion::Monitor,
        SchedulerRegion::NonBlockingAssign,
        SchedulerRegion::Inactive,
        SchedulerRegion::Active,
    ] {
        scheduler
            .schedule_at(7, region, target("dut", "out", 1, 0), digital(1))
            .expect("schedule");
    }

    let mut order = Vec::new();
    let report = scheduler
        .run_time_slot(|event, _| order.push(region_tag(event.region)))
        .expect("slot settles")
        .expect("a slot ran");

    assert_eq!(order, vec!["A", "I", "N", "M"]);
    assert_eq!(report.tick, 7);
    assert_eq!(report.events_executed, 4);
    // Three promotions: active -> inactive -> NBA -> monitor.
    assert_eq!(report.delta_cycles, 3);
    assert_eq!(scheduler.pending(), 0);
    assert_eq!(scheduler.current_tick(), 7);
    assert_eq!(scheduler.next_tick(), None);
}

#[test]
fn nonblocking_updates_become_visible_only_in_the_next_delta() {
    let mut scheduler = scheduler();
    let mut state: BTreeMap<usize, u8> = BTreeMap::new();
    state.insert(1, 0);
    let mut observations: Vec<(&'static str, u8)> = Vec::new();

    // A seed active event that queues a nonblocking update to node 1 and a
    // same-delta active reader.
    scheduler
        .schedule_at(
            0,
            SchedulerRegion::Active,
            target("dut", "seed", 0, 0),
            digital(1),
        )
        .expect("schedule");

    scheduler
        .run_time_slot(|event, ctx| match event.target.port_name.as_str() {
            "seed" => {
                ctx.schedule_after(
                    0,
                    SchedulerRegion::NonBlockingAssign,
                    target("dut", "nba_update", 1, 0),
                    digital(1),
                )
                .expect("queue the nonblocking update");
                ctx.schedule_after(
                    0,
                    SchedulerRegion::Active,
                    target("dut", "same_delta_reader", 1, 0),
                    digital(0),
                )
                .expect("queue the same-delta reader");
            }
            "same_delta_reader" => {
                observations.push(("same_delta", state[&1]));
            }
            "nba_update" => {
                state.insert(1, 1);
                ctx.schedule_after(
                    0,
                    SchedulerRegion::Active,
                    target("dut", "next_delta_reader", 1, 0),
                    digital(0),
                )
                .expect("queue the next-delta reader");
            }
            "next_delta_reader" => {
                observations.push(("next_delta", state[&1]));
            }
            other => panic!("unexpected event port {other}"),
        })
        .expect("slot settles")
        .expect("a slot ran");

    assert_eq!(
        observations,
        vec![("same_delta", 0), ("next_delta", 1)],
        "a nonblocking update must not be visible to the delta that queued it"
    );
}

#[test]
fn monitor_region_observes_a_settled_slot() {
    let mut scheduler = scheduler();
    let mut writes = 0usize;
    let mut monitor_saw = Vec::new();

    scheduler
        .schedule_at(
            3,
            SchedulerRegion::Monitor,
            target("dut", "monitor", 9, 0),
            digital(0),
        )
        .expect("schedule");
    scheduler
        .schedule_at(
            3,
            SchedulerRegion::Active,
            target("dut", "write", 1, 0),
            digital(1),
        )
        .expect("schedule");

    scheduler
        .run_time_slot(|event, ctx| match event.target.port_name.as_str() {
            "write" => {
                writes += 1;
                // Chain one more write through the inactive region; the monitor
                // must still run after all of it.
                if writes == 1 {
                    ctx.schedule_after(
                        0,
                        SchedulerRegion::Inactive,
                        target("dut", "write", 2, 0),
                        digital(1),
                    )
                    .expect("queue a deferred write");
                }
            }
            "monitor" => monitor_saw.push(writes),
            other => panic!("unexpected event port {other}"),
        })
        .expect("slot settles")
        .expect("a slot ran");

    assert_eq!(writes, 2);
    assert_eq!(
        monitor_saw,
        vec![2],
        "the monitor region must observe every write in the slot"
    );
}

//=============================================================================
// Quiescence and the oscillation diagnostic
//=============================================================================

#[test]
fn a_bounded_delta_chain_settles_and_reports_its_cycles() {
    let mut scheduler = scheduler();
    scheduler
        .schedule_at(
            0,
            SchedulerRegion::Active,
            target("dut", "step", 0, 0),
            digital(0),
        )
        .expect("schedule");

    let mut remaining = 5u32;
    let report = scheduler
        .run_time_slot(|_, ctx| {
            if remaining > 0 {
                remaining -= 1;
                ctx.schedule_after(
                    0,
                    SchedulerRegion::Inactive,
                    target("dut", "step", 0, 0),
                    digital(0),
                )
                .expect("queue the next delta");
            }
        })
        .expect("a terminating chain settles")
        .expect("a slot ran");

    assert_eq!(report.events_executed, 6);
    assert_eq!(
        report.delta_cycles, 5,
        "each inactive promotion is one delta cycle"
    );
    assert_eq!(scheduler.pending(), 0);
}

#[test]
fn a_zero_delay_active_loop_is_diagnosed_with_its_drivers() {
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 64,
        max_events_per_tick: 500,
        max_reported_oscillating_entities: 4,
    };
    let mut scheduler = EventScheduler::new(TimeResolution::default(), limits);
    scheduler
        .schedule_at(
            0,
            SchedulerRegion::Active,
            target("loop_a", "q", 1, 0),
            digital(0),
        )
        .expect("schedule");

    // Two drivers feeding each other with no delay, plus a quiet bystander that
    // fires once. The report must name the loop, not the bystander.
    scheduler
        .schedule_at(
            0,
            SchedulerRegion::Active,
            target("quiet", "q", 3, 0),
            digital(0),
        )
        .expect("schedule");

    let error = scheduler
        .run_time_slot(|event, ctx| {
            let next = match event.target.instance.as_str() {
                "loop_a" => "loop_b",
                "loop_b" => "loop_a",
                _ => return,
            };
            ctx.schedule_after(
                0,
                SchedulerRegion::Active,
                target(next, "q", 2, 0),
                digital(1),
            )
            .expect("queue the zero-delay partner");
        })
        .expect_err("a zero-delay loop must be diagnosed, not run forever");

    let SchedulerError::Oscillation(diagnostic) = error else {
        panic!("wrong error for a zero-delay loop: {error}");
    };
    assert_eq!(diagnostic.cause, OscillationCause::EventLimit);
    assert_eq!(diagnostic.tick, 0);
    assert_eq!(diagnostic.event_limit, 500);
    assert!(diagnostic.events_executed > 500);
    assert!(
        diagnostic.entities.len() <= 4,
        "the report must respect its own cap"
    );

    let named: Vec<&str> = diagnostic
        .entities
        .iter()
        .map(|(target, _)| target.instance.as_str())
        .collect();
    assert!(
        named.contains(&"loop_a") && named.contains(&"loop_b"),
        "the diagnostic must name the oscillating drivers, got {named:?}"
    );
    // The bystander fired once; the loop members fired hundreds of times, so
    // descending activation order puts the loop first.
    assert_ne!(named.first(), Some(&"quiet"));
    assert!(diagnostic.entities[0].1 > 1);
}

#[test]
fn region_ping_pong_trips_the_delta_cycle_limit() {
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 32,
        max_events_per_tick: u64::MAX,
        max_reported_oscillating_entities: 8,
    };
    let mut scheduler = EventScheduler::new(TimeResolution::default(), limits);
    scheduler
        .schedule_at(
            0,
            SchedulerRegion::Active,
            target("ping", "q", 1, 0),
            digital(0),
        )
        .expect("schedule");

    // Every event lands in the inactive region, so the active region empties on
    // each iteration and the tick advances one delta cycle at a time forever.
    let error = scheduler
        .run_time_slot(|_, ctx| {
            ctx.schedule_after(
                0,
                SchedulerRegion::Inactive,
                target("ping", "q", 1, 0),
                digital(0),
            )
            .expect("queue the next delta");
        })
        .expect_err("an unbounded delta chain must be diagnosed");

    let SchedulerError::Oscillation(diagnostic) = error else {
        panic!("wrong error for a non-settling delta chain: {error}");
    };
    assert_eq!(diagnostic.cause, OscillationCause::DeltaCycleLimit);
    assert_eq!(diagnostic.delta_cycle_limit, 32);
    assert!(diagnostic.delta_cycles > 32);
    assert_eq!(diagnostic.entities.len(), 1);
    assert_eq!(diagnostic.entities[0].0.instance, "ping");
    assert_eq!(diagnostic.entities[0].1, diagnostic.events_executed);
}

//=============================================================================
// Future tier
//=============================================================================

#[test]
fn an_empty_scheduler_runs_no_slot() {
    let mut scheduler = scheduler();
    assert_eq!(scheduler.next_tick(), None);
    assert_eq!(scheduler.pending(), 0);
    let report = scheduler
        .run_time_slot(|_, _| panic!("no event should execute"))
        .expect("an empty scheduler is not an error");
    assert!(report.is_none());
}

#[test]
fn future_events_run_in_tick_then_region_then_sequence_order() {
    let mut scheduler = scheduler();
    // Insert across ticks and regions in a deliberately scrambled order.
    let plan = [
        (30u64, SchedulerRegion::Active),
        (10, SchedulerRegion::Monitor),
        (20, SchedulerRegion::NonBlockingAssign),
        (10, SchedulerRegion::Active),
        (30, SchedulerRegion::Inactive),
        (10, SchedulerRegion::Active),
        (20, SchedulerRegion::Active),
    ];
    for (index, (tick, region)) in plan.iter().enumerate() {
        scheduler
            .schedule_at(*tick, *region, target("dut", "out", index, 0), digital(1))
            .expect("schedule");
    }
    assert_eq!(scheduler.pending(), plan.len());
    assert_eq!(scheduler.next_tick(), Some(10));

    let mut observed = Vec::new();
    while let Some(report) = scheduler
        .run_time_slot(|event, _| {
            observed.push((event.tick, region_tag(event.region), event.sequence))
        })
        .expect("slots settle")
    {
        assert!(report.tick >= 10);
    }

    // Two events share tick 10 in the active region; the earlier-scheduled one
    // (sequence 3) must precede the later (sequence 5).
    assert_eq!(
        observed,
        vec![
            (10, "A", 3),
            (10, "A", 5),
            (10, "M", 1),
            (20, "A", 6),
            (20, "N", 2),
            (30, "A", 0),
            (30, "I", 4),
        ]
    );
    assert_eq!(scheduler.pending(), 0);
}

#[test]
fn scheduling_into_a_tick_already_left_is_refused() {
    let mut scheduler = scheduler();
    scheduler
        .schedule_at(
            5,
            SchedulerRegion::Active,
            target("dut", "out", 1, 0),
            digital(1),
        )
        .expect("schedule");
    scheduler
        .run_time_slot(|_, _| {})
        .expect("slot settles")
        .expect("a slot ran");

    let error = scheduler
        .schedule_at(
            5,
            SchedulerRegion::Active,
            target("dut", "out", 1, 0),
            digital(1),
        )
        .expect_err("the scheduler has already left tick 5");
    assert_eq!(
        error,
        SchedulerError::ScheduleInThePast {
            current_tick: 5,
            requested_tick: 5,
        }
    );
    assert!(error.to_string().contains("has reached tick 5"));

    scheduler
        .schedule_at(
            6,
            SchedulerRegion::Active,
            target("dut", "out", 1, 0),
            digital(1),
        )
        .expect("the next tick is still schedulable");
}

#[test]
fn an_executing_event_cannot_schedule_into_the_past() {
    let mut scheduler = scheduler();
    scheduler
        .schedule_at(
            4,
            SchedulerRegion::Active,
            target("dut", "out", 1, 0),
            digital(1),
        )
        .expect("schedule");

    let mut refused = None;
    scheduler
        .run_time_slot(|_, ctx| {
            refused = Some(ctx.schedule_at(
                ctx.current_tick() - 1,
                SchedulerRegion::Active,
                target("dut", "out", 1, 0),
                digital(1),
            ));
        })
        .expect("slot settles")
        .expect("a slot ran");

    assert_eq!(
        refused,
        Some(Err(SchedulerError::ScheduleInThePast {
            current_tick: 4,
            requested_tick: 3,
        }))
    );
}

#[test]
fn the_last_representable_tick_is_scheduled_into_the_future_tier() {
    // A sentinel tick standing in for "no slot is open" would route an event
    // scheduled at that tick into whichever slot was running. `u64::MAX` is a
    // schedulable tick, so it must reach the future tier like any other.
    let mut scheduler = scheduler();
    scheduler
        .schedule_at(
            u64::MAX,
            SchedulerRegion::Active,
            target("dut", "far", 1, 0),
            digital(1),
        )
        .expect("schedule");
    scheduler
        .schedule_at(
            1,
            SchedulerRegion::Active,
            target("dut", "near", 1, 0),
            digital(1),
        )
        .expect("schedule");

    assert_eq!(scheduler.next_tick(), Some(1));

    let mut order = Vec::new();
    while scheduler
        .run_time_slot(|event, _| order.push((event.tick, event.target.port_name.clone())))
        .expect("slots settle")
        .is_some()
    {}

    assert_eq!(
        order,
        vec![(1, "near".to_string()), (u64::MAX, "far".to_string())]
    );
    // The far tick has no exact seconds image, which is a conversion concern
    // and not a scheduling one: it still orders correctly.
    assert!(scheduler.resolution().ticks_to_seconds(u64::MAX).is_err());
}

//=============================================================================
// Driver supersession and the outer-loop due slot
//=============================================================================

/// Run every event due at or before `bound`, returning `(tick, port, value)`.
fn drain_due(scheduler: &mut EventScheduler, bound: u64) -> Vec<(u64, String, EventValue)> {
    let mut seen = Vec::new();
    scheduler
        .run_due_events(bound, |event, _| {
            seen.push((event.tick, event.target.port_name.clone(), event.value))
        })
        .expect("a due slot with no feedback settles");
    seen
}

#[test]
fn a_later_output_supersedes_this_driver_but_not_a_co_driver() {
    let mut scheduler = scheduler();
    let driver = target("a_driver", "out", 1, 0);

    scheduler.schedule_superseding_at(5, SchedulerRegion::Active, driver.clone(), digital(0));
    scheduler.schedule_superseding_at(20, SchedulerRegion::Active, driver.clone(), digital(1));
    // Same node, different port: a co-driver, so untouched by the supersede.
    scheduler.schedule_superseding_at(
        20,
        SchedulerRegion::Active,
        target("a_driver", "other", 1, 0),
        digital(2),
    );

    // Deciding on 10 cancels the driver's own event at 20 and leaves the one
    // at 5, which is already committed to a tick this one does not reach back
    // to.
    let cancelled =
        scheduler.schedule_superseding_at(10, SchedulerRegion::Active, driver.clone(), digital(2));
    assert_eq!(cancelled, 1, "only this driver's event at 20 is cancelled");
    assert_eq!(scheduler.pending(), 3);

    let drained = drain_due(&mut scheduler, 20);
    let values: Vec<_> = drained.iter().map(|(_, _, value)| *value).collect();
    assert!(values.contains(&digital(0)), "the event at 5 survives");
    assert!(values.contains(&digital(2)), "the superseding event runs");
    assert!(
        drained
            .iter()
            .any(|(tick, port, _)| *tick == 20 && port == "other"),
        "the co-driver keeps its event at 20"
    );
    assert!(
        !drained
            .iter()
            .any(|(tick, port, _)| *tick == 20 && port == "out"),
        "the superseded output must not be delivered"
    );
}

#[test]
fn superseding_cancels_every_pending_event_at_or_after_the_tick_and_no_other() {
    // The two-event case above fixes the boundary; this one fixes the *set*.
    // A driver may hold several unexecuted outputs at once — a code model that
    // queues a waveform does — and deciding again at one of their ticks must
    // cancel that one and everything after it, deliver everything before it,
    // and report the count exactly. The driver index is what answers this, so
    // this is the test that holds its shape.
    let mut scheduler = scheduler();
    let driver = target("a_driver", "out", 1, 0);
    let bystander = target("a_driver", "other", 1, 0);

    for (tick, value) in [(5u64, 0u8), (12, 1), (18, 2), (25, 1), (31, 0)] {
        scheduler
            .schedule_at(
                tick,
                SchedulerRegion::Active,
                driver.clone(),
                digital(value),
            )
            .expect("nothing has run");
    }
    for tick in [12u64, 25] {
        scheduler
            .schedule_at(tick, SchedulerRegion::Active, bystander.clone(), digital(2))
            .expect("nothing has run");
    }
    assert_eq!(scheduler.pending(), 7);

    let cancelled =
        scheduler.schedule_superseding_at(12, SchedulerRegion::Active, driver.clone(), digital(2));
    assert_eq!(
        cancelled, 4,
        "the events at 12, 18, 25 and 31 are cancelled"
    );
    assert_eq!(
        scheduler.pending(),
        4,
        "the driver's event at 5, its replacement at 12, and both of the \
         co-driver's events are left"
    );

    let drained = drain_due(&mut scheduler, 40);
    assert_eq!(
        drained,
        vec![
            (5, "out".to_string(), digital(0)),
            (12, "other".to_string(), digital(2)),
            (12, "out".to_string(), digital(2)),
            (25, "other".to_string(), digital(2)),
        ],
        "only the surviving events run, in total order"
    );

    // And the driver's index is left consistent by the cancellation: deciding
    // again after everything has run cancels nothing.
    assert_eq!(
        scheduler.schedule_superseding_at(41, SchedulerRegion::Active, driver, digital(1)),
        0
    );
}

#[test]
fn superseding_at_the_same_tick_replaces_the_pending_value() {
    let mut scheduler = scheduler();
    let driver = target("a_driver", "out", 1, 0);

    scheduler.schedule_superseding_at(7, SchedulerRegion::Active, driver.clone(), digital(0));
    let cancelled =
        scheduler.schedule_superseding_at(7, SchedulerRegion::Active, driver.clone(), digital(1));

    assert_eq!(cancelled, 1);
    assert_eq!(
        drain_due(&mut scheduler, 7),
        vec![(7, "out".to_string(), digital(1))]
    );
}

#[test]
fn an_executed_event_is_no_longer_superseded() {
    let mut scheduler = scheduler();
    let driver = target("a_driver", "out", 1, 0);

    scheduler.schedule_superseding_at(3, SchedulerRegion::Active, driver.clone(), digital(0));
    assert_eq!(drain_due(&mut scheduler, 3).len(), 1);

    // Nothing is left to cancel: the value is already in the world.
    let cancelled =
        scheduler.schedule_superseding_at(3, SchedulerRegion::Active, driver.clone(), digital(1));
    assert_eq!(cancelled, 0);
    assert_eq!(scheduler.pending(), 1);
}

#[test]
fn a_due_slot_runs_several_ticks_in_one_call_in_total_order() {
    let mut scheduler = scheduler();
    for tick in [30u64, 10, 20] {
        scheduler.schedule_superseding_at(
            tick,
            SchedulerRegion::Active,
            target("dut", "p", 1, tick as usize),
            digital(1),
        );
    }

    let ticks: Vec<u64> = drain_due(&mut scheduler, 25)
        .into_iter()
        .map(|(tick, _, _)| tick)
        .collect();
    assert_eq!(
        ticks,
        vec![10, 20],
        "everything at or under the bound, in order"
    );
    assert_eq!(scheduler.next_tick(), Some(30));
}

#[test]
fn a_due_slot_delivers_an_event_dated_before_the_bound_it_already_reached() {
    // A code model interpolates an input crossing inside the accepted analog
    // step and dates its output from that crossing, which lands before the
    // timepoint being settled. `schedule_at` refuses that; the superseding
    // schedule admits it and the next drain delivers it.
    let mut scheduler = scheduler();
    scheduler.schedule_superseding_at(
        100,
        SchedulerRegion::Active,
        target("dut", "seed", 1, 0),
        digital(1),
    );
    assert_eq!(drain_due(&mut scheduler, 100).len(), 1);

    assert!(matches!(
        scheduler.schedule_at(
            60,
            SchedulerRegion::Active,
            target("dut", "back", 1, 0),
            digital(0)
        ),
        Err(SchedulerError::ScheduleInThePast { .. })
    ));

    scheduler.schedule_superseding_at(
        60,
        SchedulerRegion::Active,
        target("dut", "back", 1, 0),
        digital(0),
    );
    assert_eq!(
        drain_due(&mut scheduler, 100),
        vec![(60, "back".to_string(), digital(0))]
    );
}

#[test]
fn an_outer_settle_loop_that_will_not_quiet_is_diagnosed_with_its_drivers() {
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 32,
        max_events_per_tick: 1_000_000,
        max_reported_oscillating_entities: 4,
    };
    let mut scheduler = EventScheduler::new(TimeResolution::default(), limits);

    // The shape the XSPICE settle loop has: drain, evaluate, schedule again at
    // the same timepoint, mark a delta, repeat. Nothing here ever quiets.
    let mut error = None;
    for cycle in 0..1_000 {
        scheduler.schedule_superseding_at(
            9,
            SchedulerRegion::Active,
            target("osc_a", "q", 1, 0),
            digital((cycle % 2) as u8),
        );
        scheduler.schedule_superseding_at(
            9,
            SchedulerRegion::Active,
            target("osc_b", "q", 2, 0),
            digital((cycle % 2) as u8),
        );
        if cycle == 0 {
            scheduler.schedule_superseding_at(
                9,
                SchedulerRegion::Active,
                target("quiet", "q", 3, 0),
                digital(1),
            );
        }
        scheduler
            .run_due_events(9, |_, _| {})
            .expect("draining never oscillates on its own here");
        if let Err(reported) = scheduler.note_delta_cycle(9) {
            error = Some(reported);
            break;
        }
    }

    let Some(SchedulerError::Oscillation(diagnostic)) = error else {
        panic!("an outer loop that never quiets must be diagnosed, got {error:?}");
    };
    assert_eq!(diagnostic.tick, 9);
    assert_eq!(diagnostic.cause, OscillationCause::DeltaCycleLimit);
    assert_eq!(diagnostic.delta_cycles, 33);
    let named: Vec<&str> = diagnostic
        .entities
        .iter()
        .map(|(target, _)| target.instance.as_str())
        .collect();
    assert_eq!(
        &named[..2],
        &["osc_a", "osc_b"],
        "the loop is named before the bystander, got {named:?}"
    );
    assert!(diagnostic.entities.iter().all(|(_, count)| *count > 0));
}

#[test]
fn moving_the_due_bound_opens_a_fresh_slot() {
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 4,
        ..SchedulerLimits::default()
    };
    let mut scheduler = EventScheduler::new(TimeResolution::default(), limits);

    for _ in 0..4 {
        scheduler.note_delta_cycle(1).expect("under the ceiling");
    }
    // A retried analog step settles a different timepoint, so its budget is
    // its own — including a bound that moved backwards.
    scheduler.note_delta_cycle(0).expect("a fresh slot");
    for _ in 0..3 {
        scheduler
            .note_delta_cycle(0)
            .expect("still under the ceiling");
    }
    assert!(matches!(
        scheduler.note_delta_cycle(0),
        Err(SchedulerError::Oscillation(_))
    ));
}

#[test]
fn a_cloned_scheduler_keeps_the_ordering_of_the_original() {
    // Transient trials snapshot the scheduler and restore it when the step is
    // rejected, so a clone has to be the same queue, not just the same events.
    let mut scheduler = scheduler();
    for tick in [4u64, 1, 9] {
        scheduler.schedule_superseding_at(
            tick,
            SchedulerRegion::Active,
            target("dut", "p", 1, tick as usize),
            digital(1),
        );
    }

    let mut restored = scheduler.clone();
    scheduler.schedule_superseding_at(
        1,
        SchedulerRegion::Active,
        target("dut", "p", 1, 1),
        digital(0),
    );

    assert_eq!(
        drain_due(&mut restored, 9)
            .into_iter()
            .map(|(tick, _, value)| (tick, value))
            .collect::<Vec<_>>(),
        vec![(1, digital(1)), (4, digital(1)), (9, digital(1))],
        "the clone must not see the supersede applied to the original"
    );
}

//=============================================================================
// Determinism
//=============================================================================

/// A schedule whose ordering depends on every part of the total order: several
/// ticks, all four regions, repeated drivers, and events created during
/// execution.
fn ordering_fingerprint() -> Vec<String> {
    let mut scheduler = EventScheduler::new(
        TimeResolution::default(),
        SchedulerLimits {
            max_delta_cycles_per_tick: 4_096,
            max_events_per_tick: 200_000,
            max_reported_oscillating_entities: 8,
        },
    );
    let mut rng = Rng::new(0x9E37_79B9_7F4A_7C15);

    for index in 0..400usize {
        let tick = rng.below(40) + 1;
        let region = SchedulerRegion::ORDERED[(rng.below(4)) as usize];
        let instance = format!("inst{}", rng.below(11));
        let port = format!("p{}", rng.below(5));
        scheduler
            .schedule_at(
                tick,
                region,
                target(&instance, &port, index % 17, index % 3),
                digital((index % 3) as u8),
            )
            .expect("schedule");
    }

    let mut order = Vec::new();
    let mut fanout = Rng::new(0x1234_5678_9ABC_DEF0);
    while scheduler
        .run_time_slot(|event, ctx| {
            order.push(format!(
                "{}|{}|{}|{}|{}|{}|{}",
                event.tick,
                region_tag(event.region),
                event.sequence,
                event.target.instance,
                event.target.port_name,
                event.target.node_id,
                event.target.driver_index
            ));
            // Deterministic fan-out: some events spawn a follow-up, exercising
            // both the delta tier and the future tier during execution.
            let roll = fanout.below(10);
            if roll < 3 && order.len() < 3_000 {
                let delay = if roll == 0 { 0 } else { roll };
                let region = SchedulerRegion::ORDERED[(roll as usize) % 4];
                ctx.schedule_after(
                    delay,
                    region,
                    target(&event.target.instance, "spawned", event.target.node_id, 1),
                    digital(1),
                )
                .expect("queue the follow-up");
            }
        })
        .expect("slots settle")
        .is_some()
    {}

    order
}

#[test]
fn repeated_runs_produce_identical_orderings() {
    let first = ordering_fingerprint();
    let second = ordering_fingerprint();
    let third = ordering_fingerprint();

    assert!(
        first.len() > 400,
        "the fingerprint schedule must exercise fan-out, got {} events",
        first.len()
    );
    assert_eq!(first, second, "two runs disagreed on event ordering");
    assert_eq!(second, third, "three runs disagreed on event ordering");
}

/// Child half of the two-process determinism check.
///
/// Without the environment variable this test is inert, so a normal run pays
/// nothing for it. The parent re-invokes the test binary with the variable set
/// and `--exact` on this name, and reads the ordering back off stdout.
#[test]
fn scheduler_order_child_probe() {
    if env::var_os(CHILD_ENV).is_none() {
        return;
    }
    println!("{ORDER_MARKER}{}", ordering_fingerprint().join(";"));
}

#[test]
fn separate_processes_produce_identical_orderings() {
    let exe = env::current_exe().expect("test binary path");
    let output = Command::new(&exe)
        .args(["--exact", "scheduler_order_child_probe", "--nocapture"])
        .env(CHILD_ENV, "1")
        .output()
        .expect("re-invoke the test binary");

    assert!(
        output.status.success(),
        "child probe failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("child stdout is utf-8");
    let line = stdout
        .lines()
        .find_map(|line| line.strip_prefix(ORDER_MARKER))
        .expect("child printed no ordering");
    let child: Vec<String> = line.split(';').map(str::to_string).collect();

    let parent = ordering_fingerprint();
    assert_eq!(
        child.len(),
        parent.len(),
        "the two processes executed different event counts"
    );
    assert_eq!(
        child, parent,
        "the same schedule ordered differently in another process"
    );
}

//=============================================================================
// Scale
//=============================================================================

#[test]
fn several_thousand_events_drain_in_total_order() {
    const EVENTS: usize = 8_000;
    let mut scheduler = scheduler();
    let mut rng = Rng::new(0xDEAD_BEEF_CAFE_F00D);

    // Record what the documented total order says the answer is, independently
    // of the queue that will produce it.
    let mut expected: Vec<(u64, SchedulerRegion, u64)> = Vec::with_capacity(EVENTS);
    for index in 0..EVENTS {
        let tick = rng.below(900) + 1;
        let region = SchedulerRegion::ORDERED[(rng.below(4)) as usize];
        let sequence = scheduler
            .schedule_at(
                tick,
                region,
                target(&format!("inst{}", index % 64), "out", index % 128, 0),
                digital((index % 3) as u8),
            )
            .expect("schedule");
        expected.push((tick, region, sequence));
    }
    expected.sort();
    assert_eq!(scheduler.pending(), EVENTS);

    let mut observed = Vec::with_capacity(EVENTS);
    let mut slots = 0usize;
    let mut executed_total = 0u64;
    while let Some(report) = scheduler
        .run_time_slot(|event, _| observed.push((event.tick, event.region, event.sequence)))
        .expect("slots settle")
    {
        slots += 1;
        executed_total += report.events_executed;
    }

    assert_eq!(observed.len(), EVENTS);
    assert_eq!(executed_total, EVENTS as u64);
    assert_eq!(
        observed, expected,
        "the drained order must be exactly (tick, region, sequence)"
    );
    assert_eq!(scheduler.pending(), 0);
    assert!(
        slots > 100,
        "the schedule should span many ticks, got {slots} slots"
    );
    // Every tick with an event ran exactly once, and empty ticks were skipped
    // rather than stepped through.
    let distinct_ticks = {
        let mut ticks: Vec<u64> = expected.iter().map(|(tick, _, _)| *tick).collect();
        ticks.dedup();
        ticks.len()
    };
    assert_eq!(slots, distinct_ticks);
}

//=============================================================================
// What the next tick reports about a part-settled slot
//=============================================================================

#[test]
fn the_next_tick_dates_a_part_settled_slot_by_its_events_not_by_the_bound() {
    // Drive the slot the way a mixed interleave does: name a bound well past
    // the earliest pending tick, and stop the settle inside it. The events the
    // kernel opened into the slot are dated at tick 2; reporting the bound
    // would date them at 40, which as a breakpoint is 38 ns late.
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 0,
        ..SchedulerLimits::default()
    };
    let mut scheduler = EventScheduler::new(TimeResolution::new(-9).expect("1 ns"), limits);
    scheduler
        .schedule_at(
            2,
            SchedulerRegion::Active,
            target("a", "out", 1, 0),
            digital(1),
        )
        .expect("schedule");
    scheduler
        .schedule_at(
            2,
            SchedulerRegion::NonBlockingAssign,
            target("b", "out", 2, 0),
            digital(1),
        )
        .expect("schedule");

    let error = scheduler
        .run_due_events(40, |_, _| {})
        .expect_err("the delta-cycle ceiling stops the slot part-settled");
    assert!(matches!(error, SchedulerError::Oscillation(_)));

    assert_eq!(
        scheduler.next_tick(),
        Some(2),
        "an event still in the slot is dated where it was scheduled, not at the bound"
    );
}

#[test]
fn the_next_tick_is_unchanged_for_a_settled_slot() {
    // The XSPICE estate's reading: every predicate asks between runs, and a
    // slot that settled is empty in every region, so the answer comes from the
    // future tier exactly as it always did.
    let mut scheduler = scheduler();
    assert_eq!(scheduler.next_tick(), None);
    scheduler
        .schedule_at(
            4,
            SchedulerRegion::Active,
            target("a", "out", 1, 0),
            digital(1),
        )
        .expect("schedule");
    scheduler
        .schedule_at(
            9,
            SchedulerRegion::Active,
            target("b", "out", 2, 0),
            digital(0),
        )
        .expect("schedule");
    assert_eq!(scheduler.next_tick(), Some(4));

    scheduler.run_due_events(4, |_, _| {}).expect("settles");
    assert_eq!(
        scheduler.next_tick(),
        Some(9),
        "the settled slot is empty, so the future tier answers"
    );

    scheduler.run_due_events(20, |_, _| {}).expect("settles");
    assert_eq!(scheduler.next_tick(), None);
}

//=============================================================================
// The ordering specification, against a reference model
//=============================================================================

/// Regions in one slot, as the reference model indexes them.
const MODEL_REGIONS: usize = 4;

/// Events one executing event is allowed to schedule during a single drain.
///
/// A bound rather than a probability: an event that schedules into its own
/// open slot can do so again from the event that lands there, and an unbounded
/// generator would therefore write programs that never settle. The budget is
/// spent identically by the kernel and by the model, so what it bounds is the
/// program, not the comparison.
const MODEL_SPAWN_BUDGET: u32 = 24;

/// One event, as the reference model holds it.
#[derive(Clone, Debug, PartialEq)]
struct ModelEvent {
    tick: u64,
    region: SchedulerRegion,
    sequence: u64,
    target: EventTarget,
    value: EventValue,
}

/// One executed event, plus what the schedule it made returned.
///
/// The comparison the whole test rests on: two runs agree exactly when they
/// execute the same events in the same order *and* every schedule made from
/// inside a slot answered the same way.
#[derive(Clone, Debug, PartialEq)]
struct Executed {
    tick: u64,
    region: SchedulerRegion,
    sequence: u64,
    target: EventTarget,
    value: EventValue,
    outcome: Option<Result<u64, SchedulerError>>,
}

/// What an executing event does, drawn before either run so that the kernel
/// and the model make the same schedules in the same places.
#[derive(Clone, Copy, Debug)]
enum Spawn {
    /// Schedule nothing.
    Nothing,
    /// `SchedulerContext::schedule_at`, at an offset from the running tick.
    /// A negative offset is the refusal case, which must be refused in both.
    At {
        offset: i64,
        region: u64,
        driver: u64,
        value: u64,
    },
    /// `SchedulerContext::schedule_after`, whose zero delay is a delta event.
    After {
        delay: u64,
        region: u64,
        driver: u64,
        value: u64,
    },
}

/// The spawns of one randomized program, addressed by execution index.
struct Program {
    spawns: Vec<Spawn>,
}

impl Program {
    fn spawn(&self, index: usize) -> Spawn {
        self.spawns[index % self.spawns.len()]
    }
}

fn model_region(pick: u64) -> SchedulerRegion {
    SchedulerRegion::ORDERED[(pick % MODEL_REGIONS as u64) as usize]
}

fn region_index(region: SchedulerRegion) -> usize {
    match region {
        SchedulerRegion::Active => 0,
        SchedulerRegion::Inactive => 1,
        SchedulerRegion::NonBlockingAssign => 2,
        SchedulerRegion::Monitor => 3,
    }
}

/// One of a handful of drivers, so that supersession has something to cancel.
fn model_target(driver: u64) -> EventTarget {
    target("dut", "out", (driver % 6) as usize, 0)
}

fn model_value(pick: u64) -> EventValue {
    digital((pick % 3) as u8)
}

fn spawn_tick(current: u64, offset: i64) -> u64 {
    if offset < 0 {
        current.saturating_sub(offset.unsigned_abs())
    } else {
        current.saturating_add(offset as u64)
    }
}

/// The reference every executed event is checked against, for a program whose
/// events schedule *while the slot is running*.
///
/// It is the ordering specification written out — a slot that empties by
/// lowest sequence, a promotion that moves a whole region at once, a future
/// tier that opens one tick at a time, and supersession that cancels a
/// driver's own unexecuted events at or after a tick — over flat vectors and
/// linear scans. Deliberately not the kernel's data structures: a model that
/// shared them could only prove they agree with themselves.
///
/// A step model rather than the single sort
/// [`the_specification_orders_a_quiet_drain`] uses, because once an executing
/// event may schedule into the slot it is running in, the executed order is
/// no longer the global sort of the surviving events. Promotion is what
/// separates them: a slot that has already promoted `NonBlockingAssign` into
/// the active region and then schedules into `Inactive` runs the
/// nonblocking event first, while `(tick, region, sequence)` sorts `Inactive`
/// ahead of it. Both are the specification — §11 stratifies a slot into
/// *passes*, and a region reopened after its pass belongs to the next delta,
/// not to the one that already ran. The sort states that for one pass; this
/// model states it across passes.
struct ReferenceKernel {
    future: Vec<ModelEvent>,
    slot: [Vec<ModelEvent>; MODEL_REGIONS],
    next_sequence: u64,
    current_tick: u64,
    started: bool,
    delta_cycles: u32,
    events_executed: u64,
    /// The ceilings, modelled so that a program driven past them stops in the
    /// same place in both runs and leaves the same part-settled slot behind.
    limits: SchedulerLimits,
}

/// A run stopped by one of the ceilings. The diagnostic itself is pinned by
/// the oscillation tests above; what matters here is *where* the run stopped
/// and what it left in the queues.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ModelOscillation;

impl ReferenceKernel {
    fn new(limits: SchedulerLimits) -> Self {
        Self {
            future: Vec::new(),
            slot: Default::default(),
            next_sequence: 0,
            current_tick: 0,
            started: false,
            delta_cycles: 0,
            events_executed: 0,
            limits,
        }
    }

    fn insert(
        &mut self,
        open_slot: Option<u64>,
        tick: u64,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        let event = ModelEvent {
            tick,
            region,
            sequence,
            target,
            value,
        };
        if open_slot == Some(tick) {
            self.slot[region_index(region)].push(event);
        } else {
            self.future.push(event);
        }
        sequence
    }

    fn schedule_at(
        &mut self,
        tick: u64,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> Result<u64, SchedulerError> {
        let horizon = if self.started {
            self.current_tick.saturating_add(1)
        } else {
            0
        };
        if tick < horizon {
            return Err(SchedulerError::ScheduleInThePast {
                current_tick: self.current_tick,
                requested_tick: tick,
            });
        }
        Ok(self.insert(None, tick, region, target, value))
    }

    /// Supersession takes no horizon, so this is also how a program back-dates
    /// an event below a bound that has already been settled.
    fn schedule_superseding_at(
        &mut self,
        tick: u64,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> usize {
        let mut cancelled = 0;
        let doomed = |event: &ModelEvent| event.target == target && event.tick >= tick;
        self.future.retain(|event| {
            if doomed(event) {
                cancelled += 1;
                false
            } else {
                true
            }
        });
        for queue in self.slot.iter_mut() {
            queue.retain(|event| {
                if doomed(event) {
                    cancelled += 1;
                    false
                } else {
                    true
                }
            });
        }
        self.insert(None, tick, region, target, value);
        cancelled
    }

    fn pop_active(&mut self) -> Option<ModelEvent> {
        let queue = &mut self.slot[0];
        let position = queue
            .iter()
            .enumerate()
            .min_by_key(|(_, event)| event.sequence)
            .map(|(position, _)| position)?;
        Some(queue.remove(position))
    }

    fn promote(&mut self) -> bool {
        for index in 1..MODEL_REGIONS {
            if self.slot[index].is_empty() {
                continue;
            }
            let promoted = std::mem::take(&mut self.slot[index]);
            self.slot[0].extend(promoted);
            return true;
        }
        false
    }

    fn open_next_due_tick(&mut self, bound: u64) -> bool {
        let Some(tick) = self.future.iter().map(|event| event.tick).min() else {
            return false;
        };
        if tick > bound {
            return false;
        }
        let mut due = Vec::new();
        self.future.retain(|event| {
            if event.tick == tick {
                due.push(event.clone());
                false
            } else {
                true
            }
        });
        for event in due {
            self.slot[region_index(event.region)].push(event);
        }
        true
    }

    fn next_tick(&self) -> Option<u64> {
        let slot = self
            .slot
            .iter()
            .flat_map(|queue| queue.iter())
            .map(|event| event.tick)
            .min();
        let future = self.future.iter().map(|event| event.tick).min();
        match (slot, future) {
            (Some(slot), Some(future)) => Some(slot.min(future)),
            (slot, future) => slot.or(future),
        }
    }

    fn pending(&self) -> usize {
        self.future.len() + self.slot.iter().map(Vec::len).sum::<usize>()
    }

    fn open_due_slot(&mut self, bound: u64) {
        if self.started && self.current_tick == bound {
            return;
        }
        self.current_tick = bound;
        self.started = true;
        self.delta_cycles = 0;
        self.events_executed = 0;
    }

    fn note_delta_cycle(&mut self, bound: u64) -> Result<(), ModelOscillation> {
        self.open_due_slot(bound);
        self.delta_cycles += 1;
        if self.delta_cycles > self.limits.max_delta_cycles_per_tick {
            return Err(ModelOscillation);
        }
        Ok(())
    }

    fn spawn(
        &mut self,
        spawn: Spawn,
        budget: &Cell<u32>,
        context_tick: u64,
    ) -> Option<Result<u64, SchedulerError>> {
        let (tick, region, driver, value) = match spawn {
            Spawn::Nothing => return None,
            Spawn::At {
                offset,
                region,
                driver,
                value,
            } => (spawn_tick(context_tick, offset), region, driver, value),
            Spawn::After {
                delay,
                region,
                driver,
                value,
            } => (context_tick.saturating_add(delay), region, driver, value),
        };
        if budget.get() == 0 {
            return None;
        }
        budget.set(budget.get() - 1);
        if tick < context_tick {
            return Some(Err(SchedulerError::ScheduleInThePast {
                current_tick: context_tick,
                requested_tick: tick,
            }));
        }
        Some(Ok(self.insert(
            Some(context_tick),
            tick,
            model_region(region),
            model_target(driver),
            model_value(value),
        )))
    }

    fn run_due_events(
        &mut self,
        bound: u64,
        program: &Program,
        spawn_base: usize,
        executed: &mut Vec<Executed>,
    ) -> Result<TimeSlotReport, ModelOscillation> {
        self.open_due_slot(bound);
        let budget = Cell::new(MODEL_SPAWN_BUDGET);
        let mut ran = 0usize;
        loop {
            let Some(event) = self.pop_active() else {
                if self.promote() {
                    self.delta_cycles += 1;
                    if self.delta_cycles > self.limits.max_delta_cycles_per_tick {
                        return Err(ModelOscillation);
                    }
                    continue;
                }
                if self.open_next_due_tick(bound) {
                    continue;
                }
                break;
            };
            // Counted before it runs, and the run abandons it where the count
            // trips: the event has already left the active queue, so a
            // ceiling reached here drops it. `pending()` has to agree.
            self.events_executed += 1;
            if self.events_executed > self.limits.max_events_per_tick {
                return Err(ModelOscillation);
            }
            let spawn = program.spawn(spawn_base + ran);
            ran += 1;
            let outcome = self.spawn(spawn, &budget, bound);
            executed.push(Executed {
                tick: event.tick,
                region: event.region,
                sequence: event.sequence,
                target: event.target,
                value: event.value,
                outcome,
            });
        }
        Ok(TimeSlotReport {
            tick: bound,
            delta_cycles: self.delta_cycles,
            events_executed: self.events_executed,
        })
    }

    fn run_time_slot(
        &mut self,
        program: &Program,
        spawn_base: usize,
        executed: &mut Vec<Executed>,
    ) -> Result<Option<TimeSlotReport>, ModelOscillation> {
        let Some(tick) = self.next_tick() else {
            return Ok(None);
        };
        self.current_tick = tick;
        self.started = true;
        self.open_next_due_tick(tick);
        self.delta_cycles = 0;
        self.events_executed = 0;

        let mut delta_cycles = 0u32;
        let mut events_executed = 0u64;
        let budget = Cell::new(MODEL_SPAWN_BUDGET);
        let mut ran = 0usize;
        loop {
            let Some(event) = self.pop_active() else {
                if !self.promote() {
                    break;
                }
                delta_cycles += 1;
                if delta_cycles > self.limits.max_delta_cycles_per_tick {
                    return Err(ModelOscillation);
                }
                continue;
            };
            events_executed += 1;
            if events_executed > self.limits.max_events_per_tick {
                return Err(ModelOscillation);
            }
            let spawn = program.spawn(spawn_base + ran);
            ran += 1;
            let outcome = self.spawn(spawn, &budget, tick);
            executed.push(Executed {
                tick: event.tick,
                region: event.region,
                sequence: event.sequence,
                target: event.target,
                value: event.value,
                outcome,
            });
        }
        Ok(Some(TimeSlotReport {
            tick,
            delta_cycles,
            events_executed,
        }))
    }
}

fn kernel_spawn(
    spawn: Spawn,
    budget: &Cell<u32>,
    context: &mut SchedulerContext<'_>,
) -> Option<Result<u64, SchedulerError>> {
    match spawn {
        Spawn::Nothing => None,
        Spawn::At {
            offset,
            region,
            driver,
            value,
        } => {
            if budget.get() == 0 {
                return None;
            }
            budget.set(budget.get() - 1);
            let tick = spawn_tick(context.current_tick(), offset);
            Some(context.schedule_at(
                tick,
                model_region(region),
                model_target(driver),
                model_value(value),
            ))
        }
        Spawn::After {
            delay,
            region,
            driver,
            value,
        } => {
            if budget.get() == 0 {
                return None;
            }
            budget.set(budget.get() - 1);
            Some(context.schedule_after(
                delay,
                model_region(region),
                model_target(driver),
                model_value(value),
            ))
        }
    }
}

fn draw_spawn(rng: &mut Rng) -> Spawn {
    let region = rng.next();
    let driver = rng.next();
    let value = rng.next();
    match rng.below(10) {
        0..=3 => Spawn::Nothing,
        4..=6 => Spawn::At {
            // Zero is the delta event that joins the open slot; a negative
            // offset is the one schedule a running slot must refuse.
            offset: rng.below(7) as i64 - 1,
            region,
            driver,
            value,
        },
        _ => Spawn::After {
            delay: rng.below(4),
            region,
            driver,
            value,
        },
    }
}

/// Drive the kernel and the model through one randomized program and check
/// that they agree on everything observable at every step.
///
/// `limits` is the caller's, because the ceilings decide whether the slot is
/// ever observed *part-settled*: with them set past anything a program
/// reaches, every call returns with all four regions empty, and the readers
/// that scan a non-empty slot — `pending`, `next_tick`, and the supersession
/// that has to reach into an open slot rather than the future tier — are
/// never asked a hard question. Tight ceilings stop a drain mid-slot and
/// leave exactly that state behind for the operations that follow.
fn compare_against_the_reference(seed: u64, operations: usize, limits: SchedulerLimits) {
    let mut rng = Rng::new(seed);
    let mut kernel = EventScheduler::new(TimeResolution::default(), limits);
    let mut model = ReferenceKernel::new(limits);
    let program = Program {
        spawns: (0..512).map(|_| draw_spawn(&mut rng)).collect(),
    };

    // Coverage counters. A generated program that happened never to supersede
    // anything, never to leave a part-settled slot, or never to execute an
    // event would pass every assertion below without exercising a thing, so
    // each is asserted non-trivial at the end.
    let mut oscillated = 0usize;
    let mut cancelled_total = 0usize;
    let mut executed_total = 0usize;
    let mut part_settled_steps = 0usize;
    let mut superseded_into_open_slot = 0usize;

    for step in 0..operations {
        let base = model.current_tick;
        let slot_before = model.slot.iter().map(Vec::len).sum::<usize>();
        // Spread over the running tick so that every case is generated: ticks
        // in the past (refused by `schedule_at`, accepted by supersession),
        // the running tick itself, and ticks ahead of it.
        let tick = (base + rng.below(6)).saturating_sub(rng.below(3));
        let region = model_region(rng.next());
        let driver = rng.next();
        let value = model_value(rng.next());
        let spawn_base = (rng.next() % 512) as usize;

        match rng.below(100) {
            0..=33 => {
                let kernel_answer = kernel.schedule_at(tick, region, model_target(driver), value);
                let model_answer = model.schedule_at(tick, region, model_target(driver), value);
                assert_eq!(
                    kernel_answer, model_answer,
                    "seed {seed} step {step}: schedule_at({tick})"
                );
            }
            34..=53 => {
                let kernel_answer =
                    kernel.schedule_superseding_at(tick, region, model_target(driver), value);
                let model_answer =
                    model.schedule_superseding_at(tick, region, model_target(driver), value);
                assert_eq!(
                    kernel_answer, model_answer,
                    "seed {seed} step {step}: schedule_superseding_at({tick}) cancelled"
                );
                cancelled_total += model_answer;
                // The case a tombstone has to survive: the cancelled event was
                // sitting in the open slot, not in the future tier, and may
                // already have been promoted out of the region its own
                // `region` field names.
                if model.slot.iter().map(Vec::len).sum::<usize>() < slot_before {
                    superseded_into_open_slot += 1;
                }
            }
            54..=83 => {
                let mut kernel_executed = Vec::new();
                let budget = Cell::new(MODEL_SPAWN_BUDGET);
                let mut ran = 0usize;
                let kernel_report = kernel.run_due_events(tick, |event, context| {
                    let spawn = program.spawn(spawn_base + ran);
                    ran += 1;
                    let outcome = kernel_spawn(spawn, &budget, context);
                    kernel_executed.push(Executed {
                        tick: event.tick,
                        region: event.region,
                        sequence: event.sequence,
                        target: event.target,
                        value: event.value,
                        outcome,
                    });
                });
                let mut model_executed = Vec::new();
                let model_report =
                    model.run_due_events(tick, &program, spawn_base, &mut model_executed);
                assert_eq!(
                    kernel_executed, model_executed,
                    "seed {seed} step {step}: run_due_events({tick}) order"
                );
                executed_total += model_executed.len();
                assert_eq!(
                    kernel_report.map_err(|_| ModelOscillation),
                    model_report,
                    "seed {seed} step {step}: run_due_events({tick}) report"
                );
                if model_report.is_err() {
                    oscillated += 1;
                }
            }
            84..=93 => {
                let mut kernel_executed = Vec::new();
                let budget = Cell::new(MODEL_SPAWN_BUDGET);
                let mut ran = 0usize;
                let kernel_report = kernel.run_time_slot(|event, context| {
                    let spawn = program.spawn(spawn_base + ran);
                    ran += 1;
                    let outcome = kernel_spawn(spawn, &budget, context);
                    kernel_executed.push(Executed {
                        tick: event.tick,
                        region: event.region,
                        sequence: event.sequence,
                        target: event.target,
                        value: event.value,
                        outcome,
                    });
                });
                let mut model_executed = Vec::new();
                let model_report = model.run_time_slot(&program, spawn_base, &mut model_executed);
                assert_eq!(
                    kernel_executed, model_executed,
                    "seed {seed} step {step}: run_time_slot order"
                );
                executed_total += model_executed.len();
                assert_eq!(
                    kernel_report.map_err(|_| ModelOscillation),
                    model_report,
                    "seed {seed} step {step}: run_time_slot report"
                );
                if model_report.is_err() {
                    oscillated += 1;
                }
            }
            _ => {
                let kernel_answer = kernel.note_delta_cycle(tick);
                let model_answer = model.note_delta_cycle(tick);
                assert_eq!(
                    kernel_answer.map_err(|_| ModelOscillation),
                    model_answer,
                    "seed {seed} step {step}: note_delta_cycle({tick})"
                );
            }
        }

        assert_eq!(
            kernel.pending(),
            model.pending(),
            "seed {seed} step {step}: pending"
        );
        assert_eq!(
            kernel.next_tick(),
            model.next_tick(),
            "seed {seed} step {step}: next_tick"
        );
        assert_eq!(
            kernel.current_tick(),
            model.current_tick,
            "seed {seed} step {step}: current_tick"
        );
        if model.slot.iter().any(|queue| !queue.is_empty()) {
            // `pending` and `next_tick` were just asked with a non-empty slot,
            // which is the reading a tick-keyed future tier cannot answer on
            // its own and a tombstoned slot has to answer exactly.
            part_settled_steps += 1;
        }
    }

    // A program that never drained anything, never cancelled anything, or
    // never left a part-settled slot would pass everything above vacuously.
    assert!(
        model.next_sequence > operations as u64 / 4,
        "seed {seed}: the program scheduled almost nothing ({} events)",
        model.next_sequence
    );
    assert!(
        executed_total > operations / 4,
        "seed {seed}: the program executed almost nothing ({executed_total} events)"
    );
    assert!(
        cancelled_total > operations / 50,
        "seed {seed}: supersession cancelled almost nothing ({cancelled_total} events)"
    );
    assert!(
        superseded_into_open_slot > 0 || limits.max_events_per_tick == u64::MAX,
        "seed {seed}: no supersession ever reached into an open slot"
    );
    if limits.max_events_per_tick == u64::MAX {
        assert_eq!(
            oscillated, 0,
            "seed {seed}: the generous ceilings were meant to be out of reach"
        );
        assert_eq!(
            part_settled_steps, 0,
            "seed {seed}: with no ceiling a returned slot is settled in every region"
        );
    } else {
        assert!(
            oscillated > operations / 50,
            "seed {seed}: the tight ceilings were never reached ({oscillated} times)"
        );
        assert!(
            part_settled_steps > operations / 50,
            "seed {seed}: the slot was almost never left part-settled ({part_settled_steps} steps)"
        );
    }
}

/// Every event the kernel executes, in the order `(tick, region, sequence)`
/// fixes, under thousands of random schedules, supersessions, promotions and
/// slot openings — checked against a model written from the specification.
///
/// This is the oracle the queue structures are replaced under: the tiers may
/// be anything as long as this holds, and nothing else pins the order of two
/// events that no hand-written case happens to place next to each other.
#[test]
fn a_random_program_runs_in_the_order_the_specification_fixes() {
    // Past anything a generated program reaches: this test is about ordering,
    // and the ceilings are the next one's subject.
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: u32::MAX,
        max_events_per_tick: u64::MAX,
        max_reported_oscillating_entities: 16,
    };
    for seed in [
        0x2545_f491_4f6c_dd1du64,
        0x9e37_79b9_7f4a_7c15,
        0x0123_4567_89ab_cdef,
        0xdead_beef_cafe_f00d,
        0x0f1e_2d3c_4b5a_6978,
    ] {
        compare_against_the_reference(seed, 12_000, limits);
    }
}

/// The same comparison under ceilings a randomized program actually reaches,
/// so that every operation after one is asked of a slot that is *not* settled.
///
/// This is the state the queue structures are hardest to get right in. A drain
/// abandoned mid-slot leaves events in all four regions, some of them promoted
/// out of the region their own key names; the next operation may supersede one
/// of those, which is a removal from the middle of a running queue rather than
/// from the future tier, and `pending` and `next_tick` are then asked to
/// answer over the remains. Nothing else in this file reaches it: every other
/// test's slot is empty by the time it looks.
#[test]
fn a_random_program_agrees_with_the_specification_over_a_part_settled_slot() {
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 3,
        max_events_per_tick: 12,
        max_reported_oscillating_entities: 16,
    };
    for seed in [
        0x2545_f491_4f6c_dd1du64,
        0x9e37_79b9_7f4a_7c15,
        0x0123_4567_89ab_cdef,
        0xdead_beef_cafe_f00d,
    ] {
        compare_against_the_reference(seed, 10_000, limits);
    }
}

//=============================================================================
// The ordering specification, applied directly as a sort
//=============================================================================

/// The specification's own words as the reference: the events that survive,
/// sorted by `(tick, region, sequence)`.
///
/// [`a_random_program_runs_in_the_order_the_specification_fixes`] compares
/// against a step model, which it has to: an event that schedules into the
/// slot it is running in can reopen a region whose pass has already run, and
/// such an event belongs to the next delta cycle rather than to the position
/// its key sorts into. That is the specification too, but it is not a sort.
///
/// So this test removes the one thing that separates them. Nothing is
/// scheduled from inside a slot; everything is scheduled from outside, where
/// a schedule is only ever filed and never routed into a running pass. Then
/// the order the kernel executes a drain in must be — literally, with no
/// model in between — the surviving events sorted by the key the module
/// documentation gives, with each driver's superseded events removed.
fn the_specification_orders_a_quiet_drain(seed: u64, operations: usize) {
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: u32::MAX,
        max_events_per_tick: u64::MAX,
        max_reported_oscillating_entities: 16,
    };
    let mut rng = Rng::new(seed);
    let mut kernel = EventScheduler::new(TimeResolution::default(), limits);

    // Every event scheduled and not yet executed or cancelled. A flat list:
    // the reference is a sort of it, so it holds no order of its own.
    let mut live: Vec<ModelEvent> = Vec::new();
    let mut next_sequence = 0u64;
    let mut current_tick = 0u64;
    let mut started = false;
    let mut executed_total = 0usize;
    let mut cancelled_total = 0usize;
    let mut multi_tick_drains = 0usize;
    let mut back_dated = 0usize;

    for step in 0..operations {
        let tick = (current_tick + rng.below(6)).saturating_sub(rng.below(3));
        let region = model_region(rng.next());
        let driver = rng.next();
        let value = model_value(rng.next());
        let target = model_target(driver);

        match rng.below(100) {
            0..=29 => {
                let answer = kernel.schedule_at(tick, region, target.clone(), value);
                let horizon = if started {
                    current_tick.saturating_add(1)
                } else {
                    0
                };
                if tick < horizon {
                    assert_eq!(
                        answer,
                        Err(SchedulerError::ScheduleInThePast {
                            current_tick,
                            requested_tick: tick,
                        }),
                        "seed {seed} step {step}: schedule_at({tick}) below the horizon"
                    );
                } else {
                    assert_eq!(
                        answer,
                        Ok(next_sequence),
                        "seed {seed} step {step}: schedule_at({tick}) sequence"
                    );
                    live.push(ModelEvent {
                        tick,
                        region,
                        sequence: next_sequence,
                        target,
                        value,
                    });
                    next_sequence += 1;
                }
            }
            30..=49 => {
                // The only door that back-dates: supersession takes no
                // horizon, so this is what files an event below a bound the
                // scheduler has already opened, for the next drain to pick up.
                if started && tick <= current_tick {
                    back_dated += 1;
                }
                let answer = kernel.schedule_superseding_at(tick, region, target.clone(), value);
                let mut cancelled = 0usize;
                live.retain(|event| {
                    if event.target == target && event.tick >= tick {
                        cancelled += 1;
                        false
                    } else {
                        true
                    }
                });
                assert_eq!(
                    answer, cancelled,
                    "seed {seed} step {step}: schedule_superseding_at({tick}) cancelled"
                );
                cancelled_total += cancelled;
                live.push(ModelEvent {
                    tick,
                    region,
                    sequence: next_sequence,
                    target,
                    value,
                });
                next_sequence += 1;
            }
            50..=84 => {
                // Everything dated at or before the bound is due, however many
                // distinct ticks that spans.
                let mut expected: Vec<ModelEvent> = live
                    .iter()
                    .filter(|event| event.tick <= tick)
                    .cloned()
                    .collect();
                expected.sort_by_key(|event| (event.tick, event.region, event.sequence));
                if expected
                    .first()
                    .zip(expected.last())
                    .is_some_and(|(first, last)| first.tick != last.tick)
                {
                    multi_tick_drains += 1;
                }
                let mut ran: Vec<ModelEvent> = Vec::new();
                kernel
                    .run_due_events(tick, |event, _| {
                        ran.push(ModelEvent {
                            tick: event.tick,
                            region: event.region,
                            sequence: event.sequence,
                            target: event.target,
                            value: event.value,
                        });
                    })
                    .expect("the ceilings are out of reach");
                assert_eq!(
                    ran, expected,
                    "seed {seed} step {step}: run_due_events({tick}) is the sorted order"
                );
                executed_total += ran.len();
                live.retain(|event| event.tick > tick);
                current_tick = tick;
                started = true;
            }
            _ => {
                // One tick, chosen by the kernel: the earliest that has an
                // event. The reference is the same sort restricted to it.
                let due = live.iter().map(|event| event.tick).min();
                let mut expected: Vec<ModelEvent> = match due {
                    Some(due) => live
                        .iter()
                        .filter(|event| event.tick == due)
                        .cloned()
                        .collect(),
                    None => Vec::new(),
                };
                expected.sort_by_key(|event| (event.tick, event.region, event.sequence));
                let mut ran: Vec<ModelEvent> = Vec::new();
                let report = kernel
                    .run_time_slot(|event, _| {
                        ran.push(ModelEvent {
                            tick: event.tick,
                            region: event.region,
                            sequence: event.sequence,
                            target: event.target,
                            value: event.value,
                        });
                    })
                    .expect("the ceilings are out of reach");
                assert_eq!(
                    ran, expected,
                    "seed {seed} step {step}: run_time_slot is the sorted order"
                );
                assert_eq!(
                    report.map(|report| report.tick),
                    due,
                    "seed {seed} step {step}: run_time_slot ran the earliest tick"
                );
                executed_total += ran.len();
                if let Some(due) = due {
                    live.retain(|event| event.tick != due);
                    current_tick = due;
                    started = true;
                }
            }
        }

        assert_eq!(
            kernel.pending(),
            live.len(),
            "seed {seed} step {step}: pending"
        );
        assert_eq!(
            kernel.next_tick(),
            live.iter().map(|event| event.tick).min(),
            "seed {seed} step {step}: next_tick"
        );
        assert_eq!(
            kernel.current_tick(),
            current_tick,
            "seed {seed} step {step}: current_tick"
        );
    }

    assert!(
        executed_total > operations / 4,
        "seed {seed}: the program executed almost nothing ({executed_total} events)"
    );
    assert!(
        cancelled_total > operations / 50,
        "seed {seed}: supersession cancelled almost nothing ({cancelled_total} events)"
    );
    assert!(
        multi_tick_drains > operations / 50,
        "seed {seed}: no drain ever spanned two ticks ({multi_tick_drains})"
    );
    assert!(
        back_dated > operations / 50,
        "seed {seed}: nothing was ever back-dated below an opened bound ({back_dated})"
    );
}

/// The order of a drain, against the sort the module documentation names, over
/// thousands of random schedules and supersessions.
#[test]
fn a_quiet_drain_is_exactly_the_sorted_order() {
    for seed in [
        0x2545_f491_4f6c_dd1du64,
        0x9e37_79b9_7f4a_7c15,
        0x0123_4567_89ab_cdef,
        0xdead_beef_cafe_f00d,
        0x0f1e_2d3c_4b5a_6978,
    ] {
        the_specification_orders_a_quiet_drain(seed, 12_000);
    }
}
