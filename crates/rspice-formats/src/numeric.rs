//! Decoded numeric waveform columns and exact integer sample conversion.

use std::collections::BTreeMap;

pub mod nullable;
pub mod sample_serde;

/// Decoded real or complex samples before result validation and admission.
#[derive(Debug)]
pub struct DecodedNumericSignal {
    pub name: String,
    /// Exact values; NaN marks an explicitly unavailable sample, never a zero.
    pub real: Vec<f64>,
    pub imag: Option<Vec<f64>>,
    pub unit: Option<String>,
}

/// Decoded waveform fields before result validation and application admission.
#[derive(Debug)]
pub struct DecodedNumericDataset {
    pub domain: crate::WaveformDomain,
    pub coordinate_name: String,
    /// Unit of the coordinate samples; absence is not a dimensionless declaration.
    pub coordinate_unit: Option<String>,
    pub coordinate: Vec<f64>,
    pub signals: Vec<DecodedNumericSignal>,
}

impl DecodedNumericDataset {
    /// Normalize declared engineering coordinates before time/frequency consumers
    /// use them. Unknown sweep units remain literal; time and frequency require
    /// a compatible known unit when one is explicitly declared.
    pub fn normalize_coordinate_unit(&mut self) -> Result<(), String> {
        use crate::delimited::unit::{EngineeringUnit, UnitDimension};
        let Some(symbol) = self.coordinate_unit.as_deref() else {
            return Ok(());
        };
        if symbol.trim().is_empty() || symbol.chars().any(char::is_control) {
            return Err("coordinate unit must be non-empty and control-free".into());
        }
        let required = match self.domain {
            crate::WaveformDomain::Transient => Some(UnitDimension::Time),
            crate::WaveformDomain::Ac => Some(UnitDimension::Frequency),
            crate::WaveformDomain::DcSweep => None,
        };
        let unit = match EngineeringUnit::parse(symbol) {
            Ok(unit) => unit,
            Err(_) if required.is_none() => return Ok(()),
            Err(error) => return Err(error),
        };
        if required.is_some_and(|dimension| dimension != unit.dimension) {
            return Err(format!(
                "coordinate unit '{symbol}' is incompatible with the analysis domain"
            ));
        }
        let values = self
            .coordinate
            .iter()
            .map(|&value| unit.normalize_binary(value))
            .collect::<Vec<_>>();
        for (index, (&before, &after)) in self.coordinate.iter().zip(&values).enumerate() {
            if !after.is_finite() || unit.lost_nonzero_sample(before, after) {
                return Err(format!(
                    "coordinate sample {index} overflows or underflows after unit conversion from '{symbol}'"
                ));
            }
            if index > 0 && before != self.coordinate[index - 1] && after == values[index - 1] {
                return Err(format!(
                    "coordinate samples {} and {index} collapse after unit conversion from '{symbol}'",
                    index - 1
                ));
            }
        }
        self.coordinate = values;
        self.coordinate_unit = Some(unit.canonical_symbol().to_owned());
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ComplexColumnError {
    DuplicateComponent(String),
    MissingReal(String),
    MissingImaginary(String),
    InconsistentUnits {
        name: String,
        real: Option<String>,
        imag: Option<String>,
    },
}

impl std::fmt::Display for ComplexColumnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateComponent(name) => write!(f, "duplicate complex component '{name}'"),
            Self::MissingReal(name) => {
                write!(f, "complex signal '{name}' is missing its real component")
            }
            Self::MissingImaginary(name) => write!(
                f,
                "complex signal '{name}' is missing its imaginary component"
            ),
            Self::InconsistentUnits { name, real, imag } => write!(
                f,
                "complex signal '{name}' has inconsistent component units: real {}, imaginary {}",
                real.as_deref().unwrap_or("(unstated)"),
                imag.as_deref().unwrap_or("(unstated)")
            ),
        }
    }
}

impl std::error::Error for ComplexColumnError {}

/// Whether rounding a validated decimal to `value` erased a nonzero mantissa.
/// This is not a syntax validator. Apply multiplicative unit conversion first;
/// an affine conversion can legitimately map a nonzero decimal to zero.
/// Both decimal E exponents and Fortran D exponents are recognized.
pub fn decimal_underflowed(decimal: &str, value: f64) -> bool {
    value == 0.0
        && decimal
            .split(['e', 'E', 'd', 'D'])
            .next()
            .is_some_and(|mantissa| mantissa.bytes().any(|digit| matches!(digit, b'1'..=b'9')))
}

/// Every integer up to this magnitude is exact; larger values can still be
/// exact when they are multiples of the binary64 spacing at that magnitude.
pub const MAX_EXACT_F64_INTEGER: u64 = 1_u64 << 53;

/// An integer refused because conversion to binary64 would round its value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExactIntegerError {
    Signed { identity: String, value: i64 },
    Unsigned { identity: String, value: u64 },
}

impl std::fmt::Display for ExactIntegerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signed { identity, value } => write!(
                f,
                "'{identity}' integer {value} cannot be represented exactly as f64"
            ),
            Self::Unsigned { identity, value } => write!(
                f,
                "'{identity}' integer {value} cannot be represented exactly as f64"
            ),
        }
    }
}

