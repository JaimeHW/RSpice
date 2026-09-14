use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayTimeSide};

fn slopes(history: &DelayBuffer, time: f64, endpoint: f64, delay: f64, expected: [f64; 2]) {
    let before = history.clone();
    for (side, expected) in [DelayTimeSide::Incoming, DelayTimeSide::Outgoing]
        .into_iter()
        .zip(expected)
    {
        let actual = history
            .fixed_trajectory_slope(time, endpoint, delay, side)
            .unwrap();
        assert!(
            (actual - expected).abs() <= 4e-15 * expected.abs(),
            "time={time:e}, side={side:?}: {actual:e} != {expected:e}"
        );
    }
    assert_eq!(*history, before);
}

#[test]
fn physical_slopes_distinguish_prehistory_jumps_corners_and_delay_operand_actions() {
    for initial_jump in [false, true] {
        let mut history = DelayBuffer::new(0);
        if initial_jump {
            history
                .accept_discontinuity(0.0, -1.0, 0.0, 2.0, None)
                .unwrap();
        } else {
            history.accept_sample(0.0, 0.0, 2.0, None).unwrap();
        }
        history
            .accept_discontinuity(1.0, 1.0, 10.0, 2.0, None)
            .unwrap();
        history.accept_sample(2.0, 12.0, 2.0, None).unwrap();
        slopes(&history, 2.0, 12.0, 2.0, [0.0, 1.0]);
        slopes(&history, 3.0, 99.0, 2.0, [1.0, 2.0]);
        slopes(&history, 3.5, 99.0, 2.0, [2.0, 2.0]);
        // An ordinary interpolation knot also has two regular slopes, but
        // it does not acquire a new physical event or scheduled arrival.
        slopes(&history, 4.0, 99.0, 2.0, [2.0, 43.5]);
        assert_eq!(history.next_discontinuity_after(3.0).unwrap(), None);
        assert_eq!(
            history
                .difference_before_arrival(3.0, 99.0, 2.0, None)
                .unwrap()
                .delay_coefficient,
            0.0
        );
    }
}

#[test]
fn a_solved_input_left_limit_closes_the_interval_before_a_local_jump() {
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 0.0, 0.25, None).unwrap();
    slopes(&history, 1.0, 1.0, 0.25, [1.0, 1.0]);
    for right in [-40.0, 20.0, 30.0] {
        let correction = history
            .difference_at_discontinuity(1.0, 1.0, right, 0.25, None)
            .unwrap();
        assert_eq!(correction.output, 0.75 - right);
        slopes(&history, 1.0, 1.0, 0.25, [1.0, 1.0]);
    }
    history
        .accept_discontinuity(1.0, 1.0, 20.0, 0.25, None)
        .unwrap();
    // Accepted history supplies its stored left value even when a caller's
    // present input is now the settled right value.
    slopes(&history, 1.0, 20.0, 0.25, [1.0, 1.0]);
    slopes(&history, 1.25, 20.0, 0.25, [1.0, 0.0]);
}

#[test]
fn sub_ulp_arrival_slopes_keep_the_incoming_and_outgoing_brackets_distinct() {
    for delay in [f64::from_bits(1), 1e-20, 0.75 * f64::EPSILON] {
        let mut history = DelayBuffer::new(0);
        history.accept_sample(0.0, 0.0, delay, None).unwrap();
        history
            .accept_discontinuity(1.0, 1.0, 2.0, delay, None)
            .unwrap();
        let arrival = 1.0_f64.next_up();
        let expected = [1.0, 97.0 / f64::EPSILON];
        slopes(&history, arrival, 99.0, delay, expected);
        history.eval(arrival, -10.0, delay, None).unwrap();
        slopes(&history, arrival, 99.0, delay, expected);
        history.accept_sample(arrival, 99.0, delay, None).unwrap();
        let restored = DelayBuffer::from_checkpoint(history.checkpoint()).unwrap();
        slopes(&restored, arrival, 99.0, delay, expected);
    }
}

#[test]
fn coalesced_arrivals_keep_the_first_incoming_segment_through_pruning_and_restore() {
    let delay = 1e12;
    let first = 1.0_f64.next_up();
    let last = first.next_up();
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 0.0, delay, None).unwrap();
    history
        .accept_discontinuity(first, 1.0, 10.0, delay, None)
        .unwrap();
    history
        .accept_discontinuity(last, 20.0, 30.0, delay, None)
        .unwrap();
    history.accept_sample(2.0, 34.0, delay, None).unwrap();
    let arrival = (delay + 1.0).next_up();
    let expected = [1.0 / first, 4.0 / (2.0 - last)];
    slopes(&history, arrival, 99.0, delay, expected);
    history.accept_sample(arrival, 99.0, delay, None).unwrap();
    let restored = DelayBuffer::from_checkpoint(history.checkpoint()).unwrap();
    slopes(&restored, arrival, 99.0, delay, expected);
    let mut incomplete = restored.checkpoint();
    incomplete.samples.drain(..2);
    incomplete.left_limits.remove(0);
    let old = DelayBuffer::from_checkpoint(incomplete).unwrap();
    assert!(
        old.fixed_trajectory_slope(arrival, 99.0, delay, DelayTimeSide::Incoming)
            .unwrap_err()
            .contains("retained bracket")
    );
    assert_eq!(
        old.fixed_trajectory_slope(arrival, 99.0, delay, DelayTimeSide::Outgoing)
            .unwrap(),
        expected[1]
    );
}

