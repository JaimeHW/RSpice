//! Convert Command - Format conversion
//!
//! Converts between simulation output formats.

use crate::cli::{CliError, ConvertArgs, OutputFormat};
use std::io::Write;

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
    let _from_format = args.from.unwrap_or_else(|| detect_format(&args.input));

    // Read input data
    let data = read_input(&args)?;

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
        _ => OutputFormat::Raw,
    }
}

/// Read input file into waveform data
fn read_input(args: &ConvertArgs) -> Result<WaveformData, CliError> {
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

    let header: Vec<String> = lines[0].split(',').map(|s| s.trim().to_string()).collect();

    let mut independent_values = Vec::new();
    let mut variable_values: Vec<Vec<f64>> = vec![Vec::new(); header.len().saturating_sub(1)];

    for line in lines.iter().skip(1) {
        let values: Vec<&str> = line.split(',').collect();
        if let Some(first) = values.first() {
            if let Ok(v) = first.trim().parse::<f64>() {
                independent_values.push(v);
            }
        }
        for (i, val) in values.iter().skip(1).enumerate() {
            if i < variable_values.len() {
                if let Ok(v) = val.trim().parse::<f64>() {
                    variable_values[i].push(v);
                }
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
    let mut file = std::fs::File::create(&args.output).map_err(|e| CliError::OutputError {
        path: args.output.clone(),
        source: e,
    })?;

    match args.to {
        OutputFormat::Csv => write_csv(&mut file, data, ",")?,
        OutputFormat::Tsv => write_csv(&mut file, data, "\t")?,
        OutputFormat::Json => write_json(&mut file, data)?,
        OutputFormat::Raw | OutputFormat::RawAscii => write_raw(&mut file, data)?,
    }

    Ok(())
}

fn write_csv(file: &mut std::fs::File, data: &WaveformData, sep: &str) -> Result<(), CliError> {
    // Header
    write!(file, "{}", data.independent_var).unwrap();
    for name in &data.variable_names {
        write!(file, "{}{}", sep, name).unwrap();
    }
    writeln!(file).unwrap();

    // Data
    for (i, &x) in data.independent_values.iter().enumerate() {
        write!(file, "{:.9e}", x).unwrap();
        for values in &data.variable_values {
            if let Some(&v) = values.get(i) {
                write!(file, "{}{:.9e}", sep, v).unwrap();
            }
        }
        writeln!(file).unwrap();
    }
    Ok(())
}

fn write_json(file: &mut std::fs::File, data: &WaveformData) -> Result<(), CliError> {
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

    writeln!(file, "{}", serde_json::to_string_pretty(&json).unwrap()).unwrap();
    Ok(())
}

fn write_raw(file: &mut std::fs::File, data: &WaveformData) -> Result<(), CliError> {
    writeln!(file, "Title: RSpice Converted Data").unwrap();
    writeln!(file, "Plotname: Data").unwrap();
    writeln!(file, "Flags: real").unwrap();
    writeln!(file, "No. Variables: {}", data.variable_names.len() + 1).unwrap();
    writeln!(file, "No. Points: {}", data.independent_values.len()).unwrap();
    writeln!(file, "Variables:").unwrap();
    writeln!(file, "\t0\t{}\ttime", data.independent_var).unwrap();
    for (i, name) in data.variable_names.iter().enumerate() {
        writeln!(file, "\t{}\t{}\tvoltage", i + 1, name).unwrap();
    }
    writeln!(file, "Values:").unwrap();
    for (idx, &x) in data.independent_values.iter().enumerate() {
        writeln!(file, "{}", idx).unwrap();
        writeln!(file, "\t{:.9e}", x).unwrap();
        for values in &data.variable_values {
            if let Some(&v) = values.get(idx) {
                writeln!(file, "\t{:.9e}", v).unwrap();
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format() {
        assert_eq!(
            detect_format(std::path::Path::new("test.csv")),
            OutputFormat::Csv
        );
        assert_eq!(
            detect_format(std::path::Path::new("test.raw")),
            OutputFormat::Raw
        );
    }
}
