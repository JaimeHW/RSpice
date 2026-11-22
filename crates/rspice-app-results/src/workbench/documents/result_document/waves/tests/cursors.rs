//! Cursor jumps, stepping, point markers read against a reference, and
//! horizontal cursors, each through the whole Results frame the reader sees:
//! the document bar, the sheet and the readout dock. The keys that raise the
//! steps are the shortcut resolver's, tested with the application's.

use super::lanes::{FRAME, cursor_lines, place_cursors, strip, theme_context, verticals};
use super::*;
use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
use crate::workbench::results::commands::ResultsCommand;
use egui::epaint::ClippedShape;
use rspice_app_runs::simulation::controller::test_execution::run_manual_batch;
use rspice_results::calculator::functions::FunctionRegistry;
use rspice_results::calculator::value::RealValue;
use rspice_results::feature_search::Direction;
use rspice_results::result_presentation::{MarkerKind, WavePanePresentationKey};
use rspice_results_ui::chrome::instrument::ResultPlotTool;
use rspice_results_ui::waves::cursor_values::marker_position;
use rspice_results_ui::waves::navigation::CursorJump;
use std::f64::consts::PI;

/// `exp(-t/TAU) sin(2 pi F t)` over five periods.
const F: f64 = 1.0e3;
const TAU: f64 = 2.0e-3;
const SAMPLES: usize = 4001;
const STOP: f64 = 5.0e-3;
const DT: f64 = STOP / (SAMPLES - 1) as f64;

fn damped(t: f64) -> f64 {
    (-t / TAU).exp() * (2.0 * PI * F * t).sin()
}

/// The kth extremum of the damped sine, `tan(wt) = w tau`: even k peaks.
fn extremum(k: usize) -> f64 {
    let omega = 2.0 * PI * F;
    ((omega * TAU).atan() + k as f64 * PI) / omega
}

fn time() -> Vec<f64> {
    (0..SAMPLES)
        .map(|index| STOP * index as f64 / (SAMPLES - 1) as f64)
        .collect()
}

fn damped_run() -> SimulationRun {
    let time = time();
    let out: Vec<f64> = time.iter().map(|t| damped(*t)).collect();
    let mut run = SimulationRun::new(1);
    run.lifecycle = rspice_results::run::SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", time, out, "#00aaff").with_unit("V"),
        ]),
    );
    run
}

fn host_on(run: SimulationRun) -> ResultsTestHost {
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state
}

/// One frame of the whole Results centre view: the sheet and the readout
/// dock.
fn whole_frame(
    ctx: &egui::Context,
    parts: &mut ResultsParts<'_>,
    events: Vec<egui::Event>,
) -> Vec<ClippedShape> {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
        events,
        ..Default::default()
    };
    let mut controller = crate::simulation::SimulationController::new();
    ctx.run_ui(input, |ctx| {
        // The dock's Cursors page under the stage, as the shell lays them out.
        egui::Panel::bottom("cursors-page")
            .exact_size(240.0)
            .show(ctx, |ui| super::super::cursors_page(ui, parts));
        egui::CentralPanel::default().show(ctx, |ui| {
            super::super::super::show(ui, parts, &mut controller);
        });
    })
    .shapes
}

/// Run `command`, as its key does, then draw the frame after it.
fn step(
    ctx: &egui::Context,
    state: &mut ResultsTestHost,
    command: ResultsCommand,
) -> Vec<ClippedShape> {
    state.run_command(command);
    whole_frame(ctx, &mut state.results_parts(), Vec::new())
}

