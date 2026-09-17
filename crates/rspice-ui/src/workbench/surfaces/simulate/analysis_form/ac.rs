//! The AC sweep form: the frequency band, and how the band is divided.
//!
//! The point field is named by the sweep mode beside it, because decades,
//! octaves and a total count are three different quantities. Its label is
//! resolved rather than written.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::workbench::app_state::AcSetup;

use super::{
    QuantityPresentationPolicy, SWEEP_KINDS, UiNumberLocale, choice_row, input_row,
    quantity_input_row, sweep_point_field_label,
};

/// Render the AC sweep fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut AcSetup,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Start",
        &mut setup.fstart,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Stop",
        &mut setup.fstop,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    input_row(ui, sweep_point_field_label(setup.sweep), &mut setup.points);
    choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep);
}
