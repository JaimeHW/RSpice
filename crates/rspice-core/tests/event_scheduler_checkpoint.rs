//! Persisted event-kernel continuation, before the enclosing mixed-state image.
use rspice_core::xspice::EventValue;
use rspice_core::xspice::event_scheduler::{
    EventScheduler, EventTarget, Instant, SchedulerCheckpoint, SchedulerCheckpointLimits,
    SchedulerError, SchedulerLimits, SchedulerRegion,
};
use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

fn at(seconds: f64) -> Instant {
    Instant::from_seconds(seconds).unwrap()
}
fn target(name: &str) -> EventTarget {
    EventTarget {
        node_id: 3,
        port_name: "out".into(),
        driver_index: 0,
        instance: name.into(),
    }
}
fn image(kernel: &EventScheduler) -> SchedulerCheckpoint {
    kernel
        .checkpoint(SchedulerCheckpointLimits::default())
        .unwrap()
}
fn restore(kernel: &mut EventScheduler, image: &SchedulerCheckpoint) {
    kernel
        .restore_checkpoint(image, SchedulerCheckpointLimits::default())
        .unwrap();
}
fn value_bits(value: EventValue) -> String {
    match value {
        EventValue::Digital(value) => format!("{:?}:{:?}", value.state, value.strength),
        EventValue::Real(value) => format!("{:016x}", value.to_bits()),
    }
}

#[test]
fn event_checkpoint_roundtrip_preserves_order_values_and_cancellation() {
    let mut original = EventScheduler::new(SchedulerLimits::default());
    original
        .schedule_at(
            at(0.0),
            SchedulerRegion::Active,
            target("init"),
            EventValue::Real(7.0),
        )
        .unwrap();
    original.run_due_events(at(1e-9), |_, _| {}).unwrap();
    // This back-dated event deliberately precedes the reporting horizon. The
    // due-slot API permits it; restore must not move it to the horizon.
    original.schedule_superseding_at(
        at(0.73e-9),
        SchedulerRegion::Active,
        target("adc"),
        EventValue::Real(-0.0),
    );
    for (region, name, value) in [
        (
            SchedulerRegion::Monitor,
            "monitor",
            EventValue::Real(f64::from_bits(0x7ff8_0000_0000_0123)),
        ),
        (
            SchedulerRegion::NonBlockingAssign,
            "nba",
            EventValue::Digital(DigitalValue::high_z()),
        ),
        (
            SchedulerRegion::Inactive,
            "inactive",
            EventValue::Real(f64::INFINITY),
        ),
        (
            SchedulerRegion::Active,
            "active",
            EventValue::Digital(DigitalValue::new(
                DigitalState::OneR,
                DigitalStrength::Strong,
            )),
        ),
    ] {
        original
            .schedule_at(at(2.13e-9), region, target(name), value)
            .unwrap();
    }
    original.schedule_superseding_at(
        at(8e-9),
        SchedulerRegion::Active,
        target("dac"),
        EventValue::Real(1.0),
    );
    assert_eq!(
        original.schedule_superseding_at(
            at(6e-9),
            SchedulerRegion::Active,
            target("dac"),
            EventValue::Real(2.0)
        ),
        1
    );
    let captured = image(&original);
    let text = serde_json::to_string(&captured).unwrap();
    assert_eq!(text, serde_json::to_string(&image(&original)).unwrap());
    let decoded: SchedulerCheckpoint = serde_json::from_str(&text).unwrap();
    let mut resumed = EventScheduler::new(SchedulerLimits::default());
    restore(&mut resumed, &decoded);
    assert_eq!(original.current_instant(), resumed.current_instant());
    assert_eq!(original.next_instant(), resumed.next_instant());
    assert_eq!(original.pending(), resumed.pending());

    let mut outputs = Vec::new();
    for kernel in [&mut original, &mut resumed] {
        let next = kernel
            .schedule_at(
                at(2.13e-9),
                SchedulerRegion::Active,
                target("new"),
                EventValue::Real(4.0),
            )
            .unwrap();
        let cancelled = kernel.schedule_superseding_at(
            at(4e-9),
            SchedulerRegion::Active,
            target("dac"),
            EventValue::Real(3.0),
        );
        let mut events = Vec::new();
        kernel
            .run_due_events(at(10e-9), |event, _| {
                events.push((
                    event.at.seconds().to_bits(),
                    event.region,
                    event.sequence,
                    event.target.instance,
                    value_bits(event.value),
                ))
            })
            .unwrap();
        outputs.push((next, cancelled, events));
        assert_eq!(kernel.pending(), 0);
    }
    assert_eq!(outputs[0], outputs[1]);
    assert_eq!(outputs[0].1, 1);
    let events = &outputs[0].2;
    assert_eq!(
        events
            .iter()
            .map(|event| event.3.as_str())
            .collect::<Vec<_>>(),
        ["adc", "active", "new", "inactive", "nba", "monitor", "dac"]
    );
    assert_eq!(events[0].0, 0.73e-9_f64.to_bits());
    assert_eq!(events[0].4, "8000000000000000");
    assert_eq!(events[1].4, "OneR:Strong");
    assert_eq!(events[5].4, "7ff8000000000123");
}

