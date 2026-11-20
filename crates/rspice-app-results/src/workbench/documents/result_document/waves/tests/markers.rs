//! The event-marker strip on real runs.
//!
//! An HDL testbench raises a `$warning`, an `$error` and a second
//! `$warning`, then `$finish`es before the deck's stop time; a deck
//! assertion fails while one of its nets is high. A vector file expects
//! outputs after a transient's stop time. Every claim is made on the frame
//! the reader sees.

use super::lanes::{cursor_lines, frame, place_cursors, strip, theme_context, verticals};
use super::*;
use crate::state::SimulationRun;
use egui::epaint::ClippedShape;
use rspice_app_runs::simulation::controller::test_execution::{
    run_manual_batch_at, run_manual_batch_with_lifecycle,
};
use rspice_results::engine_evidence::HdlPoint;
use rspice_results::run::SimulationRunLifecycle;
use rspice_results_ui::waves::logic::{SourceLine, markers::MARKER_STRIP_HEIGHT};
use std::sync::OnceLock;

/// The testbench. Line 10 is the first `$warning`, 11 the `$error`, 13 the
/// second `$warning` and 14 the `$finish`; `p` is high from 20 ns to 55 ns.
const CHECKS: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module checks(q, p);\n\
output q; reg q;\n\
output p; reg p;\n\
initial begin q = 1'b0; p = 1'b0; end\n\
always #10 q = ~q;\n\
initial begin\n\
#20 p = 1'b1;\n\
$warning(\"p rose before q settled\");\n\
#20 $error(\"count mismatch: %0d\", 3);\n\
#15 p = 1'b0;\n\
#15 $warning(\"slow edge on q\");\n\
#10 $finish;\n\
end\n\
endmodule\n";

/// A directory of this test process's own, for the files its decks read.
fn scratch() -> &'static std::path::Path {
    static DIR: OnceLock<std::path::PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("rspice-marker-strip-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("the scratch directory is made");
        dir
    })
}

/// `name` written into the scratch directory, as a deck spells its path: a
/// deck reads a backslash as an escape.
fn scratch_file(name: &str, text: &str) -> String {
    let path = scratch().join(name);
    std::fs::write(&path, text).expect("the scratch file is written");
    path.display().to_string().replace('\\', "/")
}

/// The testbench's run, through the real controller, with a pulse on `in`
/// for the pane.
fn checks_run() -> &'static SimulationRun {
    static RUN: OnceLock<SimulationRun> = OnceLock::new();
    RUN.get_or_init(|| {
        let module = scratch_file("checks.vams", CHECKS);
        // The `$error` and the error-level assertion fail the run's checks;
        // its results are kept whole.
        run_manual_batch_with_lifecycle(
            &format!(
                "* event markers\n.hdl \"{module}\"\nx1 q p checks\nr1 q 0 1meg\n\
                 v1 in 0 PULSE(0 1 5n 1n 1n 20n 40n)\nr2 in 0 1k\n.tran 0.1n 100n\n\
                 .ASSERT plow COND='D(p)==0' LEVEL=ERROR MESSAGE='p must stay low'\n.end\n"
            ),
            SimulationRunLifecycle::Completed,
        )
    })
}

/// A host on `run` with only `in` on the plot.
fn checks_host(run: &SimulationRun) -> ResultsTestHost {
    let mut run = run.clone();
    for analysis in &mut run.analyses {
        for waveform in &mut analysis.waveforms {
            waveform.visible = waveform.name.eq_ignore_ascii_case("in");
        }
    }
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state
}

/// The instant of each severity record the run kept, in emission order,
/// and the instant its finish ended it.
fn record_times(run: &SimulationRun) -> (Vec<f64>, f64) {
    let checks = &run.analyses[0].checks;
    let records = checks
        .hdl
        .iter()
        .map(|record| match record.point {
            HdlPoint::Transient { time } => time,
            other => panic!("a transient record: {other:?}"),
        })
        .collect();
    let finish = checks
        .finish
        .as_ref()
        .expect("the testbench finished the run");
    let rspice_results::engine_evidence::ModelFinishPoint::Transient { time } = finish.point else {
        panic!("a transient finish: {finish:?}");
    };
    (records, time)
}

