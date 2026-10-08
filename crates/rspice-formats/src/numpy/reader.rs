//! Bounded NPY decoding and projection into numeric waveform columns.

use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal};
mod header;

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
    NumericValueCountOverflow,
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
            Self::NumericValueLimit { values, limit } => write!(
                f,
                "NPY decoding requires {values} numeric values; the remaining limit is {limit}"
            ),
            Self::NumericValueCountOverflow => f.write_str("NPY numeric-value count overflow"),
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
    max_values: usize,
    shape: Vec<usize>,
    fortran: bool,
    real: Vec<f64>,
    imag: Option<Vec<f64>>,
}

impl NpyArray {
    pub(super) fn is_complex(&self) -> bool {
        self.imag.is_some()
    }

    pub(super) fn numeric_values(&self) -> usize {
        // Both arrays are admitted together before decoding; their sum fits.
        self.real.len() + self.imag.as_ref().map_or(0, Vec::len)
    }
}

pub(super) fn read_error(format: &str, reason: NumpyReadFailure) -> NumpyReadError {
    NumpyReadError {
        format: format.to_owned(),
        reason,
    }
}

fn check_numeric_values(
    values: Option<usize>,
    limit: usize,
    format: &str,
) -> Result<(), NumpyReadError> {
    let values =
        values.ok_or_else(|| read_error(format, NumpyReadFailure::NumericValueCountOverflow))?;
    if values > limit {
        return Err(read_error(
            format,
            NumpyReadFailure::NumericValueLimit { values, limit },
        ));
    }
    Ok(())
}

