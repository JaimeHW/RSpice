//! One mutation boundary for a retained yield population and its provenance.

use std::sync::Arc;

#[cfg(feature = "engine-evidence")]
use super::YieldBasis;
use super::{YieldAnalysisProvenance, YieldResult};
#[cfg(feature = "engine-evidence")]
use crate::specification::{SpecEntry, SpecificationDefinition};
use rspice_vocabulary::product::{DatasetId, RunId};
use rspice_vocabulary::source_revision::SourceRevision;

#[derive(Debug, Clone, Default)]
pub struct YieldEvidence {
    results: Arc<[YieldResult]>,
    provenance: Option<YieldAnalysisProvenance>,
    revision: SourceRevision,
    /// What the held population was judged from. `None` until it is judged,
    /// and after any replacement that did not say.
    #[cfg(feature = "engine-evidence")]
    basis: Option<YieldBasis>,
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
        #[cfg(feature = "engine-evidence")]
        {
            self.basis = None;
        }
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

/// A population judged from the plan's limits, which only engine-backed
/// evidence can state.
#[cfg(feature = "engine-evidence")]
impl YieldEvidence {
    /// Replace the population with one judged from `basis`.
    ///
    /// The provenance stays even when no limit produced a row: the run was
    /// judged, against nothing, and a reader of it still names its trials.
    pub fn replace_judged(
        &mut self,
        results: Vec<YieldResult>,
        provenance: Option<YieldAnalysisProvenance>,
        basis: YieldBasis,
    ) {
        self.revision.advance();
        self.provenance = provenance;
        self.results = results.into();
        self.basis = Some(basis);
    }

    /// Whether the held population was judged from exactly these inputs.
    #[must_use]
    pub fn is_judged_from(
        &self,
        analysis: Option<(DatasetId, u64)>,
        specs: &[SpecEntry],
        definitions: &[SpecificationDefinition],
    ) -> bool {
        self.basis
            .as_ref()
            .is_some_and(|basis| basis.is(analysis, specs, definitions))
    }
}
