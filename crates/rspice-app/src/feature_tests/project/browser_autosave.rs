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
use super::with_cx;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::services_access::initialize_from_session;
use crate::workbench::app_state::session::session_store::read_browser_session;
use rspice_app_project::workbench::project::library_dialogs::new_cell::create_new_cell;
use rspice_app_project::workbench::workflows::project_workflow::SaveRequestOutcome;
use rspice_app_services::workbench::app_state::session::browser_session::{
    PROJECT_AUTOSAVE_KEY, PROJECT_PART_KEY, RUNS_KEY,
};
use rspice_app_services::workbench::lifecycle::durable_records::{
    BrowserRecords, DurableRecords, KeptInMemory, MemoryOrigin, MemoryRecords, Record, RecordError,
    RecordFuture, block_on, forget_browser_tasks, keep_browser_records_in, run_browser_tasks,
};
use rspice_app_services::workbench::lifecycle::project_checkpoint::{
    CHECKPOINT_ENCODINGS, ProjectCheckpointCatalog, ProjectCheckpointReason, capture_for_records,
    catalog_records, delete_records, list_records,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    SaveScope, active_document_is_dirty, has_unsaved_changes, has_unsaved_changes_fresh,
};
use rspice_app_state::diagnostics::LogSeverity;
use rspice_app_state::state::{ComponentType, Point};
use rspice_app_state::workbench::workspace::Workspace;

/// A browser's local storage: it refuses a write past its quota and keeps
/// what it held, as a browser's does. Every write is counted, by its key.
#[derive(Default)]
struct LocalStorage {
    values: HashMap<String, String>,
    quota: Option<usize>,
    writes: HashMap<String, usize>,
}

