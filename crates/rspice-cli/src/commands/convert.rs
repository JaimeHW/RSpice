//! Convert Command - Format conversion
//!
//! Converts between simulation output formats.

use crate::cli::{CliError, ConvertArgs, OutputFormat};
use crate::hdf5::{Hdf5SimulationData, Hdf5WaveformSection, read_hdf5, write_hdf5};
use std::fmt;
use std::io::{BufWriter, Write};

/// Execute the convert command
pub fn execute(args: ConvertArgs, _verbose: bool, quiet: bool) -> Result<(), CliError> {
    if !args.input.exists() {
        return Err(CliError::InputNotFound {
            path: args.input.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    if !quiet {
        println!(
            "Converting: {} -> {} ({})",
            args.input.display(),
            args.output.display(),
            format!("{:?}", args.to).to_lowercase()
        );
    }

    // Detect input format
    let from_format = args.from.unwrap_or_else(|| detect_format(&args.input));

    // Read input data
    let data = read_input(&args, from_format)?;

    // Write output
    write_output(&args, &data)?;

    if !quiet {
        println!("✓ Conversion complete: {}", args.output.display());
    }

    Ok(())
}

/// Waveform data structure
struct WaveformData {
    independent_var: String,
    independent_values: Vec<f64>,
    variable_names: Vec<String>,
    variable_values: Vec<Vec<f64>>,
}

/// Detect format from file extension
fn detect_format(path: &std::path::Path) -> OutputFormat {
    match path.extension().and_then(|e| e.to_str()) {
        Some("raw") => OutputFormat::Raw,
        Some("csv") => OutputFormat::Csv,
        Some("json") => OutputFormat::Json,
        Some("tsv") => OutputFormat::Tsv,
        Some("h5") | Some("hdf5") => OutputFormat::Hdf5,
        _ => OutputFormat::Raw,
    }
}

/// Read input file into waveform data
fn read_input(args: &ConvertArgs, from_format: OutputFormat) -> Result<WaveformData, CliError> {
    if matches!(from_format, OutputFormat::Hdf5) {
        let data = read_hdf5(&args.input).map_err(|err| CliError::ConversionError {
            message: err.to_string(),
        })?;
        return waveform_data_from_hdf5(data);
    }

    let separator = if matches!(from_format, OutputFormat::Tsv) {
        '\t'
    } else {
        ','
    };

    let content = std::fs::read_to_string(&args.input).map_err(|e| CliError::InputReadError {
        path: args.input.clone(),
        source: e,
    })?;

    // Simple CSV parser for now
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Err(CliError::ConversionError {
            message: "Empty input file".to_string(),
        });
    }

    let header: Vec<String> = lines[0]
        .split(separator)
        .map(|s| s.trim().to_string())
        .collect();

    let mut independent_values = Vec::new();
    let mut variable_values: Vec<Vec<f64>> = vec![Vec::new(); header.len().saturating_sub(1)];

    for line in lines.iter().skip(1) {
        let values: Vec<&str> = line.split(separator).collect();
        if let Some(first) = values.first()
            && let Ok(v) = first.trim().parse::<f64>()
        {
            independent_values.push(v);
        }
        for (i, val) in values.iter().skip(1).enumerate() {
            if i < variable_values.len()
                && let Ok(v) = val.trim().parse::<f64>()
            {
                variable_values[i].push(v);
            }
        }
    }

    Ok(WaveformData {
        independent_var: header.first().cloned().unwrap_or_else(|| "x".to_string()),
        independent_values,
        variable_names: header.into_iter().skip(1).collect(),
        variable_values,
    })
}

/// Write output file
fn write_output(args: &ConvertArgs, data: &WaveformData) -> Result<(), CliError> {
    match args.to {
        OutputFormat::Csv
        | OutputFormat::Tsv
        | OutputFormat::Json
        | OutputFormat::Raw
        | OutputFormat::RawAscii => {
            let file = std::fs::File::create(&args.output)
                .map_err(|e| CliError::output_error(&args.output, e))?;
            let mut writer = BufWriter::new(file);
            match args.to {
                OutputFormat::Csv => write_csv(&mut writer, &args.output, data, ",")?,
                OutputFormat::Tsv => write_csv(&mut writer, &args.output, data, "\t")?,
                OutputFormat::Json => write_json(&mut writer, &args.output, data)?,
                OutputFormat::Raw | OutputFormat::RawAscii => {
                    write_raw(&mut writer, &args.output, data)?
                }
                OutputFormat::Hdf5 => unreachable!("HDF5 handled separately"),
            }
        }
        OutputFormat::Hdf5 => write_hdf5_output(&args.output, data)?,
    }

    Ok(())
}

fn write_hdf5_output(path: &std::path::Path, data: &WaveformData) -> Result<(), CliError> {
    let mut hdf5_data = Hdf5SimulationData::new();
    hdf5_data.title = "Converted Waveform Data".to_string();

    let mut section = Hdf5WaveformSection::new(
        data.independent_var.clone(),
        data.independent_values.clone(),
    );
    for (name, values) in data.variable_names.iter().zip(&data.variable_values) {
        section.add_signal(name.clone(), values.clone());
    }
    hdf5_data.transient = Some(section);

    write_hdf5(path, &hdf5_data).map_err(|err| CliError::ConversionError {
        message: err.to_string(),
    })
}

fn waveform_data_from_hdf5(data: Hdf5SimulationData) -> Result<WaveformData, CliError> {
    if let Some(section) = data.transient {
        return Ok(waveform_data_from_section(section));
    }
    if let Some(section) = data.dc_sweep {
        return Ok(waveform_data_from_section(section));
    }
    if let Some(section) = data.operating_point {
        return Ok(waveform_data_from_section(section));
    }
    if let Some(ac) = data.ac {
        let mut variable_names = Vec::with_capacity(ac.signals.len() * 2);
        let mut variable_values = Vec::with_capacity(ac.signals.len() * 2);
        for signal in ac.signals {
            variable_names.push(format!("{}.real", signal.name));
            variable_values.push(signal.real);
            variable_names.push(format!("{}.imag", signal.name));
            variable_values.push(signal.imag);
        }
        return Ok(WaveformData {
            independent_var: "frequency".to_string(),
            independent_values: ac.frequency,
            variable_names,
            variable_values,
        });
    }

    Err(CliError::ConversionError {
        message: "HDF5 file did not contain a supported waveform section".to_string(),
    })
}

fn waveform_data_from_section(section: Hdf5WaveformSection) -> WaveformData {
    WaveformData {
        independent_var: section.independent_name,
        independent_values: section.independent_values,
        variable_names: section
            .signals
            .iter()
            .map(|signal| signal.name.clone())
            .collect(),
        variable_values: section
            .signals
            .into_iter()
            .map(|signal| signal.values)
            .collect(),
    }
}

fn write_csv<W: Write>(
    writer: &mut W,
    path: &std::path::Path,
    data: &WaveformData,
    sep: &str,
) -> Result<(), CliError> {
    // Header
    write_fmt(writer, path, format_args!("{}", data.independent_var))?;
    for name in &data.variable_names {
        write_fmt(writer, path, format_args!("{}{}", sep, name))?;
    }
    write_newline(writer, path)?;

    // Data
    for (i, &x) in data.independent_values.iter().enumerate() {
        write_fmt(writer, path, format_args!("{:.9e}", x))?;
        for values in &data.variable_values {
            if let Some(&v) = values.get(i) {
                write_fmt(writer, path, format_args!("{}{:.9e}", sep, v))?;
            }
        }
        write_newline(writer, path)?;
    }
    Ok(())
}

fn write_json<W: Write>(
    writer: &mut W,
    path: &std::path::Path,
    data: &WaveformData,
) -> Result<(), CliError> {
    let mut vars = serde_json::Map::new();
    vars.insert(
        data.independent_var.clone(),
        serde_json::json!(data.independent_values),
    );
    for (name, values) in data.variable_names.iter().zip(&data.variable_values) {
        vars.insert(name.clone(), serde_json::json!(values));
    }

    let json = serde_json::json!({
        "variables": vars,
        "points": data.independent_values.len(),
    });

    serde_json::to_writer_pretty(&mut *writer, &json)
        .map_err(|e| CliError::output_json_error(path, e))?;
    write_newline(writer, path)?;
    Ok(())
}

fn write_raw<W: Write>(
    writer: &mut W,
    path: &std::path::Path,
    data: &WaveformData,
) -> Result<(), CliError> {
    write_line(writer, path, format_args!("Title: RSpice Converted Data"))?;
    write_line(writer, path, format_args!("Plotname: Data"))?;
    write_line(writer, path, format_args!("Flags: real"))?;
    write_line(
        writer,
        path,
        format_args!("No. Variables: {}", data.variable_names.len() + 1),
    )?;
    write_line(
        writer,
        path,
        format_args!("No. Points: {}", data.independent_values.len()),
    )?;
    write_line(writer, path, format_args!("Variables:"))?;
    write_line(
        writer,
        path,
        format_args!("\t0\t{}\ttime", data.independent_var),
    )?;
    for (i, name) in data.variable_names.iter().enumerate() {
        write_line(writer, path, format_args!("\t{}\t{}\tvoltage", i + 1, name))?;
    }
    write_line(writer, path, format_args!("Values:"))?;
    for (idx, &x) in data.independent_values.iter().enumerate() {
        write_line(writer, path, format_args!("{}", idx))?;
        write_line(writer, path, format_args!("\t{:.9e}", x))?;
        for values in &data.variable_values {
            if let Some(&v) = values.get(idx) {
                write_line(writer, path, format_args!("\t{:.9e}", v))?;
            }
        }
    }
    Ok(())
}

fn write_fmt<W: Write>(
    writer: &mut W,
    path: &std::path::Path,
    args: fmt::Arguments<'_>,
) -> Result<(), CliError> {
    writer
        .write_fmt(args)
        .map_err(|e| CliError::output_error(path, e))
}

fn write_newline<W: Write>(writer: &mut W, path: &std::path::Path) -> Result<(), CliError> {
    writer
        .write_all(b"\n")
        .map_err(|e| CliError::output_error(path, e))
}

fn write_line<W: Write>(
    writer: &mut W,
    path: &std::path::Path,
    args: fmt::Arguments<'_>,
) -> Result<(), CliError> {
    write_fmt(writer, path, args)?;
    write_newline(writer, path)
}