fn texts(shapes: &[ClippedShape]) -> Vec<String> {
    fn walk(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => out.push(text.galley.text().to_owned()),
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// Every crossing of `level` the calculator's `cross(w, level, n)` names.
fn calculator_crossings(x: &[f64], y: &[f64], level: f64) -> Vec<f64> {
    (1..)
        .map_while(|n| {
            match FunctionRegistry::dispatch(
                "cross",
                vec![
                    RealValue::create_waveform(x.to_vec(), y.to_vec()),
                    RealValue::Scalar(level),
                    RealValue::Scalar(f64::from(n)),
                ],
            ) {
                Ok(RealValue::Scalar(at)) => Some(at),
                _ => None,
            }
        })
        .collect()
}

#[track_caller]
fn assert_near_extremum(found: f64, k: usize) {
    assert!(
        (found - extremum(k)).abs() <= 0.5 * DT * (1.0 + 1e-9),
        "extremum {k}: the cursor is at {found}, the analytic one at {}",
        extremum(k)
    );
}

fn mid_level() -> f64 {
    let y: Vec<f64> = time().iter().map(|t| damped(*t)).collect();
    let (low, high) = y
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(*value), high.max(*value))
        });
    (low + high) * 0.5
}

/// The sample steps move cursor A sample by sample, the peak steps from
/// peak to peak and the crossing steps from crossing to crossing of the
/// trace's mid level, then of H1 once one is placed, each crossing exactly
/// where the calculator's `cross()` puts it.
#[test]
fn the_steps_move_the_cursor_by_sample_peak_and_crossing() {
    let mut state = host_on(damped_run());
    let start = 0.1003e-3;
    place_cursors(&mut state, Some(start), None);
    let ctx = theme_context();
    whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    let a = |state: &ResultsTestHost| state.results.results.session.cursors.a.unwrap();
    let time = time();
    let y: Vec<f64> = time.iter().map(|t| damped(*t)).collect();

    let shapes = step(&ctx, &mut state, ResultsCommand::CursorToNextSample);
    let next = *time.iter().find(|t| **t > start).unwrap();
    assert_eq!(a(&state), next, "Right steps to the next sample");
    step(&ctx, &mut state, ResultsCommand::CursorToPreviousSample);
    let previous = *time.iter().rev().find(|t| **t < next).unwrap();
    assert_eq!(a(&state), previous, "Left steps back one sample");
    let color = Tokens::get(&ctx).color.canvas_cursor_a;
    assert!(
        !cursor_lines(&verticals(&shapes), color).is_empty(),
        "the frame draws cursor A where the step put it: {:?}",
        texts(&shapes)
    );

    for k in [0, 2] {
        step(&ctx, &mut state, ResultsCommand::CursorToNextPeak);
        assert_near_extremum(a(&state), k);
    }
    step(&ctx, &mut state, ResultsCommand::CursorToPreviousPeak);
    assert_near_extremum(a(&state), 0);

    let crossings = calculator_crossings(&time, &y, mid_level());
    let from = a(&state);
    step(&ctx, &mut state, ResultsCommand::CursorToNextCrossing);
    let expected = crossings.iter().copied().find(|at| *at > from).unwrap();
    assert_eq!(
        a(&state).to_bits(),
        expected.to_bits(),
        "Alt+Right lands on cross(w, mid, n) exactly"
    );

    // With H1 on the pane, the crossings are of its level.
    let analysis = strip(&mut state)[0].analysis_key;
    state.results.results.session.set_horizontal_cursor(
        WavePanePresentationKey {
            analysis,
            unit: "V".to_owned(),
        },
        0,
        0.3,
    );
    let from = a(&state);
    step(&ctx, &mut state, ResultsCommand::CursorToNextCrossing);
    let expected = calculator_crossings(&time, &y, 0.3)
        .into_iter()
        .find(|at| *at > from)
        .unwrap();
    assert_eq!(a(&state).to_bits(), expected.to_bits());
    step(&ctx, &mut state, ResultsCommand::CursorToPreviousCrossing);
    let expected = calculator_crossings(&time, &y, 0.3)
        .into_iter()
        .rev()
        .find(|at| *at < expected)
        .unwrap();
    assert_eq!(
        a(&state).to_bits(),
        expected.to_bits(),
        "Alt+Left steps back"
    );
}

/// A step that finds nothing leaves the cursor and says why.
#[test]
fn a_step_past_the_last_peak_says_so_and_moves_nothing() {
    let mut state = host_on(damped_run());
    let last = extremum(8) + DT;
    place_cursors(&mut state, Some(last), None);
    let ctx = theme_context();
    whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    step(&ctx, &mut state, ResultsCommand::CursorToNextPeak);
    assert_eq!(state.results.results.session.cursors.a, Some(last));
    assert_eq!(
        state.prompt_text(),
        Some("V(out) has no next peak after the cursor.")
    );
}

