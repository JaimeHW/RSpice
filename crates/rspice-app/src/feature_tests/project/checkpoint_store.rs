//! A project's checkpoints through whole saves: where they are kept before
//! and after the project has a file, what a save adds and removes, and what
//! the reader changes on the Recovery page.

#![cfg(not(target_arch = "wasm32"))]

use std::path::Path;

use super::recovery_frames::{app_frame, click_on, offscreen, paint_until, painted, saved_project};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_services::workbench::lifecycle::project_checkpoint::{
    ProjectCheckpointReason, ProjectCheckpointSummary, budget_use, create,
    keep_unsaved_checkpoints_in, list, storage_location,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{ComponentType, Point};
use rspice_app_state::workbench::preferences::ScalarPreference;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_formats::project_results::segment_folder;
use rspice_results::store_reader::kept_segment_name;

/// One more part on the open schematic: a change a save writes.
fn edit(app: &mut RSpiceApp, index: i32) {
    app.state.model.schematic.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Capacitor, Point::new(40 * index, 0));
    });
}

/// The open project's checkpoints, newest first, where it keeps them now.
fn listed(app: &RSpiceApp) -> Vec<ProjectCheckpointSummary> {
    list(&app.state.model)
        .expect("the checkpoints are listed")
        .checkpoints
}

fn kept_manual(
    checkpoints: &[ProjectCheckpointSummary],
    manual: &ProjectCheckpointSummary,
) -> bool {
    checkpoints
        .iter()
        .any(|checkpoint| checkpoint.checkpoint_id() == manual.checkpoint_id())
}

/// The files directly inside `folder`, none when it is not there.
fn files_in(folder: &Path) -> usize {
    std::fs::read_dir(folder).map_or(0, Iterator::count)
}

/// Taken before the project had a file, under the smallest automatic space
/// there is and with more automatic checkpoints than fit in it, a manual
/// checkpoint is still listed after the first save brings the project's
/// checkpoints beside its file, after a save that changed the project, and
/// after a Save as to another folder.
#[test]
fn a_manual_checkpoint_survives_automatic_ones_a_first_save_and_a_save_as() {
    let root = unique_temp_dir("checkpoint-store-manual-survives");
    let unsaved = root.join("unsaved");
    let _unsaved = keep_unsaved_checkpoints_in(unsaved.clone());
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .preferences
        .set_scalar(ScalarPreference::CheckpointBudgetMib, 1)
        .expect("one MiB is inside the domain");
    edit(&mut app, 0);
    let manual =
        create(&mut app.state, ProjectCheckpointReason::Manual).expect("the manual one is written");
    let first = create(&mut app.state, ProjectCheckpointReason::TechnologyChange)
        .expect("an automatic one is written");
    // More automatic checkpoints than the space holds.
    let fit = (1024 * 1024 / first.snapshot_byte_len()).max(1);
    for _ in 0..fit + 4 {
        create(&mut app.state, ProjectCheckpointReason::TechnologyChange)
            .expect("an automatic one is written");
    }
    let before_save = listed(&app);
    let automatic = before_save
        .iter()
        .filter(|checkpoint| checkpoint.reason() == ProjectCheckpointReason::TechnologyChange)
        .count() as u64;
    assert!(
        automatic < fit + 5 && budget_use(&before_save) <= 1024 * 1024,
        "the oldest automatic ones were dropped to fit: {automatic} kept"
    );
    assert!(kept_manual(&before_save, &manual));

    let project_id = app.state.model.workspace.content.project.id().to_string();
    let file = root.join("amp.rspiceproj");
    assert!(app.state.save_project_to_path(&file), "the first save");
    let beside = root.join("amp.rspice-recovery");
    assert!(
        !unsaved.join(&project_id).exists(),
        "the first save moved every checkpoint out of the data folder"
    );
    let after_first_save = listed(&app);
    assert!(kept_manual(&after_first_save, &manual));
    assert_eq!(after_first_save.len(), before_save.len());
    assert_eq!(
        files_in(&beside),
        2 * after_first_save.len(),
        "they are kept beside the file, a manifest and a snapshot each and nothing else"
    );

    edit(&mut app, 1);
    assert!(
        app.state.save_project_to_path(&file),
        "a save that changed it"
    );
    let after_save = listed(&app);
    assert!(kept_manual(&after_save, &manual));
    assert_eq!(
        after_save[0].reason(),
        ProjectCheckpointReason::Save,
        "the version the save replaced is kept"
    );

    let elsewhere = root.join("v2");
    std::fs::create_dir_all(&elsewhere).expect("another folder");
    assert!(
        app.state
            .save_project_to_path(&elsewhere.join("amp-v2.rspiceproj")),
        "a Save as"
    );
    let after_save_as = listed(&app);
    assert!(kept_manual(&after_save_as, &manual));
    assert!(files_in(&elsewhere.join("amp-v2.rspice-recovery")) > 0);
    assert!(
        files_in(&beside) > 0,
        "the file saved before keeps its own checkpoints"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// A save that changes nothing keeps nothing; one that changes the project
/// keeps the version it replaced, at that version's revision.
#[test]
fn only_a_save_that_changes_the_project_keeps_the_version_it_replaced() {
    let (mut app, root) = saved_project("checkpoint-store-before-save");
    let file = root.join("idle.rspiceproj");
    let saved_revision = app.state.model.workspace.content.project.revision().get();
    assert!(app.state.save_project_to_path(&file), "an unchanged save");
    assert!(listed(&app).is_empty(), "an unchanged save keeps nothing");

    edit(&mut app, 0);
    assert!(app.state.save_project_to_path(&file), "a changed save");
    let kept = listed(&app);
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0].reason(), ProjectCheckpointReason::Save);
    assert_eq!(kept[0].project_revision(), saved_revision);
    let _ = std::fs::remove_dir_all(root);
}

