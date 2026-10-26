//! Staging a shortcut profile import against the product's own commands.

use crate::state::shortcuts::MAX_SHORTCUT_SEQUENCE_STROKES;
use crate::workbench::shortcuts::profile_workflow::*;

const VALID_SOURCE: &str = r#"{
    "format": "rspice.shortcuts/1",
    "profile": {
        "commands": {
            "future-command": {
                "bindings": [{
                    "slot": "primary",
                    "platforms": ["desktop"],
                    "sequence": [{"key": "F9"}],
                    "future-binding-field": {"preserve": true}
                }],
                "future-command-field": 73
            }
        },
        "future-profile-field": [1, 2, 3]
    }
}"#;

#[test]
fn stage_is_transactional_and_preserves_unknown_future_commands() {
    let staged = stage_shortcut_profile_json(VALID_SOURCE).unwrap();
    assert!(
        staged
            .candidate()
            .audit(&crate::workbench::commands::vocabulary::CommandRegistry)
            .issues()
            .iter()
            .any(|issue| {
                issue
                    .command_id()
                    .is_some_and(|id| id.as_str() == "future-command")
            })
    );

    let reparsed = serde_json::json!({ "profile": staged.candidate() });
    assert_eq!(
        reparsed["profile"]["commands"]["future-command"]["future-command-field"],
        73
    );
    assert_eq!(
        reparsed["profile"]["commands"]["future-command"]["bindings"][0]["future-binding-field"]["preserve"],
        true
    );
    assert_eq!(
        reparsed["profile"]["future-profile-field"],
        serde_json::json!([1, 2, 3])
    );
}

#[test]
fn syntax_envelope_and_sequence_errors_have_stable_paths() {
    let syntax = stage_shortcut_profile_json("{").unwrap_err();
    assert_eq!(syntax.json_path(), "$");
    assert_eq!(syntax.code(), "shortcut-profile.invalid-json");

    let format =
        stage_shortcut_profile_json(r#"{"format":"rspice.shortcuts/2","profile":{}}"#).unwrap_err();
    assert_eq!(format.json_path(), "$.format");

    let strokes = (0..=MAX_SHORTCUT_SEQUENCE_STROKES)
        .map(|_| serde_json::json!({"key": "A"}))
        .collect::<Vec<_>>();
    let source = serde_json::json!({
        "format": SHORTCUT_PROFILE_FORMAT,
        "profile": {"commands": {"save": {"bindings": [{
            "slot": "primary", "platforms": ["desktop"], "sequence": strokes
        }]}}}
    });
    let error = stage_shortcut_profile_json(&source.to_string()).unwrap_err();
    assert_eq!(
        error.json_path(),
        "$.profile.commands[\"save\"].bindings[0].sequence"
    );
    assert!(error.context().contains("maximum is 4"));
}
