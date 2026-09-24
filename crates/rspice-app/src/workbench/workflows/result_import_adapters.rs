//! Bounded adapters for structured and simulator-native result formats.
//!
//! Every entry point consumes the already size-limited byte slice owned by the
//! import transaction.  Adapters reject ambiguous mappings rather than
//! inventing domain or signal identity.

use super::*;
use rspice_formats::numeric::MAX_EXACT_F64_INTEGER;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{BufReader, Cursor, Read};

const MAX_ARCHIVE_MEMBERS: usize = 1_024;
const MAX_ARCHIVE_EXPANDED_BYTES: u64 = MAX_RESULT_DATASET_BYTES;
const MAX_SIGNAL_NAME_BYTES: usize = 1_024;
const MAX_RESULT_VALUES: usize = MAX_RESULT_DATASET_BYTES as usize / std::mem::size_of::<f64>();

#[derive(Debug)]
struct ImportedSignal {
    name: String,
    real: Vec<f64>,
    imag: Option<Vec<f64>>,
    unit: Option<String>,
}

fn adapter_error(format: ResultImportFormat, detail: impl std::fmt::Display) -> String {
    format!("{} import: {detail}", format.canonical_id())
}

fn analysis_from_coordinate(name: &str) -> AnalysisType {
    let lower = name.to_ascii_lowercase();
    if lower.contains("freq") || lower == "hz" {
        AnalysisType::Ac
    } else if lower.contains("time") || lower == "t" {
        AnalysisType::Transient
    } else {
        AnalysisType::DcSweep
    }
}

fn parse_analysis(format: ResultImportFormat, value: &str) -> Result<AnalysisType, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "tran" | "transient" | "time" => Ok(AnalysisType::Transient),
        "ac" | "frequency" | "freq" => Ok(AnalysisType::Ac),
        "dc" | "dc_sweep" | "dc-sweep" | "sweep" => Ok(AnalysisType::DcSweep),
        other => Err(adapter_error(
            format,
            format_args!("unsupported analysis domain '{other}'"),
        )),
    }
}

