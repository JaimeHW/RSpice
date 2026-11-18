//! Libraries kept in folders of their own, through the application: the
//! window regaining focus marks what another writer saved and reloads
//! nothing, Refresh… reads again only the cellviews picked, Save refuses a
//! cellview edited here that changed on disk until the reader chooses, a
//! cellview with edits here holds its lock and one locked elsewhere is
//! never written, the first edit takes the lock and another session sees
//! who holds it before it edits, an edit a crash recovery puts back counts
//! as unsaved and takes its lock once edited again, naming a missing
//! library's root on the Paths page reads it at once, every status reads
//! alike on the Dependencies page's rail, table and inspector and in the
//! Overview, and a reference this session cannot read is saved back as it
//! was.

#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::workbench::{RSpiceApp, features};
use rspice_app_models::workbench::models::pdk_workflow::apply_pdk_configuration_with_persistence;
use rspice_app_project::workbench::project::DependencyLibrary;
use rspice_app_project::workbench::project::commands::ProjectRequest;
use rspice_app_project::workbench::project::library_dialogs::library_files::{
    LibraryFileIntent, run_library_file_intent,
};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::surfaces::project::overview::{OverviewSnapshot, Tone};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    ProjectCloseDestination, has_unsaved_changes_fresh,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{Cell, CellViewRef, Library, Point, SchematicState, View, ViewType};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;
use rspice_design::library::folder::{
    CellViewStatus, LibraryFile, LibraryFileLock, LibraryMode, LibraryPath, ReferenceStatus,
};

use super::dependencies::said_about;
use super::recovery_frames::{app_frame, click_on, offscreen, paint_until, painted};
use super::with_cx;

const LIBRARY: &str = "work";
const CELLS: [&str; 2] = ["amp", "stage"];

/// The folders a test writes under, removed when it ends.
struct Folders {
    root: PathBuf,
    /// The project file of the session under test.
    project: PathBuf,
    /// The folder the library `work` is kept in.
    folder: PathBuf,
}

impl Folders {
    /// The project file of another writer.
    fn elsewhere(&self) -> PathBuf {
        self.root.join("other").join("other.rspiceproj")
    }

    fn bytes(&self, cell: &str) -> Vec<u8> {
        std::fs::read(self.folder.join(file(cell).relative_path())).expect("the cellview's file")
    }
}

impl Drop for Folders {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn file(cell: &str) -> LibraryFile {
    LibraryFile::cell_view(cell, &View::new("schematic", ViewType::Schematic))
}

/// An open project whose library `work`, an amplifier and a stage with a
/// schematic each, is kept in a folder of its own, editable, drawn for
/// `technology` (none when empty); as just saved, on the Project workspace.
fn kept_in_a_folder_for(label: &str, technology: &str) -> (RSpiceApp, Folders) {
    let root = unique_temp_dir(label);
    let folders = Folders {
        project: root.join("design").join("design.rspiceproj"),
        folder: root.join("libraries").join(LIBRARY),
        root,
    };
    std::fs::create_dir_all(folders.project.parent().expect("a folder"))
        .expect("the project's folder");
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let mut library = Library::new(LIBRARY);
    library.technology = technology.to_owned();
    for name in CELLS {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
        app.state.model.workspace.insert_schematic_editor(
            CellViewRef::new(LIBRARY, name, "schematic").key(),
            SchematicState::default(),
        );
    }
    app.state.model.library_manager.add_library(library);
    assert!(
        app.state.save_project_to_path(&folders.project),
        "the project is saved"
    );
    app.state
        .model
        .move_library_to_folder(LIBRARY, &folders.folder)
        .expect("work moves to a folder of its own");
    assert!(
        app.state.save_project_to_path(&folders.project),
        "the project is saved with the library in its folder"
    );
    app.state.workbench.activate(Workspace::Project);
    (app, folders)
}

fn kept_in_a_folder(label: &str) -> (RSpiceApp, Folders) {
    kept_in_a_folder_for(label, "")
}

/// A second project with the same folder attached, editable: another
/// writer.
fn another_writer(folders: &Folders) -> RSpiceApp {
    let project = folders.elsewhere();
    std::fs::create_dir_all(project.parent().expect("a folder")).expect("its folder");
    let mut writer = RSpiceApp::test_instance();
    writer.state.open_project_session();
    assert!(writer.state.save_project_to_path(&project), "it is saved");
    let model = &mut writer.state.model;
    let path = model
        .library_path_choice(&folders.folder)
        .expect("a path for the folder")
        .stored;
    model
        .attach_library(&folders.folder, path)
        .expect("the folder attaches");
    model
        .set_library_mode(LIBRARY, LibraryMode::Editable)
        .expect("it becomes editable");
    writer
}

/// Another writer saves `note` into each of `cells`' schematics.
fn saved_elsewhere(writer: &mut RSpiceApp, folders: &Folders, cells: &[&str], note: &str) {
    for cell in cells {
        annotate(writer, cell, note);
    }
    assert!(
        writer.state.save_project_to_path(&folders.elsewhere()),
        "the other writer saves"
    );
}

/// Change what `cell`'s schematic records, as an edit would.
fn annotate(app: &mut RSpiceApp, cell: &str, note: &str) {
    app.state
        .model
        .library_manager
        .get_library_mut(LIBRARY)
        .expect("work is in the catalog")
        .cells
        .get_mut(cell)
        .expect("the cell")
        .views
        .get_mut("schematic")
        .expect("its schematic")
        .metadata
        .insert("note".to_owned(), note.to_owned());
}

fn note(app: &RSpiceApp, cell: &str) -> Option<String> {
    app.state.model.library_manager.get_library(LIBRARY)?.cells[cell].views["schematic"]
        .metadata
        .get("note")
        .cloned()
}

fn status(app: &RSpiceApp, cell: &str) -> Option<CellViewStatus> {
    app.state.model.library_file_status(LIBRARY, &file(cell))
}

/// One frame in which the window regains focus.
fn focus(app: &mut RSpiceApp) {
    app_frame(&offscreen(), app, vec![egui::Event::WindowFocused(true)]);
}

/// A press on the table row that `prefix` starts the announcement of.
fn click_row(output: &egui::FullOutput, prefix: &str) -> Vec<egui::Event> {
    let label = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| {
            node.label()
                .filter(|label| label.starts_with(prefix))
                .map(str::to_owned)
        })
        .unwrap_or_else(|| panic!("the frame announces a row `{prefix}…`"));
    click_on(output, &label)
}

