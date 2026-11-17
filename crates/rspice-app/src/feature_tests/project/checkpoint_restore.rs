//! Going back to a checkpoint on the Recovery page: Restore puts it back in
//! the open project as unsaved changes after keeping the project as it was,
//! Compare names each document that differs, and a checkpoint saved as a new
//! project is offered to open. A project file that will not open is reached
//! through its checkpoints: offered when the open fails, and listed on the
//! launcher's Recovery page.

#![cfg(not(target_arch = "wasm32"))]

use super::recovery_frames::{app_frame, click_on, offscreen, paint_until, painted, saved_project};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::services_access::restore_checkpoint_in_place;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_services::workbench::lifecycle::project_checkpoint::{
    ProjectCheckpointReason, ProjectCheckpointSummary, capture_current_state, create,
    keep_unsaved_checkpoints_in, list, load, publish_recovery_copy,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    canonical_native_path, has_unsaved_changes_fresh,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{ComponentType, Point};
use rspice_app_state::workbench::workspace::Workspace;

/// One more part on the open schematic.
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

fn listed(app: &RSpiceApp) -> Vec<ProjectCheckpointSummary> {
    list(&app.state.model)
        .expect("the checkpoints are listed")
        .checkpoints
}

/// The Recovery page in front, once it lists `count` checkpoints.
fn recovery_page(ctx: &egui::Context, app: &mut RSpiceApp, count: &str) -> egui::FullOutput {
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Recovery;
    paint_until(ctx, app, |text| text.lines().any(|line| line == count))
}

/// The frame after `shown` that paints `text`, clicking nothing more.
fn next_with(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    shown: egui::FullOutput,
    text: &str,
) -> egui::FullOutput {
    if painted(&shown).contains(text) {
        shown
    } else {
        paint_until(ctx, app, |painted| painted.contains(text))
    }
}

/// Restore asks in place, keeps the project as it was in a "Before restore"
/// checkpoint, then puts the checkpoint back in the same project: its parts
/// are the checkpoint's, the work is unsaved, the file is untouched, and the
/// status bar and the page say what happened.
#[test]
fn restore_puts_the_checkpoint_back_in_the_open_project_as_unsaved_changes() {
    let (mut app, root) = saved_project("restore-in-place");
    let file = root.join("idle.rspiceproj");
    edit(&mut app, 0);
    let at_checkpoint = parts(&app);
    create(&mut app.state, ProjectCheckpointReason::Manual).expect("the checkpoint is written");
    edit(&mut app, 1);
    edit(&mut app, 2);
    let as_it_was = capture_current_state(&app.state).expect("the project is read");
    let saved_bytes = std::fs::read(&file).expect("the project file is read");
    let project_id = app.state.model.workspace.content.project.id().to_string();

    let ctx = offscreen();
    let shown = recovery_page(&ctx, &mut app, "1 checkpoint");
    let asked = app_frame(
        &ctx,
        &mut app,
        click_on(&shown, "Restore Manual checkpoint into the project"),
    );
    let asked = next_with(
        &ctx,
        &mut app,
        asked,
        "Restore \u{201c}Manual checkpoint\u{201d} from ",
    );
    assert!(
        painted(&asked)
            .contains("the project as it is now is kept as \u{201c}Before restore\u{201d}"),
        "the question says what is kept:\n{}",
        painted(&asked)
    );
    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Restore Manual checkpoint"),
    );
    paint_until(&ctx, &mut app, |text| {
        text.lines().any(|line| line == "2 checkpoints")
    });

    assert_eq!(
        parts(&app),
        at_checkpoint,
        "the checkpoint's drawing is back"
    );
    assert_eq!(
        app.state.model.workspace.content.project.id().to_string(),
        project_id,
        "the open project is still the same project"
    );
    assert!(
        has_unsaved_changes_fresh(&app.state),
        "the restore is unsaved work"
    );
    assert_eq!(
        std::fs::canonicalize(canonical_native_path(&app.state.platform).expect("still bound"))
            .expect("the bound file exists"),
        std::fs::canonicalize(&file).expect("the file exists"),
        "the project is still bound to its own file"
    );
    assert_eq!(
        std::fs::read(&file).expect("the project file is read again"),
        saved_bytes,
        "the saved file is untouched until the reader saves"
    );
    let prompt = app.state.prompt_text().unwrap_or_default().to_owned();
    assert!(
        prompt.starts_with("Restored \u{201c}Manual checkpoint\u{201d} from ")
            && prompt.contains("kept as \u{201c}Before restore\u{201d}"),
        "{prompt}"
    );
    let before = listed(&app)
        .into_iter()
        .find(|checkpoint| checkpoint.reason() == ProjectCheckpointReason::Restore)
        .expect("the project as it was is a checkpoint");
    assert!(
        as_it_was.matches(&before).expect("compared"),
        "the Before restore checkpoint holds the project as it was"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// Compare lists each document the open project differs in, with how; a
/// checkpoint of the project as it is answers on one line.
#[test]
fn compare_names_each_document_that_differs() {
    let (mut app, root) = saved_project("compare-by-document");
    create(&mut app.state, ProjectCheckpointReason::Manual).expect("the checkpoint is written");
    edit(&mut app, 0);
    let drawing = app.state.model.workspace.content.active_view.clone();

    let ctx = offscreen();
    let shown = recovery_page(&ctx, &mut app, "1 checkpoint");
    app_frame(
        &ctx,
        &mut app,
        click_on(&shown, "Compare Manual checkpoint with the current project"),
    );
    let found = paint_until(&ctx, &mut app, |text| {
        text.contains("The current project differs in 1 document:")
    });
    let text = painted(&found);
    let line = format!("{} \u{b7} {}", drawing.cell, drawing.view);
    assert!(
        text.lines().any(|painted| painted == line)
            && text.lines().any(|painted| painted == "Changed"),
        "the drawing is named as changed:\n{text}"
    );
    assert!(
        app.state
            .prompt_text()
            .is_some_and(|prompt| prompt.ends_with("in 1 document.")),
        "{:?}",
        app.state.prompt_text()
    );
    app_frame(&ctx, &mut app, click_on(&found, "Close the comparison"));
    let closed = app_frame(&ctx, &mut app, Vec::new());
    assert!(
        !painted(&closed).contains("differs in 1 document"),
        "the list is put away"
    );

    // A checkpoint of the project as it is now: identical, on one line.
    create(&mut app.state, ProjectCheckpointReason::Manual).expect("a second one is written");
    app.state.project.project_checkpoint_recovery.invalidate();
    let shown = paint_until(&ctx, &mut app, |text| {
        text.lines().any(|line| line == "2 checkpoints")
    });
    // Both rows carry the same title; the newest is listed first.
    app_frame(
        &ctx,
        &mut app,
        click_on(&shown, "Compare Manual checkpoint with the current project"),
    );
    frames_until(&ctx, &mut app, |app| {
        app.state.prompt_text() == Some("The current project is identical to this checkpoint.")
    });
    let _ = std::fs::remove_dir_all(root);
}

/// Paint frames until `done` holds of the application.
fn frames_until(ctx: &egui::Context, app: &mut RSpiceApp, done: impl Fn(&RSpiceApp) -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    while !done(app) {
        assert!(std::time::Instant::now() < deadline, "never came about");
        app_frame(ctx, app, Vec::new());
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}

/// A checkpoint's results come back with it: a run the project dropped after
/// the checkpoint is retained again once the checkpoint is restored.
#[test]
fn a_restored_checkpoint_brings_its_results_back() {
    let root = unique_temp_dir("restore-results");
    let _unsaved = keep_unsaved_checkpoints_in(root.join("unsaved"));
    let mut state: AppState =
        rspice_app_runs::simulation::controller::test_execution::run_manual_batch_session(
            "a run restored with its checkpoint
V1 in 0 PULSE(0 1 0 1u 1u 50u 100u)
R1 in out 1k
C1 out 0 10n
.tran 0.1u 300u
.end
",
        );
    state.open_project_session();
    assert!(state.save_project_to_path(&root.join("runs.rspiceproj")));
    let runs = state.model.simulation.retained.runs.len();
    assert!(runs > 0, "the session holds its run");
    let checkpoint =
        create(&mut state, ProjectCheckpointReason::Manual).expect("the checkpoint is written");
    drop(std::mem::take(&mut state.model.simulation));
    assert_eq!(state.model.simulation.retained.runs.len(), 0);

    restore_checkpoint_in_place(&mut state, load(&checkpoint).expect("read back"))
        .expect("the checkpoint is restored");
    assert_eq!(
        state.model.simulation.retained.runs.len(),
        runs,
        "the run is retained again"
    );
    drop(state);
    let _ = std::fs::remove_dir_all(root);
}

/// "Save as new project…" writes the checkpoint as a project of its own and
/// the row offers to open it; opening it leaves the project that was open.
#[test]
fn a_checkpoint_saved_as_a_new_project_is_offered_to_open() {
    let (mut app, root) = saved_project("saved-copy-offer");
    edit(&mut app, 0);
    create(&mut app.state, ProjectCheckpointReason::Manual).expect("the checkpoint is written");
    // Saved, so opening the copy has no unsaved work to ask about.
    assert!(
        app.state
            .save_project_to_path(&root.join("idle.rspiceproj"))
    );
    let ctx = offscreen();
    recovery_page(&ctx, &mut app, "2 checkpoints");
    let checkpoint = listed(&app)
        .into_iter()
        .find(|checkpoint| checkpoint.reason() == ProjectCheckpointReason::Manual)
        .expect("the manual checkpoint is listed");
    let copy = root.join("idle-recovered.rspiceproj");
    publish_recovery_copy(&checkpoint, &copy).expect("the copy is written");
    // What the export answers once the reader has picked where to save.
    app.state.project.project_checkpoint_recovery.saved_copy =
        Some((checkpoint.checkpoint_id(), copy.clone()));
    let shown = paint_until(&ctx, &mut app, |text| {
        text.contains("idle-recovered.rspiceproj")
    });
    app_frame(&ctx, &mut app, click_on(&shown, "Open the saved project"));
    for _ in 0..3 {
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(
        std::fs::canonicalize(canonical_native_path(&app.state.platform).expect("bound"))
            .expect("the bound file exists"),
        std::fs::canonicalize(&copy).expect("the copy exists"),
        "the copy is the open project"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// The save picker of a test: it answers with the path it holds, as if the
/// reader had picked it.
struct PickedPath(std::path::PathBuf);

impl rspice_app_services::workbench::workflows::export_workflow::ExportWorkflowIo for PickedPath {
    fn show_save_dialog(
        &self,
        _config: rspice_app_services::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
    ) -> Result<Option<std::path::PathBuf>, String> {
        Ok(Some(self.0.clone()))
    }

    fn write_text_file(&self, _path: &std::path::Path, _contents: &str) -> Result<(), String> {
        Err("no text is exported here".to_owned())
    }

    fn write_waveform_csv(
        &self,
        _dataset: &rspice_app_services::io::WaveformDataset,
        _path: &std::path::Path,
    ) -> Result<(), String> {
        Err("no waveform is exported here".to_owned())
    }
}

/// A saved project with two more parts in a checkpoint, and its file cut
/// short: the folder, the file, and how many parts the checkpoint holds.
fn damaged_project(label: &str) -> (std::path::PathBuf, std::path::PathBuf, usize, Vec<u8>) {
    let (mut app, root) = saved_project(label);
    let file = root.join("idle.rspiceproj");
    edit(&mut app, 0);
    edit(&mut app, 1);
    let at_checkpoint = parts(&app);
    create(&mut app.state, ProjectCheckpointReason::Manual).expect("the checkpoint is written");
    drop(app);
    let bytes = std::fs::read(&file).expect("the file is read");
    std::fs::write(&file, &bytes[..bytes.len() / 2]).expect("the file is cut short");
    let damaged = std::fs::read(&file).expect("the damaged file is read");
    (root, file, at_checkpoint, damaged)
}

/// The copy is the open project, with the checkpoint's drawing, and the
/// damaged file is as it was.
fn opened_from_checkpoint(
    app: &RSpiceApp,
    copy: &std::path::Path,
    at_checkpoint: usize,
    file: &std::path::Path,
    damaged: &[u8],
) {
    assert_eq!(
        std::fs::canonicalize(canonical_native_path(&app.state.platform).expect("bound"))
            .expect("the bound file exists"),
        std::fs::canonicalize(copy).expect("the copy exists"),
        "the checkpoint, saved as a new project, is the open project"
    );
    assert_eq!(parts(app), at_checkpoint, "with the checkpoint's drawing");
    assert_eq!(
        std::fs::read(file).expect("the damaged file is read again"),
        damaged,
        "the damaged file is left as it is"
    );
}

/// A project file cut short does not open. The newest of its checkpoints
/// that reads intact is offered, and saving it as a new project opens it.
#[test]
fn a_truncated_project_file_opens_from_its_checkpoint() {
    let (root, file, at_checkpoint, damaged) = damaged_project("truncated-file");
    let copy = root.join("idle-recovered.rspiceproj");
    let mut app = RSpiceApp::test_instance();
    app.export_workflow_io = Box::new(PickedPath(copy.clone()));
    assert!(
        !app.state.load_project_from_path(&file),
        "the damaged file does not open"
    );
    let ctx = offscreen();
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Open a checkpoint instead?")
    });
    assert!(
        painted(&asked).contains("\u{201c}Manual checkpoint\u{201d}, is from "),
        "the question names the checkpoint and its time:\n{}",
        painted(&asked)
    );
    assert!(
        painted(&asked).contains(" UTC ("),
        "at the time the Recovery page gives it:\n{}",
        painted(&asked)
    );
    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Save as new project\u{2026}"),
    );
    for _ in 0..3 {
        app_frame(&ctx, &mut app, Vec::new());
    }
    opened_from_checkpoint(&app, &copy, at_checkpoint, &file, &damaged);
    let _ = std::fs::remove_dir_all(root);
}

/// The launcher's Recovery page lists the checkpoints kept beside a recent
/// project's file, under the project; one selected there is saved as a new
/// project and opened, without the damaged file ever opening.
#[test]
fn the_launcher_lists_a_recent_projects_checkpoints() {
    use rspice_app_project::workbench::project::open_project_launcher;
    use rspice_app_project::workbench::project::workspace_state::ProjectLauncherPage;
    use rspice_app_services::services::recent_files::{RecentFile, RecentKind};

    let (root, file, at_checkpoint, damaged) = damaged_project("launcher-checkpoints");
    let copy = root.join("idle-recovered.rspiceproj");
    let mut app = RSpiceApp::test_instance();
    app.export_workflow_io = Box::new(PickedPath(copy.clone()));
    app.state.platform.recent_files = vec![RecentFile {
        kind: RecentKind::Project,
        path: file.clone(),
        opened_at_unix_ms: 0,
        pinned: false,
        missing: false,
    }];
    open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );
    app.state.project.project_launcher_page = ProjectLauncherPage::Recovery;
    let ctx = offscreen();
    let shown = paint_until(&ctx, &mut app, |text| {
        text.lines().any(|line| line == "Project checkpoints")
            && text.lines().any(|line| line == "Manual checkpoint")
    });
    let row = click_where(&shown, |label| {
        label.starts_with("Checkpoint: Manual checkpoint, Taken: ")
    });
    app_frame(&ctx, &mut app, row);
    let selected = app_frame(&ctx, &mut app, Vec::new());
    app_frame(
        &ctx,
        &mut app,
        click_on(&selected, "Save as new project\u{2026}"),
    );
    for _ in 0..3 {
        app_frame(&ctx, &mut app, Vec::new());
    }
    opened_from_checkpoint(&app, &copy, at_checkpoint, &file, &damaged);
    assert!(
        !app.state.project.project_launcher_open,
        "the launcher made way"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// A click on the first control whose spoken label passes `pick`.
fn click_where(output: &egui::FullOutput, pick: impl Fn(&str) -> bool) -> Vec<egui::Event> {
    let label = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| node.label().filter(|label| pick(label)))
        .unwrap_or_else(|| panic!("no control is announced as asked"));
    click_on(output, label)
}
