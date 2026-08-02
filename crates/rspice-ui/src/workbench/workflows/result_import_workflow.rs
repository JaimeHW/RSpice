//! Validated external result-dataset ingest.
//!
//! CSV/TSV imports are deliberately classified as legacy-unattributed
//! external evidence. They become immutable retained runs, but never claim
//! native-solver provenance or prepared-plan authority.

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    AnalysisResult, AnalysisType, SimulationRunLifecycle, SimulationRunProvenance, WaveformData,
};
use crate::workbench::app_state::AppState;
use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read as _;
use std::path::Path;
use std::sync::Arc;

pub(crate) const RESULT_DATASET_FILTER: (&str, &[&str]) =
    ("Delimited result dataset", &["csv", "tsv"]);
pub(crate) const MAX_RESULT_DATASET_BYTES: u64 = 64 * 1024 * 1024;
const MAX_RESULT_COLUMNS: usize = 1_024;
const MAX_RESULT_ROWS: usize = 1_000_000;
const MAX_HEADER_BYTES: usize = 256;
const MIN_RESULT_ROWS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
enum UnitDimension {
    Dimensionless,
    Time,
    Frequency,
    Voltage,
    Current,
    Resistance,
    Conductance,
    Power,
    Capacitance,
    Inductance,
    Temperature,
    Angle,
    LogRatio,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct UnitContract {
    dimension: UnitDimension,
    scale: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct ColumnContract {
    name: String,
    unit: Option<UnitContract>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParsedResultDataset {
    pub(crate) analysis_type: AnalysisType,
    pub(crate) coordinate_name: String,
    pub(crate) sample_count: usize,
    pub(crate) waveforms: Vec<WaveformData>,
    pub(crate) delimiter: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoordinateDirection {
    Increasing,
    Decreasing,
}

pub(crate) fn import_result_dataset(state: &mut AppState) -> bool {
    if let Some(reason) = result_import_block_reason(state) {
        state.push_user_message(ConsoleMessage::warning(reason));
        return false;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(path) = rfd::FileDialog::new()
            .add_filter(RESULT_DATASET_FILTER.0, RESULT_DATASET_FILTER.1)
            .set_title("Import result dataset")
            .pick_file()
        else {
            return false;
        };
        match read_native_file_bounded(&path).and_then(|bytes| {
            let display_name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            apply_imported_result_dataset(state, &display_name, &bytes)
        }) {
            Ok(()) => true,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Result dataset import failed: {error}"
                )));
                false
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        match start_browser_result_dataset_import() {
            Ok(()) => true,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Result dataset import failed: {error}"
                )));
                false
            }
        }
    }
}