fn finish_dataset(
    format: ResultImportFormat,
    analysis_type: AnalysisType,
    coordinate_name: impl Into<String>,
    coordinate: Vec<f64>,
    signals: Vec<ImportedSignal>,
) -> Result<ParsedResultDataset, String> {
    let coordinate_name = coordinate_name.into();
    validate_name(format, "coordinate", &coordinate_name)?;
    if coordinate.len() < MIN_RESULT_ROWS {
        return Err(adapter_error(
            format,
            format_args!(
                "the '{coordinate_name}' coordinate carries no samples, so the source holds no \
                 result to import"
            ),
        ));
    }
    if coordinate.len() > MAX_RESULT_ROWS {
        return Err(adapter_error(
            format,
            format_args!(
                "the coordinate contains {} samples; the limit is {MAX_RESULT_ROWS}",
                coordinate.len()
            ),
        ));
    }
    if signals.is_empty() {
        return Err(adapter_error(
            format,
            "the source contains no importable signals",
        ));
    }
    if signals.len() + 1 > MAX_RESULT_COLUMNS {
        return Err(adapter_error(
            format,
            format_args!(
                "the source contains {} columns; the limit is {MAX_RESULT_COLUMNS}",
                signals.len() + 1
            ),
        ));
    }
    validate_finite(format, &coordinate_name, &coordinate)?;
    validate_coordinate(format, analysis_type, &coordinate)?;

    let retained_values = coordinate
        .len()
        .checked_mul(1 + signals.len().saturating_mul(2))
        .ok_or_else(|| adapter_error(format, "retained-value count overflow"))?;
    if retained_values > MAX_RESULT_VALUES {
        return Err(adapter_error(
            format,
            format_args!(
                "the source expands to {retained_values} numeric values; the limit is {MAX_RESULT_VALUES}"
            ),
        ));
    }

    let coordinate = Arc::new(coordinate);
    let mut names = HashSet::with_capacity(signals.len());
    let mut waveforms = Vec::with_capacity(signals.len());
    for (index, signal) in signals.into_iter().enumerate() {
        validate_name(format, "signal", &signal.name)?;
        if !names.insert(signal.name.to_ascii_lowercase()) {
            return Err(adapter_error(
                format,
                format_args!("duplicate signal identity '{}'", signal.name),
            ));
        }
        if signal.real.len() != coordinate.len() {
            return Err(adapter_error(
                format,
                format_args!(
                    "signal '{}' has {} samples; expected {}",
                    signal.name,
                    signal.real.len(),
                    coordinate.len()
                ),
            ));
        }
        validate_finite(format, &signal.name, &signal.real)?;
        let mut waveform = if let Some(imag) = signal.imag {
            if imag.len() != coordinate.len() {
                return Err(adapter_error(
                    format,
                    format_args!(
                        "signal '{}' imaginary component has {} samples; expected {}",
                        signal.name,
                        imag.len(),
                        coordinate.len()
                    ),
                ));
            }
            validate_finite(
                format,
                &format!("{} imaginary component", signal.name),
                &imag,
            )?;
            let magnitude = signal
                .real
                .iter()
                .zip(&imag)
                .map(|(real, imag)| real.hypot(*imag))
                .collect::<Vec<_>>();
            WaveformData::new(
                format!("|{}|", signal.name),
                Arc::clone(&coordinate),
                magnitude,
                trace_color(index),
            )
            .with_complex_components(signal.name, signal.real, imag)
        } else {
            WaveformData::new(
                signal.name,
                Arc::clone(&coordinate),
                signal.real,
                trace_color(index),
            )
        };
        if let Some(unit) = signal.unit {
            waveform = waveform.with_unit(unit);
        }
        waveforms.push(waveform);
    }

    Ok(ParsedResultDataset {
        source_format: format,
        analysis_type,
        coordinate_name,
        sample_count: coordinate.len(),
        waveforms,
        family_metadata: None,
        delimiter: 0,
        notes: Vec::new(),
        event_payload: None,
    })
}

fn validate_name(format: ResultImportFormat, kind: &str, name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err(adapter_error(format, format_args!("{kind} name is empty")));
    }
    if name.len() > MAX_SIGNAL_NAME_BYTES {
        return Err(adapter_error(
            format,
            format_args!("{kind} name exceeds {MAX_SIGNAL_NAME_BYTES} bytes"),
        ));
    }
    if name.chars().any(char::is_control) {
        return Err(adapter_error(
            format,
            format_args!("{kind} name contains a control character"),
        ));
    }
    Ok(())
}

