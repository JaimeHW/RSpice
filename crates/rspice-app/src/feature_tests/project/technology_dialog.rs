//! Attaching and detaching the project's technology from the Dependencies
//! page, through whole application frames.

#![cfg(not(target_arch = "wasm32"))]

use std::path::PathBuf;

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::feature::CloudSessionPort;

use rspice_app_project::workbench::app::dialogs::project::CheckpointJobHold;

use super::recovery_frames::{IDLE_FRAMES, app_frame, click_on, offscreen, painted, saved_project};

/// The line the dialog states while the checkpoint before a change is
/// written.
const PENDING: &str = "Writing the recovery checkpoint\u{2026}";

/// A saved project, open on Dependencies, with the test PDK installed and
/// one model library read from a file in the project's folder, and that
/// folder.
fn attachable_project(label: &str) -> (RSpiceApp, PathBuf) {
    let (mut app, root) = saved_project(label);
    let model = &mut app.state.model;
    let (archive, authority) = rspice_app_state::state::pdk_config::pdk_technology_test_fixture();
    model
        .pdk_config
        .technology_registry
        .install_archive_bytes(&archive, &authority, "Install the test technology")
        .expect("the test PDK installs");
    let file = root.join("demo180.lib");
    std::fs::write(&file, ".model nmos_demo nmos level=1 vto=0.55\n")
        .expect("the model file writes");
    model
        .model_library_manager
        .load_library_file(&file, None)
        .expect("the model file imports");
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    (app, root)
}

/// The attach dialog opened from the page, as its first settled frame.
fn open_attach_dialog(ctx: &egui::Context, app: &mut RSpiceApp) -> egui::FullOutput {
    let page = app_frame(ctx, app, Vec::new());
    app_frame(ctx, app, click_on(&page, "Attach model technology\u{2026}"));
    app_frame(ctx, app, Vec::new());
    app_frame(ctx, app, Vec::new())
}

/// Frames until the project's technology is attached; the last one.
fn paint_until_attached(ctx: &egui::Context, app: &mut RSpiceApp) -> egui::FullOutput {
    let mut text = String::new();
    for _ in 0..500 {
        let output = app_frame(ctx, app, Vec::new());
        text = painted(&output);
        if app
            .state
            .model
            .workspace
            .content
            .project
            .technology_binding()
            .is_some()
        {
            return output;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    panic!("the technology never attached: {text}");
}

/// Whether the button labelled `label` can be pressed.
fn enabled(output: &egui::FullOutput, label: &str) -> Option<bool> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label()? == label).then(|| !node.is_disabled()))
}

/// The value the frame announced for the control labelled `label`.
fn announced_value(output: &egui::FullOutput, label: &str) -> Option<String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label()? == label).then(|| node.value().map(str::to_owned)))
        .flatten()
}

/// The attach dialog names who changes the technology and who approves it in
/// labelled fields that start on the person at this application, and asks
/// for no reason: Attach commits with the reason left empty, and the
/// project's technology history records those names.
#[test]
fn the_attach_dialog_starts_on_the_user_and_needs_no_reason() {
    let (mut app, root) = attachable_project("technology-attach-fields");
    let user = app.cloud.review_actor(&*app.state.cloud);
    let ctx = offscreen();
    let dialog = open_attach_dialog(&ctx, &mut app);
    let text = painted(&dialog);
    for label in ["Changed by", "Approved by", "Reason (optional)"] {
        assert!(text.contains(label), "the dialog labels {label}: {text}");
    }
    assert_eq!(announced_value(&dialog, "Changed by"), Some(user.clone()));
    assert_eq!(announced_value(&dialog, "Approved by"), Some(user.clone()));
    assert!(!text.contains("Still to choose"), "{text}");

    app_frame(&ctx, &mut app, click_on(&dialog, "Attach binding"));
    paint_until_attached(&ctx, &mut app);
    let project = &app.state.model.workspace.content.project;
    let receipt = project
        .technology_change_audit()
        .last()
        .expect("the change is in the technology history");
    assert_eq!(receipt.actor_id(), user);
    assert_eq!(receipt.authority_id(), user);
    assert_eq!(receipt.reason(), "");
    let _ = std::fs::remove_dir_all(root);
}

/// The checkpoint written before the technology changes is written off the
/// frame: while it is, the application goes on painting, the dialog states
/// one line about it and holds its button, and the project keeps its
/// technology. Once it is in, the change commits, the dialog closes and the
/// Console records it.
#[test]
fn an_attach_writes_its_checkpoint_while_the_application_paints() {
    let (mut app, root) = attachable_project("technology-attach-pending");
    let hold = CheckpointJobHold::default();
    app.state.project.project_checkpoint_recovery.job_hold = Some(hold.clone());
    let ctx = offscreen();
    let dialog = open_attach_dialog(&ctx, &mut app);
    assert_eq!(enabled(&dialog, "Attach binding"), Some(true));

    app_frame(&ctx, &mut app, click_on(&dialog, "Attach binding"));
    for _ in 0..IDLE_FRAMES {
        let waiting = app_frame(&ctx, &mut app, Vec::new());
        let text = painted(&waiting);
        assert_eq!(text.matches(PENDING).count(), 1, "{text}");
        assert_eq!(enabled(&waiting, "Attach binding"), Some(false), "{text}");
        assert!(
            app.state
                .model
                .workspace
                .content
                .project
                .technology_binding()
                .is_none(),
            "the technology waits on its checkpoint"
        );
    }

    hold.release();
    paint_until_attached(&ctx, &mut app);
    let closed = painted(&app_frame(&ctx, &mut app, Vec::new()));
    assert!(!closed.contains(PENDING), "{closed}");
    assert!(
        !app.state.project.technology_attachment.open,
        "the dialog closes once the change commits"
    );
    assert!(
        app.state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message.starts_with("Attached ")),
        "the Console records the change"
    );
    let _ = std::fs::remove_dir_all(root);
}