fn row(cell: &str) -> String {
    format!("Library: {LIBRARY}, Cell: {cell}, ")
}

/// The project opened again in a new session, the first one closed.
fn reopened(app: RSpiceApp, folders: &Folders) -> RSpiceApp {
    drop(app);
    let mut reopened = RSpiceApp::test_instance();
    assert!(
        reopened.state.load_project_from_path(&folders.project),
        "the project opens"
    );
    reopened.state.workbench.activate(Workspace::Project);
    reopened
}

/// Spec test 6: another writer's save reads Changed on just its cellview
/// once the window regains focus, and nothing is read again until Refresh…,
/// which reads only the cellviews left picked.
#[test]
fn a_focus_check_marks_what_another_writer_saved_and_refresh_reads_only_the_picked() {
    let (mut app, folders) = kept_in_a_folder("library-focus-check");
    let mut writer = another_writer(&folders);
    saved_elsewhere(&mut writer, &folders, &["amp"], "theirs");

    let ctx = offscreen();
    app_frame(&ctx, &mut app, Vec::new());
    assert!(
        matches!(status(&app, "amp"), Some(CellViewStatus::Found { .. })),
        "a frame reads no folder"
    );
    focus(&mut app);
    assert_eq!(
        status(&app, "amp"),
        Some(CellViewStatus::Changed {
            loaded: 1,
            on_disk: 2
        })
    );
    assert!(matches!(
        status(&app, "stage"),
        Some(CellViewStatus::Found { revision: 1, .. })
    ));
    assert_eq!(
        app.state.model.reference_status(LIBRARY),
        Some(ReferenceStatus::Changed { cellviews: 1 })
    );
    assert_eq!(note(&app, "amp"), None, "what was loaded stays loaded");

    saved_elsewhere(&mut writer, &folders, &["stage"], "theirs");
    focus(&mut app);
    with_cx(&mut app, |cx| {
        run_library_file_intent(cx, LibraryFileIntent::Refresh);
    });
    app_frame(&ctx, &mut app, Vec::new());
    let output = app_frame(&ctx, &mut app, Vec::new());
    let press = click_row(&output, &row("amp"));
    let output = app_frame(&ctx, &mut app, press);
    let press = click_on(&output, "Reload picked");
    app_frame(&ctx, &mut app, press);

    assert!(!app.state.project.library_files.is_open(), "Refresh closes");
    assert_eq!(note(&app, "amp").as_deref(), Some("theirs"), "the picked");
    assert_eq!(note(&app, "stage"), None, "the one left unpicked");
    assert!(matches!(
        status(&app, "amp"),
        Some(CellViewStatus::Found { revision: 2, .. })
    ));
    assert_eq!(
        status(&app, "stage"),
        Some(CellViewStatus::Changed {
            loaded: 1,
            on_disk: 2
        })
    );
}

