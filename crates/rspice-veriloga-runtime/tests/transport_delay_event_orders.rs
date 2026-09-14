use rspice_veriloga_runtime::transport_delay::{
    DelayBuffer, DelayCheckpoint, DelayConfiguration, DelayEvent, DelayEventOrder,
    MAX_DELAY_HISTORY_SAMPLES,
};

fn corner(value: f64, order: u32) -> DelayEvent {
    DelayEvent {
        left: value,
        right: value,
        order: DelayEventOrder::AtLeast(order),
    }
}

#[test]
fn event_order_acceptance_is_atomic_and_independent_of_interpolation_knots() {
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 0.0, 2.0, None).unwrap();
    history.accept_sample(0.25, 9.0, 2.0, None).unwrap();
    assert_eq!(history.next_event_after(0.25).unwrap(), None);
    history.eval(1.0, 123.0, 2.0, None).unwrap();
    let staged = history.clone();
    for event in [
        DelayEvent {
            left: 1.0,
            right: 2.0,
            order: DelayEventOrder::AtLeast(1),
        },
        DelayEvent {
            left: f64::NAN,
            right: 2.0,
            order: DelayEventOrder::AtLeast(0),
        },
        corner(f64::INFINITY, 3),
    ] {
        assert!(history.validate_event(0.5, event, 2.0, None).is_err());
        assert!(history.accept_event(0.5, event, 2.0, None).is_err());
        assert_eq!(history, staged);
    }
    history
        .validate_event(0.5, corner(1.0, 3), 2.0, None)
        .unwrap();
    assert_eq!(history, staged);
    history
        .accept_event(0.5, corner(1.0, 3), 2.0, None)
        .unwrap();
    assert!(history.validate_checkpoint_ready().is_ok());
    history
        .accept_discontinuity(1.0, 1.0, 2.0, 2.0, None)
        .unwrap();
    assert_eq!(
        history.accepted_event_orders().collect::<Vec<_>>(),
        vec![(0.5, 3)]
    );
    let accepted = history.clone();
    for _ in 0..3 {
        history.eval(1.5, 99.0, 2.0, None).unwrap();
        history.begin_evaluation();
        assert_eq!(history, accepted);
    }
    let restored = DelayBuffer::from_checkpoint(history.checkpoint()).unwrap();
    assert_eq!(restored, history);
    assert_eq!(
        restored.next_event_after(1.0).unwrap().unwrap().order,
        DelayEventOrder::AtLeast(3)
    );
    assert_eq!(
        restored.next_event_after(2.5).unwrap().unwrap().order,
        DelayEventOrder::Unknown
    );
}

#[test]
fn event_orders_merge_every_contributor_at_a_compensated_arrival() {
    let delay = 1e12;
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 0.0, delay, None).unwrap();
    let a = 1.0_f64.next_up();
    let b = a.next_up();
    history
        .accept_event(a, corner(2.0, 3), delay, None)
        .unwrap();
    history
        .accept_event(b, corner(4.0, 1), delay, None)
        .unwrap();
    let arrival = (delay + 1.0).next_up();
    let event = history.next_event_after(2.0).unwrap().unwrap();
    assert_eq!(event.time, arrival);
    assert_eq!(event.order, DelayEventOrder::AtLeast(1));
    assert_eq!(
        history.next_event_after(arrival.next_down()).unwrap(),
        Some(event)
    );
    assert_eq!(history.next_event_after(arrival).unwrap(), None);
    let copy = DelayBuffer::from_checkpoint(history.checkpoint()).unwrap();
    assert_eq!(copy.next_event_after(2.0).unwrap(), Some(event));
    history
        .accept_discontinuity(b.next_up(), 5.0, 5.0, delay, None)
        .unwrap();
    assert_eq!(
        history.next_event_after(2.0).unwrap().unwrap().order,
        DelayEventOrder::Unknown
    );
    assert_eq!(
        history.next_discontinuity_after(2.0).unwrap(),
        Some(arrival)
    );
}

#[test]
fn event_orders_preserve_initial_and_sub_ulp_arrivals_and_signed_zero() {
    for delay in [f64::from_bits(1), 1e-20, 2.0] {
        let mut history = DelayBuffer::new(0);
        history
            .accept_event(
                0.0,
                DelayEvent {
                    left: -0.0,
                    right: 0.0,
                    order: DelayEventOrder::AtLeast(u32::MAX),
                },
                delay,
                None,
            )
            .unwrap();
        let event = history.next_event_after(0.0).unwrap().unwrap();
        assert_eq!(event.time, delay);
        assert_eq!(event.order, DelayEventOrder::AtLeast(u32::MAX));
        let restored = DelayBuffer::from_checkpoint(history.checkpoint()).unwrap();
        assert_eq!(
            restored.accepted_left_limits().next().unwrap().1.to_bits(),
            (-0.0_f64).to_bits()
        );
        history
            .accept_event(1.0, corner(0.0, 2), delay, None)
            .unwrap();
        let next = history.next_event_after(delay.max(1.0)).unwrap().unwrap();
        assert_eq!(
            next.time,
            history
                .next_discontinuity_after(delay.max(1.0))
                .unwrap()
                .unwrap()
        );
        assert_eq!(next.order, DelayEventOrder::AtLeast(2));
    }
    let mut bounded = DelayBuffer::new(0);
    bounded
        .accept_event(0.0, corner(0.0, 1), 1.0, Some(2.0))
        .unwrap();
    assert!(
        bounded
            .next_event_after(0.0)
            .unwrap_err()
            .contains("owning solver")
    );
}

