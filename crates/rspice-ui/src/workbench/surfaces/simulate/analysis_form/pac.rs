//! The periodic AC form: the band the small signal is measured over, and the
//! two ends of the measurement.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::PacDialogState;

use super::{
    QuantityPresentationPolicy, SWEEP_KINDS, UiNumberLocale, choice_row, input_row,
    quantity_input_row, sweep_point_field_label, switch_row,
};

/// Render the periodic AC fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut PacDialogState,
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
    input_row(ui, "Input src", &mut setup.input_source);
    input_row(ui, "Output", &mut setup.output_node);
    input_row(ui, "Output ref", &mut setup.output_ref);
    input_row(ui, "Magnitude", &mut setup.pac_magnitude);
    input_row(ui, "Max sideband", &mut setup.max_sideband);
    switch_row(ui, "Include DC", &mut setup.include_dc);
}
