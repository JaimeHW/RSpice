//! The transient-noise form: the window the transient form also asks for,
//! plus the noise the run injects into it and the seed that makes the
//! injection repeatable.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::plan::TransientNoiseDraft;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, input_row, quantity_input_row, switch_row,
};

/// Render the transient-noise fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut TransientNoiseDraft,
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
    quantity_input_row(
        ui,
        "Start time",
        &mut setup.start_time,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Max step",
        &mut setup.max_step,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    input_row(ui, "Seed", &mut setup.seed);
    quantity_input_row(
        ui,
        "Noise fmax",
        &mut setup.noise_fmax,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    input_row(ui, "Noise scale", &mut setup.scale);
    switch_row(
        ui,
        "Use initial conditions",
        &mut setup.use_initial_conditions,
    );
}
