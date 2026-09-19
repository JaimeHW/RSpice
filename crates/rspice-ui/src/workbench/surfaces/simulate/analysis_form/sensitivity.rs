//! The sensitivity form: the output the derivatives are taken of, the
//! variables they are taken against, and the point they are taken at.
//!
//! The filter is the engine's own selection, written into the card verbatim,
//! so this form offers no second vocabulary for it and no checkbox that would
//! mean something the `.SENS` grammar cannot say. An empty filter is the
//! engine's default and the card it writes is `.sens V(out)` — the statement
//! `rspice run` reads.
//!
//! The frequency row belongs to the AC mode alone, so it is greyed rather
//! than hidden: a row that disappears takes the reason with it.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::SensDialogState;
use crate::ui::tokens::Tokens;

use super::{
    QuantityPresentationPolicy, UiNumberLocale, choice_row, clear_pending_cell, full_width_field,
    input_row, mono_input, quantity_input_row_enabled,
};

/// The notations the filter accepts, one of each kind it can address.
const FILTER_HINT: &str = "R1 · M1:W · MOD:VTO · PARAM:name";

/// What the filter selects, stated where it is typed.
const FILTER_HELP: &str = "Globs select what the output is differentiated against. \
                           Empty: every device and model parameter. \
                           PARAM:* selects the design parameters.";

/// Render the sensitivity fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut SensDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    input_row(ui, "Output", &mut setup.output_expr);
    choice_row(ui, "Mode", &["DC", "AC"], &mut setup.sens_type_idx);
    // A filter list is a line of globs rather than a number, so it takes the
    // whole width. The half-cell beside the mode selector is released first,
    // or the rows below would start out of step with it.
    clear_pending_cell(ui);
    full_width_field(
        ui,
        "Filter",
        Some(FILTER_HINT),
        Tokens::get(ui.ctx()).metrics.ctl_h,
        |ui| {
            mono_input(ui, "Filter", &mut setup.filter, ui.available_width())
                .on_hover_text(FILTER_HELP)
        },
    );
    quantity_input_row_enabled(
        ui,
        "Frequency",
        &mut setup.ac_freq,
        QuantityInputKind::Frequency,
        policy,
        locale,
        setup.sens_type_idx == 1,
    );
    clear_pending_cell(ui);
}
