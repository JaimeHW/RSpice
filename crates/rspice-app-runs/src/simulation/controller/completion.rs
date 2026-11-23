//! Application reaction to a completed simulation task.

use super::*;
use std::sync::Arc;

impl SimulationController {
    /// Keep `result` with its conversion done on this frame.
    #[cfg(test)]
    pub(super) fn accept_completion(
        &mut self,
        state: &mut impl RunHost,
        export_io: &(impl ExportWorkflowIo + ?Sized),
        result: Result<crate::simulation::SimulationResult, SimulationError>,
    ) {
        if let Some((head, conversion)) = self.begin_completion(state, result) {
            let converted = self.convert_here(conversion);
            self.end_completion(state, export_io, head, converted);
        }
    }

    /// Keep `result` as a frame finds it: the result is converted into
    /// the run's analysis away from it, and the frame that finds it converted
    /// keeps it. A point a family is assembled from is converted here, where
    /// the family is kept.
    pub(super) fn accept_completion_off_frame(
        &mut self,
        state: &mut impl RunHost,
        export_io: &(impl ExportWorkflowIo + ?Sized),
        result: Result<crate::simulation::SimulationResult, SimulationError>,
    ) {
        let Some((head, conversion)) = self.begin_completion(state, result) else {
            return;
        };
        let assembled = self
            .current_provenance
            .as_ref()
            .is_some_and(|provenance| self.point_families.reduces(provenance.source_instance_id()));
        let conversion = if assembled {
            conversion
        } else {
            match convert_off_frame(conversion) {
                Ok(converting) => {
                    self.finishing = Some(Finishing { head, converting });
                    return;
                }
                Err(conversion) => *conversion,
            }
        };
        let converted = self.convert_here(conversion);
        self.end_completion(state, export_io, head, converted);
    }

    /// Keep the result converted away from the frame, once it has been: says
    /// whether a completion is still being converted.
    pub(super) fn finish_converted(
        &mut self,
        state: &mut impl RunHost,
        export_io: &(impl ExportWorkflowIo + ?Sized),
    ) -> bool {
        let Some(finishing) = self.finishing.take() else {
            return false;
        };
        if !finishing.is_converted() {
            self.finishing = Some(finishing);
            return true;
        }
        match finishing.converting.join() {
            Ok(converted) => {
                self.end_completion(state, export_io, finishing.head, converted);
                false
            }
            // A conversion that failed fails the frame, as it did when the
            // frame converted.
            Err(panic) => std::panic::resume_unwind(panic),
        }
    }

    /// `conversion` converted on this frame, the point a family is
    /// assembled from kept as it is converted.
    fn convert_here(&mut self, conversion: Conversion) -> Converted {
        let instance = self
            .current_provenance
            .as_ref()
            .map(|provenance| provenance.source_instance_id());
        let families = &mut self.point_families;
        conversion.convert(|analysis| {
            if let Some(instance) = instance {
                families.capture_result(instance, analysis);
            }
        })
    }

    /// What converting `result` into the run's analysis takes, owned so it
    /// can be converted away from the frame. The task's saved-output
    /// contracts go with it.
    fn conversion(
        &mut self,
        result: crate::simulation::SimulationResult,
        analysis_type: AnalysisType,
        label: &str,
        artifact_failure: Option<String>,
    ) -> Conversion {
        let (analysis_type, label) = match &self.current_config {
            Some(config) => (
                self.config_to_analysis_type(config),
                self.analysis_name(config).to_string(),
            ),
            None => (analysis_type, label.to_owned()),
        };
        let contracts = std::mem::take(&mut self.current_saved_output_contracts);
        Conversion {
            result,
            analysis_type,
            label,
            pnoise: self
                .current_spec_options
                .as_ref()
                .and_then(|options| options.pnoise.clone()),
            carrier_hz: self.current_periodic_carrier_hz,
            op_digest: self.current_op_effective_source_content_digest,
            artifact_failure,
            save_policy: self.current_save_policy,
            contracts,
            // Asked once the contracts are taken, as it always was.
            deck_outputs: self
                .previews_every_column()
                .then(|| self.current_deck_outputs.clone()),
        }
    }

