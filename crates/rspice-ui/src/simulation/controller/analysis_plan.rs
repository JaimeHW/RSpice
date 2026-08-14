//! Preflight over a saved analysis plan.
//!
//! Checks that a stored plan still describes something the current design
//! can run — that its saved outputs still exist, and that its analyses are
//! still available — before a run is allowed to start on it.

use super::*;
use std::collections::HashMap;

use crate::simulation::execution::{PreparedDependencyBinding, PreparedTask};
use crate::simulation::plan::FrozenSimulationPlan;

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
                errors.push(format!("{}: {reason}", instance.kind().label()));
                continue;
            }
            projected_state.sim_setup =
                match state.sim_setup.frozen_instance_projection(plan, instance) {
                    Ok(projection) => projection,
                    Err(error) => {
                        errors.push(format!("{}: {error}", instance.kind().label()));
                        continue;
                    }
                };
            let dependency_ids = instance
                .dependencies()
                .iter()
                .map(|dependency| dependency.target())
                .collect();

            let spec = match self.build_manifest_preview_spec(state, instance.draft()) {
                Ok(Some(spec)) => spec,
                Ok(None) => match self
                    .build_analysis_spec_for_index(&projected_state, instance.kind().legacy_index())
                {
                    Ok(spec) => spec,
                    Err(error) => {
                        errors.push(format!("{}: {error}", instance.kind().label()));
                        continue;
                    }
                },
                Err(error) => {
                    errors.push(format!("{}: {error}", instance.kind().label()));
                    continue;
                }
            };
            let analysis_line = match self.analysis_spec_to_spice_line(&projected_state, &spec) {
                Ok(line) => line,
                Err(e) => {
                    errors.push(format!("{}: {}", instance.kind().label(), e));
                    continue;
                }
            };
            let spec_options = match self.analysis_spec_execution_options(
                &projected_state,
                &spec,
                sealed_model_sources,
            ) {
                Ok(opts) => opts,
                Err(e) => {
                    errors.push(format!("{}: {}", instance.kind().label(), e));
                    continue;
                }
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
                    instance.kind().label().to_owned(),
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
                                instance.kind().label(),
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
                        errors.push(format!("{}: {}", instance.kind().label(), e));
                        continue;
                    }
                }
            };
            let mut prepared = PreparedTask::new(
                instance.id(),
                plan.revision(),
                dependency_ids,
                instance.kind().label(),
                task,
            );
            if instance.kind() == crate::simulation::plan::AnalysisKind::SParameter {
                match prepared_run::touchstone_export_policy_for_dialog(
                    &projected_state.sim_setup.sp,
                    state.schematic.current_file.as_deref(),
                ) {
                    Ok(policy) => {
                        prepared = prepared.with_touchstone_export_policy(policy);
                    }
                    Err(error) => {
                        errors.push(format!("{}: {error}", instance.kind().label()));
                        continue;
                    }
                }
            }
            queue.push(prepared);

            if let Some((num_harmonics, analysis_line, spec_options, producer, label)) =
                spectrum_seed
            {
                queue.push(PreparedTask::new(
                    crate::product::AnalysisInstanceId::new(),
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
                ));
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
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
                        ),
                    )
                })
                .collect::<HashMap<_, _>>();
            for task in &mut queue {
                let required_kind = match task.queued_analysis().spec {
                    AnalysisSpec::Fourier { .. } => Some(
                        crate::simulation::execution::ExecutionArtifactKind::TransientTrajectory,
                    ),
                    AnalysisSpec::Pss {
                        method: PssMethod::Shooting,
                        ..
                    } => Some(
                        crate::simulation::execution::ExecutionArtifactKind::DcOperatingPointSeed,
                    ),
                    AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. } => {
                        Some(crate::simulation::execution::ExecutionArtifactKind::HbState)
                    }
                    AnalysisSpec::Pac
                    | AnalysisSpec::Pxf
                    | AnalysisSpec::Pnoise
                    | AnalysisSpec::Pstb
                    | AnalysisSpec::Psp { .. }
                    // The spectrum is a reading of a converged steady state,
                    // so it binds the same artifact its small-signal siblings
                    // do rather than re-solving the period.
                    | AnalysisSpec::PssSpectrum { .. } => {
                        Some(crate::simulation::execution::ExecutionArtifactKind::PeriodicState)
                    }
                    _ => None,
                };
                let Some(required_kind) = required_kind else {
                    continue;
                };
                let producers = task
                    .dependencies()
                    .iter()
                    .filter_map(|dependency| {
                        producer_identities
                            .get(dependency)
                            .filter(|(_, _, transient, pss, hb, op)| match required_kind {
                                crate::simulation::execution::ExecutionArtifactKind::TransientTrajectory => *transient,
                                crate::simulation::execution::ExecutionArtifactKind::PeriodicState => *pss,
                                crate::simulation::execution::ExecutionArtifactKind::HbState => *hb,
                                crate::simulation::execution::ExecutionArtifactKind::DcOperatingPointSeed => *op,
                            })
                            .map(|(revision, config_digest, _, _, _, _)| {
                                match required_kind {
                                    crate::simulation::execution::ExecutionArtifactKind::TransientTrajectory => PreparedDependencyBinding::transient_trajectory(*dependency, *revision, *config_digest),
                                    crate::simulation::execution::ExecutionArtifactKind::PeriodicState => PreparedDependencyBinding::periodic_state(*dependency, *revision, *config_digest),
                                    crate::simulation::execution::ExecutionArtifactKind::HbState => PreparedDependencyBinding::hb_state(*dependency, *revision, *config_digest),
                                    crate::simulation::execution::ExecutionArtifactKind::DcOperatingPointSeed => PreparedDependencyBinding::dc_operating_point_seed(*dependency, *revision, *config_digest),
                                }
                            })
                    })
                    .collect::<Vec<_>>();
                if producers.len() != 1 {
                    errors.push(format!(
                        "{} must bind exactly one prepared {} task, found {}",
                        task.queued_analysis().spec.run_type().display_name(),
                        match required_kind {
                            crate::simulation::execution::ExecutionArtifactKind::TransientTrajectory => "Transient",
                            crate::simulation::execution::ExecutionArtifactKind::PeriodicState => "shooting PSS",
                            crate::simulation::execution::ExecutionArtifactKind::HbState => "Harmonic Balance",
                            crate::simulation::execution::ExecutionArtifactKind::DcOperatingPointSeed => "operating point",
                        },
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

    pub(super) fn analysis_spec_execution_options(
        &self,
        state: &AppState,
        spec: &AnalysisSpec,
        sealed_model_sources: &crate::state::model_library::SealedModelExecutionSources,
    ) -> Result<SpecExecutionOptions, String> {
        match spec {
            AnalysisSpec::Parametric => {
                let mut temp_state = state.sim_setup.temp.clone();
                temp_state.ensure_initialized();
                let temp_cfg = temp_state
                    .to_config()
                    .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;
                Ok(SpecExecutionOptions {
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
                    .to_config(state.sim_setup.reference_pvt)
                    .map_err(|e| format!("invalid corner settings: {}", e))?;
                Ok(SpecExecutionOptions {
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
                temp: None,
                parametric_base: None,
                corner: None,
                pac: Some(Self::pac_run_config_from_dialog(state)?),
                pxf: None,
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Pxf => Ok(SpecExecutionOptions {
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
                temp: None,
                parametric_base: None,
                corner: None,
                pac: None,
                pxf: None,
                pnoise: Some(Self::pnoise_run_config_from_dialog(state)?),
                pstb: None,
            }),
            AnalysisSpec::Pstb => Ok(SpecExecutionOptions {
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
        })
        .expect("first PSS edits");
        plan.edit(second_pss, |draft| {
            let AnalysisDraft::Pss(draft) = draft else {
                panic!("expected PSS draft");
            };
            draft.fund_freq = "2Meg".to_owned();
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
