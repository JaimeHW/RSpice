//! A profile projected for export over the product's own commands.

use egui::os::OperatingSystem;
use rspice_app_settings::workbench::shortcuts::artifacts::projection::*;
use rspice_app_state::state::command_catalog::{CommandPlatform, ShortcutContext};
use rspice_app_state::state::shortcuts::ShortcutPreferences;

#[test]
fn resolved_export_materializes_defaults_and_current_workspace_filters_contexts() {
    let profile = ShortcutPreferences::default();
    let complete = build_shortcut_reference_model(
        &profile,
        &ShortcutExportRequest {
            scope: ShortcutExportScope::CompleteResolved,
            include_platform_mappings: true,
            runtime_platform: CommandPlatform::Desktop,
            operating_system: OperatingSystem::Windows,
            current_contexts: Vec::new(),
        },
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    assert!(complete.profile()["commands"].get("save-project").is_some());

    let current = build_shortcut_reference_model(
        &profile,
        &ShortcutExportRequest {
            scope: ShortcutExportScope::CurrentWorkspace,
            include_platform_mappings: false,
            runtime_platform: CommandPlatform::Desktop,
            operating_system: OperatingSystem::Windows,
            current_contexts: vec![ShortcutContext::ResultsWorkspace],
        },
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    assert!(
        current.profile()["commands"]
            .get("toggle-linked-cursors")
            .is_some()
    );
    assert!(current.profile()["commands"].get("place-wire").is_none());
    assert!(current.profile()["commands"].get("save-project").is_some());
}