    /// Everything a completed task says up to converting its result: what
    /// it reported, and the result with what converting it takes. A failure
    /// is handled whole here, and leaves nothing to convert.
    fn begin_completion(
        &mut self,
        state: &mut impl RunHost,
        result: Result<crate::simulation::SimulationResult, SimulationError>,
    ) -> Option<(CompletionHead, Conversion)> {
        self.view_current_for = None;
        // What the task raised and went on past. Taken on every outcome, so
        // an aborted task's notices cannot be read as the next task's. Each
        // goes with the place in the run's log of the line that stated it,
        // which the task's last lines have reached by now.
        let notices: Vec<_> = self
            .runner
            .drain_engine_notices()
            .into_iter()
            .map(|notice| {
                let line = notice
                    .line
                    .and_then(|ordinal| self.current_engine_lines.get(&ordinal).copied());
                (notice, line)
            })
            .collect();
        match result {
            Ok(sim_result) => {
                log::info!(
                    "Analysis {}/{} completed! Result type: {:?}",
                    self.current_analysis_idx,
                    self.total_analyses,
                    std::mem::discriminant(&sim_result)
                );

                // Log completion to console
                let current_label = self
                    .current_analysis_label
                    .clone()
                    .or_else(|| {
                        self.current_spec
                            .as_ref()
                            .map(|spec| Self::analysis_name_for_spec(spec).to_owned())
                    })
                    .or_else(|| {
                        self.current_config
                            .as_ref()
                            .map(|config| self.analysis_name(config).to_owned())
                    })
                    .unwrap_or_else(|| "Analysis".to_owned());
                let figures = super::run_lines::analysis_figures(
                    &current_label,
                    self.current_task_started
                        .take()
                        .map(|started| started.elapsed()),
                    &sim_result,
                );
                self.run_line(state.model_mut(), LogSeverity::Info, figures);

                // A short FFT record is a successful result with no
                // spectrum. It is stated in the run's log as well as on the
                // sheet, because the reader is looking at the run here.
                if let crate::simulation::SimulationResult::Fft { spectrum, .. } = &sim_result
                    && let Some(notice) =
                        rspice_simulation::result_conversion::incomplete_history_notice(spectrum)
                {
                    self.run_line(state.model_mut(), LogSeverity::Warning, notice);
                }

                // Convert SimulationResult to AnalysisResult and add to run
                let analysis_type = self
                    .current_spec
                    .as_ref()
                    .map(|spec| self.spec_to_analysis_type(spec))
                    .or_else(|| {
                        self.current_config
                            .as_ref()
                            .map(|cfg| self.config_to_analysis_type(cfg))
                    })
                    .unwrap_or(AnalysisType::DcOp);
                let target_run_id = self.target_run_id();

                // A resume bound to this transient takes the checkpoint image
                // it captured at the resume's time, so the images it published
                // as it ran travel with its result.
                let transient_checkpoints = self.runner.take_task_transient_checkpoints();
                let produced_artifact = self.current_artifact_producer.as_ref()
                    .map_or(Ok(None), |producer| {
                        producer.capture_with_transient_checkpoints(
                            &sim_result,
                            &transient_checkpoints,
                            &self.pending_analyses,
                        )
                    })
                    .map_err(|error| format!(
                        "{current_label}: the result could not be passed to the analyses that depend on it: {error}"
                    ));
                let (produced_artifact, artifact_failure) = match produced_artifact {
                    Ok(artifact) => (artifact, None),
                    Err(message) => {
                        self.run_line(state.model_mut(), LogSeverity::Error, message.clone());
                        (None, Some(message))
                    }
                };

                // Prepare external and derived evidence while the raw
                // solver result is available, but publish neither until
                // the immutable analysis has crossed the exact run's
                // retention boundary successfully.
                let prepared_touchstone_export = target_run_id.and_then(|run_id| {
                    match self.prepare_touchstone_export(&sim_result, run_id) {
                        Ok(prepared) => prepared,
                        Err(error) => {
                            self.run_line(
                                state.model_mut(),
                                LogSeverity::Warning,
                                format!("Touchstone export skipped: {error}"),
                            );
                            None
                        }
                    }
                });
                self.apply_result_side_effects(state, &sim_result);
                // A transient its results storage stopped keeps the rows it
                // wrote: its result stands, and the stop is said beside it.
                self.forget_released_segments(&*state);
                let storage_stop =
                    self.note_results_store(&sim_result, &current_label, target_run_id);
                // A recorded FFT carries the quality of the transient it
                // transformed, which that transient has already reported.
                let solved_transient = match &sim_result {
                    crate::simulation::SimulationResult::Fft { .. } => None,
                    solved => solved.transient_convergence().cloned(),
                };
                let truncation_line = match sim_result.transient_convergence() {
                    Some(quality) if quality.has_lte_exceptions() => self.placed_run_line(
                        state.model_mut(),
                        LogSeverity::Warning,
                        format!(
                            "{current_label}: accepted points exceeded the local truncation-error criterion. Review convergence details in the result inspector."
                        ),
                    ),
                    _ => None,
                };

                let artifact_failed = artifact_failure.is_some();
                let conversion =
                    self.conversion(sim_result, analysis_type, &current_label, artifact_failure);
                return Some((
                    CompletionHead {
                        label: current_label,
                        target_run_id,
                        notices,
                        produced_artifact,
                        artifact_failed,
                        prepared_touchstone_export,
                        storage_stop,
                        solved_transient,
                        truncation_line,
                    },
                    conversion,
                ));
            }
            Err(SimulationError::Aborted) => {
                log::info!("Analysis aborted; retaining any accepted transient prefix");
                let partial = if self.live_transient.is_empty()
                    || !self.current_save_policy.retain_failure_diagnostics()
                {
                    None
                } else {
                    let analysis_type = self
                        .current_spec
                        .as_ref()
                        .map(|spec| self.spec_to_analysis_type(spec))
                        .or_else(|| {
                            self.current_config
                                .as_ref()
                                .map(|config| self.config_to_analysis_type(config))
                        })
                        .unwrap_or(AnalysisType::Transient);
                    let label = self
                        .current_analysis_label
                        .clone()
                        .or_else(|| {
                            self.current_spec
                                .as_ref()
                                .map(|spec| Self::analysis_name_for_spec(spec).to_owned())
                        })
                        .or_else(|| {
                            self.current_config
                                .as_ref()
                                .map(|config| self.analysis_name(config).to_owned())
                        })
                        .unwrap_or_else(|| "Transient".to_owned());
                    self.current_provenance.take().map(|provenance| {
                        self.partial_failure_analysis(
                            analysis_type,
                            &label,
                            "Simulation aborted by user",
                            provenance,
                        )
                    })
                };
                let mut partial = partial.map(|mut analysis| {
                    self.materialize_current_saved_outputs(&mut analysis);
                    analysis
                });
                self.seal_aborted_run(state.model_mut(), partial.take());
                if let Some(run_sequence) = self.current_run_id {
                    super::run_lines::announce_run_end(state.model_mut(), run_sequence);
                }
                self.pending_analyses.clear();
                self.successful_analysis_instances.clear();
                self.execution_artifacts.clear();
                self.point_families.clear();
                self.cached_netlist = None;
                self.current_config = None;
                self.current_spec = None;
                self.current_analysis_label = None;
                self.current_spec_options = None;
                self.current_execution_limits = None;
                self.current_transient_checkpoint_error = None;
                self.current_periodic_carrier_hz = None;
                self.current_artifact_producer = None;
                self.current_provenance = None;
                self.current_config_digest = None;
                self.current_op_effective_source_content_digest = None;
                self.current_saved_output_contracts.clear();
                self.current_save_policy =
                    crate::simulation::execution::SavePolicy::RetainEngineProducedResults;
                self.live_transient.clear();
                self.current_source_domain = AnalysisResultSourceDomain::SimulationPlan;
                self.current_deck_outputs = Default::default();
                self.current_run_id = None;
                self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
                self.current_analysis_idx = 0;
                self.total_analyses = 0;
                let execution = &mut state.model_mut().simulation.execution;
                execution.active_execution = None;
                execution.abort_request = None;
                let manual_run = &mut state.runs_mut().manual_run;
                manual_run.pending_manual_run_id = None;
                manual_run.pending_run_buffer = None;
                state.model_mut().simulation.execution.status = "Aborted".to_string();
                self.complete_campaign_member(state, false);
            }
            Err(e) => {
                let attribution = e.attribution().cloned();

                // Mark run as partially failed and add failed analysis entry
                let failed_label = self
                    .current_analysis_label
                    .clone()
                    .or_else(|| {
                        self.current_spec
                            .as_ref()
                            .map(|spec| Self::analysis_name_for_spec(spec).to_owned())
                    })
                    .or_else(|| {
                        self.current_config
                            .as_ref()
                            .map(|config| self.analysis_name(config).to_owned())
                    })
                    .unwrap_or_else(|| "Analysis".to_owned());
                // A limit the run went past is said in the reader's words,
                // never the engine's.
                let why = Self::limit_refusal(state, &e).unwrap_or_else(|| e.to_string());
                let console_line = Self::refusal_console_line(&failed_label, &e)
                    .unwrap_or_else(|| format!("{failed_label} failed: {why}"));
                Self::report_failed_analysis(
                    state,
                    &failed_label,
                    &console_line,
                    &attribution,
                    self.target_run_id(),
                );
                // A clamp the analysis raised before it failed is part of
                // the account of the failure.
                if let Some(run_sequence) = self.target_run_id() {
                    let facts = state.slot_facts();
                    state.model_mut().publish_run_notices(
                        &failed_label,
                        notices,
                        None,
                        &[],
                        run_sequence,
                        facts,
                    );
                }
                let failed_type = self
                    .current_spec
                    .as_ref()
                    .map(|spec| self.spec_to_analysis_type(spec))
                    .or_else(|| {
                        self.current_config
                            .as_ref()
                            .map(|cfg| self.config_to_analysis_type(cfg))
                    })
                    .unwrap_or(AnalysisType::DcOp);
                let target_run_id = self.target_run_id();
                let failed_analysis = if let Some(provenance) = self.current_provenance.take() {
                    let mut analysis =
                        self.partial_failure_analysis(failed_type, &failed_label, why, provenance);
                    analysis.failure_attribution = attribution;
                    analysis.failure_capability = match &e {
                        SimulationError::UnsupportedCapability { capability, .. } => {
                            Some(capability.clone())
                        }
                        _ => None,
                    };
                    self.materialize_current_saved_outputs(&mut analysis);
                    Some(analysis)
                } else {
                    self.run_line(
                        state.model_mut(),
                        LogSeverity::Error,
                        format!(
                            "{failed_label} finished without the task it belongs to; its result was not kept"
                        ),
                    );
                    None
                };
                let errors =
                    self.seal_failed_run(state.model_mut(), target_run_id, failed_analysis, None);
                self.report_seal_errors(state.model_mut(), errors);

                // Continue with remaining analyses (commercial behavior: don't abort batch)
                if !self.pending_analyses.is_empty() {
                    log::info!(
                        "Analysis failed, continuing with {} remaining",
                        self.pending_analyses.len()
                    );
                    self.start_next_analysis(state);
                } else {
                    state.model_mut().simulation.execution.status =
                        "Completed with errors".to_string();
                    self.finish_simulation_batch(state);
                }
            }
        }
        None
    }

