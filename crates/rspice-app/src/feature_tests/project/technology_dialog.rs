//! Attaching and detaching the project's technology from the Dependencies
//! page, through whole application frames.

#![cfg(not(target_arch = "wasm32"))]

use std::path::PathBuf;

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::feature::CloudSessionPort;

use super::recovery_frames::{app_frame, click_on, offscreen, painted, saved_project};

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
    let page = app_frame(&ctx, &mut app, Vec::new());
    app_frame(
        &ctx,
        &mut app,
        click_on(&page, "Attach model technology\u{2026}"),
    );
    app_frame(&ctx, &mut app, Vec::new());
    let dialog = app_frame(&ctx, &mut app, Vec::new());
    let text = painted(&dialog);
    for label in ["Changed by", "Approved by", "Reason (optional)"] {
        assert!(text.contains(label), "the dialog labels {label}: {text}");
    }
    assert_eq!(announced_value(&dialog, "Changed by"), Some(user.clone()));
    assert_eq!(announced_value(&dialog, "Approved by"), Some(user.clone()));
    assert!(!text.contains("Still to choose"), "{text}");

    app_frame(&ctx, &mut app, click_on(&dialog, "Attach binding"));
    let project = &app.state.model.workspace.content.project;
    assert!(
        project.technology_binding().is_some(),
        "the technology attaches"
    );
    let receipt = project
        .technology_change_audit()
        .last()
        .expect("the change is in the technology history");
    assert_eq!(receipt.actor_id(), user);
    assert_eq!(receipt.authority_id(), user);
    assert_eq!(receipt.reason(), "");
    let _ = std::fs::remove_dir_all(root);
}
