//! The periodic AC form: the band the small signal is measured over, and the
//! two ends of the measurement.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::PacDialogState;

use super::{
    QuantityPresentationPolicy, SWEEP_KINDS, UiNumberLocale, choice_row, hinted_input_row,
    hinted_input_row_enabled, input_row, periodic_carrier_row, plan_policy_tolerance_row,
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
    // The card states the sideband range in one of two ways and refuses both
    // at once, so the symmetric field is withheld while either end below is
    // authored rather than painted beside a range it is not stating.
    let states_symmetric =
        setup.sideband_min.trim().is_empty() && setup.sideband_max.trim().is_empty();
    hinted_input_row_enabled(
        ui,
        "Max sideband",
        &mut setup.max_sideband,
        "symmetric ±n",
        states_symmetric,
    );
    hinted_input_row(
        ui,
        "Sideband min",
        &mut setup.sideband_min,
        "replaces max sideband",
    );
    hinted_input_row(
        ui,
        "Sideband max",
        &mut setup.sideband_max,
        "replaces max sideband",
    );
    switch_row(ui, "Include DC", &mut setup.include_dc);
    plan_policy_tolerance_row(ui, "Relative tolerance", &mut setup.reltol);
    plan_policy_tolerance_row(ui, "Absolute tolerance", &mut setup.abstol);
    // Last, because it is the only row that is about something outside this
    // analysis: every field above describes the small signal, and this one
    // names the large-signal solve the whole measurement sits on.
    periodic_carrier_row(ui, &mut setup.carrier_idx);
}
