//! HDF5 and MATLAB 7.3 waveform byte decoding.

#[cfg(feature = "result-hdf5")]
pub mod result;

#[cfg(test)]
mod table_tests;

use crate::numeric::{
    DecodedNumericDataset, DecodedNumericSignal, combine_real_imag_columns, stated_coordinate_names,
};
use std::collections::HashMap;

/// A container/metadata failure or a bounded waveform decoding refusal.
#[derive(Debug)]
pub struct Hdf5ReadError {
    pub format: String,
    pub reason: Hdf5ReadFailure,
}

#[derive(Debug)]
pub enum Hdf5ReadFailure {
    Decode {
        context: String,
        source: Box<rustyhdf5::Error>,
    },
    MultipleSections(Vec<String>),
    UnsupportedSection {
        section: String,
        kind: String,
    },
    UnrepresentableSignalCount {
        section: String,
        count: i64,
    },
    SignalCount {
        section: String,
        count: usize,
        max_columns: usize,
    },
    RootDatasetLimit {
        datasets: usize,
        max_columns: usize,
    },
    MissingCoordinate {
        expected: String,
    },
    ShapeOverflow {
        dataset: String,
    },
    CoordinateLength {
        dataset: String,
        values: u64,
        coordinate: String,
        samples: usize,
    },
    DatasetValueLimit {
        dataset: String,
        values: u64,
        limit: usize,
    },
    StringAttribute {
        name: String,
        found: rustyhdf5::AttrValue,
    },
    IntegerAttribute {
        name: String,
        found: rustyhdf5::AttrValue,
    },
    MissingAttribute {
        name: String,
    },
    TableValueOverflow,
    TableValueLimit {
        values: usize,
        limit: usize,
    },
    InexactInteger(crate::numeric::ExactIntegerError),
    ComplexColumns(crate::numeric::ComplexColumnError),
    Column(String),
    Coordinate(String),
}

impl std::fmt::Display for Hdf5ReadFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode { context, source } => write!(f, "{context}: {source}"),
            Self::MultipleSections(names) => write!(
                f,
                "the file contains multiple result sections ({}); use rspice convert --section NAME_OR_INDEX to export one waveform analysis before importing",
                names.join(", ")
            ),
            Self::UnsupportedSection { section, kind } => write!(
                f,
                "section /{section} declares {kind:?}, which cannot be represented as a waveform"
            ),
            Self::UnrepresentableSignalCount { section, .. } => {
                write!(f, "/{section} signal_count is negative or too large")
            }
            Self::SignalCount { section, count, .. } => {
                write!(f, "/{section} declares invalid signal_count {count}")
            }
            Self::RootDatasetLimit { .. } => f.write_str("too many root datasets"),
            Self::MissingCoordinate { expected } => {
                write!(f, "no unambiguous root coordinate dataset named {expected}")
            }
            Self::ShapeOverflow { dataset } => write!(f, "dataset '{dataset}' shape overflows"),
            Self::CoordinateLength {
                dataset,
                values,
                coordinate,
                samples,
            } => write!(
                f,
                "dataset '{dataset}' has {values} values; coordinate '{coordinate}' has {samples}"
            ),
            Self::DatasetValueLimit {
                dataset,
                values,
                limit,
            } => write!(
                f,
                "dataset '{dataset}' contains {values} values; the limit is {limit}"
            ),
            Self::StringAttribute { name, found } => write!(
                f,
                "attribute '{name}' must be a non-empty string, found {found:?}"
            ),
            Self::IntegerAttribute { name, found } => {
                write!(f, "attribute '{name}' must be an integer, found {found:?}")
            }
            Self::MissingAttribute { name } => write!(f, "missing attribute '{name}'"),
            Self::TableValueOverflow => f.write_str("table value count overflow"),
            Self::TableValueLimit { values, limit } => write!(
                f,
                "the table contains {values} values; the limit is {limit}"
            ),
            Self::InexactInteger(source) => source.fmt(f),
            Self::ComplexColumns(source) => source.fmt(f),
            Self::Column(message) | Self::Coordinate(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for Hdf5ReadFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Decode { source, .. } => Some(source.as_ref()),
            Self::InexactInteger(source) => Some(source),
            Self::ComplexColumns(source) => Some(source),
            _ => None,
        }
    }
}

impl std::fmt::Display for Hdf5ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} import: {}", self.format, self.reason)
    }
}

