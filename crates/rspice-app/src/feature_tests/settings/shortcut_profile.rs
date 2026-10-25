//! The shortcut profile and library held to the product's command registry:
//! the shipped bindings, and the cases that edit or audit a profile through
//! the commands this build ships.

use crate::state::ComponentType;
use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::ProtectedShortcutPolicy;
use crate::state::shortcuts::ShortcutBindingSlot;
use crate::state::shortcuts::ShortcutPreferences;
use crate::state::shortcuts::ShortcutProfileIssueCode;
use crate::state::shortcuts::ShortcutProfileIssueSeverity;
use crate::state::shortcuts::ShortcutSequence;
use crate::state::shortcuts::ShortcutStroke;
use crate::state::shortcuts::SingleKeyCanvasPolicy;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::commands::vocabulary::CommandRegistry;
use crate::workbench::state::Workspace;
use egui::Key;
use egui::os::OperatingSystem;

fn sequence(key: Key, primary: bool) -> ShortcutSequence {
    ShortcutSequence::single(ShortcutStroke::new(key, primary, false, false))
}

#[test]
fn immutable_workspace_ownership_projects_to_results_and_verify() {
    let preferences = ShortcutPreferences::default();
    assert_eq!(
        preferences.resolved_bindings(
            &CommandRegistry,
            Command::OpenWorkspace(Workspace::Results).key()
        )[0]
        .display_label(),
        "Alt+4"
    );
    assert_eq!(
        preferences.resolved_bindings(
            &CommandRegistry,
            Command::OpenWorkspace(Workspace::Verify).key()
        )[0]
        .display_label(),
        "Alt+5"
    );
    assert!(
        preferences
            .resolved_bindings(
                &CommandRegistry,
                Command::ResultViewer(crate::workbench::ResultViewer::Waves).key()
            )
            .is_empty()
    );
    let audit = preferences.audit(&CommandRegistry);
    assert!(audit.is_valid(), "default shortcut profile: {audit:#?}");
}

#[test]
fn default_profile_is_valid_with_complete_symbol_tool_registry() {
    let preferences = ShortcutPreferences::default();
    let audit = preferences.audit(&CommandRegistry);
    assert!(audit.is_valid(), "default shortcut profile: {audit:#?}");

    for command in [
        Command::SymbolPinTool,
        Command::SymbolPolylineTool,
        Command::SymbolCircleTool,
        Command::SymbolArcTool,
        Command::ZoomFit,
        Command::Cancel,
    ] {
        assert!(
            !preferences
                .effective_bindings(&CommandRegistry, command.key())
                .is_empty(),
            "missing effective symbol-editor binding: {command:?}"
        );
    }
    assert!(
        preferences
            .effective_bindings(&CommandRegistry, Command::SelectTool.key())
            .is_empty(),
        "Select tool intentionally has no factory key; Escape remains the universal return path"
    );
    assert_eq!(
        preferences.resolved_bindings(&CommandRegistry, Command::StretchSelection.key())[0]
            .display_label(),
        "S"
    );
    assert!(
        preferences
            .resolved_bindings(&CommandRegistry, Command::ArraySelection.key())
            .is_empty(),
        "Create array intentionally has no factory shortcut"
    );
}

#[test]
fn default_profile_keeps_place_pin_on_the_mockup_binding() {
    let preferences = ShortcutPreferences::default();
    let audit = preferences.audit(&CommandRegistry);
    assert!(audit.is_valid(), "default shortcut profile: {audit:#?}");

    for platform in CommandPlatform::ALL {
        let labels = preferences
            .resolved_bindings(&CommandRegistry, Command::PlacePin.key())
            .into_iter()
            .filter(|binding| binding.supports(platform))
            .map(|binding| binding.display_label())
            .collect::<Vec<_>>();
        assert_eq!(labels, ["Shift+P"]);
    }
}

#[test]
fn default_profile_keeps_place_text_on_the_mockup_binding() {
    let preferences = ShortcutPreferences::default();
    let audit = preferences.audit(&CommandRegistry);
    assert!(audit.is_valid(), "default shortcut profile: {audit:#?}");

    for platform in CommandPlatform::ALL {
        let labels = preferences
            .resolved_bindings(&CommandRegistry, Command::PlaceText.key())
            .into_iter()
            .filter(|binding| binding.supports(platform))
            .map(|binding| binding.display_label())
            .collect::<Vec<_>>();
        assert_eq!(labels, ["Shift+T"]);
    }
}

