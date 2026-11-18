//! The browser keeps work as the desktop does: the open project's unsaved
//! work in an autosave written off the frame and offered back at the next
//! start, and the session in its own records beside it, its runs apart. When
//! the browser's storage is full the project's autosave comes first, and what
//! could not be kept is said once.
//!
//! The browser's records are kept in memory here, behind the same store the
//! browser's IndexedDB answers through, and the browser's tasks are run
//! between frames as its event loop would run them.

#![cfg(not(target_arch = "wasm32"))]

use std::rc::Rc;
use std::time::{Duration, Instant};

use super::recovery_frames::{app_frame, click_on, offscreen, painted, saved_project};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::services_access::initialize_from_session;
use crate::workbench::app_state::session::session_store::read_browser_session;
use rspice_app_services::workbench::app_state::session::browser_session::{RUNS_KEY, SESSION_KEY};
use rspice_app_services::workbench::lifecycle::durable_records::{
    BrowserRecords, DurableRecords, KeptInMemory, MemoryOrigin, MemoryRecords, block_on,
    keep_browser_records_in, run_browser_tasks,
};
use rspice_app_services::workbench::lifecycle::project_checkpoint::{
    CHECKPOINT_ENCODINGS, ProjectCheckpointReason, list_records,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::has_unsaved_changes;
use rspice_app_state::diagnostics::LogSeverity;
use rspice_app_state::state::{ComponentType, Point};

/// A browser's storage for RSpice: its session records and its checkpoint
/// records, under one quota.
struct Browser {
    origin: Rc<MemoryOrigin>,
    session: Rc<MemoryRecords>,
    checkpoints: Rc<MemoryRecords>,
    _kept: KeptInMemory,
}

impl Browser {
    fn open(quota: Option<usize>) -> Self {
        let origin = MemoryOrigin::new(quota);
        let session = origin.store();
        let checkpoints = origin.store();
        let kept = keep_browser_records_in(BrowserRecords {
            session: Rc::clone(&session) as Rc<dyn DurableRecords>,
            checkpoints: Rc::clone(&checkpoints) as Rc<dyn DurableRecords>,
        });
        Self {
            origin,
            session,
            checkpoints,
            _kept: kept,
        }
    }

    /// The autosaves the browser keeps of the open project.
    fn autosaves(&self, app: &RSpiceApp) -> usize {
        let project_id = app.state.model.workspace.content.project.id().to_string();
        block_on(list_records(&*self.checkpoints, &project_id))
            .expect("the checkpoints are listed")
            .into_iter()
            .filter(|checkpoint| checkpoint.reason() == ProjectCheckpointReason::Autosave)
            .count()
    }
}

fn edit(app: &mut RSpiceApp, index: i32) {
    app.state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Capacitor, Point::new(40 * index, 0));
    });
}

fn parts(app: &RSpiceApp) -> usize {
    app.state
        .model
        .schematic
        .editor_ref()
        .design
        .document()
        .components
        .len()
}

/// A finished run of `samples` points, as a simulation leaves one.
fn keep_a_run(app: &mut RSpiceApp, samples: usize) {
    let mut run = rspice_app_state::state::SimulationRun::new(1);
    run.mark_running().expect("a new run starts");
    run.finish_lifecycle(rspice_app_state::state::SimulationRunLifecycle::Completed)
        .expect("a running run completes");
    run.label = "Bias sweep".to_owned();
    let time = (0..samples).map(|i| i as f64 * 1e-9).collect::<Vec<_>>();
    let value = (0..samples)
        .map(|i| (i as f64 * 0.001).sin())
        .collect::<Vec<_>>();
    run.add_analysis(
        rspice_app_state::state::AnalysisResult::new(
            1,
            rspice_app_state::state::AnalysisType::Transient,
            "TRAN",
        )
        .with_waveforms(vec![rspice_app_state::state::RetainedTrace::new(
            "V(out)", time, value, "#00aaff",
        )]),
    );
    run.restore_provenance(rspice_app_state::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("the run's provenance seals");
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.model.simulation.retained.next_run_id = 1;
}

/// The page is left: eframe saves, as it does when the page is hidden, and
/// the browser runs what that started.
fn leave_the_page(ctx: &egui::Context, app: &mut RSpiceApp) {
    app.state.sync_active_schematic_to_workspace();
    app.state.keep_browser_session(None);
    run_browser_tasks();
    app_frame(ctx, app, Vec::new());
}

/// The autosave interval passes: the frame captures the project, the
/// browser's task writes it, and the next frame takes what it wrote.
fn autosave_interval_passes(ctx: &egui::Context, app: &mut RSpiceApp) {
    app.state.platform.project_autosave.arm_due();
    app_frame(ctx, app, Vec::new());
    run_browser_tasks();
    app_frame(ctx, app, Vec::new());
}

/// The page is reloaded: the session the browser kept is read before the
/// application starts, and the project it holds bound again.
fn reload() -> RSpiceApp {
    block_on(read_browser_session());
    let mut app = RSpiceApp::test_instance();
    app.state = AppState::restore_with_store(None, None);
    initialize_from_session(&mut app.state);
    app
}

/// Paint, running the browser's tasks between frames, until `done` holds
/// for what was painted.
fn paint_until(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    done: impl Fn(&str) -> bool,
) -> egui::FullOutput {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        run_browser_tasks();
        let output = app_frame(ctx, app, Vec::new());
        if done(&painted(&output)) {
            return output;
        }
        assert!(
            Instant::now() < deadline,
            "the page never showed what it waited for:\n{}",
            painted(&output)
        );
    }
}

