//! Preflight over a saved analysis plan.
//!
//! Checks that a stored plan still describes something the current design
//! can run — that its saved outputs still exist, and that its analyses are
//! still available — before a run is allowed to start on it.

use super::*;
use std::collections::HashMap;

use crate::simulation::execution::{PreparedDependencyBinding, PreparedTask, bound_cards};
use crate::simulation::plan::{AnalysisKind, FrozenSimulationPlan};

impl SimulationController {
    /// Compile a candidate saved output through the same frozen-plan and
    /// prepared-task path used by run preflight. This intentionally does not
    /// consult mutable draft rows after the task specs have been prepared.
    pub fn saved_output_preflight(
        &self,
        state: &AppState,
        output: &crate::state::SavedOutput,
    ) -> crate::simulation::SavedOutputPreflightReport {
        self.saved_outputs_preflight(state, std::slice::from_ref(output))
            .pop()
            .expect("single-output preflight always returns one report")
    }

    /// Compile an output table against one frozen plan/task projection. Model
    /// source sealing and spec construction happen once, independent of the
    /// number of rows rendered by Simulation Studio.
    pub fn saved_outputs_preflight(
        &self,
        state: &AppState,
        outputs: &[crate::state::SavedOutput],
    ) -> Vec<crate::simulation::SavedOutputPreflightReport> {
        if outputs.is_empty() {
            return Vec::new();
        }
        let plan = match self.build_analysis_plan(state) {
            Ok(plan) => plan,
            Err(errors) => {
                return invalid_saved_output_reports(outputs.len(), errors.join("; "));
            }
        };
        // Rendering an output table is not authorization to run, so a project
        // without an attached technology compiles it from the model library.
        let sealed_sources = if state.project_technology_in_effect() {
            state.seal_project_execution_model_sources()
        } else {
            state
                .model_library_manager
                .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
        };
        let sealed_models = match sealed_sources {
            Ok(sealed) => sealed,
            Err(error) => {
                return invalid_saved_output_reports(outputs.len(), error);
            }
        };
        let tasks = match self.build_queue_from_plan(state, &plan, &sealed_models) {
            Ok(tasks) => tasks,
            Err(errors) => {
                return invalid_saved_output_reports(outputs.len(), errors.join("; "));
            }
        };
        outputs
            .iter()
            .map(|output| {
                crate::simulation::output_contract::preflight_saved_output(
                    output,
                    tasks
                        .iter()
                        .map(|task| (task.instance_id(), &task.queued_analysis().spec)),
                )
            })
            .collect()
    }

    pub(super) fn build_analysis_plan(
        &self,
        state: &AppState,
    ) -> Result<FrozenSimulationPlan, Vec<String>> {
        let plan = state.sim_setup.analysis_plan.as_ref().ok_or_else(|| {
            vec![
                "The simulation plan has not been migrated to stable analysis instances".to_owned(),
            ]
        })?;
        plan.freeze().map_err(|error| vec![error.to_string()])
    }

