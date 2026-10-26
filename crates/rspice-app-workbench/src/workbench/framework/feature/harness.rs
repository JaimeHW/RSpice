//! One feature on its own, for its tests: its state, a model, a session and
//! the platform services' state, with every peer it reads showing what a
//! fresh session shows, an export IO that writes nothing, and a run
//! controller of its own with no run prepared.
//!
//! The harness answers and runs the feature's own commands as the feature
//! does, in the same context. The shell's prologue (a project operation in
//! flight, no project open, a read-only drawing, a pending inspector edit)
//! reads the whole application, so a test of the prologue runs in the shell.
//! A command the feature raises waits in the session's outbox; nothing runs
//! it here.

use std::sync::LazyLock;

use rspice_results::result_presentation::{
    AnalysisPresentationKey, ResultViewer, SourceWaveformPresentationKey,
};
use rspice_results::visualization_document::{PageId, PaneId};
use rspice_results_ui::fft::FftState;
use rspice_results_ui::histogram::HistogramState;
use rspice_results_ui::presentation::PlotView;
use rspice_results_ui::session::MarkerView;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;
use rspice_ui_kit::plot::CursorPair;
use rspice_vocabulary::product::ResultDocumentId;

use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::framework::command_registry::CommandAvailability;
use rspice_app_runs::simulation::SimulationController;
use rspice_app_runs::workbench::slots::runs::RunsState;

use rspice_app_services::io::WaveformDataset;
use rspice_app_services::workbench::workflows::export_workflow::{
    ExportWorkflowIo, SaveDialogConfig,
};

use super::{
    CanvasGesturePort, CloudPort, Cx, DesignPort, Feature, FeatureCommand, FeatureRegistry,
    HardcopyPort, LowerHandles, Peers, PublicationDraft, Reads, SimulatePort,
};
use crate::workbench::framework::ports::{CapturedViewport, DisplayedDatasetView, ResultsPort};

/// Peers as a fresh session shows them.
struct NullPorts;

/// The export IO a feature's own test runs with: its save picker is
/// cancelled, and it writes no file.
struct NullExportIo;

impl ExportWorkflowIo for NullExportIo {
    fn show_save_dialog(
        &self,
        _config: SaveDialogConfig<'_>,
    ) -> Result<Option<std::path::PathBuf>, String> {
        Ok(None)
    }

    fn write_text_file(&self, _path: &std::path::Path, _contents: &str) -> Result<(), String> {
        Err("a feature's own test writes no file".to_owned())
    }

    fn write_waveform_csv(
        &self,
        _dataset: &WaveformDataset,
        _path: &std::path::Path,
    ) -> Result<(), String> {
        Err("a feature's own test writes no file".to_owned())
    }
}

impl DesignPort for NullPorts {
    fn review_markers(&self) -> SchematicReviewMarkerVisibility {
        SchematicReviewMarkerVisibility::default()
    }

    /// A fresh session's pointer rests on no canvas.
    fn canvas_hover(&self) -> Option<(f64, f64)> {
        None
    }
}

/// A fresh session holds no canvas gesture to let go of.
impl CanvasGesturePort for NullPorts {
    fn let_go_of_canvas_gesture(&mut self, _model: &mut Model) {}
}

impl CloudPort for NullPorts {
    fn license_dialog_open(&self) -> bool {
        false
    }

    fn license_holder(&self) -> Option<&str> {
        None
    }
}

/// A feature's own test builds no publication: the snapshot is the hardcopy
/// feature's to build, over the whole project.
impl HardcopyPort for NullPorts {
    fn build_publication_snapshot(
        &self,
        _model: &Model,
        _workbench: &crate::workbench::state::WorkbenchSession,
        _results: &dyn ResultsPort,
        _runs: &RunsState,
        _draft: &PublicationDraft,
    ) -> Result<rspice_publication_contract::PublicationSnapshot, String> {
        Err("a feature's own test builds no publication snapshot".to_owned())
    }
}

/// A fresh session has selected no specification.
impl SimulatePort for NullPorts {
    fn selected_specification(&self) -> Option<&str> {
        None
    }
}

/// The FFT and histogram viewers as a fresh session sets them up.
static FRESH_FFT: LazyLock<FftState> = LazyLock::new(FftState::default);
static FRESH_HISTOGRAM: LazyLock<HistogramState> = LazyLock::new(HistogramState::default);

