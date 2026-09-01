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
                unit: None,
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
            unit: None,
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
// VCD and FST digital event traces

#[derive(Debug)]
struct DigitalSignal {
    name: String,
    width: usize,
}

#[derive(Debug)]
struct DigitalEvent {
    tick: u64,
    signal: usize,
    value: f64,
}

pub(super) fn parse_vcd(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| adapter_error(format, format_args!("source is not UTF-8: {error}")))?;
    let mut timescale = None;
    let mut scopes = Vec::new();
    let mut signals: Vec<DigitalSignal> = Vec::new();
    let mut identifiers: HashMap<String, usize> = HashMap::new();
    let mut aliases = Vec::new();
    let mut events = Vec::new();
    let mut current_tick = 0_u64;
    let mut in_definitions = true;
    let mut in_dumpvars = false;
    let mut directive = String::new();

    for (line_index, raw_line) in text.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "$dumpvars" || line == "$dumpon" {
            in_dumpvars = true;
            continue;
        }
        if in_dumpvars && line == "$end" {
            in_dumpvars = false;
            continue;
        }
        if line.starts_with('$') && !line.contains("$end") {
            directive.clear();
            directive.push_str(line);
            continue;
        }
        let owned;
        let line = if !directive.is_empty() {
            directive.push(' ');
            directive.push_str(line);
            owned = std::mem::take(&mut directive);
            owned.as_str()
        } else {
            line
        };
        if line.starts_with("$timescale") {
            let body = line
                .trim_start_matches("$timescale")
                .trim_end_matches("$end")
                .trim();
            if timescale
                .replace(parse_vcd_timescale(body, format)?)
                .is_some()
            {
                return Err(adapter_error(
                    format,
                    "VCD declares timescale more than once",
                ));
            }
        } else if line.starts_with("$scope") {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 4 || fields.last() != Some(&"$end") {
                return Err(adapter_error(
                    format,
                    format_args!("malformed $scope at line {line_number}"),
                ));
            }
            validate_name(format, "scope", fields[2])?;
            scopes.push(fields[2].to_owned());
        } else if line.starts_with("$upscope") {
            if scopes.pop().is_none() {
                return Err(adapter_error(
                    format,
                    format_args!("unbalanced $upscope at line {line_number}"),
                ));
            }
        } else if line.starts_with("$var") {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 6 || fields.last() != Some(&"$end") {
                return Err(adapter_error(
                    format,
                    format_args!("malformed $var at line {line_number}"),
                ));
            }
            let width = fields[2].parse::<usize>().map_err(|_| {
                adapter_error(
                    format,
                    format_args!("invalid $var width at line {line_number}"),
                )
            })?;
            if width == 0 || width > 53 {
                return Err(adapter_error(
                    format,
                    format_args!(
                        "VCD variable '{}' width {width} cannot be represented exactly as f64",
                        fields[4]
                    ),
                ));
            }
            let mut name = scopes.join(".");
            if !name.is_empty() {
                name.push('.');
            }
            name.push_str(fields[4]);
            validate_name(format, "signal", &name)?;
            let identifier = fields[3].to_owned();
            let signal_index = if let Some(existing) = identifiers.get(&identifier).copied() {
                if signals[existing].width != width {
                    return Err(adapter_error(format, "VCD alias changes declared width"));
                }
                aliases.push((existing, name));
                existing
            } else {
                if signals.len() >= MAX_RESULT_COLUMNS - 1 {
                    return Err(adapter_error(format, "VCD signal-count limit exceeded"));
                }
                let index = signals.len();
                signals.push(DigitalSignal { name, width });
                identifiers.insert(identifier, index);
                index
            };
            let _ = signal_index;
        } else if line.starts_with("$enddefinitions") {
            in_definitions = false;
        } else if line.starts_with('$') {
            continue;
        } else if in_definitions {
            return Err(adapter_error(
                format,
                format_args!("unexpected VCD definition text at line {line_number}"),
            ));
        } else if let Some(tick) = line.strip_prefix('#') {
            current_tick = tick.parse::<u64>().map_err(|_| {
                adapter_error(
                    format,
                    format_args!("invalid timestamp at line {line_number}"),
                )
            })?;
        } else {
            let (identifier, value) = parse_vcd_change(line, format, line_number)?;
            let signal = identifiers.get(identifier).copied().ok_or_else(|| {
                adapter_error(
                    format,
                    format_args!("line {line_number} changes unknown identifier '{identifier}'"),
                )
            })?;
            if events.len() >= MAX_RESULT_VALUES {
                return Err(adapter_error(format, "VCD event-count limit exceeded"));
            }
            if value > (2_f64.powi(signals[signal].width as i32) - 1.0) {
                return Err(adapter_error(
                    format,
                    format_args!("line {line_number} value exceeds declared width"),
                ));
            }
            events.push(DigitalEvent {
                tick: current_tick,
                signal,
                value,
            });
        }
    }
    if !directive.is_empty() {
        return Err(adapter_error(
            format,
            "truncated VCD directive at end of file",
        ));
    }
    if in_definitions {
        return Err(adapter_error(format, "VCD is missing $enddefinitions"));
    }
    let mut parsed = digital_events_to_dataset(
        format,
        timescale.ok_or_else(|| adapter_error(format, "VCD is missing $timescale"))?,
        signals,
        events,
    )?;
    append_digital_aliases(format, &mut parsed, aliases)?;
    Ok(parsed)
}

fn parse_vcd_timescale(raw: &str, format: ResultImportFormat) -> Result<f64, String> {
    let compact = raw.split_whitespace().collect::<String>();
    let split = compact
        .find(|character: char| !character.is_ascii_digit())
        .ok_or_else(|| adapter_error(format, format_args!("invalid VCD timescale '{raw}'")))?;
    let magnitude = compact[..split]
        .parse::<u32>()
        .map_err(|_| adapter_error(format, format_args!("invalid VCD timescale '{raw}'")))?;
    if !matches!(magnitude, 1 | 10 | 100) {
        return Err(adapter_error(
            format,
            "VCD timescale magnitude must be 1, 10, or 100",
        ));
    }
    let scale = match compact[split..].to_ascii_lowercase().as_str() {
        "s" => 1.0,
        "ms" => 1e-3,
        "us" => 1e-6,
        "ns" => 1e-9,
        "ps" => 1e-12,
        "fs" => 1e-15,
        unit => {
            return Err(adapter_error(
                format,
                format_args!("unsupported VCD timescale unit '{unit}'"),
            ));
        }
    };
    Ok(f64::from(magnitude) * scale)
}

fn parse_vcd_change(
    line: &str,
    format: ResultImportFormat,
    line_number: usize,
) -> Result<(&str, f64), String> {
    let first = line.as_bytes()[0] as char;
    if matches!(first, '0' | '1' | 'x' | 'X' | 'z' | 'Z') {
        if matches!(first, 'x' | 'X' | 'z' | 'Z') {
            return Err(adapter_error(
                format,
                format_args!(
                    "line {line_number} contains X/Z state that cannot be losslessly mapped to an analog trace"
                ),
            ));
        }
        let identifier = line.get(1..).unwrap_or("").trim();
        if identifier.is_empty() {
            return Err(adapter_error(
                format,
                format_args!("missing identifier at line {line_number}"),
            ));
        }
        return Ok((identifier, if first == '1' { 1.0 } else { 0.0 }));
    }
    let mut fields = line.split_whitespace();
    let value = fields.next().unwrap_or("");
    let identifier = fields.next().unwrap_or("");
    if identifier.is_empty() || fields.next().is_some() {
        return Err(adapter_error(
            format,
            format_args!("malformed value change at line {line_number}"),
        ));
    }
    if let Some(bits) = value.strip_prefix(['b', 'B']) {
        return Ok((identifier, logic_bits_to_f64(bits.as_bytes(), format)?));
    }
    if let Some(real) = value.strip_prefix(['r', 'R']) {
        let real = real.parse::<f64>().map_err(|_| {
            adapter_error(
                format,
                format_args!("invalid real change at line {line_number}"),
            )
        })?;
        if !real.is_finite() {
            return Err(adapter_error(
                format,
                format_args!("non-finite real change at line {line_number}"),
            ));
        }
        return Ok((identifier, real));
    }
    Err(adapter_error(
        format,
        format_args!("unsupported value change at line {line_number}"),
    ))
}

fn logic_bits_to_f64(bits: &[u8], format: ResultImportFormat) -> Result<f64, String> {
    if bits.is_empty() || bits.len() > 53 {
        return Err(adapter_error(
            format,
            "digital vector is empty or wider than 53 exact bits",
        ));
    }
    let mut value = 0_u64;
    for bit in bits {
        value = value
            .checked_mul(2)
            .ok_or_else(|| adapter_error(format, "digital vector overflow"))?;
        match bit {
            b'0' => {}
            b'1' => value += 1,
            _ => {
                return Err(adapter_error(
                    format,
                    "digital vector contains X/Z/U/W/- state that cannot be losslessly mapped to an analog trace",
                ));
            }
        }
    }
    Ok(value as f64)
}

#[derive(Debug, Clone, Copy)]
struct FstHeaderPreflight {
    var_count: usize,
    max_handle: usize,
    value_change_sections: usize,
}

