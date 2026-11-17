//! An idle frame asks whether the project holds unsaved work and captures
//! nothing: the answer is held for the working state until it changes.

use super::*;
use crate::workbench::RSpiceApp;
use crate::workbench::app::FrameSimulationInput;
use rspice_app_state::workbench::workspace::Workspace;

/// One whole application frame at 1600 x 900, as the shell paints it.
fn frame(ctx: &egui::Context, app: &mut RSpiceApp) {
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600.0, 900.0),
            )),
            ..Default::default()
        },
        |ui| {
            let ctx = ui.ctx().clone();
            let before = FrameSimulationInput::capture(&app.state);
            app.prepare_frame(&ctx);
            app.pump_live_session();
            app.render_frame_chrome(ui);
            app.render_secondary_application_windows(&ctx);
            let _ = app.state.refresh_live_checks(std::time::Instant::now());
            app.render_frame_dialogs(&ctx);
            app.end_frame(&before);
        },
    );
}

fn captures() -> usize {
    rspice_project::registry::PROJECT_CAPTURES.with(std::cell::Cell::get)
}

fn stamps(app: &RSpiceApp) -> String {
    let model = &app.state.model;
    format!(
        "workspace {} schematic {} libraries {} plan {} models {:?}",
        model.workspace.content.stamp(),
        model.schematic.editor_ref().design.content_stamp(),
        model.library_manager.content_stamp(),
        model.sim_setup.setup.content_stamp(),
        model.model_library_manager.content_stamp(),
    )
}

/// A saved project with a resistor on its sheet.
fn saved_project() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(2, 4));
    app.state.sync_active_schematic_to_workspace();
    let baseline = snapshot(&app.state).unwrap();
    app.state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    app
}

#[test]
fn idle_frames_in_every_workspace_capture_nothing() {
    let mut app = saved_project();
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    for workspace in Workspace::ALL {
        app.state.workbench.activate(workspace);
        for _ in 0..3 {
            frame(&ctx, &mut app);
        }
        let before = captures();
        let stamps_before = stamps(&app);
        for _ in 0..10 {
            frame(&ctx, &mut app);
        }
        assert_eq!(
            captures() - before,
            0,
            "{workspace:?}: idle frames captured the whole project; stamps {stamps_before} -> {}",
            stamps(&app)
        );
    }
}

/// Every surface a project with results brings up, as the user sees it: the
/// schematic, a testbench, a Verilog-A source in the code editor, each page
/// of Simulate and Verify, and each other workspace after a run.
#[cfg(not(target_arch = "wasm32"))]
mod every_surface {
    use std::time::{Duration, Instant};

    use egui::{Key, Modifiers};
    use rspice_app_state::state::{
        Cell, Library, ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner,
        SimulationPage, SimulationRunLifecycle, VerificationPage, View,
    };
    use rspice_app_state::workbench::examples::EXAMPLES;

    use super::*;
    use crate::feature_tests::replay::Replay;
    use crate::workbench::session::project_transactions::create_example_project;

    /// Frames a surface is given to settle after it is brought up, before
    /// its idle frames are counted.
    const SETTLE_FRAMES: usize = 3;
    const IDLE_FRAMES: usize = 5;

    /// The first example as a saved project, with a Verilog-A cell whose
    /// source the code editor shows and a testbench beside its schematic,
    /// run once so its results are on hand. The cell views it adds.
    fn saved_example_after_a_run() -> (Replay, CellViewRef, CellViewRef, CellViewRef) {
        let mut replay = Replay::new();
        let state = &mut replay.app.state;
        create_example_project(state, EXAMPLES[0]).expect("the example opens as a project");
        let top = state.model.workspace.content.active_view.clone();

        let module = CellViewRef::new("behavioral", "gain", "veriloga");
        let mut view = View::new("veriloga", ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "gain".to_owned());
        let mut cell = Cell::new("gain");
        cell.add_view(view);
        let mut library = Library::new("behavioral");
        library.add_cell(cell);
        let mut bench = Cell::new("bench");
        bench.add_view(View::new("testbench", ViewType::Testbench));
        library.add_cell(bench);
        state.model.library_manager.add_library(library);
        state
            .model
            .workspace
            .content
            .project_sources
            .insert_bundle(
                ProjectSourceBundle::try_new(
                    ProjectSourceOwner::cell_view(module.clone()),
                    ProjectSourceLanguage::VerilogA,
                    "gain.va",
                    "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ V(p, n);\nendmodule\n",
                    [],
                    [],
                )
                .expect("a Verilog-A source"),
            )
            .expect("one source for the view");
        let testbench = CellViewRef::new("behavioral", "bench", "testbench");

        state.model.sim_setup.tran.step = "1n".to_owned();
        state.model.sim_setup.tran.stop = "10u".to_owned();
        state.open_workspace_view(top.clone());
        state.workbench.activate(Workspace::Design);
        run_to_its_end(&mut replay);

        let state = &mut replay.app.state;
        let baseline = snapshot(state).expect("the project captures");
        state
            .platform
            .project_lifecycle
            .accept_project(baseline, None);
        (replay, top, module, testbench)
    }

