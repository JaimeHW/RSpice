//! The loop-stability form: the probe the loop is broken at, and the band the
//! response is measured over.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::StbDialogState;

use super::{
    QuantityPresentationPolicy, SWEEP_KINDS, UiNumberLocale, choice_row, input_row,
    quantity_input_row, stb_probe, sweep_point_field_label, switch_row,
};

/// Render the loop-stability fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut StbDialogState,
    placed_loop_probes: &[String],
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    stb_probe::row(
        ui,
        placed_loop_probes,
        &mut setup.probe_source,
        &mut setup.probe_reference,
    );
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
    switch_row(ui, "Nyquist contour", &mut setup.compute_nyquist);
}
