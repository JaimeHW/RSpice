//! Execution of the time-domain and DC analysis cards: `.OP`, `.DC`, `.TRAN`.
//!
//! Each entry point runs one card against the shared [`RunContext`], prints
//! the console report, and exports the `op`/`dc`/`tran` tagged file when `-o`
//! is set. `.TRAN` also owns the progress bar, the `--checkpoint`/`--resume`
//! path, and `--tran-stop`.
//!
//! What a transient's post-processing publishes lives beside it rather than
//! here: [`super::fft_document`] owns the `.FFT` artifact in every format,
//! [`super::fourier_document`] owns the per-operand `.FOUR` artifacts, and
//! [`super::restart`] owns the authored segmented-restart schedule.

use super::RunContext;
use super::shared::map_hdf5_output_error;
use crate::cli::{CliError, OutputFormat, map_atomic_output_error};
use crate::commands::publish;
use crate::commands::run_signals::{
    SignalKind, checked_dc_operating_point_signals, dc_export_signals,
    dc_operating_point_export_signals, transient_export_signals,
};
use crate::hdf5::{Hdf5SimulationData, Hdf5WaveformSection, write_hdf5, write_hdf5_to_writer};
use std::path::Path;

use super::fft_document::{validate_fft_result_count, write_transient_fft_output_pair};
use super::restart::{map_restart_simulation_error, restart_cli_error, run_authored_restart};

pub(super) fn map_output_projection_error(
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
            OutputFormat::Vcd => {
                return Err(crate::commands::vcd_io::unsupported_analysis("operating point"));
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
        // Carry the engine's typed failure rather than its text. Stringifying
        // here re-decided every DC-sweep failure as the simulation category,
        // so a refused capability and a singular matrix left this process with
        // the same status.
        Err(source) => Err(CliError::CoreSimulationError {
            source,
            analysis: Some("DC Sweep".to_string()),
        }),
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
            "Worst compression error: none — all {} accepted samples were retained exactly",
            report.input_points
        );
    };
    let relative = worst
        .relative_error
        .map_or_else(|| "n/a".to_string(), |relative| format!("{:.3e}", relative));
    format!(
        "Worst compression error: {} {} at t={:.6e}s — absolute {:.3e}, relative {}, \
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
                    "✓ Transient complete: {} time points computed",
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
                    OutputFormat::Vcd => {
                        if result.digital_traces.is_empty() && result.real_traces.is_empty() {
                            eprintln!(
                                "Warning: this transient captured no digital or real event node; \
                                 {} declares no signal and records no change",
                                output_path.display()
                            );
                        }
                        TransientOutputDocument::Vcd(Box::new(
                            crate::commands::vcd_io::event_document(
                                &output_path,
                                &result.digital_traces,
                                &result.real_traces,
                                // `--expand-buses` asks for the members as
                                // scalars and no vector, which is what an
                                // empty table means to the projection.
                                if ctx.args.expand_buses {
                                    &[]
                                } else {
                                    &result.digital_buses
                                },
                            )?,
                        ))
                    }
                    OutputFormat::Raw
                    | OutputFormat::RawAscii
                    | OutputFormat::Csv
                    | OutputFormat::Tsv => TransientOutputDocument::Table {
                        table: super::export::scalar_table(
                            "transient",
                            "Transient Analysis",
                            "time",
                            "time",
                            output_time,
                            &signals,
                        ),
                        events: rspice_core::execution::transient_event_plots(
                            &result.digital_traces,
                            &result.real_traces,
                        ),
                        buses: rspice_core::execution::transient_bus_plots(
                            &result.digital_traces,
                            &result.digital_buses,
                        )
                        .map_err(|error| {
                            crate::commands::waveform_io::conversion_error(&output_path, error)
                        })?,
                    },
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
        // Carry the engine's typed failure rather than its text. Stringifying
        // here re-decided every transient failure as the simulation category,
        // so a refused capability and a singular matrix left this process with
        // the same status.
        Err(source) => Err(CliError::CoreSimulationError {
            source,
            analysis: Some("Transient".to_string()),
        }),
    }
}

pub(super) enum TransientOutputDocument {
    Hdf5(Box<Hdf5SimulationData>),
    /// The flat tabular projection, and the event timelines the run captured.
    ///
    /// The table is exactly what it has always been. The timelines travel
    /// beside it rather than in it because a rawfile is a sequence of plots
    /// and an event history — irregular, with times of its own — is a plot
    /// rather than a column. Only the rawfile formats have anywhere to put
    /// them; the delimited ones carry the grid-sampled `D()` columns the
    /// table already holds and nothing more.
    Table {
        table: super::export::ExportTable,
        events: Vec<rspice_core::io::RawEventTimeline>,
        /// The declared buses, as their own plot family. The member plots are
        /// in `events` and stay the authoritative copy; these carry the
        /// grouping and the bit order, which nothing else in a rawfile does.
        buses: Vec<rspice_core::io::RawBusTimeline>,
    },
    /// The shared typed result document, published for `-f json`.
    Typed(Box<rspice_core::execution::AnalysisResultDocument>),
    /// The event timelines as a Value Change Dump, published for `-f vcd`.
    ///
    /// This is the one transient artifact that carries no analog waveform:
    /// the analysis grid is what the other formats are for, and an event dump
    /// is for the times the events actually happened.
    Vcd(Box<rspice_core::io::VcdDocument>),
}

impl TransientOutputDocument {
    pub(super) fn write_to(
        &self,
        writer: &mut dyn std::io::Write,
        path: &Path,
        format: OutputFormat,
        byte_limit: u64,
    ) -> Result<(), CliError> {
        match self {
            Self::Hdf5(data) => write_hdf5_to_writer(writer, data)
                .map_err(|error| map_hdf5_output_error(path, error)),
            Self::Table {
                table,
                events,
                buses,
            } => {
                table.write_to(writer, path, format)?;
                // Appended inside the caller's staging closure, so a rawfile
                // that carries event plots is still published whole or not at
                // all.
                let raw_format = match format {
                    OutputFormat::Raw => rspice_core::io::RawFormat::Binary,
                    OutputFormat::RawAscii => rspice_core::io::RawFormat::Ascii,
                    OutputFormat::Csv
                    | OutputFormat::Tsv
                    | OutputFormat::Json
                    | OutputFormat::Hdf5
                    | OutputFormat::Vcd => return Ok(()),
                };
                rspice_core::io::write_event_plots(writer, events, buses, raw_format)
                    .map_err(|error| CliError::output_error(path, error))
            }
            Self::Vcd(document) => rspice_core::io::write_vcd(&mut *writer, document)
                .map_err(|error| crate::commands::vcd_io::write_error(path, error)),
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
