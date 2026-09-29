//! Decoded numeric waveform columns and exact integer sample conversion.

use std::collections::BTreeMap;

/// Decoded real or complex samples before result validation and admission.
#[derive(Debug)]
pub struct DecodedNumericSignal {
    pub name: String,
    pub real: Vec<f64>,
    pub imag: Option<Vec<f64>>,
    pub unit: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ComplexColumnError {
    DuplicateComponent(String),
    MissingReal(String),
    MissingImaginary(String),
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
        }
    }
}

impl std::error::Error for ComplexColumnError {}

pub const MAX_EXACT_F64_INTEGER: u64 = 1_u64 << 53;

/// An integer refused by the existing exact-sample conversion policy.
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
    if value.unsigned_abs() > MAX_EXACT_F64_INTEGER {
        Err(ExactIntegerError::Signed {
            identity: identity.to_owned(),
            value,
        })
    } else {
        Ok(value as f64)
    }
}

pub fn exact_unsigned_integer(identity: &str, value: u64) -> Result<f64, ExactIntegerError> {
    if value > MAX_EXACT_F64_INTEGER {
        Err(ExactIntegerError::Unsigned {
            identity: identity.to_owned(),
            value,
        })
    } else {
        Ok(value as f64)
    }
}

type ComplexComponentColumns = (Option<Vec<f64>>, Option<Vec<f64>>);

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
    let mut plain = Vec::new();
    let mut complex: BTreeMap<String, ComplexComponentColumns> = BTreeMap::new();
    for (name, values) in columns {
        if let Some((base, imag)) = complex_component(&name) {
            let entry = complex.entry(base.clone()).or_default();
            let slot = if imag { &mut entry.1 } else { &mut entry.0 };
            if slot.replace(values).is_some() {
                return Err(ComplexColumnError::DuplicateComponent(name));
            }
        } else {
            plain.push(DecodedNumericSignal {
                name,
                real: values,
                imag: None,
                unit: None,
            });
        }
    }
    for (name, (real, imag)) in complex {
        plain.push(DecodedNumericSignal {
            name: name.clone(),
            real: real.ok_or_else(|| ComplexColumnError::MissingReal(name.clone()))?,
            imag: Some(imag.ok_or_else(|| ComplexColumnError::MissingImaginary(name.clone()))?),
            unit: None,
        });
    }
    Ok(plain)
}

#[cfg(test)]
mod tests {
    use super::*;

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
