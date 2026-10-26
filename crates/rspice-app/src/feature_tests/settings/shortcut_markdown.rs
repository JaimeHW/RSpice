//! The Markdown keyboard reference over the product's own commands.

use egui::os::OperatingSystem;

use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::ShortcutPreferences;
use crate::workbench::shortcuts::artifacts::markdown::*;
use crate::workbench::shortcuts::artifacts::projection::{
    ShortcutExportRequest, ShortcutExportScope, build_shortcut_reference_model,
};

#[test]
fn markdown_is_deterministic_and_uses_the_shared_reference_rows() {
    let model = build_shortcut_reference_model(
        &ShortcutPreferences::default(),
        &ShortcutExportRequest {
            scope: ShortcutExportScope::CompleteResolved,
            include_platform_mappings: false,
            runtime_platform: CommandPlatform::Desktop,
            operating_system: OperatingSystem::Windows,
            current_contexts: Vec::new(),
        },
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap();
    let first = serialize_shortcut_reference_markdown(&model);
    let second = serialize_shortcut_reference_markdown(&model);
    assert_eq!(first, second);
    assert!(first.starts_with("# RSpice keyboard shortcuts\n"));
    assert!(first.contains("| Save | `save-project` | Ctrl+S |"));
    assert!(!first.contains("protected-override-acknowledgements"));
}

#[test]
fn markdown_escaping_is_total_for_tables_and_line_breaks() {
    assert_eq!(escape_markdown("a|b\\c\nd"), r"a\|b\\c<br>d");
}
