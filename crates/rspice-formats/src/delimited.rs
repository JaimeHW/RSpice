//! Strict CSV/TSV waveform decoding with canonical engineering units.

use crate::WaveformDomain;
use std::collections::HashSet;
use unit::{EngineeringUnit, UnitDimension};

mod unit;

/// Limits supplied by the consumer of an already byte-bounded source.
#[derive(Debug, Clone, Copy)]
pub struct DelimitedReadLimits {
    pub max_columns: usize,
    pub max_rows: usize,
    pub max_header_bytes: usize,
    pub min_rows: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DelimitedColumn {
    pub name: String,
    unit: Option<EngineeringUnit>,
}

impl DelimitedColumn {
    /// Unit of the normalized samples; absent metadata remains absent.
    pub fn canonical_unit(&self) -> Option<&'static str> {
        self.unit.map(EngineeringUnit::canonical_symbol)
    }
}

/// The first column describes the coordinate; remaining columns describe signals.
#[derive(Debug)]
pub struct DecodedDelimitedWaveforms {
    pub domain: WaveformDomain,
    pub columns: Vec<DelimitedColumn>,
    pub coordinate: Vec<f64>,
    pub signal_values: Vec<Vec<f64>>,
}

#[derive(Debug)]
pub enum DelimitedReadError {
    InvalidData(String),
    Csv { context: String, source: csv::Error },
}

impl From<String> for DelimitedReadError {
    fn from(detail: String) -> Self {
        Self::InvalidData(detail)
    }
}

impl std::fmt::Display for DelimitedReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidData(detail) => f.write_str(detail),
            Self::Csv { context, source } => {
                if let Some(position) = source.position() {
                    write!(
                        f,
                        "{context} is malformed near row {}, byte {}: {}",
                        position.line(),
                        position.byte(),
                        source
                    )
                } else {
                    write!(f, "{context} is malformed: {source}")
                }
            }
        }
    }
}

impl std::error::Error for DelimitedReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidData(_) => None,
            Self::Csv { source, .. } => Some(source),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoordinateDirection {
    Increasing,
    Decreasing,
}

pub fn decode_delimited_waveforms(
    text: &str,
    delimiter: u8,
    limits: DelimitedReadLimits,
) -> Result<DecodedDelimitedWaveforms, DelimitedReadError> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(true)
        .flexible(false)
        .trim(csv::Trim::All)
        .from_reader(text.as_bytes());
    let raw_headers = reader
        .headers()
        .map_err(|error| DelimitedReadError::Csv {
            context: "header".into(),
            source: error,
        })?
        .clone();
    if raw_headers.len() < 2 {
        return Err(
            "the header must contain one coordinate and at least one signal"
                .to_owned()
                .into(),
        );
    }
    if raw_headers.len() > limits.max_columns {
        return Err(format!(
            "the dataset has {} columns; the import limit is {}",
            raw_headers.len(),
            limits.max_columns
        )
        .into());
    }

    let mut headers = Vec::with_capacity(raw_headers.len());
    let mut unique_names = HashSet::with_capacity(raw_headers.len());
    for (index, raw) in raw_headers.iter().enumerate() {
        let raw = if index == 0 {
            raw.strip_prefix('\u{feff}').unwrap_or(raw)
        } else {
            raw
        };
        let header = parse_column_header(raw, index + 1, limits.max_header_bytes)?;
        let key = header.name.to_lowercase();
        if !unique_names.insert(key) {
            return Err(format!(
                "column {} repeats the signal/header name {:?}",
                index + 1,
                header.name
            )
            .into());
        }
        headers.push(header);
    }

    let analysis_type = infer_analysis_type(&headers[0])?;
    let mut coordinate = Vec::new();
    let mut signal_values = vec![Vec::new(); headers.len() - 1];
    let mut direction = None;

    for (row_index, record) in reader.records().enumerate() {
        let line = row_index + 2;
        if row_index >= limits.max_rows {
            return Err(format!(
                "the dataset exceeds the {}-row import limit",
                limits.max_rows
            )
            .into());
        }
        let record = record.map_err(|error| DelimitedReadError::Csv {
            context: format!("row {line}"),
            source: error,
        })?;
        let x = parse_finite_cell(record.get(0), line, 1, &headers[0].name, headers[0].unit)?;
        if analysis_type == WaveformDomain::Ac && x <= 0.0 {
            return Err(format!(
                "row {line} frequency must be greater than zero after unit conversion"
            )
            .into());
        }
        if let Some(previous) = coordinate.last().copied() {
            let step = if x > previous {
                CoordinateDirection::Increasing
            } else if x < previous {
                CoordinateDirection::Decreasing
            } else {
                return Err(format!(
                    "row {line} repeats coordinate {x}; coordinates must be strictly monotonic"
                )
                .into());
            };
            if let Some(expected) = direction {
                if step != expected {
                    return Err(format!(
                        "row {line} reverses coordinate ordering; coordinates must remain strictly monotonic"
                    ).into());
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
                headers[signal_index + 1].unit,
            )?);
        }
    }
    if coordinate.len() < limits.min_rows {
        return Err(format!(
            "the '{}' column carries no samples, so the file holds no result to import",
            headers[0].name
        )
        .into());
    }

    Ok(DecodedDelimitedWaveforms {
        domain: analysis_type,
        columns: headers,
        coordinate,
        signal_values,
    })
}

