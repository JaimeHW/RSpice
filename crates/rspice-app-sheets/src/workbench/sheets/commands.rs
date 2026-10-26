//! The commands the sheets feature declares, as the registry lists them: the
//! sheet's properties and formats, its layers, and paging through the sheets
//! of the open drawing.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use egui::Key;

use crate::state::command_catalog::{
    CommandKey, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};

/// One command of the sheets feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SheetsCommand {
    SheetProperties,
    SheetFormatManager,
    CustomSheetSizes,
    SheetDefaults,
    DrawingSheetLayers,
    NextSheet,
    PreviousSheet,
    NewSheet,
    DeleteSheet,
}

const NONE: &[ShortcutBinding] = &[];

const fn primary(
    key: Key,
    primary: bool,
    alt: bool,
    shift: bool,
    label: &'static str,
) -> ShortcutBinding {
    ShortcutBinding {
        chord: ShortcutChord::new(key, primary, alt, shift, label),
        platforms: &CommandPlatform::ALL,
        kind: ShortcutKind::Primary,
    }
}

const SHEET_PROPERTIES: &[ShortcutBinding] =
    &[primary(Key::P, true, true, true, "Ctrl+Shift+Alt+P")];
// Paging through the sheets of one drawing is what an unmodified Page
// Up/Page Down means on an engineering canvas. The document strip's browser
// alternates are Ctrl+Alt+Page Up/Down, so neither chord is shadowed.
const PREVIOUS_SHEET: &[ShortcutBinding] = &[primary(Key::PageUp, false, false, false, "Page Up")];
const NEXT_SHEET: &[ShortcutBinding] = &[primary(Key::PageDown, false, false, false, "Page Down")];

/// What every sheet command that reaches the project declares: it is listed
/// in the palette, needs an open project and waits for a project operation.
const PROJECT_COMMAND: CommandFlags = CommandFlags::PALETTE_VISIBLE
    .with(CommandFlags::REQUIRES_OPEN_PROJECT, true)
    .with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);

/// A [`PROJECT_COMMAND`] that edits the drawing: a read-only one refuses it,
/// and the inspector's pending edit is committed first.
const DRAWING_EDIT: CommandFlags = PROJECT_COMMAND
    .with(CommandFlags::EDITS_ACTIVE_DOCUMENT, true)
    .with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);

impl SheetsCommand {
    /// Every command of the feature.
    pub const ALL: [Self; 9] = [
        Self::SheetProperties,
        Self::SheetFormatManager,
        Self::CustomSheetSizes,
        Self::SheetDefaults,
        Self::DrawingSheetLayers,
        Self::NextSheet,
        Self::PreviousSheet,
        Self::NewSheet,
        Self::DeleteSheet,
    ];

    /// The key a shortcut profile names this command by.
    pub const fn key(self) -> CommandKey {
        match self {
            // The stable ID is the one shortcut profiles saved before the
            // sheet's properties moved into the inspector.
            Self::SheetProperties => ids::SHEET_PROPERTIES,
            Self::SheetFormatManager => CommandKey::new("sheet-format-manager"),
            Self::CustomSheetSizes => CommandKey::new("sheet-preset-library"),
            Self::SheetDefaults => CommandKey::new("sheet-defaults"),
            Self::DrawingSheetLayers => CommandKey::new("sheet-layer-visibility"),
            Self::NextSheet => CommandKey::new("next-sheet"),
            Self::PreviousSheet => CommandKey::new("previous-sheet"),
            Self::NewSheet => CommandKey::new("new-sheet"),
            Self::DeleteSheet => CommandKey::new("delete-sheet"),
        }
    }

    /// What the command declares about itself.
    pub const fn declared_spec(self) -> CommandSpec {
        let (label, group, flags, context, bindings) = match self {
            Self::SheetProperties => (
                "Sheet properties",
                "File",
                PROJECT_COMMAND,
                ShortcutContext::DesignWorkspace,
                SHEET_PROPERTIES,
            ),
            Self::SheetFormatManager => (
                "Sheet formats across this document…",
                "File",
                PROJECT_COMMAND,
                ShortcutContext::DesignWorkspace,
                NONE,
            ),
            Self::CustomSheetSizes => (
                "Custom sheet sizes…",
                "File",
                PROJECT_COMMAND,
                ShortcutContext::DesignWorkspace,
                NONE,
            ),
            Self::SheetDefaults => (
                "Sheet defaults…",
                "File",
                PROJECT_COMMAND,
                ShortcutContext::DesignWorkspace,
                NONE,
            ),
            Self::DrawingSheetLayers => (
                "Drawing-sheet layers\u{2026}",
                "View",
                CommandFlags::PALETTE_VISIBLE,
                ShortcutContext::DesignWorkspace,
                NONE,
            ),
            Self::NextSheet => (
                "Next sheet",
                "Design",
                PROJECT_COMMAND,
                ShortcutContext::EngineeringCanvas,
                NEXT_SHEET,
            ),
            Self::PreviousSheet => (
                "Previous sheet",
                "Design",
                PROJECT_COMMAND,
                ShortcutContext::EngineeringCanvas,
                PREVIOUS_SHEET,
            ),
            Self::NewSheet => (
                "New sheet",
                "Design",
                DRAWING_EDIT,
                ShortcutContext::EngineeringCanvas,
                NONE,
            ),
            Self::DeleteSheet => (
                "Delete active sheet",
                "Design",
                DRAWING_EDIT,
                ShortcutContext::EngineeringCanvas,
                NONE,
            ),
        };
        CommandSpec {
            key: self.key(),
            label,
            group,
            flags,
            context,
            bindings,
            owner: CommandOwner::Sheets,
        }
    }
}