fn warnings_containing(app: &RSpiceApp, text: &str) -> usize {
    app.state
        .model
        .log_buffer
        .entries()
        .filter(|entry| entry.severity == LogSeverity::Warning && entry.message.contains(text))
        .count()
}

/// The autosave is written at its interval, the frame only capturing the
/// project: the checkpoint is encoded and stored by the browser's task.
#[test]
fn in_the_browser_no_frame_encodes_the_autosave() {
    let browser = Browser::open(None);
    let (mut app, root) = saved_project("browser-autosave-frame");
    edit(&mut app, 1);
    let ctx = offscreen();
    CHECKPOINT_ENCODINGS.with(|count| count.set(0));

    app.state.platform.project_autosave.arm_due();
    for _ in 0..5 {
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(
        CHECKPOINT_ENCODINGS.with(std::cell::Cell::get),
        0,
        "no frame encodes the checkpoint"
    );
    assert!(app.state.platform.project_autosave.written().is_empty());

    run_browser_tasks();
    assert_eq!(CHECKPOINT_ENCODINGS.with(std::cell::Cell::get), 1);
    app_frame(&ctx, &mut app, Vec::new());
    assert_eq!(app.state.platform.project_autosave.written().len(), 1);
    assert_eq!(browser.autosaves(&app), 1, "the browser keeps it");
    let _ = std::fs::remove_dir_all(root);
}

/// Leave the page with an unsaved part and reload it: the session brings the
/// project back, and the same question the desktop asks after a crash is
/// asked of the autosave. Keep puts the part back as unsaved work; the
/// project as saved is what the reload offered it over.
#[test]
fn in_the_browser_a_reload_offers_the_autosave_back() {
    let browser = Browser::open(None);
    let (mut app, root) = saved_project("browser-autosave-reload");
    let saved_parts = parts(&app);
    edit(&mut app, 1);
    let ctx = offscreen();
    leave_the_page(&ctx, &mut app);
    assert_eq!(browser.autosaves(&app), 1, "leaving the page wrote it");
    assert!(browser.session.record(SESSION_KEY).is_some());
    drop(app);

    let mut app = reload();
    assert_eq!(parts(&app), saved_parts + 1, "the session kept the work");
    let ctx = offscreen();
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Recover unsaved changes?")
    });
    let text = painted(&asked);
    assert!(
        text.contains("RSpice closed with unsaved changes to \u{201c}")
            && text.contains("differs from the saved project in 1 document"),
        "{text}"
    );
    assert_eq!(parts(&app), saved_parts, "asked over the project as saved");
    assert!(!has_unsaved_changes(&app.state));

    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Keep the recovered changes"),
    );
    app_frame(&ctx, &mut app, Vec::new());
    assert_eq!(parts(&app), saved_parts + 1, "the recovered part is back");
    assert!(has_unsaved_changes(&app.state), "as unsaved work");
    assert!(app.state.platform.project_autosave.offer.is_none());
    let _ = std::fs::remove_dir_all(root);
}

