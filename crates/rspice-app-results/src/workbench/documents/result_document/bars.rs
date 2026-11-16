//! Retained-source controls and application transactions for Results bars.

use super::{
    ResultViewer, ResultsParts, Ui, eye, instrument, sheet_domain_controls, sheet_purpose, table,
};
use crate::workbench::framework::requests::{ExportCsv, ExportFigure};
use rspice_results_ui::chrome::{
    ExportRequests,
    bars::{self, SheetBarHost},
};

pub(super) fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    bars::show(ui, &mut Host(state));
}

/// Raise what a result document's export control asked for. The dataset
/// export needs the application's IO, and the figure goes through the
/// hardcopy workflow, so neither is done here.
pub(super) fn raise_exports(state: &mut ResultsParts<'_>, requests: ExportRequests) {
    if requests.csv {
        state.workbench.outbox.push(ExportCsv);
    }
    if requests.figure {
        state.workbench.outbox.push(ExportFigure);
    }
}

struct Host<'a, 'p>(&'a mut ResultsParts<'p>);
impl SheetBarHost for Host<'_, '_> {
    fn viewer(&self) -> ResultViewer {
        self.0.results.session.viewer
    }
    fn export(&mut self, requests: ExportRequests) {
        raise_exports(self.0, requests);
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
        &mut self.0.results.session.op_filter
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
        sheet_purpose(self.0.results_reads())
    }
}
