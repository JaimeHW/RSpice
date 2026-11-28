//! The commit rule on the Analyses page, in whole frames of the application.
//! A text field holds what is typed while it has the keyboard and commits
//! when it lets go of it. Run commits it first: with the caret still in a
//! field, Run runs what the field shows, and text there that does not read
//! starts nothing and stays in its field with the reason. A value typed and
//! left is one step of Undo, however many keystrokes it took.

use std::time::{Duration, Instant};

use egui::accesskit::{Role, TreeUpdate};
use egui::{Event, Key, Modifiers, Pos2, vec2};
use rspice_app_state::simulation::plan::AnalysisDraft;
use rspice_app_state::state::{ComponentType, Point, SimulationPage, SimulationRun, Wire};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_results::run::SimulationRunLifecycle;
use rspice_simulation_contract::run_set::RunSetDimensionKind;

use crate::feature_tests::replay::{Control, PRIMARY, Replay, controls, only};
use crate::workbench::RSpiceApp;

/// What the fixture's transient stops at before anything is typed.
const PLANNED_STOP: &str = "1u";

/// A source across a resistor to ground, drawn in the open root and run at
/// one point, on the Analyses page with the plan's transient selected.
fn analyses_page() -> Replay {
    let mut replay = Replay::of(RSpiceApp::test_instance()).sized(vec2(1600.0, 1800.0));
    let state = &mut replay.app.state;
    for (kind, at) in [
        (ComponentType::Ground, Point::new(0, 40)),
        (ComponentType::VoltageSource, Point::new(0, 10)),
        (ComponentType::Resistor, Point::new(100, -10)),
    ] {
        state.model.schematic.add_component(kind, at);
    }
    let document = state.model.schematic.document_mut_for_test();
    for (id, (from, to)) in (1001..).zip([
        (Point::new(0, -10), Point::new(80, -10)),
        (Point::new(120, -10), Point::new(120, 30)),
        (Point::new(120, 30), Point::new(0, 30)),
    ]) {
        document.wires.push(Wire::segment(id, from, to));
    }
    for dimension in &mut state.model.sim_setup.run_set.dimensions {
        if dimension.kind == RunSetDimensionKind::ProcessSection {
            dimension.enabled = false;
        }
    }
    // Set up as a reader leaves a plan, so Run checks the design rather than
    // first choosing the analysis from the drawing's sources.
    let plan = state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("a default state owns a stable plan");
    let transient = plan.instances()[0].id();
    plan.edit(transient, |draft| {
        if let AnalysisDraft::Transient(setup) = draft {
            PLANNED_STOP.clone_into(&mut setup.stop);
        }
    })
    .expect("the transient takes its stop time");
    state.model.sim_setup.refresh_legacy_analysis_projections();
    state.workbench.activate(Workspace::Simulate);
    state.simulate.simulation_page = SimulationPage::Analyses;
    state.simulate.active_analysis_instance = Some(transient);
    replay.frames(Vec::new());
    replay
}

/// The transient's stop as the plan holds it.
fn planned_stop(app: &RSpiceApp) -> String {
    app.state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the plan resolves")
        .instances()
        .iter()
        .find_map(|instance| match instance.draft() {
            AnalysisDraft::Transient(setup) => Some(setup.stop.clone()),
            _ => None,
        })
        .expect("the plan holds the transient")
}

/// What the plan's transient states for TRTOL.
fn planned_trtol(app: &RSpiceApp) -> Option<String> {
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the plan resolves");
    plan.instances()
        .iter()
        .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))?
        .numeric_override()?
        .entries()
        .into_iter()
        .find_map(|(option, value)| (option.key() == "TRTOL").then_some(value))
}

/// A press or release of the primary button at `at`.
fn button(at: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    }
}

/// A key pressed and released with `modifiers`.
fn stroke(key: Key, modifiers: Modifiers) -> Vec<Event> {
    [true, false]
        .map(|pressed| Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat: false,
            modifiers,
        })
        .into()
}

/// The one text field whose label `names`, as a fresh frame draws it.
fn find_field(replay: &mut Replay, names: impl Fn(&str) -> bool) -> Control {
    let shown = replay.frames(Vec::new());
    let found: Vec<_> = shown
        .iter()
        .filter(|control| control.role == Role::TextInput && names(&control.label))
        .collect();
    match found.as_slice() {
        [field] => (*field).clone(),
        _ => panic!(
            "expected one such field, found {:?}",
            found.iter().map(|field| &field.label).collect::<Vec<_>>()
        ),
    }
}

