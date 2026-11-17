//! Attaching and detaching the project's technology from the Dependencies
//! page, through whole application frames.

#![cfg(not(target_arch = "wasm32"))]

use std::path::PathBuf;

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::app::dialogs::project::CheckpointJobHold;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::feature::CloudSessionPort;

use super::recovery_frames::{IDLE_FRAMES, app_frame, click_on, offscreen, painted, saved_project};

/// The line the dialog states while the checkpoint before a change is
/// written.
const PENDING: &str = "Writing the recovery checkpoint\u{2026}";

/// The one model the test library holds.
const MODEL_CARD: &str = ".model nmos_demo nmos level=1 vto=0.55\n";

/// A saved project, open on Dependencies, with the test PDK installed and
/// one model library read from a file in the project's folder, and that
/// folder.
fn attachable_project(label: &str) -> (RSpiceApp, PathBuf) {
    let (mut app, root) = project_with_pdk(label);
    let file = root.join("demo180.lib");
    std::fs::write(&file, MODEL_CARD).expect("the model file writes");
    app.state
        .model
        .model_library_manager
        .load_library_file(&file, None)
        .expect("the model file imports");
    (app, root)
}

/// A saved project, open on Dependencies, with the test PDK installed.
fn project_with_pdk(label: &str) -> (RSpiceApp, PathBuf) {
    let (mut app, root) = saved_project(label);
    let (archive, authority) = rspice_app_state::state::pdk_config::pdk_technology_test_fixture();
    app.state
        .model
        .pdk_config
        .technology_registry
        .install_archive_bytes(&archive, &authority, "Install the test technology")
        .expect("the test PDK installs");
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

/// A model library whose bytes the project retains attaches on the desktop
/// as it does in the browser: the retained bytes are its proof, and the
/// files it was imported from are not read, here where they never existed.
#[test]
fn a_retained_model_library_attaches_without_its_original_files() {
    let (mut app, root) = project_with_pdk("technology-attach-retained");
    let name = app
        .state
        .model
        .model_library_manager
        .load_library_bytes("demo180.lib", MODEL_CARD.as_bytes().to_vec(), None)
        .expect("the model bytes import");
    let library = app
        .state
        .model
        .model_library_manager
        .get_library(&name)
        .expect("the library is in the catalogue");
    let original = library.root_path.clone().expect("the library has a root");
    assert!(!original.exists(), "{}", original.display());
    let ctx = offscreen();
    let dialog = open_attach_dialog(&ctx, &mut app);
    app_frame(&ctx, &mut app, click_on(&dialog, "Attach binding"));
    paint_until_attached(&ctx, &mut app);
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .project
            .technology_binding()
            .map(|binding| binding.model_library().to_owned()),
        Some(name)
    );
    let _ = std::fs::remove_dir_all(root);
}

/// Detach sits beside Attach and takes the same checkpointed change: who
/// detaches it starts on the person at this application, the checkpoint is
/// written first, and the project is left with no technology and a receipt
/// that says it was detached.
#[test]
fn the_technology_detaches_from_the_page_through_a_checkpoint() {
    let (mut app, root) = attachable_project("technology-detach");
    let user = app.cloud.review_actor(&*app.state.cloud);
    let ctx = offscreen();
    let dialog = open_attach_dialog(&ctx, &mut app);
    app_frame(&ctx, &mut app, click_on(&dialog, "Attach binding"));
    paint_until_attached(&ctx, &mut app);
    let page = app_frame(&ctx, &mut app, Vec::new());
    assert_eq!(enabled(&page, "Detach\u{2026}"), Some(true));

    app_frame(&ctx, &mut app, click_on(&page, "Detach\u{2026}"));
    app_frame(&ctx, &mut app, Vec::new());
    let detach = app_frame(&ctx, &mut app, Vec::new());
    let text = painted(&detach);
    assert!(text.contains("Detach technology"), "{text}");
    assert_eq!(announced_value(&detach, "Changed by"), Some(user.clone()));
    app_frame(&ctx, &mut app, click_on(&detach, "Detach"));
    let mut last = String::new();
    for _ in 0..500 {
        last = painted(&app_frame(&ctx, &mut app, Vec::new()));
        if app
            .state
            .model
            .workspace
            .content
            .project
            .technology_binding()
            .is_none()
        {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let project = &app.state.model.workspace.content.project;
    assert!(project.technology_binding().is_none(), "{last}");
    let receipt = project
        .technology_change_audit()
        .last()
        .expect("the detach is in the technology history");
    assert_eq!(
        receipt.action(),
        rspice_project::ProjectTechnologyChangeAction::Detach
    );
    assert_eq!(receipt.actor_id(), user);
    assert!(
        app.state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message.starts_with("Detached ")),
        "the Console records the change"
    );
    let page = app_frame(&ctx, &mut app, Vec::new());
    assert_eq!(enabled(&page, "Detach\u{2026}"), Some(false));
    let _ = std::fs::remove_dir_all(root);
}
