//! How a saved project carries the occurrence each open document is editing.
//!
//! A save must not write the session-global breadcrumb: two places holding the
//! same fact is how they drift.

use super::*;

fn schematic(cell: &str) -> CellViewRef {
    CellViewRef::new("user", cell, "schematic")
}

/// A project whose libraries hold `top` and `amp`, with both open.
fn two_document_project() -> ProjectSnapshot {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    libraries
        .get_library_mut("user")
        .expect("the bootstrapped project owns its design library")
        .add_cell(amp);
    workspace.open_view(schematic("amp"), ViewType::Schematic);
    workspace.ensure_library_model(&mut libraries);
    ProjectSnapshot::new(workspace, libraries)
}

#[test]
fn a_saved_project_writes_occurrences_and_not_the_old_global_breadcrumb() {
    let mut project = two_document_project();
    // The descent has to start at the parent: descending into the document
    // that is already open is the same occurrence, and records no step.
    let mut workspace =
        ProjectWorkspace::from_parts(project.file.workspace, project.workspace_session);
    workspace.activate_view(schematic("top"), ViewType::Schematic);
    workspace.descend_into("XAMP".to_owned(), schematic("amp"), ViewType::Schematic);
    (project.file.workspace, project.workspace_session) = workspace.into_parts();

    let text = serialize_project_file(&project).expect("the project serializes");
    let value: serde_json::Value = serde_json::from_str(&text).expect("the project is JSON");
    assert!(
        value["workspace"].get("hierarchy_stack").is_none()
            && value["workspace"].get("hierarchy_instances").is_none(),
        "the session-global breadcrumb is a projection, so it is never written"
    );
    assert!(
        text.contains("\"occurrence\"") && text.contains("\"XAMP\""),
        "the occurrence rides on the document that is editing it"
    );

    let restored = load_project_text(&text, None).expect("the project reloads");
    restored
        .file
        .validate()
        .expect("the restored project validates");
    assert!(restored.file.workspace_migration_warning.is_none());
    assert_eq!(
        restored.file.workspace.occurrence_path().to_string(),
        "/XAMP"
    );
    assert_eq!(restored.file.workspace.active_view, schematic("amp"));
}
