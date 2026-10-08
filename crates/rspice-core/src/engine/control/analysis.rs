//! Non-executing command admission shared by static checks and execution.
use super::*;
use crate::ResourceLimits;
use crate::netlist::{NetlistParseOptions, ParseError, ParseWithAbortError};

impl ControlCircuit {
    /// Check host availability without executing a command or its arguments.
    pub fn validate_command_name(command: &ControlCommand) -> Result<(), ControlError> {
        CommandKind::parse(command).map(|_| ())
    }

    /// Whether this command names an explicit electrical analysis. This does
    /// not parse its arguments or classify the orchestration command `run`.
    pub fn is_analysis_command(command: &ControlCommand) -> bool {
        matches!(CommandKind::parse(command), Ok(CommandKind::Analysis))
    }

    /// Check that this analysis has a control execution handler, without
    /// validating its parameters, circuit capabilities or resource needs.
    pub fn validate_analysis_support(
        analysis: &AnalysisCommand,
        line: usize,
    ) -> Result<(), ControlError> {
        identity(analysis, line).map(|_| ())
    }

    /// Parse a supported analysis request using the execution grammar and limits.
    /// Arguments must already be literal or substituted. No analysis is run.
    pub fn parse_analysis_command(
        command: &ControlCommand,
        resource_limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisCommand, ControlExecutionError> {
        let line = command.line;
        if !matches!(CommandKind::parse(command)?, CommandKind::Analysis) {
            return Err(
                command_error(line, "expected a supported electrical analysis command").into(),
            );
        }
        if command.arguments.contains(['\r', '\n']) {
            return Err(command_error(
                line,
                "a control command cannot contain another physical line",
            )
            .into());
        }
        if command.name == "op" && !command.arguments.trim().is_empty() {
            return Err(command_error(line, "op does not accept arguments").into());
        }
        let source = format!(
            "control analysis\n.{} {}\n.end\n",
            command.name, command.arguments
        );
        let parsed = Netlist::parse_with_options_and_abort(
            &source,
            NetlistParseOptions {
                resource_limits,
                ..NetlistParseOptions::default()
            },
            abort,
        )
        .map_err(|error| match error {
            ParseWithAbortError::Aborted => simulation_error(line, SimulationError::Aborted),
            ParseWithAbortError::Parse(ParseError::ResourceLimit(error)) => {
                simulation_error(line, error.into())
            }
            error => ControlError::new(line, ControlErrorKind::Syntax, error.to_string()).into(),
        })?;
        let mut analyses = parsed.analyses.into_iter();
        let analysis = analyses
            .next()
            .ok_or_else(|| command_error(line, "analysis command did not produce an analysis"))?;
        if analyses.next().is_some() {
            return Err(
                command_error(line, "analysis command produced more than one analysis").into(),
            );
        }
        Ok(analysis)
    }
}

pub(super) fn identity(
    analysis: &AnalysisCommand,
    line: usize,
) -> Result<(&'static str, crate::identity::AnalysisKind), ControlError> {
    Ok(match analysis {
        AnalysisCommand::Op => ("op", crate::identity::AnalysisKind::Op),
        AnalysisCommand::Dc { .. } => ("dc", crate::identity::AnalysisKind::Dc),
        AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => {
            ("ac", crate::identity::AnalysisKind::Ac)
        }
        AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => {
            ("noise", crate::identity::AnalysisKind::Noise)
        }
        AnalysisCommand::Tran { .. } => ("tran", crate::identity::AnalysisKind::Tran),
        AnalysisCommand::Tf { .. } => ("tf", crate::identity::AnalysisKind::TransferFunction),
        AnalysisCommand::PoleZero { .. } => ("pz", crate::identity::AnalysisKind::PoleZero),
        AnalysisCommand::Sensitivity { .. } => ("sens", crate::identity::AnalysisKind::Sensitivity),
        AnalysisCommand::Disto { .. } => ("disto", crate::identity::AnalysisKind::Distortion),
        _ => {
            return Err(command_error(
                line,
                "this analysis has no control-host execution handler",
            ));
        }
    })
}

// Preserve resource failures and cooperative cancellation during grid construction.
pub(super) fn frequency_error(
    line: usize,
    error: crate::analysis::FrequencyGridError,
) -> ControlExecutionError {
    use crate::analysis::FrequencyGridError;
    match error {
        error @ (FrequencyGridError::Aborted
        | FrequencyGridError::LimitExceeded { .. }
        | FrequencyGridError::Allocation { .. }) => simulation_error(line, error.into()),
        other => command_error(line, other.to_string()).into(),
    }
}

/// Keep frequency setup/retention outside the generic control dispatch frame.
/// These result families use the same bounded, cancellable physical grid.
pub(super) fn frequency(
    engine: &Engine,
    netlist: &Netlist,
    command: &AnalysisCommand,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<(ControlAnalysisResult, usize), ControlExecutionError> {
    let (variation, points, start_freq, stop_freq) = match command {
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
        | AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        } => (*variation, *points, *start_freq, *stop_freq),
        _ => {
            return Err(
                command_error(line, "expected AC, NOISE or DISTO frequency analysis").into(),
            );
        }
    };
    let frequencies = crate::analysis::ac::try_ac_sweep_frequencies_bounded_with_abort(
        variation,
        points,
        start_freq,
        stop_freq,
        engine.config().resource_limits.max_analysis_points,
        abort,
    )
    .map_err(|error| frequency_error(line, error))?;
    if let AnalysisCommand::Disto { f2_over_f1, .. } = command {
        let result = engine
            .run_distortion_with_abort(netlist, &frequencies, *f2_over_f1, abort)
            .map_err(|error| simulation_error(line, error))?;
        let count = result.retained_value_count();
        return Ok((ControlAnalysisResult::Distortion(Box::new(result)), count));
    }
    if let AnalysisCommand::Noise {
        output_node,
        reference_node,
        input_source,
        ..
    } = command
    {
        let result = engine
            .run_noise_named_with_input_source_and_abort(
                netlist,
                output_node,
                reference_node.as_deref(),
                input_source,
                &frequencies,
                engine.config().temperature,
                abort,
            )
            .map_err(|error| simulation_error(line, error))?;
        let count = result
            .iter()
            .fold(0usize, |n, p| n.saturating_add(p.retained_value_count()));
        Ok((ControlAnalysisResult::Noise(result), count))
    } else {
        let result = engine
            .run_ac_with_abort(netlist, &frequencies, abort)
            .map_err(|error| simulation_error(line, error))?;
        let count = result
            .iter()
            .map(|point| {
                point
                    .voltages
                    .len()
                    .saturating_add(point.currents.len())
                    .saturating_mul(2)
                    .saturating_add(1)
            })
            .fold(0usize, usize::saturating_add);
        Ok((ControlAnalysisResult::Ac(result), count))
    }
}

pub(super) fn frequency_table(
    engine: &Engine,
    netlist: &Netlist,
    command: &AnalysisCommand,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<(ControlAnalysisResult, usize), ControlExecutionError> {
    match command {
        AnalysisCommand::AcData { table_name } => {
            let result = engine
                .run_ac_table_with_abort(netlist, table_name, abort)
                .map_err(|error| simulation_error(line, error))?;
            let count = result.retained_value_count();
            Ok((ControlAnalysisResult::AcTable(result), count))
        }
        AnalysisCommand::NoiseData {
            output_node,
            reference_node,
            input_source,
            table_name,
        } => {
            let result = engine
                .run_noise_table_named_with_input_source_and_abort(
                    netlist,
                    output_node,
                    reference_node.as_deref(),
                    input_source,
                    table_name,
                    engine.config().temperature,
                    abort,
                )
                .map_err(|error| simulation_error(line, error))?;
            let count = result.retained_value_count();
            Ok((ControlAnalysisResult::NoiseTable(result), count))
        }
        _ => Err(command_error(line, "expected an AC or NOISE table").into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::FrequencyGridError;
    use crate::{SimulationErrorCategory, SimulationErrorCode};
    use std::error::Error;

    #[test]
    fn frequency_grid_allocation_failure_keeps_control_line_and_allocator_cause() {
        let source = Vec::<f64>::new().try_reserve_exact(usize::MAX).unwrap_err();
        let error = frequency_error(
            17,
            FrequencyGridError::Allocation {
                requested: usize::MAX,
                source: source.clone(),
            },
        );
        let ControlExecutionError::Simulation {
            line,
            source: simulation,
        } = error
        else {
            panic!("allocation failure was reported as a command error");
        };
        assert_eq!(line, 17);
        assert_eq!(
            simulation.descriptor().category,
            SimulationErrorCategory::ResourceLimit
        );
        assert_eq!(
            simulation.descriptor().code,
            SimulationErrorCode::AllocationFailed
        );
        assert_eq!(
            simulation
                .source()
                .unwrap()
                .downcast_ref::<std::collections::TryReserveError>(),
            Some(&source)
        );
        assert!(matches!(
            frequency_error(17, FrequencyGridError::InvalidStartFrequency),
            ControlExecutionError::Command(_)
        ));
        assert!(matches!(
            frequency_error(17, FrequencyGridError::Aborted),
            ControlExecutionError::Simulation {
                line: 17,
                source: SimulationError::Aborted
            }
        ));
    }
}
