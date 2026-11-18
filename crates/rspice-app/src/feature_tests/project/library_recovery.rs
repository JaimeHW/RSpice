//! Unsaved edits to a library kept in a folder of its own outlive a crash:
//! the autosave holds them, and keeping the recovered changes puts each back
//! over the folder as it is now, marked edited, with no library file written
//! until a Save.

#![cfg(not(target_arch = "wasm32"))]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::recovery_frames::{app_frame, click_on, offscreen, paint_until};
use crate::workbench::RSpiceApp;
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    has_unsaved_changes, has_unsaved_changes_fresh,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{
    Cell, CellViewRef, ComponentType, Library, OpenCellView, Point, SchematicState, View, ViewType,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_design::library::folder::{
    CHANGED_SINCE_EDITING, CellViewFile, CellViewStatus, LibraryFile, encode_library_file,
};

const PROJECT: &str = "design.rspiceproj";

fn amp() -> CellViewRef {
    CellViewRef::new("parts", "amp", "schematic")
}

fn amp_file() -> LibraryFile {
    LibraryFile::cell_view("amp", &View::new("schematic", ViewType::Schematic))
}

/// A project in `root/design` whose library `parts`, kept in a folder of its
/// own beside it, holds `amp`: open in front, one resistor. Saved and clean.
fn saved_with_library_folder(root: &Path) -> RSpiceApp {
    let design = root.join("design");
    std::fs::create_dir_all(&design).expect("a folder for the project");
    let mut app = RSpiceApp::test_instance();
    let state = &mut app.state;
    let mut parts = Library::new("parts");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    parts.add_cell(cell);
    state.model.library_manager.add_library(parts);
    let mut drawing = SchematicState::default();
    drawing.add_component(ComponentType::Resistor, Point::new(0, 0));
    let workspace = &mut state.model.workspace;
    workspace.insert_schematic_editor(amp().key(), drawing.clone());
    workspace.content.open_views = [OpenCellView::new(amp(), ViewType::Schematic)]
        .into_iter()
        .collect();
    workspace.content.active_view = amp();
    workspace.content.reroot_active_occurrence();
    state.model.schematic = drawing;
    state.model.workspace.mark_all_clean();
    state.model.schematic.session.is_dirty = false;
    state.open_project_session();
    state.workbench.activate(Workspace::Project);
    assert!(state.save_project_to_path(&design.join(PROJECT)));
    state
        .model
        .move_library_to_folder("parts", &design.join("parts"))
        .expect("parts moves to a folder of its own beside the project");
    assert!(state.save_project_to_path(&design.join(PROJECT)));
    assert!(!has_unsaved_changes_fresh(&*state), "saved and clean");
    app
}

/// Every file under `folder`, with its bytes.
fn folder_bytes(folder: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = BTreeMap::new();
    let mut pending = vec![folder.to_path_buf()];
    while let Some(next) = pending.pop() {
        for entry in std::fs::read_dir(&next).expect("the folder reads") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                pending.push(path);
            } else {
                let bytes = std::fs::read(&path).expect("the file reads");
                files.insert(path, bytes);
            }
        }
    }
    files
}

/// How many parts `amp` holds in `app`.
fn amp_parts(app: &RSpiceApp) -> usize {
    let drawing = if app.state.model.workspace.content.active_view == amp() {
        app.state.model.schematic.clone()
    } else {
        app.state
            .model
            .workspace
            .clone_schematic_editor(&amp().key())
            .expect("amp has a drawing")
    };
    drawing.document().components.len()
}

/// Place a capacitor in `amp`, let the autosave write it, and end the
/// session as a crash would, saving nothing.
fn edit_autosave_and_crash(mut app: RSpiceApp) {
    assert_eq!(app.state.model.workspace.content.active_view, amp());
    app.state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Capacitor, Point::new(40, 0));
    });
    assert!(has_unsaved_changes_fresh(&app.state));
    // A one-minute interval, armed a minute back, so the next frame writes
    // the autosave.
    app.state.platform.autosave_minutes = 1;
    let armed = Instant::now()
        .checked_sub(Duration::from_secs(60))
        .expect("the clock has run for a minute");
    app.state.platform.project_autosave.arm_at(armed);
    let ctx = offscreen();
    let deadline = Instant::now() + Duration::from_secs(60);
    while app.state.platform.project_autosave.written().is_empty() {
        assert!(Instant::now() < deadline, "the autosave was never written");
        app_frame(&ctx, &mut app, Vec::new());
        std::thread::sleep(Duration::from_millis(2));
    }
    drop(app);
}

