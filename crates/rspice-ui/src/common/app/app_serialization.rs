use super::{AppState, ConsoleMessage};
use crate::io::{ProjectExecutionContext, ProjectFile, ProjectSimulationResults};

const LEGACY_SESSION_PROJECT_ID_NAMESPACE: uuid::Uuid =
    uuid::Uuid::from_u128(0x655d_ae12_6cdd_5971_b7d8_aafe_02b6_b367);

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
        let execution_context = ProjectExecutionContext::from_state(
            self.workspace.project.id(),
            &self.sim_setup,
            &self.model_library_manager,
        )
        .map_err(|error| {
            <S::Error as serde::ser::Error>::custom(format!(
                "session execution context is structurally invalid: {error}"
            ))
        })?;
        if ProjectFile::validate_result_plan_references_for(
            &simulation_results,
            Some(&execution_context.simulation_plan),
            self.workspace.project.revision(),
        )
        .is_err()
        {
            simulation_results = ProjectSimulationResults::default();
        }
        let field_count = if simulation_results.is_empty() { 9 } else { 10 };
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
        state.serialize_field(
            "native_project_binding_receipt",
            &self.native_project_binding_receipt,
        )?;
        state.serialize_field(
            "browser_project_binding_receipt",
            &self.browser_project_binding_receipt,
        )?;
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
            #[serde(default)]
            native_project_binding_receipt: Option<serde_json::Value>,
            #[serde(default)]
            browser_project_binding_receipt: Option<serde_json::Value>,
        }

        // Capture the complete session artifact before nested defaults run.
        // Genuine legacy sessions did not persist a project identity; scoping
        // their migration to the whole session prevents distinct unsaved
        // workspaces with identical project labels from aliasing each other.
        let mut session = serde_json::Value::deserialize(deserializer)?;
        inject_legacy_session_project_identity(&mut session).map_err(serde::de::Error::custom)?;

        // Deserialize minimal persisted data and use defaults for the rest.
        let de = AppStateDe::deserialize(session).map_err(serde::de::Error::custom)?;
        let mut library_manager = de.library_manager;
        let mut project_workspace = de.project_workspace;
        let project_id = project_workspace.project.id();
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
        let (native_project_binding_receipt, native_receipt_warning) = decode_session_authority(
            de.native_project_binding_receipt,
            "native project binding receipt",
        );
        let (browser_project_binding_receipt, browser_receipt_warning) = decode_session_authority(
            de.browser_project_binding_receipt,
            "browser project binding receipt",
        );
        let mut state = Self {
            schematic,
            workspace: project_workspace,
            library_manager,
            ui: de.ui.into(),
            workbench: de.workbench,
            recent_files: de.recent_files,
            license_key: de.license_key,
            license,
            native_project_binding_receipt,
            browser_project_binding_receipt,
            ..Default::default()
        };
        state.workbench.reconcile_restored_navigation();
        let navigation_warning = state.workbench.take_route_diagnostic();
        let execution_warnings = match de.execution_context {
            Some(value) => {
                let restored = serde_json::from_value::<ProjectExecutionContext>(value)
                    .map_err(|error| error.to_string())
                    .and_then(|context| context.into_state(project_id));
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
        let mut simulation_results = de.simulation_results;
        let simulation_results_warning = simulation_results
            .migrate_to_current(project_id)
            .and_then(|()| simulation_results.validate())
            .and_then(|()| {
                ProjectFile::validate_result_plan_references_for(
                    &simulation_results,
                    Some(&state.sim_setup),
                    state.workspace.project.revision(),
                )
            })
            .and_then(|()| simulation_results.apply_to_state(&mut state.simulation))
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
        for warning in [native_receipt_warning, browser_receipt_warning]
            .into_iter()
            .flatten()
        {
            state.push_user_message(ConsoleMessage::warning(warning));
        }
        if let Some(warning) = navigation_warning {
            state.push_user_message(ConsoleMessage::warning(warning));
        }
        state.workspace.save_active_schematic(&state.schematic);
        Ok(state)
    }
}

