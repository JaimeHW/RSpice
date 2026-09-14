use rspice_veriloga_runtime::transport_delay::{
    DelayBuffer, DelayCheckpoint, DelayConfiguration, MAX_DELAY_HISTORY_SAMPLES,
};

#[test]
fn incoming_reads_select_exact_initial_and_later_arrivals_without_mutation() {
    let mut history = DelayBuffer::new(0);
    history
        .accept_discontinuity(0.0, -1.0, 0.0, 2.0, None)
        .unwrap();
    history
        .accept_discontinuity(1.0, 1.0, 10.0, 2.0, None)
        .unwrap();
    let accepted = history.clone();
    for (time, expected) in [(2.0, -1.0), (3.0, 1.0)] {
        let incoming = history
            .difference_before_arrival(time, 99.0, 2.0, None)
            .unwrap();
        assert_eq!(incoming.output, expected - 99.0);
        assert_eq!(incoming.apply_input_derivative(3.0).unwrap(), -3.0);
        assert_eq!(incoming.delay_coefficient, 0.0);
        for adjacent in [time.next_down(), time.next_up()] {
            assert_eq!(
                history
                    .difference_before_arrival(adjacent, 99.0, 2.0, None)
                    .unwrap(),
                history
                    .difference_with_coefficients(adjacent, 99.0, 2.0, None)
                    .unwrap()
            );
        }
    }
    assert_eq!(
        history
            .difference_with_coefficients(3.0, 99.0, 2.0, None)
            .unwrap()
            .output,
        -89.0
    );
    assert_eq!(history, accepted);
}

#[test]
fn sub_ulp_arrival_keeps_incoming_limit_and_outgoing_physical_interpolation_distinct() {
    for delay in [f64::from_bits(1), 1e-20, f64::EPSILON * 0.75] {
        let mut history = DelayBuffer::new(0);
        history.accept_sample(0.0, 0.0, delay, None).unwrap();
        history
            .accept_discontinuity(1.0, 1.0, 2.0, delay, None)
            .unwrap();
        let arrival = 1.0_f64.next_up();
        assert_eq!(
            history.next_discontinuity_after(1.0).unwrap(),
            Some(arrival)
        );
        let incoming = history
            .difference_before_arrival(arrival, 99.0, delay, None)
            .unwrap();
        assert_eq!(incoming.output, -98.0);
        assert_eq!(incoming.apply_input_derivative(1.0).unwrap(), -1.0);
        // Do not snap the outgoing read back to 2. The exact physical target
        // can have traversed almost all of the interval from 2 to 99 by the
        // first representable clock at or after the physical arrival.
        let outgoing = history
            .difference_with_coefficients(arrival, 99.0, delay, None)
            .unwrap();
        let expected = (2.0 - 99.0) * (delay / f64::EPSILON);
        assert!((outgoing.output - expected).abs() <= expected.abs() * 1e-14);
        history.accept_sample(arrival, 99.0, delay, None).unwrap();
        let restored = DelayBuffer::from_checkpoint(history.checkpoint()).unwrap();
        assert_eq!(
            restored
                .difference_before_arrival(arrival, 99.0, delay, None)
                .unwrap(),
            incoming
        );
    }
}

