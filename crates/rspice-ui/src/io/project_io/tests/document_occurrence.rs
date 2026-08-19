//! How a saved project carries the occurrence each open document is editing.
//!
//! The persisted shape moved from one session-global breadcrumb to a per
//! document occurrence. A save written before that still has to open, and a
//! save written after must not write the old pair back — two places holding
//! the same fact is how they drift.

use super::*;

fn schematic(cell: &str) -> CellViewRef {
    CellViewRef::new("user", cell, "schematic")
}

/// A project whose libraries hold `top` and `amp`, with both open.
fn two_document_project() -> ProjectFile {
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
    ProjectFile::new(workspace, libraries)
}

#[test]
fn a_saved_project_writes_occurrences_and_not_the_old_global_breadcrumb() {
    let mut project = two_document_project();
    project
        .workspace
        .descend_into("XAMP".to_owned(), schematic("amp"), ViewType::Schematic);

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
    restored.validate().expect("the restored project validates");
    assert!(restored.workspace_migration_warning.is_none());
    assert_eq!(restored.workspace.occurrence_path().to_string(), "/XAMP");
    assert_eq!(restored.workspace.active_view, schematic("amp"));
}

#[test]
fn a_save_written_before_occurrences_folds_its_breadcrumb_onto_the_active_document() {
    let project = two_document_project();
    let text = serialize_project_file(&project).expect("the project serializes");
    let mut value: serde_json::Value = serde_json::from_str(&text).expect("the project is JSON");
    for open_view in value["workspace"]["open_views"]
        .as_array_mut()
        .expect("the workspace has open documents")
    {
        open_view
            .as_object_mut()
            .expect("an open document is an object")
            .remove("occurrence");
    }
    value["workspace"]["active_view"] = serde_json::json!(schematic("amp"));
    value["workspace"]["hierarchy_stack"] = serde_json::json!([schematic("top"), schematic("amp")]);
    value["workspace"]["hierarchy_instances"] = serde_json::json!(["XAMP"]);

    let restored = load_project_text(&value.to_string(), None).expect("the legacy save loads");
    restored.validate().expect("the migrated project validates");
    assert!(restored.workspace_migration_warning.is_none());
    assert_eq!(restored.workspace.occurrence_path().to_string(), "/XAMP");
    assert_eq!(
        restored
            .workspace
            .active_occurrence()
            .map(|occurrence| occurrence.root.clone()),
        Some(schematic("top")),
        "the breadcrumb's root becomes the document's occurrence root"
    );

    let rewritten = serialize_project_file(&restored).expect("the migrated project serializes");
    assert!(
        !rewritten.contains("hierarchy_stack") && !rewritten.contains("hierarchy_instances"),
        "migration is a one-time rewrite onto the documents"
    );
}

#[test]
fn a_save_whose_breadcrumb_arrays_disagree_keeps_the_shorter_prefix_and_warns() {
    let project = two_document_project();
    let text = serialize_project_file(&project).expect("the project serializes");
    let mut value: serde_json::Value = serde_json::from_str(&text).expect("the project is JSON");
    value["workspace"]["hierarchy_stack"] = serde_json::json!([schematic("top"), schematic("amp")]);
    value["workspace"]["hierarchy_instances"] = serde_json::json!([]);

    let restored = load_project_text(&value.to_string(), None).expect("the legacy save loads");
    restored.validate().expect("the repaired project validates");
    let warning = restored
        .workspace_migration_warning
        .as_deref()
        .expect("dropping a level the save could not name owes the reader a warning");
    assert!(warning.contains("1 level"), "{warning}");
    assert!(
        restored.workspace.occurrence_path().is_root(),
        "an instance name that is missing is dropped, never invented from the cell name"
    );
}

/// The frozen reference is the one save on disk that predates occurrences
/// entirely, so it is the case that proves the migration reads real bytes.
#[test]
fn the_frozen_reference_project_gains_a_rooted_occurrence_per_document() {
    let project = load_project_text(include_str!("hierarchy_reference_legacy.rspiceproj"), None)
        .expect("the frozen reference loads");
    project.validate().expect("the frozen reference validates");
    assert!(project.workspace_migration_warning.is_none());

    for open_view in &project.workspace.open_views {
        assert!(!open_view.occurrence.is_unrooted());
        assert_eq!(open_view.occurrence.terminal_master(), &open_view.reference);
    }
    assert!(project.workspace.occurrence_path().is_root());
}
