//! Time-domain and DC analysis cards: `.OP`, `.DC`, `.TRAN`, `.FOUR`, `.TEMP`.
//!
//! Each entry point runs one card against the shared [`RunContext`], prints
//! the console report, and exports the `op`/`dc`/`tran`/`four`/`temp` tagged
//! file when `-o` is set. `.TRAN` also owns the progress bar, the
//! `--checkpoint`/`--resume` segmented path, and `--tran-stop`.

use super::RunContext;
use super::shared::map_hdf5_output_error;
use crate::cli::{CliError, OutputFormat, map_atomic_output_error};
use crate::commands::publish;
use crate::commands::run_signals::{
    SignalKind, checked_dc_operating_point_signals, dc_export_signals,
    dc_operating_point_export_signals, transient_export_signals,
};
use crate::hdf5::{
    Hdf5FftCoordinate, Hdf5FftHarmonic, Hdf5FftMetrics, Hdf5FftResult, Hdf5FftSection,
    Hdf5SimulationData, Hdf5WaveformSection, write_hdf5, write_hdf5_to_writer,
};
use std::path::{Component, Path, PathBuf};

fn map_output_projection_error(
    ctx: &RunContext<'_>,
    error: rspice_core::SimulationError,
    analysis: &str,
) -> CliError {
    if matches!(error, rspice_core::SimulationError::Aborted) {
        super::cancellation_cli_error(ctx.args.timeout)
    } else {
        CliError::CoreSimulationError {
            source: error,
            analysis: Some(format!("{analysis} output projection")),
        }
    }
}

pub(super) fn run_dc_op(ctx: &RunContext<'_>) -> Result<(), CliError> {
    if !ctx.quiet {
        println!("Running DC operating point...");
    }

    match ctx
        .engine
        .run_dc_op_with_report_and_abort(ctx.netlist, &crate::abort::ProcessAbort)
    {
        Ok((result, op_report)) => {
            let operating_point_signals = checked_dc_operating_point_signals(&result)
                .map_err(|error| map_output_projection_error(ctx, error, "DC OP"))?;
            let exported_operating_point_signals = dc_operating_point_export_signals(
                ctx.netlist,
                &result,
                &crate::abort::ProcessAbort,
            )
            .map_err(|error| map_output_projection_error(ctx, error, "DC OP"))?;
            super::shared::ensure_finite_series(
                ctx.args.allow_nonfinite,
                "DC OP",
                (1..result.node_voltages.len())
                    .map(|node| {
                        let name = result
                            .node_names
                            .get(node)
                            .map(|n| n.as_str())
                            .unwrap_or("node");
                        (name, std::slice::from_ref(&result.node_voltages[node]))
                    })
                    .chain(
                        result
                            .branch_currents
                            .iter()
                            .enumerate()
                            .map(|(index, current)| {
                                let name = result
                                    .branch_names
                                    .get(index)
                                    .map(|n| n.as_str())
                                    .unwrap_or("branch");
                                (name, std::slice::from_ref(current))
                            }),
                    ),
            )?;
            super::shared::ensure_finite_series(
                ctx.args.allow_nonfinite,
                "DC OP output projection",
                exported_operating_point_signals
                    .iter()
                    .map(|signal| (signal.display_name.as_str(), signal.values.as_slice())),
            )?;

            if !ctx.quiet {
                let voltage_signals = operating_point_signals
                    .iter()
                    .filter(|signal| signal.kind == SignalKind::Voltage)
                    .collect::<Vec<_>>();
                let current_signals = operating_point_signals
                    .iter()
                    .filter(|signal| signal.kind == SignalKind::Current)
                    .collect::<Vec<_>>();
                println!("DC Operating Point:");
                for signal in voltage_signals.iter().take(10) {
                    println!("  {} = {:.6} V", signal.display_name, signal.values[0]);
                }
                if voltage_signals.len() > 10 {
                    println!("  ... ({} more node voltages)", voltage_signals.len() - 10);
                }
                for signal in current_signals.iter().take(5) {
                    println!("  {} = {:.6} A", signal.display_name, signal.values[0]);
                }
                if current_signals.len() > 5 {
                    println!("  ... ({} more branch currents)", current_signals.len() - 5);
                }

                print_device_op_report(&op_report, ctx.verbose);
            }

            if let Some(output) = ctx.resolve_output("op") {
                let analysis_id = output.analysis("op")?;
                super::document::publish_analysis_result(
                    ctx,
                    &output.path,
                    analysis_id,
                    super::document::scalar_schema(&exported_operating_point_signals)?,
                    || {
                        rspice_core::execution::AnalysisResultDocument::from_operating_point(
                            analysis_id,
                            &result,
                            Some(&op_report),
                        )
                    },
                    |path, format| {
                        write_dc_op_output(
                            path,
                            &exported_operating_point_signals,
                            format,
                            Some(&super::document::hdf5_identity(ctx, analysis_id)?),
                        )
                    },
                )?;
                if !ctx.quiet {
                    println!("Results exported to: {}", output.path.display());
                }
            }

            Ok(())
        }
        Err(error) => {
            if matches!(error, rspice_core::SimulationError::Aborted) {
                Err(super::cancellation_cli_error(ctx.args.timeout))
            } else {
                Err(CliError::CoreSimulationError {
                    source: error,
                    analysis: Some("DC OP".to_string()),
                })
            }
        }
    }
}

/// Engineering-notation formatter for operating-point quantities.
fn format_engineering(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if !value.is_finite() {
        return format!("{}", value);
    }
    let magnitude = value.abs();
    let (scaled, suffix) = if magnitude >= 1e9 {
        (value / 1e9, "G")
    } else if magnitude >= 1e6 {
        (value / 1e6, "Meg")
    } else if magnitude >= 1e3 {
        (value / 1e3, "k")
    } else if magnitude >= 1.0 {
        (value, "")
    } else if magnitude >= 1e-3 {
        (value * 1e3, "m")
    } else if magnitude >= 1e-6 {
        (value * 1e6, "u")
    } else if magnitude >= 1e-9 {
        (value * 1e9, "n")
    } else if magnitude >= 1e-12 {
        (value * 1e12, "p")
    } else {
        (value * 1e15, "f")
    };
    format!("{:.4}{}", scaled, suffix)
}

/// Print the per-device operating-point table (the Spectre-style OP info).
///
/// Compact by default; `verbose` lifts the row cap so large circuits can
/// dump every device.
fn print_device_op_report(report: &rspice_core::circuit::DeviceOpReport, verbose: bool) {
    if report.is_empty() {
        return;
    }

    const COMPACT_ROW_CAP: usize = 25;
    let cap = if verbose { usize::MAX } else { COMPACT_ROW_CAP };

    println!("Device Operating Points:");
    for entry in report.entries.iter().take(cap) {
        let region = entry
            .region
            .map(|region| format!(" [{}]", region))
            .unwrap_or_default();
        let params = entry
            .params
            .iter()
            .map(|(name, value)| format!("{}={}", name, format_engineering(*value)))
            .collect::<Vec<_>>()
            .join("  ");
        println!(
            "  {:<16} {:<7}{} {}",
            entry.name, entry.device_kind, region, params
        );
    }
    if report.entries.len() > cap {
        println!(
            "  ... ({} more devices; rerun with --verbose for the full table)",
            report.entries.len() - cap
        );
    }
}

pub(super) fn write_dc_op_output(
    path: &Path,
    signals: &[crate::commands::run_signals::ScalarSignal],
    format: OutputFormat,
    identity: Option<&crate::hdf5::Hdf5ResultIdentity>,
) -> Result<(), CliError> {
    if matches!(format, OutputFormat::Hdf5) {
        let mut data = Hdf5SimulationData::new();
        data.title = "DC Operating Point".to_string();
        data.identity = identity.cloned();

        let mut operating_point = Hdf5WaveformSection::new("point", vec![0.0]);
        for signal in signals {
            operating_point.add_typed_signal(
                signal.display_name.clone(),
                signal.raw_variable_type(),
                signal.values.clone(),
            );
        }
        data.operating_point = Some(operating_point);

        write_hdf5(path, &data).map_err(|err| map_hdf5_output_error(path, err))?;
        return Ok(());
    }

    publish::artifact(path, |file| {
        match format {
            OutputFormat::Json => {
                return Err(CliError::InternalError {
                    message:
                        "the operating point publishes JSON as a typed result document, not a flat table"
                            .to_string(),
                });
            }
            OutputFormat::Csv => {
                writeln!(file, "signal,value").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                for signal in signals {
                    writeln!(
                        file,
                        "{},{:.17e}",
                        super::export::delimited_cell(&signal.display_name, ','),
                        signal.values[0]
                    )
                    .map_err(|e| CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    })?;
                }
            }
            OutputFormat::Tsv => {
                writeln!(file, "signal\tvalue").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                for signal in signals {
                    writeln!(file, "{}\t{:.17e}", signal.display_name, signal.values[0]).map_err(
                        |e| CliError::OutputError {
                            path: path.to_path_buf(),
                            source: e,
                        },
                    )?;
                }
            }
            OutputFormat::Raw | OutputFormat::RawAscii => {
                writeln!(file, "Title: DC Operating Point").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                writeln!(file, "Plotname: DC OP").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                writeln!(file, "Flags: real").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                writeln!(file, "No. Variables: {}", signals.len()).map_err(|e| {
                    CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    }
                })?;
                writeln!(file, "No. Points: 1").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                writeln!(file, "Variables:").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                for (index, signal) in signals.iter().enumerate() {
                    writeln!(
                        file,
                        "\t{}\t{}\t{}",
                        index,
                        signal.display_name,
                        signal.raw_variable_type()
                    )
                    .map_err(|e| CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    })?;
                }
                writeln!(file, "Values:").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                writeln!(file, "0").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                for signal in signals {
                    writeln!(file, "\t{:.17e}", signal.values[0]).map_err(|e| {
                        CliError::OutputError {
                            path: path.to_path_buf(),
                            source: e,
                        }
                    })?;
                }
            }
            OutputFormat::Hdf5 => {
                return Err(CliError::InternalError {
                    message: "HDF5 handled before text writers".to_string(),
                });
            }
        }

        Ok(())
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

