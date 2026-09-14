use rspice_veriloga_runtime::transport_delay::{
    DelayBuffer, DelayCheckpoint, DelayConfiguration, MAX_DELAY_HISTORY_SAMPLES,
};

fn value(buffer: &DelayBuffer, time: f64, input: f64, delay: f64) -> f64 {
    let mut probe = buffer.clone();
    probe
        .eval_with_coefficients(time, input, delay, None)
        .unwrap()
        .output
}

#[test]
fn jump_closes_the_incoming_interval_and_opens_the_outgoing_interval() {
    let mut history = DelayBuffer::new(4);
    history.accept_sample(0.0, 0.0, 2.0, None).unwrap();
    history
        .accept_discontinuity(1.0, 1.0, 10.0, 2.0, None)
        .unwrap();
    history.accept_sample(2.0, 12.0, 2.0, None).unwrap();
    let before = history.clone();
    for (time, expected) in [(2.5, 0.5), (2.75, 0.75), (3.0, 10.0), (3.5, 11.0)] {
        assert_eq!(value(&history, time, 99.0, 2.0), expected);
        let correction = history
            .difference_with_coefficients(time, 99.0, 2.0, None)
            .unwrap();
        assert_eq!(correction.output, expected - 99.0);
        assert_eq!(correction.apply_input_derivative(3.0).unwrap(), -3.0);
    }
    assert!(value(&history, 3.0_f64.next_down(), 99.0, 2.0) < 1.0);
    assert!(value(&history, 3.0_f64.next_up(), 99.0, 2.0) >= 10.0);
    assert_eq!(
        history.accepted_knots().collect::<Vec<_>>(),
        vec![(0.0, 0.0, 0.0), (1.0, 1.0, 10.0), (2.0, 12.0, 12.0)]
    );
    assert_eq!(history, before);
}

#[test]
fn right_side_newton_probe_holds_the_solved_left_limit_fixed() {
    let mut history = DelayBuffer::new(4);
    history.accept_sample(0.0, 0.0, 0.25, None).unwrap();
    let before = history.clone();
    for right in [20.0, 30.0, -40.0] {
        let trial = history
            .difference_at_discontinuity(1.0, 1.0, right, 0.25, None)
            .unwrap();
        assert_eq!(trial.output, 0.75 - right);
        assert_eq!(trial.apply_input_derivative(1.0).unwrap(), -1.0);
        assert_eq!(trial.delay_coefficient, 0.0);
    }
    assert_eq!(history, before);
    history
        .accept_discontinuity(1.0, 1.0, 20.0, 0.25, None)
        .unwrap();
    let accepted = history
        .difference_with_coefficients(1.0, 20.0, 0.25, None)
        .unwrap();
    assert_eq!(accepted.output, -19.25);
    assert!(
        history
            .difference_at_discontinuity(1.0, 2.0, 20.0, 0.25, None)
            .is_err()
    );
    assert_eq!(history.next_discontinuity_after(1.0).unwrap(), Some(1.25));
}

#[test]
fn initial_jump_retains_prehistory_until_its_exact_arrival() {
    for delay in [f64::from_bits(1), 0.5, 1e200] {
        let mut history = DelayBuffer::new(2);
        history
            .accept_discontinuity(0.0, -1.0, 3.0, delay, None)
            .unwrap();
        history.validate_accepted_time(0.0).unwrap();
        assert_eq!(value(&history, 0.0, 3.0, delay), -1.0);
        assert_eq!(value(&history, delay.next_down(), 3.0, delay), -1.0);
        assert_eq!(value(&history, delay, 3.0, delay), 3.0);
        assert_eq!(history.next_discontinuity_after(0.0).unwrap(), Some(delay));
        assert_eq!(history.next_discontinuity_after(delay).unwrap(), None);
    }
}

