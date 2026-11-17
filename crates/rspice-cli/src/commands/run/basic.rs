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
use super::stored_transient::RunEvents;
use crate::cli::{CliError, OutputFormat, map_atomic_output_error};
use crate::commands::publish;
use crate::commands::run_signals::{
    SignalKind, checked_dc_operating_point_signals, dc_export_signals, project_dc_operating_point,
    transient_export_signals,
};
use crate::hdf5::{Hdf5SimulationData, Hdf5WaveformSection, write_hdf5, write_hdf5_to_writer};
use std::path::Path;

use super::fft_document::{validate_fft_result_count, write_transient_fft_output_pair};
use super::restart::{restart_cli_error, run_authored_restart};

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
        crate::console::line(format_args!("Running DC operating point..."))?;
    }

    match ctx
        .engine
        .run_dc_op_with_report_and_abort(ctx.netlist, &crate::abort::ProcessAbort)
    {
        Ok((result, op_report)) => finish_dc_op_result(ctx, &result, &op_report),
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

pub(super) fn finish_dc_op_result(
    ctx: &RunContext<'_>,
    result: &rspice_core::solver::SimulationResult,
    op_report: &rspice_core::circuit::DeviceOpReport,
) -> Result<(), CliError> {
    let operating_point_signals = checked_dc_operating_point_signals(result)
        .map_err(|error| map_output_projection_error(ctx, error, "DC OP"))?;
    let exported_operating_point_signals = project_dc_operating_point(
        ctx.netlist,
        result,
        &operating_point_signals,
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
        crate::console::line(format_args!("DC Operating Point:"))?;
        for signal in voltage_signals.iter().take(10) {
            // A thermal net's value is a temperature, in kelvin.
            crate::console::line(format_args!(
                "  {} = {:.9e} {}",
                signal.display_name,
                signal.values[0],
                signal.unit_symbol().unwrap_or_default()
            ))?;
        }
        if voltage_signals.len() > 10 {
            crate::console::line(format_args!(
                "  ... ({} more node voltages)",
                voltage_signals.len() - 10
            ))?;
        }
        for signal in current_signals.iter().take(5) {
            crate::console::line(format_args!(
                "  {} = {:.9e} A",
                signal.display_name, signal.values[0]
            ))?;
        }
        if current_signals.len() > 5 {
            crate::console::line(format_args!(
                "  ... ({} more branch currents)",
                current_signals.len() - 5
            ))?;
        }

        print_device_op_report(op_report, ctx.verbose)?;
    }

    ctx.record_assertions(rspice_core::analysis::evaluate_op_assertions(
        ctx.netlist,
        result,
        op_report,
    ));
    let measurements = rspice_core::analysis::evaluate_op_measurements_with_abort(
        ctx.netlist,
        result,
        &crate::abort::ProcessAbort,
    )
    .map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some("OP measurement projection".to_string()),
    })?;
    ctx.record_measurements("OP", measurements)?;

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
                    result,
                    Some(op_report),
                )
            },
            |path, format| {
                // Only an HDF5 artifact carries the run's identity, whose
                // topology fingerprint can cost an elaboration of its own.
                let identity = match format {
                    OutputFormat::Hdf5 => Some(super::document::hdf5_identity(ctx, analysis_id)?),
                    _ => None,
                };
                write_dc_op_output(
                    path,
                    &exported_operating_point_signals,
                    format,
                    identity.as_ref(),
                )
            },
        )?;
        if !ctx.quiet {
            crate::console::line(format_args!(
                "Results exported to: {}",
                output.path.display()
            ))?;
        }
    }

    Ok(())
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
fn print_device_op_report(
    report: &rspice_core::circuit::DeviceOpReport,
    verbose: bool,
) -> Result<(), CliError> {
    if report.is_empty() {
        return Ok(());
    }

    const COMPACT_ROW_CAP: usize = 25;
    let cap = if verbose { usize::MAX } else { COMPACT_ROW_CAP };

    crate::console::line(format_args!("Device Operating Points:"))?;
    for entry in report.entries.iter().take(cap) {
        let region = entry
            .region
            .map(|region| format!(" [{}]", region))
            .unwrap_or_default();
        // A Verilog-A instance is listed under its module, its quantities
        // under the module's names and units.
        let kind = entry
            .module
            .as_ref()
            .map_or(entry.device_kind, |module| module.module.as_str());
        let params = entry
            .params
            .iter()
            .map(|(name, value)| format!("{}={}", name, format_engineering(*value)))
            .chain(entry.module.iter().flat_map(|module| {
                module.quantities.iter().map(|quantity| {
                    format!(
                        "{}={}{}",
                        quantity.name,
                        format_engineering(quantity.value),
                        quantity.units
                    )
                })
            }))
            .collect::<Vec<_>>()
            .join("  ");
        crate::console::line(format_args!(
            "  {:<16} {:<7}{} {}",
            entry.name, kind, region, params
        ))?;
    }
    if report.entries.len() > cap {
        crate::console::line(format_args!(
            "  ... ({} more devices; rerun with --verbose for the full table)",
            report.entries.len() - cap
        ))?;
    }
    Ok(())
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
                signal.unit_symbol(),
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
                // The rule the shared rawfile writer follows: every value is a
                // double, which LTspice's readers decode as doubles only on
                // `double`, and one point never decreases, so it is `forward`.
                writeln!(file, "Flags: real forward double").map_err(|e| CliError::OutputError {
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
                if matches!(format, OutputFormat::Raw) {
                    // `raw` is the binary rawfile: one little-endian double per
                    // variable for the single point.
                    writeln!(file, "Binary:").map_err(|e| CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    })?;
                    for signal in signals {
                        file.write_all(&signal.values[0].to_le_bytes())
                            .map_err(|e| CliError::OutputError {
                                path: path.to_path_buf(),
                                source: e,
                            })?;
                    }
                    return Ok(());
                }
                writeln!(file, "Values:").map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
                // The point index belongs on the first value's line, not on a
                // line of its own: a rawfile's column-oriented body is one
                // value per line with the row index prefixed to the first, and
                // an index alone is a row this product's own reader — and
                // ngspice's — counts as a value and then runs out of file.
                for (index, signal) in signals.iter().enumerate() {
                    let value = signal.values[0];
                    if index == 0 {
                        writeln!(file, "0\t{value:.17e}")
                    } else {
                        writeln!(file, "\t{value:.17e}")
                    }
                    .map_err(|e| CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
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

/// Run a `.DC` card. When it is the sweep the deck's Xyce `.SENS` study
/// reads and that study has yet to run, the study is read from this same
/// sweep and kept for its card.
pub(super) fn run_dc_sweep(
    ctx: &RunContext<'_>,
    card: &rspice_core::netlist::AnalysisCommand,
    source: &str,
    spec: &rspice_core::netlist::DcSweepSpec,
    sweep2: Option<&rspice_core::netlist::DcSecondSweep>,
) -> Result<(), CliError> {
    let result = match super::frequency::xyce_sensitivity_cards_reading(ctx, card) {
        Some(cards) => ctx
            .engine
            .run_dc_analysis_with_xyce_sensitivity_and_abort(
                ctx.netlist,
                source,
                spec,
                sweep2,
                &cards,
                &crate::abort::ProcessAbort,
            )
            .map(|(result, study)| {
                ctx.retain_xyce_dc_sensitivity(study);
                result
            }),
        None => ctx.engine.run_dc_analysis_with_abort(
            ctx.netlist,
            source,
            spec,
            sweep2,
            &crate::abort::ProcessAbort,
        ),
    }
    .map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some("DC Sweep".to_string()),
    })?;
    finish_dc_sweep_result(ctx, source, &result)
}

pub(super) fn finish_dc_sweep_result(
    ctx: &RunContext<'_>,
    source: &str,
    result: &rspice_core::engine::DcSweepResult,
) -> Result<(), CliError> {
    let axis = result.axes.last().ok_or_else(|| CliError::InternalError {
        message: "DC sweep result has no axis".into(),
    })?;
    let axis_type = match &axis.unit {
        rspice_core::execution::SignalUnit::Volt => "voltage",
        rspice_core::execution::SignalUnit::Ampere => "current",
        rspice_core::execution::SignalUnit::Custom(unit) if unit == "degC" => "temperature",
        _ => "notype",
    };
    let results = result
        .points
        .iter()
        .map(|p| (p.sweep_value, p.result.clone()))
        .collect::<Vec<_>>();
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
        crate::console::line(format_args!("DC Sweep: {} points computed", results.len()))?;
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
    ctx.record_measurements("DC", measurements)?;
    let continuous_measurements =
        rspice_core::analysis::evaluate_dc_continuous_measurements_with_limits_and_abort(
            ctx.netlist,
            &results,
            &ctx.engine.config().resource_limits,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| CliError::CoreSimulationError {
            source,
            analysis: Some("DC continuous measurement projection".to_string()),
        })?;
    super::shared::record_continuous_measurements(ctx, "DC_CONT", continuous_measurements)?;
    ctx.record_assertions(rspice_core::analysis::evaluate_dc_assertions(
        ctx.netlist,
        &results,
    ));

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
    super::xyce_files::record_print(
        ctx,
        rspice_core::netlist::OutputAnalysisKind::Dc,
        super::xyce_files::Scale::None,
        || {
            Ok(super::export::scalar_table(
                "dc_sweep",
                "DC transfer characteristic",
                source,
                axis_type,
                results.iter().map(|(value, _)| *value).collect(),
                &signals,
            ))
        },
    )?;

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
                rspice_core::execution::AnalysisResultDocument::from_dc_analysis(
                    analysis_id,
                    result,
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
                            signal.unit_symbol(),
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
                    axis_type,
                    sweep_vals.clone(),
                    &signals,
                )
                .write(path, format),
            },
        )?;

        if !ctx.quiet {
            crate::console::line(format_args!(
                "Results exported to: {}",
                output.path.display()
            ))?;
        }
    }
    Ok(())
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
) -> Result<(), CliError> {
    if ctx.quiet {
        return Ok(());
    }
    crate::console::line(format_args!(
        "{headline}: {} of {} accepted points (compression ratio: {:.1}x)",
        compressed.time.len(),
        compressed.input_points,
        compressed.compression_ratio
    ))?;
    crate::console::line(format_args!(
        "  {}",
        describe_compression_error(&compressed.compression_report)
    ))?;
    Ok(())
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
    /// With [`TransientStore::Fourier`], what its `.FOUR` cards read of the run besides the rows
    /// around their windows, which are all `result` holds; `None` when `result` holds every row.
    pub(super) fourier: Option<super::stored_transient::FourierRecords>,
}

