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