fn result_import_block_reason(state: &AppState) -> Option<String> {
    if !state.project_lifecycle.project_open {
        return Some("Open a project before importing a result dataset.".to_owned());
    }
    if state.workbench.safe_mode.project_read_only() {
        return Some("The project is read-only; result history cannot be changed.".to_owned());
    }
    if state.simulation.active_execution.is_some() || state.simulation.is_running {
        return Some(
            "Wait for the active simulation execution to finish before importing result data."
                .to_owned(),
        );
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn read_native_file_bounded(path: &Path) -> Result<Vec<u8>, String> {
    let file = std::fs::File::open(path)
        .map_err(|error| format!("could not open {}: {error}", path.display()))?;
    let size = file
        .metadata()
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
        .len();
    if size > MAX_RESULT_DATASET_BYTES {
        return Err(format!(
            "{} is {} bytes; the import limit is {} bytes",
            path.display(),
            size,
            MAX_RESULT_DATASET_BYTES
        ));
    }

    let mut bytes = Vec::with_capacity(
        usize::try_from(size.min(MAX_RESULT_DATASET_BYTES))
            .unwrap_or(MAX_RESULT_DATASET_BYTES as usize),
    );
    file.take(MAX_RESULT_DATASET_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    if bytes.len() as u64 > MAX_RESULT_DATASET_BYTES {
        return Err(format!(
            "{} exceeds the {}-byte import limit",
            path.display(),
            MAX_RESULT_DATASET_BYTES
        ));
    }
    Ok(bytes)
}

pub(crate) fn apply_imported_result_dataset(
    state: &mut AppState,
    source_name: &str,
    bytes: &[u8],
) -> Result<(), String> {
    if let Some(reason) = result_import_block_reason(state) {
        return Err(reason);
    }
    let parsed = parse_result_dataset(source_name, bytes)?;
    let analysis_type = parsed.analysis_type;
    let coordinate_name = parsed.coordinate_name.clone();
    let sample_count = parsed.sample_count;
    let signal_count = parsed.waveforms.len();
    let analysis_label = match parsed.analysis_type {
        AnalysisType::Transient => format!("Imported transient · {coordinate_name}"),
        AnalysisType::Ac => format!("Imported AC · {coordinate_name}"),
        AnalysisType::DcSweep => format!("Imported DC sweep · {coordinate_name}"),
        _ => return Err("import inferred an unsupported analysis domain".to_owned()),
    };
    let analysis =
        AnalysisResult::new(1, analysis_type, analysis_label).with_waveforms(parsed.waveforms);

    let (run_sequence, run_id, dataset_id) = {
        let run = state.simulation.start_run();
        run.job_id = None;
        run.execution_target = None;
        run.label = format!("Imported · {source_name}");
        run.add_analysis(analysis);
        let run_sequence = run.id;
        let run_id = run.run_id;
        let dataset_id = run.dataset_id;
        let sealed = run
            .restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .and_then(|()| run.mark_running())
            .and_then(|()| run.finish_lifecycle(SimulationRunLifecycle::Completed));
        if let Err(error) = sealed {
            let _ = state.simulation.delete_run(0);
            return Err(format!("could not seal imported result history: {error}"));
        }
        (run_sequence, run_id, dataset_id)
    };

    state.simulation.complete_run();
    state.synchronize_specialized_viewer_cache_authority();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    let document = crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id);
    if !crate::workbench::chrome::document_bar::activate_document_by_id(state, &document) {
        return Err(format!(
            "imported dataset {dataset_id} was retained but its Results document could not be activated"
        ));
    }
    state.ui.results.viewer = crate::workbench::ResultViewer::Waves;
    state.push_user_message(ConsoleMessage::info(format!(
        "Imported {source_name} as immutable dataset {dataset_id}: {signal_count} signals × {sample_count} samples ({})",
        analysis_domain_label(analysis_type)
    )));
    state.push_user_message(ConsoleMessage::warning(format!(
        "Dataset {dataset_id} / run {run_id} (Run {run_sequence}) is external legacy-unattributed evidence, not native RSpice solver output"
    )));
    Ok(())
}

fn analysis_domain_label(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::Transient => "transient",
        AnalysisType::Ac => "AC",
        AnalysisType::DcSweep => "DC sweep",
        _ => "unsupported",
    }
}

pub(crate) fn parse_result_dataset(
    source_name: &str,
    bytes: &[u8],
) -> Result<ParsedResultDataset, String> {
    if bytes.is_empty() {
        return Err("the selected file is empty".to_owned());
    }
    if bytes.len() as u64 > MAX_RESULT_DATASET_BYTES {
        return Err(format!(
            "the selected file exceeds the {}-byte import limit",
            MAX_RESULT_DATASET_BYTES
        ));
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("the selected file is not valid UTF-8: {error}"))?;
    let delimiter = infer_delimiter(source_name, text)?;
    parse_delimited_result_dataset(text, delimiter)
}

fn infer_delimiter(source_name: &str, text: &str) -> Result<u8, String> {
    let extension = Path::new(source_name)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    match extension.as_deref() {
        Some("csv") => return Ok(b','),
        Some("tsv") => return Ok(b'\t'),
        Some(extension) => {
            return Err(format!(
                "unsupported .{extension} result format; select a UTF-8 CSV or TSV file"
            ));
        }
        None => {}
    }

    let header = text
        .lines()
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| "the selected file contains no header row".to_owned())?;
    let commas = header.bytes().filter(|byte| *byte == b',').count();
    let tabs = header.bytes().filter(|byte| *byte == b'\t').count();
    match (commas, tabs) {
        (0, 0) => Err("could not infer CSV or TSV delimiter from the header".to_owned()),
        (commas, tabs) if commas > 0 && tabs > 0 => Err(
            "the header mixes comma and tab delimiters; use a consistent CSV or TSV file"
                .to_owned(),
        ),
        (_, 0) => Ok(b','),
        (0, _) => Ok(b'\t'),
        _ => unreachable!(),
    }
}

