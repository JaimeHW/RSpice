//! The transient-noise form: the window the transient form also asks for,
//! plus the noise the run injects into it and the seed that makes the
//! injection repeatable.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::plan::TransientNoiseDraft;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, hinted_input_row, hinted_quantity_input_row,
    quantity_input_row, switch_row,
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
    hinted_input_row(ui, "Seed", &mut setup.seed, "blank = inherit; 0 is a seed");
    quantity_input_row(
        ui,
        "Noise fmax",
        &mut setup.noise_fmax,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    // The one engine control with no default of the form's own: left empty,
    // the run represents flicker down to `1/stop`, the longest period the
    // window can resolve. The row says so where it is typed, because a blank
    // field otherwise reads as something nobody has filled in yet.
    hinted_quantity_input_row(
        ui,
        "Noise fmin",
        "empty = 1/stop",
        &mut setup.noise_fmin,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    hinted_input_row(
        ui,
        "Noise scale",
        &mut setup.scale,
        "0 = deterministic baseline",
    );
    switch_row(
        ui,
        "Use initial conditions",
        &mut setup.use_initial_conditions,
    );
}
