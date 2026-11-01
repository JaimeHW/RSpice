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
        self.netlist
            .retain_control_options(self.runtime_options.clone());
        Ok(())
    }

    pub(super) fn apply_set(
        &mut self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlExecutionError> {
        use crate::execution::control::shell::{SetValue, assignments};
        let assignments = assignments(&command.arguments, command.line)?;
        if assignments.is_empty() {
            return Err(command_error(command.line, "set requires a variable").into());
        }
        // Validate every assignment before applying any, so a refused
        // variable leaves the session's settings as they were.
        let mut settings = self.settings.clone();
        for assignment in &assignments {
            let name = assignment.name.to_ascii_lowercase();
            if name == "noinit" || name == "noacct" {
                // ngspice frontend/options.c sets ft_noinitprint (noinit) or
                // ft_noacctprint (noacct). Both suppress the initial transient
                // listing (spicelib/analysis/dctran.c:251); noacct also drops
                // the batch accounting summary (frontend/dotcards.c:401), which
                // this host never prints. Neither changes the circuit, charge
                // state, or .TRAN UIC flag.
                settings.suppress_initial_listing = true;
            } else if name == "num_threads" {
                let SetValue::Word(value) = &assignment.value else {
                    return Err(command_error(
                        command.line,
                        "num_threads requires a positive representable integer",
                    )
                    .into());
                };
                let count = scalar(&value.text, variables, command.line, abort)?;
                if count < 1.0 || count.fract() != 0.0 || count >= usize::MAX as Value {
                    return Err(command_error(
                        command.line,
                        "num_threads requires a positive representable integer",
                    )
                    .into());
                }
                settings.maximum_parallel_workers = Some(count as usize);
            } else if !presentation_variable(&name) {
                return Err(command_error(
                    command.line,
                    format!(
                        "set variable '{}' has no control-host handler",
                        assignment.name
                    ),
                )
                .into());
            }
        }
        self.settings = settings;
        Ok(())
    }
}

/// Variables that only steer interactive use, plotting or hardcopy. A batch
/// run has no window, printer or prompt, so ngspice -b accepts them with no
/// effect on any result (manual section 17.7, "Variables").
fn presentation_variable(name: &str) -> bool {
    const NAMES: &[&str] = &[
        "askquit",
        "noaskquit",
        "nolegend",
        "nogrid",
        "gridstyle",
        "gridsize",
        "plotstyle",
        "pointchars",
        "ticmarks",
        "ticlist",
        "ticchar",
        "pensize",
        "xbrushwidth",
        "xgridwidth",
        "xfont",
        "xfont_size",
        "wfont",
        "wfont_size",
        "x11lineararcs",
        "plotwinwidth",
        "plotwinheight",
        "plotwininfo",
        "device",
        "term",
        "lprplot",
        "lprps",
        "editor",
        "prompt",
        "history",
        "moremode",
        "nomoremode",
        "interactive",
        "controlswait",
        "curplottitle",
    ];
    NAMES.contains(&name)
        || ["hcopy", "gnuplot_", "svg_"]
            .iter()
            .any(|prefix| name.starts_with(prefix))
        || name
            .strip_prefix("color")
            .is_some_and(|index| !index.is_empty() && index.bytes().all(|b| b.is_ascii_digit()))
}
