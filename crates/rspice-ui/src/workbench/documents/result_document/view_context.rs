//! One authoritative identity for the result presentation the reader sees.
//!
//! Rendering, measurement, export, and publication must not independently
//! consult the simulation's ordinal selections. A quick-view document can
//! display a compatible analysis even when the run selector points elsewhere,
//! and a persistent pane owns an immutable dataset/analysis binding of its
//! own. This module resolves those cases once into stable project identities.

use crate::product::{ContentDigest, DatasetId, ResultDocumentId};
use crate::results::visualization_document::{PageId, PaneId};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType,
    SimulationRun,
};
use crate::workbench::app_state::AppState;
use crate::workbench::state::{Workspace, WorkspaceDocumentId};

use super::{AnalysisPresentationKey, ResultViewer};

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
    pub(crate) fn run<'a>(&self, state: &'a AppState) -> Option<&'a SimulationRun> {
        state.simulation.runs.get(self.run_index).filter(|run| {
            run.dataset_id == self.dataset_id
                && super::retained_dataset_digest(state, run) == self.dataset_digest
        })
    }

    pub(crate) fn primary_analysis<'a>(&self, state: &'a AppState) -> Option<&'a AnalysisResult> {
        self.run(state)?.analyses.get(self.primary_analysis_index?)
    }

    pub(crate) fn analyses<'a>(
        &'a self,
        state: &'a AppState,
    ) -> impl Iterator<Item = &'a AnalysisResult> + 'a {
        let run = self.run(state);
        self.analysis_indices
            .iter()
            .filter_map(move |index| run.and_then(|run| run.analyses.get(*index)))
    }
}

/// Resolve the exact active Results document, viewer, and retained analyses.
pub(crate) fn resolve_displayed_result_view(
    state: &AppState,
) -> Result<ResolvedResultView, String> {
    let document = state
        .workbench
        .documents
        .active(Workspace::Results)
        .ok_or_else(|| "No active Results document is selected.".to_owned())?;
    match document {
        WorkspaceDocumentId::ResultDataset(dataset_id) => resolve_dataset_view(state, *dataset_id),
        WorkspaceDocumentId::VisualizationDocument(document_id) => {
            resolve_visualization_pane(state, *document_id)
        }
        _ => Err("The active Results document is not a result presentation.".to_owned()),
    }
}

fn resolve_dataset_view(
    state: &AppState,
    dataset_id: DatasetId,
) -> Result<ResolvedResultView, String> {
    let run_index = state
        .simulation
        .runs
        .iter()
        .position(|run| run.dataset_id == dataset_id)
        .ok_or_else(|| "The active result dataset is no longer retained.".to_owned())?;
    let run = &state.simulation.runs[run_index];
    let viewer = state.ui.results.viewer;
    let mut analysis_indices = run
        .analyses
        .iter()
        .enumerate()
        .filter_map(|(index, analysis)| {
            analysis_supports_viewer_memoized(state, dataset_id, viewer, analysis).then_some(index)
        })
        .collect::<Vec<_>>();

    if viewer_uses_analysis_stack(viewer) {
        analysis_indices.retain(|index| {
            let key = AnalysisPresentationKey::new(dataset_id, &run.analyses[*index]);
            !state.ui.results.hidden_strips.contains(&key)
        });
        if let Some(maximized) = state.ui.results.maximized_strip
            && let Some(index) = analysis_indices.iter().copied().find(|index| {
                AnalysisPresentationKey::new(dataset_id, &run.analyses[*index]) == maximized
            })
        {
            analysis_indices.clear();
            analysis_indices.push(index);
        }
    } else {
        let primary = selected_primary_index(state, run_index, run, &analysis_indices);
        analysis_indices.clear();
        analysis_indices.extend(primary);
    }

    let primary_analysis_index = selected_primary_index(state, run_index, run, &analysis_indices);
    Ok(ResolvedResultView {
        owner: ResultViewOwner::Dataset,
        dataset_id,
        dataset_digest: super::retained_dataset_digest(state, run),
        run_index,
        viewer,
        analysis_indices,
        primary_analysis_index,
    })
}

fn selected_primary_index(
    state: &AppState,
    run_index: usize,
    run: &SimulationRun,
    candidates: &[usize],
) -> Option<usize> {
    state
        .ui
        .results
        .active_wave_pane
        .as_ref()
        .map(|pane| pane.analysis)
        .and_then(|key| key.resolve(run).map(|(index, _)| index))
        .filter(|index| candidates.contains(index))
        .or_else(|| {
            (state.simulation.active_run_idx == Some(run_index))
                .then_some(state.simulation.active_analysis_idx)
                .flatten()
                .filter(|index| candidates.contains(index))
        })
        .or_else(|| candidates.first().copied())
}

