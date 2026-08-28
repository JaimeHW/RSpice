//! Sweeps that do not simply increase, and what the instrument owes a reader
//! looking at one.
//!
//! A hysteresis measurement is a measurement because the sweep goes out and
//! comes back as one continued solve: at almost every abscissa the curve has
//! two values, and reporting one of them as "the" reading is not a rounding
//! error, it is the wrong branch. A reverse sweep has one value everywhere and
//! still broke every reduction written around `partition_point`.

use super::*;

/// A DC sweep out to 1 V and back, with a different ordinate on the way home.
///
/// Forward: y = 2x over x ∈ [0, 1]. Return: y = 4 − 2x over the same span. The
/// two branches share the turnaround, and every closed form below is read off
/// those two lines.
pub(super) fn hysteresis_run() -> AppState {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::DcSweep, "DC").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 0.5, 1.0, 0.5, 0.0],
                vec![0.0, 1.0, 2.0, 3.0, 4.0],
                "#fff",
            ),
        ]),
    );
    state.ui.results.viewer = super::super::super::ResultViewer::DcSweep;
    state.ui.results.cursor_strip = Some(0);
    state
}

fn rows_of(state: &mut AppState, a: Option<f64>, b: Option<f64>) -> Vec<ReadoutRow> {
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    readout_rows(
        &models[0],
        CursorPair { a, b },
        presentation,
        quantity_policy,
    )
}

fn is_negative(text: &str) -> bool {
    text.starts_with('-') || text.starts_with('\u{2212}')
}

/// The closed form of a two-branch loop, read at a quarter and three quarters
/// of the way out.
///
/// Forward reads 0.5 then 1.5, rising at 2 V per volt of sweep. The return leg
/// reads 3.5 then 2.5 over the same interval, falling at the same rate. One
/// composite number cannot say that, and one of the two numbers alone is a
/// reading of a curve the reader did not ask about.
#[test]
fn a_hysteresis_sweep_reports_one_row_per_branch() {
    let mut state = hysteresis_run();
    let rows = rows_of(&mut state, Some(0.25), Some(0.75));

    assert_eq!(rows.len(), 2, "one row per branch: {:?}", names(&rows));
    assert!(
        rows[0].name.ends_with("fwd") && rows[1].name.ends_with("rev"),
        "branches are named by the direction they travel: {:?}",
        names(&rows)
    );

    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let model = &models[0];
    let trace = &model.traces[0];
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let shown = |value: f64| model.format_trace_value(trace, value, digits, quantity_policy);

    assert_eq!(rows[0].a, shown(0.5), "forward branch at a quarter out");
    assert_eq!(rows[0].b, shown(1.5), "forward branch at three quarters");
    assert_eq!(rows[0].delta, shown(1.0), "forward branch rises by 1 V");
    assert_eq!(rows[1].a, shown(3.5), "return branch at a quarter out");
    assert_eq!(rows[1].b, shown(2.5), "return branch at three quarters");
    assert_eq!(rows[1].delta, shown(-1.0), "return branch falls by 1 V");

    assert!(
        !is_negative(&rows[0].slope) && is_negative(&rows[1].slope),
        "the two branches travel in opposite directions: {} / {}",
        rows[0].slope,
        rows[1].slope
    );
    assert_eq!(
        readout_branch_note(model).as_deref(),
        Some("2 branches"),
        "the X row states the shape it is reporting"
    );
}

/// A branch that has not reached the cursor has no value there, and inventing
/// one — by clamping to its endpoint, which is what the unshaped kernel does —
/// puts a number in the table that the curve never takes.
#[test]
fn a_branch_that_does_not_reach_the_cursor_reports_nothing() {
    let mut state = hysteresis_run();
    let rows = rows_of(&mut state, Some(2.0), None);

    assert_eq!(rows.len(), 2);
    for row in &rows {
        assert_eq!(
            row.a, READOUT_ABSENT,
            "{} answered past the end of its branch",
            row.name
        );
        assert_eq!(row.delta, READOUT_ABSENT);
        assert_eq!(row.slope, READOUT_ABSENT);
    }
}

/// A reverse sweep has one value everywhere, so it keeps the single unlabelled
/// row it always had — and that value is now read the right way round.
#[test]
fn a_descending_sweep_keeps_one_untagged_row() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::DcSweep, "DC").with_waveforms(vec![
            WaveformData::new("V(out)", vec![2.0, 1.0, 0.0], vec![20.0, 10.0, 0.0], "#fff"),
        ]),
    );
    state.ui.results.viewer = super::super::super::ResultViewer::DcSweep;
    state.ui.results.cursor_strip = Some(0);

    let rows = rows_of(&mut state, Some(0.5), Some(1.5));
    assert_eq!(
        rows.len(),
        1,
        "a monotone sweep is one row: {:?}",
        names(&rows)
    );
    assert_eq!(rows[0].name, "V(out)", "no branch tag on a single branch");

    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let model = &models[0];
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let shown =
        |value: f64| model.format_trace_value(&model.traces[0], value, digits, quantity_policy);
    assert_eq!(rows[0].a, shown(5.0), "y = 10x, read at x = 0.5");
    assert_eq!(rows[0].b, shown(15.0), "y = 10x, read at x = 1.5");
}