pub(super) fn run_dc_sweep(
    ctx: &RunContext<'_>,
    source: &str,
    start: f64,
    stop: f64,
    step: f64,
    sweep2: Option<&rspice_core::netlist::DcSecondSweep>,
) -> Result<(), CliError> {
    if !ctx.quiet {
        match sweep2 {
            Some(outer) => println!(
                "Running DC sweep on {} from {} to {} by {} for each {} from {} to {} by {}...",
                source, start, stop, step, outer.source, outer.start, outer.stop, outer.step
            ),
            None => println!(
                "Running DC sweep on {} from {} to {} by {}...",
                source, start, stop, step
            ),
        }
    }

    // The reporting form is used so the typed result document can carry each
    // point's device operating report. The flat exporters read the same points.
    match ctx
        .engine
        .run_dc_sweep2_spec_with_report_and_abort(
            ctx.netlist,
            source,
            &rspice_core::netlist::DcSweepSpec::linear(start, stop, step),
            sweep2,
            &crate::abort::ProcessAbort,
        )
        .map(|points| {
            let pairs = points
                .iter()
                .map(|point| (point.sweep_value, point.result.clone()))
                .collect::<Vec<_>>();
            (points, pairs)
        }) {
        Ok((points, results)) => {
            for (_, point) in &results {
                super::shared::ensure_finite_series(
                    ctx.args.allow_nonfinite,
                    "DC Sweep",
                    (1..point.node_voltages.len()).map(|node| {
                        let name = point
                            .node_names
                            .get(node)
                            .map(|n| n.as_str())
                            .unwrap_or("node");
                        (name, std::slice::from_ref(&point.node_voltages[node]))
                    }),
                )?;
            }

            if !ctx.quiet {
                println!("DC Sweep: {} points computed", results.len());
            }

            let measurements = rspice_core::analysis::evaluate_dc_measurements_with_abort(
                ctx.netlist,
                &results,
                &crate::abort::ProcessAbort,
            )
            .map_err(|source| CliError::CoreSimulationError {
                source,
                analysis: Some("DC measurement projection".to_string()),
            })?;
            ctx.record_measurements("DC", measurements);
            let continuous_measurements =
                rspice_core::analysis::evaluate_dc_continuous_measurements(ctx.netlist, &results);
            super::shared::record_continuous_measurements(ctx, "DC_CONT", continuous_measurements);

            // Resolve the authored output contract even when the caller did
            // not request a file. A valid `.SAVE @device[param]` is part of
            // deck execution semantics; an unavailable probe must not become
            // a silent successful run merely because `-o` was omitted.
            let signals = dc_export_signals(
                ctx.netlist,
                &results,
                ctx.engine.config().resource_limits,
                &crate::abort::ProcessAbort,
            )
            .map_err(|error| map_output_projection_error(ctx, error, "DC"))?;

            if let Some(output) = ctx.resolve_output("dc") {
                let analysis_id = output.analysis("dc")?;
                let sweep_vals: Vec<f64> = results.iter().map(|(v, _)| *v).collect();
                super::shared::ensure_finite_series(
                    ctx.args.allow_nonfinite,
                    "DC Sweep output projection",
                    signals
                        .iter()
                        .map(|signal| (signal.display_name.as_str(), signal.values.as_slice())),
                )?;
                super::document::publish_analysis_result(
                    ctx,
                    &output.path,
                    analysis_id,
                    super::document::scalar_schema(&signals)?,
                    || {
                        rspice_core::execution::AnalysisResultDocument::from_dc_sweep(
                            analysis_id,
                            source,
                            rspice_core::execution::SignalUnit::Volt,
                            &points,
                        )
                    },
                    |path, format| match format {
                        OutputFormat::Hdf5 => {
                            let mut data = Hdf5SimulationData::new();
                            data.title = "DC Sweep".to_string();
                            data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);

                            let mut dc_sweep = Hdf5WaveformSection::new(source, sweep_vals.clone());
                            for signal in &signals {
                                dc_sweep.add_typed_signal(
                                    signal.display_name.clone(),
                                    signal.raw_variable_type(),
                                    signal.values.clone(),
                                );
                            }
                            data.dc_sweep = Some(dc_sweep);
                            write_hdf5(path, &data).map_err(|err| map_hdf5_output_error(path, err))
                        }
                        format => super::export::scalar_table(
                            "dc_sweep",
                            "DC transfer characteristic",
                            source,
                            "voltage",
                            sweep_vals.clone(),
                            &signals,
                        )
                        .write(path, format),
                    },
                )?;

                if !ctx.quiet {
                    println!("Results exported to: {}", output.path.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "DC Sweep")),
    }
}

/// One line of compression evidence: which signal came closest to leaving its
/// declared tolerance, when, and by how much.
///
/// A compression ratio on its own says only how much data was dropped, never
/// whether dropping it was allowed, so the ratio is never reported alone. The
/// certificate the core produces travels with every compressed result and this
/// is where the CLI shows it.
fn describe_compression_error(report: &rspice_core::engine::TransientCompressionReport) -> String {
    let Some(worst) = report.worst_observed.as_ref() else {
        return format!(
            "Worst compression error: none â€” all {} accepted samples were retained exactly",
            report.input_points
        );
    };
    let relative = worst
        .relative_error
        .map_or_else(|| "n/a".to_string(), |relative| format!("{:.3e}", relative));
    format!(
        "Worst compression error: {} {} at t={:.6e}s â€” absolute {:.3e}, relative {}, \
         tolerance {:.3e} ({:.1}% used)",
        worst.signal.kind.as_str(),
        worst.signal.canonical_name,
        worst.time,
        worst.absolute_error,
        relative,
        worst.allowed_tolerance,
        worst.tolerance_utilization * 100.0,
    )
}

/// Announce one compressed result's retained-point ratio and the worst
/// reconstruction error it observed.
fn report_compression(
    ctx: &RunContext<'_>,
    compressed: &rspice_core::engine::TransientResultCompressed,
    headline: &str,
) {
    if ctx.quiet {
        return;
    }
    println!(
        "{headline}: {} of {} accepted points (compression ratio: {:.1}x)",
        compressed.time.len(),
        compressed.input_points,
        compressed.compression_ratio
    );
    println!(
        "  {}",
        describe_compression_error(&compressed.compression_report)
    );
}

/// Expand one compressed container into the waveform the artifact writers
/// publish. The typed post-process products are taken from the container
/// before this call; the expansion is only the decimated waveform.
fn expand_compressed(
    compressed: rspice_core::engine::TransientResultCompressed,
    what: &str,
) -> Result<rspice_core::engine::TransientResult, CliError> {
    compressed
        .try_into_transient()
        .map_err(|message| CliError::InternalError {
            message: format!("core returned a malformed compressed {what}: {message}"),
        })
}

/// One completed transient, with the typed post-processing products the core
/// evaluated on the exact accepted trajectory when the run was compressed.
///
/// A compressed run publishes a decimated waveform. Recomputing `.MEASURE` or
/// `.FOUR` from it would report different numbers than the same deck without
/// `--compress`, so the core evaluates all three before decimation and the CLI
/// consumes those. An uncompressed run has no decimation to correct for and
/// carries `None`.
pub(super) struct TransientOutcome {
    pub(super) result: rspice_core::engine::TransientResult,
    pub(super) post_results: Option<rspice_core::engine::TransientPostResults>,
}

pub(super) fn run_transient(
    ctx: &RunContext<'_>,
    tstop: f64,
    tstep: f64,
    tstart: f64,
    max_step: Option<f64>,
    uic: bool,
) -> Result<TransientOutcome, CliError> {
    // --tran-stop overrides the deck's stop time so checkpoint segments can
    // share byte-identical source (the checkpoint fingerprint covers it).
    let tstop = ctx.args.tran_stop.unwrap_or(tstop);
    let internal_max_step = rspice_core::execution::resolve_transient_maximum_step(
        tstep,
        tstop,
        Some(tstart),
        max_step,
    )
    .map_err(|error| CliError::InvalidArgument {
        message: error.to_string(),
        suggestion: Some(
            "use finite TSTEP/TMAX values with 0 <= TSTART < TSTOP in the .TRAN directive"
                .to_owned(),
        ),
    })?;

    let pb = if ctx.quiet {
        indicatif::ProgressBar::hidden()
    } else if ctx.show_progress {
        // The engine reports its completed fraction at the abort-poll
        // cadence, so this is a real percentage, not a spinner.
        let pb = indicatif::ProgressBar::new(crate::abort::ProgressAbort::SCALE);
        let style = indicatif::ProgressStyle::default_bar()
            .template("{bar:30.green} {percent:>3}% [{elapsed_precise}] {msg}")
            .map_err(|error| CliError::InternalError {
                message: format!("invalid built-in transient progress template: {error}"),
            })?;
        pb.set_style(style);
        pb.set_message(format!(
            "Transient: {} to {} s (step {})",
            tstart, tstop, tstep
        ));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    } else {
        let pb = indicatif::ProgressBar::new_spinner();
        let style = indicatif::ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .map_err(|error| CliError::InternalError {
                message: format!("invalid built-in transient spinner template: {error}"),
            })?;
        pb.set_style(style);
        pb.set_message(format!(
            "Running transient: {} to {} (step {})...",
            tstart, tstop, tstep
        ));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    };

    let authored_restart = ctx.netlist.options.restart.as_ref();
    let checkpoint_path = ctx.transient_checkpoint_path(ctx.checkpoint.as_deref());
    let resume_path = ctx.transient_checkpoint_path(ctx.resume.as_deref());
    if authored_restart.is_some() && (checkpoint_path.is_some() || resume_path.is_some()) {
        return Err(restart_cli_error(
            ".OPTIONS RESTART cannot be combined with --checkpoint or --resume; choose one restart control plane",
        ));
    }

    let checkpointing = checkpoint_path.is_some() || resume_path.is_some();
    let startup_mode = rspice_core::engine::TransientStartupMode::from_uic(uic);
    let resolved_static_solver_ceiling = [
        Some(internal_max_step),
        Some(ctx.engine.config().max_timestep),
        ctx.engine.config().transient_timeint_max_timestep,
    ]
    .into_iter()
    .flatten()
    .filter(|step| step.is_finite() && *step > 0.0)
    .fold(f64::INFINITY, f64::min);
    let compression_config = || rspice_core::engine::CompressionConfig {
        enabled: true,
        abs_tol: ctx.compress_tol,
        rel_tol: ctx.compress_tol,
        maximum_retained_interval: resolved_static_solver_ceiling,
    };
    // Every compressed entry point returns the typed `.FFT`/`.FOUR`/`.MEASURE`
    // products the core evaluated on the exact accepted trajectory, before
    // decimation. They are carried out of this block so the publication path
    // consumes them instead of recomputing from the decimated expansion, which
    // is what made a compressed run report different numbers.
    let mut post_results: Option<rspice_core::engine::TransientPostResults> = None;
    let mut compression_report: Option<rspice_core::engine::TransientCompressionReport> = None;
    let result = if let Some(restart) = authored_restart {
        let restart_run =
            run_authored_restart(ctx, restart, tstop, internal_max_step, startup_mode, &pb);
        pb.finish_and_clear();
        let accepted = restart_run?;
        if ctx.compress {
            // The restart schedule writes its checkpoints from the accepted
            // trajectory, so compressing the published waveform afterwards
            // leaves every authored restart file byte-identical. This is the
            // composition point the core documents for exactly this case.
            let compressed = ctx
                .engine
                .compress_transient_result_with_abort(
                    ctx.netlist,
                    &accepted,
                    &compression_config(),
                    &crate::abort::ProcessAbort,
                )
                .map_err(|error| map_restart_simulation_error(ctx, error))?;
            report_compression(ctx, &compressed, "Transient complete (compressed)");
            compression_report = Some(compressed.compression_report.clone());
            post_results = Some(compressed.post_results.clone());
            Ok(expand_compressed(compressed, "restart segment")?)
        } else {
            Ok(accepted)
        }
    } else if checkpointing {
        // Segmented integration: restore the saved state (when resuming),
        // run to this segment's stop time, and persist the new state (when
        // checkpointing). The core validates the netlist fingerprint, so a
        // checkpoint can never silently continue a different circuit.
        enum SegmentResult {
            Full(Box<rspice_core::engine::TransientResult>),
            Compressed(Box<rspice_core::engine::TransientResultCompressed>),
        }

        let progress_abort = crate::abort::ProgressAbort::new(&pb);
        let run = if let Some(ref resume_path) = resume_path {
            let checkpoint_limit = ctx.engine.config().resource_limits.max_external_data_bytes;
            let checkpoint = rspice_core::engine::TransientCheckpoint::load_with_limit_and_abort(
                resume_path,
                checkpoint_limit,
                checkpoint_limit,
                &progress_abort,
            )
            .map_err(|source| CliError::CoreSimulationError {
                source,
                analysis: Some(format!(
                    "Transient checkpoint load ({})",
                    resume_path.display()
                )),
            })?;
            if checkpoint.startup_mode() != Some(startup_mode) {
                return Err(CliError::SimulationError {
                    message: format!(
                        "checkpoint startup mode {:?} does not match the selected .TRAN startup mode {:?}",
                        checkpoint.startup_mode(),
                        startup_mode
                    ),
                    analysis: Some("Transient".to_string()),
                });
            }
            if ctx.compress {
                ctx.engine
                    .run_tran_resume_compressed_with_abort(
                        ctx.netlist,
                        &checkpoint,
                        tstop,
                        internal_max_step,
                        compression_config(),
                        &progress_abort,
                    )
                    .map(|(result, checkpoint)| {
                        (SegmentResult::Compressed(Box::new(result)), checkpoint)
                    })
            } else {
                ctx.engine
                    .run_tran_resume_with_abort(
                        ctx.netlist,
                        &checkpoint,
                        tstop,
                        internal_max_step,
                        &progress_abort,
                    )
                    .map(|(result, checkpoint)| (SegmentResult::Full(Box::new(result)), checkpoint))
            }
        } else if ctx.compress {
            ctx.engine
                .run_tran_checkpointed_compressed_with_startup_mode_and_abort(
                    ctx.netlist,
                    tstop,
                    internal_max_step,
                    startup_mode,
                    compression_config(),
                    &progress_abort,
                )
                .map(|(result, checkpoint)| {
                    (SegmentResult::Compressed(Box::new(result)), checkpoint)
                })
        } else {
            ctx.engine
                .run_tran_checkpointed_with_startup_mode_and_abort(
                    ctx.netlist,
                    tstop,
                    internal_max_step,
                    startup_mode,
                    &progress_abort,
                )
                .map(|(result, checkpoint)| (SegmentResult::Full(Box::new(result)), checkpoint))
        };
        pb.finish_and_clear();
        match run {
            Ok((segment, checkpoint)) => {
                let result = match segment {
                    SegmentResult::Full(result) => *result,
                    SegmentResult::Compressed(compressed) => {
                        if !ctx.quiet {
                            println!(
                                "  Compressed segment: {} of {} accepted points ({:.1}x)",
                                compressed.time.len(),
                                compressed.input_points,
                                compressed.compression_ratio
                            );
                            println!(
                                "  {}",
                                describe_compression_error(&compressed.compression_report)
                            );
                        }
                        compression_report = Some(compressed.compression_report.clone());
                        post_results = Some(compressed.post_results.clone());
                        expand_compressed(*compressed, "checkpoint segment")?
                    }
                };
                if let Some(ref checkpoint_path) = checkpoint_path {
                    checkpoint
                        .save_with_abort(checkpoint_path, &progress_abort)
                        .map_err(|source| CliError::CoreSimulationError {
                            source,
                            analysis: Some(format!(
                                "Transient checkpoint save ({})",
                                checkpoint_path.display()
                            )),
                        })?;
                    if !ctx.quiet {
                        println!(
                            "  Checkpoint saved (t={:.6e}s): {}",
                            checkpoint.time,
                            checkpoint_path.display()
                        );
                    }
                }
                Ok(result)
            }
            Err(e) => Err(e),
        }
    } else if ctx.compress {
        let result = ctx.engine.run_tran_compressed_with_startup_mode_and_abort(
            ctx.netlist,
            tstop,
            internal_max_step,
            startup_mode,
            compression_config(),
            &crate::abort::ProgressAbort::new(&pb),
        );
        pb.finish_and_clear();
        match result {
            Ok(compressed) => {
                report_compression(ctx, &compressed, "Transient complete (compressed)");
                compression_report = Some(compressed.compression_report.clone());
                post_results = Some(compressed.post_results.clone());
                Ok(expand_compressed(compressed, "transient result")?)
            }
            Err(e) => Err(e),
        }
    } else {
        let result = ctx.engine.run_tran_with_startup_mode_and_abort(
            ctx.netlist,
            tstop,
            internal_max_step,
            startup_mode,
            &crate::abort::ProgressAbort::new(&pb),
        );
        pb.finish_and_clear();
        result
    };

    match result {
        Ok(result) => {
            super::shared::ensure_finite_series(
                ctx.args.allow_nonfinite,
                "Transient",
                result
                    .voltages
                    .iter()
                    .enumerate()
                    .map(|(index, waveform)| {
                        let name = result
                            .node_names
                            .get(index)
                            .map(|n| n.as_str())
                            .unwrap_or("node");
                        (name, waveform.as_slice())
                    })
                    .chain(
                        result
                            .branch_currents
                            .iter()
                            .enumerate()
                            .map(|(index, waveform)| {
                                let name = result
                                    .branch_names
                                    .get(index)
                                    .map(|n| n.as_str())
                                    .unwrap_or("branch");
                                (name, waveform.as_slice())
                            }),
                    ),
            )?;

            if !ctx.quiet && !ctx.compress {
                println!(
                    "âœ“ Transient complete: {} time points computed",
                    result.time.len()
                );
            }

            // A compressed run already carries the `.MEASURE` results the core
            // evaluated on the exact accepted trajectory. Re-evaluating them
            // here would measure the decimated expansion and report different
            // numbers than the same deck run without `--compress`.
            let measurements = match post_results.as_ref() {
                Some(post) => post.measurements.clone(),
                None => rspice_core::analysis::evaluate_tran_measurements_with_abort(
                    ctx.netlist,
                    &result,
                    &crate::abort::ProcessAbort,
                )
                .map_err(|source| CliError::CoreSimulationError {
                    source,
                    analysis: Some("Transient measurement projection".to_string()),
                })?,
            };
            ctx.record_measurements("TRAN", measurements);
            let continuous_measurements =
                rspice_core::analysis::evaluate_tran_continuous_measurements(ctx.netlist, &result);
            super::shared::record_continuous_measurements(
                ctx,
                "TRAN_CONT",
                continuous_measurements,
            );

            // Perform checked SAVE/PRINT materialization independently of
            // file publication, matching OP and DC behavior.
            let mut signals = transient_export_signals(
                ctx.netlist,
                &result,
                ctx.engine.config().resource_limits,
                &crate::abort::ProcessAbort,
            )
            .map_err(|error| map_output_projection_error(ctx, error, "Transient"))?;

            validate_fft_result_count(&result.fft_results, &ctx.netlist.fft_analyses)?;

            if let Some(resolved) = ctx.resolve_output("tran") {
                let analysis_id = resolved.analysis("tran")?;
                let output_path = resolved.path;
                let output_start = result
                    .time
                    .first()
                    .copied()
                    .map_or(tstart, |first| tstart.max(first));
                let projection = result
                    .output_projection(
                        &ctx.netlist.options.output_time_points,
                        ctx.netlist.options.output_interval_schedule.as_ref(),
                        output_start,
                        tstop,
                        ctx.engine.config().resource_limits.max_analysis_points,
                    )
                    .map_err(|message| CliError::simulation_error_in(message, "Transient"))?;
                let output_time = projection.times().to_vec();
                for signal in &mut signals {
                    signal.values = projection
                        .project(&signal.values)
                        .map_err(|message| CliError::simulation_error_in(message, "Transient"))?;
                }
                super::shared::ensure_finite_series(
                    ctx.args.allow_nonfinite,
                    "Transient output projection",
                    signals
                        .iter()
                        .map(|signal| (signal.display_name.as_str(), signal.values.as_slice())),
                )?;
                let document = match ctx.format {
                    OutputFormat::Hdf5 => {
                        let mut data = Hdf5SimulationData::new();
                        data.title = "Transient Analysis".to_string();
                        data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);

                        let mut transient = Hdf5WaveformSection::new("time", output_time.clone());
                        for signal in &signals {
                            transient.add_typed_signal(
                                signal.display_name.clone(),
                                signal.raw_variable_type(),
                                signal.values.clone(),
                            );
                        }
                        data.transient = Some(transient);
                        TransientOutputDocument::Hdf5(Box::new(data))
                    }
                    OutputFormat::Json => {
                        // `.FFT` spectra attached to this transient are named
                        // as children of it, so a reader of the typed document
                        // can find the sibling artifact that carries them.
                        let children = ctx
                            .fft_analysis_instances()
                            .into_iter()
                            .zip(&result.fft_results)
                            .map(|(analysis, spectrum)| {
                                rspice_core::execution::result_document::FftChildReference {
                                    analysis,
                                    output_name: spectrum.output_name.clone(),
                                }
                            })
                            .collect();
                        let builder =
                            rspice_core::execution::AnalysisResultDocument::from_transient(
                                analysis_id,
                                &result,
                                compression_report.as_ref(),
                                children,
                            )
                            .map_err(|error| {
                                super::document::document_error(ctx, analysis_id, error)
                            })?;
                        TransientOutputDocument::Typed(Box::new(super::document::finish(
                            ctx,
                            analysis_id,
                            builder,
                        )?))
                    }
                    OutputFormat::Raw
                    | OutputFormat::RawAscii
                    | OutputFormat::Csv
                    | OutputFormat::Tsv => {
                        TransientOutputDocument::Table(super::export::scalar_table(
                            "transient",
                            "Transient Analysis",
                            "time",
                            "time",
                            output_time,
                            &signals,
                        ))
                    }
                };
                ctx.record_published(super::PublishedResult {
                    analysis_id: analysis_id.tag(),
                    schema: super::document::scalar_schema(&signals)?,
                    artifact: output_path.clone(),
                });

                if result.fft_results.is_empty() {
                    publish::artifact(&output_path, |writer| {
                        document.write_to(
                            writer,
                            &output_path,
                            ctx.format,
                            super::document::json_byte_limit(ctx),
                        )
                    })
                    .map_err(|error| map_atomic_output_error(&output_path, error))?;
                } else {
                    let parent_analysis_id = ctx.current_transient_analysis_id()?;
                    let fft_output_path =
                        ctx.fft_output_path_for(&parent_analysis_id)
                            .ok_or_else(|| CliError::InternalError {
                                message:
                                    "FFT publication was requested without a resolved output path"
                                        .to_string(),
                            })?;
                    write_transient_fft_output_pair(
                        &output_path,
                        &document,
                        &fft_output_path,
                        ctx.format,
                        &parent_analysis_id,
                        ctx.fft_analysis_ids(),
                        ctx.coordinate.as_ref(),
                        &result.fft_results,
                        ctx.netlist,
                        ctx.args.timeout,
                        super::document::json_byte_limit(ctx),
                    )?;
                    if !ctx.quiet {
                        println!("  FFT results exported to: {}", fft_output_path.display());
                    }
                }

                if !ctx.quiet {
                    println!("  Results exported to: {}", output_path.display());
                }
            }
            Ok(TransientOutcome {
                result,
                post_results,
            })
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Transient")),
    }
}

enum TransientOutputDocument {
    Hdf5(Box<Hdf5SimulationData>),
    Table(super::export::ExportTable),
    /// The shared typed result document, published for `-f json`.
    Typed(Box<rspice_core::execution::AnalysisResultDocument>),
}

impl TransientOutputDocument {
    fn write_to(
        &self,
        writer: &mut dyn std::io::Write,
        path: &Path,
        format: OutputFormat,
        byte_limit: u64,
    ) -> Result<(), CliError> {
        match self {
            Self::Hdf5(data) => write_hdf5_to_writer(writer, data)
                .map_err(|error| map_hdf5_output_error(path, error)),
            Self::Table(table) => table.write_to(writer, path, format),
            Self::Typed(document) => {
                let json = document
                    .to_json_with_abort(&crate::abort::ProcessAbort, byte_limit)
                    .map_err(|error| CliError::CoreSimulationError {
                        source: rspice_core::SimulationError::Circuit(format!(
                            "{} cannot publish a typed result document: {error}",
                            document.analysis().tag()
                        )),
                        analysis: Some(document.analysis().tag()),
                    })?;
                writer
                    .write_all(json.as_bytes())
                    .and_then(|()| writer.write_all(b"\n"))
                    .map_err(|error| CliError::output_error(path, error))
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn write_transient_fft_output_pair(
    transient_path: &Path,
    transient: &TransientOutputDocument,
    fft_path: &Path,
    format: OutputFormat,
    parent_analysis_id: &str,
    analysis_ids: &[String],
    coordinate: Option<&super::ArtifactCoordinate>,
    results: &[rspice_core::engine::TransientFftResult],
    netlist: &rspice_core::Netlist,
    timeout_seconds: Option<f64>,
    byte_limit: u64,
) -> Result<(), CliError> {
    validate_fft_publication(results, analysis_ids, netlist)?;
    let requests = &netlist.fft_analyses;
    // A transient result and the FFT derived from it are one logical result:
    // publishing the transient alone would advertise a run whose declared
    // spectrum is missing. They are staged together and published together.
    publish::artifact_pair(
        transient_path,
        |writer| transient.write_to(writer, transient_path, format, byte_limit),
        fft_path,
        |writer| {
            write_fft_to_writer(
                writer,
                fft_path,
                format,
                parent_analysis_id,
                analysis_ids,
                coordinate,
                results,
                requests,
                timeout_seconds,
            )?;
            // Both members are complete here and nothing has been published
            // yet, so a cancellation that arrived during serialization still
            // leaves both destinations untouched.
            if crate::abort::reason().is_some() {
                return Err(super::cancellation_cli_error(timeout_seconds));
            }
            Ok(())
        },
    )
    .map_err(|error| map_atomic_output_error(fft_path, error))
}

const FFT_ARTIFACT_SCHEMA_VERSION: u32 = 2;

#[derive(serde::Serialize)]
struct FftJsonCoordinate<'a> {
    coordinate_id: &'a str,
    ordinal: usize,
    tag: &'a str,
    assignment: &'a str,
}

#[derive(serde::Serialize)]
struct FftJsonFormatPolicy {
    selected: &'static str,
    representation: &'static str,
    supported: [&'static str; 6],
}

#[derive(serde::Serialize)]
struct FftJsonDocument<'a> {
    schema_version: u32,
    analysis: &'static str,
    parent_analysis_id: &'a str,
    coordinate: Option<FftJsonCoordinate<'a>>,
    result_count: usize,
    results: FftJsonResults<'a>,
    format_policy: FftJsonFormatPolicy,
}

struct FftJsonResults<'a> {
    parent_analysis_id: &'a str,
    /// Canonical `fft-NNN` identity of each authored request, in source order.
    analysis_ids: &'a [String],
    results: &'a [rspice_core::engine::TransientFftResult],
    requests: &'a [rspice_core::netlist::FftAnalysis],
}

impl serde::Serialize for FftJsonResults<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{Error as _, SerializeSeq as _};

        let mut sequence = serializer.serialize_seq(Some(self.results.len()))?;
        for (index, (result, request)) in self.results.iter().zip(self.requests).enumerate() {
            if index.is_multiple_of(32) && crate::abort::reason().is_some() {
                return Err(S::Error::custom("FFT JSON serialization was cancelled"));
            }
            let analysis_id = self
                .analysis_ids
                .get(index)
                .map(String::as_str)
                .ok_or_else(|| S::Error::custom(missing_fft_identity(index)))?;
            sequence.serialize_element(&FftJsonResult::new(
                result,
                index + 1,
                analysis_id,
                self.parent_analysis_id,
                request,
            ))?;
        }
        sequence.end()
    }
}

#[derive(serde::Serialize)]
struct FftJsonSource<'a> {
    kind: &'static str,
    text: &'a str,
    authored_output: String,
}

