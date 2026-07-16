use egui::Key;

use super::ShortcutInputSnapshot;
use crate::workbench::commands::{COMMAND_REGISTRY, Command, CommandPlatform, ShortcutBinding};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedShortcut {
    pub(crate) command: Command,
    pub(crate) key: Key,
}

fn binding_matches(binding: ShortcutBinding, snapshot: &ShortcutInputSnapshot) -> bool {
    snapshot.key_pressed(binding.chord.key)
        && snapshot.primary() == binding.chord.primary
        && snapshot.shift() == binding.chord.shift
        && snapshot.alt() == binding.chord.alt
}

/// Resolve the typed command registry for one platform. Context selection is
/// applied by the caller after it has access to the active workbench state;
/// this intentionally permits the same chord in disjoint typed contexts.
pub(crate) fn resolve_shortcuts(
    snapshot: &ShortcutInputSnapshot,
    platform: CommandPlatform,
) -> Vec<ResolvedShortcut> {
    let mut resolved = Vec::with_capacity(4);
    for command in COMMAND_REGISTRY.iter().copied() {
        let context = command.shortcut_context();
        if snapshot.has_focus() && context.suppressed_by_text_focus() {
            continue;
        }
        if let Some(binding) = command
            .shortcut_bindings()
            .iter()
            .copied()
            .find(|binding| binding.supports(platform) && binding_matches(*binding, snapshot))
        {
            resolved.push(ResolvedShortcut {
                command,
                key: binding.chord.key,
            });
        }
    }
    resolved
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use egui::Modifiers;

    use super::*;
    use crate::workbench::commands::{ShortcutContext, ShortcutKind};

    fn modifiers(primary: bool, alt: bool, shift: bool) -> Modifiers {
        Modifiers {
            command: primary,
            ctrl: primary && !cfg!(target_os = "macos"),
            mac_cmd: primary && cfg!(target_os = "macos"),
            alt,
            shift,
        }
    }

    fn resolve(
        key: Key,
        primary: bool,
        alt: bool,
        shift: bool,
        has_focus: bool,
        platform: CommandPlatform,
    ) -> Vec<Command> {
        let snapshot =
            ShortcutInputSnapshot::for_test(key, modifiers(primary, alt, shift), has_focus);
        resolve_shortcuts(&snapshot, platform)
            .into_iter()
            .map(|resolved| resolved.command)
            .collect()
    }

    #[test]
    fn every_platform_context_chord_has_one_owner() {
        let mut owners = HashMap::new();
        for command in COMMAND_REGISTRY.iter().copied() {
            for binding in command.shortcut_bindings() {
                for platform in binding.platforms {
                    let chord = binding.chord;
                    let key = (
                        command.shortcut_context(),
                        *platform,
                        chord.key,
                        chord.primary,
                        chord.alt,
                        chord.shift,
                    );
                    let previous = owners.insert(key, command);
                    assert!(
                        previous.is_none() || previous == Some(command),
                        "shortcut collision between {previous:?} and {command:?} at {key:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn protected_mockup_shortcuts_have_exact_browser_and_touch_alternates() {
        let protected = [
            (Command::RunSimulation, "F5", "Ctrl+Enter"),
            (Command::StopSimulation, "Shift+F5", "Ctrl+Shift+Enter"),
            (Command::OpenProject, "Ctrl+O", "Ctrl+Alt+O"),
            (Command::NewProject, "Ctrl+Shift+N", "Ctrl+Alt+Shift+N"),
            (Command::Save, "Ctrl+S", "Ctrl+Alt+S"),
            (
                Command::CloseActiveDocument,
                "Ctrl+W",
                "Ctrl+Shift+Backspace",
            ),
            (Command::ToggleFullScreen, "F11", "Ctrl+Alt+F"),
            (Command::GenerateNetlist, "Ctrl+L", "Ctrl+Alt+L"),
            (Command::ToggleConsole, "Ctrl+J", "Ctrl+Alt+J"),
        ];
        for (command, desktop, host) in protected {
            assert_eq!(command.shortcut_label(CommandPlatform::Desktop), desktop);
            for platform in [
                CommandPlatform::Browser,
                CommandPlatform::Tablet,
                CommandPlatform::Phone,
            ] {
                assert_eq!(command.shortcut_label(platform), host);
                assert!(command.shortcut_bindings().iter().any(|binding| {
                    binding.kind == ShortcutKind::Alternate && binding.supports(platform)
                }));
            }
            assert!(command.primary_is_reserved_on(CommandPlatform::Browser));
            assert!(!command.primary_is_reserved_on(CommandPlatform::Desktop));
            assert!(!command.primary_is_reserved_on(CommandPlatform::Tablet));
            assert!(!command.primary_is_reserved_on(CommandPlatform::Phone));
        }
    }

    #[test]
    fn lifecycle_commands_remain_global_with_text_focus() {
        assert_eq!(
            resolve(Key::S, true, false, true, true, CommandPlatform::Desktop),
            vec![Command::SaveAll]
        );
        assert!(resolve(Key::C, true, false, false, true, CommandPlatform::Desktop).is_empty());
    }

    #[test]
    fn protected_semantics_do_not_regress_to_legacy_shell_actions() {
        assert_eq!(
            resolve(Key::Q, false, false, false, false, CommandPlatform::Desktop),
            vec![Command::ObjectProperties]
        );
        assert_eq!(
            resolve(Key::G, false, false, false, false, CommandPlatform::Desktop),
            vec![Command::CycleGrid]
        );
        assert_eq!(
            resolve(Key::B, true, false, false, false, CommandPlatform::Desktop),
            vec![Command::ToggleNavigator]
        );
        assert_eq!(
            resolve(Key::L, true, false, false, false, CommandPlatform::Desktop),
            vec![Command::GenerateNetlist]
        );
        assert!(
            resolve(
                Key::Tab,
                true,
                false,
                false,
                false,
                CommandPlatform::Desktop
            )
            .is_empty()
        );
        assert!(
            resolve(
                Key::F1,
                false,
                false,
                false,
                false,
                CommandPlatform::Desktop
            )
            .is_empty()
        );
    }

    #[test]
    fn ctrl_e_is_shared_only_across_disjoint_typed_contexts() {
        let commands = resolve(Key::E, true, false, false, false, CommandPlatform::Desktop);
        assert_eq!(commands.len(), 2);
        assert!(commands.contains(&Command::RunChecks));
        assert!(commands.contains(&Command::PreflightChecks));
        assert_eq!(
            Command::RunChecks.shortcut_context(),
            ShortcutContext::DesignWorkspace
        );
        assert_eq!(
            Command::PreflightChecks.shortcut_context(),
            ShortcutContext::SimulationWorkspace
        );
    }

    #[test]
    fn former_workbench_shell_shortcuts_are_owned_by_the_central_resolver() {
        assert_eq!(
            resolve(
                Key::F11,
                false,
                false,
                false,
                false,
                CommandPlatform::Desktop
            ),
            vec![Command::ToggleFullScreen]
        );

        for (key, workspace) in [
            (Key::Num1, crate::workbench::state::Workspace::Project),
            (Key::Num2, crate::workbench::state::Workspace::Design),
            (Key::Num3, crate::workbench::state::Workspace::Simulate),
            (Key::Num6, crate::workbench::state::Workspace::Models),
            (Key::Num7, crate::workbench::state::Workspace::Netlist),
        ] {
            assert_eq!(
                resolve(key, false, true, false, false, CommandPlatform::Desktop),
                vec![Command::OpenWorkspace(workspace)]
            );
        }
        assert_eq!(
            resolve(
                Key::Num4,
                false,
                true,
                false,
                false,
                CommandPlatform::Desktop
            ),
            vec![Command::ResultViewer(crate::workbench::ResultViewer::Waves)]
        );
        assert_eq!(
            resolve(
                Key::Num5,
                false,
                true,
                false,
                false,
                CommandPlatform::Desktop
            ),
            vec![Command::VerificationPage(
                crate::workbench::state::VerificationPage::Yield
            )]
        );

        assert_eq!(
            resolve(
                Key::Escape,
                false,
                false,
                false,
                false,
                CommandPlatform::Desktop
            ),
            vec![Command::Cancel]
        );
        assert_eq!(
            Command::Cancel.shortcut_context(),
            ShortcutContext::ApplicationChrome
        );
    }
}