#[test]
fn slopes_preserve_tiny_intervals_large_levels_and_raw_quotient_cancellation() {
    for exponent in [-1000, -500, 0, 500, 1000] {
        let unit = 2.0_f64.powi(exponent);
        let delay = 2.0 * unit;
        let mut history = DelayBuffer::new(0);
        history.accept_sample(0.0, 0.0, delay, None).unwrap();
        history
            .accept_discontinuity(unit, 1.0, 1.0, delay, None)
            .unwrap();
        history.accept_sample(2.0 * unit, 3.0, delay, None).unwrap();
        slopes(&history, 3.0 * unit, 9.0, delay, [1.0 / unit, 2.0 / unit]);
    }
    let mut history = DelayBuffer::new(0);
    history
        .accept_sample(0.0, -f64::MAX, f64::MAX, None)
        .unwrap();
    history
        .accept_sample(f64::MAX, f64::MAX, f64::MAX, None)
        .unwrap();
    slopes(&history, f64::MAX, f64::MAX, f64::MAX, [0.0, 2.0]);
    let tiny = f64::from_bits(1);
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 0.0, tiny, None).unwrap();
    history.accept_sample(tiny, tiny, tiny, None).unwrap();
    slopes(&history, 2.0 * tiny, 2.0 * tiny, tiny, [1.0, 1.0]);
}

#[test]
fn a_compensated_target_rounded_onto_the_present_knot_keeps_the_physical_segment() {
    let delay = 1e-20;
    let mut history = DelayBuffer::new(0);
    for (time, value) in [(0.0, 0.0), (1.0, 1.0), (2.0, 3.0)] {
        history.accept_sample(time, value, delay, None).unwrap();
    }
    assert_eq!(2.0 - delay, 2.0);
    slopes(&history, 2.0, 3.0, delay, [2.0, 2.0]);
}

#[test]
fn an_unrepresentable_slope_does_not_disable_an_ordinary_finite_value_read() {
    let delay = f64::from_bits(1);
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 0.0, delay, None).unwrap();
    assert!(
        history
            .fixed_trajectory_slope(delay, 1.0, delay, DelayTimeSide::Outgoing)
            .unwrap_err()
            .contains("not representable")
    );
    assert_eq!(
        history
            .eval_with_coefficients(delay, 1.0, delay, None)
            .unwrap()
            .output,
        0.0
    );
}

#[test]
fn slope_queries_reject_invalid_clocks_missing_history_and_nonfixed_ownership() {
    let mut history = DelayBuffer::new(0);
    assert_eq!(
        history
            .fixed_trajectory_slope(0.0, 0.0, 1.0, DelayTimeSide::Incoming)
            .unwrap(),
        0.0
    );
    assert!(
        history
            .fixed_trajectory_slope(1.0, 0.0, 1.0, DelayTimeSide::Outgoing)
            .is_err()
    );
    history.accept_sample(0.0, 0.0, 1.0, None).unwrap();
    history.accept_sample(0.5, 1.0, 1.0, None).unwrap();
    let before = history.clone();
    for (time, endpoint, delay) in [
        (f64::NAN, 0.0, 1.0),
        (f64::INFINITY, 0.0, 1.0),
        (-1.0, 0.0, 1.0),
        (0.25, 0.0, 1.0),
        (1.0, f64::NAN, 1.0),
    ] {
        assert!(
            history
                .fixed_trajectory_slope(time, endpoint, delay, DelayTimeSide::Outgoing)
                .is_err()
        );
    }
    assert_eq!(history, before);
    for ignored_delay in [-1.0, 2.0, f64::NAN, f64::INFINITY] {
        slopes(&history, 1.0, 0.0, ignored_delay, [0.0, 2.0]);
    }
    for invalid_delay in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(
            DelayBuffer::new(0)
                .fixed_trajectory_slope(0.0, 0.0, invalid_delay, DelayTimeSide::Outgoing)
                .is_err()
        );
    }
    let mut variable = DelayBuffer::new(0);
    variable.accept_sample(0.0, 0.0, 1.0, Some(2.0)).unwrap();
    assert!(
        variable
            .fixed_trajectory_slope(1.0, 1.0, 1.0, DelayTimeSide::Outgoing)
            .is_err()
    );
}
