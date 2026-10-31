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

use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    LifecycleParts, LifecycleSource, LifecycleSourceMut, LifecycleView, ResultPresentationSource,
    close_project_session_for_test, open_project_session_for_test,
};
use rspice_results::result_presentation::{
    AnalysisPresentationKey, ResultPresentation, ResultViewer, SourceWaveformPresentationKey,
};
use rspice_results::visualization_document::{PageId, PaneId};
use rspice_results_ui::fft::FftState;
use rspice_results_ui::histogram::HistogramState;
use rspice_results_ui::presentation::PlotView;
use rspice_results_ui::session::MarkerView;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;
use rspice_ui_kit::plot::CursorPair;
use rspice_vocabulary::product::ResultDocumentId;

use crate::state::SimulationState;
use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::framework::command_registry::{CommandAvailability, Registry};
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::simulation::SimulationController;
use rspice_app_runs::workbench::slots::runs::RunsState;

use rspice_app_services::io::WaveformDataset;
use rspice_app_services::workbench::workflows::export_workflow::{
    ExportWorkflowIo, SaveDialogConfig,
};

use super::{
    CanvasGesturePort, CloudPort, CodePort, Cx, DesignPort, Feature, FeatureCommand,
    FeatureRegistry, HardcopyPort, LowerHandles, ModelsPort, Peers, PublicationDraft, Reads,
    SimulatePort,
};
use crate::workbench::framework::ports::{
    CapturedViewport, DisplayedDatasetView, ModelGateFact, ResultsPort, RunEventPort,
};

/// Peers as a fresh session shows them.
struct NullPorts;

/// The commands a feature tested on its own reads: none is registered, so a
/// command's label is its key. A test that needs the application's commands
/// lends the shell's registry through [`FeatureHarness::commands`].
static NO_COMMANDS: LazyLock<Registry> = LazyLock::new(|| Registry::new(&[], Vec::new(), &[]));

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

    fn publication_timestamp_utc(&self) -> String {
        "1970-01-01T00:00:00Z".to_owned()
    }
}

/// A fresh session has selected no specification.
impl SimulatePort for NullPorts {
    fn selected_specification(&self) -> Option<&str> {
        None
    }
}

/// A fresh session's netlist editor shows no source, its caret on the first
/// line, and has settled nothing yet; no Verilog-A compilation is current.
impl CodePort for NullPorts {
    fn owned_source_active(&self) -> bool {
        false
    }

    fn shows_owned_root(&self) -> bool {
        false
    }

    fn netlist_cursor_line(&self) -> usize {
        0
    }

    fn netlist_shown(&self) -> rspice_app_runs::simulation::NetlistShown {
        rspice_app_runs::simulation::NetlistShown {
            settled: false,
            comparison: false,
            owned_source: false,
        }
    }

    fn veriloga_compilation(&self) -> Option<&rspice_app_runs::simulation::ProjectCompileReceipt> {
        None
    }
}

/// A fresh session's models authoring holds nothing unsaved, and it has
/// gated no model.
impl ModelsPort for NullPorts {
    fn has_unsaved_authoring_changes(&self) -> bool {
        false
    }

    fn model_editor_has_unsaved_changes(&self) -> bool {
        false
    }