fn resolve_visualization_pane(
    state: &AppState,
    document_id: ResultDocumentId,
) -> Result<ResolvedResultView, String> {
    let document = state
        .workspace
        .visualization_document(document_id)
        .ok_or_else(|| "The active visualization document no longer exists.".to_owned())?;
    let page_id = state
        .ui
        .results
        .persistent_document_page(document_id)
        .filter(|page_id| document.pages().iter().any(|page| page.id == *page_id))
        .or_else(|| document.pages().first().map(|page| page.id))
        .ok_or_else(|| "The active visualization document has no page.".to_owned())?;
    let pane = state
        .workbench
        .visualization_studio
        .active_pane
        .and_then(|pane_id| {
            document
                .panes()
                .iter()
                .find(|pane| pane.id.get() == pane_id && pane.page_id == page_id)
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
        .ok_or_else(|| "The selected result pane has no immutable dataset binding.".to_owned())?;
    let run_index = state
        .simulation
        .runs
        .iter()
        .position(|run| run.dataset_id == binding.dataset.dataset_id)
        .ok_or_else(|| "The pane's immutable result dataset is no longer retained.".to_owned())?;
    let run = &state.simulation.runs[run_index];
    if super::retained_dataset_digest(state, run) != binding.dataset.content_digest {
        return Err("The retained dataset does not match the pane's immutable binding.".to_owned());
    }
    let analysis_index = run
        .analyses
        .iter()
        .position(|analysis| analysis_instance_id(run, analysis) == binding.analysis_id)
        .ok_or_else(|| "The pane's bound analysis is no longer retained.".to_owned())?;
    let analysis = &run.analyses[analysis_index];
    let viewer =
        analysis_supports_viewer_memoized(state, run.dataset_id, state.ui.results.viewer, analysis)
            .then_some(state.ui.results.viewer)
            .or_else(|| ResultViewer::from_viewer_document_id(&pane.viewer_id))
            .ok_or_else(|| "The selected pane has no implemented viewer.".to_owned())?;

    Ok(ResolvedResultView {
        owner: ResultViewOwner::VisualizationPane {
            document_id,
            page_id,
            pane_id: pane.id,
        },
        dataset_id: run.dataset_id,
        dataset_digest: super::retained_dataset_digest(state, run),
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
    if !analysis.success || analysis.validate_retained_evidence().is_err() {
        return false;
    }
    viewer_can_render(viewer, analysis, |gate, analysis| {
        super::structural_gate_is_answered_directly(gate, analysis)
    })
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
    state: &AppState,
    dataset_id: DatasetId,
    viewer: ResultViewer,
    analysis: &AnalysisResult,
) -> bool {
    analysis.success
        && super::analysis_evidence_is_valid(state, dataset_id, analysis)
        && viewer_can_render(viewer, analysis, |gate, analysis| {
            super::analysis_answers_structural_gate(state, dataset_id, analysis, gate)
        })
}

/// Whether this viewer has anything to draw from the shape of the retained
/// result.
///
/// Most arms cost nothing — a payload discriminant, a non-empty vector. The
/// five that do not are asked through `structural`, so the caller decides
/// whether they are walked or read from the workspace memo; see
/// [`super::StructuralGate`].
fn viewer_can_render(
    viewer: ResultViewer,
    analysis: &AnalysisResult,
    structural: impl Fn(super::StructuralGate, &AnalysisResult) -> bool,
) -> bool {
    match viewer {
        ResultViewer::Waves => {
            analysis.analysis_type.is_time_domain() && !analysis.waveforms.is_empty()
        }
        ResultViewer::DcSweep => {
            analysis.analysis_type == AnalysisType::DcSweep && !analysis.waveforms.is_empty()
        }
        ResultViewer::Bode => structural(super::StructuralGate::BodeResponse, analysis),
        ResultViewer::NoiseContrib => {
            structural(super::StructuralGate::OrdinaryNoiseSpectrum, analysis)
        }
        ResultViewer::Fft | ResultViewer::Eye => {
            analysis.analysis_type.is_time_domain() && !analysis.waveforms.is_empty()
        }
        ResultViewer::HarmonicBalance => {
            structural(super::StructuralGate::HarmonicSpectrum, analysis)
        }
        ResultViewer::PhaseNoise => structural(super::StructuralGate::PhaseNoiseSpectrum, analysis),
        ResultViewer::Nyquist => analysis.waveforms.iter().any(|waveform| {
            waveform.complex.as_ref().is_some_and(|complex| {
                !complex.real.is_empty()
                    && complex.real.len() == complex.imag.len()
                    && complex.real.len() == waveform.x.len()
            })
        }),
        ResultViewer::Smith => structural(super::StructuralGate::SParameterStructure, analysis),
        // The analysis kind is part of the question, exactly as it is in the
        // tab strip's own gate: `op_inspector` renders nothing for an
        // analysis that is not a DC operating point, so a transient that
        // retained its bias solution must not be bindable to the OP sheet
        // from a Studio pane or a persistent document either.
        ResultViewer::Op => {
            analysis.analysis_type == AnalysisType::DcOp
                && operating_point_evidence_is_renderable(analysis)
        }
        ResultViewer::Contribution => {
            matches!(
                analysis.result_payload,
                Some(AnalysisResultPayload::Sensitivity { .. })
            )
        }
        ResultViewer::TransferFunction => {
            matches!(
                analysis.result_payload,
                Some(AnalysisResultPayload::TransferFunction { .. })
            )
        }
        ResultViewer::Specs => {
            !analysis.measurements.is_empty()
                || matches!(
                    analysis.result_payload,
                    Some(AnalysisResultPayload::ScalarMeasurements { .. })
                )
        }
        ResultViewer::Table => {
            !analysis.waveforms.is_empty() || operating_point_evidence_is_renderable(analysis)
        }
        ResultViewer::Hist => matches!(
            analysis.family_metadata,
            Some(AnalysisResultFamilyMetadata::MonteCarlo { ref variables, .. })
                if variables.iter().any(|variable| !variable.samples.is_empty())
        ),
        ResultViewer::PoleZero => {
            matches!(
                analysis.result_payload,
                Some(AnalysisResultPayload::PoleZero { .. })
            )
        }
        ResultViewer::Events => matches!(
            analysis.result_payload,
            Some(AnalysisResultPayload::TransientEvents {
                ref digital_traces,
                ref real_traces,
            }) if !digital_traces.is_empty() || !real_traces.is_empty()
        ),
        ResultViewer::Soa => matches!(
            analysis.result_payload,
            Some(AnalysisResultPayload::Soa { ref evaluations, .. }) if !evaluations.is_empty()
        ),
        ResultViewer::Reliability => matches!(
            analysis.result_payload,
            Some(AnalysisResultPayload::Reliability { ref devices }) if !devices.is_empty()
        ),
        ResultViewer::Optimization => matches!(
            analysis.family_metadata,
            Some(AnalysisResultFamilyMetadata::Optimization { ref iterations, .. }) if !iterations.is_empty()
        ),
        ResultViewer::Manifest => false,
    }
}

fn operating_point_evidence_is_renderable(analysis: &AnalysisResult) -> bool {
    analysis.dc_op.is_some()
        || analysis
            .device_op
            .as_ref()
            .is_some_and(|report| !report.is_empty())
        || matches!(
            analysis.result_payload,
            Some(AnalysisResultPayload::OperatingPoint { .. })
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, SimulationRun, WaveformData};

    fn waveform(name: &str) -> WaveformData {
        WaveformData::new(name, vec![1.0, 2.0], vec![0.0, 1.0], "#55aaff")
    }

    fn state_with_run(analyses: Vec<AnalysisResult>) -> AppState {
        let mut run = SimulationRun::new(1);
        for analysis in analyses {
            run.add_analysis(analysis);
        }
        let dataset_id = run.dataset_id;
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
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
        state.ui.results.viewer = ResultViewer::Bode;

        let resolved = resolve_displayed_result_view(&state).expect("displayed Bode context");

        assert_eq!(resolved.analysis_indices, vec![1]);
        assert_eq!(resolved.primary_analysis_index, Some(1));
        assert_eq!(resolved.primary_analysis(&state).unwrap().label, "AC");
    }

    #[test]
    fn waveform_context_preserves_every_visible_strip_and_respects_maximize() {
        let tran_a = AnalysisResult::new(1, AnalysisType::Transient, "TRAN A")
            .with_waveforms(vec![waveform("V(a)")]);
        let tran_b = AnalysisResult::new(2, AnalysisType::Transient, "TRAN B")
            .with_waveforms(vec![waveform("V(b)")]);
        let mut state = state_with_run(vec![tran_a, tran_b]);
        state.ui.results.viewer = ResultViewer::Waves;

        let all = resolve_displayed_result_view(&state).expect("wave stack context");
        assert_eq!(all.analysis_indices, vec![0, 1]);

        let run = state.simulation.active_run().unwrap();
        state.ui.results.maximized_strip = Some(AnalysisPresentationKey::new(
            run.dataset_id,
            &run.analyses[1],
        ));
        let maximized = resolve_displayed_result_view(&state).expect("maximized stack context");
        assert_eq!(maximized.analysis_indices, vec![1]);
        assert_eq!(maximized.primary_analysis_index, Some(1));
    }
}
