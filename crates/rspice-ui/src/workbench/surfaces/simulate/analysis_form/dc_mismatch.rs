//! The DC mismatch form: the expression the spread is measured on, how wide a
//! spread is reported, and which variation sources are included in it.

use egui::Ui;

use crate::simulation::plan::DcMismatchDraft;

use super::{field_note, hinted_input_row, switch_row};

/// Render the DC mismatch fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut DcMismatchDraft) {
    // The probe grammar is the engine card's, and it accepts a differential
    // pair and a branch current as well as a node, so the row says which
    // spellings it takes rather than leaving the reader to guess from one
    // example.
    hinted_input_row(
        ui,
        "Output expression",
        &mut setup.output_expression,
        "V(n[,ref]) or I(elem)",
    );
    hinted_input_row(
        ui,
        "Sigma multiplier",
        &mut setup.sigma_multiplier,
        "quoted multiple",
    );
    // Zero is the card's own spelling of "every contributor", so it is a
    // value with a meaning rather than an empty field, and the slot says so.
    hinted_input_row(
        ui,
        "Contributor limit",
        &mut setup.contributor_limit,
        "0 keeps all",
    );
    // The second trimming control, and the one with no default of the form's
    // own: left empty, every contributor the limit above allows is listed,
    // because the card's own threshold is zero. The row says so where it is
    // typed, since a blank field otherwise reads as unfilled.
    hinted_input_row(
        ui,
        "Share threshold",
        &mut setup.share_threshold,
        "empty keeps all",
    );
    switch_row(ui, "Process variation", &mut setup.include_process);
    switch_row(ui, "Local mismatch", &mut setup.include_mismatch);
    switch_row(
        ui,
        "Normalize contributions",
        &mut setup.normalized_contributions,
    );
    // Where the spread comes from, said where the analysis is authored. This
    // form declares no distribution of its own: every standard deviation is
    // the design's, and the engine resolves each spread in the global
    // parameter context — a spread written against instance parameters does
    // not resolve, which is a limitation worth meeting here rather than in a
    // refused run.
    field_note(
        ui,
        "Reads the design's Spectre statistics block. Spreads resolve against global parameters.",
    );
}
