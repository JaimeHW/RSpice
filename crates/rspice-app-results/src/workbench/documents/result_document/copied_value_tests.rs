//! Tests proving a copied sample reads back as the double it was stored as.

use super::copied_sample_value;
use rspice_vocabulary::quantity::{AngleDisplay, CopiedValueFormat, QuantityPresentationPolicy};

/// A phase sample is stored in degrees. Its copy went through radians and
/// back, which landed a unit in the last place off for about one degree
/// value in eight, so the pasted phase was not the stored one.
#[test]
fn a_copied_phase_sample_reads_back_as_the_stored_degrees() {
    let policy = QuantityPresentationPolicy {
        copied_value_format: CopiedValueFormat::EngineeringNotationWithUnit,
        angle_display: AngleDisplay::Degrees,
        ..QuantityPresentationPolicy::default()
    };
    let mut degrees = (-72_000..=72_000)
        .map(|step| f64::from(step) * 0.01)
        .collect::<Vec<_>>();
    degrees.extend([1.0 / 3.0, 0.1, 5e-324, f64::MIN_POSITIVE, 1e300]);
    for exponent in -300..=300 {
        for mantissa in [1.0, 1.234_567_890_123_456_7, 3.2, 9.999_999_999_999_998] {
            let value = mantissa * 10_f64.powi(exponent);
            degrees.extend([value, -value]);
        }
    }
    for value in degrees {
        let copied = copied_sample_value(policy, value, "°");
        let (number, unit) = copied.split_once(' ').expect("a value and its unit");
        assert_eq!(unit, "°");
        let read_back: f64 = number.parse().expect("the copy parses as a number");
        assert_eq!(
            read_back.to_bits(),
            value.to_bits(),
            "{value:e} copied as {copied}"
        );
    }
}