pub(super) fn run_transient(
    ctx: &RunContext<'_>,
    tstop: f64,
    tstep: f64,
    tstart: f64,
    max_step: Option<f64>,
    uic: bool,
    state_times: &[f64],
    store: TransientStore,
) -> Result<TransientOutcome, CliError> {
    // --tran-stop overrides the deck's stop time so checkpoint segments can
    // share byte-identical source (the checkpoint fingerprint covers it).
    let deck_stop = tstop;
    let tstop = ctx.args.tran_stop.unwrap_or(tstop);
    let maximum_step = |tstop| {
        rspice_core::execution::resolve_dialect_transient_maximum_step(
            ctx.engine.config().spice_dialect,
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
        })
    };
    let internal_max_step = maximum_step(tstop)?;

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
    let checkpoint_path = ctx.transient_checkpoint_path(ctx.args.checkpoint.as_deref());
    let resume_path = ctx.transient_checkpoint_path(ctx.args.resume.as_deref());
    if authored_restart.is_some() && (checkpoint_path.is_some() || resume_path.is_some()) {
        return Err(restart_cli_error(
            ".OPTIONS RESTART cannot be combined with --checkpoint or --resume; choose one restart control plane",
        ));
    }

    let checkpointing = checkpoint_path.is_some() || resume_path.is_some();
    // A checkpoint before the deck's own stop interrupts the deck's run, which takes the deck's
    // steps: its maximum step is the deck's, and so is every ceiling derived from it.
    let interrupted = checkpoint_path.is_some() && resume_path.is_none() && tstop < deck_stop;
    let internal_max_step = if interrupted {
        maximum_step(deck_stop)?
    } else {
        internal_max_step
    };
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
    let compression = ctx.compress.then(compression_config);
    // Every transient writes its rows to a results store as it goes. Compression decides what
    // the published waveform keeps, not where the rows live while the run goes: the compressed
    // output is chosen from the trajectory in the store once the run is over, and its
    // post-processing products are the ones evaluated on that whole trajectory.
    let progress_abort = crate::abort::ProgressAbort::new(&pb);
    let stored = if let Some(restart) = authored_restart {
        // The restart schedule writes its checkpoints from the accepted trajectory, so
        // compressing the published waveform afterwards leaves every authored restart file
        // byte-identical.
        let run = run_authored_restart(
            ctx,
            restart,
            (tstop, tstep, internal_max_step),
            startup_mode,
            &pb,
        );
        pb.finish_and_clear();
        run?
    } else if checkpointing {
        // Segmented integration: restore the saved state (when resuming), run this segment into
        // a results store, and persist the new state (when checkpointing). The core validates
        // the netlist fingerprint, so a checkpoint can never silently continue a different
        // circuit.
        let resumed = match resume_path.as_deref() {
            Some(resume_path) => Some(load_resume_checkpoint(
                ctx,
                resume_path,
                checkpoint_path.as_deref(),
                startup_mode,
                &progress_abort,
            )?),
            None => None,
        };
        let segment = match &resumed {
            Some(checkpoint) => super::stored_transient::Segment::Resumed(checkpoint),
            // A stop before the deck's own interrupts the deck's run there, as a checkpoint
            // taken mid-run does: the run keeps the deck's window, so a resume to the deck's
            // stop takes every step the run takes with a breakpoint at the interruption.
            None if interrupted => super::stored_transient::Segment::Interrupted {
                tstop: deck_stop,
                max_step: internal_max_step,
            },
            None => super::stored_transient::Segment::Whole,
        };
        let run = super::stored_transient::run_segment(
            ctx,
            segment,
            (tstop, tstep, internal_max_step),
            startup_mode,
            &progress_abort,
        );
        pb.finish_and_clear();
        let (stored, end) = run?;
        if let Err(error) =
            publish_segment_end(ctx, &end, checkpoint_path.as_deref(), &progress_abort)
        {
            // The run's store goes with the run it could not finish publishing.
            let _ = stored.rows.publish(None, false);
            return Err(error);
        }
        stored
    } else if !state_times.is_empty() {
        // The `ACTIMES=` cards that share this run: it lands on their times,
        // captures each state there for them, and runs on to its stop.
        let span = super::stored_transient::StoreSpan {
            start: 0.0,
            tstop,
            tstep,
            max_step: internal_max_step,
        };
        let run = super::stored_transient::run_with(ctx, span, None, |sink| {
            ctx.engine.run_tran_capturing_states_to_stop_into_sink(
                ctx.netlist,
                (tstop, internal_max_step),
                startup_mode,
                state_times,
                &progress_abort,
                sink,
            )
        });
        pb.finish_and_clear();
        let (stored, states) = run?;
        ctx.hold_shared_transient_states(state_times, states);
        stored
    } else {
        let run = super::stored_transient::run(
            ctx,
            tstop,
            tstep,
            internal_max_step,
            startup_mode,
            &progress_abort,
        );
        pb.finish_and_clear();
        run?
    };
    finish_stored_transient(ctx, stored, (tstart, tstop), store, compression.as_ref())
}

