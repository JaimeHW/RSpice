//! Strict CSV/TSV waveform decoding with canonical engineering units.

use crate::WaveformDomain;
use std::collections::HashSet;
use unit::{EngineeringUnit, UnitDimension};

pub mod layout;
pub mod metadata;
pub(crate) mod unit;

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
    stated_unit: Option<String>,
}

impl DelimitedColumn {
    /// Unit of the decoded samples; absent metadata remains absent.
    /// Explicit table signal units retain their original numeric representation.
    pub fn canonical_unit(&self) -> Option<&str> {
        self.stated_unit
            .as_deref()
            .or_else(|| self.unit.map(EngineeringUnit::canonical_symbol))
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
        .trim(csv::Trim::Fields)
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
    for (index, header) in raw_headers.iter().enumerate() {
        validate_header_text(header.trim(), index + 1, limits.max_header_bytes)?;
    }

    // Metadata follows the data so general CSV consumers can skip comment
    // records. Inspect it before interpreting unit-like text in literal labels.
    // This bounded pass retains only one CSV record, never a second sample table.
    let declared = declared_table_metadata(text, delimiter, &raw_headers, limits.max_rows)?;
    let mut headers = Vec::with_capacity(raw_headers.len());
    let mut unique_names = HashSet::with_capacity(raw_headers.len());
    for (index, raw) in raw_headers.iter().enumerate() {
        let raw = if index == 0 && declared.is_none() {
            raw.strip_prefix('\u{feff}').unwrap_or(raw)
        } else {
            raw
        };
        let header = if let Some(metadata) = &declared {
            validate_header_text(raw, index + 1, limits.max_header_bytes)?;
            DelimitedColumn {
                name: raw.to_owned(),
                unit: None,
                stated_unit: metadata.columns[index].unit.clone(),
            }
        } else {
            parse_column_header(raw, index + 1, limits.max_header_bytes)?
        };
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

    let analysis_type = if let Some(metadata) = &declared {
        match metadata.columns[0].quantity.as_deref() {
            Some("time") => WaveformDomain::Transient,
            Some("frequency") => WaveformDomain::Ac,
            Some(_) => WaveformDomain::DcSweep,
            None => {
                let coordinate = DelimitedColumn {
                    name: headers[0].name.clone(),
                    unit: metadata.columns[0]
                        .unit
                        .as_deref()
                        .and_then(|unit| EngineeringUnit::parse(unit).ok()),
                    stated_unit: None,
                };
                infer_analysis_type(&coordinate)?
            }
        }
    } else {
        infer_analysis_type(&headers[0])?
    };
    let mut coordinate = Vec::new();
    let mut signal_values = vec![Vec::new(); headers.len() - 1];
    let mut direction = None;
    let mut has_layout = false;

    for (row_index, record) in reader.records().enumerate() {
        let line = row_index + 2;
        if has_layout {
            return Err("the table layout record must be unique and final"
                .to_owned()
                .into());
        }
        // A final layout declaration carries no samples, even at the row limit.
        // Preserve the historical limit error for excess malformed data rows.
        let is_layout = record
            .as_ref()
            .ok()
            .and_then(|row| row.get(0))
            .is_some_and(layout::is_layout_record);
        if coordinate.len() >= limits.max_rows && !is_layout {
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
        if is_layout {
            let names: Vec<_> = headers.iter().map(|header| header.name.as_str()).collect();
            let fields: Vec<_> = record.iter().collect();
            let layout_headers: Vec<_> = if record.get(0) == Some(metadata::RECORD_MARKER) {
                raw_headers.iter().collect()
            } else {
                names
            };
            metadata::parse_table_record(&layout_headers, &fields)?;
            has_layout = true;
            continue;
        }
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
            let cell = record.get(signal_index + 1);
            // An explicit empty signal cell is unavailable; a missing field,
            // nonnumeric token, or non-finite literal remains malformed input.
            values.push(if cell == Some("") {
                f64::NAN
            } else {
                parse_finite_cell(
                    cell,
                    line,
                    column,
                    &headers[signal_index + 1].name,
                    headers[signal_index + 1].unit,
                )?
            });
        }
    }
    if coordinate.len() < limits.min_rows {
        return Err(format!(
            "the '{}' column carries no samples, so the file holds no result to import",
            headers[0].name
        )
        .into());
    }

    if declared.is_some() {
        let mut dataset = crate::numeric::DecodedNumericDataset {
            domain: analysis_type,
            coordinate_name: headers[0].name.clone(),
            coordinate_unit: headers[0].stated_unit.take(),
            coordinate,
            signals: Vec::new(),
        };
        dataset.normalize_coordinate_unit()?;
        coordinate = dataset.coordinate;
        headers[0].stated_unit = dataset.coordinate_unit;
    }

    Ok(DecodedDelimitedWaveforms {
        domain: analysis_type,
        columns: headers,
        coordinate,
        signal_values,
    })
}

fn declared_table_metadata(
    text: &str,
    delimiter: u8,
    headers: &csv::StringRecord,
    max_rows: usize,
) -> Result<Option<metadata::TableMetadata>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(true)
        .flexible(false)
        .trim(csv::Trim::Fields)
        .from_reader(text.as_bytes());
    for record in reader.records().take(max_rows.saturating_add(1)) {
        // The numeric pass retains the existing position-rich CSV error and
        // row-limit refusal order for malformed records.
        let Ok(record) = record else {
            break;
        };
        if record.get(0) == Some(metadata::RECORD_MARKER) {
            let names = headers.iter().collect::<Vec<_>>();
            let fields = record.iter().collect::<Vec<_>>();
            return metadata::parse_table_record(&names, &fields)
                .map(|layout| layout.and_then(|layout| layout.metadata));
        }
    }
    Ok(None)
}

fn validate_header_text(raw: &str, column: usize, max_header_bytes: usize) -> Result<(), String> {
    if raw.trim().is_empty() {
        return Err(format!("column {column} has an empty header"));
    }
    if raw.len() > max_header_bytes {
        return Err(format!(
            "column {column} header exceeds the {max_header_bytes}-byte limit"
        ));
    }
    if raw.chars().any(char::is_control) {
        return Err(format!(
            "column {column} header contains a control character"
        ));
    }
    Ok(())
}

fn parse_column_header(
    raw: &str,
    column: usize,
    max_header_bytes: usize,
) -> Result<DelimitedColumn, String> {
    let raw = raw.trim();
    validate_header_text(raw, column, max_header_bytes)?;

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
        stated_unit: None,
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
    if unit.map_or_else(
        || crate::numeric::decimal_underflowed(value, scaled),
        |unit| unit.lost_nonzero_decimal(value, scaled),
    ) {
        let stage = if unit.is_some() {
            "after unit conversion"
        } else {
            "at binary64 precision"
        };
        return Err(format!(
            "row {row}, column {column} ({header:?}) underflows {stage}"
        ));
    }
    Ok(scaled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn decimal_underflow_cannot_become_an_authored_zero_sample() {
        let limits = DelimitedReadLimits {
            max_columns: 2,
            max_rows: 1,
            max_header_bytes: 32,
            min_rows: 1,
        };
        for separator in [b',', b'\t'] {
            for source in [
                "time,v\n1e-999,1\n",
                "time,v\n0,-1e-999\n",
                "time [s],v\n1e-999,1\n",
                "time,v [V]\n0,1e-999\n",
                "time,v [mV]\n0,-1e-999\n",
            ] {
                let source = source.replace(',', &(separator as char).to_string());
                let error = decode_delimited_waveforms(&source, separator, limits).unwrap_err();
                assert!(error.to_string().contains("underflow"), "{error}");
                assert!(error.to_string().contains("row 2"), "{error}");
            }
            for (source, expected) in [
                ("time,v\n0,-0e-999\n", -0.0_f64),
                ("time,v\n0,0e999\n", 0.0_f64),
                ("time,v\n0,5e-324\n", 5e-324_f64),
                ("time,v\n0,-5e-324\n", -5e-324_f64),
                ("time,v [kV]\n0,1e-326\n", 1e-323_f64),
                ("time,temperature [degC]\n0,-273.15\n", 0.0_f64),
            ] {
                let source = source.replace(',', &(separator as char).to_string());
                let data = decode_delimited_waveforms(&source, separator, limits).unwrap();
                assert_eq!(
                    data.signal_values[0][0].to_bits(),
                    expected.to_bits(),
                    "{source}"
                );
            }
        }
    }

    #[test]
    fn unit_conversion_cannot_replace_nonzero_samples_with_zero() {
        let limits = DelimitedReadLimits {
            max_columns: 2,
            max_rows: 1,
            max_header_bytes: 32,
            min_rows: 1,
        };
        for source in [
            "time [ns],v\n5e-324,1\n",
            "time,v [mV]\n0,5e-324\n",
            "time,i [pA]\n0,-5e-324\n",
        ] {
            let error = decode_delimited_waveforms(source, b',', limits).unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains("underflows after unit conversion"),
                "{error}"
            );
            assert!(error.to_string().contains("row 2"), "{error}");
        }
        for (source, expected) in [
            ("time,v [mV]\n0,-0\n", -0.0_f64),
            ("time,v [mV]\n0,1e-320\n", 1e-323_f64),
            ("time,temperature [degC]\n0,-273.15\n", 0.0_f64),
        ] {
            let decoded = decode_delimited_waveforms(source, b',', limits).unwrap();
            assert_eq!(decoded.signal_values[0][0].to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn layout_is_validated_without_becoming_a_sample_or_changing_physical_columns() {
        let limits = DelimitedReadLimits {
            max_columns: 3,
            max_rows: 1,
            max_header_bytes: 32,
            min_rows: 1,
        };
        for delimiter in [b',', b'\t'] {
            for layout in ["real,real", "complex_real,complex_imag"] {
                let source = format!("time,Re(x),Im(x)\n0,1,2\n# RSpiceTableLayoutV1,{layout}\n\n")
                    .replace(',', &(delimiter as char).to_string());
                let decoded = decode_delimited_waveforms(&source, delimiter, limits).unwrap();
                assert_eq!(decoded.coordinate, [0.0]);
                assert_eq!(decoded.signal_values, [vec![1.0], vec![2.0]]);
                assert_eq!(decoded.columns[1].name, "Re(x)");
                assert_eq!(decoded.columns[2].name, "Im(x)");
            }
        }
        for footer in [
            "# RSpiceTableLayoutV2,real,real\n",
            "# RSpiceTableLayoutV1,real\n",
            "# RSpiceTableLayoutV1,unknown,real\n",
            "# RSpiceTableLayoutV1,complex_real,real\n",
            "# RSpiceTableLayoutV1,real,real\n1,3,4\n",
            "# RSpiceTableLayoutV1,real,real\n# RSpiceTableLayoutV1,real,real\n",
        ] {
            let source = format!("time,Re(x),Im(x)\n0,1,2\n{footer}");
            assert!(
                decode_delimited_waveforms(&source, b',', limits).is_err(),
                "{footer}"
            );
        }
        assert!(
            decode_delimited_waveforms(
                "time,Re(x),Im(x)\n# RSpiceTableLayoutV1,real,real\n",
                b',',
                limits
            )
            .is_err()
        );
    }

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
