//! Read simulation results back into an [`ExportTable`] from any format the
//! CLI can write: SPICE rawfile (binary or ASCII), CSV, TSV, JSON, HDF5, and
//! VCD.
//!
//! Used by `convert` and `compare` so every command understands every format.
//! A VCD is not a table on disk; [`crate::commands::vcd_io`] makes one out of
//! it, so a dump reads here like everything else.

#[cfg(test)]
use crate::cli::OutputFormat;
use crate::cli::{CliError, InputFormat};
use crate::commands::export_table::{ColumnData, ExportColumn, ExportTable};
use crate::hdf5::read_hdf5_sections_with_limits;
use std::io::Read;
use std::path::Path;

mod snapshot;
mod touchstone;
pub(crate) use snapshot::ResultSnapshot;

pub(crate) enum ImportedResult {
    Table(ExportTable),
    Fft(crate::commands::run::FftBundle),
}

impl From<ExportTable> for ImportedResult {
    fn from(table: ExportTable) -> Self {
        Self::Table(table)
    }
}

/// Guess a format from the file extension; rawfile when unknown.
pub(crate) fn detect_format(path: &Path) -> InputFormat {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .as_deref()
    {
        Some("csv") => InputFormat::Csv,
        Some("tsv") => InputFormat::Tsv,
        Some("json") => InputFormat::Json,
        Some("h5") | Some("hdf5") => InputFormat::Hdf5,
        Some("vcd") => InputFormat::Vcd,
        Some(ext)
            if ext == "ts"
                || ext
                    .strip_prefix('s')
                    .and_then(|ext| ext.strip_suffix('p'))
                    .is_some_and(|ports| {
                        !ports.is_empty() && ports.bytes().all(|byte| byte.is_ascii_digit())
                    }) =>
        {
            InputFormat::Touchstone
        }
        _ => InputFormat::Raw,
    }
}

/// Formats whose readers can select one named or indexed result section.
pub(crate) fn supports_sections(format: InputFormat) -> bool {
    matches!(
        format,
        InputFormat::Raw | InputFormat::RawAscii | InputFormat::Hdf5 | InputFormat::Touchstone
    )
}

/// Load a result file into a table.
pub(crate) fn load_table(
    path: &Path,
    format: impl Into<InputFormat>,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    load_table_selected(path, format, resource_limits, None)
}

/// Select one fully validated container section by exact name or one-based index.
pub(crate) fn load_table_selected(
    path: &Path,
    format: impl Into<InputFormat>,
    resource_limits: rspice_core::ResourceLimits,
    section: Option<&str>,
) -> Result<ExportTable, CliError> {
    match load_result_selected(path, format, resource_limits, section)? {
        ImportedResult::Table(table) => Ok(table),
        ImportedResult::Fft(_) => Err(conversion_error(
            path,
            "typed transient FFT artifacts cannot be flattened for waveform comparison; use convert to retain the complete FFT document",
        )),
    }
}

pub(crate) fn load_result_selected(
    path: &Path,
    format: impl Into<InputFormat>,
    resource_limits: rspice_core::ResourceLimits,
    section: Option<&str>,
) -> Result<ImportedResult, CliError> {
    let format = format.into();
    if section.is_some() && !supports_sections(format) {
        return Err(CliError::InvalidArgument {
            message: format!(
                "--section is not supported for {format:?} input '{}'",
                path.display()
            ),
            suggestion: Some(
                "omit --section for files without RAW, HDF5 or Touchstone sections".into(),
            ),
        });
    }
    let result = match format {
        InputFormat::Raw | InputFormat::RawAscii => load_rawfile(path, resource_limits, section),
        InputFormat::Csv => load_delimited(path, ',', resource_limits),
        InputFormat::Tsv => load_delimited(path, '\t', resource_limits),
        InputFormat::Json => load_json(path, resource_limits),
        InputFormat::Hdf5 => load_hdf5(path, resource_limits, section),
        InputFormat::Vcd => {
            crate::commands::vcd_io::load_vcd_table(path, resource_limits).map(Into::into)
        }
        InputFormat::Touchstone => touchstone::load(path, resource_limits, section).map(Into::into),
    }?;
    validate_result(path, result, resource_limits)
}

