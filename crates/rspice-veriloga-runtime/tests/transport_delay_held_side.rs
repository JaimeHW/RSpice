use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayTimeSide};

#[test]
fn held_endpoint_removes_present_input_from_both_history_sides() {
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 0.0, 0.25, None).unwrap();
    let original = history.clone();
    for side in [DelayTimeSide::Incoming, DelayTimeSide::Outgoing] {
        for present in [-20.0, 0.75, 3.0, 40.0] {
            let value = history
                .difference_at_discontinuity_on_side(1.0, 1.0, present, 0.25, None, side)
                .unwrap();
            assert_eq!(value.output, 0.75 - present);
            assert_eq!(value.apply_input_derivative(2.0).unwrap(), -2.0);
            assert_eq!(value.delay_coefficient, 0.0);
        }
    }
    let moving = history
        .difference_before_arrival(1.0, 3.0, 0.25, None)
        .unwrap();
    assert_eq!(moving.output, -0.75);
    assert_eq!(moving.apply_input_derivative(2.0).unwrap(), -0.5);
    assert_eq!(history, original);
}

#[test]
fn held_history_side_selects_arrival_values_and_preserves_staged_candidates() {
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 0.0, 1.0, None).unwrap();
    history
        .accept_discontinuity(1.0, 2.0, 10.0, 1.0, None)
        .unwrap();
    history.accept_sample(1.5, 12.0, 1.0, None).unwrap();
    history
        .eval_with_coefficients(1.75, 80.0, 1.0, None)
        .unwrap();
    let original = history.clone();
    for (side, delayed, slope) in [
        (DelayTimeSide::Incoming, 2.0, 2.0),
        (DelayTimeSide::Outgoing, 10.0, 4.0),
    ] {
        let value = history
            .difference_at_discontinuity_on_side(2.0, 14.0, 20.0, 1.0, None, side)
            .unwrap();
        assert_eq!(value.output, delayed - 20.0);
        assert_eq!(value.apply_input_derivative(1.0).unwrap(), -1.0);
        assert_eq!(
            history
                .fixed_trajectory_slope(2.0, 14.0, 1.0, side)
                .unwrap(),
            slope
        );
    }
    assert_eq!(history, original);
}

#[test]
fn held_endpoint_correction_does_not_round_through_a_large_endpoint_difference() {
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 1.0, 1.0, None).unwrap();
    for side in [DelayTimeSide::Incoming, DelayTimeSide::Outgoing] {
        let value = history
            .difference_at_discontinuity_on_side(1.0, 1e20, 1.0, 1.0, None, side)
            .unwrap();
        assert_eq!(value.output, 0.0);
        assert_eq!(value.apply_input_derivative(1.0).unwrap(), -1.0);
    }
    let rounded = history
        .difference_before_arrival(1.0, 1e20, 1.0, None)
        .unwrap()
        .output;
    assert_eq!(
        rounded + 1e20 - 1.0,
        -1.0,
        "reconstructing the delayed value from that difference loses it"
    );
}

#[test]
fn held_history_sides_keep_frozen_definitions_and_accepted_endpoint_rules() {
    let mut history = DelayBuffer::new(0);
    history
        .accept_discontinuity(0.0, -1.0, 0.0, 1.0, None)
        .unwrap();
    history
        .accept_discontinuity(1.0, 1.0, 10.0, 1.0, None)
        .unwrap();
    let original = history.clone();
    for (side, expected) in [
        (DelayTimeSide::Incoming, -8.0),
        (DelayTimeSide::Outgoing, -7.0),
    ] {
        let value = history
            .difference_at_discontinuity_on_side(1.0, 1.0, 7.0, f64::NAN, None, side)
            .unwrap();
        assert_eq!(value.output, expected);
        assert!(
            history
                .difference_at_discontinuity_on_side(1.0, 10.0, 7.0, 1.0, None, side)
                .is_err()
        );
        assert!(
            history
                .difference_at_discontinuity_on_side(2.0, f64::NAN, 7.0, 1.0, None, side)
                .is_err()
        );
    }
    assert_eq!(history, original);
    let mut bounded = DelayBuffer::new(0);
    bounded.accept_sample(0.0, 0.0, 1.0, Some(2.0)).unwrap();
    assert!(
        bounded
            .difference_at_discontinuity_on_side(
                1.0,
                2.0,
                3.0,
                1.0,
                Some(2.0),
                DelayTimeSide::Incoming
            )
            .is_err()
    );
}
