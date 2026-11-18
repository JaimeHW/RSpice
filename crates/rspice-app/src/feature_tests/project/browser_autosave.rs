//! The browser keeps work as the desktop does: the open project's unsaved
//! work in an autosave written off the frame and offered back at the next
//! start, and the session in local storage when the page is left, its runs
//! apart in the browser's durable records. When the browser's storage is
//! full the project's autosave comes first, and what could not be kept is
//! said once.
//!
//! The browser's records are kept in memory here, behind the same store the
//! browser's IndexedDB answers through, its local storage in a map with a
//! quota of its own, and the browser's tasks are run between frames as its
//! event loop would run them.

#![cfg(not(target_arch = "wasm32"))]

use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use super::recovery_frames::{app_frame, click_on, offscreen, painted, saved_project};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::services_access::initialize_from_session;
use crate::workbench::app_state::session::session_store::read_browser_session;
use rspice_app_services::workbench::app_state::session::browser_session::RUNS_KEY;
use rspice_app_services::workbench::lifecycle::durable_records::{
    BrowserRecords, DurableRecords, KeptInMemory, MemoryOrigin, MemoryRecords, block_on,
    forget_browser_tasks, keep_browser_records_in, run_browser_tasks,
};
use rspice_app_services::workbench::lifecycle::project_checkpoint::{
    CHECKPOINT_ENCODINGS, ProjectCheckpointReason, list_records,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::has_unsaved_changes;
use rspice_app_state::diagnostics::LogSeverity;
use rspice_app_state::state::{ComponentType, Point};

/// A browser's local storage: it refuses a write past its quota and keeps
/// what it held, as a browser's does.
#[derive(Default)]
struct LocalStorage {
    values: HashMap<String, String>,
    quota: Option<usize>,
}

impl eframe::Storage for LocalStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.values.get(key).cloned()
    }

    fn set_string(&mut self, key: &str, value: String) {
        let others = self
            .values
            .iter()
            .filter(|(stored, _)| stored.as_str() != key)
            .map(|(stored, text)| stored.len() + text.len())
            .sum::<usize>();
        if self
            .quota
            .is_none_or(|quota| others + key.len() + value.len() <= quota)
        {
            self.values.insert(key.to_owned(), value);
        }
    }

    fn remove_string(&mut self, key: &str) {
        self.values.remove(key);
    }

    fn flush(&mut self) {}
}

/// A browser's storage for RSpice: its local storage, and its session and
/// checkpoint records under one quota.
struct Browser {
    local: LocalStorage,
    origin: Rc<MemoryOrigin>,
    session: Rc<MemoryRecords>,
    checkpoints: Rc<MemoryRecords>,
    _kept: KeptInMemory,
}