#[test]
fn coalesced_arrivals_retain_the_earliest_left_limit_across_acceptance_and_restore() {
    let delay = 1e12;
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 0.0, delay, None).unwrap();
    for (time, left, right) in [
        (1.0_f64.next_up(), 1.0, 10.0),
        (1.0_f64.next_up().next_up(), 20.0, 30.0),
    ] {
        history
            .accept_discontinuity(time, left, right, delay, None)
            .unwrap();
    }
    let arrival = (delay + 1.0).next_up();
    assert_eq!(
        history.next_discontinuity_after(2.0).unwrap(),
        Some(arrival)
    );
    assert_eq!(
        history
            .difference_before_arrival(arrival, 99.0, delay, None)
            .unwrap()
            .output,
        -98.0
    );
    history.accept_sample(arrival, 99.0, delay, None).unwrap();
    let restored = DelayBuffer::from_checkpoint(history.checkpoint()).unwrap();
    assert_eq!(
        restored
            .difference_before_arrival(arrival, 99.0, delay, None)
            .unwrap()
            .output,
        -98.0
    );
    assert_eq!(restored.accepted_left_limits().len(), 2);
    assert_eq!(restored.next_discontinuity_after(arrival).unwrap(), None);
    // An older checkpoint may have already pruned the first event of this
    // group. Refuse its unprovable incoming value; future interpolation and
    // ordinary reads remain available.
    let mut incomplete = restored.checkpoint();
    incomplete.samples.drain(..2);
    incomplete.left_limits.remove(0);
    let old = DelayBuffer::from_checkpoint(incomplete).unwrap();
    assert!(
        old.difference_before_arrival(arrival, 99.0, delay, None)
            .unwrap_err()
            .contains("retained bracket")
    );
    assert!(
        old.difference_with_coefficients(arrival, 99.0, delay, None)
            .is_ok()
    );
    assert!(
        old.difference_before_arrival(arrival.next_up(), 99.0, delay, None)
            .is_ok()
    );
    history
        .accept_sample(arrival.next_up(), 99.0, delay, None)
        .unwrap();
    assert!(history.accepted_sample_count() <= 3);
    assert!(history.accepted_left_limits().len() <= 1);
}

#[test]
fn incoming_arrivals_validate_clocks_values_and_fixed_delay_ownership() {
    let mut history = DelayBuffer::new(0);
    history
        .accept_discontinuity(0.0, f64::MAX, 0.0, 1.0, None)
        .unwrap();
    for time in [f64::NAN, f64::INFINITY, -1.0] {
        assert!(
            history
                .difference_before_arrival(time, 1.0, 1.0, None)
                .is_err()
        );
    }
    assert!(
        history
            .difference_before_arrival(1.0, f64::NAN, 1.0, None)
            .is_err()
    );
    assert!(
        history
            .difference_before_arrival(1.0, -f64::MAX, 1.0, None)
            .unwrap_err()
            .contains("not representable")
    );
    let mut variable = DelayBuffer::new(0);
    variable
        .accept_discontinuity(0.0, 0.0, 1.0, 1.0, Some(2.0))
        .unwrap();
    assert!(
        variable
            .difference_before_arrival(1.0, 1.0, 1.0, Some(2.0))
            .unwrap_err()
            .contains("owning solver")
    );
}

#[test]
fn coalesced_event_retention_obeys_the_combined_history_budget() {
    let count = (MAX_DELAY_HISTORY_SAMPLES - 1) / 2;
    let delay = 1e12;
    let mut samples = vec![(0.0, 0.0)];
    let mut left_limits = Vec::with_capacity(count);
    for index in 1..=count {
        let time = 1.0 + index as f64 * f64::EPSILON;
        samples.push((time, 0.0));
        left_limits.push((time, 0.0));
    }
    let mut history = DelayBuffer::from_checkpoint(DelayCheckpoint {
        event_orders: Vec::new(),
        configuration: Some(DelayConfiguration::Fixed { delay }),
        samples,
        left_limits,
    })
    .unwrap();
    let before = history.clone();
    let arrival = (delay + 1.0).next_up();
    assert!(
        history
            .accept_discontinuity(arrival, 0.0, 0.0, delay, None)
            .unwrap_err()
            .contains("supported")
    );
    assert_eq!(history, before);
    history.accept_sample(arrival, 0.0, delay, None).unwrap();
    assert_eq!(
        history.accepted_sample_count() + history.accepted_left_limits().len(),
        MAX_DELAY_HISTORY_SAMPLES
    );
    history
        .accept_sample(arrival.next_up(), 0.0, delay, None)
        .unwrap();
    assert!(history.accepted_sample_count() + history.accepted_left_limits().len() <= 6);
}
