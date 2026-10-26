//! Replacing what the session executes against.
//!
//! Opening, closing or replacing a design or a project retires everything the
//! previous one left behind: its netlist, runs and results, its findings and
//! the viewers drawn from them. Every feature holds part of that state, so the
//! replacement is one session transaction that clears each part in turn, in
//! a fixed order, within one frame; no frame ever shows two designs' state.

use super::AppState;

impl AppState {
    /// Replace all execution context, including project-owned marker identity,
    /// and state the replacement in the Console.
    ///
    /// The Console carries on across a project switch: it is the session's
    /// log, not the project's. So `announcement`, "Opened project amp (…)",
    /// is where one project's history ends and the next one's begins, and
    /// every replacement passes through here so that each states itself
    /// exactly once however it was reached.
    pub(crate) fn replace_project_execution_context(&mut self, announcement: impl Into<String>) {
        self.clear_design_execution_context();
        self.results.clear_project_scoped(&self.model.preferences);
        self.record(
            crate::diagnostics::LogSeverity::Info,
            crate::diagnostics::LogSource::Studio,
            announcement,
        );
    }

    /// Clear execution and viewer state tied to the previous design document.
    ///
    /// Use when replacing the active schematic with unrelated design
    /// content. File identity and dirty state are intentionally left to the
    /// caller because open/import/new workflows each own those semantics.
    pub(crate) fn clear_design_execution_context(&mut self) {
        self.model.design_execution_epoch = self.model.design_execution_epoch.wrapping_add(1);
        self.model.workspace.content.netlist_source = None;
        self.model.workspace.content.netlist_document = None;
        self.model.workspace.content.netlist_descriptor = None;
        self.model.workspace.content.retained_netlist_decks.clear();
        self.model.workspace.content.netlist_source_path = None;
        self.model.workspace.content.netlist_source_dirty = false;
        self.model.simulation = crate::state::SimulationState::default();
        // A notice's "Open in Results" names its run by number, and the new
        // history numbers its runs from one again: kept, the offer would
        // open this design's run of the same number. The notices stay, as
        // does "Show in Console": the session's log outlives the design.
        self.workbench.toasts.retire_offers(|action| {
            matches!(
                action,
                crate::ui::widgets::NotificationAction::OpenRunInResults { .. }
            )
        });
        self.code.netlist = Default::default();
        *self.model.netlist_artifacts = Default::default();
        *self.runs.manual_run = Default::default();
        self.code.code_workspace = Default::default();
        self.code.netlist_open_documents.clear();
        self.results.clear_design_scoped(&self.model.preferences);
        self.model.design_checks.drc_results = None;
        self.model.design_checks.drc_checked_version = 0;
        self.model.clear_all_design_checks();
        // Problems and debugger evidence are project-scoped under the
        // source-workspace contract. Clear them in the same transaction as
        // documents and results so a replacement can never render one frame
        // containing two designs' findings. The Console is not cleared: it
        // is the session's chronological record, and keeps the history from
        // before the replacement.
        self.model.diagnostics.clear();
        // The reading position goes with the findings it was read over: a
        // filter carried into another design hides conditions nobody there
        // chose to hide.
        self.problems_view = Default::default();
        self.results.analysis.clear_specialized_viewer_data();
    }

    /// Restore the result presentation a project retained, when it is opened
    /// or its result history is reverted: the markers, log-scaled panes and
    /// expression groups. It runs once the datasets and visualization
    /// documents it annotates are loaded, so a marker is kept only where its
    /// analysis still is.
    pub(crate) fn restore_result_presentation(
        &mut self,
        presentation: crate::state::result_presentation::ResultPresentation,
    ) {
        crate::workbench::documents::result_document::restore_presentation(self, presentation);
    }
}
