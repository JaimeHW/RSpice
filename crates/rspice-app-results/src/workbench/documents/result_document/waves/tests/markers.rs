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
use egui::epaint::{ClippedShape, ColorMode};
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

/// The run's one assertion violation.
fn violation(run: &SimulationRun) -> (f64, f64) {
    let report = &run.analyses[0].checks.assertions[0];
    let [violation] = report.violations.as_slice() else {
        panic!("one violation: {report:?}");
    };
    (
        violation
            .start
            .expect("a transient violation starts at an instant"),
        violation.end.expect("p falls before the run ends"),
    )
}

/// A severity glyph's triangle: where its tip points and how it is inked.
#[derive(Debug, Clone, Copy)]
struct Glyph {
    tip: egui::Pos2,
    filled: bool,
    color: egui::Color32,
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
                let filled = path.fill != egui::Color32::TRANSPARENT;
                let color = match (&path.stroke.color, filled) {
                    (_, true) => path.fill,
                    (ColorMode::Solid(color), false) => *color,
                    (ColorMode::UV(_), false) => egui::Color32::TRANSPARENT,
                };
                out.push(Glyph { tip, filled, color });
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

/// Every filled rectangle of `color` the frame painted.
fn filled_rects(shapes: &[ClippedShape], color: egui::Color32) -> Vec<egui::Rect> {
    fn walk(shape: &egui::Shape, color: egui::Color32, out: &mut Vec<egui::Rect>) {
        match shape {
            egui::Shape::Rect(rect) if rect.fill == color => out.push(rect.rect),
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
/// and `tb`: the x of any instant, and the strip's own cursor-A line.
struct Axis {
    ta: f64,
    tb: f64,
    xa: f32,
    xb: f32,
    strip_a: super::lanes::Vertical,
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
        strip_a: a[0],
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

/// The glyphs stand at their records' exact instants on the panes' own time
/// axis: an outlined warning triangle at each `$warning`, a filled error
/// triangle at the `$error`, the finish flag where `$finish` ended the run.
#[test]
fn glyphs_stand_at_their_records_instants_on_the_panes_axis() {
    let run = checks_run();
    let (records, finish) = record_times(run);
    assert_eq!(records.len(), 3, "two warnings and an error");
    let mut state = checks_host(run);
    let ctx = theme_context();
    let tokens = Tokens::get(&ctx);
    let axis = axis(&ctx, &mut state, records[0], finish);
    let painted = glyphs(&frame(&ctx, &mut state, Vec::new()));
    let kinds: Vec<_> = painted
        .iter()
        .map(|glyph| (glyph.filled, glyph.color))
        .collect();
    assert_eq!(
        kinds,
        [
            (false, tokens.color.warn),
            (true, tokens.color.err),
            (false, tokens.color.warn)
        ],
        "a warning, the error and a warning, left to right: {painted:?}"
    );
    for (glyph, t) in painted.iter().zip(&records) {
        assert!(
            (glyph.tip.x - axis.x(*t)).abs() <= 0.5,
            "the record at {t:e} s is drawn at x {}, the panes put it at {}",
            glyph.tip.x,
            axis.x(*t)
        );
        assert!(
            glyph.tip.y <= axis.strip_a.bottom + 0.5 && glyph.tip.y >= axis.strip_a.top,
            "the glyph sits in the strip: {glyph:?} {:?}",
            axis.strip_a
        );
    }
    // The finish flag's staff is a vertical of the dim ink at its instant.
    let staff = verticals(&frame(&ctx, &mut state, Vec::new()))
        .into_iter()
        .find(|line| {
            line.color == tokens.color.canvas_text_dim
                && (line.bottom - line.top - 8.0).abs() < 0.01
        })
        .expect("the finish flag's staff");
    assert!(
        (staff.x - axis.x(finish)).abs() <= 0.5,
        "the finish at {finish:e} s is drawn at x {}, the panes put it at {}",
        staff.x,
        axis.x(finish)
    );
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

/// A run with no marker has no strip: its panes start where they would, and
/// with the same run's checks they move down by exactly the strip.
#[test]
fn a_run_without_markers_gives_the_strip_no_room() {
    let run = checks_run();
    let (records, finish) = record_times(run);
    let mut plain = run.clone();
    plain.analyses[0].checks = rspice_results::analysis_result::RunChecks::default();
    let ctx = theme_context();
    let tokens = Tokens::get(&ctx);
    let top_cursor = |state: &mut ResultsTestHost| {
        place_cursors(state, Some(records[0]), Some(finish));
        frame(&ctx, state, Vec::new());
        let shapes = frame(&ctx, state, Vec::new());
        (
            cursor_lines(&verticals(&shapes), tokens.color.canvas_cursor_a),
            glyphs(&shapes).len(),
        )
    };
    let mut without = checks_host(&plain);
    assert!(strip(&mut without)[0].event_markers.is_none());
    let (lines, glyph_count) = top_cursor(&mut without);
    assert_eq!(glyph_count, 0, "no glyphs");
    let mut with = checks_host(run);
    let (marked, _) = top_cursor(&mut with);
    assert_eq!(
        marked.len(),
        lines.len() + 1,
        "only the strip adds a cursor line: {marked:?} {lines:?}"
    );
    assert_eq!(
        marked[1].top - lines[0].top,
        MARKER_STRIP_HEIGHT,
        "the strip takes its own height and no more"
    );
}

/// The assertion's violation is shaded in the strip and on the lane of the
/// net it reads, from its first instant to its last, on the panes' axis.
#[test]
fn an_assertion_violation_shades_the_strip_and_its_nets_lane() {
    let run = checks_run();
    let (start, end) = violation(run);
    let mut state = checks_host(run);
    let models = strip(&mut state);
    let p = models[0]
        .lane_traces
        .iter()
        .find(|lane| lane.net.eq_ignore_ascii_case("p"))
        .map(|lane| lane.source_waveform_name.clone())
        .expect("p is kept as event history");
    let key = models[0].analysis_key;
    set_event_net_plotted(&mut state.results_parts(), key, &p, true);
    let ctx = theme_context();
    let tokens = Tokens::get(&ctx);
    let axis = axis(&ctx, &mut state, start, end);
    let shade = tokens.color.err.gamma_multiply(0.28);
    let spans = filled_rects(&frame(&ctx, &mut state, Vec::new()), shade);
    let heights: Vec<f32> = spans.iter().map(|span| span.height()).collect();
    assert_eq!(
        heights,
        [
            MARKER_STRIP_HEIGHT - 1.0,
            rspice_results_ui::waves::logic::LOGIC_ROW_HEIGHT
        ],
        "one span in the strip and one on p's lane: {spans:?}"
    );
    for span in &spans {
        assert!(
            (span.left() - axis.x(start)).abs() <= 0.5 && (span.right() - axis.x(end)).abs() <= 0.5,
            "the violation from {start:e} s to {end:e} s is shaded over {span:?}; the panes put \
             it at {} to {}",
            axis.x(start),
            axis.x(end)
        );
    }
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
    let last = *run.analyses[0].waveforms[0].x.last().unwrap();
    assert_eq!((stop.time, stop.beyond), (last, 2));
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let painted = texts(&frame(&ctx, &mut state, Vec::new()));
    assert!(
        painted.iter().any(|text| text == "+2"),
        "the strip counts them: {painted:?}"
    );
}

/// One frame of the Waves sheet in its document well, at twice the points'
/// resolution, with the tooltip delay cleared so a parked pointer's hover is
/// part of the picture.
fn picture(
    state: &mut ResultsTestHost,
    pointer: Option<egui::Pos2>,
) -> rspice_ui_kit::raster::Canvas {
    let ctx = theme_context();
    ctx.all_styles_mut(|style| {
        style.interaction.tooltip_delay = 0.0;
        style.interaction.show_tooltips_only_when_still = false;
    });
    let mut output = None;
    // Enough passes for a tooltip to finish fading in.
    for pass in 0..24 {
        let mut input = rspice_ui_kit::raster::display_input(super::lanes::FRAME, 2.0);
        if pass >= 2
            && let Some(pointer) = pointer
        {
            input.events.push(egui::Event::PointerMoved(pointer));
        }
        output = Some(ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                super::lanes::well(ui);
                show(ui, &mut state.results_parts());
            });
        }));
    }
    rspice_ui_kit::raster::rasterize(&ctx, super::lanes::FRAME, &output.expect("a frame"))
}

/// The pictures for review, written under `RSPICE_PICTURES` when it is set:
/// the testbench's strip with its warning, error and finish glyphs and the
/// assertion's span on `p`'s lane, each marker hovered, and the vector run's
/// stop time.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_marker_pictures() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    // Each picture whole, and the strip's band with its hover at full size.
    let write = |name: &str, canvas: rspice_ui_kit::raster::Canvas| {
        for (suffix, height) in [("", canvas.height()), ("-top", canvas.height().min(560))] {
            std::fs::write(
                format!("{directory}/{name}{suffix}.png"),
                canvas.png(height),
            )
            .expect("the picture is written");
        }
    };
    let run = checks_run();
    let (records, finish) = record_times(run);
    let (start, _) = violation(run);
    let mut state = checks_host(run);
    let models = strip(&mut state);
    let key = models[0].analysis_key;
    let p = models[0]
        .lane_traces
        .iter()
        .find(|lane| lane.net.eq_ignore_ascii_case("p"))
        .map(|lane| lane.source_waveform_name.clone())
        .expect("p is kept as event history");
    set_event_net_plotted(&mut state.results_parts(), key, &p, true);
    let ctx = theme_context();
    let marks = axis(&ctx, &mut state, records[0], finish);
    let centre_y = (marks.strip_a.top + marks.strip_a.bottom) * 0.5;
    place_cursors(&mut state, Some(records[1]), None);
    write("marks-strip", picture(&mut state, None));
    for (name, t) in [
        ("marks-hover-error", records[1]),
        ("marks-hover-warning", records[2]),
        ("marks-hover-finish", finish),
        ("marks-hover-span", 0.5 * (start + records[1])),
    ] {
        write(
            name,
            picture(&mut state, Some(egui::pos2(marks.x(t), centre_y))),
        );
    }

    scratch_file(
        "late.vec",
        "radix 1 1\nvname a y\nio i o\ntunit ns\nvih 1.8\nvil 0\ntrise 1\n\
         0 0 0\n10 1 X\n15 1 1\n25 1 1\n30 1 0\n",
    );
    let late = run_manual_batch_at(
        "* vec late\nr1 a y 1\nr2 y 0 1meg\n.vec 'late.vec'\n.tran 0.1n 20n\n.end\n",
        &scratch().join("late.cir"),
        SimulationRunLifecycle::Completed,
    );
    let mut state = checks_host(&late);
    for waveform in &mut state.model.simulation.retained.runs[0].analyses[0].waveforms {
        waveform.visible = true;
    }
    let stop = *late.analyses[0].waveforms[0].x.last().unwrap();
    let late_axis = axis(&ctx, &mut state, 0.25 * stop, 0.75 * stop);
    place_cursors(&mut state, None, None);
    let centre_y = (late_axis.strip_a.top + late_axis.strip_a.bottom) * 0.5;
    write(
        "marks-stop",
        picture(&mut state, Some(egui::pos2(late_axis.x(stop), centre_y))),
    );
}

/// The picture of a corpus design, written under `RSPICE_PICTURES`: the C12
/// testbench's failing variant flags fifteen trim codes with `$error` and
/// `$finish`es at the end, with `ok` and `csn` on lanes.
#[test]
#[ignore = "writes a review picture from a long run"]
fn write_the_c12_picture() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let deck = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus/c12-ldo-tb/deck/fail.tran.cir");
    let source = std::fs::read_to_string(&deck).expect("the corpus deck is readable");
    let run = run_manual_batch_at(&source, &deck, SimulationRunLifecycle::Completed);
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    let models = strip(&mut state);
    let key = models[0].analysis_key;
    let lanes: Vec<String> = models[0]
        .lane_traces
        .iter()
        .filter(|lane| {
            ["ok", "csn"]
                .iter()
                .any(|net| lane.net.eq_ignore_ascii_case(net))
        })
        .map(|lane| lane.source_waveform_name.clone())
        .collect();
    for lane in &lanes {
        set_event_net_plotted(&mut state.results_parts(), key, lane, true);
    }
    let canvas = picture(&mut state, None);
    for (suffix, height) in [("", canvas.height()), ("-top", canvas.height().min(560))] {
        std::fs::write(
            format!("{directory}/marks-c12-fail{suffix}.png"),
            canvas.png(height),
        )
        .expect("the picture is written");
    }
}
