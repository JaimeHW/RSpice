//! Recovery, the launcher and revision history through whole application
//! frames: an idle frame reads no project file, lists no checkpoint
//! directory, stats no file and rebuilds no list, and checkpoint work runs
//! while the page goes on painting.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};

use egui::{Context, Modifiers, Pos2, Rect, Vec2, pos2, vec2};

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::app::dialogs::project::CheckpointJobHold;
use rspice_app_project::workbench::app::dialogs::project_revision_history::AUDIT_ROW_BUILDS;
use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_project::workbench::project::open_project_launcher;
use rspice_app_project::workbench::project::workspace_state::{ProjectLauncherPage, ProjectPage};
use rspice_app_project::workbench::tools::project_launcher::PROJECT_LIST_BUILDS;
use rspice_app_services::services::recent_files::{RecentFile, RecentKind};
use rspice_app_services::workbench::lifecycle::project_checkpoint::{
    ProjectCheckpointReason, create,
};
use rspice_app_services::workbench::lifecycle::recovery::{DiskReads, take_disk_reads};
use rspice_app_services::workbench::lifecycle::recovery::{
    RecoveryCandidate, RecoveryCatalog, unique_temp_dir,
};
use rspice_app_services::workbench::lifecycle::recovery_checkpoint::{
    AutosaveRestoreCandidate, CheckpointBinding, CheckpointIdentity, CheckpointOwnership,
};
use rspice_app_state::workbench::workspace::Workspace;

pub(super) const IDLE_FRAMES: usize = 30;
const SIZE: Vec2 = vec2(1600.0, 900.0);

/// One whole application frame, as the shell paints it, with `events`.
pub(super) fn app_frame(
    ctx: &Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SIZE)),
            events,
            ..Default::default()
        },
        |ui| {
            let ctx = ui.ctx().clone();
            app.prepare_frame(&ctx);
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(&ctx);
        },
    )
}

/// The frame a window that just opened is first seen on: egui lays a new
/// window out without painting it on its first frame and paints it on the
/// next.
fn first_seen(ctx: &Context, app: &mut RSpiceApp) -> egui::FullOutput {
    app_frame(ctx, app, Vec::new());
    app_frame(ctx, app, Vec::new())
}

pub(super) fn offscreen() -> Context {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx
}

/// Every string a frame painted, one per line.
pub(super) fn painted(output: &egui::FullOutput) -> String {
    fn collect(shape: &egui::epaint::Shape, text: &mut String) {
        match shape {
            egui::epaint::Shape::Text(shape) => {
                text.push_str(shape.galley.text());
                text.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, text);
                }
            }
            _ => {}
        }
    }
    let mut text = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut text);
    }
    text
}

/// A press and release in the middle of the control `output` announced as
/// `label`.
fn click_on(output: &egui::FullOutput, label: &str) -> Vec<egui::Event> {
    let bounds = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label()? == label).then_some(node.bounds()?))
        .unwrap_or_else(|| panic!("the frame announces `{label}`"));
    let at = pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: Modifiers::default(),
        },
    ]
}