impl std::error::Error for ExactIntegerError {}

pub fn exact_signed_integer(identity: &str, value: i64) -> Result<f64, ExactIntegerError> {
    let converted = value as f64;
    // Use a wider integer for the check: casting a rounded 2^63 back to i64
    // saturates to i64::MAX and would falsely certify that input as exact.
    if converted as i128 != i128::from(value) {
        Err(ExactIntegerError::Signed {
            identity: identity.to_owned(),
            value,
        })
    } else {
        Ok(converted)
    }
}

pub fn exact_unsigned_integer(identity: &str, value: u64) -> Result<f64, ExactIntegerError> {
    let converted = value as f64;
    // Likewise, u128 can distinguish a rounded 2^64 from u64::MAX.
    if converted as u128 != u128::from(value) {
        Err(ExactIntegerError::Unsigned {
            identity: identity.to_owned(),
            value,
        })
    } else {
        Ok(converted)
    }
}

type NumericComponentColumn = (Vec<f64>, Option<String>);
type ComplexComponentColumns = (
    Option<NumericComponentColumn>,
    Option<NumericComponentColumn>,
);

fn complex_component(name: &str) -> Option<(String, bool)> {
    for (suffix, imag) in [
        ("__real", false),
        ("__imag", true),
        ("_RE", false),
        ("_IM", true),
    ] {
        if let Some(base) = name.strip_suffix(suffix) {
            return Some((base.to_owned(), imag));
        }
    }
    if let Some(base) = name
        .strip_prefix("Re(")
        .and_then(|value| value.strip_suffix(')'))
    {
        return Some((base.to_owned(), false));
    }
    if let Some(base) = name
        .strip_prefix("Im(")
        .and_then(|value| value.strip_suffix(')'))
    {
        return Some((base.to_owned(), true));
    }
    None
}

/// Decode paired numeric columns, retaining plain-column input order followed
/// by complex signals in name order. Samples are moved without numeric changes.
pub fn combine_real_imag_columns(
    columns: Vec<(String, Vec<f64>)>,
) -> Result<Vec<DecodedNumericSignal>, ComplexColumnError> {
    combine_real_imag_columns_with_units(
        columns
            .into_iter()
            .map(|(name, values)| (name, values, None)),
    )
}

/// Preserve each plain column's literal unit. Complex components must declare
/// the same unit (or both leave it unstated); silently choosing either unit
/// would change the meaning of the other component's samples.
pub(crate) fn combine_real_imag_columns_with_units(
    columns: impl IntoIterator<Item = (String, Vec<f64>, Option<String>)>,
) -> Result<Vec<DecodedNumericSignal>, ComplexColumnError> {
    let mut plain = Vec::new();
    let mut complex: BTreeMap<String, ComplexComponentColumns> = BTreeMap::new();
    for (name, values, unit) in columns {
        if let Some((base, imag)) = complex_component(&name) {
            let entry = complex.entry(base.clone()).or_default();
            let slot = if imag { &mut entry.1 } else { &mut entry.0 };
            if slot.replace((values, unit)).is_some() {
                return Err(ComplexColumnError::DuplicateComponent(name));
            }
        } else {
            plain.push(DecodedNumericSignal {
                name,
                real: values,
                imag: None,
                unit,
            });
        }
    }
    for (name, (real, imag)) in complex {
        let (real, real_unit) =
            real.ok_or_else(|| ComplexColumnError::MissingReal(name.clone()))?;
        let (imag, imag_unit) =
            imag.ok_or_else(|| ComplexColumnError::MissingImaginary(name.clone()))?;
        if real_unit != imag_unit {
            return Err(ComplexColumnError::InconsistentUnits {
                name,
                real: real_unit,
                imag: imag_unit,
            });
        }
        plain.push(DecodedNumericSignal {
            name,
            real,
            imag: Some(imag),
            unit: real_unit,
        });
    }
    Ok(plain)
}

