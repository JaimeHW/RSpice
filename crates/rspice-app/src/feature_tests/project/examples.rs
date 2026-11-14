//! The examples browser through the whole application: every example opens
//! as a project that runs, unsaved work is asked about first, and Open hands
//! back the example it selected.

use egui::{Context, vec2};
use rspice_app_state::workbench::examples::{EXAMPLES, Example, load_example};
use std::time::{Duration, Instant};

use crate::state::AnalysisType;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::session::project_transactions::create_example_project;
use crate::workbench::session::project_transactions::request_example;
use crate::workbench::simulate::commands::SimulateCommand;
use crate::workbench::tools::examples_browser::{open_refusal, show};

/// Every example opens as a project whose default plan the engine
/// prepares and authorizes, through the path the toolbar's Run takes:
/// netlisted, its models bound, its decks built and checked, and the run
/// queued. Running each to its end is left to the test below: in an
/// unoptimized build that takes from seconds to minutes per example.
#[test]
fn every_example_opens_as_a_project_whose_default_analysis_is_authorized() {
    for example in EXAMPLES {
        let app = open_and_press_run(example, "10n");
        let report = app
            .state
            .runs
            .preflight
            .report
            .as_ref()
            .expect("Run retains its report");
        assert!(
            report.prepared.is_some(),
            "{}: the engine prepared the run",
            example.name
        );
        assert!(
            app.state.model.simulation.execution.trigger_simulation,
            "{}: the authorized run is queued",
            example.name
        );
    }
}

/// An example opened as a project runs to its transient result.
#[test]
fn an_example_project_runs_to_its_result() {
    let example = EXAMPLES[0];
    let mut app = open_and_press_run(example, "10u");
    // What the frame loop does each frame while the run is dispatched
    // and collected.
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        app.simulation_controller
            .update(&mut app.state, &*app.export_workflow_io);
        let execution = &app.state.model.simulation.execution;
        if !execution.has_active_execution() && !execution.trigger_simulation {
            break;
        }
        assert!(Instant::now() < deadline, "{} timed out", example.name);
        std::thread::sleep(Duration::from_millis(5));
    }
    let run = app
        .state
        .model
        .simulation
        .retained
        .runs
        .last()
        .expect("the run is retained");
    assert!(
        run.analyses.iter().any(|analysis| {
            analysis.analysis_type == AnalysisType::Transient
                && analysis.success
                && !analysis.waveforms.is_empty()
        }),
        "{}: {:?}",
        example.name,
        run.analyses
            .iter()
            .map(|analysis| &analysis.error_message)
            .collect::<Vec<_>>()
    );
}

/// Open `example` as a project and press Run, as the toolbar does, with
/// its transient cut to `stop`; preflight has settled on return.
fn open_and_press_run(example: Example, stop: &str) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    create_example_project(&mut app.state, example).expect("the example opens as a project");
    assert_eq!(
        app.state.model.workspace.content.project.display_name(),
        example.name
    );
    assert!(
        !app.state.model.schematic.document().components.is_empty(),
        "{} draws its circuit",
        example.name
    );
    // The default plan runs a millisecond at 10 ns steps; the engine
    // checks the same plan over a shorter window.
    app.state.model.sim_setup.tran.step = "1n".to_owned();
    app.state.model.sim_setup.tran.stop = stop.to_owned();
    // The frame after the project opens, on which the controller lets go
    // of the design it replaced; a check started before it would be
    // dropped with that design.
    app.simulation_controller
        .update(&mut app.state, &*app.export_workflow_io);
    assert!(app.command_key_enabled(SimulateCommand::Run.key()));
    app.execute_command_key(SimulateCommand::Run.key());
    assert!(app.state.runs.preflight.take_run_and_queue_request());
    crate::workbench::preflight::start(&mut app.state, &mut app.simulation_controller, true);
    app.settle_preflight();
    let blockers = app
        .state
        .runs
        .preflight
        .report
        .as_ref()
        .map(|report| report.blockers.clone())
        .unwrap_or_default();
    assert!(
        blockers.is_empty(),
        "{} is blocked: {blockers:#?}",
        example.name
    );
    app
}

/// Open stays offered while the open project has unsaved changes, and
/// asks about them, as New project does, instead of replacing them.
#[test]
fn an_example_asks_to_save_unsaved_work_before_replacing_it() {
    let mut state = AppState::default();
    state.open_project_session();
    load_example("Voltage Divider", &mut state.model.schematic);
    state.sync_active_schematic_to_workspace();
    assert!(open_refusal(&state.project, &state.model, &state.platform).is_none());
    let components = state.model.schematic.document().components.len();
    request_example(&mut state, EXAMPLES[0]);
    let confirmation = &state.workbench.confirmation_dialog;
    assert!(confirmation.visible);
    assert_eq!(
        confirmation.pending_action,
        Some(crate::workbench::app::ConfirmationAction::ProjectNew)
    );
    assert_eq!(
        confirmation.pending_example.as_deref(),
        Some(EXAMPLES[0].name)
    );
    assert_eq!(
        state.model.schematic.document().components.len(),
        components
    );
}

/// Open hands the shell the example the reader selected, in the frame it
/// was pressed in, and the dialog closes; the dialog itself replaces
/// nothing.
#[test]
fn open_hands_back_the_selected_example_and_closes_the_dialog() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.project.examples_selection = Some(1);
    let components = app.state.model.schematic.document().components.len();
    let frame = |app: &mut RSpiceApp, events: Vec<egui::Event>| {
        let mut asked = None;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(1440.0, 900.0),
                )),
                events,
                ..Default::default()
            },
            |ui| asked = show(ui.ctx(), &mut crate::workbench::features::cx(app)),
        );
        (output, asked)
    };
    let mut settled = frame(&mut app, Vec::new());
    for _ in 0..2 {
        settled = frame(&mut app, Vec::new());
    }
    assert_eq!(settled.1, None);
    let bounds = settled
        .0
        .platform_output
        .accesskit_update
        .expect("the settled frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label()? == "Open").then_some(node.bounds()?))
        .expect("the dialog announces Open");
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    };
    let (_, asked) = frame(
        &mut app,
        vec![egui::Event::PointerMoved(at), press(true), press(false)],
    );

    assert_eq!(asked, Some(EXAMPLES[1]));
    assert_eq!(app.state.project.examples_selection, None);
    assert_eq!(
        app.state.model.schematic.document().components.len(),
        components
    );
}
