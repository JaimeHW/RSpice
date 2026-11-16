//! The failing Monte Carlo trial the reader selected: its value against the
//! limit, the seed that reproduces it, and where it ran.

use egui::Ui;

use crate::workbench::app_state::Model;
use crate::workbench::state::VerificationSessionState;
use crate::workbench::surfaces::verify::trial_facts;

/// Paint the selected trial, and say whether there was one to paint.
pub(super) fn show(ui: &mut Ui, model: &Model, session: &VerificationSessionState) -> bool {
    let sections = trial_facts(model, session);
    super::point::facts(ui, &sections);
    !sections.is_empty()
}
