//! The harmonic-balance form: the first tone, the tones added beside it, and
//! how the truncated spectrum is solved.
//!
//! Every tone after the first is authored the way the first is, under a
//! sub-header that numbers it, because a second fundamental is not a variant
//! of the first: it is another one.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::HbDialogState;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, action_line, choice_row, input_row,
    quantity_input_row, sub_header, switch_row,
};

/// Render the harmonic-balance fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut HbDialogState,
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
    input_row(ui, "Source", &mut setup.fundamental_source);
    input_row(ui, "Oversample", &mut setup.oversample);
    input_row(ui, "Max iters", &mut setup.maxiter);
    choice_row(ui, "Solver", &["newton", "krylov"], &mut setup.solver_idx);
    switch_row(ui, "Source stepping", &mut setup.source_stepping);
    let mut remove: Option<usize> = None;
    for (idx, tone) in setup.additional_tones.iter_mut().enumerate() {
        sub_header(ui, &format!("Tone {}", idx + 2));
        quantity_input_row(
            ui,
            "Frequency",
            &mut tone.frequency,
            QuantityInputKind::Frequency,
            policy,
            locale,
        );
        input_row(ui, "Harmonics", &mut tone.harmonics);
        input_row(ui, "Source", &mut tone.source);
        if action_line(ui, "Remove tone") {
            remove = Some(idx);
        }
    }
    if let Some(idx) = remove {
        setup.additional_tones.remove(idx);
    }
    ui.add_space(4.0);
    if action_line(ui, "+ Add tone") {
        setup.additional_tones.push(Default::default());
    }
}