/// Open the project again and keep the recovered changes.
fn reopen_and_keep(project: &Path) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.load_project_from_path(project));
    assert_eq!(amp_parts(&app), 1, "the saved library is what opened");
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
    app
}

fn edited(app: &RSpiceApp) -> bool {
    app.state
        .model
        .library_files
        .library("parts")
        .and_then(|session| session.files.get(&amp_file()))
        .is_some_and(|file| file.edited)
}

/// An edit to a cellview of a library kept in a folder of its own, autosaved
/// before a crash, comes back with the recovered changes: in the drawing,
/// marked edited, with the library's files and the project file untouched.
/// Only the Save after writes it to its folder.
#[test]
fn a_crash_keeps_an_unsaved_edit_to_a_library_kept_in_its_own_folder() {
    let root = unique_temp_dir("library-recovery-keep");
    let project = root.join("design").join(PROJECT);
    let folder = root.join("design").join("parts");
    let app = saved_with_library_folder(&root);
    let library_before = folder_bytes(&folder);
    let project_before = std::fs::read(&project).expect("the project file reads");

    edit_autosave_and_crash(app);
    assert_eq!(
        folder_bytes(&folder),
        library_before,
        "the autosave wrote no library file"
    );

    let mut app = reopen_and_keep(&project);
    assert_eq!(amp_parts(&app), 2, "the capacitor is back");
    assert!(edited(&app), "amp is marked edited");
    assert!(has_unsaved_changes(&app.state), "as unsaved work");
    assert!(
        matches!(
            app.state.model.library_file_status("parts", &amp_file()),
            Some(CellViewStatus::Found { .. })
        ),
        "the folder still holds what the edit started from"
    );
    assert_eq!(
        folder_bytes(&folder),
        library_before,
        "no library file written"
    );
    assert_eq!(
        std::fs::read(&project).expect("the project file reads"),
        project_before,
        "the project file is untouched"
    );

    assert!(
        app.state.save_project_to_path(&project),
        "the project saves"
    );
    let saved = std::fs::read(folder.join(amp_file().relative_path())).expect("amp's file reads");
    let saved = CellViewFile::decode(&saved).expect("amp's file decodes");
    assert_eq!(saved.revision, 2, "the save wrote amp's next revision");
    assert_eq!(
        saved
            .drawing
            .map(|drawing| drawing.document().components.len())
            .unwrap_or_default(),
        2,
        "with the recovered capacitor"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// Another writer saved the cellview after the edit began: the recovered
/// edit still comes back, over the folder as it is now, and the file reads
/// "Changed on disk since you started editing". Nothing is written over
/// the other writer's file.
#[test]
fn a_recovered_edit_to_a_file_saved_since_reads_changed_since_editing() {
    let root = unique_temp_dir("library-recovery-moved-on");
    let project = root.join("design").join(PROJECT);
    let folder = root.join("design").join("parts");
    let app = saved_with_library_folder(&root);
    edit_autosave_and_crash(app);

    let path = folder.join(amp_file().relative_path());
    let mut theirs = CellViewFile::decode(&std::fs::read(&path).expect("amp's file reads"))
        .expect("amp's file decodes");
    theirs.revision += 1;
    let theirs = encode_library_file(&theirs).expect("their revision encodes");
    std::fs::write(&path, &theirs.bytes).expect("another writer saves amp");
    let library_before = folder_bytes(&folder);

    let app = reopen_and_keep(&project);
    assert_eq!(amp_parts(&app), 2, "the recovered capacitor is back");
    assert!(edited(&app), "amp is marked edited");
    let status = app
        .state
        .model
        .library_file_status("parts", &amp_file())
        .expect("amp has a status");
    assert!(
        matches!(status, CellViewStatus::ChangedSinceEditing { .. }),
        "{status:?}"
    );
    assert_eq!(status.label(), CHANGED_SINCE_EDITING);
    assert_eq!(
        folder_bytes(&folder),
        library_before,
        "the other writer's file is untouched"
    );
    let _ = std::fs::remove_dir_all(root);
}
