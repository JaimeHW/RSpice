//! Which result owns a specialized viewer's cache.
//!
//! The Bode, Nyquist, Smith, FFT and eye viewers hold mutable data derived
//! from one immutable result. The run tier that derives it and the Results
//! workspace that shows it both name that result the same way.

use crate::product::{AnalysisInstanceId, DatasetId};

/// Exact owner of one mutable specialized-viewer cache.
///
/// Current results use their prepared analysis-instance identity. Migrated
/// result history that predates prepared-task provenance falls back to the
/// retained run-local analysis ID; paired with the immutable dataset ID this
/// is still an exact identity and never aliases another dataset's result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpecializedViewerCacheProvenance {
    pub(crate) dataset_id: DatasetId,
    pub(crate) analysis_identity: SpecializedViewerAnalysisIdentity,
}

impl SpecializedViewerCacheProvenance {
    pub(crate) fn for_analysis(
        dataset_id: DatasetId,
        analysis: &crate::state::AnalysisResult,
    ) -> Self {
        let analysis_identity = analysis.provenance().map_or(
            SpecializedViewerAnalysisIdentity::LegacyResultId(analysis.id),
            |provenance| {
                SpecializedViewerAnalysisIdentity::Prepared(provenance.source_instance_id())
            },
        );
        Self {
            dataset_id,
            analysis_identity,
        }
    }

    pub(crate) const fn for_prepared_analysis(
        dataset_id: DatasetId,
        source_instance_id: AnalysisInstanceId,
    ) -> Self {
        Self {
            dataset_id,
            analysis_identity: SpecializedViewerAnalysisIdentity::Prepared(source_instance_id),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpecializedViewerAnalysisIdentity {
    Prepared(AnalysisInstanceId),
    LegacyResultId(u64),
}
