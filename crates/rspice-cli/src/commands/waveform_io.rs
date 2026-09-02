//! Read simulation results back into an [`ExportTable`] from any format the
//! CLI can write: SPICE rawfile (binary or ASCII), CSV, TSV, JSON, and HDF5.
//!
//! Used by `convert` and `compare` so every command understands every format.

use crate::cli::{CliError, OutputFormat};
use crate::commands::export_table::{ColumnData, ExportColumn, ExportTable};
use crate::hdf5::read_hdf5;
use std::io::Read;
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
pub(crate) fn load_table(
    path: &Path,
    format: OutputFormat,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    let table = match format {
        OutputFormat::Raw | OutputFormat::RawAscii => load_rawfile(path, resource_limits),
        OutputFormat::Csv => load_delimited(path, ',', resource_limits),
        OutputFormat::Tsv => load_delimited(path, '\t', resource_limits),
        OutputFormat::Json => load_json(path, resource_limits),
        OutputFormat::Hdf5 => load_hdf5(path, resource_limits),
    }?;
    validate_table_shape(path, table, resource_limits)
}

fn conversion_error(path: &Path, message: impl std::fmt::Display) -> CliError {
    CliError::ConversionError {
        message: format!("{}: {}", path.display(), message),
    }
}

fn resource_limit_error(
    path: &Path,
    resource: rspice_core::ResourceKind,
    requested: usize,
    limit: usize,
) -> CliError {
    CliError::ResourceLimit {
        path: path.to_path_buf(),
        source: rspice_core::ResourceLimitError {
            resource,
            requested,
            limit,
        },
    }
}

fn enforce_resource_limit(
    path: &Path,
    resource: rspice_core::ResourceKind,
    requested: usize,
    limit: usize,
) -> Result<(), CliError> {
    if requested <= limit {
        Ok(())
    } else {
        Err(resource_limit_error(path, resource, requested, limit))
    }
}

fn read_utf8_input_limited(path: &Path, limit: usize) -> Result<String, CliError> {
    let file = std::fs::File::open(path).map_err(|source| CliError::InputReadError {
        path: path.to_path_buf(),
        source,
    })?;
    let metadata_bytes = usize::try_from(
        file.metadata()
            .map_err(|source| CliError::InputReadError {
                path: path.to_path_buf(),
                source,
            })?
            .len(),
    )
    .unwrap_or(usize::MAX);
    enforce_resource_limit(
        path,
        rspice_core::ResourceKind::ExternalDataBytes,
        metadata_bytes,
        limit,
    )?;

    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(metadata_bytes)
        .map_err(|error| CliError::InputReadError {
            path: path.to_path_buf(),
            source: std::io::Error::other(format!(
                "unable to reserve {metadata_bytes} bytes for input: {error}"
            )),
        })?;
    let read_limit = u64::try_from(limit).unwrap_or(u64::MAX).saturating_add(1);
    file.take(read_limit)
        .read_to_end(&mut bytes)
        .map_err(|source| CliError::InputReadError {
            path: path.to_path_buf(),
            source,
        })?;
    enforce_resource_limit(
        path,
        rspice_core::ResourceKind::ExternalDataBytes,
        bytes.len(),
        limit,
    )?;
    String::from_utf8(bytes).map_err(|error| CliError::InputReadError {
        path: path.to_path_buf(),
        source: std::io::Error::new(std::io::ErrorKind::InvalidData, error.utf8_error()),
    })
}

