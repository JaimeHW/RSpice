use egui::{InputState, Key, Modifiers};

use crate::workbench::ShortcutPreferences;
use crate::workbench::commands::{Command, CommandPlatform};

mod canvas_focus;
mod category;
mod input_snapshot;
mod platform;
mod resolver;

pub(super) use canvas_focus::engineering_canvas_has_focus;
pub(crate) use canvas_focus::report_engineering_canvas_focus;
pub(super) use category::ShortcutCategory;
pub(super) use input_snapshot::ShortcutInputSnapshot;
pub(crate) use platform::runtime_command_platform;
pub(super) use resolver::ShortcutEnvironment;
pub(crate) use resolver::ShortcutResolverState;

/// Accessibility projection of the same effective bindings consumed by the
/// runtime resolver. Empty/unsupported bindings are omitted rather than
/// announcing a key that cannot execute on the current host.
pub(crate) fn accessibility_shortcut_summary(
    shortcuts: &ShortcutPreferences,
    platform: CommandPlatform,
    operating_system: egui::os::OperatingSystem,
    commands: &[Command],
) -> String {
    let entries = commands
        .iter()
        .filter_map(|command| {
            let labels = shortcuts.resolved_labels(*command, platform, operating_system);
            (!labels.is_empty()).then(|| {
                format!(
                    "{}: {}",
                    labels.join(" or "),
                    command.spec().label.trim_end_matches('\u{2026}')
                )
            })
        })
        .collect::<Vec<_>>();
    if entries.is_empty() {
        String::new()
    } else {
        format!(" Shortcuts: {}.", entries.join("; "))
    }
}

#[cfg(test)]
mod accessibility_tests {
    use super::*;
    use crate::workbench::{ShortcutBindingSlot, ShortcutSequence, ShortcutStroke};

    #[test]
    fn accessibility_projection_uses_effective_custom_profile_and_host() {
        let mut shortcuts = ShortcutPreferences::default();
        shortcuts
            .set_binding(
                Command::SymbolCircleTool,
                ShortcutBindingSlot::Primary,
                vec![CommandPlatform::Desktop],
                Some(ShortcutSequence::single(ShortcutStroke::new(
                    Key::F9,
                    false,
                    true,
                    false,
                ))),
            )
            .unwrap();

        let summary = accessibility_shortcut_summary(
            &shortcuts,
            CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
            &[Command::SymbolCircleTool],
        );

        assert_eq!(summary, " Shortcuts: Alt+F9: Draw symbol circle.");
    }
}
