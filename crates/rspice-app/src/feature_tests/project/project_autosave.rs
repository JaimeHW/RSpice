//! The open project's autosave: written off the frame while there is unsaved
//! work, and offered back once, with its time, when the session that wrote it
//! ended without saving. The session itself keeps no unsaved project work.

#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::recovery_frames::{app_frame, click_on, offscreen, paint_until, painted};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_services::workbench::lifecycle::project_checkpoint::{
    ProjectCheckpointReason, list,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    has_unsaved_changes, has_unsaved_changes_fresh,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{ComponentType, Point};
use rspice_app_state::workbench::workspace::Workspace;

/// The folder the crashed session saves its project in, handed to the child
/// process that plays it.
const CRASH_FOLDER: &str = "RSPICE_TEST_CRASHED_SESSION_FOLDER";
const PROJECT: &str = "amp.rspiceproj";

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

/// Paint frames until `done` holds of the application.
fn frames_until(ctx: &egui::Context, app: &mut RSpiceApp, done: impl Fn(&RSpiceApp) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while !done(app) {
        assert!(Instant::now() < deadline, "never came about");
        app_frame(ctx, app, Vec::new());
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// The session the crash tests end: it saves a project, adds a part, lets
/// the autosave interval pass and, once the autosave is on disk, stops the
/// process as a crash would, cleaning nothing up. Run only by
/// [`crashed_session`] in a process of its own.
#[test]
#[ignore = "played in a child process by the crash tests"]
fn crashed_session_child() {
    let Some(folder) = std::env::var_os(CRASH_FOLDER) else {
        return;
    };
    let folder = PathBuf::from(folder);
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.save_project_to_path(&folder.join(PROJECT)));
    edit(&mut app, 7);
    assert!(has_unsaved_changes_fresh(&app.state));
    app.state
        .platform
        .project_autosave
        .arm_at(Instant::now() - Duration::from_secs(3600));
    let ctx = offscreen();
    frames_until(&ctx, &mut app, |app| {
        !app.state.platform.project_autosave.written().is_empty()
    });
    std::process::exit(0);
}

/// A folder holding a saved project and the autosave of one more part that a
/// session wrote before it was killed, and how many parts the saved project
/// has.
fn crashed_session(label: &str) -> (PathBuf, usize) {
    let folder = unique_temp_dir(label);
    let status = std::process::Command::new(std::env::current_exe().expect("the test binary"))
        .args([
            "--exact",
            "feature_tests::project::project_autosave::crashed_session_child",
            "--ignored",
            "--test-threads=1",
        ])
        .env(CRASH_FOLDER, &folder)
        .stdout(std::process::Stdio::null())
        .status()
        .expect("the crashed session runs");
    assert!(status.success(), "the crashed session ran: {status}");
    let saved = rspice_app_services::io::load_project_file(&folder.join(PROJECT))
        .expect("the saved project reads");
    let autosaves = rspice_app_services::workbench::lifecycle::project_checkpoint::list_beside(
        &folder.join(PROJECT),
    )
    .expect("the checkpoints are listed")
    .checkpoints
    .into_iter()
    .filter(|checkpoint| checkpoint.reason() == ProjectCheckpointReason::Autosave)
    .count();
    assert_eq!(autosaves, 1, "the crashed session left its autosave");
    let mut opened = RSpiceApp::test_instance();
    assert!(opened.state.load_project_from_path(&folder.join(PROJECT)));
    let saved_parts = parts(&opened);
    drop(saved);
    (folder, saved_parts)
}

/// Open the project the crashed session left, and paint until the question
/// about its autosave is asked.
fn open_after_crash(folder: &Path) -> (RSpiceApp, egui::Context, egui::FullOutput) {
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.load_project_from_path(&folder.join(PROJECT)));
    let ctx = offscreen();
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Recover unsaved changes?")
    });
    (app, ctx, asked)
}

