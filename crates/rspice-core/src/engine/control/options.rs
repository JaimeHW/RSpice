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

/// Overlay only the subset supported by the runtime grammar. Authored options
/// outside this command retain their parameter-dependent source bindings.
pub(in crate::engine) fn apply_runtime_options(
    target: &mut crate::netlist::SimulationOptions,
    overrides: &crate::netlist::SimulationOptions,
) {
    macro_rules! apply {
        ($($field:ident),+ $(,)?) => { $(
            if overrides.$field.is_some() {
                target.$field = overrides.$field;
            }
        )+ };
    }
    apply!(
        reltol,
        abstol,
        vntol,
        gmin,
        chgtol,
        eventfluxtol,
        trtol,
        xmu,
        itl1,
        itl2,
        itl4,
        temp,
        tnom
    );
    if overrides.method.is_some() {
        target.method = overrides.method.clone();
    }
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
        apply_runtime_options(&mut self.netlist.options, &candidate);
        self.runtime_options = candidate;
        self.netlist.ast_overlay.control_options = Some(self.runtime_options.clone());
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