impl std::error::Error for Hdf5ReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.reason)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hdf5Limits<'a> {
    pub max_columns: usize,
    pub max_values: usize,
    pub coordinate_names: &'a [&'a str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hdf5SectionFamily {
    Table(crate::WaveformDomain),
    Transient,
    DcSweep,
    Ac,
}

enum Hdf5SectionKind {
    Waveform(Hdf5SectionFamily),
    Unsupported(String),
}

#[derive(Debug)]
enum DecodedHdf5 {
    Section {
        family: Hdf5SectionFamily,
        coordinate_name: String,
        coordinate_unit: Option<String>,
        coordinate: Vec<f64>,
        signals: Vec<DecodedNumericSignal>,
    },
    Root {
        coordinate_name: String,
        coordinate: Vec<f64>,
        columns: Vec<(String, Vec<f64>)>,
    },
}

fn adapter_error(format: &str, reason: Hdf5ReadFailure) -> Hdf5ReadError {
    Hdf5ReadError {
        format: format.to_owned(),
        reason,
    }
}

fn decode_error(
    format: &str,
    context: impl Into<String>,
    source: rustyhdf5::Error,
) -> Hdf5ReadError {
    adapter_error(
        format,
        Hdf5ReadFailure::Decode {
            context: context.into(),
            source: Box::new(source),
        },
    )
}

pub fn decode_hdf5(
    bytes: &[u8],
    limits: Hdf5Limits<'_>,
    format: &str,
) -> Result<DecodedNumericDataset, Hdf5ReadError> {
    finish_hdf5(decode_hdf5_container(bytes, limits, format)?, format)
}

pub fn decode_matlab_v73(
    bytes: &[u8],
    limits: Hdf5Limits<'_>,
    format: &str,
) -> Result<DecodedNumericDataset, Hdf5ReadError> {
    finish_hdf5(decode_matlab_v73_container(bytes, limits, format)?, format)
}

fn finish_hdf5(decoded: DecodedHdf5, format: &str) -> Result<DecodedNumericDataset, Hdf5ReadError> {
    let mut dataset = match decoded {
        DecodedHdf5::Section {
            family,
            coordinate_name,
            coordinate_unit,
            coordinate,
            signals,
        } => {
            let domain = match family {
                Hdf5SectionFamily::Transient => crate::WaveformDomain::Transient,
                Hdf5SectionFamily::DcSweep => crate::WaveformDomain::DcSweep,
                Hdf5SectionFamily::Ac => crate::WaveformDomain::Ac,
                Hdf5SectionFamily::Table(domain) => domain,
            };
            DecodedNumericDataset {
                coordinate_unit,
                domain,
                coordinate_name,
                coordinate,
                signals,
            }
        }
        DecodedHdf5::Root {
            coordinate_name,
            coordinate,
            columns,
        } => {
            let signals = combine_real_imag_columns(columns)
                .map_err(|error| adapter_error(format, Hdf5ReadFailure::ComplexColumns(error)))?;
            DecodedNumericDataset {
                coordinate_unit: None,
                domain: crate::WaveformDomain::from_coordinate_name(&coordinate_name),
                coordinate_name,
                coordinate,
                signals,
            }
        }
    };
    dataset
        .normalize_coordinate_unit()
        .map_err(|error| adapter_error(format, Hdf5ReadFailure::Coordinate(error)))?;
    Ok(dataset)
}

fn decode_hdf5_container(
    bytes: &[u8],
    limits: Hdf5Limits<'_>,
    format: &str,
) -> Result<DecodedHdf5, Hdf5ReadError> {
    let file = rustyhdf5::File::from_bytes(bytes.to_vec())
        .map_err(|error| decode_error(format, "invalid HDF5 container", error))?;
    let root = file.root();
    let groups = root
        .groups()
        .map_err(|error| decode_error(format, "could not enumerate groups", error))?;
    let mut sections = Vec::new();
    for name in groups {
        if let Some(kind) = hdf5_section_kind(&file, &name, format)? {
            sections.push((name, kind));
        }
    }
    if sections.len() > 1 {
        return Err(adapter_error(
            format,
            Hdf5ReadFailure::MultipleSections(
                sections.iter().map(|(name, _)| name.clone()).collect(),
            ),
        ));
    }
    if let Some((section, kind)) = sections.pop() {
        return match kind {
            Hdf5SectionKind::Waveform(family) => {
                parse_rspice_hdf5_section(&file, &section, family, format, limits)
            }
            Hdf5SectionKind::Unsupported(kind) => Err(adapter_error(
                format,
                Hdf5ReadFailure::UnsupportedSection { section, kind },
            )),
        };
    }
    parse_generic_hdf5_root(&file, format, limits)
}

