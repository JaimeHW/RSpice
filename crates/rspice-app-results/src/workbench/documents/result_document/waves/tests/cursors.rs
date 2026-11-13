//! Cursor jumps, keyboard stepping, point markers read against a reference,
//! and horizontal cursors, each through the whole Results frame the reader
//! sees: the document bar, the sheet, its keys and the readout dock.

use super::lanes::{FRAME, cursor_lines, place_cursors, strip, theme_context, verticals};
use super::*;
use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::results::ResultsFeature;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::state::{WorkbenchSession, Workspace};
use egui::epaint::ClippedShape;
use rspice_app_runs::simulation::controller::test_execution::run_manual_batch;
use rspice_results::calculator::functions::FunctionRegistry;
use rspice_results::calculator::value::RealValue;
use rspice_results::feature_search::Direction;
use rspice_results::result_presentation::{MarkerKind, WavePanePresentationKey};
use rspice_results_ui::chrome::instrument::ResultPlotTool;
use rspice_results_ui::session::MarkerSelector;
use rspice_results_ui::waves::dock::marker_position;
use rspice_results_ui::waves::navigation::CursorJump;
use rspice_results_ui::waves::readout::x_separation;
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

/// One frame of the whole Results centre view: the keys it answers, the
/// sheet and the readout dock.
fn whole_frame(
    ctx: &egui::Context,
    parts: &mut ResultsParts<'_>,
    events: Vec<egui::Event>,
) -> Vec<ClippedShape> {
    let modifiers = events
        .iter()
        .find_map(|event| match event {
            egui::Event::Key { modifiers, .. } => Some(*modifiers),
            _ => None,
        })
        .unwrap_or_default();
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
        modifiers,
        events,
        ..Default::default()
    };
    let mut controller = crate::simulation::SimulationController::new();
    ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            super::super::super::show(ui, parts, &mut controller, None);
        });
    })
    .shapes
}

fn key(key: egui::Key, modifiers: egui::Modifiers) -> Vec<egui::Event> {
    vec![egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }]
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

/// Left and Right step cursor A sample by sample, Shift+arrow from peak to
/// peak and Alt+arrow from crossing to crossing of the trace's mid level,
/// then of H1 once one is placed, each crossing exactly where the
/// calculator's `cross()` puts it.
#[test]
fn the_arrow_keys_step_the_cursor_by_sample_peak_and_crossing() {
    let mut state = host_on(damped_run());
    let start = 0.1003e-3;
    place_cursors(&mut state, Some(start), None);
    let ctx = theme_context();
    whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    let a = |state: &ResultsTestHost| state.results.results.session.cursors.a.unwrap();
    let time = time();
    let y: Vec<f64> = time.iter().map(|t| damped(*t)).collect();
    let none = egui::Modifiers::NONE;

    let shapes = whole_frame(
        &ctx,
        &mut state.results_parts(),
        key(egui::Key::ArrowRight, none),
    );
    let next = *time.iter().find(|t| **t > start).unwrap();
    assert_eq!(a(&state), next, "Right steps to the next sample");
    whole_frame(
        &ctx,
        &mut state.results_parts(),
        key(egui::Key::ArrowLeft, none),
    );
    let previous = *time.iter().rev().find(|t| **t < next).unwrap();
    assert_eq!(a(&state), previous, "Left steps back one sample");
    let color = Tokens::get(&ctx).color.canvas_cursor_a;
    assert!(
        !cursor_lines(&verticals(&shapes), color).is_empty(),
        "the frame draws cursor A where the step put it: {:?}",
        texts(&shapes)
    );

    let shift = egui::Modifiers::SHIFT;
    for k in [0, 2] {
        whole_frame(
            &ctx,
            &mut state.results_parts(),
            key(egui::Key::ArrowRight, shift),
        );
        assert_near_extremum(a(&state), k);
    }
    whole_frame(
        &ctx,
        &mut state.results_parts(),
        key(egui::Key::ArrowLeft, shift),
    );
    assert_near_extremum(a(&state), 0);

    let alt = egui::Modifiers::ALT;
    let crossings = calculator_crossings(&time, &y, mid_level());
    let from = a(&state);
    whole_frame(
        &ctx,
        &mut state.results_parts(),
        key(egui::Key::ArrowRight, alt),
    );
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
    whole_frame(
        &ctx,
        &mut state.results_parts(),
        key(egui::Key::ArrowRight, alt),
    );
    let expected = calculator_crossings(&time, &y, 0.3)
        .into_iter()
        .find(|at| *at > from)
        .unwrap();
    assert_eq!(a(&state).to_bits(), expected.to_bits());
    whole_frame(
        &ctx,
        &mut state.results_parts(),
        key(egui::Key::ArrowLeft, alt),
    );
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
    whole_frame(
        &ctx,
        &mut state.results_parts(),
        key(egui::Key::ArrowRight, egui::Modifiers::SHIFT),
    );
    assert_eq!(state.results.results.session.cursors.a, Some(last));
    assert_eq!(
        state.prompt_text(),
        Some("V(out) has no next peak after the cursor.")
    );
}

