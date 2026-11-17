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

    /// The accepted content with `ids` taken from `working`.
    pub fn document_candidate(
        &self,
        working: &ProjectFile,
        ids: &[ProjectDocumentId],
    ) -> Result<ProjectFile, String> {
        let mut candidate = self.baseline.clone();
        for id in ids {
            candidate.overlay_document(working, id)?;
        }
        Ok(candidate)
    }

    /// Whether `working`'s netlist source and its project sources each
    /// differ from the accepted content, in that order.
    pub fn source_dirty_flags(&self, working: &crate::ProjectWorkspace) -> (bool, bool) {
        let baseline = &self.baseline.workspace;
        let netlist_source = working.netlist_source != baseline.netlist_source
            || working.netlist_source_path != baseline.netlist_source_path
            || working.netlist_document != baseline.netlist_document
            || working.netlist_descriptor != baseline.netlist_descriptor
            || working.retained_netlist_decks != baseline.retained_netlist_decks;
        let project_sources = !working
            .project_sources
            .same_authored_content(&baseline.project_sources);
        (netlist_source, project_sources)
    }
}
