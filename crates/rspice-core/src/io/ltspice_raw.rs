//! SPICE .raw file reader.
//!
//! Parses simulation output files written by RSpice or by another simulator —
//! LTspice and ngspice both emit this format. Supports ASCII and binary
//! (IEEE 754) payloads. [`super::raw_export`] writes the same format.
//!
//! ## Format Overview
//!
//! .raw files contain:
//! 1. ASCII header with metadata (title, date, simulation type, variables)
//! 2. Binary or ASCII data section with simulation values
//!
//! The header uses key-value pairs terminated by "Values:" or "Binary:"

use std::io::{BufRead, BufReader, Cursor, Read};
use std::path::Path;
use thiserror::Error;

use crate::resource::{
    ResourceKind, ResourceLimitError, ResourceLimits, ResourceReadError, read_bytes_limited,
    read_file_bytes_limited,
};

/// Errors that can occur when parsing .raw files
#[derive(Debug, Error)]
pub enum RawParseError {
    #[error(transparent)]
    ResourceLimit(#[from] ResourceLimitError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid header: {0}")]
    InvalidHeader(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Data parse error: {0}")]
    DataError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

impl From<ResourceReadError> for RawParseError {
    fn from(error: ResourceReadError) -> Self {
        match error {
            ResourceReadError::Io(error) => Self::Io(error),
            ResourceReadError::ResourceLimit(error) => Self::ResourceLimit(error),
        }
    }
}

/// Raw file header metadata
#[derive(Debug, Clone)]
pub struct RawFileHeader {
    /// Simulation title
    pub title: String,
    /// Date string
    pub date: String,
    /// Plot name (e.g., "Transient Analysis")
    pub plotname: String,
    /// Simulation flags
    pub flags: Vec<String>,
    /// Number of variables (columns)
    pub no_variables: usize,
    /// Number of data points
    pub no_points: usize,
    /// Command that produced this data
    pub command: String,
    /// Whether data is binary (vs ASCII)
    pub is_binary: bool,
    /// Whether data is complex (AC analysis)
    pub is_complex: bool,
    /// Whether data uses double precision (vs single)
    pub is_double: bool,
}

impl Default for RawFileHeader {
    fn default() -> Self {
        Self {
            title: String::new(),
            date: String::new(),
            plotname: String::new(),
            flags: Vec::new(),
            no_variables: 0,
            no_points: 0,
            command: String::new(),
            is_binary: false,
            is_complex: false,
            is_double: true,
        }
    }
}

/// Variable definition from .raw file
#[derive(Debug, Clone)]
pub struct RawVariable {
    /// Variable index
    pub index: usize,
    /// Variable name (e.g., "time", "V(out)", "I(R1)")
    pub name: String,
    /// Variable type (e.g., "time", "voltage", "current")
    pub var_type: String,
}

/// Parsed waveform data
#[derive(Debug, Clone)]
pub struct RawWaveform {
    /// Variable name
    pub name: String,
    /// X-axis data (typically time or frequency)
    pub x: Vec<f64>,
    /// Y-axis data (real part for complex)
    pub y: Vec<f64>,
    /// Imaginary part for complex (AC analysis)
    pub y_imag: Option<Vec<f64>>,
}

/// Complete parsed .raw file data
#[derive(Debug)]
pub struct RawWaveformData {
    /// File header
    pub header: RawFileHeader,
    /// Variable definitions
    pub variables: Vec<RawVariable>,
    /// Parsed waveforms (one per variable)
    pub waveforms: Vec<RawWaveform>,
}

/// Every plot one .raw file declares, in file order.
///
/// A rawfile is a sequence of plots: each `Plotname:` line opens a block with
/// its own variables and its own point count, and the block's data ends where
/// its declared `No. Points` says it does. Files that carry one plot — which
/// is what a single analysis writes — parse into a single-entry `plots`, so
/// [`parse_raw_file`] and its siblings remain the reader for that case and
/// return the first plot unchanged.
#[derive(Debug)]
pub struct RawFile {
    /// Plots in the order the file declares them; never empty.
    pub plots: Vec<RawWaveformData>,
}

/// Parse a .raw file from a file path
pub fn parse_raw_file(path: &Path) -> Result<RawWaveformData, RawParseError> {
    parse_raw_file_with_limits(path, ResourceLimits::default())
}

/// Parse a `.raw` file with explicit external-data and retained-result limits.
pub fn parse_raw_file_with_limits(
    path: &Path,
    resource_limits: ResourceLimits,
) -> Result<RawWaveformData, RawParseError> {
    let bytes = read_file_bytes_limited(
        path,
        ResourceKind::ExternalDataBytes,
        resource_limits.max_external_data_bytes,
    )?;
    parse_raw_bytes(&bytes, resource_limits)
}

/// Parse a .raw file from a reader
pub fn parse_raw_reader<R: Read>(reader: &mut R) -> Result<RawWaveformData, RawParseError> {
    parse_raw_reader_with_limits(reader, ResourceLimits::default())
}

/// Parse `.raw` data from a reader with explicit resource limits.
pub fn parse_raw_reader_with_limits<R: Read>(
    reader: &mut R,
    resource_limits: ResourceLimits,
) -> Result<RawWaveformData, RawParseError> {
    let bytes = read_bytes_limited(
        reader,
        ResourceKind::ExternalDataBytes,
        resource_limits.max_external_data_bytes,
    )?;
    parse_raw_bytes(&bytes, resource_limits)
}

/// Parse every plot in a `.raw` file with explicit resource limits.
///
/// Each plot is bounded by its own declared `No. Points`, so a file whose
/// analysis plot is followed by further plots yields all of them. Limits are
/// enforced per plot, before that plot's storage is allocated.
pub fn parse_raw_plots_file_with_limits(
    path: &Path,
    resource_limits: ResourceLimits,
) -> Result<RawFile, RawParseError> {
    let bytes = read_file_bytes_limited(
        path,
        ResourceKind::ExternalDataBytes,
        resource_limits.max_external_data_bytes,
    )?;
    parse_raw_plot_bytes(&bytes, resource_limits)
}

/// Parse every plot in `.raw` data from a reader with explicit resource limits.
///
/// The file-backed sibling is [`parse_raw_plots_file_with_limits`].
pub fn parse_raw_plots_reader_with_limits<R: Read>(
    reader: &mut R,
    resource_limits: ResourceLimits,
) -> Result<RawFile, RawParseError> {
    let bytes = read_bytes_limited(
        reader,
        ResourceKind::ExternalDataBytes,
        resource_limits.max_external_data_bytes,
    )?;
    parse_raw_plot_bytes(&bytes, resource_limits)
}

fn parse_raw_bytes(
    bytes: &[u8],
    resource_limits: ResourceLimits,
) -> Result<RawWaveformData, RawParseError> {
    // The single-plot readers answer for the first plot only, so they parse
    // that plot and check the boundary behind it rather than paying for plots
    // the caller will not see. A file that continues with another plot is
    // still a well-formed file for them; anything else is corruption at the
    // end of plot 1 and is refused as such.
    let (plot, consumed) = parse_plot(bytes, resource_limits)?;
    let remainder = bytes.get(consumed..).unwrap_or_default();
    if !remainder_opens_plot(remainder) {
        return Err(trailing_bytes_error(1, remainder.len()));
    }
    Ok(plot)
}

fn parse_raw_plot_bytes(
    bytes: &[u8],
    resource_limits: ResourceLimits,
) -> Result<RawFile, RawParseError> {
    let mut plots = Vec::new();
    let mut offset = 0usize;
    loop {
        let plot_number = plots.len().saturating_add(1);
        let remaining = bytes.get(offset..).ok_or_else(|| {
            RawParseError::DataError("raw plot offset exceeds the input length".to_string())
        })?;
        let (plot, consumed) =
            parse_plot(remaining, resource_limits).map_err(|error| in_plot(plot_number, error))?;
        plots.try_reserve(1).map_err(|error| {
            RawParseError::DataError(format!("unable to retain raw plot {plot_number}: {error}"))
        })?;
        plots.push(plot);
        offset = offset.saturating_add(consumed);

        let remainder = bytes.get(offset..).unwrap_or_default();
        if is_blank(remainder) {
            break;
        }
        if !starts_plot_header(remainder) {
            return Err(trailing_bytes_error(plot_number, remainder.len()));
        }
    }
    Ok(RawFile { plots })
}

/// Parse one plot block, returning it and the byte length it consumed.
fn parse_plot(
    bytes: &[u8],
    resource_limits: ResourceLimits,
) -> Result<(RawWaveformData, usize), RawParseError> {
    let mut reader = BufReader::new(Cursor::new(bytes));
    // Parse ASCII header
    let (mut header, variables, data_offset) = parse_header(&mut reader, resource_limits)?;
    let data_offset = usize::try_from(data_offset).map_err(|_| {
        RawParseError::DataError("raw data offset exceeds this platform".to_string())
    })?;
    let data_bytes = bytes.get(data_offset..).ok_or_else(|| {
        RawParseError::DataError("raw data offset exceeds the input length".to_string())
    })?;

    // Parse data
    let (waveforms, actual_points, data_length) = if header.is_binary {
        parse_binary_data(data_bytes, &header, &variables, resource_limits)?
    } else {
        parse_ascii_data(data_bytes, &header, &variables, resource_limits)?
    };
    if header.no_points == 0 {
        header.no_points = actual_points;
    }

    Ok((
        RawWaveformData {
            header,
            variables,
            waveforms,
        },
        data_offset.saturating_add(data_length),
    ))
}

/// Header keys that can open a plot block.
///
/// ngspice's own reader keys on these lines and aborts the load on any line it
/// does not recognise, so they are exactly what "the next plot starts here"
/// means to every reader of the format.
const PLOT_HEADER_KEYS: [&str; 8] = [
    "plotname:",
    "title:",
    "date:",
    "flags:",
    "command:",
    "no. variables:",
    "no. points:",
    "variables:",
];

/// Longest [`PLOT_HEADER_KEYS`] entry, so the probe never decodes a plot's
/// worth of trailing binary to answer a yes/no question.
const PLOT_HEADER_PROBE_BYTES: usize = 16;

fn is_blank(bytes: &[u8]) -> bool {
    bytes.iter().all(u8::is_ascii_whitespace)
}

/// Whether `bytes` open another plot, ignoring leading blank space.
fn starts_plot_header(bytes: &[u8]) -> bool {
    let Some(start) = bytes.iter().position(|byte| !byte.is_ascii_whitespace()) else {
        return false;
    };
    let rest = bytes.get(start..).unwrap_or_default();
    let probe = rest
        .get(..rest.len().min(PLOT_HEADER_PROBE_BYTES))
        .unwrap_or(rest);
    let text = String::from_utf8_lossy(probe).to_ascii_lowercase();
    PLOT_HEADER_KEYS.iter().any(|key| text.starts_with(key))
}

/// Whether nothing but blank space or another plot follows a plot's data.
fn remainder_opens_plot(bytes: &[u8]) -> bool {
    is_blank(bytes) || starts_plot_header(bytes)
}

fn trailing_bytes_error(plot_number: usize, trailing: usize) -> RawParseError {
    RawParseError::DataError(format!(
        "plot {plot_number} ends before {trailing} remaining byte(s) that do not open another \
         plot; a plot begins with a header line such as 'Plotname:'"
    ))
}

/// Name the plot a failure happened in, keeping the failure's own category.
fn in_plot(plot_number: usize, error: RawParseError) -> RawParseError {
    match error {
        RawParseError::InvalidHeader(message) => {
            RawParseError::InvalidHeader(format!("plot {plot_number}: {message}"))
        }
        RawParseError::UnsupportedFormat(message) => {
            RawParseError::UnsupportedFormat(format!("plot {plot_number}: {message}"))
        }
        RawParseError::DataError(message) => {
            RawParseError::DataError(format!("plot {plot_number}: {message}"))
        }
        RawParseError::MissingField(field) => {
            RawParseError::MissingField(format!("{field} in plot {plot_number}"))
        }
        error @ (RawParseError::ResourceLimit(_) | RawParseError::Io(_)) => error,
    }
}

/// Parse the ASCII header section
fn parse_header<R: BufRead>(
    reader: &mut R,
    resource_limits: ResourceLimits,
) -> Result<(RawFileHeader, Vec<RawVariable>, u64), RawParseError> {
    let mut header = RawFileHeader::default();
    let mut variables = Vec::new();
    let mut in_variables = false;
    let mut bytes_read: u64 = 0;
    let mut saw_no_variables = false;
    let mut saw_no_points = false;

    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            return Err(RawParseError::InvalidHeader(
                "Unexpected end of file".to_string(),
            ));
        }
        bytes_read += n as u64;

        let line = line.trim();

        // Check for data section start
        if line == "Binary:" {
            header.is_binary = true;
            break;
        } else if line == "Values:" {
            header.is_binary = false;
            break;
        }

        if in_variables && !line.is_empty() {
            // Parse variable line: "index name type"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                return Err(RawParseError::InvalidHeader(format!(
                    "Malformed variable definition: {}",
                    line
                )));
            }
            let index = parts[0].parse::<usize>().map_err(|_| {
                RawParseError::InvalidHeader(format!(
                    "Invalid variable index '{}' in '{}'",
                    parts[0], line
                ))
            })?;
            ResourceLimitError::ensure(
                ResourceKind::ExternalDataValues,
                variables.len().saturating_add(1),
                resource_limits.max_external_data_values,
            )?;
            variables.try_reserve(1).map_err(|error| {
                RawParseError::DataError(format!(
                    "unable to allocate raw variable metadata: {error}"
                ))
            })?;
            variables.push(RawVariable {
                index,
                name: parts[1].to_string(),
                var_type: parts[2].to_string(),
            });
            continue;
        }

        // Parse header fields
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim();

            match key.as_str() {
                "title" => header.title = value.to_string(),
                "date" => header.date = value.to_string(),
                "plotname" => header.plotname = value.to_string(),
                "command" => header.command = value.to_string(),
                "flags" => {
                    header.flags = value.split_whitespace().map(|s| s.to_string()).collect();
                    header.is_complex = header
                        .flags
                        .iter()
                        .any(|f| f.eq_ignore_ascii_case("complex"));
                    header.is_double = header
                        .flags
                        .iter()
                        .any(|f| f.eq_ignore_ascii_case("double"));
                }
                "no. variables" => {
                    saw_no_variables = true;
                    header.no_variables = value.parse().map_err(|_| {
                        RawParseError::InvalidHeader(format!("Invalid no. variables: {}", value))
                    })?;
                    ResourceLimitError::ensure(
                        ResourceKind::ExternalDataValues,
                        header.no_variables,
                        resource_limits.max_external_data_values,
                    )?;
                }
                "no. points" => {
                    saw_no_points = true;
                    header.no_points = value.parse().map_err(|_| {
                        RawParseError::InvalidHeader(format!("Invalid no. points: {}", value))
                    })?;
                    if header.no_points > 0 {
                        ensure_waveform_dimensions(&header, header.no_points, resource_limits)?;
                    }
                }
                "variables" => {
                    in_variables = true;
                }
                _ => {}
            }
        }
    }

    if !saw_no_variables || header.no_variables == 0 {
        return Err(RawParseError::MissingField("No. Variables".to_string()));
    }
    if !saw_no_points {
        return Err(RawParseError::MissingField("No. Points".to_string()));
    }
    if variables.len() != header.no_variables {
        return Err(RawParseError::InvalidHeader(format!(
            "No. Variables declares {} variable(s), but {} definition(s) were listed",
            header.no_variables,
            variables.len()
        )));
    }
    for (expected_index, variable) in variables.iter().enumerate() {
        if variable.index != expected_index {
            return Err(RawParseError::InvalidHeader(format!(
                "Variable '{}' has index {}, expected {}",
                variable.name, variable.index, expected_index
            )));
        }
    }

    Ok((header, variables, bytes_read))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryEncoding {
    RealAllF64,
    RealMixedAxisF64RestF32,
    ComplexAllF64,
    ComplexMixedAxisF64RestF32,
}

