//! Read simulation results back into an [`ExportTable`] from any format the
//! CLI can write: SPICE rawfile (binary or ASCII), CSV, TSV, JSON, and HDF5.
//!
//! Used by `convert` and `compare` so every command understands every format.

use crate::cli::{CliError, OutputFormat};
use crate::commands::export_table::{ColumnData, ExportColumn, ExportTable};
use crate::hdf5::read_hdf5;
use std::path::Path;

/// Guess a format from the file extension; rawfile when unknown.
pub(crate) fn detect_format(path: &Path) -> OutputFormat {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .as_deref()
    {
        Some("csv") => OutputFormat::Csv,
        Some("tsv") => OutputFormat::Tsv,
        Some("json") => OutputFormat::Json,
        Some("h5") | Some("hdf5") => OutputFormat::Hdf5,
        _ => OutputFormat::Raw,
    }
}

/// Load a result file into a table.
pub(crate) fn load_table(path: &Path, format: OutputFormat) -> Result<ExportTable, CliError> {
    let table = match format {
        OutputFormat::Raw | OutputFormat::RawAscii => load_rawfile(path),
        OutputFormat::Csv => load_delimited(path, ','),
        OutputFormat::Tsv => load_delimited(path, '\t'),
        OutputFormat::Json => load_json(path),
        OutputFormat::Hdf5 => load_hdf5(path),
    }?;
    validate_table_shape(path, table)
}

fn conversion_error(path: &Path, message: impl std::fmt::Display) -> CliError {
    CliError::ConversionError {
        message: format!("{}: {}", path.display(), message),
    }
}

fn validate_table_shape(path: &Path, table: ExportTable) -> Result<ExportTable, CliError> {
    validate_values(path, &table.scale_name, "scale", &table.scale)?;
    let expected = table.scale.len();
    for column in &table.columns {
        match &column.data {
            ColumnData::Real(values) => {
                validate_series_len(path, &column.name, "values", values.len(), expected)?;
                validate_values(path, &column.name, "values", values)?;
            }
            ColumnData::Complex { real, imag } => {
                validate_series_len(path, &column.name, "real", real.len(), expected)?;
                validate_series_len(path, &column.name, "imag", imag.len(), expected)?;
                validate_values(path, &column.name, "real", real)?;
                validate_values(path, &column.name, "imag", imag)?;
            }
        }
    }
    Ok(table)
}

fn validate_series_len(
    path: &Path,
    signal: &str,
    part: &str,
    actual: usize,
    expected: usize,
) -> Result<(), CliError> {
    if actual != expected {
        return Err(conversion_error(
            path,
            format!(
                "signal '{}' {} has {} points; expected {} to match the scale",
                signal, part, actual, expected
            ),
        ));
    }
    Ok(())
}