/// What a transient's caller reads of the result once its outputs are written. Every transient
/// writes its rows to a results store; a caller that reads the waveforms back gets them in the
/// result, and the outputs of one that does not may stream from the store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TransientStore {
    /// The caller reads nothing of the waveforms.
    Streamed,
    /// The caller's `.FOUR` cards read the rows around their windows and what their operands
    /// read of the rest.
    Fourier,
    /// The caller reads the whole trajectory: a control script.
    ReadBack,
}

/// Load the checkpoint `--resume` names, refusing one whose startup mode is not the deck's.
fn load_resume_checkpoint(
    ctx: &RunContext<'_>,
    resume_path: &std::path::Path,
    checkpoint_path: Option<&std::path::Path>,
    startup_mode: rspice_core::engine::TransientStartupMode,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<rspice_core::engine::TransientCheckpoint, CliError> {
    super::restart::protect_input(resume_path, checkpoint_path)?;
    let checkpoint_limit = ctx.engine.config().resource_limits.max_external_data_bytes;
    let checkpoint = rspice_core::engine::TransientCheckpoint::load_with_limit_and_abort(
        resume_path,
        checkpoint_limit,
        checkpoint_limit,
        abort,
    )
    .map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some(format!(
            "Transient checkpoint load ({})",
            resume_path.display()
        )),
    })?;
    if checkpoint.startup_mode() != startup_mode {
        return Err(CliError::SimulationError {
            message: format!(
                "checkpoint startup mode {:?} does not match the selected .TRAN startup mode {:?}",
                checkpoint.startup_mode(),
                startup_mode
            ),
            analysis: Some("Transient".to_string()),
        });
    }
    Ok(checkpoint)
}