/// Spec test 7: a cellview edited here that another writer saved is never
/// written over; Save refuses until the reader chooses, and opens the
/// choice. Keep my version writes that cellview at the next save, and Take
/// the file's version reads that cellview, each touching no other.
#[test]
fn an_edited_cellview_changed_on_disk_is_never_overwritten_and_each_choice_acts_on_it_alone() {
    let (mut app, folders) = kept_in_a_folder("library-changed-since-editing");
    let project = std::fs::read(&folders.project).expect("the project file");
    for cell in CELLS {
        annotate(&mut app, cell, "mine");
    }
    let mut writer = another_writer(&folders);
    saved_elsewhere(&mut writer, &folders, &CELLS, "theirs");
    let theirs = CELLS.map(|cell| folders.bytes(cell));

    assert!(
        !app.state.save_project_to_path(&folders.project),
        "Save refuses"
    );
    assert_eq!(
        CELLS.map(|cell| folders.bytes(cell)),
        theirs,
        "nothing is written over"
    );
    assert_eq!(
        std::fs::read(&folders.project).expect("the project file"),
        project,
        "nor the project file"
    );
    for cell in CELLS {
        assert_eq!(
            status(&app, cell),
            Some(CellViewStatus::ChangedSinceEditing {
                loaded: 1,
                on_disk: 2
            })
        );
    }

    // The refusal asks which version to keep, the first cellview chosen.
    let ctx = offscreen();
    app_frame(&ctx, &mut app, Vec::new());
    let output = app_frame(&ctx, &mut app, Vec::new());
    let press = click_on(&output, "Keep my version");
    let output = app_frame(&ctx, &mut app, press);
    let press = click_row(&output, &row("stage"));
    let output = app_frame(&ctx, &mut app, press);
    let press = click_on(&output, "Take the file's version");
    app_frame(&ctx, &mut app, press);

    assert_eq!(note(&app, "amp").as_deref(), Some("mine"), "kept");
    assert_eq!(note(&app, "stage").as_deref(), Some("theirs"), "taken");
    assert_eq!(CELLS.map(|cell| folders.bytes(cell)), theirs, "no save yet");

    assert!(
        app.state.save_project_to_path(&folders.project),
        "Save writes once each is chosen"
    );
    let amp = String::from_utf8(folders.bytes("amp")).expect("text");
    assert!(amp.contains("mine"), "my version is written: {amp}");
    assert_eq!(
        folders.bytes("stage"),
        theirs[1],
        "the file's version stays"
    );
    assert!(matches!(
        status(&app, "amp"),
        Some(CellViewStatus::Found { revision: 3, .. })
    ));
    assert!(matches!(
        status(&app, "stage"),
        Some(CellViewStatus::Found { revision: 2, .. })
    ));
}

/// Take back the edit `annotate` made to `cell`'s schematic.
fn unannotate(app: &mut RSpiceApp, cell: &str) {
    app.state
        .model
        .library_manager
        .get_library_mut(LIBRARY)
        .expect("work is in the catalog")
        .cells
        .get_mut(cell)
        .expect("the cell")
        .views
        .get_mut("schematic")
        .expect("its schematic")
        .metadata
        .remove("note");
}

