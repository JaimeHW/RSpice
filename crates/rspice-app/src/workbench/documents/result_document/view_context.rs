//! One authoritative identity for the result presentation the reader sees.
//!
//! Rendering, measurement, export, and publication must not independently
//! consult the simulation's ordinal selections. A quick-view document can
//! display a compatible analysis even when the run selector points elsewhere,
//! and a persistent pane owns an immutable dataset/analysis binding of its
//! own. This module resolves those cases once into stable project identities.

use crate::product::{ContentDigest, DatasetId, ResultDocumentId};
use crate::results::visualization_document::{PageId, PaneId};
use crate::state::{AnalysisResult, SimulationRun};
use crate::workbench::state::{Workspace, WorkspaceDocumentId};

use super::{AnalysisPresentationKey, ResultViewer, ResultsReads};

/// The document presentation that owns a resolved result view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultViewOwner {
    /// A retained dataset opened directly in the Results workspace.
    Dataset,
    /// One exact pane in a project-owned visualization document.
    VisualizationPane {
        document_id: ResultDocumentId,
        page_id: PageId,
        pane_id: PaneId,
    },
}

/// Stable authority for the result presentation currently shown to the user.
///
/// `analysis_indices` is deliberately plural. The waveform, DC, Bode, and
/// ordinary-noise sheets can display a stack of analyses simultaneously; an
/// exporter that silently collapses that stack to the simulation selector is
/// not exporting the displayed result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedResultView {
    pub owner: ResultViewOwner,
    pub dataset_id: DatasetId,
    pub dataset_digest: ContentDigest,
    pub run_index: usize,
    pub viewer: ResultViewer,
    pub analysis_indices: Vec<usize>,
    pub primary_analysis_index: Option<usize>,
}

impl ResolvedResultView {
    /// Resolve the retained run after state changes without trusting an old
    /// vector ordinal on its own.
    pub(crate) fn run<'a>(&self, reads: ResultsReads<'a>) -> Option<&'a SimulationRun> {
        reads
            .model
            .simulation
            .retained
            .runs
            .get(self.run_index)
            .filter(|run| {
                run.dataset_id == self.dataset_id
                    && super::retained_dataset_digest(reads, run) == self.dataset_digest
            })
    }

    pub(crate) fn primary_analysis<'a>(
        &self,
        reads: ResultsReads<'a>,
    ) -> Option<&'a AnalysisResult> {
        self.run(reads)?.analyses.get(self.primary_analysis_index?)
    }

    pub(crate) fn analyses<'a>(
        &'a self,
        reads: ResultsReads<'a>,
    ) -> impl Iterator<Item = &'a AnalysisResult> + 'a {
        let run = self.run(reads);
        self.analysis_indices
            .iter()
            .filter_map(move |index| run.and_then(|run| run.analyses.get(*index)))
    }
}

/// Resolve the exact active Results document, viewer, and retained analyses.
pub(crate) fn resolve_displayed_result_view(
    reads: ResultsReads<'_>,
) -> Result<ResolvedResultView, String> {
    let document = reads
        .workbench
        .documents
        .active(Workspace::Results)
        .ok_or_else(|| "No active Results document is selected.".to_owned())?;
    match document {
        WorkspaceDocumentId::ResultDataset(dataset_id) => resolve_dataset_view(reads, *dataset_id),
        WorkspaceDocumentId::VisualizationDocument(document_id) => {
            resolve_visualization_pane(reads, *document_id)
        }
        _ => Err("The active Results document is not a result presentation.".to_owned()),
    }
}

fn resolve_dataset_view(
    reads: ResultsReads<'_>,
    dataset_id: DatasetId,
) -> Result<ResolvedResultView, String> {
    let run_index = reads
        .model
        .simulation
        .retained
        .runs
        .iter()
        .position(|run| run.dataset_id == dataset_id)
        .ok_or_else(|| "The active result dataset is no longer retained.".to_owned())?;
    let run = &reads.model.simulation.retained.runs[run_index];
    let viewer = reads.results.session.viewer;
    let mut analysis_indices = run
        .analyses
        .iter()
        .enumerate()
        .filter_map(|(index, analysis)| {
            analysis_supports_viewer_memoized(reads, dataset_id, viewer, analysis).then_some(index)
        })
        .collect::<Vec<_>>();

    if viewer_uses_analysis_stack(viewer) {
        analysis_indices.retain(|index| {
            let key = AnalysisPresentationKey::new(dataset_id, &run.analyses[*index]);
            !reads.results.session.hidden_strips.contains(&key)
        });
        if let Some(maximized) = reads.results.session.maximized_strip
            && let Some(index) = analysis_indices.iter().copied().find(|index| {
                AnalysisPresentationKey::new(dataset_id, &run.analyses[*index]) == maximized
            })
        {
            analysis_indices.clear();
            analysis_indices.push(index);
        }
    } else {
        let primary = selected_primary_index(reads, run_index, run, &analysis_indices);
        analysis_indices.clear();
        analysis_indices.extend(primary);
    }

    let primary_analysis_index = selected_primary_index(reads, run_index, run, &analysis_indices);
    Ok(ResolvedResultView {
        owner: ResultViewOwner::Dataset,
        dataset_id,
        dataset_digest: super::retained_dataset_digest(reads, run),
        run_index,
        viewer,
        analysis_indices,
        primary_analysis_index,
    })
}

