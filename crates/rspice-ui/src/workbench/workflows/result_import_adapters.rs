//! Bounded adapters for structured and simulator-native result formats.
//!
//! Every entry point consumes the already size-limited byte slice owned by the
//! import transaction.  Adapters reject ambiguous mappings rather than
//! inventing domain or signal identity.

use super::*;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{BufReader, Cursor, Read};

const MAX_ARCHIVE_MEMBERS: usize = 1_024;
const MAX_ARCHIVE_EXPANDED_BYTES: u64 = MAX_RESULT_DATASET_BYTES;
const MAX_SIGNAL_NAME_BYTES: usize = 1_024;
const MAX_EXACT_F64_INTEGER: u64 = 1_u64 << 53;
const MAX_RESULT_VALUES: usize = MAX_RESULT_DATASET_BYTES as usize / std::mem::size_of::<f64>();
const MAX_FST_TOP_LEVEL_BLOCKS: usize = 1_024;
const FST_HEADER_SECTION_BYTES: u64 = 329;

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
                "the coordinate contains {} samples; at least {MIN_RESULT_ROWS} are required",
                coordinate.len()
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

fn exact_signed_integer(
    format: ResultImportFormat,
    identity: &str,
    value: i64,
) -> Result<f64, String> {
    if value.unsigned_abs() > MAX_EXACT_F64_INTEGER {
        Err(adapter_error(
            format,
            format_args!("'{identity}' integer {value} cannot be represented exactly as f64"),
        ))
    } else {
        Ok(value as f64)
    }
}

fn exact_unsigned_integer(
    format: ResultImportFormat,
    identity: &str,
    value: u64,
) -> Result<f64, String> {
    if value > MAX_EXACT_F64_INTEGER {
        Err(adapter_error(
            format,
            format_args!("'{identity}' integer {value} cannot be represented exactly as f64"),
        ))
    } else {
        Ok(value as f64)
    }
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
    let file = rustyhdf5::File::from_bytes(bytes.to_vec())
        .map_err(|error| adapter_error(format, format_args!("invalid HDF5 container: {error}")))?;
    let root = file.root();
    let groups = root.groups().map_err(|error| {
        adapter_error(format, format_args!("could not enumerate groups: {error}"))
    })?;
    let supported = groups
        .iter()
        .filter(|name| matches!(name.as_str(), "transient" | "dc_sweep" | "ac"))
        .cloned()
        .collect::<Vec<_>>();
    if supported.len() > 1 {
        return Err(adapter_error(
            format,
            format_args!(
                "the file contains multiple waveform sections ({}); import one analysis per file",
                supported.join(", ")
            ),
        ));
    }
    if let Some(section) = supported.first() {
        return parse_rspice_hdf5_section(&file, section, format);
    }
    parse_generic_hdf5_root(&file, format)
}

pub(super) fn parse_matlab_v73(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let file = rustyhdf5::File::from_bytes(bytes.to_vec()).map_err(|error| {
        adapter_error(
            format,
            format_args!("invalid MATLAB 7.3/HDF5 container: {error}"),
        )
    })?;
    parse_generic_hdf5_root(&file, format)
}

