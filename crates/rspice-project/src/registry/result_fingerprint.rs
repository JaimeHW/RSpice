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
            fields.signal_sets,
            fields.x_scales,
            fields.xy_plots,
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
        signal_sets,
        x_scales,
        xy_plots,
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
    let digest = match marker_history {
        Some(highest) => super::digest(&("result-marker-allocation-v1", result_fields, highest)),
        None => super::digest(&result_fields),
    }?;
    // Signal sets extend it the same way, and only when a project has any.
    let digest = if signal_sets.is_empty() {
        digest
    } else {
        super::digest(&("result-signal-sets-v1", digest, signal_sets))?
    };
    // And the reader's X scales, only when a project has any.
    let digest = if x_scales.is_empty() {
        digest
    } else {
        super::digest(&("result-x-scales-v1", digest, x_scales))?
    };
    // And the strips drawn against a signal, only when there are any.
    if xy_plots.is_empty() {
        Ok(digest)
    } else {
        super::digest(&("result-xy-plots-v1", digest, xy_plots))
    }
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
        let presentation = ResultPresentation::default();
        let fingerprint = |results: &ProjectSimulationResults| {
            digest(results, &[], &[], &presentation).expect("the history fingerprints")
        };
        assert_eq!(fingerprint(&history(quiet)), fingerprint(&history(logged)));
    }

    /// A strip drawn against a signal is part of the project's result
    /// content, so making one or narrowing its sweep span dirties the
    /// project; a project with none keeps the digest it had before such
    /// strips existed.
    #[test]
    fn a_strip_drawn_against_a_signal_changes_the_fingerprint() {
        use rspice_results::result_presentation::{AnalysisPresentationKey, StripXyPlot};
        use rspice_results::visualization_document::Abscissa;
        use rspice_vocabulary::product::DatasetId;
        let results = ProjectSimulationResults::from(ProjectSimulationResultsData::default());
        let plain = ResultPresentation::default();
        let before = full_digest(&results, &[], &[], plain.fingerprint_fields().unwrap()).unwrap();
        assert_eq!(digest(&results, &[], &[], &plain).unwrap(), before);
        let plot = StripXyPlot {
            analysis: serde_json::from_value::<AnalysisPresentationKey>(serde_json::json!({
                "dataset_id": DatasetId::new(),
                "source": {"Legacy": 1},
            }))
            .unwrap(),
            unit: "V".to_owned(),
            abscissa: Abscissa::Signal {
                name: "V(in)".to_owned(),
            },
            sweep_range: None,
        };
        let presentation = |plot: &StripXyPlot| ResultPresentation {
            xy_plots: vec![plot.clone()],
            ..ResultPresentation::default()
        };
        let whole = digest(&results, &[], &[], &presentation(&plot)).unwrap();
        assert_ne!(whole, before);
        let narrowed = StripXyPlot {
            sweep_range: Some(
                rspice_results::visualization_document::AxisRange::new(0.5e-3, 1.0e-3).unwrap(),
            ),
            ..plot
        };
        assert_ne!(
            digest(&results, &[], &[], &presentation(&narrowed)).unwrap(),
            whole
        );
    }
}
