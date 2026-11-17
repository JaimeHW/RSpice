//! A file dropped on the window through whole application frames: a project
//! opens wherever it lands, after the review of any unsaved work it would
//! replace, and a SPICE deck dropped on the start screen opens as a netlist
//! project.

#![cfg(not(target_arch = "wasm32"))]

use std::path::Path;

use crate::workbench::RSpiceApp;
use rspice_app_services::workbench::lifecycle::project_lifecycle::canonical_native_path;
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::workbench::workspace::Workspace;

use super::recovery_frames::offscreen;

/// One whole application frame with `file` dropped on it.
fn drop_on(app: &mut RSpiceApp, file: egui::DroppedFile) {
    let ctx = offscreen();
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            dropped_files: vec![file],
            ..Default::default()
        },
        |ui| {
            app.prepare_frame(ui.ctx());
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        },
    );
}

fn dropped(path: &Path) -> egui::DroppedFile {
    egui::DroppedFile {
        path: Some(path.to_path_buf()),
        name: path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default(),
        ..Default::default()
    }
}

/// A saved project on disk, its folder and its file.
fn project_on_disk(label: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let folder = unique_temp_dir(label);
    std::fs::create_dir_all(&folder).expect("fixture folder");
    let file = folder.join("amp.rspiceproj");
    let mut saver = RSpiceApp::test_instance();
    assert!(saver.state.save_project_to_path(&file));
    (folder, file)
}

/// The start screen: no project open.
fn start_screen() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.close_project_session();
    app.state.platform.recovery_catalog = Default::default();
    app
}

fn names(app: &RSpiceApp, file: &Path) -> bool {
    canonical_native_path(&app.state.platform)
        .as_deref()
        .and_then(Path::file_name)
        == file.file_name()
}

/// Dropped on the start screen, or on a workspace of another project, a
/// project file opens.
#[test]
fn a_dropped_project_opens_on_the_landing_and_over_a_workspace() {
    let (folder, file) = project_on_disk("dropped-project");

    let mut landing = start_screen();
    assert!(!landing.state.platform.project_lifecycle.is_open());
    drop_on(&mut landing, dropped(&file));
    assert!(names(&landing, &file), "the landing opens it");

    let (mut working, other) = super::recovery_frames::saved_project("dropped-over");
    working.state.workbench.activate(Workspace::Netlist);
    drop_on(&mut working, dropped(&file));
    assert!(
        names(&working, &file),
        "the Netlist page does not take it for a deck"
    );
    let _ = std::fs::remove_dir_all(folder);
    let _ = std::fs::remove_dir_all(other);
}

/// A project dropped over unsaved work is not opened until that work is
/// reviewed, as File > Open recent asks.
#[test]
fn a_dropped_project_asks_about_unsaved_work_first() {
    let (folder, file) = project_on_disk("dropped-review");
    let (mut app, other) = super::recovery_frames::saved_project("dropped-unsaved");
    let mut properties = app.state.model.workspace.content.project.properties();
    properties.description = "Unsaved".to_owned();
    app.state
        .model
        .publish_project_properties(properties, "test")
        .expect("the description changes");

    drop_on(&mut app, dropped(&file));

    assert!(!names(&app, &file), "nothing was replaced");
    assert!(
        app.state.workbench.confirmation_dialog.visible,
        "the unsaved work is reviewed first"
    );
    let _ = std::fs::remove_dir_all(folder);
    let _ = std::fs::remove_dir_all(other);
}

/// A deck dropped on the start screen opens as a netlist project, as File >
/// Open netlist does.
#[test]
fn a_deck_dropped_on_the_landing_opens_as_a_netlist_project() {
    let mut app = start_screen();
    drop_on(
        &mut app,
        egui::DroppedFile {
            name: "divider.cir".to_owned(),
            bytes: Some(
                b"* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n"
                    .as_slice()
                    .into(),
            ),
            ..Default::default()
        },
    );
    let review = app
        .state
        .code
        .netlist
        .import_review
        .as_ref()
        .expect("the deck is staged for its review");
    assert_eq!(review.display_name, "divider.cir");
}

/// A file RSpice does not open, dropped on the start screen, is answered.
#[test]
fn a_file_the_start_screen_cannot_open_is_answered() {
    let mut app = start_screen();
    drop_on(
        &mut app,
        egui::DroppedFile {
            name: "notes.txt".to_owned(),
            bytes: Some(b"notes".as_slice().into()),
            ..Default::default()
        },
    );
    assert!(!app.state.platform.project_lifecycle.is_open());
    assert!(
        app.state
            .workbench
            .prompt
            .as_ref()
            .is_some_and(|prompt| prompt.text.contains(".rspiceproj")),
        "the prompt names what it opens"
    );
}