#[test]
fn event_checkpoint_keeps_slot_budgets_and_rejects_failed_slots() {
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 2,
        max_events_per_tick: 3,
        ..Default::default()
    };
    let mut original = EventScheduler::new(limits);
    original
        .schedule_at(
            at(0.0),
            SchedulerRegion::Active,
            target("first"),
            EventValue::Real(1.0),
        )
        .unwrap();
    original.run_due_events(at(0.0), |_, _| {}).unwrap();
    original.note_delta_cycle(at(0.0)).unwrap();
    let mut resumed = EventScheduler::new(limits);
    restore(&mut resumed, &image(&original));
    let mut errors = Vec::new();
    for kernel in [&mut original, &mut resumed] {
        kernel.note_delta_cycle(at(0.0)).unwrap();
        errors.push(kernel.note_delta_cycle(at(0.0)).unwrap_err());
        assert!(
            kernel
                .checkpoint(SchedulerCheckpointLimits::default())
                .is_err()
        );
    }
    assert_eq!(errors[0], errors[1]);
    let mut failed = EventScheduler::new(SchedulerLimits {
        max_events_per_tick: 0,
        ..Default::default()
    });
    failed
        .schedule_at(
            at(0.0),
            SchedulerRegion::Active,
            target("failed"),
            EventValue::Real(1.0),
        )
        .unwrap();
    assert!(failed.run_time_slot(|_, _| {}).is_err());
    assert_eq!(failed.pending(), 0); // Even a failure that emptied the queue is not a safe point.
    assert!(
        failed
            .checkpoint(SchedulerCheckpointLimits::default())
            .is_err()
    );
}

#[test]
fn event_checkpoint_rejects_corrupt_or_incompatible_images_atomically() {
    let mut kernel = EventScheduler::new(SchedulerLimits::default());
    kernel
        .schedule_at(
            at(3e-9),
            SchedulerRegion::Active,
            target("a"),
            EventValue::Real(1.0),
        )
        .unwrap();
    let original = image(&kernel);
    let good = serde_json::to_value(&original).unwrap();
    let mut invalid = Vec::new();
    for (pointer, value) in [
        ("/version", serde_json::json!(999)),
        ("/current_bits", serde_json::json!(f64::INFINITY.to_bits())),
        ("/current_bits", serde_json::json!((-0.0_f64).to_bits())),
        ("/events/0/at_bits", serde_json::json!(f64::NAN.to_bits())),
        ("/events/0/target", serde_json::json!(7)),
        ("/events/0/sequence", serde_json::json!(1)),
        ("/delta_cycles", serde_json::json!(1)), // Unstarted kernel.
        ("/limits/max_events_per_tick", serde_json::json!(1)),
    ] {
        let mut candidate = good.clone();
        *candidate.pointer_mut(pointer).unwrap() = value;
        invalid.push(candidate);
    }
    let mut duplicate = good.clone();
    duplicate["events"]
        .as_array_mut()
        .unwrap()
        .push(good["events"][0].clone());
    invalid.push(duplicate);
    let mut duplicate = good.clone();
    duplicate["targets"]
        .as_array_mut()
        .unwrap()
        .push(good["targets"][0].clone());
    duplicate["activations"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!(0));
    invalid.push(duplicate);
    let mut counts = good.clone();
    counts["activations"] = serde_json::json!([]);
    invalid.push(counts);
    for invalid in invalid {
        let invalid: SchedulerCheckpoint = serde_json::from_value(invalid).unwrap();
        assert!(
            kernel
                .restore_checkpoint(&invalid, SchedulerCheckpointLimits::default())
                .is_err()
        );
        assert_eq!(image(&kernel), original);
    }
    for budget in [
        SchedulerCheckpointLimits {
            max_events: 0,
            ..Default::default()
        },
        SchedulerCheckpointLimits {
            max_targets: 0,
            ..Default::default()
        },
        SchedulerCheckpointLimits {
            max_name_bytes: 0,
            ..Default::default()
        },
    ] {
        assert!(kernel.restore_checkpoint(&original, budget).is_err());
        assert_eq!(image(&kernel), original);
        assert!(kernel.checkpoint(budget).is_err());
    }
    let mut other = EventScheduler::new(SchedulerLimits::default());
    other
        .schedule_at(
            at(4e-9),
            SchedulerRegion::Active,
            target("other"),
            EventValue::Real(0.0),
        )
        .unwrap();
    let before = image(&other);
    assert!(
        other
            .restore_checkpoint(&original, SchedulerCheckpointLimits::default())
            .is_err()
    );
    assert_eq!(image(&other), before);
}