/// Classify every declared result, including families the waveform viewer
/// cannot represent. Only unmarked, non-legacy groups are ordinary metadata.
///
/// The layout contract in `rspice_core::io::hdf5` is explicit that a section
/// group's *name* is the producer's choice and its `section_type` attribute is
/// what names the family. The command line names its one section group after
/// the analysis instance that produced it — `tran1`, not `transient` — so a
/// reader keyed on the name alone read every identity-named file as an
/// anonymous root, fell through to [`parse_generic_hdf5_root`], and refused a
/// file this product had just written. The name is still consulted, because a
/// file written before the attribute existed carries nothing else.
///
/// Malformed declarations cannot fall back to a legacy group name, and an
/// unsupported result cannot disappear beside a supported waveform section.
fn hdf5_section_kind(
    file: &rustyhdf5::File,
    group: &str,
    format: &str,
) -> Result<Option<Hdf5SectionKind>, Hdf5ReadError> {
    let opened = file
        .group(group)
        .map_err(|error| decode_error(format, format!("could not open /{group}"), error))?;
    let attrs = opened.attrs().map_err(|error| {
        decode_error(format, format!("could not read /{group} attributes"), error)
    })?;
    let declared = if attrs.contains_key("section_type") {
        hdf_string_attr(&attrs, "section_type", format)?
    } else if matches!(
        group,
        "transient"
            | "dc_sweep"
            | "ac"
            | "table"
            | "operating_point"
            | "noise"
            | "distortion"
            | "fft"
    ) {
        group.to_owned()
    } else {
        return Ok(None);
    };
    let family = match declared.as_str() {
        "transient" => Hdf5SectionFamily::Transient,
        "dc_sweep" => Hdf5SectionFamily::DcSweep,
        "ac" => Hdf5SectionFamily::Ac,
        "table" => {
            let coordinate_type = hdf_string_attr(&attrs, "coordinate_type", format)?;
            let domain = match coordinate_type.as_str() {
                "time" => crate::WaveformDomain::Transient,
                "frequency" => crate::WaveformDomain::Ac,
                "voltage" | "current" | "temperature" => crate::WaveformDomain::DcSweep,
                // A sweep may use a parameter with no declared physical
                // quantity. Its analysis still identifies the waveform domain.
                "value"
                    if attrs.contains_key("analysis")
                        && hdf_string_attr(&attrs, "analysis", format)? == "dc_sweep" =>
                {
                    crate::WaveformDomain::DcSweep
                }
                // Report/index coordinates have no waveform-domain equivalent.
                _ => {
                    return Ok(Some(Hdf5SectionKind::Unsupported(format!(
                        "table with coordinate type {coordinate_type:?}"
                    ))));
                }
            };
            Hdf5SectionFamily::Table(domain)
        }
        _ => return Ok(Some(Hdf5SectionKind::Unsupported(declared))),
    };
    Ok(Some(Hdf5SectionKind::Waveform(family)))
}

fn decode_matlab_v73_container(
    bytes: &[u8],
    limits: Hdf5Limits<'_>,
    format: &str,
) -> Result<DecodedHdf5, Hdf5ReadError> {
    let file = rustyhdf5::File::from_bytes(bytes.to_vec())
        .map_err(|error| decode_error(format, "invalid MATLAB 7.3/HDF5 container", error))?;
    parse_generic_hdf5_root(&file, format, limits)
}

