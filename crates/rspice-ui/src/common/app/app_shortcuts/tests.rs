use super::*;
use std::collections::HashSet;

fn mods(ctrl: bool, shift: bool) -> Modifiers {
    Modifiers {
        ctrl,
        shift,
        ..Default::default()
    }
}

fn commands(keys: &[Key], ctrl: bool, shift: bool, has_focus: bool) -> Vec<ShortcutCommand> {
    let snapshot = ShortcutInputSnapshot::from_keys(keys, mods(ctrl, shift), has_focus);
    collect_shortcut_commands(&snapshot)
}

#[test]
fn test_file_shortcuts_with_ctrl() {
    let cmds = commands(&[Key::N, Key::O, Key::S], true, false, false);
    assert!(cmds.contains(&ShortcutCommand::FileNew));
    assert!(cmds.contains(&ShortcutCommand::FileOpen));
    assert!(cmds.contains(&ShortcutCommand::FileSave));
}

#[test]
fn test_file_shortcuts_still_work_with_text_focus() {
    let cmds = commands(&[Key::N, Key::S], true, false, true);
    assert!(cmds.contains(&ShortcutCommand::FileNew));
    assert!(cmds.contains(&ShortcutCommand::FileSave));
}

#[test]
fn test_redo_shortcuts_both_variants() {
    let ctrl_y = commands(&[Key::Y], true, false, false);
    assert!(ctrl_y.contains(&ShortcutCommand::EditRedo));

    let ctrl_shift_z = commands(&[Key::Z], true, true, false);
    assert!(ctrl_shift_z.contains(&ShortcutCommand::EditRedo));
}

#[test]
fn test_redo_deduplicates_when_multiple_bindings_match() {
    let cmds = commands(&[Key::Y, Key::Z], true, true, false);
    let redo_count = cmds
        .iter()
        .filter(|command| **command == ShortcutCommand::EditRedo)
        .count();
    assert_eq!(redo_count, 1);
}

#[test]
fn test_undo_requires_ctrl_z_without_shift() {
    let ctrl_z = commands(&[Key::Z], true, false, false);
    assert!(ctrl_z.contains(&ShortcutCommand::EditUndo));

    let ctrl_shift_z = commands(&[Key::Z], true, true, false);
    assert!(!ctrl_shift_z.contains(&ShortcutCommand::EditUndo));
}

#[test]
fn test_ctrl_shortcuts_do_not_emit_tool_commands() {
    let cmds = commands(&[Key::C, Key::V, Key::L], true, false, false);
    assert!(cmds.contains(&ShortcutCommand::EditCopy));
    assert!(cmds.contains(&ShortcutCommand::EditPaste));
    assert!(!cmds.contains(&ShortcutCommand::ToggleBrowserPanel));
    assert!(!cmds.contains(&ShortcutCommand::PlaceCapacitor));
    assert!(!cmds.contains(&ShortcutCommand::PlaceVoltageSource));
    assert!(!cmds.contains(&ShortcutCommand::PlaceInductor));
}

#[test]
fn test_focus_blocks_tool_shortcuts() {
    let cmds = commands(&[Key::W, Key::R, Key::Escape, Key::E], false, false, true);
    assert!(!cmds.contains(&ShortcutCommand::ToolWire));
    assert!(!cmds.contains(&ShortcutCommand::RotateSelectionOrPreview));
    assert!(!cmds.contains(&ShortcutCommand::EscapeCancel));
    assert!(!cmds.contains(&ShortcutCommand::OpenPropertiesEditor));
}

#[test]
fn test_tool_shortcuts_without_focus() {
    let cmds = commands(
        &[
            Key::S,
            Key::W,
            Key::G,
            Key::D,
            Key::M,
            Key::Q,
            Key::P,
            Key::E,
        ],
        false,
        false,
        false,
    );
    assert!(cmds.contains(&ShortcutCommand::ToolSelect));
    assert!(cmds.contains(&ShortcutCommand::ToolWire));
    assert!(cmds.contains(&ShortcutCommand::PlaceGround));
    assert!(cmds.contains(&ShortcutCommand::PlaceDiode));
    assert!(cmds.contains(&ShortcutCommand::PlaceNmos));
    assert!(cmds.contains(&ShortcutCommand::PlaceNpnBjt));
    assert!(cmds.contains(&ShortcutCommand::ToolProbe));
    assert!(cmds.contains(&ShortcutCommand::OpenPropertiesEditor));
}

#[test]
fn test_tool_select_shortcut_works_with_shift_modifier() {
    let cmds = commands(&[Key::S], false, true, false);
    assert!(cmds.contains(&ShortcutCommand::ToolSelect));
}

#[test]
fn test_resistor_vs_rotate_shortcut_modes() {
    let place = commands(&[Key::R], false, true, false);
    assert!(place.contains(&ShortcutCommand::PlaceResistor));
    assert!(!place.contains(&ShortcutCommand::RotateSelectionOrPreview));

    let rotate = commands(&[Key::R], false, false, false);
    assert!(!rotate.contains(&ShortcutCommand::PlaceResistor));
    assert!(rotate.contains(&ShortcutCommand::RotateSelectionOrPreview));
}

#[test]
fn test_place_inductor_requires_no_shift() {
    let plain = commands(&[Key::L], false, false, false);
    assert!(plain.contains(&ShortcutCommand::PlaceInductor));

    let shifted = commands(&[Key::L], false, true, false);
    assert!(!shifted.contains(&ShortcutCommand::PlaceInductor));
}

#[test]
fn test_browser_shortcut_requires_ctrl_shift_l() {
    let no_shift = commands(&[Key::L], true, false, false);
    assert!(!no_shift.contains(&ShortcutCommand::ToggleBrowserPanel));

    let with_shift = commands(&[Key::L], true, true, false);
    assert!(with_shift.contains(&ShortcutCommand::ToggleBrowserPanel));
}

#[test]
fn test_untracked_key_produces_no_commands() {
    let cmds = commands(&[Key::B], false, false, false);
    assert!(cmds.is_empty());
}

#[test]
fn test_shortcut_metadata_is_non_empty_for_all_commands() {
    for command in ShortcutCommand::ALL {
        assert!(!command.display_name().trim().is_empty());
        assert!(!command.shortcut_string().trim().is_empty());
    }
}

#[test]
fn test_shortcut_categories_cover_all_commands_exactly_once() {
    let mut seen = HashSet::new();
    for category in ShortcutCategory::ALL {
        let category_commands = category.commands();
        assert!(!category_commands.is_empty());
        for command in category_commands {
            assert!(
                seen.insert(*command),
                "command {:?} appears in more than one category",
                command
            );
        }
    }

    let all_commands: HashSet<_> = ShortcutCommand::ALL.into_iter().collect();
    assert_eq!(seen, all_commands);
}
