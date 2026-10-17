//! Review renders of the window chrome and the floating tools, seeded with
//! state the product made itself.
//!
//! The project is open on a real circuit, the design checks ran and found its
//! missing ground, one run failed in the engine's own words and another is
//! live, and the launcher lists a checkpoint discovered on disk. Nothing on
//! these pages is invented copy: every row is one the product wrote.

use std::io::Write as _;

use super::*;
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{AnalysisResult, AnalysisType, ComponentType, SchematicState};
use crate::workbench::app_state::RecentKind;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::preferences::TogglePreference;
use crate::workbench::state::{ProjectLauncherPage, Workspace};

const DESKTOP: egui::Vec2 = egui::vec2(1440.0, 900.0);

/// Load the RC low-pass example and delete its ground and one wire, so the
/// design checks have a real error (no reference node) and real advisories
/// (the pins that wire connected).
fn load_ungrounded_rc(schematic: &mut SchematicState) {
    crate::workbench::examples::load_example("RC Lowpass Filter", schematic);
    let document = schematic.document_mut_for_test();
    document
        .components
        .retain(|component| component.kind != ComponentType::Ground);
    document.wires.pop();
}

fn ungrounded_rc() -> SchematicState {
    let mut schematic = SchematicState::default();
    load_ungrounded_rc(&mut schematic);
    schematic
}

/// An open project on Design, its checks run, a console row of every
/// severity, one failed run and one live run.
fn seeded_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    // A headless pass has no clock for an opening fade to run against, so a
    // dialog would be rasterized part-transparent over the workbench.
    app.state
        .ui
        .preferences
        .set_toggle(TogglePreference::ReducedMotion, true);
    app.state.project_lifecycle.authority.open_session();
    app.state.workbench.activate(Workspace::Design);
    load_ungrounded_rc(&mut app.state.schematic);

    // The checks log their own summary (Info) and one row per finding
    // (Error for the missing ground, Warning for each open pin).
    Command::CheckCellview.execute(&mut app);
    // What the harmonic-balance solver logs per Krylov solve, at Debug.
    app.state.log_buffer.log(
        LogSeverity::Debug,
        LogSource::Engine,
        "HB Krylov solve: 12 iterations, relative residual 3.40e-09",
        None,
    );

    let error =
        rspice_core::analysis::transient::resolve_transient_maximum_step(0.0, 1.0e-3, None, None)
            .expect_err("a zero TSTEP is refused")
            .to_string();
    let failed = app.state.simulation.start_run();
    failed.add_analysis(AnalysisResult::failed(
        1,
        AnalysisType::Transient,
        "TRAN",
        error.clone(),
    ));
    failed.mark_running().expect("the failed run started");
    failed
        .finish_lifecycle(crate::state::SimulationRunLifecycle::Failed)
        .expect("the failed run ends");
    app.state
        .push_sim_message(crate::diagnostics::ConsoleMessage::error(error));

    let live = app.state.simulation.start_run();
    live.mark_running().expect("the live run started");
    let identity = live.execution_identity();
    app.state.simulation.execution.active_execution = identity;
    app.state.simulation.execution.is_running = true;
    app.state.simulation.execution.progress = 0.42;

    Command::OpenConsole.execute(&mut app);
    app
}

/// Renders for review: the status bar and console, the jobs manager, and the
/// launcher's recovery page, each at desktop size.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_chrome_console_jobs_and_launcher_for_review() {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    let mut write = |name: &str, app: &mut RSpiceApp| {
        let canvas = crate::ui::raster::render(DESKTOP, |ui, _| {
            app.prepare_frame(ui.ctx());
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        });
        let path = directory.join(format!("{name}.png"));
        std::fs::write(&path, canvas.png(DESKTOP.y as usize)).expect("write the render");
        writeln!(report, "{}", path.display()).ok();
    };

    let mut chrome = seeded_app();
    assert!(
        chrome.state.dialogs.drc_results.is_some(),
        "the checks ran and their result is the status bar's"
    );
    write("chrome-status-console-1440x900", &mut chrome);

    let mut jobs = seeded_app();
    Command::JobsManager.execute(&mut jobs);
    write("jobs-manager-1440x900", &mut jobs);

    // A saved design and a newer checkpoint of it with one object fewer, the
    // shape an interrupted edit leaves behind.
    let root =
        std::env::temp_dir().join(format!("rspice-launcher-review-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&root).expect("fixture directory");
    let source = root.join("rc_lowpass.rsch");
    let mut saved = SchematicState::default();
    crate::workbench::examples::load_example("RC Lowpass Filter", &mut saved);
    crate::io::save_schematic(&saved, &source).expect("save the design");
    // Spelled as `file_workflow::autosave_checkpoint_path` writes it, so this
    // review adds no app -> workflows edge.
    let mut checkpoint_name = source.clone().into_os_string();
    checkpoint_name.push(format!(".autosave.generation-{}", uuid::Uuid::new_v4()));
    let mut bytes = serde_json::to_vec_pretty(&crate::io::SchematicFile::new(ungrounded_rc()))
        .expect("serialize the checkpoint");
    bytes.push(b'\n');
    std::fs::write(std::path::PathBuf::from(checkpoint_name), bytes).expect("write the checkpoint");

    let mut launcher = seeded_app();
    launcher
        .state
        .remember_recent_file(RecentKind::Schematic, &source);
    launcher.state.workbench.open_project_launcher();
    launcher.state.workbench.project_launcher_page = ProjectLauncherPage::Recovery;
    launcher
        .state
        .workbench
        .project_launcher_recovery
        .request_refresh();
    write("launcher-recovery-1440x900", &mut launcher);
    assert!(
        !launcher
            .state
            .workbench
            .project_launcher_recovery
            .candidates
            .is_empty(),
        "the launcher discovered the checkpoint"
    );

    std::fs::remove_dir_all(root).ok();
}
