//! In-memory project snapshots retain editor state outside the canonical file.

#[cfg(test)]
use super::schematic::{SchematicEditorMut, SchematicEditorRef};
use super::workspace::WorkspaceSession;
use super::{LibraryManager, ProjectWorkspace, SchematicState};
use rspice_project::{ProjectExecutionContext, ProjectFile, results::ProjectSimulationResults};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ProjectSnapshot {
    pub file: ProjectFile,
    pub(crate) workspace_session: WorkspaceSession,
}

impl Serialize for ProjectSnapshot {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.file.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for ProjectSnapshot {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            file: Deserialize::deserialize(deserializer)?,
            workspace_session: Default::default(),
        })
    }
}

impl ProjectSnapshot {
    #[cfg(test)]
    pub fn new(workspace: ProjectWorkspace, libraries: LibraryManager) -> Self {
        let (content, workspace_session) = workspace.into_parts();
        Self {
            file: ProjectFile::new(content, libraries),
            workspace_session,
        }
    }
    #[cfg(test)]
    pub fn new_with_simulation_results(
        workspace: ProjectWorkspace,
        libraries: LibraryManager,
        results: ProjectSimulationResults,
    ) -> Self {
        let (content, workspace_session) = workspace.into_parts();
        Self {
            file: ProjectFile::new_with_simulation_results(content, libraries, results),
            workspace_session,
        }
    }
    pub fn new_with_execution_context(
        workspace: ProjectWorkspace,
        libraries: LibraryManager,
        results: ProjectSimulationResults,
        context: ProjectExecutionContext,
    ) -> Self {
        let (content, workspace_session) = workspace.into_parts();
        Self {
            file: ProjectFile::new_with_execution_context(content, libraries, results, context),
            workspace_session,
        }
    }
    #[must_use]
    pub(crate) fn with_result_presentation(
        mut self,
        presentation: rspice_results::result_presentation::ResultPresentation,
    ) -> Self {
        self.file = self.file.with_result_presentation(presentation);
        self
    }
    pub(crate) fn from_decoded(decoded: rspice_project::DecodedProject) -> Self {
        let mut workspace_session = WorkspaceSession::default();
        for key in decoded.restored_schematic_documents {
            workspace_session
                .schematic_sessions
                .entry(key)
                .or_default()
                .is_dirty = true;
        }
        Self {
            file: decoded.file,
            workspace_session,
        }
    }
    #[cfg(test)]
    pub(crate) fn schematic_editor(&self, key: &str) -> Option<SchematicEditorRef<'_>> {
        self.workspace_session
            .schematic_editor(&self.file.workspace, key)
    }
    pub(crate) fn clone_schematic_editor(&self, key: &str) -> Option<SchematicState> {
        self.workspace_session
            .clone_schematic_editor(&self.file.workspace, key)
    }
    #[cfg(test)]
    pub(crate) fn schematic_editor_mut(&mut self, key: &str) -> Option<SchematicEditorMut<'_>> {
        self.workspace_session
            .schematic_editor_mut(&mut self.file.workspace, key)
    }
    pub(crate) fn insert_schematic_editor(
        &mut self,
        key: String,
        editor: SchematicState,
    ) -> Option<SchematicState> {
        self.workspace_session
            .insert_schematic_editor(&mut self.file.workspace, key, editor)
    }
    pub(crate) fn remove_schematic_editor(&mut self, key: &str) -> Option<SchematicState> {
        self.workspace_session
            .remove_schematic_editor(&mut self.file.workspace, key)
    }
    pub(crate) fn ensure_library_model(&mut self) {
        self.workspace_session
            .ensure_library_model(&mut self.file.workspace, &mut self.file.libraries)
    }
    #[cfg(test)]
    pub(crate) fn restore_pending_annotation(&mut self) -> Result<usize, String> {
        self.workspace_session
            .restore_pending_annotation(&mut self.file.workspace, &self.file.libraries)
    }
}