    pub(super) fn build_queue_from_plan(
        &self,
        state: &AppState,
        plan: &FrozenSimulationPlan,
        sealed_model_sources: &crate::state::model_library::SealedModelExecutionSources,
    ) -> Result<Vec<PreparedTask>, Vec<String>> {
        let mut queue = Vec::with_capacity(plan.instances().len());
        let mut errors = Vec::new();
        // Remaining legacy engine builders still read the retired singleton
        // setup view. Clone once, then project each frozen instance (and its
        // exact bound prerequisites) into that short-lived view.
        // The live state and frozen plan remain untouched.
        let mut projected_state = state.clone();

        for instance in plan.instances() {
            if let Some(reason) = instance.kind().execution_blocker() {
                errors.push(format!("{}: {reason}", instance.display_name()));
                continue;
            }
            projected_state.sim_setup =
                match state.sim_setup.frozen_instance_projection(plan, instance) {
                    Ok(projection) => projection,
                    Err(error) => {
                        errors.push(format!("{}: {error}", instance.display_name()));
                        continue;
                    }
                };
            let dependency_ids = instance
                .dependencies()
                .iter()
                .map(|dependency| dependency.target())
                .collect();

            // Projected, like every other builder below it, and resolved
            // through the one owner. The three steps a directive is built from
            // — preview spec, legacy-index fallback, spice line — are one
            // derivation, and this loop wrote the first two out a second time:
            // the queue and the statement the plan displays beside it were two
            // spellings of the same thing. `analysis_draft_spec` is that pair,
            // and `analysis_draft_directive` is it plus the line. Exact preview
            // builders read the draft; the remaining fallback builders read
            // the projection. Handing a fallback the unprojected state made
            // the queue capable of dispatching a directive the plan never
            // displayed and the parse ratchet never read.
            let spec = match self.analysis_draft_spec(&projected_state, instance.draft()) {
                Ok(spec) => spec,
                Err(error) => {
                    errors.push(format!("{}: {error}", instance.display_name()));
                    continue;
                }
            };
            let analysis_line =
                match self.analysis_spec_to_spice_line(&projected_state, instance.draft(), &spec) {
                    Ok(line) => line,
                    Err(e) => {
                        errors.push(format!("{}: {}", instance.display_name(), e));
                        continue;
                    }
                };
            let mut spec_options = match self.analysis_spec_execution_options(
                &projected_state,
                instance.draft(),
                &spec,
                sealed_model_sources,
            ) {
                Ok(opts) => opts,
                Err(e) => {
                    errors.push(format!("{}: {}", instance.display_name(), e));
                    continue;
                }
            };

            if instance.draft().pvt_base_analysis().is_some() {
                let mode = spec_options
                    .temp
                    .as_mut()
                    .map(|config| &mut config.base_mode)
                    .or_else(|| {
                        spec_options
                            .corner
                            .as_mut()
                            .map(|config| &mut config.base_mode)
                    });
                if let Some(mode @ crate::services::simulation_runner::CornerBaseMode::Op) = mode {
                    let config = self
                        .build_legacy_analysis_spec_for_index(&projected_state, 0)
                        .and_then(|spec| self.analysis_spec_to_config(&projected_state, &spec));
                    match config {
                        Ok(AnalysisConfig::DcOp(config)) => {
                            *mode =
                                crate::services::simulation_runner::CornerBaseMode::ConfiguredOp(
                                    Box::new(config),
                                );
                        }
                        Ok(_) => {
                            unreachable!("the operating-point builder returns an OP configuration")
                        }
                        Err(error) => {
                            errors.push(format!("{}: {error}", instance.display_name()));
                            continue;
                        }
                    }
                }
            }

            if matches!(
                instance.draft(),
                crate::simulation::plan::AnalysisDraft::MonteCarlo(_)
                    | crate::simulation::plan::AnalysisDraft::Optimization(_)
            ) {
                match self.compile_study_base(state, plan, instance.draft()) {
                    Ok(base) => spec_options.study_base = base,
                    Err(error) => {
                        errors.push(format!("{}: {error}", instance.display_name()));
                        continue;
                    }
                }
            }

            let monte_carlo_resumes =
                if let crate::simulation::plan::AnalysisDraft::MonteCarlo(draft) = instance.draft()
                {
                    match draft.to_config().and_then(|config| {
                        monte_carlo_checkpoint::resume_inputs_from_config(
                            state,
                            config.checkpoint.as_ref(),
                        )
                    }) {
                        Ok(resumes) => resumes,
                        Err(error) => {
                            errors.push(format!("{}: {error}", instance.display_name()));
                            continue;
                        }
                    }
                } else {
                    Vec::new()
                };

            // A PSS request that asks to retain harmonics earns a second
            // prepared task for them. It is a task in its own right, with its
            // own identity and config digest, rather than a second result
            // smuggled out of the PSS task: harmonics are indexed by frequency
            // and the periodic waveform by time, so one analysis cannot carry
            // both, and aliasing the PSS task's authored identity would make
            // `find_analysis_by_source_instance` resolve the spectrum in its
            // place for every dependent analysis and retained pane binding.
            let spectrum_seed = match &spec {
                AnalysisSpec::Pss { num_harmonics, .. } if *num_harmonics > 0 => Some((
                    *num_harmonics,
                    analysis_line.clone(),
                    spec_options.clone(),
                    instance.id(),
                    instance.display_name().to_owned(),
                )),
                _ => None,
            };

            // The analysis's own numerics travel with the task rather than
            // being resolved here: snapshot preparation owns the seam where a
            // task's deck is written, and it is the only place that can splice
            // them after per-point expansion has chosen that deck.
            let numeric_override = if instance.kind().inherits_periodic_solver_options() {
                let carriers = instance
                    .dependencies()
                    .iter()
                    .filter_map(|edge| {
                        plan.instances().iter().find(|producer| {
                            producer.id() == edge.target()
                                && matches!(
                                    producer.kind(),
                                    AnalysisKind::Pss
                                        | AnalysisKind::HarmonicBalance
                                        | AnalysisKind::Qpss
                                )
                        })
                    })
                    .collect::<Vec<_>>();
                let [carrier] = carriers.as_slice() else {
                    errors.push(format!(
                        "{} requires exactly one bound periodic producer for its solver options",
                        instance.display_name()
                    ));
                    continue;
                };
                // Reusing an orbit requires its authenticated numerical
                // circuit, including every producer-local .OPTIONS package.
                // Consumer response controls travel separately in its spec.
                carrier.numeric_override().cloned()
            } else if let Some(base_id) = instance.draft().pvt_base_analysis() {
                let base = plan
                    .instances()
                    .iter()
                    .find(|base| base.id() == base_id)
                    .expect("PVT base was resolved by frozen_instance_projection");
                let mut options = instance.numeric_override().cloned().unwrap_or_default();
                if let Some(base_options) = base.numeric_override() {
                    options = options.with_base_options(base_options);
                }
                (!options.is_empty()).then_some(options)
            } else {
                instance.numeric_override().cloned()
            };

            let task = if Self::executes_via_spec(&spec) {
                QueuedAnalysis {
                    spec,
                    config: None,
                    spec_options,
                    analysis_line,
                    numeric_override: numeric_override.clone(),
                }
            } else {
                match self.analysis_spec_to_config(&projected_state, &spec) {
                    Ok(config) => {
                        if let Err(errs) = config.validate() {
                            errors.push(format!(
                                "{} config is invalid: {}",
                                instance.display_name(),
                                errs.join(", ")
                            ));
                            continue;
                        } else {
                            QueuedAnalysis {
                                spec,
                                config: Some(config),
                                spec_options,
                                analysis_line,
                                numeric_override: numeric_override.clone(),
                            }
                        }
                    }
                    Err(e) => {
                        errors.push(format!("{}: {}", instance.display_name(), e));
                        continue;
                    }
                }
            };
            let run_at = instance.run_at().clone();
            let mut prepared = PreparedTask::new(
                instance.id(),
                plan.revision(),
                dependency_ids,
                instance.display_name(),
                task,
            )
            .with_run_at(run_at.clone())
            .with_monte_carlo_resumes(monte_carlo_resumes);
            if instance.kind() == crate::simulation::plan::AnalysisKind::SParameter {
                match prepared_run::touchstone_export_policy_for_dialog(
                    &projected_state.sim_setup.sp,
                    &crate::simulation::placed_sources::placed_rf_ports(&state.schematic, None),
                    state.schematic.current_file.as_deref(),
                ) {
                    Ok(policy) => {
                        prepared = prepared.with_touchstone_export_policy(policy);
                    }
                    Err(error) => {
                        errors.push(format!("{}: {error}", instance.display_name()));
                        continue;
                    }
                }
            }
            queue.push(prepared);

            if let Some((num_harmonics, analysis_line, spec_options, producer, label)) =
                spectrum_seed
            {
                queue.push(
                    PreparedTask::derived(
                        // Derived from the PSS it reads, not minted: the prepared
                        // snapshot digest covers task identity, so a fresh id
                        // would make an unchanged plan prepare differently every
                        // time and expire its own authorization at dispatch. The
                        // derivation travels on the task so a saved project can
                        // re-derive it from the PSS the plan authored.
                        producer,
                        crate::simulation::execution::PSS_SPECTRUM_ROLE,
                        plan.revision(),
                        vec![producer],
                        format!("{label} Spectrum"),
                        QueuedAnalysis {
                            spec: AnalysisSpec::PssSpectrum { num_harmonics },
                            config: None,
                            spec_options,
                            analysis_line,
                            // The spectrum is the same authored PSS solve read at
                            // a different index, so it resolves under the same
                            // numerics; a second task under the plan policy would
                            // report harmonics of a solve that never happened.
                            numeric_override,
                        },
                    )
                    // And at the same points. A spectrum that ran everywhere while
                    // the PSS it reads ran at one point would be asking for
                    // harmonics of solves that were never dispatched.
                    .with_run_at(run_at),
                );
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            // Before the producer identities are taken, because attaching a
            // card to a transient changes that transient's payload digest —
            // and the bindings below capture the digest a dependent must see.
            // A `.fft` card makes every requested sample time a solver stop,
            // so it is part of the solve it rides on, not an observation of it.
            bound_cards::attach_bound_observation_cards(&mut queue);
            let producer_identities = queue
                .iter()
                .map(|task| {
                    (
                        task.instance_id(),
                        (
                            task.source_revision(),
                            task.config_digest(),
                            matches!(task.queued_analysis().spec, AnalysisSpec::Transient { .. }),
                            matches!(task.queued_analysis().spec, AnalysisSpec::Pss { .. }),
                            matches!(
                                task.queued_analysis().spec,
                                AnalysisSpec::HarmonicBalance { .. }
                            ),
                            matches!(
                                task.queued_analysis().spec,
                                AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }
                            ),
                            matches!(
                                task.queued_analysis().spec,
                                AnalysisSpec::Qpss {
                                    autonomous: false,
                                    ..
                                }
                            ),
                        ),
                    )
                })
                .collect::<HashMap<_, _>>();
            for task in &mut queue {
                use crate::simulation::execution::ExecutionArtifactKind;

                // The kinds this task's request admits, in preference order.
                // A periodic small-signal request whose carrier is the
                // preceding periodic solve admits either family, and the one
                // it binds is whichever family the plan's own edge points at.
                let required_kinds = crate::simulation::execution::required_artifact_kinds(
                    &task.queued_analysis().spec,
                    &task.queued_analysis().spec_options,
                );
                if required_kinds.is_empty() {
                    continue;
                }
                let producers = task
                    .dependencies()
                    .iter()
                    .filter_map(|dependency| {
                        let (revision, config_digest, transient, pss, hb, op, qpss) =
                            producer_identities.get(dependency)?;
                        let kind = required_kinds.iter().copied().find(|kind| match kind {
                            ExecutionArtifactKind::TransientTrajectory => *transient,
                            ExecutionArtifactKind::PeriodicState => *pss,
                            ExecutionArtifactKind::HbState => *hb,
                            ExecutionArtifactKind::QpssState => *qpss,
                            ExecutionArtifactKind::DcOperatingPointSeed => *op,
                        })?;
                        Some(match kind {
                            ExecutionArtifactKind::TransientTrajectory => {
                                PreparedDependencyBinding::transient_trajectory(
                                    *dependency,
                                    *revision,
                                    *config_digest,
                                )
                            }
                            ExecutionArtifactKind::PeriodicState => {
                                PreparedDependencyBinding::periodic_state(
                                    *dependency,
                                    *revision,
                                    *config_digest,
                                )
                            }
                            ExecutionArtifactKind::QpssState => {
                                PreparedDependencyBinding::qpss_state(
                                    *dependency,
                                    *revision,
                                    *config_digest,
                                )
                            }
                            ExecutionArtifactKind::HbState => PreparedDependencyBinding::hb_state(
                                *dependency,
                                *revision,
                                *config_digest,
                            ),
                            ExecutionArtifactKind::DcOperatingPointSeed => {
                                PreparedDependencyBinding::dc_operating_point_seed(
                                    *dependency,
                                    *revision,
                                    *config_digest,
                                )
                            }
                        })
                    })
                    .collect::<Vec<_>>();
                if producers.len() != 1 {
                    errors.push(format!(
                        "{} must bind exactly one prepared {} task, found {}",
                        task.queued_analysis().spec.run_type().display_name(),
                        required_kinds
                            .iter()
                            .map(|kind| kind.producer_label())
                            .collect::<Vec<_>>()
                            .join(" or "),
                        producers.len()
                    ));
                } else {
                    task.set_dependency_bindings(producers);
                }
            }
            if errors.is_empty() {
                Ok(queue)
            } else {
                Err(errors)
            }
        }
    }

