use super::{AppState, ConsoleMessage};
use crate::io::{ProjectExecutionContext, ProjectSimulationResults};

impl serde::Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Safe mode may intentionally replace the visible project/session for
        // this process. Persist the exact pre-safe-mode payload so an
        // isolation launch can never erase document restoration state,
        // working buffers, recent files, or the prior workspace layout.
        if let Some(preserved) = self.state_safe_mode_session_snapshot() {
            let snapshot: serde_json::Value = serde_json::from_str(preserved).map_err(|error| {
                <S::Error as serde::ser::Error>::custom(format!(
                    "safe-mode session snapshot is invalid: {error}"
                ))
            })?;
            return serde::Serialize::serialize(&snapshot, serializer);
        }

        // Serialize durable state needed for session recovery. Runtime runner
        // flags stay out of the session; only user-visible result history is
        // persisted through the project-file DTO.
        use serde::ser::SerializeStruct;
        let mut simulation_results = ProjectSimulationResults::from_state(&self.simulation);
        if simulation_results.validate().is_err() {
            simulation_results = ProjectSimulationResults::default();
        }
        let execution_context =
            ProjectExecutionContext::from_state(&self.sim_setup, &self.model_library_manager)
                .map_err(|error| {
                    <S::Error as serde::ser::Error>::custom(format!(
                        "session execution context is structurally invalid: {error}"
                    ))
                })?;
        let field_count = if simulation_results.is_empty() { 7 } else { 8 };
        let mut state = serializer.serialize_struct("AppState", field_count)?;
        state.serialize_field("project_workspace", &self.workspace)?;
        state.serialize_field("library_manager", &self.library_manager)?;
        state.serialize_field(
            "ui_session",
            &crate::workbench::UiSessionStateSer::from(&self.ui),
        )?;
        state.serialize_field("workbench", &self.workbench)?;
        state.serialize_field("recent_files", &self.recent_files)?;
        state.serialize_field("license_key", &self.license_key)?;
        state.serialize_field("execution_context", &execution_context)?;
        if !simulation_results.is_empty() {
            state.serialize_field("simulation_results", &simulation_results)?;
        }
        state.end()
    }
}

impl AppState {
    fn state_safe_mode_session_snapshot(&self) -> Option<&str> {
        self.workbench.safe_mode.preserved_session()
    }
}

impl<'de> serde::Deserialize<'de> for AppState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Unknown fields from older sessions (panel layout, the retired
        // viewer workspace) are ignored.
        #[derive(serde::Deserialize)]
        struct AppStateDe {
            #[serde(default)]
            project_workspace: crate::state::ProjectWorkspace,
            #[serde(default = "default_library_manager")]
            library_manager: crate::state::LibraryManager,
            // `shell` is a read-only alias for sessions written before the
            // clean-room workbench migration.
            #[serde(default, rename = "ui_session", alias = "shell")]
            ui: crate::workbench::UiSessionStateSer,
            #[serde(default)]
            workbench: crate::workbench::WorkbenchState,
            #[serde(default)]
            recent_files: Vec<super::RecentFile>,
            #[serde(default)]
            license_key: Option<String>,
            #[serde(default)]
            simulation_results: ProjectSimulationResults,
            // Keep this as raw JSON so a corrupt/future execution context can
            // be rejected independently without discarding document recovery,
            // recent files, or the rest of the session.
            #[serde(default)]
            execution_context: Option<serde_json::Value>,
        }

        // Deserialize minimal persisted data and use defaults for the rest.
        let de = AppStateDe::deserialize(deserializer)?;
        let mut library_manager = de.library_manager;
        let mut project_workspace = de.project_workspace;
        project_workspace.ensure_library_model(&mut library_manager);
        let schematic = project_workspace
            .active_context_schematic()
            .cloned()
            .unwrap_or_default();
        // Re-verify the stored key; the grant itself is never trusted from disk.
        let license = de
            .license_key
            .as_deref()
            .and_then(|key| crate::services::license::parse_and_verify(key).ok());
        let mut state = Self {
            schematic,
            workspace: project_workspace,
            library_manager,
            ui: de.ui.into(),
            workbench: de.workbench,
            recent_files: de.recent_files,
            license_key: de.license_key,
            license,
            ..Default::default()
        };
        let execution_warnings = match de.execution_context {
            Some(value) => {
                let restored = serde_json::from_value::<ProjectExecutionContext>(value)
                    .map_err(|error| error.to_string())
                    .and_then(ProjectExecutionContext::into_state);
                match restored {
                    Ok((simulation_plan, model_library_manager, warnings)) => {
                        state.sim_setup = simulation_plan;
                        state.model_library_manager = model_library_manager;
                        warnings
                    }
                    Err(error) => vec![format!(
                        "Simulation plan and model libraries were not restored because their persisted session data is invalid: {error}; documented defaults were loaded instead"
                    )],
                }
            }
            None => vec![
                "This legacy session predates durable simulation plans; RSpice initialized the documented default Transient plan and built-in model catalog"
                    .to_owned(),
            ],
        };
        let simulation_results_warning = de
            .simulation_results
            .apply_to_state(&mut state.simulation)
            .err()
            .map(|error| {
                format!(
                    "Simulation results were not restored because their persisted session data is invalid: {error}"
                )
            });
        if let Some(warning) = simulation_results_warning {
            state.push_user_message(ConsoleMessage::warning(warning));
        }
        for warning in execution_warnings {
            state.push_user_message(ConsoleMessage::warning(warning));
        }
        state.workspace.save_active_schematic(&state.schematic);
        Ok(state)
    }
}

