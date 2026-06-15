//! Convert Command - Format conversion
//!
//! Converts simulation results between every format the CLI can produce:
//! SPICE rawfile (binary or ASCII), CSV, TSV, JSON, and HDF5. Complex AC
//! data survives every round trip; `--variables`, `--start`, and `--stop`
//! select a subset of the data.

use crate::cli::{CliError, ConvertArgs, OutputFormat};
use crate::commands::export_table::{ColumnData, ExportTable};
use crate::commands::waveform_io::{detect_format, load_table};
use crate::hdf5::{Hdf5AcSection, Hdf5SimulationData, Hdf5WaveformSection, write_hdf5};

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

    let from_format = args.from.unwrap_or_else(|| detect_format(&args.input));
    let mut table = load_table(&args.input, from_format)?;

    table.select_variables(&args.variables)?;
    table.clip_scale_range(args.start, args.stop);
    if table.scale.is_empty() {
        return Err(CliError::ConversionError {
            message: "no data points remain after applying --start/--stop".to_string(),
        });
    }

    match args.to {
        OutputFormat::Hdf5 => write_hdf5_output(&args.output, &table)?,
        format => table.write(&args.output, format)?,
    }

    if !quiet {
        println!("✓ Conversion complete: {}", args.output.display());
    }

    Ok(())
}

fn write_hdf5_output(path: &std::path::Path, table: &ExportTable) -> Result<(), CliError> {
    let mut data = Hdf5SimulationData::new();
    data.title = table.plot_name.clone();

    if table.is_complex() {
        let mut section = Hdf5AcSection::new(table.scale.clone());
        for column in &table.columns {
            match &column.data {
                ColumnData::Complex { real, imag } => {
                    section.add_signal(column.name.clone(), real.clone(), imag.clone());
                }
                ColumnData::Real(values) => {
                    section.add_signal(
                        column.name.clone(),
                        values.clone(),
                        vec![0.0; values.len()],
                    );
                }
            }
        }
        data.ac = Some(section);
    } else {
        let mut section = Hdf5WaveformSection::new(table.scale_name.clone(), table.scale.clone());
        for column in &table.columns {
            if let ColumnData::Real(values) = &column.data {
                section.add_signal(column.name.clone(), values.clone());
            }
        }
        data.transient = Some(section);
    }

    write_hdf5(path, &data).map_err(|err| CliError::ConversionError {
        message: err.to_string(),
    })
}
