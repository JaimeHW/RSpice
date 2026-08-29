//! External public-API contracts for derived scalar waveform measurements.

use rspice_core::analysis::measurements::{MeasurementError, Waveform};

fn waveform(time: &[f64], values: &[f64]) -> Waveform {
    Waveform::new(time, values).expect("finite strictly increasing scalar fixture is valid")
}

fn assert_positive_zero(value: f64) {
    assert_eq!(value.to_bits(), 0, "expected positive zero, got {value:?}");
}

fn assert_finite_exact(value: f64, expected: f64) {
    assert!(value.is_finite(), "derived scalar is non-finite: {value}");
    assert_eq!(value, expected);
}

#[test]
fn single_point_and_constant_waveforms_have_explicit_zero_contracts() {
    let single = waveform(&[3.0], &[f64::MAX]);
    assert_positive_zero(
        single
            .duration()
            .expect("one-point duration is exactly zero"),
    );
    assert!(matches!(
        single.sample_rate(),
        Err(MeasurementError::InsufficientData(_))
    ));
    assert_positive_zero(
        single
            .peak_to_peak()
            .expect("one-point peak-to-peak is exactly zero"),
    );
    assert!(matches!(
        single.overshoot(),
        Err(MeasurementError::InsufficientData(_))
    ));
    assert!(matches!(
        single.undershoot(),
        Err(MeasurementError::InsufficientData(_))
    ));

    let constant = waveform(&[0.0, 1.0, 2.0], &[-0.0, 0.0, -0.0]);
    assert_positive_zero(
        constant
            .peak_to_peak()
            .expect("signed-zero waveform has zero span"),
    );
    assert!(matches!(
        constant.overshoot(),
        Err(MeasurementError::CalculationError(_))
    ));
    assert!(matches!(
        constant.undershoot(),
        Err(MeasurementError::CalculationError(_))
    ));

    let returning_excursion = waveform(&[0.0, 1.0, 2.0], &[0.0, 1.0, 0.0]);
    assert!(matches!(
        returning_excursion.overshoot(),
        Err(MeasurementError::CalculationError(_))
    ));
    assert!(matches!(
        returning_excursion.undershoot(),
        Err(MeasurementError::CalculationError(_))
    ));

    let constant_max = waveform(&[0.0, 1.0, 2.0], &[f64::MAX; 3]);
    assert_positive_zero(
        constant_max
            .peak_to_peak()
            .expect("constant maximum-finite waveform has zero span"),
    );
    assert!(matches!(
        constant_max.overshoot(),
        Err(MeasurementError::CalculationError(_))
    ));
    assert!(matches!(
        constant_max.undershoot(),
        Err(MeasurementError::CalculationError(_))
    ));
}

#[test]
fn nonuniform_sample_rate_is_interval_count_over_total_duration() {
    let waveform = waveform(&[0.0, 0.25, 1.0, 4.0], &[0.0; 4]);

    assert_finite_exact(waveform.duration().expect("finite duration qualifies"), 4.0);
    assert_finite_exact(
        waveform
            .sample_rate()
            .expect("nonuniform average sample rate qualifies"),
        0.75,
    );
}

#[test]
fn maximum_finite_duration_and_its_subnormal_sample_rate_are_preserved() {
    let waveform = waveform(&[0.0, f64::MAX], &[0.0, 0.0]);

    assert_finite_exact(
        waveform
            .duration()
            .expect("maximum finite duration is representable"),
        f64::MAX,
    );
    let sample_rate = waveform
        .sample_rate()
        .expect("reciprocal maximum duration is representable");
    assert!(sample_rate.is_finite() && sample_rate > 0.0);
    assert_eq!(sample_rate, 1.0 / f64::MAX);
}

#[test]
fn unrepresentable_duration_or_sample_rate_fails_closed() {
    let overflowing_duration = waveform(&[-f64::MAX, f64::MAX], &[0.0, 0.0]);
    assert!(matches!(
        overflowing_duration.duration(),
        Err(MeasurementError::CalculationError(_))
    ));
    let scaled_rate = overflowing_duration
        .sample_rate()
        .expect("the reciprocal overflowing span remains representable");
    assert_finite_exact(scaled_rate, 0.5 / f64::MAX);

    let minimum_subnormal = f64::from_bits(1);
    let overflowing_rate = waveform(&[0.0, minimum_subnormal], &[0.0, 0.0]);
    assert_eq!(
        overflowing_rate
            .duration()
            .expect("minimum subnormal duration is representable"),
        minimum_subnormal
    );
    assert!(matches!(
        overflowing_rate.sample_rate(),
        Err(MeasurementError::CalculationError(_))
    ));
}

