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
    input_row(ui, "Histogram bins", &mut setup.histogram_bins);
    if configured {
        field_note(
            ui,
            "Each trial runs this analysis at the study's Run Set point. Separate measurements with semicolons or newlines; semicolons within trace parentheses or brackets belong to the trace. A plain name reads a .MEAS result; scalar:V(out) reads an operating-point value; last:signal explicitly reads the final waveform sample. For AC, last:signal reads the real component; use .MEAS VM/VP/VDB for magnitude, phase, or decibels. For HB/Fourier/FFT, use bin:1:magnitude (or real, imag, phase in degrees); append :signal when HB/Fourier has multiple outputs. Fourier scalars include scalar:THD(%) and scalar:DC. Fourier/FFT rerun their bound transient producer for each trial; HB reruns its harmonic balance solve and reports peak-amplitude phasors. HBSP/HBNOISE rerun their bound HB producer on each varied circuit. Use bin:0:real:S11 or bin:0:real:PN_NF for HBSP; bin:0:real:output_noise, bin:0:real:noise_figure_db and scalar:noise.output_rms for HBNOISE. Noise figure and integrated noise must be enabled to measure those results. PSS first reruns its bound OP configuration, then shooting; use bin:1:magnitude:V(out), bin:1:imag:I(V1), last:V(out), or scalar:pss.frequency/period/iterations. PSS harmonics use the retained time grid and configured harmonic count. PAC/PXF/PNOISE/PSTB/PSP also rerun their bound periodic producer with the full selected configuration. Use bin:0:magnitude:V(out)[sb=+0] for PAC, bin:0:real:H(sb1->sb1, V(out)) for PXF, scalar:pstb.max_multiplier_magnitude for PSTB, and the noise or scattering selectors above for PNOISE/PSP. QPSS/QPAC/QPXF/QPNOISE use their full selected configurations and rerun the bound QPSS producer. For QPSS use tuple:1,-1:magnitude:V(out); for dependent traces use bin:index:quantity:trace with the complete lattice label. QPNOISE supports scalar:qpnoise.output_rms(1), scalar:qpnoise.input_rms(1), and scalar:qpnoise.contributor_rms(1,RS thermal), with output numbers starting at 1.",
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
    super::sub_header(ui, "Retained trials");
    super::switch_row(
        ui,
        "Retain trials for resume",
        &mut setup.checkpoint.retain_trials,
    );
    input_row_enabled(ui, "Checkpoint every", &mut setup.checkpoint.publish_every, setup.checkpoint.retain_trials)
        .on_hover_text("Publish a checkpoint after this many newly completed trials. Normal completion also saves the final remainder. A forced browser stop can recover only trials already published.");
    if setup.checkpoint.retain_trials {
        field_note(
            ui,
            "Completed trials are kept with the run and saved with the project. Checkpoints count toward the run's storage limit.",
        );
    }

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

/// Offer retained evidence without decoding or cloning large trial buffers on
/// every frame. The controller resolves and validates selected bytes at prepare.
pub(in crate::workbench::surfaces::simulate) fn checkpoint_sources(
    ui: &mut Ui,
    setup: &mut McDialogState,
    simulation: &crate::state::SimulationState,
    selected: crate::product::AnalysisInstanceId,
) -> Option<crate::workbench::surfaces::simulate::checkpoint_exchange::Action> {
    use crate::workbench::surfaces::simulate::checkpoint_exchange;
    let mut action = None;
    super::sub_header(ui, "Resume previous trials");
    if super::action_line(ui, "Import checkpoint…") {
        action = Some(checkpoint_exchange::Action::Import);
    }
    let mut seen = std::collections::HashSet::new();
    let candidates = simulation
        .runs
        .iter()
        .rev()
        .flat_map(|run| {
            run.analyses.iter().filter_map(move |analysis| {
                if analysis.provenance()?.authored_source_instance_id() != selected {
                    return None;
                }
                let checkpoint = analysis.monte_carlo_checkpoint.as_ref()?;
                Some((
                    format!(
                        "{} · {} · {} trials",
                        run.label,
                        analysis.label,
                        checkpoint.completed_trials()
                    ),
                    checkpoint.clone(),
                ))
            })
        })
        .chain(
            simulation
                .imported_monte_carlo_checkpoints
                .iter()
                .map(|entry| {
                    (
                        format!(
                            "Imported · {} · {} trials",
                            entry.name(),
                            entry.checkpoint().completed_trials()
                        ),
                        entry.checkpoint().clone(),
                    )
                }),
        )
        .filter(|(_, checkpoint)| seen.insert(checkpoint.digest()))
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        field_note(
            ui,
            "No checkpoints available. Run with trial retention enabled or import a checkpoint file.",
        );
    } else {
        field_note(
            ui,
            "Each Run Set point reuses its matching selected trials; other points run fresh. Matching trials are counted once and missing indices are simulated. Preparation checks the circuit, sampler and analysis settings. Imported checkpoints are saved with the project.",
        );
        if !setup.checkpoint.retain_trials {
            field_note(
                ui,
                "Enable trial retention above to resume from selected checkpoints.",
            );
        }
    }
    for (label, checkpoint) in candidates {
        let digest = checkpoint.digest();
        let mut chosen = setup.checkpoint.resume.contains(&digest);
        ui.push_id(digest, |ui| {
            ui.add_enabled_ui(setup.checkpoint.retain_trials, |ui| {
                if super::switch_row(ui, &label, &mut chosen) {
                    if chosen {
                        setup.checkpoint.resume.push(digest);
                    } else {
                        setup.checkpoint.resume.retain(|id| *id != digest);
                    }
                }
            });
            if super::action_line(ui, "Inspect checkpoint") {
                action = Some(checkpoint_exchange::Action::Inspect(
                    label,
                    checkpoint.clone(),
                ));
            }
            if super::action_line(ui, "Export checkpoint…") {
                action = Some(checkpoint_exchange::Action::Export(checkpoint.clone()));
            }
            if simulation
                .imported_monte_carlo_checkpoints
                .get(digest)
                .is_some()
                && super::action_line(ui, "Remove imported copy")
            {
                action = Some(checkpoint_exchange::Action::Remove(digest));
                setup.checkpoint.resume.retain(|id| *id != digest);
            }
        });
    }
    if setup
        .checkpoint
        .resume
        .iter()
        .any(|digest| !seen.contains(digest))
    {
        field_note(
            ui,
            "A selected checkpoint is no longer available. Restore its run, import its checkpoint file, or clear the selection.",
        );
    }
    if !setup.checkpoint.resume.is_empty()
        && super::action_line(ui, "Clear selection — simulate all requested trials")
    {
        setup.checkpoint.resume.clear();
    }
    super::clear_pending_cell(ui);
    action
}
