//! The sensitivity form: the output the derivatives are taken of, and the
//! point they are taken at.
//!
//! The frequency row belongs to the AC mode alone, so it is greyed rather
//! than hidden: a row that disappears takes the reason with it.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::SensDialogState;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, choice_row, input_row, quantity_input_row_enabled,
};

/// Render the sensitivity fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut SensDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    input_row(ui, "Output", &mut setup.output_expr);
    choice_row(ui, "Mode", &["DC", "AC"], &mut setup.sens_type_idx);
    quantity_input_row_enabled(
        ui,
        "Frequency",
        &mut setup.ac_freq,
        QuantityInputKind::Frequency,
        policy,
        locale,
        setup.sens_type_idx == 1,
    );
}