impl eframe::Storage for LocalStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.values.get(key).cloned()
    }

    fn set_string(&mut self, key: &str, value: String) {
        *self.writes.entry(key.to_owned()).or_default() += 1;
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
        block_on(read_browser_session(eframe::Storage::get_string(
            &self.local,
            PROJECT_AUTOSAVE_KEY,
        )));
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

/// The bytes the open project's autosave takes in a browser's records now,
/// written into a browser of its own to measure them.
fn autosave_bytes(app: &RSpiceApp) -> usize {
    let scratch = MemoryOrigin::new(None);
    let records = scratch.store();
    let saved_generation = app
        .state
        .platform
        .browser_project_binding_receipt
        .as_ref()
        .map(|receipt| receipt.accepted_generation);
    let capture = capture_for_records(
        &app.state,
        ProjectCheckpointReason::Autosave,
        saved_generation,
    )
    .expect("the project is read for its autosave");
    block_on(capture.publish(&*records, None)).expect("the autosave is measured");
    scratch.used()
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

/// Keep after a reload, then File > Save: nothing reads unsaved any more,
/// not the title, the toolbar, a tab or the Overview; an edit made after it
/// and undone back to where the save left it reads clean. Played for a part
/// and for a new cell left unsaved when the page went.
#[test]
fn in_the_browser_keep_then_save_leaves_nothing_unsaved() {
    for (label, new_cell) in [
        ("browser-autosave-keep-save", false),
        ("browser-autosave-keep-save-cell", true),
    ] {
        let mut browser = Browser::open();
        let (mut app, root) = saved_project(label);
        if new_cell {
            let project = &mut app.state.project;
            project.new_cell_library = app
                .state
                .model
                .workspace
                .content
                .project
                .root_library
                .clone();
            project.new_cell_name = "crashcell".to_owned();
            project.new_cell_create_schematic = true;
            project.new_cell_library_revision = app.state.model.library_manager.revision();
            assert!(
                with_cx(&mut app, create_new_cell),
                "{label}: the cell is created"
            );
            app.after_contribution();
        } else {
            edit(&mut app, 1);
        }
        let ctx = offscreen();
        browser.leave_the_page(&ctx, &mut app, true);
        assert_eq!(
            browser.autosaves(&app),
            1,
            "{label}: leaving the page wrote it"
        );
        drop(app);

        let mut app = browser.reload();
        let asked = paint_until(&ctx, &mut app, |text| {
            text.contains("Recover unsaved changes?")
        });
        app_frame(
            &ctx,
            &mut app,
            click_on(&asked, "Keep the recovered changes"),
        );
        app_frame(&ctx, &mut app, Vec::new());
        assert!(
            has_unsaved_changes_fresh(&app.state),
            "{label}: kept as unsaved work"
        );

        assert!(
            matches!(
                app.state.save_project_scope(SaveScope::ActiveDocument),
                SaveRequestOutcome::CanonicalComplete
            ),
            "{label}: the save completes"
        );
        app_frame(&ctx, &mut app, Vec::new());
        let unsaved_marks = |app: &RSpiceApp| {
            (
                has_unsaved_changes_fresh(&app.state),
                has_unsaved_changes(&app.state),
                active_document_is_dirty(&app.state),
                app.last_window_title.contains('*'),
                app.state
                    .model
                    .workspace
                    .content
                    .open_views
                    .iter()
                    .filter(|view| view.dirty)
                    .map(|view| view.reference.display_path())
                    .collect::<Vec<_>>(),
            )
        };
        assert_eq!(
            unsaved_marks(&app),
            (false, false, false, false, Vec::new()),
            "{label}: after the save nothing reads unsaved"
        );

        edit(&mut app, 9);
        app_frame(&ctx, &mut app, Vec::new());
        assert!(
            has_unsaved_changes_fresh(&app.state),
            "{label}: an edit after the save is unsaved"
        );
        assert!(app.state.model.schematic.undo(), "{label}: undone");
        app_frame(&ctx, &mut app, Vec::new());
        assert_eq!(
            unsaved_marks(&app),
            (false, false, false, false, Vec::new()),
            "{label}: the edit undone back to the save reads clean"
        );
        let _ = std::fs::remove_dir_all(root);
    }
}

/// A cell made before the page was left is only in the autosave: the
/// reload's question lists it as new in the autosave, not as removed.
#[test]
fn in_the_browser_a_cell_only_the_autosave_holds_is_offered_as_new_in_it() {
    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-autosave-cell");
    super::project_autosave::add_cell(&mut app, "crashcell");
    let ctx = offscreen();
    browser.leave_the_page(&ctx, &mut app, true);
    drop(app);

    let mut app = browser.reload();
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Recover unsaved changes?")
    });
    let text = painted(&asked);
    assert!(
        text.lines()
            .any(|line| line.starts_with("New in the autosave") && line.contains("crashcell")),
        "{text}"
    );
    assert!(!text.contains("Removed since"), "{text}");
    let _ = std::fs::remove_dir_all(root);
}

/// A browser's checkpoint records that list the project's checkpoints just
/// before each manifest is written: the moment a checkpoint's project is
/// stored and its record is not, as a listing the Recovery page starts
/// while a checkpoint is being written meets it.
struct ListsMidWrite {
    records: Rc<MemoryRecords>,
    project_id: String,
    listed: std::cell::RefCell<Vec<ProjectCheckpointCatalog>>,
}

impl DurableRecords for ListsMidWrite {
    fn get<'a>(&'a self, key: &'a str) -> RecordFuture<'a, Option<Record>> {
        self.records.get(key)
    }

    fn put<'a>(&'a self, key: &'a str, record: Record) -> RecordFuture<'a, ()> {
        Box::pin(async move {
            if key.ends_with(".manifest") {
                let catalog = catalog_records(&*self.records, &self.project_id)
                    .await
                    .map_err(RecordError::Failed)?;
                self.listed.borrow_mut().push(catalog);
            }
            self.records.put(key, record).await
        })
    }

    fn delete<'a>(&'a self, key: &'a str) -> RecordFuture<'a, ()> {
        self.records.delete(key)
    }

    fn keys(&self) -> RecordFuture<'_, Vec<String>> {
        self.records.keys()
    }
}

