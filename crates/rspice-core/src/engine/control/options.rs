//! Source-ordered solver options and supported control-session settings.

use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ControlSettings {
    /// `set noinit` suppresses the automatic initial operating-point listing.
    /// It does not select UIC or change the transient's electrical startup.
    pub suppress_initial_listing: bool,
    /// Requested upper bound from `set num_threads`. Available parallelism
    /// and the caller's resource ceiling may reduce the actual worker count.
    pub maximum_parallel_workers: Option<usize>,
}

impl ControlCircuit {
    pub(super) fn apply_options(
        &mut self,
        engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlExecutionError> {
        if command.arguments.trim().is_empty() {
            return Err(command_error(command.line, "option requires at least one setting").into());
        }
        let candidate = crate::netlist::control_options(
            &command.arguments,
            command.line,
            variables,
            &self.runtime_options,
            engine.config().resource_limits.max_analysis_points,
            abort,
        )
        .map_err(|error| match error {
            crate::netlist::ParseWithAbortError::Aborted => {
                simulation_error(command.line, SimulationError::Aborted)
            }
            crate::netlist::ParseWithAbortError::Parse(
                crate::netlist::ParseError::ResourceLimit(error),
            ) => simulation_error(command.line, error.into()),
            crate::netlist::ParseWithAbortError::Parse(error) => {
                ControlError::new(command.line, ControlErrorKind::Syntax, error.to_string()).into()
            }
        })?;
        let resolved = engine.resolved_for_netlist(&self.netlist);
        let configured = crate::resolve_simulation_config(
            resolved.config(),
            Some(&candidate),
            &crate::SimulationConfigOverrides::default(),
        );
        configured
            .validate()
            .map_err(|source| ControlExecutionError::Configuration {
                line: command.line,
                source,
            })?;
        // This list is exactly the runtime grammar's supported numeric subset.
        macro_rules! apply {
            ($($field:ident),+ $(,)?) => { $(
                if candidate.$field.is_some() {
                    self.netlist.options.$field = candidate.$field;
                }
            )+ };
        }
        apply!(
            reltol, abstol, vntol, gmin, chgtol, trtol, xmu, itl1, itl2, itl4, temp, tnom
        );
        if candidate.method.is_some() {
            self.netlist.options.method = candidate.method.clone();
        }
        self.runtime_options = candidate;
        self.netlist.ast_overlay.control_options = Some(self.netlist.options.clone());
        Ok(())
    }

    pub(super) fn apply_set(
        &mut self,
        command: &ControlCommand,
        variables: &ParamContext,
    ) -> Result<(), ControlExecutionError> {
        let text = command.arguments.trim();
        if text.eq_ignore_ascii_case("noinit") {
            // ngspice frontend/options.c sets ft_noinitprint. It does not
            // change the circuit, charge state, or .TRAN UIC flag.
            self.settings.suppress_initial_listing = true;
            return Ok(());
        }
        let split = text
            .find(|character: char| character.is_whitespace() || character == '=')
            .unwrap_or(text.len());
        let name = text.get(..split).unwrap_or_default();
        let value = text.get(split..).unwrap_or_default().trim();
        let value = value.strip_prefix('=').unwrap_or(value).trim();
        if !name.eq_ignore_ascii_case("num_threads") {
            return Err(command_error(
                command.line,
                format!("set variable '{name}' has no control-host handler"),
            )
            .into());
        }
        let count = scalar(value, variables, command.line)?;
        if count < 1.0 || count.fract() != 0.0 || count >= usize::MAX as Value {
            return Err(command_error(
                command.line,
                "num_threads requires a positive representable integer",
            )
            .into());
        }
        self.settings.maximum_parallel_workers = Some(count as usize);
        Ok(())
    }
}