fn validate_finite(
    format: ResultImportFormat,
    identity: &str,
    values: &[f64],
) -> Result<(), String> {
    if let Some((index, value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(adapter_error(
            format,
            format_args!("'{identity}' contains non-finite value {value} at sample {index}"),
        ));
    }
    Ok(())
}

fn validate_coordinate(
    format: ResultImportFormat,
    analysis_type: AnalysisType,
    coordinate: &[f64],
) -> Result<(), String> {
    let mut direction = None;
    for (index, pair) in coordinate.windows(2).enumerate() {
        let step = pair[1].total_cmp(&pair[0]);
        if step.is_eq() {
            return Err(adapter_error(
                format,
                format_args!(
                    "coordinate repeats {} at samples {} and {}",
                    pair[0],
                    index,
                    index + 1
                ),
            ));
        }
        if let Some(expected) = direction {
            if step != expected {
                return Err(adapter_error(
                    format,
                    format_args!("coordinate reverses direction at sample {}", index + 1),
                ));
            }
        } else {
            direction = Some(step);
        }
    }
    if analysis_type == AnalysisType::Ac && coordinate.iter().any(|value| *value <= 0.0) {
        return Err(adapter_error(
            format,
            "frequency coordinates must all be greater than zero",
        ));
    }
    Ok(())
}

// -------------------------------------------------------------------------
// Native RSpice bundles

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeBundleManifest {
    schema: String,
    dataset_member: String,
    dataset_sha256: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeDataset {
    schema: String,
    analysis: String,
    coordinate: NativeCoordinate,
    signals: Vec<NativeSignal>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeCoordinate {
    name: String,
    values: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeSignal {
    name: String,
    #[serde(default)]
    unit: Option<String>,
    #[serde(default)]
    values: Option<Vec<f64>>,
    #[serde(default)]
    real: Option<Vec<f64>>,
    #[serde(default)]
    imag: Option<Vec<f64>>,
}

pub(super) fn parse_native_bundle(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|error| adapter_error(format, format_args!("invalid ZIP container: {error}")))?;
    if archive.len() > MAX_ARCHIVE_MEMBERS {
        return Err(adapter_error(
            format,
            format_args!(
                "archive has {} members; the limit is {MAX_ARCHIVE_MEMBERS}",
                archive.len()
            ),
        ));
    }
    let mut names = HashSet::with_capacity(archive.len());
    let mut expanded = 0_u64;
    for index in 0..archive.len() {
        let file = archive.by_index(index).map_err(|error| {
            adapter_error(format, format_args!("invalid member {index}: {error}"))
        })?;
        expanded = expanded
            .checked_add(file.size())
            .ok_or_else(|| adapter_error(format, "archive expanded-size accounting overflow"))?;
        if expanded > MAX_ARCHIVE_EXPANDED_BYTES {
            return Err(adapter_error(
                format,
                format_args!(
                    "archive expands to {expanded} bytes; the limit is {MAX_ARCHIVE_EXPANDED_BYTES}"
                ),
            ));
        }
        let name = file.name().to_owned();
        if !names.insert(name.clone()) {
            return Err(adapter_error(
                format,
                format_args!("archive repeats member '{name}'"),
            ));
        }
        if file.is_dir() || name.starts_with('/') || name.contains("..") || name.contains('\\') {
            return Err(adapter_error(
                format,
                format_args!("unsafe or unsupported archive member '{name}'"),
            ));
        }
    }
    let manifest_bytes = read_zip_member(&mut archive, "manifest.json", format)?;
    let manifest: NativeBundleManifest =
        serde_json::from_slice(&manifest_bytes).map_err(|error| {
            adapter_error(format, format_args!("manifest.json is invalid: {error}"))
        })?;
    let expected_schema = match format {
        ResultImportFormat::RSpiceResultBundle => "rspice-result-bundle/1",
        ResultImportFormat::RSpiceDatasetBundle => "rspice-dataset-bundle/1",
        _ => unreachable!(),
    };
    if manifest.schema != expected_schema {
        return Err(adapter_error(
            format,
            format_args!(
                "manifest schema '{}' is not supported; expected '{expected_schema}'",
                manifest.schema
            ),
        ));
    }
    if manifest.dataset_member != "dataset.json" {
        return Err(adapter_error(
            format,
            "manifest must bind the canonical dataset.json member",
        ));
    }
    let dataset_bytes = read_zip_member(&mut archive, &manifest.dataset_member, format)?;
    use sha2::Digest as _;
    let digest = format!("{:x}", sha2::Sha256::digest(&dataset_bytes));
    if !manifest.dataset_sha256.eq_ignore_ascii_case(&digest) {
        return Err(adapter_error(
            format,
            "dataset.json SHA-256 does not match the signed manifest identity",
        ));
    }
    let dataset: NativeDataset = serde_json::from_slice(&dataset_bytes)
        .map_err(|error| adapter_error(format, format_args!("dataset.json is invalid: {error}")))?;
    if dataset.schema != "rspice-waveform-dataset/1" {
        return Err(adapter_error(
            format,
            format_args!("unsupported dataset schema '{}'", dataset.schema),
        ));
    }
    let analysis = parse_analysis(format, &dataset.analysis)?;
    let mut signals = Vec::with_capacity(dataset.signals.len());
    for signal in dataset.signals {
        let (real, imag) = match (signal.values, signal.real, signal.imag) {
            (Some(values), None, None) => (values, None),
            (None, Some(real), Some(imag)) => (real, Some(imag)),
            _ => {
                return Err(adapter_error(
                    format,
                    format_args!(
                        "signal '{}' must provide either values or both real and imag",
                        signal.name
                    ),
                ));
            }
        };
        signals.push(ImportedSignal {
            name: signal.name,
            real,
            imag,
            unit: signal.unit,
        });
    }
    finish_dataset(
        format,
        analysis,
        dataset.coordinate.name,
        dataset.coordinate.values,
        signals,
    )
}

fn read_zip_member(
    archive: &mut zip::ZipArchive<Cursor<&[u8]>>,
    name: &str,
    format: ResultImportFormat,
) -> Result<Vec<u8>, String> {
    let file = archive
        .by_name(name)
        .map_err(|error| adapter_error(format, format_args!("missing '{name}': {error}")))?;
    if file.size() > MAX_RESULT_DATASET_BYTES {
        return Err(adapter_error(
            format,
            format_args!("'{name}' exceeds the byte limit"),
        ));
    }
    let mut bytes = Vec::with_capacity(usize::try_from(file.size()).unwrap_or(0));
    file.take(MAX_RESULT_DATASET_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| {
            adapter_error(format, format_args!("could not decode '{name}': {error}"))
        })?;
    if bytes.len() as u64 > MAX_RESULT_DATASET_BYTES {
        return Err(adapter_error(
            format,
            format_args!("'{name}' exceeds the byte limit"),
        ));
    }
    Ok(bytes)
}

// -------------------------------------------------------------------------
// HDF5 and MATLAB 7.3

pub(super) fn parse_hdf5(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let decoded = rspice_formats::hdf5::decode_hdf5(bytes, hdf5_limits(), format.canonical_id())?;
    finish_hdf5(format, decoded)
}

pub(super) fn parse_matlab_v73(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let decoded =
        rspice_formats::hdf5::decode_matlab_v73(bytes, hdf5_limits(), format.canonical_id())?;
    finish_hdf5(format, decoded)
}

fn hdf5_limits() -> rspice_formats::hdf5::Hdf5Limits<'static> {
    rspice_formats::hdf5::Hdf5Limits {
        max_columns: MAX_RESULT_COLUMNS,
        max_values: MAX_RESULT_VALUES,
        coordinate_names: &RESULT_COORDINATE_NAMES,
    }
}

fn finish_hdf5(
    format: ResultImportFormat,
    decoded: rspice_formats::hdf5::DecodedHdf5,
) -> Result<ParsedResultDataset, String> {
    match decoded {
        rspice_formats::hdf5::DecodedHdf5::Section {
            family,
            coordinate_name,
            coordinate,
            signals,
        } => {
            let analysis = match family {
                rspice_formats::hdf5::Hdf5SectionFamily::Transient => AnalysisType::Transient,
                rspice_formats::hdf5::Hdf5SectionFamily::DcSweep => AnalysisType::DcSweep,
                rspice_formats::hdf5::Hdf5SectionFamily::Ac => AnalysisType::Ac,
            };
            let signals = signals
                .into_iter()
                .map(|signal| ImportedSignal {
                    name: signal.name,
                    real: signal.real,
                    imag: signal.imag,
                    unit: signal.unit,
                })
                .collect();
            finish_dataset(format, analysis, coordinate_name, coordinate, signals)
        }
        rspice_formats::hdf5::DecodedHdf5::Root {
            coordinate_name,
            coordinate,
            columns,
        } => {
            let signals = combine_real_imag_columns(format, columns)?;
            finish_dataset(
                format,
                analysis_from_coordinate(&coordinate_name),
                coordinate_name,
                coordinate,
                signals,
            )
        }
    }
}

/// Whether `name` is one of the coordinate names a headerless source may use.
fn is_coordinate_name(name: &str) -> bool {
    RESULT_COORDINATE_NAMES
        .iter()
        .any(|candidate| name.eq_ignore_ascii_case(candidate))
}

/// [`RESULT_COORDINATE_NAMES`] as a refusal spells them, so the sentence a
/// reader is shown cannot drift from the list the reader actually accepts.
fn stated_coordinate_names() -> String {
    rspice_formats::hdf5::stated_coordinate_names(&RESULT_COORDINATE_NAMES)
}

// -------------------------------------------------------------------------
// Arrow IPC and Parquet

pub(super) fn parse_arrow_ipc(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let table = rspice_formats::columnar::decode_arrow_ipc(
        bytes,
        columnar_limits(),
        format.canonical_id(),
    )?;
    finish_columnar_table(format, table)
}

pub(super) fn parse_parquet(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let table =
        rspice_formats::columnar::decode_parquet(bytes, columnar_limits(), format.canonical_id())?;
    finish_columnar_table(format, table)
}

fn columnar_limits() -> rspice_formats::columnar::ColumnarLimits {
    rspice_formats::columnar::ColumnarLimits {
        max_columns: MAX_RESULT_COLUMNS,
        max_rows: MAX_RESULT_ROWS,
        max_values: MAX_RESULT_VALUES,
    }
}

fn finish_columnar_table(
    format: ResultImportFormat,
    table: rspice_formats::columnar::DecodedColumnarTable,
) -> Result<ParsedResultDataset, String> {
    let rspice_formats::columnar::DecodedColumnarTable {
        metadata,
        mut columns,
    } = table;
    let coordinate_name = metadata
        .get("rspice.coordinate")
        .cloned()
        .unwrap_or_else(|| columns[0].0.clone());
    let coordinate_index = columns
        .iter()
        .position(|(name, _)| name == &coordinate_name)
        .ok_or_else(|| {
            adapter_error(
                format,
                format_args!("schema metadata names missing coordinate '{coordinate_name}'"),
            )
        })?;
    let coordinate = columns.remove(coordinate_index).1;
    let analysis = metadata
        .get("rspice.analysis")
        .map(|value| parse_analysis(format, value))
        .transpose()?
        .unwrap_or_else(|| analysis_from_coordinate(&coordinate_name));
    let signals = combine_real_imag_columns(format, columns)?;
    finish_dataset(format, analysis, coordinate_name, coordinate, signals)
}

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

fn combine_real_imag_columns(
    format: ResultImportFormat,
    columns: Vec<(String, Vec<f64>)>,
) -> Result<Vec<ImportedSignal>, String> {
    let mut plain = Vec::new();
    let mut complex: BTreeMap<String, ComplexComponentColumns> = BTreeMap::new();
    for (name, values) in columns {
        if let Some((base, imag)) = complex_component(&name) {
            let entry = complex.entry(base.clone()).or_default();
            let slot = if imag { &mut entry.1 } else { &mut entry.0 };
            if slot.replace(values).is_some() {
                return Err(adapter_error(
                    format,
                    format_args!("duplicate complex component '{name}'"),
                ));
            }
        } else {
            plain.push(ImportedSignal {
                name,
                real: values,
                imag: None,
                unit: None,
            });
        }
    }
    for (name, (real, imag)) in complex {
        plain.push(ImportedSignal {
            name: name.clone(),
            real: real.ok_or_else(|| {
                adapter_error(
                    format,
                    format_args!("complex signal '{name}' is missing its real component"),
                )
            })?,
            imag: Some(imag.ok_or_else(|| {
                adapter_error(
                    format,
                    format_args!("complex signal '{name}' is missing its imaginary component"),
                )
            })?),
            unit: None,
        });
    }
    Ok(plain)
}

// -------------------------------------------------------------------------
// NumPy NPY and NPZ

pub(super) fn parse_npy(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let array =
        rspice_formats::numpy::reader::decode_npy(bytes, MAX_RESULT_VALUES, format.canonical_id())?;
    let (coordinate, signals) = rspice_formats::numpy::reader::npy_matrix_to_dataset(
        array,
        MAX_RESULT_ROWS,
        MAX_RESULT_COLUMNS,
        format.canonical_id(),
    )?;
    let signals = signals
        .into_iter()
        .map(|signal| ImportedSignal {
            name: signal.name,
            real: signal.real,
            imag: signal.imag,
            unit: None,
        })
        .collect();
    finish_dataset(format, AnalysisType::DcSweep, "sample", coordinate, signals)
}

pub(super) fn parse_npz(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let mut arrays = rspice_formats::numpy::archive::decode_npz_arrays(
        bytes,
        rspice_formats::numpy::archive::NpzReadLimits {
            max_members: MAX_ARCHIVE_MEMBERS,
            max_expanded_bytes: MAX_ARCHIVE_EXPANDED_BYTES,
            max_numeric_values: MAX_RESULT_VALUES,
        },
        format.canonical_id(),
    )?;
    let coordinate_index = arrays
        .iter()
        .position(|(name, _)| is_coordinate_name(name))
        .ok_or_else(|| {
            adapter_error(
                format,
                format_args!(
                    "NPZ requires one coordinate array named {}",
                    stated_coordinate_names()
                ),
            )
        })?;
    let (coordinate_name, coordinate_array) = arrays.remove(coordinate_index);
    if coordinate_array.is_complex() {
        return Err(adapter_error(
            format,
            "NPZ coordinate array cannot be complex",
        ));
    }
    let coordinate = rspice_formats::numpy::reader::npy_vector(
        &coordinate_array,
        format.canonical_id(),
        &coordinate_name,
    )?
    .0;
    let mut signals = Vec::with_capacity(arrays.len());
    for (name, array) in arrays {
        let (real, imag) =
            rspice_formats::numpy::reader::npy_vector(&array, format.canonical_id(), &name)?;
        signals.push(ImportedSignal {
            name,
            real,
            imag,
            unit: None,
        });
    }
    finish_dataset(
        format,
        analysis_from_coordinate(&coordinate_name),
        coordinate_name,
        coordinate,
        signals,
    )
}

// -------------------------------------------------------------------------
// MATLAB v5

pub(super) fn parse_matlab_v5(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let parsed = rspice_formats::matlab::reader::MatFile::parse(
        bytes,
        MAX_RESULT_COLUMNS.saturating_mul(2),
        format.canonical_id(),
    )?;
    let arrays = parsed.arrays();
    let coordinate_index = arrays
        .iter()
        .position(|array| is_coordinate_name(array.name()));
    if let Some(coordinate_index) = coordinate_index {
        let coordinate_array = &arrays[coordinate_index];
        let (coordinate, coordinate_imag) = coordinate_array.values(format.canonical_id())?;
        if coordinate_imag.is_some() || !coordinate_array.is_vector() {
            return Err(adapter_error(
                format,
                "MATLAB coordinate variable must be a real vector",
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
                    format_args!("MATLAB variable '{}' is not a vector", array.name()),
                ));
            }
            let (real, imag) = array.values(format.canonical_id())?;
            signals.push(ImportedSignal {
                name: array.name().to_owned(),
                real,
                imag,
                unit: None,
            });
        }
        return finish_dataset(
            format,
            analysis_from_coordinate(coordinate_array.name()),
            coordinate_array.name(),
            coordinate,
            signals,
        );
    }

    if arrays.len() != 1 {
        return Err(adapter_error(
            format,
            format_args!(
                "MATLAB file requires a coordinate variable named {}",
                stated_coordinate_names()
            ),
        ));
    }
    let array = &arrays[0];
    let size = array.size();
    if size.len() != 2 || size[0] < MIN_RESULT_ROWS || size[1] < 2 {
        return Err(adapter_error(
            format,
            "without a named coordinate, MATLAB data must be one rows-by-columns table with the coordinate in column one",
        ));
    }
    let (real, imag) = array.values(format.canonical_id())?;
    if imag.is_some() {
        return Err(adapter_error(
            format,
            "a complex MATLAB table requires separate named coordinate and signal variables",
        ));
    }
    let rows = size[0];
    let columns = size[1];
    let coordinate = real[..rows].to_vec();
    let signals = (1..columns)
        .map(|column| ImportedSignal {
            name: format!("{}.{}", array.name(), column),
            real: real[column * rows..(column + 1) * rows].to_vec(),
            imag: None,
            unit: None,
        })
        .collect();
    finish_dataset(format, AnalysisType::DcSweep, "x", coordinate, signals)
}

// -------------------------------------------------------------------------
// SPICE RAW

pub(super) fn parse_spice_raw(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let mut limits = rspice_core::ResourceLimits::default();
    limits.max_external_data_bytes = MAX_RESULT_DATASET_BYTES as usize;
    limits.max_external_data_values = MAX_RESULT_VALUES;
    let parsed = rspice_core::io::parse_raw_reader_with_limits(&mut Cursor::new(bytes), limits)
        .map_err(|error| adapter_error(format, error))?;
    let mut waveforms = parsed.waveforms.into_iter();
    let scale = waveforms
        .next()
        .ok_or_else(|| adapter_error(format, "rawfile contains no variables"))?;
    let coordinate_name = scale.name;
    let coordinate = scale.y;
    let mut signals: Vec<ImportedSignal> = Vec::new();
    for waveform in waveforms {
        signals.push(ImportedSignal {
            name: waveform.name,
            real: waveform.y,
            imag: waveform.y_imag,
            unit: None,
        });
    }
    let plot = parsed.header.plotname.to_ascii_lowercase();
    let analysis = if parsed.header.is_complex || plot.contains("ac") {
        AnalysisType::Ac
    } else if plot.contains("tran") || coordinate_name.to_ascii_lowercase().contains("time") {
        AnalysisType::Transient
    } else {
        AnalysisType::DcSweep
    };
    finish_dataset(format, analysis, coordinate_name, coordinate, signals)
}

// -------------------------------------------------------------------------
// Cadence PSF ASCII

pub(super) fn parse_psf_ascii(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| adapter_error(format, format_args!("source is not UTF-8: {error}")))?;
    let mut analysis = None;
    let mut coordinate_name = None;
    let mut signal_names = Vec::new();
    let mut value_lines = Vec::new();
    let mut section = "";
    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
            continue;
        }
        match line.to_ascii_uppercase().as_str() {
            "HEADER" | "TYPE" | "SWEEP" | "TRACE" | "VALUE" | "END" => {
                section = line;
                continue;
            }
            _ => {}
        }
        if section.eq_ignore_ascii_case("HEADER") {
            let fields = psf_tokens(line, format, line_number)?;
            if fields.len() >= 2
                && matches!(
                    fields[0].to_ascii_lowercase().as_str(),
                    "analysis" | "type" | "sweepmode"
                )
            {
                analysis = Some(parse_analysis(format, &fields[1])?);
            }
        } else if section.eq_ignore_ascii_case("SWEEP") {
            let fields = psf_tokens(line, format, line_number)?;
            if fields.is_empty() {
                continue;
            }
            if coordinate_name.replace(fields[0].clone()).is_some() {
                return Err(adapter_error(
                    format,
                    "PSF ASCII declares multiple sweep axes",
                ));
            }
        } else if section.eq_ignore_ascii_case("TRACE") {
            let fields = psf_tokens(line, format, line_number)?;
            if fields.is_empty() {
                continue;
            }
            signal_names.push(fields[0].clone());
        } else if section.eq_ignore_ascii_case("VALUE") {
            value_lines.push((line_number, line));
        }
    }
    let coordinate_name = coordinate_name
        .ok_or_else(|| adapter_error(format, "PSF ASCII is missing a SWEEP axis declaration"))?;
    if signal_names.is_empty() {
        return Err(adapter_error(
            format,
            "PSF ASCII is missing TRACE declarations",
        ));
    }
    if signal_names.len() + 1 > MAX_RESULT_COLUMNS {
        return Err(adapter_error(
            format,
            "PSF ASCII trace-count limit exceeded",
        ));
    }
    let mut coordinate = Vec::new();
    let mut components = vec![Vec::new(); signal_names.len()];
    for (line_number, line) in value_lines {
        if coordinate.len() >= MAX_RESULT_ROWS {
            return Err(adapter_error(format, "PSF ASCII row limit exceeded"));
        }
        let fields = psf_tokens(line, format, line_number)?;
        if fields.len() != signal_names.len() + 1 {
            return Err(adapter_error(
                format,
                format_args!(
                    "VALUE row {line_number} has {} fields; expected {}",
                    fields.len(),
                    signal_names.len() + 1
                ),
            ));
        }
        coordinate.push(parse_psf_number(
            format,
            &fields[0],
            line_number,
            &coordinate_name,
        )?);
        for (index, signal) in signal_names.iter().enumerate() {
            components[index].push(parse_psf_number(
                format,
                &fields[index + 1],
                line_number,
                signal,
            )?);
        }
    }
    let signals = signal_names
        .into_iter()
        .zip(components)
        .map(|(name, real)| ImportedSignal {
            name,
            real,
            imag: None,
            unit: None,
        })
        .collect();
    finish_dataset(
        format,
        analysis.unwrap_or_else(|| analysis_from_coordinate(&coordinate_name)),
        coordinate_name,
        coordinate,
        signals,
    )
}