/// The checkpoints are listed while an autosave is being written, its
/// project stored and its record not yet: nothing is damaged, the
/// checkpoint written before is listed whole, and once the write is done
/// the new autosave is listed in its place.
#[test]
fn in_the_browser_a_checkpoint_being_written_is_not_listed_as_damaged() {
    let origin = MemoryOrigin::new(None);
    let checkpoints = origin.store();
    let (mut app, root) = saved_project("browser-autosave-listing");
    let project_id = app.state.model.workspace.content.project.id().to_string();
    let lists = Rc::new(ListsMidWrite {
        records: Rc::clone(&checkpoints),
        project_id: project_id.clone(),
        listed: std::cell::RefCell::default(),
    });
    let _kept = keep_browser_records_in(BrowserRecords {
        session: origin.store() as Rc<dyn DurableRecords>,
        checkpoints: Rc::clone(&lists) as Rc<dyn DurableRecords>,
    });
    let ctx = offscreen();
    edit(&mut app, 1);
    autosave_interval_passes(&ctx, &mut app);
    edit(&mut app, 2);
    autosave_interval_passes(&ctx, &mut app);

    let listed = lists.listed.borrow();
    assert_eq!(listed.len(), 2, "each autosave was listed mid-write");
    for catalog in listed.iter() {
        assert!(
            catalog.quarantined.is_empty(),
            "a checkpoint being written was called damaged: {:?}",
            catalog.quarantined
        );
    }
    assert_eq!(
        listed[1].checkpoints.len(),
        1,
        "the autosave written before is listed whole"
    );
    let after = block_on(catalog_records(&*checkpoints, &project_id)).expect("listed");
    assert!(after.quarantined.is_empty(), "{:?}", after.quarantined);
    assert_eq!(after.checkpoints.len(), 1, "the newer autosave replaced it");
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
/// Console says once that run results are not kept. A reload offers the
/// autosave, which holds them, over the project as saved, without them.
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
    let runs = browser
        .session
        .record(RUNS_KEY)
        .map(|record| record.stored_len())
        .unwrap_or_default();

    for index in 1..=5 {
        edit(&mut app, index);
    }
    // Nothing more fits: the autosave the edits make has room only once the
    // runs record gives up its space. The run's samples are in the results
    // store, not in the browser's records, so the autosave is measured.
    let autosave = autosave_bytes(&app);
    browser.origin.set_quota(Some(autosave + runs / 2));
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
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Recover unsaved changes?")
    });
    assert!(app.state.model.simulation.retained.runs.is_empty());
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

/// The writes local storage took, by key, since `before`.
fn writes_since(browser: &Browser, before: &HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut writes = browser
        .local
        .writes
        .iter()
        .map(|(key, count)| (key.clone(), count - before.get(key).copied().unwrap_or(0)))
        .filter(|(_, count)| *count > 0)
        .collect::<Vec<_>>();
    writes.sort();
    writes
}