fn decode_session_authority<T>(
    value: Option<serde_json::Value>,
    label: &str,
) -> (Option<T>, Option<String>)
where
    T: serde::de::DeserializeOwned,
{
    let Some(value) = value else {
        return (None, None);
    };
    match serde_json::from_value(value) {
        Ok(receipt) => (Some(receipt), None),
        Err(error) => (
            None,
            Some(format!(
                "Ignored an invalid or unsupported {label}: {error}. Recovered project documents and working buffers were preserved without persistence authority"
            )),
        ),
    }
}

fn default_library_manager() -> crate::state::LibraryManager {
    crate::state::LibraryManager::with_primitives()
}

fn inject_legacy_session_project_identity(session: &mut serde_json::Value) -> Result<(), String> {
    let canonical_material = canonical_session_identity_value(session);
    let canonical_bytes = serde_json::to_vec(&canonical_material)
        .map_err(|error| format!("legacy session identity could not be encoded: {error}"))?;
    let migrated_id = crate::product::ProjectId::from_namespace(
        LEGACY_SESSION_PROJECT_ID_NAMESPACE,
        &canonical_bytes,
    );

    let session_object = session
        .as_object_mut()
        .ok_or_else(|| "persisted application session must be a JSON object".to_owned())?;
    if !session_object.contains_key("project_workspace") {
        let mut workspace = serde_json::to_value(crate::state::ProjectWorkspace::default())
            .map_err(|error| format!("default project workspace could not be encoded: {error}"))?;
        workspace["project"]["id"] = serde_json::Value::String(migrated_id.to_string());
        session_object.insert("project_workspace".to_owned(), workspace);
        return Ok(());
    }

    let Some(descriptor) = session_object
        .get_mut("project_workspace")
        .and_then(|workspace| workspace.get_mut("project"))
        .and_then(serde_json::Value::as_object_mut)
    else {
        return Ok(());
    };
    if !descriptor.contains_key("schema_version") && !descriptor.contains_key("id") {
        descriptor.insert(
            "id".to_owned(),
            serde_json::Value::String(migrated_id.to_string()),
        );
    }
    Ok(())
}

