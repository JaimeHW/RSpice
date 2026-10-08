//! Bounded MATLAB v5 numeric parsing for import adapters.

use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal, stated_coordinate_names};
use std::io::Cursor;

#[derive(Debug, Clone, Copy)]
pub struct MatlabReadLimits<'a> {
    pub max_variables: usize,
    pub min_rows: usize,
    pub coordinate_names: &'a [&'a str],
}

#[derive(Debug)]
pub struct MatlabReadError {
    pub format: String,
    pub reason: MatlabReadFailure,
}

#[derive(Debug)]
pub enum MatlabReadFailure {
    ParserPanicked,
    Parse(matfile::Error),
    VariableLimit {
        variables: usize,
        limit: usize,
    },
    InexactInteger(crate::numeric::ExactIntegerError),
    ShapeOverflow {
        name: String,
        shape: Vec<usize>,
    },
    PayloadShape {
        name: String,
        shape: Vec<usize>,
        real: usize,
        imag: Option<usize>,
    },
    CoordinateNotRealVector {
        name: String,
        shape: Vec<usize>,
        complex: bool,
    },
    NonVector {
        name: String,
        shape: Vec<usize>,
    },
    MissingCoordinate {
        expected: String,
    },
    TableShape {
        shape: Vec<usize>,
        min_rows: usize,
    },
    ComplexTable {
        name: String,
    },
}

impl std::fmt::Display for MatlabReadFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParserPanicked => f.write_str("MATLAB parser rejected malformed input"),
            Self::Parse(source) => write!(f, "invalid MATLAB v5 file: {source}"),
            Self::VariableLimit { .. } => f.write_str("too many MATLAB variables"),
            Self::InexactInteger(source) => source.fmt(f),
            Self::ShapeOverflow { name, .. } => write!(f, "MATLAB variable '{name}' shape overflows"),
            Self::PayloadShape { name, .. } => write!(f, "MATLAB variable '{name}' payload does not match its shape"),
            Self::CoordinateNotRealVector { .. } => f.write_str("MATLAB coordinate variable must be a real vector"),
            Self::NonVector { name, .. } => write!(f, "MATLAB variable '{name}' is not a vector"),
            Self::MissingCoordinate { expected } => write!(f, "MATLAB file requires a coordinate variable named {expected}"),
            Self::TableShape { .. } => f.write_str("without a named coordinate, MATLAB data must be one rows-by-columns table with the coordinate in column one"),
            Self::ComplexTable { .. } => f.write_str("a complex MATLAB table requires separate named coordinate and signal variables"),
        }
    }
}

impl std::error::Error for MatlabReadFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(source) => Some(source),
            Self::InexactInteger(source) => Some(source),
            _ => None,
        }
    }
}

impl std::fmt::Display for MatlabReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} import: {}", self.format, self.reason)
    }
}

impl std::error::Error for MatlabReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.reason)
    }
}

/// A parsed MATLAB file whose numeric arrays remain owned by the codec.
struct MatFile {
    inner: matfile::MatFile,
}

/// Borrowed view of one numeric MATLAB array.
#[derive(Clone, Copy)]
struct MatArray<'a> {
    inner: &'a matfile::Array,
}

fn adapter_error(format: &str, reason: MatlabReadFailure) -> MatlabReadError {
    MatlabReadError {
        format: format.to_owned(),
        reason,
    }
}

impl MatFile {
    fn parse(bytes: &[u8], max_variables: usize, format: &str) -> Result<Self, MatlabReadError> {
        let inner = std::panic::catch_unwind(|| matfile::MatFile::parse(Cursor::new(bytes)))
            .map_err(|_| adapter_error(format, MatlabReadFailure::ParserPanicked))?
            .map_err(|error| adapter_error(format, MatlabReadFailure::Parse(error)))?;
        if inner.arrays().len() > max_variables {
            return Err(adapter_error(
                format,
                MatlabReadFailure::VariableLimit {
                    variables: inner.arrays().len(),
                    limit: max_variables,
                },
            ));
        }
        Ok(Self { inner })
    }

    fn arrays(&self) -> Vec<MatArray<'_>> {
        self.inner
            .arrays()
            .iter()
            .map(|inner| MatArray { inner })
            .collect()
    }
}

