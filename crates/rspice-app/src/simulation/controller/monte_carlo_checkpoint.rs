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
            || self
                .current_spec_options
                .as_ref()
                .is_none_or(|options| options.mc_checkpoint.is_none())
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

/// Capture cadence is shared across points; selected trial data is routed after
/// the Run Set has materialized each point's source and numerical environment.
pub(super) fn request_from_config(
    config: Option<&crate::simulation::dialog::mc::checkpoint::McCheckpointConfig>,
) -> Option<crate::simulation::runner::monte_carlo_checkpoint::MonteCarloCheckpointRequest> {
    config.map(|config| {
        crate::simulation::runner::monte_carlo_checkpoint::MonteCarloCheckpointRequest {
            publish_every: config.publish_every,
            trial_range: None,
            resume: None,
        }
    })
}

/// Resolve selected history into checked populations before freezing a task.
/// Different populations stay separate; matching trial journals pool exactly.
pub(super) fn resume_inputs_from_config(
    state: &AppState,
    config: Option<&crate::simulation::dialog::mc::checkpoint::McCheckpointConfig>,
) -> Result<Vec<crate::simulation::execution::PreparedMonteCarloResume>, String> {
    use crate::simulation::runner::monte_carlo_checkpoint::MonteCarloCheckpointInput;
    use rspice_results::monte_carlo_checkpoint::StudyMonteCarloCheckpoint;
    let Some(config) = config else {
        return Ok(Vec::new());
    };
    let limits = rspice_core::ResourceLimits::default();
    let mut selected = Vec::new();
    let mut bytes = 0usize;
    let mut seen = std::collections::HashSet::new();
    for digest in &config.resume {
        if !seen.insert(*digest) {
            return Err("A Monte Carlo checkpoint is selected more than once".into());
        }
        let history = state
            .simulation
            .runs
            .iter()
            .flat_map(|run| &run.analyses)
            .find_map(|analysis| {
                analysis
                    .monte_carlo_checkpoint
                    .as_ref()
                    .filter(|checkpoint| checkpoint.digest() == *digest)
                    .map(|checkpoint| (analysis, checkpoint))
            });
        let (analysis, evidence) = if let Some((analysis, evidence)) = history {
            (Some(analysis), evidence)
        } else {
            (None, state.simulation.imported_monte_carlo_checkpoints.get(*digest)
                .ok_or("A selected Monte Carlo checkpoint is no longer retained. Clear its selection, restore the run or import the checkpoint file before preparing again.")?)
        };
        bytes = bytes.saturating_add(evidence.bytes().len());
        if bytes > limits.max_external_data_bytes {
            return Err("Selected Monte Carlo checkpoints exceed the combined byte limit".into());
        }
        selected.push((analysis, evidence));
    }
    let mut populations: std::collections::BTreeMap<[u8; 32], StudyMonteCarloCheckpoint> =
        Default::default();
    for (analysis, evidence) in selected {
        if let Some(analysis) = analysis {
            evidence.validate_for(analysis.into())?;
        }
        let checkpoint = StudyMonteCarloCheckpoint::from_bytes_with_limits(
            evidence.bytes(),
            limits,
            &rspice_core::NoAbort,
        )
        .map_err(|error| error.to_string())?;
        if let Some(pooled) = populations.get_mut(&checkpoint.population_identity()) {
            pooled
                .merge_with_limits(&checkpoint, limits, &rspice_core::NoAbort)
                .map_err(|error| {
                    format!("Selected Monte Carlo checkpoints cannot be pooled: {error}")
                })?;
        } else {
            populations.insert(checkpoint.population_identity(), checkpoint);
        }
    }
    populations
        .into_values()
        .map(|checkpoint| {
            let bytes = checkpoint
                .to_bytes_with_limits(limits, &rspice_core::NoAbort)
                .map_err(|error| error.to_string())?;
            let input =
                MonteCarloCheckpointInput::from_bytes(bytes).map_err(|error| error.to_string())?;
            crate::simulation::execution::PreparedMonteCarloResume::from_input(input)
        })
        .collect()
}
