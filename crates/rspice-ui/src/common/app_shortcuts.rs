use egui::{InputState, Key, Modifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ShortcutCommand {
    FileNew,
    FileOpen,
    FileSave,
    EditUndo,
    EditRedo,
    EditCopy,
    EditPaste,
    EditCut,
    EditDelete,
    EditSelectAll,
    ToggleBrowserPanel,
    ToggleLogPanel,
    ShowShortcutsHelp,
    ToolSelect,
    ToolWire,
    PlaceGround,
    PlaceVoltageSource,
    PlaceCurrentSource,
    PlaceCapacitor,
    PlaceInductor,
    PlaceDiode,
    PlaceNmos,
    PlaceNpnBjt,
    ToolProbe,
    PlaceResistor,
    RotateSelectionOrPreview,
    MirrorSelectionHorizontal,
    MirrorSelectionVertical,
    OpenPropertiesEditor,
    EscapeCancel,
}

#[derive(Debug, Clone)]
pub(super) struct ShortcutInputSnapshot {
    pressed_keys: Vec<Key>,
    modifiers: Modifiers,
    has_focus: bool,
}

impl ShortcutInputSnapshot {
    pub(super) fn from_input_state(input: &InputState, has_focus: bool) -> Self {
        let tracked_keys = [
            Key::N,
            Key::O,
            Key::S,
            Key::Z,
            Key::Y,
            Key::C,
            Key::V,
            Key::X,
            Key::Delete,
            Key::A,
            Key::L,
            Key::Backtick,
            Key::F1,
            Key::W,
            Key::G,
            Key::I,
            Key::D,
            Key::M,
            Key::Q,
            Key::P,
            Key::R,
            Key::H,
            Key::E,
            Key::Escape,
        ];
        let pressed_keys = tracked_keys
            .iter()
            .copied()
            .filter(|key| input.key_pressed(*key))
            .collect();
        Self {
            pressed_keys,
            modifiers: input.modifiers,
            has_focus,
        }
    }

    #[cfg(test)]
    fn from_keys(pressed_keys: &[Key], modifiers: Modifiers, has_focus: bool) -> Self {
        Self {
            pressed_keys: pressed_keys.to_vec(),
            modifiers,
            has_focus,
        }
    }

    fn key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }
}