impl BinaryEncoding {
    fn row_size_bytes(self, num_vars: usize) -> Option<usize> {
        match self {
            Self::RealAllF64 => num_vars.checked_mul(8),
            Self::RealMixedAxisF64RestF32 => num_vars
                .saturating_sub(1)
                .checked_mul(4)
                .and_then(|bytes| bytes.checked_add(8)),
            Self::ComplexAllF64 => num_vars.checked_mul(16),
            Self::ComplexMixedAxisF64RestF32 => num_vars
                .saturating_sub(1)
                .checked_mul(8)
                .and_then(|bytes| bytes.checked_add(16)),
        }
    }

    fn is_all_f64(self) -> bool {
        matches!(self, Self::RealAllF64 | Self::ComplexAllF64)
    }
}

/// How well one candidate encoding explains the bytes that follow it.
///
/// A plot's payload ends where its declared `No. Points` says it does, and
/// what sits immediately behind that boundary is the evidence that says which
/// encoding was in force: end of file for the last plot, another plot's header
/// for every earlier one. Only when neither holds — a truncated or corrupt
/// tail — does the `Flags:` preference decide, and the caller then refuses the
/// trailing bytes anyway.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EncodingFit {
    /// Bytes remain that do not open another plot.
    Trailing,
    /// The payload ends exactly where another plot's header begins.
    NextPlot,
    /// The payload ends exactly at the end of the input.
    Exact,
}