#[derive(Debug)]
struct FstGeometryPreflight {
    widths: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
struct FstDataPreflight {
    block_type: u8,
    section_start: usize,
    section_end: usize,
    section_length: usize,
    memory_required: usize,
}

/// Validate every allocation-relevant FST framing field before handing the
/// bytes to `fst-reader`. That crate trusts several sizes with `Vec` capacity
/// reservations and contains arithmetic assertions intended for trusted
/// files, so the public import boundary cannot delegate this job to it.
fn preflight_fst(bytes: &[u8], format: ResultImportFormat) -> Result<FstGeometryPreflight, String> {
    if bytes.len() as u64 > MAX_RESULT_DATASET_BYTES {
        return Err(adapter_error(format, "FST input exceeds the byte limit"));
    }
    let mut cursor = 0_usize;
    let mut block_count = 0_usize;
    let mut header = None;
    let mut geometry = None;
    let mut hierarchy_seen = false;
    let mut blackout_seen = false;
    let mut data_sections = Vec::new();
    let mut terminated = false;

    while cursor < bytes.len() {
        block_count = block_count
            .checked_add(1)
            .ok_or_else(|| adapter_error(format, "FST block-count accounting overflow"))?;
        if block_count > MAX_FST_TOP_LEVEL_BLOCKS {
            return Err(adapter_error(
                format,
                "FST top-level block-count limit exceeded",
            ));
        }
        let block_offset = cursor;
        let block_type = *bytes
            .get(cursor)
            .ok_or_else(|| adapter_error(format, "truncated FST block type"))?;
        cursor += 1;
        let section_start = cursor;
        let section_length_u64 = fst_be_u64(bytes, section_start, format, "section length")?;

        if block_type == 255 && section_length_u64 == 0 {
            cursor = section_start + 8;
            if cursor != bytes.len() {
                return Err(adapter_error(
                    format,
                    "FST contains trailing bytes after its end marker",
                ));
            }
            terminated = true;
            break;
        }
        if section_length_u64 < 8 {
            return Err(adapter_error(
                format,
                format_args!(
                    "FST block at byte {block_offset} declares a section shorter than its length field"
                ),
            ));
        }
        let section_length = fst_bounded_size(
            section_length_u64,
            format,
            format_args!("block at byte {block_offset} section"),
        )?;
        let section_end = section_start
            .checked_add(section_length)
            .ok_or_else(|| adapter_error(format, "FST section offset overflow"))?;
        if section_end > bytes.len() {
            return Err(adapter_error(
                format,
                format_args!("truncated FST block at byte {block_offset}"),
            ));
        }

        match block_type {
            0 => {
                if header.is_some() {
                    return Err(adapter_error(format, "FST repeats its header block"));
                }
                if section_length_u64 != FST_HEADER_SECTION_BYTES {
                    return Err(adapter_error(
                        format,
                        format_args!(
                            "FST header length is {section_length_u64}; expected {FST_HEADER_SECTION_BYTES}"
                        ),
                    ));
                }
                let body = section_start + 8;
                let start_time = fst_be_u64(bytes, body, format, "header start time")?;
                let end_time = fst_be_u64(bytes, body + 8, format, "header end time")?;
                if end_time < start_time {
                    return Err(adapter_error(
                        format,
                        "FST header end time precedes its start time",
                    ));
                }
                let endian_marker: [u8; 8] = bytes
                    .get(body + 16..body + 24)
                    .ok_or_else(|| {
                        adapter_error(format, "truncated FST floating-point endian marker")
                    })?
                    .try_into()
                    .expect("eight-byte slice");
                if endian_marker != std::f64::consts::E.to_le_bytes()
                    && endian_marker != std::f64::consts::E.to_be_bytes()
                {
                    return Err(adapter_error(
                        format,
                        "FST header has an invalid floating-point endian marker",
                    ));
                }
                let _scope_count = fst_count(
                    fst_be_u64(bytes, body + 32, format, "header scope count")?,
                    MAX_RESULT_COLUMNS,
                    format,
                    "scope",
                )?;
                let var_count = fst_count(
                    fst_be_u64(bytes, body + 40, format, "header variable count")?,
                    MAX_RESULT_COLUMNS - 1,
                    format,
                    "variable",
                )?;
                let max_handle = fst_count(
                    fst_be_u64(bytes, body + 48, format, "header signal count")?,
                    MAX_RESULT_COLUMNS - 1,
                    format,
                    "unique signal",
                )?;
                let value_change_sections = fst_count(
                    fst_be_u64(bytes, body + 56, format, "header data-block count")?,
                    MAX_FST_TOP_LEVEL_BLOCKS,
                    format,
                    "data block",
                )?;
                if var_count == 0
                    || max_handle == 0
                    || max_handle > var_count
                    || value_change_sections == 0
                {
                    return Err(adapter_error(
                        format,
                        "FST header declares inconsistent scope, variable, signal, or data-block counts",
                    ));
                }
                header = Some(FstHeaderPreflight {
                    var_count,
                    max_handle,
                    value_change_sections,
                });
            }
            1 | 5 | 8 => {
                if section_length < 32 {
                    return Err(adapter_error(format, "truncated FST value-change header"));
                }
                let memory_required = fst_bounded_size(
                    fst_be_u64(
                        bytes,
                        section_start + 24,
                        format,
                        "value-change allocation size",
                    )?,
                    format,
                    "value-change allocation",
                )?;
                data_sections.push(FstDataPreflight {
                    block_type,
                    section_start,
                    section_end,
                    section_length,
                    memory_required,
                });
            }
            2 => {
                if blackout_seen {
                    return Err(adapter_error(format, "FST repeats its blackout block"));
                }
                preflight_fst_blackout(bytes, section_start, section_end, format)?;
                blackout_seen = true;
            }
            3 => {
                if geometry.is_some() {
                    return Err(adapter_error(format, "FST repeats its geometry block"));
                }
                geometry = Some(preflight_fst_geometry(
                    bytes,
                    section_start,
                    section_end,
                    section_length,
                    format,
                )?);
            }
            4 | 6 | 7 => {
                if hierarchy_seen {
                    return Err(adapter_error(format, "FST repeats its hierarchy block"));
                }
                preflight_fst_hierarchy(
                    bytes,
                    block_type,
                    section_start,
                    section_end,
                    section_length,
                    format,
                )?;
                hierarchy_seen = true;
            }
            254 => {
                if section_length < 16 {
                    return Err(adapter_error(
                        format,
                        "truncated FST whole-file gzip wrapper",
                    ));
                }
                let expanded = fst_bounded_size(
                    fst_be_u64(
                        bytes,
                        section_start + 8,
                        format,
                        "gzip wrapper expanded size",
                    )?,
                    format,
                    "gzip wrapper expanded allocation",
                )?;
                return Err(adapter_error(
                    format,
                    format_args!(
                        "whole-file gzip-wrapped FST ({expanded} declared expanded bytes) is rejected because nested framing cannot be preflighted before fst-reader decompresses it"
                    ),
                ));
            }
            255 => {}
            other => {
                return Err(adapter_error(
                    format,
                    format_args!("unknown FST top-level block type {other}"),
                ));
            }
        }
        cursor = section_end;
    }

    if !terminated && cursor != bytes.len() {
        return Err(adapter_error(
            format,
            "FST framing did not end at the input boundary",
        ));
    }
    let header = header.ok_or_else(|| adapter_error(format, "FST header block is missing"))?;
    let geometry =
        geometry.ok_or_else(|| adapter_error(format, "FST geometry block is missing"))?;
    if !hierarchy_seen {
        return Err(adapter_error(format, "FST hierarchy block is missing"));
    }
    if geometry.widths.len() != header.max_handle {
        return Err(adapter_error(
            format,
            "FST geometry signal count disagrees with its header",
        ));
    }
    if data_sections.len() != header.value_change_sections {
        return Err(adapter_error(
            format,
            "FST data-block count disagrees with its header",
        ));
    }
    if header.var_count < geometry.widths.len() {
        return Err(adapter_error(
            format,
            "FST variable count is smaller than its unique signal count",
        ));
    }
    for section in data_sections {
        preflight_fst_data_section(bytes, section, &geometry.widths, format)?;
    }
    Ok(geometry)
}

fn fst_be_u64(
    bytes: &[u8],
    offset: usize,
    format: ResultImportFormat,
    field: impl std::fmt::Display,
) -> Result<u64, String> {
    let end = offset
        .checked_add(8)
        .ok_or_else(|| adapter_error(format, format_args!("{field} offset overflow")))?;
    let raw = bytes
        .get(offset..end)
        .ok_or_else(|| adapter_error(format, format_args!("truncated FST {field}")))?;
    Ok(u64::from_be_bytes(
        raw.try_into().expect("eight-byte slice"),
    ))
}

fn fst_bounded_size(
    value: u64,
    format: ResultImportFormat,
    field: impl std::fmt::Display,
) -> Result<usize, String> {
    if value > MAX_RESULT_DATASET_BYTES {
        return Err(adapter_error(
            format,
            format_args!(
                "FST {field} declares {value} bytes; the limit is {MAX_RESULT_DATASET_BYTES}"
            ),
        ));
    }
    usize::try_from(value)
        .map_err(|_| adapter_error(format, format_args!("FST {field} does not fit this target")))
}

fn fst_count(
    value: u64,
    maximum: usize,
    format: ResultImportFormat,
    field: &str,
) -> Result<usize, String> {
    let value = usize::try_from(value)
        .map_err(|_| adapter_error(format, format_args!("FST {field} count overflow")))?;
    if value > maximum {
        Err(adapter_error(
            format,
            format_args!("FST {field} count {value} exceeds the limit {maximum}"),
        ))
    } else {
        Ok(value)
    }
}

fn fst_uleb(
    bytes: &[u8],
    cursor: &mut usize,
    limit: usize,
    maximum_bits: u32,
    format: ResultImportFormat,
    field: &str,
) -> Result<(u64, usize), String> {
    let start = *cursor;
    let max_bytes = maximum_bits.div_ceil(7) as usize;
    let mut value = 0_u128;
    for index in 0..max_bytes {
        if *cursor >= limit {
            return Err(adapter_error(format, format_args!("truncated FST {field}")));
        }
        let byte = bytes[*cursor];
        *cursor += 1;
        value |= u128::from(byte & 0x7f) << (7 * index);
        if byte & 0x80 == 0 {
            let maximum = if maximum_bits == 64 {
                u128::from(u64::MAX)
            } else {
                (1_u128 << maximum_bits) - 1
            };
            if value > maximum {
                return Err(adapter_error(format, format_args!("FST {field} overflow")));
            }
            return Ok((value as u64, *cursor - start));
        }
    }
    Err(adapter_error(
        format,
        format_args!("FST {field} uses an overlong integer"),
    ))
}

fn fst_sleb_i64(
    bytes: &[u8],
    cursor: &mut usize,
    limit: usize,
    format: ResultImportFormat,
    field: &str,
) -> Result<i64, String> {
    let mut value = 0_i128;
    for index in 0..10_usize {
        if *cursor >= limit {
            return Err(adapter_error(format, format_args!("truncated FST {field}")));
        }
        let byte = bytes[*cursor];
        *cursor += 1;
        let shift = 7 * index;
        value |= i128::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            if byte & 0x40 != 0 {
                value |= (!0_i128) << (shift + 7);
            }
            return i64::try_from(value)
                .map_err(|_| adapter_error(format, format_args!("FST {field} overflow")));
        }
    }
    Err(adapter_error(
        format,
        format_args!("FST {field} uses an overlong integer"),
    ))
}