#[test]
fn event_order_checkpoint_validation_rejects_orphans_and_false_value_continuity() {
    let mut history = DelayBuffer::new(0);
    history
        .accept_event(0.0, corner(0.0, 2), 2.0, None)
        .unwrap();
    history
        .accept_discontinuity(1.0, 0.0, 3.0, 2.0, None)
        .unwrap();
    let checkpoint = history.checkpoint();
    history.eval(1.5, 4.0, 2.0, None).unwrap();
    let before = history.clone();
    for records in [
        vec![(0.5, 1)],
        vec![(f64::NAN, 1)],
        vec![(-0.0, 1)],
        vec![(0.0, 1), (0.0, 2)],
        vec![(1.0, 0), (0.0, 2)],
        vec![(1.0, 1)],
    ] {
        let mut invalid = checkpoint.clone();
        invalid.event_orders = records;
        assert!(history.restore_checkpoint(&invalid).is_err());
        assert!(DelayBuffer::from_checkpoint(invalid).is_err());
        assert_eq!(history, before);
    }
    let orphan = DelayCheckpoint {
        event_orders: vec![(0.0, 0)],
        ..Default::default()
    };
    assert!(DelayBuffer::from_checkpoint(orphan).is_err());
    let mut unknown = checkpoint;
    unknown.event_orders.clear();
    let restored = DelayBuffer::from_checkpoint(unknown).unwrap();
    assert_eq!(
        restored.next_event_after(0.0).unwrap().unwrap().order,
        DelayEventOrder::Unknown
    );
}

#[test]
fn event_order_pruning_tracks_the_retained_incoming_bracket() {
    let mut history = DelayBuffer::new(0);
    for time in 0..5 {
        history
            .accept_event(f64::from(time), corner(f64::from(time), 2), 2.0, None)
            .unwrap();
    }
    let first = history.accepted_samples().next().unwrap().0;
    assert!(first > 0.0);
    assert_eq!(history.accepted_event_orders().next().unwrap().0, first);
    assert!(
        history
            .accepted_event_orders()
            .all(|(time, _)| time >= first)
    );
    assert_eq!(
        DelayBuffer::from_checkpoint(history.checkpoint()).unwrap(),
        history
    );
    history.clear();
    assert_eq!(history.accepted_event_orders().len(), 0);
    assert_eq!(history.next_event_after(0.0).unwrap(), None);
}

#[test]
fn event_order_records_share_the_existing_hard_history_budget() {
    let count = (MAX_DELAY_HISTORY_SAMPLES - 3) / 3;
    let samples = (0..=count).map(|i| (i as f64, 0.0)).collect();
    let left_limits = (1..=count).map(|i| (i as f64, 0.0)).collect();
    let event_orders = (1..=count).map(|i| (i as f64, 2)).collect();
    let delay = 1e12;
    let mut history = DelayBuffer::from_checkpoint(DelayCheckpoint {
        configuration: Some(DelayConfiguration::Fixed { delay }),
        samples,
        left_limits,
        event_orders,
    })
    .unwrap();
    history
        .accept_sample(count as f64 + 0.5, 0.0, delay, None)
        .unwrap();
    let time = count as f64 + 1.0;
    history.eval(time, 1.0, delay, None).unwrap();
    let before = history.clone();
    assert!(
        history
            .validate_event(time, corner(0.0, 2), delay, None)
            .unwrap_err()
            .contains("supported")
    );
    assert!(
        history
            .accept_event(time, corner(0.0, 2), delay, None)
            .is_err()
    );
    assert_eq!(history, before);
    history
        .accept_discontinuity(time, 0.0, 0.0, delay, None)
        .unwrap();
    let checkpoint = history.checkpoint();
    assert_eq!(
        checkpoint.samples.len() + checkpoint.left_limits.len() + checkpoint.event_orders.len(),
        MAX_DELAY_HISTORY_SAMPLES
    );
    let mut invalid = checkpoint;
    invalid.event_orders.push((time, 2));
    assert!(
        DelayBuffer::validate_checkpoint(&invalid)
            .unwrap_err()
            .contains("supported")
    );
}
