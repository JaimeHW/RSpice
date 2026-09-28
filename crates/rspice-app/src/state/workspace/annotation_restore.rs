//! Restore approved annotation receipts into canonical project documents.

use super::*;

impl ProjectWorkspace {
    pub(crate) fn restore_pending_annotation(
        &mut self,
        libraries: &LibraryManager,
    ) -> Result<usize, String> {
        self.session
            .restore_pending_annotation(&mut self.content, libraries)
    }
    pub(crate) fn annotation_restoration_error(&self) -> Option<&str> {
        self.session.annotation_restoration_error()
    }
}

impl super::WorkspaceSession {
    /// Prepare the complete current reference closure before replacing any
    /// source. Reading a project may materialize its already-approved journal;
    /// it must neither invent an edit nor partially update its consumers.
    pub(crate) fn restore_pending_annotation(
        &mut self,
        content: &mut rspice_project::ProjectWorkspace,
        libraries: &LibraryManager,
    ) -> Result<usize, String> {
        let result = self.apply_pending_annotation(content, libraries);
        self.annotation_restoration_error = result.as_ref().err().cloned();
        // Success changes reference owners; refusal changes their eligibility.
        // Neither may reuse a projection handed out before this attempt.
        self.design_projection_cache.invalidate();
        result
    }

    pub(crate) fn annotation_restoration_error(&self) -> Option<&str> {
        self.annotation_restoration_error.as_deref()
    }

    fn apply_pending_annotation(
        &mut self,
        content: &mut rspice_project::ProjectWorkspace,
        libraries: &LibraryManager,
    ) -> Result<usize, String> {
        let Some(prepared) = self
            .project_hierarchy(content, libraries, None)
            .prepare_pending_annotation()?
        else {
            return Ok(0);
        };
        let sessions: Vec<_> = prepared
            .changed_documents()
            .map(|key| {
                let mut session = self
                    .schematic_sessions
                    .get(key)
                    .cloned()
                    .unwrap_or_default();
                session.is_dirty = true;
                (key.to_owned(), session)
            })
            .collect();
        let count = prepared.publish(content);
        self.schematic_sessions.extend(sessions);
        Ok(count)
    }
}
