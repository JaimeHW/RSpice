//! One mutation boundary for a retained yield population and its provenance.

use std::sync::Arc;

use super::{SimulationRun, YieldAnalysisProvenance, YieldResult};
use crate::source_revision::SourceRevision;

#[derive(Debug, Clone, Default)]
pub struct YieldEvidence {
    results: Arc<[YieldResult]>,
    provenance: Option<YieldAnalysisProvenance>,
    revision: SourceRevision,
}

impl YieldEvidence {
    pub(super) fn replace(
        &mut self,
        results: Vec<YieldResult>,
        provenance: Option<YieldAnalysisProvenance>,
    ) {
        self.revision.advance();
        self.provenance = if results.is_empty() { None } else { provenance };
        self.results = results.into();
    }

    pub(super) fn results(&self) -> &[YieldResult] {
        &self.results
    }

    pub(super) fn provenance(&self) -> Option<YieldAnalysisProvenance> {
        self.provenance
    }

    pub(super) fn for_run(&self, run: &SimulationRun) -> Option<&[YieldResult]> {
        self.provenance
            .is_some_and(|source| {
                source.source_dataset_id == run.dataset_id && source.source_run_id == run.run_id
            })
            .then_some(self.results())
    }

    pub(crate) fn revision(&self) -> SourceRevision {
        self.revision.clone()
    }
}
