use super::*;
use crate::simulation::execution::PreparedTask;
use crate::simulation::plan::FrozenSimulationPlan;

impl SimulationController {
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

            let spec = match self
                .build_analysis_spec_for_index(&projected_state, instance.kind().legacy_index())
            {
                Ok(spec) => spec,
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

            let task = if Self::executes_via_spec(&spec) {
                QueuedAnalysis {
                    spec,
                    config: None,
                    spec_options,
                    analysis_line,
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
        }

        if errors.is_empty() {
            Ok(queue)
        } else {
            Err(errors)
        }
    }

    pub(super) fn executes_via_spec(spec: &AnalysisSpec) -> bool {
        matches!(
            spec,
            AnalysisSpec::Tf
                | AnalysisSpec::Disto { .. }
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pstb
                | AnalysisSpec::Stb { .. }
                | AnalysisSpec::MonteCarlo
                | AnalysisSpec::Parametric
                | AnalysisSpec::Corner
                | AnalysisSpec::Pss { .. }
                | AnalysisSpec::HarmonicBalance { .. }
                | AnalysisSpec::Pac
                | AnalysisSpec::SParameter { .. }
                | AnalysisSpec::Envelope { .. }
                | AnalysisSpec::Fourier { .. }
                | AnalysisSpec::Reliability { .. }
                | AnalysisSpec::Optimization { .. }
                | AnalysisSpec::Soa { .. }
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
                    corner: None,
                    pac: None,
                    pxf: None,
                    tf: None,
                    pnoise: None,
                    pstb: None,
                })
            }
            AnalysisSpec::Corner => {
                let mut corner_state = state.sim_setup.corner.clone();
                corner_state.ensure_initialized();
                let corner_cfg = corner_state
                    .to_config()
                    .map_err(|e| format!("invalid corner settings: {}", e))?;
                Ok(SpecExecutionOptions {
                    temp: None,
                    corner: Some(Self::corner_run_config_from_dialog(
                        state,
                        &corner_cfg,
                        sealed_model_sources,
                    )?),
                    pac: None,
                    pxf: None,
                    tf: None,
                    pnoise: None,
                    pstb: None,
                })
            }
            AnalysisSpec::Pac => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: Some(Self::pac_run_config_from_dialog(state)?),
                pxf: None,
                tf: None,
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Pxf => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: Some(Self::pxf_run_config_from_dialog(state)?),
                tf: None,
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Tf => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: Some(Self::tf_run_config_from_dialog(state)?),
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Pnoise => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: None,
                pnoise: Some(Self::pnoise_run_config_from_dialog(state)?),
                pstb: None,
            }),
            AnalysisSpec::Pstb => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: None,
                pnoise: None,
                pstb: Some(Self::pstb_run_config_from_dialog(state)?),
            }),
            _ => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: None,
                pnoise: None,
                pstb: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind};

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