/// The version a save replaces is kept with the results it read: a save that
/// drops a stored run still leaves that run's segment beside the "Before
/// save" checkpoint, so restoring it brings the run back with its samples.
#[test]
fn the_version_a_save_replaced_keeps_its_results_beside_its_checkpoint() {
    let root = unique_temp_dir("checkpoint-store-results");
    let _unsaved = keep_unsaved_checkpoints_in(root.join("unsaved"));
    let mut state: AppState =
        rspice_app_runs::simulation::controller::test_execution::run_manual_batch_session(
            "a run kept with the version it belongs to
V1 in 0 PULSE(0 1 0 1u 1u 50u 100u)
R1 in out 1k
C1 out 0 10n
.tran 0.1u 300u
.end
",
        );
    state.open_project_session();
    let file = root.join("kept.rspiceproj");
    assert!(state.save_project_to_path(&file), "saved with its run");
    let segment = state
        .platform
        .project_lifecycle
        .result_cache
        .capture(&state.model.simulation)
        .stored_segments()[0]
        .digest;

    drop(std::mem::take(&mut state.model.simulation));
    assert!(state.save_project_to_path(&file), "saved without it");

    let kept = list(&state.model).expect("listed").checkpoints;
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0].reason(), ProjectCheckpointReason::Save);
    // Each checkpoint keeps its results in a folder of its own in there.
    let location = storage_location(&state.model).expect("the folder is named");
    let results = segment_folder(&location.join("checkpoint"));
    let results = results.parent().expect("the results folder");
    assert!(
        std::fs::read_dir(results)
            .into_iter()
            .flatten()
            .flatten()
            .any(|folder| folder.path().join(kept_segment_name(&segment)).is_file()),
        "the run's segment is kept beside the checkpoint of the version that read it"
    );
    drop(state);
    let _ = std::fs::remove_dir_all(root);
}

