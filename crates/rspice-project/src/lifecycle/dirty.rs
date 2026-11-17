//! Working-versus-accepted comparison and lifecycle registry updates.

use std::sync::Arc;

use super::{ProjectLifecycle, ProjectLifecycleError, SaveScope};
use crate::registry::{
    DocumentFingerprints, DocumentRegistry, ProjectDocumentId,
    document_fingerprints_with_results_cache,
};
use crate::{AcceptedProject, ProjectFile};
use rspice_vocabulary::product::ContentDigest;

/// What working content is compared with: whether a project is open, the
/// accepted baseline and, before the first save, what the project started
/// from. Two equal values compare the same working content alike.
#[derive(Debug, Clone)]
pub struct DirtyBaseline {
    open: bool,
    /// Held, so the baseline a comparison was made against outlives it and
    /// its identity cannot be taken by another.
    accepted: Option<Arc<AcceptedProject>>,
    pristine: Option<ContentDigest>,
}

impl PartialEq for DirtyBaseline {
    fn eq(&self, other: &Self) -> bool {
        self.open == other.open
            && self.pristine == other.pristine
            && match (&self.accepted, &other.accepted) {
                (Some(left), Some(right)) => Arc::ptr_eq(left, right),
                (None, None) => true,
                _ => false,
            }
    }
}

/// Every dirty answer for one working content against one baseline: whether
/// anything is unsaved, and which documents.
#[derive(Debug, Clone)]
pub struct DirtyComparison {
    open: bool,
    unsaved: bool,
    documents: ComparedDocuments,
}

#[derive(Debug, Clone)]
enum ComparedDocuments {
    /// No accepted baseline: the configuration stands for the whole project
    /// whenever anything is unsaved.
    Unaccepted,
    /// Each document compared with the accepted baseline; `None` when the
    /// comparison failed, which leaves everything unsaved.
    Accepted(Option<DocumentRegistry>),
}

impl DirtyComparison {
    pub const fn has_unsaved_changes(&self) -> bool {
        self.unsaved
    }

    pub fn document_is_dirty(&self, document: &ProjectDocumentId) -> bool {
        match &self.documents {
            ComparedDocuments::Unaccepted => self.open && self.unsaved,
            ComparedDocuments::Accepted(registry) => registry
                .as_ref()
                .is_none_or(|registry| registry.is_dirty(document)),
        }
    }

    /// The configuration represents the whole unsaved project when there is
    /// no accepted baseline or the working content cannot be compared.
    pub fn dirty_documents(&self) -> Vec<ProjectDocumentId> {
        match &self.documents {
            ComparedDocuments::Unaccepted if self.open && self.unsaved => {
                vec![ProjectDocumentId::ProjectConfiguration]
            }
            ComparedDocuments::Unaccepted => Vec::new(),
            ComparedDocuments::Accepted(None) => vec![ProjectDocumentId::ProjectConfiguration],
            ComparedDocuments::Accepted(Some(registry)) => registry
                .records()
                .iter()
                .filter(|record| record.dirty)
                .map(|record| record.id.clone())
                .collect(),
        }
    }
}

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

    /// What [`Self::compare_working`] compares working content with now.
    pub fn dirty_baseline(&self) -> DirtyBaseline {
        DirtyBaseline {
            open: self.project_open,
            accepted: self.accepted.clone(),
            pristine: self.pristine,
        }
    }

    /// Whether a dirty answer reads the working content at all. A project
    /// with no accepted baseline and no pristine start is unsaved whatever
    /// it holds, and a closed one without a baseline holds nothing.
    pub const fn compares_working_content(&self) -> bool {
        self.accepted.is_some() || (self.project_open && self.pristine.is_some())
    }

    /// Every dirty answer for the working content whose fingerprints are
    /// `working`: `None` when it could not be captured or fingerprinted,
    /// which leaves it unsaved. `working` is read only when
    /// [`Self::compares_working_content`].
    pub fn compare_working(&self, working: Option<&DocumentFingerprints>) -> DirtyComparison {
        let Some(accepted) = self.accepted() else {
            // Whether this never-saved project still holds what it started
            // from: see [`ProjectLifecycle::mark_pristine`].
            let pristine = self.project_open
                && self.pristine.is_some_and(|pristine| {
                    working.is_some_and(|working| working.content_digest() == pristine)
                });
            return DirtyComparison {
                open: self.project_open,
                unsaved: self.project_open && !pristine,
                documents: ComparedDocuments::Unaccepted,
            };
        };
        let baseline = accepted.fingerprints().ok();
        let unsaved = self.project_open
            && match (working, baseline) {
                (Some(working), Some(baseline)) => {
                    working.content_digest() != baseline.content_digest()
                }
                _ => true,
            };
        let registry = working.zip(baseline).map(|(working, baseline)| {
            let mut registry = DocumentRegistry::default();
            registry.rebuild_from_fingerprints(working, Some(baseline));
            registry
        });
        DirtyComparison {
            open: self.project_open,
            unsaved,
            documents: ComparedDocuments::Accepted(registry),
        }
    }

    /// [`Self::compare_working`] over content `capture` takes now, captured
    /// only when the answer reads it.
    pub fn compare_captured(
        &self,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> DirtyComparison {
        let working = self
            .compares_working_content()
            .then(|| capture().and_then(|current| self.working_fingerprints(&current)));
        self.compare_working(working.as_ref().and_then(|working| working.as_ref().ok()))
    }

    pub fn has_unsaved_changes(
        &self,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> bool {
        self.compare_captured(capture).has_unsaved_changes()
    }

    pub fn document_is_dirty(
        &self,
        document: impl FnOnce() -> ProjectDocumentId,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> bool {
        self.compare_captured(capture)
            .document_is_dirty(&document())
    }

    /// The configuration represents the whole unsaved project when there is
    /// no accepted baseline or the working content cannot be compared.
    pub fn dirty_documents(
        &self,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> Vec<ProjectDocumentId> {
        self.compare_captured(capture).dirty_documents()
    }

    fn current_registry(
        &self,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> Result<DocumentRegistry, ProjectLifecycleError> {
        self.registry_from_working(&self.working_fingerprints(&capture()?)?)
    }

    /// The registry [`Self::prepare_registry_refresh`] installs, from
    /// working fingerprints already taken.
    pub fn registry_from_working(
        &self,
        working: &DocumentFingerprints,
    ) -> Result<DocumentRegistry, ProjectLifecycleError> {
        let accepted = self
            .accepted()
            .map(AcceptedProject::fingerprints)
            .transpose()
            .map_err(ProjectLifecycleError::InvalidState)?;
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(working, accepted);
        Ok(registry)
    }

    /// Capture through a shared borrow before installing the comparison.
    /// A closed project clears its registry without capturing working content.
    pub fn prepare_registry_refresh(
        &self,
        capture: impl FnOnce() -> Result<ProjectFile, ProjectLifecycleError>,
    ) -> Result<DocumentRegistry, ProjectLifecycleError> {
        if !self.is_open() {
            return Ok(DocumentRegistry::default());
        }
        self.current_registry(capture)
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
