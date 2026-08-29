//! External public-API contracts for qualified threshold-crossing primitives.

use rspice_core::analysis::measurements::{
    CrossingEvent, EdgeDirection, MeasurementError, Waveform,
};

fn waveform(time: &[f64], values: &[f64]) -> Waveform {
    Waveform::new(time, values).expect("finite strictly increasing crossing fixture is valid")
}

fn assert_event(
    event: &CrossingEvent,
    time: f64,
    index: usize,
    direction: EdgeDirection,
    slew_rate: f64,
    threshold: f64,
) {
    assert_eq!(event.time, time);
    assert_eq!(event.index, index);
    assert_eq!(event.direction, direction);
    assert_eq!(event.slew_rate, slew_rate);
    assert_eq!(event.value, threshold);
}

#[test]
fn tiny_power_of_two_quarter_crossing_is_not_clamped_to_midpoint() {
    let amplitude = 2.0_f64.powi(-500);
    let threshold = amplitude / 4.0;
    let waveform = waveform(&[0.0, 1.0], &[0.0, amplitude]);

    let crossings = waveform
        .find_crossings(threshold, EdgeDirection::Rising)
        .expect("tiny finite crossing qualifies");
    assert_eq!(crossings.len(), 1);
    assert_event(
        &crossings[0],
        0.25,
        0,
        EdgeDirection::Rising,
        amplitude,
        threshold,
    );
    assert_eq!(
        waveform
            .first_crossing(threshold, EdgeDirection::Rising)
            .expect("tiny first-crossing request qualifies")
            .expect("fixture crosses once")
            .time,
        0.25
    );
    assert_eq!(
        waveform
            .cross_time(threshold, EdgeDirection::Rising, 0)
            .expect("tiny ordinal request qualifies"),
        Some(0.25)
    );
    assert_eq!(
        waveform
            .cross_time(threshold, EdgeDirection::Rising, 1)
            .expect("missing tiny ordinal request qualifies"),
        None
    );
}

#[test]
fn sub_femtosecond_falling_segment_retains_finite_signed_slew() {
    let interval = 2.0_f64.powi(-60);
    let amplitude = 2.0_f64.powi(-40);
    let threshold = 0.75 * amplitude;
    let waveform = waveform(&[0.0, interval], &[amplitude, 0.0]);

    let crossings = waveform
        .find_crossings(threshold, EdgeDirection::Falling)
        .expect("sub-femtosecond crossing qualifies");
    assert_eq!(crossings.len(), 1);
    assert_event(
        &crossings[0],
        interval / 4.0,
        0,
        EdgeDirection::Falling,
        -2.0_f64.powi(20),
        threshold,
    );
}

#[test]
fn maximum_finite_voltage_span_has_finite_midpoint_and_slew() {
    let waveform = waveform(&[0.0, 2.0], &[-f64::MAX, f64::MAX]);

    let crossings = waveform
        .find_crossings(0.0, EdgeDirection::Rising)
        .expect("scale-safe extreme crossing qualifies");
    assert_eq!(crossings.len(), 1);
    assert_event(&crossings[0], 1.0, 0, EdgeDirection::Rising, f64::MAX, 0.0);
}

#[test]
fn crossing_time_retains_a_small_residual_after_large_time_cancellation() {
    let large_endpoint = 2.0_f64.powi(60);
    let threshold = 2.0_f64.powi(-60);
    let waveform = waveform(&[-1.0, large_endpoint], &[0.0, 1.0]);

    let crossings = waveform
        .find_crossings(threshold, EdgeDirection::Rising)
        .expect("representable cancellation residual must be recovered");
    assert_eq!(crossings.len(), 1);
    assert_eq!(crossings[0].time, threshold);
    assert!(
        crossings[0].slew_rate.is_finite() && crossings[0].slew_rate > 0.0,
        "large-span crossing slew must remain finite: {}",
        crossings[0].slew_rate
    );
    assert_eq!(
        waveform
            .first_crossing(threshold, EdgeDirection::Rising)
            .expect("first-crossing cancellation residual must be recovered")
            .expect("fixture has one crossing")
            .time,
        threshold
    );
    assert_eq!(
        waveform
            .cross_time(threshold, EdgeDirection::Rising, 0)
            .expect("ordinal cancellation residual must be recovered"),
        Some(threshold)
    );
}