/// Three peak markers placed off their peaks land on them, and with the
/// first as the base the others' difference columns read Δx and Δy equal
/// to the analytic differences between the damped sine's peaks.
#[test]
fn a_base_marker_reads_the_analytic_differences() {
    let mut state = host_on(damped_run());
    let models = strip(&mut state);
    let model = &models[0];
    let trace = &model.traces[0];
    state.quick_frame();
    let mut ids = Vec::new();
    for (k, offset) in [(0, 9.0), (2, -6.0), (4, 4.0)] {
        let marker = state
            .place_marker(&trace.source_waveform_name, extremum(k) + offset * DT)
            .expect("a marker");
        state.mark(marker, "", MarkerKind::Peak);
        ids.push(marker);
    }
    state.results.results.session.readout_base =
        Some(rspice_results_ui::session::ReadingColumnId::Marker(ids[0]));

    let found: Vec<f64> = state.results.results.session.document_markers[..3]
        .iter()
        .map(|marker| marker_position(model, &marker.anchor, marker.x, marker.kind).unwrap())
        .collect();
    for (found, k) in found.iter().zip([0, 2, 4]) {
        assert_near_extremum(*found, k);
    }
    for (index, k) in [(1, 2), (2, 4)] {
        let dx = found[index] - found[0];
        let dy = damped(found[index]) - damped(found[0]);
        assert!(
            (dx - (extremum(k) - extremum(0))).abs() <= DT * (1.0 + 1e-9),
            "Δx {dx} against {}",
            extremum(k) - extremum(0)
        );
        let curvature = (2.0 * PI * F + 1.0 / TAU).powi(2);
        assert!(
            (dy - (damped(extremum(k)) - damped(extremum(0)))).abs() <= curvature * DT * DT * 0.25,
            "Δy {dy}"
        );
    }

    let ctx = theme_context();
    whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    let painted = texts(&whole_frame(&ctx, &mut state.results_parts(), Vec::new()));
    let presentation = state.model.preferences.result_presentation_policy();
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let quantity = state.model.preferences.quantity_presentation_policy();
    for index in [1, 2] {
        let head = format!(
            "M{}-M{}",
            ids[index].marker_id.get(),
            ids[0].marker_id.get()
        );
        let dx = rspice_results_ui::waves::cursor_values::x_delta(
            model,
            found[0],
            found[index],
            digits,
            quantity,
        );
        let dy = model.format_trace_value(
            trace,
            damped(found[index]) - damped(found[0]),
            digits,
            quantity,
        );
        for expected in [&head, &dx, &dy] {
            assert!(
                painted.contains(expected),
                "{head} reads {expected}: {painted:?}"
            );
        }
    }
}

/// A cursor's X is typed in place: a click on it in the Cursors table opens
/// it, and the typed value is where the cursor then stands.
#[test]
fn a_typed_cursor_value_lands_on_that_x() {
    let mut state = host_on(damped_run());
    place_cursors(&mut state, Some(1.0e-3), Some(2.0e-3));
    let ctx = theme_context();
    whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    let shapes = whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    let page_top = FRAME.y - 240.0;
    let (_, cell) = placed_texts(&shapes)
        .into_iter()
        .find(|(text, rect)| text == "1 ms" && rect.top() > page_top)
        .expect("A's X in the table");
    let at = cell.center();
    whole_frame(
        &ctx,
        &mut state.results_parts(),
        vec![egui::Event::PointerMoved(at)],
    );
    for pressed in [true, false] {
        whole_frame(
            &ctx,
            &mut state.results_parts(),
            vec![egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            }],
        );
    }
    let key = |key: egui::Key| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    };
    // The field opens on the printed X and takes the keyboard.
    whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    let mut typing = vec![key(egui::Key::End)];
    typing.extend(std::iter::repeat_with(|| key(egui::Key::Backspace)).take(8));
    typing.push(egui::Event::Text("1.25 ms".to_owned()));
    whole_frame(&ctx, &mut state.results_parts(), typing);
    whole_frame(
        &ctx,
        &mut state.results_parts(),
        vec![key(egui::Key::Enter)],
    );
    let a = state.results.results.session.cursors.a.expect("A stands");
    assert!(
        (a - 1.25e-3).abs() < 1e-15,
        "A typed to 1.25 ms stands at {a}"
    );
    assert_eq!(state.results.results.session.cursors.b, Some(2.0e-3));
}

