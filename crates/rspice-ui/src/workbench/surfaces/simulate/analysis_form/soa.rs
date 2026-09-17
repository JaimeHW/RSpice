//! The safe-operating-area form: the transient window the checks are run
//! over, and the four terminal bounds each check is against.
//!
//! Each bound is greyed by the check that reads it, so a limit can never be
//! typed into a check that is off.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::SoaDialogState;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, input_row_enabled, quantity_input_row, switch_row,
};

/// Render the safe-operating-area fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut SoaDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Stop time",
        &mut setup.stop_time,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Step time",
        &mut setup.step_time,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    switch_row(ui, "Check Vgs", &mut setup.check_vgs_max);
    input_row_enabled(ui, "Max Vgs", &mut setup.max_vgs, setup.check_vgs_max);
    switch_row(ui, "Check Vds", &mut setup.check_vds_max);
    input_row_enabled(ui, "Max Vds", &mut setup.max_vds, setup.check_vds_max);
    switch_row(ui, "Check Vbe", &mut setup.check_vbe_max);
    input_row_enabled(ui, "Max Vbe", &mut setup.max_vbe, setup.check_vbe_max);
    switch_row(ui, "Check Vce", &mut setup.check_vce_max);
    input_row_enabled(ui, "Max Vce", &mut setup.max_vce, setup.check_vce_max);
}
