//! External public-API contracts for time-weighted waveform statistics.

use rspice_core::analysis::measurements::{MeasurementError, Waveform};

fn waveform(time: &[f64], values: &[f64]) -> Waveform {
    Waveform::new(time, values).expect("finite strictly increasing statistics fixture is valid")
}

fn assert_positive_relative(actual: f64, expected: f64, relative_tolerance: f64) {
    assert!(actual.is_finite(), "actual result is non-finite: {actual}");
    assert!(actual > 0.0 && expected > 0.0);
    let ratio = actual / expected;
    assert!(
        (ratio - 1.0).abs() <= relative_tolerance,
        "actual={actual:.17e}, expected={expected:.17e}, ratio={ratio:.17e}, relative tolerance={relative_tolerance:.3e}"
    );
}

#[test]
fn average_and_rms_use_nonuniform_trapezoidal_time_weights() {
    // Only the final one-second interval ramps from zero to two. Over the
    // ten-second record, trapezoidal integral(v)/T = 0.1 and
    // trapezoidal integral(v^2)/T = 0.2.
    let waveform = waveform(&[0.0, 9.0, 10.0], &[0.0, 0.0, 2.0]);

    let average = waveform.average().expect("finite time average is defined");
    let rms = waveform.rms().expect("finite time RMS is defined");
    assert_positive_relative(average, 0.1, 32.0 * f64::EPSILON);
    assert_positive_relative(rms, 0.2_f64.sqrt(), 32.0 * f64::EPSILON);
}

#[test]
fn statistics_are_invariant_to_extra_samples_on_a_flat_segment() {
    let sparse = waveform(&[0.0, 1.0, 3.0], &[0.0, 2.0, 2.0]);
    let dense = waveform(
        &[0.0, 1.0, 1.25, 1.5, 2.0, 2.5, 3.0],
        &[0.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0],
    );
    let expected_average = 5.0 / 3.0;
    let expected_rms = (10.0_f64 / 3.0).sqrt();

    let sparse_average = sparse.average().expect("sparse average is defined");
    let dense_average = dense.average().expect("dense average is defined");
    let sparse_rms = sparse.rms().expect("sparse RMS is defined");
    let dense_rms = dense.rms().expect("dense RMS is defined");
    for actual in [sparse_average, dense_average] {
        assert_positive_relative(actual, expected_average, 32.0 * f64::EPSILON);
    }
    for actual in [sparse_rms, dense_rms] {
        assert_positive_relative(actual, expected_rms, 32.0 * f64::EPSILON);
    }
}

#[test]
fn constant_maximum_finite_waveform_has_finite_scale_safe_statistics() {
    let waveform = waveform(&[0.0, 0.25, 1.0], &[f64::MAX; 3]);

    let average = waveform
        .average()
        .expect("constant maximum finite average is representable");
    let rms = waveform
        .rms()
        .expect("constant maximum finite RMS is representable");
    assert_positive_relative(average, f64::MAX, 16.0 * f64::EPSILON);
    assert_positive_relative(rms, f64::MAX, 16.0 * f64::EPSILON);
}

#[test]
fn constant_smallest_normal_waveform_rms_does_not_underflow() {
    let waveform = waveform(&[0.0, 0.25, 1.0], &[f64::MIN_POSITIVE; 3]);

    let rms = waveform
        .rms()
        .expect("constant minimum-normal RMS is representable");
    assert!(rms > 0.0, "a nonzero RMS must not be clamped to zero");
    assert_positive_relative(rms, f64::MIN_POSITIVE, 16.0 * f64::EPSILON);
}

#[test]
fn average_preserves_a_small_residual_after_near_exact_cancellation() {
    let waveform = waveform(
        &[0.0, 1.0, 2.0, 3.0],
        &[1.0, 1.0, (-1.0_f64).next_up(), -1.0],
    );
    let expected = 2.0_f64.powi(-53) / 3.0;

    let average = waveform
        .average()
        .expect("the representable cancellation residual must be recovered");
    assert!(
        average > 0.0,
        "the representable positive residual must not be cancelled to zero"
    );
    assert_positive_relative(average, expected, 16.0 * f64::EPSILON);
}

