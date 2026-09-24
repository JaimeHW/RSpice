//! Bounded MATLAB v5 numeric parsing for import adapters.

use std::io::Cursor;

/// A parsed MATLAB file whose numeric arrays remain owned by the codec.
pub struct MatFile {
    inner: matfile::MatFile,
}

/// Borrowed view of one numeric MATLAB array.
#[derive(Clone, Copy)]
pub struct MatArray<'a> {
    inner: &'a matfile::Array,
}

fn adapter_error(format: &str, detail: impl std::fmt::Display) -> String {
    format!("{format} import: {detail}")
}

impl MatFile {
    pub fn parse(bytes: &[u8], max_variables: usize, format: &str) -> Result<Self, String> {
        let inner = std::panic::catch_unwind(|| matfile::MatFile::parse(Cursor::new(bytes)))
            .map_err(|_| adapter_error(format, "MATLAB parser rejected malformed input"))?
            .map_err(|error| {
                adapter_error(format, format_args!("invalid MATLAB v5 file: {error}"))
            })?;
        if inner.arrays().len() > max_variables {
            return Err(adapter_error(format, "too many MATLAB variables"));
        }
        Ok(Self { inner })
    }

    pub fn arrays(&self) -> Vec<MatArray<'_>> {
        self.inner
            .arrays()
            .iter()
            .map(|inner| MatArray { inner })
            .collect()
    }
}

impl MatArray<'_> {
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    pub fn size(&self) -> &[usize] {
        self.inner.size()
    }

    pub fn is_vector(&self) -> bool {
        self.size()
            .iter()
            .filter(|dimension| **dimension > 1)
            .count()
            <= 1
    }

    pub fn values(&self, format: &str) -> Result<(Vec<f64>, Option<Vec<f64>>), String> {
        let array = self.inner;
        macro_rules! convert {
            ($real:expr, $imag:expr) => {{
                (
                    $real.iter().map(|value| *value as f64).collect(),
                    $imag
                        .as_ref()
                        .map(|values| values.iter().map(|value| *value as f64).collect()),
                )
            }};
        }
        let values = match array.data() {
            matfile::NumericData::Int8 { real, imag } => convert!(real, imag),
            matfile::NumericData::UInt8 { real, imag } => convert!(real, imag),
            matfile::NumericData::Int16 { real, imag } => convert!(real, imag),
            matfile::NumericData::UInt16 { real, imag } => convert!(real, imag),
            matfile::NumericData::Int32 { real, imag } => convert!(real, imag),
            matfile::NumericData::UInt32 { real, imag } => convert!(real, imag),
            matfile::NumericData::Int64 { real, imag } => (
                real.iter()
                    .map(|value| {
                        crate::numeric::exact_signed_integer(array.name(), *value)
                            .map_err(|detail| adapter_error(format, detail))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                imag.as_ref()
                    .map(|values| {
                        values
                            .iter()
                            .map(|value| {
                                crate::numeric::exact_signed_integer(array.name(), *value)
                                    .map_err(|detail| adapter_error(format, detail))
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .transpose()?,
            ),
            matfile::NumericData::UInt64 { real, imag } => (
                real.iter()
                    .map(|value| {
                        crate::numeric::exact_unsigned_integer(array.name(), *value)
                            .map_err(|detail| adapter_error(format, detail))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                imag.as_ref()
                    .map(|values| {
                        values
                            .iter()
                            .map(|value| {
                                crate::numeric::exact_unsigned_integer(array.name(), *value)
                                    .map_err(|detail| adapter_error(format, detail))
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .transpose()?,
            ),
            matfile::NumericData::Single { real, imag } => convert!(real, imag),
            matfile::NumericData::Double { real, imag } => (real.clone(), imag.clone()),
        };
        let expected = array
            .size()
            .iter()
            .try_fold(1_usize, |count, dimension| count.checked_mul(*dimension))
            .ok_or_else(|| {
                adapter_error(
                    format,
                    format_args!("MATLAB variable '{}' shape overflows", array.name()),
                )
            })?;
        if values.0.len() != expected
            || values.1.as_ref().is_some_and(|imag| imag.len() != expected)
        {
            return Err(adapter_error(
                format,
                format_args!(
                    "MATLAB variable '{}' payload does not match its shape",
                    array.name()
                ),
            ));
        }
        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use super::MatFile;
    use crate::matlab::{MatVariable, write_mat_v5};

    #[test]
    fn reads_complex_numeric_values_and_limits_variable_count() {
        let bytes = write_mat_v5(
            "MATLAB 5.0 MAT-file, RSpice codec test",
            &[MatVariable {
                name: "V_out".to_owned(),
                real: vec![1.0, 3.0],
                imag: Some(vec![-2.0, 4.0]),
            }],
        )
        .expect("MAT fixture");
        let parsed = MatFile::parse(&bytes, 1, "matlab_v5").expect("parse");
        let arrays = parsed.arrays();
        assert_eq!(arrays.len(), 1);
        assert_eq!(arrays[0].name(), "V_out");
        assert!(arrays[0].is_vector());
        let (real, imag) = arrays[0].values("matlab_v5").expect("values");
        assert_eq!(real, [1.0, 3.0]);
        assert_eq!(imag.as_deref(), Some(&[-2.0, 4.0][..]));

        let error = MatFile::parse(&bytes, 0, "matlab_v5")
            .err()
            .expect("variable limit");
        assert!(error.contains("too many MATLAB variables"));
    }
}
