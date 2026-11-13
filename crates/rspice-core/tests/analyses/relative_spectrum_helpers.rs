//! Public fail-closed contracts for relative Fourier-spectrum helpers.

use rspice_core::analysis::fourier::{FourierError, FourierResult, HarmonicComponent};

fn harmonic(magnitude: f64) -> HarmonicComponent {
    HarmonicComponent {
        harmonic_number: 2,
        frequency: 2.0e3,
        magnitude,
        phase: 0.0,
    }
}

fn result_with_thd(thd: Option<f64>) -> FourierResult {
    FourierResult {
        fundamental_freq: 1.0e3,
        dc_component: 0.0,
        harmonics: Vec::new(),
        thd,
    }
}

fn assert_close(actual: f64, expected: f64) {
    let scale = actual.abs().max(expected.abs()).max(1.0);
    assert!(
        (actual - expected).abs() <= 8.0 * f64::EPSILON * scale,
        "actual={actual:.17e}, expected={expected:.17e}"
    );
}

#[test]
fn normalized_and_db_preserve_ratios_without_an_absolute_cutoff() {
    for (magnitude, reference, expected_percent) in [
        (1.0, 1.0, 100.0),
        (0.5, 2.0, 25.0),
        (0.5e-20, 2.0e-20, 25.0),
    ] {
        let component = harmonic(magnitude);
        assert_close(
            component
                .normalized(reference)
                .expect("valid magnitudes qualify")
                .expect("nonzero reference defines a ratio"),
            expected_percent,
        );
        assert_close(
            component
                .db(reference)
                .expect("valid magnitudes qualify")
                .expect("nonzero reference defines a ratio"),
            20.0 * (magnitude.log10() - reference.log10()),
        );
    }
}

#[test]
fn zero_numerator_and_zero_reference_have_distinct_semantics() {
    let zero = harmonic(0.0);
    assert_eq!(zero.normalized(2.0), Ok(Some(0.0)));
    let zero_db = zero
        .db(2.0)
        .expect("zero is a valid numerator")
        .expect("nonzero reference defines a dB result");
    assert!(zero_db == f64::NEG_INFINITY);

    let nonzero = harmonic(1.0);
    assert_eq!(nonzero.normalized(0.0), Ok(None));
    assert_eq!(nonzero.db(0.0), Ok(None));
}

#[test]
fn malformed_component_and_reference_magnitudes_are_typed_errors() {
    for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let component = harmonic(invalid);
        assert!(matches!(
            component.normalized(1.0),
            Err(FourierError::InvalidMagnitude {
                role: "harmonic magnitude",
                ..
            })
        ));
        assert!(matches!(
            component.db(1.0),
            Err(FourierError::InvalidMagnitude {
                role: "harmonic magnitude",
                ..
            })
        ));

        let component = harmonic(1.0);
        assert!(matches!(
            component.normalized(invalid),
            Err(FourierError::InvalidMagnitude {
                role: "fundamental reference magnitude",
                ..
            })
        ));
        assert!(matches!(
            component.db(invalid),
            Err(FourierError::InvalidMagnitude {
                role: "fundamental reference magnitude",
                ..
            })
        ));
    }
}

#[test]
fn normalized_overflow_fails_closed_with_a_specific_error() {
    assert!(matches!(
        harmonic(f64::MAX).normalized(f64::MIN_POSITIVE),
        Err(FourierError::UnrepresentableRelativeSpectrum {
            quantity: "normalized harmonic magnitude"
        })
    ));
}

#[test]
fn thd_db_distinguishes_defined_zero_from_undefined() {
    assert_close(
        result_with_thd(Some(25.0))
            .thd_db()
            .expect("valid THD qualifies")
            .expect("defined THD has a dB result"),
        20.0 * 0.25_f64.log10(),
    );

    let zero_db = result_with_thd(Some(0.0))
        .thd_db()
        .expect("defined zero-percent THD qualifies")
        .expect("defined zero-percent THD has an exact logarithmic result");
    assert!(zero_db == f64::NEG_INFINITY);
    assert_eq!(result_with_thd(None).thd_db(), Ok(None));
}

#[test]
fn malformed_retained_thd_is_a_typed_error() {
    for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(matches!(
            result_with_thd(Some(invalid)).thd_db(),
            Err(FourierError::InvalidThd { .. })
        ));
    }
}
