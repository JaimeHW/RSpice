//! External public-API contracts for aggregate waveform timing measurements.

use rspice_core::analysis::measurements::{EdgeDirection, MeasurementError, Waveform};

fn waveform(time: &[f64], values: &[f64]) -> Waveform {
    Waveform::new(time, values).expect("finite strictly increasing timing fixture is valid")
}

#[test]
fn rise_and_fall_pair_thresholds_on_the_same_completed_edge() {
    let rising = waveform(
        &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        &[0.0, 0.2, 0.0, 0.2, 1.0, 1.0],
    );
    let falling = waveform(
        &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        &[1.0, 0.8, 1.0, 0.8, 0.0, 0.0],
    );

    assert_eq!(
        rising
            .rise_time(0.1, 0.9)
            .expect("completed rising edge qualifies"),
        1.375
    );
    assert_eq!(
        falling
            .fall_time(0.9, 0.1)
            .expect("completed falling edge qualifies"),
        1.375
    );
}

#[test]
fn equality_return_invalidates_a_partially_armed_rise_or_fall() {
    let rising = waveform(&[0.0, 1.0, 2.0, 3.0], &[0.0, 0.2, 0.1, 1.0]);
    let falling = waveform(&[0.0, 1.0, 2.0, 3.0], &[1.0, 0.8, 0.9, 0.0]);

    assert!(matches!(
        rising.rise_time(0.1, 0.9),
        Err(MeasurementError::ThresholdNotCrossed(_))
    ));
    assert!(matches!(
        falling.fall_time(0.9, 0.1),
        Err(MeasurementError::ThresholdNotCrossed(_))
    ));
}

#[test]
fn rise_and_fall_reject_every_invalid_percentage_class() {
    let waveform = waveform(&[0.0, 1.0, 2.0], &[0.0, 1.0, 0.0]);
    let invalid_low_high = [
        (f64::NAN, 0.9),
        (0.1, f64::NAN),
        (f64::NEG_INFINITY, 0.9),
        (f64::INFINITY, 0.9),
        (0.1, f64::NEG_INFINITY),
        (0.1, f64::INFINITY),
        (-0.1, 0.9),
        (0.1, 1.1),
        (-0.2, -0.1),
        (1.1, 1.2),
        (0.5, 0.5),
        (0.9, 0.1),
    ];

    for (low, high) in invalid_low_high {
        assert!(matches!(
            waveform.rise_time(low, high),
            Err(MeasurementError::InvalidThreshold(_))
        ));
        assert!(matches!(
            waveform.fall_time(high, low),
            Err(MeasurementError::InvalidThreshold(_))
        ));
    }
}

#[test]
fn thresholds_within_one_segment_have_exact_rise_and_fall_times() {
    let rising = waveform(&[0.0, 4.0], &[0.0, 1.0]);
    let falling = waveform(&[0.0, 4.0], &[1.0, 0.0]);

    assert_eq!(
        rising
            .rise_time(0.25, 0.75)
            .expect("single-segment rising thresholds qualify"),
        2.0
    );
    assert_eq!(
        falling
            .fall_time(0.75, 0.25)
            .expect("single-segment falling thresholds qualify"),
        2.0
    );
}

#[test]
fn rise_and_fall_construct_levels_safely_across_maximum_finite_span() {
    let rising = waveform(&[0.0, 2.0], &[-f64::MAX, f64::MAX]);
    let falling = waveform(&[0.0, 2.0], &[f64::MAX, -f64::MAX]);

    assert_eq!(
        rising
            .rise_time(0.25, 0.75)
            .expect("extreme rising levels remain representable"),
        1.0
    );
    assert_eq!(
        falling
            .fall_time(0.75, 0.25)
            .expect("extreme falling levels remain representable"),
        1.0
    );
}

#[test]
fn start_high_multicycle_record_uses_first_complete_pulse() {
    let time: Vec<_> = (0..=8).map(|index| index as f64).collect();
    let waveform = waveform(&time, &[1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0]);

    assert_eq!(
        waveform
            .pulse_width(0.5)
            .expect("first complete pulse qualifies"),
        2.0
    );
    assert_eq!(
        waveform.period(0.5).expect("two complete cycles qualify"),
        4.0
    );
    assert_eq!(
        waveform
            .frequency(0.5)
            .expect("finite period defines frequency"),
        0.25
    );
    let duty = waveform
        .duty_cycle(0.5)
        .expect("complete pulse and period define duty cycle");
    assert_eq!(duty, 50.0);
    assert!((0.0..=100.0).contains(&duty));
}

#[test]
fn equality_departure_cannot_leave_stale_duty_cycle_evidence() {
    let time: Vec<_> = (0..=5).map(|index| index as f64).collect();
    let waveform = waveform(&time, &[0.0, 1.0, 0.5, 1.0, 0.0, 1.0]);

    assert!(matches!(
        waveform.duty_cycle(0.5),
        Err(MeasurementError::ThresholdNotCrossed(_))
    ));
}

