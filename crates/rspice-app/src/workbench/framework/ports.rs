//! What one feature shows that another reads, for a port whose answers are
//! values every part of the workbench above the session can name.
//!
//! A port here names only lower crates' values and the session, nothing of a
//! feature's, so the code that answers from a feature's reads, wherever it
//! sits above the session, takes the port and not the feature.

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

/// The retained dataset the results workspace shows opened directly, as the
/// reader sees it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DisplayedDatasetView {
    /// The run's index among the model's retained runs. Its dataset is still
    /// the one the view resolved, with the same content.
    pub(crate) run_index: usize,
    /// The viewer the dataset shows in.
    pub(crate) viewer: ResultViewer,
    /// The run's analyses the viewer shows, in order.
    pub(crate) analysis_indices: Vec<usize>,
    /// The analysis the viewer shows first, when it singles one out.
    pub(crate) primary_analysis_index: Option<usize>,
}

/// What the results workspace shows that another feature reads.
pub(crate) trait ResultsPort {
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
}
