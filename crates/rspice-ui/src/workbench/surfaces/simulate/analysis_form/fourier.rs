//! The Fourier form: the fundamental the spectrum is taken at, the transient
//! window it is taken over, and the two derived quantities it reports.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::FourierDialogState;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, input_row, quantity_input_row, switch_row,
};

/// Render the Fourier fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut FourierDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Fundamental",
        &mut setup.fundamental,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    input_row(ui, "Harmonics", &mut setup.harmonics);
    input_row(ui, "Output", &mut setup.output_node);
    quantity_input_row(
        ui,
        "From",
        &mut setup.start_time,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "To",
        &mut setup.stop_time,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    switch_row(ui, "Compute THD", &mut setup.compute_thd);
    switch_row(ui, "Normalize", &mut setup.normalize);
}
