//! Time-domain and DC analysis cards: `.OP`, `.DC`, `.TRAN`, `.FOUR`, `.TEMP`.
//!
//! Each entry point runs one card against the shared [`RunContext`], prints
//! the console report, and exports the `op`/`dc`/`tran`/`four`/`temp` tagged
//! file when `-o` is set. `.TRAN` also owns the progress bar, the
//! `--checkpoint`/`--resume` segmented path, and `--tran-stop`.

use super::RunContext;
use super::shared::{NodeResolver, map_hdf5_output_error};
use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::{
    dc_export_signals, dc_operating_point_current_signals, dc_operating_point_signals,
    dc_operating_point_voltage_signals, transient_export_signals,
};
use crate::hdf5::{Hdf5SimulationData, Hdf5WaveformSection, write_hdf5};
use std::borrow::Cow;
use std::collections::HashSet;
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

            if !ctx.quiet {
                let voltage_signals = dc_operating_point_voltage_signals(&result);
                let current_signals = dc_operating_point_current_signals(&result);
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

            if let Some(ref output_path) = ctx.output_path_for("op") {
                write_dc_op_output(output_path, &result, ctx.format, &ctx.netlist.saves)?;
                if !ctx.quiet {
                    println!("Results exported to: {}", output_path.display());
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

fn write_dc_op_output(
    path: &Path,
    result: &rspice_core::solver::SimulationResult,
    format: OutputFormat,
    saves: &rspice_core::netlist::SaveSet,
) -> Result<(), CliError> {
    use std::io::Write;

    let signals =
        crate::commands::run_signals::apply_save_set(dc_operating_point_signals(result), saves);

    if matches!(format, OutputFormat::Hdf5) {
        let mut data = Hdf5SimulationData::new();
        data.title = "DC Operating Point".to_string();

        let mut operating_point = Hdf5WaveformSection::new("point", vec![0.0]);
        for signal in &signals {
            operating_point.add_typed_signal(
                signal.display_name.clone(),
                signal.kind.raw_variable_type(),
                signal.values.clone(),
            );
        }
        data.operating_point = Some(operating_point);

        write_hdf5(path, &data).map_err(|err| map_hdf5_output_error(path, err))?;
        return Ok(());
    }

    let mut file = std::fs::File::create(path).map_err(|e| CliError::OutputError {
        path: path.to_path_buf(),
        source: e,
    })?;

    match format {
        OutputFormat::Json => {
            let mut vars = serde_json::Map::new();
            for signal in &signals {
                vars.insert(
                    signal.display_name.clone(),
                    serde_json::json!(signal.values[0]),
                );
            }
            let json = serde_json::json!({
                "analysis": "dc_op",
                "variables": vars,
            });
            let text = serde_json::to_string_pretty(&json)
                .map_err(|e| CliError::output_json_error(path, e))?;
            writeln!(file, "{}", text).map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
        }
        OutputFormat::Csv => {
            writeln!(file, "signal,value").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            for signal in &signals {
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
            for signal in &signals {
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
                    signal.kind.raw_variable_type()
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
            for signal in &signals {
                writeln!(file, "\t{:.17e}", signal.values[0]).map_err(|e| {
                    CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    }
                })?;
            }
        }
        OutputFormat::Hdf5 => unreachable!("HDF5 handled before text writers"),
    }

    Ok(())
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

    match ctx.engine.run_dc_sweep2_with_abort(
        ctx.netlist,
        source,
        start,
        stop,
        step,
        sweep2,
        &crate::abort::ProcessAbort,
    ) {
        Ok(results) => {
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

            ctx.record_measurements(
                "DC",
                rspice_core::analysis::evaluate_dc_measurements(ctx.netlist, &results),
            );

            if let Some(ref output_path) = ctx.output_path_for("dc") {
                let sweep_vals: Vec<f64> = results.iter().map(|(v, _)| *v).collect();
                let signals = dc_export_signals(
                    ctx.netlist,
                    &results,
                    ctx.engine.config().resource_limits,
                    &crate::abort::ProcessAbort,
                )
                .map_err(|error| map_output_projection_error(ctx, error, "DC"))?;
                super::shared::ensure_finite_series(
                    ctx.args.allow_nonfinite,
                    "DC Sweep output projection",
                    signals
                        .iter()
                        .map(|signal| (signal.display_name.as_str(), signal.values.as_slice())),
                )?;
                match ctx.format {
                    OutputFormat::Hdf5 => {
                        let mut data = Hdf5SimulationData::new();
                        data.title = "DC Sweep".to_string();

                        let mut dc_sweep = Hdf5WaveformSection::new(source, sweep_vals.clone());
                        for signal in &signals {
                            dc_sweep.add_typed_signal(
                                signal.display_name.clone(),
                                signal.kind.raw_variable_type(),
                                signal.values.clone(),
                            );
                        }
                        data.dc_sweep = Some(dc_sweep);

                        write_hdf5(output_path, &data)
                            .map_err(|err| map_hdf5_output_error(output_path, err))?;
                    }
                    OutputFormat::Raw
                    | OutputFormat::RawAscii
                    | OutputFormat::Csv
                    | OutputFormat::Tsv
                    | OutputFormat::Json => {
                        super::export::scalar_table(
                            "dc_sweep",
                            "DC transfer characteristic",
                            source,
                            "voltage",
                            sweep_vals,
                            &signals,
                        )
                        .write(output_path, ctx.format)?;
                    }
                }

                if !ctx.quiet {
                    println!("Results exported to: {}", output_path.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "DC Sweep")),
    }
}

pub(super) fn run_transient(
    ctx: &RunContext<'_>,
    tstop: f64,
    tstep: f64,
    tstart: f64,
    max_step: Option<f64>,
    uic: bool,
) -> Result<(), CliError> {
    // --tran-stop overrides the deck's stop time so checkpoint segments can
    // share byte-identical source (the checkpoint fingerprint covers it).
    let tstop = ctx.args.tran_stop.unwrap_or(tstop);
    let internal_max_step = resolve_transient_max_step(tstep, tstop, tstart, max_step);

    let pb = if ctx.quiet {
        indicatif::ProgressBar::hidden()
    } else if ctx.show_progress {
        // The engine reports its completed fraction at the abort-poll
        // cadence, so this is a real percentage, not a spinner.
        let pb = indicatif::ProgressBar::new(crate::abort::ProgressAbort::SCALE);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{bar:30.green} {percent:>3}% [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        pb.set_message(format!(
            "Transient: {} to {} s (step {})",
            tstart, tstop, tstep
        ));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    } else {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message(format!(
            "Running transient: {} to {} (step {})...",
            tstart, tstop, tstep
        ));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    };

    let authored_restart = ctx.netlist.options.restart.as_ref();
    if authored_restart.is_some() && (ctx.args.checkpoint.is_some() || ctx.args.resume.is_some()) {
        return Err(restart_cli_error(
            ".OPTIONS RESTART cannot be combined with --checkpoint or --resume; choose one restart control plane",
        ));
    }

    let checkpointing = ctx.args.checkpoint.is_some() || ctx.args.resume.is_some();
    let startup_mode = rspice_core::engine::TransientStartupMode::from_uic(uic);
    let result = if let Some(restart) = authored_restart {
        let restart_run =
            run_authored_restart(ctx, restart, tstop, internal_max_step, startup_mode, &pb);
        pb.finish_and_clear();
        Ok(restart_run?)
    } else if checkpointing {
        // Segmented integration: restore the saved state (when resuming),
        // run to this segment's stop time, and persist the new state (when
        // checkpointing). The core validates the netlist fingerprint, so a
        // checkpoint can never silently continue a different circuit.
        let run = if let Some(ref resume_path) = ctx.args.resume {
            let checkpoint =
                rspice_core::engine::TransientCheckpoint::load(resume_path).map_err(|e| {
                    CliError::SimulationError {
                        message: format!("cannot resume from {}: {e}", resume_path.display()),
                        analysis: Some("Transient".to_string()),
                    }
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
            ctx.engine
                .run_tran_resume(ctx.netlist, &checkpoint, tstop, internal_max_step)
        } else {
            ctx.engine.run_tran_checkpointed_with_startup_mode(
                ctx.netlist,
                tstop,
                internal_max_step,
                startup_mode,
            )
        };
        pb.finish_and_clear();
        match run {
            Ok((result, checkpoint)) => {
                if let Some(ref checkpoint_path) = ctx.args.checkpoint {
                    checkpoint
                        .save(checkpoint_path)
                        .map_err(|e| CliError::SimulationError {
                            message: format!(
                                "cannot save checkpoint to {}: {e}",
                                checkpoint_path.display()
                            ),
                            analysis: Some("Transient".to_string()),
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
    } else if ctx.compress && ctx.netlist.options.output_time_points.is_empty() {
        let compression_tol = ctx.compress_tol;
        let compression = rspice_core::engine::CompressionConfig {
            enabled: true,
            abs_tol: compression_tol,
            rel_tol: compression_tol,
            min_interval: tstep / 10.0,
        };
        let result = ctx.engine.run_tran_compressed_with_startup_mode_and_abort(
            ctx.netlist,
            tstop,
            internal_max_step,
            startup_mode,
            compression,
            &crate::abort::ProgressAbort::new(&pb),
        );
        pb.finish_and_clear();
        match result {
            Ok(compressed) => {
                if !ctx.quiet {
                    println!(
                        "✓ Transient complete (compressed): {} points (compression ratio: {:.1}x)",
                        compressed.time.len(),
                        (tstop / tstep) / compressed.time.len() as f64
                    );
                }
                Ok(compressed.into())
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

            ctx.record_measurements(
                "TRAN",
                rspice_core::analysis::evaluate_tran_measurements(ctx.netlist, &result),
            );

            if let Some(ref output_path) = ctx.output_path_for("tran") {
                let output_start = result
                    .time
                    .first()
                    .copied()
                    .map_or(tstart, |first| tstart.max(first));
                let projection = result
                    .output_projection(&ctx.netlist.options.output_time_points, output_start, tstop)
                    .map_err(|message| CliError::simulation_error_in(message, "Transient"))?;
                let output_time = projection
                    .project(&result.time)
                    .map_err(|message| CliError::simulation_error_in(message, "Transient"))?;
                let mut signals = transient_export_signals(
                    ctx.netlist,
                    &result,
                    ctx.engine.config().resource_limits,
                    &crate::abort::ProcessAbort,
                )
                .map_err(|error| map_output_projection_error(ctx, error, "Transient"))?;
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
                match ctx.format {
                    OutputFormat::Hdf5 => {
                        let mut data = Hdf5SimulationData::new();
                        data.title = "Transient Analysis".to_string();

                        let mut transient = Hdf5WaveformSection::new("time", output_time.clone());
                        for signal in &signals {
                            transient.add_typed_signal(
                                signal.display_name.clone(),
                                signal.kind.raw_variable_type(),
                                signal.values.clone(),
                            );
                        }
                        data.transient = Some(transient);

                        write_hdf5(output_path, &data)
                            .map_err(|err| map_hdf5_output_error(output_path, err))?;
                    }
                    OutputFormat::Raw
                    | OutputFormat::RawAscii
                    | OutputFormat::Csv
                    | OutputFormat::Tsv
                    | OutputFormat::Json => {
                        super::export::scalar_table(
                            "transient",
                            "Transient Analysis",
                            "time",
                            "time",
                            output_time,
                            &signals,
                        )
                        .write(output_path, ctx.format)?;
                    }
                }

                if !ctx.quiet {
                    println!("  Results exported to: {}", output_path.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Transient")),
    }
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
                .saturating_sub(ctx.netlist.options.timeint_breakpoints.len())
                .saturating_sub(restart.intervals.len());
            let schedule =
                build_restart_schedule(interval, &restart.intervals, tstop, available_points)?;
            // Validate the complete nominal namespace before starting an
            // expensive simulation. Xyce may intentionally skip nominal
            // files when one accepted step crosses several cadence points.
            validate_restart_checkpoint_names(job, &schedule)?;
            let (result, checkpoints) = ctx
                .engine
                .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                    ctx.netlist,
                    tstop,
                    max_step,
                    startup_mode,
                    &schedule,
                    &abort,
                )
                .map_err(|error| map_restart_simulation_error(ctx, error))?;
            let mut previous_nominal = None;
            for scheduled in &checkpoints {
                let nominal_time = scheduled.nominal_time;
                if schedule
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
                let name = format!("{job}{}", xyce_restart_time_suffix(nominal_time));
                let path = safe_restart_write_path(&parent, &name)?;
                scheduled.checkpoint.save(&path).map_err(|error| {
                    restart_cli_error(format!(
                        "cannot save .OPTIONS RESTART checkpoint {}: {error}",
                        path.display()
                    ))
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
            let checkpoint =
                rspice_core::engine::TransientCheckpoint::load(&path).map_err(|error| {
                    restart_cli_error(format!(
                        "cannot load .OPTIONS RESTART checkpoint {}: {error}",
                        path.display()
                    ))
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
    if restart.pack.is_some() {
        return Err(restart_cli_error(
            ".OPTIONS RESTART PACK selects Xyce's packed restart-file encoding, which RSpice does not read or write",
        ));
    }
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

fn build_restart_schedule(
    initial_interval: f64,
    intervals: &[rspice_core::netlist::XyceRestartInterval],
    tstop: f64,
    max_points: usize,
) -> Result<Vec<f64>, CliError> {
    if !initial_interval.is_finite() || initial_interval <= 0.0 {
        return Err(restart_cli_error(
            ".OPTIONS RESTART INITIAL_INTERVAL must be finite and positive",
        ));
    }
    if !tstop.is_finite() || tstop <= 0.0 {
        return Err(restart_cli_error(
            ".OPTIONS RESTART requires a finite, positive .TRAN stop time",
        ));
    }
    let mut previous_transition = None;
    for (index, transition) in intervals.iter().enumerate() {
        if !transition.time.is_finite()
            || transition.time < 0.0
            || previous_transition.is_some_and(|previous| transition.time <= previous)
        {
            return Err(restart_cli_error(format!(
                ".OPTIONS RESTART transition {index} time must be finite, nonnegative, and strictly increasing"
            )));
        }
        if !transition.interval.is_finite() || transition.interval <= 0.0 {
            return Err(restart_cli_error(format!(
                ".OPTIONS RESTART transition {index} interval must be finite and positive"
            )));
        }
        previous_transition = Some(transition.time);
    }

    let mut schedule = Vec::new();
    push_bounded_checkpoint(&mut schedule, 0.0, max_points)?;
    let mut current = 0.0;
    loop {
        let next = next_restart_time(current, initial_interval, intervals)?;
        if next > tstop {
            break;
        }
        push_bounded_checkpoint(&mut schedule, next, max_points)?;
        current = next;
    }
    Ok(schedule)
}

fn next_restart_time(
    current: f64,
    initial_interval: f64,
    intervals: &[rspice_core::netlist::XyceRestartInterval],
) -> Result<f64, CliError> {
    let first_transition = intervals.first().map(|transition| transition.time);
    let candidate = if first_transition.is_none_or(|first| current < first) {
        let cadence = current + initial_interval;
        first_transition.map_or(cadence, |first| cadence.min(first))
    } else {
        let active_index = intervals.partition_point(|transition| transition.time <= current) - 1;
        let active = intervals[active_index];
        let steps = ((current - active.time) / active.interval).floor();
        let cadence = active.time + (steps + 1.0) * active.interval;
        intervals
            .get(active_index + 1)
            .map_or(cadence, |next| cadence.min(next.time))
    };
    if !candidate.is_finite() || candidate <= current {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART cadence cannot advance beyond {current:.17e}s; increase the interval or reduce the simulated time scale"
        )));
    }
    Ok(candidate)
}

fn push_bounded_checkpoint(
    schedule: &mut Vec<f64>,
    time: f64,
    max_points: usize,
) -> Result<(), CliError> {
    if schedule.len() >= max_points {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART schedule exceeds the configured analysis-point limit of {max_points}"
        )));
    }
    schedule.push(time);
    Ok(())
}

fn validate_restart_checkpoint_names(job: &str, schedule: &[f64]) -> Result<(), CliError> {
    let mut unique = HashSet::with_capacity(schedule.len());
    for &time in schedule {
        let name = format!("{job}{}", xyce_restart_time_suffix(time));
        if !unique.insert(name.clone()) {
            return Err(restart_cli_error(format!(
                ".OPTIONS RESTART filename precision maps more than one checkpoint to '{name}'; choose a wider checkpoint interval or shorter stop time"
            )));
        }
    }
    Ok(())
}

fn xyce_restart_time_suffix(time: f64) -> String {
    if time == 0.0 {
        return "0".to_string();
    }
    let exponent = time.abs().log10().floor() as i32;
    if (-4..6).contains(&exponent) {
        let decimals = usize::try_from((5 - exponent).max(0)).unwrap_or(0);
        let mut text = format!("{time:.decimals$}");
        while text.ends_with('0') && text.contains('.') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        text
    } else {
        let scientific = format!("{time:.5e}");
        let (mantissa, exponent) = scientific
            .split_once('e')
            .expect("Rust scientific formatting always contains an exponent");
        let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        let exponent = exponent
            .parse::<i32>()
            .expect("Rust scientific formatting emits a numeric exponent");
        format!("{mantissa}e{exponent:+03}")
    }
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
    fundamental: f64,
    outputs: &[String],
    num_harmonics: usize,
) -> Result<(), CliError> {
    use rspice_core::analysis::{FourierAnalysis, FourierConfig};

    if !ctx.quiet {
        println!(
            "Running Fourier analysis: fundamental = {} Hz, {} harmonics",
            fundamental, num_harmonics
        );
        if ctx.verbose {
            println!("  Output nodes: {:?}", outputs);
        }
    }

    let period = 1.0 / fundamental;
    let analysis_time = period * (num_harmonics + 2) as f64;
    let tstep = period / 100.0;

    let tran_result = ctx
        .engine
        .run_tran(ctx.netlist, analysis_time, tstep)
        .map_err(|e| CliError::simulation_error_in(e.to_string(), "Fourier (transient)"))?;

    let config = FourierConfig::new(fundamental).with_harmonics(num_harmonics);
    let fourier = FourierAnalysis::new(config);
    let resolver = NodeResolver::from_netlist(ctx.engine, ctx.netlist)?;

    let mut analyzed: Vec<(String, rspice_core::analysis::FourierResult)> = Vec::new();
    for output in outputs {
        if let Some((node_idx, reference_idx)) = resolver.parse_voltage_probe(output) {
            let result = if reference_idx == 0 {
                let waveform = fourier_voltage_waveform(&tran_result, output, node_idx)?;
                fourier.analyze(&tran_result.time, waveform.as_ref())
            } else {
                let pos_waveform = fourier_voltage_waveform(&tran_result, output, node_idx)?;
                let neg_waveform = fourier_voltage_waveform(&tran_result, output, reference_idx)?;
                let diff_waveform: Vec<f64> = pos_waveform
                    .iter()
                    .zip(neg_waveform.iter())
                    .map(|(vp, vn)| vp - vn)
                    .collect();
                fourier.analyze(&tran_result.time, &diff_waveform)
            };
            analyzed.push((output.clone(), result));
        } else if !ctx.quiet {
            println!("Warning: Could not find node for output '{}'", output);
        }
    }

    if !ctx.quiet {
        println!("\n┌────────────────────────────────────────────────────────────────┐");
        println!("│                    FOURIER ANALYSIS RESULTS                    │");
        println!("├────────────────────────────────────────────────────────────────┤");

        for (output, result) in &analyzed {
            println!("│ Output: {:54} │", output);
            println!("│ DC component = {:<47.6e} │", result.dc_component);
            println!("├────────────────────────────────────────────────────────────────┤");
            println!("│  Harmonic    Frequency (Hz)    Magnitude    Phase (deg)        │");
            println!("├────────────────────────────────────────────────────────────────┤");

            for harmonic in result.harmonics.iter().filter(|h| h.harmonic_number > 0) {
                println!(
                    "│  {:3}        {:12.4e}      {:10.6}   {:10.2}          │",
                    harmonic.harmonic_number,
                    harmonic.frequency,
                    harmonic.magnitude,
                    harmonic.phase
                );
            }

            println!("├────────────────────────────────────────────────────────────────┤");
            println!(
                "│  THD:            {:10.4} %                                  │",
                result.thd
            );
            println!("└────────────────────────────────────────────────────────────────┘");
        }
    }

    if let Some(ref output_path) = ctx.output_path_for("four") {
        write_fourier_output(
            output_path,
            ctx.format,
            fundamental,
            num_harmonics,
            &analyzed,
        )?;
        if !ctx.quiet {
            println!("\nResults written to: {}", output_path.display());
        }
    }

    Ok(())
}

fn fourier_voltage_waveform<'a>(
    result: &'a rspice_core::engine::TransientResult,
    output: &str,
    node: usize,
) -> Result<Cow<'a, [f64]>, CliError> {
    if node == 0 {
        return Ok(Cow::Owned(vec![0.0; result.time.len()]));
    }

    result
        .try_voltage_waveform(node)
        .map(Cow::Borrowed)
        .ok_or_else(|| {
            CliError::simulation_error_in(
                format!(
                    "Fourier output '{output}' is not available: node {node} is outside transient result node range 0..={}",
                    result.num_nodes
                ),
                "Fourier",
            )
        })
}

/// Export Fourier results with full harmonic data (JSON or CSV).
fn write_fourier_output(
    path: &Path,
    format: OutputFormat,
    fundamental: f64,
    num_harmonics: usize,
    analyzed: &[(String, rspice_core::analysis::FourierResult)],
) -> Result<(), CliError> {
    use std::io::Write;

    let io_err = |e: std::io::Error| CliError::output_error(path, e);
    let mut file = std::fs::File::create(path).map_err(io_err)?;

    match format {
        OutputFormat::Csv | OutputFormat::Tsv => {
            let sep = if matches!(format, OutputFormat::Tsv) {
                '\t'
            } else {
                ','
            };
            writeln!(
                file,
                "output{0}harmonic{0}frequency_hz{0}magnitude{0}phase_deg{0}dc_component{0}thd_percent",
                sep
            )
            .map_err(io_err)?;
            for (output, result) in analyzed {
                for harmonic in &result.harmonics {
                    writeln!(
                        file,
                        "{1}{0}{2}{0}{3:.17e}{0}{4:.17e}{0}{5:.6}{0}{6:.17e}{0}{7:.6}",
                        sep,
                        output,
                        harmonic.harmonic_number,
                        harmonic.frequency,
                        harmonic.magnitude,
                        harmonic.phase,
                        result.dc_component,
                        result.thd,
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
                .map(|(output, result)| {
                    serde_json::json!({
                        "output": output,
                        "dc_component": result.dc_component,
                        // Core's thd field is already a percentage.
                        "thd": result.thd / 100.0,
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
}

pub(super) fn run_temp(ctx: &RunContext<'_>, temperatures: &[f64]) -> Result<(), CliError> {
    if !ctx.quiet {
        println!("Running temperature sweep: {} points", temperatures.len());
        if ctx.verbose {
            println!("  Temperatures: {:?} °C", temperatures);
        }
    }

    let mut results: Vec<(f64, Vec<f64>)> = Vec::new();
    let mut node_names: Vec<String> = Vec::new();

    for (i, temp_c) in temperatures.iter().enumerate() {
        let temp_k = temp_c + 273.15;

        if !ctx.quiet {
            println!(
                "\n[{}/{}] Temperature: {:.1} °C ({:.1} K)",
                i + 1,
                temperatures.len(),
                temp_c,
                temp_k
            );
        }

        let mut temp_config = ctx.engine.config().clone();
        temp_config.temperature = temp_k;
        let temp_engine = rspice_core::Engine::new(temp_config);

        match temp_engine.run_dc_op(ctx.netlist) {
            Ok(result) => {
                if ctx.verbose && !ctx.quiet {
                    for j in 1..=result.node_voltages.len().min(5) {
                        println!("  V({}) = {:.6} V", j, result.voltage(j));
                    }
                }
                if node_names.is_empty() {
                    node_names = result.node_names.clone();
                }
                results.push((*temp_c, result.node_voltages.clone()));
            }
            Err(e) => {
                if !ctx.quiet {
                    eprintln!("  DC OP failed at {:.1} °C: {}", temp_c, e);
                }
            }
        }
    }

    if results.is_empty() && !temperatures.is_empty() {
        return Err(CliError::simulation_error_in(
            "no temperature point converged",
            "Temperature Sweep",
        ));
    }

    if !ctx.quiet {
        println!("\n┌─────────────────────────────────────┐");
        println!("│     Temperature Sweep Summary       │");
        println!("├─────────────────────────────────────┤");
        println!(
            "│  Points:  {:3}/{:3} converged         │",
            results.len(),
            temperatures.len()
        );
        println!(
            "│  Range:   {:6.1} °C to {:6.1} °C    │",
            temperatures.first().unwrap_or(&0.0),
            temperatures.last().unwrap_or(&0.0)
        );
        println!("└─────────────────────────────────────┘");
    }

    if let Some(ref output_path) = ctx.output_path_for("temp") {
        use std::io::Write;

        let mut file = std::fs::File::create(output_path).map_err(|e| CliError::OutputError {
            path: output_path.clone(),
            source: e,
        })?;

        match ctx.format {
            OutputFormat::Csv => {
                let num_nodes = results.first().map(|(_, v)| v.len()).unwrap_or(0);
                let header: String = (1..num_nodes)
                    .map(|i| {
                        let name = node_names.get(i).cloned().unwrap_or_else(|| i.to_string());
                        format!(",V({})", name)
                    })
                    .collect();
                writeln!(file, "Temperature_C{}", header).map_err(|e| CliError::OutputError {
                    path: output_path.clone(),
                    source: e,
                })?;

                for (temp, voltages) in &results {
                    // Skip index 0: ground is not a column
                    let values: String = voltages
                        .iter()
                        .skip(1)
                        .map(|v| format!(",{:.17e}", v))
                        .collect();
                    writeln!(file, "{:.2}{}", temp, values).map_err(|e| CliError::OutputError {
                        path: output_path.clone(),
                        source: e,
                    })?;
                }
            }
            OutputFormat::Json => {
                let json = serde_json::json!({
                    "analysis": "temperature_sweep",
                    "temperatures_c": temperatures,
                    "results": results.iter().map(|(t, v)| {
                        serde_json::json!({
                            "temperature_c": t,
                            "voltages": v,
                        })
                    }).collect::<Vec<_>>(),
                });
                let text = serde_json::to_string_pretty(&json)
                    .map_err(|e| CliError::output_json_error(output_path, e))?;
                writeln!(file, "{}", text).map_err(|e| CliError::OutputError {
                    path: output_path.clone(),
                    source: e,
                })?;
            }
            _ => {
                for (temp, voltages) in &results {
                    writeln!(file, "T={:.2}C: {:?}", temp, voltages).map_err(|e| {
                        CliError::OutputError {
                            path: output_path.clone(),
                            source: e,
                        }
                    })?;
                }
            }
        }

        if !ctx.quiet {
            println!("\nResults written to: {}", output_path.display());
        }
    }

    Ok(())
}

fn resolve_transient_max_step(
    tstep: f64,
    tstop: f64,
    tstart: f64,
    explicit_max_step: Option<f64>,
) -> f64 {
    explicit_max_step
        .filter(|step| step.is_finite() && *step > 0.0)
        .unwrap_or_else(|| default_transient_max_step(tstep, tstop, tstart))
        .max(1e-18)
}

fn default_transient_max_step(tstep: f64, tstop: f64, tstart: f64) -> f64 {
    let analysis_window = tstop - tstart;
    let fallback_window = if analysis_window.is_finite() && analysis_window > 0.0 {
        analysis_window
    } else {
        tstop.abs().max(tstep.abs())
    };
    let window_limit = if fallback_window.is_finite() && fallback_window > 0.0 {
        fallback_window / 50.0
    } else {
        1e-18
    };

    if tstep.is_finite() && tstep > 0.0 && tstep < window_limit {
        tstep
    } else {
        window_limit
    }
}

#[cfg(test)]
mod restart_tests {
    use super::*;

    #[test]
    fn restart_schedule_applies_each_interval_at_its_transition() {
        let intervals = [
            rspice_core::netlist::XyceRestartInterval {
                time: 10.0,
                interval: 4.0,
            },
            rspice_core::netlist::XyceRestartInterval {
                time: 17.0,
                interval: 1.0,
            },
        ];
        assert_eq!(
            build_restart_schedule(3.0, &intervals, 19.0, 32).unwrap(),
            vec![0.0, 3.0, 6.0, 9.0, 10.0, 14.0, 17.0, 18.0, 19.0]
        );
    }

    #[test]
    fn restart_schedule_is_resource_bounded() {
        let error = build_restart_schedule(1.0, &[], 4.0, 4)
            .expect_err("five requested checkpoints exceed a four-point limit");
        assert!(error.to_string().contains("analysis-point limit of 4"));
    }

    #[test]
    fn restart_suffix_matches_xyce_compact_default_float_spelling() {
        assert_eq!(xyce_restart_time_suffix(0.0), "0");
        assert_eq!(xyce_restart_time_suffix(5e-9), "5e-09");
        assert_eq!(xyce_restart_time_suffix(20e-9), "2e-08");
        assert_eq!(xyce_restart_time_suffix(15e-9), "1.5e-08");
        assert_eq!(xyce_restart_time_suffix(0.0001), "0.0001");
        assert_eq!(xyce_restart_time_suffix(1e6), "1e+06");
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
}