    /// Keep a converted result in its run, show it, and go on to the run's
    /// next task or close the run.
    fn end_completion(
        &mut self,
        state: &mut impl RunHost,
        export_io: &(impl ExportWorkflowIo + ?Sized),
        head: CompletionHead,
        converted: Converted,
    ) {
        let CompletionHead {
            label: current_label,
            target_run_id,
            notices,
            produced_artifact,
            artifact_failed,
            mut prepared_touchstone_export,
            storage_stop,
            solved_transient,
            truncation_line,
        } = head;
        let Converted {
            analysis: mut analysis_result,
            shown,
        } = converted;
        // The Console has had the prose as the engine logged it; the
        // run's batch gets the conditions, one record each for the
        // whole run.
        if let Some(run_sequence) = target_run_id {
            let facts = state.slot_facts();
            state.model_mut().publish_run_notices(
                &current_label,
                notices,
                solved_transient
                    .as_deref()
                    .map(|transient| (transient, truncation_line)),
                &analysis_result.measurements,
                run_sequence,
                facts,
            );
            if let Some(stop) = &storage_stop {
                self.report_results_storage_stop(state, stop, run_sequence);
            }
            // What the run's checks reported is filed beside them,
            // one row each. A testbench that reported an error
            // completed all the same: its results stand, and the
            // run's check verdict, not its lifecycle, says it failed.
            if !analysis_result.checks.is_empty() {
                state.model_mut().publish_run_checks(
                    &current_label,
                    &analysis_result.checks,
                    run_sequence,
                    facts,
                );
            }
        }
        let retention_error = analysis_result.error_message.clone();
        let mut retained = None;
        if let Some(provenance) = self.current_provenance.take() {
            // One member of a Monte Carlo or .STEP transient is named
            // after the trial or point it solved, as its task is.
            if let Some(member) = provenance.family_member() {
                analysis_result.label = member.titled(&analysis_result.label);
            }
            let completed_instance = provenance.source_instance_id();
            let kept = self.retain_completed_analysis(
                state.model_mut(),
                target_run_id,
                analysis_result,
                provenance,
            );
            if kept.is_ok() {
                retained = Some(completed_instance);
            }
            match kept {
                Ok(true) => {
                    if let Some(artifact) = produced_artifact {
                        self.execution_artifacts
                            .insert(completed_instance, artifact);
                    }
                    if let Some(prepared) = prepared_touchstone_export.take() {
                        self.commit_touchstone_export(state.model_mut(), export_io, prepared);
                    }
                }
                // The artifact failure has its own line above.
                Ok(false) if artifact_failed => {}
                Ok(false) => {
                    self.run_line(
                        state.model_mut(),
                        LogSeverity::Error,
                        format!(
                            "{current_label} failed: {}",
                            retention_error
                                .as_deref()
                                .unwrap_or("its result was not kept")
                        ),
                    );
                }
                Err(error) => {
                    self.run_line(
                        state.model_mut(),
                        LogSeverity::Error,
                        format!("{current_label}: the result was not kept: {error}"),
                    );
                    let errors = self.seal_failed_run(state.model_mut(), target_run_id, None, None);
                    self.report_seal_errors(state.model_mut(), errors);
                    self.pending_analyses.clear();
                }
            }
        } else {
            self.run_line(
                state.model_mut(),
                LogSeverity::Error,
                format!(
                    "{current_label} finished without the task it belongs to; its result was not kept"
                ),
            );
            let errors = self.seal_failed_run(state.model_mut(), target_run_id, None, None);
            self.report_seal_errors(state.model_mut(), errors);
        }

        // Display the just-completed analysis from the view's copy built with
        // it, letting go of what the view showed away from the frame.
        match target_run_id {
            Some(run_id) => {
                let index = retained.and_then(|instance| {
                    state
                        .model()
                        .simulation
                        .retained
                        .run_by_sequence(run_id)?
                        .analyses
                        .iter()
                        .rposition(|analysis| {
                            analysis.provenance.as_ref().is_some_and(|provenance| {
                                provenance.source_instance_id() == instance
                            })
                        })
                });
                match (index, shown) {
                    (Some(index), Some(shown)) => {
                        let previous = state
                            .model_mut()
                            .simulation
                            .refresh_run_samples_with(run_id, index, shown);
                        self.release(previous);
                    }
                    (_, shown) => {
                        state.model_mut().simulation.refresh_run_samples(run_id);
                        self.release(shown);
                    }
                }
                self.view_current_for = Some(run_id);
            }
            None => self.release(shown),
        }

        // The run reached its plan's storage budget: the analyses it
        // has not started stop there, as on Stop, and what it kept
        // stands.
        if storage_stop
            .as_ref()
            .is_some_and(|stop| stop.budget.is_some())
            && !self.pending_analyses.is_empty()
        {
            let unstarted = self.pending_analyses.len();
            self.pending_analyses.clear();
            self.run_line(
                state.model_mut(),
                LogSeverity::Error,
                format!(
                    "{unstarted} {} not run: the run reached its plan's storage budget",
                    if unstarted == 1 {
                        "analysis was"
                    } else {
                        "analyses were"
                    }
                ),
            );
        }

        // =========================================================
        // Multi-analysis chaining: start next or finish batch
        // =========================================================
        if !self.pending_analyses.is_empty() {
            log::info!(
                "Starting next analysis ({} remaining)",
                self.pending_analyses.len()
            );
            self.start_next_analysis(state);
        } else {
            // All analyses complete - finalize the batch.
            self.finish_simulation_batch(state);
        }
    }