fn psf_tokens(
    line: &str,
    format: ResultImportFormat,
    line_number: usize,
) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut quoted = false;
    let mut chars = line.chars();
    while let Some(character) = chars.next() {
        if quoted {
            match character {
                '"' => quoted = false,
                '\\' => {
                    let escaped = chars.next().ok_or_else(|| {
                        adapter_error(format, format_args!("line {line_number} ends in an escape"))
                    })?;
                    token.push(escaped);
                }
                _ => token.push(character),
            }
        } else if character == '"' {
            quoted = true;
        } else if character.is_whitespace() || character == '(' || character == ')' {
            if !token.is_empty() {
                tokens.push(std::mem::take(&mut token));
            }
        } else {
            token.push(character);
        }
    }
    if quoted {
        return Err(adapter_error(
            format,
            format_args!("line {line_number} has an unterminated quoted token"),
        ));
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    Ok(tokens)
}

fn parse_psf_number(
    format: ResultImportFormat,
    token: &str,
    line: usize,
    identity: &str,
) -> Result<f64, String> {
    let value = token.parse::<f64>().map_err(|_| {
        adapter_error(
            format,
            format_args!("line {line} has invalid numeric token '{token}' for '{identity}'"),
        )
    })?;
    if !value.is_finite() {
        return Err(adapter_error(
            format,
            format_args!("line {line} has non-finite value for '{identity}'"),
        ));
    }
    Ok(value)
}

// -------------------------------------------------------------------------

#[path = "result_import_adapters/digital.rs"]
mod digital;

pub(super) use digital::{looks_like_fst, parse_fst, parse_vcd};

#[cfg(test)]
use digital::preflight_fst_for_test as preflight_fst;

#[cfg(test)]
#[path = "result_import_adapters/tests.rs"]
mod tests;
