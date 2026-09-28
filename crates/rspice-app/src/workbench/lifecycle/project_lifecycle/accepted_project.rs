//! Pair project-owned accepted content with its application editor session.

use std::rc::Rc;

use super::{PersistenceBinding, ProjectDocumentId, ProjectLifecycleError, ProjectLifecycleState};
use crate::io::ProjectSnapshot;
use crate::state::SchematicState;

impl ProjectLifecycleState {
    pub(super) fn accept_project(
        &mut self,
        baseline: ProjectSnapshot,
        binding: Option<PersistenceBinding>,
    ) {
        let ProjectSnapshot {
            file,
            workspace_session,
        } = baseline;
        self.authority.accept_content(file);
        self.accepted_session = Rc::new(workspace_session);
        self.accepted_binding = binding;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn restore_project(
        &mut self,
        baseline: ProjectSnapshot,
        binding: PersistenceBinding,
    ) {
        let ProjectSnapshot {
            file,
            workspace_session,
        } = baseline;
        self.authority.restore_accepted_content(file);
        self.accepted_session = Rc::new(workspace_session);
        self.accepted_binding = Some(binding);
    }

    pub(super) fn accepted_snapshot(&self) -> Result<ProjectSnapshot, ProjectLifecycleError> {
        let accepted = self
            .authority
            .accepted()
            .ok_or(ProjectLifecycleError::NoAcceptedBaseline)?;
        Ok(ProjectSnapshot {
            file: accepted.baseline().clone(),
            workspace_session: (*self.accepted_session).clone(),
        })
    }

    pub(super) fn accepted_schematic_editor(&self, key: &str) -> Option<SchematicState> {
        let accepted = self.authority.accepted()?;
        self.accepted_session
            .clone_schematic_editor(&accepted.baseline().workspace, key)
    }

    pub(super) fn accepted_document_candidate(
        &self,
        working: &ProjectSnapshot,
        id: &ProjectDocumentId,
    ) -> Result<ProjectSnapshot, ProjectLifecycleError> {
        let accepted = self
            .authority
            .accepted()
            .ok_or(ProjectLifecycleError::NoAcceptedBaseline)?;
        let file = accepted
            .document_candidate(&working.file, id)
            .map_err(ProjectLifecycleError::InvalidState)?;
        let mut workspace_session = (*self.accepted_session).clone();
        if let ProjectDocumentId::CellView(reference) = id {
            let key = reference.key();
            if working.file.workspace.schematic_buffers.contains_key(&key) {
                let session = working
                    .workspace_session
                    .schematic_sessions
                    .get(&key)
                    .cloned()
                    .unwrap_or_default();
                workspace_session.schematic_sessions.insert(key, session);
            } else {
                workspace_session.schematic_sessions.remove(&key);
            }
        }
        Ok(ProjectSnapshot {
            file,
            workspace_session,
        })
    }
}
