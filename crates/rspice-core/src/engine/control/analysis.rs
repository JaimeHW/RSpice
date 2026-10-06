//! Non-executing command admission shared by static checks and execution.
use super::*;
use crate::ResourceLimits;
use crate::netlist::{NetlistParseOptions, ParseError, ParseWithAbortError};

impl ControlCircuit {
    /// Check host availability without executing a command or its arguments.
    pub fn validate_command_name(command: &ControlCommand) -> Result<(), ControlError> {
        CommandKind::parse(command).map(|_| ())
    }

    /// Parse one OP, DC, AC, NOISE or TRAN request using the execution grammar and limits.
    /// Arguments must already be literal or substituted. No analysis is run.
    pub fn parse_analysis_command(
        command: &ControlCommand,
        resource_limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisCommand, ControlExecutionError> {
        let line = command.line;
        if !matches!(CommandKind::parse(command)?, CommandKind::Analysis) {
            return Err(command_error(
                line,
                "expected an OP, DC, AC, NOISE or TRAN analysis command",
            )
            .into());
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

// Keep cooperative cancellation and configured limits typed during grid construction.
pub(super) fn frequency_error(
    line: usize,
    error: crate::analysis::FrequencyGridError,
) -> ControlExecutionError {
    use crate::analysis::FrequencyGridError;
    match error {
        FrequencyGridError::Aborted => simulation_error(line, SimulationError::Aborted),
        FrequencyGridError::LimitExceeded { requested, limit } => simulation_error(
            line,
            ResourceLimitError {
                resource: ResourceKind::AnalysisPoints,
                requested,
                limit,
            }
            .into(),
        ),
        other => command_error(line, other.to_string()).into(),
    }
}

/// Keep frequency setup/retention outside the generic control dispatch frame.
/// Both result families use the same bounded, cancellable physical grid.
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
        } => (*variation, *points, *start_freq, *stop_freq),
        _ => return Err(command_error(line, "expected AC or NOISE frequency analysis").into()),
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
