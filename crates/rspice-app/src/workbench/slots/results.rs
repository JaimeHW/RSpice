//! The results feature's dialogs, the dataset export format question, Run
//! details…, the waveform calculator and the expression diagnostics tool,
//! and the results workspace's session; the result-document forms, the
//! external-result import and the request to open the specification editor.

/// What the results feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct ResultsState {
    /// The format question "Export dataset…" asks, while it is open.
    pub(crate) dataset_export: Option<crate::workbench::menu_bar::DatasetExportPicker>,
    /// Run details…: the provenance of one retained run.
    pub(crate) run_details: Option<crate::workbench::docks::RunDetailsDialogState>,

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
    pub(crate) specification_editor_route_pending: bool,
    /// Runtime-only draft for the project-owned result-document transaction.
    pub create_result_document: rspice_results_ui::create_document::CreateResultDocumentDialogState,
    /// Runtime-only draft of the form open over a project result document.
    pub result_document_dialog: Option<rspice_results_ui::document_dialogs::ResultDocumentDialog>,
    /// Runtime-only external-result draft. Retained history changes only when
    /// this transaction explicitly commits.
    pub result_import:
        crate::workbench::documents::result_document::import::ResultImportDialogState,
    /// Calculator panel state
    pub(crate) calculator_panel: crate::workbench::app_state::session::calculator::CalculatorPanel,
    /// Specialized analysis viewer state grouped by analysis workspace.
    /// Boxed: six viewers' state and their caches.
    pub(crate) analysis:
        Box<crate::workbench::app_state::viewer_capabilities::AnalysisWorkspaceState>,
}

impl ResultsState {
    /// The results frame hook. How AC traces are drawn has no control but
    /// Preferences, so the viewer takes it from there on every frame and never
    /// drifts from the page.
    pub(crate) fn follow_preferences(
        &mut self,
        preferences: &crate::workbench::preferences::UserPreferences,
    ) {
        self.results.session.ac_display = preferences.ac_display();
    }
}