/// Add places a cursor pair a third and two thirds across the strip, and a
/// vertical marker at its middle, which the table reads as a column of its
/// own over every trace.
#[test]
fn add_places_a_delta_pair_and_a_vertical_marker_the_table_reads() {
    let mut state = host_on(damped_run());
    state.quick_frame();
    super::super::readout::add_from_cursors_page(&mut state.results_parts(), "Delta pair");
    let cursors = state.results.results.session.cursors;
    assert!((cursors.a.unwrap() - STOP / 3.0).abs() < 1e-15);
    assert!((cursors.b.unwrap() - STOP * 2.0 / 3.0).abs() < 1e-15);
    super::super::readout::add_from_cursors_page(&mut state.results_parts(), "Vertical marker");
    let marker = state
        .results
        .results
        .session
        .document_markers
        .last()
        .expect("the vertical marker")
        .clone();
    assert_eq!(marker.kind, MarkerKind::Vertical);
    assert!((marker.x - STOP / 2.0).abs() < 1e-15);

    let tables = super::super::readout::cursor_tables(&mut state.results_parts());
    assert_eq!(tables.len(), 1);
    let table = &tables[0];
    assert_eq!(
        table.heads(),
        vec![
            "A".to_owned(),
            "B".to_owned(),
            format!("M{}", marker.retained_id.get()),
            "B-A".to_owned(),
            format!("M{}-A", marker.retained_id.get()),
        ],
        "three columns read against A, and no slope past a pair"
    );
    // The strip's middle is the sine's fifth zero crossing: the marker's
    // column reads 0 there, not the floating point's remainder.
    let row = &table.bands[0].rows[0];
    assert_eq!(row.name, "V(out)");
    assert_eq!(row.cells[2], "0 V");
}

/// The A/B cursors are the working view's: saved with the project, they
/// stand where they stood when it is opened again, and clearing them clears
/// the view's.
#[test]
fn the_cursors_are_saved_with_the_project() {
    let mut state = host_on(damped_run());
    state.quick_frame();
    {
        let session = &mut state.results.results.session;
        session.cursors.a = Some(1.0e-3);
        session.cursors.b = Some(2.0e-3);
        session.cursor_strip = Some(0);
    }
    state.quick_frame();
    use crate::results::visualization_document::TypedValue;
    let mut held: Vec<_> = state
        .working_view()
        .expect("the run's view")
        .cursors()
        .iter()
        .map(|cursor| (cursor.label.clone(), cursor.position.clone()))
        .collect();
    held.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(
        held,
        vec![
            ("A".to_owned(), TypedValue::Real(1.0e-3)),
            ("B".to_owned(), TypedValue::Real(2.0e-3)),
        ]
    );

    let mut reopened = state.reopened();
    reopened.quick_frame();
    let session = &reopened.results.results.session;
    assert_eq!(
        (session.cursors.a, session.cursors.b, session.cursor_strip),
        (Some(1.0e-3), Some(2.0e-3), Some(0))
    );

    reopened.results.results.session.clear_cursors();
    reopened.quick_frame();
    assert!(
        reopened
            .working_view()
            .expect("the run's view")
            .cursors()
            .is_empty()
    );
}