fn parse_delimited_result_dataset(
    text: &str,
    delimiter: u8,
) -> Result<ParsedResultDataset, String> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(true)
        .flexible(false)
        .trim(csv::Trim::All)
        .from_reader(text.as_bytes());
    let raw_headers = reader
        .headers()
        .map_err(|error| csv_error("header", error))?
        .clone();
    if raw_headers.len() < 2 {
        return Err("the header must contain one coordinate and at least one signal".to_owned());
    }
    if raw_headers.len() > MAX_RESULT_COLUMNS {
        return Err(format!(
            "the dataset has {} columns; the import limit is {}",
            raw_headers.len(),
            MAX_RESULT_COLUMNS
        ));
    }

    let mut headers = Vec::with_capacity(raw_headers.len());
    let mut unique_names = HashSet::with_capacity(raw_headers.len());
    for (index, raw) in raw_headers.iter().enumerate() {
        let raw = if index == 0 {
            raw.strip_prefix('\u{feff}').unwrap_or(raw)
        } else {
            raw
        };
        let header = parse_column_header(raw, index + 1)?;
        let key = header.name.to_lowercase();
        if !unique_names.insert(key) {
            return Err(format!(
                "column {} repeats the signal/header name {:?}",
                index + 1,
                header.name
            ));
        }
        headers.push(header);
    }

    let (analysis_type, coordinate_scale) = infer_analysis_type(&headers[0])?;
    let signal_scales: Vec<f64> = headers[1..]
        .iter()
        .map(|header| header.unit.map_or(1.0, |unit| unit.scale))
        .collect();
    let mut coordinate = Vec::new();
    let mut signal_values = vec![Vec::new(); headers.len() - 1];
    let mut direction = None;

    for (row_index, record) in reader.records().enumerate() {
        let line = row_index + 2;
        if row_index >= MAX_RESULT_ROWS {
            return Err(format!(
                "the dataset exceeds the {}-row import limit",
                MAX_RESULT_ROWS
            ));
        }
        let record = record.map_err(|error| csv_error(&format!("row {line}"), error))?;
        let x = parse_finite_cell(record.get(0), line, 1, &headers[0].name, coordinate_scale)?;
        if analysis_type == AnalysisType::Ac && x <= 0.0 {
            return Err(format!(
                "row {line} frequency must be greater than zero after unit conversion"
            ));
        }
        if let Some(previous) = coordinate.last().copied() {
            let step = if x > previous {
                CoordinateDirection::Increasing
            } else if x < previous {
                CoordinateDirection::Decreasing
            } else {
                return Err(format!(
                    "row {line} repeats coordinate {x}; coordinates must be strictly monotonic"
                ));
            };
            if let Some(expected) = direction {
                if step != expected {
                    return Err(format!(
                        "row {line} reverses coordinate ordering; coordinates must remain strictly monotonic"
                    ));
                }
            } else {
                direction = Some(step);
            }
        }
        coordinate.push(x);

        for (signal_index, values) in signal_values.iter_mut().enumerate() {
            let column = signal_index + 2;
            values.push(parse_finite_cell(
                record.get(signal_index + 1),
                line,
                column,
                &headers[signal_index + 1].name,
                signal_scales[signal_index],
            )?);
        }
    }
    if coordinate.len() < MIN_RESULT_ROWS {
        return Err(format!(
            "the dataset contains {} sample rows; at least {} are required",
            coordinate.len(),
            MIN_RESULT_ROWS
        ));
    }

    let coordinate = Arc::new(coordinate);
    let waveforms = signal_values
        .into_iter()
        .enumerate()
        .map(|(index, values)| {
            WaveformData::new(
                headers[index + 1].name.clone(),
                Arc::clone(&coordinate),
                values,
                trace_color(index),
            )
        })
        .collect();
    Ok(ParsedResultDataset {
        analysis_type,
        coordinate_name: headers[0].name.clone(),
        sample_count: coordinate.len(),
        waveforms,
        delimiter,
    })
}