    /// What `error` says, in `state`'s words, when the run went past one of
    /// the engine's limits, or that the machine has not the memory for the
    /// run when the limit was one sized to it; `None` for every other
    /// failure.
    pub(super) fn limit_refusal(state: &impl RunHost, error: &SimulationError) -> Option<String> {
        let (device, resource, requested, limit) = match error {
            SimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => (None, resource, *requested, *limit),
            SimulationError::ResourceFailure(
                rspice_simulation_contract::resource_failure::ResourceFailure::DeviceLimit {
                    instance,
                    resource,
                    requested,
                    limit,
                },
            ) => (Some(instance.as_str()), resource, *requested, *limit),
            _ => return None,
        };
        let refusal = rspice_core::ResourceLimitError {
            resource: rspice_core::ResourceKind::from_name(resource)?,
            requested,
            limit,
        };
        let machine = rspice_simulation::execution::run_memory::machine_memory();
        match rspice_simulation::execution::run_memory::sized_refusal(&refusal, machine) {
            Some(words) => {
                Some(device.map_or_else(|| words.clone(), |at| format!("{at}: {words}")))
            }
            None => Some(state.resource_refusal(&refusal, device)),
        }
    }

    /// The console line for a deliberate refusal: the analysis the author
    /// ran, the instance it declined for, the reason, and the token —
    /// `Not supported in <label>: '<instance>' — <reason> [token]`. `None`
    /// for every failure that is not a refusal.
    pub(super) fn refusal_console_line(label: &str, error: &SimulationError) -> Option<String> {
        let SimulationError::UnsupportedCapability {
            capability,
            instance,
            message,
            ..
        } = error
        else {
            return None;
        };
        // The core rendering is `unsupported capability [token] ...: reason`.
        let reason = message
            .split_once(&format!("[{capability}]"))
            .and_then(|(_, rest)| rest.split_once(": "))
            .map_or(message.as_str(), |(_, reason)| reason);
        Some(match instance {
            Some(instance) => {
                format!("Not supported in {label}: '{instance}' \u{2014} {reason} [{capability}]")
            }
            None => format!("Not supported in {label}: {reason} [{capability}]"),
        })
    }
}

