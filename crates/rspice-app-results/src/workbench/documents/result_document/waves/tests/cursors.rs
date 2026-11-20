//! Cursor jumps, keyboard stepping, point markers read against a reference,
//! and horizontal cursors, each through the whole Results frame the reader
//! sees: the document bar, the sheet, its keys and the readout dock.

use super::lanes::{FRAME, cursor_lines, place_cursors, strip, theme_context, verticals};
use super::*;
use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
use egui::epaint::ClippedShape;
use rspice_app_runs::simulation::controller::test_execution::run_manual_batch;
use rspice_results::calculator::functions::FunctionRegistry;
use rspice_results::calculator::value::RealValue;
use rspice_results::feature_search::Direction;
use rspice_results::result_presentation::{MarkerKind, WavePanePresentationKey};
use rspice_results_ui::chrome::instrument::ResultPlotTool;
use rspice_results_ui::session::MarkerSelector;
use rspice_results_ui::waves::cursor_values::{marker_position, x_separation};
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
            super::super::super::show(ui, parts, &mut controller);
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
    assert!(painted.iter().any(|text| text == "Reference"));
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
        rspice_results_ui::waves::stack::shared_x_view(&state.results.results.session, analysis, 1)
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