fn parse_column_header(raw: &str, column: usize) -> Result<ColumnContract, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(format!("column {column} has an empty header"));
    }
    if raw.len() > MAX_HEADER_BYTES {
        return Err(format!(
            "column {column} header exceeds the {}-byte limit",
            MAX_HEADER_BYTES
        ));
    }
    if raw.chars().any(char::is_control) {
        return Err(format!(
            "column {column} header contains a control character"
        ));
    }

    let (name, unit_text) = if raw.ends_with(']') {
        let open = raw.rfind('[').ok_or_else(|| {
            format!("column {column} header has a closing unit bracket without an opening bracket")
        })?;
        let name = raw[..open].trim();
        let unit = raw[open + 1..raw.len() - 1].trim();
        if name.is_empty() || unit.is_empty() {
            return Err(format!(
                "column {column} must provide both a name and a non-empty bracketed unit"
            ));
        }
        (name, Some(unit))
    } else {
        if raw.contains('[') || raw.contains(']') {
            return Err(format!("column {column} has an unmatched unit bracket"));
        }
        (raw, None)
    };
    let unit = unit_text
        .map(parse_unit)
        .transpose()
        .map_err(|error| format!("column {column} ({name:?}) has an invalid unit: {error}"))?;
    Ok(ColumnContract {
        name: name.to_owned(),
        unit,
    })
}

