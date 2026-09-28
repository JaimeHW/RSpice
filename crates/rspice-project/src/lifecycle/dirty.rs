//! Working-versus-accepted comparison and lifecycle registry updates.

use super::{ProjectLifecycle, ProjectLifecycleError, SaveScope};
use crate::registry::{
    DocumentFingerprints, DocumentRegistry, ProjectDocumentId,
    document_fingerprints_with_results_cache,
};
use crate::{AcceptedProject, ProjectFile};

impl ProjectLifecycle {
    pub fn registry(&self) -> &DocumentRegistry {
        &self.registry
    }

    pub fn working_fingerprints(
        &self,
        current: &ProjectFile,
    ) -> Result<DocumentFingerprints, ProjectLifecycleError> {
        document_fingerprints_with_results_cache(current, &self.result_fingerprints)
            .map_err(ProjectLifecycleError::InvalidState)
    }

    pub fn has_unsaved_changes(
        &self,
        accepted: Option<&AcceptedProject>,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> bool {
        if !self.is_open() {
            return false;
        }
        let Some(accepted) = accepted else {
            return true;
        };
        match capture()
            .and_then(|current| self.working_fingerprints(&current))
            .map(|current| current.content_digest())
        {
            Ok(current) => accepted
                .fingerprints()
                .map(|baseline| current != baseline.content_digest())
                .unwrap_or(true),
            Err(_) => true,
        }
    }

    pub fn document_is_dirty(
        &self,
        accepted: Option<&AcceptedProject>,
        document: impl FnOnce() -> ProjectDocumentId,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> bool {
        if accepted.is_none() {
            return self.is_open();
        }
        self.current_registry(accepted, capture)
            .map(|registry| registry.is_dirty(&document()))
            .unwrap_or(true)
    }

    /// The configuration represents the whole unsaved project when there is
    /// no accepted baseline or the working content cannot be compared.
    pub fn dirty_documents(
        &self,
        accepted: Option<&AcceptedProject>,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> Vec<ProjectDocumentId> {
        if accepted.is_none() {
            return if self.is_open() {
                vec![ProjectDocumentId::ProjectConfiguration]
            } else {
                Vec::new()
            };
        }
        let Ok(registry) = self.current_registry(accepted, capture) else {
            return vec![ProjectDocumentId::ProjectConfiguration];
        };
        registry
            .records()
            .iter()
            .filter(|record| record.dirty)
            .map(|record| record.id.clone())
            .collect()
    }

    fn current_registry(
        &self,
        accepted: Option<&AcceptedProject>,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> Result<DocumentRegistry, ProjectLifecycleError> {
        let current = self.working_fingerprints(&capture()?)?;
        let accepted = accepted
            .map(AcceptedProject::fingerprints)
            .transpose()
            .map_err(ProjectLifecycleError::InvalidState)?;
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(&current, accepted);
        Ok(registry)
    }

    /// Capture through a shared borrow before installing the comparison.
    /// A closed project clears its registry without capturing working content.
    pub fn prepare_registry_refresh(
        &self,
        accepted: Option<&AcceptedProject>,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> Result<DocumentRegistry, ProjectLifecycleError> {
        if !self.is_open() {
            return Ok(DocumentRegistry::default());
        }
        self.current_registry(accepted, capture)
    }

    pub fn finish_registry_refresh(
        &mut self,
        comparison: Result<DocumentRegistry, ProjectLifecycleError>,
    ) -> Result<(), ProjectLifecycleError> {
        match comparison {
            Ok(registry) => self.registry = registry,
            Err(error) => {
                self.registry.invalidate();
                return Err(error);
            }
        }
        Ok(())
    }

    pub fn prepare_post_save_registry(
        &self,
        current: ProjectFile,
        candidate: &ProjectFile,
        scope: SaveScope,
        active_document: impl FnOnce() -> ProjectDocumentId,
    ) -> Result<DocumentRegistry, ProjectLifecycleError> {
        #[cfg(not(target_arch = "wasm32"))]
        let mut current = current;
        #[cfg(not(target_arch = "wasm32"))]
        {
            if scope == SaveScope::AllDocuments
                || active_document() == ProjectDocumentId::ProjectConfiguration
            {
                current.workspace.project = candidate.workspace.project.clone();
            } else {
                current.workspace.project.path = candidate.workspace.project.path.clone();
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = (scope, active_document);
        let mut registry = DocumentRegistry::default();
        let candidate = self.working_fingerprints(candidate)?;
        let current = self.working_fingerprints(&current)?;
        registry.rebuild_from_fingerprints(&current, Some(&candidate));
        Ok(registry)
    }

    /// A verified publication remains accepted even if a newer draft cannot
    /// be compared. Its previous document records remain conservatively dirty.
    pub fn unverified_registry(&self) -> DocumentRegistry {
        let mut registry = self.registry.clone();
        registry.invalidate();
        registry
    }

    pub fn adopt_registry(&mut self, registry: DocumentRegistry) {
        self.registry = registry;
    }
}