/// Write the checkpoint a segment ended with to `--checkpoint`'s file, or say why there is none.
fn publish_segment_end(
    ctx: &RunContext<'_>,
    end: &rspice_core::engine::TransientEnd,
    checkpoint_path: Option<&std::path::Path>,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<(), CliError> {
    match (end, checkpoint_path) {
        (rspice_core::engine::TransientEnd::Checkpoint(checkpoint), Some(path)) => {
            super::restart::publish_checkpoint(
                ctx,
                checkpoint,
                path,
                rspice_core::engine::TransientCheckpointEncoding::Unpacked,
                abort,
            )?;
            if !ctx.quiet {
                crate::console::line(format_args!(
                    "  Checkpoint saved (t={:.6e}s): {}",
                    checkpoint.time,
                    path.display()
                ))?;
            }
        }
        // A finished run has nothing left to resume.
        (rspice_core::engine::TransientEnd::Finished(finish), Some(path)) => {
            if !ctx.quiet {
                crate::console::line(format_args!(
                    "  No checkpoint saved to {}: {finish}, which completes the run",
                    path.display()
                ))?;
            }
        }
        (_, None) => {}
    }
    Ok(())
}

/// Publish a transient whose rows are in a store, streaming them from it when the outputs allow
/// and reading them back otherwise, then leave the store beside the output or delete it.
///
/// A run its storage stopped publishes what it kept, as a run that ended at its last kept row,
/// and then ends as stopped; an output that cannot be written for the shortened run is a
/// warning, since the stop is what the run reports. When writing the outputs fills the disk, the
/// store keeps the rows the outputs could not take and the run ends as stopped the same way.
///
/// With `compression`, the outputs are the compressed waveform of the trajectory in the store, and
/// its post-processing products are the ones evaluated on that whole trajectory.
/// A run its storage stopped publishes what it kept uncompressed, since the stop is what it
/// reports.
fn finish_stored_transient(
    ctx: &RunContext<'_>,
    mut stored: super::stored_transient::StoredTransient,
    (tstart, tstop): (f64, f64),
    store: TransientStore,
    compression: Option<&rspice_core::engine::CompressionConfig>,
) -> Result<TransientOutcome, CliError> {
    let stopped = stored.rows.stopped();
    let tstop = if stopped {
        stored.rows.last_time()?.unwrap_or(tstart)
    } else {
        tstop
    };
    let mut compressed = None;
    // A run stopped before its first whole chunk kept nothing to publish.
    let (published, written) = if stopped && stored.rows.rows() == 0 {
        (None, Ok(()))
    } else if let Some(compression) = compression.filter(|_| !stopped) {
        let (published, written) =
            publish_compressed_transient(ctx, &mut stored, (tstart, tstop), compression);
        (published, written.map(|outcome| compressed = Some(outcome)))
    } else {
        publish_stored_transient(ctx, &mut stored, tstart, tstop, store)
    };
    let mut keep = !ctx.args.no_store;
    match written {
        Ok(()) => {}
        Err(error) if stopped || error.is_storage_full() => {
            crate::observability::diagnostic(
                "results_store_output_incomplete",
                None,
                format_args!("Warning: the transient's outputs are incomplete: {error}"),
            );
            // The store is all that holds the rows the outputs did not take.
            keep = true;
            if !stopped {
                return Err(super::stored_transient::output_filled(
                    stored.rows,
                    published.as_deref(),
                    &error,
                ));
            }
        }
        Err(error) => {
            let _ = stored.rows.publish(None, false);
            return Err(error);
        }
    }
    // Outputs that streamed left the shell without rows; the `.FOUR` cards read theirs now.
    let fourier = if store == TransientStore::Fourier
        && compressed.is_none()
        && stored.rows.rows() > 0
        && stored.shell.time.is_empty()
    {
        match stored.fourier_window(ctx, tstop) {
            Ok(fourier) => fourier,
            Err(error) => {
                let _ = stored.rows.publish(None, false);
                return Err(error);
            }
        }
    } else {
        None
    };
    // A caller that reads the run gets its event histories with it; one that reads nothing
    // leaves them in the store.
    if !matches!(store, TransientStore::Streamed)
        && let Err(error) = stored.hold_events()
    {
        let _ = stored.rows.publish(None, false);
        return Err(error);
    }
    let super::stored_transient::StoredTransient { shell, rows } = stored;
    super::stored_transient::record_store(rows, published.as_deref(), keep)?;
    Ok(compressed.unwrap_or(TransientOutcome {
        result: shell,
        post_results: None,
        fourier,
    }))
}

/// Write a stored transient's compressed outputs: compress its trajectory, and publish the
/// compressed waveform with the products evaluated on the trajectory. Returns
/// the output it was published to, which a failed write names too, and the published result.
fn publish_compressed_transient(
    ctx: &RunContext<'_>,
    stored: &mut super::stored_transient::StoredTransient,
    (tstart, tstop): (f64, f64),
    compression: &rspice_core::engine::CompressionConfig,
) -> (
    Option<std::path::PathBuf>,
    Result<TransientOutcome, CliError>,
) {
    let compressed = compress_stored(ctx, stored, tstop, compression);
    let compressed = match compressed.and_then(|compressed| {
        report_compression(ctx, &compressed, "Transient complete (compressed)")?;
        Ok(compressed)
    }) {
        Ok(compressed) => compressed,
        Err(error) => return (None, Err(error)),
    };
    let compression_report = compressed.compression_report.clone();
    let post_results = compressed.post_results.clone();
    let result = match expand_compressed(compressed, "transient result") {
        Ok(result) => result,
        Err(error) => return (None, Err(error)),
    };
    let mut output = super::stored_transient::StoredOutput {
        rows: &mut stored.rows,
        plan: None,
        published: None,
    };
    let written = finish_transient_outputs(
        ctx,
        &result,
        tstart,
        tstop,
        Some(&post_results),
        Some(&compression_report),
        Some(&mut output),
    );
    (
        output.published,
        written.map(|()| TransientOutcome {
            result,
            post_results: Some(post_results),
            fourier: None,
        }),
    )
}

/// Compress a stored transient: from its store a point at a time, holding only its time grid,
/// step sizes and retained points, when its post-processing products read the store in passes,
/// and otherwise from the run read back whole.
fn compress_stored(
    ctx: &RunContext<'_>,
    stored: &mut super::stored_transient::StoredTransient,
    tstop: f64,
    compression: &rspice_core::engine::CompressionConfig,
) -> Result<rspice_core::engine::TransientResultCompressed, CliError> {
    let failed = |source| CliError::CoreSimulationError {
        source,
        analysis: Some("Transient".to_string()),
    };
    let abort = &crate::abort::ProcessAbort;
    let post_results = match stored.rows.compression_slots(&stored.shell) {
        Some(slots) => stored.post_results(ctx, tstop)?.map(|post| (slots, post)),
        None => {
            stored.read_back(ctx, tstop)?;
            None
        }
    };
    let Some((slots, post_results)) = post_results else {
        log::debug!("the transient was read back whole to be compressed");
        return ctx
            .engine
            .compress_transient_result_with_abort(ctx.netlist, &stored.shell, compression, abort)
            .map_err(failed);
    };
    log::debug!("the transient is compressed from its store");
    let (time, step_sizes, mut rows) = stored.rows.compression_rows(slots)?;
    let compressed = ctx
        .engine
        .compress_transient_rows_with_abort(
            ctx.netlist,
            &stored.shell,
            rspice_core::engine::AcceptedTransientPoints {
                time: &time,
                step_sizes: &step_sizes,
                rows: &mut rows,
            },
            compression,
            post_results,
            abort,
        )
        .map_err(failed);
    rows.finish(compressed)
}

/// Write a stored transient's outputs. Returns the output it was published to, which a failed
/// write names too, and how writing went.
fn publish_stored_transient(
    ctx: &RunContext<'_>,
    stored: &mut super::stored_transient::StoredTransient,
    tstart: f64,
    tstop: f64,
    store: TransientStore,
) -> (Option<std::path::PathBuf>, Result<(), CliError>) {
    let plan = match store {
        TransientStore::Streamed | TransientStore::Fourier => stored.stream_plan(ctx, tstart),
        TransientStore::ReadBack => Ok(None),
    };
    let plan = match plan {
        Ok(plan) => plan,
        Err(error) => return (None, Err(error)),
    };
    if plan.is_none()
        && let Err(error) = stored.read_back(ctx, tstop)
    {
        return (None, Err(error));
    }
    let mut output = super::stored_transient::StoredOutput {
        rows: &mut stored.rows,
        plan: plan.as_ref(),
        published: None,
    };
    let written = finish_transient_outputs(
        ctx,
        &stored.shell,
        tstart,
        tstop,
        None,
        None,
        Some(&mut output),
    );
    (output.published, written)
}

pub(super) fn finish_transient_result(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::transient::TransientResult,
    tstart: f64,
    tstop: f64,
    post_results: Option<&rspice_core::engine::TransientPostResults>,
    compression_report: Option<&rspice_core::engine::TransientCompressionReport>,
) -> Result<(), CliError> {
    finish_transient_outputs(
        ctx,
        result,
        tstart,
        tstop,
        post_results,
        compression_report,
        None,
    )
}

/// Report, measure and publish one transient. With `stored`, its rows are in a store: a run with
/// a stream plan has `result` without waveforms and reads them from the store; one without has
/// them read back into `result`.
fn finish_transient_outputs(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::transient::TransientResult,
    tstart: f64,
    tstop: f64,
    post_results: Option<&rspice_core::engine::TransientPostResults>,
    compression_report: Option<&rspice_core::engine::TransientCompressionReport>,
    mut stored: Option<&mut super::stored_transient::StoredOutput<'_>>,
) -> Result<(), CliError> {
    if let Some(stored) = stored.as_deref_mut().filter(|stored| stored.plan.is_some()) {
        stored
            .rows
            .ensure_finite(ctx.args.allow_nonfinite, result)?;
    } else {
        finite_transient_columns(ctx, result)?;
    }

    if !ctx.quiet && !ctx.compress {
        let (points, last_time) = match stored.as_deref_mut().filter(|stored| stored.plan.is_some())
        {
            Some(stored) => (stored.rows.rows() as usize, stored.rows.last_time()?),
            None => (result.time.len(), result.time.last().copied()),
        };
        // A model's `$finish` ends a run successfully at the point the solver
        // accepted; the headline must not claim the requested interval.
        match last_time.filter(|last| tstop - last > 1e-9 * tstop.abs()) {
            Some(last) => crate::console::line(format_args!(
                "Transient ended early at {} of the requested {}: {} time points computed",
                crate::console::si_seconds(last),
                crate::console::si_seconds(tstop),
                points
            ))?,
            None => crate::console::line(format_args!(
                "✓ Transient complete: {} time points computed",
                points
            ))?,
        }
    }
    finish_transient_products(
        ctx,
        result,
        tstart,
        tstop,
        post_results,
        compression_report,
        stored,
    )
}

fn finite_transient_columns(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::transient::TransientResult,
) -> Result<(), CliError> {
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
    )
}