#[derive(serde::Serialize)]
struct FftJsonSignal<'a> {
    name: &'a str,
    physical_type: &'a str,
    unit: Option<&'static str>,
}

#[derive(serde::Serialize)]
struct FftJsonSampling {
    start_time_s: f64,
    stop_time_s: f64,
    sample_interval_s: f64,
    point_count: usize,
    accurate_sampling: bool,
}

#[derive(serde::Serialize)]
struct FftJsonTransform<'a> {
    format: &'static str,
    mode: &'static str,
    window: &'static str,
    window_name: &'a str,
    alpha: f64,
    coherent_gain: f64,
    frequency_resolution_hz: f64,
    fundamental_bin: usize,
    minimum_metric_bin: usize,
    maximum_metric_bin: usize,
    sfdr_search_minimum_bin: usize,
}

#[derive(serde::Serialize)]
struct FftJsonSpectrum<'a> {
    frequency_unit: &'static str,
    value_unit: Option<&'static str>,
    phase_unit: &'static str,
    complex_representation: &'static str,
    bins: FftJsonBins<'a>,
}

struct FftJsonBins<'a>(&'a [rspice_core::engine::TransientFftBin]);

impl serde::Serialize for FftJsonBins<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{Error as _, SerializeSeq as _};

        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for bin in self.0 {
            if bin.index.is_multiple_of(256) && crate::abort::reason().is_some() {
                return Err(S::Error::custom("FFT JSON serialization was cancelled"));
            }
            sequence.serialize_element(&FftJsonBin {
                index: bin.index,
                frequency_hz: bin.frequency,
                value: FftJsonComplex {
                    real: bin.real,
                    imaginary: bin.imaginary,
                },
                magnitude: bin.magnitude,
                phase_degrees: bin.phase_degrees,
            })?;
        }
        sequence.end()
    }
}

#[derive(serde::Serialize)]
struct FftJsonBin {
    index: usize,
    frequency_hz: f64,
    value: FftJsonComplex,
    magnitude: f64,
    phase_degrees: f64,
}

#[derive(serde::Serialize)]
struct FftJsonComplex {
    real: f64,
    imaginary: f64,
}

#[derive(serde::Serialize)]
struct FftJsonMetricUnits {
    fundamental_magnitude: Option<&'static str>,
    thd_ratio: &'static str,
    thd_db: &'static str,
    sndr_db: &'static str,
    enob_bits: &'static str,
    snr_db: &'static str,
    sfdr_db: &'static str,
    sfdr_spur_frequency: &'static str,
}

#[derive(serde::Serialize)]
struct FftJsonMetrics<'a> {
    units: FftJsonMetricUnits,
    fundamental_magnitude: f64,
    thd_ratio: f64,
    thd_db: f64,
    sndr_db: f64,
    enob_bits: f64,
    snr_db: f64,
    sfdr_db: f64,
    sfdr_spur_bin: Option<usize>,
    sfdr_spur_frequency_hz: Option<f64>,
    largest_harmonics: FftJsonHarmonics<'a>,
}

struct FftJsonHarmonics<'a>(&'a [rspice_core::engine::TransientFftHarmonic]);

impl serde::Serialize for FftJsonHarmonics<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{Error as _, SerializeSeq as _};

        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for harmonic in self.0 {
            if harmonic.rank.is_multiple_of(256) && crate::abort::reason().is_some() {
                return Err(S::Error::custom("FFT JSON serialization was cancelled"));
            }
            sequence.serialize_element(&FftJsonHarmonic {
                rank: harmonic.rank,
                bin: harmonic.bin,
                frequency_hz: harmonic.frequency,
                magnitude: harmonic.magnitude,
                magnitude_db: harmonic.magnitude_db,
                phase_degrees: harmonic.phase_degrees,
            })?;
        }
        sequence.end()
    }
}

#[derive(serde::Serialize)]
struct FftJsonHarmonic {
    rank: usize,
    bin: usize,
    frequency_hz: f64,
    magnitude: f64,
    magnitude_db: f64,
    phase_degrees: f64,
}

#[derive(serde::Serialize)]
struct FftJsonResult<'a> {
    analysis_id: String,
    parent_analysis_id: &'a str,
    ordinal: usize,
    source: FftJsonSource<'a>,
    signal: FftJsonSignal<'a>,
    sampling: FftJsonSampling,
    transform: FftJsonTransform<'a>,
    spectrum: FftJsonSpectrum<'a>,
    metrics: Option<FftJsonMetrics<'a>>,
}

impl<'a> FftJsonResult<'a> {
    fn new(
        result: &'a rspice_core::engine::TransientFftResult,
        ordinal: usize,
        analysis_id: &'a str,
        parent_analysis_id: &'a str,
        request: &'a rspice_core::netlist::FftAnalysis,
    ) -> Self {
        let (source_kind, source_text, authored_output) = fft_output_identity(&result.output);
        let unit = fft_value_unit(result.physical_type, result.format).unwrap_or(None);
        Self {
            analysis_id: analysis_id.to_string(),
            parent_analysis_id,
            ordinal,
            source: FftJsonSource {
                kind: source_kind,
                text: source_text,
                authored_output,
            },
            signal: FftJsonSignal {
                name: &result.output_name,
                physical_type: result.physical_type,
                unit,
            },
            sampling: FftJsonSampling {
                start_time_s: result.start_time,
                stop_time_s: result.stop_time,
                sample_interval_s: result.sample_interval,
                point_count: result.point_count,
                accurate_sampling: result.accurate_sampling,
            },
            transform: FftJsonTransform {
                format: fft_format_name(result.format),
                mode: fft_mode_name(result.mode),
                window: fft_window_name(result.window),
                window_name: &result.window_name,
                alpha: result.alpha,
                coherent_gain: result.coherent_gain,
                frequency_resolution_hz: result.frequency_resolution,
                fundamental_bin: result.fundamental_bin,
                minimum_metric_bin: result.minimum_metric_bin,
                maximum_metric_bin: result.maximum_metric_bin,
                sfdr_search_minimum_bin: fft_sfdr_search_minimum_bin(result, request),
            },
            spectrum: FftJsonSpectrum {
                frequency_unit: "Hz",
                value_unit: unit,
                phase_unit: "degree",
                complex_representation: "cartesian",
                bins: FftJsonBins(&result.bins),
            },
            metrics: result.metrics.as_ref().map(|metrics| FftJsonMetrics {
                units: FftJsonMetricUnits {
                    fundamental_magnitude: unit,
                    thd_ratio: "1",
                    thd_db: "dB",
                    sndr_db: "dB",
                    enob_bits: "bit",
                    snr_db: "dB",
                    sfdr_db: "dB",
                    sfdr_spur_frequency: "Hz",
                },
                fundamental_magnitude: metrics.fundamental_magnitude,
                thd_ratio: metrics.thd_ratio,
                thd_db: metrics.thd_db,
                sndr_db: metrics.sndr_db,
                enob_bits: metrics.enob_bits,
                snr_db: metrics.snr_db,
                sfdr_db: metrics.sfdr_db,
                sfdr_spur_bin: metrics.sfdr_spur_bin,
                sfdr_spur_frequency_hz: metrics.sfdr_spur_frequency,
                largest_harmonics: FftJsonHarmonics(&metrics.largest_harmonics),
            }),
        }
    }
}

fn fft_output_identity(output: &rspice_core::netlist::FftOutput) -> (&'static str, &str, String) {
    match output {
        rspice_core::netlist::FftOutput::Probe(probe) => ("probe", probe, probe.clone()),
        rspice_core::netlist::FftOutput::Expression(expression) => {
            ("expression", expression, format!("{{{expression}}}"))
        }
    }
}

const fn fft_format_name(format: rspice_core::netlist::FftFormat) -> &'static str {
    match format {
        rspice_core::netlist::FftFormat::Normalized => "normalized",
        rspice_core::netlist::FftFormat::Unnormalized => "unnormalized",
    }
}

const fn fft_mode_name(mode: rspice_core::netlist::XyceFftMode) -> &'static str {
    match mode {
        rspice_core::netlist::XyceFftMode::HspiceCompatible => "hspice_compatible",
        rspice_core::netlist::XyceFftMode::SpectreCompatible => "spectre_compatible",
    }
}

const fn fft_window_name(window: rspice_core::netlist::FftWindow) -> &'static str {
    match window {
        rspice_core::netlist::FftWindow::Rectangular => "rectangular",
        rspice_core::netlist::FftWindow::Bartlett => "bartlett",
        rspice_core::netlist::FftWindow::BartlettHann => "bartlett_hann",
        rspice_core::netlist::FftWindow::Hamming => "hamming",
        rspice_core::netlist::FftWindow::Hann => "hann",
        rspice_core::netlist::FftWindow::Blackman67Db => "blackman_67db",
        rspice_core::netlist::FftWindow::Blackman => "blackman",
        rspice_core::netlist::FftWindow::BlackmanHarris => "blackman_harris",
        rspice_core::netlist::FftWindow::Nuttall => "nuttall",
        rspice_core::netlist::FftWindow::HalfCycleSine => "half_cycle_sine",
        rspice_core::netlist::FftWindow::HalfCycleSine3 => "half_cycle_sine_3",
        rspice_core::netlist::FftWindow::HalfCycleSine6 => "half_cycle_sine_6",
        rspice_core::netlist::FftWindow::Cosine2 => "cosine_2",
        rspice_core::netlist::FftWindow::Cosine4 => "cosine_4",
    }
}

fn fft_value_unit(
    physical_type: &str,
    format: rspice_core::netlist::FftFormat,
) -> Result<Option<&'static str>, &'static str> {
    let physical_unit = match physical_type.as_bytes() {
        b"voltage" => Some("V"),
        b"current" => Some("A"),
        b"parameter" => None,
        _ => return Err("unsupported FFT physical type"),
    };
    Ok(
        if matches!(format, rspice_core::netlist::FftFormat::Normalized) {
            Some("1")
        } else {
            physical_unit
        },
    )
}

fn fft_sfdr_search_minimum_bin(
    result: &rspice_core::engine::TransientFftResult,
    request: &rspice_core::netlist::FftAnalysis,
) -> usize {
    if request.minimum_frequency.is_none() && result.maximum_metric_bin >= result.fundamental_bin {
        result.fundamental_bin
    } else {
        result.minimum_metric_bin
    }
}

fn validate_fft_result_count(
    results: &[rspice_core::engine::TransientFftResult],
    requests: &[rspice_core::netlist::FftAnalysis],
) -> Result<(), CliError> {
    if results.len() != requests.len() {
        return Err(CliError::InternalError {
            message: format!(
                "core returned {} transient FFT result(s) for {} authored directive(s)",
                results.len(),
                requests.len()
            ),
        });
    }
    Ok(())
}

fn fft_validation_error(analysis: &str, message: impl std::fmt::Display) -> CliError {
    CliError::InternalError {
        message: format!("cannot publish {analysis}: {message}"),
    }
}

fn fft_expected_frequency_bin(
    analysis: &str,
    name: &str,
    requested: Option<f64>,
    default: usize,
    frequency_resolution: f64,
    nyquist_bin: usize,
) -> Result<usize, CliError> {
    let Some(requested) = requested else {
        return Ok(default);
    };
    let rounded = (requested / frequency_resolution).round();
    if !rounded.is_finite() || rounded < 0.0 || rounded > usize::MAX as f64 {
        return Err(fft_validation_error(
            analysis,
            format!("authored {name} cannot be represented as a transform bin"),
        ));
    }
    let bin = rounded as usize;
    if (name == "FREQ" && bin == 0) || bin > nyquist_bin {
        return Err(fft_validation_error(
            analysis,
            format!("authored {name} is outside the retained one-sided spectrum"),
        ));
    }
    Ok(bin)
}

fn fft_validation_window_coefficient(
    window: rspice_core::netlist::FftWindow,
    index: usize,
    points: usize,
    denominator: f64,
) -> f64 {
    use std::f64::consts::PI;

    if window == rspice_core::netlist::FftWindow::Rectangular {
        return 1.0;
    }
    let x = index as f64 / denominator;
    let cosine = |multiple: f64| (multiple * 2.0 * PI * x).cos();
    match window {
        rspice_core::netlist::FftWindow::Rectangular => 1.0,
        rspice_core::netlist::FftWindow::Bartlett => {
            if (index as f64) < 0.5 * (points - 1) as f64 {
                2.0 * x
            } else {
                2.0 - 2.0 * x
            }
        }
        rspice_core::netlist::FftWindow::BartlettHann => {
            0.62 - 0.48 * (x - 0.5).abs() + 0.38 * (2.0 * PI * (x - 0.5)).cos()
        }
        rspice_core::netlist::FftWindow::Hamming => 0.54 - 0.46 * cosine(1.0),
        rspice_core::netlist::FftWindow::Hann | rspice_core::netlist::FftWindow::Cosine2 => {
            0.5 - 0.5 * cosine(1.0)
        }
        rspice_core::netlist::FftWindow::Blackman67Db => {
            0.42323 - 0.49755 * cosine(1.0) + 0.07922 * cosine(2.0)
        }
        rspice_core::netlist::FftWindow::Blackman => 0.42 - 0.5 * cosine(1.0) + 0.08 * cosine(2.0),
        rspice_core::netlist::FftWindow::BlackmanHarris => {
            0.35875 - 0.48829 * cosine(1.0) + 0.14128 * cosine(2.0) - 0.01168 * cosine(3.0)
        }
        rspice_core::netlist::FftWindow::Nuttall => {
            0.3635819 - 0.4891775 * cosine(1.0) + 0.1365995 * cosine(2.0) - 0.0106411 * cosine(3.0)
        }
        rspice_core::netlist::FftWindow::HalfCycleSine => (PI * x).sin(),
        rspice_core::netlist::FftWindow::HalfCycleSine3 => (PI * x).sin().powi(3),
        rspice_core::netlist::FftWindow::HalfCycleSine6 => (PI * x).sin().powi(6),
        rspice_core::netlist::FftWindow::Cosine4 => 0.375 - 0.5 * cosine(1.0) + 0.125 * cosine(2.0),
    }
}

