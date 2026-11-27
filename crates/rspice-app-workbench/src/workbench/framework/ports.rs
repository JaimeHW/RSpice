//! What one feature shows that another reads, for a port whose answers are
//! values every part of the workbench above the session can name.
//!
//! A port here names only lower crates' values and the session, nothing of a
//! feature's, so the code that answers from a feature's reads, wherever it
//! sits above the session, takes the port and not the feature.

use rspice_app_services::workbench::lifecycle::project_lifecycle::SignalSetSource;
use rspice_app_services::workbench::slots::platform::PlatformState;
use rspice_app_state::workbench::app_state::model::Model;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, ResultViewer, SourceWaveformPresentationKey,
};
use rspice_results::visualization_document::{PageId, PaneId};
use rspice_results_ui::eye_diagram::EyeTimebase;
use rspice_results_ui::fft::{FftInputOptions, FftState};
use rspice_results_ui::histogram::HistogramState;
use rspice_results_ui::presentation::PlotView;
use rspice_results_ui::selection::SelectedResultTrace;
use rspice_results_ui::session::MarkerView;
use rspice_ui_kit::plot::CursorPair;
use rspice_vocabulary::product::ResultDocumentId;

use crate::simulation::ViewerReading;
use crate::state::{RunEvent, SpecializedViewerCacheProvenance};
use crate::workbench::framework::command_registry::Registry;
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::workbench::app_state::run_preflight::PreflightDialogState;

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

/// The results workspace as another feature's context lends it: what it
/// shows, and where the run tier's reports reach it.
pub trait ResultsPeer: ResultsPort + RunEventPort {}

impl<T: ResultsPort + RunEventPort> ResultsPeer for T {}

/// Where the run tier's reports reach the results workspace: the one write
/// another feature's context makes into it.
pub trait RunEventPort {
    /// Apply `event` to the results workspace before returning, over the
    /// parts of the session it writes, as the application's own sink does.
    fn apply_run_event(
        &mut self,
        event: RunEvent,
        model: &mut Model,
        workbench: &mut WorkbenchSession,
        platform: &PlatformState,
        preflight: &mut PreflightDialogState,
        commands: &'static Registry,
    );
}

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
    /// The trace the reader selected, while the model's active run still
    /// holds it.
    fn selected_trace(&self, model: &Model) -> Option<&SelectedResultTrace>;
    /// Where the reader's A and B cursors stand.
    fn cursors(&self) -> CursorPair;
    /// The signals of the shown run the plot drew last, each with the
    /// colour of its curve, the first strip's first; empty where the host
    /// keeps no plot.
    fn plotted_traces(&self) -> Vec<(String, egui::Color32)> {
        Vec::new()
    }
    /// The signal of the curve the pointer is over on the plot, while it is
    /// over one.
    fn pointed_trace(&self) -> Option<&str> {
        None
    }
    /// How the FFT viewer is set up.
    fn fft_settings(&self) -> &FftState;
    /// The response the reader chose for the Bode card to measure on the
    /// authored analysis of `analysis`; `None` where they chose none.
    fn bode_measured_signal(&self, _analysis: AnalysisPresentationKey) -> Option<String> {
        None
    }
    /// What the sheet shown prints beyond what its page reads for itself:
    /// a table sheet's tables in its own words, and the figures the card
    /// beside a plot shows. Empty where the host keeps no sheet.
    fn sheet_print(
        &self,
        _model: &Model,
        _workbench: &WorkbenchSession,
    ) -> rspice_results_ui::table::SheetPrint {
        rspice_results_ui::table::SheetPrint::default()
    }
    /// Whether the table sheet shown holds a table its page can print,
    /// without reading its words; `true` for a sheet whose page reads its
    /// own evidence.
    fn sheet_table_is_printable(&self, _model: &Model) -> bool {
        true
    }
    /// The Smith sheet as the reader set it: its grid, its reading and Z₀,
    /// the responses it lists and the network terms switched from how it
    /// opens them. `None` where the host keeps no Smith sheet.
    fn smith_sheet(&self) -> Option<&rspice_results_ui::smith_chart::SmithSheetState> {
        None
    }
    /// How the histogram viewer is set up.
    fn histogram_settings(&self) -> &HistogramState;
    /// What the transient viewers hold now.
    fn viewer_reading(&self) -> ViewerReading;
    /// How the FFT prepares the input it is given, as the reader
    /// set it.
    fn fft_input_options(&self) -> FftInputOptions;
    /// The bit period the reader chose for the eye of the result `owner`.
    fn eye_timebase_for(&self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase;
    /// What folds the eye of the result `owner`: its signal, its clock and
    /// the clock's period.
    fn eye_fold_for(
        &self,
        owner: SpecializedViewerCacheProvenance,
    ) -> rspice_results_ui::eye_diagram::EyeFold {
        rspice_results_ui::eye_diagram::EyeFold {
            timebase: self.eye_timebase_for(owner),
            ..rspice_results_ui::eye_diagram::EyeFold::default()
        }
    }
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
    fn saved_presentation(&self) -> &dyn SignalSetSource;

    /// The workspace as it stands now, as a copy that owns everything it
    /// holds. A campaign's frozen host reports into the copy and reads it
    /// back, so the workspace the reader sees is left as it was.
    fn frozen(&self) -> Box<dyn ResultsPeer>;
}
