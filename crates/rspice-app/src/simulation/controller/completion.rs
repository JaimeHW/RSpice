//! Application reaction to a completed simulation task.

use super::*;

impl SimulationController {
    pub(super) fn accept_completion(
        &mut self,
        state: &mut AppState,
        export_io: &(impl ExportWorkflowIo + ?Sized),
        result: Result<crate::simulation::SimulationResult, SimulationError>,
    ) {
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

                // A short FFT record is a successful result with no
                // spectrum. It is stated on the Console as well as on the
                // sheet, because the reader is looking at the run here.
                if let crate::simulation::SimulationResult::Fft { spectrum, .. } = &sim_result
                    && let Some(notice) =
                        rspice_simulation::result_conversion::incomplete_history_notice(spectrum)
                {
                    state.push_sim_message(ConsoleMessage::warning(notice));
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
                let target_run_id = self.target_run_id(state);

                let produced_artifact = self.current_artifact_producer.as_ref()
                    .map_or(Ok(None), |producer| {
                        producer.capture(&sim_result, &self.pending_analyses)
                    })
                    .map_err(|error| format!(
                        "Result could not produce its authenticated dependency artifact: {error}"
                    ));
                let (produced_artifact, artifact_failure) = match produced_artifact {
                    Ok(artifact) => (artifact, None),
                    Err(message) => {
                        log::error!("{message}");
                        state.push_sim_message(ConsoleMessage::error(message.clone()));
                        (None, Some(message))
                    }
                };

                // Prepare external and derived evidence while the raw
                // solver result is available, but publish neither until
                // the immutable analysis has crossed the exact run's
                // retention boundary successfully.
                let mut prepared_touchstone_export = target_run_id.and_then(|run_id| {
                    match self.prepare_touchstone_export(&sim_result, run_id) {
                        Ok(prepared) => prepared,
                        Err(error) => {
                            state.push_sim_message(ConsoleMessage::warning(format!(
                                "Touchstone export skipped: {error}"
                            )));
                            None
                        }
                    }
                });
                let mut prepared_yield_evidence = self
                    .yield_manager
                    .analyze_monte_carlo(&sim_result)
                    .map(|yield_results| {
                        let provenance = target_run_id
                            .and_then(|run_sequence| {
                                state.simulation.retained.run_by_sequence(run_sequence)
                            })
                            .and_then(|run| {
                                rspice_simulation::results::yield_provenance_from_monte_carlo_result(
                                    run.run_id,
                                    run.dataset_id,
                                    &sim_result,
                                )
                            });
                        (yield_results.values().cloned().collect(), provenance)
                    });

                self.apply_result_side_effects(state, &sim_result);
                if let Some(quality) = sim_result.transient_convergence()
                    && quality.has_lte_exceptions()
                {
                    state.push_sim_message(ConsoleMessage::warning(format!(
                        "{current_label}: accepted points exceeded the local truncation-error criterion. Review convergence details in the result inspector."
                    )));
                }

                let mut analysis_result = if let Some(config) = &self.current_config {
                    self.convert_to_analysis_result_owned(sim_result, config)
                } else {
                    self.convert_to_analysis_result_with_metadata_owned(
                        sim_result,
                        analysis_type,
                        &current_label,
                    )
                };
                self.retain_periodic_noise_result_metadata(&mut analysis_result);
                if let Some(AnalysisResultPayload::OperatingPoint {
                    effective_source_content_digest,
                    ..
                }) = analysis_result.result_payload.as_mut()
                {
                    *effective_source_content_digest =
                        self.current_op_effective_source_content_digest;
                }
                if let Some(message) = artifact_failure {
                    analysis_result.success = false;
                    analysis_result.error_message = Some(message);
                }
                self.materialize_current_saved_outputs(&mut analysis_result);
                if analysis_result.analysis_type == AnalysisType::Transient {
                    self.populate_transient_post_views(state, &analysis_result);
                }
                let retention_error = analysis_result.error_message.clone();
                if let Some(provenance) = self.current_provenance.take() {
                    let completed_instance = provenance.source_instance_id();
                    match self.retain_completed_analysis(
                        state,
                        target_run_id,
                        analysis_result,
                        provenance,
                    ) {
                        Ok(true) => {
                            if let Some(artifact) = produced_artifact {
                                self.execution_artifacts
                                    .insert(completed_instance, artifact);
                            }
                            if let Some(prepared) = prepared_touchstone_export.take() {
                                Self::commit_touchstone_export(state, export_io, prepared);
                            }
                            if let Some((yield_results, provenance)) =
                                prepared_yield_evidence.take()
                            {
                                state
                                    .simulation
                                    .replace_yield_evidence(yield_results, provenance);
                            }
                            if self.total_analyses > 1 {
                                state.push_sim_message(ConsoleMessage::info(format!(
                                    "{} completed ({}/{})",
                                    current_label, self.current_analysis_idx, self.total_analyses
                                )));
                            }
                        }
                        Ok(false) => {
                            state.push_sim_message(ConsoleMessage::error(format!(
                                "{} result retention failed: {}",
                                current_label,
                                retention_error
                                    .as_deref()
                                    .unwrap_or("unknown retention error")
                            )));
                        }
                        Err(error) => {
                            log::error!("{error}");
                            state.push_sim_message(ConsoleMessage::error(error));
                            let errors = self.seal_failed_run(state, target_run_id, None, None);
                            Self::report_seal_errors(state, errors);
                            self.pending_analyses.clear();
                        }
                    }
                } else {
                    let message = format!(
                        "Internal error: completed {} has no prepared-task provenance",
                        current_label
                    );
                    log::error!("{message}");
                    state.push_sim_message(ConsoleMessage::error(message));
                    let errors = self.seal_failed_run(state, target_run_id, None, None);
                    Self::report_seal_errors(state, errors);
                }

                // Display the just-completed analysis without rebuilding waveform buffers.
                if let Some(run_id) = target_run_id {
                    state
                        .simulation
                        .select_latest_analysis_in_run_sequence(run_id);
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
                self.seal_aborted_run(state, partial.take());
                self.pending_analyses.clear();
                self.successful_analysis_instances.clear();
                self.execution_artifacts.clear();
                self.point_families.clear();
                self.cached_netlist = None;
                self.current_config = None;
                self.current_spec = None;
                self.current_analysis_label = None;
                self.current_spec_options = None;
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
                self.current_run_id = None;
                self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
                self.current_analysis_idx = 0;
                self.total_analyses = 0;
                state.simulation.execution.active_execution = None;
                state.simulation.execution.abort_request = None;
                state.ui.netlist.pending_manual_run_id = None;
                state.ui.netlist.pending_run_buffer = None;
                state.simulation.execution.status = "Aborted".to_string();
                state.push_sim_message(ConsoleMessage::warning(
                    "Simulation aborted by user".to_owned(),
                ));
                self.complete_campaign_member(state, false);
            }
            Err(e) => {
                let attribution = e.attribution().cloned();
                Self::report_failed_analysis(state, &format!("Analysis failed: {e}"), &attribution);

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
                let target_run_id = self.target_run_id(state);
                let failed_analysis = if let Some(provenance) = self.current_provenance.take() {
                    let mut analysis = self.partial_failure_analysis(
                        failed_type,
                        &failed_label,
                        e.to_string(),
                        provenance,
                    );
                    analysis.failure_attribution = attribution;
                    self.materialize_current_saved_outputs(&mut analysis);
                    Some(analysis)
                } else {
                    let message = "Internal error: failed analysis has no prepared-task provenance";
                    log::error!("{message}");
                    state.push_sim_message(ConsoleMessage::error(message.to_owned()));
                    None
                };
                let errors = self.seal_failed_run(state, target_run_id, failed_analysis, None);
                Self::report_seal_errors(state, errors);

                // Continue with remaining analyses (commercial behavior: don't abort batch)
                if !self.pending_analyses.is_empty() {
                    log::info!(
                        "Analysis failed, continuing with {} remaining",
                        self.pending_analyses.len()
                    );
                    self.start_next_analysis(state);
                } else {
                    state.simulation.execution.status = "Completed with errors".to_string();
                    self.finish_simulation_batch(state);
                }
            }
        }
    }
}