    fn model_gate_facts(&self, _model: &Model) -> Vec<ModelGateFact> {
        Vec::new()
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

    fn saved_presentation(&self) -> &dyn ResultPresentationSource {
        self
    }
}

/// A fresh session's results workspace holds nothing a run's report changes
/// that a harness reads back, so a report reaches nothing.
impl RunEventPort for NullPorts {
    fn apply_run_event(
        &mut self,
        _event: crate::state::RunEvent,
        _model: &mut Model,
        _workbench: &mut WorkbenchSession,
        _platform: &PlatformState,
        _preflight: &mut rspice_app_runs::workbench::app_state::run_preflight::PreflightDialogState,
        _commands: &'static Registry,
    ) {
    }
}

/// What a fresh session saves of the results workspace: no markers, panes or
/// expression groups, and a marker high-water mark of zero.
impl ResultPresentationSource for NullPorts {
    fn project_presentation(&self, _simulation: &SimulationState) -> ResultPresentation {
        ResultPresentation {
            marker_id_high_water: Some(0),
            ..ResultPresentation::default()
        }
    }
}

/// A feature, its own state and what it runs, and the model and session its
/// commands run over.
pub struct FeatureHarness<F: Feature> {
    pub state: F::State,
    pub own: F::Handles,
    pub model: Model,
    pub wb: F::Session,
    /// What the platform services keep: whether a project is open among it.
    pub platform: PlatformState,
    /// What the run tier keeps: the run preflight among it.
    pub runs: RunsState,
    pub(crate) run_controller: SimulationController,
    pub(crate) model_hub: rspice_app_services::services::model_hub::ModelHubService,
    /// Every command the feature reads as registered.
    pub commands: &'static Registry,
    /// The design peer, which a context lends mutably.
    ports: NullPorts,
    /// The results peer, which a context lends mutably for the run tier's
    /// reports.
    results_peer: NullPorts,
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
            model_hub: rspice_app_services::services::model_hub::ModelHubService::unavailable(
                "The feature harness runs without a model-pack store.",
            ),
            commands: &NO_COMMANDS,
            ports: NullPorts,
            results_peer: NullPorts,
        }
    }

    /// The feature's context over the harness, as the shell lends one for a
    /// call: the feature's own state and handles, the model, the session and
    /// the platform and run tiers, with every peer as a fresh session shows
    /// it.
    pub fn cx(&mut self) -> Cx<'_, F> {
        Cx::new(
            &mut self.state,
            &mut self.own,
            &mut self.model,
            &mut self.wb,
            &mut self.platform,
            &mut self.runs,
            LowerHandles {
                export_io: &NullExportIo,
                run_controller: &mut self.run_controller,
                model_hub: &mut self.model_hub,
            },
            Peers::new(
                &mut self.ports,
                Some(&NullPorts),
                Some(&NullPorts),
                Some(&mut self.results_peer),
                Some(&NullPorts),
                &NullPorts,
                Some(&NullPorts),
            ),
            self.commands,
        )
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
            &self.runs,
            &NullPorts,
            self.commands,
        ))
    }

    /// Carry out the feature's command `key`, as the prologue would have let
    /// it through.
    pub fn execute(&mut self, key: CommandKey) {
        command(&FeatureRegistry::<F>::build(), key).execute(&mut self.cx());
    }
}

/// The project lifecycle reads a harness over the workbench session as it
/// reads the application: the model, the platform services' state and the
/// workspace in front, with the results a fresh session presents.
impl<F: Feature<Session = WorkbenchSession>> LifecycleSource for FeatureHarness<F> {
    fn lifecycle_view(&self) -> LifecycleView<'_> {
        LifecycleView {
            model: &self.model,
            platform: &self.platform,
            workspace: self.wb.workspace,
            presentation: &NullPorts,
        }
    }
}

impl<F: Feature<Session = WorkbenchSession>> LifecycleSourceMut for FeatureHarness<F> {
    fn lifecycle_parts(&mut self) -> LifecycleParts<'_> {
        LifecycleParts {
            model: &mut self.model,
            platform: &mut self.platform,
            workspace: self.wb.workspace,
            presentation: &NullPorts,
        }
    }
}

impl<F: Feature<Session = WorkbenchSession>> FeatureHarness<F> {
    /// Open a project session as the application's test setter does: the
    /// lifecycle's authority moves, with no baseline or binding.
    pub fn open_project_session(&mut self) {
        open_project_session_for_test(self);
    }

    /// Close the project session as the application's test setter does.
    pub fn close_project_session(&mut self) {
        close_project_session_for_test(self);
    }
}

/// The command `key` names in `registry`; a test that names another
/// feature's command is wrong, so it fails.
fn command<F: Feature>(registry: &FeatureRegistry<F>, key: CommandKey) -> &FeatureCommand<F> {
    registry
        .command(key)
        .unwrap_or_else(|| panic!("the feature declares no {key:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A feature's own test reads no clock: the null hardcopy port stamps a
    /// publication with the epoch, the stamp the hardcopy feature falls back
    /// to when the clock is out of range.
    #[test]
    fn the_null_hardcopy_port_stamps_the_epoch() {
        assert_eq!(
            NullPorts.publication_timestamp_utc(),
            "1970-01-01T00:00:00Z"
        );
    }
}
