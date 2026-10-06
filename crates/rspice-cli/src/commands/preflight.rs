//! Bounded, non-executing request validation shared by `check` and `run`.

use crate::cli::CliError;
use rspice_core::execution::control::{
    ControlError, ControlErrorKind, ControlLimits, ControlProgram,
};
use rspice_core::netlist::{AnalysisCommand, ControlScriptSource, DcSweepSpec, Netlist};
use rspice_core::{ResourceKind, ResourceLimitError, ResourceLimits};
use std::path::Path;

pub(crate) fn control_program(
    script: &ControlScriptSource,
    input: &Path,
    limits: ResourceLimits,
) -> Result<ControlProgram, CliError> {
    ControlProgram::parse_deck_with_abort(
        script.text(),
        ControlLimits {
            max_source_bytes: limits.max_expanded_source_bytes,
            max_source_lines: limits.max_netlist_lines,
            max_loop_values: limits.max_batch_runs,
            ..ControlLimits::default()
        },
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| control_error(error, script, input))
}

pub(crate) fn control_error(
    error: ControlError,
    script: &ControlScriptSource,
    input: &Path,
) -> CliError {
    if error.kind == ControlErrorKind::Aborted {
        return CliError::Interrupted;
    }
    CliError::ControlScriptError {
        origin: script.origin(error.line).cloned().unwrap_or_else(|| {
            rspice_core::netlist::NetlistSourceLocation::in_file(input, error.line)
        }),
        source: error,
    }
}

fn invalid(message: impl std::fmt::Display) -> CliError {
    CliError::InvalidArgument {
        message: message.to_string(),
        suggestion: None,
    }
}

fn points_limit(requested: usize, limit: usize) -> CliError {
    rspice_core::SimulationError::ResourceLimit(ResourceLimitError {
        resource: ResourceKind::AnalysisPoints,
        requested,
        limit,
    })
    .into()
}

fn frequency(
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start: f64,
    stop: f64,
    limits: ResourceLimits,
) -> Result<(), CliError> {
    use rspice_core::analysis::FrequencyGridError;
    rspice_core::analysis::ac::try_ac_sweep_point_count_bounded_with_abort(
        variation,
        points,
        start,
        stop,
        limits.max_analysis_points,
        &crate::abort::ProcessAbort,
    )
    .map(|_| ())
    .map_err(|error| match error {
        FrequencyGridError::Aborted => CliError::Interrupted,
        FrequencyGridError::LimitExceeded { requested, limit } => points_limit(requested, limit),
        error => invalid(error),
    })
}

fn dc_points(spec: DcSweepSpec, limits: ResourceLimits) -> Result<usize, CliError> {
    use rspice_core::netlist::SweepPointGenerationError;
    let values = spec
        .points_bounded_with_abort(limits.max_analysis_points, &crate::abort::ProcessAbort)
        .map_err(|error| match error {
            SweepPointGenerationError::Aborted => CliError::Interrupted,
            SweepPointGenerationError::LimitExceeded { requested, limit } => {
                points_limit(requested, limit)
            }
            error => invalid(error),
        })?;
    if values.is_empty() {
        return Err(invalid("DC sweep must produce at least one finite point"));
    }
    Ok(values.len())
}

pub(crate) fn netlist(
    netlist: &Netlist,
    input: &Path,
    limits: ResourceLimits,
    transient_stop: Option<f64>,
) -> Result<(), CliError> {
    if let Some(script) = netlist.control_script.as_deref() {
        control_program(script, input, limits)?;
        // Declarative cards may be executed by `run`, but a script is also
        // allowed to replace them. Validate the program without executing it.
        return Ok(());
    }
    for (index, analysis) in netlist.analyses.iter().enumerate() {
        let result = analysis_request(netlist, analysis, limits, transient_stop);
        result.map_err(|error| {
            CliError::reported(
                format!("analysis {}: {error}", index + 1),
                Some(error.details()),
            )
        })?;
    }
    Ok(())
}

