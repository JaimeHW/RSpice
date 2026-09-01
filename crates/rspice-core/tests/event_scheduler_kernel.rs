//! Conformance tests for the discrete-event scheduler kernel.
//!
//! The XSPICE path runs on the kernel, but it exercises the kernel through one
//! shape: everything in the active region, driven by an outer settle loop.
//! These tests are what holds the rest — the stratified regions, the total
//! order, driver supersession, and the two properties everything above the
//! kernel rests on: the same schedule produces the same ordering on every run
//! and in every process, and that ordering is the one IEEE 1364-2005
//! specifies.

use std::collections::BTreeMap;
use std::env;
use std::process::Command;

use rspice_core::xspice::event_scheduler::{
    EventScheduler, EventTarget, OscillationCause, SchedulerError, SchedulerLimits,
    SchedulerRegion, TimeResolution,
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
// Flooring an analog time onto the tick grid
//=============================================================================

#[test]
fn flooring_maps_an_off_grid_analog_time_to_the_tick_at_or_before_it() {
    let resolution = TimeResolution::new(-9).expect("1 ns");

    // The case the round-to-nearest conversion gets wrong for a mixed
    // interleave: a time three quarters of the way through a tick belongs to
    // the tick it is inside, not to the one it is closest to. Rounding here
    // would run the digital world a quarter of a nanosecond past an instant
    // the integrator has accepted.
    assert_eq!(resolution.seconds_to_ticks(2.75e-9), Ok(3));
    assert_eq!(resolution.seconds_to_floor_ticks(2.75e-9), Ok(2));
    assert_eq!(resolution.seconds_to_floor_ticks(2.25e-9), Ok(2));
    assert_eq!(resolution.seconds_to_floor_ticks(0.0), Ok(0));
    assert_eq!(resolution.seconds_to_floor_ticks(0.999e-9), Ok(0));

    // Monotone: a non-decreasing sequence of analog times gives a
    // non-decreasing sequence of ticks, which is what lets `advance_to` be
    // driven straight from accepted timepoints.
    let mut previous = 0u64;
    let mut seconds = 0.0f64;
    while seconds < 5.0e-9 {
        let tick = resolution.seconds_to_floor_ticks(seconds).expect("in range");
        assert!(
            tick >= previous,
            "flooring must be monotone: {seconds:e} s gave {tick} after {previous}"
        );
        previous = tick;
        seconds += 3.7e-11;
    }
}

#[test]
fn flooring_a_tick_boundary_returns_that_tick_and_not_the_one_before() {
    // The error a bare division would make: an event time handed back as a
    // breakpoint, floored, must be delivered at its own tick. One ulp low and
    // the digital slot runs a whole tick late.
    for exponent in [-9i8, -12, -15] {
        let resolution = TimeResolution::new(exponent).expect("declared precision");
        for tick in [0u64, 1, 2, 3, 7, 999, 1_000, 1_001, 123_456_789] {
            let seconds = resolution.ticks_to_seconds(tick).expect("in range");
            assert_eq!(
                resolution.seconds_to_floor_ticks(seconds),
                Ok(tick),
                "exponent {exponent} tick {tick} round trip"
            );
        }
    }
}

#[test]
fn flooring_refuses_what_rounding_refuses() {
    let resolution = TimeResolution::new(-9).expect("1 ns");
    assert!(resolution.seconds_to_floor_ticks(-1.0e-9).is_err());
    assert!(resolution.seconds_to_floor_ticks(f64::NAN).is_err());
    assert!(resolution.seconds_to_floor_ticks(f64::INFINITY).is_err());
    assert!(
        resolution
            .seconds_to_floor_ticks(TimeResolution::MAX_EXACT_TICKS as f64 * 1.0e-9 * 2.0)
            .is_err()
    );
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
