//! The results feature's dialogs, the dataset export format question, Run
//! details…, the waveform calculator and the expression diagnostics tool,
//! and the results workspace's session; the result-document forms, the
//! external-result import and the request to open the specification editor.

use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::framework::command_registry::Registry;
use crate::workbench::state::WorkbenchSession;

/// What the results feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub struct ResultsState {
    /// The format question "Export dataset…" asks, while it is open.
    pub dataset_export: Option<crate::workbench::menu_bar::DatasetExportPicker>,
    /// Run details…: the provenance of one retained run. Boxed: the
    /// manifest it carries would otherwise outgrow the slot's stack budget.
    pub run_details: Option<Box<crate::workbench::docks::RunDetailsDialogState>>,

    /// Waveform calculator dialog open
    pub waveform_calculator_dialog: bool,
    /// The expression and unit diagnostics tool.
    pub expression_diagnostics_dialog: bool,

    /// `simulation.data_version` at the last time the Results view was shown;
    /// drives the "new results" badge on the Results tab.
    pub results_seen_version: u64,
    /// Results workspace state (viewer, cursors, plot caches).
    pub results: crate::workbench::documents::result_document::ResultsState,

    /// One-shot request to open the canonical specification authoring surface.
    /// Runtime-only and owned by navigation rather than Results presentation:
    /// activating the destination document is allowed to reconcile viewer UI,
    /// but must not erase the route that caused that activation.
    pub specification_editor_route_pending: bool,
    /// Runtime-only draft for the project-owned result-document transaction.
    pub create_result_document: rspice_results_ui::create_document::CreateResultDocumentDialogState,
    /// Runtime-only draft of the form open over a project result document.
    pub result_document_dialog: Option<rspice_results_ui::document_dialogs::ResultDocumentDialog>,
    /// Runtime-only external-result draft. Retained history changes only when
    /// this transaction explicitly commits.
    pub result_import:
        crate::workbench::documents::result_document::import::ResultImportDialogState,
    /// Calculator panel state
    pub calculator_panel: crate::workbench::app_state::session::calculator::CalculatorPanel,
    /// Specialized analysis viewer state grouped by analysis workspace.
    /// Boxed: six viewers' state and their caches.
    pub analysis: Box<crate::workbench::app_state::viewer_capabilities::AnalysisWorkspaceState>,
}

impl ResultsState {
    /// What the results workspace's predicates read: this slot's viewer
    /// state, the model and the session.
    pub fn reads<'a>(
        &'a self,
        model: &'a Model,
        workbench: &'a WorkbenchSession,
    ) -> ResultsReads<'a> {
        ResultsReads {
            results: &self.results,
            analysis: &self.analysis,
            model,
            workbench,
        }
    }

    /// What the results workspace's commands write: this slot's viewer state
    /// and drafts, the model, the session, the platform state, and the run
    /// preflight, beside the application's command registry.
    pub fn parts<'a>(
        &'a mut self,
        model: &'a mut Model,
        workbench: &'a mut WorkbenchSession,
        platform: &'a PlatformState,
        preflight: &'a mut rspice_app_runs::workbench::app_state::run_preflight::PreflightDialogState,
        commands: &'static Registry,
    ) -> ResultsParts<'a> {
        self.parts_and_run_details(model, workbench, platform, preflight, commands)
            .0
    }

    /// The results workspace's parts, and beside them Run details…, which the
    /// data browser opens on a retained run.
    pub(crate) fn parts_and_run_details<'a>(
        &'a mut self,
        model: &'a mut Model,
        workbench: &'a mut WorkbenchSession,
        platform: &'a PlatformState,
        preflight: &'a mut rspice_app_runs::workbench::app_state::run_preflight::PreflightDialogState,
        commands: &'static Registry,
    ) -> (
        ResultsParts<'a>,
        &'a mut Option<Box<crate::workbench::docks::RunDetailsDialogState>>,
    ) {
        let parts = ResultsParts {
            results: &mut self.results,
            analysis: &mut self.analysis,
            result_document_dialog: &mut self.result_document_dialog,
            create_result_document: &mut self.create_result_document,
            result_import: &mut self.result_import,
            specification_editor_route_pending: &mut self.specification_editor_route_pending,
            model,
            workbench,
            platform,
            preflight,
            commands,
        };
        (parts, &mut self.run_details)
    }

    /// The results frame hook. How AC traces are drawn has no control but
    /// Preferences, so the viewer takes it from there on every frame and never
    /// drifts from the page.
    pub fn follow_preferences(
        &mut self,
        preferences: &crate::workbench::preferences::UserPreferences,
    ) {
        self.results.session.ac_display = preferences.ac_display();
    }

    /// Start the session's waveform viewer from Results ▸ Waveform defaults:
    /// wrapped or unwrapped phase, and the minor grid. Its own toggles change
    /// both afterwards.
    pub fn start_waveform_viewer_session(
        &mut self,
        preferences: &crate::workbench::preferences::UserPreferences,
    ) {
        self.results.session.phase_continuous = preferences.unwrapped_phase();
        self.start_waveform_view_from_preferences(preferences);
    }

    /// Give a cleared waveform view the minor grid Results ▸ Waveform
    /// defaults asks for; clearing alone would always start it without.
    pub(crate) fn start_waveform_view_from_preferences(
        &mut self,
        preferences: &crate::workbench::preferences::UserPreferences,
    ) {
        self.results.session.show_minor_grid =
            preferences.toggle(crate::workbench::TogglePreference::WaveMinorGrid);
    }

    /// The results feature's part of replacing the design the session
    /// executes against: nothing seen, viewed or routed to under the previous
    /// design survives it.
    pub fn clear_design_scoped(
        &mut self,
        preferences: &crate::workbench::preferences::UserPreferences,
    ) {
        self.results_seen_version = 0;
        self.results.clear_design_scoped_state();
        self.start_waveform_view_from_preferences(preferences);
        self.specification_editor_route_pending = false;
    }

    /// The results feature's part of replacing the project: its
    /// project-owned presentation goes as well.
    pub fn clear_project_scoped(
        &mut self,
        preferences: &crate::workbench::preferences::UserPreferences,
    ) {
        self.results.clear_project_scoped_state();
        self.start_waveform_view_from_preferences(preferences);
    }
}