#[test]
fn unrepresentable_interior_crossing_time_fails_closed() {
    let threshold = 2.0_f64.powi(-60);
    let waveform = waveform(&[1.0, 2.0], &[0.0, 1.0]);

    assert!(matches!(
        waveform.find_crossings(threshold, EdgeDirection::Rising),
        Err(MeasurementError::CalculationError(_))
    ));
    assert!(matches!(
        waveform.first_crossing(threshold, EdgeDirection::Rising),
        Err(MeasurementError::CalculationError(_))
    ));
    assert!(matches!(
        waveform.cross_time(threshold, EdgeDirection::Rising, 0),
        Err(MeasurementError::CalculationError(_))
    ));
}

#[test]
fn near_endpoint_extreme_crossing_is_exact_or_fails_closed() {
    let amplitude = 2.0_f64.powi(1023);
    let threshold = amplitude.next_down();
    let expected_time = 1.0_f64.next_down();
    let waveform = waveform(&[-1.0, 1.0], &[-amplitude, amplitude]);

    match waveform.find_crossings(threshold, EdgeDirection::Rising) {
        Ok(crossings) => {
            assert_eq!(crossings.len(), 1);
            assert_eq!(
                crossings[0].time, expected_time,
                "successful interpolation must not round the crossing onto the endpoint"
            );
            assert!(crossings[0].slew_rate.is_finite());
        }
        Err(MeasurementError::CalculationError(_)) => {}
        Err(error) => panic!("extreme crossing used the wrong error category: {error}"),
    }
    match waveform.first_crossing(threshold, EdgeDirection::Rising) {
        Ok(Some(event)) => assert_eq!(
            event.time, expected_time,
            "successful first crossing must retain the pre-endpoint time"
        ),
        Ok(None) => panic!("extreme fixture must not lose its crossing"),
        Err(MeasurementError::CalculationError(_)) => {}
        Err(error) => panic!("extreme first crossing used the wrong error category: {error}"),
    }
    match waveform.cross_time(threshold, EdgeDirection::Rising, 0) {
        Ok(Some(time)) => assert_eq!(
            time, expected_time,
            "successful ordinal crossing must retain the pre-endpoint time"
        ),
        Ok(None) => panic!("extreme fixture must not lose its ordinal crossing"),
        Err(MeasurementError::CalculationError(_)) => {}
        Err(error) => panic!("extreme ordinal crossing used the wrong error category: {error}"),
    }
}

#[test]
fn every_crossing_primitive_rejects_nonfinite_thresholds() {
    let waveform = waveform(&[0.0, 1.0], &[-1.0, 1.0]);

    for threshold in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(matches!(
            waveform.find_crossings(threshold, EdgeDirection::Either),
            Err(MeasurementError::InvalidThreshold(_))
        ));
        assert!(matches!(
            waveform.first_crossing(threshold, EdgeDirection::Either),
            Err(MeasurementError::InvalidThreshold(_))
        ));
        assert!(matches!(
            waveform.cross_time(threshold, EdgeDirection::Either, 0),
            Err(MeasurementError::InvalidThreshold(_))
        ));
    }
}

#[test]
fn crossings_are_chronological_directional_and_zero_based() {
    let waveform = waveform(&[0.0, 1.0, 2.0, 4.0, 5.0], &[-1.0, 1.0, -1.0, 1.0, -1.0]);
    let crossings = waveform
        .find_crossings(0.0, EdgeDirection::Either)
        .expect("alternating crossing record qualifies");

    assert_eq!(crossings.len(), 4);
    for (event, (time, index, direction, slew)) in crossings.iter().zip([
        (0.5, 0, EdgeDirection::Rising, 2.0),
        (1.5, 1, EdgeDirection::Falling, -2.0),
        (3.0, 2, EdgeDirection::Rising, 1.0),
        (4.5, 3, EdgeDirection::Falling, -2.0),
    ]) {
        assert_event(event, time, index, direction, slew, 0.0);
    }

    let rising = waveform
        .find_crossings(0.0, EdgeDirection::Rising)
        .expect("rising filter qualifies");
    let falling = waveform
        .find_crossings(0.0, EdgeDirection::Falling)
        .expect("falling filter qualifies");
    assert_eq!(
        rising.iter().map(|event| event.time).collect::<Vec<_>>(),
        vec![0.5, 3.0]
    );
    assert_eq!(
        falling.iter().map(|event| event.time).collect::<Vec<_>>(),
        vec![1.5, 4.5]
    );
    assert_eq!(
        waveform
            .first_crossing(0.0, EdgeDirection::Either)
            .expect("first crossing request qualifies")
            .expect("fixture has crossings")
            .time,
        0.5
    );
    for (ordinal, expected) in [0.5, 1.5, 3.0, 4.5].into_iter().enumerate() {
        assert_eq!(
            waveform
                .cross_time(0.0, EdgeDirection::Either, ordinal)
                .expect("ordinal crossing request qualifies"),
            Some(expected)
        );
    }
    assert_eq!(
        waveform
            .cross_time(0.0, EdgeDirection::Either, 4)
            .expect("missing ordinal request qualifies"),
        None
    );
}

