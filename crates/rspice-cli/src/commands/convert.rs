//! Convert Command - Format conversion
//!
//! Converts simulation results between every format the CLI can produce:
//! SPICE rawfile (binary or ASCII), CSV, TSV, JSON, HDF5, and VCD. Complex AC
//! data survives every round trip; `--variables`, `--start`, and `--stop`
//! select a subset of the data.
//!
//! VCD is the one target that is not a table, so it is built from the event
//! timelines rather than from the tabular model — exactly when the source
//! carries them, and from the flattened `D(node)` / `E(node)` grid columns
//! when it does not. [`crate::commands::vcd_io`] holds both directions and
//! states what each one keeps.

use crate::cli::{CliError, Config, ConvertArgs, OutputFormat};
use crate::commands::vcd_io;
use crate::commands::waveform_io::{ImportedResult, detect_format, load_result_selected};

/// Execute the convert command
pub fn execute(
    args: ConvertArgs,
    config: &Config,
    _verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    validate_range(args.start, args.stop)?;
    crate::commands::publish::destinations::protect_sources(&args.output, config.source_paths())?;
    if !args.input.exists() {
        return Err(CliError::InputNotFound {
            path: args.input.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    if !quiet {
        crate::console::line(format_args!(
            "Converting: {} -> {} ({})",
            args.input.display(),
            args.output.display(),
            format!("{:?}", args.to).to_lowercase()
        ))?;
    }

    let from_format = args.from.unwrap_or_else(|| detect_format(&args.input));

    if args.expand_buses {
        vcd_io::expand_buses_needs_vcd("--expand-buses", args.to)?;
    }

    if args.to == OutputFormat::Vcd && args.section.is_none() {
        let mut document = vcd_io::load_vcd_document(
            &args.input,
            from_format,
            config.resources.limits(),
            args.expand_buses,
        )?;
        let notes = vcd_io::select_and_clip(&mut document, &args.variables, args.start, args.stop)?;
        // Not gated on `--quiet`: the selection is wider than what was asked
        // for, and silently writing more than a caller requested is the thing
        // the note exists to prevent.
        for note in &notes {
            crate::observability::diagnostic("conversion_note", None, format_args!("Note: {note}"));
        }
        vcd_io::write_vcd_artifact(&args.output, &document)?;
        if !quiet {
            crate::console::line(format_args!(
                "✓ Conversion complete: {}",
                args.output.display()
            ))?;
        }
        return Ok(());
    }

    let imported = load_result_selected(
        &args.input,
        from_format,
        config.resources.limits(),
        args.section.as_deref(),
    )?;

    let mut table = match imported {
        ImportedResult::Table(table) => table,
        ImportedResult::Fft(fft) => {
            if !args.variables.is_empty() || args.start.is_some() || args.stop.is_some() {
                return Err(CliError::ConversionError { message: "FFT conversion preserves the complete transform; --variables, --start, and --stop apply only to waveform tables".into() });
            }
            fft.write(&args.output, args.to)?;
            if !quiet {
                crate::console::line(format_args!(
                    "✓ Conversion complete: {}",
                    args.output.display()
                ))?;
            }
            return Ok(());
        }
    };

    table.select_variables(&args.variables)?;
    match args.to {
        OutputFormat::Vcd => {
            // Project before clipping so a selected container section retains
            // the same held state as an ordinary event-document conversion.
            let mut document = vcd_io::table_document(&args.input, &table)?;
            let _notes = vcd_io::select_and_clip(&mut document, &[], args.start, args.stop)?;
            vcd_io::write_vcd_artifact(&args.output, &document)?;
        }
        format => {
            table.clip_scale_range(args.start, args.stop);
            // Input admission already refused empty tables.
            if table.scale.len() < crate::commands::waveform_io::MIN_RESULT_SAMPLES {
                return Err(CliError::ConversionError {
                    message: "no data points remain after applying --start/--stop".to_string(),
                });
            }
            table.write(&args.output, format)?;
        }
    }

    if !quiet {
        crate::console::line(format_args!(
            "✓ Conversion complete: {}",
            args.output.display()
        ))?;
    }

    Ok(())
}

fn validate_range(start: Option<f64>, stop: Option<f64>) -> Result<(), CliError> {
    for (name, value) in [("--start", start), ("--stop", stop)] {
        if value.is_some_and(|value| !value.is_finite()) {
            return Err(CliError::InvalidArgument {
                message: format!("{name} must be finite"),
                suggestion: None,
            });
        }
    }
    if start.zip(stop).is_some_and(|(start, stop)| start > stop) {
        return Err(CliError::InvalidArgument {
            message: "--start must not exceed --stop".into(),
            suggestion: None,
        });
    }
    Ok(())
}
