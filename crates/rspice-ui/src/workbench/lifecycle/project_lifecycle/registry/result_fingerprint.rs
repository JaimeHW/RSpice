//! The existing Results digest, reused only for identical immutable inputs.

use std::cell::RefCell;

use super::*;
use crate::io::ProjectSimulationResults;

#[derive(Debug, Clone, Default)]
pub(crate) struct ResultFingerprintCache(RefCell<Option<CachedFingerprint>>);

#[derive(Debug, Clone)]
struct CachedFingerprint {
    results: ProjectSimulationResults,
    presentation: ContentDigest,
    digest: ContentDigest,
}

impl ResultFingerprintCache {
    pub(super) fn digest(&self, project: &ProjectFile) -> Result<ContentDigest, String> {
        let fields = project.result_presentation.fingerprint_fields()?;
        // These owners are still mutable without revisions. Compare their
        // exact canonical encoding, including signed zero, before reusing a
        // digest that includes them. Retained samples are absent from this key.
        let presentation = super::digest(&(
            &project.workspace.report_documents,
            &project.workspace.visualization_documents,
            fields.markers,
            fields.log_y_panes,
            fields.expression_groups,
            fields.marker_history,
        ))?;
        let mut cache = self.0.borrow_mut();
        if let Some(held) = cache.as_ref()
            && held
                .results
                .shares_content_with(&project.simulation_results)
            && held.presentation == presentation
        {
            return Ok(held.digest);
        }
        let digest = full_digest(project, fields)?;
        *cache = Some(CachedFingerprint {
            results: project.simulation_results.clone(),
            presentation,
            digest,
        });
        Ok(digest)
    }
}

pub(super) fn digest(project: &ProjectFile) -> Result<ContentDigest, String> {
    full_digest(project, project.result_presentation.fingerprint_fields()?)
}

fn full_digest(
    project: &ProjectFile,
    fields: ResultFingerprintFields<'_>,
) -> Result<ContentDigest, String> {
    #[cfg(test)]
    super::RESULT_FINGERPRINT_PASSES.with(|passes| passes.set(passes.get() + 1));
    let ResultFingerprintFields {
        markers,
        log_y_panes,
        expression_groups,
        marker_history,
    } = fields;
    let result_fields = (
        &project.simulation_results,
        &project.workspace.report_documents,
        &project.workspace.visualization_documents,
        markers,
        log_y_panes,
        expression_groups,
    );
    // Preserve the published six-element digest and its allocation extension.
    match marker_history {
        Some(highest) => super::digest(&("result-marker-allocation-v1", result_fields, highest)),
        None => super::digest(&result_fields),
    }
}
