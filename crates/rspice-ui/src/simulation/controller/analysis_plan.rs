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
        // The existing engine configuration builders still read the retired
        // singleton setup view. Clone once, then project each frozen instance
        // (and its exact bound prerequisites) into that short-lived view.
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
            // and `analysis_draft_directive` is it plus the line. Both state
            // the contract they share: the builders read the projection, never
            // the draft. Handing the first of them the unprojected state made
            // the queue capable of dispatching a directive the plan never
            // displayed and the parse ratchet never read.
            let spec = match self.analysis_draft_spec(&projected_state, instance.draft()) {
                Ok(spec) => spec,
                Err(error) => {
                    errors.push(format!("{}: {error}", instance.display_name()));
                    continue;
                }
            };
            let analysis_line = match self.analysis_spec_to_spice_line(&projected_state, &spec) {
                Ok(line) => line,
                Err(e) => {
                    errors.push(format!("{}: {}", instance.display_name(), e));
                    continue;
                }
            };
            let mut spec_options = match self.analysis_spec_execution_options(
                &projected_state,
                &spec,
                sealed_model_sources,
            ) {
                Ok(opts) => opts,
                Err(e) => {
                    errors.push(format!("{}: {}", instance.display_name(), e));
                    continue;
                }
            };

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
            let numeric_override = instance.numeric_override().cloned();

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
            .with_run_at(run_at.clone());
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
                | AnalysisSpec::Reliability { .. }
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

    fn compile_study_pss(
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
            return Err("A PSS study requires exactly one explicitly bound, enabled operating-point producer".into());
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
            return Err("PSS study dependency is not an operating-point configuration".into());
        };
        Ok(crate::simulation::runner::study::StudyAnalysis::Pss(
            Box::new(crate::simulation::runner::study::StudyPssConfig {
                request: spec.clone(),
                operating_point: crate::simulation::runner::study::StudyOperatingPoint {
                    instance_id: producer.id(),
                    source_revision: plan.revision(),
                    config,
                    numeric_options: producer
                        .numeric_override()
                        .map(|options| options.to_spice_options())
                        .unwrap_or_default(),
                },
            }),
        ))
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
        if !crate::simulation::runner::study::supports_kind(base.kind()) {
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
        let periodic_options = match &spec {
            AnalysisSpec::Pac => Some(StudyPeriodicOptions::Pac(Self::pac_run_config_from_dialog(
                &projected,
            )?)),
            AnalysisSpec::Pxf => Some(StudyPeriodicOptions::Pxf(Self::pxf_run_config_from_dialog(
                &projected,
            )?)),
            AnalysisSpec::Pnoise => Some(StudyPeriodicOptions::Pnoise(
                Self::pnoise_run_config_from_dialog(&projected)?,
            )),
            AnalysisSpec::Pstb => Some(StudyPeriodicOptions::Pstb(
                Self::pstb_run_config_from_dialog(&projected)?,
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
                if matches!(producer_spec, AnalysisSpec::Pss { .. }) {
                    self.compile_study_pss(state, plan, producer, &producer_spec)?
                } else if matches!(
                    producer_spec,
                    AnalysisSpec::HarmonicBalance { .. } | AnalysisSpec::Qpss { .. }
                ) {
                    crate::simulation::runner::study::StudyAnalysis::Native(producer_spec.clone())
                } else {
                    self.analysis_spec_to_config(&producer_state, &producer_spec)?
                        .into()
                },
                Some(crate::simulation::runner::study::StudyPostprocess {
                    producer_instance_id: producer.id(),
                    producer_source_revision: plan.revision(),
                    producer_analysis_line: self
                        .analysis_spec_to_spice_line(&producer_state, &producer_spec)?,
                    producer_numeric_options: producer
                        .numeric_override()
                        .map(|options| options.to_spice_options())
                        .unwrap_or_default(),
                    request: spec.clone(),
                    periodic_options,
                }),
            )
        } else if matches!(spec, AnalysisSpec::Pss { .. }) {
            (self.compile_study_pss(state, plan, base, &spec)?, None)
        } else if matches!(
            spec,
            AnalysisSpec::HarmonicBalance { .. } | AnalysisSpec::Qpss { .. }
        ) {
            (
                crate::simulation::runner::study::StudyAnalysis::Native(spec.clone()),
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
            analysis_line: self.analysis_spec_to_spice_line(&projected, &spec)?,
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
        spec: &AnalysisSpec,
        sealed_model_sources: &crate::state::model_library::SealedModelExecutionSources,
    ) -> Result<SpecExecutionOptions, String> {
        match spec {
            AnalysisSpec::MonteCarlo { .. } => {
                let mut draft = state.sim_setup.mc.clone();
                draft.ensure_initialized();
                Ok(SpecExecutionOptions {
                    mc_statistics: draft.to_config()?.statistics,
                    ..Default::default()
                })
            }
            AnalysisSpec::Parametric => {
                let mut temp_state = state.sim_setup.temp.clone();
                temp_state.ensure_initialized();
                let temp_cfg = temp_state
                    .to_config(&state.sim_setup.run_set, state.sim_setup.reference_pvt)
                    .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;
                Ok(SpecExecutionOptions {
                    mc_statistics: None,
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
                let mut corner_state = state.sim_setup.corner.clone();
                corner_state.ensure_initialized();
                let corner_cfg = corner_state
                    .to_config(&state.sim_setup.run_set, state.sim_setup.reference_pvt)
                    .map_err(|e| format!("invalid corner settings: {}", e))?;
                Ok(SpecExecutionOptions {
                    mc_statistics: None,
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
            AnalysisSpec::Pac => Ok(SpecExecutionOptions {
                mc_statistics: None,
                study_base: None,
                temp: None,
                parametric_base: None,
                corner: None,
                pac: Some(Self::pac_run_config_from_dialog(state)?),
                pxf: None,
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Pxf => Ok(SpecExecutionOptions {
                mc_statistics: None,
                study_base: None,
                temp: None,
                parametric_base: None,
                corner: None,
                pac: None,
                pxf: Some(Self::pxf_run_config_from_dialog(state)?),
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Tf { .. } => Ok(SpecExecutionOptions::default()),
            AnalysisSpec::Pnoise => Ok(SpecExecutionOptions {
                mc_statistics: None,
                study_base: None,
                temp: None,
                parametric_base: None,
                corner: None,
                pac: None,
                pxf: None,
                pnoise: Some(Self::pnoise_run_config_from_dialog(state)?),
                pstb: None,
            }),
            AnalysisSpec::Pstb => Ok(SpecExecutionOptions {
                mc_statistics: None,
                study_base: None,
                temp: None,
                parametric_base: None,
                corner: None,
                pac: None,
                pxf: None,
                pnoise: None,
                pstb: Some(Self::pstb_run_config_from_dialog(state)?),
            }),
            AnalysisSpec::Psp { .. } => Ok(SpecExecutionOptions::default()),
            _ => Ok(SpecExecutionOptions {
                mc_statistics: None,
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
mod tests {
    use super::*;
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind};

    #[test]
    fn qp_study_freezes_the_exact_producer_and_complete_consumer_controls() {
        use crate::simulation::dialog::{McDialogState, mc::McConfig};
        use crate::simulation::plan::{QpnoiseOutputDraft, QpssDraft};
        use crate::simulation::runner::study::StudyAnalysis;
        for kind in [
            AnalysisKind::Qpss,
            AnalysisKind::Qpac,
            AnalysisKind::Qpxf,
            AnalysisKind::Qpnoise,
        ] {
            let mut state = AppState::default();
            let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
            let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
            let (producer, _) = plan.insert(AnalysisKind::Qpss).unwrap();
            plan.bind_dependency(producer, AnalysisKind::OperatingPoint, op)
                .unwrap();
            plan.edit(producer, |draft| {
                *draft = AnalysisDraft::Qpss(QpssDraft {
                    tones: "1k,1414.2135623730951".into(),
                    harmonics: "1,1".into(),
                    relative_tolerance: "1e-8".into(),
                    max_backtracks: "7".into(),
                    collocation_points: "8,8".into(),
                    source_tones: "V1=1;I1=2".into(),
                    dc_initialization: true,
                    ..Default::default()
                })
            })
            .unwrap();
            let consumer = if kind == AnalysisKind::Qpss {
                producer
            } else {
                let (id, _) = plan.insert(kind).unwrap();
                plan.bind_dependency(id, AnalysisKind::Qpss, producer)
                    .unwrap();
                plan.edit(id, |draft| match draft {
                    AnalysisDraft::Qpac(d) => {
                        d.explicit_offsets = "100,300,700".into();
                        d.magnitude = ".002".into();
                        d.phase_degrees = "73".into();
                        d.input_lattice = "1,-1".into();
                        d.output_lattice = "1,-1".into();
                    }
                    AnalysisDraft::Qpxf(d) => {
                        d.explicit_frequencies = "-100,0,117".into();
                        d.group_delay = true;
                        d.input_lattice = "1,-1".into();
                        d.output_lattice = "1,-1".into();
                    }
                    AnalysisDraft::Qpnoise(d) => {
                        d.explicit_frequencies = "100,300,700".into();
                        d.band_start = "150".into();
                        d.band_stop = "600".into();
                        d.additional_outputs = vec![QpnoiseOutputDraft {
                            current: true,
                            branch: "V1".into(),
                            ..Default::default()
                        }];
                    }
                    _ => unreachable!(),
                })
                .unwrap();
                id
            };
            let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
            plan.edit(mc, |draft| {
                *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                    base_analysis: Some(consumer),
                    measurements: vec![
                        if kind == AnalysisKind::Qpss {
                            "tuple:1,0:magnitude:V(out)"
                        } else {
                            "bin:0:real:result"
                        }
                        .into(),
                    ],
                    ..Default::default()
                }))
            })
            .unwrap();
            let frozen = plan.freeze().unwrap();
            plan.edit(producer, |draft| {
                let AnalysisDraft::Qpss(d) = draft else {
                    unreachable!()
                };
                d.relative_tolerance = ".001".into();
            })
            .unwrap();
            let sealed = state
                .model_library_manager
                .seal_execution_sources()
                .unwrap();
            let queue = SimulationController::new()
                .build_queue_from_plan(&state, &frozen, &sealed)
                .unwrap();
            let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
            let base = task
                .queued_analysis()
                .spec_options
                .study_base
                .as_ref()
                .unwrap();
            let StudyAnalysis::Native(spec) = &base.analysis else {
                panic!("native")
            };
            assert_eq!(
                *spec,
                queue
                    .iter()
                    .find(|task| task.instance_id() == producer)
                    .unwrap()
                    .queued_analysis()
                    .spec
            );
            let AnalysisSpec::Qpss {
                relative_tolerance,
                controls,
                ..
            } = spec
            else {
                panic!("QPSS")
            };
            assert_eq!(*relative_tolerance, 1e-8);
            assert_eq!(controls.max_backtracks, 7);
            if kind == AnalysisKind::Qpss {
                assert!(base.postprocess.is_none());
            } else {
                let post = base.postprocess.as_ref().unwrap();
                assert_eq!(post.producer_instance_id, producer);
                assert_eq!(post.producer_source_revision, frozen.revision());
                assert_eq!(
                    post.request,
                    queue
                        .iter()
                        .find(|task| task.instance_id() == consumer)
                        .unwrap()
                        .queued_analysis()
                        .spec
                );
            }
            for change_producer in [true, false] {
                let mut changed = task.queued_analysis().clone();
                let base = changed.spec_options.study_base.as_mut().unwrap();
                if change_producer || base.postprocess.is_none() {
                    let StudyAnalysis::Native(AnalysisSpec::Qpss { controls, .. }) =
                        &mut base.analysis
                    else {
                        unreachable!()
                    };
                    controls.max_backtracks += 1;
                } else {
                    match &mut base.postprocess.as_mut().unwrap().request {
                        AnalysisSpec::Qpac { controls, .. } => controls.phase_degrees += 1.0,
                        AnalysisSpec::Qpxf { group_delay, .. } => *group_delay = false,
                        AnalysisSpec::Qpnoise { controls, .. } => {
                            controls.integration_band = Some([200.0, 500.0])
                        }
                        _ => unreachable!(),
                    }
                }
                assert_ne!(
                    task.config_digest(),
                    PreparedTask::new(mc, task.source_revision(), vec![], "MC", changed)
                        .config_digest()
                );
            }
        }
    }

    #[test]
    fn periodic_rf_study_freezes_all_consumer_options_and_exact_pss_op_chain() {
        use crate::simulation::dialog::{McDialogState, mc::McConfig};
        use crate::simulation::runner::study::{StudyAnalysis, StudyPeriodicOptions};
        for kind in [
            AnalysisKind::Pac,
            AnalysisKind::Pxf,
            AnalysisKind::Pnoise,
            AnalysisKind::Pstb,
            AnalysisKind::Psp,
        ] {
            let mut state = AppState::default();
            let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
            let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
            let (pss, _) = plan.insert(AnalysisKind::Pss).unwrap();
            plan.bind_dependency(pss, AnalysisKind::OperatingPoint, op)
                .unwrap();
            plan.edit(pss, |draft| {
                let AnalysisDraft::Pss(draft) = draft else {
                    unreachable!()
                };
                draft.tone_sources = "VIN".into();
            })
            .unwrap();
            let (consumer, _) = plan.insert(kind).unwrap();
            plan.bind_dependency(consumer, AnalysisKind::Pss, pss)
                .unwrap();
            plan.edit(consumer, |draft| match draft {
                AnalysisDraft::Pac(d) => {
                    d.pac_magnitude = "2.5".into();
                    d.sideband_min = "-1".into();
                    d.sideband_max = "0".into();
                    d.reltol = "2e-7".into();
                }
                AnalysisDraft::Pxf(d) => {
                    d.input_sideband = "-1".into();
                    d.output_sideband = "1".into();
                    d.max_sideband = "2".into();
                }
                AnalysisDraft::Pnoise(d) => {
                    d.input_sideband = "-1".into();
                    d.output_sideband = "1".into();
                    d.max_sideband = "2".into();
                    d.noise_summary = true;
                }
                AnalysisDraft::Pstb(d) => {
                    d.stability_threshold = "1.01".into();
                    d.detect_subharmonics = false;
                }
                AnalysisDraft::Psp(d) => {
                    d.max_sideband = "2".into();
                    d.mixed_mode = true;
                }
                _ => unreachable!(),
            })
            .unwrap();
            let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
            plan.edit(mc, |draft| {
                *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                    base_analysis: Some(consumer),
                    measurements: vec!["bin:0:real:result".into()],
                    ..Default::default()
                }))
            })
            .unwrap();
            let frozen = plan.freeze().unwrap();
            plan.edit(consumer, |draft| *draft = AnalysisDraft::for_kind(kind))
                .unwrap();
            let sealed = state
                .model_library_manager
                .seal_execution_sources()
                .unwrap();
            let queue = SimulationController::new()
                .build_queue_from_plan(&state, &frozen, &sealed)
                .unwrap();
            let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
            let consumer_task = queue
                .iter()
                .find(|task| task.instance_id() == consumer)
                .unwrap();
            let base = task
                .queued_analysis()
                .spec_options
                .study_base
                .as_ref()
                .unwrap();
            let StudyAnalysis::Pss(pss_config) = &base.analysis else {
                panic!("PSS")
            };
            assert_eq!(pss_config.operating_point.instance_id, op);
            let post = base.postprocess.as_ref().unwrap();
            assert_eq!(post.producer_instance_id, pss);
            assert_eq!(post.request, consumer_task.queued_analysis().spec);
            match &post.periodic_options {
                Some(StudyPeriodicOptions::Pac(c)) => {
                    assert_eq!(
                        Some(c),
                        consumer_task.queued_analysis().spec_options.pac.as_ref()
                    );
                    assert_eq!(c.pac_magnitude, 2.5);
                }
                Some(StudyPeriodicOptions::Pxf(c)) => {
                    assert_eq!(
                        Some(c),
                        consumer_task.queued_analysis().spec_options.pxf.as_ref()
                    );
                    assert_eq!(c.input_sideband, -1);
                }
                Some(StudyPeriodicOptions::Pnoise(c)) => {
                    assert_eq!(
                        Some(c),
                        consumer_task.queued_analysis().spec_options.pnoise.as_ref()
                    );
                    assert_eq!(c.output_sideband, 1);
                }
                Some(StudyPeriodicOptions::Pstb(c)) => {
                    assert_eq!(
                        Some(c),
                        consumer_task.queued_analysis().spec_options.pstb.as_ref()
                    );
                    assert_eq!(c.stability_threshold, 1.01);
                }
                None => assert!(matches!(
                    post.request,
                    AnalysisSpec::Psp {
                        mixed_mode: true,
                        ..
                    }
                )),
            }
            let mut changed = task.queued_analysis().clone();
            let post = changed
                .spec_options
                .study_base
                .as_mut()
                .unwrap()
                .postprocess
                .as_mut()
                .unwrap();
            match &mut post.periodic_options {
                Some(StudyPeriodicOptions::Pac(c)) => c.pac_magnitude = 3.0,
                Some(StudyPeriodicOptions::Pxf(c)) => c.input_sideband = 0,
                Some(StudyPeriodicOptions::Pnoise(c)) => c.output_sideband = 0,
                Some(StudyPeriodicOptions::Pstb(c)) => c.detect_subharmonics = true,
                None => {
                    let AnalysisSpec::Psp { mixed_mode, .. } = &mut post.request else {
                        unreachable!()
                    };
                    *mixed_mode = false;
                }
            }
            assert_ne!(
                task.config_digest(),
                PreparedTask::new(mc, task.source_revision(), vec![], "MC", changed)
                    .config_digest()
            );
        }
    }

    #[test]
    fn pss_study_freezes_its_exact_op_producer_and_complete_shooting_configuration() {
        use crate::simulation::dialog::{McDialogState, mc::McConfig};
        use crate::simulation::runner::study::StudyAnalysis;
        let mut state = AppState::default();
        let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
        let (first, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
        let (second, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
        let (pss, _) = plan.insert(AnalysisKind::Pss).unwrap();
        plan.bind_dependency(pss, AnalysisKind::OperatingPoint, first)
            .unwrap();
        plan.edit(pss, |draft| {
            let AnalysisDraft::Pss(draft) = draft else {
                unreachable!()
            };
            draft.fund_freq = "1k".into();
            draft.tone_sources = "V1".into();
        })
        .unwrap();
        let mut numerics = crate::simulation::plan::AnalysisNumericOverride::default();
        numerics
            .set_for_instance(
                AnalysisKind::OperatingPoint,
                Default::default(),
                crate::simulation::plan::NumericOverrideOption::Reltol,
                "1e-7",
            )
            .unwrap();
        plan.set_numeric_override(first, Some(numerics)).unwrap();
        let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
        plan.edit(mc, |draft| {
            *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                base_analysis: Some(pss),
                measurements: vec!["bin:1:magnitude:V(out)".into()],
                ..Default::default()
            }))
        })
        .unwrap();
        let frozen = plan.freeze().unwrap();
        plan.bind_dependency(pss, AnalysisKind::OperatingPoint, second)
            .unwrap();
        plan.edit(pss, |draft| {
            let AnalysisDraft::Pss(draft) = draft else {
                unreachable!()
            };
            draft.fund_freq = "2k".into();
        })
        .unwrap();
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .unwrap();
        let queue = SimulationController::new()
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap();
        let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
        let base = task
            .queued_analysis()
            .spec_options
            .study_base
            .as_ref()
            .unwrap();
        let StudyAnalysis::Pss(config) = &base.analysis else {
            panic!("PSS")
        };
        assert_eq!(config.operating_point.instance_id, first);
        assert_eq!(config.operating_point.source_revision, frozen.revision());
        assert!(config.operating_point.numeric_options.contains("RELTOL"));
        assert_eq!(
            config.request,
            queue
                .iter()
                .find(|task| task.instance_id() == pss)
                .unwrap()
                .queued_analysis()
                .spec
        );
        for change in 0..3 {
            let mut queued = task.queued_analysis().clone();
            let StudyAnalysis::Pss(config) =
                &mut queued.spec_options.study_base.as_mut().unwrap().analysis
            else {
                unreachable!()
            };
            match change {
                0 => config.operating_point.instance_id = second,
                1 => config.operating_point.config.temperature_celsius = 85.0,
                _ => config.operating_point.numeric_options = ".options RELTOL=.01".into(),
            }
            assert_ne!(
                task.config_digest(),
                PreparedTask::new(mc, task.source_revision(), vec![], "MC", queued).config_digest()
            );
        }
    }

    #[test]
    fn hb_study_freezes_the_selected_instance_and_authenticates_its_native_settings() {
        use crate::simulation::dialog::{
            HbDialogState, McDialogState,
            hb::{HbConfig, HbSolverType, HbToneConfig},
            mc::McConfig,
        };
        use crate::simulation::runner::study::StudyAnalysis;
        let mut state = AppState::default();
        let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
        let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
        let (hb, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
        plan.bind_dependency(hb, AnalysisKind::OperatingPoint, op)
            .unwrap();
        plan.edit(hb, |draft| {
            *draft = AnalysisDraft::HarmonicBalance(HbDialogState::from_config(&HbConfig {
                fundamental_freq: 1000.0,
                fundamental_source: Some("V1".into()),
                num_harmonics: 2,
                additional_tones: vec![
                    HbToneConfig::new(2000.0, 2)
                        .with_source("V2")
                        .with_name("second"),
                ],
                oversample: 3,
                max_mixing_order: 2,
                reltol: 2e-7,
                abstol: 3e-12,
                maxiter: 73,
                damping: 0.8,
                min_damping: 0.02,
                collocation_points: Some(31),
                solver: HbSolverType::Krylov,
                gmres_restart: 17,
                source_stepping: true,
                use_exact_jacobian: false,
                verbose: true,
            }));
        })
        .unwrap();
        let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
        plan.edit(mc, |draft| {
            *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                base_analysis: Some(hb),
                measurements: vec!["bin:1:magnitude:V(out)".into()],
                ..Default::default()
            }));
        })
        .unwrap();
        let frozen = plan.freeze().unwrap();
        plan.edit(hb, |draft| {
            let AnalysisDraft::HarmonicBalance(draft) = draft else {
                unreachable!()
            };
            draft.fundamental = "3k".into();
            draft.verbose = false;
        })
        .unwrap();
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .unwrap();
        let queue = SimulationController::new()
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap();
        let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
        let base = task
            .queued_analysis()
            .spec_options
            .study_base
            .as_ref()
            .unwrap();
        assert_eq!(base.instance_id, hb);
        let StudyAnalysis::Native(spec) = &base.analysis else {
            panic!("native base")
        };
        let selected = queue.iter().find(|task| task.instance_id() == hb).unwrap();
        assert_eq!(spec, &selected.queued_analysis().spec);
        assert!(
            matches!(spec, AnalysisSpec::HarmonicBalance { tones, verbose: true, .. } if tones[0].frequency == 1000.0)
        );
        let mut changed = task.queued_analysis().clone();
        let StudyAnalysis::Native(AnalysisSpec::HarmonicBalance {
            collocation_points, ..
        }) = &mut changed.spec_options.study_base.as_mut().unwrap().analysis
        else {
            unreachable!()
        };
        *collocation_points = Some(33);
        assert_ne!(
            task.config_digest(),
            PreparedTask::new(mc, task.source_revision(), vec![], "MC", changed).config_digest()
        );
    }

    #[test]
    fn hb_rf_study_freezes_exact_producer_consumer_and_noise_references() {
        use crate::simulation::dialog::{McDialogState, mc::McConfig};
        use crate::simulation::runner::study::StudyAnalysis;
        for kind in [AnalysisKind::Hbsp, AnalysisKind::Hbnoise] {
            let mut state = AppState::default();
            let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
            let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
            let (first, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
            let (second, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
            for (id, frequency) in [(first, "1meg"), (second, "2meg")] {
                plan.bind_dependency(id, AnalysisKind::OperatingPoint, op)
                    .unwrap();
                plan.edit(id, |draft| {
                    let AnalysisDraft::HarmonicBalance(draft) = draft else {
                        unreachable!()
                    };
                    draft.fundamental = frequency.into();
                })
                .unwrap();
            }
            let mut numerics = crate::simulation::plan::AnalysisNumericOverride::default();
            numerics
                .set_for_instance(
                    AnalysisKind::HarmonicBalance,
                    Default::default(),
                    crate::simulation::plan::NumericOverrideOption::Reltol,
                    "1e-6",
                )
                .unwrap();
            plan.set_numeric_override(first, Some(numerics)).unwrap();
            let (consumer, _) = plan.insert(kind).unwrap();
            plan.bind_dependency(consumer, AnalysisKind::HarmonicBalance, first)
                .unwrap();
            plan.edit(consumer, |draft| match draft {
                AnalysisDraft::Hbsp(draft) => {
                    draft.max_sideband = "1".into();
                    draft.noise_parameters = true;
                    draft.noise.report_parameters = true;
                    draft.noise.input_sideband = "-1".into();
                    draft.noise.output_sideband = "1".into();
                    draft.noise.reference_temperature = "310".into();
                    draft.noise.termination_temperature = "295".into();
                }
                AnalysisDraft::Hbnoise(draft) => {
                    draft.max_sideband = "1".into();
                    draft.noise_figure = true;
                    draft.source_resistor = "RSRC".into();
                    draft.reference_temperature = "310".into();
                    draft.input_sideband = "-1".into();
                    draft.output_sideband = "1".into();
                    draft.integrated_noise = false;
                    draft.contributor_ranking = false;
                }
                _ => unreachable!(),
            })
            .unwrap();
            let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
            plan.edit(mc, |draft| {
                *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                    base_analysis: Some(consumer),
                    measurements: vec![
                        if kind == AnalysisKind::Hbsp {
                            "bin:0:real:PN_NF"
                        } else {
                            "bin:0:real:noise_figure_db"
                        }
                        .into(),
                    ],
                    ..Default::default()
                }));
            })
            .unwrap();
            let frozen = plan.freeze().unwrap();
            plan.edit(first, |draft| {
                let AnalysisDraft::HarmonicBalance(draft) = draft else {
                    unreachable!()
                };
                draft.fundamental = "3meg".into();
            })
            .unwrap();
            plan.edit(consumer, |draft| match draft {
                AnalysisDraft::Hbsp(draft) => draft.noise.reference_temperature = "350".into(),
                AnalysisDraft::Hbnoise(draft) => draft.reference_temperature = "350".into(),
                _ => unreachable!(),
            })
            .unwrap();
            let sealed = state
                .model_library_manager
                .seal_execution_sources()
                .unwrap();
            let queue = SimulationController::new()
                .build_queue_from_plan(&state, &frozen, &sealed)
                .unwrap();
            let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
            let base = task
                .queued_analysis()
                .spec_options
                .study_base
                .as_ref()
                .unwrap();
            let StudyAnalysis::Native(producer) = &base.analysis else {
                panic!("HB producer")
            };
            let selected = queue
                .iter()
                .find(|task| task.instance_id() == first)
                .unwrap();
            assert_eq!(producer, &selected.queued_analysis().spec);
            assert!(
                matches!(producer, AnalysisSpec::HarmonicBalance { tones, .. } if tones[0].frequency == 1e6)
            );
            let post = base.postprocess.as_ref().unwrap();
            assert_eq!(base.instance_id, consumer);
            assert_eq!(post.producer_instance_id, first);
            assert_ne!(post.producer_instance_id, second);
            assert_eq!(post.producer_source_revision, frozen.revision());
            assert!(post.producer_numeric_options.contains("RELTOL"));
            assert_eq!(
                &post.request,
                &queue
                    .iter()
                    .find(|task| task.instance_id() == consumer)
                    .unwrap()
                    .queued_analysis()
                    .spec
            );
            for change in 0..4 {
                let mut queued = task.queued_analysis().clone();
                let base = queued.spec_options.study_base.as_mut().unwrap();
                let post = base.postprocess.as_mut().unwrap();
                match change {
                    0 => post.producer_instance_id = second,
                    1 => post.producer_numeric_options = ".OPTIONS RELTOL=0.01".into(),
                    2 => match &mut post.request {
                        AnalysisSpec::Hbsp {
                            noise_reference: Some(reference),
                            ..
                        } => reference.reference_temperature_kelvin = 350.0,
                        AnalysisSpec::Hbnoise {
                            noise_reference: Some(reference),
                            ..
                        } => reference.temperature_kelvin = 350.0,
                        _ => unreachable!(),
                    },
                    _ => {
                        let StudyAnalysis::Native(AnalysisSpec::HarmonicBalance { tones, .. }) =
                            &mut base.analysis
                        else {
                            unreachable!()
                        };
                        tones[0].frequency = 4e6;
                    }
                }
                assert_ne!(
                    task.config_digest(),
                    PreparedTask::new(mc, task.source_revision(), vec![], "MC", queued)
                        .config_digest()
                );
            }
        }
    }

    #[test]
    fn spectral_study_freezes_the_bound_transient_and_all_postprocess_settings() {
        use crate::simulation::dialog::{McDialogState, mc::McConfig};
        for kind in [AnalysisKind::Fourier, AnalysisKind::Fft] {
            let mut state = AppState::default();
            let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
            let (first, _) = plan.insert(AnalysisKind::Transient).unwrap();
            let (second, _) = plan.insert(AnalysisKind::Transient).unwrap();
            for (id, stop) in [(first, "1m"), (second, "2m")] {
                plan.edit(id, |draft| {
                    let AnalysisDraft::Transient(draft) = draft else {
                        unreachable!()
                    };
                    draft.stop = stop.into();
                    draft.step = "2u".into();
                    draft.max_step = "2u".into();
                })
                .unwrap();
            }
            let mut numerics = crate::simulation::plan::AnalysisNumericOverride::default();
            numerics
                .set_for_instance(
                    AnalysisKind::Transient,
                    Default::default(),
                    crate::simulation::plan::NumericOverrideOption::Reltol,
                    "1e-6",
                )
                .unwrap();
            plan.set_numeric_override(first, Some(numerics)).unwrap();
            let (spectrum, _) = plan.insert(kind).unwrap();
            plan.edit(spectrum, |draft| match draft {
                AnalysisDraft::Fourier(draft) => {
                    *draft = crate::simulation::dialog::FourierDialogState::from_config(
                        &crate::simulation::dialog::fourier::FourierConfig {
                            fundamental_freq: 1000.0,
                            num_harmonics: 5,
                            num_periods: 1,
                            output_node: "out".into(),
                            output_ref: "0".into(),
                            additional_outputs: vec![],
                            start_time: 0.0,
                            stop_time: 0.001,
                            compute_thd: true,
                            normalize: false,
                        },
                    )
                }
                AnalysisDraft::Fft(draft) => {
                    draft.stop = "1m".into();
                    draft.points = 64;
                    draft.format = "UNORM".into();
                }
                _ => unreachable!(),
            })
            .unwrap();
            plan.bind_dependency(spectrum, AnalysisKind::Transient, first)
                .unwrap();
            let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
            plan.edit(mc, |draft| {
                *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
                    base_analysis: Some(spectrum),
                    measurements: vec!["bin:1:magnitude".into()],
                    ..Default::default()
                }))
            })
            .unwrap();
            let frozen = plan.freeze().unwrap();
            plan.edit(first, |draft| {
                let AnalysisDraft::Transient(draft) = draft else {
                    unreachable!()
                };
                draft.stop = "3m".into();
            })
            .unwrap();
            let sealed = state
                .model_library_manager
                .seal_execution_sources()
                .unwrap();
            let queue = SimulationController::new()
                .build_queue_from_plan(&state, &frozen, &sealed)
                .unwrap();
            let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
            let base = task
                .queued_analysis()
                .spec_options
                .study_base
                .as_ref()
                .unwrap();
            let Some(AnalysisConfig::Transient(config)) = base.analysis.as_basic() else {
                panic!("transient producer")
            };
            assert_eq!(config.stop_time, 0.001);
            let post = base.postprocess.as_ref().unwrap();
            assert_eq!(post.producer_instance_id, first);
            assert_ne!(post.producer_instance_id, second);
            assert_eq!(post.producer_source_revision, frozen.revision());
            assert!(post.producer_numeric_options.contains("RELTOL"));
            for change in 0..5 {
                let mut queued = task.queued_analysis().clone();
                let base = queued.spec_options.study_base.as_mut().unwrap();
                let post = base.postprocess.as_mut().unwrap();
                match change {
                    0 => post.producer_instance_id = second,
                    1 => post.producer_analysis_line.push_str(" UIC"),
                    2 => post.producer_numeric_options = ".OPTIONS RELTOL=0.01".into(),
                    3 => match &mut post.request {
                        AnalysisSpec::Fft { request } => request.points = 128,
                        AnalysisSpec::Fourier { normalize, .. } => *normalize = true,
                        _ => unreachable!(),
                    },
                    _ => {
                        let Some(AnalysisConfig::Transient(config)) = base.analysis.as_basic_mut()
                        else {
                            unreachable!()
                        };
                        config.max_timestep = Some(1e-6);
                    }
                }
                let changed = PreparedTask::new(mc, task.source_revision(), vec![], "MC", queued);
                assert_ne!(task.config_digest(), changed.config_digest());
            }
        }
    }

    #[test]
    fn configured_study_freezes_exact_base_and_survives_persistence_and_identity() {
        use crate::simulation::dialog::{McDialogState, mc::McConfig};
        let mut state = AppState::default();
        let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
        let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
        let (ac, _) = plan.insert(AnalysisKind::Ac).unwrap();
        let (other, _) = plan.insert(AnalysisKind::Ac).unwrap();
        let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
        for (id, stop) in [(ac, "1000"), (other, "9000")] {
            plan.bind_dependency(id, AnalysisKind::OperatingPoint, op)
                .unwrap();
            plan.edit(id, |draft| {
                let AnalysisDraft::Ac(draft) = draft else {
                    unreachable!()
                };
                draft.fstart = "1000".into();
                draft.fstop = stop.into();
                draft.sweep = 2;
                draft.points = "1".into();
            })
            .unwrap();
        }
        let mut numerics = crate::simulation::plan::AnalysisNumericOverride::default();
        numerics
            .set_for_instance(
                AnalysisKind::Ac,
                Default::default(),
                crate::simulation::plan::NumericOverrideOption::Reltol,
                "1e-5",
            )
            .unwrap();
        plan.set_numeric_override(ac, Some(numerics)).unwrap();
        let draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
            statistics: Some(
                crate::simulation::dialog::mc::statistics::McStatisticsConfig {
                    variations: vec![
                        crate::simulation::dialog::mc::statistics::McParameterVariation {
                            bounds: Some(
                                crate::simulation::dialog::mc::statistics::McParameterBounds {
                                    lower: Some(900.0),
                                    upper: Some(1100.0),
                                    sigma_cutoff: Some(3.0),
                                    max_attempts: 10000,
                                },
                            ),
                            parameter: "rval".into(),
                            scope: crate::simulation::dialog::mc::statistics::McScope::Process,
                            distribution:
                                crate::simulation::dialog::mc::statistics::McShape::Gaussian,
                            spread: 10.0,
                            percent: true,
                        },
                    ],
                    correlations: Vec::new(),
                },
            ),
            variation_source: crate::simulation::dialog::McVariationSource::DeckStatistics,
            base_analysis: Some(ac),
            measurements: vec!["gain".into(), "last:V(out)".into()],
            histogram_bins: 7,
            num_runs: 3,
            ..Default::default()
        }));
        for mut restored in [
            serde_json::from_str::<AnalysisDraft>(&serde_json::to_string(&draft).unwrap()).unwrap(),
            ron::from_str::<AnalysisDraft>(&ron::to_string(&draft).unwrap()).unwrap(),
        ] {
            restored.prepare_after_restore();
            let AnalysisDraft::MonteCarlo(mut restored) = restored else {
                unreachable!()
            };
            restored.ensure_initialized();
            let config = restored.to_config().unwrap();
            assert_eq!(config.base_analysis, Some(ac));
            assert_eq!(config.histogram_bins, 7);
            assert_eq!(
                config.statistics.as_ref().unwrap().variations[0].spread,
                10.0
            );
            assert_eq!(config.measurements, ["gain", "last:V(out)"]);
        }
        plan.edit(mc, |target| *target = draft).unwrap();
        let frozen = plan.freeze().unwrap();
        // Later live edits must not replace the base in this prepared plan.
        plan.edit(ac, |draft| {
            let AnalysisDraft::Ac(draft) = draft else {
                unreachable!()
            };
            draft.fstop = "3000".into();
        })
        .unwrap();
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .unwrap();
        let controller = SimulationController::new();
        let tasks = controller
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap();
        let task = tasks.iter().find(|task| task.instance_id() == mc).unwrap();
        let base = task
            .queued_analysis()
            .spec_options
            .study_base
            .as_ref()
            .unwrap();
        assert_eq!(base.instance_id, ac);
        let Some(AnalysisConfig::Ac(config)) = base.analysis.as_basic() else {
            panic!("AC base")
        };
        assert_eq!(config.stop_freq, 1000.0);
        assert!(base.numeric_options.to_ascii_uppercase().contains("RELTOL"));
        assert_eq!(
            task.queued_analysis()
                .spec_options
                .mc_statistics
                .as_ref()
                .unwrap()
                .variations[0]
                .spread,
            10.0
        );
        for change in 0..15 {
            let mut queued = task.queued_analysis().clone();
            let base = queued.spec_options.study_base.as_mut().unwrap();
            match change {
                0 => base.instance_id = other,
                1 => base.histogram_bins += 1,
                2 => base.measurements = vec!["last:V(out)".into()],
                3 => base.numeric_options = ".OPTIONS RELTOL=0.02".into(),
                4 => {
                    let Some(AnalysisConfig::Ac(config)) = base.analysis.as_basic_mut() else {
                        unreachable!()
                    };
                    config.stop_freq = 2000.0;
                }
                _ => {
                    use crate::simulation::dialog::mc::statistics::{McScope, McShape};
                    let row = &mut queued
                        .spec_options
                        .mc_statistics
                        .as_mut()
                        .unwrap()
                        .variations[0];
                    match change {
                        5 => row.parameter = "other".into(),
                        6 => row.scope = McScope::Mismatch,
                        7 => row.distribution = McShape::Uniform,
                        8 => row.spread = 20.0,
                        9 => row.percent = false,
                        10 => row.bounds.as_mut().unwrap().lower = Some(800.0),
                        11 => row.bounds.as_mut().unwrap().upper = Some(1200.0),
                        12 => row.bounds.as_mut().unwrap().sigma_cutoff = Some(4.0),
                        13 => row.bounds.as_mut().unwrap().max_attempts = 20000,
                        _ => row.bounds = None,
                    }
                }
            }
            let changed = PreparedTask::new(mc, task.source_revision(), vec![], "MC", queued);
            assert_ne!(task.config_digest(), changed.config_digest());
        }
        let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
        plan.set_enabled(ac, false).unwrap();
        let frozen = plan.freeze().unwrap();
        let errors = controller
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap_err();
        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing or disabled")),
            "{errors:?}"
        );
    }

    #[test]
    fn configured_optimization_base_persists_and_freezes_before_live_edits() {
        use crate::simulation::dialog::optimization::{
            OptimizationConfig, OptimizationDialogState,
        };
        use crate::simulation::optimizer::{OptimizationObjectiveGoal, OptimizationObjectiveTerm};
        let objectives = vec![OptimizationObjectiveTerm {
            measurement: "gain".into(),
            goal: OptimizationObjectiveGoal::Target,
            target: Some(2.5),
            scale: 0.5,
            weight: 3.0,
        }];
        let mut state = AppState::default();
        let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
        let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
        let (ac, _) = plan.insert(AnalysisKind::Ac).unwrap();
        plan.bind_dependency(ac, AnalysisKind::OperatingPoint, op)
            .unwrap();
        let (opt, _) = plan.insert(AnalysisKind::Optimization).unwrap();
        let mut setup = OptimizationDialogState::from_config(&OptimizationConfig {
            base_analysis: Some(ac),
            objective_measurement: "gain".into(),
            objective_terms: objectives.clone(),
            constraints: vec![crate::simulation::optimizer::OptimizationConstraint {
                measurement: "gain".into(),
                lower: Some(1.0),
                upper: Some(3.0),
                tolerance: 0.01,
                scale: 2.0,
            }],
            ..Default::default()
        });
        // Inactive expression buffers survive switching back without blocking a measured objective.
        setup.objective_expression = "unfinished{".into();
        setup.objective_node.clear();
        setup.target_value = "inactive unfinished".into();
        let draft = AnalysisDraft::Optimization(setup);
        for mut restored in [
            serde_json::from_str::<AnalysisDraft>(&serde_json::to_string(&draft).unwrap()).unwrap(),
            ron::from_str::<AnalysisDraft>(&ron::to_string(&draft).unwrap()).unwrap(),
        ] {
            restored.prepare_after_restore();
            let AnalysisDraft::Optimization(ref mut setup) = restored else {
                unreachable!()
            };
            setup.ensure_initialized();
            let config = setup.to_config().unwrap();
            assert_eq!(config.base_analysis, Some(ac));
            assert_eq!(config.objective_measurement, "gain");
            assert_eq!(setup.objective_expression, "unfinished{");
            assert_eq!(config.objective_terms, objectives);
            assert_eq!(config.constraints[0].upper, Some(3.0));
            assert_eq!(setup.target_value, "inactive unfinished");
            assert!(config.to_spice().contains("weighted_objectives="));
        }
        plan.edit(opt, |target| *target = draft).unwrap();
        let frozen = plan.freeze().unwrap();
        plan.edit(opt, |draft| {
            let AnalysisDraft::Optimization(setup) = draft else {
                unreachable!()
            };
            setup.objective_measurement = "changed".into();
            setup.objective_terms[0].weight = "9".into();
            setup.constraints[0].upper = "99".into();
        })
        .unwrap();
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .unwrap();
        let tasks = SimulationController::new()
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap();
        let task = tasks.iter().find(|task| task.instance_id() == opt).unwrap();
        let base = task
            .queued_analysis()
            .spec_options
            .study_base
            .as_ref()
            .unwrap();
        assert_eq!(base.instance_id, ac);
        assert_eq!(base.measurements, ["gain"]);
        assert_eq!(base.objective_terms, objectives);
        assert_eq!(base.constraints[0].upper, Some(3.0));
        for change in 0..10 {
            let mut queued = task.queued_analysis().clone();
            let term = &mut queued
                .spec_options
                .study_base
                .as_mut()
                .unwrap()
                .objective_terms[0];
            match change {
                0 => term.weight = 4.0,
                1 => term.scale = 2.0,
                2 => term.target = Some(7.0),
                3 => term.goal = OptimizationObjectiveGoal::Maximize,
                4 => {
                    let AnalysisSpec::Optimization { search, .. } = &mut queued.spec else {
                        unreachable!()
                    };
                    search.variable_domains.insert(
                        "RLOAD".into(),
                        crate::simulation::optimizer::OptimizationVariableDomain::Logarithmic,
                    );
                }
                _ => {
                    let constraint =
                        &mut queued.spec_options.study_base.as_mut().unwrap().constraints[0];
                    match change {
                        5 => constraint.measurement = "other".into(),
                        6 => constraint.lower = None,
                        7 => constraint.upper = Some(4.0),
                        8 => constraint.tolerance = 0.02,
                        _ => constraint.scale = 3.0,
                    }
                }
            }
            let changed = PreparedTask::new(opt, task.source_revision(), vec![], "OPT", queued);
            assert_ne!(task.config_digest(), changed.config_digest());
        }
        assert!(matches!(
            base.analysis.as_basic(),
            Some(AnalysisConfig::Ac(_))
        ));
    }

    #[test]
    fn frozen_noise_task_keeps_exact_draft_and_reference_pvt() {
        let mut state = AppState::default();
        state.sim_setup.reference_pvt.temperature_celsius = -40.0;
        state.sim_setup.noise.output = "singleton_must_not_leak".to_owned();
        state.sim_setup.ac.points = "777".to_owned();
        let plan = state.sim_setup.analysis_plan.as_mut().expect("stable plan");
        let (op, _) = plan
            .insert(AnalysisKind::OperatingPoint)
            .expect("OP inserts");
        let (noise, _) = plan.insert(AnalysisKind::Noise).expect("noise inserts");
        plan.edit(noise, |draft| {
            let AnalysisDraft::Noise(draft) = draft else {
                panic!("noise draft")
            };
            draft.output = "V(out,ref)".to_owned();
            draft.input = "VSTIM".to_owned();
            draft.sweep = crate::simulation::config::NoiseSweepType::ExplicitFrequencyList;
            draft.explicit_frequencies = "1, 5, 25".to_owned();
            draft.contribution_detail = crate::simulation::config::NoiseContributionDetail::Top20;
            draft.integration_mode = crate::simulation::config::NoiseIntegrationMode::Disabled;
        })
        .expect("noise edits");
        plan.bind_dependency(noise, AnalysisKind::OperatingPoint, op)
            .expect("noise binds OP");

        let controller = SimulationController::new();
        let frozen = controller
            .build_analysis_plan(&state)
            .expect("plan freezes");
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .expect("model sources seal");
        let tasks = controller
            .build_queue_from_plan(&state, &frozen, &sealed)
            .expect("noise plan compiles");
        let task = tasks
            .iter()
            .find(|task| task.instance_id() == noise)
            .expect("noise task");
        assert!(matches!(
            &task.queued_analysis().spec,
            AnalysisSpec::Noise {
                output_node,
                reference_node,
                input_source,
                explicit_frequencies: Some(frequencies),
                contribution_detail: crate::simulation::config::NoiseContributionDetail::Top20,
                integration_mode: crate::simulation::config::NoiseIntegrationMode::Disabled,
                temperature,
                ..
            } if output_node == "out"
                && reference_node == "ref"
                && input_source == "VSTIM"
                && frequencies == &[1.0, 5.0, 25.0]
                && (*temperature - 233.15).abs() < 1.0e-12
        ));
        let Some(AnalysisConfig::Noise(config)) = &task.queued_analysis().config else {
            panic!("noise config retained")
        };
        assert_eq!(config.output_node, "out");
        assert_eq!(config.input_source, "VSTIM");
        assert_eq!(config.num_points, 3);
        assert!((config.temperature_kelvin - 233.15).abs() < 1.0e-12);
    }

    #[test]
    fn frozen_plan_ids_revisions_and_exact_dependency_bindings_reach_prepared_tasks() {
        let mut state = AppState::default();
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("new state owns a stable plan");
        let (op, _) = plan
            .insert(AnalysisKind::OperatingPoint)
            .expect("OP inserts");
        let (first_pss, _) = plan.insert(AnalysisKind::Pss).expect("first PSS inserts");
        let (second_pss, _) = plan.insert(AnalysisKind::Pss).expect("second PSS inserts");
        plan.edit(first_pss, |draft| {
            let AnalysisDraft::Pss(draft) = draft else {
                panic!("expected PSS draft");
            };
            draft.fund_freq = "1Meg".to_owned();
            // A driven solve needs a tone, and only the design can name one.
            draft.tone_sources = "VSRC".to_owned();
        })
        .expect("first PSS edits");
        plan.edit(second_pss, |draft| {
            let AnalysisDraft::Pss(draft) = draft else {
                panic!("expected PSS draft");
            };
            draft.fund_freq = "2Meg".to_owned();
            draft.tone_sources = "VSRC".to_owned();
        })
        .expect("second PSS edits");
        plan.bind_dependency(first_pss, AnalysisKind::OperatingPoint, op)
            .expect("first PSS binds OP");
        plan.bind_dependency(second_pss, AnalysisKind::OperatingPoint, op)
            .expect("second PSS binds OP");
        let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
        plan.bind_dependency(pac, AnalysisKind::Pss, first_pss)
            .expect("PAC binds exact first PSS");
        let expected_revision = plan.revision();

        let controller = SimulationController::new();
        let frozen = controller
            .build_analysis_plan(&state)
            .expect("plan freezes");
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .expect("default model sources seal");
        let tasks = controller
            .build_queue_from_plan(&state, &frozen, &sealed)
            .expect("frozen plan compiles");
        let pac_task = tasks
            .iter()
            .find(|task| task.instance_id() == pac)
            .expect("PAC task is present");

        assert_eq!(pac_task.source_revision(), expected_revision);
        assert_eq!(pac_task.dependencies(), &[first_pss]);
        let pac_options = pac_task
            .queued_analysis()
            .spec_options
            .pac
            .as_ref()
            .expect("PAC options compile");
        assert!((pac_options.pss_fundamental_freq - 1.0e6).abs() < 1.0e-6);
        assert!((pac_options.pss_fundamental_freq - 2.0e6).abs() > 1.0);
    }

    #[test]
    fn same_kind_sparameter_instances_freeze_independent_export_policies() {
        let mut state = AppState::default();
        state.schematic.current_file = Some(std::path::PathBuf::from("rf/duplexer.rsch"));
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("new state owns a stable plan");
        let (op_id, _) = plan
            .insert(AnalysisKind::OperatingPoint)
            .expect("operating-point prerequisite inserts");
        let (v1_id, _) = plan
            .insert(AnalysisKind::SParameter)
            .expect("first S-parameter analysis inserts");
        let (disabled_id, _) = plan
            .insert(AnalysisKind::SParameter)
            .expect("second S-parameter analysis inserts");
        plan.bind_dependency(v1_id, AnalysisKind::OperatingPoint, op_id)
            .expect("first S-parameter analysis binds OP");
        plan.bind_dependency(disabled_id, AnalysisKind::OperatingPoint, op_id)
            .expect("second S-parameter analysis binds OP");
        plan.edit(v1_id, |draft| {
            let AnalysisDraft::SParameter(draft) = draft else {
                panic!("expected S-parameter draft");
            };
            draft.touchstone_export = true;
            draft.touchstone_version = 1;
        })
        .expect("first policy edits");
        plan.edit(disabled_id, |draft| {
            let AnalysisDraft::SParameter(draft) = draft else {
                panic!("expected S-parameter draft");
            };
            draft.touchstone_export = false;
            draft.touchstone_version = 2;
        })
        .expect("second policy edits");

        let controller = SimulationController::new();
        let frozen = controller
            .build_analysis_plan(&state)
            .expect("plan freezes");
        let sealed = state
            .model_library_manager
            .seal_execution_sources()
            .expect("default model sources seal");
        let tasks = controller
            .build_queue_from_plan(&state, &frozen, &sealed)
            .expect("same-kind S-parameter tasks compile");
        let first_policy = tasks
            .iter()
            .find(|task| task.instance_id() == v1_id)
            .and_then(|task| task.touchstone_export_policy())
            .expect("first task freezes an explicit policy");
        let second_policy = tasks
            .iter()
            .find(|task| task.instance_id() == disabled_id)
            .and_then(|task| task.touchstone_export_policy())
            .expect("second task freezes an explicit policy");

        assert_eq!(first_policy.version(), Some(1));
        assert!(first_policy.output_path(4, 1, 2).is_some());
        assert_eq!(second_policy.version(), None);
        assert!(second_policy.output_path(4, 2, 2).is_none());
    }
}