fn validate_result(
    path: &Path,
    result: ImportedResult,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ImportedResult, CliError> {
    match result {
        ImportedResult::Table(table) => {
            validate_table_shape(path, table, resource_limits).map(Into::into)
        }
        fft => Ok(fft),
    }
}

pub(crate) fn conversion_error(path: &Path, message: impl std::fmt::Display) -> CliError {
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

pub(crate) fn enforce_resource_limit(
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

pub(crate) fn read_utf8_input_limited(path: &Path, limit: usize) -> Result<String, CliError> {
    String::from_utf8(read_input_bytes_limited(path, limit)?).map_err(|error| {
        CliError::InputReadError {
            path: path.to_path_buf(),
            source: std::io::Error::new(std::io::ErrorKind::InvalidData, error.utf8_error()),
        }
    })
}

fn read_input_bytes_limited(path: &Path, limit: usize) -> Result<Vec<u8>, CliError> {
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
    Ok(bytes)
}

/// The fewest coordinate samples a result may have and still be a result.
///
/// **One.** A `.OP` is a single point by construction, and the command line
/// writes it in every table format and reads every one of them back; so a rule
/// that wanted two would have to make an exception for the one analysis whose
/// result is always one sample, and an export the product refuses to reopen is
/// worse than a small file. A caller who wants two points asks for two points.
///
/// Zero is not a result. An empty coordinate has no row to write, no shape for
/// a column to match, and nothing for `--start`/`--stop` to clip, so it is
/// refused where it is read rather than after a later step fails for a reason
/// that is not the real one.
///
/// The GUI's result *importer* agrees: its own `MIN_RESULT_ROWS` is one, and
/// its doc comment names this constant as the thing it agrees with. One rule,
/// stated on both sides of the boundary a file crosses.
pub(crate) const MIN_RESULT_SAMPLES: usize = 1;

fn validate_table_shape(
    path: &Path,
    table: ExportTable,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    if table.scale.len() < MIN_RESULT_SAMPLES {
        return Err(conversion_error(
            path,
            format!(
                "the '{}' coordinate carries no samples, so the file holds no result to convert",
                table.scale_name
            ),
        ));
    }
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

/// The default is safe only when the container carries one result. Names are
/// exact; a one-based index can disambiguate repeated names in a RAW file.
pub(super) fn select_section(
    path: &Path,
    names: &[&str],
    selector: Option<&str>,
) -> Result<usize, CliError> {
    if selector.is_none() && names.len() == 1 {
        return Ok(0);
    }
    if let Some(selector) = selector {
        if let Ok(index) = selector.parse::<usize>() {
            if index > 0 && index <= names.len() {
                return Ok(index - 1);
            }
        } else {
            let mut matches = names
                .iter()
                .enumerate()
                .filter(|(_, name)| **name == selector);
            if let Some((index, _)) = matches.next()
                && matches.next().is_none()
            {
                return Ok(index);
            }
        }
    }
    let available = names
        .iter()
        .enumerate()
        .map(|(index, name)| format!("{}: {name}", index + 1))
        .collect::<Vec<_>>()
        .join(", ");
    Err(conversion_error(
        path,
        format!(
            "{}; use --section with an exact name or one-based index. Available sections: [{available}]",
            selector.map_or_else(
                || format!("file contains {} result sections", names.len()),
                |value| format!("section '{value}' is missing or ambiguous")
            )
        ),
    ))
}

fn load_rawfile(
    path: &Path,
    resource_limits: rspice_core::ResourceLimits,
    section: Option<&str>,
) -> Result<ImportedResult, CliError> {
    let file = rspice_core::io::parse_raw_plots_file_with_limits(path, resource_limits)
        .map_err(|error| raw_read_error(path, error))?;
    raw_result(path, file, section)
}

/// Preserve parser admission and I/O errors for every RAW read path.
pub(crate) fn raw_read_error(path: &Path, error: rspice_core::io::RawParseError) -> CliError {
    match error {
        rspice_core::io::RawParseError::ResourceLimit(source) => CliError::ResourceLimit {
            path: path.to_owned(),
            source,
        },
        rspice_core::io::RawParseError::Io(source) => CliError::InputReadError {
            path: path.to_owned(),
            source,
        },
        error => conversion_error(path, error),
    }
}

fn raw_result(
    path: &Path,
    file: rspice_core::io::ltspice_raw::RawFile,
    section: Option<&str>,
) -> Result<ImportedResult, CliError> {
    rspice_core::execution::decode_event_plots(&file)
        .map_err(|error| conversion_error(path, error))?;
    // Validate every typed plot before choosing one, including unselected FFTs.
    let mut fft_plots = std::collections::BTreeMap::new();
    for (index, plot) in file.plots.iter().enumerate() {
        if plot.header.plotname == "Transient FFT" {
            let decoded = crate::commands::run::decode_fft_raw_plot(plot)
                .and_then(crate::commands::run::FftBundle::from_raw)
                .map_err(|error| conversion_error(path, error))?;
            fft_plots.insert(index, decoded);
        }
    }
    let names: Vec<_> = file
        .plots
        .iter()
        .map(|plot| plot.header.plotname.as_str())
        .collect();
    let index = select_section(path, &names, section)?;
    let data = file
        .plots
        .into_iter()
        .nth(index)
        .expect("selected existing plot");
    let units = rspice_core::io::ltspice_raw::raw_table_units(&data.header)
        .map_err(|error| conversion_error(path, error))?
        .unwrap_or_else(|| vec![None; data.variables.len()]);
    if let Some(fft) = fft_plots.remove(&index) {
        return Ok(ImportedResult::Fft(fft));
    }

    let operating_point = matches!(
        data.header.plotname.trim().to_ascii_lowercase().as_str(),
        "dc op" | "operating point" | "dc operating point"
    );
    let mut waveforms = data.waveforms.into_iter().peekable();
    let Some(first) = waveforms.peek() else {
        return Err(conversion_error(path, "rawfile contains no variables"));
    };
    let (scale_name, scale_type, scale) = if operating_point {
        (
            "point".to_string(),
            "index".to_string(),
            (0..first.y.len()).map(|index| index as f64).collect(),
        )
    } else {
        let first = waveforms.next().expect("first waveform was checked");
        (
            first.name,
            data.variables
                .first()
                .map(|v| v.var_type.clone())
                .unwrap_or_else(|| "time".to_string()),
            first.y,
        )
    };

    let columns = waveforms
        .zip(data.variables.iter().skip(usize::from(!operating_point)))
        .map(|(waveform, variable)| ExportColumn {
            unit: units[variable.index].clone(),
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
        scale_unit: if operating_point {
            None
        } else {
            units[0].clone()
        },
        analysis: "converted".to_string(),
        plot_name: if data.header.plotname.is_empty() {
            "Converted Data".to_string()
        } else {
            data.header.plotname
        },
        scale_name,
        scale_type,
        scale,
        columns,
    }
    .into())
}

/// The transposed table an operating point publishes, read back as a table.
///
/// `run -f csv` writes a `.OP` as `signal,value` rows rather than as one wide
/// row, because a single point read down a column is what a reader of an
/// operating point wants. That shape is not a second CSV dialect being
/// invented here: it is the one this command line writes and, before this,
/// could not read — `convert op.csv --to csv` refused its own output with
/// "non-numeric value 'V(IN)' in column 'signal'".
///
/// It comes back as the same table the HDF5 artifact of that run comes back
/// as: one `point` sample at zero, one column per signal. So all six formats
/// an operating point is published in read back to one result.
///
/// `Ok(None)` means this is not that shape, and the ordinary numeric reader
/// takes over. The discriminator is both halves of the header *and* a
/// non-numeric first field: a genuine numeric table whose columns happen to be
/// called `signal` and `value` still reads as a table.
fn load_operating_point_report(
    path: &Path,
    content: &str,
    separator: char,
    header: &[String],
) -> Result<Option<ExportTable>, CliError> {
    if header.len() != 2
        || !header[0].trim().eq_ignore_ascii_case("signal")
        || !header[1].trim().eq_ignore_ascii_case("value")
    {
        return Ok(None);
    }

    let rows: Vec<&str> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .skip(1)
        .collect();
    let Some(first) = rows.first() else {
        return Ok(None);
    };
    let first = parse_delimited_record(first, separator)
        .map_err(|message| conversion_error(path, format!("row 2: {message}")))?;
    if first
        .first()
        .is_some_and(|name| name.trim().parse::<f64>().is_ok())
    {
        return Ok(None);
    }

    let mut columns = Vec::new();
    for (row, line) in rows.iter().enumerate() {
        let line_number = row + 2;
        let fields = parse_delimited_record(line, separator)
            .map_err(|message| conversion_error(path, format!("row {line_number}: {message}")))?;
        if fields.len() != 2 {
            return Err(conversion_error(
                path,
                format!(
                    "row {line_number} has {} columns; an operating point states one signal and \
                     one value per row",
                    fields.len()
                ),
            ));
        }
        let name = fields[0].trim();
        if name.is_empty() {
            return Err(conversion_error(
                path,
                format!("row {line_number} names no signal"),
            ));
        }
        let token = fields[1].trim();
        let value = token.parse::<f64>().map_err(|_| {
            conversion_error(
                path,
                format!("non-numeric value '{token}' for signal '{name}', row {line_number}"),
            )
        })?;
        if !value.is_finite() {
            return Err(conversion_error(
                path,
                format!("non-finite value '{token}' for signal '{name}', row {line_number}"),
            ));
        }
        columns.push(ExportColumn {
            unit: None,
            name: name.to_string(),
            var_type: signal_var_type(name),
            data: ColumnData::Real(vec![value]),
        });
    }

    Ok(Some(ExportTable {
        scale_unit: None,
        analysis: "dc_op".to_string(),
        plot_name: "DC Operating Point".to_string(),
        scale_name: "point".to_string(),
        scale_type: "index".to_string(),
        scale: vec![0.0],
        columns,
    }))
}

fn load_delimited(
    path: &Path,
    separator: char,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ImportedResult, CliError> {
    let content = read_utf8_input_limited(path, resource_limits.max_external_data_bytes)?;
    parse_delimited(path, &content, separator, resource_limits)
}

fn parse_delimited(
    path: &Path,
    content: &str,
    separator: char,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ImportedResult, CliError> {
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
    if crate::commands::run::FftBundle::is_delimited(&header) {
        return crate::commands::run::FftBundle::from_delimited(
            path,
            content,
            separator,
            resource_limits,
        )
        .map(ImportedResult::Fft);
    }
    enforce_resource_limit(
        path,
        rspice_core::ResourceKind::ExternalDataValues,
        header.len(),
        resource_limits.max_external_data_values,
    )?;

    if let Some(table) = load_operating_point_report(path, content, separator, &header)? {
        return Ok(table.into());
    }

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
                unit: None,
                var_type: signal_var_type(&inner),
                name: inner,
                data: ColumnData::Complex { real: values, imag },
            });
            continue;
        }
        columns.push(ExportColumn {
            unit: None,
            name: name.clone(),
            var_type: signal_var_type(name),
            data: ColumnData::Real(values),
        });
    }

    let scale_name = header[0].clone();
    Ok(ExportTable {
        scale_unit: None,
        analysis: "converted".to_string(),
        plot_name: "Converted Data".to_string(),
        scale_type: scale_var_type(&scale_name),
        scale_name,
        scale,
        columns,
    }
    .into())
}

pub(crate) fn parse_delimited_record(line: &str, separator: char) -> Result<Vec<String>, String> {
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
) -> Result<ImportedResult, CliError> {
    let content = read_utf8_input_limited(path, resource_limits.max_external_data_bytes)?;
    parse_json(path, &content, resource_limits)
}

fn parse_json(
    path: &Path,
    content: &str,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ImportedResult, CliError> {
    let value: serde_json::Value =
        serde_json::from_str(content).map_err(|e| conversion_error(path, e))?;
    let read_unit = |object: &serde_json::Value| -> Result<Option<String>, CliError> {
        match object.get("unit") {
            None | Some(serde_json::Value::Null) => Ok(None),
            Some(serde_json::Value::String(unit)) if !unit.trim().is_empty() => {
                Ok(Some(unit.trim().to_owned()))
            }
            _ => Err(conversion_error(
                path,
                "unit must be a nonempty string or null",
            )),
        }
    };
    if value.get("analysis").and_then(serde_json::Value::as_str) == Some("fft") {
        return crate::commands::run::FftBundle::from_json(path, value, resource_limits)
            .map(ImportedResult::Fft);
    }

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
                unit: read_unit(signal)?,
                var_type: signal
                    .get("type")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned)
                    .unwrap_or_else(|| signal_var_type(&name)),
                name,
                data,
            });
        }

        return Ok(ExportTable {
            scale_unit: read_unit(scale_obj)?,
            analysis: value
                .get("analysis")
                .and_then(|v| v.as_str())
                .unwrap_or("converted")
                .to_string(),
            plot_name: value
                .get("plot_name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("Converted Data")
                .to_owned(),
            scale_type: scale_obj
                .get("type")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
                .unwrap_or_else(|| scale_var_type(&scale_name)),
            scale_name,
            scale,
            columns,
        }
        .into());
    }

    // A `run` artifact is a shared typed result document. Flatten its axis and
    // series into the same table so `convert` and `compare` read what `run`
    // wrote. A series the document declares as not retained has no samples and
    // becomes no column, rather than a column of zeros.
    if value.get("schema").and_then(serde_json::Value::as_str)
        == Some(rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_SCHEMA)
    {
        let document =
            rspice_core::execution::AnalysisResultDocument::from_json_with_limits_and_abort(
                content,
                &resource_limits,
                &crate::abort::ProcessAbort,
                resource_limits.max_external_data_bytes as u64,
            )
            .map_err(|error| match error {
                rspice_core::execution::ResultDocumentError::ResourceLimit(source) => {
                    CliError::ResourceLimit {
                        path: path.to_path_buf(),
                        source,
                    }
                }
                error => conversion_error(path, error),
            })?;
        return result_document_table(path, &document, resource_limits).map(Into::into);
    }

    Err(conversion_error(
        path,
        "unrecognized JSON schema: expected a typed result document, or 'scale' and 'signals'",
    ))
}