    pub(super) fn executes_via_spec(spec: &AnalysisSpec) -> bool {
        matches!(
            spec,
            AnalysisSpec::Tf { .. }
                | AnalysisSpec::AcData { .. }
                | AnalysisSpec::Disto { .. }
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pstb
                | AnalysisSpec::Stb { .. }
                | AnalysisSpec::MonteCarlo { .. }
                | AnalysisSpec::Parametric
                | AnalysisSpec::Corner
                | AnalysisSpec::Pss { .. }
                | AnalysisSpec::PssSpectrum { .. }
                | AnalysisSpec::HarmonicBalance { .. }
                | AnalysisSpec::Pac
                | AnalysisSpec::SParameter { .. }
                | AnalysisSpec::Envelope { .. }
                | AnalysisSpec::Fourier { .. }
                | AnalysisSpec::Fft { .. }
                | AnalysisSpec::Optimization { .. }
                | AnalysisSpec::Soa { .. }
                | AnalysisSpec::Qpss { .. }
                | AnalysisSpec::Hbsp { .. }
                | AnalysisSpec::Hbnoise { .. }
                | AnalysisSpec::Psp { .. }
                | AnalysisSpec::Qpac { .. }
                | AnalysisSpec::Qpnoise { .. }
                | AnalysisSpec::Qpxf { .. }
                | AnalysisSpec::TransientNoise { .. }
                | AnalysisSpec::DcMismatch { .. }
        )
    }