impl Browser {
    fn open() -> Self {
        let origin = MemoryOrigin::new(None);
        let session = origin.store();
        let checkpoints = origin.store();
        let kept = keep_browser_records_in(BrowserRecords {
            session: Rc::clone(&session) as Rc<dyn DurableRecords>,
            checkpoints: Rc::clone(&checkpoints) as Rc<dyn DurableRecords>,
        });
        Self {
            local: LocalStorage::default(),
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

    fn session_text(&self) -> Option<String> {
        eframe::Storage::get_string(&self.local, eframe::APP_KEY)
    }

    /// The page is left: eframe saves, as it does when the page is hidden.
    /// What that started in browser tasks runs when `tasks_run`; otherwise
    /// the page is gone first and they never do.
    fn leave_the_page(&mut self, ctx: &egui::Context, app: &mut RSpiceApp, tasks_run: bool) {
        app.state.sync_active_schematic_to_workspace();
        app.state.keep_browser_session(&mut self.local);
        if tasks_run {
            run_browser_tasks();
            app_frame(ctx, app, Vec::new());
        } else {
            forget_browser_tasks();
        }
    }

    /// The page is reloaded: the runs the browser kept are read before the
    /// application starts, the session from local storage, and the project
    /// it holds bound again.
    fn reload(&self) -> RSpiceApp {
        block_on(read_browser_session());
        let mut app = RSpiceApp::test_instance();
        app.state = AppState::restore_with_store(Some(&self.local), None);
        initialize_from_session(&mut app.state);
        app
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

/// The autosave interval passes: the frame captures the project, the
/// browser's task writes it, and the next frame takes what it wrote.
fn autosave_interval_passes(ctx: &egui::Context, app: &mut RSpiceApp) {
    app.state.platform.project_autosave.arm_due();
    app_frame(ctx, app, Vec::new());
    run_browser_tasks();
    app_frame(ctx, app, Vec::new());
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

fn lines_containing(app: &RSpiceApp, severity: LogSeverity, text: &str) -> usize {
    app.state
        .model
        .log_buffer
        .entries()
        .filter(|entry| entry.severity == severity && entry.message.contains(text))
        .count()
}

/// The autosave is written at its interval, the frame only capturing the
/// project: the checkpoint is encoded and stored by the browser's task.
#[test]
fn in_the_browser_no_frame_encodes_the_autosave() {
    let browser = Browser::open();
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
    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-autosave-reload");
    let saved_parts = parts(&app);
    edit(&mut app, 1);
    let ctx = offscreen();
    browser.leave_the_page(&ctx, &mut app, true);
    assert_eq!(browser.autosaves(&app), 1, "leaving the page wrote it");
    assert!(
        browser.session_text().is_some(),
        "the session is in local storage"
    );
    drop(app);

    let mut app = browser.reload();
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

/// A page left before its autosave was written still brings the work back:
/// the session kept it, written at once as the page went. Work newer than
/// the autosave is never put aside for it: it stays open and nothing is
/// asked.
#[test]
fn in_the_browser_work_newer_than_the_autosave_is_not_asked_about() {
    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-autosave-newer");
    let saved_parts = parts(&app);
    edit(&mut app, 1);
    let ctx = offscreen();
    autosave_interval_passes(&ctx, &mut app);
    assert_eq!(browser.autosaves(&app), 1);
    // More work, and the page left before the autosave of it was written.
    edit(&mut app, 2);
    browser.leave_the_page(&ctx, &mut app, false);
    drop(app);

    let mut app = browser.reload();
    for _ in 0..5 {
        run_browser_tasks();
        let output = app_frame(&ctx, &mut app, Vec::new());
        assert!(!painted(&output).contains("Recover unsaved changes?"));
    }
    assert_eq!(parts(&app), saved_parts + 2, "the newer work stays open");
    assert!(has_unsaved_changes(&app.state));
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
/// the session without them, and offers the autosave, which holds them.
#[test]
fn when_the_browser_is_full_the_autosave_is_kept_and_the_runs_left_out_are_named() {
    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-autosave-full");
    let saved_parts = parts(&app);
    // A run's results are the project's unsaved work too: the autosave the
    // page left with holds them, as the runs record does.
    keep_a_run(&mut app, 100_000);
    let ctx = offscreen();
    browser.leave_the_page(&ctx, &mut app, true);
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
    let left_out = "Run results are not kept for the next session";
    assert_eq!(
        lines_containing(&app, LogSeverity::Warning, left_out),
        1,
        "what could not be kept is named"
    );

    // Leaving the page keeps the session without them, saying nothing more.
    browser.leave_the_page(&ctx, &mut app, true);
    assert!(browser.session.record(RUNS_KEY).is_none());
    assert_eq!(
        lines_containing(&app, LogSeverity::Warning, left_out),
        1,
        "named once"
    );
    drop(app);

    let mut app = browser.reload();
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

/// Local storage refuses the session: the project's autosave is kept all the
/// same, and the Console says once that the session was not.
#[test]
fn when_local_storage_refuses_the_session_the_autosave_is_kept_and_it_is_named_once() {
    let mut browser = Browser::open();
    browser.local.quota = Some(64);
    let (mut app, root) = saved_project("browser-session-refused");
    edit(&mut app, 1);
    let ctx = offscreen();
    browser.leave_the_page(&ctx, &mut app, true);
    edit(&mut app, 2);
    browser.leave_the_page(&ctx, &mut app, true);

    assert_eq!(browser.autosaves(&app), 1, "the project's work is kept");
    assert!(browser.session_text().is_none());
    assert_eq!(
        lines_containing(&app, LogSeverity::Error, "The session could not be kept"),
        1,
        "named once"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// A run the session holds is kept in its own record once it is finished,
/// without waiting for the page to be left, read back at the next start, and
/// not written again while it is unchanged.
#[test]
fn in_the_browser_the_session_keeps_its_runs_apart_and_brings_them_back() {
    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-session-runs");
    // Autosave off: what is written is the session alone.
    app.state.platform.autosave_minutes = 0;
    keep_a_run(&mut app, 50);
    let ctx = offscreen();
    app_frame(&ctx, &mut app, Vec::new());
    run_browser_tasks();
    app_frame(&ctx, &mut app, Vec::new());
    let runs = browser
        .session
        .record(RUNS_KEY)
        .expect("the finished run is kept");

    browser.leave_the_page(&ctx, &mut app, true);
    let session = browser.session_text().expect("the session is kept");
    assert!(
        !session.contains("V(out)"),
        "the session's own text holds no run"
    );
    // Unchanged runs are not written again; the browser's record stays.
    browser.origin.set_quota(Some(browser.origin.used()));
    browser.leave_the_page(&ctx, &mut app, true);
    assert_eq!(browser.session.record(RUNS_KEY), Some(runs));
    assert_eq!(lines_containing(&app, LogSeverity::Warning, "not kept"), 0);
    drop(app);

    let app = browser.reload();
    assert_eq!(app.state.model.simulation.retained.runs.len(), 1);
    assert_eq!(
        app.state.model.simulation.retained.runs[0].label,
        "Bias sweep"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// A page left before the browser kept its new run: the reload restores the
/// session and says the run was not, rather than putting other runs under
/// it.
#[test]
fn a_run_the_page_was_left_before_keeping_is_named_at_the_reload() {
    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-session-runs-lost");
    app.state.platform.autosave_minutes = 0;
    let ctx = offscreen();
    keep_a_run(&mut app, 50);
    browser.leave_the_page(&ctx, &mut app, false);
    drop(app);

    let app = browser.reload();
    assert!(app.state.model.simulation.retained.runs.is_empty());
    assert!(
        app.state.platform.project_lifecycle.is_open(),
        "the session came back"
    );
    assert_eq!(
        lines_containing(
            &app,
            LogSeverity::Warning,
            "The run results of the previous session were not restored"
        ),
        1
    );
    let _ = std::fs::remove_dir_all(root);
}