fn parse_unit(raw: &str) -> Result<UnitContract, String> {
    let symbol = raw
        .trim()
        .replace(['µ', 'μ'], "u")
        .replace('Ω', "ohm")
        .replace('°', "deg");
    let contract = match symbol.as_str() {
        "s" => (UnitDimension::Time, 1.0),
        "ms" => (UnitDimension::Time, 1e-3),
        "us" => (UnitDimension::Time, 1e-6),
        "ns" => (UnitDimension::Time, 1e-9),
        "ps" => (UnitDimension::Time, 1e-12),
        "fs" => (UnitDimension::Time, 1e-15),
        "Hz" | "hz" => (UnitDimension::Frequency, 1.0),
        "mHz" => (UnitDimension::Frequency, 1e-3),
        "kHz" | "KHz" => (UnitDimension::Frequency, 1e3),
        "MHz" => (UnitDimension::Frequency, 1e6),
        "GHz" => (UnitDimension::Frequency, 1e9),
        "THz" => (UnitDimension::Frequency, 1e12),
        "V" => (UnitDimension::Voltage, 1.0),
        "mV" => (UnitDimension::Voltage, 1e-3),
        "uV" => (UnitDimension::Voltage, 1e-6),
        "nV" => (UnitDimension::Voltage, 1e-9),
        "kV" => (UnitDimension::Voltage, 1e3),
        "A" => (UnitDimension::Current, 1.0),
        "mA" => (UnitDimension::Current, 1e-3),
        "uA" => (UnitDimension::Current, 1e-6),
        "nA" => (UnitDimension::Current, 1e-9),
        "pA" => (UnitDimension::Current, 1e-12),
        "ohm" => (UnitDimension::Resistance, 1.0),
        "mohm" => (UnitDimension::Resistance, 1e-3),
        "kohm" => (UnitDimension::Resistance, 1e3),
        "Mohm" => (UnitDimension::Resistance, 1e6),
        "Gohm" => (UnitDimension::Resistance, 1e9),
        "S" => (UnitDimension::Conductance, 1.0),
        "mS" => (UnitDimension::Conductance, 1e-3),
        "uS" => (UnitDimension::Conductance, 1e-6),
        "nS" => (UnitDimension::Conductance, 1e-9),
        "W" => (UnitDimension::Power, 1.0),
        "mW" => (UnitDimension::Power, 1e-3),
        "uW" => (UnitDimension::Power, 1e-6),
        "nW" => (UnitDimension::Power, 1e-9),
        "kW" => (UnitDimension::Power, 1e3),
        "F" => (UnitDimension::Capacitance, 1.0),
        "mF" => (UnitDimension::Capacitance, 1e-3),
        "uF" => (UnitDimension::Capacitance, 1e-6),
        "nF" => (UnitDimension::Capacitance, 1e-9),
        "pF" => (UnitDimension::Capacitance, 1e-12),
        "fF" => (UnitDimension::Capacitance, 1e-15),
        "H" => (UnitDimension::Inductance, 1.0),
        "mH" => (UnitDimension::Inductance, 1e-3),
        "uH" => (UnitDimension::Inductance, 1e-6),
        "nH" => (UnitDimension::Inductance, 1e-9),
        "K" => (UnitDimension::Temperature, 1.0),
        "C" | "degC" => (UnitDimension::Temperature, 1.0),
        _ => match symbol.to_ascii_lowercase().as_str() {
            "1" | "unitless" | "dimensionless" => (UnitDimension::Dimensionless, 1.0),
            "second" | "seconds" => (UnitDimension::Time, 1.0),
            "hertz" => (UnitDimension::Frequency, 1.0),
            "volt" | "volts" => (UnitDimension::Voltage, 1.0),
            "amp" | "amps" | "ampere" | "amperes" => (UnitDimension::Current, 1.0),
            "ohms" => (UnitDimension::Resistance, 1.0),
            "megohm" | "megohms" => (UnitDimension::Resistance, 1e6),
            "siemens" | "siemen" => (UnitDimension::Conductance, 1.0),
            "watt" | "watts" => (UnitDimension::Power, 1.0),
            "farad" | "farads" => (UnitDimension::Capacitance, 1.0),
            "henry" | "henries" => (UnitDimension::Inductance, 1.0),
            "kelvin" | "celsius" => (UnitDimension::Temperature, 1.0),
            "rad" | "radian" | "radians" => (UnitDimension::Angle, 1.0),
            "deg" | "degree" | "degrees" => (UnitDimension::Angle, 1.0),
            "db" => (UnitDimension::LogRatio, 1.0),
            "%" | "percent" => (UnitDimension::Dimensionless, 0.01),
            _ => return Err(format!("{raw:?} is not a recognized engineering unit")),
        },
    };
    Ok(UnitContract {
        dimension: contract.0,
        scale: contract.1,
    })
}