#[test]
fn peak_to_peak_preserves_representable_extreme_and_subnormal_spans() {
    let maximum_span = waveform(&[0.0, 1.0], &[-f64::MAX / 2.0, f64::MAX / 2.0]);
    assert_finite_exact(
        maximum_span
            .peak_to_peak()
            .expect("maximum finite value span is representable"),
        f64::MAX,
    );

    let minimum_subnormal = f64::from_bits(1);
    let tiny_span = waveform(&[0.0, 1.0], &[0.0, minimum_subnormal]);
    assert_eq!(
        tiny_span
            .peak_to_peak()
            .expect("minimum subnormal value span is representable"),
        minimum_subnormal
    );

    let overflowing_span = waveform(&[0.0, 1.0], &[-f64::MAX, f64::MAX]);
    assert!(matches!(
        overflowing_span.peak_to_peak(),
        Err(MeasurementError::CalculationError(_))
    ));
}

#[test]
fn rising_and_falling_excursion_percentages_are_exact() {
    let rising = waveform(&[0.0, 1.0, 2.0, 3.0], &[0.0, -0.25, 1.25, 1.0]);
    let falling = waveform(&[0.0, 1.0, 2.0, 3.0], &[1.0, 1.25, -0.25, 0.0]);

    for value in [
        rising.overshoot().expect("rising overshoot qualifies"),
        rising.undershoot().expect("rising undershoot qualifies"),
        falling.overshoot().expect("falling overshoot qualifies"),
        falling.undershoot().expect("falling undershoot qualifies"),
    ] {
        assert_finite_exact(value, 25.0);
    }
}

#[test]
fn excursion_percentages_handle_an_unmaterializable_opposite_sign_step() {
    let rising = waveform(&[0.0, 1.0, 2.0], &[-f64::MAX, f64::MAX, f64::MAX / 2.0]);
    let falling = waveform(&[0.0, 1.0, 2.0], &[f64::MAX, -f64::MAX, -f64::MAX / 2.0]);
    let expected = 100.0 / 3.0;

    for value in [
        rising
            .overshoot()
            .expect("opposite-sign rising step qualifies"),
        falling
            .overshoot()
            .expect("opposite-sign falling step qualifies"),
    ] {
        assert!(value.is_finite());
        assert!((value - expected).abs() <= 1.0e-14, "got {value}");
    }
}

#[test]
fn two_sample_nonzero_steps_have_exactly_zero_excursions() {
    let rising = waveform(&[0.0, 1.0], &[0.0, 1.0]);
    let falling = waveform(&[0.0, 1.0], &[1.0, 0.0]);

    for value in [
        rising
            .overshoot()
            .expect("two-point rising overshoot is defined"),
        rising
            .undershoot()
            .expect("two-point rising undershoot is defined"),
        falling
            .overshoot()
            .expect("two-point falling overshoot is defined"),
        falling
            .undershoot()
            .expect("two-point falling undershoot is defined"),
    ] {
        assert_positive_zero(value);
    }
}

#[test]
fn excursion_percentages_are_scale_invariant_for_subnormal_steps() {
    let minimum_subnormal = f64::from_bits(1);
    let step = 16.0 * minimum_subnormal;
    let quarter = 4.0 * minimum_subnormal;
    let five_quarters = 20.0 * minimum_subnormal;
    let rising = waveform(&[0.0, 1.0, 2.0, 3.0], &[0.0, -quarter, five_quarters, step]);
    let falling = waveform(&[0.0, 1.0, 2.0, 3.0], &[step, five_quarters, -quarter, 0.0]);

    for value in [
        rising
            .overshoot()
            .expect("subnormal rising overshoot qualifies"),
        rising
            .undershoot()
            .expect("subnormal rising undershoot qualifies"),
        falling
            .overshoot()
            .expect("subnormal falling overshoot qualifies"),
        falling
            .undershoot()
            .expect("subnormal falling undershoot qualifies"),
    ] {
        assert_finite_exact(value, 25.0);
    }
}

#[test]
fn percentage_scaling_rescues_a_representable_subnormal_excursion() {
    let minimum_subnormal = f64::from_bits(1);
    let waveform = waveform(&[0.0, 1.0, 2.0], &[0.0, -minimum_subnormal, 64.0]);

    assert_eq!(
        waveform
            .undershoot()
            .expect("percentage scaling makes the tiny undershoot representable"),
        2.0 * minimum_subnormal
    );
}

#[test]
fn unrepresentable_excursion_percentages_fail_closed() {
    let minimum_subnormal = f64::from_bits(1);
    let overshoot = waveform(&[0.0, 1.0, 2.0], &[0.0, f64::MAX, minimum_subnormal]);
    let undershoot = waveform(&[0.0, 1.0, 2.0], &[0.0, -f64::MAX, minimum_subnormal]);

    assert!(matches!(
        overshoot.overshoot(),
        Err(MeasurementError::CalculationError(_))
    ));
    assert!(matches!(
        undershoot.undershoot(),
        Err(MeasurementError::CalculationError(_))
    ));
}
