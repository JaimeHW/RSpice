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
    let results = results.without_run_logs();
    let result_fields = (
        &*results,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::{ProjectSimulationResultsData, ProjectSimulationRun};
    use rspice_app_types::diagnostics::{LogSeverity, LogSource};
    use rspice_app_types::run_log::RunLogLine;
    use rspice_results::run::{ExecutionTarget, SimulationRun};
    use rspice_results::waveform::RetainedWaveform;

    /// A run's log is not result content: neither the run's dataset digest
    /// nor the result history's fingerprint reads it.
    #[test]
    fn two_runs_differing_only_in_their_logs_digest_equal() {
        let quiet = SimulationRun::<rspice_results::analysis_result::AnalysisResult>::new(
            3,
            1_760_613_845.0,
            ExecutionTarget::LocalDesktop,
        );
        let mut logged = quiet.clone();
        logged.log.push(RunLogLine {
            unix_ms: 1_760_613_845_250,
            severity: LogSeverity::Warning,
            source: LogSource::Engine,
            message: "Gmin stepping was needed to find the operating point".to_owned(),
        });
        let persist = |run: &SimulationRun| {
            ProjectSimulationRun::from_run::<_, RetainedWaveform>(run, |_| {
                unreachable!("the run retained no analyses")
            })
        };
        let (quiet, logged) = (persist(&quiet), persist(&logged));
        assert!(!logged.log.is_empty());
        assert_eq!(quiet.dataset_content_digest, logged.dataset_content_digest);

        let history = |run: ProjectSimulationRun| {
            ProjectSimulationResults::from(ProjectSimulationResultsData {
                runs: vec![run],
                ..ProjectSimulationResultsData::default()
            })
        };
        let presentation = ResultPresentation::default();
        let fingerprint = |results: &ProjectSimulationResults| {
            digest(results, &[], &[], &presentation).expect("the history fingerprints")
        };
        assert_eq!(fingerprint(&history(quiet)), fingerprint(&history(logged)));
    }
}
