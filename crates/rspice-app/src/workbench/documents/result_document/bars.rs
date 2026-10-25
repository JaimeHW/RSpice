//! Retained-source controls and application transactions for Results bars.

use super::{
    AppState, ResultViewer, Ui, eye, instrument, sheet_domain_controls, sheet_purpose, specs, table,
};
use crate::workbench::framework::requests::{ExportCsv, ExportFigure};
use rspice_results_ui::chrome::{
    ExportRequests,
    bars::{self, SheetBarHost, SpecificationAction},
};

pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    bars::show(ui, &mut Host(state));
}

/// Raise what a result document's export control asked for. The dataset
/// export needs the application's IO, and the figure goes through the
/// hardcopy workflow, so neither is done here.
pub(super) fn raise_exports(state: &mut AppState, requests: ExportRequests) {
    if requests.csv {
        state.workbench.outbox.push(ExportCsv);
    }
    if requests.figure {
        state.workbench.outbox.push(ExportFigure);
    }
}

struct Host<'a>(&'a mut AppState);
impl SheetBarHost for Host<'_> {
    fn viewer(&self) -> ResultViewer {
        self.0.results.results.session.viewer
    }
    fn export(&mut self, requests: ExportRequests) {
        raise_exports(self.0, requests);
    }
    fn fft_summary(&self) -> Option<String> {
        self.0
            .results
            .analysis
            .fft_state
            .data
            .as_ref()
            .map(|d| format!("{} · {}", d.window.display_name(), d.fft_size))
    }
    fn operating_point_filter(&mut self) -> &mut String {
        &mut self.0.results.results.session.op_filter
    }
    fn specification_editing(&self) -> bool {
        self.0.results.results.session.spec_drafts.is_some()
    }
    fn request(&mut self, action: SpecificationAction) {
        let state = &mut *self.0;
        match action {
            SpecificationAction::Discard => {
                state.results.results.session.spec_drafts = None;
                state.results.specification_editor_route_pending = false;
                // With no retained dataset there is no read-only Specs
                // evidence sheet to fall back to. Move to the ordinary
                // empty Results landing so the destination boundary does
                // not immediately reopen the editor the user dismissed.
                let selected_plan_dataset = state
                    .model
                    .sim_setup
                    .stable_analysis_plan()
                    .ok()
                    .and_then(|plan| state.model.simulation.active_run_for_plan(plan.id()));
                if selected_plan_dataset.is_none() {
                    state.results.results.session.viewer = ResultViewer::Waves;
                }
            }
            SpecificationAction::Apply => {
                if !specs::apply_drafts(state) {
                    state.prompt("Specs not applied: fix the invalid bound first.");
                }
            }
            SpecificationAction::Edit => specs::open_editor(&mut state.results_parts()),
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