fn selected_primary_index(
    reads: ResultsReads<'_>,
    run_index: usize,
    run: &SimulationRun,
    candidates: &[usize],
) -> Option<usize> {
    reads
        .results
        .session
        .active_wave_pane
        .as_ref()
        .map(|pane| pane.analysis)
        .and_then(|key| key.resolve(run).map(|(index, _)| index))
        .filter(|index| candidates.contains(index))
        .or_else(|| {
            (reads.model.simulation.view.active_run_idx == Some(run_index))
                .then_some(reads.model.simulation.view.active_analysis_idx)
                .flatten()
                .filter(|index| candidates.contains(index))
        })
        .or_else(|| candidates.first().copied())
}

fn resolve_visualization_pane(
    reads: ResultsReads<'_>,
    document_id: ResultDocumentId,
) -> Result<ResolvedResultView, String> {
    let document = reads
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| "The active visualization document no longer exists.".to_owned())?;
    let page_id = reads
        .results
        .session
        .persistent_document_page(document_id)
        .filter(|page_id| document.pages().iter().any(|page| page.id == *page_id))
        .or_else(|| document.pages().first().map(|page| page.id))
        .ok_or_else(|| "The active visualization document has no page.".to_owned())?;
    let pane = reads
        .results
        .session
        .persistent_document_pane(document_id)
        .and_then(|pane_id| {
            document
                .panes()
                .iter()
                .find(|pane| pane.id == pane_id && pane.page_id == page_id)
        })
        .or_else(|| {
            document
                .panes()
                .iter()
                .filter(|pane| pane.page_id == page_id)
                .min_by_key(|pane| (pane.order, pane.id.get()))
        })
        .ok_or_else(|| "The selected visualization page has no pane.".to_owned())?;
    let binding = pane
        .binding
        .ok_or_else(|| "The selected result pane is not bound to a run.".to_owned())?;
    let run_index = reads
        .model
        .simulation
        .retained
        .runs
        .iter()
        .position(|run| run.dataset_id == binding.dataset.dataset_id)
        .ok_or_else(|| "The pane's run is no longer in the run history.".to_owned())?;
    let run = &reads.model.simulation.retained.runs[run_index];
    if super::retained_dataset_digest(reads, run) != binding.dataset.content_digest {
        return Err(
            "The run's results no longer match the ones this pane was bound to.".to_owned(),
        );
    }
    let analysis_index = run
        .analyses
        .iter()
        .position(|analysis| analysis_instance_id(run, analysis) == binding.analysis_id)
        .ok_or_else(|| "The pane's bound analysis is no longer retained.".to_owned())?;
    let analysis = &run.analyses[analysis_index];
    let viewer = analysis_supports_viewer_memoized(
        reads,
        run.dataset_id,
        reads.results.session.viewer,
        analysis,
    )
    .then_some(reads.results.session.viewer)
    .or_else(|| ResultViewer::from_viewer_document_id(&pane.viewer_id))
    .ok_or_else(|| "The selected pane has no implemented viewer.".to_owned())?;

    Ok(ResolvedResultView {
        owner: ResultViewOwner::VisualizationPane {
            document_id,
            page_id,
            pane_id: pane.id,
        },
        dataset_id: run.dataset_id,
        dataset_digest: super::retained_dataset_digest(reads, run),
        run_index,
        viewer,
        analysis_indices: vec![analysis_index],
        primary_analysis_index: Some(analysis_index),
    })
}

fn analysis_instance_id(
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> crate::product::AnalysisInstanceId {
    analysis.provenance().map_or_else(
        || {
            let name = format!("legacy-analysis-v1/{}", analysis.id);
            crate::product::AnalysisInstanceId::from_namespace(
                run.dataset_id.as_uuid(),
                name.as_bytes(),
            )
        },
        |provenance| provenance.source_instance_id(),
    )
}

const fn viewer_uses_analysis_stack(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    )
}