fn finish_transient_products(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::transient::TransientResult,
    tstart: f64,
    tstop: f64,
    post_results: Option<&rspice_core::engine::TransientPostResults>,
    compression_report: Option<&rspice_core::engine::TransientCompressionReport>,
    mut stored: Option<&mut super::stored_transient::StoredOutput<'_>>,
) -> Result<(), CliError> {
    // A compressed run already carries the `.MEASURE` results the core
    // evaluated on the exact accepted trajectory. Re-evaluating them
    // here would measure the decimated expansion and report different
    // numbers than the same deck run without `--compress`.
    let plan = stored.as_ref().and_then(|stored| stored.plan);
    let measurements = match (
        post_results,
        plan.and_then(|plan| plan.measurements.as_ref()),
    ) {
        (Some(post), _) => post.measurements.clone(),
        // A streamed run was measured from its store.
        (None, Some(measured)) => measured.clone(),
        (None, None) => rspice_core::analysis::evaluate_tran_measurements_with_abort(
            ctx.netlist,
            result,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| CliError::CoreSimulationError {
            source,
            analysis: Some("Transient measurement projection".to_string()),
        })?,
    };
    ctx.record_measurements("TRAN", measurements)?;
    // `.MEASURE FFT` statistics read the spectra of the run's `.FFT` cards.
    let spectra = match post_results {
        Some(post) => &post.fft,
        None => &result.fft_results,
    };
    let fft_measurements = rspice_core::analysis::evaluate_fft_measurements_with_abort(
        ctx.netlist,
        spectra,
        &crate::abort::ProcessAbort,
    )
    .map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some("FFT measurement projection".to_string()),
    })?;
    ctx.record_measurements("FFT", fft_measurements)?;
    let continuous_measurements = match post_results {
        Some(post) => post.continuous_measurements.clone(),
        None => rspice_core::analysis::evaluate_tran_continuous_measurements_with_limits_and_abort(
            ctx.netlist,
            result,
            &ctx.transient_limits(),
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| CliError::CoreSimulationError {
            source,
            analysis: Some("TRAN continuous measurement projection".to_string()),
        })?,
    };
    super::shared::record_continuous_measurements(ctx, "TRAN_CONT", continuous_measurements)?;
    let assertions = match post_results {
        Some(post) => post.assertions.clone(),
        None => rspice_core::analysis::evaluate_tran_assertions(
            ctx.netlist,
            result,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| CliError::CoreSimulationError {
            source,
            analysis: Some("Transient assertions".to_string()),
        })?,
    };
    ctx.record_assertions(assertions);

    // Perform checked SAVE/PRINT materialization independently of
    // file publication, matching OP and DC behavior.
    let mut signals = match plan {
        Some(plan) => plan.signals.clone(),
        None => transient_export_signals(
            ctx.netlist,
            result,
            ctx.transient_limits(),
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_output_projection_error(ctx, error, "Transient"))?,
    };

    validate_fft_result_count(&result.fft_results, &ctx.netlist.fft_analyses)?;

    // The rows a printed table carries: every accepted step, or the times
    // `.OPTIONS OUTPUT` selects.
    let output_projection = || {
        let output_start = result
            .time
            .first()
            .copied()
            .map_or(tstart, |first| tstart.max(first));
        result
            .output_projection(
                &ctx.netlist.options.output_time_points,
                ctx.netlist.options.output_interval_schedule.as_ref(),
                output_start,
                tstop,
                ctx.transient_limits().max_result_values,
            )
            .map_err(|error| CliError::from(rspice_core::SimulationError::from(error)))
    };
    super::xyce_files::record_print(
        ctx,
        rspice_core::netlist::OutputAnalysisKind::Tran,
        super::xyce_files::Scale::Time,
        || {
            let projection = output_projection()?;
            let mut printed = signals.clone();
            for signal in &mut printed {
                signal.values = projection
                    .project(&signal.values)
                    .map_err(|message| CliError::simulation_error_in(message, "Transient"))?;
            }
            Ok(super::export::scalar_table(
                "transient",
                "Transient Analysis",
                "time",
                "time",
                projection.times().to_vec(),
                &printed,
            ))
        },
    )?;

    if let Some(resolved) = ctx.resolve_output("tran") {
        // The transient publishes here rather than through
        // `publish_analysis_result`, so its output is timed here.
        let _output_timing = crate::timings::guard(
            ctx.args.phase_timings.as_deref(),
            crate::timings::Phase::Output,
        );
        let analysis_id = resolved.analysis("tran")?;
        let output_path = resolved.path;
        if let Some(stored) = stored.as_deref_mut() {
            stored.published = Some(output_path.clone());
        }
        // A streamed run's rows stay in its store: the table is written from
        // there, its finite values already checked with the whole run's.
        let mut streamed_rows = None;
        let output_time = match (plan, stored.as_deref_mut()) {
            (Some(_), Some(stored)) => {
                streamed_rows = Some(stored.rows.output_rows(tstart, tstop)?);
                Vec::new()
            }
            _ => {
                let projection = output_projection()?;
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
                projection.times().to_vec()
            }
        };
        // The run's event histories: in its store, for a streamed run that has not read them
        // back, and otherwise in its traces.
        let run_events = || match stored.as_deref() {
            Some(stored) => stored.rows.events(result),
            None => Ok(RunEvents::Held(rspice_core::execution::HeldEvents {
                digital: &result.digital_traces,
                real: &result.real_traces,
            })),
        };
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
                        signal.unit_symbol(),
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
                let builder = rspice_core::execution::AnalysisResultDocument::from_transient(
                    analysis_id,
                    result,
                    compression_report,
                    children,
                )
                .map_err(|error| super::document::document_error(ctx, analysis_id, error))?;
                TransientOutputDocument::Typed(Box::new(super::document::finish(
                    ctx,
                    analysis_id,
                    builder,
                )?))
            }
            OutputFormat::Vcd => {
                if result.digital_traces.is_empty() && result.real_traces.is_empty() {
                    crate::observability::diagnostic(
                        "empty_event_capture",
                        None,
                        format_args!(
                            "Warning: this transient captured no digital or real event node; \
                         {} declares no signal and records no change",
                            output_path.display()
                        ),
                    );
                }
                TransientOutputDocument::Vcd(Box::new(crate::commands::vcd_io::event_stream(
                    &output_path,
                    run_events()?,
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
                )?))
            }
            OutputFormat::Raw | OutputFormat::RawAscii | OutputFormat::Csv | OutputFormat::Tsv => {
                TransientOutputDocument::Table {
                    table: Box::new(super::export::scalar_table_taking_samples(
                        "transient",
                        "Transient Analysis",
                        "time",
                        "time",
                        output_time,
                        &mut signals,
                    )),
                    events: Box::new(EventPlots::new(&output_path, result, run_events()?)?),
                }
            }
        };
        let publication = if let TransientOutputDocument::Typed(typed) = &document {
            super::document::typed_publication(&output_path, typed)?
        } else {
            super::PublishedResult {
                analysis_id: analysis_id.tag(),
                result_kind: None,
                schema: match &document {
                    TransientOutputDocument::Vcd(events) => {
                        crate::commands::vcd_io::event_schema(&output_path, events.header())?
                    }
                    _ => super::document::scalar_schema(&signals)?,
                },
                artifact: output_path.clone(),
                source_sample_presence: None,
            }
        };

        let fft_output_path = if result.fft_results.is_empty() {
            publish::artifact(&output_path, |writer| {
                match (&document, streamed_rows, plan, stored) {
                    (
                        TransientOutputDocument::Table { table, events },
                        Some((first, points)),
                        Some(plan),
                        Some(stored),
                    ) => {
                        let points =
                            usize::try_from(points).map_err(|_| CliError::InternalError {
                                message: "a transient output has more rows than this host \
                                          can address"
                                    .to_owned(),
                            })?;
                        let column_events = plan
                            .reads_events()
                            .then(|| stored.rows.events(result))
                            .transpose()?;
                        let mut cursor = stored.rows.cursor(
                            plan,
                            first,
                            column_events
                                .as_ref()
                                .map(|events| events as &dyn rspice_core::execution::EventSource),
                        );
                        table.write_streamed_to(
                            writer,
                            &output_path,
                            ctx.format,
                            points,
                            |row, variable| cursor.value(row, variable),
                        )?;
                        cursor.finish(&output_path)?;
                        write_raw_event_plots(writer, &output_path, ctx.format, events)
                    }
                    (
                        TransientOutputDocument::Hdf5(data),
                        Some((first, points)),
                        Some(plan),
                        Some(stored),
                    ) => {
                        let points =
                            usize::try_from(points).map_err(|_| CliError::InternalError {
                                message: "a transient output has more rows than this host \
                                          can address"
                                    .to_owned(),
                            })?;
                        // Each dataset is one column of the store, written as it is read, in
                        // the order the document refuses a non-finite sample in.
                        let names = std::iter::once("time")
                            .chain(signals.iter().map(|signal| signal.display_name.as_str()))
                            .collect::<Vec<_>>();
                        let column_events = plan
                            .reads_events()
                            .then(|| stored.rows.events(result))
                            .transpose()?;
                        let mut cursor = stored.rows.cursor(
                            plan,
                            first,
                            column_events
                                .as_ref()
                                .map(|events| events as &dyn rspice_core::execution::EventSource),
                        );
                        crate::hdf5::write_streamed_transient_to_writer(
                            writer,
                            data,
                            points,
                            |variable, out| {
                                cursor
                                    .write_column(variable, points, out, &output_path)
                                    .map_err(|stop| match stop {
                                        super::stored_transient::ColumnStop::NonFinite(row) => {
                                            map_hdf5_output_error(
                                                &output_path,
                                                crate::hdf5::Hdf5Error::InvalidSchema(format!(
                                                    "non-finite value in '{}' at sample {row}",
                                                    names[variable]
                                                )),
                                            )
                                        }
                                        super::stored_transient::ColumnStop::Failed(error) => error,
                                    })
                            },
                        )
                        .map_err(|failure| match failure {
                            crate::hdf5::Hdf5StreamFailure::Hdf5(error) => {
                                map_hdf5_output_error(&output_path, error)
                            }
                            crate::hdf5::Hdf5StreamFailure::Source(error) => error,
                        })?;
                        cursor.finish(&output_path)
                    }
                    _ => document.write_to(
                        writer,
                        &output_path,
                        ctx.format,
                        super::document::json_byte_limit(ctx),
                        ctx.args.timeout,
                    ),
                }
            })
            .map_err(|error| map_atomic_output_error(&output_path, error))?;
            None
        } else {
            let parent_analysis_id = ctx.current_transient_analysis_id()?;
            let fft_output_path =
                ctx.fft_output_path_for(&parent_analysis_id)
                    .ok_or_else(|| CliError::InternalError {
                        message: "FFT publication was requested without a resolved output path"
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
                crate::console::line(format_args!(
                    "  FFT results exported to: {}",
                    fft_output_path.display()
                ))?;
            }
            Some(fft_output_path)
        };
        ctx.record_published(publication);
        if let Some(path) = fft_output_path {
            ctx.record_output(path);
        }

        if !ctx.quiet {
            crate::console::line(format_args!(
                "  Results exported to: {}",
                output_path.display()
            ))?;
        }
    }
    Ok(())
}