#[test]
fn editing_one_slot_preserves_the_other_then_resets_to_defaults() {
    let mut preferences = ShortcutPreferences::default();
    let command = Command::Save.key();
    let default_alternate = preferences
        .resolved_bindings(&CommandRegistry, command)
        .into_iter()
        .find(|binding| binding.slot() == ShortcutBindingSlot::Alternate)
        .unwrap()
        .display_label();

    preferences
        .set_binding(
            &CommandRegistry,
            command,
            ShortcutBindingSlot::Primary,
            CommandPlatform::ALL.to_vec(),
            Some(sequence(Key::F6, false)),
        )
        .unwrap();
    let resolved = preferences.resolved_bindings(&CommandRegistry, command);
    assert_eq!(
        resolved
            .iter()
            .find(|binding| binding.slot() == ShortcutBindingSlot::Primary)
            .unwrap()
            .display_label(),
        "F6"
    );
    assert_eq!(
        resolved
            .iter()
            .find(|binding| binding.slot() == ShortcutBindingSlot::Alternate)
            .unwrap()
            .display_label(),
        default_alternate
    );

    preferences.reset_command(command);
    assert!(preferences.command_override(command).is_none());
}

#[test]
fn malformed_keys_are_isolated_and_audited_without_hiding_defaults() {
    let preferences: ShortcutPreferences = serde_json::from_str(
        r#"{"commands":{"save-project":{"bindings":[{"slot":"primary","platforms":["desktop"],"sequence":[{"key":"DefinitelyNotAKey"}]}]}}}"#,
    )
    .unwrap();
    assert!(
        !preferences
            .resolved_bindings(&CommandRegistry, Command::Save.key())
            .is_empty()
    );
    assert!(
        preferences
            .audit(&CommandRegistry)
            .issues()
            .iter()
            .any(|issue| {
                issue.code() == ShortcutProfileIssueCode::MalformedKey
                    && issue
                        .command_id()
                        .is_some_and(|id| id.as_str() == "save-project")
            })
    );
    let encoded = serde_json::to_string(&preferences).unwrap();
    assert!(encoded.contains("DefinitelyNotAKey"));
}

#[test]
fn audit_finds_context_collisions_and_missing_browser_alternates() {
    let mut preferences = ShortcutPreferences::default();
    preferences
        .set_binding(
            &CommandRegistry,
            Command::Save.key(),
            ShortcutBindingSlot::Primary,
            CommandPlatform::ALL.to_vec(),
            Some(sequence(Key::W, false)),
        )
        .unwrap();
    preferences
        .set_binding(
            &CommandRegistry,
            Command::Save.key(),
            ShortcutBindingSlot::Alternate,
            CommandPlatform::ALL.to_vec(),
            None,
        )
        .unwrap();
    let audit = preferences.audit(&CommandRegistry);
    assert!(
        audit
            .issues()
            .iter()
            .any(|issue| { issue.code() == ShortcutProfileIssueCode::ExactCollision })
    );
    assert!(
        audit
            .issues()
            .iter()
            .any(|issue| { issue.code() == ShortcutProfileIssueCode::MissingBrowserAlternate })
    );
}

#[test]
fn single_key_canvas_policy_is_part_of_the_effective_projection() {
    let mut preferences = ShortcutPreferences::default();
    assert_eq!(
        preferences.resolved_label(
            &CommandRegistry,
            Command::PlaceWire.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows,
        ),
        "W"
    );

    preferences
        .policies_mut()
        .set_single_key_canvas(SingleKeyCanvasPolicy::RequireAlt);
    assert_eq!(
        preferences.resolved_label(
            &CommandRegistry,
            Command::PlaceWire.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows,
        ),
        "Alt+W"
    );

    preferences
        .policies_mut()
        .set_single_key_canvas(SingleKeyCanvasPolicy::Disabled);
    assert!(
        preferences
            .effective_bindings(&CommandRegistry, Command::PlaceWire.key())
            .is_empty()
    );
}

