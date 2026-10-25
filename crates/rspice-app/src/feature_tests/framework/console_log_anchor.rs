//! Console rows anchored to a run's log: a double click marks what the run
//! named, an anchor that no longer resolves renders inert and says why, and a
//! run line refuses once its run is gone.

use crate::diagnostics::LogAnchor;
use crate::diagnostics::LogSource;
use crate::state::SimulationRun;
use crate::workbench::docks::console::log_anchor::*;
use crate::workbench::docks::console::transcript::console;
use crate::workbench::state::{ConsoleChannel, ConsoleView};
use crate::workbench::{AppState, RSpiceApp};
use egui::Vec2;

/// Where the console painted the row whose message contains `needle`.
///
/// Rows are painted galleys rather than widgets, so there is no widget id
/// to look a rect up by. The text the row drew is the only handle a test
/// has on it, which is the same handle a reader has.
fn console_row_position(ctx: &egui::Context, state: &mut AppState, needle: &str) -> egui::Pos2 {
    fn scan(shape: &egui::epaint::Shape, needle: &str, found: &mut Option<egui::Pos2>) {
        match shape {
            egui::epaint::Shape::Text(text) if found.is_none() => {
                if text.galley.job.text.contains(needle) {
                    *found = Some(text.pos + text.galley.size() * 0.5);
                }
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    scan(shape, needle, found);
                }
            }
            _ => {}
        }
    }

    let output = ctx.run_ui(console_input(Vec::new()), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
    });
    let mut found = None;
    for clipped in &output.shapes {
        scan(&clipped.shape, needle, &mut found);
    }
    found.unwrap_or_else(|| panic!("the console painted no row containing {needle:?}"))
}

fn console_context_for_tests() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

fn console_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(900.0, 600.0),
        )),
        events,
        ..Default::default()
    }
}

fn press(position: egui::Pos2) -> Vec<egui::Event> {
    let button = |pressed| egui::Event::PointerButton {
        pos: position,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    };
    vec![
        egui::Event::PointerMoved(position),
        button(true),
        button(false),
    ]
}

/// Double-click the console row whose message contains `needle`.
///
/// egui resolves a press against the widget rects the previous frame
/// registered, so the rows are laid out first; the two presses then land
/// in successive frames, inside the double-click interval.
fn double_click_console_row(state: &mut AppState, needle: &str) {
    let ctx = console_context_for_tests();
    let position = console_row_position(&ctx, state, needle);
    for _ in 0..2 {
        let _ = ctx.run_ui(console_input(press(position)), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
        });
    }
}

/// A sheet drawing the conductor `OUT`, with a cross-probe map captured at
/// the topology it is currently drawn at.
fn state_with_probed_conductor() -> AppState {
    use crate::state::{Point, Wire};
    use std::collections::HashMap;

    let mut state = AppState::default();
    let a = Point::new(0, 0);
    let b = Point::new(40, 0);
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::new(91, vec![a, b]));
    state.model.simulation.source.cross_probe.update(
        state.model.workspace.content.active_view.clone(),
        HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
        HashMap::from([("OUT".to_owned(), vec![a, b])]),
        HashMap::new(),
        state.model.schematic.topology_version(),
    );
    state.model.log_buffer.clear();
    state
}

/// The whole point of an anchored line: the objects a failed run named
/// are a double-click from the sentence that named them.
#[test]
fn double_clicking_an_anchored_console_row_marks_what_the_run_named() {
    let mut state = state_with_probed_conductor();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    state.model.log_buffer.log_anchored(
        crate::diagnostics::LogSeverity::Error,
        LogSource::Run,
        "Analysis failed: no DC path at OUT",
        Some(LogAnchor::Simulation {
            nets: vec!["OUT".to_owned()],
            devices: Vec::new(),
        }),
    );

    double_click_console_row(&mut state, "no DC path at OUT");

    assert!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .contains(&91),
        "the row's anchor must mark the conductor the run named"
    );
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Design,
        "and take the reader to the drawing it marked it on"
    );
}

/// A name the drawing no longer carries must not look like a jump. The
/// row stays put, says why on hover, and navigates nowhere when followed.
#[test]
fn an_unresolvable_anchor_renders_inert_and_says_why() {
    let mut state = state_with_probed_conductor();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    let anchor = LogAnchor::Simulation {
        nets: vec!["DELETED".to_owned()],
        devices: Vec::new(),
    };
    state.model.log_buffer.log_anchored(
        crate::diagnostics::LogSeverity::Error,
        LogSource::Run,
        "Analysis failed: no DC path at DELETED",
        Some(anchor.clone()),
    );

    let refusal = state
        .log_anchor_refusal(&anchor)
        .expect("a name this sheet does not draw cannot be jumped to");
    assert!(
        refusal.contains("DELETED"),
        "the tooltip must name what it could not find: {refusal}"
    );

    double_click_console_row(&mut state, "no DC path at DELETED");

    assert!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty(),
        "an inert row must mark nothing"
    );
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Results,
        "and must not navigate away from what the reader was looking at"
    );
    assert!(
        state
            .model
            .log_buffer
            .entries()
            .all(|entry| !entry.message.contains("Marked")),
        "nor report a marking it did not perform"
    );
}

/// A run's line opens that run's log on the page, and is inert once the
/// run is no longer held rather than opening an empty channel.
#[test]
fn a_run_line_opens_its_log_and_refuses_once_the_run_is_gone() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.log_buffer.clear();
    app.state
        .model
        .simulation
        .retained
        .runs
        .push(SimulationRun::new(7));
    app.state.model.log_buffer.log_anchored(
        crate::diagnostics::LogSeverity::Info,
        LogSource::Run,
        "Run 7 started: Transient",
        Some(LogAnchor::ResultRun { run_sequence: 7 }),
    );

    let workspace = app.state.workbench.workspace;
    double_click_console_row(&mut app.state, "Run 7 started");
    assert_eq!(
        app.state.workbench.console_view.channel,
        ConsoleChannel::Run(7),
        "the run's line opens the run's own log"
    );
    assert_eq!(
        app.state.workbench.workspace, workspace,
        "on the page, without leaving the workspace"
    );

    app.state
        .workbench
        .console_view
        .open(ConsoleChannel::Session);
    app.state.model.simulation.retained.runs.clear();
    let anchor = LogAnchor::ResultRun { run_sequence: 7 };
    let refusal = app
        .state
        .log_anchor_refusal(&anchor)
        .expect("a run this session dropped cannot be opened");
    assert!(refusal.contains("Run 7"), "{refusal}");
    let mut view = ConsoleView::default();
    let mut jump = None;
    assert!(!follow(&app.state, &mut view, &anchor, &mut jump));
    assert_eq!(view.channel, ConsoleChannel::Session);
    assert!(jump.is_none());
}
