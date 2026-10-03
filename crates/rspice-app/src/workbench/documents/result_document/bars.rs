//! Retained-source controls and application transactions for Results bars.

use super::{
    AppState, ResultViewer, Ui, eye, instrument, sheet_domain_controls, sheet_purpose, specs, table,
};
use rspice_results_ui::chrome::{
    ExportRequests,
    bars::{self, SheetBarHost, SpecificationAction},
};

pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    bars::show(ui, &mut Host(state));
}

struct Host<'a>(&'a mut AppState);
impl SheetBarHost for Host<'_> {
    fn viewer(&self) -> ResultViewer {
        self.0.ui.results.viewer
    }
    fn export(&mut self, requests: ExportRequests) {
        self.0.ui.export_csv_requested |= requests.csv;
        self.0.ui.export_figure_requested |= requests.figure;
    }
    fn fft_summary(&self) -> Option<String> {
        self.0
            .analysis
            .fft_state
            .data
            .as_ref()
            .map(|d| format!("{} · {}", d.window.display_name(), d.fft_size))
    }
    fn operating_point_filter(&mut self) -> &mut String {
        &mut self.0.ui.results.op_filter
    }
    fn specification_editing(&self) -> bool {
        self.0.ui.results.spec_drafts.is_some()
    }
    fn request(&mut self, action: SpecificationAction) {
        let state = &mut *self.0;
        match action {
            SpecificationAction::Discard => {
                state.ui.results.spec_drafts = None;
                state.workbench.specification_editor_route_pending = false;
                // With no retained dataset there is no read-only Specs
                // evidence sheet to fall back to. Move to the ordinary
                // empty Results landing so the destination boundary does
                // not immediately reopen the editor the user dismissed.
                let selected_plan_dataset = state
                    .sim_setup
                    .stable_analysis_plan()
                    .ok()
                    .and_then(|plan| state.simulation.active_run_for_plan(plan.id()));
                if selected_plan_dataset.is_none() {
                    state.ui.results.viewer = ResultViewer::Waves;
                }
            }
            SpecificationAction::Apply => {
                if !specs::apply_drafts(state) {
                    state.push_sim_message(crate::diagnostics::ConsoleMessage::warning(
                        "Specs not applied — fix the invalid bound first",
                    ));
                }
            }
            SpecificationAction::Edit => specs::open_editor(state),
        }
    }
    fn instrument(&mut self, ui: &mut Ui) {
        instrument::show(ui, self.0);
    }
    fn eye_actions(&mut self, ui: &mut Ui) {
        eye::inline_actions(ui, self.0);
    }
    fn table_actions(&mut self, ui: &mut Ui) {
        table::inline_actions(ui, self.0);
    }
    fn domain_controls(&mut self, ui: &mut Ui) -> bool {
        sheet_domain_controls(ui, self.0)
    }
    fn purpose(&self) -> String {
        sheet_purpose(self.0)
    }
}
