//! The pole-zero form: the two port pairs the transfer is taken between, and
//! which of its roots are reported.

use egui::Ui;

use crate::simulation::dialog::PzDialogState;

use super::{choice_row, input_row};

/// Render the pole-zero fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut PzDialogState) {
    input_row(ui, "Input +", &mut setup.input_pos);
    input_row(ui, "Input −", &mut setup.input_neg);
    input_row(ui, "Output +", &mut setup.output_pos);
    input_row(ui, "Output −", &mut setup.output_neg);
    choice_row(ui, "Transfer", &["V", "I"], &mut setup.transfer_idx);
    choice_row(
        ui,
        "Roots",
        &["both", "poles", "zeros"],
        &mut setup.analysis_idx,
    );
}