/// The page is left again with nothing changed: nothing is written. A
/// change to the layout writes the layout alone; an edit to the project
/// writes the project; and when the autosave already holds the project as it
/// is, the session names that record and the project is not written again.
#[test]
fn a_session_kept_with_nothing_changed_writes_nothing() {
    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-session-unchanged");
    edit(&mut app, 1);
    let ctx = offscreen();
    app_frame(&ctx, &mut app, Vec::new());
    browser.leave_the_page(&ctx, &mut app, true);
    assert!(browser.local.writes.contains_key(PROJECT_PART_KEY));

    let before = browser.local.writes.clone();
    browser.leave_the_page(&ctx, &mut app, true);
    assert_eq!(
        writes_since(&browser, &before),
        Vec::new(),
        "nothing changed"
    );

    let before = browser.local.writes.clone();
    app.state.workbench.activate(Workspace::Results);
    app_frame(&ctx, &mut app, Vec::new());
    browser.leave_the_page(&ctx, &mut app, true);
    assert_eq!(
        writes_since(&browser, &before),
        vec![(eframe::APP_KEY.to_owned(), 1)],
        "the layout alone"
    );

    let before = browser.local.writes.clone();
    edit(&mut app, 2);
    browser.leave_the_page(&ctx, &mut app, true);
    assert_eq!(
        writes_since(&browser, &before),
        vec![(PROJECT_PART_KEY.to_owned(), 1)],
        "the project alone"
    );

    let before = browser.local.writes.clone();
    edit(&mut app, 3);
    autosave_interval_passes(&ctx, &mut app);
    browser.leave_the_page(&ctx, &mut app, true);
    assert_eq!(
        writes_since(&browser, &before),
        vec![(PROJECT_AUTOSAVE_KEY.to_owned(), 1)],
        "the autosave named in place of the project"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// A page left with unsaved work reopens with it through the browser's real
/// store: kept in the session when the page went before its autosave was
/// written, and offered back from the autosave the session names when that
/// one held it.
#[test]
fn a_page_left_with_unsaved_work_reopens_with_it() {
    let ctx = offscreen();

    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-session-left-early");
    let saved_parts = parts(&app);
    edit(&mut app, 1);
    browser.leave_the_page(&ctx, &mut app, false);
    drop(app);
    let mut app = browser.reload();
    for _ in 0..3 {
        run_browser_tasks();
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(parts(&app), saved_parts + 1, "the work is open");
    assert!(has_unsaved_changes(&app.state), "as unsaved work");
    drop(app);
    let _ = std::fs::remove_dir_all(root);

    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-session-left-named");
    let saved_parts = parts(&app);
    edit(&mut app, 1);
    browser.leave_the_page(&ctx, &mut app, true);
    edit(&mut app, 2);
    autosave_interval_passes(&ctx, &mut app);
    browser.leave_the_page(&ctx, &mut app, true);
    assert!(
        eframe::Storage::get_string(&browser.local, PROJECT_AUTOSAVE_KEY).is_some(),
        "the session names the autosave"
    );
    drop(app);
    let mut app = browser.reload();
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Recover unsaved changes?")
    });
    assert_eq!(parts(&app), saved_parts, "asked over the project as saved");
    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Keep the recovered changes"),
    );
    app_frame(&ctx, &mut app, Vec::new());
    assert_eq!(parts(&app), saved_parts + 2, "the work is back");
    assert!(has_unsaved_changes(&app.state), "as unsaved work");
    let _ = std::fs::remove_dir_all(root);
}

/// The session names an autosave that is gone when the page is next opened,
/// taken from the browser's storage: the work the session kept beside it,
/// older, reopens as unsaved work rather than being given up.
#[test]
fn a_session_naming_an_autosave_that_is_gone_reopens_with_the_work_kept_beside_it() {
    let mut browser = Browser::open();
    let (mut app, root) = saved_project("browser-session-autosave-gone");
    let saved_parts = parts(&app);
    let ctx = offscreen();
    edit(&mut app, 1);
    browser.leave_the_page(&ctx, &mut app, true);
    edit(&mut app, 2);
    autosave_interval_passes(&ctx, &mut app);
    browser.leave_the_page(&ctx, &mut app, true);
    assert!(eframe::Storage::get_string(&browser.local, PROJECT_AUTOSAVE_KEY).is_some());
    let project_id = app.state.model.workspace.content.project.id().to_string();
    drop(app);
    for checkpoint in block_on(list_records(&*browser.checkpoints, &project_id))
        .expect("the checkpoints are listed")
    {
        block_on(delete_records(&*browser.checkpoints, &checkpoint))
            .expect("the autosave is removed");
    }

    let mut app = browser.reload();
    for _ in 0..3 {
        run_browser_tasks();
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(parts(&app), saved_parts + 1, "the work kept beside it");
    assert!(has_unsaved_changes(&app.state), "as unsaved work");
    let _ = std::fs::remove_dir_all(root);
}
