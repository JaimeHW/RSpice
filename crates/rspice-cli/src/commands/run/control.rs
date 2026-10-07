//! CLI host for the shared ordered control interpreter and result exporters.

use super::*;
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
};
use rspice_core::execution::control::{ControlError, ControlErrorKind};
use rspice_core::netlist::ControlScriptSource;

mod output;
mod transient;

pub(super) fn run(
    netlist: &Netlist,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
    run_label: Option<&str>,
    identity: RunIdentity<'_>,
) -> Result<ConcreteDeckOutcome, CliError> {
    let script = netlist
        .control_script
        .as_deref()
        .ok_or_else(|| CliError::InternalError {
            message: "control execution requires retained script source".into(),
        })?;
    let limits = config.resources.limits();
    let program = crate::commands::preflight::control_program(script, &args.input, limits)
        .map_err(|error| {
            if matches!(error, CliError::Interrupted) {
                cancellation_cli_error(args.timeout)
            } else {
                error
            }
        })?;
    let engine = build_observed_engine(args, config, netlist, quiet)?;
    let mut circuit =
        ControlCircuit::new(netlist.clone()).map_err(|error| map_execution(error, script, args))?;
    let mut session = program.start(netlist.params.clone());
    let transaction = if publish::current().is_none() {
        Some(publish::begin()?)
    } else {
        None
    };
    let started = Instant::now();
    let mut outputs = Vec::new();
    let mut published = Vec::new();
    let mut measurements = Vec::new();
    let mut evaluated = std::collections::HashSet::new();
    let mut presentation_ordinal = 0usize;
    let mut post_ordinals = std::collections::BTreeMap::new();

    let execution = (|| -> Result<(), CliError> {
        while let Some(command) = session
            .next_command(&mut circuit, &crate::abort::ProcessAbort)
            .map_err(|error| map_command(error, script, args))?
        {
            let previous = circuit.datasets().len();
            let effect = circuit
                .execute_with_transient_runner(
                    &engine,
                    &command,
                    session.variables(),
                    &crate::abort::ProcessAbort,
                    &mut |engine, snapshot, analysis, analysis_id, _| -> Result<_, HostError> {
                        let completed = transient::run(
                            engine,
                            snapshot,
                            analysis,
                            analysis_id,
                            &mut post_ordinals,
                            args,
                            config,
                            verbose,
                            quiet,
                            run_label,
                            &identity,
                        )?;
                        outputs.extend(completed.outputs);
                        published.extend(completed.published);
                        measurements.extend(completed.measurements);
                        evaluated.extend(completed.evaluated);
                        Ok(completed.result)
                    },
                )
                .map_err(|error| match error {
                    HostError::Core(error) => map_execution(error, script, args),
                    HostError::Output(error) => error,
                })?;
            match effect {
                ControlCommandEffect::CircuitChanged => {}
                ControlCommandEffect::Analyses(_) => {
                    for dataset in &circuit.datasets()[previous..] {
                        // The transient callback already published the exact trajectory
                        // it returned to the shared script dataset store.
                        if matches!(dataset.result, ControlAnalysisResult::Transient(_)) {
                            continue;
                        }
                        let mut snapshot = circuit.netlist().clone();
                        snapshot.analyses = vec![dataset.command.clone()];
                        snapshot.fft_analyses.clear();
                        let mut ctx = RunContext::new(
                            &engine,
                            &snapshot,
                            args,
                            config,
                            verbose,
                            quiet,
                            run_label,
                            RunIdentity {
                                coordinate: identity.coordinate,
                                topology: identity.topology,
                                analyses: PlannedAnalysisIdentities::from_pairs(
                                    [(&dataset.command, dataset.analysis_id)],
                                    &[],
                                ),
                            },
                        )?;
                        ctx.qualify_control_outputs();
                        if !quiet {
                            crate::console::line(format_args!(
                                "Control dataset {} ({})",
                                dataset.name, dataset.analysis_id
                            ))?;
                        }
                        match &dataset.result {
                            ControlAnalysisResult::OperatingPoint(result) => {
                                let report =
                                    dataset.device_op_report.as_deref().ok_or_else(|| {
                                        CliError::InternalError {
                                            message: "control OP dataset lost its device report"
                                                .into(),
                                        }
                                    })?;
                                basic::finish_dc_op_result(&ctx, result, report)?;
                            }
                            ControlAnalysisResult::DcSweep(result) => {
                                let AnalysisCommand::Dc { source, .. } = &dataset.command else {
                                    return Err(CliError::InternalError {
                                        message: "control DC dataset lost its authored command"
                                            .into(),
                                    });
                                };
                                basic::finish_dc_sweep_result(&ctx, source, result)?;
                            }
                            ControlAnalysisResult::Noise(result) => {
                                let AnalysisCommand::Noise {
                                    output_node,
                                    reference_node,
                                    input_source,
                                    ..
                                } = &dataset.command
                                else {
                                    return Err(CliError::InternalError {
                                        message: "control noise dataset lost its authored command"
                                            .into(),
                                    });
                                };
                                frequency::finish_noise_results(
                                    &ctx,
                                    output_node,
                                    reference_node.as_deref(),
                                    input_source,
                                    result,
                                    true,
                                )?;
                            }
                            ControlAnalysisResult::Ac(result) => {
                                frequency::finish_ac_results(&ctx, result)?
                            }
                            ControlAnalysisResult::TransferFunction(result) => {
                                frequency::finish_tf_result(&ctx, result)?;
                            }
                            ControlAnalysisResult::AcTable(table) => {
                                frequency::finish_ac_table(&ctx, table)?;
                            }
                            ControlAnalysisResult::NoiseTable(table) => {
                                let AnalysisCommand::NoiseData {
                                    output_node,
                                    reference_node,
                                    input_source,
                                    ..
                                } = &dataset.command
                                else {
                                    return Err(CliError::InternalError {
                                        message: "control noise table lost its authored command"
                                            .into(),
                                    });
                                };
                                frequency::finish_noise_table(
                                    &ctx,
                                    output_node,
                                    reference_node.as_deref(),
                                    input_source,
                                    table,
                                )?;
                            }
                            ControlAnalysisResult::Transient(_) => {
                                unreachable!("published by the transient host")
                            }
                        }
                        measurements.extend(ctx.measurements.into_inner());
                        evaluated.extend(ctx.evaluated_meas.into_inner());
                        outputs.extend(ctx.outputs.into_inner());
                        published.extend(ctx.published.into_inner());
                    }
                }
                ControlCommandEffect::Presentation(request) => {
                    presentation_ordinal += 1;
                    outputs.extend(output::present(
                        &request,
                        &circuit,
                        args,
                        config,
                        run_label,
                        presentation_ordinal,
                        quiet,
                    )?);
                }
            }
        }
        let ran_transient = circuit
            .datasets()
            .iter()
            .any(|dataset| matches!(dataset.result, ControlAnalysisResult::Transient(_)));
        if !ran_transient
            && (args.checkpoint.is_some() || args.resume.is_some() || args.tran_stop.is_some())
        {
            return Err(CliError::InvalidArgument {
                message: "--checkpoint, --resume and --tran-stop require the control script to execute a transient".into(),
                suggestion: None,
            });
        }
        if !ran_transient
            && (!netlist.fft_analyses.is_empty()
                || netlist
                    .analyses
                    .iter()
                    .any(|card| matches!(card, AnalysisCommand::Four { .. })))
        {
            return Err(CliError::InvalidArgument {
                message: "declarative Fourier/FFT processing requires the control script to execute a transient".into(),
                suggestion: None,
            });
        }
        // A measurement attached to a skipped analysis must remain a failed
        // measurement. Use the same finalizer as ordinary deck execution.
        let mut report_netlist = netlist.clone();
        report_netlist.fft_analyses.clear();
        let ctx = RunContext::new(
            &engine,
            &report_netlist,
            args,
            config,
            verbose,
            quiet,
            run_label,
            RunIdentity {
                coordinate: identity.coordinate,
                topology: identity.topology,
                analyses: PlannedAnalysisIdentities::default(),
            },
        )?;
        *ctx.evaluated_meas.borrow_mut() = evaluated;
        ctx.record_unevaluated_measurements()?;
        measurements.extend(ctx.measurements.into_inner());
        Ok(())
    })();
    let mut error = None;
    let mut error_details = None;
    match execution {
        Ok(()) => {
            if let Some(transaction) = transaction {
                transaction.commit()?;
            }
        }
        Err(failure) => {
            error_details = Some(failure.details());
            error = Some(simulation_error_message(&failure));
            // The transaction drops every staged dataset/plot, so the report
            // cannot advertise files from a partially executed control script.
            drop(transaction);
            outputs.clear();
            published.clear();
        }
    }
    let base = args
        .input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| *stem != "-")
        .unwrap_or("stdin");
    let name = run_label.map_or_else(|| base.to_string(), |label| format!("{base} [{label}]"));
    Ok(ConcreteDeckOutcome {
        report: SimulationReport {
            name,
            netlist: args.input.display().to_string(),
            passed: error.is_none() && measurements.iter().all(|measurement| measurement.passed),
            duration_secs: started.elapsed().as_secs_f64(),
            error,
            error_details,
            measurements,
        },
        outputs,
        published,
    })
}