/// Paint until `done` holds for what was painted, as the reader's screen
/// would while background work finishes.
pub(super) fn paint_until(
    ctx: &Context,
    app: &mut RSpiceApp,
    done: impl Fn(&str) -> bool,
) -> egui::FullOutput {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let output = app_frame(ctx, app, Vec::new());
        if done(&painted(&output)) {
            return output;
        }
        assert!(
            Instant::now() < deadline,
            "the page never showed what it waited for:\n{}",
            painted(&output)
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// A saved, clean project in its own folder, so its checkpoints are kept
/// there and nowhere else on the machine.
fn saved_project(label: &str) -> (RSpiceApp, std::path::PathBuf) {
    let root = unique_temp_dir(label);
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .save_project_to_path(&root.join("idle.rspiceproj"))
    );
    (app, root)
}

/// The launcher's Recovery page, with a recoverable checkpoint selected over
/// a saved, clean project, says whether the comparison may open. Working
/// that out reads the saved project: once when the page first shows, once
/// more when the selection changes, and never on a frame that only paints.
#[test]
fn the_launcher_recovery_page_reads_the_saved_project_once_per_selection() {
    let (mut app, root) = saved_project("launcher-recovery-frames");
    open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );
    app.state.project.project_launcher_page = ProjectLauncherPage::Recovery;
    let second = root.join("bias.rsp.autosave");
    app.state.platform.recovery_catalog = RecoveryCatalog::fixture(vec![
        RecoveryCandidate::fixture(root.join("amp.rsp"), root.join("amp.rsp.autosave"), "amp"),
        RecoveryCandidate::fixture(root.join("bias.rsp"), second.clone(), "bias"),
    ]);
    let ctx = offscreen();

    take_disk_reads();
    let first = first_seen(&ctx, &mut app);
    assert!(
        painted(&first).contains("Open recovery comparison"),
        "{}",
        painted(&first)
    );
    assert_eq!(take_disk_reads().project_files, 1);
    for _ in 0..IDLE_FRAMES {
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(take_disk_reads(), DiskReads::default());

    app.state.platform.recovery_catalog.select(second);
    for _ in 0..IDLE_FRAMES {
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(
        take_disk_reads(),
        DiskReads {
            project_files: 1,
            ..DiskReads::default()
        }
    );
    let _ = std::fs::remove_dir_all(root);
}

/// The launcher's project list is built when what it is filtered and sorted
/// by changes; it was built twice on every frame.
#[test]
fn the_launcher_builds_its_project_list_once_per_change() {
    let mut app = RSpiceApp::test_instance();
    let recent = |path: &str| RecentFile {
        kind: RecentKind::Project,
        path: std::path::PathBuf::from(path),
        opened_at_unix_ms: 0,
        pinned: false,
        missing: false,
    };
    app.state.platform.recent_files = vec![
        recent("C:/work/amplifier.rspiceproj"),
        recent("C:/work/bandgap.rspiceproj"),
        recent("C:/work/filter.rspiceproj"),
    ];
    open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );
    let ctx = offscreen();
    let builds = || PROJECT_LIST_BUILDS.with(|builds| builds.replace(0));

    builds();
    let first = first_seen(&ctx, &mut app);
    assert!(painted(&first).contains("bandgap"), "{}", painted(&first));
    for _ in 0..IDLE_FRAMES {
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(builds(), 1);

    app.state.project.project_launcher_query = "band".to_owned();
    let filtered = app_frame(&ctx, &mut app, Vec::new());
    for _ in 0..IDLE_FRAMES {
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(builds(), 1);
    let text = painted(&filtered);
    assert!(
        text.contains("bandgap") && !text.contains("amplifier"),
        "{text}"
    );
}

/// "Create checkpoint" writes the checkpoint on a background thread. While
/// it runs the page goes on painting and says so; when it is done the new
/// checkpoint is in the list.
#[test]
fn a_checkpoint_created_on_the_recovery_page_is_written_while_the_page_paints() {
    let (mut app, root) = saved_project("recovery-page-create");
    for _ in 0..2 {
        create(&mut app.state, ProjectCheckpointReason::Manual).expect("checkpoint written");
    }
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Recovery;
    let ctx = offscreen();
    let listed = paint_until(&ctx, &mut app, |text| text.contains("2 checkpoints"));

    let hold = CheckpointJobHold::default();
    app.state.project.project_checkpoint_recovery.job_hold = Some(hold.clone());
    assert_eq!(restores_offered(&listed), [true, true]);
    app_frame(&ctx, &mut app, click_on(&listed, "Create checkpoint"));
    for _ in 0..IDLE_FRAMES {
        let waiting = app_frame(&ctx, &mut app, Vec::new());
        let text = painted(&waiting);
        assert!(text.contains("Creating checkpoint\u{2026}"), "{text}");
        assert!(text.contains("2 checkpoints"), "{text}");
        // Restoring saves a copy from the list the job is about to change.
        assert_eq!(restores_offered(&waiting), [false, false]);
    }

    hold.release();
    let created = paint_until(&ctx, &mut app, |text| text.contains("3 checkpoints"));
    let text = painted(&created);
    assert!(text.contains("Create checkpoint"), "{text}");
    assert!(!text.contains("Creating checkpoint\u{2026}"), "{text}");
    assert_eq!(restores_offered(&created), [true, true, true]);
    assert!(
        app.state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message.starts_with("Created project checkpoint")),
        "the Console records the checkpoint"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// Whether each checkpoint row's Restore… can be pressed, top to bottom.
fn restores_offered(output: &egui::FullOutput) -> Vec<bool> {
    let Some(update) = output.platform_output.accesskit_update.as_ref() else {
        return Vec::new();
    };
    let mut restores = update
        .nodes
        .iter()
        .filter(|(_, node)| {
            node.label().is_some_and(|label| {
                label.starts_with("Restore ") && label.ends_with(" as a new project")
            })
        })
        .map(|(_, node)| {
            (
                node.bounds().map_or(0.0, |bounds| bounds.y0),
                !node.is_disabled(),
            )
        })
        .collect::<Vec<_>>();
    restores.sort_by(|a, b| a.0.total_cmp(&b.0));
    restores.into_iter().map(|(_, offered)| offered).collect()
}

/// A checkpoint that cannot be written raises a notice: the list does not
/// change, so without one the button looks as if it did nothing.
#[test]
fn a_checkpoint_that_cannot_be_written_raises_a_notice() {
    let (mut app, root) = saved_project("recovery-page-create-fails");
    create(&mut app.state, ProjectCheckpointReason::Manual).expect("checkpoint written");
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Recovery;
    let ctx = offscreen();
    let listed = paint_until(&ctx, &mut app, |text| {
        text.lines().any(|line| line == "1 checkpoint")
    });
    let raised = |app: &RSpiceApp| {
        app.state
            .workbench
            .toasts
            .activity()
            .iter()
            .filter(|toast| toast.title() == "Checkpoint not created")
            .count()
    };
    assert_eq!(raised(&app), 0);

    // The recovery folder becomes a file: nothing can be written into it.
    let folder = root.join(".rspice-recovery");
    std::fs::remove_dir_all(&folder).expect("recovery folder removed");
    std::fs::write(&folder, b"not a folder").expect("a file in its place");

    let mut events = click_on(&listed, "Create checkpoint");
    let deadline = Instant::now() + Duration::from_secs(30);
    while raised(&app) == 0 {
        assert!(Instant::now() < deadline, "no notice was raised");
        app_frame(&ctx, &mut app, std::mem::take(&mut events));
        app.state.drain_notices(&ctx);
        std::thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(raised(&app), 1);
    let _ = std::fs::remove_dir_all(root);
}

/// Revision history builds its table when it opens; a frame that only
/// paints the open dialog builds nothing.
#[test]
fn revision_history_builds_its_rows_when_it_opens_not_every_frame() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Project);
    app.execute_command_key(ProjectCommand::RevisionHistory.key());
    assert!(app.state.project.project_revision_history.open);
    let ctx = offscreen();
    let builds = || AUDIT_ROW_BUILDS.with(|builds| builds.replace(0));

    builds();
    let first = first_seen(&ctx, &mut app);
    assert!(
        painted(&first).contains("Project revision and audit history"),
        "{}",
        painted(&first)
    );
    assert_eq!(builds(), 1);
    for _ in 0..IDLE_FRAMES {
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(builds(), 0);
}

/// "Restore autosave?" states the checkpoint's age from the time it was
/// found with, not from the file on every frame: the file is gone here, and
/// the dialog still says how old the autosave is.
#[test]
fn the_restore_autosave_dialog_states_the_age_it_was_offered_with() {
    let root = unique_temp_dir("autosave-age");
    let mut app = RSpiceApp::test_instance();
    let now = rspice_app_state::time_compat::checked_unix_time_ms().expect("clock");
    app.state.platform.pending_autosave_restore = Some(AutosaveRestoreCandidate {
        source: root.join("amp.rsp"),
        checkpoint: root.join("amp.rsp.autosave"),
        binding: CheckpointBinding {
            identity: CheckpointIdentity {
                content_digest: rspice_app_state::product::ContentDigest::from_bytes([0x5a; 32]),
                byte_len: 1,
                modified_unix_ms: now - 7 * 60_000,
            },
            ownership: CheckpointOwnership::NoManifest,
            source_snapshot: None,
        },
    });
    let ctx = offscreen();

    take_disk_reads();
    let mut text = String::new();
    for _ in 0..=IDLE_FRAMES {
        text = painted(&app_frame(&ctx, &mut app, Vec::new()));
    }
    assert!(text.contains("from 7 min ago"), "{text}");
    assert_eq!(take_disk_reads(), DiskReads::default());
    let _ = std::fs::remove_dir_all(root);
}
