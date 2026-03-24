//! Convert Command - Format conversion
//!
//! Converts between simulation output formats.

use crate::cli::{CliError, ConvertArgs, OutputFormat};
use crate::hdf5::{Hdf5SimulationData, Hdf5WaveformSection, read_hdf5, write_hdf5};
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
        OutputFormat::Hdf5 => {
            drop(file);
            write_hdf5_output(&args.output, data)?;
        }
    }

    Ok(())
}

fn write_hdf5_output(path: &std::path::Path, data: &WaveformData) -> Result<(), CliError> {
    let mut hdf5_data = Hdf5SimulationData::new();
    hdf5_data.title = "Converted Waveform Data".to_string();

    let mut section =
        Hdf5WaveformSection::new(data.independent_var.clone(), data.independent_values.clone());
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
        variable_names: section.signals.iter().map(|signal| signal.name.clone()).collect(),
        variable_values: section.signals.into_iter().map(|signal| signal.values).collect(),
    }
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
    use crate::hdf5::{Hdf5AcSection, Hdf5SimulationData};

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
        assert_eq!(
            detect_format(std::path::Path::new("test.h5")),
            OutputFormat::Hdf5
        );
    }

    #[test]
    fn test_waveform_data_from_hdf5_transient_section() {
        let mut data = Hdf5SimulationData::new();
        let mut transient = Hdf5WaveformSection::new("time", vec![0.0, 1.0]);
        transient.add_signal("V(out)", vec![0.0, 1.0]);
        data.transient = Some(transient);

        let waveform = waveform_data_from_hdf5(data).expect("transient section should convert");
        assert_eq!(waveform.independent_var, "time");
        assert_eq!(waveform.variable_names, vec!["V(out)"]);
        assert_eq!(waveform.variable_values[0], vec![0.0, 1.0]);
    }

    #[test]
    fn test_waveform_data_from_hdf5_ac_section_flattens_complex_signals() {
        let mut data = Hdf5SimulationData::new();
        let mut ac = Hdf5AcSection::new(vec![1.0, 10.0]);
        ac.add_signal("V(out)", vec![1.0, 0.5], vec![0.0, -0.25]);
        data.ac = Some(ac);

        let waveform = waveform_data_from_hdf5(data).expect("AC section should convert");
        assert_eq!(waveform.independent_var, "frequency");
        assert_eq!(
            waveform.variable_names,
            vec!["V(out).real".to_string(), "V(out).imag".to_string()]
        );
        assert_eq!(waveform.variable_values[0], vec![1.0, 0.5]);
        assert_eq!(waveform.variable_values[1], vec![0.0, -0.25]);
    }

    #[test]
    fn test_write_hdf5_output_round_trip() {
        let temp = tempfile::tempdir().expect("temp dir");
        let path = temp.path().join("converted.h5");
        let waveform = WaveformData {
            independent_var: "time".to_string(),
            independent_values: vec![0.0, 1.0],
            variable_names: vec!["V(out)".to_string()],
            variable_values: vec![vec![0.1, 0.2]],
        };

        write_hdf5_output(&path, &waveform).expect("HDF5 output should succeed");
        let restored = read_hdf5(&path).expect("round-trip read should succeed");
        let section = restored.transient.expect("transient section");
        assert_eq!(section.independent_name, "time");
        assert_eq!(section.signals[0].name, "V(out)");
        assert_eq!(section.signals[0].values, vec![0.1, 0.2]);
    }
}