fn harness_on(run: SimulationRun) -> FeatureHarness<ResultsFeature> {
    let host = host_on(run);
    let mut harness =
        FeatureHarness::<ResultsFeature>::with_model(host.model, WorkbenchSession::default());
    harness.wb.workspace = Workspace::Results;
    harness
}

fn harness_frame(
    ctx: &egui::Context,
    harness: &mut FeatureHarness<ResultsFeature>,
    events: Vec<egui::Event>,
) -> Vec<ClippedShape> {
    let mut parts = harness.state.parts(
        &mut harness.model,
        &mut harness.wb,
        &harness.platform,
        &mut harness.runs.preflight,
        harness.commands,
    );
    whole_frame(ctx, &mut parts, events)
}

/// Each cursor command moves the stepping cursor to its feature of the
/// trace, the readout dock then states the reading, and Step the other
/// cursor hands the jumps to B.
#[test]
fn the_cursor_commands_move_the_stepping_cursor_to_each_feature() {
    let mut harness = harness_on(damped_run());
    let ctx = theme_context();
    harness_frame(&ctx, &mut harness, Vec::new());
    for command in [
        ResultsCommand::CursorToMaximum,
        ResultsCommand::CursorToMinimum,
        ResultsCommand::CursorToNextPeak,
        ResultsCommand::CursorToPreviousPeak,
        ResultsCommand::CursorToNextCrossing,
        ResultsCommand::CursorToPreviousCrossing,
    ] {
        assert_eq!(
            harness.availability(command.key()),
            CommandAvailability::Available,
            "{command:?}"
        );
    }
    assert!(matches!(
        harness.availability(ResultsCommand::SwitchSteppingCursor.key()),
        CommandAvailability::Disabled(_)
    ));
    let cursors = |harness: &FeatureHarness<ResultsFeature>| harness.state.results.session.cursors;

    harness.execute(ResultsCommand::CursorToMaximum.key());
    assert_near_extremum(cursors(&harness).a.unwrap(), 0);
    harness.execute(ResultsCommand::CursorToMinimum.key());
    assert_near_extremum(cursors(&harness).a.unwrap(), 1);
    harness.execute(ResultsCommand::CursorToNextPeak.key());
    assert_near_extremum(cursors(&harness).a.unwrap(), 2);
    harness.execute(ResultsCommand::CursorToPreviousPeak.key());
    assert_near_extremum(cursors(&harness).a.unwrap(), 0);

    harness.execute(ResultsCommand::SwitchSteppingCursor.key());
    let time = time();
    let y: Vec<f64> = time.iter().map(|t| damped(*t)).collect();
    let crossings = calculator_crossings(&time, &y, mid_level());
    let a = cursors(&harness).a.unwrap();
    harness.execute(ResultsCommand::CursorToNextCrossing.key());
    let b = cursors(&harness).b.expect("B is placed from A");
    assert_eq!(
        b.to_bits(),
        crossings
            .iter()
            .copied()
            .find(|at| *at > a)
            .unwrap()
            .to_bits(),
        "B goes to the crossing after A"
    );
    assert_eq!(cursors(&harness).a, Some(a), "A stays");
    harness.execute(ResultsCommand::CursorToNextCrossing.key());
    harness.execute(ResultsCommand::CursorToPreviousCrossing.key());
    assert_eq!(cursors(&harness).b, Some(b), "and back");
    harness.execute(ResultsCommand::CursorToNextPeak.key());
    assert_near_extremum(cursors(&harness).b.unwrap(), 2);

    // A is on the first peak and B on the third: one period apart.
    let painted = texts(&harness_frame(&ctx, &mut harness, Vec::new()));
    harness_frame(&ctx, &mut harness, Vec::new());
    assert!(
        painted.iter().any(|text| text == "1 ms"),
        "the Δ column reads one period: {painted:?}"
    );
    assert!(
        painted.iter().any(|text| text == "1/\u{0394} 1 kHz"),
        "and 1/Δ its frequency: {painted:?}"
    );

    harness.execute(ResultsCommand::MarkerAtCursor.key());
    let session = &harness.state.results.session;
    assert_eq!(session.markers.len(), 1, "a marker is placed at A");
    assert_eq!(session.markers[0].x, a);
    assert!(session.marker_edit.is_some(), "and its dialog opens on it");
}