fn parse_rspice_hdf5_section(
    file: &rustyhdf5::File,
    section: &str,
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let group = file.group(section).map_err(|error| {
        adapter_error(format, format_args!("could not open /{section}: {error}"))
    })?;
    let attrs = group.attrs().map_err(|error| {
        adapter_error(
            format,
            format_args!("could not read /{section} attributes: {error}"),
        )
    })?;
    let signal_count = hdf_i64_attr(&attrs, "signal_count", format)?;
    let signal_count = usize::try_from(signal_count).map_err(|_| {
        adapter_error(
            format,
            format_args!("/{section} signal_count is negative or too large"),
        )
    })?;
    if signal_count == 0 || signal_count + 1 > MAX_RESULT_COLUMNS {
        return Err(adapter_error(
            format,
            format_args!("/{section} declares invalid signal_count {signal_count}"),
        ));
    }
    if section == "ac" {
        let coordinate = hdf_f64_dataset(&group, "frequency", format)?;
        ensure_table_value_limit(format, coordinate.len(), 1 + signal_count.saturating_mul(2))?;
        let mut signals = Vec::with_capacity(signal_count);
        for index in 0..signal_count {
            let prefix = format!("signal_{index:04}");
            let name = hdf_string_attr(&attrs, &format!("{prefix}_name"), format)?;
            signals.push(ImportedSignal {
                name,
                real: hdf_f64_dataset(&group, &format!("{prefix}_real"), format)?,
                imag: Some(hdf_f64_dataset(&group, &format!("{prefix}_imag"), format)?),
                unit: hdf_stated_unit(&attrs, &format!("{prefix}_unit")),
            });
        }
        return finish_dataset(format, AnalysisType::Ac, "frequency", coordinate, signals);
    }

    let coordinate_name = hdf_string_attr(&attrs, "independent_name", format)?;
    let coordinate = hdf_f64_dataset(&group, "independent", format)?;
    ensure_table_value_limit(format, coordinate.len(), 1 + signal_count)?;
    let mut signals = Vec::with_capacity(signal_count);
    for index in 0..signal_count {
        let prefix = format!("signal_{index:04}");
        signals.push(ImportedSignal {
            name: hdf_string_attr(&attrs, &format!("{prefix}_name"), format)?,
            real: hdf_f64_dataset(&group, &prefix, format)?,
            imag: None,
            unit: hdf_stated_unit(&attrs, &format!("{prefix}_unit")),
        });
    }
    let analysis = if section == "transient" {
        AnalysisType::Transient
    } else {
        AnalysisType::DcSweep
    };
    finish_dataset(format, analysis, coordinate_name, coordinate, signals)
}

fn parse_generic_hdf5_root(
    file: &rustyhdf5::File,
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let root = file.root();
    let names = root.datasets().map_err(|error| {
        adapter_error(
            format,
            format_args!("could not enumerate datasets: {error}"),
        )
    })?;
    if names.len() > MAX_RESULT_COLUMNS.saturating_mul(2) {
        return Err(adapter_error(format, "too many root datasets"));
    }
    let coordinate_name =
        select_coordinate_name(names.iter().map(String::as_str)).ok_or_else(|| {
            adapter_error(
                format,
                "no unambiguous root coordinate dataset named time, frequency, freq, sweep, or x",
            )
        })?;
    let coordinate = hdf_f64_dataset(&root, &coordinate_name, format)?;
    ensure_table_value_limit(format, coordinate.len(), names.len())?;
    let mut columns = Vec::new();
    for name in names {
        if name.eq_ignore_ascii_case(&coordinate_name) || name.starts_with('#') {
            continue;
        }
        let dataset = root.dataset(&name).map_err(|error| {
            adapter_error(
                format,
                format_args!("could not open dataset '{name}': {error}"),
            )
        })?;
        let shape = dataset.shape().map_err(|error| {
            adapter_error(
                format,
                format_args!("could not inspect dataset '{name}': {error}"),
            )
        })?;
        let count = shape
            .iter()
            .try_fold(1_u64, |count, dim| count.checked_mul(*dim))
            .ok_or_else(|| {
                adapter_error(format, format_args!("dataset '{name}' shape overflows"))
            })?;
        if count != coordinate.len() as u64 {
            return Err(adapter_error(
                format,
                format_args!(
                    "dataset '{name}' has {count} values; coordinate '{coordinate_name}' has {}",
                    coordinate.len()
                ),
            ));
        }
        let values = hdf_dataset_values(&dataset, &name, format)?;
        columns.push((name, values));
    }
    let signals = combine_real_imag_columns(format, columns)?;
    finish_dataset(
        format,
        analysis_from_coordinate(&coordinate_name),
        coordinate_name,
        coordinate,
        signals,
    )
}

fn hdf_f64_dataset(
    group: &rustyhdf5::Group<'_>,
    name: &str,
    format: ResultImportFormat,
) -> Result<Vec<f64>, String> {
    let dataset = group.dataset(name).map_err(|error| {
        adapter_error(
            format,
            format_args!("could not open numeric dataset '{name}': {error}"),
        )
    })?;
    hdf_dataset_values(&dataset, name, format)
}