fn parse_rspice_hdf5_section(
    file: &rustyhdf5::File,
    section: &str,
    family: Hdf5SectionFamily,
    format: &str,
    limits: Hdf5Limits<'_>,
) -> Result<DecodedHdf5, Hdf5ReadError> {
    let group = file
        .group(section)
        .map_err(|error| decode_error(format, format!("could not open /{section}"), error))?;
    let attrs = group.attrs().map_err(|error| {
        decode_error(
            format,
            format!("could not read /{section} attributes"),
            error,
        )
    })?;
    let coordinate_unit = attrs
        .contains_key("coordinate_unit")
        .then(|| hdf_string_attr(&attrs, "coordinate_unit", format))
        .transpose()?;
    let signal_count = hdf_i64_attr(&attrs, "signal_count", format)?;
    let signal_count = usize::try_from(signal_count).map_err(|_| {
        adapter_error(
            format,
            Hdf5ReadFailure::UnrepresentableSignalCount {
                section: section.to_owned(),
                count: signal_count,
            },
        )
    })?;
    if signal_count == 0 || signal_count.saturating_add(1) > limits.max_columns {
        return Err(adapter_error(
            format,
            Hdf5ReadFailure::SignalCount {
                section: section.to_owned(),
                count: signal_count,
                max_columns: limits.max_columns,
            },
        ));
    }
    if family == Hdf5SectionFamily::Ac {
        let coordinate_name = if attrs.contains_key("independent_name") {
            hdf_string_attr(&attrs, "independent_name", format)?
        } else {
            "frequency".to_owned()
        };
        let coordinate = hdf_f64_dataset(&group, "frequency", format, limits)?;
        ensure_table_value_limit(
            format,
            coordinate.len(),
            1 + signal_count.saturating_mul(2),
            limits,
        )?;
        let mut signals = Vec::with_capacity(signal_count);
        for index in 0..signal_count {
            let prefix = format!("signal_{index:04}");
            let name = hdf_string_attr(&attrs, &format!("{prefix}_name"), format)?;
            signals.push(DecodedNumericSignal {
                name,
                real: hdf_f64_dataset(&group, &format!("{prefix}_real"), format, limits)?,
                imag: Some(hdf_f64_dataset(
                    &group,
                    &format!("{prefix}_imag"),
                    format,
                    limits,
                )?),
                unit: hdf_stated_unit(&attrs, &format!("{prefix}_unit")),
            });
        }
        return Ok(DecodedHdf5::Section {
            family,
            coordinate_name,
            coordinate_unit,
            coordinate,
            signals,
        });
    }

    let coordinate_name = hdf_string_attr(&attrs, "independent_name", format)?;
    let coordinate = hdf_f64_dataset(&group, "independent", format, limits)?;
    ensure_table_value_limit(format, coordinate.len(), 1 + signal_count, limits)?;
    let mut signals = Vec::with_capacity(signal_count);
    for index in 0..signal_count {
        let prefix = format!("signal_{index:04}");
        signals.push(DecodedNumericSignal {
            name: hdf_string_attr(&attrs, &format!("{prefix}_name"), format)?,
            real: hdf_f64_dataset(&group, &prefix, format, limits)?,
            imag: None,
            unit: hdf_stated_unit(&attrs, &format!("{prefix}_unit")),
        });
    }
    if matches!(family, Hdf5SectionFamily::Table(_)) {
        use crate::numeric::nullable::{
            DenseNumericColumn, decode_dense_validity, nullable_value_type,
        };
        let mut decoded = Vec::new();
        let mut columns = signals.into_iter().enumerate().peekable();
        while let Some((index, mut signal)) = columns.next() {
            let mut kind = hdf_string_attr(&attrs, &format!("signal_{index:04}_type"), format)?;
            if let Some(quantity) = kind.strip_prefix("complex_real:") {
                let name = signal
                    .name
                    .strip_prefix("Re(")
                    .and_then(|name| name.strip_suffix(')'))
                    .unwrap_or(&signal.name)
                    .to_owned();
                let expected = format!("Im({name})");
                let valid = columns.peek().is_some_and(|(next, imag)| {
                    imag.name == expected
                        && hdf_optional_string_attr(&attrs, &format!("signal_{next:04}_type"))
                            .as_deref()
                            == Some(format!("complex_imag:{quantity}").as_str())
                });
                if !valid {
                    return Err(adapter_error(
                        format,
                        Hdf5ReadFailure::ComplexColumns(
                            crate::numeric::ComplexColumnError::MissingImaginary(name),
                        ),
                    ));
                }
                let (_, imaginary) = columns.next().expect("validated imaginary column");
                if imaginary.real.len() != signal.real.len()
                    || imaginary.unit.is_some() && imaginary.unit != signal.unit
                {
                    return Err(adapter_error(
                        format,
                        Hdf5ReadFailure::Column(format!(
                            "complex signal '{name}' has inconsistent component lengths or units"
                        )),
                    ));
                }
                signal.name = name;
                signal.imag = Some(imaginary.real);
                kind = quantity.to_owned();
            } else if kind.starts_with("complex_imag:") {
                return Err(adapter_error(
                    format,
                    Hdf5ReadFailure::ComplexColumns(
                        crate::numeric::ComplexColumnError::MissingReal(signal.name),
                    ),
                ));
            }
            if nullable_value_type(&kind).is_some() {
                let (index, mask) = columns.next().ok_or_else(|| {
                    adapter_error(
                        format,
                        Hdf5ReadFailure::Column(
                            "nullable value column has no validity column".into(),
                        ),
                    )
                })?;
                let mask_kind =
                    hdf_string_attr(&attrs, &format!("signal_{index:04}_type"), format)?;
                let defined = decode_dense_validity(
                    DenseNumericColumn {
                        kind: &kind,
                        unit: signal.unit.as_deref(),
                        real: &signal.real,
                        imag: signal.imag.as_deref(),
                    },
                    DenseNumericColumn {
                        kind: &mask_kind,
                        unit: mask.unit.as_deref(),
                        real: &mask.real,
                        imag: None,
                    },
                )
                .map_err(|error| adapter_error(format, Hdf5ReadFailure::Column(error)))?;
                for (index, defined) in defined.into_iter().enumerate() {
                    if !defined {
                        signal.real[index] = f64::NAN;
                        if let Some(imag) = &mut signal.imag {
                            imag[index] = f64::NAN;
                        }
                    }
                }
            } else if kind.starts_with("nullable_validity:") {
                return Err(adapter_error(
                    format,
                    Hdf5ReadFailure::Column(
                        "nullable validity column has no preceding value column".into(),
                    ),
                ));
            }
            decoded.push(signal);
        }
        signals = decoded;
    }
    Ok(DecodedHdf5::Section {
        family,
        coordinate_name,
        coordinate_unit,
        coordinate,
        signals,
    })
}