#[test]
fn event_checkpoint_sequence_exhaustion_does_not_wrap_or_cancel_pending_output() {
    let mut source = EventScheduler::new(SchedulerLimits::default());
    source
        .schedule_at(
            at(4e-9),
            SchedulerRegion::Active,
            target("a"),
            EventValue::Real(1.0),
        )
        .unwrap();
    let mut encoded = serde_json::to_value(image(&source)).unwrap();
    encoded["next_sequence"] = serde_json::json!(u64::MAX);
    let exhausted: SchedulerCheckpoint = serde_json::from_value(encoded).unwrap();
    let mut kernel = EventScheduler::new(SchedulerLimits::default());
    restore(&mut kernel, &exhausted);
    assert_eq!(
        kernel.schedule_superseding_at(
            at(2e-9),
            SchedulerRegion::Active,
            target("a"),
            EventValue::Real(2.0)
        ),
        0
    );
    assert_eq!(kernel.pending(), 1);
    assert_eq!(kernel.next_instant(), Some(at(4e-9)));
    assert_eq!(
        kernel.run_due_events(at(5e-9), |_, _| panic!("failed kernel executed")),
        Err(SchedulerError::SequenceExhausted)
    );
    assert!(
        kernel
            .checkpoint(SchedulerCheckpointLimits::default())
            .is_err()
    );
    restore(&mut kernel, &exhausted);
    assert_eq!(
        kernel.schedule_at(
            at(5e-9),
            SchedulerRegion::Active,
            target("a"),
            EventValue::Real(0.0)
        ),
        Err(SchedulerError::SequenceExhausted)
    );
    assert_eq!(kernel.pending(), 1);
}

#[test]
fn event_checkpoint_reserved_indices_and_counter_boundaries_are_checked() {
    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: u32::MAX,
        max_events_per_tick: u64::MAX,
        ..Default::default()
    };
    let mut kernel = EventScheduler::new(limits);
    let reserved = EventTarget {
        node_id: usize::MAX,
        driver_index: usize::MAX,
        ..target("wake")
    };
    kernel
        .schedule_at(
            at(4e-9),
            SchedulerRegion::Active,
            reserved,
            EventValue::Real(1.0),
        )
        .unwrap();
    let mut encoded = serde_json::to_value(image(&kernel)).unwrap();
    assert_eq!(
        encoded["targets"][0]["node_id"],
        serde_json::json!(u64::MAX)
    );
    assert_eq!(
        encoded["targets"][0]["driver_index"],
        serde_json::json!(u64::MAX)
    );
    let decoded: SchedulerCheckpoint = serde_json::from_value(encoded.clone()).unwrap();
    restore(&mut kernel, &decoded);
    assert_eq!(image(&kernel), decoded);
    encoded["started"] = serde_json::json!(true);
    encoded["delta_cycles"] = serde_json::json!(u32::MAX);
    restore(
        &mut kernel,
        &serde_json::from_value(encoded.clone()).unwrap(),
    );
    assert_eq!(
        kernel.note_delta_cycle(at(0.0)),
        Err(SchedulerError::AccountingExhausted)
    );
    assert!(
        kernel
            .checkpoint(SchedulerCheckpointLimits::default())
            .is_err()
    );
    encoded["delta_cycles"] = serde_json::json!(0);
    encoded["events_executed"] = serde_json::json!(u64::MAX);
    restore(&mut kernel, &serde_json::from_value(encoded).unwrap());
    kernel.schedule_superseding_at(
        at(0.0),
        SchedulerRegion::Active,
        target("now"),
        EventValue::Real(2.0),
    );
    assert_eq!(
        kernel.run_due_events(at(0.0), |_, _| panic!("overflow executed")),
        Err(SchedulerError::AccountingExhausted)
    );
    assert!(
        kernel
            .checkpoint(SchedulerCheckpointLimits::default())
            .is_err()
    );
}