/// A plotted expression reads the cursors and the markers by name: `xB - xA`
/// is the cursors' span, and `m<n>.y` follows its marker when it moves.
#[test]
fn a_plotted_expression_reads_the_cursors_and_a_marker_by_name() {
    let mut state = host_on(damped_run());
    state.quick_frame();
    place_cursors(&mut state, Some(1.0e-3), Some(2.5e-3));
    let selector = state
        .place_marker("V(out)", 1.0e-3)
        .expect("a marker on V(out)");
    state.quick_frame();
    let number = state
        .results
        .results
        .session
        .document_markers
        .last()
        .expect("the marker")
        .retained_id
        .get();
    let key = {
        let run = state.model.simulation.active_run().expect("the run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    };
    for text in ["xB - xA".to_owned(), format!("m{number}.y")] {
        state
            .results
            .results
            .add_expression_trace(&state.model.simulation, key, text)
            .expect("the strip takes the expression");
    }
    let read = |state: &mut ResultsTestHost| -> Vec<f64> {
        state.quick_frame();
        let models = strip(state);
        super::super::expressions::resolve_strip_exprs(
            &mut state.results_parts(),
            &models[0],
            &Tokens::default(),
        )
        .iter()
        .map(|curve| curve.y.read_whole().expect("in memory")[0])
        .collect()
    };
    let values = read(&mut state);
    assert_eq!(values.len(), 2, "both expressions evaluate: {values:?}");
    assert!(
        (values[0] - 1.5e-3).abs() < 1e-15,
        "xB - xA reads {}",
        values[0]
    );
    assert!(
        (values[1] - damped(1.0e-3)).abs() < 1e-6,
        "m{number}.y reads {} where V(out) is {}",
        values[1],
        damped(1.0e-3)
    );

    super::super::super::move_marker(
        &mut state.results_parts(),
        rspice_results_ui::waves::dock::MarkerMove {
            selector,
            x: 2.0e-3,
            nearest_sample: None,
        },
    );
    let values = read(&mut state);
    assert!(
        (values[1] - damped(2.0e-3)).abs() < 1e-6,
        "m{number}.y follows the marker to {} where V(out) is {}",
        values[1],
        damped(2.0e-3)
    );
}

/// With Snap on, a cursor put between samples stands on the nearest one.
#[test]
fn snap_stands_a_cursor_on_the_nearest_sample() {
    let mut state = host_on(damped_run());
    state.results.results.session.snap_to_samples = true;
    let between = 400.4 * DT;
    place_cursors(&mut state, Some(between), None);
    let ctx = theme_context();
    whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    let a = state.results.results.session.cursors.a.expect("A stands");
    assert_eq!(
        a.to_bits(),
        time()[400].to_bits(),
        "A snapped from {between} to {a}"
    );
}

/// With the horizontal-cursor tool, a click on the pane places H1, a second
/// H2, and a press near H1 moves H1; the plot dimensions ΔY between them and
/// the dock states H1, H2 and ΔY in the pane's unit.
#[test]
fn two_horizontal_cursors_are_placed_moved_and_read_per_pane() {
    let mut state = host_on(damped_run());
    place_cursors(&mut state, Some(extremum(0)), None);
    let ctx = theme_context();
    whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    let shapes = whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    let color = Tokens::get(&ctx).color.canvas_cursor_a;
    let pane = *cursor_lines(&verticals(&shapes), color)
        .iter()
        .max_by(|a, b| (a.bottom - a.top).total_cmp(&(b.bottom - b.top)))
        .expect("cursor A spans the pane");
    state.results.results.session.plot_tool = ResultPlotTool::HorizontalCursor;
    let click = |state: &mut ResultsTestHost, at: egui::Pos2| {
        whole_frame(
            &ctx,
            &mut state.results_parts(),
            vec![egui::Event::PointerMoved(at)],
        );
        for pressed in [true, false] {
            whole_frame(
                &ctx,
                &mut state.results_parts(),
                vec![egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::default(),
                }],
            );
        }
    };
    let height = pane.bottom - pane.top;
    let x = pane.x + 60.0;
    let analysis = strip(&mut state)[0].analysis_key;
    let key = WavePanePresentationKey {
        analysis,
        unit: "V".to_owned(),
    };
    let cursors =
        |state: &ResultsTestHost| state.results.results.session.pane_horizontal_cursors(&key);

    click(&mut state, egui::pos2(x, pane.top + height * 0.25));
    let [Some(h1), None] = cursors(&state) else {
        panic!("the first click places H1: {:?}", cursors(&state));
    };
    click(&mut state, egui::pos2(x, pane.top + height * 0.75));
    let [Some(same), Some(h2)] = cursors(&state) else {
        panic!("the second click places H2: {:?}", cursors(&state));
    };
    assert_eq!(same, h1);
    assert!(h1 > h2, "H1 was placed above H2");
    click(&mut state, egui::pos2(x, pane.top + height * 0.25 + 3.0));
    let [Some(moved), Some(kept)] = cursors(&state) else {
        panic!("both stay placed");
    };
    assert!(moved < h1 && moved > h2, "a press near H1 moves H1");
    assert_eq!(kept, h2);

    let painted = texts(&whole_frame(&ctx, &mut state.results_parts(), Vec::new()));
    let digits = usize::from(
        state
            .model
            .preferences
            .result_presentation_policy()
            .displayed_significant_digits()
            .get(),
    );
    let delta = rspice_results::number_format::fmt_si_significant(kept - moved, "V", digits);
    assert!(
        painted.contains(&format!("\u{0394}Y {delta}")),
        "the dock reads ΔY {delta}: {painted:?}"
    );
    assert_eq!(
        painted
            .iter()
            .filter(|text| text.contains(&format!("\u{0394}Y {delta}")))
            .count(),
        2,
        "and the plot dimensions it on its own axis: {painted:?}"
    );
    assert!(painted.iter().any(|text| text.starts_with("H2 ")));
}

/// On a real run — the ngspice corpus' ECL Schmitt trigger — every next
/// crossing the cursor steps to is the calculator's `cross()` for the same
/// level and count.
#[test]
fn the_next_crossing_on_a_corpus_transient_is_the_calculators_cross() {
    let deck = std::fs::read_to_string(format!(
        "{}/../../tests/ngspice/general/schmitt.cir",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("the corpus deck is readable");
    let run = run_manual_batch(&deck);
    let mut state = host_on(run.clone());
    for analysis in &mut state.model.simulation.retained.runs[0].analyses {
        for waveform in &mut analysis.waveforms {
            waveform.visible = waveform.name == "6";
        }
    }
    let waveform = run.analyses[0]
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "6")
        .unwrap_or_else(|| {
            let names: Vec<_> = run
                .analyses
                .iter()
                .flat_map(|analysis| analysis.waveforms.iter().map(|w| w.name.clone()))
                .collect();
            panic!("the run keeps V(6): {names:?}")
        });
    let (x, y) = (
        waveform.x.read_whole().unwrap().to_vec(),
        waveform.y.read_whole().unwrap().to_vec(),
    );
    let (low, high) = y
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(*value), high.max(*value))
        });
    let level = (low + high) * 0.5;
    let crossings = calculator_crossings(&x, &y, level);
    assert!(crossings.len() >= 2, "the trigger switches: {crossings:?}");

    place_cursors(&mut state, Some(x[0]), None);
    let tokens = Tokens::default();
    for (n, expected) in crossings.iter().enumerate() {
        let at = jump_now(
            &mut state.results_parts(),
            &tokens,
            CursorJump::Crossing(Direction::Next),
        )
        .unwrap_or_else(|refusal| panic!("crossing {}: {refusal}", n + 1));
        assert_eq!(
            at.to_bits(),
            expected.to_bits(),
            "crossing {} of {level} V: the cursor at {at}, cross() at {expected}",
            n + 1
        );
    }
    assert!(
        jump_now(
            &mut state.results_parts(),
            &tokens,
            CursorJump::Crossing(Direction::Next),
        )
        .is_err(),
        "and there is no crossing past the last"
    );
}