/// Kill the process after an edit and open the project again: the question
/// names the project and the autosave's time, and Keep puts the part back as
/// unsaved work over the file, which is untouched.
#[test]
fn after_a_crash_keep_puts_the_recovered_changes_back() {
    let (folder, saved_parts) = crashed_session("autosave-crash-keep");
    let saved_bytes = std::fs::read(folder.join(PROJECT)).expect("the file reads");
    let (mut app, ctx, asked) = open_after_crash(&folder);
    let text = painted(&asked);
    assert!(
        text.contains("RSpice closed with unsaved changes to \u{201c}")
            && text.contains("differs from the saved project in 1 document"),
        "{text}"
    );
    assert_eq!(parts(&app), saved_parts, "the saved project is what opened");

    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Keep the recovered changes"),
    );
    app_frame(&ctx, &mut app, Vec::new());
    assert_eq!(parts(&app), saved_parts + 1, "the recovered part is back");
    assert!(has_unsaved_changes(&app.state), "as unsaved work");
    assert_eq!(
        std::fs::read(folder.join(PROJECT)).expect("the file reads again"),
        saved_bytes,
        "the file is untouched until the reader saves"
    );
    assert!(
        app.state.platform.project_autosave.offer.is_none(),
        "asked once"
    );
    let _ = std::fs::remove_dir_all(folder);
}

/// "Open the saved project" leaves the saved project open, clean, and the
/// autosave on the Recovery page until the next save.
#[test]
fn after_a_crash_open_the_saved_project_keeps_the_autosave_on_the_recovery_page() {
    let (folder, saved_parts) = crashed_session("autosave-crash-saved");
    let (mut app, ctx, asked) = open_after_crash(&folder);
    app_frame(&ctx, &mut app, click_on(&asked, "Open the saved project"));
    app_frame(&ctx, &mut app, Vec::new());
    assert_eq!(parts(&app), saved_parts);
    assert!(!has_unsaved_changes_fresh(&app.state), "nothing unsaved");
    assert!(
        list(&app.state.model)
            .expect("listed")
            .checkpoints
            .iter()
            .any(|checkpoint| checkpoint.reason() == ProjectCheckpointReason::Autosave),
        "the autosave is still there to restore"
    );
    assert!(
        app.state.platform.project_autosave.offer.is_none(),
        "asked once"
    );
    let _ = std::fs::remove_dir_all(folder);
}

/// Compare shows the Recovery page with the autosave selected and what it
/// changes listed under it.
#[test]
fn after_a_crash_compare_lists_what_the_autosave_changes() {
    let (folder, saved_parts) = crashed_session("autosave-crash-compare");
    let (mut app, ctx, asked) = open_after_crash(&folder);
    app_frame(&ctx, &mut app, click_on(&asked, "Compare"));
    let shown = paint_until(&ctx, &mut app, |text| {
        text.contains("The current project differs in 1 document:")
    });
    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(app.state.project.project_page, ProjectPage::Recovery);
    assert!(
        painted(&shown)
            .lines()
            .any(|line| line == "Added since" || line == "Changed"),
        "{}",
        painted(&shown)
    );
    assert_eq!(parts(&app), saved_parts, "nothing is restored by comparing");
    let _ = std::fs::remove_dir_all(folder);
}

/// The session the store keeps, held in memory.
#[derive(Default)]
struct MemoryStorage(std::collections::HashMap<String, String>);

impl eframe::Storage for MemoryStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }
    fn set_string(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
    }
    fn remove_string(&mut self, key: &str) {
        self.0.remove(key);
    }
    fn flush(&mut self) {}
}

fn autosaves_beside(file: &Path) -> usize {
    rspice_app_services::workbench::lifecycle::project_checkpoint::list_beside(file)
        .expect("the checkpoints are listed")
        .checkpoints
        .into_iter()
        .filter(|checkpoint| checkpoint.reason() == ProjectCheckpointReason::Autosave)
        .count()
}