#[test]
fn average_preserves_an_exactly_symmetric_zero() {
    let symmetric = waveform(&[0.0, 1.0, 2.0, 3.0], &[1.0, 1.0, -1.0, -1.0]);

    assert_eq!(
        symmetric
            .average()
            .expect("exactly symmetric cancellation is representable"),
        0.0
    );

    let zero = waveform(&[0.0, 1.0], &[0.0, 0.0]);
    assert_eq!(zero.average().expect("zero average is exact"), 0.0);
    assert_eq!(zero.rms().expect("zero RMS is exact"), 0.0);
}

#[test]
fn average_recovers_or_rejects_an_exact_subnormal_residual() {
    let minimum_subnormal = f64::from_bits(1);
    let waveform = waveform(
        &[0.0, 1.0, 2.0, 3.0],
        &[1.0, -0.5, 3.0 * minimum_subnormal, 0.0],
    );

    match waveform.average() {
        Ok(average) => assert_eq!(
            average, minimum_subnormal,
            "a successful result must preserve the exact subnormal residual"
        ),
        Err(MeasurementError::CalculationError(_)) => {
            // This is the only acceptable fail-closed outcome when the exact
            // subnormal residual cannot be certified.
        }
        Err(error) => panic!("subnormal cancellation used the wrong error category: {error}"),
    }
}

#[test]
fn rms_recovers_or_rejects_an_unrepresentable_subnormal_mean_square() {
    let minimum_subnormal = f64::from_bits(1);
    let waveform = waveform(
        &[
            0.0,
            3.0 * minimum_subnormal,
            6.0 * minimum_subnormal,
            9.0 * minimum_subnormal,
            1.0,
        ],
        &[1.0, 0.0, 1.0, 0.0, 0.0],
    );

    match waveform.rms() {
        Ok(rms) => {
            assert!(rms.is_finite() && rms > 0.0);
            // The exact mean square is 4.5*m, which is not itself an f64.
            // Normalize before squaring so the test does not underflow its
            // own oracle: (rms / sqrt(m))^2 must recover the exact factor 4.5.
            let normalized = rms / minimum_subnormal.sqrt();
            assert_positive_relative(normalized * normalized, 4.5, 64.0 * f64::EPSILON);
        }
        Err(MeasurementError::CalculationError(_)) => {
            // Failing closed is valid when the subnormal mean-square evidence
            // cannot be represented without changing the certified result.
        }
        Err(error) => panic!("subnormal RMS used the wrong error category: {error}"),
    }
}

#[test]
fn one_point_waveform_statistics_report_insufficient_data() {
    let waveform = waveform(&[2.0], &[3.0]);

    assert!(matches!(
        waveform.average(),
        Err(MeasurementError::InsufficientData(_))
    ));
    assert!(matches!(
        waveform.rms(),
        Err(MeasurementError::InsufficientData(_))
    ));
}

#[test]
fn statistics_normalize_an_unrepresentable_total_time_span() {
    let constant = waveform(&[-f64::MAX, f64::MAX], &[1.0, 1.0]);
    assert_eq!(
        constant
            .average()
            .expect("constant average does not require a materialized duration"),
        1.0
    );
    assert_eq!(
        constant
            .rms()
            .expect("constant RMS does not require a materialized duration"),
        1.0
    );

    let triangular = waveform(&[-f64::MAX, 0.0, f64::MAX], &[0.0, 1.0, 0.0]);
    assert_eq!(
        triangular
            .average()
            .expect("scaled interval weights define the extreme-span average"),
        0.5
    );
    assert_eq!(
        triangular
            .rms()
            .expect("scaled interval weights define the extreme-span RMS"),
        0.5_f64.sqrt()
    );

    let unequal = waveform(&[-f64::MAX, -f64::MAX / 2.0, f64::MAX], &[0.0, 0.0, 8.0]);
    assert_eq!(
        unequal
            .average()
            .expect("unequal scaled interval weights define the average"),
        3.0
    );
    assert_eq!(
        unequal
            .rms()
            .expect("unequal scaled interval weights define the RMS"),
        24.0_f64.sqrt()
    );
}