fn detect_binary_encoding(
    payload: &[u8],
    header: &RawFileHeader,
    num_vars: usize,
) -> Result<(BinaryEncoding, usize, usize), RawParseError> {
    let payload_len = payload.len();
    let candidates = if header.is_complex {
        [
            BinaryEncoding::ComplexAllF64,
            BinaryEncoding::ComplexMixedAxisF64RestF32,
        ]
    } else {
        [
            BinaryEncoding::RealAllF64,
            BinaryEncoding::RealMixedAxisF64RestF32,
        ]
    };

    let mut matches = Vec::new();
    for encoding in candidates {
        let Some(row_size) = encoding.row_size_bytes(num_vars) else {
            continue;
        };
        if row_size == 0 {
            continue;
        }

        if header.no_points > 0 {
            let Some(consumed) = row_size.checked_mul(header.no_points) else {
                continue;
            };
            if consumed > payload_len {
                continue;
            }
            let tail = payload.get(consumed..).unwrap_or_default();
            let fit = if consumed == payload_len {
                EncodingFit::Exact
            } else if starts_plot_header(tail) {
                EncodingFit::NextPlot
            } else {
                EncodingFit::Trailing
            };
            matches.push((encoding, header.no_points, consumed, fit));
        } else if payload_len.is_multiple_of(row_size) {
            matches.push((
                encoding,
                payload_len / row_size,
                payload_len,
                EncodingFit::Exact,
            ));
        }
    }

    // The boundary decides first; the `Flags:` preference only breaks a tie
    // between candidates the boundary cannot separate. Exactly one of the two
    // candidates agrees with `is_double`, so the winner is never arbitrary.
    let chosen = matches
        .into_iter()
        .max_by_key(|(encoding, _, _, fit)| (*fit, encoding.is_all_f64() == header.is_double))
        .ok_or_else(|| {
            RawParseError::DataError(format!(
                "Binary payload length {} does not match any supported encoding for {} variable(s) and {} point(s)",
                payload_len, num_vars, header.no_points
            ))
        })?;

    Ok((chosen.0, chosen.1, chosen.2))
}