#[test]
fn outside_range_and_constant_thresholds_produce_no_events() {
    let varying = waveform(&[0.0, 1.0, 2.0], &[0.0, 1.0, 0.0]);
    let constant = waveform(&[0.0, 1.0, 2.0], &[1.0, 1.0, 1.0]);

    for (waveform, threshold) in [(&varying, 2.0), (&constant, 1.0)] {
        assert!(
            waveform
                .find_crossings(threshold, EdgeDirection::Either)
                .expect("finite no-crossing request qualifies")
                .is_empty()
        );
        assert!(
            waveform
                .first_crossing(threshold, EdgeDirection::Either)
                .expect("finite first-crossing request qualifies")
                .is_none()
        );
        assert_eq!(
            waveform
                .cross_time(threshold, EdgeDirection::Either, 0)
                .expect("finite ordinal request qualifies"),
            None
        );
    }
}

#[test]
fn equality_plateaus_emit_one_event_at_the_arrival_endpoint() {
    let rising = waveform(&[0.0, 1.0, 2.0, 3.0], &[-1.0, 0.0, 0.0, 1.0]);
    let falling = waveform(&[0.0, 1.0, 2.0, 3.0], &[1.0, 0.0, 0.0, -1.0]);

    let rising_events = rising
        .find_crossings(0.0, EdgeDirection::Either)
        .expect("rising plateau qualifies");
    let falling_events = falling
        .find_crossings(0.0, EdgeDirection::Either)
        .expect("falling plateau qualifies");
    assert_eq!(rising_events.len(), 1);
    assert_eq!(falling_events.len(), 1);
    assert_event(&rising_events[0], 1.0, 0, EdgeDirection::Rising, 1.0, 0.0);
    assert_event(
        &falling_events[0],
        1.0,
        0,
        EdgeDirection::Falling,
        -1.0,
        0.0,
    );
}

#[test]
fn equality_touches_emit_only_the_arrival_direction() {
    let touches_from_below = waveform(&[0.0, 1.0, 2.0], &[-1.0, 0.0, -1.0]);
    let touches_from_above = waveform(&[0.0, 1.0, 2.0], &[1.0, 0.0, 1.0]);

    let below_events = touches_from_below
        .find_crossings(0.0, EdgeDirection::Either)
        .expect("touch from below qualifies");
    let above_events = touches_from_above
        .find_crossings(0.0, EdgeDirection::Either)
        .expect("touch from above qualifies");
    assert_eq!(below_events.len(), 1);
    assert_eq!(above_events.len(), 1);
    assert_event(&below_events[0], 1.0, 0, EdgeDirection::Rising, 1.0, 0.0);
    assert_event(&above_events[0], 1.0, 0, EdgeDirection::Falling, -1.0, 0.0);
    assert!(
        touches_from_below
            .find_crossings(0.0, EdgeDirection::Falling)
            .expect("departure filter qualifies")
            .is_empty()
    );
    assert!(
        touches_from_above
            .find_crossings(0.0, EdgeDirection::Rising)
            .expect("departure filter qualifies")
            .is_empty()
    );
}

#[test]
fn departing_an_initial_equality_is_not_a_crossing() {
    let departs_above = waveform(&[0.0, 1.0], &[0.0, 1.0]);
    let departs_below = waveform(&[0.0, 1.0], &[0.0, -1.0]);

    for waveform in [&departs_above, &departs_below] {
        assert!(
            waveform
                .find_crossings(0.0, EdgeDirection::Either)
                .expect("initial-equality departure qualifies")
                .is_empty()
        );
    }
}