    fn compile_study_seeded_periodic(
        &self,
        state: &AppState,
        plan: &FrozenSimulationPlan,
        base: &crate::simulation::plan::FrozenAnalysisInstance,
        spec: &AnalysisSpec,
    ) -> Result<crate::simulation::runner::study::StudyAnalysis, String> {
        let producers = base
            .dependencies()
            .iter()
            .filter(|edge| edge.prerequisite() == AnalysisKind::OperatingPoint)
            .filter_map(|edge| {
                plan.instances()
                    .iter()
                    .find(|instance| instance.id() == edge.target())
            })
            .collect::<Vec<_>>();
        let [producer] = producers.as_slice() else {
            return Err("A periodic study requires exactly one explicitly bound, enabled operating-point producer".into());
        };
        let mut producer_state = state.clone();
        producer_state.sim_setup = state
            .sim_setup
            .frozen_instance_projection(plan, producer)
            .map_err(|error| error.to_string())?;
        let producer_spec = self.analysis_draft_spec(&producer_state, producer.draft())?;
        let AnalysisConfig::DcOp(config) =
            self.analysis_spec_to_config(&producer_state, &producer_spec)?
        else {
            return Err("Periodic study dependency is not an operating-point configuration".into());
        };
        use crate::simulation::runner::study::{
            StudyAnalysis, StudyHbConfig, StudyOperatingPoint, StudyPssConfig, StudyQpssConfig,
        };
        let operating_point = StudyOperatingPoint {
            instance_id: producer.id(),
            source_revision: plan.revision(),
            config,
            numeric_options: producer
                .numeric_override()
                .map(|options| options.to_spice_options())
                .unwrap_or_default(),
        };
        if matches!(spec, AnalysisSpec::HarmonicBalance { .. }) {
            Ok(StudyAnalysis::Hb(Box::new(StudyHbConfig {
                request: spec.clone(),
                operating_point,
            })))
        } else if matches!(spec, AnalysisSpec::Qpss { .. }) {
            Ok(StudyAnalysis::Qpss(Box::new(StudyQpssConfig {
                request: spec.clone(),
                operating_point,
            })))
        } else {
            Ok(StudyAnalysis::Pss(Box::new(StudyPssConfig {
                request: spec.clone(),
                operating_point,
            })))
        }
    }