fn preflight_fst_blackout(
    bytes: &[u8],
    section_start: usize,
    section_end: usize,
    format: ResultImportFormat,
) -> Result<(), String> {
    let mut cursor = section_start + 8;
    let (count, _) = fst_uleb(
        bytes,
        &mut cursor,
        section_end,
        32,
        format,
        "blackout count",
    )?;
    let count = fst_count(count, MAX_RESULT_ROWS, format, "blackout")?;
    let mut time = 0_u64;
    for _ in 0..count {
        if cursor >= section_end {
            return Err(adapter_error(format, "truncated FST blackout entry"));
        }
        cursor += 1;
        let (delta, _) = fst_uleb(
            bytes,
            &mut cursor,
            section_end,
            64,
            format,
            "blackout time delta",
        )?;
        time = time
            .checked_add(delta)
            .ok_or_else(|| adapter_error(format, "FST blackout time overflow"))?;
    }
    if cursor != section_end {
        return Err(adapter_error(
            format,
            "FST blackout section has inconsistent framing",
        ));
    }
    Ok(())
}

fn preflight_fst_geometry(
    bytes: &[u8],
    section_start: usize,
    section_end: usize,
    section_length: usize,
    format: ResultImportFormat,
) -> Result<FstGeometryPreflight, String> {
    if section_length < 24 {
        return Err(adapter_error(format, "truncated FST geometry block"));
    }
    let uncompressed = fst_bounded_size(
        fst_be_u64(bytes, section_start + 8, format, "geometry expanded size")?,
        format,
        "geometry expanded allocation",
    )?;
    let handle_count = fst_count(
        fst_be_u64(bytes, section_start + 16, format, "geometry signal count")?,
        MAX_RESULT_COLUMNS - 1,
        format,
        "geometry signal",
    )?;
    if handle_count == 0 {
        return Err(adapter_error(format, "FST geometry declares no signals"));
    }
    let compressed = section_length - 24;
    if compressed > MAX_RESULT_DATASET_BYTES as usize {
        return Err(adapter_error(
            format,
            "FST geometry compressed-size limit exceeded",
        ));
    }
    // fst-reader's geometry inflater exposes only the resulting SignalInfo
    // vector, after which a malicious encoded width could cause a much larger
    // frame allocation. Without an independently preflightable payload, the
    // safe boundary is to accept the format's permitted uncompressed geometry
    // representation and fail closed on compressed geometry.
    if uncompressed != compressed {
        return Err(adapter_error(
            format,
            "compressed FST geometry is rejected because signal widths cannot be bounded before decompression",
        ));
    }
    let mut cursor = section_start + 24;
    let mut widths = Vec::with_capacity(handle_count);
    for _ in 0..handle_count {
        let (encoded, _) = fst_uleb(
            bytes,
            &mut cursor,
            section_end,
            32,
            format,
            "geometry signal width",
        )?;
        let width = if encoded == 0 {
            8 // FST's geometry marker for an IEEE-754 real signal.
        } else if encoded == u64::from(u32::MAX) {
            return Err(adapter_error(
                format,
                "FST variable-length signal records are not supported",
            ));
        } else {
            let width = usize::try_from(encoded)
                .map_err(|_| adapter_error(format, "FST signal width overflow"))?;
            if width > 53 {
                return Err(adapter_error(
                    format,
                    format_args!(
                        "FST digital signal width {width} exceeds the 53-bit lossless import limit"
                    ),
                ));
            }
            width
        };
        widths.push(width);
    }
    if cursor != section_end {
        return Err(adapter_error(
            format,
            "FST geometry payload has inconsistent signal-count framing",
        ));
    }
    Ok(FstGeometryPreflight { widths })
}

fn preflight_fst_hierarchy(
    bytes: &[u8],
    block_type: u8,
    section_start: usize,
    section_end: usize,
    section_length: usize,
    format: ResultImportFormat,
) -> Result<(), String> {
    if section_length < 16 {
        return Err(adapter_error(format, "truncated FST hierarchy block"));
    }
    let expanded = fst_bounded_size(
        fst_be_u64(bytes, section_start + 8, format, "hierarchy expanded size")?,
        format,
        "hierarchy expanded allocation",
    )?;
    let compressed = section_length - 16;
    if expanded == 0 || compressed == 0 {
        return Err(adapter_error(format, "FST hierarchy block is empty"));
    }
    if compressed > MAX_RESULT_DATASET_BYTES as usize {
        return Err(adapter_error(
            format,
            "FST hierarchy compressed-size limit exceeded",
        ));
    }
    if block_type == 4 {
        let payload = bytes
            .get(section_start + 16..section_end)
            .ok_or_else(|| adapter_error(format, "truncated FST gzip hierarchy payload"))?;
        if payload.len() < 10 || payload[0..2] != [0x1f, 0x8b] || payload[2] != 8 || payload[3] != 0
        {
            return Err(adapter_error(
                format,
                "FST gzip hierarchy has an unsupported or truncated header",
            ));
        }
    }
    if block_type == 7 {
        let mut cursor = section_start + 16;
        let (first_stage, encoded_bytes) = fst_uleb(
            bytes,
            &mut cursor,
            section_end,
            64,
            format,
            "LZ4-duo first-stage size",
        )?;
        let first_stage = fst_bounded_size(first_stage, format, "LZ4-duo first-stage allocation")?;
        if first_stage == 0 || encoded_bytes > compressed || cursor >= section_end {
            return Err(adapter_error(
                format,
                "FST LZ4-duo hierarchy has inconsistent compressed framing",
            ));
        }
    }
    Ok(())
}

