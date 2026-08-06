//! Trace transforms: how a bound trace's stored bit patterns become plotted
//! ordinates.
//!
//! The contract validates binding references but deliberately leaves
//! transform semantics to the runtime, so this module is the single owner of
//! that vocabulary. Compatibility is fail-closed: a transform that is
//! ambiguous for a trace kind rejects the figure instead of guessing —
//! `Identity` on a complex trace has no single honest reading, and
//! `PhaseDegrees` of a real trace is a producer defect, not a plot.

use rspice_publication_contract::{TraceTransform, TraceValues};

/// A transform applied to a trace kind it has no defined meaning for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("transform {transform:?} is not defined for a {kind} trace")]
pub struct IncompatibleTransform {
    pub transform: TraceTransform,
    pub kind: &'static str,
}

fn real_samples(bits: &[u64]) -> impl Iterator<Item = f64> + '_ {
    bits.iter().copied().map(f64::from_bits)
}

/// Apply one transform to one trace, yielding plotted ordinates.
///
/// `MagnitudeDb` of an exactly zero magnitude yields `-inf`; the plot layer
/// treats non-finite ordinates as line breaks rather than clamping them to
/// an invented floor.
pub fn apply(
    transform: TraceTransform,
    values: &TraceValues,
) -> Result<Vec<f64>, IncompatibleTransform> {
    let incompatible = |kind: &'static str| IncompatibleTransform { transform, kind };
    match values {
        TraceValues::Real { bits } => match transform {
            TraceTransform::Identity | TraceTransform::RealPart => Ok(real_samples(bits).collect()),
            TraceTransform::Magnitude => Ok(real_samples(bits).map(f64::abs).collect()),
            TraceTransform::MagnitudeDb => Ok(real_samples(bits)
                .map(|value| 20.0 * value.abs().log10())
                .collect()),
            TraceTransform::PhaseDegrees | TraceTransform::ImaginaryPart => {
                Err(incompatible("real"))
            }
        },
        TraceValues::Complex {
            real_bits,
            imaginary_bits,
        } => {
            let pairs = || real_samples(real_bits).zip(real_samples(imaginary_bits));
            match transform {
                TraceTransform::Identity => Err(incompatible("complex")),
                TraceTransform::Magnitude => Ok(pairs().map(|(re, im)| re.hypot(im)).collect()),
                TraceTransform::MagnitudeDb => Ok(pairs()
                    .map(|(re, im)| 20.0 * re.hypot(im).log10())
                    .collect()),
                TraceTransform::PhaseDegrees => {
                    Ok(pairs().map(|(re, im)| im.atan2(re).to_degrees()).collect())
                }
                TraceTransform::RealPart => Ok(real_samples(real_bits).collect()),
                TraceTransform::ImaginaryPart => Ok(real_samples(imaginary_bits).collect()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real(values: &[f64]) -> TraceValues {
        TraceValues::Real {
            bits: values.iter().map(|value| value.to_bits()).collect(),
        }
    }

    fn complex(pairs: &[(f64, f64)]) -> TraceValues {
        TraceValues::Complex {
            real_bits: pairs.iter().map(|(re, _)| re.to_bits()).collect(),
            imaginary_bits: pairs.iter().map(|(_, im)| im.to_bits()).collect(),
        }
    }

    #[test]
    fn identity_reproduces_real_bits_exactly() {
        let values = real(&[1.5, -2.25, 0.0, f64::MIN_POSITIVE]);
        let out = apply(TraceTransform::Identity, &values).expect("identity on real");
        assert_eq!(
            out.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            [1.5, -2.25, 0.0, f64::MIN_POSITIVE]
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn complex_magnitude_phase_and_parts_agree_with_the_math() {
        let values = complex(&[(3.0, 4.0), (0.0, -1.0)]);
        assert_eq!(
            apply(TraceTransform::Magnitude, &values).expect("magnitude"),
            vec![5.0, 1.0]
        );
        let db = apply(TraceTransform::MagnitudeDb, &values).expect("db");
        assert!((db[0] - 20.0 * 5.0f64.log10()).abs() < 1e-12);
        assert!(db[1].abs() < 1e-12);
        let phase = apply(TraceTransform::PhaseDegrees, &values).expect("phase");
        assert!((phase[0] - 53.13010235415598).abs() < 1e-9);
        assert!((phase[1] + 90.0).abs() < 1e-12);
        assert_eq!(
            apply(TraceTransform::RealPart, &values).expect("real part"),
            vec![3.0, 0.0]
        );
        assert_eq!(
            apply(TraceTransform::ImaginaryPart, &values).expect("imaginary part"),
            vec![4.0, -1.0]
        );
    }

    #[test]
    fn zero_magnitude_db_yields_negative_infinity_not_a_clamp() {
        let out = apply(TraceTransform::MagnitudeDb, &real(&[0.0])).expect("db of zero");
        assert_eq!(out, vec![f64::NEG_INFINITY]);
    }

    #[test]
    fn ambiguous_pairings_reject_instead_of_guessing() {
        assert!(apply(TraceTransform::Identity, &complex(&[(1.0, 2.0)])).is_err());
        assert!(apply(TraceTransform::PhaseDegrees, &real(&[1.0])).is_err());
        assert!(apply(TraceTransform::ImaginaryPart, &real(&[1.0])).is_err());
    }

    #[test]
    fn real_part_of_a_real_trace_is_the_value_itself() {
        let out = apply(TraceTransform::RealPart, &real(&[7.25])).expect("real part");
        assert_eq!(out, vec![7.25]);
    }
}