/// Append a rawfile's event plots after its table, inside the caller's staging closure, so a
/// rawfile that carries them is still published whole or not at all. Other formats carry none.
fn write_raw_event_plots(
    writer: &mut dyn std::io::Write,
    path: &Path,
    format: OutputFormat,
    plots: &EventPlots<'_>,
) -> Result<(), CliError> {
    let raw_format = match format {
        OutputFormat::Raw => rspice_core::io::RawFormat::Binary,
        OutputFormat::RawAscii => rspice_core::io::RawFormat::Ascii,
        OutputFormat::Csv
        | OutputFormat::Tsv
        | OutputFormat::Json
        | OutputFormat::Hdf5
        | OutputFormat::Vcd => return Ok(()),
    };
    rspice_core::execution::write_transient_event_plots(
        writer,
        plots.digital,
        plots.real,
        plots.buses,
        &plots.events,
        raw_format,
    )
    .map_err(|error| match error {
        rspice_core::execution::EventPlotWriteError::Io(error) => {
            CliError::output_error(path, error)
        }
        rspice_core::execution::EventPlotWriteError::Plot(error) => {
            crate::commands::waveform_io::conversion_error(path, error)
        }
    })
}

pub(super) enum TransientOutputDocument<'r> {
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
        table: Box<super::export::ExportTable>,
        /// The run's event histories, read as their plots are written.
        events: Box<EventPlots<'r>>,
    },
    /// The shared typed result document, published for `-f json`.
    Typed(Box<rspice_core::execution::AnalysisResultDocument>),
    /// The event timelines as a Value Change Dump, published for `-f vcd`.
    ///
    /// This is the one transient artifact that carries no analog waveform:
    /// the analysis grid is what the other formats are for, and an event dump
    /// is for the times the events actually happened. Its changes are read from the run's
    /// traces as it is written.
    Vcd(Box<rspice_core::execution::EventVcdStream<RunEvents<'r>>>),
}