fn checked_product(values: &[usize]) -> usize {
    values
        .iter()
        .copied()
        .try_fold(1usize, usize::checked_mul)
        .unwrap_or(usize::MAX)
}

fn ensure_waveform_dimensions(
    header: &RawFileHeader,
    num_points: usize,
    resource_limits: ResourceLimits,
) -> Result<(), RawParseError> {
    ResourceLimitError::ensure(
        ResourceKind::ExternalDataValues,
        checked_product(&[
            header.no_variables,
            num_points,
            if header.is_complex { 2 } else { 1 },
        ]),
        resource_limits.max_external_data_values,
    )?;
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        checked_product(&[
            header.no_variables,
            num_points,
            if header.is_complex { 3 } else { 2 },
        ]),
        resource_limits.max_result_values,
    )?;
    Ok(())
}

fn value_buffer(capacity: usize) -> Result<Vec<f64>, RawParseError> {
    let mut values = Vec::new();
    values.try_reserve_exact(capacity).map_err(|error| {
        RawParseError::DataError(format!(
            "unable to allocate storage for {capacity} raw values: {error}"
        ))
    })?;
    Ok(values)
}

fn data_columns(count: usize, capacity: usize) -> Result<Vec<Vec<f64>>, RawParseError> {
    let mut columns = Vec::new();
    columns.try_reserve_exact(count).map_err(|error| {
        RawParseError::DataError(format!(
            "unable to allocate {count} raw data columns: {error}"
        ))
    })?;
    for _ in 0..count {
        columns.push(value_buffer(capacity)?);
    }
    Ok(columns)
}

fn clone_values(values: &[f64]) -> Result<Vec<f64>, RawParseError> {
    let mut cloned = value_buffer(values.len())?;
    cloned.extend_from_slice(values);
    Ok(cloned)
}

fn build_waveforms(
    data: Vec<Vec<f64>>,
    data_imag: Option<Vec<Vec<f64>>>,
    variables: &[RawVariable],
) -> Result<Vec<RawWaveform>, RawParseError> {
    let mut x_data = data
        .first()
        .map(|values| clone_values(values))
        .transpose()?
        .unwrap_or_default();
    let mut waveforms = Vec::new();
    waveforms
        .try_reserve_exact(variables.len())
        .map_err(|error| {
            RawParseError::DataError(format!(
                "unable to allocate {} raw waveforms: {error}",
                variables.len()
            ))
        })?;
    let mut imaginary_columns = data_imag.map(Vec::into_iter);

    let last_variable = variables.len().saturating_sub(1);
    for (index, (var, values)) in variables.iter().zip(data).enumerate() {
        let x = if index == last_variable {
            std::mem::take(&mut x_data)
        } else {
            clone_values(&x_data)?
        };
        waveforms.push(RawWaveform {
            name: var.name.clone(),
            x,
            y: values,
            y_imag: imaginary_columns.as_mut().and_then(Iterator::next),
        });
    }

    Ok(waveforms)
}

