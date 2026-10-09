//! Retain accepted continuation state without discarding the live waveform prefix.
use super::*;
use rspice_results::transient_checkpoint::TransientCheckpointEvidence;

impl SimulationController {
    pub(super) fn transient_checkpoint_capture_requested(&self) -> bool {
        matches!(self.current_spec, Some(AnalysisSpec::Transient { .. }))
            && self.current_spec_options.as_ref().is_some_and(|options| {
                options
                    .tran_checkpoint
                    .as_ref()
                    .is_some_and(|request| !request.times.is_empty())
            })
    }

    pub(super) fn publish_transient_checkpoint(&mut self, state: &mut AppState) {
        let Some(bytes) = self.runner.take_transient_checkpoint() else {
            return;
        };
        self.accept_transient_checkpoint(state, bytes);
    }

    pub(super) fn accept_transient_checkpoint(
        &mut self,
        state: &mut AppState,
        bytes: std::sync::Arc<[u8]>,
    ) {
        if self.current_transient_checkpoint_error.is_some() {
            return;
        }
        if let Err(error) = self.retain_transient_checkpoint(state, bytes) {
            self.runner.abort();
            let message = format!("Could not retain transient checkpoint: {error}");
            self.current_transient_checkpoint_error = Some(message.clone());
            state.push_sim_message(ConsoleMessage::error(message));
        }
    }

    pub(super) fn retain_transient_checkpoint(
        &self,
        state: &mut AppState,
        bytes: std::sync::Arc<[u8]>,
    ) -> Result<(), String> {
        if !self.transient_checkpoint_capture_requested() {
            return Err("the active analysis did not request transient checkpoint capture".into());
        }
        let provenance = self
            .current_provenance
            .clone()
            .ok_or("checkpoint has no active prepared-task provenance")?;
        let limits = self
            .current_execution_limits
            .ok_or("checkpoint has no active prepared-task resource limits")?;
        let run_id = self
            .current_run_id
            .ok_or("checkpoint has no active simulation run")?;
        let checkpoint = TransientCheckpointEvidence::from_bytes_with_limits(
            bytes,
            limits,
            &rspice_core::NoAbort,
        )
        .map_err(|error| error.to_string())?;
        let run = state
            .simulation
            .retained
            .run_by_sequence_mut(run_id)
            .ok_or("checkpoint target run no longer exists")?;
        let mut partial = if let Some(previous) =
            run.find_analysis_by_source_instance(provenance.source_instance_id())
        {
            if previous.provenance() != Some(&provenance) || !previous.is_live_partial() {
                return Err("checkpoint target no longer matches the active prepared task".into());
            }
            if let Some(prior) = &previous.transient_checkpoint {
                if checkpoint.digest() == prior.digest() {
                    return Ok(());
                }
                if checkpoint.accepted_time() <= prior.accepted_time() {
                    return Err(
                        "checkpoint changed or lost previously accepted transient state".into(),
                    );
                }
            }
            previous.clone()
        } else {
            AnalysisResult::live_transient_partial(
                1,
                AnalysisType::Transient,
                self.current_analysis_label
                    .as_deref()
                    .unwrap_or("Transient"),
            )
            .with_provenance(provenance)
        };
        checkpoint.validate_for(&partial.data)?;
        partial.transient_checkpoint = Some(checkpoint);
        self.validate_analysis_retention(run, &partial)?;
        run.upsert_live_analysis(partial)?;
        state
            .simulation
            .select_latest_analysis_in_run_sequence(run_id);
        Ok(())
    }
}
