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

use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

/// Errors that can occur when parsing .raw files
#[derive(Debug, Error)]
pub enum RawParseError {
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
    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    parse_raw_reader(&mut reader)
}

/// Parse a .raw file from a reader
pub fn parse_raw_reader<R: BufRead + Read + Seek>(
    reader: &mut R,
) -> Result<RawWaveformData, RawParseError> {
    // Parse ASCII header
    let (mut header, variables, data_offset) = parse_header(reader)?;

    // Seek to data section
    reader.seek(SeekFrom::Start(data_offset))?;

    // Parse data
    let (waveforms, actual_points) = if header.is_binary {
        parse_binary_data(reader, &header, &variables)?
    } else {
        parse_ascii_data(reader, &header, &variables)?
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
                    header.is_double = !header.flags.iter().any(|f| f.eq_ignore_ascii_case("real"));
                }
                "no. variables" => {
                    saw_no_variables = true;
                    header.no_variables = value.parse().map_err(|_| {
                        RawParseError::InvalidHeader(format!("Invalid no. variables: {}", value))
                    })?;
                }
                "no. points" => {
                    saw_no_points = true;
                    header.no_points = value.parse().map_err(|_| {
                        RawParseError::InvalidHeader(format!("Invalid no. points: {}", value))
                    })?;
                }
                "variables" => {
                    in_variables = true;
                }
                _ => {}
            }
        } else if in_variables && !line.is_empty() {
            // Parse variable line: "index name type"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3
                && let Ok(index) = parts[0].parse::<usize>()
            {
                variables.push(RawVariable {
                    index,
                    name: parts[1].to_string(),
                    var_type: parts[2].to_string(),
                });
            }
        }
    }

    if !saw_no_variables || header.no_variables == 0 {
        return Err(RawParseError::MissingField("No. Variables".to_string()));
    }
    if !saw_no_points {
        return Err(RawParseError::MissingField("No. Points".to_string()));
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
    fn row_size_bytes(self, num_vars: usize) -> usize {
        match self {
            Self::RealAllF64 => num_vars * 8,
            Self::RealMixedAxisF64RestF32 => 8 + num_vars.saturating_sub(1) * 4,
            Self::ComplexAllF64 => num_vars * 16,
            Self::ComplexMixedAxisF64RestF32 => 16 + num_vars.saturating_sub(1) * 8,
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
        let row_size = encoding.row_size_bytes(num_vars);
        if row_size == 0 {
            continue;
        }

        if header.no_points > 0 {
            if payload_len == row_size.saturating_mul(header.no_points) {
                matches.push((encoding, header.no_points));
            }
        } else if payload_len % row_size == 0 {
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
        .find(|(encoding, _)| encoding.is_all_f64())
    {
        return Ok(preferred);
    }

    Ok(matches[0])
}

fn build_waveforms(
    data: Vec<Vec<f64>>,
    data_imag: Option<Vec<Vec<f64>>>,
    variables: &[RawVariable],
) -> Vec<RawWaveform> {
    let x_data = data.first().cloned().unwrap_or_default();
    let mut waveforms = Vec::with_capacity(variables.len());

    for (var_idx, var) in variables.iter().enumerate() {
        waveforms.push(RawWaveform {
            name: var.name.clone(),
            x: x_data.clone(),
            y: data.get(var_idx).cloned().unwrap_or_default(),
            y_imag: data_imag
                .as_ref()
                .map(|imag| imag.get(var_idx).cloned().unwrap_or_default()),
        });
    }

    waveforms
}

/// Parse binary data section (IEEE 754 format)
fn parse_binary_data<R: Read>(
    reader: &mut R,
    header: &RawFileHeader,
    variables: &[RawVariable],
) -> Result<(Vec<RawWaveform>, usize), RawParseError> {
    let num_vars = header.no_variables;
    let mut payload = Vec::new();
    reader.read_to_end(&mut payload)?;
    let (encoding, num_points) = detect_binary_encoding(payload.len(), header, num_vars)?;
    let mut cursor = Cursor::new(payload);

    // Initialize storage
    let mut data: Vec<Vec<f64>> = vec![Vec::with_capacity(num_points); num_vars];
    let mut data_imag = header
        .is_complex
        .then(|| vec![Vec::with_capacity(num_points); num_vars]);

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
                imag[var_idx].push(imag_value);
            }
        }
    }

    Ok((build_waveforms(data, data_imag, variables), num_points))
}