/// A peak search reads its trace from the start, so on a stored trace it runs on a Results job:
/// the cursor stays where it was until the job answers, and then moves to the peak.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_peak_of_a_stored_trace_is_searched_off_the_frame_and_the_cursor_moves_when_it_answers() {
    use crate::workbench::documents::result_document::test_fixtures::stored_transient;
    use crate::workbench::results_job::hold_jobs_for_tests;
    // A 10 kHz sine sampled every microsecond peaks at 25 us, sample 25.
    let time: Vec<f64> = (0..=200).map(|step| f64::from(step) * 1.0e-6).collect();
    let sine: Vec<f64> = time.iter().map(|t| (2.0 * PI * 1.0e4 * t).sin()).collect();
    let peak = time[25];
    let (analysis, _columns) =
        stored_transient(time, vec![("V(out)".to_owned(), sine)], Some([5; 32]));
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(analysis);
    place_cursors(&mut state, Some(0.0), None);
    let tokens = Tokens::default();

    hold_jobs_for_tests(true);
    let jumped = jump_cursor(
        &mut state.results_parts(),
        &tokens,
        CursorJump::Peak(Direction::Next),
    );
    hold_jobs_for_tests(false);
    assert_eq!(
        jumped,
        Ok(None),
        "the search runs on a job, not on the frame"
    );
    assert!(
        poll_cursor_jump(&mut state.results_parts(), &tokens),
        "a frame finds it still searching"
    );
    assert_eq!(
        state.results.results.session.cursors.a,
        Some(0.0),
        "the cursor waits for the answer"
    );

    state
        .results
        .results
        .cursor_jump
        .as_ref()
        .expect("the search is held")
        .job
        .release();
    let answer = settle_cursor_jump(&mut state.results_parts(), &tokens);
    assert_eq!(answer, Some(Ok(peak)));
    assert_eq!(
        state.results.results.session.cursors.a,
        Some(peak),
        "the cursor moves to the peak the search found"
    );
}