fn hdf_dataset_values(
    dataset: &rustyhdf5::Dataset<'_>,
    name: &str,
    format: ResultImportFormat,
) -> Result<Vec<f64>, String> {
    let count = dataset
        .shape()
        .map_err(|error| {
            adapter_error(
                format,
                format_args!("could not inspect dataset '{name}' shape: {error}"),
            )
        })?
        .into_iter()
        .try_fold(1_u64, |count, dimension| count.checked_mul(dimension))
        .ok_or_else(|| adapter_error(format, format_args!("dataset '{name}' shape overflows")))?;
    if count > MAX_RESULT_VALUES as u64 {
        return Err(adapter_error(
            format,
            format_args!(
                "dataset '{name}' contains {count} values; the limit is {MAX_RESULT_VALUES}"
            ),
        ));
    }
    let dtype = dataset.dtype().map_err(|error| {
        adapter_error(
            format,
            format_args!("could not inspect dataset '{name}': {error}"),
        )
    })?;
    match dtype {
        rustyhdf5::DType::I64 => dataset
            .read_i64()
            .map_err(|error| {
                adapter_error(format, format_args!("could not read '{name}': {error}"))
            })?
            .into_iter()
            .map(|value| exact_signed_integer(format, name, value))
            .collect(),
        rustyhdf5::DType::U64 => dataset
            .read_u64()
            .map_err(|error| {
                adapter_error(format, format_args!("could not read '{name}': {error}"))
            })?
            .into_iter()
            .map(|value| exact_unsigned_integer(format, name, value))
            .collect(),
        _ => dataset.read_f64().map_err(|error| {
            adapter_error(
                format,
                format_args!("could not read numeric dataset '{name}': {error}"),
            )
        }),
    }
}

