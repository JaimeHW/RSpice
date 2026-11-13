//! The Results cursor commands, run through the feature's registry against a
//! damped sine the whole Results frame has drawn: each jump lands on its
//! feature of the trace, the readout states it, and the commands refuse
//! outside a waveform sheet.

use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
use crate::workbench::documents::result_document::{self, ResultsParts, ResultsTestHost};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::results::ResultsFeature;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::state::{WorkbenchSession, Workspace};
use egui::epaint::ClippedShape;
use rspice_results::calculator::functions::FunctionRegistry;
use rspice_results::calculator::value::RealValue;
use std::f64::consts::PI;

/// `exp(-t/TAU) sin(2 pi F t)` over five periods.
const F: f64 = 1.0e3;
const TAU: f64 = 2.0e-3;
const SAMPLES: usize = 4001;
const STOP: f64 = 5.0e-3;
const DT: f64 = STOP / (SAMPLES - 1) as f64;
const FRAME: egui::Vec2 = egui::vec2(1200.0, 760.0);

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

fn theme_context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// One frame of the whole Results centre view.
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
        egui::CentralPanel::default().show(ctx, |ui| {
            result_document::show(ui, parts, &mut controller, None);
        });
    })
    .shapes
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
