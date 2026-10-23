//! The results feature's dialogs, the dataset export format question, the
//! waveform calculator and the expression diagnostics tool, and the results
//! workspace's session; the result-document forms, the external-result import
//! and the request to open the specification editor.

/// What the results feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct ResultsState {
    /// The format question "Export dataset…" asks, while it is open.
    pub(crate) dataset_export: Option<crate::workbench::menu_bar::DatasetExportPicker>,

    /// Waveform calculator dialog open
    pub waveform_calculator_dialog: bool,
    /// The expression and unit diagnostics tool.
    pub expression_diagnostics_dialog: bool,

    /// A retained result document asked to be taken back to the plan that
    /// produced it.
    ///
    /// Result documents are read-only projections and do not dispatch; the
    /// frame drains this the way it already drains the export requests they
    /// raise. Transient.
    pub(crate) open_producing_plan_requested: bool,
    /// `simulation.data_version` at the last time the Results view was shown;
    /// drives the "new results" badge on the Results tab.
    pub results_seen_version: u64,
    /// One-shot request to export the visible waveforms as CSV (needs the
    /// app's IO backend, so it is handled at the workbench boundary).
    pub export_csv_requested: bool,
    /// One-shot exact export of one dataset-bound typed Data Browser entity.
    /// The key, rather than a copied payload, crosses the UI/IO boundary so
    /// the export adapter must re-resolve immutable evidence at publication.
    pub(crate) export_result_quantities_requested:
        Option<Vec<rspice_results_ui::selection::ResultBrowserSelectionKey>>,
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
