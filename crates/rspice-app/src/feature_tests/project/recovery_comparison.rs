//! Opening a recovery comparison: the comparison is a project of its own,
//! and no save of it ever writes the project that was open before it.

#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};

use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_services::io::NativeFileWorkflowIo;
use rspice_app_services::services::recent_files::{RecentFile, RecentKind};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    DestinationAuthority, SaveScope, canonical_native_path,
};
use rspice_app_services::workbench::lifecycle::recovery::{
    RecoveryCandidate, refresh_catalog_if_requested, unique_temp_dir,
};
use rspice_app_services::workbench::lifecycle::recovery_checkpoint::{
    orphan_test_checkpoint, write_checkpoint,
};
use rspice_app_state::state::{ComponentType, Point, SchematicState};

use crate::workbench::RSpiceApp;
use crate::workbench::app_state::services_access::{mark_project_closed, open_comparison};

/// A standalone schematic on disk and an autosave of a later edit to it,
/// left behind as a session that ended without cleaning up leaves it.
fn crashed_autosave(root: &Path) -> PathBuf {
    let schematic_path = root.join("amp.rsch");
    let mut drawing = SchematicState::default();
    rspice_app_services::io::save_schematic(&drawing, &schematic_path, false)
        .expect("the standalone schematic is saved");
    drawing.with_undo("Place", |schematic| {
        schematic.add_component(ComponentType::Resistor, Point::new(40, 40));
    });
    let checkpoint =
        write_checkpoint(&schematic_path, &drawing).expect("the autosave checkpoint is written");
    orphan_test_checkpoint(&checkpoint).expect("the checkpoint is left as a crash leaves it");
    schematic_path
}

/// The candidate the launcher's Recovery page lists for `schematic_path`.
fn listed_candidate(app: &mut RSpiceApp, schematic_path: &Path) -> RecoveryCandidate {
    app.state.platform.recent_files.push(RecentFile {
        kind: RecentKind::Schematic,
        path: schematic_path.to_path_buf(),
        opened_at_unix_ms: 0,
        pinned: false,
        owner: None,
        tags: Vec::new(),
    });
    app.state.platform.recovery_catalog.request_refresh();
    refresh_catalog_if_requested(
        &mut app.state.platform,
        &app.state.model,
        &NativeFileWorkflowIo,
    );
    app.state
        .platform
        .recovery_catalog
        .candidates
        .first()
        .cloned()
        .expect("the Recovery page lists the autosave")
}

fn project_id_on_disk(path: &Path) -> String {
    rspice_app_services::io::load_project_file(path)
        .expect("the project file loads")
        .file
        .workspace
        .project
        .id()
        .to_string()
}

#[test]
fn a_recovery_comparison_never_saves_over_the_project_that_was_open() {
    let root = unique_temp_dir("comparison-keeps-open-project");
    let schematic_path = crashed_autosave(&root);

    let mut app = RSpiceApp::test_instance();
    for index in 0..3 {
        app.state.model.schematic.with_undo("Place", |schematic| {
            schematic.add_component(ComponentType::Capacitor, Point::new(40 * index, 0));
        });
    }
    let project_path = root.join("p.rspiceproj");
    assert!(
        app.state.save_project_to_path(&project_path),
        "the open project is saved"
    );
    let saved_bytes = std::fs::read(&project_path).expect("the project file is read");
    let saved_id = project_id_on_disk(&project_path);

    let candidate = listed_candidate(&mut app, &schematic_path);
    open_comparison(&mut app.state, &candidate).expect("the comparison opens");

    assert_eq!(
        canonical_native_path(&app.state.platform),
        None,
        "the comparison belongs to no file, so Save asks where to write"
    );
    assert!(app.state.platform.project_lifecycle.is_open());
    assert_ne!(
        app.state.model.workspace.content.project.id().to_string(),
        saved_id,
        "the comparison is a project of its own"
    );

    // Saving the comparison where the reader picks writes there and only there.
    let comparison_path = root.join("amp recovery.rspiceproj");
    app.state
        .save_project_scope_to(
            SaveScope::AllDocuments,
            &comparison_path,
            DestinationAuthority::UserSelected,
        )
        .expect("the comparison is saved where the reader picked");
    assert_eq!(
        std::fs::read(&project_path).expect("the project file is read again"),
        saved_bytes,
        "the project that was open is byte for byte as it was saved"
    );
    assert_ne!(project_id_on_disk(&comparison_path), saved_id);
    let bound = canonical_native_path(&app.state.platform)
        .expect("after its first save the comparison is bound to a file");
    assert_eq!(
        std::fs::canonicalize(&bound).expect("the bound file exists"),
        std::fs::canonicalize(&comparison_path).expect("the saved file exists"),
        "after its first save the comparison is bound to its own file"
    );

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn a_recovery_comparison_opened_with_no_project_can_be_saved() {
    let root = unique_temp_dir("comparison-without-project");
    let schematic_path = crashed_autosave(&root);

    let mut app = RSpiceApp::test_instance();
    mark_project_closed(&mut app.state);
    assert!(!app.state.platform.project_lifecycle.is_open());

    let candidate = listed_candidate(&mut app, &schematic_path);
    open_comparison(&mut app.state, &candidate).expect("the comparison opens");

    assert!(app.state.platform.project_lifecycle.is_open());
    for command in [ProjectCommand::Save, ProjectCommand::SaveAs] {
        assert!(
            app.command_availability(command.key()).is_available(),
            "{command:?} is offered for the recovered drawing"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}
