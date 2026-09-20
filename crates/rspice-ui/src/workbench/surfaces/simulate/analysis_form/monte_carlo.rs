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
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut McDialogState,
    bases: &[(crate::product::AnalysisInstanceId, String)],
) {
    use crate::simulation::dialog::McVariationSource;

    let mut choices = vec![(None, "Operating point: all node voltages".to_owned())];
    choices.extend(bases.iter().map(|(id, label)| (Some(*id), label.clone())));
    if let Some(id) = setup.base_analysis {
        if !choices.iter().any(|(candidate, _)| *candidate == Some(id)) {
            choices.push((Some(id), format!("Unavailable analysis ({id})")));
        }
    }
    let mut selected = choices
        .iter()
        .position(|(id, _)| *id == setup.base_analysis)
        .unwrap_or(0);
    let labels = choices
        .iter()
        .map(|(_, label)| label.as_str())
        .collect::<Vec<_>>();
    if choice_row(ui, "Base analysis", &labels, &mut selected) {
        setup.base_analysis = choices[selected].0;
    }
    let configured = setup.base_analysis.is_some();
    hinted_input_row_enabled(
        ui,
        "Measurements",
        &mut setup.measurements,
        "gain; settling; last:V(out)",
        configured,
    );
    input_row_enabled(ui, "Histogram bins", &mut setup.histogram_bins, configured);
    if configured {
        field_note(
            ui,
            "Each trial runs this analysis at the study's Run Set point. Separate measurements with semicolons. A plain name reads a .MEAS result; scalar:V(out) reads an operating-point value; last:signal explicitly reads the final waveform sample. For AC, last:signal reads the real component; use .MEAS VM/VP/VDB for magnitude, phase, or decibels.",
        );
    }
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
    confidence_fields(ui, setup);
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

fn confidence_fields(ui: &mut Ui, setup: &mut McDialogState) {
    input_row(ui, "Confidence %", &mut setup.confidence_pct)
        .on_hover_text("Two-sided confidence interval for each population mean; strictly between 0 and 100%. This is separate from yield confidence and population percentiles.");
    choice_row(
        ui,
        "Mean interval",
        &["Student t", "bootstrap"],
        &mut setup.confidence_method_idx,
    );
    let bootstrap = setup.confidence_method_idx == 1;
    input_row_enabled(ui, "Resamples", &mut setup.bootstrap_resamples, bootstrap)
        .on_hover_text("Number of percentile-bootstrap resamples, at least two. More resamples improve percentile resolution and take longer.");
    input_row_enabled(ui, "Bootstrap seed", &mut setup.bootstrap_seed, bootstrap)
        .on_hover_text("An integer from 0 to 18446744073709551615 for the resampling stream. Changing it leaves the circuit trials unchanged.");
    field_note(
        ui,
        "Mean intervals assume independent trials. Student t is exact for normal observations; bootstrap accuracy depends on the sample size and resampling count.",
    );
}