/// Decode at most `max_values` f64 components (two per complex element).
/// The same budget also applies when constructing an implicit coordinate.
pub fn decode_npy(
    bytes: &[u8],
    max_values: usize,
    format: &str,
) -> Result<NpyArray, NumpyReadError> {
    use npyz::{DType, Order, TypeChar};
    // npyz multiplies dimensions and allocates the declared header before it
    // returns control. Validate those operations before entering the reader.
    let (shape, count) = header::preflight(bytes, format)?;
    let mut payload = bytes;
    let header = npyz::NpyHeader::from_reader(&mut payload)
        .map_err(|error| read_error(format, NumpyReadFailure::Header(error)))?;
    let fortran = header.order() == Order::Fortran;
    let dtype = header.dtype();
    let DType::Plain(type_string) = dtype else {
        return Err(read_error(
            format,
            NumpyReadFailure::StructuredDtype(Box::new(dtype)),
        ));
    };
    let components = if type_string.type_char() == TypeChar::Complex {
        2
    } else {
        1
    };
    check_numeric_values(count.checked_mul(components), max_values, format)?;
    // Check payload availability before reserving output arrays. Header-only
    // files cannot make a decoder reserve memory for nonexistent samples.
    if let Some(size) = type_string.num_bytes()
        && size > 0
        && count > payload.len() / size
    {
        return Err(read_error(
            format,
            NumpyReadFailure::Values(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "NPY payload is shorter than its declared shape and dtype",
            )),
        ));
    }
    let file = npyz::NpyFile::with_header(header, payload);
    macro_rules! values {
        ($ty:ty) => {
            file.data::<$ty>().map_err(|error| {
                read_error(
                    format,
                    NumpyReadFailure::Values(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        error,
                    )),
                )
            })?
        };
    }
    macro_rules! real {
        ($ty:ty) => {{
            let real = values!($ty)
                .map(|value| value.map(|value| value as f64))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
            NpyArray {
                max_values,
                shape,
                fortran,
                real,
                imag: None,
            }
        }};
    }
    macro_rules! complex {
        ($ty:ty) => {{
            let mut real = Vec::with_capacity(count);
            let mut imag = Vec::with_capacity(count);
            for value in values!($ty) {
                let value =
                    value.map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
                real.push(value.re as f64);
                imag.push(value.im as f64);
            }
            NpyArray {
                max_values,
                shape,
                fortran,
                real,
                imag: Some(imag),
            }
        }};
    }
    let array = match (type_string.type_char(), type_string.size_field()) {
        (TypeChar::Float, 4) => real!(f32),
        (TypeChar::Float, 8) => real!(f64),
        (TypeChar::Int, 1) => real!(i8),
        (TypeChar::Int, 2) => real!(i16),
        (TypeChar::Int, 4) => real!(i32),
        (TypeChar::Int, 8) => NpyArray {
            max_values,
            shape,
            fortran,
            real: values!(i64)
                .map(|value| {
                    let value = value
                        .map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
                    crate::numeric::exact_signed_integer("NPY array", value).map_err(|detail| {
                        read_error(format, NumpyReadFailure::InexactInteger(detail))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
            imag: None,
        },
        (TypeChar::Uint, 1) => real!(u8),
        (TypeChar::Uint, 2) => real!(u16),
        (TypeChar::Uint, 4) => real!(u32),
        (TypeChar::Uint, 8) => NpyArray {
            max_values,
            shape,
            fortran,
            real: values!(u64)
                .map(|value| {
                    let value = value
                        .map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
                    crate::numeric::exact_unsigned_integer("NPY array", value).map_err(|detail| {
                        read_error(format, NumpyReadFailure::InexactInteger(detail))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
            imag: None,
        },
        (TypeChar::Bool, 1) => {
            let real = values!(bool)
                .map(|value| value.map(|value| if value { 1.0 } else { 0.0 }))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| read_error(format, NumpyReadFailure::Values(error)))?;
            NpyArray {
                max_values,
                shape,
                fortran,
                real,
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
    array: NpyArray,
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
    Ok((array.real, array.imag))
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
    let signal_columns = if columns >= 2 { columns - 1 } else { 1 };
    let components = if array.is_complex() { 2 } else { 1 };
    let projected_values = signal_columns
        .checked_mul(components)
        .and_then(|count| count.checked_add(1))
        .and_then(|count| rows.checked_mul(count));
    check_numeric_values(projected_values, array.max_values, format)?;
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
    let signals = vec![DecodedNumericSignal {
        name: "value".to_owned(),
        real: array.real,
        imag: array.imag,
        unit: None,
    }];
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
    fn compact_dtypes_expand_to_exact_f64_components_within_the_budget() {
        macro_rules! encoded {
            ($ty:ty, $values:expr) => {{
                let mut bytes = Vec::new();
                let mut writer = npyz::WriteOptions::<$ty>::new()
                    .default_dtype()
                    .shape(&[2])
                    .writer(&mut bytes)
                    .begin_nd()
                    .unwrap();
                writer.extend($values).unwrap();
                writer.finish().unwrap();
                bytes
            }};
        }
        for (bytes, expected) in [
            (encoded!(i8, [-128, 127]), [-128.0, 127.0]),
            (encoded!(i16, [-32768, 32767]), [-32768.0, 32767.0]),
            (
                encoded!(i32, [i32::MIN, i32::MAX]),
                [i32::MIN as f64, i32::MAX as f64],
            ),
            (encoded!(i64, [-1, 2]), [-1.0, 2.0]),
            (encoded!(u8, [0, 255]), [0.0, 255.0]),
            (encoded!(u16, [0, 65535]), [0.0, 65535.0]),
            (encoded!(u32, [0, u32::MAX]), [0.0, u32::MAX as f64]),
            (encoded!(u64, [0, 1u64 << 53]), [0.0, (1u64 << 53) as f64]),
            (encoded!(f32, [-0.0, 1.25]), [-0.0, 1.25]),
            (encoded!(bool, [false, true]), [0.0, 1.0]),
        ] {
            let decoded = decode_npy(&bytes, 2, "numpy_npy").unwrap();
            assert_eq!(
                decoded.real.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                expected.map(f64::to_bits)
            );
            assert!(decoded.imag.is_none());
            assert!(matches!(
                decode_npy(&bytes, 1, "numpy_npy").unwrap_err().reason,
                NumpyReadFailure::NumericValueLimit {
                    values: 2,
                    limit: 1
                }
            ));
        }
        let bytes = encoded!(
            num_complex::Complex32,
            [num_complex::Complex32::new(1.25, -0.0); 2]
        );
        let decoded = decode_npy(&bytes, 4, "numpy_npy").unwrap();
        assert_eq!(decoded.real, [1.25, 1.25]);
        assert_eq!(decoded.imag.unwrap()[0].to_bits(), (-0.0_f64).to_bits());
        assert!(matches!(
            decode_npy(&bytes, 3, "numpy_npy").unwrap_err().reason,
            NumpyReadFailure::NumericValueLimit {
                values: 4,
                limit: 3
            }
        ));
    }

    #[test]
    fn complex_components_count_toward_the_decode_budget_before_reading_samples() {
        let bytes =
            crate::numpy::encode_complex_array(&[2], &[num_complex::Complex64::new(1.0, -0.0); 2])
                .unwrap();
        for source in [&bytes[..], &bytes[..bytes.len() - 1]] {
            let error = decode_npy(source, 3, "numpy_npy").unwrap_err();
            assert!(matches!(
                error.reason,
                NumpyReadFailure::NumericValueLimit {
                    values: 4,
                    limit: 3
                }
            ));
        }
        let array = decode_npy(&bytes, 4, "numpy_npy").unwrap();
        assert_eq!(array.real, [1.0, 1.0]);
        assert_eq!(array.imag.unwrap()[0].to_bits(), (-0.0_f64).to_bits());
    }

    #[test]
    fn implicit_coordinates_count_toward_the_projected_decode_budget() {
        let bytes = crate::numpy::encode_real_array(&[2], &[5.0, 6.0]).unwrap();
        let array = decode_npy(&bytes, 3, "numpy_npy").unwrap();
        let error = npy_matrix_to_dataset(array, 2, 2, "numpy_npy").unwrap_err();
        assert!(matches!(
            error.reason,
            NumpyReadFailure::NumericValueLimit {
                values: 4,
                limit: 3
            }
        ));
        let array = decode_npy(&bytes, 4, "numpy_npy").unwrap();
        let dataset = npy_matrix_to_dataset(array, 2, 2, "numpy_npy").unwrap();
        assert_eq!(dataset.coordinate, [0.0, 1.0]);
        assert_eq!(dataset.signals[0].real, [5.0, 6.0]);
    }

    #[test]
    fn overflowing_header_dimensions_are_refused_without_panicking() {
        for fortran in [false, true] {
            let text = format!(
                "{{'descr': '<f8', 'fortran_order': {}, 'shape': (18446744073709551615, 2)}}\n",
                if fortran { "True" } else { "False" }
            );
            let mut bytes = b"\x93NUMPY\x01\x00".to_vec();
            bytes.extend_from_slice(&(text.len() as u16).to_le_bytes());
            bytes.extend_from_slice(text.as_bytes());
            let error = decode_npy(&bytes, 1024, "numpy_npy").unwrap_err();
            assert!(matches!(
                error.reason,
                NumpyReadFailure::ShapeProductOverflow(_) | NumpyReadFailure::DimensionTooLarge(_)
            ));
        }
    }

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
        let array = decode_npy(&bytes, 6, "numpy_npy").expect("decode");
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
