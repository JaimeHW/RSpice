//! A shortcut profile projected to its portable artifact, in the case that
//! binds the product's own commands.

use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::ShortcutBindingSlot;
use crate::state::shortcuts::ShortcutPreferences;
use crate::state::shortcuts::ShortcutSequence;
use crate::state::shortcuts::ShortcutStroke;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::commands::vocabulary::CommandRegistry;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::shortcuts::artifacts::projection::*;
use crate::workbench::shortcuts::artifacts::schema::ShortcutArtifactScope as ShortcutExportScope;
use egui::os::OperatingSystem;
use serde_json::Value;

#[test]
fn user_override_json_is_deterministic_private_and_platform_sliced() {
    let mut profile = ShortcutPreferences::default();
    profile
        .set_binding(
            &CommandRegistry,
            Command::ToggleNavigator.key(),
            ShortcutBindingSlot::Primary,
            CommandPlatform::ALL.to_vec(),
            Some(ShortcutSequence::single(ShortcutStroke::new(
                egui::Key::F6,
                false,
                false,
                false,
            ))),
        )
        .unwrap();
    profile.acknowledge_protected_override(ids::SAVE);
    let request = ShortcutExportRequest {
        scope: ShortcutExportScope::UserOverrides,
        include_platform_mappings: false,
        runtime_platform: CommandPlatform::Browser,
        operating_system: OperatingSystem::Windows,
        current_contexts: Vec::new(),
    };
    let model = build_shortcut_reference_model(&profile, &request, command_registry()).unwrap();
    let first = serialize_shortcut_reference_json(&model).unwrap();
    let second = serialize_shortcut_reference_json(&model).unwrap();
    assert_eq!(first, second);
    assert!(!first.contains("protected-override-acknowledgements"));
    assert_eq!(
        model.manifest().coverage.platforms,
        [CommandPlatform::Browser]
    );
    let value: Value = serde_json::from_str(&first).unwrap();
    let platforms = &value["profile"]["commands"]["toggle-navigator"]["bindings"][0]["platforms"];
    assert_eq!(platforms, &serde_json::json!(["browser"]));
}
