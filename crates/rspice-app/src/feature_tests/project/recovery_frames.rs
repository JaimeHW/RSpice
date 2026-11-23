//! Recovery, the launcher and the History page through whole application
//! frames: an idle frame reads no project file, lists no checkpoint
//! directory, stats no file and rebuilds no list, and checkpoint work runs
//! while the page goes on painting.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};

use egui::{Context, Modifiers, Pos2, Rect, Vec2, pos2, vec2};

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::app::dialogs::project::CheckpointJobHold;
use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_project::workbench::project::open_project_launcher;
use rspice_app_project::workbench::project::revision_history::AUDIT_ROW_BUILDS;
use rspice_app_project::workbench::project::workspace_state::{ProjectLauncherPage, ProjectPage};
use rspice_app_project::workbench::surfaces::project::history::HISTORY_ROW_BUILDS;
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
    orphan_test_checkpoint, write_checkpoint,
};
use rspice_app_state::state::{ComponentType, Point, SchematicState};
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
/// `label`: the topmost, when several are, as a reader takes the first row
/// of a list.
pub(super) fn click_on(output: &egui::FullOutput, label: &str) -> Vec<egui::Event> {
    click_where(
        output,
        |announced| announced == label,
        egui::PointerButton::Primary,
    )
}

/// A press and release of `button` in the middle of the topmost control
/// `output` announces with a label `named` picks.
pub(super) fn click_where(
    output: &egui::FullOutput,
    named: impl Fn(&str) -> bool,
    button: egui::PointerButton,
) -> Vec<egui::Event> {
    let bounds = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| named(node.label()?).then_some(node.bounds()?))
        .min_by(|a, b| a.y0.total_cmp(&b.y0))
        .expect("the frame announces the control");
    let at = pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button,
            pressed: true,
            modifiers: Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button,
            pressed: false,
            modifiers: Modifiers::default(),
        },
    ]
}

/// A click on the lowest control announced as `label`: a dialog's button
/// rather than one of the same name on the window's frame above it.
pub(super) fn click_lowest(output: &egui::FullOutput, label: &str) -> Vec<egui::Event> {
    let bounds = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| (node.label()? == label).then_some(node.bounds()?))
        .max_by(|a, b| a.y0.total_cmp(&b.y0))
        .unwrap_or_else(|| panic!("the frame announces `{label}`"));
    let at = pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    std::iter::once(egui::Event::PointerMoved(at))
        .chain([true, false].map(|pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: Modifiers::default(),
        }))
        .collect()
}

/// Whether the control `output` announces as `label` can be pressed.
pub(super) fn offered(output: &egui::FullOutput, label: &str) -> bool {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find(|(_, node)| node.label() == Some(label))
        .map(|(_, node)| !node.is_disabled())
        .unwrap_or_else(|| panic!("the frame announces `{label}`"))
}

/// A press and release of `key`, with no modifier held.
pub(super) fn key_press(key: egui::Key) -> Vec<egui::Event> {
    [true, false]
        .into_iter()
        .map(|pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: Modifiers::default(),
        })
        .collect()
}

