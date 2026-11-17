//! A project this window cannot or should not write opens read-only, through
//! whole application frames: the notice under the toolbar says why in every
//! workspace, every write is refused with that reason, and its one action
//! changes it.

#![cfg(not(target_arch = "wasm32"))]

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::workflows::project_workflow::SaveRequestOutcome;
use rspice_app_services::io::durable_file::open_lease::{OpenLeaseHolder, UnwritableFolder};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    ConflictSide, SaveScope, canonical_native_path,
};
use rspice_app_state::state::write_policy::OpenedReadOnly;
use rspice_app_state::workbench::workspace::Workspace;

use super::recovery_frames::{app_frame, click_on, offscreen, painted, saved_project};

const PROJECT: &str = "idle.rspiceproj";

/// A project saved by a window that has since closed, so nothing holds it.
fn closed_project(label: &str) -> std::path::PathBuf {
    let (app, root) = saved_project(label);
    drop(app);
    root
}

/// What two frames of `app` paint: the first lays the notice out.
fn shown(app: &mut RSpiceApp) -> egui::FullOutput {
    let ctx = offscreen();
    app_frame(&ctx, app, Vec::new());
    app_frame(&ctx, app, Vec::new())
}

#[test]
fn a_project_in_a_folder_that_cannot_be_written_opens_read_only_and_says_why() {
    let root = closed_project("read-only-folder");
    let path = root.join(PROJECT);
    let denied = UnwritableFolder::deny_new_files(&root).expect("the folder takes no new file");
    if denied.still_writable() {
        // An account the folder's permissions do not bind, as root.
        return;
    }
    let before = std::fs::read(&path).unwrap();

    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state.load_project_from_path(&path),
        "the project opens: the folder could be read"
    );
    let Some(OpenedReadOnly::FolderNotWritable { folder }) =
        app.state.model.safe_mode.opened_read_only.clone()
    else {
        panic!(
            "the project opened read-only for its folder: {:?}",
            app.state.model.safe_mode.opened_read_only
        );
    };
    assert_eq!(
        std::fs::canonicalize(&folder).unwrap(),
        std::fs::canonicalize(&root).unwrap()
    );
    assert!(app.state.model.safe_mode.project_read_only());

    for workspace in [Workspace::Project, Workspace::Design, Workspace::Results] {
        app.state.workbench.activate(workspace);
        let text = painted(&shown(&mut app));
        assert!(
            text.contains("Opened read-only: the folder"),
            "{workspace:?} says why the project is read-only:\n{text}"
        );
        assert!(text.contains("Save as\u{2026}"), "{workspace:?}:\n{text}");
    }

    let refused = app.state.save_project_scope(SaveScope::AllDocuments);
    assert!(
        matches!(
            &refused,
            SaveRequestOutcome::Failed(reason)
                if reason.contains("Opened read-only: the folder")
        ),
        "a save to the read-only folder is refused with the reason: {refused:?}"
    );
    assert_eq!(
        std::fs::read(&path).unwrap(),
        before,
        "the file is unchanged"
    );

    // Save as to a folder this window can write keeps the project there, and
    // the project is writable again.
    let writable = rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir(
        "read-only-folder-kept",
    );
    let kept = writable.join(PROJECT);
    assert!(app.state.save_project_as_to_path(&kept));
    assert_eq!(app.state.model.safe_mode.opened_read_only, None);
    assert!(!app.state.model.safe_mode.project_read_only());
    assert!(!painted(&shown(&mut app)).contains("Opened read-only"));
    drop(denied);
}

#[test]
fn a_project_file_marked_read_only_opens_read_only() {
    let root = closed_project("read-only-file");
    let path = root.join(PROJECT);
    let mut permissions = std::fs::metadata(&path).unwrap().permissions();
    permissions.set_readonly(true);
    std::fs::set_permissions(&path, permissions.clone()).unwrap();

    let mut app = RSpiceApp::test_instance();
    assert!(app.state.load_project_from_path(&path));
    assert_eq!(
        app.state.model.safe_mode.opened_read_only,
        Some(OpenedReadOnly::FileReadOnly)
    );
    let text = painted(&shown(&mut app));
    assert!(
        text.contains("Opened read-only: the project file is marked read-only."),
        "{text}"
    );
    #[allow(clippy::permissions_set_readonly_false)]
    permissions.set_readonly(false);
    std::fs::set_permissions(&path, permissions).unwrap();
}