fn analysis_request(
    netlist: &Netlist,
    analysis: &AnalysisCommand,
    limits: ResourceLimits,
    transient_stop: Option<f64>,
) -> Result<(), CliError> {
    match analysis {
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            ..
        } => rspice_core::execution::resolve_transient_maximum_step(
            *step,
            transient_stop.unwrap_or(*stop),
            *start,
            *max_step,
        )
        .map(|_| ())
        .map_err(invalid),
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        }
        | AnalysisCommand::Noise {
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        }
        | AnalysisCommand::Sp {
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        }
        | AnalysisCommand::Stb {
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        }
        | AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        } => frequency(*variation, *points, *start_freq, *stop_freq, limits),
        AnalysisCommand::Sensitivity {
            ac_sweep: Some(sweep),
            ..
        } => frequency(
            sweep.variation,
            sweep.points,
            sweep.start_freq,
            sweep.stop_freq,
            limits,
        ),
        AnalysisCommand::Dc {
            start,
            stop,
            step,
            mode,
            sweep2,
            ..
        } => {
            let inner = dc_points(
                DcSweepSpec {
                    start: *start,
                    stop: *stop,
                    step: *step,
                    mode: mode.clone(),
                },
                limits,
            )?;
            let outer = sweep2
                .as_ref()
                .map(|sweep| dc_points(sweep.spec(), limits))
                .transpose()?
                .unwrap_or(1);
            let count = inner.saturating_mul(outer);
            if count > limits.max_analysis_points {
                Err(points_limit(count, limits.max_analysis_points))
            } else {
                Ok(())
            }
        }
        AnalysisCommand::Pss(card) => rspice_core::analysis::PssConfig::from(card.as_ref())
            .validate()
            .map_err(invalid),
        AnalysisCommand::Pac(card) => rspice_core::analysis::PacConfig::from(card.as_ref())
            .validate()
            .map_err(invalid),
        AnalysisCommand::Pxf(card) => rspice_core::analysis::PacConfig::from(card.as_ref())
            .validate()
            .map_err(invalid),
        AnalysisCommand::Hb(card) => {
            rspice_core::analysis::HbConfig::from_hb_card(card, &netlist.options)
                .map(|_| ())
                .map_err(invalid)
        }
        AnalysisCommand::Qpss(card) => rspice_core::engine::QpssConfig::from_qpss_card(card)
            .map(|_| ())
            .map_err(CliError::from),
        // Remaining typed cards validate their scalar contracts while parsing.
        _ => Ok(()),
    }
}

/// Static checking visits all authored commands, including conditional bodies.
/// Substituted arguments are checked during execution after their values exist.
pub(crate) fn check_requests(
    netlist: &Netlist,
    input: &Path,
    limits: ResourceLimits,
) -> Result<Vec<usize>, CliError> {
    use rspice_core::engine::{ControlCircuit, ControlExecutionError};
    let Some(script) = netlist.control_script.as_deref() else {
        return self::netlist(netlist, input, limits, None).map(|()| Vec::new());
    };
    let program = control_program(script, input, limits)?;
    let mut deferred = Vec::new();
    for command in program.authored_commands() {
        ControlCircuit::validate_command_name(&command)
            .map_err(|error| control_error(error, script, input))?;
        let located = |error: CliError| {
            let location = script.origin(command.line).cloned().unwrap_or_else(|| {
                rspice_core::netlist::NetlistSourceLocation::in_file(input, command.line)
            });
            let mut details = error.details();
            details.line = Some(location.line);
            details.path = location
                .path
                .as_ref()
                .map(|path| path.display().to_string());
            CliError::reported(format!("{location}: {error}"), Some(details))
        };
        if command.name == "run" {
            if command.arguments.contains('$') {
                deferred.push(command.line);
            } else if !command.arguments.is_empty() {
                return Err(located(invalid("run does not accept arguments")));
            }
            let mut count = 0;
            for analysis in &netlist.analyses {
                if matches!(
                    analysis,
                    AnalysisCommand::Four { .. }
                        | AnalysisCommand::Step(_)
                        | AnalysisCommand::Temp { .. }
                ) {
                    continue;
                }
                if !matches!(
                    analysis,
                    AnalysisCommand::Op
                        | AnalysisCommand::Dc { .. }
                        | AnalysisCommand::Ac { .. }
                        | AnalysisCommand::Tran { .. }
                ) {
                    return Err(located(invalid(
                        "this analysis has no control-host execution handler",
                    )));
                }
                analysis_request(netlist, analysis, limits, None).map_err(located)?;
                count += 1;
            }
            if count == 0 {
                return Err(located(invalid(
                    "run has no declarative analysis to execute",
                )));
            }
        } else if matches!(command.name.as_str(), "op" | "dc" | "ac" | "tran") {
            if command.arguments.contains('$') {
                deferred.push(command.line);
                continue;
            }
            let analysis = ControlCircuit::parse_analysis_command(
                &command,
                limits,
                &crate::abort::ProcessAbort,
            )
            .map_err(|error| match error {
                ControlExecutionError::Command(error) => control_error(error, script, input),
                ControlExecutionError::Simulation { source, .. } => located(source.into()),
                ControlExecutionError::Configuration { source, .. } => located(source.into()),
            })?;
            analysis_request(netlist, &analysis, limits, None).map_err(located)?;
        }
    }
    Ok(deferred)
}