/// One compatibility predicate shared by context resolution and later viewer
/// operations. It answers whether this retained analysis can be the evidence
/// behind the named viewer; it never mutates global selection to make it so.
pub(crate) fn analysis_supports_viewer(viewer: ResultViewer, analysis: &AnalysisResult) -> bool {
    rspice_results::renderability::analysis_supports_viewer(
        viewer,
        analysis,
        super::frame_work::note_scan,
    )
}

/// The same predicate, with the retained-evidence verdict and every
/// structural question taken from the workspace's memos instead of walked
/// here.
///
/// The validator reads every sample of every waveform in the analysis, and
/// resolving the displayed view asks the question once per retained analysis
/// — on every frame, for a reader who is not touching anything. The memos are
/// keyed by dataset generation, so they answer the same questions these walks
/// would have. Every per-frame caller must use this one; the plain predicate
/// above is for callers that hold a run without a session to memoize against.
pub(crate) fn analysis_supports_viewer_memoized(
    reads: ResultsReads<'_>,
    dataset_id: DatasetId,
    viewer: ResultViewer,
    analysis: &AnalysisResult,
) -> bool {
    analysis.success
        && super::analysis_evidence_is_valid(reads, dataset_id, analysis)
        && rspice_results::renderability::viewer_can_render(viewer, analysis, |gate| {
            super::analysis_answers_structural_gate(reads, dataset_id, analysis, gate)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultPayload, AnalysisType, RetainedTrace, SimulationRun,
    };
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;

    fn waveform(name: &str) -> RetainedTrace {
        RetainedTrace::new(name, vec![1.0, 2.0], vec![0.0, 1.0], "#55aaff")
    }

    fn state_with_run(analyses: Vec<AnalysisResult>) -> ResultsTestHost {
        let mut run = SimulationRun::new(1);
        for analysis in analyses {
            run.add_analysis(analysis);
        }
        let dataset_id = run.dataset_id;
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        state.model.simulation.view.active_run_idx = Some(0);
        state.model.simulation.view.active_analysis_idx = Some(0);
        state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::ResultDataset(dataset_id));
        state
    }

    #[test]
    fn bode_resolves_the_displayed_frequency_analysis_not_the_global_selector() {
        let op = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
        let ac = AnalysisResult::new(2, AnalysisType::Ac, "AC")
            .with_waveforms(vec![waveform("|V(out)|")]);
        let mut state = state_with_run(vec![op, ac]);
        state.results.results.session.viewer = ResultViewer::Bode;

        let resolved =
            resolve_displayed_result_view(state.results_reads()).expect("displayed Bode context");

        assert_eq!(resolved.analysis_indices, vec![1]);
        assert_eq!(resolved.primary_analysis_index, Some(1));
        assert_eq!(
            resolved
                .primary_analysis(state.results_reads())
                .unwrap()
                .label,
            "AC"
        );
    }

    #[test]
    fn waveform_context_preserves_every_visible_strip_and_respects_maximize() {
        let tran_a = AnalysisResult::new(1, AnalysisType::Transient, "TRAN A")
            .with_waveforms(vec![waveform("V(a)")]);
        let tran_b = AnalysisResult::new(2, AnalysisType::Transient, "TRAN B")
            .with_waveforms(vec![waveform("V(b)")]);
        let mut state = state_with_run(vec![tran_a, tran_b]);
        state.results.results.session.viewer = ResultViewer::Waves;

        let all = resolve_displayed_result_view(state.results_reads()).expect("wave stack context");
        assert_eq!(all.analysis_indices, vec![0, 1]);

        let run = state.model.simulation.active_run().unwrap();
        state.results.results.session.maximized_strip = Some(AnalysisPresentationKey::new(
            run.dataset_id,
            &run.analyses[1],
        ));
        let maximized =
            resolve_displayed_result_view(state.results_reads()).expect("maximized stack context");
        assert_eq!(maximized.analysis_indices, vec![1]);
        assert_eq!(maximized.primary_analysis_index, Some(1));
    }

    #[test]
    fn table_accepts_payload_only_and_zero_mode_periodic_results() {
        for analysis_type in [AnalysisType::Pss, AnalysisType::Pstb] {
            let analysis = AnalysisResult::new(1, analysis_type, "Periodic").with_result_payload(
                AnalysisResultPayload::legacy_periodic_marker(analysis_type).unwrap(),
            );
            assert!(analysis_supports_viewer(ResultViewer::Table, &analysis));
            assert!(!analysis_supports_viewer(ResultViewer::Waves, &analysis));
        }
    }
}