/// The commands refuse outside a waveform sheet, and say so.
#[test]
fn the_cursor_commands_need_a_waveform_sheet() {
    let mut harness = harness_on(damped_run());
    harness.wb.workspace = Workspace::Design;
    assert_eq!(
        harness.availability(ResultsCommand::CursorToMaximum.key()),
        CommandAvailability::Disabled("open a result in Results")
    );
    let mut empty = FeatureHarness::<ResultsFeature>::with_model(
        ResultsTestHost::default().model,
        WorkbenchSession::default(),
    );
    empty.wb.workspace = Workspace::Results;
    assert_eq!(
        empty.availability(ResultsCommand::CursorToMaximum.key()),
        CommandAvailability::Disabled("no result dataset is available")
    );
}

/// Three peak markers placed off their peaks land on them, and with the
/// first as the reference the others read Δx and Δy equal to the analytic
/// differences between the damped sine's peaks.
#[test]
fn a_reference_marker_reads_the_analytic_differences() {
    let mut state = host_on(damped_run());
    let models = strip(&mut state);
    let model = &models[0];
    let trace = &model.traces[0];
    let anchor = rspice_results_ui::waves::anchor_key(model, trace);
    let session = &mut state.results.results.session;
    let mut ids = Vec::new();
    for (k, offset) in [(0, 9.0), (2, -6.0), (4, 4.0)] {
        let id = session
            .add_marker(
                model.analysis_key,
                anchor.clone(),
                trace.name.clone(),
                extremum(k) + offset * DT,
            )
            .unwrap();
        session.marker_mut(id).unwrap().kind = MarkerKind::Peak;
        ids.push(id);
    }
    session.toggle_reference_marker(MarkerSelector::Quick(ids[0]));

    let found: Vec<f64> = state.results.results.session.markers[..3]
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
        let dx = x_separation(model, found[0], found[index], digits, quantity);
        let dy = model.format_trace_value(
            trace,
            damped(found[index]) - damped(found[0]),
            digits,
            quantity,
        );
        let expected = format!("\u{0394}x {dx}  \u{0394}y {dy}");
        assert!(
            painted.contains(&expected),
            "M{} reads {expected}: {painted:?}",
            ids[index]
        );
    }
    assert!(painted.iter().any(|text| text == "ref"));
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
    let delta = rspice_results_ui::waves::fmt_in_unit_readout(kept - moved, "V", digits);
    assert!(
        painted.contains(&format!("\u{0394}Y {delta}")),
        "the dock reads ΔY {delta}: {painted:?}"
    );
    assert_eq!(
        painted
            .iter()
            .filter(|text| text.starts_with("\u{0394}Y "))
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
    let (x, y) = (waveform.x.to_vec(), waveform.y.to_vec());
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
        let at = jump_cursor(
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
        jump_cursor(
            &mut state.results_parts(),
            &tokens,
            CursorJump::Crossing(Direction::Next),
        )
        .is_err(),
        "and there is no crossing past the last"
    );
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

/// The shared ruler under the panes reads each cursor's X value in the
/// cursor's colour box, as EZwave's cursor bar does: no tick value stands
/// in a box's column, no cursor line runs through the tick values, and two
/// cursors close together part their boxes so both stay readable.
#[test]
fn the_ruler_flags_read_the_cursor_values_clear_of_the_tick_values() {
    let mut state = host_on(damped_run());
    // B stands on the 2 ms tick and A close enough that the boxes must part.
    let (a, b) = (1.85e-3, 2.0e-3);
    place_cursors(&mut state, Some(a), Some(b));
    let models = strip(&mut state);
    let presentation = state.model.preferences.result_presentation_policy();
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let quantity = state.model.preferences.quantity_presentation_policy();
    let (a_value, b_value) = (
        models[0].format_x(a, digits, quantity),
        models[0].format_x(b, digits, quantity),
    );
    let ctx = theme_context();
    let mut shapes = Vec::new();
    for _ in 0..2 {
        shapes = whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    }
    let placed = placed_texts(&shapes);
    let tick_row = placed
        .iter()
        .find(|(text, _)| text == "1m")
        .expect("the ruler reads its 1 ms tick")
        .1;
    let ticks: Vec<&(String, egui::Rect)> = placed
        .iter()
        .filter(|(_, rect)| (rect.top() - tick_row.top()).abs() < 0.5)
        .collect();
    let flag = |value: &str| {
        placed
            .iter()
            .find(|(text, rect)| {
                (text == value
                    || text.strip_prefix("A ").or(text.strip_prefix("B ")) == Some(value))
                    && rect.top() >= tick_row.bottom() - 0.5
                    && rect.top() <= tick_row.bottom() + 20.0
            })
            .unwrap_or_else(|| panic!("the ruler reads {value} in a box: {placed:?}"))
            .1
    };
    let (a_box, b_box) = (flag(&a_value), flag(&b_value));
    assert!(
        a_box.max.x < b_box.min.x,
        "A's box stands left of B's and clear of it: {a_box:?} {b_box:?}"
    );
    assert!(
        !ticks.iter().any(|(text, _)| text == "2m"),
        "the 2 ms tick under B gives way to its box: {ticks:?}"
    );
    for (text, rect) in &ticks {
        for flag in [a_box, b_box] {
            assert!(
                !rect.x_range().intersects(flag.x_range()),
                "the tick value {text} stands in a cursor box's column"
            );
        }
    }
    // The box is all the ruler draws of a cursor: no line rises from the
    // ruler toward the pane above, nor crosses the tick values.
    let tokens = Tokens::get(&ctx);
    let lines = verticals(&shapes);
    for color in [tokens.color.canvas_cursor_a, tokens.color.canvas_cursor_b] {
        for line in cursor_lines(&lines, color) {
            assert!(
                line.bottom <= tick_row.top() - 6.0 || line.top >= tick_row.bottom() - 0.5,
                "a cursor line rises out of the ruler or through its tick values: {line:?}"
            );
        }
    }
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
/// holding the overview's "FULL" name, and the canvas fill inside it.
fn ruler_band(shapes: &[ClippedShape], tokens: &Tokens) -> (egui::Rect, egui::Rect) {
    let name = placed_texts(shapes)
        .into_iter()
        .find(|(text, _)| text == "FULL")
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

/// What the ruler painted in a cursor's colour: its box, and every other
/// mark inside the band.
fn ruler_marks(
    shapes: &[ClippedShape],
    color: egui::Color32,
    (band, track): (egui::Rect, egui::Rect),
) -> (egui::Rect, Vec<egui::Rect>) {
    let (boxes, rest): (Vec<_>, Vec<_>) = painted_in(shapes, color)
        .into_iter()
        .filter(|mark| {
            mark.max.y > band.min.y + 0.5
                && mark.min.y < band.max.y - 0.5
                && band.x_range().contains(mark.center().x)
        })
        // The overview's viewport window and its edge handles are drawn in
        // the accent, which cursor A shares. They span the track's whole
        // height, which no cursor mark does.
        .filter(|mark| {
            !(mark.height() >= track.height() - 1.0
                && track.expand2(egui::vec2(4.0, 2.0)).contains_rect(*mark))
        })
        .partition(|mark| mark.height() > 8.0 && mark.width() > 8.0);
    assert_eq!(boxes.len(), 1, "one value box per cursor: {boxes:?}");
    (boxes[0], rest)
}

/// The pane's own line for a cursor, above the ruler.
fn pane_cursor_line(shapes: &[ClippedShape], color: egui::Color32, band: egui::Rect) -> f32 {
    cursor_lines(&verticals(shapes), color)
        .into_iter()
        .filter(|line| line.bottom <= band.top() + 0.5)
        .max_by(|a, b| (a.bottom - a.top).total_cmp(&(b.bottom - b.top)))
        .expect("the pane draws the cursor's line")
        .x
}

/// The ruler draws a cursor as its value box and a one-pixel caret under it
/// at the cursor's exact X, and nothing else: no line rises toward the pane
/// or hangs toward the overview, and at full zoom-out the overview carries
/// no notch, the boxes having located the cursors already. Two boxes pushed
/// apart keep at least 4 px between them, each caret still under its own
/// cursor's line.
#[test]
fn the_ruler_draws_a_cursor_as_its_value_box_and_caret_alone() {
    let mut state = host_on(damped_run());
    place_cursors(&mut state, Some(1.85e-3), Some(2.0e-3));
    let ctx = theme_context();
    let mut shapes = Vec::new();
    for _ in 0..2 {
        shapes = whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    }
    let tokens = Tokens::get(&ctx);
    let ruler = ruler_band(&shapes, &tokens);
    let mut boxes = Vec::new();
    for color in [tokens.color.canvas_cursor_a, tokens.color.canvas_cursor_b] {
        let line = pane_cursor_line(&shapes, color, ruler.0);
        let (flag, rest) = ruler_marks(&shapes, color, ruler);
        let [caret] = rest.as_slice() else {
            panic!("the box's caret is the ruler's only other mark: {rest:?}");
        };
        assert!(caret.width() <= 1.0 && caret.height() <= 2.0 + 1e-3);
        assert!(
            (caret.top() - flag.bottom()).abs() < 0.5,
            "{caret:?} under {flag:?}"
        );
        assert!(
            (caret.center().x - line).abs() < 0.5,
            "the caret stands at the cursor's x {line}: {caret:?}"
        );
        boxes.push(flag);
    }
    let gap = boxes[1].min.x - boxes[0].max.x;
    assert!(gap >= 4.0 - 1e-3, "the boxes stand {gap} px apart");
}

/// With the panes zoomed in, the overview marks where each cursor stands in
/// the whole record with a tick of at most 3 px on its top edge, apart from
/// the box's caret above it.
#[test]
fn a_zoomed_ruler_notches_the_overview_at_each_cursor() {
    let mut state = host_on(damped_run());
    let (a, b) = (1.85e-3, 2.0e-3);
    place_cursors(&mut state, Some(a), Some(b));
    zoom_to(&mut state, (1.7e-3, 2.7e-3));
    let ctx = theme_context();
    let mut shapes = Vec::new();
    for _ in 0..2 {
        shapes = whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    }
    let tokens = Tokens::get(&ctx);
    let ruler = ruler_band(&shapes, &tokens);
    let track = ruler.1;
    for (value, color) in [
        (a, tokens.color.canvas_cursor_a),
        (b, tokens.color.canvas_cursor_b),
    ] {
        let (flag, rest) = ruler_marks(&shapes, color, ruler);
        let (carets, notches): (Vec<_>, Vec<_>) = rest
            .into_iter()
            .partition(|mark| (mark.top() - flag.bottom()).abs() < 0.5);
        let ([caret], [notch]) = (carets.as_slice(), notches.as_slice()) else {
            panic!("one caret and one notch: {carets:?} {notches:?}");
        };
        let at = track.left() + track.width() * (value / STOP) as f32;
        assert!(
            notch.width() <= 1.0 && notch.height() <= 3.0 + 1e-3,
            "{notch:?}"
        );
        assert!(
            (notch.top() - track.top()).abs() < 0.5,
            "{notch:?} on {track:?}"
        );
        assert!(
            (notch.center().x - at).abs() < 0.5,
            "the notch stands at {at}: {notch:?}"
        );
        assert!((notch.center().x - caret.center().x).abs() > 2.0);
    }
}

/// The accent marks spanning the overview track's height: the viewport
/// thumb and its two grab handles.
fn thumb_marks(shapes: &[ClippedShape], tokens: &Tokens, track: egui::Rect) -> Vec<egui::Rect> {
    painted_in(shapes, tokens.color.accent)
        .into_iter()
        .filter(|mark| {
            mark.height() >= track.height() - 1.0
                && track.expand2(egui::vec2(4.0, 2.0)).contains_rect(*mark)
        })
        .collect()
}

/// At full view the overview is its miniature in a quiet frame of border
/// ink: no accent thumb outlines the whole track and no handles stand at its
/// ends. Zoomed in, the thumb takes the accent with a handle at each edge,
/// over exactly the part of the record the panes show, and the track keeps
/// its height so the layout never jumps.
#[test]
fn the_overview_thumb_takes_the_accent_only_while_zoomed() {
    let frame = |state: &mut ResultsTestHost| {
        let ctx = theme_context();
        let mut shapes = Vec::new();
        for _ in 0..2 {
            shapes = whole_frame(&ctx, &mut state.results_parts(), Vec::new());
        }
        let tokens = Tokens::get(&ctx);
        let (_, track) = ruler_band(&shapes, &tokens);
        (shapes, tokens, track)
    };

    let mut state = host_on(damped_run());
    let (shapes, tokens, full_track) = frame(&mut state);
    assert_eq!(
        thumb_marks(&shapes, &tokens, full_track),
        Vec::<egui::Rect>::new(),
        "no thumb at full view"
    );
    assert!(
        painted_in(&shapes, tokens.color.border)
            .iter()
            .any(|rect| (rect.min - full_track.min).length() < 0.5
                && (rect.max - full_track.max).length() < 0.5),
        "the track keeps its quiet frame"
    );

    zoom_to(&mut state, (1.7e-3, 2.7e-3));
    let (shapes, tokens, track) = frame(&mut state);
    assert!((track.height() - full_track.height()).abs() < 1e-3);
    let marks = thumb_marks(&shapes, &tokens, track);
    let (left, right) = (
        track.left() + track.width() * 0.34,
        track.left() + track.width() * 0.54,
    );
    assert!(
        marks
            .iter()
            .any(|mark| (mark.left() - left).abs() < 0.5 && (mark.right() - right).abs() < 0.5),
        "the thumb spans 1.7-2.7 ms of the 5 ms record: {marks:?}"
    );
    for edge in [left, right] {
        assert!(
            marks
                .iter()
                .any(|mark| mark.width() < 8.0 && (mark.center().x - edge).abs() < 0.5),
            "a handle stands at {edge}: {marks:?}"
        );
    }
}

/// On a logarithmic X axis the ruler boxes read each cursor's value in the
/// reader's format, placed by the log scale: a startup transient read over
/// three decades of time.
#[test]
fn a_ruler_flag_on_a_logarithmic_axis_reads_the_cursor_value() {
    let x: Vec<f64> = (0..=60)
        .map(|k| 10f64.powf(-6.0 + f64::from(k) / 20.0))
        .collect();
    let y: Vec<f64> = x.iter().map(|t| 1.0 - (-t / 1.0e-4).exp()).collect();
    let mut run = SimulationRun::new(1);
    run.lifecycle = rspice_results::run::SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", x, y, "#00aaff").with_unit("V"),
        ]),
    );
    let mut state = host_on(run);
    let analysis = strip(&mut state)[0].analysis_key;
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Log10);
    place_cursors(&mut state, Some(12.5e-6), Some(300.0e-6));
    assert_eq!(strip(&mut state)[0].x_scale, XScale::Log10);
    let ctx = theme_context();
    let mut shapes = Vec::new();
    for _ in 0..2 {
        shapes = whole_frame(&ctx, &mut state.results_parts(), Vec::new());
    }
    let placed = placed_texts(&shapes);
    let at = |wanted: &str| {
        placed
            .iter()
            .find(|(text, _)| text == wanted)
            .unwrap_or_else(|| panic!("the ruler reads {wanted}"))
            .1
    };
    let (a_box, b_box) = (at("A 12.5 µs"), at("B 300 µs"));
    // Over the three decades 300 µs stands log10(24)/3 = 0.46 of the plot
    // width past 12.5 µs; a linear placement would put it 0.29 past.
    let apart = b_box.center().x - a_box.center().x;
    assert!(
        apart > 0.38 * FRAME.x,
        "the boxes stand {apart} apart, not on the log scale"
    );
}

