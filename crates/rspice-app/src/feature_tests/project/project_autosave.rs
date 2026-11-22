//! The open project's autosave: written off the frame while there is unsaved
//! work, and offered back once, with its time, when the session that wrote it
//! ended without saving. The session itself keeps no unsaved project work.

#![cfg(not(target_arch = "wasm32"))]

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::recovery_frames::{app_frame, click_on, offscreen, paint_until, painted};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::project::library_dialogs::new_cell::create_new_cell;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::workflows::project_workflow::SaveRequestOutcome;
use rspice_app_services::workbench::lifecycle::project_checkpoint::{
    ProjectCheckpointReason, list,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    SaveScope, active_document_is_dirty, has_unsaved_changes, has_unsaved_changes_fresh,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{CellViewRef, ComponentType, Point};
use rspice_app_state::workbench::workspace::Workspace;

/// The folder the crashed session saves its project in, handed to the child
/// process that plays it.
const CRASH_FOLDER: &str = "RSPICE_TEST_CRASHED_SESSION_FOLDER";
/// Set when the crashed session's unsaved work is a new cell, not a part.
const CRASH_NEW_CELL: &str = "RSPICE_TEST_CRASHED_SESSION_NEW_CELL";
/// Set when the crashed session attaches a technology before its save.
const CRASH_TECHNOLOGY: &str = "RSPICE_TEST_CRASHED_SESSION_TECHNOLOGY";
const PROJECT: &str = "amp.rspiceproj";

fn edit(app: &mut RSpiceApp, index: i32) {
    app.state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Capacitor, Point::new(40 * index, 0));
    });
}

