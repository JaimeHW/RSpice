//! In-memory project snapshots retain editor state outside the canonical file.

use super::SchematicState;
#[cfg(test)]
use super::schematic::{SchematicEditorMut, SchematicEditorRef};
use super::workspace::WorkspaceSession;
#[cfg(test)]
use super::{LibraryManager, ProjectWorkspace};
use rspice_project::ProjectFile;
#[cfg(test)]
use rspice_project::{ProjectExecutionContext, results::ProjectSimulationResults};
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
    pub(crate) fn insert_schematic_editor(
        &mut self,
        key: String,
        editor: SchematicState,
    ) -> Option<SchematicState> {
        self.workspace_session
            .insert_schematic_editor(&mut self.file.workspace, key, editor)
    }
    #[cfg(test)]
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

/// Captured editor state follows the project owner's choice of design content.
pub(crate) struct SnapshotSessionCapture<'a> {
    pub(crate) workspace: WorkspaceSession,
    pub(crate) active: &'a super::schematic::SchematicSession,
}

impl rspice_project::SnapshotSessions for SnapshotSessionCapture<'_> {
    fn replace_active(&mut self, key: &str) {
        self.workspace
            .schematic_sessions
            .insert(key.to_owned(), self.active.clone());
    }

    fn reconcile_cancelled_operation(
        &mut self,
        key: &str,
        design: &mut rspice_design::schematic::owned::Schematic,
        cancelled: Option<rspice_design::schematic::owned::CancelledOperation>,
    ) {
        let session = self
            .workspace
            .schematic_sessions
            .entry(key.to_owned())
            .or_default();
        if let Some(cancelled) = cancelled {
            SchematicState::borrow_parts(design, session)
                .editor
                .reconcile_cancelled_operation(cancelled);
        }
    }

    fn mark_all_clean(&mut self) {
        for session in self.workspace.schematic_sessions.values_mut() {
            session.is_dirty = false;
        }
    }

    fn strip_schematic_runtime(&mut self, key: &str) {
        let session = self
            .workspace
            .schematic_sessions
            .entry(key.to_owned())
            .or_default();
        session.selection = Default::default();
        session.wire_drawing = Default::default();
        session.clipboard = Default::default();
        session.preview_rotation = Default::default();
        session.preview_mirror_h = false;
        session.is_dirty = false;
    }
}