fn parse_generic_hdf5_root(
    file: &rustyhdf5::File,
    format: &str,
    limits: Hdf5Limits<'_>,
) -> Result<DecodedHdf5, Hdf5ReadError> {
    let root = file.root();
    let names = root
        .datasets()
        .map_err(|error| decode_error(format, "could not enumerate datasets", error))?;
    if names.len() > limits.max_columns.saturating_mul(2) {
        return Err(adapter_error(
            format,
            Hdf5ReadFailure::RootDatasetLimit {
                datasets: names.len(),
                max_columns: limits.max_columns,
            },
        ));
    }
    let coordinate_name =
        select_coordinate_name(names.iter().map(String::as_str), limits.coordinate_names)
            .ok_or_else(|| {
                adapter_error(
                    format,
                    Hdf5ReadFailure::MissingCoordinate {
                        expected: stated_coordinate_names(limits.coordinate_names),
                    },
                )
            })?;
    let coordinate = hdf_f64_dataset(&root, &coordinate_name, format, limits)?;
    ensure_table_value_limit(format, coordinate.len(), names.len(), limits)?;
    let mut columns = Vec::new();
    for name in names {
        if name.eq_ignore_ascii_case(&coordinate_name) || name.starts_with('#') {
            continue;
        }
        let dataset = root.dataset(&name).map_err(|error| {
            decode_error(format, format!("could not open dataset '{name}'"), error)
        })?;
        let shape = dataset.shape().map_err(|error| {
            decode_error(format, format!("could not inspect dataset '{name}'"), error)
        })?;
        let count = shape
            .iter()
            .try_fold(1_u64, |count, dim| count.checked_mul(*dim))
            .ok_or_else(|| {
                adapter_error(
                    format,
                    Hdf5ReadFailure::ShapeOverflow {
                        dataset: name.to_owned(),
                    },
                )
            })?;
        if count != coordinate.len() as u64 {
            return Err(adapter_error(
                format,
                Hdf5ReadFailure::CoordinateLength {
                    dataset: name,
                    values: count,
                    coordinate: coordinate_name,
                    samples: coordinate.len(),
                },
            ));
        }
        let values = hdf_dataset_values(&dataset, &name, format, limits)?;
        columns.push((name, values));
    }
    Ok(DecodedHdf5::Root {
        coordinate_name,
        coordinate,
        columns,
    })
}

fn hdf_f64_dataset(
    group: &rustyhdf5::Group<'_>,
    name: &str,
    format: &str,
    limits: Hdf5Limits<'_>,
) -> Result<Vec<f64>, Hdf5ReadError> {
    let dataset = group.dataset(name).map_err(|error| {
        decode_error(
            format,
            format!("could not open numeric dataset '{name}'"),
            error,
        )
    })?;
    hdf_dataset_values(&dataset, name, format, limits)
}