/// A cell named `name` with a schematic, created in the project's library as
/// New cell creates one.
pub(super) fn add_cell(app: &mut RSpiceApp, name: &str) {
    app.state.project.new_cell_library = app
        .state
        .model
        .workspace
        .content
        .project
        .root_library
        .clone();
    app.state.project.new_cell_name = name.to_owned();
    app.state.project.new_cell_create_schematic = true;
    app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
    assert!(super::with_cx(app, create_new_cell), "the cell is created");
    app.after_contribution();
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

/// The session the crash tests end: it saves a project, adds a part (or,
/// asked to, creates a new cell), lets the autosave interval pass and, once
/// the autosave is on disk, stops the process as a crash would, cleaning
/// nothing up. Run only by [`crashed_session`] in a process of its own.
#[test]
#[ignore = "played in a child process by the crash tests"]
fn crashed_session_child() {
    let Some(folder) = std::env::var_os(CRASH_FOLDER) else {
        // The crate keeps `eprintln!` out of the app; a test writes to the
        // runner's stderr directly.
        let _ = writeln!(
            std::io::stderr(),
            "{CRASH_FOLDER} is unset; the crashed session only runs under its parent test"
        );
        return;
    };
    let folder = PathBuf::from(folder);
    let mut app = RSpiceApp::test_instance();
    if std::env::var_os(CRASH_TECHNOLOGY).is_some() {
        app.state.provision_test_project_technology_contract();
    }
    assert!(app.state.save_project_to_path(&folder.join(PROJECT)));
    if std::env::var_os(CRASH_NEW_CELL).is_some() {
        add_cell(&mut app, "crashcell");
    } else {
        edit(&mut app, 7);
    }
    assert!(has_unsaved_changes_fresh(&app.state));
    app.state.platform.project_autosave.arm_due();
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
    crashed_session_making(label, &[])
}

/// [`crashed_session`], whose session also did what each of `asked` (a
/// `CRASH_` setting) asks: create a cell instead of a part, attach a
/// technology.
fn crashed_session_making(label: &str, asked: &[&str]) -> (PathBuf, usize) {
    let folder = unique_temp_dir(label);
    let mut child = std::process::Command::new(std::env::current_exe().expect("the test binary"));
    child
        .args([
            "--exact",
            "feature_tests::project::project_autosave::crashed_session_child",
            "--ignored",
            "--test-threads=1",
        ])
        .env(CRASH_FOLDER, &folder);
    for setting in asked {
        child.env(setting, "1");
    }
    let status = child
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
    // This open looks for recovered work on a thread of its own and pins the
    // autosave it finds, which rewrites the autosave's record. Let it finish,
    // so the open a test makes next is the only one reading that record.
    frames_until(&offscreen(), &mut opened, |app| {
        !app.state.platform.project_autosave.busy()
    });
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

/// Keep, then File > Save: the file takes the recovered work and nothing
/// reads unsaved any more, not the title, the toolbar, a tab or the
/// Overview; an edit made after it and undone back to where the save left it
/// reads clean. Played for a recovered part and for a recovered new cell.
#[test]
fn after_a_crash_keep_then_save_leaves_nothing_unsaved() {
    for (label, new_cell) in [
        ("autosave-crash-keep-save", false),
        ("autosave-crash-keep-save-cell", true),
    ] {
        let (folder, _) = crashed_session_making(
            label,
            if new_cell { &[CRASH_NEW_CELL] } else { &[] },
        );
        let (mut app, ctx, asked) = open_after_crash(&folder);
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
        let _ = std::fs::remove_dir_all(folder);
    }
}

/// A cell the crashed session created is only in the autosave: the question
/// lists it as new in the autosave, as the autosave holds it, not as removed.
#[test]
fn after_a_crash_a_cell_only_the_autosave_holds_is_offered_as_new_in_it() {
    let (folder, _) = crashed_session_making("autosave-crash-cell", &[CRASH_NEW_CELL]);
    let (_app, _ctx, asked) = open_after_crash(&folder);
    let text = painted(&asked);
    assert!(
        text.lines()
            .any(|line| line.starts_with("New in the autosave") && line.contains("crashcell")),
        "{text}"
    );
    assert!(!text.contains("Removed since"), "{text}");
    let _ = std::fs::remove_dir_all(folder);
}

/// A cell only the kept autosave holds has never been saved: the Overview's
/// Cells table says "Unsaved" for when it was modified, not a dash.
#[test]
fn a_kept_cell_never_saved_reads_unsaved_on_the_overview() {
    let (folder, _) = crashed_session_making("autosave-crash-cell-overview", &[CRASH_NEW_CELL]);
    let (mut app, ctx, asked) = open_after_crash(&folder);
    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Keep the recovered changes"),
    );
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Overview;
    let row = |output: &egui::FullOutput| {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .and_then(|update| {
                update.nodes.iter().find_map(|(_, node)| {
                    node.label()
                        .filter(|label| label.starts_with("Cell: crashcell"))
                        .map(str::to_owned)
                })
            })
    };
    let deadline = Instant::now() + Duration::from_secs(60);
    let label = loop {
        let shown = app_frame(&ctx, &mut app, Vec::new());
        if let Some(label) = row(&shown) {
            break label;
        }
        assert!(
            Instant::now() < deadline,
            "no crashcell row:\n{}",
            painted(&shown)
        );
    };
    assert!(label.contains("Modified: Unsaved"), "{label}");
    let _ = std::fs::remove_dir_all(folder);
}

/// A crashed session whose technology this computer does not have: the
/// question says so before anything is kept, and the Console and the
/// Overview's Dependencies row say it in the same words.
#[test]
fn after_a_crash_a_technology_not_installed_here_is_said_in_the_question() {
    let (folder, _) = crashed_session_making("autosave-crash-technology", &[CRASH_TECHNOLOGY]);
    let (app, ctx, asked) = open_after_crash(&folder);
    let model = &app.state.model;
    let notice = model
        .workspace
        .content
        .project
        .technology_binding()
        .and_then(|binding| model.technology_not_installed(binding))
        .expect("the saved project's technology is not installed here");
    assert!(
        notice.ends_with(
            " is not installed on this computer. Simulation is blocked until it is installed."
        ),
        "{notice}"
    );
    assert!(painted(&asked).contains(&notice), "{}", painted(&asked));
    assert!(
        model
            .log_buffer
            .entries()
            .any(|entry| entry.message == notice),
        "the Console says it in the same words"
    );
    let dependencies = rspice_app_project::workbench::project::health::dependencies(&ctx, model);
    assert_eq!(dependencies.state, "Technology: not installed");
    assert_eq!(dependencies.detail, notice);
    let _ = std::fs::remove_dir_all(folder);
}

/// "Open the saved project" leaves the saved project open, clean, and the
/// autosave on the Recovery page until the next save, which its row says:
/// the recovery keeps it, the reader never pinned it.
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
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Recovery;
    let shown = paint_until(&ctx, &mut app, |text| {
        text.contains("Kept for recovery until you save \u{b7} ")
    });
    assert!(!painted(&shown).contains("Pinned"), "{}", painted(&shown));
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

/// Give the open project a second cell with a schematic, named `name`, and
/// return its schematic view.
fn add_cell(app: &mut RSpiceApp, name: &str) -> CellViewRef {
    let library = app
        .state
        .model
        .workspace
        .content
        .project
        .root_library
        .clone();
    let project = &mut app.state.project;
    project.new_cell_library = library.clone();
    project.new_cell_name = name.to_owned();
    project.new_cell_create_schematic = true;
    project.new_cell_library_revision = app.state.model.library_manager.revision();
    assert!(with_cx(app, create_new_cell), "the cell is created");
    app.after_contribution();
    CellViewRef::new(&library, name, "schematic")
}

/// Every edited cell view is in the autosave, not only the one in front:
/// with two cells edited and the interval passed, an open of the project in
/// another session offers both, and Keep brings both edits back.
#[test]
fn the_autosave_holds_every_edited_cell_and_keep_brings_them_back() {
    let folder = unique_temp_dir("autosave-every-cell");
    let file = folder.join(PROJECT);
    let mut app = RSpiceApp::test_instance();
    let top = app.state.model.workspace.content.active_view.clone();
    let second = add_cell(&mut app, "second");
    assert!(app.state.save_project_to_path(&file));
    app.state.open_workspace_view(second.clone());
    let saved_second = parts(&app);
    edit(&mut app, 1);
    app.state.open_workspace_view(top.clone());
    let saved_top = parts(&app);
    edit(&mut app, 2);
    assert!(has_unsaved_changes_fresh(&app.state));
    app.state.platform.project_autosave.arm_due();
    frames_until(&offscreen(), &mut app, |app| {
        !app.state.platform.project_autosave.written().is_empty()
    });
    // The session ends as a crash ends it: nothing cleaned up, and the
    // project's open lock let go with it.
    drop(app);

    let mut reopened = RSpiceApp::test_instance();
    assert!(reopened.state.load_project_from_path(&file));
    let ctx = offscreen();
    let asked = paint_until(&ctx, &mut reopened, |text| {
        text.contains("Recover unsaved changes?")
    });
    assert!(
        painted(&asked).contains("differs from the saved project in 2 documents"),
        "{}",
        painted(&asked)
    );
    app_frame(
        &ctx,
        &mut reopened,
        click_on(&asked, "Keep the recovered changes"),
    );
    app_frame(&ctx, &mut reopened, Vec::new());
    reopened.state.open_workspace_view(top);
    assert_eq!(parts(&reopened), saved_top + 1, "the cell in front is back");
    reopened.state.open_workspace_view(second);
    assert_eq!(parts(&reopened), saved_second + 1, "and the cell behind it");
    let _ = std::fs::remove_dir_all(folder);
}

/// An interval in which nothing changed writes nothing, since the autosave
/// on disk already holds the work; the next edit is written at the next one.
#[test]
fn an_interval_with_nothing_changed_writes_no_autosave() {
    let folder = unique_temp_dir("autosave-idle");
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.save_project_to_path(&folder.join(PROJECT)));
    edit(&mut app, 1);
    let ctx = offscreen();
    app.state.platform.project_autosave.arm_due();
    frames_until(&ctx, &mut app, |app| {
        !app.state.platform.project_autosave.written().is_empty()
    });
    let newest = |app: &RSpiceApp| app.state.platform.project_autosave.written()[0].checkpoint_id();
    let first = newest(&app);

    app.state.platform.project_autosave.arm_due();
    for _ in 0..4 {
        app_frame(&ctx, &mut app, Vec::new());
    }
    frames_until(&ctx, &mut app, |app| {
        !app.state.platform.project_autosave.busy()
    });
    assert_eq!(
        newest(&app),
        first,
        "nothing changed, so nothing was written"
    );

    edit(&mut app, 2);
    app.state.platform.project_autosave.arm_due();
    frames_until(&ctx, &mut app, |app| newest(app) != first);
    let _ = std::fs::remove_dir_all(folder);
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
    app.state.platform.project_autosave.arm_due();
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
        app.state.platform.project_autosave.arm_due();
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
