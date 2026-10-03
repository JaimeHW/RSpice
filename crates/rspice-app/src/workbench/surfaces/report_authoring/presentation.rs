//! Source queries and authoritative edits for report presentation.
use super::*;
use rspice_app_types::product::ResultDocumentId;
use rspice_results_ui::report::{
    inspector::InspectorHost,
    preview::{PreviewHost, SummaryMetrics},
};

pub(super) struct ReportHost<'a>(pub(super) &'a mut RSpiceApp);

impl PreviewHost for ReportHost<'_> {
    fn summary_metrics(&self) -> SummaryMetrics {
        let metrics = ReportSummaryMetrics::from_state(&self.0.state);
        SummaryMetrics {
            checks_passing: metrics.checks_passing,
            checks_total: metrics.checks_total,
            joint_yield_percent: metrics.joint_yield.map(ReportJointYield::percent),
            pvt_completed: metrics.pvt_completed,
            pvt_total: metrics.pvt_total,
        }
    }
    fn selected_block(&self) -> Option<ReportBlockId> {
        self.0
            .state
            .workbench
            .report_authoring
            .selected_report_block
    }
    fn select_block(&mut self, block: ReportBlockId) {
        self.0
            .state
            .workbench
            .report_authoring
            .selected_report_block = Some(block);
    }
    fn reference_resolves(&self, reference: &ReportReferenceMode) -> bool {
        report_reference_resolves(&self.0.state, reference)
    }
    fn writable(&self) -> bool {
        report_mutation_allowed(&self.0.state)
    }
    fn blocked_reason(&self) -> &'static str {
        report_mutation_block_reason(&self.0.state)
    }
    fn set_block_enabled(
        &mut self,
        document: ResultDocumentId,
        block: ReportBlockId,
        enabled: bool,
    ) {
        set_report_block_enabled(self.0, document, block, enabled);
    }
    fn retained_figure_available(&self) -> bool {
        !report_figure_options(&self.0.state).is_empty()
    }
    fn add_element(&mut self) {
        open_add_report_element(self.0);
    }
    fn remove_element(&mut self) {
        self.0
            .state
            .workbench
            .report_authoring
            .remove_report_block_open = true;
        self.0.state.workbench.report_authoring.transaction_error = None;
    }
    fn insert_result(&mut self) {
        open_insert_result_document(self.0);
    }
}

impl InspectorHost for ReportHost<'_> {
    fn writable(&self) -> bool {
        report_mutation_allowed(&self.0.state)
    }
    fn blocked_reason(&self) -> &'static str {
        report_mutation_block_reason(&self.0.state)
    }
    fn bound_result_label(&self, document: &ReportDocument) -> (String, bool) {
        report_bound_result_label(&self.0.state, document)
    }
    fn transaction_error(&self) -> Option<&str> {
        self.0
            .state
            .workbench
            .report_authoring
            .transaction_error
            .as_deref()
    }
    fn commit_publication(&mut self, document: ResultDocumentId, setting: DocumentPublicationEdit) {
        commit_document_publication_setting(self.0, document, setting);
    }
    fn open_release(&mut self) {
        open_release_cockpit(self.0);
    }
}

impl rspice_results_ui::report::composer::ComposerHost for ReportHost<'_> {
    fn project_open(&self) -> bool {
        self.0.state.project_lifecycle.is_open()
    }
    fn synchronize_selection(&mut self) {
        synchronize_report_selection(&mut self.0.state);
    }
    fn document_snapshot(&self) -> Option<ReportDocument> {
        active_document(&self.0.state).cloned()
    }
    fn selected_page(&self, document: &ReportDocument) -> Option<ReportPageId> {
        selected_page_id(&self.0.state, document)
    }
    fn create_document(&mut self) {
        open_create_document(self.0);
    }
    fn add_page(&mut self) {
        open_add_page(self.0);
    }
    fn page_properties(&mut self) {
        open_page_properties(self.0);
    }
    fn can_move_page(&self, direction: PageMoveDirection) -> bool {
        can_move_selected_page(&self.0.state, direction)
    }
    fn move_page(&mut self, direction: PageMoveDirection) {
        move_selected_page(self.0, direction);
    }
    fn select_page(&mut self, page: ReportPageId) {
        self.0.state.workbench.report_authoring.selected_page = Some(page);
        self.0
            .state
            .workbench
            .report_authoring
            .selected_report_block = None;
    }
    fn prepare_page_settings(&mut self, page: &rspice_results::report_document::ReportPage) {
        let page_id = page.id();
        let editor = &mut self.0.state.workbench.report_authoring;
        if editor.inline_page_settings_page != Some(page_id) {
            editor.inline_page_settings_page = Some(page_id);
            editor.inline_page_title_draft = page.title().to_owned();
            editor.transaction_error = None;
        }
    }
    fn title_draft(&mut self) -> &mut String {
        &mut self
            .0
            .state
            .workbench
            .report_authoring
            .inline_page_title_draft
    }
    fn commit_page_setting(&mut self, page: ReportPageId, setting: PageSettingEdit) {
        commit_page_setting(self.0, page, setting);
    }
    fn evidence_options(
        &self,
        binding: ReportPageEvidenceBinding,
    ) -> Vec<(String, ReportPageEvidenceBinding)> {
        evidence_binding_options(&self.0.state, binding)
    }
    fn evidence_label(&self, binding: ReportPageEvidenceBinding) -> String {
        evidence_binding_label(&self.0.state, binding)
    }
}
