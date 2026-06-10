use super::AppState;

impl serde::Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize minimal state needed for session recovery.
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AppState", 3)?;
        state.serialize_field("project_workspace", &self.workspace)?;
        state.serialize_field("library_manager", &self.library_manager)?;
        state.serialize_field("shell", &crate::shell::ShellStateSer::from(&self.shell))?;
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
        }

        // Deserialize minimal persisted data and use defaults for the rest.
        let de = AppStateDe::deserialize(deserializer)?;
        let mut library_manager = de.library_manager;
        let mut project_workspace = de.project_workspace;
        project_workspace.ensure_library_model(&mut library_manager);
        let schematic = project_workspace
            .active_schematic()
            .cloned()
            .unwrap_or_default();
        let mut state = Self {
            schematic,
            workspace: project_workspace,
            library_manager,
            shell: de.shell.into(),
            ..Default::default()
        };
        state.workspace.save_active_schematic(&state.schematic);
        Ok(state)
    }
}

fn default_library_manager() -> crate::state::LibraryManager {
    crate::state::LibraryManager::with_primitives()
}
