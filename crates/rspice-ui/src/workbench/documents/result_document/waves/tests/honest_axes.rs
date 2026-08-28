//! What a strip says about itself, and whether it is entitled to say it.
//!
//! Each of these is the same failure wearing a different hat: a surface that
//! states something it did not read. An axis that calls every DC sweep volts,
//! a legend chip whose colour is a different arithmetic from the curve it
//! names, a row that does not say which run it came from, and a painted table
//! that says nothing at all to a reader who cannot see it.

use super::*;

use crate::state::{ExecutedDeck, ExecutedDeckPoint};

/// A DC sweep of a current source, with the deck the run actually executed.
fn swept_current_source() -> AppState {
    let mut state = AppState::default();
    let run = state.simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcSweep, "DC").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0e-3], vec![0.0, 5.0], "#fff"),
        ]),
    );
    let run_id = state.simulation.runs[0].id;
    state.simulation.executed_decks.retain(ExecutedDeck {
        run_id,
        points: vec![ExecutedDeckPoint {
            label: "DC".to_owned(),
            deck: "* bias sweep\nIbias 0 in DC 0\n.dc Ibias 0 1m 10u\n.end\n".into(),
            model_sources: Vec::new(),
        }],
    });
    state.ui.results.viewer = super::super::super::ResultViewer::DcSweep;
    state
}

fn model_axis(state: &mut AppState) -> (String, String) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    (models[0].x_label().to_owned(), models[0].x_unit.clone())
}

/// The retained result keeps the sweep's values and not the source they came
/// from, so the abscissa called every DC sweep volts — a swept current source
/// read as a voltage, which is not a unit error but a quantity error. The deck
/// the run executed names the source, and it is that run's own evidence.
#[test]
fn a_dc_sweep_names_the_source_the_run_actually_swept() {
    let mut state = swept_current_source();
    assert_eq!(
        model_axis(&mut state),
        ("Ibias".to_owned(), "A".to_owned()),
        "the abscissa did not read the swept source out of the run's deck"
    );
}

/// A run whose decks were not retained has nothing to read, and inventing a
/// source would be the failure this replaces. The analysis default stands.
#[test]
fn a_dc_sweep_without_a_retained_deck_keeps_the_analysis_default() {
    let mut state = swept_current_source();
    state.simulation.executed_decks = Default::default();
    state.ui.results.models = Default::default();
    state.simulation.data_version = state.simulation.data_version.wrapping_add(1);
    assert_eq!(model_axis(&mut state), ("x".to_owned(), "V".to_owned()));
}

/// The legend chip and the curve have to be the same colour, which means they
/// have to count palette slots the same way. The chip counted the active run's
/// traces and the canvas counted every trace it held, so a strip with an
/// overlay run drew an expression in one colour and named it in another.
#[test]
fn an_expression_chip_takes_the_palette_slot_its_curve_draws_in() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
        ]),
    );
    let earlier = state.simulation.runs[0].dataset_id;
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
        ]),
    );
    state.simulation.overlay_dataset_ids.push(earlier);

    let tokens = Tokens::default();
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &tokens,
    );
    let model = &models[0];
    assert!(
        model.traces.len() > model.signal_trace_count,
        "the fixture needs an overlay run for the two counts to differ"
    );
    let chip = expr_color(&tokens, expr_palette_slot(model, 0));
    assert_ne!(
        chip,
        expr_color(&tokens, model.signal_trace_count),
        "the fixture no longer separates the legend's count from the canvas'"
    );
    let analysis = model.analysis_key;
    drop(models);

    state
        .ui
        .results
        .add_expression_trace(&state.simulation, analysis, "V(out)*2".to_owned())
        .expect("the strip accepts an expression");
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &tokens,
    );
    let model = Arc::clone(&models);
    drop(models);
    let resolved = resolve_strip_exprs(&mut state, &model[0], &tokens);
    assert_eq!(
        resolved.len(),
        1,
        "the expression resolved to one drawn curve"
    );
    assert_eq!(
        resolved[0].color, chip,
        "the chip and the curve took different palette slots"
    );
}

/// Two runs of the same signal arrive as two rows spelled identically. Which
/// is the overlay is the whole reason both are on the sheet.
#[test]
fn an_overlay_row_names_the_run_it_came_from() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
        ]),
    );
    let earlier = state.simulation.runs[0].dataset_id;
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#fff"),
        ]),
    );
    state.simulation.overlay_dataset_ids.push(earlier);
    state.ui.results.cursor_strip = Some(0);

    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let rows = readout_rows(
        &models[0],
        CursorPair {
            a: Some(0.5),
            b: None,
        },
        presentation,
        quantity_policy,
    );

    assert_eq!(rows.len(), 2, "the active run and its overlay");
    assert_eq!(rows[0].name, "V(out)", "the active run owns the plain name");
    assert!(
        rows[1].name.contains("run "),
        "an overlay row does not say which run it is: {}",
        rows[1].name
    );
}

/// The cursor table is painted, not built from widgets, so nothing about it
/// reaches a screen reader on its own. The marker half of the same strip
/// already published its rows; this half stated the same numbers to a sighted
/// reader and nothing at all to anyone else.
#[test]
fn the_cursor_table_publishes_what_it_paints() {
    let mut state = super::branches::hysteresis_run();
    if !state.ui.results.cursor_tool.is_armed() {
        state.ui.results.toggle_cursor_tool();
    }
    state.ui.results.cursors.place(0.25);
    state.ui.results.cursors.place(0.75);
    state.ui.results.readout_collapsed = false;

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    let mut labels: Vec<String> = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(input.clone(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let height = readout_strip_height(&mut state);
                readout_strip(ui, &mut state, height);
            });
        });
        labels = output
            .platform_output
            .accesskit_update
            .expect("the workbench publishes an accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Table)
            .filter_map(|(_, node)| node.label())
            .map(str::to_owned)
            .collect();
    }

    let spoken = labels
        .iter()
        .find(|label| label.starts_with("Cursor readout"))
        .unwrap_or_else(|| panic!("the cursor table published no table node: {labels:?}"));
    assert!(
        spoken.contains("fwd") && spoken.contains("rev"),
        "the spoken table lost the branches the painted one shows: {spoken}"
    );
    assert!(
        spoken.contains("2 branches"),
        "the spoken table did not state the shape of the sweep: {spoken}"
    );
    assert!(
        spoken.contains("slope"),
        "the spoken table named no columns: {spoken}"
    );
}