fn parse_column_header(
    raw: &str,
    column: usize,
    max_header_bytes: usize,
) -> Result<DelimitedColumn, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(format!("column {column} has an empty header"));
    }
    if raw.len() > max_header_bytes {
        return Err(format!(
            "column {column} header exceeds the {}-byte limit",
            max_header_bytes
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
    Ok(DelimitedColumn {
        name: name.to_owned(),
        unit,
    })
}

fn parse_unit(raw: &str) -> Result<EngineeringUnit, String> {
    // Historical CSV headers used bare C for Celsius. Keep that adapter
    // spelling local; it is not a general engineering-unit abbreviation.
    EngineeringUnit::parse(if raw.trim() == "C" { "degC" } else { raw })
}
fn infer_analysis_type(header: &DelimitedColumn) -> Result<WaveformDomain, String> {
    let normalized: String = header
        .name
        .chars()
        .filter(|character| !matches!(character, '_' | '-' | ' '))
        .flat_map(char::to_lowercase)
        .collect();
    let (analysis_type, required_dimension) = match normalized.as_str() {
        "time" | "t" | "timestamp" => (WaveformDomain::Transient, Some(UnitDimension::Time)),
        "frequency" | "freq" => (WaveformDomain::Ac, Some(UnitDimension::Frequency)),
        "f" if header
            .unit
            .is_some_and(|unit| unit.dimension == UnitDimension::Frequency) =>
        {
            (WaveformDomain::Ac, Some(UnitDimension::Frequency))
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
            (WaveformDomain::Transient, Some(UnitDimension::Time))
        }
        _ if header
            .unit
            .is_some_and(|unit| unit.dimension == UnitDimension::Frequency) =>
        {
            (WaveformDomain::Ac, Some(UnitDimension::Frequency))
        }
        _ => (WaveformDomain::DcSweep, None),
    };

    if let (Some(required), Some(unit)) = (required_dimension, header.unit)
        && unit.dimension != required
    {
        return Err(format!(
            "{} coordinate {:?} requires a {} unit",
            analysis_type.label(),
            header.name,
            match required {
                UnitDimension::Time => "time",
                UnitDimension::Frequency => "frequency",
                _ => "compatible",
            }
        ));
    }
    if analysis_type == WaveformDomain::DcSweep
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
    Ok(analysis_type)
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
    unit: Option<EngineeringUnit>,
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
    let scaled = unit.map_or(parsed, |unit| unit.normalize_decimal(value, parsed));
    if !scaled.is_finite() {
        return Err(format!(
            "row {row}, column {column} ({header:?}) overflows after unit conversion"
        ));
    }
    Ok(scaled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn reader_limits_preserve_refusal_order_and_csv_error_sources() {
        let limits = DelimitedReadLimits {
            max_columns: 2,
            max_rows: 1,
            max_header_bytes: 32,
            min_rows: 1,
        };
        let source = "time [ms],\"V(out,ref) [mV]\"\n1,2.5\n";
        let decoded = decode_delimited_waveforms(source, b',', limits).unwrap();
        assert_eq!(decoded.domain, WaveformDomain::Transient);
        assert_eq!(decoded.coordinate, [0.001]);
        assert_eq!(decoded.signal_values, [vec![0.0025]]);
        assert_eq!(decoded.columns[1].name, "V(out,ref)");
        assert_eq!(decoded.columns[1].canonical_unit(), Some("V"));

        for (source, limits, expected) in [
            (
                "time,v,w\n0,1,2\n",
                limits,
                "the dataset has 3 columns; the import limit is 2",
            ),
            (
                "time,v\n0,1\n2\n",
                limits,
                "the dataset exceeds the 1-row import limit",
            ),
            (
                "time,v\n0,1\n",
                DelimitedReadLimits {
                    max_header_bytes: 3,
                    ..limits
                },
                "column 1 header exceeds the 3-byte limit",
            ),
            (
                "time,v\n",
                limits,
                "the 'time' column carries no samples, so the file holds no result to import",
            ),
        ] {
            let error = decode_delimited_waveforms(source, b',', limits).unwrap_err();
            assert!(matches!(error, DelimitedReadError::InvalidData(_)));
            assert_eq!(error.to_string(), expected);
            assert!(error.source().is_none());
        }
        let error = decode_delimited_waveforms("time,v\n0\n", b',', limits).unwrap_err();
        assert!(matches!(&error, DelimitedReadError::Csv { context, .. } if context == "row 2"));
        assert!(
            error
                .to_string()
                .starts_with("row 2 is malformed near row 2, byte 7:")
        );
        assert!(
            error
                .source()
                .unwrap()
                .downcast_ref::<csv::Error>()
                .is_some()
        );
    }
}