fn preflight_fst_data_section(
    bytes: &[u8],
    section: FstDataPreflight,
    widths: &[usize],
    format: ResultImportFormat,
) -> Result<(), String> {
    let mut cursor = section.section_start + 32;
    let (frame_expanded, _) = fst_uleb(
        bytes,
        &mut cursor,
        section.section_end,
        64,
        format,
        "initial-frame expanded size",
    )?;
    let frame_expanded =
        fst_bounded_size(frame_expanded, format, "initial-frame expanded allocation")?;
    let (frame_compressed, _) = fst_uleb(
        bytes,
        &mut cursor,
        section.section_end,
        64,
        format,
        "initial-frame compressed size",
    )?;
    let frame_compressed = fst_bounded_size(
        frame_compressed,
        format,
        "initial-frame compressed allocation",
    )?;
    let (frame_handles, _) = fst_uleb(
        bytes,
        &mut cursor,
        section.section_end,
        64,
        format,
        "initial-frame signal count",
    )?;
    let frame_handles = fst_count(
        frame_handles,
        MAX_RESULT_COLUMNS - 1,
        format,
        "initial-frame signal",
    )?;
    if frame_handles != widths.len() {
        return Err(adapter_error(
            format,
            "FST initial-frame signal count disagrees with geometry",
        ));
    }
    let minimum_frame_bytes = widths.iter().try_fold(0_usize, |total, width| {
        total
            .checked_add(*width)
            .ok_or_else(|| adapter_error(format, "FST initial-frame width accounting overflow"))
    })?;
    if minimum_frame_bytes != frame_expanded {
        return Err(adapter_error(
            format,
            "FST initial-frame size disagrees with its declared signal widths",
        ));
    }
    if frame_compressed == 0 {
        return Err(adapter_error(format, "FST initial frame is empty"));
    }
    if frame_compressed != frame_expanded && bytes.get(cursor).copied() != Some(0x78) {
        return Err(adapter_error(
            format,
            "FST compressed initial frame does not have zlib framing",
        ));
    }
    cursor = cursor
        .checked_add(frame_compressed)
        .ok_or_else(|| adapter_error(format, "FST initial-frame offset overflow"))?;
    if cursor > section.section_end {
        return Err(adapter_error(format, "truncated FST initial frame"));
    }

    let (data_handles, _) = fst_uleb(
        bytes,
        &mut cursor,
        section.section_end,
        64,
        format,
        "value-change signal count",
    )?;
    let data_handles = fst_count(
        data_handles,
        MAX_RESULT_COLUMNS - 1,
        format,
        "value-change signal",
    )?;
    if data_handles != widths.len() {
        return Err(adapter_error(
            format,
            "FST value-change signal count disagrees with geometry",
        ));
    }
    let value_change_start = cursor;
    let pack_type = *bytes
        .get(cursor)
        .ok_or_else(|| adapter_error(format, "truncated FST value-change packing type"))?;
    cursor += 1;
    let value_payload_start = cursor;

    if section.section_length < 24 {
        return Err(adapter_error(format, "truncated FST time-table metadata"));
    }
    let time_meta_start = section.section_end - 24;
    let time_expanded = fst_bounded_size(
        fst_be_u64(bytes, time_meta_start, format, "time-table expanded size")?,
        format,
        "time-table expanded allocation",
    )?;
    let time_compressed = fst_bounded_size(
        fst_be_u64(
            bytes,
            time_meta_start + 8,
            format,
            "time-table compressed size",
        )?,
        format,
        "time-table compressed allocation",
    )?;
    let time_count = fst_count(
        fst_be_u64(bytes, time_meta_start + 16, format, "time-table item count")?,
        MAX_RESULT_ROWS,
        format,
        "time-table item",
    )?;
    if time_count > time_expanded {
        return Err(adapter_error(
            format,
            "FST time table declares more items than its expanded byte stream can contain",
        ));
    }
    let time_data_start = time_meta_start
        .checked_sub(time_compressed)
        .ok_or_else(|| adapter_error(format, "FST time-table offset underflow"))?;
    if (time_expanded == 0) != (time_compressed == 0) {
        return Err(adapter_error(
            format,
            "FST time table has inconsistent empty framing",
        ));
    }
    if time_compressed != 0
        && time_compressed != time_expanded
        && bytes.get(time_data_start).copied() != Some(0x78)
    {
        return Err(adapter_error(
            format,
            "FST compressed time table does not have zlib framing",
        ));
    }
    if time_compressed == time_expanded {
        let mut time_cursor = time_data_start;
        let mut time = 0_u64;
        for _ in 0..time_count {
            let (delta, _) = fst_uleb(
                bytes,
                &mut time_cursor,
                time_meta_start,
                64,
                format,
                "time-table delta",
            )?;
            time = time
                .checked_add(delta)
                .ok_or_else(|| adapter_error(format, "FST time-table value overflow"))?;
        }
        if time_cursor != time_meta_start {
            return Err(adapter_error(
                format,
                "FST uncompressed time table has inconsistent item-count framing",
            ));
        }
    }
    let chain_length_offset = time_data_start
        .checked_sub(8)
        .ok_or_else(|| adapter_error(format, "FST offset-table length underflow"))?;
    if chain_length_offset < value_payload_start {
        return Err(adapter_error(
            format,
            "FST time table overlaps its value-change payload",
        ));
    }
    let offset_table_bytes = fst_bounded_size(
        fst_be_u64(
            bytes,
            chain_length_offset,
            format,
            "offset-table compressed size",
        )?,
        format,
        "offset-table allocation",
    )?;
    let offset_table_start = chain_length_offset
        .checked_sub(offset_table_bytes)
        .ok_or_else(|| adapter_error(format, "FST offset-table start underflow"))?;
    if offset_table_start < value_payload_start {
        return Err(adapter_error(
            format,
            "FST offset table overlaps its value-change header",
        ));
    }
    let last_payload_offset = offset_table_start
        .checked_sub(value_change_start)
        .ok_or_else(|| adapter_error(format, "FST value-change offset underflow"))?;
    if last_payload_offset > u32::MAX as usize {
        return Err(adapter_error(format, "FST value-change offsets exceed u32"));
    }
    let ranges = preflight_fst_offset_table(
        bytes,
        section.block_type,
        offset_table_start,
        chain_length_offset,
        widths.len(),
        last_payload_offset,
        format,
    )?;
    let mut actual_memory = 0_usize;
    for (offset, length) in ranges {
        let signal_start = value_change_start
            .checked_add(offset)
            .ok_or_else(|| adapter_error(format, "FST signal payload offset overflow"))?;
        let signal_end = signal_start
            .checked_add(length)
            .ok_or_else(|| adapter_error(format, "FST signal payload length overflow"))?;
        if signal_start < value_payload_start || signal_end > offset_table_start {
            return Err(adapter_error(
                format,
                "FST signal payload points outside the value-change region",
            ));
        }
        let mut signal_cursor = signal_start;
        let (declared_expanded, marker_bytes) = fst_uleb(
            bytes,
            &mut signal_cursor,
            signal_end,
            32,
            format,
            "packed signal expanded size",
        )?;
        let compressed_bytes = match pack_type {
            b'4' | b'F' => length
                .checked_sub(marker_bytes)
                .ok_or_else(|| adapter_error(format, "FST packed signal length underflow"))?,
            _ => length,
        };
        if compressed_bytes > MAX_RESULT_DATASET_BYTES as usize {
            return Err(adapter_error(
                format,
                "FST packed signal compressed-size limit exceeded",
            ));
        }
        let expanded_bytes = if declared_expanded == 0 {
            length
                .checked_sub(marker_bytes)
                .ok_or_else(|| adapter_error(format, "FST direct signal length underflow"))?
        } else {
            let expanded = fst_bounded_size(
                declared_expanded,
                format,
                match pack_type {
                    b'4' => "LZ4 signal expanded allocation",
                    b'F' => "FastLZ signal expanded allocation",
                    _ => "zlib signal expanded allocation",
                },
            )?;
            if pack_type != b'4'
                && pack_type != b'F'
                && bytes.get(signal_cursor).copied() != Some(0x78)
            {
                return Err(adapter_error(
                    format,
                    "FST packed zlib signal does not have zlib framing",
                ));
            }
            expanded
        };
        actual_memory = actual_memory
            .checked_add(expanded_bytes)
            .ok_or_else(|| adapter_error(format, "FST signal allocation accounting overflow"))?;
        if actual_memory > MAX_RESULT_DATASET_BYTES as usize {
            return Err(adapter_error(
                format,
                "FST aggregate expanded signal allocation exceeds the byte limit",
            ));
        }
    }
    if actual_memory > section.memory_required {
        return Err(adapter_error(
            format,
            "FST value-change allocation is larger than its section memory declaration",
        ));
    }
    Ok(())
}

fn preflight_fst_offset_table(
    bytes: &[u8],
    block_type: u8,
    table_start: usize,
    table_end: usize,
    signal_count: usize,
    payload_end_offset: usize,
    format: ResultImportFormat,
) -> Result<Vec<(usize, usize)>, String> {
    let mut cursor = table_start;
    let mut signal_index = 0_usize;
    let mut offsets = Vec::with_capacity(signal_count);
    let mut direct_signals = Vec::with_capacity(signal_count);
    let mut current_offset = 0_usize;
    let mut previous_alias = None;

    while cursor < table_end {
        if block_type == 8 {
            let kind = bytes[cursor];
            if kind & 1 == 1 {
                let encoded = fst_sleb_i64(
                    bytes,
                    &mut cursor,
                    table_end,
                    format,
                    "dynamic-alias offset",
                )?;
                let value = encoded >> 1;
                match value.cmp(&0) {
                    std::cmp::Ordering::Greater => {
                        let delta = usize::try_from(value).map_err(|_| {
                            adapter_error(format, "FST dynamic-alias offset overflow")
                        })?;
                        current_offset = current_offset.checked_add(delta).ok_or_else(|| {
                            adapter_error(format, "FST dynamic-alias offset overflow")
                        })?;
                        offsets.push(current_offset);
                        direct_signals.push(true);
                        signal_index = signal_index.checked_add(1).ok_or_else(|| {
                            adapter_error(format, "FST offset-table count overflow")
                        })?;
                    }
                    std::cmp::Ordering::Less => {
                        let alias = value
                            .checked_neg()
                            .and_then(|value| value.checked_sub(1))
                            .and_then(|value| usize::try_from(value).ok())
                            .ok_or_else(|| {
                                adapter_error(format, "FST dynamic alias index overflow")
                            })?;
                        if alias >= signal_index || !direct_signals[alias] {
                            return Err(adapter_error(
                                format,
                                "FST dynamic alias does not refer to an earlier direct signal",
                            ));
                        }
                        previous_alias = Some(alias);
                        direct_signals.push(false);
                        signal_index += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        if previous_alias.is_none() {
                            return Err(adapter_error(
                                format,
                                "FST repeated dynamic alias has no preceding alias",
                            ));
                        }
                        direct_signals.push(false);
                        signal_index += 1;
                    }
                }
            } else {
                let (encoded, _) = fst_uleb(
                    bytes,
                    &mut cursor,
                    table_end,
                    32,
                    format,
                    "dynamic-alias empty-signal run",
                )?;
                let empty = usize::try_from(encoded >> 1)
                    .map_err(|_| adapter_error(format, "FST empty-signal count overflow"))?;
                if empty == 0 {
                    return Err(adapter_error(
                        format,
                        "FST offset table contains an empty zero-length run",
                    ));
                }
                let next_signal_index = signal_index
                    .checked_add(empty)
                    .ok_or_else(|| adapter_error(format, "FST offset-table count overflow"))?;
                if next_signal_index > signal_count {
                    return Err(adapter_error(
                        format,
                        "FST offset table declares more signals than geometry",
                    ));
                }
                direct_signals.resize(next_signal_index, false);
                signal_index = next_signal_index;
            }
        } else {
            let (raw, _) = fst_uleb(
                bytes,
                &mut cursor,
                table_end,
                32,
                format,
                "offset-table entry",
            )?;
            let raw = u32::try_from(raw)
                .map_err(|_| adapter_error(format, "FST offset-table entry overflow"))?;
            if raw == 0 {
                let (alias, _) =
                    fst_uleb(bytes, &mut cursor, table_end, 32, format, "signal alias")?;
                let alias = usize::try_from(alias)
                    .ok()
                    .and_then(|alias| alias.checked_sub(1))
                    .ok_or_else(|| adapter_error(format, "FST signal alias index underflow"))?;
                if alias >= signal_index || !direct_signals[alias] {
                    return Err(adapter_error(
                        format,
                        "FST signal alias does not refer to an earlier direct signal",
                    ));
                }
                direct_signals.push(false);
                signal_index += 1;
            } else if raw & 1 == 1 {
                let delta = (raw >> 1) as usize;
                if delta == 0 {
                    return Err(adapter_error(format, "FST signal offset does not advance"));
                }
                current_offset = current_offset
                    .checked_add(delta)
                    .ok_or_else(|| adapter_error(format, "FST signal offset overflow"))?;
                offsets.push(current_offset);
                direct_signals.push(true);
                signal_index += 1;
            } else {
                let empty = (raw >> 1) as usize;
                if empty == 0 {
                    return Err(adapter_error(
                        format,
                        "FST offset table contains an empty zero-length run",
                    ));
                }
                let next_signal_index = signal_index
                    .checked_add(empty)
                    .ok_or_else(|| adapter_error(format, "FST offset-table count overflow"))?;
                if next_signal_index > signal_count {
                    return Err(adapter_error(
                        format,
                        "FST offset table declares more signals than geometry",
                    ));
                }
                direct_signals.resize(next_signal_index, false);
                signal_index = next_signal_index;
            }
        }
        if signal_index > signal_count {
            return Err(adapter_error(
                format,
                "FST offset table declares more signals than geometry",
            ));
        }
    }
    if signal_index != signal_count {
        return Err(adapter_error(
            format,
            "FST offset-table signal count disagrees with geometry",
        ));
    }
    if offsets
        .last()
        .is_some_and(|offset| *offset >= payload_end_offset)
    {
        return Err(adapter_error(
            format,
            "FST signal offset points outside the value-change payload",
        ));
    }
    let mut ranges = Vec::with_capacity(offsets.len());
    for (index, offset) in offsets.iter().copied().enumerate() {
        let next = offsets
            .get(index + 1)
            .copied()
            .unwrap_or(payload_end_offset);
        let length = next
            .checked_sub(offset)
            .ok_or_else(|| adapter_error(format, "FST signal offsets are not ordered"))?;
        if length == 0 || length > u32::MAX as usize {
            return Err(adapter_error(
                format,
                "FST signal payload length is zero or exceeds u32",
            ));
        }
        ranges.push((offset, length));
    }
    Ok(ranges)
}

