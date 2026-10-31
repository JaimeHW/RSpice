//! What one feature shows that another reads, for a port whose answers are
//! values every part of the workbench above the session can name.
//!
//! A port here names only lower crates' values and the session, nothing of a
//! feature's, so the code that answers from a feature's reads, wherever it
//! sits above the session, takes the port and not the feature.

use rspice_app_services::workbench::lifecycle::project_lifecycle::ResultPresentationSource;
use rspice_app_state::workbench::app_state::model::Model;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, ResultViewer, SourceWaveformPresentationKey,
};
use rspice_results::visualization_document::{PageId, PaneId};
use rspice_results_ui::fft::FftState;
use rspice_results_ui::histogram::HistogramState;
use rspice_results_ui::presentation::PlotView;
use rspice_results_ui::session::MarkerView;
use rspice_ui_kit::plot::CursorPair;
use rspice_vocabulary::product::ResultDocumentId;

use crate::workbench::state::WorkbenchSession;

/// A model's release verdict.
///
/// Readable across the surfaces because it is a fact about the project, not
/// about the qualification page: a simulation plan cannot honestly sign off on a model the
/// gate has not cleared, and the alternative — a second verdict enum on the
/// reading side — is how two surfaces come to disagree about one model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualificationGate {
    Qualified,
    Review,
    Unqualified,
    Blocked,
    /// The model is one of the simulator's own compiled-in equation defaults,
    /// and the source-owned gate is not its gate.
    ///
    /// Off the ramp above rather than a shade of [`Self::Blocked`]: a built-in
    /// card has no project source to bind evidence to, so "not source-owned"
    /// is its design and not a finding against it. Deriving it from a failed
    /// source resolution is what once had a project nobody had touched yet
    /// reporting every foundation family as a red release-gate failure.
    EngineOwned,
}

impl QualificationGate {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Review => "review",
            Self::Unqualified => "unqualified",
            Self::Blocked => "blocked",
            Self::EngineOwned => "engine-owned",
        }
    }

    /// Whether the source-owned release gate governs this model at all.
    ///
    /// The counts that read as verdicts — qualified of how many, parity of how
    /// many — are ratios over the governed population, and totalling the
    /// exempt models into their denominators is the same false alarm the
    /// per-model badge used to carry.
    pub const fn is_gate_subject(self) -> bool {
        !matches!(self, Self::EngineOwned)
    }
}

/// One model's release gate, as another surface may read it.
///
/// The qualification page keeps its own working record of each model — the
/// per-domain vector counts, the desktop/wasm parity split, the evidence
/// digests — and it moves whenever that page does. This is the narrow
/// projection of it, so a field added there does not silently become another
/// surface's dependency.
#[derive(Debug, Clone)]
pub struct ModelGateFact {
    pub library: String,
    pub model: String,
    pub vectors: usize,
    pub evidenced_vectors: usize,
    pub passing_vectors: usize,
    pub open_dispositions: usize,
    /// The gate could not be read for this model at all, so its counts say
    /// nothing either way and a reader must not total them as passing.
    pub unreadable: bool,
    pub gate: QualificationGate,
}

/// The retained dataset the results workspace shows opened directly, as the
/// reader sees it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayedDatasetView {
    /// The run's index among the model's retained runs. Its dataset is still
    /// the one the view resolved, with the same content.
    pub run_index: usize,
    /// The viewer the dataset shows in.
    pub viewer: ResultViewer,
    /// The run's analyses the viewer shows, in order.
    pub analysis_indices: Vec<usize>,
    /// The analysis the viewer shows first, when it singles one out.
    pub primary_analysis_index: Option<usize>,
}

/// The two halves of a pinned view: the abscissa window, then the ordinate
/// window. Either may be absent — the reader can pin one axis and leave the
/// other to the page — which is why the pair is not one `Option`.
pub type CapturedViewport = (Option<(f64, f64)>, Option<(f64, f64)>);

/// What the results workspace shows that another feature reads.
pub trait ResultsPort {
    /// The viewer the results workspace shows a dataset in.
    fn result_viewer(&self) -> ResultViewer;
    /// Whether the retained evidence of `analysis`, an analysis of the
    /// dataset `dataset_id` among the model's runs, passes its own
    /// validator. The results workspace's memo answers it, so asking every
    /// frame walks the samples once per dataset revision.
    fn analysis_evidence_is_valid(
        &self,
        model: &rspice_app_state::workbench::app_state::model::Model,
        dataset_id: rspice_vocabulary::product::DatasetId,
        analysis: &rspice_app_state::state::AnalysisResult,
    ) -> bool;
    /// The measurement the histogram viewer shows, when one was chosen.
    fn histogram_selection(&self) -> Option<&str>;
    /// The pane of the result document `document` the reader last selected.
    fn persistent_document_pane(&self, document: ResultDocumentId) -> Option<PaneId>;
    /// The page of the result document `document` the reader last showed.
    fn persistent_document_page(&self, document: ResultDocumentId) -> Option<PageId>;

    /// The view the viewer `viewer` shows in its plot `index`.
    fn plot_view(&self, viewer: ResultViewer, index: usize) -> PlotView;
    /// The view pane `pane` of `analysis` shows in the viewer `viewer`.
    fn analysis_plot_view_pane(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        pane: usize,
    ) -> PlotView;
    /// Whether the waveform `key` shows, where its dataset shows it by
    /// `dataset_default` until the reader says otherwise.
    fn waveform_visibility(
        &self,
        key: &SourceWaveformPresentationKey,
        dataset_default: bool,
    ) -> bool;
    /// The markers the strip of `analysis` shows, quick ones first.
    fn strip_markers(&self, analysis: AnalysisPresentationKey) -> Vec<MarkerView<'_>>;
    /// Where the reader's A and B cursors stand.
    fn cursors(&self) -> CursorPair;
    /// How the FFT viewer is set up.
    fn fft_settings(&self) -> &FftState;
    /// How the histogram viewer is set up.
    fn histogram_settings(&self) -> &HistogramState;
    /// The retained dataset the workspace shows opened directly, over the
    /// model's runs and the session's open documents; `None` when the
    /// workspace shows no dataset, or one that is no longer retained as it
    /// was.
    fn displayed_dataset_view(
        &self,
        model: &Model,
        workbench: &WorkbenchSession,
    ) -> Option<DisplayedDatasetView>;

    /// The window the reader has pinned on the active sheet, as a capture of
    /// `analysis` takes it: the abscissa, and the ordinate where the sheet
    /// hands one over. `None` when neither is pinned.
    fn captured_viewport(&self, analysis: AnalysisPresentationKey) -> Option<CapturedViewport>;

    /// What a project file saves of the workspace's presentation, as the
    /// project lifecycle reads it beside the model when it captures the
    /// project: whether the project holds unsaved work asks it.
    fn saved_presentation(&self) -> &dyn ResultPresentationSource;
}