/// A search the reader overtook, by moving the cursor while it ran, lands nothing.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_search_answering_after_the_cursor_moved_leaves_the_cursor_where_the_reader_put_it() {
    use crate::workbench::documents::result_document::test_fixtures::stored_transient;
    use crate::workbench::results_job::hold_jobs_for_tests;
    let time: Vec<f64> = (0..=200).map(|step| f64::from(step) * 1.0e-6).collect();
    let sine: Vec<f64> = time.iter().map(|t| (2.0 * PI * 1.0e4 * t).sin()).collect();
    let (analysis, _columns) =
        stored_transient(time, vec![("V(out)".to_owned(), sine)], Some([6; 32]));
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(analysis);
    place_cursors(&mut state, Some(0.0), None);
    let tokens = Tokens::default();

    hold_jobs_for_tests(true);
    let jumped = jump_cursor(
        &mut state.results_parts(),
        &tokens,
        CursorJump::Crossing(Direction::Next),
    );
    hold_jobs_for_tests(false);
    assert_eq!(jumped, Ok(None));
    state.results.results.session.cursors.a = Some(120.0e-6);
    state
        .results
        .results
        .cursor_jump
        .as_ref()
        .expect("the search is held")
        .job
        .release();
    assert!(
        settle_cursor_jump(&mut state.results_parts(), &tokens)
            .is_some_and(|answer| answer.is_ok())
    );
    assert_eq!(state.results.results.session.cursors.a, Some(120.0e-6));
}

/// Move the stepping cursor to `jump` and wait for a search that runs off the frame: where it
/// landed, or why it found nothing.
pub(super) fn jump_now(
    state: &mut ResultsParts<'_>,
    tokens: &Tokens,
    jump: CursorJump,
) -> Result<f64, String> {
    match jump_cursor(state, tokens, jump)? {
        Some(at) => Ok(at),
        None => settle_cursor_jump(state, tokens).expect("the jump searches"),
    }
}

