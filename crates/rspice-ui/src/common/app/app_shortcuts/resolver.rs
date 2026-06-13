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

const GLOBAL_RULES: [ShortcutRule; 20] = [
    ShortcutRule::new(
        ShortcutCommand::OpenPreferences,
        Key::Comma,
        Some(true),
        None,
    ),
    ShortcutRule::new(
        ShortcutCommand::OpenCommandPalette,
        Key::K,
        Some(true),
        None,
    ),
    ShortcutRule::new(ShortcutCommand::FileNew, Key::N, Some(true), None),
    ShortcutRule::new(ShortcutCommand::FileOpen, Key::O, Some(true), None),
    ShortcutRule::new(ShortcutCommand::FileSave, Key::S, Some(true), None),
    ShortcutRule::new(
        ShortcutCommand::ToggleBrowserPanel,
        Key::L,
        Some(true),
        Some(true),
    ),
    ShortcutRule::new(ShortcutCommand::ToggleBrowserPanel, Key::B, Some(true), None),
    ShortcutRule::new(
        ShortcutCommand::ToggleLogPanel,
        Key::Backtick,
        Some(true),
        None,
    ),
    ShortcutRule::new(ShortcutCommand::ToggleLogPanel, Key::J, Some(true), None),
    ShortcutRule::new(ShortcutCommand::ShowShortcutsHelp, Key::F1, None, None),
    ShortcutRule::new(ShortcutCommand::RunSimulation, Key::F5, None, Some(false)),
    ShortcutRule::new(ShortcutCommand::StopSimulation, Key::F5, None, Some(true)),
    ShortcutRule::new(ShortcutCommand::RunChecks, Key::E, Some(true), None),
    ShortcutRule::new(ShortcutCommand::NextViolation, Key::F4, None, Some(false)),
    ShortcutRule::new(ShortcutCommand::PrevViolation, Key::F4, None, Some(true)),
    ShortcutRule::new(ShortcutCommand::NextWorkspaceTab, Key::Tab, Some(true), None),
    ShortcutRule::new(ShortcutCommand::ZoomIn, Key::Plus, Some(true), None),
    ShortcutRule::new(ShortcutCommand::ZoomIn, Key::Equals, Some(true), None),
    ShortcutRule::new(ShortcutCommand::ZoomOut, Key::Minus, Some(true), None),
    ShortcutRule::new(ShortcutCommand::Zoom100, Key::Num0, Some(true), None),
];

/// Edit-class shortcuts act on the schematic and must never fire while a
/// text field has focus — Ctrl+V into the license textarea must not also
/// paste components, and Delete while editing the palette query must not
/// delete the selection. Their text-editing meaning belongs to the widget.
const GLOBAL_EDIT_RULES: [ShortcutRule; 8] = [
    ShortcutRule::new(ShortcutCommand::EditUndo, Key::Z, Some(true), Some(false)),
    ShortcutRule::new(ShortcutCommand::EditRedo, Key::Y, Some(true), None),
    ShortcutRule::new(ShortcutCommand::EditRedo, Key::Z, Some(true), Some(true)),
    ShortcutRule::new(ShortcutCommand::EditCopy, Key::C, Some(true), None),
    ShortcutRule::new(ShortcutCommand::EditPaste, Key::V, Some(true), None),
    ShortcutRule::new(ShortcutCommand::EditCut, Key::X, Some(true), None),
    ShortcutRule::new(ShortcutCommand::EditDelete, Key::Delete, None, None),
    ShortcutRule::new(ShortcutCommand::EditSelectAll, Key::A, Some(true), None),
];

const FOCUS_FREE_RULES: [ShortcutRule; 23] = [
    ShortcutRule::new(
        ShortcutCommand::FocusDesignSearch,
        Key::Slash,
        Some(false),
        None,
    ),
    // Hierarchy: descend into the selected instance / ascend one level
    // (the Virtuoso-style E/U pair; double-click and the breadcrumb are
    // the mouse paths).
    ShortcutRule::new(
        ShortcutCommand::DescendIntoSelected,
        Key::E,
        Some(false),
        Some(true),
    ),
    ShortcutRule::new(ShortcutCommand::AscendHierarchy, Key::U, Some(false), None),
    ShortcutRule::new(ShortcutCommand::ToolSelect, Key::S, Some(false), None),
    ShortcutRule::new(ShortcutCommand::ToolWire, Key::W, Some(false), None),
    ShortcutRule::new(ShortcutCommand::ToolLabel, Key::N, Some(false), None),
    ShortcutRule::new(ShortcutCommand::ZoomFit, Key::F, Some(false), None),
    ShortcutRule::new(
        ShortcutCommand::FocusCellSearch,
        Key::I,
        Some(false),
        Some(true),
    ),
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
        Some(false),
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
    // Plain E only — Shift+E is hierarchy descend (Virtuoso pairing).
    ShortcutRule::new(
        ShortcutCommand::OpenPropertiesEditor,
        Key::E,
        Some(false),
        Some(false),
    ),
    ShortcutRule::new(ShortcutCommand::EscapeCancel, Key::Escape, None, None),
];

pub(crate) fn collect_shortcut_commands(snapshot: &ShortcutInputSnapshot) -> Vec<ShortcutCommand> {
    let mut commands = Vec::with_capacity(8);

    collect_matching_rules(&mut commands, snapshot, &GLOBAL_RULES);

    if snapshot.has_focus() {
        return commands;
    }

    collect_matching_rules(&mut commands, snapshot, &GLOBAL_EDIT_RULES);
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
