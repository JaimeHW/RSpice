//! Bounded NPY decoding and projection into numeric waveform columns.

use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal};
use std::io::Cursor;

/// A NumPy array/container failure or a bounded decoding refusal.
#[derive(Debug)]
pub struct NumpyReadError {
    pub format: String,
    pub reason: NumpyReadFailure,
}

#[derive(Debug)]
pub enum NumpyReadFailure {
    Header(std::io::Error),
    DimensionTooLarge(u64),
    WaveformDimensions(Vec<usize>),
    ShapeProductOverflow(Vec<usize>),
    NumericValueLimit {
        values: usize,
        limit: usize,
    },
    StructuredDtype(Box<npyz::DType>),
    Values(std::io::Error),
    InexactInteger(crate::numeric::ExactIntegerError),
    UnsupportedDtype {
        kind: npyz::TypeChar,
        size: u64,
    },
    NonVector {
        name: String,
        shape: Vec<usize>,
    },
    PayloadShape {
        name: String,
        shape: Vec<usize>,
        real: usize,
        imag: Option<usize>,
    },
    WaveformBounds {
        shape: Vec<usize>,
        max_rows: usize,
        max_columns: usize,
    },
    Archive(zip::result::ZipError),
    MemberCount {
        members: usize,
        limit: usize,
    },
    ArchiveMember {
        index: usize,
        source: zip::result::ZipError,
    },
    UnsafeMember(String),
    UnsupportedMember(String),
    ExpandedSizeOverflow,
    ExpandedByteLimit {
        expanded: u64,
        limit: u64,
        decoded: bool,
    },
    InvalidMemberName(String),
    DuplicateArray(String),
    MemberRead {
        name: String,
        source: std::io::Error,
    },
    MissingCoordinate {
        expected: String,
    },
    ComplexCoordinate {
        name: String,
    },
    ImaginaryMatrixCoordinate {
        sample: usize,
    },
}

impl std::fmt::Display for NumpyReadFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Header(source) => write!(f, "invalid NPY header: {source}"),
            Self::DimensionTooLarge(_) => f.write_str("NPY dimension exceeds this platform"),
            Self::WaveformDimensions(shape) => write!(
                f,
                "NPY shape {shape:?} is not a one- or two-dimensional waveform table"
            ),
            Self::ShapeProductOverflow(_) => f.write_str("NPY shape product overflow"),
            Self::NumericValueLimit { .. } => f.write_str("NPY numeric-value limit exceeded"),
            Self::StructuredDtype(_) => f.write_str(
                "structured and nested NPY dtypes require an explicit mapping and are not accepted",
            ),
            Self::Values(source) => write!(f, "could not decode NPY values: {source}"),
            Self::InexactInteger(source) => source.fmt(f),
            Self::UnsupportedDtype { kind, size } => {
                write!(f, "unsupported NPY dtype {kind:?}{size}")
            }
            Self::NonVector { name, shape } => {
                write!(f, "NPZ array '{name}' has non-vector shape {shape:?}")
            }
            Self::PayloadShape { name, .. } => {
                write!(f, "NPZ array '{name}' payload does not match its shape")
            }
            Self::WaveformBounds { shape, .. } => {
                write!(f, "NPY waveform shape {shape:?} exceeds import bounds")
            }
            Self::Archive(source) => write!(f, "invalid NPZ archive: {source}"),
            Self::MemberCount { members, limit } => {
                write!(f, "archive has {members} members; the limit is {limit}")
            }
            Self::ArchiveMember { index, source } => {
                write!(f, "invalid archive member {index}: {source}")
            }
            Self::UnsafeMember(name) => write!(f, "unsafe archive member '{name}'"),
            Self::UnsupportedMember(name) => write!(
                f,
                "unsupported NPZ member '{name}'; only .npy arrays are accepted"
            ),
            Self::ExpandedSizeOverflow => f.write_str("archive expanded-size accounting overflow"),
            Self::ExpandedByteLimit { .. } => f.write_str("NPZ expanded-byte limit exceeded"),
            Self::InvalidMemberName(name) => write!(f, "invalid member name '{name}'"),
            Self::DuplicateArray(name) => write!(f, "archive repeats array identity '{name}'"),
            Self::MemberRead { name, source } => write!(f, "could not decode '{name}': {source}"),
            Self::MissingCoordinate { expected } => {
                write!(f, "NPZ requires one coordinate array named {expected}")
            }
            Self::ComplexCoordinate { .. } => f.write_str("NPZ coordinate array cannot be complex"),
            Self::ImaginaryMatrixCoordinate { sample } => write!(
                f,
                "NPY coordinate column must have zero imaginary components; invalid sample {sample}"
            ),
        }
    }
}