fn hdf_dataset_values(
    dataset: &rustyhdf5::Dataset<'_>,
    name: &str,
    format: &str,
    limits: Hdf5Limits<'_>,
) -> Result<Vec<f64>, Hdf5ReadError> {
    let count = dataset
        .shape()
        .map_err(|error| {
            decode_error(
                format,
                format!("could not inspect dataset '{name}' shape"),
                error,
            )
        })?
        .into_iter()
        .try_fold(1_u64, |count, dimension| count.checked_mul(dimension))
        .ok_or_else(|| {
            adapter_error(
                format,
                Hdf5ReadFailure::ShapeOverflow {
                    dataset: name.to_owned(),
                },
            )
        })?;
    if count > limits.max_values as u64 {
        return Err(adapter_error(
            format,
            Hdf5ReadFailure::DatasetValueLimit {
                dataset: name.to_owned(),
                values: count,
                limit: limits.max_values,
            },
        ));
    }
    let dtype = dataset.dtype().map_err(|error| {
        decode_error(format, format!("could not inspect dataset '{name}'"), error)
    })?;
    match dtype {
        rustyhdf5::DType::I64 => dataset
            .read_i64()
            .map_err(|error| decode_error(format, format!("could not read '{name}'"), error))?
            .into_iter()
            .map(|value| {
                crate::numeric::exact_signed_integer(name, value).map_err(|detail| {
                    adapter_error(format, Hdf5ReadFailure::InexactInteger(detail))
                })
            })
            .collect(),
        rustyhdf5::DType::U64 => dataset
            .read_u64()
            .map_err(|error| decode_error(format, format!("could not read '{name}'"), error))?
            .into_iter()
            .map(|value| {
                crate::numeric::exact_unsigned_integer(name, value).map_err(|detail| {
                    adapter_error(format, Hdf5ReadFailure::InexactInteger(detail))
                })
            })
            .collect(),
        _ => dataset.read_f64().map_err(|error| {
            decode_error(
                format,
                format!("could not read numeric dataset '{name}'"),
                error,
            )
        }),
    }
}

fn hdf_string_attr(
    attrs: &HashMap<String, rustyhdf5::AttrValue>,
    name: &str,
    format: &str,
) -> Result<String, Hdf5ReadError> {
    match attrs.get(name) {
        Some(rustyhdf5::AttrValue::String(value)) if !value.trim().is_empty() => Ok(value.clone()),
        Some(other) => Err(adapter_error(
            format,
            Hdf5ReadFailure::StringAttribute {
                name: name.to_owned(),
                found: other.clone(),
            },
        )),
        None => Err(adapter_error(
            format,
            Hdf5ReadFailure::MissingAttribute {
                name: name.to_owned(),
            },
        )),
    }
}

/// The unit a section states for one column, if it states one.
///
/// `rspice_core::io::hdf5` writes `signal_NNNN_unit` only when the producer
/// had a unit to state, so an absent attribute means unstated rather than
/// dimensionless, and it is never an import error. An empty or non-string
/// value is read the same way: a waveform must not come back claiming "" as
/// its unit.
fn hdf_stated_unit(attrs: &HashMap<String, rustyhdf5::AttrValue>, name: &str) -> Option<String> {
    hdf_optional_string_attr(attrs, name)
}

/// A text attribute the producer may or may not have written.
///
/// An absent attribute, a non-string one and an empty one are all "unstated":
/// the reader is asking whether the file says something, and "" is not a
/// statement.
fn hdf_optional_string_attr(
    attrs: &HashMap<String, rustyhdf5::AttrValue>,
    name: &str,
) -> Option<String> {
    match attrs.get(name) {
        Some(rustyhdf5::AttrValue::String(value)) if !value.trim().is_empty() => {
            Some(value.clone())
        }
        _ => None,
    }
}

fn hdf_i64_attr(
    attrs: &HashMap<String, rustyhdf5::AttrValue>,
    name: &str,
    format: &str,
) -> Result<i64, Hdf5ReadError> {
    match attrs.get(name) {
        Some(rustyhdf5::AttrValue::I64(value)) => Ok(*value),
        Some(other) => Err(adapter_error(
            format,
            Hdf5ReadFailure::IntegerAttribute {
                name: name.to_owned(),
                found: other.clone(),
            },
        )),
        None => Err(adapter_error(
            format,
            Hdf5ReadFailure::MissingAttribute {
                name: name.to_owned(),
            },
        )),
    }
}

