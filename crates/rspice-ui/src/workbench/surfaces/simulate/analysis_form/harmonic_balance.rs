//! The harmonic-balance form: the first tone, the tones added beside it, and
//! how the truncated spectrum is solved.
//!
//! Every tone after the first is authored the way the first is, under a
//! sub-header that numbers it, because a second fundamental is not a variant
//! of the first: it is another one.
//!
//! Every solver control below the spectrum is one the engine's `HbConfig`
//! reads. Eight of them used to be literals wired in three layers down —
//! mixing order 5, abstol 1e-12, an automatic collocation grid, the exact
//! Jacobian — or buffers the dialog carried with no row to type into, and an
//! RF designer could not reach any of them without hand-writing a deck. The
//! one control this form still does not offer is the verbose solver log: the
//! engine writes it with `log::debug!` on the `rspice_core` target, which
//! reaches stderr under `RSPICE_LOG` and never the studio's Console, so a
//! switch here would turn on output the product cannot show.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::HbDialogState;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, action_line, choice_row, engineering_input_row,
    hinted_input_row, input_row, input_row_enabled, quantity_input_row, sub_header, switch_row,
};

/// The order and wording of the harmonic-balance form's own fields.
///
/// The tone sub-rows are not here: they are the same three captions repeated
/// under a numbered sub-header, and they belong to the tone rather than to the
/// form.
pub(super) const HB_FIELD_LABELS: [&str; 15] = [
    "Fundamental",
    "Harmonics",
    "Source",
    "Oversample",
    "Max iters",
    "Solver",
    "Source stepping",
    "Mixing order",
    "Relative tolerance",
    "Absolute tolerance",
    "Damping",
    "Damping floor",
    "GMRES restart",
    "Collocation points",
    "Exact Jacobian",
];

/// The solver choices, in the order the engine's own flag reads them: index
/// one is `use_krylov`.
pub(super) const HB_SOLVER_CHOICES: [&str; 2] = ["newton", "krylov"];

/// Which solver choice the GMRES restart belongs to.
const HB_KRYLOV_CHOICE: usize = 1;

/// Render the harmonic-balance fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut HbDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        HB_FIELD_LABELS[0],
        &mut setup.fundamental,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    input_row(ui, HB_FIELD_LABELS[1], &mut setup.harmonics);
    input_row(ui, HB_FIELD_LABELS[2], &mut setup.fundamental_source);
    input_row(ui, HB_FIELD_LABELS[3], &mut setup.oversample);
    input_row(ui, HB_FIELD_LABELS[4], &mut setup.maxiter);
    choice_row(
        ui,
        HB_FIELD_LABELS[5],
        &HB_SOLVER_CHOICES,
        &mut setup.solver_idx,
    );
    switch_row(ui, HB_FIELD_LABELS[6], &mut setup.source_stepping);
    hinted_input_row(
        ui,
        HB_FIELD_LABELS[7],
        "products up to this order",
        &mut setup.max_mixing_order,
    );
    engineering_input_row(ui, HB_FIELD_LABELS[8], &mut setup.reltol);
    engineering_input_row(ui, HB_FIELD_LABELS[9], &mut setup.abstol);
    input_row(ui, HB_FIELD_LABELS[10], &mut setup.damping);
    input_row(ui, HB_FIELD_LABELS[11], &mut setup.min_damping);
    // Withheld rather than hidden on the Newton choice: the restart is the
    // Krylov solver's own parameter, and a row that disappeared would move
    // every field under it when the chooser moved
    // (`hb_solver_choice_preserves_form_geometry`).
    input_row_enabled(
        ui,
        HB_FIELD_LABELS[12],
        &mut setup.gmres_restart,
        setup.solver_idx == HB_KRYLOV_CHOICE,
    );
    hinted_input_row(
        ui,
        HB_FIELD_LABELS[13],
        "odd, empty is automatic",
        &mut setup.collocation_points,
    );
    switch_row(ui, HB_FIELD_LABELS[14], &mut setup.use_exact_jacobian);
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