fn validate_values(path: &Path, signal: &str, part: &str, values: &[f64]) -> Result<(), CliError> {
    if let Some((index, value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(conversion_error(
            path,
            format!(
                "non-finite value {} in '{}' {} at point {}",
                value, signal, part, index
            ),
        ));
    }
    Ok(())
}

fn load_rawfile(path: &Path) -> Result<ExportTable, CliError> {
    let data = rspice_core::compat::parse_raw_file(path).map_err(|e| conversion_error(path, e))?;

    let mut waveforms = data.waveforms.into_iter();
    let Some(scale) = waveforms.next() else {
        return Err(conversion_error(path, "rawfile contains no variables"));
    };
    let scale_variable = data.variables.first();

    let columns = waveforms
        .zip(data.variables.iter().skip(1))
        .map(|(waveform, variable)| ExportColumn {
            name: waveform.name,
            var_type: variable.var_type.clone(),
            data: match waveform.y_imag {
                Some(imag) => ColumnData::Complex {
                    real: waveform.y,
                    imag,
                },
                None => ColumnData::Real(waveform.y),
            },
        })
        .collect();

    Ok(ExportTable {
        analysis: "converted".to_string(),
        plot_name: if data.header.plotname.is_empty() {
            "Converted Data".to_string()
        } else {
            data.header.plotname
        },
        scale_name: scale.name,
        scale_type: scale_variable
            .map(|v| v.var_type.clone())
            .unwrap_or_else(|| "time".to_string()),
        scale: scale.y,
        columns,
    })
}

fn load_delimited(path: &Path, separator: char) -> Result<ExportTable, CliError> {
    let content = std::fs::read_to_string(path).map_err(|e| CliError::InputReadError {
        path: path.to_path_buf(),
        source: e,
    })?;

    let mut lines = content.lines().filter(|line| !line.trim().is_empty());
    let header = parse_delimited_record(
        lines
            .next()
            .ok_or_else(|| conversion_error(path, "empty input file"))?,
        separator,
    )
    .map_err(|message| conversion_error(path, format!("header row: {message}")))?;
    if header.is_empty() {
        return Err(conversion_error(path, "missing header row"));
    }

    let mut scale = Vec::new();
    let mut series: Vec<Vec<f64>> = vec![Vec::new(); header.len().saturating_sub(1)];
    for (row, line) in lines.enumerate() {
        let line_number = row + 2;
        let fields = parse_delimited_record(line, separator)
            .map_err(|message| conversion_error(path, format!("row {line_number}: {message}")))?;
        let parse = |field: &str, column: &str| {
            let token = field.trim();
            let value = token.parse::<f64>().map_err(|_| {
                conversion_error(
                    path,
                    format!(
                        "non-numeric value '{}' in column '{}', row {}",
                        token, column, line_number
                    ),
                )
            })?;
            if !value.is_finite() {
                return Err(conversion_error(
                    path,
                    format!(
                        "non-finite value '{}' in column '{}', row {}",
                        token, column, line_number
                    ),
                ));
            }
            Ok(value)
        };

        if fields.len() != header.len() {
            return Err(conversion_error(
                path,
                format!(
                    "row {} has {} columns; expected {} columns",
                    line_number,
                    fields.len(),
                    header.len()
                ),
            ));
        }

        scale.push(parse(&fields[0], &header[0])?);
        for (i, field) in fields.iter().skip(1).enumerate() {
            series[i].push(parse(field, &header[i + 1])?);
        }
    }

    let mut columns: Vec<ExportColumn> = Vec::with_capacity(series.len());
    let mut iter = header.iter().skip(1).zip(series).peekable();
    while let Some((name, values)) = iter.next() {
        // Fold adjacent `Re(x)` / `Im(x)` pairs back into one complex column.
        let complex_pair = complex_part_name(name, "Re(").and_then(|inner| {
            let has_matching_imag = iter.peek().is_some_and(|(next_name, _)| {
                complex_part_name(next_name, "Im(").as_deref() == Some(inner.as_str())
            });
            has_matching_imag
                .then(|| iter.next().map(|(_, imag)| (inner, imag)))
                .flatten()
        });
        if let Some((inner, imag)) = complex_pair {
            columns.push(ExportColumn {
                name: inner,
                var_type: "voltage".to_string(),
                data: ColumnData::Complex { real: values, imag },
            });
            continue;
        }
        columns.push(ExportColumn {
            name: name.clone(),
            var_type: signal_var_type(name),
            data: ColumnData::Real(values),
        });
    }

    let scale_name = header[0].clone();
    Ok(ExportTable {
        analysis: "converted".to_string(),
        plot_name: "Converted Data".to_string(),
        scale_type: scale_var_type(&scale_name),
        scale_name,
        scale,
        columns,
    })
}

fn parse_delimited_record(line: &str, separator: char) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut chars = line.chars().peekable();
    let mut in_quotes = false;
    let mut quoted = false;

    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    in_quotes = false;
                    quoted = true;
                }
            } else {
                field.push(ch);
            }
            continue;
        }

        if ch == separator {
            fields.push(finish_delimited_field(&field, quoted));
            field.clear();
            quoted = false;
        } else if ch == '"' && field.trim().is_empty() {
            field.clear();
            in_quotes = true;
        } else {
            field.push(ch);
        }
    }

    if in_quotes {
        return Err("unterminated quoted field".to_string());
    }
    fields.push(finish_delimited_field(&field, quoted));
    Ok(fields)
}

fn finish_delimited_field(field: &str, quoted: bool) -> String {
    if quoted {
        field.to_string()
    } else {
        field.trim().to_string()
    }
}