fn hdf_string_attr(
    attrs: &HashMap<String, rustyhdf5::AttrValue>,
    name: &str,
    format: ResultImportFormat,
) -> Result<String, String> {
    match attrs.get(name) {
        Some(rustyhdf5::AttrValue::String(value)) if !value.trim().is_empty() => Ok(value.clone()),
        Some(other) => Err(adapter_error(
            format,
            format_args!("attribute '{name}' must be a non-empty string, found {other:?}"),
        )),
        None => Err(adapter_error(
            format,
            format_args!("missing attribute '{name}'"),
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
    format: ResultImportFormat,
) -> Result<i64, String> {
    match attrs.get(name) {
        Some(rustyhdf5::AttrValue::I64(value)) => Ok(*value),
        Some(other) => Err(adapter_error(
            format,
            format_args!("attribute '{name}' must be an integer, found {other:?}"),
        )),
        None => Err(adapter_error(
            format,
            format_args!("missing attribute '{name}'"),
        )),
    }
}

fn select_coordinate_name<'a>(names: impl Iterator<Item = &'a str>) -> Option<String> {
    let names = names.collect::<Vec<_>>();
    for candidate in ["time", "frequency", "freq", "sweep", "x"] {
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
    format: ResultImportFormat,
    rows: usize,
    columns: usize,
) -> Result<(), String> {
    let values = rows
        .checked_mul(columns)
        .ok_or_else(|| adapter_error(format, "table value count overflow"))?;
    if values > MAX_RESULT_VALUES {
        Err(adapter_error(
            format,
            format_args!("the table contains {values} values; the limit is {MAX_RESULT_VALUES}"),
        ))
    } else {
        Ok(())
    }
}

// -------------------------------------------------------------------------
// Arrow IPC and Parquet

pub(super) fn parse_arrow_ipc(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    use arrow_ipc::reader::{FileReader, StreamReader};
    let mut batches = Vec::new();
    let file_attempt = FileReader::try_new(Cursor::new(bytes), None);
    let metadata = match file_attempt {
        Ok(reader) => {
            let metadata = reader.schema().metadata().clone();
            for batch in reader {
                batches.push(batch.map_err(|error| {
                    adapter_error(format, format_args!("invalid Arrow record batch: {error}"))
                })?);
            }
            metadata
        }
        Err(file_error) => {
            let reader = StreamReader::try_new(Cursor::new(bytes), None).map_err(|stream_error| {
                adapter_error(
                    format,
                    format_args!(
                        "neither Arrow file nor stream framing is valid (file: {file_error}; stream: {stream_error})"
                    ),
                )
            })?;
            let metadata = reader.schema().metadata().clone();
            for batch in reader {
                batches.push(batch.map_err(|error| {
                    adapter_error(format, format_args!("invalid Arrow stream batch: {error}"))
                })?);
            }
            metadata
        }
    };
    parse_arrow_batches(format, batches, metadata)
}

pub(super) fn parse_parquet(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    let builder = ParquetRecordBatchReaderBuilder::try_new(bytes::Bytes::copy_from_slice(bytes))
        .map_err(|error| {
            adapter_error(format, format_args!("invalid Parquet metadata: {error}"))
        })?;
    let metadata = builder.schema().metadata().clone();
    let reader = builder.with_batch_size(16_384).build().map_err(|error| {
        adapter_error(
            format,
            format_args!("could not create Parquet reader: {error}"),
        )
    })?;
    let mut batches = Vec::new();
    for batch in reader {
        batches.push(batch.map_err(|error| {
            adapter_error(format, format_args!("invalid Parquet row group: {error}"))
        })?);
    }
    parse_arrow_batches(format, batches, metadata)
}

fn parse_arrow_batches(
    format: ResultImportFormat,
    batches: Vec<arrow_array::RecordBatch>,
    metadata: HashMap<String, String>,
) -> Result<ParsedResultDataset, String> {
    let first = batches
        .first()
        .ok_or_else(|| adapter_error(format, "the table contains no record batches"))?;
    let schema = first.schema();
    if schema.fields().len() < 2 || schema.fields().len() > MAX_RESULT_COLUMNS.saturating_mul(2) {
        return Err(adapter_error(
            format,
            format_args!(
                "the table has {} fields; expected 2..={}",
                schema.fields().len(),
                MAX_RESULT_COLUMNS
            ),
        ));
    }
    let row_count = batches
        .iter()
        .try_fold(0_usize, |count, batch| count.checked_add(batch.num_rows()))
        .ok_or_else(|| adapter_error(format, "row count overflow"))?;
    if row_count > MAX_RESULT_ROWS {
        return Err(adapter_error(
            format,
            format_args!("the table has {row_count} rows; the limit is {MAX_RESULT_ROWS}"),
        ));
    }
    let table_values = row_count
        .checked_mul(schema.fields().len())
        .ok_or_else(|| adapter_error(format, "table value count overflow"))?;
    if table_values > MAX_RESULT_VALUES {
        return Err(adapter_error(
            format,
            format_args!(
                "the table contains {table_values} values; the limit is {MAX_RESULT_VALUES}"
            ),
        ));
    }
    let mut columns = schema
        .fields()
        .iter()
        .map(|field| (field.name().clone(), Vec::with_capacity(row_count)))
        .collect::<Vec<_>>();
    for batch in batches {
        if batch.schema().as_ref() != schema.as_ref() {
            return Err(adapter_error(
                format,
                "record-batch schema changed within the source",
            ));
        }
        for (index, array) in batch.columns().iter().enumerate() {
            let name = columns[index].0.clone();
            let values = arrow_numeric_values(format, &name, array.as_ref())?;
            columns[index].1.extend(values);
        }
    }
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

fn arrow_numeric_values(
    format: ResultImportFormat,
    name: &str,
    array: &dyn arrow_array::Array,
) -> Result<Vec<f64>, String> {
    use arrow_array::{
        BooleanArray, Float32Array, Float64Array, Int8Array, Int16Array, Int32Array, Int64Array,
        UInt8Array, UInt16Array, UInt32Array, UInt64Array,
    };
    if array.null_count() != 0 {
        return Err(adapter_error(
            format,
            format_args!(
                "column '{name}' contains {} null values",
                array.null_count()
            ),
        ));
    }
    macro_rules! float_values {
        ($ty:ty) => {
            array
                .as_any()
                .downcast_ref::<$ty>()
                .map(|array| array.values().iter().map(|value| *value as f64).collect())
        };
    }
    macro_rules! signed_values {
        ($ty:ty) => {
            array.as_any().downcast_ref::<$ty>().map(|array| {
                array
                    .values()
                    .iter()
                    .map(|value| exact_signed_integer(format, name, *value as i64))
                    .collect::<Result<Vec<_>, _>>()
            })
        };
    }
    macro_rules! unsigned_values {
        ($ty:ty) => {
            array.as_any().downcast_ref::<$ty>().map(|array| {
                array
                    .values()
                    .iter()
                    .map(|value| exact_unsigned_integer(format, name, *value as u64))
                    .collect::<Result<Vec<_>, _>>()
            })
        };
    }
    let values = float_values!(Float64Array)
        .or_else(|| float_values!(Float32Array))
        .map(Ok)
        .or_else(|| signed_values!(Int64Array))
        .or_else(|| signed_values!(Int32Array))
        .or_else(|| signed_values!(Int16Array))
        .or_else(|| signed_values!(Int8Array))
        .or_else(|| unsigned_values!(UInt64Array))
        .or_else(|| unsigned_values!(UInt32Array))
        .or_else(|| unsigned_values!(UInt16Array))
        .or_else(|| unsigned_values!(UInt8Array))
        .or_else(|| {
            array.as_any().downcast_ref::<BooleanArray>().map(|array| {
                Ok((0..array.len())
                    .map(|index| if array.value(index) { 1.0 } else { 0.0 })
                    .collect())
            })
        });
    values.ok_or_else(|| {
        adapter_error(
            format,
            format_args!(
                "column '{name}' has unsupported Arrow type {}",
                array.data_type()
            ),
        )
    })?
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

#[derive(Debug)]
struct NpyArray {
    shape: Vec<usize>,
    fortran: bool,
    real: Vec<f64>,
    imag: Option<Vec<f64>>,
}

pub(super) fn parse_npy(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let array = decode_npy(bytes, format)?;
    let (coordinate, signals) = npy_matrix_to_dataset(array, format)?;
    finish_dataset(format, AnalysisType::DcSweep, "sample", coordinate, signals)
}

pub(super) fn parse_npz(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|error| adapter_error(format, format_args!("invalid NPZ archive: {error}")))?;
    if archive.len() > MAX_ARCHIVE_MEMBERS {
        return Err(adapter_error(
            format,
            format_args!(
                "archive has {} members; the limit is {MAX_ARCHIVE_MEMBERS}",
                archive.len()
            ),
        ));
    }
    let mut arrays = Vec::new();
    let mut names = HashSet::new();
    let mut expanded = 0_u64;
    for index in 0..archive.len() {
        let member = archive.by_index(index).map_err(|error| {
            adapter_error(
                format,
                format_args!("invalid archive member {index}: {error}"),
            )
        })?;
        if member.is_dir() {
            continue;
        }
        let member_name = member.name().to_owned();
        if member_name.starts_with('/') || member_name.contains("..") || member_name.contains('\\')
        {
            return Err(adapter_error(
                format,
                format_args!("unsafe archive member '{member_name}'"),
            ));
        }
        if !member_name.to_ascii_lowercase().ends_with(".npy") {
            return Err(adapter_error(
                format,
                format_args!(
                    "unsupported NPZ member '{member_name}'; only .npy arrays are accepted"
                ),
            ));
        }
        expanded = expanded
            .checked_add(member.size())
            .ok_or_else(|| adapter_error(format, "archive expanded-size accounting overflow"))?;
        if expanded > MAX_ARCHIVE_EXPANDED_BYTES {
            return Err(adapter_error(format, "NPZ expanded-byte limit exceeded"));
        }
        let stem = Path::new(&member_name)
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                adapter_error(format, format_args!("invalid member name '{member_name}'"))
            })?
            .to_owned();
        if !names.insert(stem.to_ascii_lowercase()) {
            return Err(adapter_error(
                format,
                format_args!("archive repeats array identity '{stem}'"),
            ));
        }
        let mut member_bytes = Vec::with_capacity(usize::try_from(member.size()).unwrap_or(0));
        member
            .take(MAX_RESULT_DATASET_BYTES + 1)
            .read_to_end(&mut member_bytes)
            .map_err(|error| {
                adapter_error(
                    format,
                    format_args!("could not decode '{member_name}': {error}"),
                )
            })?;
        arrays.push((stem, decode_npy(&member_bytes, format)?));
    }
    let coordinate_index = arrays
        .iter()
        .position(|(name, _)| {
            matches!(
                name.to_ascii_lowercase().as_str(),
                "time" | "frequency" | "freq" | "sweep" | "x"
            )
        })
        .ok_or_else(|| {
            adapter_error(
                format,
                "NPZ requires one coordinate array named time, frequency, freq, sweep, or x",
            )
        })?;
    let (coordinate_name, coordinate_array) = arrays.remove(coordinate_index);
    if coordinate_array.imag.is_some() {
        return Err(adapter_error(
            format,
            "NPZ coordinate array cannot be complex",
        ));
    }
    let coordinate = npy_vector(&coordinate_array, format, &coordinate_name)?.0;
    let mut signals = Vec::with_capacity(arrays.len());
    for (name, array) in arrays {
        let (real, imag) = npy_vector(&array, format, &name)?;
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

fn decode_npy(bytes: &[u8], format: ResultImportFormat) -> Result<NpyArray, String> {
    use npyz::{DType, Order, TypeChar};
    let file = npyz::NpyFile::new(Cursor::new(bytes))
        .map_err(|error| adapter_error(format, format_args!("invalid NPY header: {error}")))?;
    let shape = file
        .shape()
        .iter()
        .map(|value| {
            usize::try_from(*value)
                .map_err(|_| adapter_error(format, "NPY dimension exceeds this platform"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if shape.is_empty() || shape.len() > 2 {
        return Err(adapter_error(
            format,
            format_args!("NPY shape {shape:?} is not a one- or two-dimensional waveform table"),
        ));
    }
    let count = shape
        .iter()
        .try_fold(1_usize, |count, dim| count.checked_mul(*dim))
        .ok_or_else(|| adapter_error(format, "NPY shape product overflow"))?;
    if count > MAX_RESULT_VALUES {
        return Err(adapter_error(format, "NPY numeric-value limit exceeded"));
    }
    let fortran = file.order() == Order::Fortran;
    let DType::Plain(type_string) = file.dtype() else {
        return Err(adapter_error(
            format,
            "structured and nested NPY dtypes require an explicit mapping and are not accepted",
        ));
    };
    macro_rules! real {
        ($ty:ty) => {{
            let values = file.into_vec::<$ty>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
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
            let values = file.into_vec::<$ty>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
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
            let values = file.into_vec::<i64>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
            NpyArray {
                shape,
                fortran,
                real: values
                    .into_iter()
                    .map(|value| exact_signed_integer(format, "NPY array", value))
                    .collect::<Result<Vec<_>, _>>()?,
                imag: None,
            }
        }
        (TypeChar::Uint, 1) => real!(u8),
        (TypeChar::Uint, 2) => real!(u16),
        (TypeChar::Uint, 4) => real!(u32),
        (TypeChar::Uint, 8) => {
            let values = file.into_vec::<u64>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
            NpyArray {
                shape,
                fortran,
                real: values
                    .into_iter()
                    .map(|value| exact_unsigned_integer(format, "NPY array", value))
                    .collect::<Result<Vec<_>, _>>()?,
                imag: None,
            }
        }
        (TypeChar::Bool, 1) => {
            let values = file.into_vec::<bool>().map_err(|error| {
                adapter_error(format, format_args!("could not decode NPY values: {error}"))
            })?;
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
            return Err(adapter_error(
                format,
                format_args!("unsupported NPY dtype {kind:?}{size}"),
            ));
        }
    };
    Ok(array)
}

fn npy_vector(
    array: &NpyArray,
    format: ResultImportFormat,
    name: &str,
) -> Result<(Vec<f64>, Option<Vec<f64>>), String> {
    let len = match array.shape.as_slice() {
        [len] => *len,
        [rows, 1] => *rows,
        [1, columns] => *columns,
        _ => {
            return Err(adapter_error(
                format,
                format_args!("NPZ array '{name}' has non-vector shape {:?}", array.shape),
            ));
        }
    };
    if array.real.len() != len || array.imag.as_ref().is_some_and(|imag| imag.len() != len) {
        return Err(adapter_error(
            format,
            format_args!("NPZ array '{name}' payload does not match its shape"),
        ));
    }
    Ok((array.real.clone(), array.imag.clone()))
}

fn npy_matrix_to_dataset(
    array: NpyArray,
    format: ResultImportFormat,
) -> Result<(Vec<f64>, Vec<ImportedSignal>), String> {
    let (rows, columns) = match array.shape.as_slice() {
        [rows] => (*rows, 1),
        [rows, columns] => (*rows, *columns),
        _ => unreachable!(),
    };
    if !(MIN_RESULT_ROWS..=MAX_RESULT_ROWS).contains(&rows)
        || columns == 0
        || columns > MAX_RESULT_COLUMNS
    {
        return Err(adapter_error(
            format,
            format_args!("NPY waveform shape {:?} exceeds import bounds", array.shape),
        ));
    }
    let index = |row: usize, column: usize| {
        if array.fortran {
            column * rows + row
        } else {
            row * columns + column
        }
    };
    if array.imag.is_none() && columns >= 2 {
        let coordinate = (0..rows).map(|row| array.real[index(row, 0)]).collect();
        let signals = (1..columns)
            .map(|column| ImportedSignal {
                name: format!("signal_{column}"),
                real: (0..rows)
                    .map(|row| array.real[index(row, column)])
                    .collect(),
                imag: None,
                unit: None,
            })
            .collect();
        return Ok((coordinate, signals));
    }
    let coordinate = (0..rows).map(|row| row as f64).collect();
    let signals = (0..columns)
        .map(|column| ImportedSignal {
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
    Ok((coordinate, signals))
}

// -------------------------------------------------------------------------
// MATLAB v5

pub(super) fn parse_matlab_v5(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let parsed = std::panic::catch_unwind(|| matfile::MatFile::parse(Cursor::new(bytes)))
        .map_err(|_| adapter_error(format, "MATLAB parser rejected malformed input"))?
        .map_err(|error| adapter_error(format, format_args!("invalid MATLAB v5 file: {error}")))?;
    if parsed.arrays().len() > MAX_RESULT_COLUMNS.saturating_mul(2) {
        return Err(adapter_error(format, "too many MATLAB variables"));
    }
    let coordinate_index = parsed.arrays().iter().position(|array| {
        matches!(
            array.name().to_ascii_lowercase().as_str(),
            "time" | "frequency" | "freq" | "sweep" | "x"
        )
    });
    if let Some(coordinate_index) = coordinate_index {
        let coordinate_array = &parsed.arrays()[coordinate_index];
        let (coordinate, coordinate_imag) = matlab_values(coordinate_array, format)?;
        if coordinate_imag.is_some() || !matlab_is_vector(coordinate_array.size()) {
            return Err(adapter_error(
                format,
                "MATLAB coordinate variable must be a real vector",
            ));
        }
        let mut signals = Vec::new();
        for (index, array) in parsed.arrays().iter().enumerate() {
            if index == coordinate_index {
                continue;
            }
            if !matlab_is_vector(array.size()) {
                return Err(adapter_error(
                    format,
                    format_args!("MATLAB variable '{}' is not a vector", array.name()),
                ));
            }
            let (real, imag) = matlab_values(array, format)?;
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

    if parsed.arrays().len() != 1 {
        return Err(adapter_error(
            format,
            "MATLAB file requires a coordinate variable named time, frequency, freq, sweep, or x",
        ));
    }
    let array = &parsed.arrays()[0];
    let size = array.size();
    if size.len() != 2 || size[0] < MIN_RESULT_ROWS || size[1] < 2 {
        return Err(adapter_error(
            format,
            "without a named coordinate, MATLAB data must be one rows-by-columns table with the coordinate in column one",
        ));
    }
    let (real, imag) = matlab_values(array, format)?;
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

fn matlab_is_vector(size: &[usize]) -> bool {
    size.iter().filter(|dimension| **dimension > 1).count() <= 1
}

fn matlab_values(
    array: &matfile::Array,
    format: ResultImportFormat,
) -> Result<(Vec<f64>, Option<Vec<f64>>), String> {
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
                .map(|value| exact_signed_integer(format, array.name(), *value))
                .collect::<Result<Vec<_>, _>>()?,
            imag.as_ref()
                .map(|values| {
                    values
                        .iter()
                        .map(|value| exact_signed_integer(format, array.name(), *value))
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
        ),
        matfile::NumericData::UInt64 { real, imag } => (
            real.iter()
                .map(|value| exact_unsigned_integer(format, array.name(), *value))
                .collect::<Result<Vec<_>, _>>()?,
            imag.as_ref()
                .map(|values| {
                    values
                        .iter()
                        .map(|value| exact_unsigned_integer(format, array.name(), *value))
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
    if values.0.len() != expected || values.1.as_ref().is_some_and(|imag| imag.len() != expected) {
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