/// What a completed task said before its result was converted, kept for
/// when the converted result is.
pub(super) struct CompletionHead {
    label: String,
    target_run_id: Option<u64>,
    notices: Vec<(rspice_core::notice::RecordedNotice, Option<usize>)>,
    produced_artifact: Option<rspice_simulation::execution_artifact::ExecutionArtifactEnvelope>,
    artifact_failed: bool,
    prepared_touchstone_export: Option<super::touchstone::PreparedTouchstoneExport>,
    storage_stop: Option<crate::state::ResultsStorageStop>,
    solved_transient:
        Option<Arc<rspice_results::convergence_quality::TransientConvergenceEvidence>>,
    truncation_line: Option<usize>,
}

/// A completed task's result with what converting it into the run's
/// analysis takes.
pub(super) struct Conversion {
    result: crate::simulation::SimulationResult,
    analysis_type: AnalysisType,
    label: String,
    pnoise: Option<rspice_simulation::periodic::PnoiseRunConfig>,
    carrier_hz: Option<f64>,
    op_digest: Option<crate::product::ContentDigest>,
    artifact_failure: Option<String>,
    save_policy: crate::simulation::execution::SavePolicy,
    contracts: Vec<PreparedSavedOutput>,
    /// The outputs a manual deck plots, when it plots only those.
    deck_outputs: Option<crate::simulation::execution::DeckOutputs>,
}