fn infer_analysis_type(header: &ColumnContract) -> Result<(AnalysisType, f64), String> {
    let normalized: String = header
        .name
        .chars()
        .filter(|character| !matches!(character, '_' | '-' | ' '))
        .flat_map(char::to_lowercase)
        .collect();
    let (analysis_type, required_dimension, default_scale) = match normalized.as_str() {
        "time" | "t" | "timestamp" => (AnalysisType::Transient, Some(UnitDimension::Time), 1.0),
        "frequency" | "freq" => (AnalysisType::Ac, Some(UnitDimension::Frequency), 1.0),
        "f" if header
            .unit
            .is_some_and(|unit| unit.dimension == UnitDimension::Frequency) =>
        {
            (AnalysisType::Ac, Some(UnitDimension::Frequency), 1.0)
        }
        "f" => {
            return Err(
                "coordinate header \"f\" is ambiguous without an explicit frequency unit such as [Hz]"
                    .to_owned(),
            );
        }
        _ if looks_like_signal_expression(&header.name) => {
            return Err(format!(
                "first column {:?} looks like a signal, not a coordinate; use time, frequency, or a DC sweep variable header",
                header.name
            ));
        }
        _ if header
            .unit
            .is_some_and(|unit| unit.dimension == UnitDimension::Time) =>
        {
            (AnalysisType::Transient, Some(UnitDimension::Time), 1.0)
        }
        _ if header
            .unit
            .is_some_and(|unit| unit.dimension == UnitDimension::Frequency) =>
        {
            (AnalysisType::Ac, Some(UnitDimension::Frequency), 1.0)
        }
        _ => (AnalysisType::DcSweep, None, 1.0),
    };

    if let (Some(required), Some(unit)) = (required_dimension, header.unit)
        && unit.dimension != required
    {
        return Err(format!(
            "{} coordinate {:?} requires a {} unit",
            analysis_domain_label(analysis_type),
            header.name,
            match required {
                UnitDimension::Time => "time",
                UnitDimension::Frequency => "frequency",
                _ => "compatible",
            }
        ));
    }
    if analysis_type == AnalysisType::DcSweep
        && header.unit.is_some_and(|unit| {
            matches!(
                unit.dimension,
                UnitDimension::Angle | UnitDimension::LogRatio
            )
        })
    {
        return Err(format!(
            "DC sweep coordinate {:?} cannot use an angular or logarithmic display unit",
            header.name
        ));
    }
    Ok((
        analysis_type,
        header.unit.map_or(default_scale, |unit| unit.scale),
    ))
}

fn looks_like_signal_expression(name: &str) -> bool {
    let trimmed = name.trim();
    let lower = trimmed.to_ascii_lowercase();
    (lower.starts_with("v(") || lower.starts_with("i(")) && trimmed.ends_with(')')
}

fn parse_finite_cell(
    value: Option<&str>,
    row: usize,
    column: usize,
    header: &str,
    scale: f64,
) -> Result<f64, String> {
    let value =
        value.ok_or_else(|| format!("row {row}, column {column} ({header:?}) is missing"))?;
    if value.is_empty() {
        return Err(format!("row {row}, column {column} ({header:?}) is empty"));
    }
    let parsed = value.parse::<f64>().map_err(|_| {
        format!("row {row}, column {column} ({header:?}) contains non-numeric value {value:?}")
    })?;
    if !parsed.is_finite() {
        return Err(format!(
            "row {row}, column {column} ({header:?}) must be finite"
        ));
    }
    // Every accepted engineering-unit scale is an exact decimal power of
    // ten. Parse the combined decimal exponent in one operation so values
    // such as `10 [uA]` round directly to the nearest representation of
    // `1e-5`, instead of accumulating a second rounding from `10.0 * 1e-6`.
    let scaled = decimal_power_scaled(value, scale).unwrap_or(parsed * scale);
    if !scaled.is_finite() {
        return Err(format!(
            "row {row}, column {column} ({header:?}) overflows after unit conversion"
        ));
    }
    Ok(scaled)
}

fn decimal_power_scaled(value: &str, scale: f64) -> Option<f64> {
    let scale_exponent = [
        (1e-15, -15),
        (1e-12, -12),
        (1e-9, -9),
        (1e-6, -6),
        (1e-3, -3),
        (0.01, -2),
        (1.0, 0),
        (1e3, 3),
        (1e6, 6),
        (1e9, 9),
        (1e12, 12),
    ]
    .into_iter()
    .find_map(|(candidate, exponent)| (scale == candidate).then_some(exponent))?;
    let (mantissa, source_exponent) = if let Some(separator) = value.find(['e', 'E']) {
        (
            &value[..separator],
            value[separator + 1..].parse::<i32>().ok()?,
        )
    } else {
        (value, 0_i32)
    };
    let combined_exponent = source_exponent.checked_add(scale_exponent)?;
    format!("{mantissa}e{combined_exponent}")
        .parse::<f64>()
        .ok()
}

