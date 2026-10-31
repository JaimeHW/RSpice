//! VS Code keybindings adapted to the product's own commands.

use crate::feature_tests::settings::shortcut_vscode_import::{
    assert_every_entry_accounted_for, has_code, vscode,
};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::shortcuts::artifacts::sha256;
use crate::workbench::shortcuts::artifacts::vscode::*;
use egui::Key;

#[test]
fn jsonc_chord_adaptation_is_deterministic_and_round_trips_as_native() {
    let source = r#"[
        // VS Code permits comments.
        { "key": "ctrl+k ctrl+c", "command": "workbench.action.closeActiveEditor", },
    ]"#;
    let first = vscode(source, VscodeHostPlatform::Windows);
    let second = vscode(source, VscodeHostPlatform::Windows);

    assert_eq!(first.source_digest(), sha256(source.as_bytes()));
    assert_eq!(first.artifact().source_digest(), sha256(source.as_bytes()));
    assert_eq!(first.source_bytes(), source.as_bytes());
    assert_eq!(
        first.canonical_artifact_json(),
        second.canonical_artifact_json()
    );
    assert_eq!(first.report(), second.report());
    assert!(first.report().is_importable());
    let command_override = first
        .artifact()
        .profile()
        .command_override(Command::CloseActiveDocument.key())
        .unwrap();
    let strokes = command_override.bindings()[0].sequence().strokes();
    assert_eq!(strokes.len(), 2);
    assert_eq!(strokes[0].key(), Key::K);
    assert_eq!(strokes[1].key(), Key::C);
    assert!(strokes.iter().all(|stroke| stroke.primary()));
    assert_eq!(first.artifact().source_name(), "keybindings.json");

    let detected = detect_shortcut_artifact(
        "adapted.json",
        first.canonical_artifact_json().as_bytes(),
        VscodeHostPlatform::Windows,
        command_registry(),
    )
    .unwrap();
    let DetectedShortcutArtifact::RSpice(round_trip) = detected else {
        panic!("canonical output must detect as a native artifact");
    };
    assert_eq!(
        round_trip.artifact().portable_profile(),
        first.artifact().portable_profile()
    );
}

#[test]
fn platform_override_and_platform_primary_semantics_are_exact() {
    let source = r#"[{
        "key":"ctrl+s",
        "win":"ctrl+shift+s",
        "mac":"cmd+alt+s",
        "linux":"ctrl+s",
        "command":"workbench.action.files.save"
    }]"#;
    let windows = vscode(source, VscodeHostPlatform::Windows);
    let windows_binding = windows
        .artifact()
        .profile()
        .command_override(ids::SAVE)
        .unwrap();
    let windows_stroke = windows_binding.bindings()[0].sequence().strokes()[0];
    assert!(windows_stroke.primary());
    assert!(windows_stroke.shift());
    assert!(!windows_stroke.alt());
    assert!(has_code(
        windows.report(),
        VscodeDiagnosticCode::PlatformVariantNotImported
    ));

    let mac = vscode(source, VscodeHostPlatform::Macos);
    let mac_binding = mac
        .artifact()
        .profile()
        .command_override(ids::SAVE)
        .unwrap();
    let mac_stroke = mac_binding.bindings()[0].sequence().strokes()[0];
    assert!(mac_stroke.primary());
    assert!(mac_stroke.alt());
    assert!(!mac_stroke.shift());
}

#[test]
fn duplicate_is_deduplicated_but_conflict_fails_closed() {
    let duplicate = vscode(
        r#"[
            {"key":"ctrl+s","command":"workbench.action.files.save"},
            {"key":"ctrl+s","command":"workbench.action.files.save"}
        ]"#,
        VscodeHostPlatform::Windows,
    );
    assert_eq!(duplicate.report().imported_entries, 1);
    assert_eq!(duplicate.report().duplicate_entries, 1);
    assert!(duplicate.report().is_importable());
    assert_every_entry_accounted_for(duplicate.report());
    assert!(has_code(
        duplicate.report(),
        VscodeDiagnosticCode::DuplicateBinding
    ));

    let conflict = vscode(
        r#"[
            {"key":"ctrl+s","command":"workbench.action.files.save"},
            {"key":"ctrl+shift+s","command":"workbench.action.files.save"}
        ]"#,
        VscodeHostPlatform::Windows,
    );
    assert_eq!(conflict.report().imported_entries, 0);
    assert_eq!(conflict.report().conflicting_entries, 2);
    assert!(conflict.report().has_errors());
    assert!(has_code(
        conflict.report(),
        VscodeDiagnosticCode::ConflictingBinding
    ));
    assert!(
        conflict
            .artifact()
            .profile()
            .command_override(ids::SAVE)
            .is_none()
    );
}

#[test]
fn unmatched_removal_fails_closed_without_creating_an_unbind() {
    let adaptation = vscode(
        r#"[{"key":"ctrl+q","command":"-workbench.action.files.save"}]"#,
        VscodeHostPlatform::Windows,
    );
    assert_eq!(adaptation.report().imported_entries, 0);
    assert_eq!(adaptation.report().rejected_entries, 1);
    assert!(!adaptation.report().is_importable());
    assert!(has_code(
        adaptation.report(),
        VscodeDiagnosticCode::UnsupportedRemoval
    ));
    assert!(
        adaptation
            .artifact()
            .profile()
            .command_override(ids::SAVE)
            .is_none()
    );
    assert_every_entry_accounted_for(adaptation.report());
}

#[test]
fn remove_then_replace_is_reviewable_but_cannot_be_partially_applied() {
    let adaptation = vscode(
        r#"[
            {"key":"ctrl+s","command":"-workbench.action.files.save"},
            {"key":"ctrl+shift+s","command":"workbench.action.files.save"}
        ]"#,
        VscodeHostPlatform::Windows,
    );
    assert!(adaptation.report().has_errors());
    assert!(!adaptation.report().is_importable());
    assert_eq!(adaptation.report().rejected_entries, 1);
    assert_eq!(adaptation.report().imported_entries, 1);
    let command_override = adaptation
        .artifact()
        .profile()
        .command_override(ids::SAVE)
        .unwrap();
    assert!(command_override.bindings()[0].sequence().strokes()[0].shift());
    assert_every_entry_accounted_for(adaptation.report());
}