/// A severity glyph's triangle: where its tip points.
#[derive(Debug, Clone, Copy)]
struct Glyph {
    tip: egui::Pos2,
}

/// The error and warning glyphs the frame painted: triangles 8 points wide
/// and 3.2 + 4 points tall.
fn glyphs(shapes: &[ClippedShape]) -> Vec<Glyph> {
    fn walk(shape: &egui::Shape, out: &mut Vec<Glyph>) {
        match shape {
            egui::Shape::Path(path) if path.closed && path.points.len() == 3 => {
                let xs = path.points.iter().map(|point| point.x);
                let ys = path.points.iter().map(|point| point.y);
                let width = xs.clone().fold(f32::MIN, f32::max) - xs.fold(f32::MAX, f32::min);
                let height = ys.clone().fold(f32::MIN, f32::max) - ys.fold(f32::MAX, f32::min);
                if (width - 8.0).abs() > 0.01 || (height - 7.2).abs() > 0.01 {
                    return;
                }
                let tip = *path
                    .points
                    .iter()
                    .max_by(|a, b| a.y.total_cmp(&b.y))
                    .expect("three points");
                out.push(Glyph { tip });
            }
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, &mut out);
    }
    out.sort_by(|a, b| a.tip.x.total_cmp(&b.tip.x));
    out
}

/// Every run of text the frame painted.
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

/// The panes' time axis, read from where they draw cursors A and B at `ta`
/// and `tb`: the x of any instant.
struct Axis {
    ta: f64,
    tb: f64,
    xa: f32,
    xb: f32,
}

impl Axis {
    fn x(&self, t: f64) -> f32 {
        let fraction = (t - self.ta) / (self.tb - self.ta);
        self.xa + (self.xb - self.xa) * fraction as f32
    }
}

fn axis(ctx: &egui::Context, state: &mut ResultsTestHost, ta: f64, tb: f64) -> Axis {
    place_cursors(state, Some(ta), Some(tb));
    frame(ctx, state, Vec::new());
    let lines = verticals(&frame(ctx, state, Vec::new()));
    let tokens = Tokens::get(ctx);
    let a = cursor_lines(&lines, tokens.color.canvas_cursor_a);
    let b = cursor_lines(&lines, tokens.color.canvas_cursor_b);
    assert!(
        a.len() >= 2 && b.len() >= 2,
        "the cursors cross the strip and a pane: {a:?} {b:?}"
    );
    assert!(
        a[0].bottom - a[0].top < MARKER_STRIP_HEIGHT,
        "the first cursor-A line is the strip's: {a:?}"
    );
    assert!(
        (a[0].x - a[1].x).abs() <= 0.5,
        "cursor A crosses the strip and the pane at one x: {a:?}"
    );
    Axis {
        ta,
        tb,
        xa: a[1].x,
        xb: b[1].x,
    }
}

fn click(at: egui::Pos2) -> [Vec<egui::Event>; 3] {
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    };
    [
        vec![egui::Event::PointerMoved(at)],
        vec![button(true)],
        vec![button(false)],
    ]
}

fn key(key: egui::Key) -> Vec<egui::Event> {
    vec![egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }]
}

/// Clicking a glyph puts cursor A on its record's instant exactly; Right and
/// Left then step along the markers, and so does the gutter's next control.
#[test]
fn a_click_and_the_steps_land_on_marker_instants_exactly() {
    let run = checks_run();
    let (records, finish) = record_times(run);
    let mut state = checks_host(run);
    let ctx = theme_context();
    let axis = axis(&ctx, &mut state, records[0], finish);
    place_cursors(&mut state, None, None);
    frame(&ctx, &mut state, Vec::new());
    let painted = glyphs(&frame(&ctx, &mut state, Vec::new()));
    let error = painted[1];
    for events in click(egui::pos2(error.tip.x, error.tip.y - 4.0)) {
        frame(&ctx, &mut state, events);
    }
    let cursors = |state: &ResultsTestHost| state.results.results.session.cursors;
    assert_eq!(
        cursors(&state).a,
        Some(records[1]),
        "the click lands on the error's instant"
    );
    assert_eq!(
        state.results.results.source_request, None,
        "a click only moves the cursor"
    );
    frame(&ctx, &mut state, key(egui::Key::ArrowRight));
    assert_eq!(
        cursors(&state).a,
        Some(records[2]),
        "Right steps to the next warning"
    );
    frame(&ctx, &mut state, key(egui::Key::ArrowRight));
    assert_eq!(cursors(&state).a, Some(finish), "and then to the finish");
    frame(&ctx, &mut state, key(egui::Key::ArrowLeft));
    frame(&ctx, &mut state, key(egui::Key::ArrowLeft));
    assert_eq!(
        cursors(&state).a,
        Some(records[1]),
        "Left steps back along them"
    );
    // The gutter's controls sit before the time axis: next, then previous
    // before it.
    let next = egui::pos2(axis.x(0.0) - 12.0, error.tip.y - 4.0);
    for events in click(next) {
        frame(&ctx, &mut state, events);
    }
    assert_eq!(
        cursors(&state).a,
        Some(records[2]),
        "the next control steps too"
    );
}

