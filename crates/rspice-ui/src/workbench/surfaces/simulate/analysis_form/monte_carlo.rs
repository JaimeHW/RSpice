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
    super::study_base_row(
        ui,
        &mut setup.base_analysis,
        bases,
        "Operating point: all node voltages",
    );
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
    input_row(ui, "First trial index", &mut setup.first_trial);
    input_row(ui, "Samples", &mut setup.num_runs);
    field_note(
        ui,
        "Trial indices start at 0. Keep the seed, circuit, and settings unchanged to replay a range or continue with the next index. Only this batch contributes to its statistics; earlier circuits are not solved again.",
    );
    input_row(ui, "Seed", &mut setup.seed)
        .on_hover_text("An integer from 0 to 18446744073709551615. Leave blank to use the repeatable default seed.");
    choice_row(
        ui,
        "From",
        &["parameters", "deck", "custom"],
        &mut setup.variation_source_idx,
    );
    // The spread and its shape belong to the parameter-tolerance
    // source. Under deck statistics the deck states its own spread, so
    // these two rows would be read by nothing.
    let states_spread = setup.variation_source_idx == 0;
    if setup.variation_source_idx == 2 {
        custom_statistics(ui, setup);
    }
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

fn custom_statistics(ui: &mut Ui, setup: &mut McDialogState) {
    super::sub_header(ui, "Parameter distributions");
    field_note(
        ui,
        "Nominal values come from circuit parameters. Process variation shares one draw across instances; mismatch draws independently per instance. Gaussian spread is standard deviation; uniform spread is half-width. Lognormal uses the nominal as median and spread as the standard deviation of ln(parameter). Existing deck statistics remain active.",
    );
    let mut remove = None;
    for (index, row) in setup.variations.iter_mut().enumerate() {
        ui.push_id(("mc-variation", index), |ui| {
            input_row(ui, "Parameter", &mut row.parameter);
            choice_row(ui, "Scope", &["process", "mismatch"], &mut row.scope);
            choice_row(ui, "Distribution", &["Gaussian", "uniform", "lognormal"], &mut row.distribution);
            input_row(ui, "Spread", &mut row.spread);
            super::switch_row(ui, "Scale spread by nominal (%)", &mut row.percent);
            if row.distribution == 2 && row.percent {
                field_note(ui, "This scales the log-space standard deviation by |nominal|/100. Disable it to enter the log-space standard deviation directly.");
            }
            hinted_input_row_enabled(ui, "Lower bound", &mut row.lower, "unbounded", true);
            hinted_input_row_enabled(ui, "Upper bound", &mut row.upper, "unbounded", true);
            hinted_input_row_enabled(ui, "Truncate at sigma", &mut row.sigma_cutoff, "unbounded", row.distribution != 1);
            let bounded = !row.lower.trim().is_empty() || !row.upper.trim().is_empty()
                || (row.distribution != 1 && !row.sigma_cutoff.trim().is_empty());
            input_row_enabled(ui, "Maximum sampling attempts", &mut row.max_attempts, bounded);
            if super::action_line(ui, "Remove variation") { remove = Some(index); }
        });
    }
    if let Some(index) = remove {
        setup.variations.remove(index);
    }
    if super::action_line(ui, "+ Add parameter variation") {
        setup.variations.push(Default::default());
    }
    field_note(
        ui,
        "Bounds use absolute parameter values and apply after each scope's draw. Sigma truncation is symmetric around the nominal (in log space for lognormal). Out-of-range draws are rejected, with an explicit error if the attempt limit is reached. Correlated groups are redrawn together and use the smallest active attempt limit. Correlations below describe the population before truncation; bounds can change its mean, spread, and correlation.",
    );
    super::sub_header(ui, "Parameter correlations");
    field_note(
        ui,
        "Enter two or more varied parameters from the same scope and their Pearson correlation coefficient (-1 to 1). For a group, that coefficient applies to every pair. Use separate pairs for a general correlation matrix. The matrix must be valid and the chosen distributions must be able to attain the requested correlations.",
    );
    let mut remove = None;
    for (index, row) in setup.correlations.iter_mut().enumerate() {
        ui.push_id(("mc-correlation", index), |ui| {
            choice_row(ui, "Scope", &["process", "mismatch"], &mut row.scope);
            input_row(ui, "Parameters", &mut row.parameters);
            input_row(ui, "Correlation", &mut row.coefficient);
            if super::action_line(ui, "Remove correlation") {
                remove = Some(index);
            }
        });
    }
    if let Some(index) = remove {
        setup.correlations.remove(index);
    }
    if super::action_line(ui, "+ Add correlation") {
        setup.correlations.push(Default::default());
    }
}