/// Project signed or unsigned 64-bit integers only when binary64 is exact.
/// A wider round trip avoids saturating i64::MAX/u64::MAX casts, and permits
/// representable multiples above 2^53 instead of rejecting all large integers.
fn exact_integer_sample(value: impl Into<i128>) -> Option<f64> {
    let integer = value.into();
    let sample = integer as f64;
    (sample as i128 == integer).then_some(sample)
}

/// Flatten one typed result document into the shared tabular model.
///
/// A document whose family carries no coordinate axis — the operating point,
/// the transfer function — flattens onto a single-point index axis, which is
/// what the flat writers already use for those families.
fn result_document_table(
    path: &Path,
    document: &rspice_core::execution::AnalysisResultDocument,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    use rspice_core::execution::result_document::{
        AxisValues, ResultPayload, ScalarValue, SeriesValues,
    };

    if document.axes().len() > 1 {
        return Err(conversion_error(
            path,
            "a multi-axis result cannot be represented by a single flat table",
        ));
    }

    let (scale_name, scale): (String, Vec<f64>) = match document.axes().first() {
        Some(axis) => (
            axis.name().to_string(),
            match axis.values() {
                AxisValues::Real { values } => values.clone(),
                AxisValues::Integer { values } => {
                    values.iter().map(|value| {
                        exact_integer_sample(*value).ok_or_else(|| conversion_error(
                            path,
                            format!("axis '{}' coordinate {value} cannot be represented exactly by a numeric flat table", axis.name()),
                        ))
                    }).collect::<Result<Vec<_>, _>>()?
                }
            },
        ),
        None => ("point".to_string(), vec![0.0]),
    };
    enforce_resource_limit(
        path,
        rspice_core::ResourceKind::ExternalDataValues,
        scale.len().saturating_mul(
            document
                .signals()
                .len()
                .saturating_add(document.scalars().len())
                .saturating_add(1),
        ),
        resource_limits.max_external_data_values,
    )?;

    let mut columns = Vec::new();
    for signal in document.signals() {
        let name = signal.descriptor().display_name().to_string();
        let present = |samples: &[Option<f64>]| -> Result<Vec<f64>, CliError> {
            samples
                .iter()
                .map(|sample| {
                    sample.ok_or_else(|| {
                        conversion_error(
                            path,
                            format!(
                                "series '{name}' has an absent sample, which a flat table cannot represent"
                            ),
                        )
                    })
                })
                .collect()
        };
        let data = match signal.values() {
            SeriesValues::Real { samples } => {
                if samples.iter().all(Option::is_none) {
                    continue;
                }
                ColumnData::Real(present(samples)?)
            }
            SeriesValues::Complex { samples } => {
                if samples.iter().all(Option::is_none) {
                    continue;
                }
                let mut real = Vec::with_capacity(samples.len());
                let mut imag = Vec::with_capacity(samples.len());
                for sample in samples {
                    let sample = sample.ok_or_else(|| {
                        conversion_error(
                            path,
                            format!(
                                "series '{name}' has an absent sample, which a flat table cannot represent"
                            ),
                        )
                    })?;
                    real.push(sample.real);
                    imag.push(sample.imaginary);
                }
                ColumnData::Complex { real, imag }
            }
            other => {
                return Err(conversion_error(
                    path,
                    format!(
                        "series '{name}' has a representation no flat table carries: {other:?}"
                    ),
                ));
            }
        };
        columns.push(ExportColumn {
            unit: crate::commands::export_table::stated_unit(signal.descriptor().unit()),
            var_type: crate::commands::export_table::unit_type(
                signal.descriptor().unit(),
                rspice_core::execution::raw_variable_type(signal.descriptor().kind()),
            )
            .to_string(),
            name,
            data,
        });
    }

    for scalar in document.scalars() {
        let unsupported = || {
            conversion_error(
                path,
                format!(
                    "scalar '{}' cannot be represented exactly by a numeric flat table",
                    scalar.name()
                ),
            )
        };
        let data = match scalar.value() {
            ScalarValue::Real { value: Some(value) } => ColumnData::Real(vec![*value; scale.len()]),
            ScalarValue::Complex { value: Some(value) } => ColumnData::Complex {
                real: vec![value.real; scale.len()],
                imag: vec![value.imaginary; scale.len()],
            },
            ScalarValue::Integer { value } => ColumnData::Real(vec![
                exact_integer_sample(*value)
                    .ok_or_else(unsupported)?;
                scale.len()
            ]),
            ScalarValue::Count { value } => ColumnData::Real(vec![
                exact_integer_sample(*value)
                    .ok_or_else(unsupported)?;
                scale.len()
            ]),
            ScalarValue::Boolean { value } => {
                ColumnData::Real(vec![u8::from(*value) as f64; scale.len()])
            }
            _ => return Err(unsupported()),
        };
        // Keep the established .TF table names so formats emitted by the same
        // analysis also compare against one another.
        let name = match document.payload() {
            ResultPayload::Tf(payload) => match scalar.name() {
                "transfer_gain" => "transfer_function".to_string(),
                "input_impedance" => format!("{}#input_impedance", payload.input.to_lowercase()),
                "output_impedance" => {
                    format!("output_impedance_at_{}", payload.output.to_lowercase())
                }
                _ => scalar.name().to_string(),
            },
            _ => scalar.name().to_string(),
        };
        columns.push(ExportColumn {
            unit: scalar
                .unit()
                .and_then(crate::commands::export_table::stated_unit),
            name,
            var_type: scalar
                .unit()
                .map_or("value", |unit| {
                    crate::commands::export_table::unit_type(unit, "value")
                })
                .to_string(),
            data,
        });
    }
    if columns.is_empty() {
        return Err(conversion_error(
            path,
            "result contains no retained numeric quantities that a flat table can represent",
        ));
    }

    Ok(ExportTable {
        scale_unit: document
            .axes()
            .first()
            .and_then(|axis| crate::commands::export_table::stated_unit(axis.unit())),
        analysis: document.result_kind().tag().to_string(),
        plot_name: format!("{} ({})", document.result_kind().tag(), document.analysis()),
        scale_type: document
            .axes()
            .first()
            .map_or("index", |axis| {
                crate::commands::export_table::unit_type(axis.unit(), "parameter")
            })
            .to_string(),
        scale_name,
        scale,
        columns,
    })
}

