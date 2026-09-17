//! The DC mismatch form: the expression the spread is measured on, how wide a
//! spread is reported, and which variation sources are included in it.

use egui::Ui;

use crate::simulation::plan::DcMismatchDraft;

use super::{input_row, switch_row};

/// Render the DC mismatch fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut DcMismatchDraft) {
    input_row(ui, "Output expression", &mut setup.output_expression);
    input_row(ui, "Sigma multiplier", &mut setup.sigma_multiplier);
    input_row(ui, "Contributor limit", &mut setup.contributor_limit);
    switch_row(ui, "Process variation", &mut setup.include_process);
    switch_row(ui, "Local mismatch", &mut setup.include_mismatch);
    switch_row(
        ui,
        "Normalize contributions",
        &mut setup.normalized_contributions,
    );
}