#[test]
fn incomplete_boundary_pulses_and_cycles_fail_closed() {
    let boundary_fragments = [
        waveform(&[0.0, 1.0, 2.0, 3.0], &[1.0, 0.0, 0.0, 1.0]),
        waveform(&[0.0, 1.0, 2.0], &[0.0, 1.0, 1.0]),
        waveform(&[0.0, 1.0, 2.0], &[1.0, 0.0, 0.0]),
    ];

    for fragment in boundary_fragments {
        assert!(matches!(
            fragment.pulse_width(0.5),
            Err(MeasurementError::ThresholdNotCrossed(_))
        ));
        assert!(matches!(
            fragment.period(0.5),
            Err(MeasurementError::ThresholdNotCrossed(_))
        ));
        assert!(matches!(
            fragment.frequency(0.5),
            Err(MeasurementError::ThresholdNotCrossed(_))
        ));
        assert!(matches!(
            fragment.duty_cycle(0.5),
            Err(MeasurementError::ThresholdNotCrossed(_))
        ));
    }
}

#[test]
fn global_slew_handles_sub_femtosecond_and_extreme_finite_segments() {
    let short_interval = 2.0_f64.powi(-60);
    let small_step = 2.0_f64.powi(-40);
    let sub_femtosecond = waveform(&[0.0, short_interval, 2.0], &[0.0, small_step, small_step]);
    let extreme = waveform(&[0.0, 2.0], &[-f64::MAX, f64::MAX]);

    assert_eq!(
        sub_femtosecond
            .slew_rate()
            .expect("sub-femtosecond finite slew qualifies"),
        2.0_f64.powi(20)
    );
    assert_eq!(
        extreme.slew_rate().expect("extreme finite slew qualifies"),
        f64::MAX
    );
}

#[test]
fn unrepresentable_period_can_have_a_representable_frequency() {
    let overflowing_period = waveform(&[-1.5e308, -1.0e308, 0.0, 1.0e308], &[-1.0, 0.0, -1.0, 0.0]);

    assert!(matches!(
        overflowing_period.period(0.0),
        Err(MeasurementError::CalculationError(_))
    ));
    assert_eq!(
        overflowing_period
            .frequency(0.0)
            .expect("the reciprocal of the unrepresentable period is finite"),
        0.5 / 1.0e308
    );

    let overflowing_width = waveform(&[-1.5e308, -1.0e308, 0.0, 1.0e308], &[-1.0, 0.0, 1.0, 0.0]);
    assert!(matches!(
        overflowing_width.pulse_width(0.0),
        Err(MeasurementError::CalculationError(_))
    ));

    let min_subnormal = f64::from_bits(1);
    let reciprocal_overflow = waveform(
        &[
            0.0,
            2.0 * min_subnormal,
            4.0 * min_subnormal,
            6.0 * min_subnormal,
        ],
        &[-min_subnormal, 0.0, -min_subnormal, 0.0],
    );
    assert_eq!(
        reciprocal_overflow
            .period(0.0)
            .expect("the subnormal period itself is representable"),
        4.0 * min_subnormal
    );
    assert!(matches!(
        reciprocal_overflow.frequency(0.0),
        Err(MeasurementError::CalculationError(_))
    ));
}

#[test]
fn duty_cycle_applies_percentage_scaling_before_subnormal_rounding() {
    let minimum_subnormal = f64::from_bits(1);
    let waveform = waveform(
        &[
            0.0,
            minimum_subnormal,
            2.0 * minimum_subnormal,
            3.0 * minimum_subnormal,
            4.0 * minimum_subnormal,
            64.0,
        ],
        &[
            -minimum_subnormal,
            0.0,
            minimum_subnormal,
            0.0,
            -64.0 * minimum_subnormal,
            0.0,
        ],
    );

    assert_eq!(
        waveform
            .duty_cycle(0.0)
            .expect("percentage scaling makes the tiny duty cycle representable"),
        3.0 * minimum_subnormal
    );
}

#[test]
fn delay_is_signed_and_overflow_fails_closed() {
    let reference = waveform(&[0.0, 1.0], &[-1.0, 1.0]);
    let target = waveform(&[0.0, 2.0], &[-1.0, 1.0]);
    assert_eq!(
        target
            .delay(&reference, 0.0, 0.0, EdgeDirection::Rising)
            .expect("positive delay qualifies"),
        0.5
    );
    assert_eq!(
        reference
            .delay(&target, 0.0, 0.0, EdgeDirection::Rising)
            .expect("negative delay qualifies"),
        -0.5
    );

    let very_early = waveform(&[-1.5e308, -1.0e308], &[-1.0, 0.0]);
    let very_late = waveform(&[1.0e308, 1.5e308], &[-1.0, 0.0]);
    assert!(matches!(
        very_late.delay(&very_early, 0.0, 0.0, EdgeDirection::Rising),
        Err(MeasurementError::CalculationError(_))
    ));
}