/// The field labelled `label` pressed, its text selected and replaced by
/// `text` one keystroke a frame, as a hand types it, and the caret left in
/// it.
fn type_into(replay: &mut Replay, label: &str, text: &str) {
    replay.click(label, Role::TextInput);
    replay.frame(stroke(Key::A, PRIMARY));
    for typed in text.chars() {
        replay.frame(vec![Event::Text(typed.to_string())]);
    }
    replay.frames(Vec::new());
}

/// The label of the control holding the keyboard in `tree`.
fn focused(tree: &TreeUpdate) -> Option<String> {
    tree.nodes
        .iter()
        .find(|(id, _)| *id == tree.focus)
        .and_then(|(_, node)| node.label().map(str::to_owned))
}

/// The field labelled `label` as a fresh frame draws it, and the label of
/// the control holding the keyboard.
fn field_and_focus(replay: &mut Replay, label: &str) -> (Control, Option<String>) {
    replay.frame(Vec::new());
    let tree = replay
        .frame(Vec::new())
        .platform_output
        .accesskit_update
        .expect("a frame with AccessKit on publishes its tree");
    (
        only(&controls(&tree), label, Role::TextInput).clone(),
        focused(&tree),
    )
}

/// The toolbar's Run control pressed and released.
fn press_run_control(replay: &mut Replay) {
    let shown = replay.frames(Vec::new());
    let run = shown
        .iter()
        .find(|control| {
            control.role == Role::Button && control.label.starts_with("Run active simulation plan")
        })
        .unwrap_or_else(|| panic!("the toolbar draws the Run control"));
    let at = run.center;
    replay.frames(vec![Event::PointerMoved(at), button(at, true)]);
    replay.frames(vec![button(at, false)]);
}