/// Parse binary data section (IEEE 754 format)
fn parse_binary_data(
    payload: &[u8],
    header: &RawFileHeader,
    variables: &[RawVariable],
    resource_limits: ResourceLimits,
) -> Result<(Vec<RawWaveform>, usize, usize), RawParseError> {
    let num_vars = header.no_variables;
    let (encoding, num_points, consumed) = detect_binary_encoding(payload, header, num_vars)?;
    ensure_waveform_dimensions(header, num_points, resource_limits)?;
    let mut cursor = Cursor::new(payload);

    // Initialize storage
    let mut data = data_columns(num_vars, num_points)?;
    let mut data_imag = header
        .is_complex
        .then(|| data_columns(num_vars, num_points))
        .transpose()?;

    // Read all data points
    for _ in 0..num_points {
        for var_idx in 0..num_vars {
            let real_value = match encoding {
                BinaryEncoding::RealAllF64 | BinaryEncoding::ComplexAllF64 => {
                    read_f64_le(&mut cursor)?
                }
                BinaryEncoding::RealMixedAxisF64RestF32
                | BinaryEncoding::ComplexMixedAxisF64RestF32 => {
                    if var_idx == 0 {
                        read_f64_le(&mut cursor)?
                    } else {
                        read_f32_le(&mut cursor)? as f64
                    }
                }
            };
            ensure_finite_binary_value(real_value)?;

            data[var_idx].push(real_value);

            if let Some(imag) = data_imag.as_mut() {
                let imag_value = match encoding {
                    BinaryEncoding::ComplexAllF64 => read_f64_le(&mut cursor)?,
                    BinaryEncoding::ComplexMixedAxisF64RestF32 => {
                        if var_idx == 0 {
                            read_f64_le(&mut cursor)?
                        } else {
                            read_f32_le(&mut cursor)? as f64
                        }
                    }
                    BinaryEncoding::RealAllF64 | BinaryEncoding::RealMixedAxisF64RestF32 => {
                        unreachable!("imaginary storage requested for real binary encoding")
                    }
                };
                ensure_finite_binary_value(imag_value)?;
                imag[var_idx].push(imag_value);
            }
        }
    }

    Ok((
        build_waveforms(data, data_imag, variables)?,
        num_points,
        consumed,
    ))
}

/// Collect up to `wanted` non-blank data lines, and say where they end.
///
/// A plot's data ends after the rows its own `No. Points` declares, so the
/// walk stops there and leaves the bytes behind it — blank space, or the next
/// plot's header — for the caller to account for. `wanted` of `None` consumes
/// everything, which is what an undeclared point count means.
fn collect_ascii_lines(
    payload: &[u8],
    wanted: Option<usize>,
    collected: &mut Vec<String>,
    offset: &mut usize,
) -> Result<(), RawParseError> {
    while wanted.is_none_or(|wanted| collected.len() < wanted) {
        let Some(rest) = payload.get(*offset..) else {
            break;
        };
        if rest.is_empty() {
            break;
        }
        let end = rest
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(rest.len(), |index| index.saturating_add(1));
        let raw = rest.get(..end).unwrap_or(rest);
        *offset = offset.saturating_add(end);
        let text = std::str::from_utf8(raw).map_err(|error| {
            RawParseError::DataError(format!("ASCII raw data is not valid UTF-8: {error}"))
        })?;
        let trimmed = text.trim();
        if trimmed.is_empty() {
            continue;
        }
        collected.try_reserve(1).map_err(|error| {
            RawParseError::DataError(format!("unable to retain raw ASCII rows: {error}"))
        })?;
        collected.push(trimmed.to_string());
    }
    Ok(())
}

