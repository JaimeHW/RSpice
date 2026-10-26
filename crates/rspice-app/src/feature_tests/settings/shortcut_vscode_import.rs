//! Importing VS Code keybindings against the product's own commands.

use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::shortcuts::artifacts::sha256;
use crate::workbench::shortcuts::artifacts::vscode::*;
use std::collections::BTreeSet;

pub(crate) fn vscode(source: &str, platform: VscodeHostPlatform) -> VscodeShortcutAdaptation {
    adapt_vscode_keybindings(r"C:\private\keybindings.json", source.as_bytes(), platform).unwrap()
}

pub(crate) fn has_code(report: &VscodeImportReport, code: VscodeDiagnosticCode) -> bool {
    report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == code)
}

pub(crate) fn assert_every_entry_accounted_for(report: &VscodeImportReport) {
    assert_eq!(report.entry_outcomes.len(), report.total_entries);
    assert_eq!(
        report
            .entry_outcomes
            .iter()
            .map(|outcome| outcome.entry_index)
            .collect::<Vec<_>>(),
        (0..report.total_entries).collect::<Vec<_>>()
    );
    assert_eq!(
        report.imported_entries
            + report.duplicate_entries
            + report.unmapped_entries
            + report.rejected_entries
            + report.conflicting_entries,
        report.total_entries
    );
}

#[test]
fn mapping_table_is_unique_and_resolves_only_registry_commands() {
    let mut source_ids = BTreeSet::new();
    let mut target_ids = BTreeSet::new();
    for mapping in VSCODE_COMMAND_MAPPINGS {
        assert!(source_ids.insert(mapping.vscode_command_id));
        assert!(target_ids.insert(mapping.rspice_command.as_str()));
        assert!(command_registry().order().contains(&mapping.rspice_command));
        assert_eq!(
            command_registry().resolve(mapping.rspice_command.as_str()),
            Some(mapping.rspice_command)
        );
    }
    for vscode_ui_zoom in [
        "workbench.action.zoomIn",
        "workbench.action.zoomOut",
        "workbench.action.zoomReset",
    ] {
        assert!(mapping_for(vscode_ui_zoom).is_none());
    }
}

#[test]
fn physical_control_on_macos_fails_closed() {
    let adaptation = vscode(
        r#"[{"key":"ctrl+s","command":"workbench.action.files.save"}]"#,
        VscodeHostPlatform::Macos,
    );
    assert!(adaptation.report().has_errors());
    assert!(!adaptation.report().is_importable());
    assert!(has_code(
        adaptation.report(),
        VscodeDiagnosticCode::UnsupportedKey
    ));
}

#[test]
fn malformed_unknown_and_unsupported_constructs_are_fully_reported() {
    let adaptation = vscode(
        r#"[
            5,
            {"key":"ctrl+q","command":"extension.privateCommand"},
            {"key":"ctrl+s","command":"workbench.action.files.save","when":"editorFocus"},
            {"key":"ctrl+c","command":"editor.action.clipboardCopyAction","args":{}},
            {"key":8,"command":"undo"},
            {"key":"ctrl+z","command":"undo","futureField":true}
        ]"#,
        VscodeHostPlatform::Windows,
    );
    assert_eq!(adaptation.report().total_entries, 6);
    assert_eq!(adaptation.report().unmapped_entries, 1);
    assert_eq!(
        adaptation.report().unmapped_command_ids,
        ["extension.privateCommand"]
    );
    for code in [
        VscodeDiagnosticCode::MalformedEntry,
        VscodeDiagnosticCode::UnsupportedCommand,
        VscodeDiagnosticCode::UnsupportedWhen,
        VscodeDiagnosticCode::UnsupportedArguments,
        VscodeDiagnosticCode::UnsupportedField,
    ] {
        assert!(has_code(adaptation.report(), code), "missing {code:?}");
    }
    assert!(adaptation.report().has_errors());
    assert!(!adaptation.report().is_importable());
    assert_every_entry_accounted_for(adaptation.report());
}