/// The autosave holds unsaved work; a save of every document leaves none, so
/// it goes.
#[test]
fn a_save_of_every_document_removes_the_autosave() {
    let (mut app, root) = saved_project("checkpoint-store-autosave");
    edit(&mut app, 0);
    create(&mut app.state, ProjectCheckpointReason::Autosave).expect("the autosave is written");
    create(&mut app.state, ProjectCheckpointReason::Manual).expect("a manual one is written");
    assert!(
        app.state
            .save_project_to_path(&root.join("idle.rspiceproj"))
    );
    let reasons = listed(&app)
        .iter()
        .map(ProjectCheckpointSummary::reason)
        .collect::<Vec<_>>();
    assert!(
        !reasons.contains(&ProjectCheckpointReason::Autosave),
        "{reasons:?}"
    );
    assert!(
        reasons.contains(&ProjectCheckpointReason::Manual),
        "{reasons:?}"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// Open the row's More menu and choose `item` in it.
fn choose(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    shown: &egui::FullOutput,
    title: &str,
    item: &str,
) -> egui::FullOutput {
    app_frame(
        ctx,
        app,
        click_on(shown, &format!("More actions for {title}")),
    );
    let menu = app_frame(ctx, app, Vec::new());
    app_frame(ctx, app, click_on(&menu, item))
}

/// On the Recovery page a checkpoint is pinned, named by a note and deleted
/// after a question that names it, each through its row's More menu, and
/// each change is in the store the page lists from.
#[test]
fn a_checkpoint_is_pinned_noted_and_deleted_on_the_recovery_page() {
    let (mut app, root) = saved_project("checkpoint-store-page-changes");
    create(&mut app.state, ProjectCheckpointReason::TechnologyChange)
        .expect("the checkpoint is written");
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Recovery;
    let ctx = offscreen();
    let shown = paint_until(&ctx, &mut app, |text| {
        text.lines().any(|line| line == "1 checkpoint")
    });

    choose(&ctx, &mut app, &shown, "Before technology change", "Pin");
    let shown = paint_until(&ctx, &mut app, |text| text.contains("Pinned \u{b7} "));
    assert!(listed(&app)[0].pinned(), "the pin is stored");

    choose(
        &ctx,
        &mut app,
        &shown,
        "Before technology change",
        "Add note\u{2026}",
    );
    // The field opens on the next frame and takes the keyboard on the one
    // after, once the menu it was chosen from has gone.
    for _ in 0..2 {
        app_frame(&ctx, &mut app, Vec::new());
    }
    app_frame(
        &ctx,
        &mut app,
        vec![egui::Event::Text("Before the gain change".to_owned())],
    );
    let writing = app_frame(&ctx, &mut app, Vec::new());
    app_frame(&ctx, &mut app, click_on(&writing, "Keep note"));
    let shown = paint_until(&ctx, &mut app, |text| {
        text.lines().any(|line| line == "Before the gain change")
    });
    assert_eq!(listed(&app)[0].note(), Some("Before the gain change"));

    let asked = choose(
        &ctx,
        &mut app,
        &shown,
        "Before the gain change",
        "Delete\u{2026}",
    );
    let asked = if painted(&asked).contains("It cannot be brought back") {
        asked
    } else {
        app_frame(&ctx, &mut app, Vec::new())
    };
    assert!(
        painted(&asked).contains("Delete \u{201c}Before the gain change\u{201d} from "),
        "the question names the checkpoint:\n{}",
        painted(&asked)
    );
    app_frame(
        &ctx,
        &mut app,
        click_on(&asked, "Delete Before the gain change"),
    );
    paint_until(&ctx, &mut app, |text| {
        text.lines().any(|line| line == "no checkpoints")
    });
    assert!(listed(&app).is_empty(), "the checkpoint is gone from disk");
    assert_eq!(
        files_in(&storage_location(&app.state.model).expect("the folder is named")),
        0,
        "nothing of the pinned and noted checkpoint is left in its folder"
    );
    assert!(
        app.state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message.starts_with("Deleted checkpoint")),
        "the Console records the deletion"
    );
    let _ = std::fs::remove_dir_all(root);
}