/// Hovering a glyph states the record, its severity, its time and its
/// `file:line`; a double click asks the application to open that line.
#[test]
fn hover_states_the_record_and_a_double_click_asks_for_its_line() {
    let run = checks_run();
    let (records, finish) = record_times(run);
    let mut state = checks_host(run);
    let ctx = theme_context();
    ctx.all_styles_mut(|style| {
        style.interaction.tooltip_delay = 0.0;
        style.interaction.show_tooltips_only_when_still = false;
    });
    axis(&ctx, &mut state, records[0], finish);
    let error = glyphs(&frame(&ctx, &mut state, Vec::new()))[1];
    let at = egui::pos2(error.tip.x, error.tip.y - 4.0);
    frame(&ctx, &mut state, vec![egui::Event::PointerMoved(at)]);
    let painted = texts(&frame(&ctx, &mut state, Vec::new()));
    let tooltip = painted
        .iter()
        .find(|text| text.starts_with("Error at "))
        .unwrap_or_else(|| panic!("the hover states the error: {painted:?}"));
    assert!(
        tooltip.starts_with("Error at 40 ns: count mismatch: 3\n"),
        "{tooltip}"
    );
    assert!(tooltip.contains("checks.vams:11\n"), "{tooltip}");
    assert!(
        tooltip.ends_with("Double-click to open the source"),
        "{tooltip}"
    );
    for _ in 0..2 {
        for events in click(at) {
            frame(&ctx, &mut state, events);
        }
    }
    let request = state.results.results.source_request.clone();
    assert!(
        request
            .as_ref()
            .is_some_and(|SourceLine { file, line }| file.ends_with("checks.vams") && *line == 11),
        "the double click asks for the error's line: {request:?}"
    );
}

/// A vector file whose expected outputs run past the stop time: the strip
/// marks the stop time with how many were not checked.
#[test]
fn samples_after_the_stop_time_are_counted_at_the_stop_time() {
    scratch_file(
        "late.vec",
        "radix 1 1\nvname a y\nio i o\ntunit ns\nvih 1.8\nvil 0\ntrise 1\n\
         0 0 0\n10 1 X\n15 1 1\n25 1 1\n30 1 0\n",
    );
    let run = run_manual_batch_at(
        "* vec late\nr1 a y 1\nr2 y 0 1meg\n.vec 'late.vec'\n.tran 0.1n 20n\n.end\n",
        &scratch().join("late.cir"),
        SimulationRunLifecycle::Completed,
    );
    let mut state = checks_host(&run);
    for waveform in &mut state.model.simulation.retained.runs[0].analyses[0].waveforms {
        waveform.visible = true;
    }
    let models = strip(&mut state);
    let markers = models[0]
        .event_markers
        .as_deref()
        .expect("the late samples are marked");
    let stop = markers.stop.as_ref().expect("the stop time is marked");
    let last = run.analyses[0].waveforms[0].x.ends().unwrap().1;
    assert_eq!((stop.time, stop.beyond), (last, 2));
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let painted = texts(&frame(&ctx, &mut state, Vec::new()));
    assert!(
        painted.iter().any(|text| text == "+2"),
        "the strip counts them: {painted:?}"
    );
}
