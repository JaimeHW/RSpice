//! Importing a legacy shortcut profile, in the case that binds the product's
//! own commands.

use crate::workbench::shortcuts::profile_workflow::*;

#[test]
fn legacy_import_never_trusts_transferred_protected_acknowledgements() {
    let staged = stage_shortcut_profile_json(
        r#"{
            "format":"rspice.shortcuts/1",
            "profile":{
                "protected-override-acknowledgements":["save-project"],
                "commands":{}
            }
        }"#,
    )
    .unwrap();

    assert!(
        !staged.candidate().protected_override_acknowledged(
            crate::workbench::framework::command_registry::ids::SAVE
        )
    );
    // The scrub happens before decoding, so the transferred entry cannot
    // reappear through the candidate's own serialization either.
    let reencoded = serde_json::to_value(staged.candidate()).unwrap();
    assert_eq!(
        reencoded["protected-override-acknowledgements"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );
}
