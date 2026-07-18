//! Third-party .raw File Parser
//!
//! Parses standard simulation output files (.raw format).
//! Supports both ASCII and binary (IEEE 754) formats.
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

fn parse_raw_bytes(
    bytes: &[u8],
    resource_limits: ResourceLimits,
) -> Result<RawWaveformData, RawParseError> {
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
    let (waveforms, actual_points) = if header.is_binary {
        parse_binary_data(data_bytes, &header, &variables, resource_limits)?
    } else {
        let mut data_reader = BufReader::new(Cursor::new(data_bytes));
        parse_ascii_data(&mut data_reader, &header, &variables, resource_limits)?
    };
    if header.no_points == 0 {
        header.no_points = actual_points;
    }

    Ok(RawWaveformData {
        header,
        variables,
        waveforms,
    })
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

fn detect_binary_encoding(
    payload_len: usize,
    header: &RawFileHeader,
    num_vars: usize,
) -> Result<(BinaryEncoding, usize), RawParseError> {
    let candidates = if header.is_complex {
        [
            BinaryEncoding::ComplexAllF64,
            BinaryEncoding::ComplexMixedAxisF64RestF32,
        ]
        .to_vec()
    } else {
        [
            BinaryEncoding::RealAllF64,
            BinaryEncoding::RealMixedAxisF64RestF32,
        ]
        .to_vec()
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
            if row_size.checked_mul(header.no_points) == Some(payload_len) {
                matches.push((encoding, header.no_points));
            }
        } else if payload_len.is_multiple_of(row_size) {
            matches.push((encoding, payload_len / row_size));
        }
    }

    if matches.is_empty() {
        return Err(RawParseError::DataError(format!(
            "Binary payload length {} does not match any supported encoding for {} variable(s) and {} point(s)",
            payload_len, num_vars, header.no_points
        )));
    }

    if matches.len() == 1 {
        return Ok(matches[0]);
    }

    if let Some(preferred) = matches
        .iter()
        .copied()
        .find(|(encoding, _)| encoding.is_all_f64() == header.is_double)
    {
        return Ok(preferred);
    }

    Ok(matches[0])
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
) -> Result<(Vec<RawWaveform>, usize), RawParseError> {
    let num_vars = header.no_variables;
    let (encoding, num_points) = detect_binary_encoding(payload.len(), header, num_vars)?;
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

    Ok((build_waveforms(data, data_imag, variables)?, num_points))
}

/// Parse ASCII data section
fn parse_ascii_data<R: BufRead>(
    reader: &mut R,
    header: &RawFileHeader,
    variables: &[RawVariable],
    resource_limits: ResourceLimits,
) -> Result<(Vec<RawWaveform>, usize), RawParseError> {
    let num_vars = header.no_variables;
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() {
            continue;
        }
        lines.try_reserve(1).map_err(|error| {
            RawParseError::DataError(format!("unable to retain raw ASCII rows: {error}"))
        })?;
        lines.push(line);
    }
    let row_oriented = lines
        .first()
        .map(|line| line.split_whitespace().count() > num_vars)
        .unwrap_or(false);

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
    Ok((build_waveforms(data, None, variables)?, actual_points))
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
        let mut limits = ResourceLimits::default();
        limits.max_external_data_bytes = source.len() - 1;
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
        let mut limits = ResourceLimits::default();
        limits.max_result_values = 3;
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
