use super::{AppState, ConsoleMessage};
use crate::io::ProjectSimulationResults;

impl serde::Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize durable state needed for session recovery. Runtime runner
        // flags stay out of the session; only user-visible result history is
        // persisted through the project-file DTO.
        use serde::ser::SerializeStruct;
        let mut simulation_results = ProjectSimulationResults::from_state(&self.simulation);
        if simulation_results.validate().is_err() {
            simulation_results = ProjectSimulationResults::default();
        }
        let field_count = if simulation_results.is_empty() { 5 } else { 6 };
        let mut state = serializer.serialize_struct("AppState", field_count)?;
        state.serialize_field("project_workspace", &self.workspace)?;
        state.serialize_field("library_manager", &self.library_manager)?;
        state.serialize_field("shell", &crate::shell::ShellStateSer::from(&self.shell))?;
        state.serialize_field("recent_files", &self.recent_files)?;
        state.serialize_field("license_key", &self.license_key)?;
        if !simulation_results.is_empty() {
            state.serialize_field("simulation_results", &simulation_results)?;
        }
        state.end()
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
            #[serde(default)]
            shell: crate::shell::ShellStateSer,
            #[serde(default)]
            recent_files: Vec<super::RecentFile>,
            #[serde(default)]
            license_key: Option<String>,
            #[serde(default)]
            simulation_results: ProjectSimulationResults,
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
            shell: de.shell.into(),
            recent_files: de.recent_files,
            license_key: de.license_key,
            license,
            ..Default::default()
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
        state.shell.results.exprs.insert(
            0,
            vec![crate::shell::results::ExprTrace {
                text: "V(out)/V(in)".to_string(),
                visible: true,
            }],
        );

        let json = serde_json::to_string(&state).expect("session serializes");

        assert!(
            !json.contains("expr_traces"),
            "project-scoped Waves expressions must not be stored in shell session JSON: {json}"
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
            restored.shell.results.exprs.is_empty(),
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
        let json = serde_json::to_string(&state)
            .expect("session serializes")
            .replace("\"schema_version\":2", "\"schema_version\":999");

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
