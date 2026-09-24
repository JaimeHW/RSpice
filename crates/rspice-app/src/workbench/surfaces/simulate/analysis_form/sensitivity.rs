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
    QuantityPresentationPolicy, SWEEP_KINDS, UiNumberLocale, choice_row, choice_row_enabled,
    clear_pending_cell, full_width_field, hinted_quantity_input_row_enabled, input_row,
    input_row_enabled, mono_input, quantity_input_row_enabled, sweep_point_field_label,
};

/// The notations the filter accepts, one of each kind it can address.
const FILTER_HINT: &str = "R1 · M1:W · MOD:VTO · PARAM:name";

/// What the filter selects, stated where it is typed.
const FILTER_HELP: &str = "Globs select what the output is differentiated against. \
                           Empty: every device and model parameter. \
                           PARAM:* selects the design parameters.";

/// What an unfilled Stop field means, said where it is typed.
const STOP_HINT: &str = "empty = one frequency";

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
    // The band, greyed rather than hidden outside AC: a row that disappears
    // takes the reason with it. An empty Stop is one frequency — the card
    // this form has always written — so the two rows that divide a band are
    // greyed until there is a band to divide.
    let ac = setup.sens_type_idx == 1;
    quantity_input_row_enabled(
        ui,
        "Start",
        &mut setup.ac_freq,
        QuantityInputKind::Frequency,
        policy,
        locale,
        ac,
    );
    hinted_quantity_input_row_enabled(
        ui,
        "Stop",
        &mut setup.ac_stop,
        STOP_HINT,
        QuantityInputKind::Frequency,
        policy,
        locale,
        ac,
    );
    let swept = ac && !setup.ac_stop.trim().is_empty();
    input_row_enabled(
        ui,
        sweep_point_field_label(setup.ac_sweep_idx),
        &mut setup.ac_points,
        swept,
    );
    choice_row_enabled(ui, "Sweep", SWEEP_KINDS, &mut setup.ac_sweep_idx, swept);
    clear_pending_cell(ui);
}
