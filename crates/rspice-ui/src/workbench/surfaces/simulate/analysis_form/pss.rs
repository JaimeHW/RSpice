//! The PSS form: its nine fields, and what autonomous mode does to two of
//! them.
//!
//! Split out for the same reason the DC sweep form is: one control changes what
//! every other one means. Autonomous mode moves the period from the Fundamental
//! field to the oscillator node and stops the tone list from being read at all,
//! so the same nine fields mean two different things.
//!
//! Those two things are said on the two fields they are about, as hints rather
//! than prose, because the form's height must not move when the switch does
//! (`pss_oscillator_toggle_preserves_form_geometry`). They are said at all
//! because the engine has no opinion here to enforce. The core's
//! `PssConfig` carries no tone list (`rspice-core/src/analysis/pss/config.rs`);
//! the list exists so a *driven* run can close over the elaborated periodic
//! source set before dispatch, which is what
//! `Engine::validate_periodic_source_contract` checks and what its own first
//! line calls "a driven PSS period"
//! (`rspice-core/src/engine/transient.rs:1349-1352`). An autonomous solve
//! stabilizes from t=0 and then re-evaluates every placed source from t=0 on
//! each shooting period (`rspice-core/src/engine/pss.rs`:
//! `pss_run_stabilization`, `pss_simulate_one_period`, and the
//! `update_transient_rhs` pair in `pss_stamp_system`). So an oscillator's
//! startup kick fires once during stabilization and then contributes only its
//! initial level — unless one of its edges falls inside the detected period,
//! which makes it a drive rather than a kick. That is a fact to state, not a
//! configuration to refuse.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::PssDialogState;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, choice_row, enabled_choice_row,
    engineering_input_row, input_row, input_row_enabled, named_periodic_source_row,
    quantity_input_row,
};

pub(super) const PSS_FIELD_LABELS: [&str; 9] = [
    "Mode",
    "Fundamental",
    "Tones",
    "Stabilization cycles",
    "Shooting points",
    "Period tolerance",
    "Autonomous oscillator",
    "Oscillator node",
    "Save harmonics",
];

pub(super) const PSS_MODE_CHOICES: [&str; 1] = ["Driven shooting"];

/// Render the PSS fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut PssDialogState,
    circuit_sources: &[String],
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    choice_row(
        ui,
        PSS_FIELD_LABELS[0],
        &PSS_MODE_CHOICES,
        &mut setup.method_idx,
    );
    quantity_input_row(
        ui,
        PSS_FIELD_LABELS[1],
        &mut setup.fund_freq,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    named_periodic_source_row(
        ui,
        PSS_FIELD_LABELS[2],
        "pss-tones",
        &mut setup.tone_sources,
        circuit_sources,
    )
    .on_hover_text(tone_hint(setup.osc_mode));
    input_row(ui, PSS_FIELD_LABELS[3], &mut setup.tstab_periods);
    input_row(ui, PSS_FIELD_LABELS[4], &mut setup.points_per_period);
    engineering_input_row(ui, PSS_FIELD_LABELS[5], &mut setup.tolerance);
    enabled_choice_row(ui, PSS_FIELD_LABELS[6], &mut setup.osc_mode);
    // The oscillator field is a stable member of the grid. Toggling autonomous
    // mode changes enablement, not the position of every field that follows it.
    input_row_enabled(ui, PSS_FIELD_LABELS[7], &mut setup.osc_node, setup.osc_mode)
        .on_hover_text(OSCILLATOR_NODE_HINT);
    input_row(ui, PSS_FIELD_LABELS[8], &mut setup.num_harmonics);
}

/// What the tone list is, on the field that holds it.
///
/// Driven, it is a contract: `Engine::validate_periodic_source_contract`
/// refuses a run whose tone list does not close over the elaborated periodic
/// sources, and nothing on this form says so until the run is attempted.
/// Autonomous, it is not read at all.
const fn tone_hint(autonomous: bool) -> &'static str {
    if autonomous {
        "Not read in autonomous mode: the period comes from the oscillator node."
    } else {
        "Must name every placed transient source. The shooting period is defined by them, and \
         an omitted source would drive the solve without being counted in it."
    }
}

/// What the engine does with the rest of the circuit once the period is the
/// oscillator's, stated on the field that makes it so.
///
/// It names what autonomous mode does *not* read and what the engine does
/// anyway, because the two together are the whole question an oscillator's
/// author has: the kick source is not in the tone list, and it is still in the
/// circuit. Saying so is what replaced a refusal that made the mode
/// unsatisfiable on any circuit carrying one.
const OSCILLATOR_NODE_HINT: &str = "Every placed source still drives the \
                                    solve — stabilization walks it from t=0, and each shooting \
                                    period re-evaluates it from t=0 — so a startup kick whose \
                                    first edge falls after the detected period contributes only \
                                    its initial level, while one with an edge inside the period \
                                    drives every orbit.";