#[test]
fn a_second_window_is_told_who_has_the_project_and_opens_it_read_only() {
    let (mut first, root) = saved_project("second-window");
    let path = root.join(PROJECT);
    assert_eq!(first.state.model.safe_mode.opened_read_only, None);

    let mut second = RSpiceApp::test_instance();
    assert!(second.state.load_project_from_path(&path));
    let me = OpenLeaseHolder::this_process(0);
    let Some(OpenedReadOnly::OpenElsewhere {
        holder: Some(holder),
    }) = second.state.model.safe_mode.opened_read_only.clone()
    else {
        panic!(
            "the second window is told the project is open: {:?}",
            second.state.model.safe_mode.opened_read_only
        );
    };
    assert_eq!(
        (holder.user.as_str(), holder.machine.as_str()),
        (me.user.as_str(), me.machine.as_str())
    );
    assert!(holder.since_unix_ms > 0, "the holder says since when");

    let output = shown(&mut second);
    let text = painted(&output);
    assert!(
        text.contains("has had this project open since"),
        "the notice names who holds it and since when:\n{text}"
    );

    // "Open anyway" is one click, and the project is writable at once.
    let ctx = offscreen();
    app_frame(&ctx, &mut second, Vec::new());
    let output = app_frame(&ctx, &mut second, Vec::new());
    app_frame(&ctx, &mut second, click_on(&output, "Open anyway"));
    assert_eq!(second.state.model.safe_mode.opened_read_only, None);
    assert!(!second.state.model.safe_mode.project_read_only());
    assert!(!painted(&app_frame(&ctx, &mut second, Vec::new())).contains("Opened read-only"));

    // Once the first window lets the project go, a window that opens it
    // holds it.
    first.state.close_project_discard(
        rspice_app_services::workbench::lifecycle::project_lifecycle::ProjectCloseDestination::EmptyWorkbench,
    );
    drop(second);
    let mut third = RSpiceApp::test_instance();
    assert!(third.state.load_project_from_path(&path));
    assert_eq!(third.state.model.safe_mode.opened_read_only, None);
    assert!(canonical_native_path(&third.state.platform).is_some());
    drop(first);
}

fn place_part(app: &mut RSpiceApp, index: i32) {
    app.state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_component(
            rspice_app_state::state::ComponentType::Capacitor,
            rspice_app_state::state::Point::new(40 * index, 0),
        );
    });
    app.state.sync_active_schematic_to_workspace();
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

#[test]
fn a_save_that_meets_the_file_saved_elsewhere_shows_what_differs_and_saves_over_it_on_request() {
    let (mut first, root) = saved_project("save-conflict");
    let path = root.join(PROJECT);
    let mut second = RSpiceApp::test_instance();
    assert!(second.state.load_project_from_path(&path));
    assert!(crate::workbench::app_state::services_access::open_anyway(
        &mut second.state
    ));
    let before = parts(&second);

    place_part(&mut first, 1);
    assert!(matches!(
        first.state.save_project_scope(SaveScope::AllDocuments),
        SaveRequestOutcome::CanonicalComplete
    ));
    let first_saved = std::fs::read(&path).unwrap();

    place_part(&mut second, 2);
    place_part(&mut second, 3);
    let met = second.state.save_project_scope(SaveScope::AllDocuments);
    assert!(matches!(met, SaveRequestOutcome::Failed(_)), "{met:?}");
    assert_eq!(
        std::fs::read(&path).unwrap(),
        first_saved,
        "nothing was written"
    );
    let conflict = second
        .state
        .platform
        .project_lifecycle
        .save_conflict
        .clone()
        .expect("the save asks what to do");
    assert!(
        conflict
            .differences
            .iter()
            .any(|difference| difference.side == ConflictSide::Both),
        "the drawing both windows changed is named: {:?}",
        conflict.differences
    );

    let ctx = offscreen();
    app_frame(&ctx, &mut second, Vec::new());
    let asked = app_frame(&ctx, &mut second, Vec::new());
    let text = painted(&asked);
    assert!(text.contains("The saved project changed"), "{text}");
    assert!(
        text.contains("was saved from another window after this one opened it"),
        "{text}"
    );
    assert!(
        text.contains("Changed here and in the saved file"),
        "{text}"
    );

    app_frame(&ctx, &mut second, click_on(&asked, "Save over it"));
    assert!(
        second
            .state
            .platform
            .project_lifecycle
            .save_conflict
            .is_none()
    );
    assert_ne!(
        std::fs::read(&path).unwrap(),
        first_saved,
        "this window's project was written"
    );
    assert!(
        !rspice_app_services::workbench::lifecycle::project_lifecycle::has_unsaved_changes(
            &second.state
        ),
        "the save over it leaves nothing unsaved"
    );
    let mut third = RSpiceApp::test_instance();
    assert!(third.state.load_project_from_path(&path));
    assert_eq!(
        parts(&third),
        before + 2,
        "the file holds this window's two parts"
    );
    drop(first);
}
