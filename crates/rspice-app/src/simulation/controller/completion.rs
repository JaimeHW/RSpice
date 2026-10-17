//! Application reaction to a completed simulation task.

use super::*;

impl SimulationController {
    pub(super) fn accept_completion(
        &mut self,
        state: &mut AppState,
        export_io: &(impl ExportWorkflowIo + ?Sized),
        result: Result<crate::simulation::SimulationResult, SimulationError>,
    ) {
        // What the task raised and went on past. Taken on every outcome, so
        // an aborted task's notices cannot be read as the next task's.
        let notices = self.runner.drain_engine_notices();
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
                self.run_line(state, LogSeverity::Info, figures);

                // A short FFT record is a successful result with no
                // spectrum. It is stated in the run's log as well as on the
                // sheet, because the reader is looking at the run here.
                if let crate::simulation::SimulationResult::Fft { spectrum, .. } = &sim_result
                    && let Some(notice) =
                        rspice_simulation::result_conversion::incomplete_history_notice(spectrum)
                {
                    self.run_line(state, LogSeverity::Warning, notice);
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
                        "{current_label}: the result could not be passed to the analyses that depend on it: {error}"
                    ));
                let (produced_artifact, artifact_failure) = match produced_artifact {
                    Ok(artifact) => (artifact, None),
                    Err(message) => {
                        self.run_line(state, LogSeverity::Error, message.clone());
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
                            self.run_line(
                                state,
                                LogSeverity::Warning,
                                format!("Touchstone export skipped: {error}"),
                            );
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
                // A recorded FFT carries the quality of the transient it
                // transformed, which that transient has already reported.
                let solved_transient = match &sim_result {
                    crate::simulation::SimulationResult::Fft { .. } => None,
                    solved => solved.transient_convergence().cloned(),
                };
                let testbench_verdict = testbench_errors(&sim_result);
                if let Some(quality) = sim_result.transient_convergence()
                    && quality.has_lte_exceptions()
                {
                    self.run_line(
                        state,
                        LogSeverity::Warning,
                        format!(
                            "{current_label}: accepted points exceeded the local truncation-error criterion. Review convergence details in the result inspector."
                        ),
                    );
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
                let artifact_failed = artifact_failure.is_some();
                if let Some(message) = artifact_failure {
                    analysis_result.success = false;
                    analysis_result.error_message = Some(message);
                }
                self.materialize_current_saved_outputs(&mut analysis_result);
                if analysis_result.analysis_type == AnalysisType::Transient {
                    self.populate_transient_post_views(state, &analysis_result);
                }
                // The Console has had the prose as the engine logged it; the
                // run's batch gets the conditions, one record each for the
                // whole run.
                if let Some(run_sequence) = target_run_id {
                    state.publish_run_notices(
                        &current_label,
                        notices,
                        solved_transient.as_deref(),
                        &analysis_result.measurements,
                        run_sequence,
                    );
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
                                self.commit_touchstone_export(state, export_io, prepared);
                            }
                            if let Some((yield_results, provenance)) =
                                prepared_yield_evidence.take()
                            {
                                state
                                    .simulation
                                    .replace_yield_evidence(yield_results, provenance);
                            }
                            if let Some(verdict) = testbench_verdict.as_deref() {
                                self.fail_testbench_verdict(
                                    state,
                                    target_run_id,
                                    &current_label,
                                    verdict,
                                );
                            }
                        }
                        // The artifact failure has its own line above.
                        Ok(false) if artifact_failed => {}
                        Ok(false) => {
                            self.run_line(
                                state,
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
                                state,
                                LogSeverity::Error,
                                format!("{current_label}: the result was not kept: {error}"),
                            );
                            let errors = self.seal_failed_run(state, target_run_id, None, None);
                            self.report_seal_errors(state, errors);
                            self.pending_analyses.clear();
                        }
                    }
                } else {
                    self.run_line(
                        state,
                        LogSeverity::Error,
                        format!(
                            "{current_label} finished without the task it belongs to; its result was not kept"
                        ),
                    );
                    let errors = self.seal_failed_run(state, target_run_id, None, None);
                    self.report_seal_errors(state, errors);
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
                if let Some(run_sequence) = self.current_run_id {
                    super::run_lines::announce_run_end(state, run_sequence);
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
                self.current_run_id = None;
                self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
                self.current_analysis_idx = 0;
                self.total_analyses = 0;
                state.simulation.execution.active_execution = None;
                state.simulation.execution.abort_request = None;
                state.ui.netlist.pending_manual_run_id = None;
                state.ui.netlist.pending_run_buffer = None;
                state.simulation.execution.status = "Aborted".to_string();
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
                let console_line = Self::refusal_console_line(&failed_label, &e)
                    .unwrap_or_else(|| format!("{failed_label} failed: {e}"));
                Self::report_failed_analysis(
                    state,
                    &failed_label,
                    &console_line,
                    &attribution,
                    self.target_run_id(state),
                );
                // A clamp the analysis raised before it failed is part of
                // the account of the failure.
                if let Some(run_sequence) = self.target_run_id(state) {
                    state.publish_run_notices(&failed_label, notices, None, &[], run_sequence);
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
                let target_run_id = self.target_run_id(state);
                let failed_analysis = if let Some(provenance) = self.current_provenance.take() {
                    let mut analysis = self.partial_failure_analysis(
                        failed_type,
                        &failed_label,
                        e.to_string(),
                        provenance,
                    );
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
                        state,
                        LogSeverity::Error,
                        format!(
                            "{failed_label} finished without the task it belongs to; its result was not kept"
                        ),
                    );
                    None
                };
                let errors = self.seal_failed_run(state, target_run_id, failed_analysis, None);
                self.report_seal_errors(state, errors);

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

    /// A testbench that reported errors did not pass, though its analysis
    /// completed: the run is sealed as not passing, its results stay
    /// available, and the Problems page files the testbench's verdict against
    /// the run, as the CLI fails the run on the same records.
    fn fail_testbench_verdict(
        &self,
        state: &mut AppState,
        target_run_id: Option<u64>,
        analysis: &str,
        verdict: &str,
    ) {
        let errors = self.seal_failed_run(state, target_run_id, None, None);
        self.report_seal_errors(state, errors);
        if let Some(run_sequence) = target_run_id {
            state.publish_run_failure(analysis, verdict, None, run_sequence);
        }
    }
}

/// The verdict of a transient's testbench, from the records its `$error` and
/// `$fatal` calls left in the result: `Testbench reported 2 errors: <the
/// first one's message>`. `None` when it reported none.
///
/// Read off the result rather than the run's log, whose bound can withhold
/// the very lines that failed the testbench.
fn testbench_errors(result: &crate::simulation::SimulationResult) -> Option<String> {
    use rspice_core::config::{HdlSeverity, HdlStream};

    let crate::simulation::SimulationResult::Transient { hdl_output, .. } = result else {
        return None;
    };
    let mut errors = hdl_output.iter().filter(|record| {
        matches!(
            record.stream,
            HdlStream::Diagnostic {
                severity: HdlSeverity::Error | HdlSeverity::Fatal
            }
        )
    });
    let first = errors.next()?;
    let count = 1 + errors.count();
    Some(format!(
        "Testbench reported {count} {}: {}",
        if count == 1 { "error" } else { "errors" },
        first.text.trim_end()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An analysis the engine failed reaches the Problems page through the
    /// controller's own failure path: one finding, about that analysis in
    /// that run, in the engine's words. Nothing drove this path into the
    /// store before, so removing the publication broke no test.
    #[test]
    fn a_failed_analysis_is_published_as_a_finding_about_its_run() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = super::super::tests::MockExportWorkflowIo::default();
        let run_sequence = state.simulation.start_run().id;
        let run = state
            .simulation
            .retained
            .run_by_sequence_mut(run_sequence)
            .expect("the test run is retained");
        run.mark_running()
            .expect("the test run enters running state");
        state.simulation.execution.active_execution = run.execution_identity();
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

        let published = state.diagnostics.published_run_failures(run_sequence);
        assert_eq!(published.len(), 1, "{published:?}");
        assert_eq!(published[0].0, format!("Transient · run {run_sequence}"));
        assert!(
            published[0].1.contains("singular matrix"),
            "the finding states the engine's failure: {published:?}"
        );
    }

    /// A transient whose testbench called `$error` and `$fatal` completes
    /// with errors: its results are kept, the run does not pass, and the
    /// Problems page files the testbench's verdict against the run.
    #[test]
    fn a_testbench_that_reported_errors_fails_its_run_and_keeps_its_results() {
        use rspice_core::config::{HdlOrigin, HdlOutputRecord, HdlPoint, HdlSeverity, HdlStream};
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = super::super::tests::MockExportWorkflowIo::default();
        let run_sequence = state.simulation.start_run().id;
        let run = state
            .simulation
            .retained
            .run_by_sequence_mut(run_sequence)
            .expect("the test run is retained");
        run.mark_running()
            .expect("the test run enters running state");
        state.simulation.execution.active_execution = run.execution_identity();
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
        let record = |stream, text: &str| HdlOutputRecord {
            point: HdlPoint::Transient { time: 1.0e-9 },
            tick: 1,
            seconds: 1.0e-9,
            stream,
            text: text.to_owned(),
            origin: HdlOrigin {
                scope: "tb".to_owned(),
                file: "tb.v".to_owned(),
                line: 9,
            },
        };
        let mut waveforms = std::collections::HashMap::new();
        waveforms.insert(
            "V(out)".to_owned(),
            rspice_simulation::results::WaveformData {
                name: "V(out)".to_owned(),
                x_values: Vec::new(),
                y_values: vec![0.0, 1.0],
                y_unit: "V".to_owned(),
                is_complex: false,
                y_imag: None,
            },
        );

        controller.accept_completion(
            &mut state,
            &export_io,
            Ok(crate::simulation::SimulationResult::Transient {
                spectra: Vec::new(),
                hdl_output: vec![
                    record(HdlStream::Stdout, "edge 1\n"),
                    record(
                        HdlStream::Diagnostic {
                            severity: HdlSeverity::Error,
                        },
                        "out stuck low\n",
                    ),
                    record(
                        HdlStream::Diagnostic {
                            severity: HdlSeverity::Fatal,
                        },
                        "giving up\n",
                    ),
                ],
                time: vec![0.0, 1.0e-9],
                waveforms,
                measurements: Vec::new(),
                periodic_state: None,
                convergence: Default::default(),
                events: Default::default(),
            }),
        );

        let run = state
            .simulation
            .retained
            .run_by_sequence(run_sequence)
            .expect("the run is retained");
        assert!(!run.success, "the run did not pass");
        assert_eq!(run.analyses.len(), 1);
        assert!(run.analyses[0].success, "its results stay available");
        assert_eq!(
            state.diagnostics.published_run_failures(run_sequence),
            [(
                format!("Transient · run {run_sequence}"),
                "Testbench reported 2 errors: out stuck low".to_owned()
            )]
        );
        let summary = &run.log.lines().last().expect("a closed log").message;
        assert!(
            summary.starts_with(&format!("Run {run_sequence} completed with errors in ")),
            "{summary}"
        );
    }
}
