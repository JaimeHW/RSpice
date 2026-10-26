//! One feature on its own, for its tests: its state, a model, a session and
//! the platform services' state, with every peer it reads showing what a
//! fresh session shows.
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

use super::{
    CanvasGesturePort, CloudPort, Cx, DesignPort, Feature, FeatureCommand, FeatureRegistry, Peers,
    Reads,
};
use crate::workbench::framework::ports::{DisplayedDatasetView, ResultsPort};

/// Peers as a fresh session shows them.
struct NullPorts;

impl DesignPort for NullPorts {
    fn review_markers(&self) -> SchematicReviewMarkerVisibility {
        SchematicReviewMarkerVisibility::default()
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
}

/// A feature, its own state, and the model and session its commands run
/// over.
pub(crate) struct FeatureHarness<F: Feature> {
    pub(crate) state: F::State,
    pub(crate) model: Model,
    pub(crate) wb: F::Session,
    pub(crate) platform: PlatformState,
}

impl<F: Feature> FeatureHarness<F>
where
    F::State: Default,
{
    /// The feature with nothing of its own open, over `model` and `wb`.
    pub(crate) fn with_model(model: Model, wb: F::Session) -> Self {
        Self {
            state: F::State::default(),
            model,
            wb,
            platform: PlatformState::default(),
        }
    }

    /// Whether the feature's command `key` can run here, as the feature
    /// answers it.
    pub(crate) fn availability(&self, key: CommandKey) -> CommandAvailability {
        command(&FeatureRegistry::<F>::build(), key).availability(&Reads::new(
            &self.state,
            &self.model,
            &self.wb,
            &self.platform,
            &NullPorts,
        ))
    }

    /// Carry out the feature's command `key`, as the prologue would have let
    /// it through.
    pub(crate) fn execute(&mut self, key: CommandKey) {
        command(&FeatureRegistry::<F>::build(), key).execute(&mut Cx::new(
            &mut self.state,
            &mut self.model,
            &mut self.wb,
            &mut self.platform,
            Peers::new(&mut NullPorts, &NullPorts),
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