fn csv_error(context: &str, error: csv::Error) -> String {
    if let Some(position) = error.position() {
        format!(
            "{context} is malformed near row {}, byte {}: {}",
            position.line(),
            position.byte(),
            error
        )
    } else {
        format!("{context} is malformed: {error}")
    }
}

fn trace_color(index: usize) -> &'static str {
    const COLORS: &[&str] = &[
        "#4FC3F7", "#FFB74D", "#81C784", "#BA68C8", "#E57373", "#4DB6AC", "#FFF176", "#7986CB",
    ];
    COLORS[index % COLORS.len()]
}

#[cfg(target_arch = "wasm32")]
enum BrowserResultDatasetImportResult {
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
    Failed(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
struct BrowserResultDatasetImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: BrowserResultDatasetImportResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_RESULT_DATASET_IMPORT_RESULT:
        std::cell::RefCell<Option<BrowserResultDatasetImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_result_dataset_import() -> Result<(), String> {
    let token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::ResultDataset,
    )?;
    crate::workbench::browser::file_import::pick_text_file(
        RESULT_DATASET_FILTER.0,
        RESULT_DATASET_FILTER.1,
        move |result| {
            if !crate::workbench::browser::file_import::text_import_is_current(token) {
                return;
            }
            let result = match result {
                Ok(Some(file)) => BrowserResultDatasetImportResult::Loaded(file),
                Ok(None) => BrowserResultDatasetImportResult::Cancelled,
                Err(error) => BrowserResultDatasetImportResult::Failed(error),
            };
            BROWSER_RESULT_DATASET_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserResultDatasetImportCompletion { token, result });
            });
        },
    );
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_result_dataset_import(state: &mut AppState) -> bool {
    let Some(completion) =
        BROWSER_RESULT_DATASET_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())
    else {
        return false;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return false;
    }
    match completion.result {
        BrowserResultDatasetImportResult::Loaded(file) => {
            match apply_imported_result_dataset(state, &file.name, file.contents.as_bytes()) {
                Ok(()) => true,
                Err(error) => {
                    state.push_user_message(ConsoleMessage::error(format!(
                        "Result dataset import failed: {error}"
                    )));
                    false
                }
            }
        }
        BrowserResultDatasetImportResult::Failed(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Result dataset import failed: {error}"
            )));
            false
        }
        BrowserResultDatasetImportResult::Cancelled => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_utf8_csv_transient_with_units_and_shared_coordinate() {
        let parsed = parse_result_dataset(
            "scope.csv",
            b"time [ms],V(out) [mV],I(VDD) [uA]\n0,0,10\n0.5,1250,11\n1,2500,12\n",
        )
        .expect("valid transient CSV");

        assert_eq!(parsed.analysis_type, AnalysisType::Transient);
        assert_eq!(parsed.sample_count, 3);
        assert_eq!(parsed.waveforms.len(), 2);
        assert_eq!(parsed.waveforms[0].x.as_slice(), &[0.0, 5e-4, 1e-3]);
        assert_eq!(parsed.waveforms[0].y.as_slice(), &[0.0, 1.25, 2.5]);
        assert_eq!(parsed.waveforms[1].y.as_slice(), &[10e-6, 11e-6, 12e-6]);
        assert!(Arc::ptr_eq(&parsed.waveforms[0].x, &parsed.waveforms[1].x));
    }

    #[test]
    fn parses_tsv_frequency_as_ac_and_requires_positive_ordered_frequency() {
        let parsed = parse_result_dataset(
            "response.tsv",
            b"frequency [kHz]\tV(out) [V]\n1\t0.5\n10\t0.25\n100\t0.1\n",
        )
        .expect("valid AC TSV");
        assert_eq!(parsed.analysis_type, AnalysisType::Ac);
        assert_eq!(parsed.waveforms[0].x.as_slice(), &[1e3, 1e4, 1e5]);

        let error = parse_result_dataset("response.csv", b"frequency [Hz],V(out) [V]\n1,1\n0,2\n")
            .expect_err("zero frequency rejected");
        assert!(error.contains("greater than zero"));
    }

    #[test]
    fn infers_domain_from_unambiguous_coordinate_units() {
        let transient = parse_result_dataset("capture.csv", b"sample [us],V(out) [V]\n0,0\n1,1\n")
            .expect("time unit identifies transient data");
        assert_eq!(transient.analysis_type, AnalysisType::Transient);

        let ac = parse_result_dataset("capture.csv", b"axis [MHz],V(out) [V]\n1,1\n2,0.5\n")
            .expect("frequency unit identifies AC data");
        assert_eq!(ac.analysis_type, AnalysisType::Ac);
    }

    #[test]
    fn infers_dc_sweep_and_accepts_strictly_decreasing_coordinate() {
        let parsed = parse_result_dataset("sweep.csv", b"V1 [V],V(out) [V]\n5,4.5\n2.5,2.2\n0,0\n")
            .expect("decreasing DC sweep remains ordered");
        assert_eq!(parsed.analysis_type, AnalysisType::DcSweep);
        assert_eq!(parsed.waveforms[0].x.as_slice(), &[5.0, 2.5, 0.0]);
    }

    #[test]
    fn preserves_case_sensitive_siemens_units() {
        let parsed = parse_result_dataset("conductance.csv", b"bias [V],g(out) [mS]\n0,1\n1,2\n")
            .expect("mS is conductance, not milliseconds");
        assert_eq!(parsed.analysis_type, AnalysisType::DcSweep);
        assert_eq!(parsed.waveforms[0].y.as_slice(), &[1e-3, 2e-3]);
    }

    #[test]
    fn rejects_duplicate_headers_unknown_units_nonfinite_values_and_reversals() {
        for (source, expected) in [
            (
                b"time [s],V(out) [V],v(OUT) [mV]\n0,0,0\n1,1,1\n".as_slice(),
                "repeats",
            ),
            (
                b"time [fortnight],V(out) [V]\n0,0\n1,1\n".as_slice(),
                "not a recognized engineering unit",
            ),
            (
                b"time [s],V(out) [V]\n0,NaN\n1,1\n".as_slice(),
                "must be finite",
            ),
            (
                b"time [s],V(out) [V]\n0,0\n2,1\n1,2\n".as_slice(),
                "reverses coordinate ordering",
            ),
        ] {
            let error = parse_result_dataset("bad.csv", source).expect_err("invalid import");
            assert!(error.contains(expected), "{error:?}");
        }
    }

    #[test]
    fn imported_dataset_is_completed_stable_immutable_and_selected() {
        let mut state = AppState::default();
        let baseline =
            crate::workbench::lifecycle::project_lifecycle::snapshot(&state).expect("baseline");
        crate::workbench::lifecycle::project_lifecycle::accept_loaded_project(
            &mut state, baseline, None,
        );
        assert!(
            !crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state),
            "accepted baseline starts clean"
        );
        apply_imported_result_dataset(
            &mut state,
            "external.csv",
            b"time [s],V(out) [V]\n0,0\n1,1\n",
        )
        .expect("import succeeds");

        let run = state
            .simulation
            .active_run()
            .expect("imported run selected");
        assert_eq!(run.lifecycle, SimulationRunLifecycle::Completed);
        assert!(run.job_id.is_none());
        assert!(run.execution_target.is_none());
        assert!(matches!(
            run.provenance(),
            Some(SimulationRunProvenance::LegacyUnattributed)
        ));
        assert_eq!(run.analyses.len(), 1);
        assert_eq!(
            state
                .simulation
                .active_analysis()
                .map(|analysis| analysis.analysis_type),
            Some(AnalysisType::Transient)
        );
        assert_ne!(run.run_id.to_string(), run.dataset_id.to_string());
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Results
        );
        assert_eq!(
            state
                .workbench
                .documents
                .active(crate::workbench::state::Workspace::Results),
            Some(&crate::workbench::state::WorkspaceDocumentId::ResultDataset(run.dataset_id))
        );
        assert!(
            crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state),
            "retaining external evidence must dirty canonical result history"
        );
    }
}