/// The pictures for review, written under `RSPICE_PICTURES` when it is set:
/// A and B on the damped sine with their Δ readouts, a reference marker with
/// two relative markers, and two horizontal cursors with their ΔY.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_cursor_pictures() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let write = |name: &str, state: &mut ResultsTestHost| {
        let mut controller = crate::simulation::SimulationController::new();
        for _ in 0..2 {
            let _ = rspice_ui_kit::raster::render(FRAME, |ui, _| {
                super::super::super::show(ui, &mut state.results_parts(), &mut controller, None);
            });
        }
        let canvas = rspice_ui_kit::raster::render(FRAME, |ui, _| {
            super::super::super::show(ui, &mut state.results_parts(), &mut controller, None);
        });
        std::fs::write(
            format!("{directory}/{name}.png"),
            canvas.png(canvas.height()),
        )
        .expect("the picture is written");
    };

    let mut state = host_on(damped_run());
    place_cursors(&mut state, Some(extremum(0)), None);
    state.results.results.session.stepping_cursor = rspice_ui_kit::plot::CursorId::B;
    jump_cursor(
        &mut state.results_parts(),
        &Tokens::default(),
        CursorJump::Peak(Direction::Next),
    )
    .unwrap();
    jump_cursor(
        &mut state.results_parts(),
        &Tokens::default(),
        CursorJump::Peak(Direction::Next),
    )
    .unwrap();
    write("ab-damped-sine", &mut state);

    let mut state = host_on(damped_run());
    let models = strip(&mut state);
    let model = &models[0];
    let trace = &model.traces[0];
    let anchor = rspice_results_ui::waves::anchor_key(model, trace);
    let session = &mut state.results.results.session;
    let mut ids = Vec::new();
    for (k, kind) in [
        (0, MarkerKind::Peak),
        (1, MarkerKind::Valley),
        (2, MarkerKind::Peak),
    ] {
        let id = session
            .add_marker(
                model.analysis_key,
                anchor.clone(),
                trace.name.clone(),
                extremum(k) + 5.0 * DT,
            )
            .unwrap();
        session.marker_mut(id).unwrap().kind = kind;
        ids.push(id);
    }
    session.toggle_reference_marker(MarkerSelector::Quick(ids[0]));
    write("reference-markers", &mut state);

    let mut state = host_on(damped_run());
    let analysis = strip(&mut state)[0].analysis_key;
    let key = WavePanePresentationKey {
        analysis,
        unit: "V".to_owned(),
    };
    let session = &mut state.results.results.session;
    session.set_horizontal_cursor(key.clone(), 0, 0.5);
    session.set_horizontal_cursor(key, 1, -0.25);
    write("horizontal-cursors", &mut state);

    // The cursors' values on the shared X ruler: B stands on the 2 ms tick
    // and A close enough that the two value boxes must make room.
    let mut state = host_on(damped_run());
    place_cursors(&mut state, Some(1.85e-3), Some(2.0e-3));
    write("x-ruler-full", &mut state);

    // The same cursors with the panes zoomed to 1.7-2.7 ms: the overview
    // marks where they stand in the whole record.
    let mut state = host_on(damped_run());
    place_cursors(&mut state, Some(1.85e-3), Some(2.0e-3));
    zoom_to(&mut state, (1.7e-3, 2.7e-3));
    write("x-ruler-zoomed", &mut state);
}

/// The panes of the damped sine's one strip zoomed to `range`.
fn zoom_to(state: &mut ResultsTestHost, range: (f64, f64)) {
    let analysis = strip(state)[0].analysis_key;
    rspice_results_ui::waves::stack::set_shared_x_view(
        &mut state.results.results.session,
        analysis,
        1,
        Some(range),
    );
}