    /// Run, and frames until the run has ended.
    fn run_to_its_end(replay: &mut Replay) {
        let runs = replay.app.state.model.simulation.retained.runs.len();
        replay.key(Key::F5, Modifiers::NONE);
        let started = Instant::now();
        loop {
            replay.frame(Vec::new());
            let retained = &replay.app.state.model.simulation.retained.runs;
            if retained.len() > runs
                && retained.iter().all(|run| {
                    !matches!(
                        run.lifecycle,
                        SimulationRunLifecycle::Preparing | SimulationRunLifecycle::Running
                    )
                })
            {
                break;
            }
            assert!(
                started.elapsed() < Duration::from_secs(120),
                "the run did not end"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// `surface` brought up and given a few frames, then idle frames: none
    /// of them captures the project.
    fn idles(replay: &mut Replay, surface: &str, bring_up: &dyn Fn(&mut RSpiceApp)) {
        bring_up(&mut replay.app);
        for _ in 0..SETTLE_FRAMES {
            replay.frame(Vec::new());
        }
        let before = captures();
        let stamps_before = stamps(&replay.app);
        for _ in 0..IDLE_FRAMES {
            replay.frame(Vec::new());
        }
        assert_eq!(
            captures() - before,
            0,
            "{surface}: idle frames captured the whole project; stamps {stamps_before} -> {}",
            stamps(&replay.app)
        );
    }

    #[test]
    fn idle_frames_on_every_surface_of_a_run_project_capture_nothing() {
        let (mut replay, top, module, testbench) = saved_example_after_a_run();
        let open = |reference: CellViewRef| {
            move |app: &mut RSpiceApp| {
                app.state.open_workspace_view(reference.clone());
                app.state.workbench.activate(Workspace::Design);
            }
        };
        idles(&mut replay, "the schematic", &open(top.clone()));
        idles(&mut replay, "the testbench", &open(testbench));
        idles(&mut replay, "the Verilog-A source", &open(module));
        for page in SimulationPage::NAVIGATION {
            idles(
                &mut replay,
                &format!("Simulate {page:?}"),
                &|app: &mut RSpiceApp| {
                    app.state.simulate.simulation_page = page;
                    app.state.workbench.activate(Workspace::Simulate);
                },
            );
        }
        for page in VerificationPage::NAVIGATION {
            idles(
                &mut replay,
                &format!("Verify {page:?}"),
                &|app: &mut RSpiceApp| {
                    app.state.simulate.verification_page = page;
                    app.state.workbench.activate(Workspace::Verify);
                },
            );
        }
        for workspace in [
            Workspace::Results,
            Workspace::Netlist,
            Workspace::Stimulus,
            Workspace::Models,
            Workspace::Project,
        ] {
            idles(
                &mut replay,
                &format!("{workspace:?}"),
                &|app: &mut RSpiceApp| {
                    app.state.workbench.activate(workspace);
                },
            );
        }
        idles(
            &mut replay,
            "the schematic after the other surfaces",
            &open(top),
        );

        // The frames do ask: an edit is captured on the frame it lands on.
        replay
            .app
            .state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(40, 40));
        let before = captures();
        replay.frame(Vec::new());
        assert_eq!(captures() - before, 1, "an edit is captured once");
    }
}
