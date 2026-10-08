//! The typed value/validity pairing used by dense RAW and HDF5 tables.
//!
//! Pairing is positional. A validity column's display name is not authoritative:
//! an authored signal may itself be named `Valid(signal)`.

pub struct DenseNumericColumn<'a> {
    pub kind: &'a str,
    pub unit: Option<&'a str>,
    pub real: &'a [f64],
    pub imag: Option<&'a [f64]>,
}

/// Underlying quantity and whether the nullable value has two components.
pub fn nullable_value_type(kind: &str) -> Option<(&str, bool)> {
    kind.strip_prefix("nullable_real:")
        .map(|quantity| (quantity, false))
        .or_else(|| {
            kind.strip_prefix("nullable_complex:")
                .map(|quantity| (quantity, true))
        })
}

/// Validate the entire pair before exposing either values or sample availability.
pub fn decode_dense_validity(
    value: DenseNumericColumn<'_>,
    validity: DenseNumericColumn<'_>,
) -> Result<Vec<bool>, String> {
    let (quantity, complex) = nullable_value_type(value.kind)
        .ok_or("value column does not declare a nullable representation")?;
    if validity.kind.strip_prefix("nullable_validity:") != Some(quantity)
        || validity.unit != Some("1")
    {
        return Err("nullable validity column does not match its value column".into());
    }
    if validity.imag.is_some() {
        return Err("nullable validity columns must be real".into());
    }
    if value.imag.is_some() != complex {
        return Err("nullable dense column representation does not match its type".into());
    }
    if value.real.len() != validity.real.len()
        || value
            .imag
            .is_some_and(|imag| imag.len() != value.real.len())
    {
        return Err("nullable value/validity lengths differ".into());
    }
    value
        .real
        .iter()
        .enumerate()
        .map(|(index, &real)| {
            let flag = validity.real[index];
            let imag = value.imag.map(|values| values[index]);
            match flag {
                0.0 if real == 0.0 && imag.is_none_or(|value| value == 0.0) => Ok(false),
                1.0 if real.is_finite() && imag.is_none_or(f64::is_finite) => Ok(true),
                _ => Err("invalid nullable value or validity flag".to_owned()),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(real: &[f64], imag: Option<&[f64]>, flags: &[f64]) -> Result<Vec<bool>, String> {
        decode_dense_validity(
            DenseNumericColumn {
                kind: if imag.is_some() {
                    "nullable_complex:voltage"
                } else {
                    "nullable_real:voltage"
                },
                unit: Some("V"),
                real,
                imag,
            },
            DenseNumericColumn {
                kind: "nullable_validity:voltage",
                unit: Some("1"),
                real: flags,
                imag: None,
            },
        )
    }

    #[test]
    fn availability_is_explicit_and_padding_is_not_a_measurement() {
        assert_eq!(
            decode(&[1.0, -0.0, 0.0], None, &[1.0, 0.0, 1.0]).unwrap(),
            [true, false, true]
        );
        assert_eq!(
            decode(&[1.0, 0.0], Some(&[-2.0, 0.0]), &[1.0, 0.0]).unwrap(),
            [true, false]
        );
        for (values, flags) in [
            (vec![2.0], vec![0.0]),
            (vec![f64::NAN], vec![0.0]),
            (vec![f64::INFINITY], vec![1.0]),
            (vec![0.0], vec![0.5]),
            (vec![0.0], vec![f64::NAN]),
            (vec![0.0], vec![]),
        ] {
            assert!(decode(&values, None, &flags).is_err());
        }
        assert!(decode(&[0.0], Some(&[1.0]), &[0.0]).is_err());
        assert!(decode(&[0.0], Some(&[]), &[0.0]).is_err());
    }

    #[test]
    fn pair_declarations_must_match_the_storage_and_quantity() {
        for (kind, unit, imag) in [
            ("nullable_validity:current", Some("1"), None),
            ("nullable_validity:voltage", None, None),
            (
                "nullable_validity:voltage",
                Some("1"),
                Some([0.0].as_slice()),
            ),
        ] {
            assert!(
                decode_dense_validity(
                    DenseNumericColumn {
                        kind: "nullable_real:voltage",
                        unit: Some("V"),
                        real: &[0.0],
                        imag: None
                    },
                    DenseNumericColumn {
                        kind,
                        unit,
                        real: &[0.0],
                        imag
                    },
                )
                .is_err()
            );
        }
        assert!(
            decode_dense_validity(
                DenseNumericColumn {
                    kind: "nullable_complex:voltage",
                    unit: Some("V"),
                    real: &[0.0],
                    imag: None
                },
                DenseNumericColumn {
                    kind: "nullable_validity:voltage",
                    unit: Some("1"),
                    real: &[1.0],
                    imag: None
                },
            )
            .is_err()
        );
    }
}
