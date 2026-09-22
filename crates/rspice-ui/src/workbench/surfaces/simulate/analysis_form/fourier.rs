//! The Fourier form: the fundamental the spectrum is taken at, the transient
//! window it is taken over, the outputs it decomposes, and the two derived
//! quantities it reports.
//!
//! Every output after the first is authored under a sub-header that numbers
//! it, the way the harmonic-balance form authors a second tone, because the
//! card's own `.FOUR freq harmonics out1 out2 ...` takes a list and one
//! transient serves every projection on it. The added rows sit below the
//! form's own fields so the fields do not move when the list grows.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::FourierDialogState;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, action_line, hinted_input_row, input_row,
    quantity_input_row, sub_header, switch_row,
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
    input_row(ui, "Periods", &mut setup.periods).on_hover_text(
        "Complete fundamental periods ending at To; From is the earliest allowed start.",
    );
    input_row(ui, "Output", &mut setup.output_node);
    input_row(ui, "Output ref", &mut setup.output_ref);
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
    let mut remove: Option<usize> = None;
    for (idx, output) in setup.additional_outputs.iter_mut().enumerate() {
        sub_header(ui, &format!("Output {}", idx + 2));
        hinted_input_row(
            ui,
            "Output",
            output,
            "V(node), V(node+, node-) or I(device)",
        );
        if action_line(ui, "Remove output") {
            remove = Some(idx);
        }
    }
    if let Some(idx) = remove {
        setup.additional_outputs.remove(idx);
    }
    ui.add_space(4.0);
    if action_line(ui, "+ Add output") {
        setup.additional_outputs.push(String::new());
    }
}