/// Parse ASCII data section
fn parse_ascii_data(
    payload: &[u8],
    header: &RawFileHeader,
    variables: &[RawVariable],
    resource_limits: ResourceLimits,
) -> Result<(Vec<RawWaveform>, usize, usize), RawParseError> {
    let num_vars = header.no_variables;
    let mut lines = Vec::new();
    let mut consumed = 0usize;
    // The first row decides the layout, and the layout decides how many rows
    // this plot owns, so it is read before the rest of the walk is bounded.
    collect_ascii_lines(payload, Some(1), &mut lines, &mut consumed)?;
    let row_oriented = lines
        .first()
        .map(|line| line.split_whitespace().count() > num_vars)
        .unwrap_or(false);
    let wanted = if header.no_points == 0 {
        None
    } else if row_oriented {
        Some(header.no_points)
    } else {
        Some(header.no_points.checked_mul(num_vars).ok_or_else(|| {
            RawParseError::DataError(
                "raw ASCII point and variable counts overflow this platform".to_string(),
            )
        })?)
    };
    collect_ascii_lines(payload, wanted, &mut lines, &mut consumed)?;

    let point_count = if row_oriented {
        if header.no_points > 0 && lines.len() != header.no_points {
            return Err(RawParseError::DataError(format!(
                "No. Points declares {} point(s), but ASCII data contains {} row(s)",
                header.no_points,
                lines.len()
            )));
        }
        header.no_points.max(lines.len())
    } else {
        let expected_rows = header.no_points.checked_mul(num_vars).ok_or_else(|| {
            RawParseError::DataError(
                "raw ASCII point and variable counts overflow this platform".to_string(),
            )
        })?;
        if header.no_points > 0 && lines.len() != expected_rows {
            return Err(RawParseError::DataError(format!(
                "No. Points declares {} point(s), but ASCII data contains {} value row(s) for {} variable(s)",
                header.no_points,
                lines.len(),
                num_vars
            )));
        }
        if !lines.len().is_multiple_of(num_vars) {
            return Err(RawParseError::DataError(format!(
                "ASCII data row count {} is not divisible by variable count {}",
                lines.len(),
                num_vars
            )));
        }
        if header.no_points > 0 {
            header.no_points
        } else {
            lines.len() / num_vars
        }
    };
    if point_count == 0 {
        return Err(RawParseError::DataError(
            "ASCII raw data contains no points".to_string(),
        ));
    }
    ensure_waveform_dimensions(header, point_count, resource_limits)?;
    let mut data = data_columns(num_vars, point_count)?;

    if row_oriented {
        for (point_idx, line) in lines.iter().enumerate().take(point_count) {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let expected_columns = num_vars.saturating_add(1);
            if parts.len() != expected_columns {
                return Err(RawParseError::DataError(format!(
                    "ASCII row {} has {} column(s), expected {}",
                    point_idx,
                    parts.len(),
                    expected_columns
                )));
            }
            let row_index = parts[0].parse::<usize>().map_err(|_| {
                RawParseError::DataError(format!("Invalid ASCII row index: {}", parts[0]))
            })?;
            if row_index != point_idx {
                return Err(RawParseError::DataError(format!(
                    "ASCII row index {} does not match expected point {}",
                    row_index, point_idx
                )));
            }

            for var_idx in 0..num_vars {
                let value_str = parts[var_idx + 1];
                let value = parse_ascii_raw_value(value_str)?;
                data[var_idx].push(value);
            }
        }
    } else {
        for point_idx in 0..point_count {
            for (var_idx, column) in data.iter_mut().enumerate().take(num_vars) {
                let line_idx = point_idx
                    .checked_mul(num_vars)
                    .and_then(|index| index.checked_add(var_idx))
                    .ok_or_else(|| {
                        RawParseError::DataError(
                            "raw ASCII row index overflowed this platform".to_string(),
                        )
                    })?;
                let line = &lines[line_idx];
                let parts = line.split_whitespace().collect::<Vec<_>>();
                let value_str = if var_idx == 0 {
                    match parts.as_slice() {
                        [value] => *value,
                        [index, value] => {
                            let row_index = index.parse::<usize>().map_err(|_| {
                                RawParseError::DataError(format!(
                                    "Invalid ASCII row index: {}",
                                    index
                                ))
                            })?;
                            if row_index != point_idx {
                                return Err(RawParseError::DataError(format!(
                                    "ASCII row index {} does not match expected point {}",
                                    row_index, point_idx
                                )));
                            }
                            *value
                        }
                        _ => {
                            return Err(RawParseError::DataError(format!(
                                "ASCII value row {} has {} column(s), expected 1 or 2",
                                line_idx,
                                parts.len()
                            )));
                        }
                    }
                } else {
                    match parts.as_slice() {
                        [value] => *value,
                        _ => {
                            return Err(RawParseError::DataError(format!(
                                "ASCII value row {} has {} column(s), expected 1",
                                line_idx,
                                parts.len()
                            )));
                        }
                    }
                };

                let value = parse_ascii_raw_value(value_str)?;
                column.push(value);
            }
        }
    }

    let actual_points = data.first().map(Vec::len).unwrap_or(0);
    for (var_idx, column) in data.iter().enumerate() {
        if column.len() != actual_points {
            return Err(RawParseError::DataError(format!(
                "ASCII raw variable '{}' has {} point(s), expected {}",
                variables
                    .get(var_idx)
                    .map(|var| var.name.as_str())
                    .unwrap_or("<unknown>"),
                column.len(),
                actual_points
            )));
        }
    }
    Ok((
        build_waveforms(data, None, variables)?,
        actual_points,
        consumed,
    ))
}

fn ensure_finite_binary_value(value: f64) -> Result<(), RawParseError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(RawParseError::DataError(format!(
            "Non-finite binary raw value: {value}"
        )))
    }
}

fn parse_ascii_raw_value(value_str: &str) -> Result<f64, RawParseError> {
    let value: f64 = value_str
        .parse()
        .map_err(|_| RawParseError::DataError(format!("Invalid value: {}", value_str)))?;
    if !value.is_finite() {
        return Err(RawParseError::DataError(format!(
            "Non-finite value: {}",
            value_str
        )));
    }
    Ok(value)
}

/// Read a little-endian f64
fn read_f64_le<R: Read>(reader: &mut R) -> Result<f64, RawParseError> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(f64::from_le_bytes(buf))
}