#[test]
fn mixed_known_and_unmapped_entries_have_no_partial_import_authority() {
    let adaptation = vscode(
        r#"[
            {"key":"ctrl+s","command":"workbench.action.files.save"},
            {"key":"ctrl+q","command":"extension.unmapped"}
        ]"#,
        VscodeHostPlatform::Windows,
    );
    assert_eq!(adaptation.report().imported_entries, 1);
    assert_eq!(adaptation.report().unmapped_entries, 1);
    assert!(adaptation.report().has_errors());
    assert!(!adaptation.report().is_importable());
    assert_every_entry_accounted_for(adaptation.report());
}

#[test]
fn mixed_known_and_unknown_field_entries_have_no_partial_import_authority() {
    let adaptation = vscode(
        r#"[
            {"key":"ctrl+s","command":"workbench.action.files.save"},
            {"key":"ctrl+z","command":"undo","futureBehavior":true}
        ]"#,
        VscodeHostPlatform::Windows,
    );
    assert_eq!(adaptation.report().imported_entries, 1);
    assert_eq!(adaptation.report().rejected_entries, 1);
    assert!(adaptation.report().has_errors());
    assert!(!adaptation.report().is_importable());
    assert_every_entry_accounted_for(adaptation.report());
}

#[test]
fn native_detection_retains_exact_source_bytes_and_digest() {
    let source = br#"{
        // Native artifacts may arrive through a JSONC-capable editor.
        "format":"rspice.shortcuts/1",
        "artifact":{
            "schemaVersion":1,
            "scope":"user-overrides",
            "coverage":{"contexts":["all"],"platforms":["desktop"],"policiesIncluded":false},
            "platformMappingsIncluded":false,
        },
        "profile":{"commands":{}},
    }"#;
    let detected =
        detect_shortcut_artifact(r"C:\private\native.json", source, VscodeHostPlatform::Linux)
            .unwrap();
    let DetectedShortcutArtifact::RSpice(native) = detected else {
        panic!("expected native artifact");
    };
    assert_eq!(native.source_bytes(), source);
    assert_eq!(native.source_digest(), sha256(source));
    assert_eq!(native.artifact().source_digest(), sha256(source));
    assert_eq!(native.artifact().source_name(), "native.json");
}

#[test]
fn diagnostics_do_not_echo_source_values_or_paths() {
    let private_values = [
        "/private/secret/project",
        "private-predicate-value",
        "private-key-token",
        "private-field-name",
    ];
    let source = format!(
        r#"[{{
            "key":"ctrl+{}",
            "command":"workbench.action.files.save",
            "when":"{}",
            "args":{{"path":"{}"}},
            "{}":true
        }}]"#,
        private_values[2], private_values[1], private_values[0], private_values[3]
    );
    let adaptation = vscode(&source, VscodeHostPlatform::Windows);
    let messages = adaptation
        .report()
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for private_value in private_values {
        assert!(!messages.contains(private_value));
    }
}

#[test]
fn unsupported_root_and_unterminated_jsonc_have_stable_codes() {
    let root_error =
        detect_shortcut_artifact("bad.json", br#"{"commands":{}}"#, VscodeHostPlatform::Linux)
            .unwrap_err();
    assert_eq!(root_error.code(), "shortcut-adapter.unknown-object");

    let comment_error =
        detect_shortcut_artifact("bad.json", b"[/* private", VscodeHostPlatform::Linux)
            .unwrap_err();
    assert_eq!(
        comment_error.code(),
        "shortcut-adapter.unterminated-comment"
    );
}

#[test]
fn colliding_known_commands_are_rejected() {
    let adaptation = vscode(
        r#"[
            {"key":"ctrl+z","command":"undo"},
            {"key":"ctrl+z","command":"redo"}
        ]"#,
        VscodeHostPlatform::Windows,
    );
    assert!(has_code(
        adaptation.report(),
        VscodeDiagnosticCode::CrossCommandCollision
    ));
    assert!(!adaptation.report().is_importable());
}
