use super::{Key, ShortcutCommand, ShortcutInputSnapshot};

#[derive(Debug, Clone, Copy)]
struct ShortcutRule {
    command: ShortcutCommand,
    key: Key,
    ctrl: Option<bool>,
    shift: Option<bool>,
}

impl ShortcutRule {
    const fn new(
        command: ShortcutCommand,
        key: Key,
        ctrl: Option<bool>,
        shift: Option<bool>,
    ) -> Self {
        Self {
            command,
            key,
            ctrl,
            shift,
        }
    }

    fn matches(self, snapshot: &ShortcutInputSnapshot) -> bool {
        if !snapshot.key_pressed(self.key) {
            return false;
        }

        if let Some(required_ctrl) = self.ctrl
            && snapshot.ctrl() != required_ctrl
        {
            return false;
        }

        if let Some(required_shift) = self.shift
            && snapshot.shift() != required_shift
        {
            return false;
        }

        true
    }
}

const GLOBAL_RULES: [ShortcutRule; 14] = [
    ShortcutRule::new(ShortcutCommand::FileNew, Key::N, Some(true), None),
    ShortcutRule::new(ShortcutCommand::FileOpen, Key::O, Some(true), None),
    ShortcutRule::new(ShortcutCommand::FileSave, Key::S, Some(true), None),
    ShortcutRule::new(ShortcutCommand::EditUndo, Key::Z, Some(true), Some(false)),
    ShortcutRule::new(ShortcutCommand::EditRedo, Key::Y, Some(true), None),
    ShortcutRule::new(ShortcutCommand::EditRedo, Key::Z, Some(true), Some(true)),
    ShortcutRule::new(ShortcutCommand::EditCopy, Key::C, Some(true), None),
    ShortcutRule::new(ShortcutCommand::EditPaste, Key::V, Some(true), None),
    ShortcutRule::new(ShortcutCommand::EditCut, Key::X, Some(true), None),
    ShortcutRule::new(ShortcutCommand::EditDelete, Key::Delete, None, None),
    ShortcutRule::new(ShortcutCommand::EditSelectAll, Key::A, Some(true), None),
    ShortcutRule::new(
        ShortcutCommand::ToggleBrowserPanel,
        Key::L,
        Some(true),
        Some(true),
    ),
    ShortcutRule::new(
        ShortcutCommand::ToggleLogPanel,
        Key::Backtick,
        Some(true),
        None,
    ),
    ShortcutRule::new(ShortcutCommand::ShowShortcutsHelp, Key::F1, None, None),
];

const FOCUS_FREE_RULES: [ShortcutRule; 17] = [
    ShortcutRule::new(ShortcutCommand::ToolSelect, Key::S, Some(false), None),
    ShortcutRule::new(ShortcutCommand::ToolWire, Key::W, Some(false), None),
    ShortcutRule::new(ShortcutCommand::PlaceGround, Key::G, Some(false), None),
    ShortcutRule::new(
        ShortcutCommand::PlaceVoltageSource,
        Key::V,
        Some(false),
        None,
    ),
    ShortcutRule::new(
        ShortcutCommand::PlaceCurrentSource,
        Key::I,
        Some(false),
        None,
    ),
    ShortcutRule::new(ShortcutCommand::PlaceCapacitor, Key::C, Some(false), None),
    ShortcutRule::new(
        ShortcutCommand::PlaceInductor,
        Key::L,
        Some(false),
        Some(false),
    ),
    ShortcutRule::new(ShortcutCommand::PlaceDiode, Key::D, Some(false), None),
    ShortcutRule::new(ShortcutCommand::PlaceNmos, Key::M, Some(false), None),
    ShortcutRule::new(ShortcutCommand::PlaceNpnBjt, Key::Q, Some(false), None),
    ShortcutRule::new(ShortcutCommand::ToolProbe, Key::P, Some(false), None),
    ShortcutRule::new(
        ShortcutCommand::PlaceResistor,
        Key::R,
        Some(false),
        Some(true),
    ),
    ShortcutRule::new(
        ShortcutCommand::RotateSelectionOrPreview,
        Key::R,
        Some(false),
        Some(false),
    ),
    ShortcutRule::new(
        ShortcutCommand::MirrorSelectionHorizontal,
        Key::H,
        Some(false),
        None,
    ),
    ShortcutRule::new(
        ShortcutCommand::MirrorSelectionVertical,
        Key::Y,
        Some(false),
        None,
    ),
    ShortcutRule::new(
        ShortcutCommand::OpenPropertiesEditor,
        Key::E,
        Some(false),
        None,
    ),
    ShortcutRule::new(ShortcutCommand::EscapeCancel, Key::Escape, None, None),
];

pub(crate) fn collect_shortcut_commands(snapshot: &ShortcutInputSnapshot) -> Vec<ShortcutCommand> {
    let mut commands = Vec::with_capacity(8);

    collect_matching_rules(&mut commands, snapshot, &GLOBAL_RULES);

    if snapshot.has_focus() {
        return commands;
    }

    collect_matching_rules(&mut commands, snapshot, &FOCUS_FREE_RULES);

    commands
}

fn collect_matching_rules(
    commands: &mut Vec<ShortcutCommand>,
    snapshot: &ShortcutInputSnapshot,
    rules: &[ShortcutRule],
) {
    for rule in rules {
        if rule.matches(snapshot) && !commands.contains(&rule.command) {
            commands.push(rule.command);
        }
    }
}
