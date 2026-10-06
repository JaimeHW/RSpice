//! Caller-owned transient execution within the shared ordered control host.
use super::*;
use crate::execution::{AnalysisInstanceId, DeckPlan, DeckPlanError, PostProcessSource};

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
        match self.prepare_command(engine, command, variables, abort)? {
            PreparedCommand::Complete(effect) => Ok(effect),
            PreparedCommand::Analyses(analyses) => {
                let mut names = Vec::with_capacity(analyses.len());
                for (analysis, authored_index) in analyses {
                    names.push(self.run_analysis(
                        engine,
                        analysis,
                        authored_index,
                        command.line,
                        abort,
                        runner,
                    )?);
                }
                Ok(ControlCommandEffect::Analyses(names))
            }
        }
    }
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

pub(super) fn analysis_netlist(
    source: &Netlist,
    analysis: &AnalysisCommand,
    authored_index: Option<usize>,
    line: usize,
    limits: &crate::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Netlist, ControlExecutionError> {
    let mut netlist = source.clone();
    netlist.analyses = vec![analysis.clone()];
    netlist.fft_analyses.clear();
    netlist.control_script = None;
    if !matches!(analysis, AnalysisCommand::Tran { .. }) {
        return Ok(netlist);
    }
    let selected = if let Some(index) = authored_index {
        let plan =
            DeckPlan::from_netlist_with_abort(source, limits, abort).map_err(
                |error| match error {
                    DeckPlanError::Aborted => simulation_error(line, SimulationError::Aborted),
                    DeckPlanError::ResourceLimit(error) => simulation_error(line, error.into()),
                    error => command_error(line, error.to_string()).into(),
                },
            )?;
        let parent = plan
            .authored_analyses(source)
            .nth(index)
            .and_then(|(_, id)| id)
            .ok_or_else(|| command_error(line, "authored transient has no planned identity"))?;
        Some(
            plan.post_process_analyses()
                .iter()
                .filter(|post| post.parent() == parent)
                .map(|post| post.source().clone())
                .collect::<Vec<_>>(),
        )
    } else {
        None
    };
    for (card_index, card) in source
        .analyses
        .iter()
        .filter(|card| matches!(card, AnalysisCommand::Four { .. }))
        .enumerate()
    {
        if selected.as_ref().is_none_or(|posts| posts.iter().any(|post| matches!(post, PostProcessSource::FourierOperand { card_index: index, .. } if *index == card_index))) {
            netlist.analyses.push(card.clone());
        }
    }
    for (card_index, card) in source.fft_analyses.iter().enumerate() {
        if selected.as_ref().is_none_or(|posts| posts.iter().any(|post| matches!(post, PostProcessSource::Fft { card_index: index } if *index == card_index))) {
            netlist.fft_analyses.push(card.clone());
        }
    }
    Ok(netlist)
}
