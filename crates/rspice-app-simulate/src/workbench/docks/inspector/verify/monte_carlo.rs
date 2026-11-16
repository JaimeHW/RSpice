//! The Monte Carlo view's inspector subject: the failing trial the reader
//! selected, or else the campaign behind the sheet's statistics (its samples,
//! seed, sampling and duration), which the view itself never states.

use egui::Ui;
use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::section_header;

use crate::workbench::app_state::Model;
use crate::workbench::state::VerificationSessionState;
use crate::workbench::surfaces::verify::{campaign_facts, trial_facts};

/// Paint the selected trial, or else the campaign the statistics came from.
pub(super) fn show(ui: &mut Ui, model: &Model, session: &VerificationSessionState) {
    let trial = trial_facts(model, session);
    if !trial.is_empty() {
        super::point::facts(ui, &trial);
        return;
    }
    let duration = model
        .simulation
        .active_run()
        .map(|run| super::super::compact_readout(run.elapsed_time, "s"));
    let campaigns = campaign_facts(model, duration.as_deref());
    if campaigns.is_empty() {
        section_header(ui, "Monte Carlo", None);
        muted_inspector_copy(ui, "No Monte Carlo run");
    } else {
        super::point::facts(ui, &campaigns);
    }
}