fn canonical_session_identity_value(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Array(values) => serde_json::Value::Array(
            values
                .iter()
                .map(canonical_session_identity_value)
                .collect(),
        ),
        serde_json::Value::Object(values) => {
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
            let mut canonical = serde_json::Map::new();
            for (key, value) in entries {
                canonical.insert(key.clone(), canonical_session_identity_value(value));
            }
            serde_json::Value::Object(canonical)
        }
        scalar => scalar.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seal_legacy_unattributed(run: &mut crate::state::SimulationRun) {
        run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
            .expect("legacy fixture provenance seals");
    }

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
        seal_legacy_unattributed(&mut run);
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
    fn session_round_trip_preserves_exact_browser_binding_receipt() {
        let mut state = AppState::default();
        let receipt = crate::common::project_lifecycle::BrowserBindingReceipt {
            binding_id: uuid::Uuid::from_u128(0xc23c_8916_2865_430a_a612_ecbb_111b_3ce1),
            project_id: state.workspace.project.id().to_string(),
            accepted_generation: 23,
            accepted_digest: "ab".repeat(32).parse().expect("valid digest fixture"),
            backend: crate::common::project_lifecycle::BrowserBindingBackend::Opfs,
        };
        state.browser_project_binding_receipt = Some(receipt.clone());

        let json = serde_json::to_string(&state).expect("session serializes");
        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");

        assert_eq!(restored.browser_project_binding_receipt, Some(receipt));
    }

    #[test]
    fn session_round_trip_preserves_exact_native_binding_receipt() {
        let mut state = AppState::default();
        let receipt = crate::common::project_lifecycle::NativeBindingReceipt {
            canonical_path: std::path::PathBuf::from(r"C:\projects\precision-afe.rspiceproj"),
            project_id: state.workspace.project.id().to_string(),
            accepted_digest: crate::product::ContentDigest::from_bytes([0x5a; 32]),
        };
        state.native_project_binding_receipt = Some(receipt.clone());

        let json = serde_json::to_string(&state).expect("session serializes");
        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");

        assert_eq!(restored.native_project_binding_receipt, Some(receipt));
    }

    #[test]
    fn malformed_or_future_receipts_drop_only_authority_and_preserve_working_session() {
        let mut state = AppState::default();
        state
            .workspace
            .project
            .rename("Recovered receipt-corruption fixture")
            .expect("valid fixture name");
        state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(17, 29),
        );
        state.workspace.save_active_schematic(&state.schematic);
        let active_key = state.workspace.active_key();

        let mut session = serde_json::to_value(&state).expect("serialize recovery fixture");
        let object = session.as_object_mut().expect("session object");
        object.insert(
            "native_project_binding_receipt".to_owned(),
            serde_json::json!({
                "canonical_path": 42,
                "project_id": null,
                "accepted_digest": "not-a-digest"
            }),
        );
        object.insert(
            "browser_project_binding_receipt".to_owned(),
            serde_json::json!({
                "binding_id": uuid::Uuid::new_v4(),
                "project_id": state.workspace.project.id().to_string(),
                "accepted_generation": 7,
                "accepted_digest": "11".repeat(32),
                "backend": "future-cloud-authority"
            }),
        );

        let restored: AppState = serde_json::from_value(session)
            .expect("receipt corruption is independently recoverable");
        assert_eq!(
            restored.workspace.project.name(),
            "Recovered receipt-corruption fixture"
        );
        assert_eq!(
            restored
                .workspace
                .schematic_buffers
                .get(&active_key)
                .expect("working schematic buffer survives")
                .components
                .len(),
            1
        );
        assert!(restored.native_project_binding_receipt.is_none());
        assert!(restored.browser_project_binding_receipt.is_none());
        let messages = restored
            .log_buffer
            .entries()
            .map(|entry| entry.message.as_str())
            .collect::<Vec<_>>();
        assert!(
            messages
                .iter()
                .any(|message| message.contains("native project binding receipt"))
        );
        assert!(
            messages
                .iter()
                .any(|message| message.contains("browser project binding receipt"))
        );
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
    fn legacy_session_without_project_id_restores_a_reproducible_identity() {
        let state = AppState::default();
        let original_id = state.workspace.project.id();
        let mut session = serde_json::to_value(&state).expect("session serializes");
        session["project_workspace"]["project"]
            .as_object_mut()
            .expect("project descriptor object")
            .remove("id");
        session["project_workspace"]["project"]
            .as_object_mut()
            .expect("project descriptor object")
            .remove("schema_version");
        session["project_workspace"]["project"]
            .as_object_mut()
            .expect("project descriptor object")
            .remove("revision");
        session
            .as_object_mut()
            .expect("session object")
            .remove("execution_context");
        let legacy_json = serde_json::to_string(&session).expect("legacy session serializes");

        let first: AppState = serde_json::from_str(&legacy_json).expect("legacy session restores");
        let second: AppState =
            serde_json::from_str(&legacy_json).expect("identical legacy session restores");

        assert_eq!(first.workspace.project.id(), second.workspace.project.id());
        assert_ne!(first.workspace.project.id(), original_id);
        assert!(!first.workspace.project.id().as_uuid().is_nil());
    }

    #[test]
    fn distinct_legacy_session_workspaces_do_not_alias_project_identity() {
        fn legacy_value(mut state: AppState) -> serde_json::Value {
            state.workspace.save_active_schematic(&state.schematic);
            let mut value = serde_json::to_value(state).expect("session serializes");
            let descriptor = value["project_workspace"]["project"]
                .as_object_mut()
                .expect("project descriptor object");
            descriptor.remove("id");
            descriptor.remove("schema_version");
            descriptor.remove("revision");
            value
                .as_object_mut()
                .expect("session object")
                .remove("execution_context");
            value
        }

        let first = AppState::default();
        let mut second = AppState::default();
        second.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(40, 40),
        );
        let first_value = legacy_value(first);
        let second_value = legacy_value(second);

        let first_restored: AppState =
            serde_json::from_value(first_value.clone()).expect("first legacy session restores");
        let first_replay: AppState =
            serde_json::from_value(first_value).expect("identical legacy session replays");
        let second_restored: AppState =
            serde_json::from_value(second_value).expect("second legacy session restores");

        assert_eq!(
            first_restored.workspace.project.id(),
            first_replay.workspace.project.id()
        );
        assert_ne!(
            first_restored.workspace.project.id(),
            second_restored.workspace.project.id()
        );
    }

    #[test]
    fn unversioned_session_with_explicitly_null_project_identity_is_rejected() {
        let state = AppState::default();
        let mut session = serde_json::to_value(state).expect("session serializes");
        let descriptor = session["project_workspace"]["project"]
            .as_object_mut()
            .expect("project descriptor object");
        descriptor.remove("schema_version");
        descriptor.insert("id".to_owned(), serde_json::Value::Null);

        let error = match serde_json::from_value::<AppState>(session) {
            Ok(_) => panic!("explicit null project identity must not migrate as legacy absence"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("must not be explicitly null"));
    }

    #[test]
    fn session_round_trip_preserves_unsaved_plan_drafts_and_model_catalog() {
        use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
        use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

        let mut state = AppState::default();
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current session owns a stable plan");
        let transient_id = plan.instances()[0].id();
        plan.edit(transient_id, |draft| {
            let AnalysisDraft::Transient(transient) = draft else {
                panic!("default instance must be transient");
            };
            transient.stop = "unfinished(".to_owned();
        })
        .expect("transient draft edit commits");
        let (op_id, _) = plan
            .insert_at(AnalysisKind::OperatingPoint, 0)
            .expect("OP inserts before dependent analyses");
        let (ac_id, _) = plan
            .insert_at(AnalysisKind::Ac, 1)
            .expect("AC inserts in explicit order");
        plan.edit(ac_id, |draft| {
            let AnalysisDraft::Ac(ac) = draft else {
                panic!("inserted instance must be AC");
            };
            ac.points = "also unfinished".to_owned();
        })
        .expect("AC draft edit commits");
        plan.bind_dependency(ac_id, AnalysisKind::OperatingPoint, op_id)
            .expect("AC binds the exact OP identity");
        state.model_library_manager.clear();
        let mut library = ModelLibrary::new("unsaved_session_models");
        let mut model = DeviceModel::new("nch_session", ModelType::Nmos);
        model.add_parameter("kp", 1.25e-3);
        library.add_model(model);
        state.model_library_manager.add_library(library);

        let json = serde_json::to_string(&state).expect("session serializes");
        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");
        let restored_plan = restored
            .sim_setup
            .stable_analysis_plan()
            .expect("session restores stable plan");
        assert_eq!(
            restored_plan
                .instances()
                .iter()
                .map(|instance| instance.id())
                .collect::<Vec<_>>(),
            vec![op_id, ac_id, transient_id]
        );
        let ac = restored_plan.instance(ac_id).expect("AC identity retained");
        let AnalysisDraft::Ac(ac_draft) = ac.draft() else {
            panic!("AC identity must retain its draft kind");
        };
        assert_eq!(ac_draft.points, "also unfinished");
        assert_eq!(ac.dependencies().len(), 1);
        assert_eq!(ac.dependencies()[0].target(), op_id);
        let transient = restored_plan
            .instance(transient_id)
            .expect("transient identity retained");
        let AnalysisDraft::Transient(transient_draft) = transient.draft() else {
            panic!("transient identity must retain its draft kind");
        };
        assert_eq!(transient_draft.stop, "unfinished(");
        let value: serde_json::Value = serde_json::from_str(&json).expect("session JSON parses");
        let persisted = &value["execution_context"]["simulation_plan"];
        for retired in ["enabled", "analysis_order", "listed", "tran", "ac", "op"] {
            assert!(
                persisted.get(retired).is_none(),
                "v4 session must omit retired singleton field {retired}"
            );
        }
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
    fn appearance_preferences_round_trip_through_the_application_session() {
        let mut state = AppState::default();
        state.ui.theme.mode = crate::ui::Mode::System;
        state.ui.theme.density = crate::ui::Density::Relaxed;
        state.ui.theme.colorblind_traces = true;
        state.ui.theme.canvas_contrast = 84;
        state.ui.theme.canvas_theme = crate::ui::EngineeringCanvasTheme::Light;

        let json = serde_json::to_string(&state).expect("session serializes");
        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");

        assert_eq!(restored.ui.theme.mode, crate::ui::Mode::System);
        assert_eq!(restored.ui.theme.density, crate::ui::Density::Relaxed);
        assert!(restored.ui.theme.colorblind_traces);
        assert_eq!(restored.ui.theme.canvas_contrast, 84);
        assert_eq!(
            restored.ui.theme.canvas_theme,
            crate::ui::EngineeringCanvasTheme::Light
        );
    }

    #[test]
    fn legacy_autosave_interval_is_migrated_before_the_session_is_rewritten() {
        let mut state = AppState::default();
        state.ui.autosave_minutes = 15;

        let json = serde_json::to_string(&state).expect("session serializes");
        let restored: AppState = serde_json::from_str(&json).expect("session deserializes");

        assert_eq!(restored.ui.autosave_minutes, 10);
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
        seal_legacy_unattributed(&mut run);
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
        persisted_run.remove("provenance_mode");

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

    #[test]
    fn app_state_session_drops_plan_result_with_an_orphaned_source_identity() {
        let mut state = AppState::default();
        let plan = state
            .sim_setup
            .stable_analysis_plan()
            .expect("default session has a stable plan");
        let transient = plan.instances().first().expect("default transient");
        let plan_id = plan.id();
        let source_instance_id = transient.id();
        let source_revision = plan.revision();
        let project_revision = state.workspace.project.revision();
        let prepared_snapshot_digest = crate::product::ContentDigest::from_bytes([0x73; 32]);
        let mut run = crate::state::SimulationRun::new(4);
        run.add_analysis(
            crate::state::AnalysisResult::new(
                1,
                crate::state::AnalysisType::Transient,
                "Prepared TRAN",
            )
            .with_provenance(
                crate::state::AnalysisResultProvenance::new(
                    source_instance_id,
                    source_revision,
                    prepared_snapshot_digest,
                    Vec::new(),
                )
                .expect("prepared provenance"),
            ),
        );
        let task_receipt = crate::state::PreparedRunTaskReceipt::new(
            source_instance_id,
            source_revision,
            Vec::new(),
            5,
            crate::product::ContentDigest::from_bytes([0x74; 32]),
        )
        .expect("prepared transient task receipt");
        let run_receipt = crate::state::PreparedRunReceipt::new(
            crate::state::AnalysisResultSourceDomain::SimulationPlan,
            Some(plan_id),
            project_revision,
            prepared_snapshot_digest,
            crate::product::ContentDigest::from_bytes([0x75; 32]),
            crate::state::PreparedSourceCheckReceipt::SchematicDrc(
                crate::product::ContentDigest::from_bytes([0x76; 32]),
            ),
            vec![task_receipt],
        )
        .expect("prepared plan run receipt");
        run.restore_provenance(crate::state::SimulationRunProvenance::Prepared(run_receipt))
            .expect("prepared run fixture seals");
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 4;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        let mut session = serde_json::to_value(&state).expect("session serializes");
        let orphaned_identity = crate::product::AnalysisInstanceId::new();
        session["simulation_results"]["runs"][0]["analyses"][0]["provenance"]["source_instance_id"] =
            serde_json::to_value(orphaned_identity).expect("orphan identity serializes");
        session["simulation_results"]["runs"][0]["prepared_receipt"]["tasks"][0]["source_instance_id"] =
            serde_json::to_value(orphaned_identity).expect("orphan identity serializes");

        let restored: AppState =
            serde_json::from_value(session).expect("orphaned result is recoverable");

        assert_eq!(restored.simulation.run_count(), 0);
        assert!(restored.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("absent from the persisted plan and its tombstones")
        }));
    }
}
