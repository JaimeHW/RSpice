//! Existing project parser bounds and identity-route checks.

use super::*;

#[test]
fn in_memory_project_text_is_size_checked_before_parsing() {
    assert!(validate_project_text_size(MAX_PROJECT_FILE_BYTES as usize).is_ok());
    let error = validate_project_text_size(MAX_PROJECT_FILE_BYTES as usize + 1)
        .expect_err("oversized project text is rejected");
    assert!(matches!(error, ProjectIoError::InvalidData(_)));
    assert!(error.to_string().contains("supported maximum"));
    assert!(validate_legacy_project_text_size(MAX_LEGACY_PROJECT_FILE_BYTES as usize).is_ok());
    let legacy_error =
        validate_legacy_project_text_size(MAX_LEGACY_PROJECT_FILE_BYTES as usize + 1)
            .expect_err("oversized legacy materialization is rejected");
    assert!(matches!(legacy_error, ProjectIoError::InvalidData(_)));
    assert!(legacy_error.to_string().contains("identity injection"));
}

#[test]
fn current_project_text_routes_to_direct_deserialization() {
    let project = ProjectFile::new(ProjectWorkspace::default(), ProjectLibraries::default());
    let json = serde_json::to_string(&project).expect("current project serializes");

    assert_eq!(
        project_text_load_route(&json).expect("current route probes"),
        ProjectTextLoadRoute::Direct
    );
    assert_eq!(
        project_text_load_route(r#"{"workspace":{"project":{"schema_version":null,"id":null}}}"#)
            .expect("present null fields still probe"),
        ProjectTextLoadRoute::Direct,
        "legacy ID injection is permitted only when both keys are absent"
    );
    let mut legacy: serde_json::Value = serde_json::from_str(&json).expect("current wire record");
    let descriptor = legacy["workspace"]["project"]
        .as_object_mut()
        .expect("project descriptor");
    descriptor.remove("id");
    descriptor.remove("schema_version");
    assert_eq!(
        project_text_load_route(&legacy.to_string()).expect("legacy route probes"),
        ProjectTextLoadRoute::LegacyProjectIdInjection
    );
}
