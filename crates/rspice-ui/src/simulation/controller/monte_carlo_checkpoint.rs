//! Adopt the runner's latest complete trial journal into project result state.

use super::*;
use crate::state::MonteCarloCheckpointEvidence;

impl SimulationController {
    pub(super) fn publish_monte_carlo_checkpoint(&mut self, state: &mut AppState) {
        let Some(bytes) = self.runner.take_monte_carlo_checkpoint() else {
            return;
        };
        if let Err(error) = self.retain_monte_carlo_checkpoint(state, bytes) {
            // A requested durable snapshot cannot silently become transient
            // data. Stop further work and preserve the last admitted journal.
            self.runner.abort();
            state.push_sim_message(ConsoleMessage::error(format!(
                "Could not retain Monte Carlo checkpoint: {error}"
            )));
        }
    }

    fn retain_monte_carlo_checkpoint(
        &self,
        state: &mut AppState,
        bytes: std::sync::Arc<[u8]>,
    ) -> Result<(), String> {
        if !matches!(self.current_spec, Some(AnalysisSpec::MonteCarlo { .. }))
            || !self
                .current_spec_options
                .as_ref()
                .is_some_and(|options| options.mc_checkpoint.is_some())
        {
            return Err(
                "the active analysis did not request Monte Carlo checkpoint capture".into(),
            );
        }
        let provenance = self
            .current_provenance
            .clone()
            .ok_or("checkpoint has no active prepared-task provenance")?;
        let run_id = self
            .current_run_id
            .ok_or("checkpoint has no active simulation run")?;
        let checkpoint = MonteCarloCheckpointEvidence::from_bytes(bytes)?;
        let run = state
            .simulation
            .run_by_sequence_mut(run_id)
            .ok_or("checkpoint target run no longer exists")?;
        if let Some(previous) =
            run.find_analysis_by_source_instance(provenance.source_instance_id())
        {
            if previous.provenance() != Some(&provenance) || !previous.is_live_partial() {
                return Err("checkpoint target no longer matches the active prepared task".into());
            }
            if let Some(prior) = &previous.monte_carlo_checkpoint {
                if checkpoint.population_identity() != prior.population_identity()
                    || checkpoint.completed_trials() < prior.completed_trials()
                {
                    return Err("checkpoint changed population or lost committed trials".into());
                }
                if checkpoint.digest() == prior.digest() {
                    return Ok(());
                }
            }
        }
        let partial = AnalysisResult::live_monte_carlo_partial(
            self.current_analysis_label
                .as_deref()
                .unwrap_or("Monte Carlo"),
            checkpoint,
        )
        .with_provenance(provenance);
        self.validate_analysis_retention(run, &partial)?;
        run.upsert_live_analysis(partial)?;
        state
            .simulation
            .select_latest_analysis_in_run_sequence(run_id);
        Ok(())
    }
}

/// Resolve selected history into owned, checked bytes before a task is prepared.
/// Later history pruning or UI selection changes cannot alter a queued request.
pub(super) fn request_from_config(
    state: &AppState,
    config: Option<&crate::simulation::dialog::mc::checkpoint::McCheckpointConfig>,
) -> Result<
    Option<crate::simulation::runner::monte_carlo_checkpoint::MonteCarloCheckpointRequest>,
    String,
> {
    use crate::simulation::runner::monte_carlo_checkpoint::{
        MonteCarloCheckpointInput, MonteCarloCheckpointRequest,
    };
    use crate::simulation::runner::study::monte_carlo::checkpoint::StudyMonteCarloCheckpoint;
    let Some(config) = config else {
        return Ok(None);
    };
    let limits = rspice_core::ResourceLimits::default();
    let mut pooled: Option<StudyMonteCarloCheckpoint> = None;
    for digest in &config.resume {
        let (analysis, evidence) = state.simulation.runs.iter()
            .flat_map(|run| &run.analyses)
            .find_map(|analysis| analysis.monte_carlo_checkpoint.as_ref()
                .filter(|checkpoint| checkpoint.digest() == *digest)
                .map(|checkpoint| (analysis, checkpoint)))
            .ok_or("A selected Monte Carlo checkpoint is no longer retained. Clear its selection or restore the run before preparing again.")?;
        evidence.validate_for(analysis)?;
        let checkpoint = StudyMonteCarloCheckpoint::from_bytes_with_limits(
            evidence.bytes(),
            limits,
            &rspice_core::NoAbort,
        )
        .map_err(|error| error.to_string())?;
        if let Some(pooled) = &mut pooled {
            pooled
                .merge_with_limits(&checkpoint, limits, &rspice_core::NoAbort)
                .map_err(|error| {
                    format!("Selected Monte Carlo checkpoints cannot be pooled: {error}")
                })?;
        } else {
            pooled = Some(checkpoint);
        }
    }
    let resume = pooled
        .map(|checkpoint| {
            let bytes = checkpoint.to_bytes_with_limits(limits, &rspice_core::NoAbort)?;
            MonteCarloCheckpointInput::from_bytes(bytes)
        })
        .transpose()
        .map_err(|error| error.to_string())?;
    Ok(Some(MonteCarloCheckpointRequest {
        publish_every: config.publish_every,
        trial_range: None,
        resume,
    }))
}