/// A fresh session shows the first viewer, unzoomed, with every waveform as
/// its dataset shows it, no markers or cursors placed, and has chosen no
/// histogram measurement and no result document's pane or page.
impl ResultsPort for NullPorts {
    fn result_viewer(&self) -> ResultViewer {
        ResultViewer::default()
    }

    /// A fresh session holds no results, so no retained evidence validates.
    fn analysis_evidence_is_valid(
        &self,
        _model: &Model,
        _dataset_id: rspice_vocabulary::product::DatasetId,
        _analysis: &crate::state::AnalysisResult,
    ) -> bool {
        false
    }

    fn histogram_selection(&self) -> Option<&str> {
        None
    }

    fn persistent_document_pane(&self, _document: ResultDocumentId) -> Option<PaneId> {
        None
    }

    fn persistent_document_page(&self, _document: ResultDocumentId) -> Option<PageId> {
        None
    }

    fn plot_view(&self, _viewer: ResultViewer, _index: usize) -> PlotView {
        PlotView::default()
    }

    fn analysis_plot_view_pane(
        &self,
        _viewer: ResultViewer,
        _analysis: AnalysisPresentationKey,
        _pane: usize,
    ) -> PlotView {
        PlotView::default()
    }

    fn waveform_visibility(
        &self,
        _key: &SourceWaveformPresentationKey,
        dataset_default: bool,
    ) -> bool {
        dataset_default
    }

    fn strip_markers(&self, _analysis: AnalysisPresentationKey) -> Vec<MarkerView<'_>> {
        Vec::new()
    }

    fn cursors(&self) -> CursorPair {
        CursorPair::default()
    }

    fn fft_settings(&self) -> &FftState {
        &FRESH_FFT
    }

    fn histogram_settings(&self) -> &HistogramState {
        &FRESH_HISTOGRAM
    }

    fn displayed_dataset_view(
        &self,
        _model: &Model,
        _workbench: &crate::workbench::state::WorkbenchSession,
    ) -> Option<DisplayedDatasetView> {
        None
    }

    /// A fresh session pins no window on any sheet.
    fn captured_viewport(&self, _analysis: AnalysisPresentationKey) -> Option<CapturedViewport> {
        None
    }
}

/// A feature, its own state and what it runs, and the model and session its
/// commands run over.
pub struct FeatureHarness<F: Feature> {
    pub state: F::State,
    pub own: F::Handles,
    pub model: Model,
    pub wb: F::Session,
    pub(crate) platform: PlatformState,
    pub(crate) runs: RunsState,
    pub(crate) run_controller: SimulationController,
}

impl<F: Feature> FeatureHarness<F>
where
    F::State: Default,
    F::Handles: Default,
{
    /// The feature with nothing of its own open, over `model` and `wb`.
    pub fn with_model(model: Model, wb: F::Session) -> Self {
        Self {
            state: F::State::default(),
            own: F::Handles::default(),
            model,
            wb,
            platform: PlatformState::default(),
            runs: RunsState::default(),
            run_controller: SimulationController::new(),
        }
    }

    /// Whether the feature's command `key` can run here, as the feature
    /// answers it.
    pub fn availability(&self, key: CommandKey) -> CommandAvailability {
        command(&FeatureRegistry::<F>::build(), key).availability(&Reads::new(
            &self.state,
            &self.own,
            &self.model,
            &self.wb,
            &self.platform,
            &NullPorts,
        ))
    }

    /// Carry out the feature's command `key`, as the prologue would have let
    /// it through.
    pub fn execute(&mut self, key: CommandKey) {
        command(&FeatureRegistry::<F>::build(), key).execute(&mut Cx::new(
            &mut self.state,
            &mut self.own,
            &mut self.model,
            &mut self.wb,
            &mut self.platform,
            &mut self.runs,
            LowerHandles {
                export_io: &NullExportIo,
                run_controller: &mut self.run_controller,
            },
            Peers::new(
                &mut NullPorts,
                Some(&NullPorts),
                Some(&NullPorts),
                Some(&NullPorts),
                &NullPorts,
            ),
        ));
    }
}

/// The command `key` names in `registry`; a test that names another
/// feature's command is wrong, so it fails.
fn command<F: Feature>(registry: &FeatureRegistry<F>, key: CommandKey) -> &FeatureCommand<F> {
    registry
        .command(key)
        .unwrap_or_else(|| panic!("the feature declares no {key:?}"))
}