/// Spec test 4, the locks: a cellview with edits here holds its lock while
/// they are unsaved and lets go once they are gone; one another project
/// holds reads "Being edited by …" on its row, and Save refuses it, writing
/// nothing.
#[test]
fn an_edited_cellview_holds_its_lock_and_one_locked_elsewhere_is_never_written() {
    let (mut app, folders) = kept_in_a_folder("library-edit-locks");
    let lock = |cell: &str| folders.folder.join(file(cell).lock_path());

    annotate(&mut app, "amp", "mine");
    focus(&mut app);
    assert!(lock("amp").exists(), "the edit holds its lock");
    assert!(!lock("stage").exists(), "a cellview not edited takes none");
    unannotate(&mut app, "amp");
    focus(&mut app);
    assert!(!lock("amp").exists(), "the edit gone, so is its lock");

    let holder = LibraryFileLock {
        user: "jsmith".to_owned(),
        computer: "LAB-PC3".to_owned(),
        process: 4242,
        project: "elsewhere.rspiceproj".to_owned(),
        since_unix_ms: 0,
    };
    rspice_project::library_folder::take_library_lock(
        &folders.folder,
        &file("stage"),
        &holder,
        &|_| None,
    )
    .expect("another project locks the stage");
    annotate(&mut app, "amp", "mine");
    annotate(&mut app, "stage", "mine");
    let theirs = CELLS.map(|cell| folders.bytes(cell));
    let project = std::fs::read(&folders.project).expect("the project file");
    focus(&mut app);
    assert!(lock("amp").exists(), "amp is locked here");
    assert_eq!(
        app.state
            .model
            .library_lock_elsewhere(LIBRARY, &file("stage")),
        Some(&holder),
        "the stage stays the other project's"
    );

    app.state.project.project_page = ProjectPage::Library;
    app.state
        .model
        .library_manager
        .select_view(LIBRARY, "stage", "schematic");
    let ctx = offscreen();
    app_frame(&ctx, &mut app, Vec::new());
    let output = app_frame(&ctx, &mut app, Vec::new());
    let announced = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect::<Vec<_>>();
    assert!(
        announced
            .iter()
            .any(|label| label == "schematic, schematic, Being edited by jsmith on LAB-PC3"),
        "the stage's view row names its holder: {announced:#?}"
    );

    assert!(
        !app.state.save_project_to_path(&folders.project),
        "Save refuses"
    );
    assert_eq!(
        CELLS.map(|cell| folders.bytes(cell)),
        theirs,
        "nothing is written over"
    );
    assert_eq!(
        std::fs::read(&folders.project).expect("the project file"),
        project,
        "nor the project file"
    );
    assert_eq!(
        note(&app, "stage").as_deref(),
        Some("mine"),
        "the edit stays"
    );
}

/// Bring `cell`'s schematic to the front of the Design workspace, opened to
/// edit, as the Library page's Open does.
fn open(app: &mut RSpiceApp, cell: &str) {
    let schematic = CellViewRef::new(LIBRARY, cell, "schematic");
    app.state
        .workbench
        .outbox
        .push(ProjectRequest::OpenView(schematic, ViewType::Schematic));
    app.after_contribution();
}

/// Draw a wire in the schematic in front, as the canvas does.
fn draw(app: &mut RSpiceApp) -> bool {
    app.state
        .model
        .schematic
        .with_undo("draw a wire", |schematic| {
            schematic.add_wire(vec![Point::new(0, 0), Point::new(100, 0)]);
        })
}

/// The first edit of a cellview takes its file's lock, before any check or
/// Save. Another session that brings the same cellview to the front reads
/// "Being edited by …" over the canvas and refuses an edit there, naming
/// the holder, before any work is made to lose. Once the holder closes it,
/// that session can edit it, and its own first edit takes the lock.
#[test]
fn the_first_edit_takes_the_lock_and_another_session_sees_the_holder_before_editing() {
    let (mut app, folders) = kept_in_a_folder("library-first-edit-lock");
    let lock = folders.folder.join(file("amp").lock_path());
    let ctx = offscreen();
    open(&mut app, "amp");
    app_frame(&ctx, &mut app, Vec::new());
    assert!(!lock.exists(), "opening the cellview takes no lock");
    assert!(draw(&mut app), "the cellview is editable");
    app_frame(&ctx, &mut app, Vec::new());
    assert!(lock.exists(), "the first edit takes the lock");
    let holder = rspice_project::library_folder::read_library_lock(&folders.folder, &file("amp"))
        .expect("the lock names its holder")
        .holder();

    let mut other = another_writer(&folders);
    open(&mut other, "amp");
    let other_ctx = offscreen();
    let banner = format!("Being edited by {holder}. Read only until they save or close it.");
    paint_until(&other_ctx, &mut other, |text| text.contains(&banner));
    assert!(other.state.model.active_view_read_only());
    other.execute_command_key(WorkbenchCommand::Delete.key());
    assert_eq!(
        other
            .state
            .workbench
            .prompt
            .as_ref()
            .map(|prompt| prompt.text.trim_end_matches('\u{a0}')),
        Some(format!("Being edited by {holder} - read only until they save or close it").as_str()),
        "an edit there is refused, naming the holder"
    );
    // Its drawing is unsaved there all the same: that project's file does
    // not hold the cell yet. The refusal leaves the drawing as it was.
    assert!(
        !other.state.model.schematic.can_undo(),
        "and nothing changes"
    );

    assert!(
        app.state
            .close_project_discard(ProjectCloseDestination::EmptyWorkbench)
    );
    assert!(!lock.exists(), "closing it lets the lock go");
    focus(&mut other);
    assert!(
        !other.state.model.active_view_read_only(),
        "the other session can edit it now"
    );
    let output = app_frame(&other_ctx, &mut other, Vec::new());
    assert!(!painted(&output).contains(&banner), "the banner is gone");
    assert!(draw(&mut other), "it edits");
    app_frame(&other_ctx, &mut other, Vec::new());
    assert!(
        other
            .state
            .model
            .held_library_lock(LIBRARY, &file("amp"))
            .is_some()
            && lock.exists(),
        "its first edit takes the lock"
    );
}