/// Work done after a reload is never put aside for an older autosave: the
/// session's work stays open and nothing is asked.
#[test]
fn in_the_browser_work_newer_than_the_autosave_is_not_asked_about() {
    let browser = Browser::open(None);
    let (mut app, root) = saved_project("browser-autosave-newer");
    let saved_parts = parts(&app);
    edit(&mut app, 1);
    let ctx = offscreen();
    autosave_interval_passes(&ctx, &mut app);
    assert_eq!(browser.autosaves(&app), 1);
    // Autosave turned off, and more work after the autosave it wrote, kept
    // by the session alone.
    app.state.platform.autosave_minutes = 0;
    edit(&mut app, 2);
    leave_the_page(&ctx, &mut app);
    drop(app);

    let mut app = reload();
    for _ in 0..5 {
        run_browser_tasks();
        let output = app_frame(&ctx, &mut app, Vec::new());
        assert!(!painted(&output).contains("Recover unsaved changes?"));
    }
    assert_eq!(parts(&app), saved_parts + 2, "the newer work stays open");
    assert!(app.state.platform.project_autosave.offer.is_none());
    assert_eq!(
        browser.autosaves(&app),
        1,
        "the autosave stays for Recovery"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// The browser's storage fills while the session holds a run: the project's
/// autosave is written all the same, in the space the run gave up, and the
/// Console says once that run results are not kept. A reload brings back
/// the session without them, and offers the autosave.
#[test]
fn when_the_browser_is_full_the_autosave_is_kept_and_the_runs_left_out_are_named() {
    let browser = Browser::open(None);
    let (mut app, root) = saved_project("browser-autosave-full");
    let saved_parts = parts(&app);
    // A run's results are the project's unsaved work too: the autosave the
    // page left with holds them, as the session's own record does.
    keep_a_run(&mut app, 100_000);
    let ctx = offscreen();
    leave_the_page(&ctx, &mut app);
    assert!(
        browser.session.record(RUNS_KEY).is_some(),
        "the runs are kept apart from the session"
    );
    // Nothing more fits.
    browser.origin.set_quota(Some(browser.origin.used()));

    for index in 1..=5 {
        edit(&mut app, index);
    }
    autosave_interval_passes(&ctx, &mut app);
    assert_eq!(browser.autosaves(&app), 1, "the autosave is kept");
    assert!(
        browser.session.record(RUNS_KEY).is_none(),
        "the runs gave way"
    );
    assert!(browser.session.record(SESSION_KEY).is_some());
    assert_eq!(
        warnings_containing(&app, "Run results are not kept for the next session"),
        1,
        "what could not be kept is named"
    );

    // Leaving the page keeps the session without them, saying nothing more.
    leave_the_page(&ctx, &mut app);
    assert!(browser.session.record(RUNS_KEY).is_none());
    assert_eq!(
        warnings_containing(&app, "Run results are not kept for the next session"),
        1,
        "named once"
    );
    drop(app);

    let mut app = reload();
    assert!(app.state.model.simulation.retained.runs.is_empty());
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Recover unsaved changes?")
    });
    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Keep the recovered changes"),
    );
    app_frame(&ctx, &mut app, Vec::new());
    assert_eq!(parts(&app), saved_parts + 5, "the project's work came back");
    assert_eq!(
        app.state.model.simulation.retained.runs.len(),
        1,
        "with the run the autosave kept"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// A run the session holds is kept in its own record, read back at the next
/// start, and not written again while it is unchanged.
#[test]
fn in_the_browser_the_session_keeps_its_runs_apart_and_brings_them_back() {
    let browser = Browser::open(None);
    let (mut app, root) = saved_project("browser-session-runs");
    // Autosave off: what is written is the session alone.
    app.state.platform.autosave_minutes = 0;
    keep_a_run(&mut app, 50);
    let ctx = offscreen();
    leave_the_page(&ctx, &mut app);
    let runs = browser.session.record(RUNS_KEY).expect("the runs are kept");
    let session = browser
        .session
        .record(SESSION_KEY)
        .and_then(|record| record.into_text())
        .expect("the session is kept as text");
    assert!(
        !session.contains("V(out)"),
        "the session's own record holds no run"
    );

    // Unchanged runs are not written again; the browser's record stays.
    browser.origin.set_quota(Some(browser.origin.used()));
    leave_the_page(&ctx, &mut app);
    assert_eq!(browser.session.record(RUNS_KEY), Some(runs));
    assert_eq!(warnings_containing(&app, "not kept"), 0);
    drop(app);

    let app = reload();
    assert_eq!(app.state.model.simulation.retained.runs.len(), 1);
    assert_eq!(
        app.state.model.simulation.retained.runs[0].label,
        "Bias sweep"
    );
    let _ = std::fs::remove_dir_all(root);
}
