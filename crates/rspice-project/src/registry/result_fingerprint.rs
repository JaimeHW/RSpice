//! The Results digest: the retained runs by identity, never by their
//! samples, with what the reader made of them. A run is immutable once
//! written, so a history holding a million-signal run digests in what its
//! runs' headers weigh, and a record holding only those headers digests as
//! the whole record does. Each view stands in it by its content digest,
//! which the view keeps for the content it holds: a view of a hundred
//! thousand traces is written out once per change of it, not once per
//! check.

use std::cell::RefCell;

use super::*;
use crate::results::ProjectSimulationResults;
use rspice_results::report_document::ReportDocument;
use rspice_results::result_presentation::SignalSet;
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
        signal_sets: &[SignalSet],
    ) -> Result<ContentDigest, String> {
        // These owners are still mutable without revisions. Compare their
        // exact canonical encoding, including signed zero, before reusing a
        // digest that includes them. Retained samples are absent from this key.
        let views = view_digests(visualizations)?;
        let presentation = super::digest(&(reports, &views, signal_sets))?;
        let mut cache = self.0.borrow_mut();
        if let Some(held) = cache.as_ref()
            && held.results.shares_content_with(results)
            && held.presentation == presentation
        {
            return Ok(held.digest);
        }
        let digest = digest_of_views(results, reports, &views, signal_sets)?;
        *cache = Some(CachedFingerprint {
            results: results.clone(),
            presentation,
            digest,
        });
        Ok(digest)
    }
}

/// The result history's fingerprint: the retained runs by identity
/// ([`crate::results::ProjectSimulationResultsData::identity`]), the reports,
/// the views (what the reader made of the runs among them) by their content
/// digests, and the saved signal sets.
pub fn digest(
    results: &ProjectSimulationResults,
    reports: &[ReportDocument],
    visualizations: &[VisualizationDocument],
    signal_sets: &[SignalSet],
) -> Result<ContentDigest, String> {
    digest_of_views(
        results,
        reports,
        &view_digests(visualizations)?,
        signal_sets,
    )
}

fn digest_of_views(
    results: &ProjectSimulationResults,
    reports: &[ReportDocument],
    views: &[ContentDigest],
    signal_sets: &[SignalSet],
) -> Result<ContentDigest, String> {
    #[cfg(any(test, feature = "document-fingerprint-observation"))]
    super::fingerprints::observe(&RESULT_FINGERPRINT_PASSES);
    super::digest(&(
        "result-history-v4",
        results.identity(),
        reports,
        views,
        signal_sets,
    ))
}

/// Each view's content digest, in the project's order. A view keeps the
/// digest of the content it holds, so only a view changed since it was last
/// asked is written out.
fn view_digests(visualizations: &[VisualizationDocument]) -> Result<Vec<ContentDigest>, String> {
    visualizations
        .iter()
        .map(|view| view.content_digest().map_err(|error| error.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::{ProjectSimulationResultsData, ProjectSimulationRun};
    use rspice_results::run::{ExecutionTarget, SimulationRun};
    use rspice_results::waveform::RetainedWaveform;
    use rspice_vocabulary::diagnostics::{LogSeverity, LogSource};
    use rspice_vocabulary::run_log::RunLogLine;

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
        let fingerprint = |results: &ProjectSimulationResults| {
            digest(results, &[], &[], &[]).expect("the history fingerprints")
        };
        assert_eq!(fingerprint(&history(quiet)), fingerprint(&history(logged)));
    }
}
