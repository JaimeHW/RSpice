//! Decoding a portable shortcut artifact, in the case that binds the
//! product's own commands.

use crate::workbench::framework::command_registry::ids;
use crate::workbench::shortcuts::artifacts::schema::*;

#[test]
fn imported_acknowledgements_and_source_paths_are_stripped() {
    let decoded = decode_shortcut_artifact_json(
        r"C:\private\portable.json",
        r#"{
            "format":"rspice.shortcuts/1",
            "artifact":{
                "schemaVersion":1,
                "scope":"user-overrides",
                "coverage":{"contexts":["all"],"platforms":["desktop"],"policiesIncluded":true},
                "platformMappingsIncluded":false
            },
            "profile":{
                "protected-override-acknowledgements":["save-project"],
                "source-path":"C:/secret/project",
                "commands":{}
            }
        }"#,
    )
    .unwrap();

    assert_eq!(decoded.source_name(), "portable.json");
    assert!(!decoded.profile().protected_override_acknowledged(ids::SAVE));
    assert!(
        decoded
            .portable_profile()
            .get("protected-override-acknowledgements")
            .is_none()
    );
    assert!(decoded.portable_profile().get("source-path").is_none());
}