/// A converted result, and the view's copy of its waveforms when that copy
/// is what the run will keep.
pub(super) struct Converted {
    analysis: AnalysisResult,
    shown: Option<ShownWaveforms>,
}

impl Conversion {
    /// The run's analysis of the result, `capture` reading it before the
    /// task's saved-output policy prunes it.
    fn convert(self, capture: impl FnOnce(&AnalysisResult)) -> Converted {
        let mut analysis =
            super::results_convert::analysis_of(self.result, self.analysis_type, &self.label);
        rspice_simulation::result_conversion::retain_periodic_noise_metadata(
            &mut analysis.data,
            self.pnoise.as_ref(),
            self.carrier_hz,
        );
        if let Some(AnalysisResultPayload::OperatingPoint {
            effective_source_content_digest,
            ..
        }) = analysis.result_payload.as_mut()
        {
            *effective_source_content_digest = self.op_digest;
        }
        if let Some(message) = self.artifact_failure {
            analysis.success = false;
            analysis.error_message = Some(message);
        }
        capture(&analysis);
        crate::simulation::output_contract::apply_saved_output_policy(
            &mut analysis,
            self.save_policy,
            &self.contracts,
        );
        if let Some(outputs) = &self.deck_outputs {
            for waveform in &mut analysis.waveforms {
                waveform.visible = outputs.plots(&waveform.name);
            }
        }
        // Samples held in memory move into the results store as the run
        // keeps them, so a copy of them now is not what the view would show.
        let shown = (!crate::state::run_store::holds_samples_in_memory(&analysis.data))
            .then(|| ShownWaveforms::new(analysis.waveforms.clone()));
        Converted { analysis, shown }
    }
}

/// A completion whose result is being converted away from the frame.
pub(super) struct Finishing {
    head: CompletionHead,
    converting: std::thread::JoinHandle<Converted>,
}

impl Finishing {
    /// Whether the result is converted, ready for a frame to keep.
    pub(super) fn is_converted(&self) -> bool {
        self.converting.is_finished()
    }
}

/// Convert `conversion` on a thread of its own; the frames of a run in
/// progress ask the thread for it. Without a thread, the conversion comes
/// back unconverted.
#[cfg(not(target_arch = "wasm32"))]
fn convert_off_frame(
    conversion: Conversion,
) -> Result<std::thread::JoinHandle<Converted>, Box<Conversion>> {
    // The thread takes the conversion from here, so a thread that could not
    // start leaves it to be handed back.
    let job = Arc::new(std::sync::Mutex::new(Some(Box::new(conversion))));
    let taken = Arc::clone(&job);
    let spawned = std::thread::Builder::new()
        .name("run completion".to_owned())
        .spawn(move || {
            let conversion = *taken
                .lock()
                .ok()
                .and_then(|mut job| job.take())
                .expect("a started thread holds its conversion");
            conversion.convert(|_| {})
        });
    spawned.map_err(|_| {
        job.lock()
            .ok()
            .and_then(|mut job| job.take())
            .expect("a thread that did not start leaves its conversion")
    })
}

