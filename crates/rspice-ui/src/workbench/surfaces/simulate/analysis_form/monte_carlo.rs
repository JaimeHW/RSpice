//! The Monte Carlo form: how many trials, drawn from where, and with what
//! spread.
//!
//! Where the spread comes from decides whether two of these rows are read at
//! all. Under deck statistics the deck states its own spread and its own
//! distribution, so both rows go quiet and a note says how far the deck's
//! statistics reach: a reader who expects only `.param` tolerances to move is
//! owed the model cards.

use egui::Ui;

use crate::simulation::dialog::McDialogState;

use super::{
    choice_row, choice_row_with_disabled, field_note, hinted_input_row_enabled, input_row,
    input_row_enabled,
};

/// Render the Monte Carlo fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut McDialogState) {
    use crate::simulation::dialog::McVariationSource;

    input_row(ui, "Samples", &mut setup.num_runs);
    input_row(ui, "Seed", &mut setup.seed)
        .on_hover_text("An integer from 0 to 18446744073709551615. Leave blank to use the repeatable default seed.");
    choice_row(
        ui,
        "From",
        &["parameters", "deck"],
        &mut setup.variation_source_idx,
    );
    // The spread and its shape belong to the parameter-tolerance
    // source. Under deck statistics the deck states its own spread, so
    // these two rows would be read by nothing.
    let states_spread = McVariationSource::ALL
        .get(setup.variation_source_idx)
        .copied()
        .unwrap_or_default()
        .uses_stated_spread();
    input_row_enabled(ui, "Spread %", &mut setup.variation_pct, states_spread);
    choice_row_with_disabled(
        ui,
        "Vary",
        &["gauss", "uniform", "worst"],
        &mut setup.distribution_idx,
        &if states_spread {
            Vec::new()
        } else {
            (0..3)
                .map(|index| (index, "the deck states its own distribution"))
                .collect::<Vec<_>>()
        },
    );
    // The subset narrows only the stated-spread source. A deck that states its
    // own statistics names what it varies itself, and the engine refuses a
    // generic filter beside them, so the field goes quiet with the two above.
    hinted_input_row_enabled(
        ui,
        "Vary only",
        &mut setup.vary_only,
        "empty = every parameter",
        states_spread,
    );
    // Which spread is drawn from is the `From` choice's own answer, and
    // the two rows above say so by going quiet. What they cannot say is
    // how far the deck's statistics reach, and a reader who expects
    // only `.param` tolerances to move is owed the model cards.
    if !states_spread {
        field_note(
            ui,
            "Each trial redraws the deck's own agauss/gauss/unif expressions, model \
             cards included.",
        );
    }
}
