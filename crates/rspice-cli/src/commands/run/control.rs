//! CLI host for the shared ordered control interpreter and result exporters.

use super::*;
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
};
use rspice_core::execution::control::{
    ControlError, ControlErrorKind, ControlLimits, ControlProgram,
};
use rspice_core::netlist::ControlScriptSource;

mod output;

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
    if args.checkpoint.is_some()
        || args.resume.is_some()
        || args.tran_stop.is_some()
        || args.compress
        || config.simulation.compress_waveforms
        || netlist.options.restart.is_some()
    {
        return Err(CliError::InvalidArgument {
            message: "control-script checkpoint, segmented-restart and compression execution is not yet implemented".into(),
            suggestion: None,
        });
    }
    if !netlist.fft_analyses.is_empty()
        || netlist
            .analyses
            .iter()
            .any(|analysis| matches!(analysis, AnalysisCommand::Four { .. }))
    {
        return Err(CliError::InvalidArgument {
            message: "declarative Fourier/FFT post-processing inside a control-script run is not yet implemented".into(),
            suggestion: None,
        });
    }
    let limits = config.resources.limits();
    let program = ControlProgram::parse_deck_with_abort(
        script.text(),
        ControlLimits {
            max_source_bytes: limits.max_expanded_source_bytes,
            max_source_lines: limits.max_netlist_lines,
            max_loop_values: limits.max_batch_runs,
            ..ControlLimits::default()
        },
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| map_command(error, script, args))?;
    let engine = Engine::try_new(build_sim_config(args, config, netlist))?;
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

    let execution = (|| -> Result<(), CliError> {
        while let Some(command) = session
            .next_command(&mut circuit, &crate::abort::ProcessAbort)
            .map_err(|error| map_command(error, script, args))?
        {
            let previous = circuit.datasets().len();
            let effect = circuit
                .execute(
                    &engine,
                    &command,
                    session.variables(),
                    &crate::abort::ProcessAbort,
                )
                .map_err(|error| map_execution(error, script, args))?;
            match effect {
                ControlCommandEffect::CircuitChanged => {}
                ControlCommandEffect::Analyses(_) => {
                    for dataset in &circuit.datasets()[previous..] {
                        let mut snapshot = circuit.netlist().clone();
                        snapshot.analyses = vec![dataset.command.clone()];
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
                            println!("Control dataset {} ({})", dataset.name, dataset.analysis_id);
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
                            ControlAnalysisResult::Ac(result) => {
                                frequency::finish_ac_results(&ctx, result)?
                            }
                            ControlAnalysisResult::Transient(result) => {
                                let AnalysisCommand::Tran { start, stop, .. } = dataset.command
                                else {
                                    return Err(CliError::InternalError {
                                        message: "transient dataset has another command kind"
                                            .into(),
                                    });
                                };
                                basic::finish_transient_result(
                                    &ctx,
                                    result,
                                    start.unwrap_or(0.0),
                                    stop,
                                    None,
                                    None,
                                )?;
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
        // A measurement attached to a skipped analysis must remain a failed
        // measurement. Use the same finalizer as ordinary deck execution.
        let ctx = RunContext::new(
            &engine, netlist, args, config, verbose, quiet, run_label, identity,
        )?;
        *ctx.evaluated_meas.borrow_mut() = evaluated;
        ctx.record_unevaluated_measurements();
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
