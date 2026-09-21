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
    super::field_note(
        ui,
        "Equal start and stop requests a spot spectrum. Integrated noise requires a frequency band; SPICE-compatible linear sweeps need at least three requested points.",
    );
    input_row(ui, "Output", &mut setup.output_node);
    input_row(ui, "Output ref", &mut setup.output_ref);
    ui.add_enabled_ui(setup.noise_ref_idx == 1, |ui| {
        input_row(ui, "Input src", &mut setup.input_source);
        input_row(ui, "Input sideband", &mut setup.input_sideband);
    });
    ui.add_enabled_ui(
        setup.noise_ref_idx != 2 && setup.sampling.mode_idx == 0,
        |ui| {
            input_row(ui, "Output sideband", &mut setup.output_sideband);
        },
    );
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
    ui.add_enabled_ui(setup.noise_ref_idx != 2, |ui| {
        choice_row(ui, "Sampling", &["sideband spectrum", "fixed phase", "edge timing", "edge-to-edge delay"], &mut setup.sampling.mode_idx);
        match setup.sampling.mode_idx {
            1 => { input_row(ui, "Sample phase (deg)", &mut setup.sampling.phase); },
            2 | 3 => {
                edge_fields(ui, &mut setup.sampling.edge, false);
                if setup.sampling.mode_idx == 3 {
                    input_row(ui, "Reference signal", &mut setup.sampling.reference_node);
                    input_row(ui, "Reference return", &mut setup.sampling.reference_ref);
                    edge_fields(ui, &mut setup.sampling.reference_edge, true);
                    input_row(ui, "Added periods", &mut setup.sampling.periods);
                }
            }
            _ => (),
        }
        if setup.sampling.mode_idx != 0 {
            super::field_note(ui, "Sampling requires a driven carrier and offsets at or below half its frequency. Crossings are counted from phase zero within one period. Add periods when the measured edge follows the reference in a later cycle.");
        }
    });
    ui.scope(|ui| { switch_row(ui, "Integrated noise", &mut setup.integrated_noise); }).response
        .on_hover_text("Edge and delay modes retain timing jitter in seconds. Phase noise retains RMS phase error and timing jitter. Voltage sampling retains voltage RMS; input-referred RMS follows the selected source's units.");
    switch_row(ui, "Noise summary", &mut setup.noise_summary);
    periodic_carrier_row(ui, &mut setup.carrier_idx);
}

fn edge_fields(
    ui: &mut Ui,
    edge: &mut crate::simulation::dialog::pnoise::sampling::EdgeDraft,
    reference: bool,
) {
    let prefix = if reference { "Ref " } else { "" };
    input_row(ui, &format!("{prefix}threshold (V)"), &mut edge.threshold);
    choice_row(
        ui,
        &format!("{prefix}direction"),
        &["rising", "falling", "either"],
        &mut edge.direction_idx,
    );
    input_row(ui, &format!("{prefix}occurrence"), &mut edge.occurrence);
    input_row(
        ui,
        &format!("{prefix}phase tolerance (deg)"),
        &mut edge.phase_tolerance,
    );
    input_row(
        ui,
        &format!("{prefix}minimum slew (V/s)"),
        &mut edge.minimum_slew,
    );
}
