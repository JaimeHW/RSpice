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

    pub fn apply_source_dirty_flags(&self, working: &mut crate::ProjectWorkspace) {
        let baseline = &self.baseline.workspace;
        working.netlist_source_dirty = working.netlist_source != baseline.netlist_source
            || working.netlist_source_path != baseline.netlist_source_path
            || working.netlist_document != baseline.netlist_document
            || working.netlist_descriptor != baseline.netlist_descriptor
            || working.retained_netlist_decks != baseline.retained_netlist_decks;
        working.project_sources_dirty = working.project_sources != baseline.project_sources;
    }
}
