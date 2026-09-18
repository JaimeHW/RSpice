//! The periodic transfer form: the band, the two ends of the transfer, and
//! the sideband each end is taken at.
//!
//! Both sidebands are authored here. The input one was pinned to 1 in the
//! run-configuration builder, which left a mixer's down-conversion path
//! unauthorable while its up-conversion path was not.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::PxfDialogState;

use super::{
    QuantityPresentationPolicy, SWEEP_KINDS, UiNumberLocale, choice_row, input_row,
    periodic_carrier_row, quantity_input_row, sweep_point_field_label,
};

/// Render the periodic transfer fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut PxfDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Start",
        &mut setup.start_freq,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Stop",
        &mut setup.stop_freq,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    input_row(
        ui,
        sweep_point_field_label(setup.sweep_type_idx),
        &mut setup.num_points,
    );
    choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
    input_row(ui, "Output", &mut setup.output_node);
    input_row(ui, "Output ref", &mut setup.output_ref);
    input_row(ui, "Out sideband", &mut setup.output_sideband);
    input_row(ui, "Input src", &mut setup.input_source);
    // Both ends of the transfer, each next to the probe it belongs to.
    // The input sideband was pinned to 1 in the run-configuration
    // builder, so a mixer's down-conversion path was not authorable
    // here while its up-conversion path was.
    input_row(ui, "In sideband", &mut setup.input_sideband);
    input_row(ui, "Max sideband", &mut setup.max_sideband);
    periodic_carrier_row(ui, &mut setup.carrier_idx);
}