/// Let the autosave write `app`'s unsaved edits, then end the session as a
/// crash would, saving nothing.
fn autosave_and_crash(mut app: RSpiceApp) {
    assert!(
        has_unsaved_changes_fresh(&app.state),
        "there is unsaved work"
    );
    // A one-minute interval, due now, so the next frame writes the autosave.
    app.state.platform.autosave_minutes = 1;
    app.state.platform.project_autosave.arm_due();
    let ctx = offscreen();
    let deadline = Instant::now() + Duration::from_secs(60);
    while app.state.platform.project_autosave.written().is_empty() {
        assert!(Instant::now() < deadline, "the autosave was never written");
        app_frame(&ctx, &mut app, Vec::new());
        std::thread::sleep(Duration::from_millis(2));
    }
    drop(app);
}

/// The project opened again after a crash, keeping the recovered changes.
fn recovered(folders: &Folders) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state.load_project_from_path(&folders.project),
        "the project opens"
    );
    let ctx = offscreen();
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Recover unsaved changes?")
    });
    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Keep the recovered changes"),
    );
    app_frame(&ctx, &mut app, Vec::new());
    app.state.workbench.activate(Workspace::Project);
    app
}

/// The Overview's Unsaved work, and what the close review names.
fn unsaved(app: &mut RSpiceApp) -> (String, String) {
    let snapshot = OverviewSnapshot::capture(
        &egui::Context::default(),
        &features::cx_with_foreign_answers(app, |_| true),
    );
    let row = snapshot
        .statuses
        .iter()
        .find(|status| status.area == "Unsaved work")
        .expect("the Overview has an Unsaved work row")
        .state()
        .to_owned();
    app.state
        .workbench
        .project_review_dialog
        .show_close_project(ProjectCloseDestination::EmptyWorkbench);
    let ctx = offscreen();
    let review = paint_until(&ctx, app, |text| text.contains("Save changes to"));
    app.state.workbench.project_review_dialog = Default::default();
    (row, painted(&review))
}

fn named(cell: &str) -> String {
    format!("{LIBRARY} \u{b7} {cell} \u{b7} schematic")
}

/// An edit a crash recovery puts back counts as unsaved work, one document
/// per cellview, and the close review names each; it takes no lock until
/// the user edits that cellview again, when it does. One another project
/// holds by then reads "Being edited by …", and Save refuses it.
#[test]
fn a_recovered_edit_counts_as_unsaved_and_takes_its_lock_once_edited_again() {
    let (mut app, folders) = kept_in_a_folder("library-recovered-edit");
    let lock = |cell: &str| folders.folder.join(file(cell).lock_path());
    for cell in CELLS {
        annotate(&mut app, cell, "mine");
    }
    autosave_and_crash(app);

    let mut app = recovered(&folders);
    assert_eq!(
        note(&app, "amp").as_deref(),
        Some("mine"),
        "amp's edit is back"
    );
    assert_eq!(
        note(&app, "stage").as_deref(),
        Some("mine"),
        "so is the stage's"
    );
    let (row, review) = unsaved(&mut app);
    assert_eq!(row, "2 unsaved documents");
    for cell in CELLS {
        assert!(
            review.contains(&named(cell)),
            "the review names {cell}:\n{review}"
        );
    }
    focus(&mut app);
    assert!(
        !lock("amp").exists() && !lock("stage").exists(),
        "a recovered edit takes no lock of itself"
    );

    rspice_project::library_folder::take_library_lock(
        &folders.folder,
        &file("stage"),
        &LibraryFileLock {
            user: "jsmith".to_owned(),
            computer: "LAB-PC3".to_owned(),
            process: 4242,
            project: "elsewhere.rspiceproj".to_owned(),
            since_unix_ms: 0,
        },
        &|_| None,
    )
    .expect("another project locks the stage");
    annotate(&mut app, "amp", "mine, again");
    focus(&mut app);
    assert!(lock("amp").exists(), "editing amp again takes its lock");
    assert_eq!(
        app.state
            .model
            .library_lock_elsewhere(LIBRARY, &file("stage"))
            .map(LibraryFileLock::holder)
            .as_deref(),
        Some("jsmith on LAB-PC3"),
        "the stage reads as the other project's"
    );
    let theirs = CELLS.map(|cell| folders.bytes(cell));
    assert!(
        !app.state.save_project_to_path(&folders.project),
        "Save refuses the stage"
    );
    assert_eq!(
        CELLS.map(|cell| folders.bytes(cell)),
        theirs,
        "nothing is written over"
    );
}