/// Each report applies over the results workspace's parts, as the
/// application state's own sink applies it.
impl crate::workbench::framework::ports::RunEventPort for ResultsState {
    fn apply_run_event(
        &mut self,
        event: crate::state::RunEvent,
        model: &mut Model,
        workbench: &mut WorkbenchSession,
        platform: &PlatformState,
        preflight: &mut rspice_app_runs::workbench::app_state::run_preflight::PreflightDialogState,
        commands: &'static Registry,
    ) {
        use crate::state::RunEventSink;
        self.parts(model, workbench, platform, preflight, commands)
            .apply_run_event(event);
    }
}

impl crate::workbench::framework::ports::ResultsPort for ResultsState {
    fn result_viewer(&self) -> rspice_results::result_presentation::ResultViewer {
        self.results.session.viewer
    }

    fn analysis_evidence_is_valid(
        &self,
        model: &crate::workbench::app_state::Model,
        dataset_id: crate::product::DatasetId,
        analysis: &crate::state::AnalysisResult,
    ) -> bool {
        crate::workbench::documents::result_document::retained_evidence_is_valid_over(
            &self.results,
            model,
            rspice_results::result_presentation::AnalysisPresentationKey::new(dataset_id, analysis),
        )
    }

    fn histogram_selection(&self) -> Option<&str> {
        self.analysis.histogram_state.selected.as_deref()
    }

    fn persistent_document_pane(
        &self,
        document: rspice_vocabulary::product::ResultDocumentId,
    ) -> Option<rspice_results::visualization_document::PaneId> {
        self.results.session.persistent_document_pane(document)
    }

    fn persistent_document_page(
        &self,
        document: rspice_vocabulary::product::ResultDocumentId,
    ) -> Option<rspice_results::visualization_document::PageId> {
        self.results.session.persistent_document_page(document)
    }

    fn plot_view(
        &self,
        viewer: rspice_results::result_presentation::ResultViewer,
        index: usize,
    ) -> rspice_results_ui::presentation::PlotView {
        self.results.session.plot_view(viewer, index)
    }

    fn analysis_plot_view_pane(
        &self,
        viewer: rspice_results::result_presentation::ResultViewer,
        analysis: rspice_results::result_presentation::AnalysisPresentationKey,
        pane: usize,
    ) -> rspice_results_ui::presentation::PlotView {
        self.results
            .session
            .analysis_plot_view_pane(viewer, analysis, pane)
    }

    fn waveform_visibility(
        &self,
        key: &rspice_results::result_presentation::SourceWaveformPresentationKey,
        dataset_default: bool,
    ) -> bool {
        self.results
            .session
            .waveform_visibility(key, dataset_default)
    }

    fn strip_markers(
        &self,
        analysis: rspice_results::result_presentation::AnalysisPresentationKey,
    ) -> Vec<rspice_results_ui::session::MarkerView<'_>> {
        self.results.session.strip_markers(analysis).collect()
    }

    fn cursors(&self) -> rspice_ui_kit::plot::CursorPair {
        self.results.session.cursors
    }

    fn fft_settings(&self) -> &rspice_results_ui::fft::FftState {
        &self.analysis.fft_state
    }

    fn histogram_settings(&self) -> &rspice_results_ui::histogram::HistogramState {
        &self.analysis.histogram_state
    }

    fn displayed_dataset_view(
        &self,
        model: &crate::workbench::app_state::Model,
        workbench: &crate::workbench::state::WorkbenchSession,
    ) -> Option<crate::workbench::framework::ports::DisplayedDatasetView> {
        use crate::workbench::documents::result_document::view_context::{
            ResultViewOwner, resolve_displayed_result_view,
        };
        let reads = crate::workbench::documents::result_document::ResultsReads {
            results: &self.results,
            analysis: &self.analysis,
            model,
            workbench,
        };
        let displayed = resolve_displayed_result_view(reads).ok()?;
        if displayed.owner != ResultViewOwner::Dataset {
            return None;
        }
        displayed.run(reads)?;
        Some(crate::workbench::framework::ports::DisplayedDatasetView {
            run_index: displayed.run_index,
            viewer: displayed.viewer,
            analysis_indices: displayed.analysis_indices,
            primary_analysis_index: displayed.primary_analysis_index,
        })
    }

    fn captured_viewport(
        &self,
        analysis: rspice_results::result_presentation::AnalysisPresentationKey,
    ) -> Option<crate::workbench::framework::ports::CapturedViewport> {
        crate::workbench::documents::result_document::captured_viewport(self, analysis)
    }

    fn saved_presentation(
        &self,
    ) -> &dyn rspice_app_services::workbench::lifecycle::project_lifecycle::ResultPresentationSource
    {
        &self.results
    }
}