impl std::error::Error for NumpyReadFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Header(source) | Self::Values(source) | Self::MemberRead { source, .. } => {
                Some(source)
            }
            Self::Archive(source) | Self::ArchiveMember { source, .. } => Some(source),
            Self::InexactInteger(source) => Some(source),
            _ => None,
        }
    }
}

impl std::fmt::Display for NumpyReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} import: {}", self.format, self.reason)
    }
}

impl std::error::Error for NumpyReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.reason)
    }
}

#[derive(Debug)]
pub struct NpyArray {
    shape: Vec<usize>,
    fortran: bool,
    real: Vec<f64>,
    imag: Option<Vec<f64>>,
}

impl NpyArray {
    pub(super) fn is_complex(&self) -> bool {
        self.imag.is_some()
    }
}

pub(super) fn read_error(format: &str, reason: NumpyReadFailure) -> NumpyReadError {
    NumpyReadError {
        format: format.to_owned(),
        reason,
    }
}

pub fn decode_npy(
    bytes: &[u8],
    max_values: usize,
    format: &str,
) -> Result<NpyArray, NumpyReadError> {
    use npyz::{DType, Order, TypeChar};
    let file = npyz::NpyFile::new(Cursor::new(bytes))
        .map_err(|error| read_error(format, NumpyReadFailure::Header(error)))?;
    let shape = file
        .shape()
        .iter()
        .map(|value| {
            usize::try_from(*value)
                .map_err(|_| read_error(format, NumpyReadFailure::DimensionTooLarge(*value)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if shape.is_empty() || shape.len() > 2 {
        return Err(read_error(
            format,
            NumpyReadFailure::WaveformDimensions(shape),
        ));
    }
    let count = shape
        .iter()
        .try_fold(1_usize, |count, dim| count.checked_mul(*dim))
        .ok_or_else(|| {
            read_error(
                format,
                NumpyReadFailure::ShapeProductOverflow(shape.clone()),
            )
        })?;
    if count > max_values {
        return Err(read_error(
            format,
            NumpyReadFailure::NumericValueLimit {
                values: count,
                limit: max_values,
            },
        ));
    }
    let fortran = file.order() == Order::Fortran;
    let dtype = file.dtype();
    let DType::Plain(type_string) = dtype else {
        return Err(read_error(
            format,
            NumpyReadFailure::StructuredDtype(Box::new(dtype)),
        ));
    };
    macro_rules! real {
        ($ty:ty) => {{
            let values = file
                .into_vec::<$ty>()
                .map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
            NpyArray {
                shape,
                fortran,
                real: values.into_iter().map(|value| value as f64).collect(),
                imag: None,
            }
        }};
    }
    macro_rules! complex {
        ($ty:ty) => {{
            let values = file
                .into_vec::<$ty>()
                .map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
            NpyArray {
                shape,
                fortran,
                real: values.iter().map(|value| value.re as f64).collect(),
                imag: Some(values.iter().map(|value| value.im as f64).collect()),
            }
        }};
    }
    let array = match (type_string.type_char(), type_string.size_field()) {
        (TypeChar::Float, 4) => real!(f32),
        (TypeChar::Float, 8) => real!(f64),
        (TypeChar::Int, 1) => real!(i8),
        (TypeChar::Int, 2) => real!(i16),
        (TypeChar::Int, 4) => real!(i32),
        (TypeChar::Int, 8) => {
            let values = file
                .into_vec::<i64>()
                .map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
            NpyArray {
                shape,
                fortran,
                real: values
                    .into_iter()
                    .map(|value| {
                        crate::numeric::exact_signed_integer("NPY array", value).map_err(|detail| {
                            read_error(format, NumpyReadFailure::InexactInteger(detail))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                imag: None,
            }
        }
        (TypeChar::Uint, 1) => real!(u8),
        (TypeChar::Uint, 2) => real!(u16),
        (TypeChar::Uint, 4) => real!(u32),
        (TypeChar::Uint, 8) => {
            let values = file
                .into_vec::<u64>()
                .map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
            NpyArray {
                shape,
                fortran,
                real: values
                    .into_iter()
                    .map(|value| {
                        crate::numeric::exact_unsigned_integer("NPY array", value).map_err(
                            |detail| read_error(format, NumpyReadFailure::InexactInteger(detail)),
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                imag: None,
            }
        }
        (TypeChar::Bool, 1) => {
            let values = file
                .into_vec::<bool>()
                .map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
            NpyArray {
                shape,
                fortran,
                real: values
                    .into_iter()
                    .map(|value| if value { 1.0 } else { 0.0 })
                    .collect(),
                imag: None,
            }
        }
        (TypeChar::Complex, 8) => complex!(num_complex::Complex32),
        (TypeChar::Complex, 16) => complex!(num_complex::Complex64),
        (kind, size) => {
            return Err(read_error(
                format,
                NumpyReadFailure::UnsupportedDtype { kind, size },
            ));
        }
    };
    Ok(array)
}

pub(super) fn npy_vector(
    array: &NpyArray,
    format: &str,
    name: &str,
) -> Result<(Vec<f64>, Option<Vec<f64>>), NumpyReadError> {
    let len = match array.shape.as_slice() {
        [len] => *len,
        [rows, 1] => *rows,
        [1, columns] => *columns,
        _ => {
            return Err(read_error(
                format,
                NumpyReadFailure::NonVector {
                    name: name.to_owned(),
                    shape: array.shape.clone(),
                },
            ));
        }
    };
    if array.real.len() != len || array.imag.as_ref().is_some_and(|imag| imag.len() != len) {
        return Err(read_error(
            format,
            NumpyReadFailure::PayloadShape {
                name: name.to_owned(),
                shape: array.shape.clone(),
                real: array.real.len(),
                imag: array.imag.as_ref().map(Vec::len),
            },
        ));
    }
    Ok((array.real.clone(), array.imag.clone()))
}

/// Interpret column zero as the coordinate in every multi-column table,
/// including complex matrices written by `matrix::encode_npy`. A vector or
/// single-column table has no explicit coordinate and uses sample indices.
pub fn npy_matrix_to_dataset(
    array: NpyArray,
    max_rows: usize,
    max_columns: usize,
    format: &str,
) -> Result<DecodedNumericDataset, NumpyReadError> {
    let (rows, columns) = match array.shape.as_slice() {
        [rows] => (*rows, 1),
        [rows, columns] => (*rows, *columns),
        _ => unreachable!(),
    };
    if !(1..=max_rows).contains(&rows) || columns == 0 || columns > max_columns {
        return Err(read_error(
            format,
            NumpyReadFailure::WaveformBounds {
                shape: array.shape,
                max_rows,
                max_columns,
            },
        ));
    }
    let index = |row: usize, column: usize| {
        if array.fortran {
            column * rows + row
        } else {
            row * columns + column
        }
    };
    if columns >= 2 {
        if let Some(imag) = &array.imag
            && let Some(sample) = (0..rows).find(|row| imag[index(*row, 0)] != 0.0)
        {
            return Err(read_error(
                format,
                NumpyReadFailure::ImaginaryMatrixCoordinate { sample },
            ));
        }
        let coordinate = (0..rows).map(|row| array.real[index(row, 0)]).collect();
        let signals = (1..columns)
            .map(|column| DecodedNumericSignal {
                name: format!("signal_{column}"),
                real: (0..rows)
                    .map(|row| array.real[index(row, column)])
                    .collect(),
                imag: array
                    .imag
                    .as_ref()
                    .map(|imag| (0..rows).map(|row| imag[index(row, column)]).collect()),
                unit: None,
            })
            .collect();
        return Ok(DecodedNumericDataset {
            coordinate_unit: None,
            domain: crate::WaveformDomain::DcSweep,
            coordinate_name: "sample".into(),
            coordinate,
            signals,
        });
    }
    let coordinate = (0..rows).map(|row| row as f64).collect();
    let signals = (0..columns)
        .map(|column| DecodedNumericSignal {
            name: if columns == 1 {
                "value".to_owned()
            } else {
                format!("signal_{}", column + 1)
            },
            real: (0..rows)
                .map(|row| array.real[index(row, column)])
                .collect(),
            imag: array
                .imag
                .as_ref()
                .map(|imag| (0..rows).map(|row| imag[index(row, column)]).collect()),
            unit: None,
        })
        .collect();
    Ok(DecodedNumericDataset {
        coordinate_unit: None,
        domain: crate::WaveformDomain::DcSweep,
        coordinate_name: "sample".into(),
        coordinate,
        signals,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use npyz::WriterBuilder as _;

    #[test]
    fn complex_coordinate_first_matrices_preserve_axes_and_rectangular_samples() {
        use num_complex::Complex64;
        let rows = [
            [
                Complex64::new(-0.0, 0.0),
                Complex64::new(3.0, 4.0),
                Complex64::new(-0.0, -2.0),
            ],
            [
                Complex64::new(1e-9, -0.0),
                Complex64::new(f64::NAN, f64::NAN),
                Complex64::new(2.0, 1.0),
            ],
            [
                Complex64::new(3e-9, 0.0),
                Complex64::new(1.0, -1.0),
                Complex64::new(0.0, -0.0),
            ],
        ];
        for fortran in [false, true] {
            let mut bytes = Vec::new();
            let mut writer = npyz::WriteOptions::<Complex64>::new()
                .default_dtype()
                .shape(&[3, 3])
                .order(if fortran {
                    npyz::Order::Fortran
                } else {
                    npyz::Order::C
                })
                .writer(&mut bytes)
                .begin_nd()
                .unwrap();
            if fortran {
                writer
                    .extend((0..3).flat_map(|column| rows.iter().map(move |row| row[column])))
                    .unwrap();
            } else {
                writer.extend(rows.iter().flatten().copied()).unwrap();
            }
            writer.finish().unwrap();
            let array = decode_npy(&bytes, 18, "numpy_npy").unwrap();
            let decoded = npy_matrix_to_dataset(array, 3, 3, "numpy_npy").unwrap();
            assert_eq!(
                decoded
                    .coordinate
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>(),
                rows.iter()
                    .map(|row| row[0].re.to_bits())
                    .collect::<Vec<_>>()
            );
            assert_eq!(decoded.signals.len(), 2);
            for (index, signal) in decoded.signals.iter().enumerate() {
                assert_eq!(signal.name, format!("signal_{}", index + 1));
                for (row, expected) in rows.iter().enumerate() {
                    assert_eq!(signal.real[row].to_bits(), expected[index + 1].re.to_bits());
                    assert_eq!(
                        signal.imag.as_ref().unwrap()[row].to_bits(),
                        expected[index + 1].im.to_bits()
                    );
                }
            }
        }
    }

    #[test]
    fn matrix_coordinates_cannot_silently_discard_an_imaginary_component() {
        for imaginary in [1.0, f64::NAN, f64::INFINITY] {
            let bytes = crate::numpy::encode_complex_array(
                &[2, 2],
                &[
                    num_complex::Complex64::new(0.0, 0.0),
                    num_complex::Complex64::new(1.0, 2.0),
                    num_complex::Complex64::new(1.0, imaginary),
                    num_complex::Complex64::new(3.0, 4.0),
                ],
            )
            .unwrap();
            let array = decode_npy(&bytes, 8, "numpy_npy").unwrap();
            let error = npy_matrix_to_dataset(array, 2, 2, "numpy_npy").unwrap_err();
            assert!(error.to_string().contains("coordinate column"), "{error}");
            assert!(error.to_string().contains("sample 1"), "{error}");
        }
    }

    #[test]
    fn projects_fortran_order_without_transposing_samples() {
        let mut bytes = Vec::new();
        let mut writer = npyz::WriteOptions::<f64>::new()
            .default_dtype()
            .shape(&[3, 2])
            .order(npyz::Order::Fortran)
            .writer(&mut bytes)
            .begin_nd()
            .expect("NPY writer");
        writer
            .extend([0.0, 1.0, 2.0, 10.0, 20.0, 30.0])
            .expect("NPY values");
        writer.finish().expect("NPY finish");

        let array = decode_npy(&bytes, 6, "numpy_npy").expect("decode");
        let DecodedNumericDataset {
            coordinate,
            signals,
            ..
        } = npy_matrix_to_dataset(array, 3, 2, "numpy_npy").expect("project");
        assert_eq!(coordinate, [0.0, 1.0, 2.0]);
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].real, [10.0, 20.0, 30.0]);
    }

    #[test]
    fn projects_complex_columns_with_implicit_sample_coordinate() {
        let values = [
            num_complex::Complex64::new(1.0, -2.0),
            num_complex::Complex64::new(3.0, 4.0),
        ];
        let bytes = crate::numpy::encode_complex_array(&[2], &values).expect("NPY fixture");
        let array = decode_npy(&bytes, 2, "numpy_npy").expect("decode");
        let DecodedNumericDataset {
            coordinate,
            signals,
            ..
        } = npy_matrix_to_dataset(array, 2, 1, "numpy_npy").expect("project");
        assert_eq!(coordinate, [0.0, 1.0]);
        assert_eq!(signals[0].name, "value");
        assert_eq!(signals[0].real, [1.0, 3.0]);
        assert_eq!(signals[0].imag.as_deref(), Some(&[-2.0, 4.0][..]));
    }

    #[test]
    fn rejects_integer_values_that_would_lose_precision() {
        let mut bytes = Vec::new();
        let mut writer = npyz::WriteOptions::<u64>::new()
            .default_dtype()
            .shape(&[1])
            .writer(&mut bytes)
            .begin_nd()
            .expect("NPY writer");
        writer
            .extend([crate::numeric::MAX_EXACT_F64_INTEGER + 1])
            .expect("NPY value");
        writer.finish().expect("NPY finish");

        let error = decode_npy(&bytes, 1, "numpy_npy").expect_err("precision loss");
        assert!(
            matches!(&error.reason, NumpyReadFailure::InexactInteger(crate::numeric::ExactIntegerError::Unsigned { value, .. }) if *value == crate::numeric::MAX_EXACT_F64_INTEGER + 1)
        );
        assert!(
            error
                .to_string()
                .contains("cannot be represented exactly as f64")
        );
    }

    #[test]
    fn malformed_npy_retains_the_header_io_cause() {
        use std::error::Error as _;
        let error = decode_npy(b"invalid", 2, "numpy_npy").unwrap_err();
        assert!(matches!(&error.reason, NumpyReadFailure::Header(_)));
        assert!(error.reason.source().unwrap().is::<std::io::Error>());
        assert!(
            error
                .to_string()
                .starts_with("numpy_npy import: invalid NPY header: ")
        );
    }
}