/// An edit a recovery holds for a library whose folder cannot be read now
/// is unsaved work all the same: the Overview counts it and the close
/// review names it.
#[test]
fn a_recovered_edit_to_a_library_out_of_reach_counts_as_unsaved() {
    let (mut app, folders) = kept_in_a_folder("library-recovered-unread");
    annotate(&mut app, "amp", "mine");
    autosave_and_crash(app);
    std::fs::rename(&folders.folder, folders.root.join("away"))
        .expect("the folder is out of reach");

    let mut app = recovered(&folders);
    assert_eq!(
        app.state.model.library_files.pending().len(),
        1,
        "the edit is held"
    );
    let (row, review) = unsaved(&mut app);
    assert_eq!(row, "1 unsaved document");
    assert!(
        review.contains(&named("amp")),
        "the review names amp:\n{review}"
    );
}

/// The path variable the tests name the folder of `work` by.
const ROOT: &str = "RSPICE_TEST_LIBRARY_ROOT";

/// As [`kept_in_a_folder`], with `work` saved as found under the path
/// variable [`ROOT`], which nothing here names yet.
fn kept_under_a_root(label: &str) -> (RSpiceApp, Folders) {
    let (mut app, folders) = kept_in_a_folder(label);
    let model = &mut app.state.model;
    let mut reference = model
        .library_manager
        .reference(LIBRARY)
        .cloned()
        .expect("work is kept in a folder of its own");
    reference.path = LibraryPath::new(&format!("${ROOT}/{LIBRARY}")).expect("a path");
    model.library_manager.insert_reference(reference);
    assert!(
        app.state.save_project_to_path(&folders.project),
        "the project is saved naming the library by its root"
    );
    (app, folders)
}

/// Name [`ROOT`] on the Paths page, as Apply does.
fn name_the_root(app: &mut RSpiceApp, folders: &Folders) {
    let mut config = app.state.model.pdk_config.clone();
    config.environment_variables.insert(
        ROOT.to_owned(),
        folders
            .folder
            .parent()
            .expect("the libraries' folder")
            .display()
            .to_string(),
    );
    apply_pdk_configuration_with_persistence(
        &mut app.state.model,
        &app.state.platform,
        config,
        |_| Ok(()),
    )
    .expect("the paths apply");
}

/// A library whose root no path variable names here reads Not found;
/// naming it on the Paths page reads it at once, with its cells, and every
/// place says Found, with no reopen or Refresh.
#[test]
fn naming_a_missing_librarys_root_on_the_paths_page_reads_it_at_once() {
    let (app, folders) = kept_under_a_root("library-paths-root");
    let mut app = reopened(app, &folders);
    assert!(
        matches!(
            app.state.model.reference_status(LIBRARY),
            Some(ReferenceStatus::NotFound(_))
        ),
        "nothing names the root yet"
    );

    name_the_root(&mut app, &folders);
    let library = app
        .state
        .model
        .library_manager
        .get_library(LIBRARY)
        .expect("work is in the catalog");
    assert!(
        CELLS.iter().all(|cell| library.cells.contains_key(*cell)),
        "its cells are read"
    );
    assert_every_place_says(&mut app, "Found", None);
}

