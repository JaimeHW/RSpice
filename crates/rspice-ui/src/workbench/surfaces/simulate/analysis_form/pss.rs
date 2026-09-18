//! The PSS form: every control the engine's `.PSS` card carries, and what
//! autonomous mode does to three of them.
//!
//! Split out for the same reason the DC sweep form is: one control changes what
//! every other one means. Autonomous mode moves the period from the Fundamental
//! field to the oscillator node, stops the tone list from being read at all,
//! and is the only mode in which a bound on the period correction means
//! anything, so the same fields mean two different things.
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
use crate::simulation::dialog::{IntegrationMethod, PssDialogState};

use super::{
    QuantityPresentationPolicy, UiNumberLocale, choice_row, enabled_choice_row,
    engineering_input_row, hinted_input_row, hinted_input_row_enabled, input_row,
    input_row_enabled, named_periodic_source_row, quantity_input_row,
};

pub(super) const PSS_FIELD_LABELS: [&str; 14] = [
    "Integration method",
    "Fundamental",
    "Tones",
    "Stabilization cycles",
    "Shooting points",
    "Period tolerance",
    "Autonomous oscillator",
    "Oscillator node",
    "Save harmonics",
    "Stabilization time",
    "Max iterations",
    "Absolute tolerance",
    "Damping",
    "Max period change",
];

/// The integration methods the chooser offers, in the engine's own order.
///
/// Assembled from [`IntegrationMethod::all`] rather than written out, so the
/// form cannot offer a method the deck has no spelling for — which is what a
/// second list of names beside the enum would eventually do. The first entry
/// is not a method: the card has no keyword for the engine's own choice, so
/// "engine default" is the absence of `METHOD=` and belongs in the chooser as
/// the position that writes nothing.
pub(super) fn pss_integration_method_choices() -> Vec<&'static str> {
    std::iter::once(PSS_ENGINE_DEFAULT_METHOD)
        .chain(
            IntegrationMethod::all()
                .iter()
                .map(IntegrationMethod::display_name),
        )
        .collect()
}

pub(super) const PSS_ENGINE_DEFAULT_METHOD: &str = "Engine default";

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
        &pss_integration_method_choices(),
        &mut setup.integration_method_idx,
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
    hinted_input_row(
        ui,
        PSS_FIELD_LABELS[9],
        &mut setup.tstab,
        "overrides the cycles",
    )
    .on_hover_text(STABILIZATION_TIME_HINT);
    input_row(ui, PSS_FIELD_LABELS[10], &mut setup.max_iterations)
        .on_hover_text(MAX_ITERATIONS_HINT);
    engineering_input_row(ui, PSS_FIELD_LABELS[11], &mut setup.abstol)
        .on_hover_text(ABSOLUTE_TOLERANCE_HINT);
    hinted_input_row(ui, PSS_FIELD_LABELS[12], &mut setup.damping, "0.1 to 1")
        .on_hover_text(DAMPING_HINT);
    // Autonomous only, and still a stable member of the grid: the period is a
    // solver unknown in exactly one mode, and that is the only mode in which a
    // bound on its correction means anything.
    hinted_input_row_enabled(
        ui,
        PSS_FIELD_LABELS[13],
        &mut setup.max_period_change,
        "autonomous only",
        setup.osc_mode,
    )
    .on_hover_text(MAX_PERIOD_CHANGE_HINT);
}

/// The rule the two stabilization fields share, stated where the overriding
/// one is.
///
/// `PssConfig::effective_tstab` takes `tstab` when it is positive and
/// `tstab_periods * period` otherwise, so the two are not additive and the
/// cycle count is not a floor. A blank field is the engine's zero.
const STABILIZATION_TIME_HINT: &str = "Seconds to settle before the shooting solve. \
                                       Blank takes the window from the stabilization cycles \
                                       instead; any positive time replaces them.";

const MAX_ITERATIONS_HINT: &str = "Shooting-Newton corrections allowed on each integration \
                                   grid. The solve refines the grid until the orbits agree, \
                                   and every grid must converge within this limit.";

const ABSOLUTE_TOLERANCE_HINT: &str = "A coordinate converges on this or on the period \
                                       tolerance, whichever it meets first, in its own unit \
                                       (V, A, or K). It is what lets a node resting at zero \
                                       converge at all, where a relative error is undefined.";

const DAMPING_HINT: &str = "Fraction of each Newton correction that is taken. Below one it \
                            converges more slowly and survives stiffer circuits; the engine \
                            admits 0.1 to 1.";

const MAX_PERIOD_CHANGE_HINT: &str = "Largest relative change one autonomous iteration may \
                                      make to the detected period, which is what stops the \
                                      period from oscillating instead of converging. A driven \
                                      solve has no period to correct.";

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