pub(super) fn parse_fst(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let geometry = preflight_fst(bytes, format)?;
    let cursor = Cursor::new(bytes);
    let mut reader = fst_reader::FstReader::open(BufReader::new(cursor))
        .map_err(|error| adapter_error(format, format_args!("invalid FST container: {error}")))?;
    let header = reader.get_header();
    if header.var_count as usize > MAX_RESULT_COLUMNS - 1 {
        return Err(adapter_error(format, "FST signal-count limit exceeded"));
    }
    if !(-30..=30).contains(&header.timescale_exponent) {
        return Err(adapter_error(
            format,
            "FST timescale exponent is outside the supported finite range",
        ));
    }
    let mut scopes = Vec::new();
    let mut scope_identity_bytes = 0_usize;
    let mut by_handle: BTreeMap<usize, Vec<DigitalSignal>> = BTreeMap::new();
    let mut hierarchy_entries = 0_usize;
    let mut hierarchy_signals = 0_usize;
    let mut hierarchy_error = None;
    let maximum_handle = usize::try_from(header.max_handle)
        .map_err(|_| adapter_error(format, "FST header signal count does not fit this target"))?;
    reader
        .read_hierarchy(|entry| {
            if hierarchy_error.is_some() {
                return;
            }
            hierarchy_entries += 1;
            if hierarchy_entries > MAX_RESULT_VALUES {
                hierarchy_error = Some("FST hierarchy-entry limit exceeded".to_owned());
                return;
            }
            match entry {
                fst_reader::FstHierarchyEntry::Scope { name, .. } => {
                    let separator = usize::from(!scopes.is_empty());
                    let Some(next_identity_bytes) = scope_identity_bytes
                        .checked_add(separator)
                        .and_then(|length| length.checked_add(name.len()))
                    else {
                        hierarchy_error = Some("FST hierarchy identity length overflow".to_owned());
                        return;
                    };
                    if name.is_empty() || name.len() > MAX_SIGNAL_NAME_BYTES {
                        hierarchy_error = Some(format!(
                            "FST scope identity exceeds {MAX_SIGNAL_NAME_BYTES} bytes or is empty"
                        ));
                    } else if next_identity_bytes > MAX_SIGNAL_NAME_BYTES {
                        hierarchy_error = Some(format!(
                            "FST scope path exceeds {MAX_SIGNAL_NAME_BYTES} bytes"
                        ));
                    } else if scopes.len() >= MAX_RESULT_COLUMNS {
                        hierarchy_error = Some("FST hierarchy-depth limit exceeded".to_owned());
                    } else {
                        scope_identity_bytes = next_identity_bytes;
                        scopes.push(name);
                    }
                }
                fst_reader::FstHierarchyEntry::UpScope => {
                    if let Some(name) = scopes.pop() {
                        scope_identity_bytes = if scopes.is_empty() {
                            0
                        } else {
                            scope_identity_bytes.saturating_sub(name.len() + 1)
                        };
                    } else {
                        hierarchy_error =
                            Some("FST hierarchy closes a scope that was not open".to_owned());
                    }
                }
                fst_reader::FstHierarchyEntry::Var {
                    name,
                    length,
                    handle,
                    ..
                } => {
                    hierarchy_signals += 1;
                    if hierarchy_signals > MAX_RESULT_COLUMNS - 1 {
                        hierarchy_error =
                            Some("FST hierarchy signal/alias limit exceeded".to_owned());
                        return;
                    }
                    let handle = handle.get_index();
                    if handle >= maximum_handle {
                        hierarchy_error = Some(
                            "FST hierarchy references an out-of-range signal handle".to_owned(),
                        );
                        return;
                    }
                    if geometry.widths[handle] != length as usize {
                        hierarchy_error = Some(format!(
                            "FST hierarchy width {length} for '{name}' disagrees with geometry width {}",
                            geometry.widths[handle]
                        ));
                        return;
                    }
                    if length == 0 || length > 53 {
                        hierarchy_error = Some(format!(
                            "FST hierarchy signal '{name}' has unsupported width {length}"
                        ));
                        return;
                    }
                    if name.is_empty() || name.len() > MAX_SIGNAL_NAME_BYTES {
                        hierarchy_error = Some(format!(
                            "FST signal identity exceeds {MAX_SIGNAL_NAME_BYTES} bytes or is empty"
                        ));
                        return;
                    }
                    let mut full_name = scopes.join(".");
                    if !full_name.is_empty() {
                        full_name.push('.');
                    }
                    full_name.push_str(&name);
                    if full_name.len() > MAX_SIGNAL_NAME_BYTES {
                        hierarchy_error = Some(format!(
                            "FST hierarchy signal identity exceeds {MAX_SIGNAL_NAME_BYTES} bytes"
                        ));
                        return;
                    }
                    by_handle.entry(handle).or_default().push(DigitalSignal {
                        name: full_name,
                        width: length as usize,
                    });
                }
                _ => {}
            }
        })
        .map_err(|error| {
            adapter_error(
                format,
                format_args!("could not read FST hierarchy: {error}"),
            )
        })?;
    if let Some(error) = hierarchy_error {
        return Err(adapter_error(format, error));
    }
    if !scopes.is_empty() {
        return Err(adapter_error(format, "FST hierarchy has unclosed scopes"));
    }
    if by_handle.len() != maximum_handle {
        return Err(adapter_error(
            format,
            "FST hierarchy unique-signal count disagrees with geometry",
        ));
    }
    let total_signals = by_handle.values().map(Vec::len).sum::<usize>();
    if total_signals == 0 || total_signals > MAX_RESULT_COLUMNS - 1 {
        return Err(adapter_error(
            format,
            "FST contains no signals or too many aliases",
        ));
    }
    let handles = by_handle
        .keys()
        .map(|index| fst_reader::FstSignalHandle::from_index(*index))
        .collect::<Vec<_>>();
    let handle_order = by_handle.keys().copied().collect::<Vec<_>>();
    let handle_to_signal = handle_order
        .iter()
        .enumerate()
        .map(|(signal, handle)| (*handle, signal))
        .collect::<HashMap<_, _>>();
    let canonical_signals = handle_order
        .iter()
        .map(|handle| DigitalSignal {
            name: by_handle[handle][0].name.clone(),
            width: by_handle[handle][0].width,
        })
        .collect::<Vec<_>>();
    let mut events = Vec::new();
    let callback_result = reader.read_signals(
        &fst_reader::FstFilter::filter_signals(handles),
        |tick, handle, value| {
            if events.len() >= MAX_RESULT_VALUES {
                return Err("FST event-count limit exceeded".to_owned());
            }
            let signal = *handle_to_signal
                .get(&handle.get_index())
                .ok_or_else(|| "FST returned an undeclared signal handle".to_owned())?;
            let value = match value {
                fst_reader::FstSignalValue::String(bits) => logic_bits_to_f64(bits, format)?,
                fst_reader::FstSignalValue::Real(value) if value.is_finite() => value,
                fst_reader::FstSignalValue::Real(_) => {
                    return Err("FST contains a non-finite real value".to_owned());
                }
            };
            events.push(DigitalEvent {
                tick,
                signal,
                value,
            });
            Ok(())
        },
    );
    callback_result.map_err(|error| {
        adapter_error(format, format_args!("could not read FST events: {error:?}"))
    })?;
    let timescale = 10_f64.powi(header.timescale_exponent as i32);
    let parsed = digital_events_to_dataset(format, timescale, canonical_signals, events)?;

    // Materialize aliases after canonical decoding. They retain independent
    // identities while sharing exact samples and axes.
    let mut aliases = Vec::new();
    for (canonical_index, handle) in handle_order.iter().enumerate() {
        for alias in by_handle[handle].iter().skip(1) {
            aliases.push((canonical_index, alias.name.clone()));
        }
    }
    let mut parsed = parsed;
    append_digital_aliases(format, &mut parsed, aliases)?;
    Ok(parsed)
}

pub(super) fn looks_like_fst(bytes: &[u8]) -> bool {
    let mut cursor = Cursor::new(bytes);
    fst_reader::is_fst_file(&mut cursor)
}