impl MatArray<'_> {
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn size(&self) -> &[usize] {
        self.inner.size()
    }

    fn is_vector(&self) -> bool {
        self.size()
            .iter()
            .filter(|dimension| **dimension > 1)
            .count()
            <= 1
    }

    fn values(&self, format: &str) -> Result<(Vec<f64>, Option<Vec<f64>>), MatlabReadError> {
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
                        crate::numeric::exact_signed_integer(array.name(), *value).map_err(
                            |detail| {
                                adapter_error(format, MatlabReadFailure::InexactInteger(detail))
                            },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                imag.as_ref()
                    .map(|values| {
                        values
                            .iter()
                            .map(|value| {
                                crate::numeric::exact_signed_integer(array.name(), *value).map_err(
                                    |detail| {
                                        adapter_error(
                                            format,
                                            MatlabReadFailure::InexactInteger(detail),
                                        )
                                    },
                                )
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .transpose()?,
            ),
            matfile::NumericData::UInt64 { real, imag } => (
                real.iter()
                    .map(|value| {
                        crate::numeric::exact_unsigned_integer(array.name(), *value).map_err(
                            |detail| {
                                adapter_error(format, MatlabReadFailure::InexactInteger(detail))
                            },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                imag.as_ref()
                    .map(|values| {
                        values
                            .iter()
                            .map(|value| {
                                crate::numeric::exact_unsigned_integer(array.name(), *value)
                                    .map_err(|detail| {
                                        adapter_error(
                                            format,
                                            MatlabReadFailure::InexactInteger(detail),
                                        )
                                    })
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
                    MatlabReadFailure::ShapeOverflow {
                        name: array.name().to_owned(),
                        shape: array.size().to_vec(),
                    },
                )
            })?;
        if values.0.len() != expected
            || values.1.as_ref().is_some_and(|imag| imag.len() != expected)
        {
            return Err(adapter_error(
                format,
                MatlabReadFailure::PayloadShape {
                    name: array.name().to_owned(),
                    shape: array.size().to_vec(),
                    real: values.0.len(),
                    imag: values.1.as_ref().map(Vec::len),
                },
            ));
        }
        Ok(values)
    }
}

/// Decode MATLAB numeric variables using their named coordinate or table layout.
pub fn decode_matlab_v5(
    bytes: &[u8],
    limits: MatlabReadLimits<'_>,
    format: &str,
) -> Result<DecodedNumericDataset, MatlabReadError> {
    let parsed = MatFile::parse(bytes, limits.max_variables, format)?;
    let arrays = parsed.arrays();
    let coordinate_index = arrays.iter().position(|array| {
        limits
            .coordinate_names
            .iter()
            .any(|name| array.name().eq_ignore_ascii_case(name))
    });
    if let Some(coordinate_index) = coordinate_index {
        let coordinate_array = &arrays[coordinate_index];
        let (coordinate, coordinate_imag) = coordinate_array.values(format)?;
        if coordinate_imag.is_some() || !coordinate_array.is_vector() {
            return Err(adapter_error(
                format,
                MatlabReadFailure::CoordinateNotRealVector {
                    name: coordinate_array.name().to_owned(),
                    shape: coordinate_array.size().to_vec(),
                    complex: coordinate_imag.is_some(),
                },
            ));
        }
        let mut signals = Vec::new();
        for (index, array) in arrays.iter().enumerate() {
            if index == coordinate_index {
                continue;
            }
            if !array.is_vector() {
                return Err(adapter_error(
                    format,
                    MatlabReadFailure::NonVector {
                        name: array.name().to_owned(),
                        shape: array.size().to_vec(),
                    },
                ));
            }
            let (real, imag) = array.values(format)?;
            signals.push(DecodedNumericSignal {
                name: array.name().to_owned(),
                real,
                imag,
                unit: None,
            });
        }
        return Ok(DecodedNumericDataset {
            coordinate_unit: None,
            domain: crate::WaveformDomain::from_coordinate_name(coordinate_array.name()),
            coordinate_name: coordinate_array.name().to_owned(),
            coordinate,
            signals,
        });
    }

    if arrays.len() != 1 {
        return Err(adapter_error(
            format,
            MatlabReadFailure::MissingCoordinate {
                expected: stated_coordinate_names(limits.coordinate_names),
            },
        ));
    }
    let array = &arrays[0];
    let size = array.size();
    if size.len() != 2 || size[0] < limits.min_rows || size[1] < 2 {
        return Err(adapter_error(
            format,
            MatlabReadFailure::TableShape {
                shape: size.to_vec(),
                min_rows: limits.min_rows,
            },
        ));
    }
    let (real, imag) = array.values(format)?;
    if imag.is_some() {
        return Err(adapter_error(
            format,
            MatlabReadFailure::ComplexTable {
                name: array.name().to_owned(),
            },
        ));
    }
    let rows = size[0];
    let columns = size[1];
    let coordinate = real[..rows].to_vec();
    let signals = (1..columns)
        .map(|column| DecodedNumericSignal {
            name: format!("{}.{}", array.name(), column),
            real: real[column * rows..(column + 1) * rows].to_vec(),
            imag: None,
            unit: None,
        })
        .collect();
    Ok(DecodedNumericDataset {
        coordinate_unit: None,
        domain: crate::WaveformDomain::DcSweep,
        coordinate_name: "x".into(),
        coordinate,
        signals,
    })
}

#[cfg(test)]
mod tests {
    use super::{MatFile, MatlabReadFailure, MatlabReadLimits, decode_matlab_v5};
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
        assert!(matches!(
            &error.reason,
            MatlabReadFailure::VariableLimit {
                variables: 1,
                limit: 0
            }
        ));
        assert_eq!(
            error.to_string(),
            "matlab_v5 import: too many MATLAB variables"
        );
    }

    #[test]
    fn named_variables_preserve_first_coordinate_and_complex_samples() {
        let variables = [
            MatVariable {
                name: "T".into(),
                real: vec![0.0, 1.0],
                imag: None,
            },
            MatVariable {
                name: "time".into(),
                real: vec![2.0, 3.0],
                imag: None,
            },
            MatVariable {
                name: "out".into(),
                real: vec![4.0, 5.0],
                imag: Some(vec![-0.0, -6.0]),
            },
        ];
        let bytes = write_mat_v5("MATLAB 5.0 MAT-file, import fixture", &variables).unwrap();
        let limits = MatlabReadLimits {
            max_variables: 3,
            min_rows: 1,
            coordinate_names: &["time", "t"],
        };
        let decoded = decode_matlab_v5(&bytes, limits, "matlab_v5").unwrap();
        assert_eq!(decoded.coordinate_name, "T");
        assert_eq!(decoded.domain, crate::WaveformDomain::Transient);
        assert_eq!(decoded.coordinate, [0.0, 1.0]);
        assert_eq!(
            decoded
                .signals
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>(),
            ["time", "out"]
        );
        assert_eq!(decoded.signals[0].real, [2.0, 3.0]);
        assert_eq!(decoded.signals[1].real, [4.0, 5.0]);
        let imag = decoded.signals[1].imag.as_ref().unwrap();
        assert_eq!(imag[0].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(imag[1], -6.0);
        assert!(decoded.signals.iter().all(|s| s.unit.is_none()));
    }

    #[test]
    fn unnamed_table_uses_column_major_layout_and_refuses_complex_data() {
        let limits = MatlabReadLimits {
            max_variables: 1,
            min_rows: 1,
            coordinate_names: &["time", "t"],
        };
        for complex in [false, true] {
            let mut bytes = write_mat_v5(
                "MATLAB 5.0 MAT-file, table fixture",
                &[MatVariable {
                    name: "measurements".into(),
                    real: vec![0.0, 1.0, 10.0, 11.0, 20.0, 21.0],
                    imag: complex.then(|| vec![0.0; 6]),
                }],
            )
            .unwrap();
            // Reshape the writer's six-row vector into a two-by-three MAT array.
            // Its dimensions element follows the outer tag and the array flags.
            let dimensions = crate::matlab::HEADER_BYTES + 8 + 16;
            assert_eq!(&bytes[dimensions..dimensions + 4], &5_u32.to_le_bytes());
            assert_eq!(&bytes[dimensions + 4..dimensions + 8], &8_u32.to_le_bytes());
            bytes[dimensions + 8..dimensions + 12].copy_from_slice(&2_i32.to_le_bytes());
            bytes[dimensions + 12..dimensions + 16].copy_from_slice(&3_i32.to_le_bytes());
            let decoded = decode_matlab_v5(&bytes, limits, "matlab_v5");
            if complex {
                let error = decoded.unwrap_err();
                assert!(
                    matches!(&error.reason, MatlabReadFailure::ComplexTable { name } if name == "measurements")
                );
                assert_eq!(
                    error.to_string(),
                    "matlab_v5 import: a complex MATLAB table requires separate named coordinate and signal variables"
                );
            } else {
                let decoded = decoded.unwrap();
                assert_eq!(decoded.domain, crate::WaveformDomain::DcSweep);
                assert_eq!(decoded.coordinate_name, "x");
                assert_eq!(decoded.coordinate, [0.0, 1.0]);
                assert_eq!(decoded.signals[0].name, "measurements.1");
                assert_eq!(decoded.signals[0].real, [10.0, 11.0]);
                assert_eq!(decoded.signals[1].name, "measurements.2");
                assert_eq!(decoded.signals[1].real, [20.0, 21.0]);
            }
        }
    }

    #[test]
    fn malformed_matlab_retains_the_parser_cause() {
        use std::error::Error as _;
        let limits = MatlabReadLimits {
            max_variables: 2,
            min_rows: 1,
            coordinate_names: &["time", "t"],
        };
        let error = decode_matlab_v5(b"invalid", limits, "matlab_v5").unwrap_err();
        assert!(matches!(&error.reason, MatlabReadFailure::Parse(_)));
        assert!(error.reason.source().unwrap().is::<matfile::Error>());
        assert!(
            error
                .to_string()
                .starts_with("matlab_v5 import: invalid MATLAB v5 file: ")
        );
    }
}