fn load_hdf5(
    path: &Path,
    resource_limits: rspice_core::ResourceLimits,
    section: Option<&str>,
) -> Result<ImportedResult, CliError> {
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
    let readback =
        read_hdf5_sections_with_limits(path, resource_limits).map_err(|error| match error {
            crate::hdf5::Hdf5Error::ResourceLimit(source) => CliError::ResourceLimit {
                path: path.to_path_buf(),
                source,
            },
            error => conversion_error(path, error),
        })?;

    hdf5_result(path, readback, section)
}

fn hdf5_result(
    path: &Path,
    readback: crate::hdf5::Hdf5Readback,
    section: Option<&str>,
) -> Result<ImportedResult, CliError> {
    let crate::hdf5::Hdf5Readback { metadata, sections } = readback;
    let names: Vec<_> = sections.iter().map(|(name, _)| name.as_str()).collect();
    let index = select_section(path, &names, section)?;
    let (_, mut data) = sections
        .into_iter()
        .nth(index)
        .expect("selected existing section");
    data.title = metadata.title;
    data.identity = metadata.identity;
    if let Some(fft) = data.fft.take() {
        return crate::commands::run::FftBundle::from_section(fft)
            .map(ImportedResult::Fft)
            .map_err(|error| conversion_error(path, error));
    }
    hdf5_table(path, data).map(Into::into)
}