#[test]
fn context_precedence_warns_across_tiers_but_blocks_same_tier_collisions() {
    let mut across_tiers = ShortcutPreferences::default();
    across_tiers
        .set_binding(
            &CommandRegistry,
            Command::Save.key(),
            ShortcutBindingSlot::Primary,
            CommandPlatform::ALL.to_vec(),
            Some(sequence(Key::W, false)),
        )
        .unwrap();
    let audit = across_tiers.audit(&CommandRegistry);
    assert!(
        audit.is_valid(),
        "precedence-resolved collision: {audit:#?}"
    );
    assert!(audit.issues().iter().any(|issue| {
        issue.code() == ShortcutProfileIssueCode::ExactCollision
            && issue.severity() == ShortcutProfileIssueSeverity::Warning
    }));

    let mut same_tier = ShortcutPreferences::default();
    same_tier
        .set_binding(
            &CommandRegistry,
            Command::Place(ComponentType::Ground).key(),
            ShortcutBindingSlot::Primary,
            CommandPlatform::ALL.to_vec(),
            Some(sequence(Key::W, false)),
        )
        .unwrap();
    let audit = same_tier.audit(&CommandRegistry);
    assert!(!audit.is_valid());
    assert!(audit.issues().iter().any(|issue| {
        issue.code() == ShortcutProfileIssueCode::ExactCollision
            && issue.severity() == ShortcutProfileIssueSeverity::Error
    }));
}

#[test]
fn protected_override_acknowledgement_is_persisted_and_invalidated_by_edits() {
    let mut preferences = ShortcutPreferences::default();
    preferences
        .policies_mut()
        .set_protected_shortcuts(ProtectedShortcutPolicy::AllowWithConfirmation);
    preferences
        .set_binding(
            &CommandRegistry,
            Command::Save.key(),
            ShortcutBindingSlot::Alternate,
            vec![
                CommandPlatform::Browser,
                CommandPlatform::Tablet,
                CommandPlatform::Phone,
            ],
            None,
        )
        .unwrap();
    assert!(!preferences.audit(&CommandRegistry).is_valid());

    preferences.acknowledge_protected_override(Command::Save.key());
    assert!(preferences.audit(&CommandRegistry).is_valid());
    let mut round_trip: ShortcutPreferences =
        serde_json::from_value(serde_json::to_value(&preferences).unwrap()).unwrap();
    assert!(round_trip.protected_override_acknowledged(Command::Save.key()));

    round_trip
        .set_binding(
            &CommandRegistry,
            Command::Save.key(),
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            Some(sequence(Key::F6, false)),
        )
        .unwrap();
    assert!(!round_trip.protected_override_acknowledged(Command::Save.key()));
    assert!(!round_trip.audit(&CommandRegistry).is_valid());
}

#[test]
fn unknown_execution_policy_is_preserved_but_fails_closed_until_repaired() {
    let source = r#"{
        "policies":{"single-key-canvas":"future-policy"},
        "commands":{
            "toggle-navigator":{"bindings":[{
                "slot":"primary",
                "platforms":["desktop","browser","tablet","phone"],
                "sequence":[{"key":"F6"}]
            }]},
            "future-command":{"bindings":[],"future-data":true}
        },
        "future-domain":{"version":2}
    }"#;
    let mut preferences: ShortcutPreferences = serde_json::from_str(source).unwrap();
    assert_eq!(
        preferences.resolved_bindings(&CommandRegistry, Command::ToggleNavigator.key())[0]
            .display_label(),
        "F6"
    );
    let audit = preferences.audit(&CommandRegistry);
    assert!(!audit.is_valid());
    assert!(
        audit
            .issues()
            .iter()
            .any(|issue| issue.code() == ShortcutProfileIssueCode::UnknownPolicy)
    );
    assert_eq!(
        preferences.resolved_label(
            &CommandRegistry,
            Command::ToggleNavigator.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows,
        ),
        "Ctrl+B"
    );
    let encoded = serde_json::to_value(&preferences).unwrap();
    assert_eq!(encoded["policies"]["single-key-canvas"], "future-policy");

    let receipt = preferences.remove_blocking_invalid_entries(&CommandRegistry);
    assert_eq!(receipt, ["policy:single-key-canvas"]);
    assert!(preferences.audit(&CommandRegistry).is_valid());
    assert_eq!(
        preferences.resolved_label(
            &CommandRegistry,
            Command::ToggleNavigator.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows,
        ),
        "F6"
    );
    let repaired = serde_json::to_value(preferences).unwrap();
    assert_eq!(repaired["commands"]["future-command"]["future-data"], true);
    assert_eq!(repaired["future-domain"]["version"], 2);
}
