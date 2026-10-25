//! The shortcut reference PDF, in the case that binds the product's own
//! commands.

use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::ShortcutPreferences;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::shortcuts::artifacts::pdf::*;
use crate::workbench::shortcuts::artifacts::projection::ShortcutExportRequest;
use crate::workbench::shortcuts::artifacts::projection::ShortcutExportScope;
use crate::workbench::shortcuts::artifacts::projection::build_shortcut_reference_model;
use egui::os::OperatingSystem;
use lopdf::Document as ParsedPdf;

#[test]
fn pdf_excludes_private_profile_state_and_filesystem_paths() {
    let mut profile = ShortcutPreferences::default();
    profile.acknowledge_protected_override(Command::Save.key());
    let model = build_shortcut_reference_model(
        &profile,
        &ShortcutExportRequest {
            scope: ShortcutExportScope::UserOverrides,
            include_platform_mappings: true,
            runtime_platform: CommandPlatform::Desktop,
            operating_system: OperatingSystem::Windows,
            current_contexts: Vec::new(),
        },
    )
    .unwrap();
    let pdf = serialize_shortcut_reference_pdf(&model).unwrap();
    let parsed = ParsedPdf::load_mem(&pdf).unwrap();
    let page_numbers = parsed.get_pages().keys().copied().collect::<Vec<_>>();
    let text = parsed.extract_text(&page_numbers).unwrap();
    let raw = String::from_utf8_lossy(&pdf);
    for private in [
        "protected-override-acknowledgements",
        "source-path",
        "project-identity",
        "recent-commands",
        "C:\\\\",
        "/Users/",
        "/home/",
    ] {
        assert!(!text.contains(private));
        assert!(!raw.contains(private));
    }
}