/// The page has one thread: a result is converted on the frame.
#[cfg(target_arch = "wasm32")]
fn convert_off_frame(
    conversion: Conversion,
) -> Result<std::thread::JoinHandle<Converted>, Box<Conversion>> {
    Err(Box::new(conversion))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::controller::test_host::RunTestHost;

    /// An analysis the engine failed reaches the Problems page through the
    /// controller's own failure path: one finding, about that analysis in
    /// that run, in the engine's words. Nothing drove this path into the
    /// store before, so removing the publication broke no test.
    #[test]
    fn a_failed_analysis_is_published_as_a_finding_about_its_run() {
        let mut state = RunTestHost::default();
        let mut controller = SimulationController::new();
        let export_io = super::super::test_execution::MockExportWorkflowIo::default();
        let run_sequence = state.model.simulation.start_run().id;
        let run = state
            .model
            .simulation
            .retained
            .run_by_sequence_mut(run_sequence)
            .expect("the test run is retained");
        run.mark_running()
            .expect("the test run enters running state");
        state.model.simulation.execution.active_execution = run.execution_identity();
        controller.current_run_id = Some(run_sequence);
        controller.current_analysis_label = Some("Transient".to_owned());
        controller.current_provenance = Some(
            AnalysisResultProvenance::new(
                crate::product::AnalysisInstanceId::new(),
                crate::product::ObjectRevision::INITIAL,
                crate::product::ContentDigest::from_bytes([0x39; 32]),
                Vec::new(),
            )
            .expect("synthetic prepared-task provenance is valid"),
        );
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;

        controller.accept_completion(
            &mut state,
            &export_io,
            Err(SimulationError::SolverError("singular matrix".to_owned())),
        );

        let published = state.model.diagnostics.published_run_failures(run_sequence);
        assert_eq!(published.len(), 1, "{published:?}");
        assert_eq!(published[0].0, format!("Transient · run {run_sequence}"));
        assert!(
            published[0].1.contains("singular matrix"),
            "the finding states the engine's failure: {published:?}"
        );
    }

    /// A run whose transient carried `checks`, completed through the
    /// controller, and the run's sequence.
    fn complete_with_checks(
        checks: rspice_simulation::results::SimulationChecks,
    ) -> (RunTestHost, u64) {
        let mut state = RunTestHost::default();
        let mut controller = SimulationController::new();
        let export_io = super::super::test_execution::MockExportWorkflowIo::default();
        let run_sequence = state.model.simulation.start_run().id;
        let run = state
            .model
            .simulation
            .retained
            .run_by_sequence_mut(run_sequence)
            .expect("the test run is retained");
        run.mark_running()
            .expect("the test run enters running state");
        state.model.simulation.execution.active_execution = run.execution_identity();
        controller.current_run_id = Some(run_sequence);
        controller.current_analysis_label = Some("Transient".to_owned());
        controller.current_provenance = Some(
            AnalysisResultProvenance::new(
                crate::product::AnalysisInstanceId::new(),
                crate::product::ObjectRevision::INITIAL,
                crate::product::ContentDigest::from_bytes([0x3a; 32]),
                Vec::new(),
            )
            .expect("synthetic prepared-task provenance is valid"),
        );
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        let mut waveforms = std::collections::HashMap::new();
        waveforms.insert(
            "V(out)".to_owned(),
            rspice_simulation::results::SampledWaveform {
                name: "V(out)".to_owned(),
                x_values: Vec::new().into(),
                y_values: vec![0.0, 1.0].into(),
                y_unit: "V".to_owned(),
                is_complex: false,
                y_imag: None,
            },
        );
        let hdl_output = checks.hdl.clone();
        controller.accept_completion(
            &mut state,
            &export_io,
            Ok(crate::simulation::SimulationResult::Transient {
                spectra: Vec::new(),
                checks,
                hdl_output,
                time: vec![0.0, 1.0e-9].into(),
                waveforms,
                measurements: Vec::new(),
                periodic_state: None,
                convergence: Default::default(),
                events: Default::default(),
                store: None,
            }),
        );
        (state, run_sequence)
    }

    fn hdl_record(
        severity: rspice_core::config::HdlSeverity,
        time: f64,
        line: u32,
        text: &str,
    ) -> rspice_core::config::HdlOutputRecord {
        use rspice_core::config::{HdlOrigin, HdlOutputRecord, HdlPoint, HdlStream};
        HdlOutputRecord {
            point: HdlPoint::Transient { time },
            tick: 1,
            seconds: time,
            stream: HdlStream::Diagnostic { severity },
            text: format!("{text}\n"),
            origin: HdlOrigin {
                scope: "tb".to_owned(),
                file: "tb.v".to_owned(),
                line,
            },
        }
    }

    /// One Problems row: its code, level, scope, source line, message and
    /// route.
    type CheckRow = (
        String,
        rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticLevel,
        String,
        Option<String>,
        String,
        Option<rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticRoute>,
    );

    /// One row per check.
    fn check_rows(state: &RunTestHost, run_sequence: u64) -> Vec<CheckRow> {
        state
            .model
            .diagnostics
            .published_run_records(run_sequence)
            .into_iter()
            .map(|record| {
                (
                    record.canonical.code.to_string(),
                    record.level,
                    record.subject.object.clone(),
                    record.subject.source_line.clone(),
                    record.message.clone(),
                    record.route.clone(),
                )
            })
            .collect()
    }

    /// A testbench that called `$error` and `$warning`, violated an
    /// error-level `.ASSERT` and ended on `$fatal` gives exactly four
    /// Problems rows, each with its severity, its time, its scope and its
    /// source line. Its analysis completed: the run is Completed with its
    /// results kept, and its check verdict is Failed. The `$fatal` is one row
    /// although the engine reports it both as a record and as the finish.
    #[test]
    fn a_testbench_s_checks_are_one_problems_row_each_and_fail_its_verdict() {
        use rspice_app_state::workbench::app_state::diagnostic_store::{
            DiagnosticLevel, DiagnosticRoute,
        };
        use rspice_core::analysis::{AssertViolation, AssertionReport};
        use rspice_core::config::HdlSeverity;
        use rspice_core::netlist::{AssertAnalysis, AssertLevel, NetlistSourceLocation};
        use rspice_core::{ModelFinish, ModelFinishPoint, ModelFinishSource, ModelFinishTask};

        let checks = rspice_simulation::results::SimulationChecks {
            hdl: vec![
                hdl_record(HdlSeverity::Error, 2.0e-9, 9, "out stuck low"),
                hdl_record(HdlSeverity::Warning, 3.0e-9, 11, "slow edge"),
                hdl_record(HdlSeverity::Fatal, 5.0e-9, 14, "giving up"),
            ],
            assertions: vec![AssertionReport {
                name: "a".into(),
                analysis: AssertAnalysis::Tran,
                target: "V(out)".into(),
                instance: None,
                level: AssertLevel::Error,
                message: None,
                origin: NetlistSourceLocation::in_memory(5),
                violations: vec![AssertViolation {
                    start: Some(1.0e-9),
                    end: None,
                    extreme: Some(1.0),
                    at: Some(1.0e-9),
                }],
                unknown_stretches: 0,
                error: None,
                after_stop: None,
            }],
            finish: Some(ModelFinish {
                instance: "tb".into(),
                model: "tb".into(),
                site: 0,
                point: ModelFinishPoint::Transient { time: 5.0e-9 },
                diagnostic_level: 1,
                task: ModelFinishTask::Fatal,
                source: Some(ModelFinishSource {
                    file: "tb.v".into(),
                    line: 14,
                }),
                message: Some("giving up".into()),
            }),
        };
        let (state, run_sequence) = complete_with_checks(checks);

        let deck_line = Some(DiagnosticRoute::Repair(
            crate::state::PreflightRemediation::NetlistSource { line: Some(5) },
        ));
        assert_eq!(
            check_rows(&state, run_sequence),
            [
                (
                    "HDL-ERROR".to_owned(),
                    DiagnosticLevel::Error,
                    "tb".to_owned(),
                    Some("tb.v:9".to_owned()),
                    "$error at t = 2ns: out stuck low".to_owned(),
                    None,
                ),
                (
                    "HDL-WARNING".to_owned(),
                    DiagnosticLevel::Warning,
                    "tb".to_owned(),
                    Some("tb.v:11".to_owned()),
                    "$warning at t = 3ns: slow edge".to_owned(),
                    None,
                ),
                (
                    "HDL-FATAL".to_owned(),
                    DiagnosticLevel::Error,
                    "tb".to_owned(),
                    Some("tb.v:14".to_owned()),
                    "$fatal at t = 5ns: giving up".to_owned(),
                    None,
                ),
                (
                    "ASSERT-VIOLATED".to_owned(),
                    DiagnosticLevel::Error,
                    "a".to_owned(),
                    Some("line 5".to_owned()),
                    "1 violation, first [1ns, open), extreme 1 at 1ns".to_owned(),
                    deck_line,
                ),
            ]
        );
        let run = state
            .model
            .simulation
            .retained
            .run_by_sequence(run_sequence)
            .expect("the run is retained");
        assert_eq!(
            run.lifecycle,
            crate::state::SimulationRunLifecycle::Completed
        );
        assert!(run.success, "the analysis completed");
        assert!(run.analyses[0].success, "its results stay available");
        assert_eq!(
            run.check_verdict(),
            Some(rspice_results::analysis_result::CheckVerdict::Failed)
        );
    }

    /// A run whose checks reported nothing files no row and claims no
    /// verdict: having checked nothing, it has passed nothing.
    #[test]
    fn a_run_without_checks_files_no_row_and_claims_no_verdict() {
        let (state, run_sequence) = complete_with_checks(Default::default());
        assert!(check_rows(&state, run_sequence).is_empty());
        let run = state
            .model
            .simulation
            .retained
            .run_by_sequence(run_sequence)
            .expect("the run is retained");
        assert_eq!(
            run.lifecycle,
            crate::state::SimulationRunLifecycle::Completed
        );
        assert_eq!(run.check_verdict(), None);
    }

    /// Warnings alone warn: the rows are warnings and the verdict is Warned.
    /// A `$finish` that ended the run is no row and does not fail it.
    #[test]
    fn warnings_alone_give_a_warned_verdict() {
        use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticLevel;
        use rspice_core::config::HdlSeverity;
        use rspice_core::{ModelFinish, ModelFinishPoint, ModelFinishTask};

        let checks = rspice_simulation::results::SimulationChecks {
            hdl: vec![hdl_record(HdlSeverity::Warning, 3.0e-9, 11, "slow edge")],
            assertions: Vec::new(),
            finish: Some(ModelFinish {
                instance: "tb".into(),
                model: "tb".into(),
                site: 0,
                point: ModelFinishPoint::Transient { time: 6.0e-9 },
                diagnostic_level: 1,
                task: ModelFinishTask::Finish,
                source: None,
                message: None,
            }),
        };
        let (state, run_sequence) = complete_with_checks(checks);
        let rows = check_rows(&state, run_sequence);
        assert_eq!(rows.len(), 1, "{rows:?}");
        assert_eq!(rows[0].1, DiagnosticLevel::Warning);
        let run = state
            .model
            .simulation
            .retained
            .run_by_sequence(run_sequence)
            .expect("the run is retained");
        assert_eq!(
            run.check_verdict(),
            Some(rspice_results::analysis_result::CheckVerdict::Warned)
        );
    }
}