/// Past the branch budget the rows would stop being a measurement and become
/// a listing. The readout says so, and reports the nearest retained sample
/// rather than interpolating a curve with no single value here.
#[test]
fn a_sweep_past_the_branch_budget_reports_the_nearest_retained_sample() {
    // Seven legs of a triangle wave: more branches than the readout will name.
    let legs = MAX_READOUT_BRANCHES + 1;
    let mut x = Vec::new();
    let mut y = Vec::new();
    for leg in 0..legs {
        let rising = leg % 2 == 0;
        for step in 0..=4 {
            let position = f64::from(step) / 4.0;
            x.push(if rising { position } else { 1.0 - position });
            y.push(leg as f64 + position);
        }
    }
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::DcSweep, "DC")
            .with_waveforms(vec![WaveformData::new("V(out)", x, y, "#fff")]),
    );
    state.ui.results.viewer = super::super::super::ResultViewer::DcSweep;
    state.ui.results.cursor_strip = Some(0);

    let rows = rows_of(&mut state, Some(0.5), Some(0.75));
    assert_eq!(
        rows.len(),
        1,
        "one row, not one per leg: {:?}",
        names(&rows)
    );
    assert!(
        rows[0].a.starts_with('\u{2248}'),
        "an approximate reading says so: {}",
        rows[0].a
    );
    assert_eq!(rows[0].delta, READOUT_ABSENT);
    assert_eq!(rows[0].slope, READOUT_ABSENT);

    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    assert_eq!(
        readout_branch_note(&models[0]).as_deref(),
        Some("multi-branch sweep")
    );
}

/// The band the dock reserves and the table that fills it have to be counting
/// the same thing. Sized from the retained waveform list, a loop's second row
/// had nowhere to go.
#[test]
fn the_readout_band_is_sized_by_the_rows_it_will_draw() {
    let mut state = hysteresis_run();
    if !state.ui.results.cursor_tool.is_armed() {
        state.ui.results.toggle_cursor_tool();
    }
    state.ui.results.cursors.place(0.25);
    state.ui.results.readout_collapsed = false;

    let retained_waveforms = state.simulation.active_run().unwrap().analyses[0]
        .waveforms
        .len();
    assert_eq!(retained_waveforms, 1, "one retained waveform, two branches");
    assert_eq!(readout_row_count(&mut state), 2);
    assert_eq!(
        readout_strip_height(&mut state),
        READOUT_HEADER_H + (2 + 2) as f32 * READOUT_ROW_H,
        "the reserved band does not match the rows the table will draw"
    );
}

/// A viewer whose projection has no strip for the cursor has no rows to draw,
/// so the band it reserved has to collapse to the header the placed cursor
/// still earns. Sized from the retained analysis instead of the projection, it
/// stayed open over a table that drew nothing.
#[test]
fn a_viewer_without_the_cursors_strip_reserves_no_band() {
    let mut state = hysteresis_run();
    if !state.ui.results.cursor_tool.is_armed() {
        state.ui.results.toggle_cursor_tool();
    }
    state.ui.results.cursors.place(0.25);
    state.ui.results.readout_collapsed = false;
    assert!(readout_strip_height(&mut state) > READOUT_HEADER_H);

    // The waveform sheet projects time-domain analyses only, so this DC sweep
    // is not on it — while the retained analysis is still there to be found.
    state.ui.results.viewer = super::super::super::ResultViewer::Waves;
    assert_eq!(
        readout_strip_height(&mut state),
        READOUT_HEADER_H,
        "the strip reserved rows for a sheet that draws nothing"
    );
}

/// Window statistics are taken over the samples the cursors enclose. On a
/// loop those are two disjoint runs of the source array, and a single bisected
/// slice spans the turnaround — reporting an extremum from samples on the far
/// side of it.
#[test]
fn window_statistics_fold_only_the_samples_between_the_cursors() {
    let mut state = hysteresis_run();
    state.ui.results.cursors.a = Some(0.0);
    state.ui.results.cursors.b = Some(0.5);

    let painted = super::interaction::painted_texts(&mut state, right_panel);
    // x ≤ 0.5 selects indices {0, 1} on the way out and {3, 4} on the way
    // back: y ∈ {0, 1, 3, 4}. A contiguous slice stops at the turnaround and
    // never sees the 4 V the return leg ends on.
    assert!(
        painted.iter().any(|text| text == "4.000000 V"),
        "the window missed the branch the cursors also enclose: {painted:?}"
    );
    assert!(
        painted.iter().any(|text| text == "0.000000 V"),
        "the window lost its minimum: {painted:?}"
    );
    // rms over {0, 1, 3, 4} is √6.5; over the bisected slice {0, 1} it is
    // √0.5, so this pins the fold as well as its extremes.
    assert!(
        painted.iter().any(|text| text == "2.549510 V"),
        "the rms was folded over the wrong samples: {painted:?}"
    );
}

/// The payoff for the whole shape program: a reverse sweep that is zoomed
/// into stays on screen. Reduced under the assumption that X increases, every
/// window asked for came back empty and the curve simply vanished.
#[test]
fn a_zoomed_reverse_sweep_still_draws_its_curve() {
    let points = 100_000;
    let x: Vec<f64> = (0..points)
        .map(|index| 1.0 - f64::from(index) / f64::from(points))
        .collect();
    let y: Vec<f64> = x.iter().map(|value| value * 2.0).collect();
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::DcSweep, "DC")
            .with_waveforms(vec![WaveformData::new("V(out)", x, y, "#fff")]),
    );
    state.ui.results.viewer = super::super::super::ResultViewer::DcSweep;
    let analysis = {
        let presentation = state.ui.preferences.result_presentation_policy();
        let models = cached_models(
            &state.simulation,
            &mut state.ui.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        );
        models[0].analysis_key
    };
    set_shared_x_view(
        &mut state.ui.results,
        analysis,
        2,
        Some((0.400_000, 0.600_000)),
    );

    let drawn = super::interaction::painted_path_points(&mut state, show);
    assert!(
        drawn > 32,
        "a zoomed reverse sweep reduced to {drawn} stroked points"
    );
}

fn names(rows: &[ReadoutRow]) -> Vec<&str> {
    rows.iter().map(|row| row.name.as_str()).collect()
}