/// Parse ASCII data section
fn parse_ascii_data<R: BufRead>(
    reader: &mut R,
    header: &RawFileHeader,
    variables: &[RawVariable],
) -> Result<(Vec<RawWaveform>, usize), RawParseError> {
    let num_vars = header.no_variables;
    let lines = reader
        .lines()
        .map(|line| line.map(|line| line.trim().to_string()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut data: Vec<Vec<f64>> = vec![Vec::new(); num_vars];
    let row_oriented = lines
        .first()
        .map(|line| line.split_whitespace().count() >= num_vars + 1)
        .unwrap_or(false);

    if row_oriented {
        let point_count = if header.no_points > 0 {
            header.no_points.min(lines.len())
        } else {
            lines.len()
        };

        for line in lines.iter().take(point_count) {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            if parts.len() < num_vars + 1 {
                return Err(RawParseError::DataError(format!(
                    "ASCII row is missing values: {}",
                    line
                )));
            }

            for var_idx in 0..num_vars {
                let value_str = parts[var_idx + 1];
                let value = value_str.parse().map_err(|_| {
                    RawParseError::DataError(format!("Invalid value: {}", value_str))
                })?;
                data[var_idx].push(value);
            }
        }
    } else {
        let max_points = if header.no_points > 0 {
            header.no_points
        } else {
            usize::MAX
        };
        let mut current_point = 0;
        let mut current_var = 0;

        for line in &lines {
            let value_str = if current_var == 0 {
                line.split_whitespace().nth(1).unwrap_or(line)
            } else {
                line.split_whitespace().next().unwrap_or(line)
            };

            let value: f64 = value_str
                .parse()
                .map_err(|_| RawParseError::DataError(format!("Invalid value: {}", value_str)))?;

            data[current_var].push(value);
            current_var += 1;

            if current_var >= num_vars {
                current_var = 0;
                current_point += 1;
                if current_point >= max_points {
                    break;
                }
            }
        }
    }

    let actual_points = data.first().map(Vec::len).unwrap_or(0);
    Ok((build_waveforms(data, None, variables), actual_points))
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
    use crate::analysis::{RawExporter, RawFormat};
    use std::io::Cursor;

    #[test]
    fn test_parse_header() {
        let raw_header = r#"Title: Test Circuit
Date: Sun Jan 17 2026
Plotname: Transient Analysis
Flags: real
No. Variables: 3
No. Points: 100
Command: .tran 0 10m 0 1u
Variables:
	0	time	time
	1	V(out)	voltage
	2	I(R1)	current
Binary:
"#;
        let mut cursor = Cursor::new(raw_header.as_bytes());
        let (header, variables, _offset) = parse_header(&mut cursor).unwrap();

        assert_eq!(header.title, "Test Circuit");
        assert_eq!(header.plotname, "Transient Analysis");
        assert_eq!(header.no_variables, 3);
        assert_eq!(header.no_points, 100);
        assert!(header.is_binary);
        assert!(!header.is_complex);
        assert_eq!(variables.len(), 3);
        assert_eq!(variables[0].name, "time");
        assert_eq!(variables[1].name, "V(out)");
        assert_eq!(variables[2].name, "I(R1)");
    }

    #[test]
    fn test_parse_ascii_data() {
        let raw_file = r#"Title: Test
Plotname: Transient
Flags: real
No. Variables: 2
No. Points: 3
Variables:
	0	time	time
	1	V(out)	voltage
Values:
0	0.000000e+00
	1.000000e+00
1	1.000000e-03
	2.000000e+00
2	2.000000e-03
	3.000000e+00
"#;
        let mut cursor = Cursor::new(raw_file.as_bytes());
        let data = parse_raw_reader(&mut cursor).unwrap();

        assert_eq!(data.waveforms.len(), 2);
        assert_eq!(data.waveforms[0].x.len(), 3);
        assert_eq!(data.waveforms[1].y.len(), 3);
        assert!((data.waveforms[1].y[0] - 1.0).abs() < 1e-6);
        assert!((data.waveforms[1].y[2] - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_parse_binary_mixed_real_data() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(
            br#"Title: Test
Plotname: Transient
Flags: real
No. Variables: 2
No. Points: 2
Variables:
	0	time	time
	1	V(out)	voltage
Binary:
"#,
        );
        bytes.extend_from_slice(&0.0_f64.to_le_bytes());
        bytes.extend_from_slice(&1.25_f32.to_le_bytes());
        bytes.extend_from_slice(&1.0e-9_f64.to_le_bytes());
        bytes.extend_from_slice(&(-0.75_f32).to_le_bytes());

        let mut cursor = Cursor::new(bytes);
        let data = parse_raw_reader(&mut cursor).unwrap();

        assert_eq!(data.header.no_points, 2);
        assert!((data.waveforms[1].y[0] - 1.25).abs() < 1e-6);
        assert!((data.waveforms[1].y[1] + 0.75).abs() < 1e-6);
    }

    #[test]
    fn test_parse_binary_rspice_all_f64_round_trip() {
        let mut exporter = RawExporter::new_transient("Round Trip");
        exporter.add_voltage("out");
        exporter.add_current("vdd");
        exporter.add_point(vec![0.0, 1.0, -2.0]);
        exporter.add_point(vec![2.5e-9, 1.5, -2.5]);

        let mut bytes = Vec::new();
        exporter.write(&mut bytes, RawFormat::Binary).unwrap();

        let mut cursor = Cursor::new(bytes);
        let data = parse_raw_reader(&mut cursor).unwrap();

        assert_eq!(data.header.no_points, 2);
        assert_eq!(data.waveforms.len(), 3);
        assert!((data.waveforms[1].y[1] - 1.5).abs() < 1e-12);
        assert!((data.waveforms[2].y[0] + 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_parse_ascii_rspice_row_oriented_round_trip() {
        let mut exporter = RawExporter::new_transient("ASCII Round Trip");
        exporter.add_voltage("out");
        exporter.add_current("vdd");
        exporter.add_point(vec![0.0, 3.0, -1.0]);
        exporter.add_point(vec![1.0e-6, 4.0, -1.5]);

        let mut bytes = Vec::new();
        exporter.write(&mut bytes, RawFormat::Ascii).unwrap();

        let mut cursor = Cursor::new(bytes);
        let data = parse_raw_reader(&mut cursor).unwrap();

        assert_eq!(data.header.no_points, 2);
        assert_eq!(data.waveforms.len(), 3);
        assert!((data.waveforms[1].y[0] - 3.0).abs() < 1e-12);
        assert!((data.waveforms[2].y[1] + 1.5).abs() < 1e-12);
    }

    #[test]
    fn test_parse_binary_rspice_streaming_header_infers_point_count() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(
            br#"Title: Streaming Test
Plotname: Transient Analysis
Flags: real
No. Variables: 2
No. Points: 0
Variables:
	0	time	time
	1	V(out)	voltage
Binary:
"#,
        );
        bytes.extend_from_slice(&0.0_f64.to_le_bytes());
        bytes.extend_from_slice(&5.0_f64.to_le_bytes());
        bytes.extend_from_slice(&2.0e-9_f64.to_le_bytes());
        bytes.extend_from_slice(&4.5_f64.to_le_bytes());

        let mut cursor = Cursor::new(bytes);
        let data = parse_raw_reader(&mut cursor).unwrap();

        assert_eq!(data.header.no_points, 2);
        assert!((data.waveforms[0].y[1] - 2.0e-9).abs() < 1e-18);
        assert!((data.waveforms[1].y[0] - 5.0).abs() < 1e-12);
    }
}
