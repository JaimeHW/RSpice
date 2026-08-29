//! External public-API contracts for validated waveform construction and copying.

use rspice_core::analysis::measurements::{MeasurementError, Waveform};

fn assert_both_constructors_reject(time: &[f64], values: &[f64]) {
    for result in [Waveform::new(time, values), Waveform::try_new(time, values)] {
        assert!(
            matches!(result, Err(MeasurementError::InvalidWaveform(_))),
            "invalid fixture unexpectedly qualified"
        );
    }
}

#[test]
fn constructors_reject_empty_mismatched_and_unordered_inputs() {
    assert_both_constructors_reject(&[], &[]);
    assert_both_constructors_reject(&[0.0, 1.0], &[0.0]);
    assert_both_constructors_reject(&[0.0], &[0.0, 1.0]);
    assert_both_constructors_reject(&[0.0, 1.0, 1.0], &[0.0, 1.0, 2.0]);
    assert_both_constructors_reject(&[0.0, 2.0, 1.0], &[0.0, 1.0, 2.0]);
}

#[test]
fn constructors_reject_every_nonfinite_timestamp_and_value_class() {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_both_constructors_reject(&[0.0, invalid], &[1.0, 2.0]);
        assert_both_constructors_reject(&[0.0, 1.0], &[1.0, invalid]);
    }
}

#[test]
fn one_point_construction_preserves_exact_signed_bits() {
    for constructor in [Waveform::new, Waveform::try_new] {
        let waveform = constructor(&[-0.0], &[-0.0]).expect("one finite sample is valid");
        assert_eq!(waveform.len(), 1);
        assert!(!waveform.is_empty());
        assert_eq!(
            waveform.time_at(0).map(f64::to_bits),
            Some((-0.0_f64).to_bits())
        );
        assert_eq!(
            waveform.value_at(0).map(f64::to_bits),
            Some((-0.0_f64).to_bits())
        );
        assert_eq!(waveform.min().to_bits(), (-0.0_f64).to_bits());
        assert_eq!(waveform.max().to_bits(), (-0.0_f64).to_bits());
        assert_eq!(
            waveform
                .duration()
                .expect("one-point duration is exactly positive zero")
                .to_bits(),
            0.0_f64.to_bits()
        );
    }
}

#[test]
fn constructors_own_exact_copies_with_matching_extrema_and_duration() {
    let mut time = vec![-0.0, 1.0, 3.0];
    let mut values = vec![-4.0, -0.0, 2.0];
    let from_new = Waveform::new(&time, &values).expect("new copy source is valid");
    let from_try_new = Waveform::try_new(&time, &values).expect("try_new copy source is valid");

    time.fill(10.0);
    values.fill(10.0);

    for copied in [&from_new, &from_try_new] {
        for (index, expected) in [-0.0_f64, 1.0, 3.0].into_iter().enumerate() {
            assert_eq!(
                copied.time_at(index).map(f64::to_bits),
                Some(expected.to_bits())
            );
        }
        for (index, expected) in [-4.0_f64, -0.0, 2.0].into_iter().enumerate() {
            assert_eq!(
                copied.value_at(index).map(f64::to_bits),
                Some(expected.to_bits())
            );
        }
        assert_eq!(copied.min(), -4.0);
        assert_eq!(copied.max(), 2.0);
        assert_eq!(copied.duration().expect("copied duration qualifies"), 3.0);
        assert_eq!(copied.time_at(usize::MAX), None);
        assert_eq!(copied.value_at(usize::MAX), None);
    }
}
