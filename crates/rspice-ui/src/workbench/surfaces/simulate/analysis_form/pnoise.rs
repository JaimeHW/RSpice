//! The periodic noise form: the offset band, the two ends of the measurement,
//! and what the result is referred to.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::PnoiseDialogState;

use super::{
    QuantityPresentationPolicy, SWEEP_KINDS, UiNumberLocale, choice_row, input_row,
    periodic_carrier_row, quantity_input_row, sweep_point_field_label, switch_row,
};

/// Render the periodic noise fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut PnoiseDialogState,
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
    ui.add_enabled_ui(setup.noise_ref_idx == 1, |ui| {
        input_row(ui, "Input src", &mut setup.input_source);
        input_row(ui, "Input sideband", &mut setup.input_sideband);
    });
    ui.add_enabled_ui(setup.noise_ref_idx != 2, |ui| {
        input_row(ui, "Output sideband", &mut setup.output_sideband);
    });
    super::field_note(
        ui,
        "Driven-noise frequencies are offsets: channel frequency = offset + sideband × carrier. Negative frequencies denote conjugate channels.",
    );
    ui.add_enabled_ui(setup.noise_ref_idx != 2, |ui| {
        input_row(ui, "Max sideband", &mut setup.max_sideband);
    }).response.on_hover_text("Driven noise truncates frequency-conversion sidebands. Phase noise integrates the PPV over the retained autonomous PSS time grid; configure that grid on the carrier.");
    choice_row(
        ui,
        "Refer to",
        &["output", "input", "phase"],
        &mut setup.noise_ref_idx,
    );
    ui.scope(|ui| { switch_row(ui, "Integrated noise", &mut setup.integrated_noise); }).response
        .on_hover_text("Phase mode retains RMS phase error in radians and timing jitter in seconds in Measurements; driven modes retain voltage RMS.");
    switch_row(ui, "Noise summary", &mut setup.noise_summary);
    periodic_carrier_row(ui, &mut setup.carrier_idx);
}
