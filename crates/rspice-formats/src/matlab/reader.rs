//! Bounded MATLAB numeric parsing for import adapters: MAT-file Level 5, and the Level 4
//! layout older tools and `save -v4` still write.

use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal, stated_coordinate_names};
use std::io::Cursor;

mod level4;
mod structure;

#[derive(Debug, Clone, Copy)]
pub struct MatlabReadLimits<'a> {
    pub max_variables: usize,
    /// What the file's compressed variables may expand to, together.
    pub max_expanded_bytes: u64,
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
    /// The file ends inside its `variable`-th element (counted from one).
    Truncated {
        variable: usize,
    },
    /// A part of a variable does not fit inside it.
    Malformed,
    ExpandedByteLimit {
        expanded: u64,
        limit: u64,
    },
    /// A compressed variable's stream does not expand to the size it declares.
    CompressedSize,
    DimensionOverflow {
        dimensions: Vec<i32>,
    },
    NegativeDimension {
        dimension: i32,
    },
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
    /// The `variable`-th Level 4 header (counted from one) does not read as one.
    Level4Header {
        variable: usize,
    },
    /// A variable holds something other than numbers, which a waveform cannot be.
    NotNumeric {
        name: String,
        what: &'static str,
    },
}

impl std::fmt::Display for MatlabReadFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParserPanicked => f.write_str("MATLAB parser rejected malformed input"),
            Self::Parse(_) => f.write_str("the file is not a MATLAB v5 file this importer can read"),
            Self::Truncated { variable } => write!(f, "the MATLAB file is cut short inside variable {variable}"),
            Self::Malformed => f.write_str("a MATLAB variable's parts do not fit inside it"),
            Self::ExpandedByteLimit { expanded, limit } => write!(f, "compressed MATLAB variables expand to at least {expanded} bytes, past the import limit of {limit} bytes"),
            Self::CompressedSize => f.write_str("a compressed MATLAB variable does not expand to the size it declares"),
            Self::DimensionOverflow { dimensions } => write!(f, "MATLAB variable dimensions {dimensions:?} multiply past the largest array size"),
            Self::NegativeDimension { dimension } => write!(f, "MATLAB variable has a negative dimension {dimension}"),
            Self::VariableLimit { .. } => f.write_str("too many MATLAB variables"),
            Self::InexactInteger(source) => source.fmt(f),
            Self::ShapeOverflow { name, .. } => write!(f, "MATLAB variable '{name}' shape overflows"),
            Self::PayloadShape { name, .. } => write!(f, "MATLAB variable '{name}' payload does not match its shape"),
            Self::CoordinateNotRealVector { .. } => f.write_str("MATLAB coordinate variable must be a real vector"),
            Self::NonVector { name, .. } => write!(f, "MATLAB variable '{name}' is not a vector"),
            Self::MissingCoordinate { expected } => write!(f, "MATLAB file requires a coordinate variable named {expected}"),
            Self::TableShape { .. } => f.write_str("without a named coordinate, MATLAB data must be one rows-by-columns table with the coordinate in column one"),
            Self::ComplexTable { .. } => f.write_str("a complex MATLAB table requires separate named coordinate and signal variables"),
            Self::Level4Header { variable } => write!(f, "MATLAB variable {variable} does not start with a Level 4 header"),
            Self::NotNumeric { name, what } => write!(f, "MATLAB variable '{name}' is {what}, not numbers"),
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
    fn parse(
        bytes: &[u8],
        max_variables: usize,
        max_expanded_bytes: u64,
        format: &str,
    ) -> Result<Self, MatlabReadError> {
        // The parser trusts every size a file states and drops a variable it
        // cannot read whole without saying so; the walk refuses those first.
        // `catch_unwind` stays for what the walk does not cover, but it cannot
        // be the guard: release web builds abort on panic.
        structure::check(bytes, max_expanded_bytes)
            .map_err(|reason| adapter_error(format, reason))?;
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

/// What a MATLAB workspace holds, read as one dataset, and what of it was left out.
#[derive(Debug)]
pub struct DecodedMatlab {
    pub dataset: DecodedNumericDataset,
    /// A sentence for each variable the dataset does not hold, saying why.
    pub notes: Vec<String>,
}

/// One variable of a MATLAB workspace, as numbers.
struct NamedArray {
    name: String,
    size: Vec<usize>,
    real: Vec<f64>,
    imag: Option<Vec<f64>>,
}

impl NamedArray {
    fn is_vector(&self) -> bool {
        self.size.iter().filter(|dimension| **dimension > 1).count() <= 1
    }
}

/// Decode MATLAB numeric variables using their named coordinate or table layout.
pub fn decode_matlab_v5(
    bytes: &[u8],
    limits: MatlabReadLimits<'_>,
    format: &str,
) -> Result<DecodedNumericDataset, MatlabReadError> {
    decode_matlab_v5_workspace(bytes, limits, format).map(|decoded| decoded.dataset)
}

/// [`decode_matlab_v5`], with a note for each variable the dataset leaves out.
pub fn decode_matlab_v5_workspace(
    bytes: &[u8],
    limits: MatlabReadLimits<'_>,
    format: &str,
) -> Result<DecodedMatlab, MatlabReadError> {
    let parsed = MatFile::parse(
        bytes,
        limits.max_variables,
        limits.max_expanded_bytes,
        format,
    )?;
    let arrays = parsed
        .arrays()
        .into_iter()
        .map(|array| {
            let (real, imag) = array.values(format)?;
            Ok(NamedArray {
                name: array.name().to_owned(),
                size: array.size().to_vec(),
                real,
                imag,
            })
        })
        .collect::<Result<Vec<_>, MatlabReadError>>()?;
    assemble(arrays, limits, format)
}

/// Whether `bytes` open as a Level 4 MAT-file, which has no signature of its own.
pub fn looks_like_matlab_v4(bytes: &[u8]) -> bool {
    level4::looks_like_level4(bytes)
}

/// Decode a Level 4 MAT-file by the rules a Level 5 one is read by.
pub fn decode_matlab_v4(
    bytes: &[u8],
    limits: MatlabReadLimits<'_>,
    format: &str,
) -> Result<DecodedMatlab, MatlabReadError> {
    assemble(
        level4::parse(bytes, limits.max_variables, format)?,
        limits,
        format,
    )
}

fn assemble(
    mut arrays: Vec<NamedArray>,
    limits: MatlabReadLimits<'_>,
    format: &str,
) -> Result<DecodedMatlab, MatlabReadError> {
    let coordinate_index = arrays.iter().position(|array| {
        limits
            .coordinate_names
            .iter()
            .any(|name| array.name.eq_ignore_ascii_case(name))
    });
    if let Some(coordinate_index) = coordinate_index {
        let coordinate_array = arrays.remove(coordinate_index);
        if coordinate_array.imag.is_some() || !coordinate_array.is_vector() {
            return Err(adapter_error(
                format,
                MatlabReadFailure::CoordinateNotRealVector {
                    complex: coordinate_array.imag.is_some(),
                    name: coordinate_array.name,
                    shape: coordinate_array.size,
                },
            ));
        }
        let mut signals = Vec::new();
        let mut notes = Vec::new();
        for array in arrays {
            // A workspace saves its settings beside its signals: a gain, a step count. One
            // value beside a longer coordinate is one of those, not a sample of a signal.
            if array.real.len() == 1 && coordinate_array.real.len() != 1 {
                notes.push(format!(
                    "The MATLAB variable '{}' holds one value, not a signal over '{}', so it is left out.",
                    array.name, coordinate_array.name
                ));
                continue;
            }
            if !array.is_vector() {
                return Err(adapter_error(
                    format,
                    MatlabReadFailure::NonVector {
                        name: array.name,
                        shape: array.size,
                    },
                ));
            }
            signals.push(DecodedNumericSignal {
                name: array.name,
                real: array.real,
                imag: array.imag,
                unit: None,
            });
        }
        return Ok(DecodedMatlab {
            dataset: DecodedNumericDataset {
                coordinate_unit: None,
                domain: crate::WaveformDomain::from_coordinate_name(&coordinate_array.name),
                coordinate_name: coordinate_array.name,
                coordinate: coordinate_array.real,
                signals,
            },
            notes,
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
    let array = arrays.remove(0);
    let size = &array.size;
    if size.len() != 2 || size[0] < limits.min_rows || size[1] < 2 {
        return Err(adapter_error(
            format,
            MatlabReadFailure::TableShape {
                shape: size.to_vec(),
                min_rows: limits.min_rows,
            },
        ));
    }
    let (real, imag) = (&array.real, &array.imag);
    if imag.is_some() {
        return Err(adapter_error(
            format,
            MatlabReadFailure::ComplexTable {
                name: array.name.clone(),
            },
        ));
    }
    let rows = size[0];
    let columns = size[1];
    let coordinate = real[..rows].to_vec();
    let signals = (1..columns)
        .map(|column| DecodedNumericSignal {
            name: format!("{}.{}", array.name, column),
            real: real[column * rows..(column + 1) * rows].to_vec(),
            imag: None,
            unit: None,
        })
        .collect();
    Ok(DecodedMatlab {
        dataset: DecodedNumericDataset {
            coordinate_unit: None,
            domain: crate::WaveformDomain::DcSweep,
            coordinate_name: "x".into(),
            coordinate,
            signals,
        },
        notes: Vec::new(),
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
        let parsed = MatFile::parse(&bytes, 1, 0, "matlab_v5").expect("parse");
        let arrays = parsed.arrays();
        assert_eq!(arrays.len(), 1);
        assert_eq!(arrays[0].name(), "V_out");
        assert_eq!(arrays[0].size(), [2, 1]);
        let (real, imag) = arrays[0].values("matlab_v5").expect("values");
        assert_eq!(real, [1.0, 3.0]);
        assert_eq!(imag.as_deref(), Some(&[-2.0, 4.0][..]));

        let error = MatFile::parse(&bytes, 0, 0, "matlab_v5")
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
            max_expanded_bytes: 0,
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
            max_expanded_bytes: 0,
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

    const LIMITS: MatlabReadLimits<'static> = MatlabReadLimits {
        max_variables: 8,
        max_expanded_bytes: 64 * 1024 * 1024,
        min_rows: 1,
        coordinate_names: &["time", "t"],
    };

    fn variable(name: &str, real: Vec<f64>) -> MatVariable {
        MatVariable {
            name: name.into(),
            real,
            imag: None,
        }
    }

    fn file_of(variables: &[MatVariable]) -> Vec<u8> {
        write_mat_v5("MATLAB 5.0 MAT-file, refusal fixture", variables).unwrap()
    }

    /// A top-level miCOMPRESSED element whose stream expands to `inner`.
    fn compressed_element(inner: &[u8]) -> Vec<u8> {
        use std::io::Write as _;
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(inner).unwrap();
        let stream = encoder.finish().unwrap();
        let mut element = 15_u32.to_le_bytes().to_vec();
        element.extend_from_slice(&u32::try_from(stream.len()).unwrap().to_le_bytes());
        element.extend_from_slice(&stream);
        element
    }

    /// The same file with every variable compressed, as MATLAB writes it.
    fn compressed_file_of(variables: &[MatVariable]) -> Vec<u8> {
        let mut bytes = file_of(&[])[..crate::matlab::HEADER_BYTES].to_vec();
        for variable in variables {
            let plain = file_of(std::slice::from_ref(variable));
            bytes.extend(compressed_element(&plain[crate::matlab::HEADER_BYTES..]));
        }
        bytes
    }

    #[test]
    fn compressed_variables_read_like_plain_ones() {
        let variables = [
            variable("time", vec![0.0, 1.0, 2.0]),
            variable("out", vec![4.0, 5.0, 6.0]),
        ];
        let decoded = decode_matlab_v5(&compressed_file_of(&variables), LIMITS, "matlab_v5")
            .expect("compressed MAT file");
        assert_eq!(decoded.coordinate, [0.0, 1.0, 2.0]);
        assert_eq!(decoded.signals.len(), 1);
        assert_eq!(decoded.signals[0].real, [4.0, 5.0, 6.0]);
    }

    #[test]
    fn a_compressed_variable_declared_past_the_limit_is_refused_before_it_inflates() {
        // The stream holds a matrix tag declaring 1 GiB and nothing else. A
        // reader that inflated past the tag would find the stream short; the
        // refusal names the declared size, so it was refused on the tag alone
        // and nothing the size of the declaration was ever allocated.
        let declared = 1_u32 << 30;
        let mut inner = 14_u32.to_le_bytes().to_vec();
        inner.extend_from_slice(&declared.to_le_bytes());
        let mut bytes = file_of(&[])[..crate::matlab::HEADER_BYTES].to_vec();
        bytes.extend(compressed_element(&inner));
        let error = decode_matlab_v5(&bytes, LIMITS, "matlab_v5").unwrap_err();
        assert!(
            matches!(
                error.reason,
                MatlabReadFailure::ExpandedByteLimit { expanded, limit }
                    if expanded == 8 + u64::from(declared) && limit == LIMITS.max_expanded_bytes
            ),
            "{error}"
        );
        assert_eq!(
            error.to_string(),
            "matlab_v5 import: compressed MATLAB variables expand to at least 1073741832 bytes, \
             past the import limit of 67108864 bytes"
        );
    }

    #[test]
    fn compressed_variables_count_against_the_limit_together() {
        let variables = [
            variable("time", vec![0.0; 64]),
            variable("out", vec![1.0; 64]),
        ];
        let bytes = compressed_file_of(&variables);
        let one = file_of(&variables[..1]).len() - crate::matlab::HEADER_BYTES;
        let limits = MatlabReadLimits {
            max_expanded_bytes: one as u64 + 8,
            ..LIMITS
        };
        let error = decode_matlab_v5(&bytes, limits, "matlab_v5").unwrap_err();
        assert!(
            matches!(error.reason, MatlabReadFailure::ExpandedByteLimit { .. }),
            "{error}"
        );
    }

    #[test]
    fn a_compressed_stream_that_runs_past_its_declared_size_is_refused() {
        let plain = file_of(&[variable("time", vec![0.0, 1.0])]);
        let mut inner = plain[crate::matlab::HEADER_BYTES..].to_vec();
        inner.resize(inner.len() + 4 * 1024 * 1024, 0);
        let mut bytes = plain[..crate::matlab::HEADER_BYTES].to_vec();
        bytes.extend(compressed_element(&inner));
        let error = decode_matlab_v5(&bytes, LIMITS, "matlab_v5").unwrap_err();
        assert!(
            matches!(error.reason, MatlabReadFailure::CompressedSize),
            "{error}"
        );
    }

    #[test]
    fn a_file_cut_short_is_refused_rather_than_read_without_its_last_variable() {
        let variables = [
            variable("time", vec![0.0, 1.0]),
            variable("a", vec![2.0, 3.0]),
            variable("b", vec![4.0, 5.0]),
        ];
        for bytes in [file_of(&variables), compressed_file_of(&variables)] {
            assert_eq!(
                decode_matlab_v5(&bytes, LIMITS, "matlab_v5")
                    .unwrap()
                    .signals
                    .len(),
                2
            );
            for cut in [1, 9, 40] {
                let error =
                    decode_matlab_v5(&bytes[..bytes.len() - cut], LIMITS, "matlab_v5").unwrap_err();
                assert!(
                    matches!(error.reason, MatlabReadFailure::Truncated { variable: 3 }),
                    "cut {cut}: {error}"
                );
            }
        }
        // A file that stops inside the tag of a fourth element.
        let mut bytes = file_of(&variables);
        bytes.extend_from_slice(&14_u32.to_le_bytes());
        let error = decode_matlab_v5(&bytes, LIMITS, "matlab_v5").unwrap_err();
        assert_eq!(
            error.to_string(),
            "matlab_v5 import: the MATLAB file is cut short inside variable 4"
        );
    }

    /// Byte offset of the first matrix's dimensions data in a plain file.
    const DIMENSIONS: usize = crate::matlab::HEADER_BYTES + 8 + 16 + 8;

    #[test]
    fn dimensions_that_overflow_are_refused_before_the_parser_multiplies_them() {
        // The parser multiplies dimensions in unchecked i32: in a debug build
        // that panics and in a release build it wraps. Either way the walk
        // refuses first, with checked arithmetic and no panic in any profile.
        let plain = file_of(&[variable("time", vec![0.0, 1.0])]);
        for compressed in [false, true] {
            let mut bytes = plain.clone();
            bytes[DIMENSIONS..DIMENSIONS + 4].copy_from_slice(&1_000_000_000_i32.to_le_bytes());
            bytes[DIMENSIONS + 4..DIMENSIONS + 8].copy_from_slice(&1_000_000_000_i32.to_le_bytes());
            if compressed {
                let mut packed = bytes[..crate::matlab::HEADER_BYTES].to_vec();
                packed.extend(compressed_element(&bytes[crate::matlab::HEADER_BYTES..]));
                bytes = packed;
            }
            let error = decode_matlab_v5(&bytes, LIMITS, "matlab_v5").unwrap_err();
            assert!(
                matches!(
                    &error.reason,
                    MatlabReadFailure::DimensionOverflow { dimensions }
                        if dimensions == &[1_000_000_000, 1_000_000_000]
                ),
                "compressed {compressed}: {error}"
            );
        }
        let mut bytes = plain;
        bytes[DIMENSIONS + 4..DIMENSIONS + 8].copy_from_slice(&(-2_i32).to_le_bytes());
        let error = decode_matlab_v5(&bytes, LIMITS, "matlab_v5").unwrap_err();
        assert!(
            matches!(
                error.reason,
                MatlabReadFailure::NegativeDimension { dimension: -2 }
            ),
            "{error}"
        );
    }

    #[test]
    fn a_part_larger_than_its_matrix_is_refused() {
        // The name tag claims nearly 4 GiB inside a matrix of a few dozen
        // bytes; the parser would round that size up to eight in u32 and
        // overflow.
        let mut bytes = file_of(&[variable("time", vec![0.0, 1.0])]);
        let name_size = DIMENSIONS + 8 + 4;
        bytes[name_size..name_size + 4].copy_from_slice(&0xFFFF_FFFD_u32.to_le_bytes());
        let error = decode_matlab_v5(&bytes, LIMITS, "matlab_v5").unwrap_err();
        assert!(
            matches!(error.reason, MatlabReadFailure::Malformed),
            "{error}"
        );
    }

    #[test]
    fn malformed_matlab_retains_the_parser_cause() {
        use std::error::Error as _;
        let limits = MatlabReadLimits {
            max_variables: 2,
            max_expanded_bytes: 0,
            min_rows: 1,
            coordinate_names: &["time", "t"],
        };
        let error = decode_matlab_v5(b"invalid", limits, "matlab_v5").unwrap_err();
        assert!(matches!(&error.reason, MatlabReadFailure::Parse(_)));
        assert!(error.reason.source().unwrap().is::<matfile::Error>());
        assert!(error.to_string().starts_with(
            "matlab_v5 import: the file is not a MATLAB v5 file this importer can read"
        ));
    }
}