pub(super) fn collect_shortcut_commands(snapshot: &ShortcutInputSnapshot) -> Vec<ShortcutCommand> {
    let mut commands = Vec::new();
    let ctrl = snapshot.modifiers.ctrl;
    let shift = snapshot.modifiers.shift;

    // File shortcuts
    if snapshot.key_pressed(Key::N) && ctrl {
        commands.push(ShortcutCommand::FileNew);
    }
    if snapshot.key_pressed(Key::O) && ctrl {
        commands.push(ShortcutCommand::FileOpen);
    }
    if snapshot.key_pressed(Key::S) && ctrl {
        commands.push(ShortcutCommand::FileSave);
    }

    // Edit shortcuts
    if snapshot.key_pressed(Key::Z) && ctrl && !shift {
        commands.push(ShortcutCommand::EditUndo);
    }
    if (snapshot.key_pressed(Key::Y) && ctrl) || (snapshot.key_pressed(Key::Z) && ctrl && shift) {
        commands.push(ShortcutCommand::EditRedo);
    }
    if snapshot.key_pressed(Key::C) && ctrl {
        commands.push(ShortcutCommand::EditCopy);
    }
    if snapshot.key_pressed(Key::V) && ctrl {
        commands.push(ShortcutCommand::EditPaste);
    }
    if snapshot.key_pressed(Key::X) && ctrl {
        commands.push(ShortcutCommand::EditCut);
    }
    if snapshot.key_pressed(Key::Delete) {
        commands.push(ShortcutCommand::EditDelete);
    }
    if snapshot.key_pressed(Key::A) && ctrl {
        commands.push(ShortcutCommand::EditSelectAll);
    }

    // View/help shortcuts
    if snapshot.key_pressed(Key::L) && ctrl && shift {
        commands.push(ShortcutCommand::ToggleBrowserPanel);
    }
    if snapshot.key_pressed(Key::Backtick) && ctrl {
        commands.push(ShortcutCommand::ToggleLogPanel);
    }
    if snapshot.key_pressed(Key::F1) {
        commands.push(ShortcutCommand::ShowShortcutsHelp);
    }

    // Tool/editing shortcuts are only active when no text widget is focused.
    if snapshot.has_focus {
        return commands;
    }

    if snapshot.key_pressed(Key::S) && !ctrl {
        commands.push(ShortcutCommand::ToolSelect);
    }
    if snapshot.key_pressed(Key::W) && !ctrl {
        commands.push(ShortcutCommand::ToolWire);
    }
    if snapshot.key_pressed(Key::G) && !ctrl {
        commands.push(ShortcutCommand::PlaceGround);
    }
    if snapshot.key_pressed(Key::V) && !ctrl {
        commands.push(ShortcutCommand::PlaceVoltageSource);
    }
    if snapshot.key_pressed(Key::I) && !ctrl {
        commands.push(ShortcutCommand::PlaceCurrentSource);
    }
    if snapshot.key_pressed(Key::C) && !ctrl {
        commands.push(ShortcutCommand::PlaceCapacitor);
    }
    if snapshot.key_pressed(Key::L) && !ctrl && !shift {
        commands.push(ShortcutCommand::PlaceInductor);
    }
    if snapshot.key_pressed(Key::D) && !ctrl {
        commands.push(ShortcutCommand::PlaceDiode);
    }
    if snapshot.key_pressed(Key::M) && !ctrl {
        commands.push(ShortcutCommand::PlaceNmos);
    }
    if snapshot.key_pressed(Key::Q) && !ctrl {
        commands.push(ShortcutCommand::PlaceNpnBjt);
    }
    if snapshot.key_pressed(Key::P) && !ctrl {
        commands.push(ShortcutCommand::ToolProbe);
    }
    if snapshot.key_pressed(Key::R) && shift && !ctrl {
        commands.push(ShortcutCommand::PlaceResistor);
    }
    if snapshot.key_pressed(Key::R) && !ctrl && !shift {
        commands.push(ShortcutCommand::RotateSelectionOrPreview);
    }
    if snapshot.key_pressed(Key::H) && !ctrl {
        commands.push(ShortcutCommand::MirrorSelectionHorizontal);
    }
    if snapshot.key_pressed(Key::Y) && !ctrl {
        commands.push(ShortcutCommand::MirrorSelectionVertical);
    }
    if snapshot.key_pressed(Key::E) && !ctrl {
        commands.push(ShortcutCommand::OpenPropertiesEditor);
    }
    if snapshot.key_pressed(Key::Escape) {
        commands.push(ShortcutCommand::EscapeCancel);
    }

    commands
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_redo_shortcuts_both_variants() {
        let ctrl_y = commands(&[Key::Y], true, false, false);
        assert!(ctrl_y.contains(&ShortcutCommand::EditRedo));

        let ctrl_shift_z = commands(&[Key::Z], true, true, false);
        assert!(ctrl_shift_z.contains(&ShortcutCommand::EditRedo));
    }

    #[test]
    fn test_ctrl_shortcuts_do_not_emit_tool_commands() {
        let cmds = commands(&[Key::C, Key::V, Key::L], true, false, false);
        assert!(cmds.contains(&ShortcutCommand::EditCopy));
        assert!(cmds.contains(&ShortcutCommand::EditPaste));
        assert!(cmds.contains(&ShortcutCommand::ToggleLogPanel) == false);
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
    fn test_resistor_vs_rotate_shortcut_modes() {
        let place = commands(&[Key::R], false, true, false);
        assert!(place.contains(&ShortcutCommand::PlaceResistor));
        assert!(!place.contains(&ShortcutCommand::RotateSelectionOrPreview));

        let rotate = commands(&[Key::R], false, false, false);
        assert!(!rotate.contains(&ShortcutCommand::PlaceResistor));
        assert!(rotate.contains(&ShortcutCommand::RotateSelectionOrPreview));
    }

    #[test]
    fn test_browser_shortcut_requires_ctrl_shift_l() {
        let no_shift = commands(&[Key::L], true, false, false);
        assert!(!no_shift.contains(&ShortcutCommand::ToggleBrowserPanel));

        let with_shift = commands(&[Key::L], true, true, false);
        assert!(with_shift.contains(&ShortcutCommand::ToggleBrowserPanel));
    }
}
