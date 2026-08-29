//! External public-API contracts for waveform interpolation and resampling.

use rspice_core::analysis::measurements::{MeasurementError, Waveform};

fn waveform(time: &[f64], values: &[f64]) -> Waveform {
    Waveform::new(time, values).expect("finite strictly increasing interpolation fixture is valid")
}

fn assert_samples(waveform: &Waveform, expected_time: &[f64], expected_values: &[f64]) {
    assert_eq!(waveform.len(), expected_time.len());
    assert_eq!(expected_time.len(), expected_values.len());
    for (index, (&time, &value)) in expected_time.iter().zip(expected_values).enumerate() {
        assert_eq!(waveform.time_at(index), Some(time));
        assert_eq!(waveform.value_at(index), Some(value));
    }
}

#[test]
fn interpolate_rejects_nonfinite_queries_and_distinguishes_finite_outside_range() {
    let waveform = waveform(&[0.0, 1.0], &[2.0, 4.0]);

    for query in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            waveform.interpolate(query).is_err(),
            "non-finite interpolation query must fail: {query}"
        );
    }
    assert_eq!(
        waveform.interpolate(-1.0).expect("finite query qualifies"),
        None
    );
    assert_eq!(
        waveform.interpolate(2.0).expect("finite query qualifies"),
        None
    );
}

#[test]
fn interpolate_preserves_exact_boundaries_and_nonuniform_linear_values() {
    let waveform = waveform(&[0.0, 1.0, 4.0], &[10.0, 12.0, 18.0]);

    assert_eq!(
        waveform.interpolate(0.0).expect("lower boundary qualifies"),
        Some(10.0)
    );
    assert_eq!(
        waveform.interpolate(4.0).expect("upper boundary qualifies"),
        Some(18.0)
    );
    assert_eq!(
        waveform
            .interpolate(1.0)
            .expect("authored interior sample qualifies"),
        Some(12.0)
    );
    assert_eq!(
        waveform.interpolate(2.5).expect("interior query qualifies"),
        Some(15.0)
    );
}

#[test]
fn interpolate_handles_extreme_finite_spans_and_values_without_overflow() {
    let waveform = waveform(&[-f64::MAX, f64::MAX], &[-f64::MAX, f64::MAX]);

    assert_eq!(
        waveform
            .interpolate(-f64::MAX)
            .expect("extreme lower endpoint qualifies"),
        Some(-f64::MAX)
    );
    assert_eq!(
        waveform
            .interpolate(0.0)
            .expect("extreme midpoint qualifies"),
        Some(0.0)
    );
    assert_eq!(
        waveform
            .interpolate(f64::MAX)
            .expect("extreme upper endpoint qualifies"),
        Some(f64::MAX)
    );
}

#[test]
fn interpolate_recovers_cancellation_and_representable_near_endpoint_values() {
    let cancellation = waveform(&[-1.0, 2.0_f64.powi(60)], &[-1.0, 2.0_f64.powi(60)]);
    let residual = 2.0_f64.powi(-60);
    assert_eq!(
        cancellation
            .interpolate(residual)
            .expect("cancellation query qualifies"),
        Some(residual)
    );

    let identity = waveform(&[0.0, 1.0], &[0.0, 1.0]);
    let near_endpoint = 1.0_f64.next_down();
    assert_eq!(
        identity
            .interpolate(near_endpoint)
            .expect("near-endpoint query qualifies"),
        Some(near_endpoint)
    );
}

#[test]
fn interpolate_fails_when_an_interior_value_rounds_onto_an_endpoint() {
    let waveform = waveform(&[0.0, 1.0], &[1.0, 2.0]);
    let query = 2.0_f64.powi(-60);

    assert!(matches!(
        waveform.interpolate(query),
        Err(MeasurementError::CalculationError(_))
    ));
}

#[test]
fn resample_rejects_invalid_counts_and_insufficient_source_range() {
    let source = waveform(&[0.0, 1.0], &[2.0, 4.0]);
    assert!(source.resample(0).is_err());
    assert!(source.resample(1).is_err());

    let one_point = waveform(&[3.0], &[7.0]);
    assert!(matches!(
        one_point.resample(2),
        Err(MeasurementError::InsufficientData(_))
    ));
}

#[test]
fn resample_includes_endpoints_and_propagates_every_linear_value() {
    let source = waveform(&[0.0, 1.0, 4.0], &[10.0, 12.0, 18.0]);
    let resampled = source
        .resample(5)
        .expect("representable resampling grid qualifies");

    assert_samples(
        &resampled,
        &[0.0, 1.0, 2.0, 3.0, 4.0],
        &[10.0, 12.0, 14.0, 16.0, 18.0],
    );
    assert!((0..resampled.len()).all(|index| resampled.value_at(index) != Some(0.0)));
}

#[test]
fn resample_handles_extreme_finite_endpoints_without_silent_fill() {
    let source = waveform(&[-f64::MAX, f64::MAX], &[-f64::MAX, f64::MAX]);
    let resampled = source
        .resample(3)
        .expect("extreme three-point grid is representable");

    assert_samples(
        &resampled,
        &[-f64::MAX, 0.0, f64::MAX],
        &[-f64::MAX, 0.0, f64::MAX],
    );
}

#[test]
fn resample_rejects_unrepresentable_steps_in_tiny_and_large_origin_ranges() {
    let minimum_subnormal = f64::from_bits(1);
    let tiny = waveform(&[0.0, minimum_subnormal], &[1.0, 2.0]);
    assert!(matches!(
        tiny.resample(3),
        Err(MeasurementError::CalculationError(_))
    ));

    let origin = 1.0e12;
    let eight_ulps = 0.000_976_562_5;
    let coarse = waveform(&[origin, origin + eight_ulps], &[1.0, 2.0]);
    assert!(matches!(
        coarse.resample(10),
        Err(MeasurementError::CalculationError(_))
    ));
}

#[test]
fn resample_rejects_impossible_capacity_before_allocation() {
    let waveform = waveform(&[0.0, 1.0], &[0.0, 1.0]);

    assert!(matches!(
        waveform.resample(usize::MAX),
        Err(MeasurementError::CalculationError(_))
    ));
}
