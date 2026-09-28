//! Immutable accepted project content and the fingerprints of that exact content.

use crate::ProjectFile;
use crate::registry::{DocumentFingerprints, ProjectDocumentId, document_fingerprints};

#[derive(Debug)]
pub struct AcceptedProject {
    baseline: ProjectFile,
    fingerprints: Result<DocumentFingerprints, String>,
}

impl AcceptedProject {
    pub fn new(baseline: ProjectFile) -> Self {
        let fingerprints = document_fingerprints(&baseline);
        Self {
            baseline,
            fingerprints,
        }
    }

    pub fn baseline(&self) -> &ProjectFile {
        &self.baseline
    }

    pub fn fingerprints(&self) -> Result<&DocumentFingerprints, String> {
        self.fingerprints.as_ref().map_err(Clone::clone)
    }

    pub fn document_candidate(
        &self,
        working: &ProjectFile,
        id: &ProjectDocumentId,
    ) -> Result<ProjectFile, String> {
        let mut candidate = self.baseline.clone();
        candidate.overlay_document(working, id)?;
        Ok(candidate)
    }
}