#[test]
fn arrivals_round_causally_and_only_schedule_declared_events() {
    for delay in [f64::from_bits(1), 1e-20, f64::EPSILON * 0.75] {
        let mut history = DelayBuffer::new(3);
        history.accept_sample(0.0, 0.0, delay, None).unwrap();
        history
            .accept_discontinuity(1.0, 1.0, 2.0, delay, None)
            .unwrap();
        assert_eq!(
            history.next_discontinuity_after(1.0).unwrap(),
            Some(1.0_f64.next_up())
        );
        assert_eq!(
            history.next_discontinuity_after(1.0_f64.next_up()).unwrap(),
            None
        );
    }
    let mut history = DelayBuffer::new(4);
    for (time, input) in [(0.0, 0.0), (1.0, 4.0), (2.0, -7.0)] {
        history.accept_sample(time, input, 1e-9, None).unwrap();
    }
    assert_eq!(history.next_discontinuity_after(2.0).unwrap(), None);
    assert!(history.next_discontinuity_after(f64::NAN).is_err());

    let mut corner = DelayBuffer::new(3);
    corner.accept_sample(0.0, 0.0, 0.5, None).unwrap();
    corner
        .accept_discontinuity(1.0, 3.0, 3.0, 0.5, None)
        .unwrap();
    assert_eq!(corner.next_discontinuity_after(1.0).unwrap(), Some(1.5));
    let restored = DelayBuffer::from_checkpoint(corner.checkpoint()).unwrap();
    assert_eq!(restored.next_discontinuity_after(1.0).unwrap(), Some(1.5));
}

#[test]
fn sided_history_replays_prunes_and_rejects_invalid_state_atomically() {
    let mut history = DelayBuffer::new(4);
    history.accept_sample(0.0, -0.0, 0.5, None).unwrap();
    for index in 1..200 {
        let time = f64::from(index) * 0.125;
        let current = f64::from(index % 7);
        let before = history.clone();
        history
            .validate_discontinuity(time, current - 0.5, current, 0.5, None)
            .unwrap();
        assert_eq!(history, before);
        history
            .accept_discontinuity(time, current - 0.5, current, 0.5, None)
            .unwrap();
        let restored = DelayBuffer::from_checkpoint(history.checkpoint()).unwrap();
        assert_eq!(restored, history);
        assert!(history.accepted_sample_count() <= 6);
        assert!(history.accepted_left_limits().len() <= 6);
        for offset in [0.0, 0.125, 0.25, 1.0] {
            assert_eq!(
                value(&restored, time + offset, current, 0.5),
                value(&history, time + offset, current, 0.5)
            );
            assert_eq!(
                restored.next_discontinuity_after(time + offset),
                history.next_discontinuity_after(time + offset)
            );
        }
    }
    let accepted = history.clone();
    let time = history.accepted_samples().next_back().unwrap().0;
    assert!(
        history
            .accept_discontinuity(time, 1.0, 2.0, 0.5, None)
            .is_err()
    );
    assert!(
        history
            .accept_discontinuity(time + 1.0, f64::NAN, 2.0, 0.5, None)
            .is_err()
    );
    assert!(
        history
            .accept_discontinuity(time + 1.0, 1.0, f64::INFINITY, 0.5, None)
            .is_err()
    );
    assert_eq!(history, accepted);
    let good = history.checkpoint();
    let first_event = good.left_limits.first().unwrap().0;
    for (bad_time, bad_value) in [(time + 1.0, 0.0), (time, f64::NAN), (first_event, 1.0)] {
        let mut bad = good.clone();
        *bad.left_limits.last_mut().unwrap() = (bad_time, bad_value);
        assert!(history.restore_checkpoint(&bad).is_err());
        assert_eq!(history, accepted);
    }
    history.clear();
    assert_eq!(history.accepted_sample_count(), 0);
    assert_eq!(history.accepted_left_limits().len(), 0);
}

#[test]
fn jump_sides_share_the_existing_history_resource_ceiling() {
    let count = MAX_DELAY_HISTORY_SAMPLES - 1;
    let mut history = DelayBuffer::from_checkpoint(DelayCheckpoint {
        event_orders: Vec::new(),
        configuration: Some(DelayConfiguration::Fixed {
            delay: 2.0 * count as f64,
        }),
        samples: (0..count).map(|index| (index as f64, 0.0)).collect(),
        left_limits: Vec::new(),
    })
    .unwrap();
    let before = history.clone();
    assert!(
        history
            .accept_discontinuity(count as f64, 0.0, 1.0, 2.0 * count as f64, None)
            .unwrap_err()
            .contains("supported")
    );
    assert_eq!(history, before);
    history
        .accept_sample(count as f64, 0.0, 2.0 * count as f64, None)
        .unwrap();
    assert_eq!(history.accepted_sample_count(), MAX_DELAY_HISTORY_SAMPLES);
}