/// Select the topmost History row whose Change is `change`, as `shown`
/// lists it, and paint the frame after.
pub(super) fn select_row(
    ctx: &Context,
    app: &mut RSpiceApp,
    shown: &egui::FullOutput,
    change: &str,
) -> egui::FullOutput {
    let named = format!("Change: {change},");
    let click = click_where(
        shown,
        |label| label.contains(&named),
        egui::PointerButton::Primary,
    );
    app_frame(ctx, app, click);
    app_frame(ctx, app, Vec::new())
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

/// Paint until the frame announces `label` with a value `done` holds for:
/// a path the inspector elides to its width is announced whole.
fn paint_until_announced(
    ctx: &Context,
    app: &mut RSpiceApp,
    label: &str,
    done: impl Fn(&str) -> bool,
) -> egui::FullOutput {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let output = app_frame(ctx, app, Vec::new());
        if super::configuration_editing::nodes(&output)
            .iter()
            .any(|node| node.label == label && done(&node.value))
        {
            return output;
        }
        assert!(
            Instant::now() < deadline,
            "the frame never announced {label:?} as waited for:\n{}",
            painted(&output)
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// A saved, clean project in its own folder, so its checkpoints are kept
/// there and nowhere else on the machine.
pub(super) fn saved_project(label: &str) -> (RSpiceApp, std::path::PathBuf) {
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
    *app.state.platform.recovery_catalog = RecoveryCatalog::fixture(vec![
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

/// The launcher looks for the recent drawings' autosaves on a background
/// thread: stating and reading them can block on a slow drive. The frame
/// that asks reads nothing and says it is looking, and the autosave is
/// listed on a later frame.
#[test]
fn the_launcher_looks_for_autosaves_off_the_frame() {
    let root = unique_temp_dir("launcher-discovery");
    let schematic_path = root.join("amp.rsch");
    let mut drawing = SchematicState::default();
    rspice_app_services::io::save_schematic(&drawing, &schematic_path, false)
        .expect("the drawing is saved");
    drawing.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Resistor, Point::new(40, 40));
    });
    let checkpoint = write_checkpoint(&schematic_path, &drawing).expect("the autosave is written");
    orphan_test_checkpoint(&checkpoint).expect("the autosave is left as a crash leaves it");

    let mut app = RSpiceApp::test_instance();
    app.state.platform.recent_files = vec![RecentFile {
        kind: RecentKind::Schematic,
        path: schematic_path,
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

    take_disk_reads();
    let asked = first_seen(&ctx, &mut app);
    let reads = take_disk_reads();
    assert_eq!((reads.checkpoint_directories, reads.file_stats), (0, 0));
    let text = painted(&asked);
    assert!(
        text.contains("Looking for autosaves\u{2026}") || text.contains("1 component"),
        "{text}"
    );
    assert!(!text.contains("No interrupted work found"), "{text}");
    paint_until(&ctx, &mut app, |text| text.contains("1 component"));
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
/// it runs the page goes on painting and says so, and the selected
/// checkpoint cannot be restored; when it is done the new checkpoint is in
/// the list.
#[test]
fn a_checkpoint_created_on_the_history_page_is_written_while_the_page_paints() {
    let (mut app, root) = saved_project("recovery-page-create");
    for _ in 0..2 {
        create(&mut app.state, ProjectCheckpointReason::Manual).expect("checkpoint written");
    }
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::History;
    let ctx = offscreen();
    let listed = paint_until(&ctx, &mut app, |text| text.contains("2 checkpoints \u{b7}"));
    let selected = select_row(&ctx, &mut app, &listed, "Manual checkpoint");

    let hold = CheckpointJobHold::default();
    app.state.project.project_checkpoint_recovery.job_hold = Some(hold.clone());
    assert!(offered(&selected, "Restore\u{2026}"));
    app_frame(&ctx, &mut app, click_on(&selected, "Create checkpoint"));
    for _ in 0..IDLE_FRAMES {
        let waiting = app_frame(&ctx, &mut app, Vec::new());
        let text = painted(&waiting);
        assert!(text.contains("Creating checkpoint\u{2026}"), "{text}");
        assert!(text.contains("2 checkpoints \u{b7}"), "{text}");
        // Restoring saves a copy from the list the job is about to change.
        assert!(!offered(&waiting, "Restore\u{2026}"));
    }

    hold.release();
    let created = paint_until(&ctx, &mut app, |text| text.contains("3 checkpoints \u{b7}"));
    let text = painted(&created);
    assert!(text.contains("Create checkpoint"), "{text}");
    assert!(!text.contains("Creating checkpoint\u{2026}"), "{text}");
    assert!(offered(&created, "Restore\u{2026}"));
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

/// A checkpoint whose copy of the project changed on disk is listed as
/// damaged, and its record says what is wrong and what to do; the store's
/// own account of it goes to the Console once, not again when the page lists
/// it again.
#[test]
fn a_damaged_checkpoint_says_what_is_wrong_and_the_console_keeps_the_store_text() {
    let (mut app, root) = saved_project("recovery-page-damaged");
    for _ in 0..2 {
        create(&mut app.state, ProjectCheckpointReason::Manual).expect("checkpoint written");
    }
    let folder = root.join("idle.rspice-recovery");
    let copy = std::fs::read_dir(&folder)
        .expect("the recovery folder lists")
        .map(|entry| entry.expect("an entry").path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension == "rspiceproj")
        })
        .expect("a checkpoint keeps a copy of the project");
    std::fs::write(&copy, b"cut short").expect("the copy changes on disk");

    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::History;
    let ctx = offscreen();
    let listed = paint_until(&ctx, &mut app, |text| text.contains("1 damaged"));
    assert!(
        painted(&listed).contains("Copy changed"),
        "{}",
        painted(&listed)
    );
    // The damaged one bears the same name as the sound one beside it.
    let set_aside = click_where(
        &listed,
        |label| label.contains("Kind: Set aside"),
        egui::PointerButton::Primary,
    );
    app_frame(&ctx, &mut app, set_aside);
    let text = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(
        text.contains("The copy of the project it keeps has changed since it was taken"),
        "{text}"
    );
    assert!(!text.contains("integrity manifest"), "{text}");
    let console = |app: &RSpiceApp| {
        app.state
            .model
            .log_buffer
            .entries()
            .filter(|entry| entry.message.starts_with("Damaged checkpoint"))
            .map(|entry| entry.message.clone())
            .collect::<Vec<_>>()
    };
    let lines = console(&app);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(lines[0].contains("integrity manifest"), "{lines:?}");

    app.state.project.project_checkpoint_recovery.invalidate();
    paint_until(&ctx, &mut app, |text| text.contains("1 damaged"));
    for _ in 0..IDLE_FRAMES {
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(console(&app).len(), 1, "a listing again finds nothing new");
    let _ = std::fs::remove_dir_all(root);
}

/// A damaged checkpoint whose record still reads is named as its row was, by
/// its name and when it was taken, never by its identity.
#[test]
fn a_damaged_checkpoint_is_named_by_its_name_and_time() {
    let (mut app, root) = saved_project("recovery-page-damaged-named");
    create(&mut app.state, ProjectCheckpointReason::Manual).expect("checkpoint written");
    let folder = root.join("idle.rspice-recovery");
    let copy = std::fs::read_dir(&folder)
        .expect("the recovery folder lists")
        .map(|entry| entry.expect("an entry").path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension == "rspiceproj")
        })
        .expect("a checkpoint keeps a copy of the project");
    std::fs::remove_file(&copy).expect("the copy goes missing");

    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::History;
    let ctx = offscreen();
    let listed = paint_until(&ctx, &mut app, |text| text.contains("1 damaged"));
    let text = painted(&listed);
    let record = painted(&select_row(&ctx, &mut app, &listed, "Manual checkpoint"));
    let damaged = &app.state.project.project_checkpoint_recovery.quarantined[0];
    assert_eq!(damaged.title(), Some("Manual checkpoint"));
    let taken = rspice_app_state::time_compat::local_stamp(
        damaged
            .created_unix_ms()
            .expect("its record says when it was taken"),
    );
    assert!(text.contains("Manual checkpoint"), "{text}");
    assert!(text.contains(&taken), "{taken} missing:\n{text}");
    assert!(text.contains("Copy missing"), "{text}");
    assert!(
        record.contains("The copy of the project it keeps is missing."),
        "{record}"
    );
    assert!(!text.contains(damaged.label()), "{text}");
    assert!(!record.contains(damaged.label()), "{record}");
    let _ = std::fs::remove_dir_all(root);
}

/// A checkpoint that cannot be written raises a notice: the list does not
/// change, so without one the button looks as if it did nothing.
#[test]
fn a_checkpoint_that_cannot_be_written_raises_a_notice() {
    let (mut app, root) = saved_project("recovery-page-create-fails");
    create(&mut app.state, ProjectCheckpointReason::Manual).expect("checkpoint written");
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::History;
    let ctx = offscreen();
    let listed = paint_until(&ctx, &mut app, |text| {
        text.lines()
            .any(|line| line.starts_with("1 checkpoint \u{b7}"))
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
    let folder = root.join("idle.rspice-recovery");
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

/// The History page builds its rows when it shows, from the journal and
/// from the checkpoints; a frame that only paints the page builds nothing.
#[test]
fn the_history_page_builds_its_rows_when_it_shows_not_every_frame() {
    let (mut app, root) = saved_project("history-page-builds");
    app.state.workbench.activate(Workspace::Project);
    app.execute_command_key(ProjectCommand::RevisionHistory.key());
    assert_eq!(app.state.project.project_page, ProjectPage::History);
    let ctx = offscreen();
    let journal = || AUDIT_ROW_BUILDS.with(|builds| builds.replace(0));
    let listed = || HISTORY_ROW_BUILDS.with(|builds| builds.replace(0));

    journal();
    listed();
    paint_until(&ctx, &mut app, |text| text.contains("0 checkpoints \u{b7}"));
    assert_eq!(journal(), 1, "the journal is read once");
    assert!(listed() > 0, "the rows are listed");
    for _ in 0..IDLE_FRAMES {
        app_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(journal(), 0);
    assert_eq!(listed(), 0);
    let _ = std::fs::remove_dir_all(root);
}

/// "Restore autosave?" states the autosave's time and age from when it was
/// found, not from the file on every frame: the file is gone here, and the
/// dialog still says how old the autosave is.
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
    assert!(
        text.contains("Restore the autosaved drawing from "),
        "{text}"
    );
    assert!(text.contains("(7 min ago)"), "{text}");
    assert_eq!(take_disk_reads(), DiskReads::default());
    let _ = std::fs::remove_dir_all(root);
}

/// After a Save as, the inspector beside the Overview and the one beside the
/// Recovery page state one revision, the project's, and the project
/// inspector names the new file, on frames painted one after another in one
/// context, as a reader moving between the pages sees them.
#[test]
fn after_a_save_as_the_overview_and_the_history_page_state_one_revision() {
    let (mut app, root) = saved_project("save-as-revision");
    for name in ["Amplifier a", "Amplifier b", "Amplifier c", "Amplifier d"] {
        app.state
            .model
            .workspace
            .content
            .project
            .rename(name)
            .expect("the project is renamed");
    }
    assert!(
        app.state
            .save_project_to_path(&root.join("idle.rspiceproj"))
    );
    let revision = |app: &RSpiceApp| {
        format!(
            "revision {}",
            app.state.model.workspace.content.project.revision().get()
        )
    };
    // The project inspector beside the Overview: at that revision, with
    // nothing left to save.
    let saved_at = |text: &str, revision: &str| {
        let lines = text.lines().collect::<Vec<_>>();
        let number = &revision["revision ".len()..];
        lines.windows(2).any(|pair| pair == ["Revision", number])
            && lines.windows(2).any(|pair| pair == ["Unsaved", "None"])
    };
    let ctx = offscreen();
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Overview;
    let before = revision(&app);
    paint_until(&ctx, &mut app, |text| saved_at(text, &before));

    assert!(
        app.state
            .save_project_as_to_path(&root.join("Amplifier.rspiceproj"))
    );
    let after = revision(&app);
    assert_eq!(after, before, "a Save as keeps the project's revision");
    let named = |path: &str| path.ends_with("Amplifier.rspiceproj");
    let overview = paint_until_announced(&ctx, &mut app, "Location", named);
    assert!(
        saved_at(&painted(&overview), &after),
        "{}",
        painted(&overview)
    );
    // With no checkpoint selected, the inspector beside the page states it.
    app.state.project.project_page = ProjectPage::History;
    let number = &after["Revision ".len()..];
    paint_until(&ctx, &mut app, |text| {
        text.lines()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|pair| pair == ["Revision", number])
    });
    app.state.project.project_page = ProjectPage::Overview;
    let overview = paint_until_announced(&ctx, &mut app, "Location", named);
    assert!(saved_at(&painted(&overview), &after));
    let _ = std::fs::remove_dir_all(root);
}

/// With nothing to recover, the launcher's Recovery page offers the one
/// action that applies, Safe mode, and none that acts on a selection there
/// cannot be.
#[test]
fn the_empty_launcher_recovery_page_offers_only_safe_mode() {
    let mut app = RSpiceApp::test_instance();
    open_project_launcher(
        &mut app.state.project,
        &mut app.state.workbench,
        &mut app.state.platform,
    );
    app.state.project.project_launcher_page = ProjectLauncherPage::Recovery;
    *app.state.platform.recovery_catalog = RecoveryCatalog::fixture(Vec::new());
    let ctx = offscreen();
    let shown = paint_until(&ctx, &mut app, |text| {
        text.contains("No interrupted work found")
    });
    let text = painted(&shown);
    assert!(text.contains("Safe mode\u{2026}"), "{text}");
    for dead in [
        "Discard selected autosave",
        "Open recovery comparison",
        "Save as new project",
    ] {
        assert!(
            !text.contains(dead),
            "{dead:?} offered over nothing:\n{text}"
        );
    }
}
