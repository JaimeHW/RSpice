//! The existing Results digest, reused only for identical immutable inputs.

use std::cell::RefCell;

use super::*;
use crate::results::ProjectSimulationResults;
use rspice_results::report_document::ReportDocument;
use rspice_results::result_presentation::{ResultFingerprintFields, ResultPresentation};
use rspice_results::visualization_document::VisualizationDocument;

#[cfg(any(test, feature = "document-fingerprint-observation"))]
thread_local! {
    pub static RESULT_FINGERPRINT_PASSES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[derive(Debug, Clone, Default)]
pub struct ResultFingerprintCache(RefCell<Option<CachedFingerprint>>);

#[derive(Debug, Clone)]
struct CachedFingerprint {
    results: ProjectSimulationResults,
    presentation: ContentDigest,
    digest: ContentDigest,
}

impl ResultFingerprintCache {
    pub fn digest(
        &self,
        results: &ProjectSimulationResults,
        reports: &[ReportDocument],
        visualizations: &[VisualizationDocument],
        presentation: &ResultPresentation,
    ) -> Result<ContentDigest, String> {
        let fields = presentation.fingerprint_fields()?;
        // These owners are still mutable without revisions. Compare their
        // exact canonical encoding, including signed zero, before reusing a
        // digest that includes them. Retained samples are absent from this key.
        let presentation = super::digest(&(
            reports,
            visualizations,
            fields.markers,
            fields.log_y_panes,
            fields.expression_groups,
            fields.marker_history,
        ))?;
        let mut cache = self.0.borrow_mut();
        if let Some(held) = cache.as_ref()
            && held.results.shares_content_with(results)
            && held.presentation == presentation
        {
            return Ok(held.digest);
        }
        let digest = full_digest(results, reports, visualizations, fields)?;
        *cache = Some(CachedFingerprint {
            results: results.clone(),
            presentation,
            digest,
        });
        Ok(digest)
    }
}

pub fn digest(
    results: &ProjectSimulationResults,
    reports: &[ReportDocument],
    visualizations: &[VisualizationDocument],
    presentation: &ResultPresentation,
) -> Result<ContentDigest, String> {
    full_digest(
        results,
        reports,
        visualizations,
        presentation.fingerprint_fields()?,
    )
}

fn full_digest(
    results: &ProjectSimulationResults,
    reports: &[ReportDocument],
    visualizations: &[VisualizationDocument],
    fields: ResultFingerprintFields<'_>,
) -> Result<ContentDigest, String> {
    #[cfg(any(test, feature = "document-fingerprint-observation"))]
    RESULT_FINGERPRINT_PASSES.with(|passes| passes.set(passes.get() + 1));
    let ResultFingerprintFields {
        markers,
        log_y_panes,
        expression_groups,
        marker_history,
    } = fields;
    let result_fields = (
        results,
        reports,
        visualizations,
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
