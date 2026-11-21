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

use crate::cli::{CliError, Config, ConvertArgs, InputFormat, NumericBound, OutputFormat};
use crate::commands::waveform_io::{
    ImportedResult, detect_format, load_result_selected, load_runs_selected,
};
use crate::commands::{store_input, vcd_io};

/// Execute the convert command
pub fn execute(args: ConvertArgs, config: &Config, quiet: bool) -> Result<(), CliError> {
    validate_range(args.start.as_ref(), args.stop.as_ref())?;
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

    // A results store converts a chunk at a time where it can.
    if from_format == InputFormat::Rsr {
        let streamed = if args.to == OutputFormat::Vcd {
            store_input::convert_vcd_streamed(&args)?
        } else {
            store_input::convert_streamed(&args, config.resources.limits())?
        };
        if streamed {
            if !quiet {
                crate::console::line(format_args!(
                    "✓ Conversion complete: {}",
                    args.output.display()
                ))?;
            }
            return Ok(());
        }
    }

    if args.to == OutputFormat::Vcd {
        let loaded = vcd_io::load_vcd_document(
            &args.input,
            from_format,
            config.resources.limits(),
            args.expand_buses,
            args.section.as_deref(),
        )?;
        let (document, notes) =
            loaded.select_and_clip(&args.variables, args.start.as_ref(), args.stop.as_ref())?;
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

    // A rawfile holds a stepped plot as it is; a table cannot, and reads one run.
    let (limits, section) = (config.resources.limits(), args.section.as_deref());
    let rawfile = matches!(args.to, OutputFormat::Raw | OutputFormat::RawAscii);
    let imported = if rawfile {
        load_result_selected(&args.input, from_format, limits, section)?
    } else {
        load_runs_selected(&args.input, from_format, limits, section)?
    };

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
    table.clip_scale_range(
        args.start.as_ref().map(NumericBound::value),
        args.stop.as_ref().map(NumericBound::value),
    );
    // Input admission already refused empty tables.
    if table.scale.len() < crate::commands::waveform_io::MIN_RESULT_SAMPLES {
        return Err(CliError::ConversionError {
            message: "no data points remain after applying --start/--stop".to_string(),
        });
    }
    refuse_what_readers_refuse(&args.input, &table, rawfile)?;
    table.write(&args.output, args.to)?;

    if !quiet {
        crate::console::line(format_args!(
            "✓ Conversion complete: {}",
            args.output.display()
        ))?;
    }

    Ok(())
}

/// A table the application's importer would refuse, or read back as another result, is not
/// written: two columns of one name cannot be told apart, and a time that falls is not one run.
/// Names are compared as the importer compares them, ignoring case, and an unnamed scale is told
/// apart by its place. A frequency list or a DC sweep may run in any order its analysis asked
/// for, so only a time is held to rising. A rawfile holds the runs of a stepped plot as they
/// are, its time restarting for each, so only a table is held to one run. The reason names the
/// column or the point.
fn refuse_what_readers_refuse(
    path: &std::path::Path,
    table: &crate::commands::export_table::ExportTable,
    rawfile: bool,
) -> Result<(), CliError> {
    let mut names = std::collections::HashSet::new();
    let columns = table.columns.iter().map(|column| column.name.as_str());
    let scale = (!table.scale_name.is_empty()).then_some(table.scale_name.as_str());
    for name in scale.into_iter().chain(columns) {
        if !names.insert(name.to_lowercase()) {
            return Err(CliError::ConversionError {
                message: format!(
                    "{}: two columns are named '{name}', and a reader cannot tell them apart",
                    path.display()
                ),
            });
        }
    }
    let falls = (!rawfile && table.scale_type == "time")
        .then(|| table.scale.windows(2).position(|pair| pair[1] < pair[0]))
        .flatten();
    if let Some(point) = falls {
        return Err(CliError::ConversionError {
            message: format!(
                "{}: the '{}' scale falls from {} to {} at point {}, so it is not one run",
                path.display(),
                table.scale_name,
                table.scale[point],
                table.scale[point + 1],
                point + 2
            ),
        });
    }
    Ok(())
}

fn validate_range(
    start: Option<&NumericBound>,
    stop: Option<&NumericBound>,
) -> Result<(), CliError> {
    if start
        .zip(stop)
        .is_some_and(|(start, stop)| start.cmp(stop).is_gt())
    {
        return Err(CliError::InvalidArgument {
            message: "--start must not exceed --stop".into(),
            suggestion: None,
        });
    }
    Ok(())
}