fn digital_events_to_dataset(
    format: ResultImportFormat,
    timescale_seconds: f64,
    signals: Vec<DigitalSignal>,
    mut events: Vec<DigitalEvent>,
) -> Result<ParsedResultDataset, String> {
    if !timescale_seconds.is_finite() || timescale_seconds <= 0.0 {
        return Err(adapter_error(
            format,
            "digital timescale is not finite and positive",
        ));
    }
    if signals.is_empty() {
        return Err(adapter_error(format, "digital source declares no signals"));
    }
    for signal in &signals {
        validate_name(format, "signal", &signal.name)?;
        if signal.width == 0 || signal.width > 53 {
            return Err(adapter_error(
                format,
                format_args!(
                    "signal '{}' width {} cannot be represented exactly",
                    signal.name, signal.width
                ),
            ));
        }
    }
    events.sort_by_key(|event| event.tick);
    let ticks = events
        .iter()
        .map(|event| event.tick)
        .collect::<BTreeSet<_>>();
    if ticks.len() < MIN_RESULT_ROWS || ticks.len() > MAX_RESULT_ROWS {
        return Err(adapter_error(
            format,
            format_args!(
                "digital trace has {} distinct event times; expected {MIN_RESULT_ROWS}..={MAX_RESULT_ROWS}",
                ticks.len()
            ),
        ));
    }
    let mut states = vec![None; signals.len()];
    let mut values = vec![Vec::with_capacity(ticks.len()); signals.len()];
    let mut coordinate = Vec::with_capacity(ticks.len());
    let mut events = events.into_iter().peekable();
    for tick in ticks {
        if tick > MAX_EXACT_F64_INTEGER {
            return Err(adapter_error(
                format,
                format_args!("digital timestamp tick {tick} cannot be represented exactly as f64"),
            ));
        }
        while events.peek().is_some_and(|event| event.tick == tick) {
            let event = events.next().expect("peeked event exists");
            if event.signal >= states.len() {
                return Err(adapter_error(
                    format,
                    "digital event references an unknown signal",
                ));
            }
            states[event.signal] = Some(event.value);
        }
        if states.iter().any(Option::is_none) {
            return Err(adapter_error(
                format,
                format_args!(
                    "not every digital signal has a known 0/1/vector value at initial tick {tick}"
                ),
            ));
        }
        let time = (tick as f64) * timescale_seconds;
        if !time.is_finite() {
            return Err(adapter_error(
                format,
                "scaled digital timestamp is not finite",
            ));
        }
        coordinate.push(time);
        for (column, state) in values.iter_mut().zip(&states) {
            column.push(state.expect("all states checked"));
        }
    }
    let signals = signals
        .into_iter()
        .zip(values)
        .map(|(signal, real)| ImportedSignal {
            name: signal.name,
            real,
            imag: None,
            unit: None,
        })
        .collect();
    finish_dataset(format, AnalysisType::Transient, "time", coordinate, signals)
}