/// An edit a recovery held for a library out of reach is put back over it,
/// unsaved, once the Paths page names its root.
#[test]
fn a_held_edit_goes_back_over_a_library_the_paths_page_finds() {
    let (mut app, folders) = kept_under_a_root("library-paths-held-edit");
    annotate(&mut app, "amp", "mine");
    autosave_and_crash(app);
    let mut app = recovered(&folders);
    assert_eq!(
        app.state.model.library_files.pending().len(),
        1,
        "the edit is held"
    );

    name_the_root(&mut app, &folders);
    assert!(
        app.state.model.library_files.pending().is_empty(),
        "the reading takes it"
    );
    assert_eq!(
        note(&app, "amp").as_deref(),
        Some("mine"),
        "the edit is back"
    );
    assert_eq!(note(&app, "stage"), None);
    assert_eq!(
        app.state.model.unsaved_library_files(),
        [(LIBRARY.to_owned(), file("amp"))],
        "unsaved, on amp alone"
    );
}

/// What the Dependencies page's rail, table and inspector say about `work`,
/// and the Overview's Dependencies row, agree with `label` and `mark`.
fn assert_every_place_says(app: &mut RSpiceApp, label: &str, mark: Option<&str>) {
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    let expected = (label.to_owned(), mark.map(str::to_owned));
    assert_eq!(
        said_about(app, &DependencyLibrary::design(LIBRARY)),
        [expected.clone(), expected.clone(), expected],
        "the rail, the table and the inspector"
    );
    let snapshot = OverviewSnapshot::capture(
        &egui::Context::default(),
        &features::cx_with_foreign_answers(app, |_| true),
    );
    let overview = snapshot
        .statuses
        .iter()
        .find(|status| status.area == "Dependencies")
        .expect("the Overview has a Dependencies row");
    let status = app
        .state
        .model
        .reference_status(LIBRARY)
        .expect("work has a status");
    assert_eq!(status.label(), label, "the one status function");
    match mark {
        None => {
            assert_eq!(overview.tone(), Tone::Neutral, "{}", overview.text());
            assert!(!overview.text().contains(LIBRARY), "{}", overview.text());
        }
        Some(mark) => {
            let tone = if mark == "Warning" {
                Tone::Warn
            } else {
                Tone::Error
            };
            assert_eq!(
                (overview.tone(), overview.text()),
                (
                    tone,
                    format!(
                        "{LIBRARY}: {} \u{b7} {}",
                        label.to_lowercase(),
                        status.detail().unwrap_or_default()
                    )
                ),
                "the Overview"
            );
        }
    }
}

/// Spec test 10, one test per status.
#[test]
fn a_library_held_in_the_project_reads_in_project_in_every_place() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state
        .model
        .library_manager
        .add_library(Library::new(LIBRARY));
    assert_every_place_says(&mut app, "In project", None);
}

#[test]
fn a_library_read_from_its_folder_reads_found_in_every_place() {
    let (mut app, _folders) = kept_in_a_folder("library-status-found");
    assert_every_place_says(&mut app, "Found", None);
}

#[test]
fn a_library_another_writer_saved_reads_changed_in_every_place() {
    let (mut app, folders) = kept_in_a_folder("library-status-changed");
    let mut writer = another_writer(&folders);
    saved_elsewhere(&mut writer, &folders, &["amp"], "theirs");
    focus(&mut app);
    assert_every_place_says(&mut app, "Changed", Some("Warning"));
}

#[test]
fn a_library_edited_here_and_saved_elsewhere_reads_changed_since_editing_in_every_place() {
    let (mut app, folders) = kept_in_a_folder("library-status-editing");
    annotate(&mut app, "amp", "mine");
    let mut writer = another_writer(&folders);
    saved_elsewhere(&mut writer, &folders, &["amp"], "theirs");
    focus(&mut app);
    assert_every_place_says(
        &mut app,
        rspice_design::library::folder::CHANGED_SINCE_EDITING,
        Some("Error"),
    );
}

#[test]
fn a_library_drawn_for_another_technology_reads_technology_differs_in_every_place() {
    let (mut app, folders) = kept_in_a_folder_for("library-status-technology", "gpdk045");
    app.state.model.workspace.content.project.technology = Some("gpdk090".to_owned());
    assert!(
        app.state.save_project_to_path(&folders.project),
        "the project is saved for another technology"
    );
    let mut app = reopened(app, &folders);
    assert_every_place_says(&mut app, "Technology differs", Some("Warning"));
}

#[test]
fn a_library_whose_folder_is_gone_reads_not_found_in_every_place() {
    let (mut app, folders) = kept_in_a_folder("library-status-not-found");
    std::fs::remove_dir_all(&folders.folder).expect("the folder goes");
    focus(&mut app);
    assert_every_place_says(&mut app, "Not found", Some("Error"));
}