fn map_command(error: ControlError, script: &ControlScriptSource, args: &RunArgs) -> CliError {
    if error.kind == ControlErrorKind::Aborted {
        return cancellation_cli_error(args.timeout);
    }
    CliError::ControlScriptError {
        origin: script.origin(error.line).cloned().unwrap_or_else(|| {
            rspice_core::netlist::NetlistSourceLocation::in_file(&args.input, error.line)
        }),
        source: error,
    }
}

fn map_execution(
    error: ControlExecutionError,
    script: &ControlScriptSource,
    args: &RunArgs,
) -> CliError {
    match error {
        ControlExecutionError::Command(error) => map_command(error, script, args),
        ControlExecutionError::Configuration { source, .. } => source.into(),
        ControlExecutionError::Simulation {
            source: rspice_core::SimulationError::Aborted,
            ..
        } => cancellation_cli_error(args.timeout),
        ControlExecutionError::Simulation { line, source } => CliError::CoreSimulationError {
            source,
            analysis: Some(
                script
                    .origin(line)
                    .map_or_else(|| format!("control line {line}"), ToString::to_string),
            ),
        },
    }
}

// Keep publication errors in their CLI category while the generic core host
// supplies source-located command and simulation failures unchanged.
enum HostError {
    Core(ControlExecutionError),
    Output(CliError),
}
impl From<ControlExecutionError> for HostError {
    fn from(error: ControlExecutionError) -> Self {
        Self::Core(error)
    }
}
impl From<ControlError> for HostError {
    fn from(error: ControlError) -> Self {
        Self::Core(error.into())
    }
}
impl From<CliError> for HostError {
    fn from(error: CliError) -> Self {
        Self::Output(error)
    }
}