fn append_digital_aliases(
    format: ResultImportFormat,
    parsed: &mut ParsedResultDataset,
    aliases: Vec<(usize, String)>,
) -> Result<(), String> {
    if parsed.waveforms.len().saturating_add(aliases.len()) > MAX_RESULT_COLUMNS - 1 {
        return Err(adapter_error(
            format,
            "digital aliases exceed the signal-count limit",
        ));
    }
    let mut known = parsed
        .waveforms
        .iter()
        .map(|waveform| waveform.name.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    let mut alias_waveforms = Vec::with_capacity(aliases.len());
    for (canonical_index, name) in aliases {
        validate_name(format, "signal", &name)?;
        if !known.insert(name.to_ascii_lowercase()) {
            return Err(adapter_error(
                format,
                format_args!("duplicate digital signal identity '{name}'"),
            ));
        }
        let canonical = parsed.waveforms.get(canonical_index).ok_or_else(|| {
            adapter_error(
                format,
                "digital alias references an unknown canonical signal",
            )
        })?;
        alias_waveforms.push(WaveformData::new(
            name,
            Arc::clone(&canonical.x),
            Arc::clone(&canonical.y),
            trace_color(parsed.waveforms.len() + alias_waveforms.len()),
        ));
    }
    parsed.waveforms.extend(alias_waveforms);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_array::{ArrayRef, Float64Array, RecordBatch};
    use arrow_schema::{DataType, Field, Schema};
    use npyz::WriterBuilder as _;
    use std::io::Write as _;
    use std::sync::Arc;

    fn assert_basic(parsed: ParsedResultDataset, format: ResultImportFormat) {
        assert_eq!(parsed.source_format, format);
        assert!(parsed.sample_count >= 2);
        assert!(!parsed.waveforms.is_empty());
        assert!(parsed.waveforms.iter().all(|waveform| {
            waveform.x.len() == parsed.sample_count && waveform.y.len() == parsed.sample_count
        }));
    }

    fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let cursor = Cursor::new(Vec::new());
        let mut writer = zip::ZipWriter::new(cursor);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        for (name, contents) in entries {
            writer.start_file(*name, options).expect("start ZIP member");
            writer.write_all(contents).expect("write ZIP member");
        }
        writer.finish().expect("finish ZIP").into_inner()
    }

    fn native_bundle(format: ResultImportFormat) -> Vec<u8> {
        let dataset = br#"{"schema":"rspice-waveform-dataset/1","analysis":"ac","coordinate":{"name":"frequency","values":[1.0,2.0]},"signals":[{"name":"V(out)","real":[1.0,2.0],"imag":[0.5,-0.5]}]}"#;
        use sha2::Digest as _;
        let schema = if format == ResultImportFormat::RSpiceResultBundle {
            "rspice-result-bundle/1"
        } else {
            "rspice-dataset-bundle/1"
        };
        let manifest = format!(
            "{{\"schema\":\"{schema}\",\"dataset_member\":\"dataset.json\",\"dataset_sha256\":\"{:x}\"}}",
            sha2::Sha256::digest(dataset)
        );
        zip_bytes(&[
            ("manifest.json", manifest.as_bytes()),
            ("dataset.json", dataset),
        ])
    }

    #[test]
    fn native_result_and_dataset_bundles_verify_digest_and_complex_samples() {
        for format in [
            ResultImportFormat::RSpiceResultBundle,
            ResultImportFormat::RSpiceDatasetBundle,
        ] {
            let parsed =
                parse_native_bundle(&native_bundle(format), format).expect("native bundle");
            assert_basic(parsed, format);
        }
        let mut bad = native_bundle(ResultImportFormat::RSpiceDatasetBundle);
        let last = bad.len() - 1;
        bad[last] ^= 0x01;
        assert!(parse_native_bundle(&bad, ResultImportFormat::RSpiceDatasetBundle).is_err());
    }

    #[test]
    fn native_export_schema_is_deterministic_and_round_trips_real_and_complex() {
        use crate::workbench::workflows::native_result_bundle::{
            NativeBundleAnalysis, NativeBundleDataset, NativeBundleKind, NativeBundleSignal,
            NativeBundleSignalValues, encode_native_bundle,
        };

        let coordinate = [1.0e3, 2.0e3, 4.0e3];
        let real_values = [0.25, 0.5, 1.0];
        let complex_real = [1.0, -2.0, 0.5];
        let complex_imag = [0.125, 0.25, -0.75];
        let dataset = NativeBundleDataset {
            analysis: NativeBundleAnalysis::Ac,
            coordinate_name: "frequency",
            coordinate: &coordinate,
            signals: vec![
                NativeBundleSignal {
                    name: "gain",
                    unit: None,
                    values: NativeBundleSignalValues::Real(&real_values),
                },
                NativeBundleSignal {
                    name: "V(out)",
                    unit: Some("V"),
                    values: NativeBundleSignalValues::Complex {
                        real: &complex_real,
                        imag: &complex_imag,
                    },
                },
            ],
        };

        for (kind, format) in [
            (
                NativeBundleKind::Result,
                ResultImportFormat::RSpiceResultBundle,
            ),
            (
                NativeBundleKind::Dataset,
                ResultImportFormat::RSpiceDatasetBundle,
            ),
        ] {
            let bytes = encode_native_bundle(kind, &dataset).expect("native bundle encode");
            assert_eq!(
                bytes,
                encode_native_bundle(kind, &dataset).expect("repeat deterministic encode")
            );

            let mut archive = zip::ZipArchive::new(Cursor::new(bytes.as_slice())).unwrap();
            assert_eq!(archive.len(), 2);
            let manifest_bytes = read_zip_member(&mut archive, "manifest.json", format).unwrap();
            let dataset_bytes = read_zip_member(&mut archive, "dataset.json", format).unwrap();
            let manifest: serde_json::Value = serde_json::from_slice(&manifest_bytes).unwrap();
            let document: serde_json::Value = serde_json::from_slice(&dataset_bytes).unwrap();
            assert_eq!(manifest["schema"], kind.manifest_schema());
            assert_eq!(manifest["dataset_member"], "dataset.json");
            assert_eq!(document["schema"], "rspice-waveform-dataset/1");
            assert_eq!(document["analysis"], "ac");
            assert_eq!(document["signals"][0]["values"][1], 0.5);
            assert_eq!(document["signals"][1]["real"][1], -2.0);
            assert_eq!(document["signals"][1]["imag"][2], -0.75);
            use sha2::Digest as _;
            assert_eq!(
                manifest["dataset_sha256"],
                format!("{:x}", sha2::Sha256::digest(&dataset_bytes))
            );

            let parsed = parse_native_bundle(&bytes, format).expect("exporter/importer round-trip");
            assert_eq!(parsed.analysis_type, AnalysisType::Ac);
            assert_eq!(parsed.coordinate_name, "frequency");
            assert_eq!(parsed.waveforms.len(), 2);
            assert_eq!(parsed.waveforms[0].name, "gain");
            assert_eq!(parsed.waveforms[0].y.as_ref(), real_values.as_slice());
            let complex = parsed.waveforms[1]
                .complex
                .as_ref()
                .expect("complex identity retained");
            assert_eq!(complex.source_name, "V(out)");
            assert_eq!(complex.real.as_ref(), complex_real.as_slice());
            assert_eq!(complex.imag.as_ref(), complex_imag.as_slice());

            let mut tampered_manifest = manifest;
            tampered_manifest["dataset_sha256"] = serde_json::Value::String("00".repeat(32));
            let tampered_manifest = serde_json::to_vec(&tampered_manifest).unwrap();
            let tampered = zip_bytes(&[
                ("manifest.json", &tampered_manifest),
                ("dataset.json", &dataset_bytes),
            ]);
            let error = parse_native_bundle(&tampered, format).expect_err("digest tamper");
            assert!(error.contains("SHA-256"), "{error}");
        }
    }

    fn generic_hdf5() -> Vec<u8> {
        let mut builder = rustyhdf5::FileBuilder::new();
        builder
            .create_dataset("time")
            .with_f64_data(&[0.0, 1e-9, 2e-9]);
        builder
            .create_dataset("V(out)")
            .with_f64_data(&[0.0, 1.0, 0.0]);
        builder.finish().expect("HDF5 fixture")
    }

    #[test]
    fn hdf5_and_matlab_v73_import_real_root_vectors() {
        let bytes = generic_hdf5();
        assert_basic(
            parse_hdf5(&bytes, ResultImportFormat::Hdf5).expect("HDF5"),
            ResultImportFormat::Hdf5,
        );
        assert_basic(
            parse_matlab_v73(&bytes, ResultImportFormat::MatlabV73).expect("MATLAB 7.3"),
            ResultImportFormat::MatlabV73,
        );
        assert!(parse_hdf5(&bytes[..16], ResultImportFormat::Hdf5).is_err());
    }

    fn arrow_batch() -> (Arc<Schema>, RecordBatch) {
        let mut metadata = HashMap::new();
        metadata.insert("rspice.coordinate".to_owned(), "frequency".to_owned());
        metadata.insert("rspice.analysis".to_owned(), "ac".to_owned());
        let schema = Arc::new(Schema::new_with_metadata(
            vec![
                Field::new("frequency", DataType::Float64, false),
                Field::new("V(out)__real", DataType::Float64, false),
                Field::new("V(out)__imag", DataType::Float64, false),
            ],
            metadata,
        ));
        let arrays: Vec<ArrayRef> = vec![
            Arc::new(Float64Array::from(vec![1.0, 2.0])),
            Arc::new(Float64Array::from(vec![1.0, 2.0])),
            Arc::new(Float64Array::from(vec![0.5, -0.5])),
        ];
        let batch = RecordBatch::try_new(Arc::clone(&schema), arrays).expect("record batch");
        (schema, batch)
    }

    #[test]
    fn arrow_file_and_stream_import_complex_columns() {
        let (schema, batch) = arrow_batch();
        let mut file_bytes = Vec::new();
        {
            let mut writer = arrow_ipc::writer::FileWriter::try_new(&mut file_bytes, &schema)
                .expect("Arrow file writer");
            writer.write(&batch).expect("Arrow file batch");
            writer.finish().expect("Arrow file finish");
        }
        assert_basic(
            parse_arrow_ipc(&file_bytes, ResultImportFormat::ArrowIpc).expect("Arrow file"),
            ResultImportFormat::ArrowIpc,
        );

        let mut stream_bytes = Vec::new();
        {
            let mut writer = arrow_ipc::writer::StreamWriter::try_new(&mut stream_bytes, &schema)
                .expect("Arrow stream writer");
            writer.write(&batch).expect("Arrow stream batch");
            writer.finish().expect("Arrow stream finish");
        }
        assert_basic(
            parse_arrow_ipc(&stream_bytes, ResultImportFormat::ArrowIpc).expect("Arrow stream"),
            ResultImportFormat::ArrowIpc,
        );
        assert!(parse_arrow_ipc(&file_bytes[..20], ResultImportFormat::ArrowIpc).is_err());
    }

    #[test]
    fn parquet_imports_complex_columns_and_rejects_truncation() {
        use parquet::arrow::ArrowWriter;
        let (schema, batch) = arrow_batch();
        let mut writer = ArrowWriter::try_new(Vec::new(), schema, None).expect("Parquet writer");
        writer.write(&batch).expect("Parquet batch");
        let bytes = writer.into_inner().expect("Parquet finish");
        assert_basic(
            parse_parquet(&bytes, ResultImportFormat::Parquet).expect("Parquet"),
            ResultImportFormat::Parquet,
        );
        assert!(parse_parquet(&bytes[..20], ResultImportFormat::Parquet).is_err());
    }

    fn npy_f64(shape: &[u64], values: &[f64]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut writer = npyz::WriteOptions::new()
            .default_dtype()
            .shape(shape)
            .writer(&mut bytes)
            .begin_nd()
            .expect("NPY writer");
        writer.extend(values.iter().copied()).expect("NPY values");
        writer.finish().expect("NPY finish");
        bytes
    }

    #[test]
    fn numpy_npy_and_npz_import_real_arrays() {
        let npy = npy_f64(&[3, 2], &[0.0, 1.0, 1.0, 2.0, 2.0, 3.0]);
        assert_basic(
            parse_npy(&npy, ResultImportFormat::NumpyNpy).expect("NPY"),
            ResultImportFormat::NumpyNpy,
        );
        let time = npy_f64(&[3], &[0.0, 1.0, 2.0]);
        let out = npy_f64(&[3], &[1.0, 2.0, 3.0]);
        let npz = zip_bytes(&[("time.npy", &time), ("V(out).npy", &out)]);
        assert_basic(
            parse_npz(&npz, ResultImportFormat::NumpyNpz).expect("NPZ"),
            ResultImportFormat::NumpyNpz,
        );
        assert!(parse_npy(&npy[..12], ResultImportFormat::NumpyNpy).is_err());
        assert!(parse_npz(&npz[..20], ResultImportFormat::NumpyNpz).is_err());
    }

    fn mat_element(kind: u32, data: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend_from_slice(&kind.to_le_bytes());
        output.extend_from_slice(&(data.len() as u32).to_le_bytes());
        output.extend_from_slice(data);
        output.resize((output.len() + 7) & !7, 0);
        output
    }

    fn mat_matrix(name: &str, values: &[f64]) -> Vec<u8> {
        let mut body = Vec::new();
        let mut flags = Vec::new();
        flags.extend_from_slice(&6_u32.to_le_bytes());
        flags.extend_from_slice(&0_u32.to_le_bytes());
        body.extend(mat_element(6, &flags));
        let mut dimensions = Vec::new();
        dimensions.extend_from_slice(&(values.len() as i32).to_le_bytes());
        dimensions.extend_from_slice(&1_i32.to_le_bytes());
        body.extend(mat_element(5, &dimensions));
        body.extend(mat_element(1, name.as_bytes()));
        let value_bytes = values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect::<Vec<_>>();
        body.extend(mat_element(9, &value_bytes));
        mat_element(14, &body)
    }

    fn matlab_v5() -> Vec<u8> {
        let mut bytes = vec![b' '; 128];
        let description = b"MATLAB 5.0 MAT-file, RSpice import fixture";
        bytes[..description.len()].copy_from_slice(description);
        bytes[116..124].fill(0);
        bytes[124..126].copy_from_slice(&0x0100_u16.to_le_bytes());
        bytes[126..128].copy_from_slice(b"IM");
        bytes.extend(mat_matrix("time", &[0.0, 1.0, 2.0]));
        bytes.extend(mat_matrix("V_out", &[1.0, 2.0, 3.0]));
        bytes
    }

    #[test]
    fn matlab_v5_imports_numeric_vectors_and_rejects_truncation() {
        let bytes = matlab_v5();
        assert_basic(
            parse_matlab_v5(&bytes, ResultImportFormat::MatlabV5).expect("MATLAB v5"),
            ResultImportFormat::MatlabV5,
        );
        assert!(parse_matlab_v5(&bytes[..140], ResultImportFormat::MatlabV5).is_err());
    }

    #[test]
    fn spice_raw_and_psf_ascii_import_real_waveforms() {
        let raw = b"Title: fixture\nDate: now\nPlotname: Transient Analysis\nFlags: real double\nNo. Variables: 2\nNo. Points: 3\nVariables:\n\t0\ttime\ttime\n\t1\tV(out)\tvoltage\nValues:\n0\t0\n\t1\n1\t1e-9\n\t2\n2\t2e-9\n\t3\n";
        assert_basic(
            parse_spice_raw(raw, ResultImportFormat::SpiceRaw).expect("SPICE RAW"),
            ResultImportFormat::SpiceRaw,
        );
        let psf = b"HEADER\n\"analysis\" \"tran\"\nSWEEP\n\"time\" \"s\"\nTRACE\n\"V(out)\" \"V\"\nVALUE\n0 1\n1e-9 2\n2e-9 3\nEND\n";
        assert_basic(
            parse_psf_ascii(psf, ResultImportFormat::PsfAscii).expect("PSF ASCII"),
            ResultImportFormat::PsfAscii,
        );
        assert!(parse_spice_raw(&raw[..40], ResultImportFormat::SpiceRaw).is_err());
        assert!(parse_psf_ascii(&psf[..30], ResultImportFormat::PsfAscii).is_err());
    }

    #[test]
    fn vcd_imports_initialized_digital_events_and_rejects_unknowns() {
        let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! clk $end\n$var wire 2 \" bus $end\n$upscope $end\n$enddefinitions $end\n#0\n0!\nb00 \"\n#5\n1!\nb01 \"\n#10\n0!\nb10 \"\n";
        let parsed = parse_vcd(vcd, ResultImportFormat::Vcd).expect("VCD");
        assert_basic(parsed, ResultImportFormat::Vcd);
        let unknown = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! a $end\n$upscope $end\n$enddefinitions $end\n#0\nx!\n#1\n1!\n";
        assert!(
            parse_vcd(unknown, ResultImportFormat::Vcd)
                .expect_err("X must reject")
                .contains("X/Z")
        );
    }

    fn generated_fst() -> Vec<u8> {
        use fst_writer::{
            FstFileType, FstInfo, FstScopeType, FstSignalType, FstVarDirection, FstVarType,
        };
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-result-import-{}-{nonce}.fst",
            std::process::id()
        ));
        let info = FstInfo {
            start_time: 0,
            timescale_exponent: -9,
            version: "RSpice import fixture".to_owned(),
            date: "2026-08-31".to_owned(),
            file_type: FstFileType::Verilog,
        };
        let mut header = fst_writer::open_fst(&path, &info).expect("open FST fixture");
        header
            .scope("top", "top", FstScopeType::Module)
            .expect("FST scope");
        let clock = header
            .var(
                "clock",
                FstSignalType::bit_vec(1),
                FstVarType::Wire,
                FstVarDirection::Implicit,
                None,
            )
            .expect("FST signal");
        header.up_scope().expect("FST upscope");
        let mut body = header.finish().expect("FST header");
        for tick in 0..96_u64 {
            body.time_change(tick).expect("FST time change");
            body.signal_change(clock, if tick & 1 == 0 { b"0" } else { b"1" })
                .expect("FST signal change");
        }
        body.finish().expect("FST finish");
        let bytes = std::fs::read(&path).expect("read FST fixture");
        let _ = std::fs::remove_file(&path);
        bytes
    }

    fn fst_test_blocks(bytes: &[u8]) -> Vec<(u8, usize, usize)> {
        let mut blocks = Vec::new();
        let mut cursor = 0_usize;
        while cursor < bytes.len() {
            let block = cursor;
            let block_type = bytes[cursor];
            cursor += 1;
            let length = u64::from_be_bytes(bytes[cursor..cursor + 8].try_into().unwrap());
            if block_type == 255 && length == 0 {
                break;
            }
            let end = cursor + usize::try_from(length).unwrap();
            blocks.push((block_type, block, end));
            cursor = end;
        }
        blocks
    }

    fn set_fst_u64(bytes: &mut [u8], offset: usize, value: u64) {
        bytes[offset..offset + 8].copy_from_slice(&value.to_be_bytes());
    }

    fn fst_test_block(block_type: u8, payload: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(9 + payload.len());
        bytes.push(block_type);
        bytes.extend_from_slice(&(8_u64 + payload.len() as u64).to_be_bytes());
        bytes.extend_from_slice(payload);
        bytes
    }

    fn test_uleb(mut value: u64) -> Vec<u8> {
        let mut bytes = Vec::new();
        loop {
            let next = value >> 7;
            bytes.push((value as u8 & 0x7f) | if next == 0 { 0 } else { 0x80 });
            value = next;
            if value == 0 {
                break;
            }
        }
        bytes
    }

    fn fst_with_packed_signal(pack_type: u8, expanded_size: u64) -> Vec<u8> {
        let mut header = vec![0_u8; (FST_HEADER_SECTION_BYTES - 8) as usize];
        header[16..24].copy_from_slice(&std::f64::consts::E.to_be_bytes());
        set_fst_u64(&mut header, 32, 1); // scopes
        set_fst_u64(&mut header, 40, 1); // variables
        set_fst_u64(&mut header, 48, 1); // handles
        set_fst_u64(&mut header, 56, 1); // value-change sections
        let mut bytes = fst_test_block(0, &header);

        let mut geometry = Vec::new();
        geometry.extend_from_slice(&1_u64.to_be_bytes());
        geometry.extend_from_slice(&1_u64.to_be_bytes());
        geometry.push(1); // one-bit signal
        bytes.extend(fst_test_block(3, &geometry));

        let mut hierarchy = Vec::new();
        hierarchy.extend_from_slice(&1_u64.to_be_bytes());
        hierarchy.push(0); // declaration is enough for a preflight failure fixture
        bytes.extend(fst_test_block(6, &hierarchy));

        let signal = test_uleb(expanded_size);
        let mut data = Vec::new();
        data.extend_from_slice(&0_u64.to_be_bytes()); // start
        data.extend_from_slice(&1_u64.to_be_bytes()); // end
        data.extend_from_slice(&MAX_RESULT_DATASET_BYTES.to_be_bytes());
        data.extend([1, 1, 1, b'0']); // direct one-byte frame, one handle
        data.push(1); // value-change handle count
        data.push(pack_type);
        data.extend_from_slice(&signal);
        data.push(3); // DynamicAlias2 first signal offset is one byte after pack type
        data.extend_from_slice(&1_u64.to_be_bytes()); // offset-table byte count
        data.push(1); // direct one-byte time delta stream
        data.extend_from_slice(&1_u64.to_be_bytes());
        data.extend_from_slice(&1_u64.to_be_bytes());
        data.extend_from_slice(&1_u64.to_be_bytes());
        bytes.extend(fst_test_block(8, &data));
        bytes
    }

    #[test]
    fn fst_reader_imports_a_real_generated_container_and_rejects_truncation() {
        let bytes = generated_fst();
        assert!(looks_like_fst(&bytes));
        preflight_fst(&bytes, ResultImportFormat::Fst).expect("ordinary FST preflight");
        assert_basic(
            parse_fst(&bytes, ResultImportFormat::Fst).expect("FST import"),
            ResultImportFormat::Fst,
        );
        assert!(parse_fst(&bytes[..64], ResultImportFormat::Fst).is_err());
    }

    #[test]
    fn fst_preflight_rejects_huge_fixed_allocation_declarations() {
        let baseline = generated_fst();
        let blocks = fst_test_blocks(&baseline);
        let geometry = blocks.iter().find(|block| block.0 == 3).copied().unwrap();
        let hierarchy = blocks
            .iter()
            .find(|block| matches!(block.0, 4 | 6 | 7))
            .copied()
            .unwrap();
        let data = blocks
            .iter()
            .find(|block| matches!(block.0, 1 | 5 | 8))
            .copied()
            .unwrap();
        let time_compressed = usize::try_from(u64::from_be_bytes(
            baseline[data.2 - 16..data.2 - 8].try_into().unwrap(),
        ))
        .unwrap();
        let offset_table_length = data.2 - 24 - time_compressed - 8;
        let too_large = MAX_RESULT_DATASET_BYTES + 1;

        for (label, offset) in [
            ("header signal count", 1 + 8 + 48),
            ("geometry expanded", geometry.1 + 1 + 8),
            ("hierarchy expanded", hierarchy.1 + 1 + 8),
            ("data allocation", data.1 + 1 + 24),
            ("offset-table compressed", offset_table_length),
            ("time-table expanded", data.2 - 24),
            ("time-table compressed", data.2 - 16),
            ("time-table item count", data.2 - 8),
        ] {
            let mut bytes = baseline.clone();
            set_fst_u64(&mut bytes, offset, too_large);
            let error = preflight_fst(&bytes, ResultImportFormat::Fst)
                .expect_err("oversized FST declaration must reject");
            assert!(
                error.contains("limit") || error.contains("count"),
                "{label}: {error}"
            );
        }
    }

    #[test]
    fn fst_preflight_rejects_huge_frame_and_signal_count_varints() {
        let baseline = generated_fst();
        let data = fst_test_blocks(&baseline)
            .into_iter()
            .find(|block| matches!(block.0, 1 | 5 | 8))
            .unwrap();
        let oversized = test_uleb(MAX_RESULT_DATASET_BYTES + 1);
        for (label, offset) in [
            ("initial-frame expanded", data.1 + 33),
            ("initial-frame compressed", data.1 + 34),
            ("initial-frame signal count", data.1 + 35),
            ("value-change signal count", data.1 + 37),
        ] {
            let mut bytes = baseline.clone();
            bytes[offset..offset + oversized.len()].copy_from_slice(&oversized);
            let error = preflight_fst(&bytes, ResultImportFormat::Fst)
                .expect_err("oversized FST variable integer must reject");
            assert!(
                error.contains("limit") || error.contains("count"),
                "{label}: {error}"
            );
        }
    }

    #[test]
    fn fst_preflight_rejects_wrappers_unknown_blocks_overflow_and_truncation() {
        let mut invalid_header = generated_fst();
        invalid_header[1 + 8 + 16..1 + 8 + 24].fill(0);
        let error = preflight_fst(&invalid_header, ResultImportFormat::Fst)
            .expect_err("invalid endian marker");
        assert!(error.contains("endian marker"), "{error}");

        let mut reversed_time = generated_fst();
        set_fst_u64(&mut reversed_time, 1 + 8, 2);
        set_fst_u64(&mut reversed_time, 1 + 8 + 8, 1);
        let error = preflight_fst(&reversed_time, ResultImportFormat::Fst)
            .expect_err("reversed header time range");
        assert!(error.contains("precedes"), "{error}");

        let wrapper = fst_test_block(254, &u64::MAX.to_be_bytes());
        let error = preflight_fst(&wrapper, ResultImportFormat::Fst).expect_err("gzip bomb");
        assert!(error.contains("gzip wrapper expanded"), "{error}");

        let bounded_wrapper = fst_test_block(254, &16_u64.to_be_bytes());
        let error = preflight_fst(&bounded_wrapper, ResultImportFormat::Fst)
            .expect_err("nested gzip cannot be preflighted");
        assert!(error.contains("nested framing"), "{error}");

        let unknown = fst_test_block(9, &[]);
        assert!(
            preflight_fst(&unknown, ResultImportFormat::Fst)
                .expect_err("unknown block")
                .contains("unknown")
        );

        let mut overflow = vec![0];
        overflow.extend_from_slice(&u64::MAX.to_be_bytes());
        assert!(preflight_fst(&overflow, ResultImportFormat::Fst).is_err());
        assert!(preflight_fst(&[0, 0, 0], ResultImportFormat::Fst).is_err());
    }

    #[test]
    fn fst_preflight_rejects_compressed_geometry_and_duo_intermediate_bombs() {
        let mut compressed_geometry = generated_fst();
        let geometry = fst_test_blocks(&compressed_geometry)
            .into_iter()
            .find(|block| block.0 == 3)
            .unwrap();
        let declared = u64::from_be_bytes(
            compressed_geometry[geometry.1 + 9..geometry.1 + 17]
                .try_into()
                .unwrap(),
        );
        set_fst_u64(
            &mut compressed_geometry,
            geometry.1 + 9,
            declared.saturating_add(1),
        );
        let error = preflight_fst(&compressed_geometry, ResultImportFormat::Fst)
            .expect_err("compressed geometry must fail closed");
        assert!(error.contains("compressed FST geometry"), "{error}");

        let mut duo = Vec::new();
        duo.extend_from_slice(&1_u64.to_be_bytes());
        duo.extend(test_uleb(MAX_RESULT_DATASET_BYTES + 1));
        let duo = fst_test_block(7, &duo);
        let error = preflight_fst(&duo, ResultImportFormat::Fst).expect_err("LZ4 duo bomb");
        assert!(error.contains("LZ4-duo"), "{error}");
    }

    #[test]
    fn fst_preflight_bounds_lz4_fastlz_and_zlib_signal_expansion() {
        for (name, pack_type) in [("LZ4", b'4'), ("FastLZ", b'F'), ("zlib", b'Z')] {
            let bytes = fst_with_packed_signal(pack_type, MAX_RESULT_DATASET_BYTES + 1);
            let error = preflight_fst(&bytes, ResultImportFormat::Fst)
                .expect_err("packed FST signal bomb must reject");
            assert!(error.contains(name), "{name}: {error}");
            assert!(error.contains("limit"), "{name}: {error}");
        }
    }
}
