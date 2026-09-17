//! The transient form: the window the run walks, and whether it starts from
//! the deck's initial conditions rather than the operating point.
//!
//! Max step is the one row with two spellings. `auto` is a word rather than a
//! time, so it is edited as text and only a stated bound is normalized as a
//! quantity: a field that rewrote `auto` into a number would be answering a
//! question the reader deliberately left to the solver.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::workbench::app_state::TranSetup;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, input_row, quantity_input_row, switch_row,
};

/// Render the transient fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut TranSetup,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Stop time",
        &mut setup.stop,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Step time",
        &mut setup.step,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Start time",
        &mut setup.start,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    if !setup.max_step.eq_ignore_ascii_case("auto") {
        quantity_input_row(
            ui,
            "Max step",
            &mut setup.max_step,
            QuantityInputKind::Time,
            policy,
            locale,
        );
    } else {
        input_row(ui, "Max step", &mut setup.max_step);
    }
    switch_row(ui, "Use initial conditions", &mut setup.uic);
}