/// Frames until the run Run started has ended; the run.
fn run_to_its_end(replay: &mut Replay) -> SimulationRun {
    let started = Instant::now();
    loop {
        replay.frame(Vec::new());
        if let Some(run) = replay.app.state.model.simulation.retained.runs.first()
            && !matches!(
                run.lifecycle,
                SimulationRunLifecycle::Preparing | SimulationRunLifecycle::Running
            )
        {
            assert_eq!(
                run.lifecycle,
                SimulationRunLifecycle::Completed,
                "{:?}",
                run.log
            );
            return run.clone();
        }
        assert!(
            started.elapsed() < Duration::from_secs(300),
            "no run ended: {:?}",
            replay.app.state.runs.preflight.report
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// The last time point the run's transient reached, in seconds.
fn ran_to(run: &SimulationRun) -> f64 {
    run.analyses
        .iter()
        .find_map(|analysis| analysis.waveforms.first())
        .and_then(|waveform| waveform.x.ends())
        .map(|(_, last)| last)
        .expect("the transient kept its time axis")
}

/// Whether anything was asked to run, is running or has run.
fn anything_ran(app: &RSpiceApp) -> bool {
    let simulation = &app.state.model.simulation;
    simulation.execution.trigger_simulation
        || simulation.execution.has_active_execution()
        || !simulation.retained.runs.is_empty()
        || app.state.runs.preflight.checking.is_some()
}

#[test]
fn f5_with_the_caret_in_stop_time_runs_what_the_field_shows() {
    let mut replay = analyses_page();
    type_into(&mut replay, "Stop time", "100n");

    replay.key(Key::F5, Modifiers::NONE);

    let run = run_to_its_end(&mut replay);
    assert!(
        (ran_to(&run) - 100e-9).abs() < 1e-15,
        "the run stopped at {}",
        ran_to(&run)
    );
    assert_eq!(planned_stop(&replay.app), "100n");
}

#[test]
fn the_run_control_with_the_caret_in_stop_time_runs_what_the_field_shows() {
    let mut replay = analyses_page();
    type_into(&mut replay, "Stop time", "100n");

    press_run_control(&mut replay);

    let run = run_to_its_end(&mut replay);
    assert!(
        (ran_to(&run) - 100e-9).abs() < 1e-15,
        "the run stopped at {}",
        ran_to(&run)
    );
}

/// `10q` names no suffix, so Run commits nothing and starts nothing: the
/// field keeps the text, states why it does not read and has the keyboard
/// back.
#[test]
fn run_with_text_that_does_not_read_starts_nothing_and_keeps_the_text() {
    let mut replay = analyses_page();
    type_into(&mut replay, "Stop time", "10q");

    replay.key(Key::F5, Modifiers::NONE);

    assert!(!anything_ran(&replay.app), "nothing starts");
    assert_eq!(planned_stop(&replay.app), PLANNED_STOP, "nothing commits");
    let (field, focus) = field_and_focus(&mut replay, "Stop time");
    assert_eq!(field.value, "10q", "the field keeps the text");
    assert!(
        field.description.starts_with("Stop time:") && field.description.contains('q'),
        "the field states why: {:?}",
        field.description
    );
    assert_eq!(
        focus.as_deref(),
        Some("Stop time"),
        "the field has the keyboard"
    );

    press_run_control(&mut replay);
    assert!(!anything_ran(&replay.app), "the Run control starts nothing");
    assert_eq!(planned_stop(&replay.app), PLANNED_STOP);
}

#[test]
fn a_value_typed_a_keystroke_at_a_time_and_left_is_one_step_of_undo() {
    let mut replay = analyses_page();
    type_into(&mut replay, "Stop time", "100n");
    replay.key(Key::Tab, Modifiers::NONE);
    assert_eq!(planned_stop(&replay.app), "100n");

    replay.app.execute_command_key(ids::UNDO);
    replay.frames(Vec::new());

    assert_eq!(
        planned_stop(&replay.app),
        PLANNED_STOP,
        "one Undo takes the whole value back, not its last keystroke"
    );
}

/// An option typed and not left is committed before the run F5 starts, and
/// by a keyboard trip to another workspace, and is still there on return.
#[test]
fn an_option_typed_and_not_left_is_committed_by_run_and_by_a_workspace_switch() {
    let mut replay = analyses_page();
    let option = find_field(&mut replay, |label| label.ends_with("TRTOL")).label;
    type_into(&mut replay, &option, "3");
    assert_eq!(planned_trtol(&replay.app), None, "typing commits nothing");

    replay.key(Key::F5, Modifiers::NONE);

    assert_eq!(
        planned_trtol(&replay.app).as_deref(),
        Some("3"),
        "Run commits the option first"
    );
    run_to_its_end(&mut replay);

    let mut replay = analyses_page();
    type_into(&mut replay, &option, "5");
    replay.app.execute_command_key(ids::DESIGN_WORKSPACE);
    replay.frames(Vec::new());
    assert_eq!(replay.app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        planned_trtol(&replay.app).as_deref(),
        Some("5"),
        "leaving the workspace commits the option"
    );
    replay.app.execute_command_key(ids::SIMULATE_WORKSPACE);
    let shown = replay.frames(Vec::new());
    assert_eq!(
        only(&shown, &option, Role::TextInput).value,
        "5",
        "the option keeps what was typed"
    );
}

/// What the Console holds from the plan, oldest first.
fn plan_lines(app: &RSpiceApp) -> Vec<String> {
    app.state
        .model
        .log_buffer
        .entries()
        .filter(|entry| entry.source == rspice_app_state::diagnostics::LogSource::Plan)
        .map(|entry| entry.message.clone())
        .collect()
}

/// A value typed and left is one line in the Console, in plain words: the
/// analysis, the field and the value, with no receipt number, revision or
/// identity in it. An option is logged the same way.
#[test]
fn a_value_left_is_one_plain_line_in_the_console() {
    let mut replay = analyses_page();
    let before = plan_lines(&replay.app);
    type_into(&mut replay, "Stop time", "100n");
    replay.key(Key::Tab, Modifiers::NONE);

    let lines = plan_lines(&replay.app);
    assert_eq!(
        lines[before.len()..],
        ["Transient: stop time 100n".to_owned()],
        "one edit is one line"
    );

    let option = find_field(&mut replay, |label| label.ends_with("TRTOL")).label;
    type_into(&mut replay, &option, "3");
    replay.key(Key::Tab, Modifiers::NONE);
    assert_eq!(
        plan_lines(&replay.app).last().map(String::as_str),
        Some("Transient: TRTOL 3")
    );
}
