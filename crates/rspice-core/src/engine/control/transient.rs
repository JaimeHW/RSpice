//! Caller-owned transient execution within the shared ordered control host,
//! and the transients a `stop` interrupts and `resume` continues.
use super::*;
use crate::engine::transient::control_stop::TransientStop;
use crate::identity::AnalysisInstanceId;

/// A transient a `stop` interrupted, which `resume` continues: ngspice's run
/// in progress (`ci_inprogress`, frontend/runcoms2.c:86).
pub(super) struct PausedTransient {
    /// The plot the run fills.
    dataset: String,
    /// The deck position of an authored transient `run` executed.
    authored_index: Option<usize>,
    /// The run's state at the stop.
    state: TransientStop,
    tstop: Value,
    max_step: Value,
    /// The analyses of the interrupted command still to run once the
    /// transient completes.
    remaining: Vec<(AnalysisCommand, Option<usize>)>,
}

impl ControlCircuit {
    /// Execute one command with the engine's ordinary transient policy.
    pub fn execute(
        &mut self,
        engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<ControlCommandEffect, ControlExecutionError> {
        self.execute_with_transient_runner(
            engine,
            command,
            variables,
            abort,
            &mut |engine, netlist, analysis, _, abort| {
                default_transient(engine, netlist, analysis, command.line, abort)
            },
        )
    }

    /// Execute using a host's transient policy, for example checkpointing or
    /// compression. The callback runs exactly once per transient, after source-
    /// ordered options, post-process binding, identity and remaining resource
    /// limits have been resolved. The returned trajectory is admitted and kept
    /// as the script's named dataset; presentation commands see that same result.
    /// Host errors remain typed and stop execution without retaining a dataset.
    pub fn execute_with_transient_runner<E, F>(
        &mut self,
        engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
        runner: &mut F,
    ) -> Result<ControlCommandEffect, E>
    where
        E: From<ControlExecutionError> + From<ControlError>,
        F: FnMut(
            &Engine,
            &Netlist,
            &AnalysisCommand,
            AnalysisInstanceId,
            &dyn AbortSignal,
        ) -> Result<TransientResult, E>,
    {
        let mut names = Vec::new();
        match self.prepare_command(engine, command, variables, abort)? {
            PreparedCommand::Complete(effect) => return Ok(effect),
            PreparedCommand::Analyses(analyses) => {
                self.run_analyses(engine, analyses, (command.line, abort), runner, &mut names)?;
            }
            PreparedCommand::Resume(paused) => {
                self.resume(engine, *paused, (command.line, abort), runner, &mut names)?;
            }
        }
        Ok(ControlCommandEffect::Analyses(names))
    }

    /// Whether `name` is the plot of a transient a `stop` interrupted: it
    /// holds the run up to the stop until `resume` completes it.
    pub fn is_paused(&self, name: &str) -> bool {
        self.paused
            .as_ref()
            .is_some_and(|paused| paused.dataset.eq_ignore_ascii_case(name))
    }

    /// Run `analyses` in order, naming each plot in `names`. A transient a
    /// stop interrupts keeps the analyses after it for `resume`.
    fn run_analyses<E, F>(
        &mut self,
        engine: &Engine,
        analyses: Vec<(AnalysisCommand, Option<usize>)>,
        (line, abort): (usize, &dyn AbortSignal),
        runner: &mut F,
        names: &mut Vec<String>,
    ) -> Result<(), E>
    where
        E: From<ControlExecutionError> + From<ControlError>,
        F: FnMut(
            &Engine,
            &Netlist,
            &AnalysisCommand,
            AnalysisInstanceId,
            &dyn AbortSignal,
        ) -> Result<TransientResult, E>,
    {
        let mut analyses = analyses.into_iter();
        while let Some((analysis, authored_index)) = analyses.next() {
            names.push(self.run_analysis(engine, analysis, authored_index, line, abort, runner)?);
            if let Some(paused) = &mut self.paused {
                paused.remaining = analyses.collect();
                break;
            }
        }
        Ok(())
    }

    /// `stop when time = t` (frontend/breakp.c:40 com_stop): every later
    /// transient that reaches `t` lands on it and is interrupted there
    /// (breakp.c:192 sets the breakpoint). ngspice reads `time=t` as one
    /// word too (breakp.c:93).
    pub(super) fn stop(
        &mut self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlError> {
        let line = command.line;
        let mut words = Vec::new();
        for word in shell::words(&command.arguments) {
            match word.text.split_once('=') {
                Some((name, value)) if !name.is_empty() && !value.is_empty() => {
                    words.extend([name.to_owned(), "=".to_owned(), value.to_owned()]);
                }
                _ => words.push(word.text),
            }
        }
        let [when, name, relation, value] = words.as_slice() else {
            return Err(stop_form(line));
        };
        if !when.eq_ignore_ascii_case("when")
            || !name.eq_ignore_ascii_case("time")
            || !matches!(relation.to_ascii_lowercase().as_str(), "=" | "eq")
        {
            return Err(stop_form(line));
        }
        let time = scalar(value, variables, line, abort)?;
        if !(time.is_finite() && time > 0.0) {
            return Err(command_error(
                line,
                format!("stop: the stop time must be positive and finite, found {value}"),
            ));
        }
        self.stops.push(time);
        Ok(())
    }

    /// The first stop a transient of `analysis` reaches after `after` (after
    /// its start when `None`) and before its end.
    pub(super) fn next_stop(
        &self,
        analysis: &AnalysisCommand,
        after: Option<Value>,
    ) -> Option<Value> {
        let AnalysisCommand::Tran { stop: tstop, .. } = analysis else {
            return None;
        };
        let after = after.unwrap_or(0.0);
        self.stops
            .iter()
            .copied()
            .filter(|&time| time > after && time < *tstop)
            .min_by(Value::total_cmp)
    }

    /// Run the transient of `analysis` on `engine` to `stop`, where ngspice
    /// interrupts it, and keep its state for `resume`; a model that finishes
    /// the run first completes it.
    pub(super) fn run_until_stop(
        &self,
        engine: &Engine,
        netlist: &Netlist,
        analysis: &AnalysisCommand,
        (authored_index, name): (Option<usize>, &str),
        (stop, line): (Value, usize),
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Option<Box<PausedTransient>>), ControlExecutionError> {
        let AnalysisCommand::Tran {
            step,
            stop: tstop,
            start,
            max_step,
            uic,
        } = analysis
        else {
            return Err(command_error(line, "a stop interrupts only a transient").into());
        };
        let maximum_step = crate::analysis::transient::resolve_dialect_transient_maximum_step(
            engine.config().spice_dialect,
            *step,
            *tstop,
            *start,
            *max_step,
        )
        .map_err(|error| command_error(line, error.to_string()))?;
        let (result, state) = engine
            .run_tran_to_control_stop(
                &segment_netlist(netlist),
                (*tstop, maximum_step),
                TransientStartupMode::from_uic(*uic),
                stop,
                abort,
            )
            .map_err(|error| simulation_error(line, error))?;
        let paused = state.map(|state| {
            Box::new(PausedTransient {
                dataset: name.to_owned(),
                authored_index,
                state,
                tstop: *tstop,
                max_step: maximum_step,
                remaining: Vec::new(),
            })
        });
        Ok((result, paused))
    }

    /// `resume` (frontend/runcoms2.c:63 com_resume): continue the
    /// interrupted transient from its stop on the circuit as the script has
    /// left it, into the same plot, to the next stop or its end; then run
    /// the analyses its command had left.
    fn resume<E, F>(
        &mut self,
        engine: &Engine,
        paused: PausedTransient,
        (line, abort): (usize, &dyn AbortSignal),
        runner: &mut F,
        names: &mut Vec<String>,
    ) -> Result<(), E>
    where
        E: From<ControlExecutionError> + From<ControlError>,
        F: FnMut(
            &Engine,
            &Netlist,
            &AnalysisCommand,
            AnalysisInstanceId,
            &dyn AbortSignal,
        ) -> Result<TransientResult, E>,
    {
        let PausedTransient {
            dataset,
            authored_index,
            state,
            tstop,
            max_step,
            remaining,
        } = paused;
        let index = self.dataset_index(&dataset).ok_or_else(|| {
            command_error(
                line,
                format!("resume: the interrupted run's plot {dataset} no longer exists"),
            )
        })?;
        let analysis = self.datasets[index].command.clone();
        let configured = self.analysis_config(engine);
        let mut bounded = engine
            .try_resolved_with_config(configured)
            .map_err(|source| ControlExecutionError::Configuration { line, source })?;
        let final_state = crate::engine::core::FinalDeviceState::default();
        bounded.final_device_state = Some(std::sync::Arc::clone(&final_state));
        let netlist = segment_netlist(&analysis_netlist(
            &self.netlist,
            &analysis,
            authored_index,
            line,
            abort,
        )?);
        let stop = self.next_stop(&analysis, Some(state.time()));
        let (segment, state) = bounded
            .resume_tran_from_control_stop(&netlist, state, (tstop, max_step), stop, abort)
            .map_err(|error| simulation_error(line, error))?;
        let retained_values = self
            .retained_values
            .saturating_add(Engine::transient_result_value_count(&segment));
        engine
            .ensure_result_values(retained_values)
            .map_err(|error| simulation_error(line, error))?;
        let ControlAnalysisResult::Transient(result) = &mut self.datasets[index].result else {
            return Err(command_error(
                line,
                "resume: the interrupted run's plot is not a transient",
            )
            .into());
        };
        result
            .extend_resumed(segment)
            .map_err(|error| command_error(line, format!("resume: {error}")))?;
        self.retained_values = retained_values;
        self.device_state = final_state
            .lock()
            .ok()
            .and_then(|mut report| report.take())
            .map(Box::new)
            .map(presentation::DeviceState::Solved);
        self.current = Some(index);
        self.vectors.leave_plot();
        names.push(dataset.clone());
        match state {
            Some(state) => {
                self.paused = Some(Box::new(PausedTransient {
                    dataset,
                    authored_index,
                    state,
                    tstop,
                    max_step,
                    remaining,
                }));
                Ok(())
            }
            None => self.run_analyses(engine, remaining, (line, abort), runner, names),
        }
    }
}

fn stop_form(line: usize) -> ControlError {
    command_error(
        line,
        "stop: a run is interrupted only by `stop when time = <time>`",
    )
}

/// The netlist one segment of an interrupted transient runs: its `.four` and
/// `.fft` cards are dropped, since their spectra belong to the whole run,
/// which no segment is.
fn segment_netlist(netlist: &Netlist) -> Netlist {
    let mut netlist = netlist.clone();
    netlist
        .analyses
        .retain(|card| matches!(card, AnalysisCommand::Tran { .. }));
    netlist.fft_analyses.clear();
    netlist
}

fn default_transient(
    engine: &Engine,
    netlist: &Netlist,
    analysis: &AnalysisCommand,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<TransientResult, ControlExecutionError> {
    let AnalysisCommand::Tran {
        step,
        stop,
        start,
        max_step,
        uic,
    } = analysis
    else {
        return Err(command_error(line, "transient runner received another analysis kind").into());
    };
    let maximum_step =
        crate::analysis::transient::resolve_transient_maximum_step(*step, *stop, *start, *max_step)
            .map_err(|error| command_error(line, error.to_string()))?;
    engine
        .run_tran_with_startup_mode_and_abort(
            netlist,
            *stop,
            maximum_step,
            TransientStartupMode::from_uic(*uic),
            abort,
        )
        .map_err(|error| simulation_error(line, error))
}

/// The single-analysis netlist a command runs, carrying the authored `.FOUR`
/// and `.FFT` cards a transient post-processes.
///
/// An authored transient (one `run` executes) carries only the cards the deck
/// binds to it, by the rule `Netlist` states; the host plans their identities
/// from this netlist. A transient the script itself issues has no authored
/// position, so it carries every card.
pub(super) fn analysis_netlist(
    source: &Netlist,
    analysis: &AnalysisCommand,
    authored_index: Option<usize>,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<Netlist, ControlExecutionError> {
    let mut netlist = source.clone();
    netlist.analyses = vec![analysis.clone()];
    netlist.fft_analyses.clear();
    netlist.control_script = None;
    if !matches!(analysis, AnalysisCommand::Tran { .. }) {
        return Ok(netlist);
    }
    if abort.is_aborted() {
        return Err(simulation_error(line, SimulationError::Aborted));
    }
    // Ordinal of the authored transient among the deck's `.TRAN` cards.
    let selected = match authored_index {
        Some(index) => {
            if !matches!(
                source.analyses.get(index),
                Some(AnalysisCommand::Tran { .. })
            ) {
                return Err(
                    command_error(line, "authored transient has no planned identity").into(),
                );
            }
            Some(
                source
                    .analyses
                    .iter()
                    .take(index)
                    .filter(|card| matches!(card, AnalysisCommand::Tran { .. }))
                    .count(),
            )
        }
        None => None,
    };
    let fourier_transients = source.fourier_transient_ordinals();
    for (card_index, card) in source
        .analyses
        .iter()
        .filter(|card| matches!(card, AnalysisCommand::Four { .. }))
        .enumerate()
    {
        if selected.is_none_or(|ordinal| fourier_transients.get(card_index) == Some(&ordinal)) {
            netlist.analyses.push(card.clone());
        }
    }
    if selected.is_none_or(|ordinal| ordinal == Netlist::FFT_TRANSIENT_ORDINAL) {
        netlist
            .fft_analyses
            .extend(source.fft_analyses.iter().cloned());
    }
    Ok(netlist)
}