/// A transient's event histories as a rawfile's event plots: a plot per node with events, then a
/// plot per declared bus. The member plots stay the authoritative copy; a bus plot carries the
/// grouping and the bit order, which nothing else in a rawfile does.
pub(super) struct EventPlots<'r> {
    digital: &'r [rspice_core::engine::DigitalTrace],
    real: &'r [rspice_core::engine::RealTrace],
    buses: &'r [rspice_core::engine::DigitalBusDeclaration],
    events: RunEvents<'r>,
}

impl<'r> EventPlots<'r> {
    /// The plots of `result`'s histories, read from `events`, refusing the bus the projected
    /// plots refuse, with its numbers, before a byte is written.
    pub(super) fn new(
        path: &Path,
        result: &'r rspice_core::analysis::transient::TransientResult,
        events: RunEvents<'r>,
    ) -> Result<Self, CliError> {
        for bus in &result.digital_buses {
            rspice_core::execution::transient_bus_plot(&result.digital_traces, bus, &events)
                .map_err(|error| crate::commands::waveform_io::conversion_error(path, error))?;
        }
        Ok(Self {
            digital: &result.digital_traces,
            real: &result.real_traces,
            buses: &result.digital_buses,
            events,
        })
    }

    /// No event plots.
    #[cfg(test)]
    pub(super) fn none() -> Self {
        Self {
            digital: &[],
            real: &[],
            buses: &[],
            events: RunEvents::Held(rspice_core::execution::HeldEvents {
                digital: &[],
                real: &[],
            }),
        }
    }
}

impl TransientOutputDocument<'_> {
    pub(super) fn write_to(
        &self,
        writer: &mut dyn std::io::Write,
        path: &Path,
        format: OutputFormat,
        byte_limit: u64,
        timeout_seconds: Option<f64>,
    ) -> Result<(), CliError> {
        match self {
            Self::Hdf5(data) => write_hdf5_to_writer(writer, data)
                .map_err(|error| map_hdf5_output_error(path, error)),
            Self::Table { table, events } => {
                table.write_to(writer, path, format)?;
                write_raw_event_plots(writer, path, format, events)
            }
            Self::Vcd(events) => crate::commands::vcd_io::write_event_stream(writer, path, events),
            Self::Typed(document) => {
                let json = document
                    .to_json_with_abort(&crate::abort::ProcessAbort, byte_limit)
                    .map_err(|error| {
                        super::document::document_error_with_timeout(
                            document.analysis(),
                            error,
                            timeout_seconds,
                        )
                    })?;
                writer
                    .write_all(json.as_bytes())
                    .and_then(|()| writer.write_all(b"\n"))
                    .map_err(|error| CliError::output_error(path, error))
            }
        }
    }
}