fn default_library_manager() -> crate::state::LibraryManager {
    crate::state::LibraryManager::with_primitives()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_state_session_round_trip_restores_results_but_not_runtime_flags() {
        let mut state = AppState::default();
        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 0.75, 1.5],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(3);
        run.label = "Run 3 (session fixture)".to_string();
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 3;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        state.simulation.is_running = true;
        state.simulation.trigger_abort = true;

        let json = serde_json::to_string(&state).expect("session serializes");
        assert!(json.contains("simulation_results"));
        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");

        assert_eq!(restored.simulation.run_count(), 1);
        assert_eq!(
            restored
                .simulation
                .active_run()
                .expect("active restored run")
                .label,
            "Run 3 (session fixture)"
        );
        assert_eq!(restored.simulation.waveforms[0].name, "V(out)");
        assert!(!restored.simulation.is_running);
        assert!(!restored.simulation.trigger_abort);
    }

    #[test]
    fn legacy_session_without_results_loads_empty_result_history() {
        let restored: AppState = serde_json::from_str("{}").expect("legacy session loads");

        assert_eq!(restored.simulation.run_count(), 0);
        assert!(restored.simulation.waveforms.is_empty());
        assert!(!restored.simulation.is_running);
        assert!(!restored.ui.browser_spoken_feedback);
        assert!(restored.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("legacy session predates durable simulation plans")
        }));
    }

    #[test]
    fn session_round_trip_preserves_unsaved_plan_drafts_and_model_catalog() {
        use crate::common::simulation_analysis_tabs::{TAB_AC, TAB_TRANSIENT};
        use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

        let mut state = AppState::default();
        state.sim_setup.ensure_initialized();
        state.sim_setup.enabled.insert(TAB_AC);
        state.sim_setup.analysis_order = vec![TAB_AC, TAB_TRANSIENT];
        state.sim_setup.tran.stop = "unfinished(".to_owned();
        state.sim_setup.ac.points = "also unfinished".to_owned();
        state.model_library_manager.clear();
        let mut library = ModelLibrary::new("unsaved_session_models");
        let mut model = DeviceModel::new("nch_session", ModelType::Nmos);
        model.add_parameter("kp", 1.25e-3);
        library.add_model(model);
        state.model_library_manager.add_library(library);

        let json = serde_json::to_string(&state).expect("session serializes");
        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");

        assert_eq!(
            restored.sim_setup.analysis_order,
            vec![TAB_AC, TAB_TRANSIENT]
        );
        assert_eq!(restored.sim_setup.tran.stop, "unfinished(");
        assert_eq!(restored.sim_setup.ac.points, "also unfinished");
        assert!(restored.sim_setup.validation_error(TAB_TRANSIENT).is_some());
        assert!(restored.sim_setup.validation_error(TAB_AC).is_some());
        assert!(
            restored
                .model_library_manager
                .get_library("unsaved_session_models")
                .and_then(|library| library.get_model("nch_session"))
                .is_some()
        );
    }

    #[test]
    fn browser_spoken_feedback_preference_round_trips() {
        let mut state = AppState::default();
        state.ui.browser_spoken_feedback = true;

        let json = serde_json::to_string(&state).expect("session serializes");
        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");

        assert!(restored.ui.browser_spoken_feedback);
    }

    #[test]
    fn current_sessions_write_ui_session_and_keep_shell_read_only() {
        let json = serde_json::to_string(&AppState::default()).expect("session serializes");

        assert!(json.contains("\"ui_session\""));
        assert!(!json.contains("\"shell\""));
    }

    #[test]
    fn safe_mode_persists_the_exact_pre_isolation_session() {
        let mut state = AppState::default();
        state
            .workspace
            .project
            .rename("Session before safe mode")
            .expect("valid project name");
        let before = serde_json::to_string(&state).expect("baseline session serializes");

        state.workspace.project.rename("Isolated session").unwrap();
        state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions::default(),
            before.clone(),
        );
        let persisted = serde_json::to_string(&state).expect("safe-mode session serializes");
        let restored: AppState = serde_json::from_str(&persisted).expect("snapshot restores");

        assert_eq!(
            restored.workspace.project.name(),
            "Session before safe mode"
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&persisted).unwrap(),
            serde_json::from_str::<serde_json::Value>(&before).unwrap()
        );
    }

    #[test]
    fn legacy_session_result_sequences_migrate_to_stable_identities() {
        let mut state = AppState::default();
        let mut run = crate::state::SimulationRun::new(3);
        run.add_analysis(crate::state::AnalysisResult::new(
            7,
            crate::state::AnalysisType::Transient,
            "TRAN legacy session",
        ));
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 3;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        let mut value = serde_json::to_value(&state).expect("session converts to JSON");
        let results = value["simulation_results"]
            .as_object_mut()
            .expect("simulation result object");
        results.insert("schema_version".to_owned(), serde_json::Value::from(1));
        results.remove("active_run_stable_id");
        results.remove("active_dataset_id");
        results.remove("active_analysis_sequence");
        results.insert("active_run_id".to_owned(), serde_json::Value::from(3));
        results.insert("active_analysis_id".to_owned(), serde_json::Value::from(7));
        let persisted_run = results["runs"][0]
            .as_object_mut()
            .expect("persisted run object");
        persisted_run.remove("run_id");
        persisted_run.remove("dataset_id");

        let restored: AppState = serde_json::from_value(value).expect("legacy session migrates");

        let active_run = restored
            .simulation
            .active_run()
            .expect("active run restores");
        let active_analysis = restored
            .simulation
            .active_analysis()
            .expect("active analysis restores");
        assert!(!active_run.run_id.as_uuid().is_nil());
        assert!(!active_run.dataset_id.as_uuid().is_nil());
        assert_eq!(active_run.id, 3);
        assert_eq!(active_analysis.id, 7);
    }

    #[test]
    fn app_state_session_omits_empty_result_history() {
        let state = AppState::default();

        let json = serde_json::to_string(&state).expect("session serializes");

        assert!(!json.contains("simulation_results"));
    }

    #[test]
    fn app_state_session_omits_invalid_result_history() {
        let mut state = AppState::default();
        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, f64::NAN],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(3);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 3;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        let json = serde_json::to_string(&state).expect("session serializes");
        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");

        assert!(!json.contains("simulation_results"));
        assert_eq!(restored.simulation.run_count(), 0);
    }

    #[test]
    fn app_state_session_does_not_persist_waves_expr_traces() {
        let mut state = AppState::default();
        state.ui.results.exprs.insert(
            0,
            vec![crate::workbench::result_document::ExprTrace {
                text: "V(out)/V(in)".to_string(),
                visible: true,
            }],
        );

        let json = serde_json::to_string(&state).expect("session serializes");

        assert!(
            !json.contains("expr_traces"),
            "project-scoped Waves expressions must not be stored in UI session JSON: {json}"
        );
        assert!(
            !json.contains("V(out)/V(in)"),
            "project-scoped Waves expression text leaked into session JSON: {json}"
        );
    }

    #[test]
    fn legacy_session_expr_traces_are_ignored_on_load() {
        let json = r#"{
            "shell": {
                "result_viewer": "Waves",
                "expr_traces": [[0, "V(out)/V(in)"]]
            }
        }"#;

        let restored: AppState = serde_json::from_str(json).expect("legacy session loads");

        assert!(
            restored.ui.results.exprs.is_empty(),
            "legacy project-scoped Waves traces must not be restored into a new session"
        );
    }

    #[test]
    fn app_state_session_drops_invalid_result_history_on_load() {
        let mut state = AppState::default();
        let mut run = crate::state::SimulationRun::new(3);
        run.add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "TRAN",
        ));
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 3;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        let mut value = serde_json::to_value(&state).expect("session serializes");
        value["simulation_results"]["schema_version"] = serde_json::Value::from(999);
        let json = serde_json::to_string(&value).expect("mutated session serializes");

        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");

        assert_eq!(restored.simulation.run_count(), 0);
        assert!(restored.simulation.waveforms.is_empty());
        assert!(restored.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("Simulation results were not restored")
        }));
    }
}
