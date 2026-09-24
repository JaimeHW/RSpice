//! One mutation boundary for a retained yield population and its provenance.

use std::sync::Arc;

use super::{YieldAnalysisProvenance, YieldResult};
use rspice_app_types::product::{DatasetId, RunId};
use rspice_app_types::source_revision::SourceRevision;

#[derive(Debug, Clone, Default)]
pub struct YieldEvidence {
    results: Arc<[YieldResult]>,
    provenance: Option<YieldAnalysisProvenance>,
    revision: SourceRevision,
}

impl YieldEvidence {
    pub fn replace(
        &mut self,
        results: Vec<YieldResult>,
        provenance: Option<YieldAnalysisProvenance>,
    ) {
        self.revision.advance();
        self.provenance = if results.is_empty() { None } else { provenance };
        self.results = results.into();
    }

    pub fn results(&self) -> &[YieldResult] {
        &self.results
    }

    pub fn provenance(&self) -> Option<YieldAnalysisProvenance> {
        self.provenance
    }

    pub fn for_run_ids(&self, run_id: RunId, dataset_id: DatasetId) -> Option<&[YieldResult]> {
        self.provenance
            .is_some_and(|source| {
                source.source_dataset_id == dataset_id && source.source_run_id == run_id
            })
            .then_some(self.results())
    }

    pub fn revision(&self) -> SourceRevision {
        self.revision.clone()
    }
}
