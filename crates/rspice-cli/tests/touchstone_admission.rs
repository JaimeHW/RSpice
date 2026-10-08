//! Shared RF serialization must not publish malformed or lossy numeric records.
use rspice_core::Complex64;
use rspice_core::analysis::s_param::{
    TouchstoneFormat, TouchstoneFrequencyUnit, TouchstoneInput, TouchstoneVersion,
    touchstone_with_version,
};

fn encode(
    frequencies: &[f64],
    samples: &[Complex64],
    reference: f64,
    unit: TouchstoneFrequencyUnit,
    format: TouchstoneFormat,
    version: TouchstoneVersion,
) -> Result<String, String> {
    touchstone_with_version(
        &TouchstoneInput {
            frequencies,
            parameters: &[vec![samples.to_vec()]],
            reference_impedances: &[reference],
            comments: &[],
        },
        format,
        unit,
        version,
    )
}

#[test]
fn malformed_numeric_networks_are_refused_by_both_versions() {
    for version in [TouchstoneVersion::V1, TouchstoneVersion::V2] {
        for frequencies in [
            vec![],
            vec![f64::NAN],
            vec![f64::INFINITY],
            vec![-1.0],
            vec![1.0, 1.0],
            vec![2.0, 1.0],
        ] {
            let samples = vec![Complex64::new(0.5, 0.0); frequencies.len()];
            assert!(
                encode(
                    &frequencies,
                    &samples,
                    50.0,
                    TouchstoneFrequencyUnit::Hz,
                    TouchstoneFormat::RealImaginary,
                    version
                )
                .is_err(),
                "{version:?}: {frequencies:?}"
            );
        }
        for reference in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(
                encode(
                    &[1.0],
                    &[Complex64::new(0.5, 0.0)],
                    reference,
                    TouchstoneFrequencyUnit::Hz,
                    TouchstoneFormat::RealImaginary,
                    version
                )
                .is_err(),
                "{version:?}: {reference}"
            );
        }
        for sample in [
            Complex64::new(f64::NAN, 0.0),
            Complex64::new(0.0, f64::INFINITY),
        ] {
            assert!(
                encode(
                    &[1.0],
                    &[sample],
                    50.0,
                    TouchstoneFrequencyUnit::Hz,
                    TouchstoneFormat::RealImaginary,
                    version
                )
                .is_err(),
                "{version:?}: {sample}"
            );
        }
    }
}

#[test]
fn output_units_cannot_round_a_positive_frequency_to_dc() {
    for version in [TouchstoneVersion::V1, TouchstoneVersion::V2] {
        let frequency = f64::from_bits(1);
        let sample = Complex64::new(0.5, 0.0);
        assert!(
            encode(
                &[frequency],
                &[sample],
                50.0,
                TouchstoneFrequencyUnit::GHz,
                TouchstoneFormat::RealImaginary,
                version
            )
            .is_err()
        );
        assert!(
            encode(
                &[frequency],
                &[sample],
                50.0,
                TouchstoneFrequencyUnit::Hz,
                TouchstoneFormat::RealImaginary,
                version
            )
            .is_ok()
        );
        assert!(
            encode(
                &[0.0, 1.0],
                &[sample, sample],
                50.0,
                TouchstoneFrequencyUnit::Hz,
                TouchstoneFormat::RealImaginary,
                version
            )
            .is_ok()
        );
    }
}

#[test]
fn magnitude_overflow_cannot_publish_infinity() {
    let sample = Complex64::new(f64::MAX, f64::MAX);
    for version in [TouchstoneVersion::V1, TouchstoneVersion::V2] {
        assert!(
            encode(
                &[1.0],
                &[sample],
                50.0,
                TouchstoneFrequencyUnit::Hz,
                TouchstoneFormat::MagnitudeAngle,
                version
            )
            .is_err()
        );
        assert!(
            encode(
                &[1.0],
                &[sample],
                50.0,
                TouchstoneFrequencyUnit::Hz,
                TouchstoneFormat::RealImaginary,
                version
            )
            .is_ok()
        );
    }
}
