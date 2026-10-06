//! Non-executing command admission shared by static checks and execution.
use super::*;
use crate::ResourceLimits;
use crate::netlist::{NetlistParseOptions, ParseError, ParseWithAbortError};

impl ControlCircuit {
    /// Check host availability without executing a command or its arguments.
    pub fn validate_command_name(command: &ControlCommand) -> Result<(), ControlError> {
        CommandKind::parse(command).map(|_| ())
    }

    /// Parse one OP, DC, AC or TRAN request using the execution grammar and limits.
    /// Arguments must already be literal or substituted. No analysis is run.
    pub fn parse_analysis_command(
        command: &ControlCommand,
        resource_limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisCommand, ControlExecutionError> {
        let line = command.line;
        if !matches!(CommandKind::parse(command)?, CommandKind::Analysis) {
            return Err(
                command_error(line, "expected an OP, DC, AC or TRAN analysis command").into(),
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