    fn compile_study_base(
        &self,
        state: &AppState,
        plan: &FrozenSimulationPlan,
        draft: &crate::simulation::plan::AnalysisDraft,
    ) -> Result<Option<crate::simulation::runner::study::StudyRunConfig>, String> {
        use crate::simulation::plan::AnalysisDraft;
        let (id, measurements, histogram_bins, objective_terms, constraints) = match draft {
            AnalysisDraft::MonteCarlo(draft) if draft.base_analysis.is_some() => {
                let config = draft.to_config()?;
                (
                    config.base_analysis.unwrap(),
                    config.measurements,
                    config.histogram_bins,
                    Vec::new(),
                    Vec::new(),
                )
            }
            AnalysisDraft::Optimization(draft) if draft.base_analysis.is_some() => {
                let config = draft.to_config()?;
                (
                    config.base_analysis.unwrap(),
                    config.measurement_names(),
                    20,
                    config.objective_terms,
                    config.constraints,
                )
            }
            _ => return Ok(None),
        };
        let base = plan
            .instances()
            .iter()
            .find(|instance| instance.id() == id)
            .ok_or_else(|| format!("Study base analysis {id} is missing or disabled"))?;
        if !base.kind().supports_study_base() {
            return Err(format!(
                "{} cannot yet be used as a study base",
                base.display_name()
            ));
        }
        let mut projected = state.clone();
        projected.sim_setup = state
            .sim_setup
            .frozen_instance_projection(plan, base)
            .map_err(|error| error.to_string())?;
        let spec = self.analysis_draft_spec(&projected, base.draft())?;
        use crate::simulation::runner::study::StudyPeriodicOptions;
        let periodic_options = match base.draft() {
            AnalysisDraft::Pac(draft) => Some(StudyPeriodicOptions::Pac(
                Self::pac_run_config_from_dialog(&projected, draft)?,
            )),
            AnalysisDraft::Pxf(draft) => Some(StudyPeriodicOptions::Pxf(
                Self::pxf_run_config_from_dialog(&projected, draft)?,
            )),
            AnalysisDraft::Pnoise(draft) => Some(StudyPeriodicOptions::Pnoise(
                Self::pnoise_run_config_from_dialog(&projected, draft)?,
            )),
            AnalysisDraft::Pstb(draft) => Some(StudyPeriodicOptions::Pstb(
                Self::pstb_run_config_from_dialog(&projected, draft)?,
            )),
            _ => None,
        };
        let execution_options = periodic_options
            .as_ref()
            .map(StudyPeriodicOptions::execution_options)
            .unwrap_or_default();
        let producer_kind = match spec {
            AnalysisSpec::Fourier { .. } | AnalysisSpec::Fft { .. } => {
                Some(AnalysisKind::Transient)
            }
            AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. } => {
                Some(AnalysisKind::HarmonicBalance)
            }
            AnalysisSpec::Psp { .. } | AnalysisSpec::Pstb => Some(AnalysisKind::Pss),
            AnalysisSpec::Qpac { .. }
            | AnalysisSpec::Qpxf { .. }
            | AnalysisSpec::Qpnoise { .. } => Some(AnalysisKind::Qpss),
            AnalysisSpec::Pac | AnalysisSpec::Pxf | AnalysisSpec::Pnoise => {
                let carriers = base
                    .dependencies()
                    .iter()
                    .filter(|edge| {
                        matches!(
                            edge.prerequisite(),
                            AnalysisKind::Pss | AnalysisKind::HarmonicBalance
                        )
                    })
                    .collect::<Vec<_>>();
                let [carrier] = carriers.as_slice() else {
                    return Err(
                        "A periodic study requires exactly one explicitly bound PSS or HB producer"
                            .into(),
                    );
                };
                Some(carrier.prerequisite())
            }
            _ => None,
        };
        let (analysis, postprocess) = if let Some(producer_kind) = producer_kind {
            let producers = base
                .dependencies()
                .iter()
                .filter(|edge| edge.prerequisite() == producer_kind)
                .filter_map(|edge| {
                    plan.instances()
                        .iter()
                        .find(|instance| instance.id() == edge.target())
                })
                .collect::<Vec<_>>();
            let [producer] = producers.as_slice() else {
                return Err("A spectral study requires exactly one explicitly bound, enabled producer of the required analysis kind".into());
            };
            let mut producer_state = state.clone();
            producer_state.sim_setup = state
                .sim_setup
                .frozen_instance_projection(plan, producer)
                .map_err(|error| error.to_string())?;
            let producer_spec = self.analysis_draft_spec(&producer_state, producer.draft())?;
            crate::simulation::execution::validate_prepared_dependency_contract_with_options(
                &spec,
                &execution_options,
                &producer_spec,
            )
            .map_err(|error| error.to_string())?;
            (
                if matches!(
                    producer_spec,
                    AnalysisSpec::Pss { .. }
                        | AnalysisSpec::Qpss { .. }
                        | AnalysisSpec::HarmonicBalance { .. }
                ) {
                    self.compile_study_seeded_periodic(state, plan, producer, &producer_spec)?
                } else {
                    self.analysis_spec_to_config(&producer_state, &producer_spec)?
                        .into()
                },
                Some(crate::simulation::runner::study::StudyPostprocess {
                    producer_instance_id: producer.id(),
                    producer_source_revision: plan.revision(),
                    producer_analysis_line: self.analysis_spec_to_spice_line(
                        &producer_state,
                        producer.draft(),
                        &producer_spec,
                    )?,
                    producer_numeric_options: producer
                        .numeric_override()
                        .map(|options| options.to_spice_options())
                        .unwrap_or_default(),
                    request: spec.clone(),
                    periodic_options,
                }),
            )
        } else if matches!(
            spec,
            AnalysisSpec::Pss { .. }
                | AnalysisSpec::Qpss { .. }
                | AnalysisSpec::HarmonicBalance { .. }
        ) {
            (
                self.compile_study_seeded_periodic(state, plan, base, &spec)?,
                None,
            )
        } else {
            (
                self.analysis_spec_to_config(&projected, &spec)?.into(),
                None,
            )
        };
        analysis.validate().map_err(|errors| errors.join("; "))?;
        Ok(Some(crate::simulation::runner::study::StudyRunConfig {
            postprocess,
            instance_id: id,
            source_revision: plan.revision(),
            analysis,
            analysis_line: self.analysis_spec_to_spice_line(&projected, base.draft(), &spec)?,
            numeric_options: base
                .numeric_override()
                .map(|options| options.to_spice_options())
                .unwrap_or_default(),
            measurements,
            histogram_bins,
            objective_terms,
            constraints,
        }))
    }

    pub(super) fn analysis_spec_execution_options(
        &self,
        state: &AppState,
        draft: &crate::simulation::plan::AnalysisDraft,
        spec: &AnalysisSpec,
        sealed_model_sources: &crate::state::model_library::SealedModelExecutionSources,
    ) -> Result<SpecExecutionOptions, String> {
        use crate::simulation::plan::AnalysisDraft;

        match spec {
            AnalysisSpec::MonteCarlo { .. } => {
                let AnalysisDraft::MonteCarlo(draft) = draft else {
                    return Err("Monte Carlo specification requires its authored draft".into());
                };
                let mut draft = draft.clone();
                draft.ensure_initialized();
                let config = draft.to_config()?;
                Ok(SpecExecutionOptions {
                    mc_checkpoint: monte_carlo_checkpoint::request_from_config(
                        config.checkpoint.as_ref(),
                    ),
                    mc_histogram_bins: config
                        .base_analysis
                        .is_none()
                        .then_some(config.histogram_bins),
                    mc_statistics: config.statistics,
                    ..Default::default()
                })
            }
            AnalysisSpec::Parametric => {
                let AnalysisDraft::Temperature(temp_state) = draft else {
                    return Err("Temperature specification requires its authored draft".into());
                };
                let mut temp_state = temp_state.clone();
                temp_state.ensure_initialized();
                if temp_state.base_analysis.is_some() {
                    // The frozen projection resolved the bound instance's kind.
                    // The authored index is only a legacy fallback and may be stale.
                    temp_state.base_idx = state.sim_setup.temp.base_idx;
                }
                let temp_cfg = temp_state
                    .to_config(&state.sim_setup.run_set, state.sim_setup.reference_pvt)
                    .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;
                Ok(SpecExecutionOptions {
                    mc_histogram_bins: None,
                    mc_statistics: None,
                    mc_checkpoint: None,
                    study_base: None,
                    temp: Some(Self::temp_run_config_from_dialog(state, &temp_cfg)?),
                    parametric_base: None,
                    corner: None,
                    pac: None,
                    pxf: None,
                    pnoise: None,
                    pstb: None,
                })
            }
            AnalysisSpec::Corner => {
                let AnalysisDraft::Corner(corner_state) = draft else {
                    return Err("Corner specification requires its authored draft".into());
                };
                let mut corner_state = corner_state.clone();
                corner_state.ensure_initialized();
                if corner_state.base_analysis.is_some() {
                    // The bound base, not the legacy index, selects the run mode.
                    corner_state.base_analysis_idx = state.sim_setup.corner.base_analysis_idx;
                }
                let corner_cfg = crate::simulation::dialog::corner::to_config(
                    &corner_state,
                    &state.sim_setup.run_set,
                    state.sim_setup.reference_pvt,
                )
                .map_err(|e| format!("invalid corner settings: {}", e))?;
                Ok(SpecExecutionOptions {
                    mc_histogram_bins: None,
                    mc_statistics: None,
                    mc_checkpoint: None,
                    study_base: None,
                    temp: None,
                    parametric_base: None,
                    corner: Some(Self::corner_run_config_from_dialog(
                        state,
                        &corner_cfg,
                        sealed_model_sources,
                    )?),
                    pac: None,
                    pxf: None,
                    pnoise: None,
                    pstb: None,
                })
            }
            AnalysisSpec::Pac => {
                let AnalysisDraft::Pac(draft) = draft else {
                    return Err("PAC specification requires its authored draft".into());
                };
                Ok(SpecExecutionOptions {
                    pac: Some(Self::pac_run_config_from_dialog(state, draft)?),
                    ..Default::default()
                })
            }
            AnalysisSpec::Pxf => {
                let AnalysisDraft::Pxf(draft) = draft else {
                    return Err("PXF specification requires its authored draft".into());
                };
                Ok(SpecExecutionOptions {
                    pxf: Some(Self::pxf_run_config_from_dialog(state, draft)?),
                    ..Default::default()
                })
            }
            AnalysisSpec::Tf { .. } => Ok(SpecExecutionOptions::default()),
            AnalysisSpec::Pnoise => {
                let AnalysisDraft::Pnoise(draft) = draft else {
                    return Err("PNOISE specification requires its authored draft".into());
                };
                Ok(SpecExecutionOptions {
                    pnoise: Some(Self::pnoise_run_config_from_dialog(state, draft)?),
                    ..Default::default()
                })
            }
            AnalysisSpec::Pstb => {
                let AnalysisDraft::Pstb(draft) = draft else {
                    return Err("PSTB specification requires its authored draft".into());
                };
                Ok(SpecExecutionOptions {
                    pstb: Some(Self::pstb_run_config_from_dialog(state, draft)?),
                    ..Default::default()
                })
            }
            AnalysisSpec::Psp { .. } => Ok(SpecExecutionOptions::default()),
            _ => Ok(SpecExecutionOptions {
                mc_histogram_bins: None,
                mc_statistics: None,
                mc_checkpoint: None,
                study_base: None,
                temp: None,
                parametric_base: None,
                corner: None,
                pac: None,
                pxf: None,
                pnoise: None,
                pstb: None,
            }),
        }
    }
}

fn invalid_saved_output_reports(
    count: usize,
    reason: impl Into<String>,
) -> Vec<crate::simulation::SavedOutputPreflightReport> {
    let report = crate::simulation::SavedOutputPreflightReport::invalid(reason);
    vec![report; count]
}

#[cfg(test)]
mod tests;