/// Every text the frame painted, with where it stands.
fn placed_texts(shapes: &[ClippedShape]) -> Vec<(String, egui::Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::Shape::Text(text) => out.push((
                text.galley.text().to_owned(),
                text.galley.rect.translate(text.pos.to_vec2()),
            )),
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// Every rectangle or stroke the frame painted in `color`, by its bounds.
fn painted_in(shapes: &[ClippedShape], color: egui::Color32) -> Vec<egui::Rect> {
    fn walk(shape: &egui::Shape, color: egui::Color32, out: &mut Vec<egui::Rect>) {
        match shape {
            egui::Shape::Rect(rect) if rect.fill == color || rect.stroke.color == color => {
                out.push(rect.rect);
            }
            egui::Shape::LineSegment { points, stroke } if stroke.color == color => {
                out.push(egui::Rect::from_two_pos(points[0], points[1]));
            }
            egui::Shape::Circle(circle) if circle.fill == color || circle.stroke.color == color => {
                out.push(circle.visual_bounding_rect());
            }
            egui::Shape::Path(path)
                if path.fill == color
                    || path.stroke.color == egui::epaint::ColorMode::Solid(color) =>
            {
                out.push(path.visual_bounding_rect());
            }
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, color, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, color, &mut out);
    }
    out
}

/// The shared ruler's band and its overview track: the smallest panel fill
/// holding the overview's "Full" name, and the canvas fill inside it.
fn ruler_band(shapes: &[ClippedShape], tokens: &Tokens) -> (egui::Rect, egui::Rect) {
    let name = placed_texts(shapes)
        .into_iter()
        .find(|(text, _)| text == "Full")
        .expect("the ruler names its overview")
        .1;
    let area = |rect: &egui::Rect| rect.area();
    let band = painted_in(shapes, tokens.color.bg_panel)
        .into_iter()
        .filter(|rect| rect.contains_rect(name))
        .min_by(|a, b| area(a).total_cmp(&area(b)))
        .expect("the ruler paints its band");
    let track = painted_in(shapes, tokens.color.canvas_bg)
        .into_iter()
        .filter(|rect| band.contains_rect(*rect) && rect.width() > 100.0)
        .min_by(|a, b| area(a).total_cmp(&area(b)))
        .expect("the ruler paints its overview track");
    (band, track)
}

/// The marks spanning the overview track's height in `ink`: the thumb's
/// frame and its two grab handles.
fn thumb_marks(shapes: &[ClippedShape], ink: egui::Color32, track: egui::Rect) -> Vec<egui::Rect> {
    painted_in(shapes, ink)
        .into_iter()
        .filter(|mark| {
            mark.height() >= track.height() - 1.0
                && track.expand2(egui::vec2(4.0, 2.0)).contains_rect(*mark)
        })
        .collect()
}

/// The grab handles among a thumb's marks: its narrow bars, left to right.
fn thumb_handles(marks: &[egui::Rect]) -> Vec<egui::Rect> {
    let mut handles: Vec<egui::Rect> = marks
        .iter()
        .copied()
        .filter(|mark| mark.width() < 8.0)
        .collect();
    handles.sort_by(|a, b| a.left().total_cmp(&b.left()));
    handles
}

/// Dragging the full view's left handle inward zooms in: the panes then show
/// from where it was let go to the record's end, which holds.
#[test]
fn dragging_a_full_view_handle_inward_zooms_in() {
    let mut state = host_on(damped_run());
    let ctx = theme_context();
    let mut shapes = Vec::new();
    for _ in 0..2 {
        shapes = whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    }
    let tokens = Tokens::get(&ctx);
    let (_, track) = ruler_band(&shapes, &tokens);
    let grip = thumb_handles(&thumb_marks(&shapes, tokens.color.text_dim, track))[0].center();
    let to = egui::pos2(track.left() + track.width() * 0.2, grip.y);
    let button = |pos: egui::Pos2, pressed: bool| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    };
    for events in [
        vec![egui::Event::PointerMoved(grip)],
        vec![button(grip, true)],
        vec![egui::Event::PointerMoved(grip + egui::vec2(12.0, 0.0))],
        vec![egui::Event::PointerMoved(to)],
        vec![button(to, false)],
        Vec::new(),
    ] {
        whole_frame(&ctx, &mut state.results_parts(), events);
    }
    let analysis = strip(&mut state)[0].analysis_key;
    let (from, until) =
        rspice_results_ui::waves::stack::shared_x_view(&state.results.results.session, analysis)
            .expect("the drag zoomed the panes in");
    assert!(
        (from - 0.2 * STOP).abs() < 0.01 * STOP,
        "the view starts where the handle was let go: {from}"
    );
    assert!(
        (until - STOP).abs() < 1e-9 * STOP,
        "the record's end holds: {until}"
    );
}