fn select_coordinate_name<'a>(
    names: impl Iterator<Item = &'a str>,
    candidates: &[&str],
) -> Option<String> {
    let names = names.collect::<Vec<_>>();
    for candidate in candidates {
        if let Some(name) = names
            .iter()
            .find(|name| name.eq_ignore_ascii_case(candidate))
        {
            return Some((*name).to_owned());
        }
    }
    None
}

fn ensure_table_value_limit(
    format: &str,
    rows: usize,
    columns: usize,
    limits: Hdf5Limits<'_>,
) -> Result<(), Hdf5ReadError> {
    let values = rows
        .checked_mul(columns)
        .ok_or_else(|| adapter_error(format, Hdf5ReadFailure::TableValueOverflow))?;
    if values > limits.max_values {
        Err(adapter_error(
            format,
            Hdf5ReadFailure::TableValueLimit {
                values,
                limit: limits.max_values,
            },
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Hdf5Limits, Hdf5ReadFailure, decode_hdf5};

    #[test]
    fn integer_datasets_preserve_exact_large_values_without_accepting_saturation() {
        let values = [
            i64::MIN,
            -9007199254740994,
            9007199254740994,
            i64::MAX - 1023,
        ];
        let mut builder = rustyhdf5::FileBuilder::new();
        builder
            .create_dataset("time")
            .with_f64_data(&[0.0, 1.0, 2.0, 3.0]);
        builder.create_dataset("count").with_i64_data(&values);
        let limits = Hdf5Limits {
            max_columns: 2,
            max_values: 8,
            coordinate_names: &["time"],
        };
        let decoded = decode_hdf5(&builder.finish().unwrap(), limits, "hdf5").unwrap();
        assert_eq!(
            decoded.signals[0]
                .real
                .iter()
                .map(|&value| value as i128)
                .collect::<Vec<_>>(),
            values.map(i128::from)
        );
        for value in [i64::MIN + 1, 9007199254740993, i64::MAX] {
            let mut builder = rustyhdf5::FileBuilder::new();
            builder.create_dataset("time").with_f64_data(&[0.0, 1.0]);
            builder.create_dataset("count").with_i64_data(&[0, value]);
            let error = decode_hdf5(&builder.finish().unwrap(), limits, "hdf5").unwrap_err();
            assert!(
                matches!(error.reason, Hdf5ReadFailure::InexactInteger(crate::numeric::ExactIntegerError::Signed { value: actual, .. }) if actual == value),
                "{error}"
            );
        }
    }

    #[test]
    fn typed_tables_preserve_waveform_domains_and_complex_columns() {
        use rustyhdf5::{AttrValue, FileBuilder};
        for (coordinate, domain) in [
            ("time", crate::WaveformDomain::Transient),
            ("frequency", crate::WaveformDomain::Ac),
        ] {
            let mut file = FileBuilder::new();
            let mut group = file.create_group("converted");
            for (key, value) in [
                ("section_type", "table"),
                ("coordinate_type", coordinate),
                ("independent_name", coordinate),
                ("analysis", "converted"),
                ("signal_0000_name", "Re(I(V1))"),
                ("signal_0001_name", "Im(I(V1))"),
                ("signal_0000_type", "complex_real:current"),
                ("signal_0001_type", "complex_imag:current"),
                ("signal_0000_unit", "A"),
            ] {
                group.set_attr(key, AttrValue::String(value.to_owned()));
            }
            group.set_attr("signal_count", AttrValue::I64(2));
            group
                .create_dataset("independent")
                .with_f64_data(&[1.0, 2.0]);
            group
                .create_dataset("signal_0000")
                .with_f64_data(&[3.0, 4.0]);
            group
                .create_dataset("signal_0001")
                .with_f64_data(&[5.0, 6.0]);
            file.add_group(group.finish());
            let bytes = file.finish().unwrap();
            let dataset = decode_hdf5(
                &bytes,
                Hdf5Limits {
                    max_columns: 3,
                    max_values: 6,
                    coordinate_names: &["time", "frequency"],
                },
                "hdf5",
            )
            .unwrap();
            assert_eq!(dataset.domain, domain);
            assert_eq!(dataset.coordinate, [1.0, 2.0]);
            assert_eq!(dataset.signals.len(), 1);
            assert_eq!(dataset.signals[0].name, "I(V1)");
            assert_eq!(dataset.signals[0].real, [3.0, 4.0]);
            assert_eq!(dataset.signals[0].imag.as_deref(), Some(&[5.0, 6.0][..]));
            assert_eq!(dataset.signals[0].unit.as_deref(), Some("A"));
        }
    }

    #[test]
    fn root_reader_preserves_coordinate_and_numeric_bounds() {
        let mut builder = rustyhdf5::FileBuilder::new();
        builder.create_dataset("time").with_f64_data(&[0.0, 1.0]);
        builder.create_dataset("V(out)").with_f64_data(&[2.0, 3.0]);
        let bytes = builder.finish().expect("HDF5 fixture");
        let limits = Hdf5Limits {
            max_columns: 2,
            max_values: 4,
            coordinate_names: &["time", "x"],
        };
        let decoded = decode_hdf5(&bytes, limits, "hdf5").expect("root table");
        assert_eq!(decoded.domain, crate::WaveformDomain::Transient);
        assert_eq!(decoded.coordinate_name, "time");
        assert_eq!(decoded.coordinate, [0.0, 1.0]);
        assert_eq!(decoded.signals.len(), 1);
        assert_eq!(decoded.signals[0].name, "V(out)");
        assert_eq!(decoded.signals[0].real, [2.0, 3.0]);
        assert!(decoded.signals[0].imag.is_none());
        assert!(decoded.signals[0].unit.is_none());

        let small_limit = Hdf5Limits {
            max_values: 1,
            ..limits
        };
        let error = decode_hdf5(&bytes, small_limit, "hdf5").expect_err("value limit");
        assert!(
            matches!(&error.reason, Hdf5ReadFailure::DatasetValueLimit { dataset, values: 2, limit: 1 } if dataset == "time")
        );
        assert_eq!(
            error.to_string(),
            "hdf5 import: dataset 'time' contains 2 values; the limit is 1"
        );
    }

    #[test]
    fn malformed_container_preserves_the_hdf5_parser_cause() {
        use std::error::Error as _;
        let limits = Hdf5Limits {
            max_columns: 2,
            max_values: 4,
            coordinate_names: &["time", "x"],
        };
        let error = decode_hdf5(b"invalid", limits, "hdf5").unwrap_err();
        let Hdf5ReadFailure::Decode { context, source } = &error.reason else {
            panic!("expected a container parser failure: {error}");
        };
        assert_eq!(context, "invalid HDF5 container");
        assert_eq!(
            error.to_string(),
            format!("hdf5 import: invalid HDF5 container: {source}")
        );
        assert!(error.reason.source().unwrap().is::<rustyhdf5::Error>());
    }

    #[test]
    fn root_coordinate_priority_and_complex_pairs_are_preserved() {
        let bytes = |with_imag| {
            let mut builder = rustyhdf5::FileBuilder::new();
            builder.create_dataset("t").with_f64_data(&[10.0, 20.0]);
            builder.create_dataset("time").with_f64_data(&[0.0, 1.0]);
            builder.create_dataset("gain_RE").with_f64_data(&[2.0, 3.0]);
            if with_imag {
                builder
                    .create_dataset("gain_IM")
                    .with_f64_data(&[-0.0, 4.0]);
            }
            builder.finish().unwrap()
        };
        let limits = Hdf5Limits {
            max_columns: 4,
            max_values: 8,
            coordinate_names: &["time", "t"],
        };
        let decoded = decode_hdf5(&bytes(true), limits, "hdf5").unwrap();
        assert_eq!(decoded.coordinate_name, "time");
        assert_eq!(decoded.coordinate, [0.0, 1.0]);
        assert_eq!(
            decoded
                .signals
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>(),
            ["t", "gain"]
        );
        assert_eq!(decoded.signals[0].real, [10.0, 20.0]);
        assert_eq!(decoded.signals[1].real, [2.0, 3.0]);
        let imag = decoded.signals[1].imag.as_ref().unwrap();
        assert_eq!(imag[0].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(imag[1], 4.0);
        let error = decode_hdf5(&bytes(false), limits, "hdf5").unwrap_err();
        assert!(
            matches!(&error.reason, Hdf5ReadFailure::ComplexColumns(crate::numeric::ComplexColumnError::MissingImaginary(name)) if name == "gain")
        );
        assert_eq!(
            error.to_string(),
            "hdf5 import: complex signal 'gain' is missing its imaginary component"
        );
    }
}