/// Check every spectrum the transient produced against the `.FFT` card that
/// asked for it, naming each by the canonical identity the plan assigned.
fn validate_fft_publication(
    results: &[rspice_core::engine::TransientFftResult],
    analysis_ids: &[String],
    netlist: &rspice_core::Netlist,
) -> Result<(), CliError> {
    let requests = &netlist.fft_analyses;
    validate_fft_result_count(results, requests)?;
    if analysis_ids.len() != requests.len() {
        return Err(missing_fft_identity(analysis_ids.len()));
    }
    let expected_mode = netlist.options.fft_mode.unwrap_or_default();
    let expected_accurate_sampling = netlist.options.fft_accurate.unwrap_or(true)
        && netlist.options.output_interval_schedule.is_none();
    let expected_metrics = netlist.options.fft_output_metrics.unwrap_or(false);
    for (index, (result, request)) in results.iter().zip(requests).enumerate() {
        let analysis = analysis_ids
            .get(index)
            .ok_or_else(|| missing_fft_identity(index))?
            .as_str();
        if result.output != request.output {
            return Err(fft_validation_error(
                analysis,
                "result source does not match the authored request at this ordinal",
            ));
        }
        fft_value_unit(result.physical_type, result.format)
            .map_err(|message| fft_validation_error(analysis, message))?;
        if result.output_name.is_empty() {
            return Err(fft_validation_error(
                analysis,
                "resolved signal name is empty",
            ));
        }
        if result.mode != expected_mode
            || result.accurate_sampling != expected_accurate_sampling
            || result.metrics.is_some() != expected_metrics
        {
            return Err(fft_validation_error(
                analysis,
                "FFT mode, sampling policy, or FFTOUT presence does not match the deck options",
            ));
        }
        if result.point_count != request.points
            || result.point_count < 4
            || !result.point_count.is_power_of_two()
        {
            return Err(fft_validation_error(
                analysis,
                "result point count does not match the authored request",
            ));
        }
        for (name, value, allow_zero) in [
            ("FREQ", request.fundamental_frequency, false),
            ("FMIN", request.minimum_frequency, true),
            ("FMAX", request.maximum_frequency, false),
        ] {
            if let Some(value) = value
                && (!value.is_finite() || value < 0.0 || (!allow_zero && value == 0.0))
            {
                return Err(fft_validation_error(
                    analysis,
                    format!("authored {name} is not a valid frequency"),
                ));
            }
        }
        if request
            .minimum_frequency
            .zip(request.maximum_frequency)
            .is_some_and(|(minimum, maximum)| minimum > maximum)
        {
            return Err(fft_validation_error(analysis, "authored FMIN exceeds FMAX"));
        }
        let expected_start = request.start.unwrap_or(0.0);
        if !fft_values_close(result.start_time, expected_start)
            || request
                .stop
                .is_some_and(|stop| !fft_values_close(result.stop_time, stop))
            || !result.start_time.is_finite()
            || !result.stop_time.is_finite()
            || result.stop_time <= result.start_time
        {
            return Err(fft_validation_error(
                analysis,
                "sampling interval does not match the authored START/STOP request",
            ));
        }
        let duration = result.stop_time - result.start_time;
        let expected_sample_interval = duration / result.point_count as f64;
        let expected_frequency_resolution = 1.0 / duration;
        if !result.sample_interval.is_finite()
            || !result.frequency_resolution.is_finite()
            || result.sample_interval <= 0.0
            || result.frequency_resolution <= 0.0
            || !fft_values_close(result.sample_interval, expected_sample_interval)
            || !fft_values_close(result.frequency_resolution, expected_frequency_resolution)
        {
            return Err(fft_validation_error(
                analysis,
                "sampling calibration is inconsistent with START, STOP, or NP",
            ));
        }
        let expected_format = request.format.unwrap_or(match result.mode {
            rspice_core::netlist::XyceFftMode::HspiceCompatible => {
                rspice_core::netlist::FftFormat::Normalized
            }
            rspice_core::netlist::XyceFftMode::SpectreCompatible => {
                rspice_core::netlist::FftFormat::Unnormalized
            }
        });
        if result.format != expected_format
            || result.window != request.window
            || result.window_name != request.window_name
            || !fft_values_close(result.alpha, request.alpha)
        {
            return Err(fft_validation_error(
                analysis,
                "effective format or window metadata does not match the authored request",
            ));
        }
        let denominator = if matches!(
            result.mode,
            rspice_core::netlist::XyceFftMode::SpectreCompatible
        ) {
            result.point_count as f64
        } else {
            (result.point_count - 1) as f64
        };
        let expected_coherent_gain = (0..result.point_count)
            .map(|sample| {
                fft_validation_window_coefficient(
                    request.window,
                    sample,
                    result.point_count,
                    denominator,
                )
            })
            .sum::<f64>()
            / result.point_count as f64;
        if !result.coherent_gain.is_finite()
            || result.coherent_gain <= 0.0
            || !fft_values_close(result.coherent_gain, expected_coherent_gain)
        {
            return Err(fft_validation_error(
                analysis,
                "coherent-gain calibration does not match the effective window",
            ));
        }

        let bin_count = result.point_count / 2 + 1;
        if result.bins.len() != bin_count {
            return Err(fft_validation_error(
                analysis,
                "one-sided bin count does not match NP",
            ));
        }
        let nyquist_bin = result.point_count / 2;
        let expected_fundamental = fft_expected_frequency_bin(
            analysis,
            "FREQ",
            request.fundamental_frequency,
            1,
            result.frequency_resolution,
            nyquist_bin,
        )?;
        let expected_minimum = fft_expected_frequency_bin(
            analysis,
            "FMIN",
            request.minimum_frequency,
            1,
            result.frequency_resolution,
            nyquist_bin,
        )?;
        let expected_maximum = fft_expected_frequency_bin(
            analysis,
            "FMAX",
            request.maximum_frequency,
            nyquist_bin,
            result.frequency_resolution,
            nyquist_bin,
        )?;
        if result.fundamental_bin != expected_fundamental
            || result.minimum_metric_bin != expected_minimum
            || result.maximum_metric_bin != expected_maximum
            || expected_maximum < expected_minimum
            || (expected_fundamental == 1 && expected_maximum < 2)
            || (expected_fundamental > 1 && expected_maximum < 1)
        {
            return Err(fft_validation_error(
                analysis,
                "metric-bin bounds do not match authored FREQ/FMIN/FMAX",
            ));
        }
        for (bin_index, bin) in result.bins.iter().enumerate() {
            let expected_frequency = bin_index as f64 * result.frequency_resolution;
            let expected_magnitude = bin.real.hypot(bin.imaginary);
            let expected_phase = bin.imaginary.atan2(bin.real).to_degrees();
            if bin.index != bin_index
                || ![
                    bin.frequency,
                    bin.real,
                    bin.imaginary,
                    bin.magnitude,
                    bin.phase_degrees,
                ]
                .iter()
                .all(|value| value.is_finite())
                || bin.magnitude < 0.0
                || !fft_values_close(bin.frequency, expected_frequency)
                || !fft_values_close(bin.magnitude, expected_magnitude)
                || fft_phase_distance_degrees(bin.phase_degrees, expected_phase) > 1.0e-9
            {
                return Err(fft_validation_error(
                    analysis,
                    format!("bin {bin_index} is inconsistent with the transform schema"),
                ));
            }
        }
        if result.format == rspice_core::netlist::FftFormat::Normalized {
            let maximum_magnitude = result
                .bins
                .iter()
                .map(|bin| bin.magnitude)
                .fold(0.0, f64::max);
            if maximum_magnitude != 0.0 && !fft_values_close(maximum_magnitude, 1.0) {
                return Err(fft_validation_error(
                    analysis,
                    "normalized spectrum does not peak at one",
                ));
            }
        }

        let Some(metrics) = &result.metrics else {
            continue;
        };
        let mut magnitudes = Vec::new();
        magnitudes
            .try_reserve_exact(result.bins.len())
            .map_err(|error| {
                fft_validation_error(
                    analysis,
                    format!("cannot allocate metric validation workspace: {error}"),
                )
            })?;
        magnitudes.extend(result.bins.iter().map(|bin| bin.magnitude));
        let sfdr_search_minimum = fft_sfdr_search_minimum_bin(result, request);
        let expected_metrics = crate::hdf5::fft_metric_expectations(
            &magnitudes,
            result.fundamental_bin,
            result.maximum_metric_bin,
            sfdr_search_minimum,
        )
        .ok_or_else(|| fft_validation_error(analysis, "metrics cannot be derived from spectrum"))?;
        let spur_frequency_matches = match (
            metrics.sfdr_spur_frequency,
            expected_metrics
                .sfdr_spur_bin
                .map(|bin| result.bins[bin].frequency),
        ) {
            (Some(actual), Some(expected)) => fft_values_close(actual, expected),
            (None, None) => true,
            _ => false,
        };
        if !fft_values_close(
            metrics.fundamental_magnitude,
            expected_metrics.fundamental_magnitude,
        ) || !fft_values_close(metrics.thd_ratio, expected_metrics.thd_ratio)
            || !fft_values_close(metrics.thd_db, expected_metrics.thd_db)
            || !fft_values_close(metrics.sndr_db, expected_metrics.sndr_db)
            || !fft_values_close(metrics.enob_bits, expected_metrics.enob_bits)
            || !fft_values_close(metrics.snr_db, expected_metrics.snr_db)
            || !fft_values_close(metrics.sfdr_db, expected_metrics.sfdr_db)
            || metrics.sfdr_spur_bin != expected_metrics.sfdr_spur_bin
            || !spur_frequency_matches
            || metrics.largest_harmonics.len() != expected_metrics.ranked_bins.len()
        {
            return Err(fft_validation_error(
                analysis,
                "FFTOUT metrics do not match the spectrum",
            ));
        }
        for (harmonic_index, (harmonic, expected_bin)) in metrics
            .largest_harmonics
            .iter()
            .zip(expected_metrics.ranked_bins)
            .enumerate()
        {
            let bin = &result.bins[expected_bin];
            if harmonic.rank != harmonic_index + 1
                || harmonic.bin != expected_bin
                || !fft_values_close(harmonic.frequency, bin.frequency)
                || !fft_values_close(harmonic.magnitude, bin.magnitude)
                || !fft_values_close(
                    harmonic.magnitude_db,
                    20.0 * bin.magnitude.max(1.0e-10).log10(),
                )
                || fft_phase_distance_degrees(harmonic.phase_degrees, bin.phase_degrees) > 1.0e-9
            {
                return Err(fft_validation_error(
                    analysis,
                    format!(
                        "ranked harmonic {} does not match the spectrum",
                        harmonic_index + 1
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn fft_json_document<'a>(
    parent_analysis_id: &'a str,
    analysis_ids: &'a [String],
    coordinate: Option<&'a super::ArtifactCoordinate>,
    results: &'a [rspice_core::engine::TransientFftResult],
    requests: &'a [rspice_core::netlist::FftAnalysis],
) -> FftJsonDocument<'a> {
    FftJsonDocument {
        schema_version: FFT_ARTIFACT_SCHEMA_VERSION,
        analysis: "fft",
        parent_analysis_id,
        coordinate: coordinate.map(|coordinate| FftJsonCoordinate {
            coordinate_id: &coordinate.id,
            ordinal: coordinate.ordinal,
            tag: &coordinate.tag,
            assignment: &coordinate.assignment,
        }),
        result_count: results.len(),
        results: FftJsonResults {
            parent_analysis_id,
            analysis_ids,
            results,
            requests,
        },
        format_policy: FftJsonFormatPolicy {
            selected: "json",
            representation: "structured",
            supported: ["json", "csv", "tsv", "raw", "ascii", "hdf5"],
        },
    }
}

const FFT_DELIMITED_HEADER: [&str; 54] = [
    "schema_version",
    "analysis",
    "artifact_format",
    "analysis_id",
    "parent_analysis_id",
    "coordinate_id",
    "coordinate_ordinal",
    "coordinate_tag",
    "coordinate_assignment",
    "fft_ordinal",
    "source_kind",
    "source_text",
    "authored_output",
    "output_name",
    "physical_type",
    "value_unit",
    "start_time_s",
    "stop_time_s",
    "sample_interval_s",
    "point_count",
    "accurate_sampling",
    "format",
    "mode",
    "window",
    "window_name",
    "alpha",
    "coherent_gain",
    "frequency_resolution_hz",
    "fundamental_bin",
    "minimum_metric_bin",
    "maximum_metric_bin",
    "sfdr_search_minimum_bin",
    "fundamental_magnitude",
    "thd_ratio",
    "thd_db",
    "sndr_db",
    "enob_bits",
    "snr_db",
    "sfdr_db",
    "sfdr_spur_bin",
    "sfdr_spur_frequency_hz",
    "record_kind",
    "bin_index",
    "frequency_hz",
    "real",
    "imaginary",
    "magnitude",
    "phase_degrees",
    "harmonic_rank",
    "harmonic_bin",
    "harmonic_frequency_hz",
    "harmonic_magnitude",
    "harmonic_magnitude_db",
    "harmonic_phase_degrees",
];

fn delimited_float(value: f64) -> String {
    format!("{value:.17e}")
}

fn delimited_optional_float(value: Option<f64>) -> String {
    value.map(delimited_float).unwrap_or_default()
}

fn delimited_optional_usize(value: Option<usize>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

fn fft_delimited_common_fields(
    format: OutputFormat,
    ordinal: usize,
    analysis_id: &str,
    parent_analysis_id: &str,
    coordinate: Option<&super::ArtifactCoordinate>,
    result: &rspice_core::engine::TransientFftResult,
    request: &rspice_core::netlist::FftAnalysis,
) -> Vec<String> {
    let (source_kind, source_text, authored_output) = fft_output_identity(&result.output);
    let metrics = result.metrics.as_ref();
    vec![
        FFT_ARTIFACT_SCHEMA_VERSION.to_string(),
        "fft".to_string(),
        match format {
            OutputFormat::Csv => "csv",
            OutputFormat::Tsv => "tsv",
            _ => unreachable!("validated flattened FFT format"),
        }
        .to_string(),
        analysis_id.to_string(),
        parent_analysis_id.to_string(),
        coordinate.map(|value| value.id.clone()).unwrap_or_default(),
        coordinate
            .map(|value| value.ordinal.to_string())
            .unwrap_or_default(),
        coordinate
            .map(|value| value.tag.clone())
            .unwrap_or_default(),
        coordinate
            .map(|value| value.assignment.clone())
            .unwrap_or_default(),
        ordinal.to_string(),
        source_kind.to_string(),
        source_text.to_string(),
        authored_output,
        result.output_name.clone(),
        result.physical_type.to_string(),
        fft_value_unit(result.physical_type, result.format)
            .unwrap_or(None)
            .unwrap_or_default()
            .to_string(),
        delimited_float(result.start_time),
        delimited_float(result.stop_time),
        delimited_float(result.sample_interval),
        result.point_count.to_string(),
        result.accurate_sampling.to_string(),
        fft_format_name(result.format).to_string(),
        fft_mode_name(result.mode).to_string(),
        fft_window_name(result.window).to_string(),
        result.window_name.clone(),
        delimited_float(result.alpha),
        delimited_float(result.coherent_gain),
        delimited_float(result.frequency_resolution),
        result.fundamental_bin.to_string(),
        result.minimum_metric_bin.to_string(),
        result.maximum_metric_bin.to_string(),
        fft_sfdr_search_minimum_bin(result, request).to_string(),
        metrics
            .map(|value| delimited_float(value.fundamental_magnitude))
            .unwrap_or_default(),
        metrics
            .map(|value| delimited_float(value.thd_ratio))
            .unwrap_or_default(),
        metrics
            .map(|value| delimited_float(value.thd_db))
            .unwrap_or_default(),
        metrics
            .map(|value| delimited_float(value.sndr_db))
            .unwrap_or_default(),
        metrics
            .map(|value| delimited_float(value.enob_bits))
            .unwrap_or_default(),
        metrics
            .map(|value| delimited_float(value.snr_db))
            .unwrap_or_default(),
        metrics
            .map(|value| delimited_float(value.sfdr_db))
            .unwrap_or_default(),
        delimited_optional_usize(metrics.and_then(|value| value.sfdr_spur_bin)),
        delimited_optional_float(metrics.and_then(|value| value.sfdr_spur_frequency)),
    ]
}

fn write_delimited_fields<'a>(
    writer: &mut dyn std::io::Write,
    path: &Path,
    delimiter: char,
    fields: impl IntoIterator<Item = &'a str>,
) -> Result<(), CliError> {
    let io_err = |error| CliError::output_error(path, error);
    let mut first = true;
    for field in fields {
        if !first {
            write!(writer, "{delimiter}").map_err(io_err)?;
        }
        first = false;
        write!(
            writer,
            "{}",
            super::export::delimited_cell(field, delimiter)
        )
        .map_err(io_err)?;
    }
    writeln!(writer).map_err(io_err)
}

fn write_fft_delimited(
    writer: &mut dyn std::io::Write,
    path: &Path,
    format: OutputFormat,
    parent_analysis_id: &str,
    analysis_ids: &[String],
    coordinate: Option<&super::ArtifactCoordinate>,
    results: &[rspice_core::engine::TransientFftResult],
    requests: &[rspice_core::netlist::FftAnalysis],
    timeout_seconds: Option<f64>,
) -> Result<(), CliError> {
    let delimiter = if matches!(format, OutputFormat::Csv) {
        ','
    } else {
        '\t'
    };
    write_delimited_fields(writer, path, delimiter, FFT_DELIMITED_HEADER)?;

    for (index, (result, request)) in results.iter().zip(requests).enumerate() {
        if crate::abort::reason().is_some() {
            return Err(super::cancellation_cli_error(timeout_seconds));
        }
        let common = fft_delimited_common_fields(
            format,
            index + 1,
            analysis_ids
                .get(index)
                .map(String::as_str)
                .ok_or_else(|| missing_fft_identity(index))?,
            parent_analysis_id,
            coordinate,
            result,
            request,
        );
        for bin in &result.bins {
            if bin.index.is_multiple_of(256) && crate::abort::reason().is_some() {
                return Err(super::cancellation_cli_error(timeout_seconds));
            }
            let record = [
                "bin".to_string(),
                bin.index.to_string(),
                delimited_float(bin.frequency),
                delimited_float(bin.real),
                delimited_float(bin.imaginary),
                delimited_float(bin.magnitude),
                delimited_float(bin.phase_degrees),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ];
            write_delimited_fields(
                writer,
                path,
                delimiter,
                common
                    .iter()
                    .map(String::as_str)
                    .chain(record.iter().map(String::as_str)),
            )?;
        }
        if let Some(metrics) = &result.metrics {
            for harmonic in &metrics.largest_harmonics {
                if harmonic.rank.is_multiple_of(256) && crate::abort::reason().is_some() {
                    return Err(super::cancellation_cli_error(timeout_seconds));
                }
                let record = [
                    "largest_harmonic".to_string(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    harmonic.rank.to_string(),
                    harmonic.bin.to_string(),
                    delimited_float(harmonic.frequency),
                    delimited_float(harmonic.magnitude),
                    delimited_float(harmonic.magnitude_db),
                    delimited_float(harmonic.phase_degrees),
                ];
                write_delimited_fields(
                    writer,
                    path,
                    delimiter,
                    common
                        .iter()
                        .map(String::as_str)
                        .chain(record.iter().map(String::as_str)),
                )?;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawMetadata {
    schema_version: u32,
    analysis: String,
    parent_analysis_id: String,
    coordinate: Option<FftRawCoordinate>,
    result_count: usize,
    results: Vec<FftRawMetadataResult>,
    data_columns: [String; 7],
    frequency_unit: String,
    phase_unit: String,
    complex_representation: String,
    selected_format: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawCoordinate {
    coordinate_id: String,
    ordinal: usize,
    tag: String,
    assignment: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawSource {
    kind: String,
    text: String,
    authored_output: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawSignal {
    name: String,
    physical_type: String,
    unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawSampling {
    start_time_s: f64,
    stop_time_s: f64,
    sample_interval_s: f64,
    point_count: usize,
    accurate_sampling: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawTransform {
    format: String,
    mode: String,
    window: String,
    window_name: String,
    alpha: f64,
    coherent_gain: f64,
    frequency_resolution_hz: f64,
    fundamental_bin: usize,
    minimum_metric_bin: usize,
    maximum_metric_bin: usize,
    sfdr_search_minimum_bin: usize,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawMetricUnits {
    fundamental_magnitude: Option<String>,
    thd_ratio: String,
    thd_db: String,
    sndr_db: String,
    enob_bits: String,
    snr_db: String,
    sfdr_db: String,
    sfdr_spur_frequency: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawHarmonic {
    rank: usize,
    bin: usize,
    frequency_hz: f64,
    magnitude: f64,
    magnitude_db: f64,
    phase_degrees: f64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawMetrics {
    units: FftRawMetricUnits,
    fundamental_magnitude: f64,
    thd_ratio: f64,
    thd_db: f64,
    sndr_db: f64,
    enob_bits: f64,
    snr_db: f64,
    sfdr_db: f64,
    sfdr_spur_bin: Option<usize>,
    sfdr_spur_frequency_hz: Option<f64>,
    largest_harmonics: Vec<FftRawHarmonic>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FftRawMetadataResult {
    analysis_id: String,
    parent_analysis_id: String,
    ordinal: usize,
    source: FftRawSource,
    signal: FftRawSignal,
    sampling: FftRawSampling,
    transform: FftRawTransform,
    metrics: Option<FftRawMetrics>,
}

impl FftRawMetadataResult {
    fn new(
        result: &rspice_core::engine::TransientFftResult,
        ordinal: usize,
        analysis_id: &str,
        parent_analysis_id: &str,
        request: &rspice_core::netlist::FftAnalysis,
    ) -> Self {
        let (kind, text, authored_output) = fft_output_identity(&result.output);
        let unit = fft_value_unit(result.physical_type, result.format)
            .unwrap_or(None)
            .map(str::to_string);
        Self {
            analysis_id: analysis_id.to_string(),
            parent_analysis_id: parent_analysis_id.to_string(),
            ordinal,
            source: FftRawSource {
                kind: kind.to_string(),
                text: text.to_string(),
                authored_output,
            },
            signal: FftRawSignal {
                name: result.output_name.clone(),
                physical_type: result.physical_type.to_string(),
                unit: unit.clone(),
            },
            sampling: FftRawSampling {
                start_time_s: result.start_time,
                stop_time_s: result.stop_time,
                sample_interval_s: result.sample_interval,
                point_count: result.point_count,
                accurate_sampling: result.accurate_sampling,
            },
            transform: FftRawTransform {
                format: fft_format_name(result.format).to_string(),
                mode: fft_mode_name(result.mode).to_string(),
                window: fft_window_name(result.window).to_string(),
                window_name: result.window_name.clone(),
                alpha: result.alpha,
                coherent_gain: result.coherent_gain,
                frequency_resolution_hz: result.frequency_resolution,
                fundamental_bin: result.fundamental_bin,
                minimum_metric_bin: result.minimum_metric_bin,
                maximum_metric_bin: result.maximum_metric_bin,
                sfdr_search_minimum_bin: fft_sfdr_search_minimum_bin(result, request),
            },
            metrics: result.metrics.as_ref().map(|metrics| FftRawMetrics {
                units: FftRawMetricUnits {
                    fundamental_magnitude: unit,
                    thd_ratio: "1".to_string(),
                    thd_db: "dB".to_string(),
                    sndr_db: "dB".to_string(),
                    enob_bits: "bit".to_string(),
                    snr_db: "dB".to_string(),
                    sfdr_db: "dB".to_string(),
                    sfdr_spur_frequency: "Hz".to_string(),
                },
                fundamental_magnitude: metrics.fundamental_magnitude,
                thd_ratio: metrics.thd_ratio,
                thd_db: metrics.thd_db,
                sndr_db: metrics.sndr_db,
                enob_bits: metrics.enob_bits,
                snr_db: metrics.snr_db,
                sfdr_db: metrics.sfdr_db,
                sfdr_spur_bin: metrics.sfdr_spur_bin,
                sfdr_spur_frequency_hz: metrics.sfdr_spur_frequency,
                largest_harmonics: metrics
                    .largest_harmonics
                    .iter()
                    .map(|harmonic| FftRawHarmonic {
                        rank: harmonic.rank,
                        bin: harmonic.bin,
                        frequency_hz: harmonic.frequency,
                        magnitude: harmonic.magnitude,
                        magnitude_db: harmonic.magnitude_db,
                        phase_degrees: harmonic.phase_degrees,
                    })
                    .collect(),
            }),
        }
    }
}

fn fft_raw_metadata(
    format: OutputFormat,
    parent_analysis_id: &str,
    analysis_ids: &[String],
    coordinate: Option<&super::ArtifactCoordinate>,
    results: &[rspice_core::engine::TransientFftResult],
    requests: &[rspice_core::netlist::FftAnalysis],
) -> Result<FftRawMetadata, CliError> {
    Ok(FftRawMetadata {
        schema_version: FFT_ARTIFACT_SCHEMA_VERSION,
        analysis: "fft".to_string(),
        parent_analysis_id: parent_analysis_id.to_string(),
        coordinate: coordinate.map(|coordinate| FftRawCoordinate {
            coordinate_id: coordinate.id.clone(),
            ordinal: coordinate.ordinal,
            tag: coordinate.tag.clone(),
            assignment: coordinate.assignment.clone(),
        }),
        result_count: results.len(),
        results: results
            .iter()
            .zip(requests)
            .enumerate()
            .map(|(index, (result, request))| {
                Ok(FftRawMetadataResult::new(
                    result,
                    index + 1,
                    analysis_ids
                        .get(index)
                        .map(String::as_str)
                        .ok_or_else(|| missing_fft_identity(index))?,
                    parent_analysis_id,
                    request,
                ))
            })
            .collect::<Result<Vec<_>, CliError>>()?,
        data_columns: [
            "frequency_hz".to_string(),
            "real".to_string(),
            "imaginary".to_string(),
            "magnitude".to_string(),
            "phase_degrees".to_string(),
            "fft_ordinal".to_string(),
            "bin_index".to_string(),
        ],
        frequency_unit: "Hz".to_string(),
        phase_unit: "degree".to_string(),
        complex_representation: "cartesian".to_string(),
        selected_format: if matches!(format, OutputFormat::Raw) {
            "raw".to_string()
        } else {
            "ascii".to_string()
        },
    })
}

/// The publication has fewer canonical identities than authored `.FFT`
/// requests, so a spectrum would be published anonymously.
fn missing_fft_identity(index: usize) -> CliError {
    CliError::InternalError {
        message: format!(
            "FFT publication has no canonical analysis identity for authored request {}",
            index.saturating_add(1)
        ),
    }
}

fn validate_fft_raw_metadata(metadata: &FftRawMetadata) -> Result<(), String> {
    const EXPECTED_COLUMNS: [&str; 7] = [
        "frequency_hz",
        "real",
        "imaginary",
        "magnitude",
        "phase_degrees",
        "fft_ordinal",
        "bin_index",
    ];
    if metadata.schema_version != FFT_ARTIFACT_SCHEMA_VERSION
        || metadata.analysis != "fft"
        || metadata.parent_analysis_id.is_empty()
        || metadata.result_count != metadata.results.len()
        || metadata.results.is_empty()
        || metadata.frequency_unit != "Hz"
        || metadata.phase_unit != "degree"
        || metadata.complex_representation != "cartesian"
        || !matches!(metadata.selected_format.as_str(), "raw" | "ascii")
        || metadata
            .data_columns
            .iter()
            .map(String::as_str)
            .ne(EXPECTED_COLUMNS)
    {
        return Err("invalid FFT RAW document envelope".to_string());
    }
    if let Some(coordinate) = &metadata.coordinate
        && (coordinate.coordinate_id.is_empty()
            || coordinate.ordinal == 0
            || coordinate.tag.is_empty()
            || coordinate.assignment.is_empty())
    {
        return Err("incomplete FFT RAW coordinate identity".to_string());
    }
    // The canonical identity of each authored `.FFT` request comes from the
    // planner, so a decoded document is checked against the same minting the
    // writer used rather than a second spelling of it.
    let canonical_ids = crate::commands::run::canonical_analysis_identities(
        rspice_core::execution::AnalysisKind::Fft,
        metadata.results.len(),
    )
    .map_err(|error| format!("cannot mint canonical FFT identities: {error}"))?;
    for (index, result) in metadata.results.iter().enumerate() {
        let ordinal = index + 1;
        let analysis = canonical_ids
            .get(index)
            .ok_or_else(|| {
                format!("FFT RAW document has no canonical identity for result {ordinal}")
            })?
            .tag();
        if result.analysis_id != analysis
            || result.parent_analysis_id != metadata.parent_analysis_id
            || result.ordinal != ordinal
            || !crate::hdf5::fft_source_identity_is_valid(
                &result.source.kind,
                &result.source.text,
                &result.source.authored_output,
            )
            || result.signal.name.is_empty()
            || result.signal.physical_type.is_empty()
            || result.sampling.point_count == 0
            || ![
                result.sampling.start_time_s,
                result.sampling.stop_time_s,
                result.sampling.sample_interval_s,
                result.transform.alpha,
                result.transform.coherent_gain,
                result.transform.frequency_resolution_hz,
            ]
            .iter()
            .all(|value| value.is_finite())
            || result.sampling.stop_time_s <= result.sampling.start_time_s
            || result.sampling.sample_interval_s <= 0.0
            || result.transform.frequency_resolution_hz <= 0.0
            || !matches!(
                result.transform.format.as_str(),
                "normalized" | "unnormalized"
            )
            || !matches!(
                result.transform.mode.as_str(),
                "hspice_compatible" | "spectre_compatible"
            )
            || !matches!(
                result.transform.window.as_str(),
                "rectangular"
                    | "bartlett"
                    | "bartlett_hann"
                    | "hamming"
                    | "hann"
                    | "blackman_67db"
                    | "blackman"
                    | "blackman_harris"
                    | "nuttall"
                    | "half_cycle_sine"
                    | "half_cycle_sine_3"
                    | "half_cycle_sine_6"
                    | "cosine_2"
                    | "cosine_4"
            )
            || result.transform.window_name.is_empty()
        {
            return Err(format!("invalid FFT RAW metadata for {analysis}"));
        }
        let bin_count = result.sampling.point_count / 2 + 1;
        if result.transform.fundamental_bin >= bin_count
            || result.transform.fundamental_bin == 0
            || result.transform.minimum_metric_bin >= bin_count
            || result.transform.maximum_metric_bin >= bin_count
            || result.transform.minimum_metric_bin > result.transform.maximum_metric_bin
            || result.transform.sfdr_search_minimum_bin >= bin_count
            || result.transform.sfdr_search_minimum_bin > result.transform.maximum_metric_bin
            || !matches!(
                result.transform.sfdr_search_minimum_bin,
                value if value == result.transform.minimum_metric_bin
                    || value == result.transform.fundamental_bin
            )
            || (result.transform.fundamental_bin == 1 && result.transform.maximum_metric_bin < 2)
            || (result.transform.fundamental_bin > 1 && result.transform.maximum_metric_bin < 1)
        {
            return Err(format!("invalid FFT RAW metric-bin bounds for {analysis}"));
        }
        let physical_unit = match result.signal.physical_type.as_str() {
            "voltage" => Some("V"),
            "current" => Some("A"),
            "parameter" => None,
            _ => return Err(format!("invalid FFT RAW signal type for {analysis}")),
        };
        let expected_unit = if result.transform.format == "normalized" {
            Some("1")
        } else {
            physical_unit
        };
        if result.signal.unit.as_deref() != expected_unit {
            return Err(format!("invalid FFT RAW signal unit for {analysis}"));
        }
        if let Some(metrics) = &result.metrics {
            if ![
                metrics.fundamental_magnitude,
                metrics.thd_ratio,
                metrics.thd_db,
                metrics.sndr_db,
                metrics.enob_bits,
                metrics.snr_db,
                metrics.sfdr_db,
            ]
            .iter()
            .all(|value| value.is_finite())
                || metrics.sfdr_spur_bin.is_some() != metrics.sfdr_spur_frequency_hz.is_some()
                || metrics
                    .sfdr_spur_frequency_hz
                    .is_some_and(|value| !value.is_finite())
                || metrics.sfdr_spur_bin.is_some_and(|bin| bin >= bin_count)
                || metrics
                    .sfdr_spur_bin
                    .zip(metrics.sfdr_spur_frequency_hz)
                    .is_some_and(|(bin, frequency)| {
                        !fft_values_close(
                            frequency,
                            bin as f64 * result.transform.frequency_resolution_hz,
                        )
                    })
                || metrics.largest_harmonics.len() > 30
                || metrics.units.fundamental_magnitude.as_deref() != expected_unit
                || metrics.units.thd_ratio != "1"
                || metrics.units.thd_db != "dB"
                || metrics.units.sndr_db != "dB"
                || metrics.units.enob_bits != "bit"
                || metrics.units.snr_db != "dB"
                || metrics.units.sfdr_db != "dB"
                || metrics.units.sfdr_spur_frequency != "Hz"
            {
                return Err(format!("invalid FFT RAW metrics for {analysis}"));
            }
            for (harmonic_index, harmonic) in metrics.largest_harmonics.iter().enumerate() {
                if harmonic.rank != harmonic_index + 1
                    || harmonic.bin == 0
                    || harmonic.bin >= bin_count
                    || metrics.largest_harmonics[..harmonic_index]
                        .iter()
                        .any(|previous| previous.bin == harmonic.bin)
                    || ![
                        harmonic.frequency_hz,
                        harmonic.magnitude,
                        harmonic.magnitude_db,
                        harmonic.phase_degrees,
                    ]
                    .iter()
                    .all(|value| value.is_finite())
                    || !fft_values_close(
                        harmonic.frequency_hz,
                        harmonic.bin as f64 * result.transform.frequency_resolution_hz,
                    )
                {
                    return Err(format!(
                        "invalid FFT RAW ranked harmonic {} for {analysis}",
                        harmonic_index + 1
                    ));
                }
            }
        }
    }
    Ok(())
}

fn fft_values_close(actual: f64, expected: f64) -> bool {
    if actual == expected {
        return true;
    }
    let scale = actual.abs().max(expected.abs());
    let tolerance = 128.0 * f64::EPSILON * scale;
    (actual - expected).abs() <= tolerance
}

fn fft_phase_distance_degrees(actual: f64, expected: f64) -> f64 {
    let delta = (actual - expected).rem_euclid(360.0);
    delta.min(360.0 - delta)
}

fn validate_fft_raw_metrics_against_bins(
    result: &FftRawMetadataResult,
    bins: &[DecodedFftRawBin],
) -> Result<(), String> {
    let Some(metrics) = &result.metrics else {
        return Ok(());
    };
    let magnitudes = bins.iter().map(|bin| bin.magnitude).collect::<Vec<_>>();
    let expected = crate::hdf5::fft_metric_expectations(
        &magnitudes,
        result.transform.fundamental_bin,
        result.transform.maximum_metric_bin,
        result.transform.sfdr_search_minimum_bin,
    )
    .ok_or_else(|| {
        format!(
            "FFT RAW {} cannot produce valid metrics from its spectrum",
            result.analysis_id
        )
    })?;
    let spur_frequency_matches = match (
        metrics.sfdr_spur_frequency_hz,
        expected.sfdr_spur_bin.map(|bin| bins[bin].frequency_hz),
    ) {
        (Some(actual), Some(expected)) => fft_values_close(actual, expected),
        (None, None) => true,
        _ => false,
    };
    if !fft_values_close(
        metrics.fundamental_magnitude,
        expected.fundamental_magnitude,
    ) || !fft_values_close(metrics.thd_ratio, expected.thd_ratio)
        || !fft_values_close(metrics.thd_db, expected.thd_db)
        || !fft_values_close(metrics.sndr_db, expected.sndr_db)
        || !fft_values_close(metrics.enob_bits, expected.enob_bits)
        || !fft_values_close(metrics.snr_db, expected.snr_db)
        || !fft_values_close(metrics.sfdr_db, expected.sfdr_db)
        || metrics.sfdr_spur_bin != expected.sfdr_spur_bin
        || !spur_frequency_matches
        || metrics.largest_harmonics.len() != expected.ranked_bins.len()
    {
        return Err(format!(
            "FFT RAW {} metrics do not match its spectrum",
            result.analysis_id
        ));
    }

    for (index, (harmonic, expected_bin)) in metrics
        .largest_harmonics
        .iter()
        .zip(expected.ranked_bins)
        .enumerate()
    {
        let bin = &bins[expected_bin];
        if harmonic.rank != index + 1
            || harmonic.bin != expected_bin
            || !fft_values_close(harmonic.frequency_hz, bin.frequency_hz)
            || !fft_values_close(harmonic.magnitude, bin.magnitude)
            || !fft_values_close(
                harmonic.magnitude_db,
                20.0 * bin.magnitude.max(1.0e-10).log10(),
            )
            || fft_phase_distance_degrees(harmonic.phase_degrees, bin.phase_degrees) > 1.0e-9
        {
            return Err(format!(
                "FFT RAW {} ranked harmonic {} does not match its spectrum",
                result.analysis_id,
                index + 1
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DecodedFftRawBin {
    pub(crate) analysis_id: String,
    pub(crate) index: usize,
    pub(crate) frequency_hz: f64,
    pub(crate) real: f64,
    pub(crate) imaginary: f64,
    pub(crate) magnitude: f64,
    pub(crate) phase_degrees: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DecodedFftRawArtifact {
    pub(crate) metadata: FftRawMetadata,
    pub(crate) bins: Vec<DecodedFftRawBin>,
}

/// Read and validate the RSpice FFT RAW extension. The ordinary waveform
/// reader intentionally remains generic; this decoder joins its numeric
/// columns with the typed JSON provenance carried by the standard `Command:`
/// header and rejects any inconsistent identity or bin layout.
pub(crate) fn read_fft_raw_artifact(path: &Path) -> Result<DecodedFftRawArtifact, String> {
    const EXPECTED_VARIABLES: [&str; 7] = [
        "frequency",
        "fft_real",
        "fft_imaginary",
        "fft_magnitude",
        "fft_phase_degrees",
        "fft_ordinal",
        "bin_index",
    ];
    const EXPECTED_VARIABLE_TYPES: [&str; 7] = [
        "frequency",
        "value",
        "value",
        "value",
        "degree",
        "index",
        "index",
    ];
    let raw = rspice_core::io::parse_raw_file(path).map_err(|error| error.to_string())?;
    if raw.header.plotname != "Transient FFT"
        || raw.variables.len() != EXPECTED_VARIABLES.len()
        || raw
            .variables
            .iter()
            .map(|variable| variable.name.as_str())
            .ne(EXPECTED_VARIABLES)
        || raw
            .variables
            .iter()
            .map(|variable| variable.var_type.as_str())
            .ne(EXPECTED_VARIABLE_TYPES)
        || raw.waveforms.len() != EXPECTED_VARIABLES.len()
        || raw
            .header
            .flags
            .iter()
            .map(String::as_str)
            .ne(["real", "double"])
        || raw.header.is_complex
        || !raw.header.is_double
    {
        return Err("invalid FFT RAW variable schema".to_string());
    }
    let metadata: FftRawMetadata =
        serde_json::from_str(&raw.header.command).map_err(|error| error.to_string())?;
    validate_fft_raw_metadata(&metadata)?;
    let expected_format = if raw.header.is_binary { "raw" } else { "ascii" };
    if metadata.selected_format != expected_format {
        return Err(format!(
            "FFT RAW metadata selects '{}', but the artifact is {expected_format}",
            metadata.selected_format
        ));
    }
    let expected_points = metadata.results.iter().try_fold(0usize, |count, result| {
        count.checked_add(result.sampling.point_count / 2 + 1)
    });
    let expected_points =
        expected_points.ok_or_else(|| "FFT RAW point count overflow".to_string())?;
    if raw.header.no_points != expected_points
        || raw
            .waveforms
            .iter()
            .any(|waveform| waveform.y.len() != expected_points)
    {
        return Err("FFT RAW point count does not match typed metadata".to_string());
    }

    let columns = &raw.waveforms;
    let mut bins = Vec::with_capacity(expected_points);
    let mut row = 0usize;
    for result in &metadata.results {
        let result_start = row;
        let bin_count = result.sampling.point_count / 2 + 1;
        for bin_index in 0..bin_count {
            let ordinal = columns[5].y[row];
            let stored_index = columns[6].y[row];
            if ordinal != result.ordinal as f64 || stored_index != bin_index as f64 {
                return Err(format!(
                    "FFT RAW row {row} does not match {} bin {bin_index}",
                    result.analysis_id
                ));
            }
            let real = columns[1].y[row];
            let imaginary = columns[2].y[row];
            let magnitude = columns[3].y[row];
            let frequency = columns[0].y[row];
            let phase_degrees = columns[4].y[row];
            if ![frequency, real, imaginary, magnitude, phase_degrees]
                .iter()
                .all(|value| value.is_finite())
                || magnitude < 0.0
            {
                return Err(format!(
                    "FFT RAW row {row} contains non-finite or invalid data"
                ));
            }
            let expected_frequency = bin_index as f64 * result.transform.frequency_resolution_hz;
            if !fft_values_close(frequency, expected_frequency) {
                return Err(format!(
                    "FFT RAW row {row} frequency does not match {} bin {bin_index}",
                    result.analysis_id
                ));
            }
            let derived_magnitude = real.hypot(imaginary);
            if !fft_values_close(magnitude, derived_magnitude) {
                return Err(format!(
                    "FFT RAW row {row} magnitude is inconsistent with its Cartesian value"
                ));
            }
            if derived_magnitude > 1.0e-14 {
                let derived_phase = imaginary.atan2(real).to_degrees();
                if fft_phase_distance_degrees(phase_degrees, derived_phase) > 1.0e-9 {
                    return Err(format!(
                        "FFT RAW row {row} phase is inconsistent with its Cartesian value"
                    ));
                }
            }
            bins.push(DecodedFftRawBin {
                analysis_id: result.analysis_id.clone(),
                index: bin_index,
                frequency_hz: frequency,
                real,
                imaginary,
                magnitude,
                phase_degrees,
            });
            row += 1;
        }
        if result.transform.format == "normalized" {
            let maximum_magnitude = bins[result_start..row]
                .iter()
                .map(|bin| bin.magnitude)
                .fold(0.0, f64::max);
            if maximum_magnitude != 0.0 && !fft_values_close(maximum_magnitude, 1.0) {
                return Err(format!(
                    "FFT RAW {} normalized spectrum does not peak at 1",
                    result.analysis_id
                ));
            }
        }
        validate_fft_raw_metrics_against_bins(result, &bins[result_start..row])?;
    }
    Ok(DecodedFftRawArtifact { metadata, bins })
}

fn write_fft_raw(
    writer: &mut dyn std::io::Write,
    path: &Path,
    format: OutputFormat,
    parent_analysis_id: &str,
    analysis_ids: &[String],
    coordinate: Option<&super::ArtifactCoordinate>,
    results: &[rspice_core::engine::TransientFftResult],
    requests: &[rspice_core::netlist::FftAnalysis],
    timeout_seconds: Option<f64>,
) -> Result<(), CliError> {
    let metadata = fft_raw_metadata(
        format,
        parent_analysis_id,
        analysis_ids,
        coordinate,
        results,
        requests,
    )?;
    validate_fft_raw_metadata(&metadata).map_err(|message| CliError::InternalError { message })?;
    let command = serde_json::to_string(&metadata).map_err(|error| {
        if crate::abort::reason().is_some() {
            super::cancellation_cli_error(timeout_seconds)
        } else {
            CliError::output_json_error(path, error)
        }
    })?;
    let point_count = results
        .iter()
        .try_fold(0usize, |count, result| count.checked_add(result.bins.len()))
        .ok_or_else(|| CliError::InternalError {
            message: "FFT RAW point count overflowed this platform".to_string(),
        })?;
    let io_err = |error| CliError::output_error(path, error);
    writeln!(writer, "Title: RSpice FFT bundle {parent_analysis_id}").map_err(io_err)?;
    writeln!(writer, "Date: Generated by RSpice").map_err(io_err)?;
    writeln!(writer, "Plotname: Transient FFT").map_err(io_err)?;
    writeln!(writer, "Command: {command}").map_err(io_err)?;
    writeln!(writer, "Flags: real double").map_err(io_err)?;
    writeln!(writer, "No. Variables: 7").map_err(io_err)?;
    writeln!(writer, "No. Points: {point_count}").map_err(io_err)?;
    writeln!(writer, "Variables:").map_err(io_err)?;
    for (index, (name, variable_type)) in [
        ("frequency", "frequency"),
        ("fft_real", "value"),
        ("fft_imaginary", "value"),
        ("fft_magnitude", "value"),
        ("fft_phase_degrees", "degree"),
        ("fft_ordinal", "index"),
        ("bin_index", "index"),
    ]
    .iter()
    .enumerate()
    {
        writeln!(writer, "\t{index}\t{name}\t{variable_type}").map_err(io_err)?;
    }

    match format {
        OutputFormat::Raw => writeln!(writer, "Binary:").map_err(io_err)?,
        OutputFormat::RawAscii => writeln!(writer, "Values:").map_err(io_err)?,
        _ => unreachable!("validated FFT RAW format"),
    }
    let mut row_index = 0usize;
    for (result_index, result) in results.iter().enumerate() {
        for bin in &result.bins {
            if row_index.is_multiple_of(256) && crate::abort::reason().is_some() {
                return Err(super::cancellation_cli_error(timeout_seconds));
            }
            let row = [
                bin.frequency,
                bin.real,
                bin.imaginary,
                bin.magnitude,
                bin.phase_degrees,
                (result_index + 1) as f64,
                bin.index as f64,
            ];
            match format {
                OutputFormat::Raw => {
                    for value in row {
                        writer.write_all(&value.to_le_bytes()).map_err(io_err)?;
                    }
                }
                OutputFormat::RawAscii => {
                    write!(writer, "{row_index}").map_err(io_err)?;
                    for value in row {
                        write!(writer, "\t{value:.17e}").map_err(io_err)?;
                    }
                    writeln!(writer).map_err(io_err)?;
                }
                _ => unreachable!("validated FFT RAW format"),
            }
            row_index = row_index.saturating_add(1);
        }
    }
    Ok(())
}

fn hdf5_fft_section(
    parent_analysis_id: &str,
    analysis_ids: &[String],
    coordinate: Option<&super::ArtifactCoordinate>,
    results: &[rspice_core::engine::TransientFftResult],
    requests: &[rspice_core::netlist::FftAnalysis],
    timeout_seconds: Option<f64>,
) -> Result<Hdf5FftSection, CliError> {
    let mut hdf5_results = Vec::new();
    hdf5_results
        .try_reserve_exact(results.len())
        .map_err(|error| CliError::InternalError {
            message: format!("cannot allocate typed HDF5 FFT result metadata: {error}"),
        })?;
    for (index, (result, request)) in results.iter().zip(requests).enumerate() {
        if crate::abort::reason().is_some() {
            return Err(super::cancellation_cli_error(timeout_seconds));
        }
        let (source_kind, source_text, authored_output) = fft_output_identity(&result.output);
        let mut bin_indices = Vec::new();
        let mut frequency_hz = Vec::new();
        let mut real = Vec::new();
        let mut imaginary = Vec::new();
        let mut magnitude = Vec::new();
        let mut phase_degrees = Vec::new();
        bin_indices
            .try_reserve_exact(result.bins.len())
            .map_err(|error| CliError::InternalError {
                message: format!("cannot allocate typed HDF5 FFT bin indices: {error}"),
            })?;
        for values in [
            &mut frequency_hz,
            &mut real,
            &mut imaginary,
            &mut magnitude,
            &mut phase_degrees,
        ] {
            values
                .try_reserve_exact(result.bins.len())
                .map_err(|error| CliError::InternalError {
                    message: format!("cannot allocate typed HDF5 FFT bins: {error}"),
                })?;
        }
        for bin in &result.bins {
            if bin.index.is_multiple_of(256) && crate::abort::reason().is_some() {
                return Err(super::cancellation_cli_error(timeout_seconds));
            }
            bin_indices.push(
                u64::try_from(bin.index).map_err(|_| CliError::InternalError {
                    message: "FFT bin index exceeds the HDF5 u64 schema".to_string(),
                })?,
            );
            frequency_hz.push(bin.frequency);
            real.push(bin.real);
            imaginary.push(bin.imaginary);
            magnitude.push(bin.magnitude);
            phase_degrees.push(bin.phase_degrees);
        }
        let metrics = result.metrics.as_ref().map(|metrics| Hdf5FftMetrics {
            fundamental_magnitude: metrics.fundamental_magnitude,
            thd_ratio: metrics.thd_ratio,
            thd_db: metrics.thd_db,
            sndr_db: metrics.sndr_db,
            enob_bits: metrics.enob_bits,
            snr_db: metrics.snr_db,
            sfdr_db: metrics.sfdr_db,
            sfdr_spur_bin: metrics.sfdr_spur_bin,
            sfdr_spur_frequency_hz: metrics.sfdr_spur_frequency,
            largest_harmonics: metrics
                .largest_harmonics
                .iter()
                .map(|harmonic| Hdf5FftHarmonic {
                    rank: harmonic.rank,
                    bin: harmonic.bin,
                    frequency_hz: harmonic.frequency,
                    magnitude: harmonic.magnitude,
                    magnitude_db: harmonic.magnitude_db,
                    phase_degrees: harmonic.phase_degrees,
                })
                .collect(),
        });
        hdf5_results.push(Hdf5FftResult {
            analysis_id: analysis_ids
                .get(index)
                .cloned()
                .ok_or_else(|| missing_fft_identity(index))?,
            ordinal: index + 1,
            source_kind: source_kind.to_string(),
            source_text: source_text.to_string(),
            authored_output,
            output_name: result.output_name.clone(),
            physical_type: result.physical_type.to_string(),
            value_unit: fft_value_unit(result.physical_type, result.format)
                .unwrap_or(None)
                .map(str::to_string),
            start_time_s: result.start_time,
            stop_time_s: result.stop_time,
            sample_interval_s: result.sample_interval,
            point_count: result.point_count,
            accurate_sampling: result.accurate_sampling,
            format: fft_format_name(result.format).to_string(),
            mode: fft_mode_name(result.mode).to_string(),
            window: fft_window_name(result.window).to_string(),
            window_name: result.window_name.clone(),
            alpha: result.alpha,
            coherent_gain: result.coherent_gain,
            frequency_resolution_hz: result.frequency_resolution,
            fundamental_bin: result.fundamental_bin,
            minimum_metric_bin: result.minimum_metric_bin,
            maximum_metric_bin: result.maximum_metric_bin,
            sfdr_search_minimum_bin: fft_sfdr_search_minimum_bin(result, request),
            bin_indices,
            frequency_hz,
            real,
            imaginary,
            magnitude,
            phase_degrees,
            metrics,
        });
    }

    Ok(Hdf5FftSection {
        parent_analysis_id: parent_analysis_id.to_string(),
        coordinate: coordinate.map(|coordinate| Hdf5FftCoordinate {
            coordinate_id: coordinate.id.clone(),
            ordinal: coordinate.ordinal,
            tag: coordinate.tag.clone(),
            assignment: coordinate.assignment.clone(),
        }),
        results: hdf5_results,
    })
}

#[cfg(test)]
fn write_fft_output(
    path: &Path,
    format: OutputFormat,
    parent_analysis_id: &str,
    analysis_ids: &[String],
    coordinate: Option<&super::ArtifactCoordinate>,
    results: &[rspice_core::engine::TransientFftResult],
    netlist: &rspice_core::Netlist,
    timeout_seconds: Option<f64>,
) -> Result<(), CliError> {
    if crate::abort::reason().is_some() {
        return Err(super::cancellation_cli_error(timeout_seconds));
    }
    validate_fft_publication(results, analysis_ids, netlist)?;
    let requests = &netlist.fft_analyses;
    publish::artifact(path, |writer| {
        write_fft_to_writer(
            writer,
            path,
            format,
            parent_analysis_id,
            analysis_ids,
            coordinate,
            results,
            requests,
            timeout_seconds,
        )
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

#[allow(clippy::too_many_arguments)]
fn write_fft_to_writer(
    writer: &mut dyn std::io::Write,
    path: &Path,
    format: OutputFormat,
    parent_analysis_id: &str,
    analysis_ids: &[String],
    coordinate: Option<&super::ArtifactCoordinate>,
    results: &[rspice_core::engine::TransientFftResult],
    requests: &[rspice_core::netlist::FftAnalysis],
    timeout_seconds: Option<f64>,
) -> Result<(), CliError> {
    if crate::abort::reason().is_some() {
        return Err(super::cancellation_cli_error(timeout_seconds));
    }
    match format {
        OutputFormat::Json => {
            let document = fft_json_document(
                parent_analysis_id,
                analysis_ids,
                coordinate,
                results,
                requests,
            );
            serde_json::to_writer_pretty(&mut *writer, &document).map_err(|error| {
                if crate::abort::reason().is_some() {
                    super::cancellation_cli_error(timeout_seconds)
                } else {
                    CliError::output_json_error(path, error)
                }
            })?;
            writer
                .write_all(b"\n")
                .map_err(|error| CliError::output_error(path, error))?;
        }
        OutputFormat::Csv | OutputFormat::Tsv => write_fft_delimited(
            writer,
            path,
            format,
            parent_analysis_id,
            analysis_ids,
            coordinate,
            results,
            requests,
            timeout_seconds,
        )?,
        OutputFormat::Raw | OutputFormat::RawAscii => write_fft_raw(
            writer,
            path,
            format,
            parent_analysis_id,
            analysis_ids,
            coordinate,
            results,
            requests,
            timeout_seconds,
        )?,
        OutputFormat::Hdf5 => {
            let mut data = Hdf5SimulationData::new();
            data.title = format!("Transient FFT ({parent_analysis_id})");
            // The bundle carries several `.FFT` instances, so the document's
            // own identity is the parent transient they were derived from;
            // each spectrum keeps its own `analysis_id` inside the section.
            data.identity = Some(crate::hdf5::Hdf5ResultIdentity {
                analysis_id: parent_analysis_id.to_string(),
                coordinate_id: coordinate.map(|value| value.id.clone()),
                coordinate_tag: coordinate.map(|value| value.tag.clone()),
                coordinate_assignment: coordinate.map(|value| value.assignment.clone()),
                topology_fingerprint: None,
            });
            data.fft = Some(hdf5_fft_section(
                parent_analysis_id,
                analysis_ids,
                coordinate,
                results,
                requests,
                timeout_seconds,
            )?);
            write_hdf5_to_writer(writer, &data)
                .map_err(|error| map_hdf5_output_error(path, error))?;
        }
    }
    if crate::abort::reason().is_some() {
        return Err(super::cancellation_cli_error(timeout_seconds));
    }
    Ok(())
}

fn run_authored_restart(
    ctx: &RunContext<'_>,
    restart: &rspice_core::netlist::XyceRestartOptions,
    tstop: f64,
    max_step: f64,
    startup_mode: rspice_core::engine::TransientStartupMode,
    progress: &indicatif::ProgressBar,
) -> Result<rspice_core::engine::TransientResult, CliError> {
    validate_supported_restart_options(restart)?;
    let abort = crate::abort::ProgressAbort::new(progress);

    match (restart.job.as_deref(), restart.file.as_deref()) {
        (Some(_), Some(_)) => Err(restart_cli_error(
            ".OPTIONS RESTART cannot specify both JOB and FILE in one run",
        )),
        (Some(job), None) => {
            let interval = restart.initial_interval.ok_or_else(|| {
                restart_cli_error(
                    ".OPTIONS RESTART JOB requires a positive INITIAL_INTERVAL checkpoint cadence",
                )
            })?;
            let parent = restart_namespace_parent(&ctx.args.input)?;
            validate_restart_logical_name(job, "JOB")?;
            let available_points = ctx
                .engine
                .config()
                .resource_limits
                .max_analysis_points
                .saturating_sub(ctx.netlist.options.output_time_points.len())
                .saturating_sub(
                    ctx.netlist
                        .options
                        .output_interval_schedule
                        .as_ref()
                        .map_or(0, |schedule| schedule.intervals.len()),
                )
                .saturating_sub(ctx.netlist.options.timeint_breakpoints.len())
                .saturating_sub(restart.intervals.len());
            // Construction validates the complete nominal namespace before
            // the expensive simulation. Xyce may intentionally skip nominal
            // files when one accepted step crosses several cadence points.
            let plan = rspice_core::engine::XyceRestartJobPlan::new(
                job,
                interval,
                &restart.intervals,
                tstop,
                restart.pack,
                available_points,
            )
            .map_err(|error| restart_cli_error(error.to_string()))?;
            let (result, checkpoints) = ctx
                .engine
                .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                    ctx.netlist,
                    tstop,
                    max_step,
                    startup_mode,
                    plan.nominal_times(),
                    &abort,
                )
                .map_err(|error| map_restart_simulation_error(ctx, error))?;
            let mut previous_nominal = None;
            for scheduled in &checkpoints {
                let nominal_time = scheduled.nominal_time;
                if plan
                    .nominal_times()
                    .binary_search_by(|time| time.total_cmp(&nominal_time))
                    .is_err()
                    || previous_nominal.is_some_and(|previous| nominal_time <= previous)
                {
                    return Err(CliError::InternalError {
                        message: format!(
                            "transient restart scheduler returned unexpected nominal time {nominal_time:.17e}s"
                        ),
                    });
                }
                previous_nominal = Some(nominal_time);
                let name = plan.logical_name(nominal_time).ok_or_else(|| {
                    CliError::InternalError {
                        message: format!(
                            "transient restart scheduler returned unnamed nominal time {nominal_time:.17e}s"
                        ),
                    }
                })?;
                let path = safe_restart_write_path(&parent, &name)?;
                scheduled
                    .checkpoint
                    .save_with_encoding_and_abort(&path, plan.encoding(), &abort)
                    .map_err(|source| CliError::CoreSimulationError {
                        source,
                        analysis: Some(format!(
                            ".OPTIONS RESTART checkpoint save ({})",
                            path.display()
                        )),
                    })?;
                if !ctx.quiet {
                    println!(
                        "  Restart checkpoint saved (nominal t={nominal_time:.6e}s, accepted t={:.6e}s): {}",
                        scheduled.checkpoint.time,
                        path.display()
                    );
                }
            }
            Ok(result)
        }
        (None, Some(file)) => {
            if restart.initial_interval.is_some() || !restart.intervals.is_empty() {
                return Err(restart_cli_error(
                    ".OPTIONS RESTART FILE cannot also specify a checkpoint interval schedule; use a separate JOB run to write checkpoints",
                ));
            }
            let parent = restart_namespace_parent(&ctx.args.input)?;
            let path = safe_restart_read_path(&parent, file)?;
            let checkpoint_limit = ctx.engine.config().resource_limits.max_external_data_bytes;
            let checkpoint = rspice_core::engine::TransientCheckpoint::load_with_limit_and_abort(
                &path,
                checkpoint_limit,
                checkpoint_limit,
                &abort,
            )
            .map_err(|source| CliError::CoreSimulationError {
                source,
                analysis: Some(format!(
                    ".OPTIONS RESTART checkpoint load ({})",
                    path.display()
                )),
            })?;
            ctx.engine
                .run_tran_restart_resume_with_abort(
                    ctx.netlist,
                    &checkpoint,
                    tstop,
                    max_step,
                    &abort,
                )
                .map(|(result, _)| result)
                .map_err(|error| map_restart_simulation_error(ctx, error))
        }
        (None, None) => Err(restart_cli_error(
            ".OPTIONS RESTART requires either JOB for checkpoint output or FILE for restart input",
        )),
    }
}

fn validate_supported_restart_options(
    restart: &rspice_core::netlist::XyceRestartOptions,
) -> Result<(), CliError> {
    if restart.print_timeint_options.is_some() {
        return Err(restart_cli_error(
            ".OPTIONS RESTART PRINT_TIMEINT_OPTIONS is not supported; RSpice validates the saved integration configuration instead of importing Xyce time-integrator options",
        ));
    }
    if restart.start_time.is_some() {
        return Err(restart_cli_error(
            ".OPTIONS RESTART START_TIME filename inference is not supported; specify the exact checkpoint logical name with FILE",
        ));
    }
    Ok(())
}

fn restart_namespace_parent(input: &Path) -> Result<PathBuf, CliError> {
    let parent = input
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::canonicalize(parent).map_err(|error| {
        restart_cli_error(format!(
            "cannot resolve the input deck directory {} for .OPTIONS RESTART: {error}",
            parent.display()
        ))
    })
}

fn validate_restart_logical_name(name: &str, option: &str) -> Result<(), CliError> {
    if name.is_empty() || name.trim() != name || name == "." || name == ".." {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART {option} must be a non-empty logical filename without surrounding whitespace"
        )));
    }
    if name.contains(['/', '\\'])
        || name
            .chars()
            .any(|character| character.is_control() || "<>:\"|?*".contains(character))
        || name.ends_with(['.', ' '])
    {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART {option} must be one portable filename, without separators or reserved filename characters"
        )));
    }
    let path = Path::new(name);
    if path.is_absolute()
        || !matches!(
            path.components().collect::<Vec<_>>().as_slice(),
            [Component::Normal(_)]
        )
    {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART {option} must name exactly one file in the input deck directory"
        )));
    }
    let device_stem = name
        .split('.')
        .next()
        .unwrap_or_default()
        .trim_end_matches(['.', ' '])
        .to_ascii_uppercase();
    let reserved = matches!(device_stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (device_stem.len() == 4
            && matches!(&device_stem[..3], "COM" | "LPT")
            && matches!(device_stem.as_bytes()[3], b'1'..=b'9'));
    if reserved {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART {option} uses reserved device filename '{name}'"
        )));
    }
    Ok(())
}

fn safe_restart_write_path(parent: &Path, name: &str) -> Result<PathBuf, CliError> {
    validate_restart_logical_name(name, "JOB output")?;
    let path = parent.join(name);
    match std::fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(restart_cli_error(format!(
            "refusing to replace .OPTIONS RESTART symlink {}",
            path.display()
        ))),
        Ok(metadata) if metadata.is_dir() => Err(restart_cli_error(format!(
            ".OPTIONS RESTART destination {} is a directory",
            path.display()
        ))),
        Ok(_) => Ok(path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(path),
        Err(error) => Err(restart_cli_error(format!(
            "cannot inspect .OPTIONS RESTART destination {}: {error}",
            path.display()
        ))),
    }
}

fn safe_restart_read_path(parent: &Path, name: &str) -> Result<PathBuf, CliError> {
    validate_restart_logical_name(name, "FILE")?;
    let path = parent.join(name);
    let metadata = std::fs::symlink_metadata(&path).map_err(|error| {
        restart_cli_error(format!(
            "cannot inspect .OPTIONS RESTART FILE {}: {error}",
            path.display()
        ))
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART FILE must be a regular, non-symlink file in the input deck directory: {}",
            path.display()
        )));
    }
    let resolved = std::fs::canonicalize(&path).map_err(|error| {
        restart_cli_error(format!(
            "cannot resolve .OPTIONS RESTART FILE {}: {error}",
            path.display()
        ))
    })?;
    if resolved.parent() != Some(parent) {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART FILE escapes the input deck directory: {}",
            path.display()
        )));
    }
    Ok(resolved)
}

fn restart_cli_error(message: impl Into<String>) -> CliError {
    CliError::SimulationError {
        message: message.into(),
        analysis: Some("Transient restart".to_string()),
    }
}

fn map_restart_simulation_error(
    ctx: &RunContext<'_>,
    error: rspice_core::SimulationError,
) -> CliError {
    if matches!(error, rspice_core::SimulationError::Aborted) {
        super::cancellation_cli_error(ctx.args.timeout)
    } else {
        CliError::CoreSimulationError {
            source: error,
            analysis: Some("Transient restart".to_string()),
        }
    }
}

pub(super) fn run_fourier(
    ctx: &RunContext<'_>,
    four_index: usize,
    fundamental: f64,
    num_harmonics: usize,
) -> Result<(), CliError> {
    use rspice_core::analysis::{FourierAnalysis, FourierConfig};

    if !fundamental.is_finite() || fundamental <= 0.0 {
        return Err(CliError::simulation_error_in(
            format!("invalid Fourier fundamental frequency {fundamental}"),
            "Fourier",
        ));
    }
    if num_harmonics == 0 {
        return Err(CliError::simulation_error_in(
            "Fourier harmonic count must be at least one",
            "Fourier",
        ));
    }

    if !ctx.quiet {
        println!(
            "Running Fourier analysis: fundamental = {} Hz, {} harmonics",
            fundamental, num_harmonics
        );
    }

    // The plan names this card's operands and the transient it post-processes;
    // `.FOUR` never invents an independent transient schedule of its own.
    let (parent, planned_operands) = ctx.planned_fourier_operands(four_index)?;
    let retained = ctx.retained_transient(parent)?;
    let retained = &*retained;
    // A compressed parent transient publishes a decimated waveform, so a DFT
    // over it would report different harmonics than the same deck without
    // `--compress`. The core already evaluated this card against the exact
    // accepted trajectory; consume that instead of re-analyzing.
    let analyzed: Vec<(String, &'static str, rspice_core::analysis::FourierResult)> = match retained
        .post_results
        .as_ref()
    {
        Some(post) => {
            let mut analyzed = Vec::new();
            for spectrum in post
                .fourier
                .iter()
                .filter(|spectrum| spectrum.card_index == four_index)
            {
                if spectrum.fundamental != fundamental || spectrum.harmonic_count != num_harmonics {
                    return Err(CliError::InternalError {
                        message: format!(
                            "retained Fourier spectrum for card {} was evaluated at {} Hz with {} harmonics, but the card authors {fundamental} Hz with {num_harmonics}",
                            four_index + 1,
                            spectrum.fundamental,
                            spectrum.harmonic_count
                        ),
                    });
                }
                analyzed.push((
                    spectrum.output.clone(),
                    spectrum.physical_type,
                    spectrum.spectrum.clone(),
                ));
            }
            analyzed
        }
        None => {
            let columns = rspice_core::analysis::evaluate_tran_four_output_requests_with_abort(
                ctx.netlist,
                &retained.result,
                four_index,
                ctx.engine.config().resource_limits,
                &crate::abort::ProcessAbort,
            )
            .map_err(|error| map_output_projection_error(ctx, error, "Fourier"))?;

            let config = FourierConfig::new(fundamental).with_harmonics(num_harmonics);
            let fourier = FourierAnalysis::new(config);

            let mut analyzed = Vec::new();
            analyzed.try_reserve_exact(columns.len()).map_err(|_| {
                CliError::simulation_error_in("cannot allocate Fourier results", "Fourier")
            })?;
            for (output, physical_type, waveform) in columns {
                let result = fourier
                    .analyze_with_abort(
                        &retained.result.time,
                        &waveform,
                        &crate::abort::ProcessAbort,
                    )
                    .map_err(|error| {
                        if matches!(error, rspice_core::analysis::FourierError::Aborted) {
                            super::cancellation_cli_error(ctx.args.timeout)
                        } else {
                            CliError::simulation_error_in(
                                format!("Fourier output `{output}` could not be analyzed: {error}"),
                                "Fourier",
                            )
                        }
                    })?;
                analyzed.push((output, physical_type, result));
            }
            analyzed
        }
    };

    if !ctx.quiet {
        println!(
            "\nâ”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”"
        );
        println!("â”‚                    FOURIER ANALYSIS RESULTS                    â”‚");
        println!(
            "â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤"
        );

        for (output, physical_type, result) in &analyzed {
            println!("â”‚ Output: {:43} ({physical_type:8}) â”‚", output);
            println!("â”‚ DC component = {:<47.6e} â”‚", result.dc_component);
            println!(
                "â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤"
            );
            println!("â”‚  Harmonic    Frequency (Hz)    Magnitude    Phase (deg)        â”‚");
            println!(
                "â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤"
            );

            for harmonic in result.harmonics.iter().filter(|h| h.harmonic_number > 0) {
                println!(
                    "â”‚  {:3}        {:12.4e}      {:10.6}   {:10.2}          â”‚",
                    harmonic.harmonic_number,
                    harmonic.frequency,
                    harmonic.magnitude,
                    harmonic.phase
                );
            }

            println!(
                "â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤"
            );
            if let Some(thd) = result.thd {
                println!("â”‚  THD:            {thd:10.4} %                                  â”‚");
            } else {
                println!("â”‚  THD:             undefined                                  â”‚");
            }
            println!(
                "â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜"
            );
        }
    }

    // The core evaluates one spectrum per resolved operand and the shared
    // result document names one spectrum, so each operand is its own analysis
    // instance and publishes its own artifact. The plan already named those
    // instances and bound each to this transient, so the identities are read
    // off it and checked against the operands the resolver produced: a
    // mismatch would publish one operand's spectrum under another's name.
    if planned_operands.len() != analyzed.len() {
        return Err(CliError::InternalError {
            message: format!(
                "the canonical plan names {} operand(s) for .FOUR card {} of {}, but the resolver produced {}",
                planned_operands.len(),
                four_index + 1,
                retained.analysis_id,
                analyzed.len()
            ),
        });
    }
    for ((analysis_id, planned_output), (output, physical_type, result)) in
        planned_operands.into_iter().zip(&analyzed)
    {
        if planned_output != output {
            return Err(CliError::InternalError {
                message: format!(
                    "planned .FOUR operand {analysis_id} names '{planned_output}' but the resolver produced '{output}'"
                ),
            });
        }
        let Some(output_path) = ctx.output_path_for(&analysis_id.tag()) else {
            continue;
        };
        let one = [(output.clone(), *physical_type, result.clone())];
        super::document::publish_analysis_result(
            ctx,
            &output_path,
            analysis_id,
            // A Fourier spectrum is a harmonic table, not a named series set;
            // the typed harmonics live in the document's payload.
            super::document::empty_schema(),
            || {
                rspice_core::execution::AnalysisResultDocument::from_fourier(
                    analysis_id,
                    retained.analysis,
                    output,
                    fourier_output_unit(physical_type),
                    result,
                )
            },
            |path, format| {
                write_fourier_output(
                    path,
                    format,
                    &retained.analysis_id,
                    &analysis_id.tag(),
                    fundamental,
                    num_harmonics,
                    &one,
                )
            },
        )?;
        if !ctx.quiet {
            println!("\nResults written to: {}", output_path.display());
        }
    }

    Ok(())
}

/// The physical unit one Fourier spectrum's magnitudes carry.
fn fourier_output_unit(physical_type: &str) -> rspice_core::execution::SignalUnit {
    use rspice_core::execution::SignalUnit;
    match physical_type {
        "voltage" => SignalUnit::Volt,
        "current" => SignalUnit::Ampere,
        // `.FOUR` also accepts a braced expression, whose value is a bare
        // circuit parameter with no declared physical dimension.
        _ => SignalUnit::Dimensionless,
    }
}

/// Export Fourier results with full harmonic data (JSON or CSV).
fn write_fourier_output(
    path: &Path,
    format: OutputFormat,
    parent_analysis_id: &str,
    fourier_analysis_id: &str,
    fundamental: f64,
    num_harmonics: usize,
    analyzed: &[(String, &'static str, rspice_core::analysis::FourierResult)],
) -> Result<(), CliError> {
    let io_err = |e: std::io::Error| CliError::output_error(path, e);
    publish::artifact(
        path,
        |file| {
        match format {
            OutputFormat::Csv | OutputFormat::Tsv => {
                let sep = if matches!(format, OutputFormat::Tsv) {
                    '\t'
                } else {
                    ','
                };
                writeln!(
                file,
                "parent_analysis_id{0}analysis_id{0}physical_type{0}output{0}harmonic{0}frequency_hz{0}magnitude{0}phase_deg{0}dc_component{0}thd_percent",
                sep
            )
            .map_err(io_err)?;
                for (output, physical_type, result) in analyzed {
                    let thd_percent = result
                        .thd
                        .map(|value| format!("{value:.6}"))
                        .unwrap_or_default();
                    for harmonic in &result.harmonics {
                        writeln!(
                        file,
                        "{1}{0}{2}{0}{3}{0}{4}{0}{5}{0}{6:.17e}{0}{7:.17e}{0}{8:.6}{0}{9:.17e}{0}{10}",
                        sep,
                        parent_analysis_id,
                        fourier_analysis_id,
                        physical_type,
                        output,
                        harmonic.harmonic_number,
                        harmonic.frequency,
                        harmonic.magnitude,
                        harmonic.phase,
                        result.dc_component,
                        thd_percent
                    )
                    .map_err(io_err)?;
                    }
                }
            }
            _ => {
                // Fourier results are tables of harmonics, not waveforms; JSON is
                // the structured default for every other requested format.
                let results: Vec<serde_json::Value> = analyzed
                    .iter()
                    .map(|(output, physical_type, result)| {
                        let thd_ratio = result.thd.map(|value| value / 100.0);
                        serde_json::json!({
                            "output": output,
                            "physical_type": physical_type,
                            "dc_component": result.dc_component,
                            // Core's thd field is already a percentage.
                            "thd": thd_ratio,
                            "thd_percent": result.thd,
                            "harmonics": result
                                .harmonics
                                .iter()
                                .map(|harmonic| {
                                    serde_json::json!({
                                        "n": harmonic.harmonic_number,
                                        "frequency_hz": harmonic.frequency,
                                        "magnitude": harmonic.magnitude,
                                        "phase_deg": harmonic.phase,
                                    })
                                })
                                .collect::<Vec<_>>(),
                        })
                    })
                    .collect();

                let json = serde_json::json!({
                    "analysis": "fourier",
                    "analysis_id": fourier_analysis_id,
                    "parent_analysis_id": parent_analysis_id,
                    "fundamental_hz": fundamental,
                    "num_harmonics": num_harmonics,
                    "results": results,
                });
                let text = serde_json::to_string_pretty(&json)
                    .map_err(|e| CliError::output_json_error(path, e))?;
                writeln!(file, "{}", text).map_err(io_err)?;
            }
        }

            Ok(())
        },
    )
    .map_err(|error| map_atomic_output_error(path, error))
}

#[cfg(test)]
mod restart_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_FFT_TEST: AtomicU64 = AtomicU64::new(0);

    /// The canonical `fft-NNN` identities a deck with `count` authored `.FFT`
    /// requests publishes under, minted the same way the executor mints them.
    fn fft_test_identities(count: usize) -> Vec<String> {
        crate::commands::run::canonical_analysis_identities(
            rspice_core::execution::AnalysisKind::Fft,
            count,
        )
        .expect("canonical FFT identities")
        .iter()
        .map(|id| id.tag())
        .collect()
    }

    struct FftTestDirectory(PathBuf);

    impl FftTestDirectory {
        fn new() -> Self {
            let id = NEXT_FFT_TEST.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("rspice-cli-fft-raw-{}-{id}", std::process::id()));
            std::fs::create_dir(&path).expect("create FFT RAW test directory");
            Self(path)
        }
    }

    impl Drop for FftTestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn restart_logical_names_are_single_portable_components() {
        validate_restart_logical_name("trans_test2e-08", "FILE").unwrap();
        for unsafe_name in [
            "",
            ".",
            "..",
            "../state",
            "sub/state",
            "sub\\state",
            "C:state",
        ] {
            assert!(validate_restart_logical_name(unsafe_name, "FILE").is_err());
        }
    }

    #[test]
    fn fft_value_units_preserve_physical_type_and_transform_semantics() {
        use rspice_core::netlist::FftFormat::{Normalized, Unnormalized};

        assert_eq!(fft_value_unit("voltage", Normalized), Ok(Some("1")));
        assert_eq!(fft_value_unit("current", Normalized), Ok(Some("1")));
        assert_eq!(fft_value_unit("parameter", Normalized), Ok(Some("1")));
        assert_eq!(fft_value_unit("voltage", Unnormalized), Ok(Some("V")));
        assert_eq!(fft_value_unit("current", Unnormalized), Ok(Some("A")));
        assert_eq!(fft_value_unit("parameter", Unnormalized), Ok(None));
        assert!(fft_value_unit("unsupported", Normalized).is_err());
    }

    fn fft_publication_fixture() -> (
        rspice_core::Netlist,
        Vec<rspice_core::engine::TransientFftResult>,
    ) {
        let netlist = rspice_core::Netlist::parse(
            "typed FFT publication validation\n\
             V1 out 0 SIN(0 1 3k)\n\
             R1 out 0 1k\n\
             .options fft fftout=1\n\
             .tran 1u 1m\n\
             .fft v(out) np=8 format=unorm window=rect freq=3k\n\
             .fft {2*v(out)} np=16 window=hann freq=3k fmin=1k\n\
             .end\n",
        )
        .expect("parse FFT publication fixture");
        let transient = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .run_tran_with_abort(&netlist, 1.0e-3, 1.0e-6, &rspice_core::NoAbort)
            .expect("run FFT publication fixture");
        (netlist, transient.fft_results)
    }

    fn assert_fft_publication_rejected_for_every_format(
        directory: &FftTestDirectory,
        label: &str,
        results: &[rspice_core::engine::TransientFftResult],
        netlist: &rspice_core::Netlist,
    ) {
        for (format, extension) in [
            (OutputFormat::Json, "json"),
            (OutputFormat::Csv, "csv"),
            (OutputFormat::Tsv, "tsv"),
            (OutputFormat::Raw, "raw"),
            (OutputFormat::RawAscii, "ascii.raw"),
            (OutputFormat::Hdf5, "h5"),
        ] {
            let path = directory.0.join(format!("{label}.{extension}"));
            write_fft_output(
                &path,
                format,
                "tran-001",
                &fft_test_identities(netlist.fft_analyses.len()),
                None,
                results,
                netlist,
                None,
            )
            .expect_err("malformed FFT publication must fail closed");
            assert!(!path.exists(), "malformed {format:?} FFT was published");
        }
        let entries = std::fs::read_dir(&directory.0)
            .expect("read rejected FFT publication directory")
            .map(|entry| entry.expect("read rejected FFT entry").file_name())
            .collect::<Vec<_>>();
        assert!(
            entries.iter().all(|name| !name
                .to_string_lossy()
                .contains(rspice_output::STAGING_MARKER)),
            "rejected FFT publication left a staging artifact: {entries:?}"
        );
    }

    #[test]
    fn fft_publication_rejects_count_identity_and_core_invariant_corruption_in_every_format() {
        let (netlist, results) = fft_publication_fixture();
        let directory = FftTestDirectory::new();

        assert!(validate_fft_result_count(&[], &netlist.fft_analyses).is_err());
        assert_fft_publication_rejected_for_every_format(
            &directory,
            "missing-results",
            &[],
            &netlist,
        );

        let mut reordered = results.clone();
        reordered.swap(0, 1);
        assert_fft_publication_rejected_for_every_format(
            &directory,
            "reordered",
            &reordered,
            &netlist,
        );

        let mut wrong_mode = results.clone();
        wrong_mode[0].mode = rspice_core::netlist::XyceFftMode::SpectreCompatible;
        assert_fft_publication_rejected_for_every_format(
            &directory,
            "wrong-mode",
            &wrong_mode,
            &netlist,
        );

        let mut wrong_sampling_policy = results.clone();
        wrong_sampling_policy[0].accurate_sampling = false;
        assert_fft_publication_rejected_for_every_format(
            &directory,
            "wrong-sampling-policy",
            &wrong_sampling_policy,
            &netlist,
        );

        let mut missing_metrics = results.clone();
        missing_metrics[0].metrics = None;
        assert_fft_publication_rejected_for_every_format(
            &directory,
            "missing-metrics",
            &missing_metrics,
            &netlist,
        );

        let transient_path = directory.0.join("reordered.tran.json");
        let fft_path = directory.0.join("reordered.fft.json");
        let transient =
            TransientOutputDocument::Table(crate::commands::export_table::ExportTable {
                analysis: "transient".to_string(),
                plot_name: "Transient Analysis".to_string(),
                scale_name: "time".to_string(),
                scale_type: "time".to_string(),
                scale: vec![0.0],
                columns: Vec::new(),
            });
        write_transient_fft_output_pair(
            &transient_path,
            &transient,
            &fft_path,
            OutputFormat::Json,
            "tran-001",
            &fft_test_identities(netlist.fft_analyses.len()),
            None,
            &reordered,
            &netlist,
            None,
            u64::MAX,
        )
        .expect_err("malformed FFT pair must fail before staging either sibling");
        assert!(!transient_path.exists());
        assert!(!fft_path.exists());

        let mut impossible_results = results.clone();
        let mut impossible_requests = netlist.fft_analyses.clone();
        impossible_requests[0].fundamental_frequency = Some(2.0e3);
        impossible_requests[0].minimum_frequency = Some(0.0);
        impossible_requests[0].maximum_frequency = Some(1.0);
        impossible_results[0].fundamental_bin = 2;
        impossible_results[0].minimum_metric_bin = 0;
        impossible_results[0].maximum_metric_bin = 0;
        impossible_results[0].metrics = None;
        let mut impossible_netlist = netlist.clone();
        impossible_netlist.fft_analyses = impossible_requests;
        assert_fft_publication_rejected_for_every_format(
            &directory,
            "impossible-bounds",
            &impossible_results,
            &impossible_netlist,
        );

        let mut not_normalized = results.clone();
        not_normalized[1].metrics = None;
        let peak = not_normalized[1]
            .bins
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| left.magnitude.total_cmp(&right.magnitude))
            .map(|(index, _)| index)
            .expect("normalized FFT has a bin");
        not_normalized[1].bins[peak].real *= 0.5;
        not_normalized[1].bins[peak].imaginary *= 0.5;
        not_normalized[1].bins[peak].magnitude *= 0.5;
        assert_fft_publication_rejected_for_every_format(
            &directory,
            "not-normalized",
            &not_normalized,
            &netlist,
        );

        let mut stale_metrics = results;
        stale_metrics[0]
            .metrics
            .as_mut()
            .expect("FFTOUT fixture")
            .thd_ratio += 0.25;
        assert_fft_publication_rejected_for_every_format(
            &directory,
            "stale-metrics",
            &stale_metrics,
            &netlist,
        );
    }

    #[test]
    fn binary_and_ascii_fft_raw_artifacts_round_trip_typed_metadata_and_ragged_bins() {
        let netlist = rspice_core::Netlist::parse(
            "typed FFT RAW round trip\n\
             V1 out 0 SIN(0 1 3k)\n\
             R1 out 0 1k\n\
             .options fft fftout=1\n\
             .tran 1u 1m\n\
             .fft v(out) np=8 format=unorm window=rect freq=3k\n\
             .fft {2*v(out)} np=16 window=hann freq=3k fmin=1k\n\
             .end\n",
        )
        .expect("parse typed FFT RAW test deck");
        let transient = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .run_tran_with_abort(&netlist, 1.0e-3, 1.0e-6, &rspice_core::NoAbort)
            .expect("run typed FFT RAW test deck");
        assert_eq!(transient.fft_results.len(), 2);
        let directory = FftTestDirectory::new();

        for (format, extension, selected) in [
            (OutputFormat::Raw, "raw", "raw"),
            (OutputFormat::RawAscii, "ascii.raw", "ascii"),
        ] {
            let path = directory.0.join(format!("fft.{extension}"));
            write_fft_output(
                &path,
                format,
                "tran-007",
                &fft_test_identities(netlist.fft_analyses.len()),
                None,
                &transient.fft_results,
                &netlist,
                None,
            )
            .expect("write typed FFT RAW artifact");
            let decoded = read_fft_raw_artifact(&path).expect("decode typed FFT RAW artifact");
            assert_eq!(decoded.metadata.selected_format, selected);
            assert_eq!(decoded.metadata.parent_analysis_id, "tran-007");
            assert_eq!(decoded.metadata.result_count, 2);
            assert_eq!(decoded.metadata.frequency_unit, "Hz");
            assert_eq!(decoded.metadata.phase_unit, "degree");
            assert_eq!(decoded.metadata.complex_representation, "cartesian");
            assert_eq!(decoded.metadata.results[0].analysis_id, "fft-001");
            assert_eq!(decoded.metadata.results[1].analysis_id, "fft-002");
            assert_eq!(decoded.metadata.results[0].signal.physical_type, "voltage");
            assert_eq!(
                decoded.metadata.results[0].signal.unit.as_deref(),
                Some("V")
            );
            assert_eq!(
                decoded.metadata.results[1].signal.physical_type,
                "parameter"
            );
            assert_eq!(
                decoded.metadata.results[1].signal.unit.as_deref(),
                Some("1")
            );
            assert_eq!(
                decoded.metadata.results[0]
                    .transform
                    .sfdr_search_minimum_bin,
                decoded.metadata.results[0].transform.fundamental_bin
            );
            assert_eq!(
                decoded.metadata.results[1]
                    .transform
                    .sfdr_search_minimum_bin,
                decoded.metadata.results[1].transform.minimum_metric_bin
            );
            assert_ne!(
                decoded.metadata.results[1].transform.fundamental_bin,
                decoded.metadata.results[1].transform.minimum_metric_bin
            );
            assert!(decoded.metadata.results[0].metrics.is_some());
            assert!(decoded.metadata.results[1].metrics.is_some());
            assert_eq!(decoded.bins[0].analysis_id, "fft-001");
            let second_start = transient.fft_results[0].bins.len();
            assert_eq!(decoded.bins[second_start].analysis_id, "fft-002");
            assert_eq!(decoded.bins[second_start].index, 0);
            assert_eq!(decoded.bins[second_start].frequency_hz, 0.0);
            assert_eq!(
                decoded.bins.len(),
                transient
                    .fft_results
                    .iter()
                    .map(|result| result.bins.len())
                    .sum::<usize>()
            );
            assert!((decoded.bins[1].real - transient.fft_results[0].bins[1].real).abs() < 1e-14);
            assert!(
                (decoded.bins[1].imaginary - transient.fft_results[0].bins[1].imaginary).abs()
                    < 1e-14
            );
            assert!(
                (decoded.bins[1].magnitude - transient.fft_results[0].bins[1].magnitude).abs()
                    < 1e-14
            );
            assert!(
                (decoded.bins[1].phase_degrees - transient.fft_results[0].bins[1].phase_degrees)
                    .abs()
                    < 1e-14
            );
            let entries = std::fs::read_dir(&directory.0)
                .expect("read FFT RAW test directory")
                .map(|entry| entry.expect("read FFT RAW entry").file_name())
                .collect::<Vec<_>>();
            assert!(
                entries.iter().all(|name| !name
                    .to_string_lossy()
                    .contains(rspice_output::STAGING_MARKER)),
                "atomic FFT RAW staging artifact remained: {entries:?}"
            );

            if matches!(format, OutputFormat::Raw) {
                let mut old = decoded.metadata.clone();
                old.schema_version = 1;
                assert!(validate_fft_raw_metadata(&old).is_err());

                let mut future = decoded.metadata.clone();
                future.schema_version = FFT_ARTIFACT_SCHEMA_VERSION + 1;
                assert!(validate_fft_raw_metadata(&future).is_err());

                let mut malformed = decoded.metadata.clone();
                malformed.results[0].analysis_id = "fft-999".to_string();
                assert!(validate_fft_raw_metadata(&malformed).is_err());

                let mut unknown_enum = decoded.metadata.clone();
                unknown_enum.results[0].transform.window = "future_window".to_string();
                assert!(validate_fft_raw_metadata(&unknown_enum).is_err());

                let mut inconsistent_source = decoded.metadata.clone();
                inconsistent_source.results[0].source.kind = "expression".to_string();
                assert!(validate_fft_raw_metadata(&inconsistent_source).is_err());

                let mut impossible_bounds = decoded.metadata.clone();
                impossible_bounds.results[0].transform.fundamental_bin = 2;
                impossible_bounds.results[0].transform.minimum_metric_bin = 0;
                impossible_bounds.results[0].transform.maximum_metric_bin = 0;
                impossible_bounds.results[0]
                    .transform
                    .sfdr_search_minimum_bin = 0;
                impossible_bounds.results[0].metrics = None;
                assert!(validate_fft_raw_metadata(&impossible_bounds).is_err());

                let first_bin_count = decoded.metadata.results[0].sampling.point_count / 2 + 1;
                let first_bins = &decoded.bins[..first_bin_count];
                let mut wrong_thd = decoded.metadata.results[0].clone();
                wrong_thd
                    .metrics
                    .as_mut()
                    .expect("metric fixture")
                    .thd_ratio += 0.25;
                assert!(validate_fft_raw_metrics_against_bins(&wrong_thd, first_bins).is_err());

                let mut wrong_spur = decoded.metadata.results[0].clone();
                let metrics = wrong_spur.metrics.as_mut().expect("metric fixture");
                metrics.sfdr_spur_bin = Some(2);
                metrics.sfdr_spur_frequency_hz =
                    Some(2.0 * wrong_spur.transform.frequency_resolution_hz);
                assert!(validate_fft_raw_metrics_against_bins(&wrong_spur, first_bins).is_err());

                let mut wrong_harmonic = decoded.metadata.results[0].clone();
                wrong_harmonic
                    .metrics
                    .as_mut()
                    .expect("metric fixture")
                    .largest_harmonics[0]
                    .magnitude += 1.0e-6;
                assert!(
                    validate_fft_raw_metrics_against_bins(&wrong_harmonic, first_bins).is_err()
                );

                let original = std::fs::read(&path).expect("read binary FFT RAW bytes");
                let mut corrupt_schema = original.clone();
                let schema_offset = corrupt_schema
                    .windows(b"fft_real\tvalue".len())
                    .position(|window| window == b"fft_real\tvalue")
                    .expect("find FFT RAW variable type")
                    + b"fft_real\t".len();
                corrupt_schema[schema_offset..schema_offset + 5].copy_from_slice(b"bogus");
                std::fs::write(&path, &corrupt_schema).expect("write corrupt variable schema");
                assert!(read_fft_raw_artifact(&path).is_err());

                let mut non_finite = original.clone();
                let binary_offset = non_finite
                    .windows(b"Binary:\n".len())
                    .position(|window| window == b"Binary:\n")
                    .expect("find FFT RAW binary payload")
                    + b"Binary:\n".len();
                non_finite[binary_offset..binary_offset + 8]
                    .copy_from_slice(&f64::NAN.to_le_bytes());
                std::fs::write(&path, &non_finite).expect("write non-finite FFT RAW row");
                assert!(read_fft_raw_artifact(&path).is_err());

                let mut wrong_phase = original.clone();
                let second_row_phase = binary_offset + 7 * 8 + 4 * 8;
                wrong_phase[second_row_phase..second_row_phase + 8]
                    .copy_from_slice(&123.0_f64.to_le_bytes());
                std::fs::write(&path, &wrong_phase).expect("write inconsistent FFT RAW phase");
                assert!(read_fft_raw_artifact(&path).is_err());

                let mut not_normalized = original.clone();
                let normalized_peak = decoded.bins[second_start..]
                    .iter()
                    .enumerate()
                    .max_by(|(_, left), (_, right)| left.magnitude.total_cmp(&right.magnitude))
                    .map(|(index, _)| second_start + index)
                    .expect("normalized FFT has bins");
                for (column, value) in [
                    (1, decoded.bins[normalized_peak].real * 0.5),
                    (2, decoded.bins[normalized_peak].imaginary * 0.5),
                    (3, decoded.bins[normalized_peak].magnitude * 0.5),
                ] {
                    let offset = binary_offset + (normalized_peak * 7 + column) * 8;
                    not_normalized[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
                }
                std::fs::write(&path, &not_normalized)
                    .expect("write non-normalized FFT RAW spectrum");
                assert!(read_fft_raw_artifact(&path).is_err());

                let mut negative_sub_pico = original.clone();
                let first_row_magnitude = binary_offset + 3 * 8;
                negative_sub_pico[first_row_magnitude..first_row_magnitude + 8]
                    .copy_from_slice(&(-1.0e-300_f64).to_le_bytes());
                std::fs::write(&path, &negative_sub_pico)
                    .expect("write negative sub-pico FFT RAW magnitude");
                assert!(read_fft_raw_artifact(&path).is_err());
                std::fs::write(&path, original).expect("restore binary FFT RAW fixture");
            }
        }

        let hdf5_path = directory.0.join("fft.h5");
        write_fft_output(
            &hdf5_path,
            OutputFormat::Hdf5,
            "tran-007",
            &fft_test_identities(netlist.fft_analyses.len()),
            None,
            &transient.fft_results,
            &netlist,
            None,
        )
        .expect("write typed FFT HDF5 artifact");
        let hdf5 = crate::hdf5::read_hdf5(&hdf5_path).expect("decode typed FFT HDF5 artifact");
        let hdf5_fft = hdf5.fft.expect("typed FFT HDF5 section");
        assert_eq!(hdf5_fft.results[0].physical_type, "voltage");
        assert_eq!(hdf5_fft.results[0].value_unit.as_deref(), Some("V"));
        assert_eq!(hdf5_fft.results[1].physical_type, "parameter");
        assert_eq!(hdf5_fft.results[1].value_unit.as_deref(), Some("1"));
        assert_eq!(
            hdf5_fft.results[0].sfdr_search_minimum_bin,
            hdf5_fft.results[0].fundamental_bin
        );
        assert_eq!(
            hdf5_fft.results[1].sfdr_search_minimum_bin,
            hdf5_fft.results[1].minimum_metric_bin
        );
    }
}