fn validate_table_shape(
    path: &Path,
    table: ExportTable,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    let mut retained_values = table.scale.len();
    for column in &table.columns {
        retained_values = retained_values.saturating_add(match &column.data {
            ColumnData::Real(values) => values.len(),
            ColumnData::Complex { real, imag } => real.len().saturating_add(imag.len()),
        });
    }
    enforce_resource_limit(
        path,
        rspice_core::ResourceKind::ExternalDataValues,
        retained_values,
        resource_limits.max_external_data_values,
    )?;
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

fn load_rawfile(
    path: &Path,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    let data = rspice_core::io::parse_raw_file_with_limits(path, resource_limits)
        .map_err(|e| conversion_error(path, e))?;
    if data.header.plotname == "Transient FFT" {
        crate::commands::run::read_fft_raw_artifact(path).map_err(|error| {
            conversion_error(path, format!("invalid typed FFT RAW artifact: {error}"))
        })?;
        return Err(conversion_error(
            path,
            "typed transient FFT RAW artifacts cannot be flattened by generic waveform conversion without losing analysis identity, transform metadata, and FFTOUT metrics",
        ));
    }

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

fn load_delimited(
    path: &Path,
    separator: char,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    let content = read_utf8_input_limited(path, resource_limits.max_external_data_bytes)?;

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
    enforce_resource_limit(
        path,
        rspice_core::ResourceKind::ExternalDataValues,
        header.len(),
        resource_limits.max_external_data_values,
    )?;

    let mut scale = Vec::new();
    let mut series: Vec<Vec<f64>> = vec![Vec::new(); header.len().saturating_sub(1)];
    let mut parsed_values = 0_usize;
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

        parsed_values = parsed_values.saturating_add(fields.len());
        enforce_resource_limit(
            path,
            rspice_core::ResourceKind::ExternalDataValues,
            parsed_values,
            resource_limits.max_external_data_values,
        )?;

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

fn load_json(
    path: &Path,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    let content = read_utf8_input_limited(path, resource_limits.max_external_data_bytes)?;
    let value: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| conversion_error(path, e))?;

    let parsed_values = std::cell::Cell::new(0_usize);
    let to_f64_vec = |value: &serde_json::Value, what: &str| -> Result<Vec<f64>, CliError> {
        let values = value
            .as_array()
            .ok_or_else(|| conversion_error(path, format!("'{}' is not an array", what)))?;
        let requested = parsed_values.get().saturating_add(values.len());
        enforce_resource_limit(
            path,
            rspice_core::ResourceKind::ExternalDataValues,
            requested,
            resource_limits.max_external_data_values,
        )?;
        parsed_values.set(requested);
        values
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

    Err(conversion_error(
        path,
        "unrecognized JSON schema: expected 'scale' and 'signals'",
    ))
}

fn load_hdf5(
    path: &Path,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    let metadata_bytes = usize::try_from(
        std::fs::metadata(path)
            .map_err(|source| CliError::InputReadError {
                path: path.to_path_buf(),
                source,
            })?
            .len(),
    )
    .unwrap_or(usize::MAX);
    enforce_resource_limit(
        path,
        rspice_core::ResourceKind::ExternalDataBytes,
        metadata_bytes,
        resource_limits.max_external_data_bytes,
    )?;
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

    if data.fft.is_some() {
        return Err(conversion_error(
            path,
            "typed transient FFT HDF5 artifacts cannot be flattened by generic waveform conversion without losing analysis identity, transform metadata, and FFTOUT metrics",
        ));
    }

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
    if let Some(distortion) = data.distortion.clone() {
        let mut columns = Vec::new();
        if let Some(ratio) = distortion.f2_over_f1 {
            columns.push(ExportColumn {
                name: "f2_over_f1".to_string(),
                var_type: "ratio".to_string(),
                data: ColumnData::Real(vec![ratio; distortion.f1_frequency.len()]),
            });
        }
        for series in distortion.series {
            if series.label != "f1" {
                columns.push(ExportColumn {
                    name: format!("frequency({})", series.label),
                    var_type: "frequency".to_string(),
                    data: ColumnData::Real(series.physical_frequency),
                });
            }
            for signal in series.signals {
                columns.push(ExportColumn {
                    name: format!("peak({}:{})", series.label, signal.name),
                    var_type: signal.var_type.clone(),
                    data: ColumnData::Complex {
                        real: signal.real,
                        imag: signal.imag,
                    },
                });
                columns.push(ExportColumn {
                    name: format!("magnitude({}:{})", series.label, signal.name),
                    var_type: signal.var_type,
                    data: ColumnData::Real(signal.magnitude),
                });
                columns.push(ExportColumn {
                    name: format!("phase_deg({}:{})", series.label, signal.name),
                    var_type: "phase".to_string(),
                    data: ColumnData::Real(signal.phase_degrees),
                });
                if let Some(ratio) = signal.magnitude_ratio_to_f1 {
                    columns.push(ExportColumn {
                        name: format!("magnitude_ratio_to_f1({}:{})", series.label, signal.name),
                        var_type: "ratio".to_string(),
                        data: ColumnData::Real(ratio),
                    });
                }
            }
        }
        return Ok(ExportTable {
            analysis: "disto".to_string(),
            plot_name: if data.title.is_empty() {
                "Volterra Distortion Analysis".to_string()
            } else {
                data.title.clone()
            },
            scale_name: "frequency(f1)".to_string(),
            scale_type: "frequency".to_string(),
            scale: distortion.f1_frequency,
            columns,
        });
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TempInput(std::path::PathBuf);

    impl TempInput {
        fn new(extension: &str, contents: &str) -> Self {
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock follows Unix epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "rspice-waveform-limit-{}-{nonce}.{extension}",
                std::process::id()
            ));
            std::fs::write(&path, contents).expect("write bounded waveform fixture");
            Self(path)
        }
    }

    impl Drop for TempInput {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    fn limits(bytes: usize, values: usize) -> rspice_core::ResourceLimits {
        let mut limits = rspice_core::ResourceLimits::default();
        limits.max_external_data_bytes = bytes;
        limits.max_external_data_values = values;
        limits
    }

    fn assert_limit(
        error: CliError,
        resource: rspice_core::ResourceKind,
        requested: usize,
        limit: usize,
    ) {
        let CliError::ResourceLimit { source, .. } = error else {
            panic!("expected a structured resource-limit error, got {error}");
        };
        assert_eq!(source.resource, resource);
        assert_eq!(source.requested, requested);
        assert_eq!(source.limit, limit);
    }

    #[test]
    fn text_waveform_inputs_enforce_the_configured_byte_limit() {
        for (extension, format, contents) in [
            ("csv", OutputFormat::Csv, "time,out\n0,1\n"),
            (
                "json",
                OutputFormat::Json,
                r#"{"scale":{"name":"time","values":[0]},"signals":[]}"#,
            ),
        ] {
            let input = TempInput::new(extension, contents);
            let limit = contents.len() - 1;
            let error = match load_table(&input.0, format, limits(limit, usize::MAX)) {
                Err(error) => error,
                Ok(_) => panic!("oversized text waveform must fail before parsing"),
            };
            assert_limit(
                error,
                rspice_core::ResourceKind::ExternalDataBytes,
                contents.len(),
                limit,
            );
        }
    }

    #[test]
    fn text_waveform_inputs_enforce_the_configured_value_limit() {
        for (extension, format, contents) in [
            ("csv", OutputFormat::Csv, "time,out\n0,1\n1,2\n"),
            (
                "json",
                OutputFormat::Json,
                r#"{"scale":{"name":"time","values":[0,1]},"signals":[{"name":"out","values":[1,2]}]}"#,
            ),
        ] {
            let input = TempInput::new(extension, contents);
            let error = match load_table(&input.0, format, limits(contents.len(), 3)) {
                Err(error) => error,
                Ok(_) => panic!("four waveform values must exceed a three-value budget"),
            };
            assert_limit(error, rspice_core::ResourceKind::ExternalDataValues, 4, 3);
        }
    }

    #[test]
    fn combined_hdf5_waveform_and_fft_is_rejected_before_flattening() {
        let input = TempInput::new("h5", "placeholder");
        let mut data = crate::hdf5::Hdf5SimulationData::new();
        data.title = "combined waveform and FFT".to_string();
        let mut transient = crate::hdf5::Hdf5WaveformSection::new("time", vec![0.0, 1.0]);
        transient.add_typed_signal("V(out)", "voltage", vec![0.0, 1.0]);
        data.transient = Some(transient);
        data.fft = Some(crate::hdf5::Hdf5FftSection {
            parent_analysis_id: "tran-001".to_string(),
            coordinate: None,
            results: vec![crate::hdf5::Hdf5FftResult {
                analysis_id: "fft-001".to_string(),
                ordinal: 1,
                source_kind: "probe".to_string(),
                source_text: "V(out)".to_string(),
                authored_output: "V(out)".to_string(),
                output_name: "V(out)".to_string(),
                physical_type: "voltage".to_string(),
                value_unit: Some("V".to_string()),
                start_time_s: 0.0,
                stop_time_s: 1.0,
                sample_interval_s: 0.5,
                point_count: 2,
                accurate_sampling: true,
                format: "normalized".to_string(),
                mode: "hspice_compatible".to_string(),
                window: "rectangular".to_string(),
                window_name: "RECT".to_string(),
                alpha: 3.0,
                coherent_gain: 1.0,
                frequency_resolution_hz: 1.0,
                fundamental_bin: 1,
                minimum_metric_bin: 0,
                maximum_metric_bin: 1,
                bin_indices: vec![0, 1],
                frequency_hz: vec![0.0, 1.0],
                real: vec![0.0, 1.0],
                imaginary: vec![0.0, 0.0],
                magnitude: vec![0.0, 1.0],
                phase_degrees: vec![0.0, 0.0],
                metrics: None,
            }],
        });
        crate::hdf5::write_hdf5(&input.0, &data).expect("write combined HDF5 fixture");

        let error = match load_hdf5(&input.0, rspice_core::ResourceLimits::default()) {
            Err(error) => error,
            Ok(_) => panic!("combined typed FFT data must not be flattened"),
        };
        assert!(
            error.to_string().contains("typed transient FFT HDF5"),
            "unexpected conversion error: {error}"
        );
    }
}