fn hdf5_table(path: &Path, data: crate::hdf5::Hdf5SimulationData) -> Result<ExportTable, CliError> {
    let from_section = |section: crate::hdf5::Hdf5WaveformSection, analysis: &str| ExportTable {
        scale_unit: None,
        analysis: analysis.to_string(),
        plot_name: if data.title.is_empty() {
            "Converted Data".to_string()
        } else {
            data.title.clone()
        },
        scale_type: scale_var_type(&section.independent_name),
        scale_name: section.independent_name,
        scale: section.independent_values,
        columns: decode_hdf5_columns(section.signals),
    };

    if let Some(table) = data.table {
        return Ok(ExportTable {
            scale_unit: table.coordinate_unit,
            analysis: table.analysis,
            plot_name: data.title,
            scale_name: table.waveform.independent_name,
            scale_type: table.coordinate_type,
            scale: table.waveform.independent_values,
            columns: decode_hdf5_columns(table.waveform.signals),
        });
    }
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
                unit: None,
                name: "f2_over_f1".to_string(),
                var_type: "ratio".to_string(),
                data: ColumnData::Real(vec![ratio; distortion.f1_frequency.len()]),
            });
        }
        for series in distortion.series {
            if series.label != "f1" {
                columns.push(ExportColumn {
                    unit: None,
                    name: format!("frequency({})", series.label),
                    var_type: "frequency".to_string(),
                    data: ColumnData::Real(series.physical_frequency),
                });
            }
            for signal in series.signals {
                columns.push(ExportColumn {
                    unit: None,
                    name: format!("peak({}:{})", series.label, signal.name),
                    var_type: signal.var_type.clone(),
                    data: ColumnData::Complex {
                        real: signal.real,
                        imag: signal.imag,
                    },
                });
                columns.push(ExportColumn {
                    unit: None,
                    name: format!("magnitude({}:{})", series.label, signal.name),
                    var_type: signal.var_type,
                    data: ColumnData::Real(signal.magnitude),
                });
                columns.push(ExportColumn {
                    unit: None,
                    name: format!("phase_deg({}:{})", series.label, signal.name),
                    var_type: "phase".to_string(),
                    data: ColumnData::Real(signal.phase_degrees),
                });
                if let Some(ratio) = signal.magnitude_ratio_to_f1 {
                    columns.push(ExportColumn {
                        unit: None,
                        name: format!("magnitude_ratio_to_f1({}:{})", series.label, signal.name),
                        var_type: "ratio".to_string(),
                        data: ColumnData::Real(ratio),
                    });
                }
            }
        }
        return Ok(ExportTable {
            scale_unit: None,
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
            scale_unit: None,
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
                    unit: signal.unit,
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

/// A general-coordinate complex conversion stores marked real/imaginary
/// columns in the ordinary waveform schema, without pretending time is hertz.
fn decode_hdf5_columns(signals: Vec<crate::hdf5::Hdf5Signal>) -> Vec<ExportColumn> {
    let mut columns = Vec::new();
    let mut signals = signals.into_iter().peekable();
    while let Some(signal) = signals.next() {
        if let Some(var_type) = signal.var_type.strip_prefix("complex_real:")
            && let Some(name) = complex_part_name(&signal.name, "Re(")
            && signals.peek().is_some_and(|imag| {
                imag.var_type == format!("complex_imag:{var_type}")
                    && imag.unit == signal.unit
                    && complex_part_name(&imag.name, "Im(").as_deref() == Some(&name)
            })
        {
            let imag = signals.next().expect("matching imaginary column");
            columns.push(ExportColumn {
                unit: signal.unit.clone(),
                name,
                var_type: var_type.to_string(),
                data: ColumnData::Complex {
                    real: signal.values,
                    imag: imag.values,
                },
            });
        } else {
            columns.push(ExportColumn {
                unit: signal.unit.clone(),
                var_type: hdf5_signal_var_type(&signal),
                name: signal.name,
                data: ColumnData::Real(signal.values),
            });
        }
    }
    columns
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
    } else if upper.starts_with("V(") {
        "voltage".to_string()
    } else {
        "value".to_string()
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
    match lower.as_str() {
        "frequency" | "freq" | "frequency_hz" | "hz" => "frequency",
        "time" | "time_s" | "t" => "time",
        "point" | "index" | "tuple_index" => "index",
        _ => "value",
    }
    .to_string()
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

    /// `convert` and `compare` resolve a file's format from its extension, so
    /// the extension table is the only thing that decides whether a dump is
    /// read as a dump or as a rawfile.
    #[test]
    fn every_written_format_is_recognised_by_the_extension_it_is_written_under() {
        for (extension, expected) in [
            ("csv", OutputFormat::Csv),
            ("CSV", OutputFormat::Csv),
            ("tsv", OutputFormat::Tsv),
            ("json", OutputFormat::Json),
            ("h5", OutputFormat::Hdf5),
            ("hdf5", OutputFormat::Hdf5),
            ("vcd", OutputFormat::Vcd),
            ("VCD", OutputFormat::Vcd),
            ("raw", OutputFormat::Raw),
            ("out", OutputFormat::Raw),
        ] {
            assert_eq!(
                detect_format(Path::new(&format!("result.{extension}"))),
                InputFormat::from(expected),
                "unexpected format for .{extension}"
            );
        }
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
        transient.add_typed_signal("V(out)", "voltage", Some("V".to_string()), vec![0.0, 1.0]);
        data.transient = Some(transient);
        data.fft = Some(crate::hdf5::Hdf5FftSection {
            parent_analysis_id: "tran-001".to_string(),
            coordinate: None,
            results: vec![crate::hdf5::Hdf5FftResult {
                status: rspice_core::engine::TransientFftStatus::Complete,
                analysis_id: "fft-001".to_string(),
                ordinal: 1,
                source_kind: "probe".to_string(),
                source_text: "V(out)".to_string(),
                authored_output: "V(out)".to_string(),
                output_name: "V(out)".to_string(),
                physical_type: "voltage".to_string(),
                value_unit: Some("1".to_string()),
                start_time_s: 0.0,
                stop_time_s: 1.0,
                sample_interval_s: 0.25,
                point_count: 4,
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
                maximum_metric_bin: 2,
                sfdr_search_minimum_bin: 1,
                bin_indices: vec![0, 1, 2],
                frequency_hz: vec![0.0, 1.0, 2.0],
                real: vec![0.0, 1.0, 0.0],
                imaginary: vec![0.0, 0.0, 0.0],
                magnitude: vec![0.0, 1.0, 0.0],
                phase_degrees: vec![0.0, 0.0, 0.0],
                metrics: None,
            }],
        });
        crate::hdf5::write_hdf5(&input.0, &data).expect("write combined HDF5 fixture");

        let error = match load_hdf5(&input.0, rspice_core::ResourceLimits::default(), None) {
            Err(error) => error,
            Ok(_) => panic!("combined typed FFT data must not be flattened"),
        };
        assert!(
            error.to_string().contains("2 result sections"),
            "unexpected conversion error: {error}"
        );
    }

    #[test]
    fn native_hdf5_signal_units_survive_tabular_import() {
        for complex in [false, true] {
            let input = TempInput::new("h5", "placeholder");
            let mut data = crate::hdf5::Hdf5SimulationData::new();
            data.title = "unit-preserving import".into();
            if complex {
                let mut section = crate::hdf5::Hdf5AcSection::new(vec![1.0, 2.0]);
                section.add_signal("probe", Some("mV".into()), vec![3.0, 4.0], vec![1.0, 2.0]);
                data.ac = Some(section);
            } else {
                let mut section = crate::hdf5::Hdf5WaveformSection::new("time", vec![0.0, 1.0]);
                section.add_typed_signal("probe", "voltage", Some("mV".into()), vec![3.0, 4.0]);
                data.transient = Some(section);
            }
            crate::hdf5::write_hdf5(&input.0, &data).unwrap();
            let table = load_table(
                &input.0,
                OutputFormat::Hdf5,
                rspice_core::ResourceLimits::default(),
            )
            .unwrap();
            assert_eq!(table.columns[0].unit.as_deref(), Some("mV"));
        }
    }
}