/// Closing the window with two parts unsaved: the work is written as an
/// autosave on the way out, the session restores the saved project with
/// nothing unsaved, and the start asks about the autosave; Keep brings both
/// parts back.
#[test]
fn closing_the_window_leaves_unsaved_work_for_the_next_start_to_offer() {
    let folder = unique_temp_dir("autosave-window-closed");
    let file = folder.join(PROJECT);
    let mut closing = RSpiceApp::test_instance();
    assert!(closing.state.save_project_to_path(&file));
    let saved_parts = parts(&closing);
    edit(&mut closing, 1);
    edit(&mut closing, 2);
    // What closing the window does: the session is stored, then the exit.
    closing.state.sync_active_schematic_to_workspace();
    let mut storage = MemoryStorage::default();
    closing.state.store_session(&mut storage);
    eframe::App::on_exit(&mut closing);
    drop(closing);
    assert_eq!(autosaves_beside(&file), 1, "the unsaved work was autosaved");

    let mut restored = AppState::restore_eframe_session(&storage).expect("the session restores");
    restored.open_session_project();
    let mut app = RSpiceApp::test_instance();
    app.state = restored;
    assert_eq!(parts(&app), saved_parts, "the saved project opened");
    assert!(
        !has_unsaved_changes_fresh(&app.state),
        "the session brought back nothing unsaved"
    );
    let ctx = offscreen();
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Recover unsaved changes?")
    });
    assert!(
        painted(&asked).contains("differs from the saved project in 1 document"),
        "{}",
        painted(&asked)
    );
    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Keep the recovered changes"),
    );
    app_frame(&ctx, &mut app, Vec::new());
    assert_eq!(parts(&app), saved_parts + 2, "both parts are back");
    assert!(has_unsaved_changes(&app.state), "as unsaved work");
    let _ = std::fs::remove_dir_all(folder);
}

/// File > Exit asks about unsaved work first; whatever the answer, the
/// autosave this session wrote goes with it and nothing is offered next time.
#[test]
fn an_exit_that_asked_about_unsaved_work_leaves_no_autosave() {
    let folder = unique_temp_dir("autosave-reviewed-exit");
    let file = folder.join(PROJECT);
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.save_project_to_path(&file));
    edit(&mut app, 4);
    app.state
        .platform
        .project_autosave
        .arm_at(Instant::now() - Duration::from_secs(3600));
    let ctx = offscreen();
    frames_until(&ctx, &mut app, |app| {
        !app.state.platform.project_autosave.written().is_empty()
    });
    assert_eq!(autosaves_beside(&file), 1);
    // The reader answered "Don't save" to File > Exit.
    app.state.shell.exit_requested = true;
    app.process_exit_request(&ctx);
    eframe::App::on_exit(&mut app);
    drop(app);
    assert_eq!(autosaves_beside(&file), 0, "the given-up work left nothing");
    let _ = std::fs::remove_dir_all(folder);
}

/// A save of the project removes the autosave this session wrote; closing
/// the project after giving up its changes does too.
#[test]
fn the_autosave_this_session_wrote_goes_when_its_work_is_saved_or_given_up() {
    let folder = unique_temp_dir("autosave-own");
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.save_project_to_path(&folder.join(PROJECT)));
    let ctx = offscreen();
    let autosave_now = |app: &mut RSpiceApp| {
        edit(app, 3);
        assert!(has_unsaved_changes_fresh(&app.state));
        app.state
            .platform
            .project_autosave
            .arm_at(Instant::now() - Duration::from_secs(3600));
    };
    let autosaves = |app: &RSpiceApp| {
        list(&app.state.model)
            .expect("listed")
            .checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.reason() == ProjectCheckpointReason::Autosave)
            .count()
    };

    autosave_now(&mut app);
    frames_until(&ctx, &mut app, |app| {
        !app.state.platform.project_autosave.written().is_empty()
    });
    assert_eq!(autosaves(&app), 1);
    assert!(app.state.save_project_to_path(&folder.join(PROJECT)));
    frames_until(&ctx, &mut app, |app| {
        autosaves(app) == 0 && app.state.platform.project_autosave.written().is_empty()
    });

    autosave_now(&mut app);
    frames_until(&ctx, &mut app, |app| {
        !app.state.platform.project_autosave.written().is_empty()
    });
    assert_eq!(autosaves(&app), 1);
    let mut reopened = RSpiceApp::test_instance();
    assert!(app.state.close_project_discard(
        rspice_app_services::workbench::lifecycle::project_lifecycle::ProjectCloseDestination::EmptyWorkbench
    ));
    assert!(reopened.state.load_project_from_path(&folder.join(PROJECT)));
    frames_until(&ctx, &mut reopened, |reopened| {
        autosaves(reopened) == 0 && !reopened.state.platform.project_autosave.busy()
    });
    assert!(
        reopened.state.platform.project_autosave.offer.is_none(),
        "work given up is not offered back"
    );
    let _ = std::fs::remove_dir_all(folder);
}