/// Read a little-endian f32
fn read_f32_le<R: Read>(reader: &mut R) -> Result<f32, RawParseError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_raw(input: &str) -> Result<RawWaveformData, RawParseError> {
        let mut reader = Cursor::new(input.as_bytes().to_vec());
        parse_raw_reader(&mut reader)
    }

    #[test]
    fn ascii_raw_rejects_variable_count_mismatch() {
        let err = parse_raw(
            "Title: t\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\nValues:\n0 0.0 1.0\n",
        )
        .expect_err("declared/listed variable count mismatch must reject");

        assert!(
            err.to_string().contains("No. Variables") || err.to_string().contains("variable"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn ascii_raw_rejects_short_declared_data() {
        let err = parse_raw(
            "Title: t\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0.0 1.0\n",
        )
        .expect_err("short data against No. Points must reject");

        assert!(
            err.to_string().contains("No. Points") || err.to_string().contains("point"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn ascii_raw_rejects_extra_columns() {
        let err = parse_raw(
            "Title: t\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0.0 1.0 2.0\n",
        )
        .expect_err("extra ASCII data columns must reject");

        assert!(
            err.to_string().contains("column") || err.to_string().contains("value"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn ascii_raw_rejects_nonfinite_values() {
        let err = parse_raw(
            "Title: t\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0.0 NaN\n",
        )
        .expect_err("non-finite ASCII raw values must reject");

        assert!(
            err.to_string().contains("NaN") || err.to_string().contains("finite"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn ascii_raw_accepts_colon_in_variable_name() {
        let parsed = parse_raw(
            "Title: t\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(n:out) voltage\nValues:\n0 0.0 1.0\n",
        )
        .expect("colon inside variable names must not be parsed as a header field");

        assert_eq!(parsed.variables[1].name, "V(n:out)");
        assert_eq!(parsed.waveforms[1].y, vec![1.0]);
    }

    #[test]
    fn raw_reader_enforces_byte_limit_before_parsing() {
        let source = b"Title: bounded\n";
        let limits = ResourceLimits {
            max_external_data_bytes: source.len() - 1,
            ..Default::default()
        };
        let mut reader = Cursor::new(source);

        let error = parse_raw_reader_with_limits(&mut reader, limits)
            .expect_err("byte limit must reject the source");

        assert!(matches!(
            error,
            RawParseError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ExternalDataBytes,
                requested,
                limit,
            }) if requested == source.len() && limit == source.len() - 1
        ));
    }

    #[test]
    fn declared_raw_dimensions_enforce_retained_result_limit() {
        let source = "Title: limited\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0.0 1.0\n";
        let limits = ResourceLimits {
            max_result_values: 3,
            ..Default::default()
        };
        let mut reader = Cursor::new(source.as_bytes());

        let error = parse_raw_reader_with_limits(&mut reader, limits)
            .expect_err("two waveforms retain four scalar values");

        assert!(matches!(
            error,
            RawParseError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ResultValues,
                requested: 4,
                limit: 3,
            })
        ));
    }

    /// The exact ASCII bytes the CLI's transient rawfile writer emits for a
    /// two-point run: header keys in order, tab-separated variable
    /// declarations, one indexed row per point. A file written before this
    /// reader learned about multi-plot files still means exactly this.
    const LEGACY_SINGLE_PLOT_ASCII: &str = concat!(
        "Title: Transient Analysis\n",
        "Date: Generated by RSpice\n",
        "Plotname: Transient Analysis\n",
        "Flags: real\n",
        "No. Variables: 3\n",
        "No. Points: 2\n",
        "Variables:\n",
        "\t0\ttime\ttime\n",
        "\t1\tV(out)\tvoltage\n",
        "\t2\tD(d)\tdigital\n",
        "Values:\n",
        "0\t0.000000000000000e0\t1.000000000000000e0\t0.000000000000000e0\n",
        "1\t1.000000000000000e-9\t2.500000000000000e0\t1.000000000000000e0\n",
    );

    fn legacy_single_plot_binary() -> Vec<u8> {
        let mut bytes = concat!(
            "Title: Transient Analysis\n",
            "Date: Generated by RSpice\n",
            "Plotname: Transient Analysis\n",
            "Flags: real\n",
            "No. Variables: 3\n",
            "No. Points: 2\n",
            "Variables:\n",
            "\t0\ttime\ttime\n",
            "\t1\tV(out)\tvoltage\n",
            "\t2\tD(d)\tdigital\n",
            "Binary:\n",
        )
        .as_bytes()
        .to_vec();
        for value in [0.0_f64, 1.0, 0.0, 1.0e-9, 2.5, 1.0] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    /// One event plot, in the layout the exporter appends after an analysis
    /// plot: no `Command:` line, a plot name carrying the layout version, and
    /// the node's own irregular times.
    const DIGITAL_EVENT_PLOT_ASCII: &str = concat!(
        "Title: Digital Events (rspice-digital-events/1)\n",
        "Date: Thu Jan 1 00:00:00 1970\n",
        "Plotname: Digital Events (rspice-digital-events/1)\n",
        "Flags: real\n",
        "No. Variables: 2\n",
        "No. Points: 3\n",
        "Variables:\n",
        "\t0\ttime\ttime\n",
        "\t1\tD(d)\tdigital\n",
        "Values:\n",
        "0\t0.00000000000000000e0\t9.00000000000000000e0\n",
        "1\t2.50000000000000000e-10\t1.00000000000000000e0\n",
        "2\t7.00000000000000000e-10\t1.20000000000000000e1\n",
    );

    fn digital_event_plot_binary() -> Vec<u8> {
        let mut bytes = concat!(
            "Plotname: Digital Events (rspice-digital-events/1)\n",
            "Flags: real\n",
            "No. Variables: 2\n",
            "No. Points: 3\n",
            "Variables:\n",
            "\t0\ttime\ttime\n",
            "\t1\tD(d)\tdigital\n",
            "Binary:\n",
        )
        .as_bytes()
        .to_vec();
        for value in [0.0_f64, 9.0, 2.5e-10, 1.0, 7.0e-10, 12.0] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    fn assert_legacy_plot(parsed: &RawWaveformData) {
        assert_eq!(parsed.header.plotname, "Transient Analysis");
        assert_eq!(parsed.header.title, "Transient Analysis");
        assert_eq!(parsed.header.no_variables, 3);
        assert_eq!(parsed.header.no_points, 2);
        assert_eq!(
            parsed
                .variables
                .iter()
                .map(|variable| (variable.name.as_str(), variable.var_type.as_str()))
                .collect::<Vec<_>>(),
            [("time", "time"), ("V(out)", "voltage"), ("D(d)", "digital")]
        );
        assert_eq!(parsed.waveforms[0].y, [0.0, 1.0e-9]);
        assert_eq!(parsed.waveforms[1].y, [1.0, 2.5]);
        assert_eq!(parsed.waveforms[2].y, [0.0, 1.0]);
        assert_eq!(parsed.waveforms[2].x, [0.0, 1.0e-9]);
    }

    fn parse_plots(bytes: Vec<u8>) -> Result<RawFile, RawParseError> {
        parse_raw_plots_reader_with_limits(&mut Cursor::new(bytes), ResourceLimits::default())
    }

    #[test]
    fn a_legacy_single_plot_file_parses_exactly_as_it_always_did() {
        assert_legacy_plot(
            &parse_raw(LEGACY_SINGLE_PLOT_ASCII).expect("legacy ASCII rawfile still parses"),
        );
        assert_legacy_plot(
            &parse_raw_reader(&mut Cursor::new(legacy_single_plot_binary()))
                .expect("legacy binary rawfile still parses"),
        );

        let file = parse_plots(LEGACY_SINGLE_PLOT_ASCII.as_bytes().to_vec())
            .expect("legacy ASCII rawfile is a one-plot file");
        assert_eq!(file.plots.len(), 1);
        assert_legacy_plot(&file.plots[0]);
    }

    #[test]
    fn appended_plots_parse_without_disturbing_the_first_one() {
        let mut ascii = LEGACY_SINGLE_PLOT_ASCII.as_bytes().to_vec();
        ascii.extend_from_slice(DIGITAL_EVENT_PLOT_ASCII.as_bytes());
        let mut binary = legacy_single_plot_binary();
        binary.extend_from_slice(&digital_event_plot_binary());

        for (label, bytes) in [("ascii", ascii), ("binary", binary)] {
            let file = parse_plots(bytes.clone())
                .unwrap_or_else(|error| panic!("{label} multi-plot file must parse: {error}"));
            assert_eq!(file.plots.len(), 2, "{label}");
            assert_legacy_plot(&file.plots[0]);

            let events = &file.plots[1];
            assert_eq!(
                events.header.plotname, "Digital Events (rspice-digital-events/1)",
                "{label}"
            );
            assert!(events.header.command.is_empty(), "{label}");
            assert_eq!(events.header.no_points, 3, "{label}");
            assert_eq!(events.waveforms[0].y, [0.0, 2.5e-10, 7.0e-10], "{label}");
            assert_eq!(events.waveforms[1].y, [9.0, 1.0, 12.0], "{label}");

            // The single-plot readers every existing caller uses still see the
            // analysis plot, and see it unchanged.
            assert_legacy_plot(
                &parse_raw_reader(&mut Cursor::new(bytes))
                    .unwrap_or_else(|error| panic!("{label} first plot must parse: {error}")),
            );
        }
    }

    #[test]
    fn a_second_binary_plot_is_no_longer_read_as_the_first_plot_s_payload() {
        let mut bytes = legacy_single_plot_binary();
        let first_plot_len = bytes.len();
        bytes.extend_from_slice(&digital_event_plot_binary());

        // The exact-length check used to refuse this file outright: the second
        // plot's bytes are trailing bytes to a reader that knows only one plot.
        let file = parse_plots(bytes).expect("a second binary plot must not be trailing garbage");
        assert_eq!(file.plots.len(), 2);
        assert_eq!(file.plots[0].waveforms[0].y.len(), 2);
        assert_eq!(
            first_plot_len,
            legacy_single_plot_binary().len(),
            "the first plot's byte length is what bounds it"
        );
    }

    #[test]
    fn a_corrupt_tail_is_refused_and_names_the_plot_it_follows() {
        let mut bytes = legacy_single_plot_binary();
        bytes.extend_from_slice(&[0x01, 0x02, 0x03]);

        let error = parse_plots(bytes.clone()).expect_err("a corrupt tail must be refused");
        assert!(
            matches!(&error, RawParseError::DataError(message) if message.contains("plot 1")),
            "unexpected error: {error}"
        );

        let error = parse_raw_reader(&mut Cursor::new(bytes))
            .expect_err("the single-plot reader must refuse it too");
        assert!(
            matches!(&error, RawParseError::DataError(message) if message.contains("plot 1")),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn a_malformed_later_plot_is_refused_and_names_itself() {
        let mut bytes = LEGACY_SINGLE_PLOT_ASCII.as_bytes().to_vec();
        bytes.extend_from_slice(
            b"Plotname: Digital Events (rspice-digital-events/1)\nFlags: real\nNo. Points: 3\nVariables:\n\t0\ttime\ttime\nValues:\n0\t0.0\t9.0\n",
        );

        let error = parse_plots(bytes).expect_err("a plot without No. Variables must be refused");
        assert!(
            matches!(&error, RawParseError::MissingField(field) if field.contains("plot 2")),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn resource_limits_are_enforced_per_plot() {
        let mut bytes = concat!(
            "Plotname: Transient Analysis\n",
            "Flags: real\n",
            "No. Variables: 2\n",
            "No. Points: 1\n",
            "Variables:\n",
            "\t0\ttime\ttime\n",
            "\t1\tV(out)\tvoltage\n",
            "Values:\n",
            "0\t0.0\t1.0\n",
        )
        .as_bytes()
        .to_vec();
        bytes.extend_from_slice(DIGITAL_EVENT_PLOT_ASCII.as_bytes());
        let limits = ResourceLimits {
            max_result_values: 8,
            ..Default::default()
        };

        // The first plot retains four values and is admitted; the second
        // declares six points' worth and is refused before it is allocated.
        let error = parse_raw_plots_reader_with_limits(&mut Cursor::new(bytes.clone()), limits)
            .expect_err("the second plot must be measured against the limit on its own");
        assert!(
            matches!(
                error,
                RawParseError::ResourceLimit(ResourceLimitError {
                    resource: ResourceKind::ResultValues,
                    requested: 12,
                    limit: 8,
                })
            ),
            "unexpected error: {error}"
        );

        parse_raw_reader_with_limits(&mut Cursor::new(bytes), limits)
            .expect("the first plot is within the limit and still parses on its own");
    }

    #[test]
    fn binary_raw_rejects_nonfinite_values() {
        let mut source = b"Title: binary\nPlotname: Transient Analysis\nFlags: double\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nBinary:\n".to_vec();
        source.extend_from_slice(&0.0_f64.to_le_bytes());
        source.extend_from_slice(&f64::NAN.to_le_bytes());
        let mut reader = Cursor::new(source);

        let error = parse_raw_reader(&mut reader).expect_err("NaN binary value must reject");

        assert!(
            matches!(error, RawParseError::DataError(message) if message.contains("Non-finite"))
        );
    }
}