fn load_json(path: &Path) -> Result<ExportTable, CliError> {
    let content = std::fs::read_to_string(path).map_err(|e| CliError::InputReadError {
        path: path.to_path_buf(),
        source: e,
    })?;
    let value: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| conversion_error(path, e))?;

    let to_f64_vec = |value: &serde_json::Value, what: &str| -> Result<Vec<f64>, CliError> {
        value
            .as_array()
            .ok_or_else(|| conversion_error(path, format!("'{}' is not an array", what)))?
            .iter()
            .map(|v| {
                v.as_f64().ok_or_else(|| {
                    conversion_error(path, format!("non-numeric entry in '{}'", what))
                })
            })
            .collect()
    };

    // Preferred schema: {"analysis", "scale": {"name", "values"}, "signals": [...]}
    if let Some(scale_obj) = value.get("scale") {
        let scale_name = scale_obj
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("scale")
            .to_string();
        let scale = to_f64_vec(
            scale_obj
                .get("values")
                .ok_or_else(|| conversion_error(path, "scale has no 'values'"))?,
            "scale.values",
        )?;

        let mut columns = Vec::new();
        for signal in value
            .get("signals")
            .and_then(|v| v.as_array())
            .ok_or_else(|| conversion_error(path, "missing 'signals' array"))?
        {
            let name = signal
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| conversion_error(path, "signal has no 'name'"))?
                .to_string();
            let data = if let Some(values) = signal.get("values") {
                ColumnData::Real(to_f64_vec(values, &name)?)
            } else {
                ColumnData::Complex {
                    real: to_f64_vec(
                        signal.get("real").ok_or_else(|| {
                            conversion_error(path, "signal has no 'values' or 'real'")
                        })?,
                        &name,
                    )?,
                    imag: to_f64_vec(
                        signal.get("imag").ok_or_else(|| {
                            conversion_error(path, "complex signal has no 'imag'")
                        })?,
                        &name,
                    )?,
                }
            };
            columns.push(ExportColumn {
                var_type: signal_var_type(&name),
                name,
                data,
            });
        }

        return Ok(ExportTable {
            analysis: value
                .get("analysis")
                .and_then(|v| v.as_str())
                .unwrap_or("converted")
                .to_string(),
            plot_name: "Converted Data".to_string(),
            scale_type: scale_var_type(&scale_name),
            scale_name,
            scale,
            columns,
        });
    }

    // Legacy schema: {"variables": {name: [...], ...}}
    if let Some(variables) = value.get("variables").and_then(|v| v.as_object()) {
        let mut entries: Vec<(String, Vec<f64>)> = Vec::with_capacity(variables.len());
        for (name, values) in variables {
            entries.push((name.clone(), to_f64_vec(values, name)?));
        }

        // The scale is the entry named like an axis, else the first one.
        let scale_index = entries
            .iter()
            .position(|(name, _)| {
                let lower = name.to_ascii_lowercase();
                lower == "time" || lower == "frequency" || lower == "scale"
            })
            .unwrap_or(0);
        if entries.is_empty() {
            return Err(conversion_error(path, "'variables' is empty"));
        }
        let (scale_name, scale) = entries.remove(scale_index);

        let columns = entries
            .into_iter()
            .map(|(name, values)| ExportColumn {
                var_type: signal_var_type(&name),
                name,
                data: ColumnData::Real(values),
            })
            .collect();

        return Ok(ExportTable {
            analysis: "converted".to_string(),
            plot_name: "Converted Data".to_string(),
            scale_type: scale_var_type(&scale_name),
            scale_name,
            scale,
            columns,
        });
    }

    Err(conversion_error(
        path,
        "unrecognized JSON schema: expected 'scale'/'signals' or 'variables'",
    ))
}

fn load_hdf5(path: &Path) -> Result<ExportTable, CliError> {
    let data = read_hdf5(path).map_err(|e| conversion_error(path, e))?;

    let from_section = |section: crate::hdf5::Hdf5WaveformSection, analysis: &str| ExportTable {
        analysis: analysis.to_string(),
        plot_name: if data.title.is_empty() {
            "Converted Data".to_string()
        } else {
            data.title.clone()
        },
        scale_type: scale_var_type(&section.independent_name),
        scale_name: section.independent_name,
        scale: section.independent_values,
        columns: section
            .signals
            .into_iter()
            .map(|signal| ExportColumn {
                var_type: hdf5_signal_var_type(&signal),
                name: signal.name,
                data: ColumnData::Real(signal.values),
            })
            .collect(),
    };

    if let Some(section) = data.transient.clone() {
        return Ok(from_section(section, "transient"));
    }
    if let Some(section) = data.dc_sweep.clone() {
        return Ok(from_section(section, "dc_sweep"));
    }
    if let Some(section) = data.operating_point.clone() {
        return Ok(from_section(section, "dc_op"));
    }
    if let Some(section) = data.noise.clone() {
        return Ok(from_section(section, "noise"));
    }
    if let Some(ac) = data.ac.clone() {
        return Ok(ExportTable {
            analysis: "ac".to_string(),
            plot_name: if data.title.is_empty() {
                "AC Analysis".to_string()
            } else {
                data.title.clone()
            },
            scale_name: "frequency".to_string(),
            scale_type: "frequency".to_string(),
            scale: ac.frequency,
            columns: ac
                .signals
                .into_iter()
                .map(|signal| ExportColumn {
                    var_type: signal_var_type(&signal.name),
                    name: signal.name,
                    data: ColumnData::Complex {
                        real: signal.real,
                        imag: signal.imag,
                    },
                })
                .collect(),
        });
    }

    Err(conversion_error(
        path,
        "HDF5 file did not contain a supported waveform section",
    ))
}

/// `Re(x)` / `Im(x)` helper: returns the inner name when `name` starts with
/// the given prefix and ends with `)`.
fn complex_part_name(name: &str, prefix: &str) -> Option<String> {
    let rest = name.strip_prefix(prefix)?;
    rest.strip_suffix(')').map(|inner| inner.to_string())
}

fn signal_var_type(name: &str) -> String {
    let upper = name.trim_start().to_ascii_uppercase();
    if upper.starts_with("D(") {
        "digital".to_string()
    } else if upper.starts_with("I(") {
        "current".to_string()
    } else {
        "voltage".to_string()
    }
}

fn hdf5_signal_var_type(signal: &crate::hdf5::Hdf5Signal) -> String {
    let var_type = signal.var_type.trim();
    if var_type.is_empty() || var_type.eq_ignore_ascii_case("value") {
        signal_var_type(&signal.name)
    } else {
        var_type.to_string()
    }
}

fn scale_var_type(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    if lower.contains("freq") {
        "frequency".to_string()
    } else {
        "time".to_string()
    }
}