pub fn stated_coordinate_names(names: &[&str]) -> String {
    let (last, rest) = names
        .split_last()
        .expect("the coordinate-name list is never empty");
    format!("{}, or {last}", rest.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_conversion_accepts_representable_large_values_and_rejects_rounding() {
        for value in [
            i64::MIN,
            i64::MIN + 1024,
            -9007199254740994,
            -9007199254740992,
            -1,
            0,
            1,
            9007199254740992,
            9007199254740994,
            i64::MAX - 1023,
        ] {
            let actual = exact_signed_integer("signed", value).unwrap();
            assert_eq!(actual as i128, i128::from(value));
        }
        for value in [
            0,
            1,
            1u64 << 53,
            (1u64 << 53) + 2,
            1u64 << 63,
            u64::MAX - 2047,
        ] {
            let actual = exact_unsigned_integer("unsigned", value).unwrap();
            assert_eq!(actual as u128, u128::from(value));
        }
        for value in [i64::MIN + 1, -9007199254740993, 9007199254740993, i64::MAX] {
            assert_eq!(
                exact_signed_integer("signed", value).unwrap_err(),
                ExactIntegerError::Signed {
                    identity: "signed".into(),
                    value
                }
            );
        }
        for value in [(1u64 << 53) + 1, (1u64 << 63) + 1, u64::MAX] {
            assert_eq!(
                exact_unsigned_integer("unsigned", value).unwrap_err(),
                ExactIntegerError::Unsigned {
                    identity: "unsigned".into(),
                    value
                }
            );
        }
    }

    fn dataset(
        domain: crate::WaveformDomain,
        unit: Option<&str>,
        coordinate: Vec<f64>,
    ) -> DecodedNumericDataset {
        DecodedNumericDataset {
            domain,
            coordinate_name: "x".into(),
            coordinate_unit: unit.map(str::to_owned),
            coordinate,
            signals: vec![],
        }
    }

    #[test]
    fn coordinate_normalization_preserves_unknown_units_and_signed_zero() {
        use crate::WaveformDomain::{DcSweep, Transient};
        for unit in [None, Some("widgets")] {
            let mut data = dataset(DcSweep, unit, vec![-0.0, 1.0]);
            data.normalize_coordinate_unit().unwrap();
            assert_eq!(data.coordinate_unit.as_deref(), unit);
            assert_eq!(data.coordinate[0].to_bits(), (-0.0_f64).to_bits());
            assert_eq!(data.coordinate[1], 1.0);
        }
        let mut data = dataset(Transient, Some("ns"), vec![-0.0, 2.0]);
        data.normalize_coordinate_unit().unwrap();
        assert_eq!(data.coordinate[0].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(data.coordinate[1], 2e-9);
        assert_eq!(data.coordinate_unit.as_deref(), Some("s"));
    }

    #[test]
    fn coordinate_normalization_refuses_incompatible_or_unrepresentable_axes_atomically() {
        use crate::WaveformDomain::{Ac, DcSweep, Transient};
        for (domain, unit, values, message) in [
            (Transient, "mV", vec![0.0, 1.0], "incompatible"),
            (Ac, "unknown", vec![1.0, 2.0], "unit"),
            (Ac, "THz", vec![1e308], "overflows or underflows"),
            (
                Transient,
                "fs",
                vec![f64::from_bits(1)],
                "overflows or underflows",
            ),
            (DcSweep, "degC", vec![0.0, f64::from_bits(1)], "collapse"),
        ] {
            let mut data = dataset(domain, Some(unit), values.clone());
            let error = data.normalize_coordinate_unit().unwrap_err();
            assert!(error.contains(message), "{error}");
            assert_eq!(data.coordinate_unit.as_deref(), Some(unit));
            assert_eq!(data.coordinate, values);
        }
    }

    #[test]
    fn complex_pairing_preserves_plain_order_sorted_pairs_and_exact_samples() {
        let adjacent = f64::from_bits(1.0_f64.to_bits() + 1);
        let signals = combine_real_imag_columns(vec![
            ("b__imag".into(), vec![-0.0, adjacent]),
            ("z".into(), vec![7.0]),
            ("Im(a)".into(), vec![-2.0]),
            ("b__real".into(), vec![3.0, 4.0]),
            ("plain".into(), vec![8.0]),
            ("Re(a)".into(), vec![1.0]),
        ])
        .unwrap();
        assert_eq!(
            signals.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(),
            ["z", "plain", "a", "b"]
        );
        assert_eq!(signals[0].real, [7.0]);
        assert_eq!(signals[1].real, [8.0]);
        assert_eq!(signals[2].real, [1.0]);
        assert_eq!(signals[2].imag.as_deref(), Some([-2.0].as_slice()));
        assert_eq!(signals[3].real, [3.0, 4.0]);
        let imaginary = signals[3].imag.as_ref().unwrap();
        assert_eq!(imaginary[0].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(imaginary[1].to_bits(), adjacent.to_bits());
        assert!(signals.iter().all(|signal| signal.unit.is_none()));
    }

    #[test]
    fn duplicate_aliases_and_missing_components_keep_their_refusal_identity() {
        let pair = |name: &str| (name.to_owned(), vec![1.0]);
        let error = combine_real_imag_columns(vec![pair("a__real"), pair("Re(a)")]).unwrap_err();
        assert_eq!(
            error,
            ComplexColumnError::DuplicateComponent("Re(a)".into())
        );
        assert_eq!(error.to_string(), "duplicate complex component 'Re(a)'");
        // A missing earlier signal is reported before a later incomplete pair.
        let error = combine_real_imag_columns(vec![pair("z_RE"), pair("a_IM")]).unwrap_err();
        assert_eq!(error, ComplexColumnError::MissingReal("a".into()));
        assert_eq!(
            error.to_string(),
            "complex signal 'a' is missing its real component"
        );
        let error = combine_real_imag_columns(vec![pair("a_RE")]).unwrap_err();
        assert_eq!(error, ComplexColumnError::MissingImaginary("a".into()));
        assert_eq!(
            error.to_string(),
            "complex signal 'a' is missing its imaginary component"
        );
    }
}