#[test]
fn a_library_with_a_damaged_file_reads_unreadable_in_every_place() {
    let (mut app, folders) = kept_in_a_folder("library-status-unreadable");
    std::fs::write(
        folders.folder.join(file("amp").relative_path()),
        b"{ not a cellview",
    )
    .expect("the file is damaged");
    focus(&mut app);
    assert_every_place_says(&mut app, "Unreadable", Some("Error"));
}

#[test]
fn a_folder_holding_another_library_reads_different_library_in_every_place() {
    let (app, folders) = kept_in_a_folder("library-status-different");
    // Another project moves a library of its own out to a folder, which
    // then takes `work`'s place.
    let mut other = RSpiceApp::test_instance();
    other.state.open_project_session();
    let stray = folders.root.join("stray").join("other");
    let mut library = Library::new("other");
    library.add_cell(Cell::new("amp"));
    other.state.model.library_manager.add_library(library);
    other
        .state
        .model
        .move_library_to_folder("other", &stray)
        .expect("other moves to a folder of its own");
    std::fs::remove_dir_all(&folders.folder).expect("work's folder goes");
    std::fs::rename(&stray, &folders.folder).expect("other's takes its place");
    let mut app = reopened(app, &folders);
    assert_every_place_says(&mut app, "Different library", Some("Error"));
}

/// Every file under `folder`, by its path inside it, as a browser's folder
/// upload hands them over.
fn uploaded(folder: &Path) -> Vec<(String, Vec<u8>)> {
    fn walk(root: &Path, at: &Path, files: &mut Vec<(String, Vec<u8>)>) {
        for entry in std::fs::read_dir(at).expect("a folder") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                walk(root, &path, files);
            } else {
                let inside = path
                    .strip_prefix(root)
                    .expect("inside")
                    .to_string_lossy()
                    .replace('\\', "/");
                files.push((inside, std::fs::read(&path).expect("its bytes")));
            }
        }
    }
    let mut files = Vec::new();
    walk(folder, folder, &mut files);
    files
}

/// The project file as saved, but for the catalog's revision, which only
/// moves forward, and the order its maps happen to be written in.
fn saved(path: &Path) -> serde_json::Value {
    let mut value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).expect("the project file"))
            .expect("the project file is JSON");
    value["libraries"]["revision"] = serde_json::Value::from(0);
    value
}

/// Spec test 11, as a browser opens a desktop project: a reference this
/// session cannot read by path is listed with no cells and saved back
/// unchanged, and a library read from a picked folder for this session
/// leaves what is saved alone.
#[test]
fn a_reference_this_session_cannot_read_is_saved_back_as_it_was() {
    let (app, folders) = kept_in_a_folder("library-browser-round-trip");
    let before = saved(&folders.project);
    let files = uploaded(&folders.folder);
    let away = folders.root.join("away");
    std::fs::rename(&folders.folder, &away).expect("the folder is out of reach");

    let mut app = reopened(app, &folders);
    assert!(matches!(
        app.state.model.reference_status(LIBRARY),
        Some(ReferenceStatus::NotFound(_))
    ));
    assert!(
        app.state
            .model
            .library_manager
            .get_library(LIBRARY)
            .is_some_and(|library| library.cells.is_empty()),
        "the library is listed, with no cells"
    );
    assert!(
        app.state.save_project_to_path(&folders.project),
        "the project is saved"
    );
    let after = saved(&folders.project);
    assert_eq!(
        after["libraries"]["references"], before["libraries"]["references"],
        "the reference round-trips unchanged"
    );
    assert!(after == before, "and so does the rest of the project");

    app.state
        .model
        .read_library_for_session(LIBRARY, LIBRARY, files)
        .expect("the picked folder is read for this session");
    assert_eq!(
        app.state.model.reference_status(LIBRARY),
        Some(ReferenceStatus::Found {
            mode: LibraryMode::Editable,
            moved_on: 0,
            read_for_session: Some(LIBRARY.to_owned()),
        })
    );
    assert!(
        app.state
            .model
            .library_manager
            .get_library(LIBRARY)
            .is_some_and(|library| CELLS.iter().all(|cell| library.cells.contains_key(*cell))),
        "its cells are in the catalog"
    );
    assert!(
        app.state.save_project_to_path(&folders.project),
        "the project is saved"
    );
    let after = saved(&folders.project);
    assert_eq!(
        after["libraries"]["references"], before["libraries"]["references"],
        "reading the folder for the session leaves the reference alone"
    );
    assert!(after == before, "and the rest of the project");
    assert!(!folders.folder.exists(), "nothing is written by path");
}
